use rltk::{Point, RGB};
use specs::prelude::*;

#[derive(Component)]
pub struct Position {
    pub pt: Point,
}

#[derive(Component)]
pub struct Renderable {
    pub glyph: u8,
    pub fg: RGB,
    pub bg: RGB,
}

#[derive(Component)]
pub struct Name {
    pub name: String,
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Player {}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Monster {}

#[derive(Component, Debug)]
pub struct CombatStats {
    pub max_hp: i32,
    pub cur_hp: i32,
    pub atk: i32,
    pub def: i32,
}

#[derive(Component, Debug)]
pub struct MeleeMessage {
    pub target: Entity,
}

#[derive(Component, Debug)]
pub struct DamageMessage {
    pub val: i32,
}

#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles: Vec<Point>,
    pub range: i32,
    pub dirty: bool,
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct BlocksTile {}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct BlocksSight {}
