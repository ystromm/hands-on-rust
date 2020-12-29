mod map;
mod player;

use bracket_lib::prelude::*;
use crate::prelude::{Map, SCREEN_WIDTH, SCREEN_HEIGHT};
use crate::player::Player;

pub mod prelude {
    pub use bracket_lib::prelude::*;

    pub const SCREEN_HEIGHT: i32 = 50;
    pub const SCREEN_WIDTH: i32 = 80;

    pub use crate::map::*;
    pub use crate::player::*;
}

struct State {
    map: Map,
    player: Player,
}

impl State {
    fn new() -> Self {
        State {
            map: Map::new(),
            player: Player::new(Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2)),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.player.update(ctx, &self.map);
        self.map.render(ctx);
        self.player.render(ctx);
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Dungeon Crawler")
        .build()?;
    main_loop(context, State::new())
}
