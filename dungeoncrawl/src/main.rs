mod map;

use bracket_lib::prelude::*;
use crate::prelude::Map;

pub mod prelude {
    pub use bracket_lib::prelude::*;

    pub const SCREEN_HEIGHT: i32 = 50;
    pub const SCREEN_WIDTH: i32 = 80;

    pub use crate::map::*;
}

struct State {
    map: Map,
}

impl State {
    fn new() -> Self { State {
        map: Map::new()
    } }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.map.render(ctx);
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Dungeon Crawler")
        .build()?;
    main_loop(context, State::new())
}
