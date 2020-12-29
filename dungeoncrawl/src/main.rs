mod map;
mod player;
mod map_builder;
mod camera;

use bracket_lib::prelude::*;
use crate::prelude::{Map, SCREEN_WIDTH, SCREEN_HEIGHT};
use crate::player::Player;
use crate::map_builder::MapBuilder;
use crate::camera::Camera;

pub mod prelude {
    pub use bracket_lib::prelude::*;

    pub const SCREEN_HEIGHT: i32 = 50;
    pub const SCREEN_WIDTH: i32 = 80;

    pub use crate::map::*;
    pub use crate::player::*;
    pub use crate::map_builder::*;
    pub use crate::camera::*;
}

struct State {
    map: Map,
    player: Player,
    camera: Camera,
}

impl State {
    fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::build(&mut rng);
        State {
            map: map_builder.map,
            player: Player::new(map_builder.player_start),
            camera: Camera::new(map_builder.player_start)
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.player.update(ctx, &self.map, &mut self.camera);
        ctx.set_active_console(0);
        self.map.render(ctx, &self.camera);
        ctx.set_active_console(1);
        self.player.render(ctx, &self.camera);
    }
}

const DISPLAY_WIDTH: i32 = SCREEN_WIDTH/2;
const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT/2;

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Dungeon Crawler")
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .build()?;
    main_loop(context, State::new())
}
