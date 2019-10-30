use std::cmp::{max, min};

use rltk::{Algorithm2D, Point, Rltk, VirtualKeyCode};
use specs::prelude::*;

use super::components::*;
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

            VirtualKeyCode::Escape => return RunState::Exit,
            _ => return RunState::AwaitingInput,
        },
    }

    return RunState::PlayerTurn;
}
