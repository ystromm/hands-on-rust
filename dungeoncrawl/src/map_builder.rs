use bracket_lib::prelude::{Rect, Point};
use crate::prelude::*;

const NUM_ROOMS: usize = 20;

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub player_start: Point,
}

impl MapBuilder {
    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn build_random_rooms(&mut self, rng: &mut RandomNumberGenerator) {
        while self.rooms.len() < NUM_ROOMS {
            let new_room = Rect::with_size(rng.range(0, SCREEN_WIDTH - 10),
                                           rng.range(0, SCREEN_HEIGHT - 10),
                                           rng.range(2, 10),
                                           rng.range(2, 10),
            );
            if let None = self.rooms.iter().find(|room| room.intersect(&new_room)) {
                new_room.for_each(|point| {
                    if point.x > 0 && point.x < SCREEN_WIDTH &&
                        point.y > 0 && point.y < SCREEN_HEIGHT {
                        let idx = map_idx(point.x, point.y);
                        self.map.tiles[idx] = TileType::Floor;
                    }
                });
                self.rooms.push(new_room);
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        use std::cmp::{min, max};
        for y in min(y1, y2)..max(y1, y2) {
            let point = Point::new(x, y);
            if let Some(idx) = self.map.try_idx(point) {
                self.map.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        use std::cmp::{min, max};
        for x in min(x1, x2)..max(x1, x2) {
            let point = Point::new(x, y);
            if let Some(idx) = self.map.try_idx(point) {
                self.map.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn build_corridors(&mut self, rng: &mut RandomNumberGenerator) {
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));
        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].center();
            let new = room.center();
            if rng.range(0, 2) == 1 {
                self.apply_horizontal_tunnel(prev.x, new.x, prev.y);
                self.apply_vertical_tunnel(prev.y, new.y, prev.x);
            } else {
                self.apply_vertical_tunnel(prev.y, new.y, prev.x);
                self.apply_horizontal_tunnel(prev.x, new.x, prev.y);
            }
        }
    }


    pub fn build(rng: &mut RandomNumberGenerator) -> Self {
        let mut map_builder = Self {
            map: Map::new(),
            rooms: vec![],
            player_start: Point::new(1, 1),
        };
        map_builder.fill(TileType::Wall);
        map_builder.build_random_rooms(rng);
        map_builder.build_corridors(rng);
        map_builder.player_start = map_builder.rooms[0].center();
        map_builder
    }
}

