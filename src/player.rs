use std::cmp::{max, min};

use rltk::{Algorithm2D, Point, Rltk, VirtualKeyCode};
use specs::prelude::*;

use super::components::*;
use super::game_log::*;
use super::map::*;
use super::{RunState, State};

pub fn move_player(dx: i32, dy: i32, ecs: &mut World) {
    let mut position = ecs.write_storage::<Position>();
    let players = ecs.write_storage::<Player>();
    let mut viewshed = ecs.write_storage::<Viewshed>();
    let mut melee = ecs.write_storage::<MeleeMessage>();
    let entities = ecs.entities();
    let combat = ecs.read_storage::<CombatStats>();
    let map = ecs.fetch::<Map>();

    for (entity, _player, pos, vs) in (&entities, &players, &mut position, &mut viewshed).join() {
        let dxy = Point::new(dx, dy);
        let dest = map.point2d_to_index(pos.pt + dxy) as usize;

        for content in map.tile_content[dest].iter() {
            let stats = combat.get(*content);
            if let Some(_t) = stats {
                melee
                    .insert(entity, MeleeMessage { target: *content })
                    .expect("Add target failed.");
            }
        }

        if !map.blocked_tiles[dest] {
            pos.pt.x = min(map.width - 1, max(0, pos.pt.x + dx));
            pos.pt.y = min(map.height - 1, max(0, pos.pt.y + dy));

            vs.dirty = true;
            let mut pos_res = ecs.write_resource::<Point>();
            *pos_res = pos.pt;
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
    match ctx.key {
        None => return RunState::AwaitingInput,
        Some(key) => match key {
            VirtualKeyCode::Numpad2 => move_player(0, 1, &mut gs.ecs),
            VirtualKeyCode::Numpad4 => move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Numpad6 => move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Numpad8 => move_player(0, -1, &mut gs.ecs),

            VirtualKeyCode::Numpad1 => move_player(-1, 1, &mut gs.ecs),
            VirtualKeyCode::Numpad3 => move_player(1, 1, &mut gs.ecs),
            VirtualKeyCode::Numpad7 => move_player(-1, -1, &mut gs.ecs),
            VirtualKeyCode::Numpad9 => move_player(1, -1, &mut gs.ecs),

            VirtualKeyCode::G => get_item(&mut gs.ecs),
            VirtualKeyCode::I => return RunState::ShowInventory,

            VirtualKeyCode::Escape => return RunState::Exit,
            _ => return RunState::AwaitingInput,
        },
    }

    return RunState::PlayerTurn;
}

fn get_item(ecs: &mut World) {
    let player_pos = ecs.fetch::<Point>();
    let player_entity = ecs.fetch::<Entity>();
    let entities = ecs.entities();
    let items = ecs.read_storage::<Item>();
    let positions = ecs.read_storage::<Position>();
    let mut gamelog = ecs.fetch_mut::<GameLog>();    

    let mut target_item : Option<Entity> = None;
    for (item_entity, _item, position) in (&entities, &items, &positions).join() {
        if position.pt.x == player_pos.x && position.pt.y == player_pos.y {
            target_item = Some(item_entity);
        }
    }

    match target_item {
        None => gamelog.add_message("There is nothing here to pick up.".to_string()),
        Some(item) => {
            let mut pickup = ecs.write_storage::<ItemPickupMessage>();
            pickup.insert(*player_entity, ItemPickupMessage{ collected_by: *player_entity, item }).expect("Unable to insert want to pickup");
        }
    }
}