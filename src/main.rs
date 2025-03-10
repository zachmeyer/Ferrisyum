// # GLOBAL LINT FLAGS
// #
#![allow(dead_code, unused_variables, unused_imports, unused_braces)]
#![feature(file_buffered)]

// > USE STD
use std::collections::BinaryHeap;

// > USE 3P
use color_eyre::Result as CEResult;
use ratatui::{
    layout::{Layout, Direction, Constraint},
    text::Text,
    widgets::{Block, Borders, Padding, Paragraph}
};

// < MOD
mod player;
pub(crate) use player::Player;

mod shared;
use shared::extlib::{
    CrosstermEvent, CrosstermKeyCode, crossterm_event,
    NAVector3, RatatuiDefaultTerminal,
};
use shared::MoveDirection;

mod world;
use world::{WorldController, WorldView, WorldUpdate, WorldUpdateEventType, ExplicitPickupType};

///////////////////////////////////////////////////////////////////////////////////////////////////

/// ### MAIN ENTRY POINT
///
/// * Initializes ratatui and starts the game loop
fn main() -> CEResult<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = init_game_loop(terminal);
    ratatui::restore();
    result
}

/// ### INIT GAME LOOP
///
/// * Sets up the terminal, initializes the render context, and handles key input
fn init_game_loop(mut terminal: RatatuiDefaultTerminal) -> CEResult<()> {

    let mut world_update_queue: BinaryHeap<WorldUpdate<WorldUpdateEventType>> 
        = BinaryHeap::new();

    let mut player = Player::new('@', NAVector3::new(2, 1, 1));
    let mut world = WorldController::new(&mut world_update_queue);
    let mut show_stats = true;
    let mut show_inventory = true;
    
    loop {
        // TODO: THIS NEEDS TO BE MOVED INTO A UI/INPUT HANDLING MODULE ///////////////////////////
        terminal.draw(|f| {
            let vertical_chunks = if show_stats {
                Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Min(30),
                        Constraint::Percentage(30), 
                    ])
                    .split(f.area())
            } else {
                // Full screen
                Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Min(0),
                    ])
                    .split(f.area())
            };

            let horizontal_chunks = if show_inventory {
                Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Percentage(70), 
                        Constraint::Percentage(30), 
                    ])
                    .split(vertical_chunks[0])
            } else {
                // Full screen
                Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Min(0),
                    ])
                    .split(f.area())
            };

            // Create the WorldView which will handle rendering the map
            let world_view = WorldView::new(&world, &player);

            // Render the WorldView in the game area
            f.render_widget(world_view, horizontal_chunks[0]);

            // Render inventory if visible
            if show_inventory {
                f.render_widget(
                    Block::default()
                        .title("Inventory")
                        .borders(Borders::ALL), 
                    horizontal_chunks[1]
                );
            }
                
            // Render messages is visible
            if show_stats {
                let stats_block = Block::default()
                    .title("Stats")
                    .borders(Borders::ALL)
                    .padding(Padding::new(1, 1, 1, 1));

                let gold_text = Paragraph::new(Text::raw(format!("Gold: {}", player.gold_qty())))
                    .block(stats_block);

                f.render_widget(gold_text, vertical_chunks[1]);
            }
        })?;
        ////////////////////////////////////////////////////////////////////////////////////// TODO
        
        if let CrosstermEvent::Key(key_event) = crossterm_event::read()? {
            match key_event.code {
                CrosstermKeyCode::Char('w') | CrosstermKeyCode::Up => {
                    world::translate(&mut player, &mut world, MoveDirection::UP);
                }
                CrosstermKeyCode::Char('a') | CrosstermKeyCode::Left => {
                    world::translate(&mut player, &mut world, MoveDirection::LEFT);
                }
                CrosstermKeyCode::Char('s') | CrosstermKeyCode::Down => {
                    world::translate(&mut player, &mut world, MoveDirection::DOWN);
                }
                CrosstermKeyCode::Char('d') | CrosstermKeyCode::Right => {
                    world::translate(&mut player, &mut world, MoveDirection::RIGHT);
                }
                CrosstermKeyCode::Char('i') => {
                    show_inventory = !show_inventory;
                },
                CrosstermKeyCode::Char('c') => {
                    show_stats = !show_stats;
                }
                CrosstermKeyCode::Char(' ') => {
                    world::pickup_explicit(
                        &mut player, &mut world, ExplicitPickupType::TreasureChest
                    );
                }
                CrosstermKeyCode::Char('q') | CrosstermKeyCode::Esc => break Ok(()),
                _ => (),
            }

            world.update_world(&mut player);
        }
    }
}
