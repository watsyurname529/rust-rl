use std::cmp::{max, min};

use super::util::*;
use rltk::{Algorithm2D, BaseMap, Console, Point, RandomNumberGenerator, Rltk, RGB};
use specs::prelude::*;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

#[derive(Default)]
pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub width: i32,
    pub height: i32,
    pub num_tiles: i32,
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
    pub blocked_tiles: Vec<bool>,
    pub tile_content: Vec<Vec<Entity>>,
}

impl Map {
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        return (y * self.width + x) as usize;
    }

    pub fn new(width: i32, height: i32) -> Map {
        let num = (width * height) as usize;
        return Map {
            tiles: vec![TileType::Wall; num],
            rooms: Vec::new(),
            width: width,
            height: height,
            num_tiles: num as i32,
            revealed_tiles: vec![false; num],
            visible_tiles: vec![false; num],
            blocked_tiles: vec![false; num],
            tile_content: vec![Vec::new(); num],
        };
    }

    pub fn new_map_rand() -> Map {
        let mut map = Map {
            tiles: vec![TileType::Floor; 80 * 50],
            rooms: Vec::new(),
            width: 80,
            height: 50,
            num_tiles: 80 * 50,
            revealed_tiles: vec![false; 80 * 50],
            visible_tiles: vec![false; 80 * 50],
            blocked_tiles: vec![false; 80 * 50],
            tile_content: vec![Vec::new(); 80 * 50],
        };

        // let mut map = Map::new(80, 50);

        map.rooms
            .push(Rect::new(1, 1, map.width - 1, map.height - 1));

        for x in 0..80 {
            let idx = map.xy_idx(x, 0);
            map.tiles[idx] = TileType::Wall;
            let idx = map.xy_idx(x, map.height - 1);
            map.tiles[idx] = TileType::Wall;
        }

        for y in 0..50 {
            let idx = map.xy_idx(0, y);
            map.tiles[idx] = TileType::Wall;
            let idx = map.xy_idx(map.width - 1, y);
            map.tiles[idx] = TileType::Wall;
        }

        let mut rng = rltk::RandomNumberGenerator::new();
        for _i in 0..400 {
            let x = rng.roll_dice(1, 79);
            let y = rng.roll_dice(1, 49);
            let idx = map.xy_idx(x, y);

            if idx != map.xy_idx(40, 25) {
                map.tiles[idx] = TileType::Wall;
            }
        }

        return map;
    }

    pub fn new_map_rooms(w: i32, h: i32) -> Map {
        let mut map = Map::new(w, h);

        const MAX_ROOMS: i32 = 32;
        const MIN_SIZE: i32 = 5;
        const MAX_SIZE: i32 = 12;
        let mut rng = RandomNumberGenerator::new();

        for _i in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.range(1, map.width - w);
            let y = rng.range(1, map.height - h);
            let new_room = Rect::new(x, y, w, h);
            let mut ok = true;
            for other_room in map.rooms.iter() {
                if new_room.intersect(other_room) {
                    ok = false;
                }
            }
            if ok {
                map.apply_room(&new_room);

                if !map.rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = map.rooms[map.rooms.len() - 1].center();
                    if rng.roll_dice(1, 2) == 1 {
                        map.apply_tunnel_h(prev_x, new_x, prev_y);
                        map.apply_tunnel_v(prev_y, new_y, new_x);
                    } else {
                        map.apply_tunnel_v(prev_y, new_y, prev_x);
                        map.apply_tunnel_h(prev_x, new_x, new_y);
                    }
                }

                map.rooms.push(new_room);
            }
        }

        return map;
    }

    pub fn apply_room(&mut self, room: &Rect) {
        for y in room.y1..room.y2 {
            for x in room.x1..room.x2 {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    pub fn apply_tunnel_h(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < self.num_tiles as usize {
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    pub fn apply_tunnel_v(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < self.num_tiles as usize {
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn is_exit_valid(&self, x: i32, y: i32) -> bool {
        if x < 1 || x > self.width - 1 || y < 1 || y > self.height - 1 {
            return false;
        }

        let idx = (y * self.width) + x;
        return !self.blocked_tiles[idx as usize];
    }

    pub fn populate_blocked(&mut self) {
        for (i, tile) in self.tiles.iter_mut().enumerate() {
            if *tile == TileType::Wall {
                self.blocked_tiles[i] = true;
            } else {
                self.blocked_tiles[i] = false;
            }
        }
    }

    pub fn clear_content_index(&mut self) {
        for content in self.tile_content.iter_mut() {
            content.clear();
        }
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: i32) -> bool {
        if -1 < idx && idx < self.num_tiles {
            return self.tiles[idx as usize] == TileType::Wall;
        } else {
            return false;
        }
    }

    fn get_available_exits(&self, idx: i32) -> Vec<(i32, f32)> {
        let mut exits: Vec<(i32, f32)> = Vec::new();
        let x = idx % self.width;
        let y = idx / self.width;

        if self.is_exit_valid(x - 1, y) {
            exits.push((self.xy_idx(x - 1, y) as i32, 1.0));
        };
        if self.is_exit_valid(x + 1, y) {
            exits.push((self.xy_idx(x + 1, y) as i32, 1.0));
        };
        if self.is_exit_valid(x, y - 1) {
            exits.push((self.xy_idx(x, y - 1) as i32, 1.0));
        };
        if self.is_exit_valid(x, y + 1) {
            exits.push((self.xy_idx(x, y + 1) as i32, 1.0));
        };
        if self.is_exit_valid(x - 1, y - 1) {
            exits.push((self.xy_idx(x - 1, y - 1) as i32, 1.0));
        };
        if self.is_exit_valid(x + 1, y - 1) {
            exits.push((self.xy_idx(x + 1, y - 1) as i32, 1.0));
        };
        if self.is_exit_valid(x - 1, y + 1) {
            exits.push((self.xy_idx(x - 1, y + 1) as i32, 1.0));
        };
        if self.is_exit_valid(x + 1, y + 1) {
            exits.push((self.xy_idx(x + 1, y + 1) as i32, 1.0));
        };

        return exits;
    }

    fn get_pathing_distance(&self, idx1: i32, idx2: i32) -> f32 {
        let p1 = self.index_to_point2d(idx1);
        let p2 = self.index_to_point2d(idx2);

        return rltk::DistanceAlg::Pythagoras.distance2d(p1, p2);
    }
}

impl Algorithm2D for Map {
    fn point2d_to_index(&self, pt: Point) -> i32 {
        return (pt.y * self.width) + pt.x;
    }

    fn index_to_point2d(&self, idx: i32) -> Point {
        return Point {
            x: idx % self.width,
            y: idx / self.width,
        };
    }
}

pub fn draw_map(map: &Map, ctx: &mut Rltk) {
    for (idx, tile) in map.tiles.iter().enumerate() {
        let pt = map.index_to_point2d(idx as i32);

        if map.revealed_tiles[idx] {
            let glyph;
            let mut fg;
            let mut bg;

            match tile {
                TileType::Floor => {
                    fg = RGB::from_f32(0.4, 0.4, 0.7);
                    bg = RGB::from_f32(0.0, 0.0, 0.0);
                    glyph = rltk::to_cp437('.');
                }
                TileType::Wall => {
                    fg = RGB::from_f32(0.0, 0.4, 0.0);
                    bg = RGB::from_f32(0.0, 0.0, 0.0);
                    glyph = rltk::to_cp437('#');
                }
            }

            if !map.visible_tiles[idx] {
                fg = fg.to_greyscale();
            }

            // if map.blocked_tiles[idx] {
            //     bg = RGB::from_f32(1.0, 0.0, 0.0);
            // }

            ctx.set(pt.x, pt.y, fg, bg, glyph)
        }
    }
}
