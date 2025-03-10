// > CRATE
use crate::shared::extlib::{RatatuiRect, RatatuiWidget, RatatuiBuffer};
use crate::Player;

// > SUPER
use super::WorldController;

///////////////////////////////////////////////////////////////////////////////////////////////////

/// Encapsulates the View which is the renderable component of the `WorldController`
pub struct WorldView<'wview> {
    world: &'wview WorldController<'wview>,
    player: &'wview Player,
}

impl<'wview> WorldView<'wview> {

    /// Creates a new `WorldView` with the specified world controller and player.
    ///
    /// # Arguments
    ///
    /// * `world` - A reference to the world controller.
    /// * `player` - A reference to the player.
    ///
    /// # Returns
    ///
    /// A new `WorldView` instance.
    pub fn new(
        world: &'wview WorldController, 
        player: &'wview Player, 
    ) -> Self 
    {
        Self { world, player }
    }
}

impl<'wview> RatatuiWidget for WorldView<'wview> {

    /// Renders the world view within the specified area and buffer.
    ///
    /// # Arguments
    ///
    /// * `area` - The area to render the world view.
    /// * `buf` - The buffer to render the world view into.
    fn render(self, area: RatatuiRect, buf: &mut RatatuiBuffer) {
            let world = self.world;
            let map_widget = world.generate_map(self.player);

            map_widget.render(area, buf);
        }
}