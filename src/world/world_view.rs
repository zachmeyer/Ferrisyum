use crate::shared::extlib::{RatatuiRect, RatatuiWidget, RatatuiBuffer};
use super::WorldController;
use crate::Player;

///////////////////////////////////////////////////////////////////////////////////////////////////

pub struct WorldView<'wview> {
    world: &'wview WorldController<'wview>,
    player: &'wview Player,
}

impl<'wview> WorldView<'wview> {
    pub fn new(
        world: &'wview WorldController, 
        player: &'wview Player, 
) -> Self {
        Self { world, player }
    }
}

impl<'wview> RatatuiWidget for WorldView<'wview> {

fn render(self, area: RatatuiRect, buf: &mut RatatuiBuffer) {
        let world = self.world;
        let player = self.player;
        let map_widget = world.generate_map(self.player);

        map_widget.render(area, buf);
    }
}