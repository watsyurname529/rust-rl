use super::{Map, MeleeMessage, Monster, Position, RunState, Viewshed};
use rltk::Algorithm2D;
use specs::prelude::*;

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadExpect<'a, Entity>,
        ReadExpect<'a, RunState>,
        Entities<'a>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Monster>,
        WriteStorage<'a, MeleeMessage>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, player, runstate, entities, mut vis, mut pos, mon, mut melee_msg) = data;
        let player_pos = pos.get(*player).unwrap().pt;

        if *runstate != RunState::MonsterTurn {
            return;
        }

        for (entity, mut vs, mut pos, _mon) in (&entities, &mut vis, &mut pos, &mon).join() {
            if vs.visible_tiles.contains(&player_pos) {
                let distance = rltk::DistanceAlg::Pythagoras.distance2d(pos.pt, player_pos);
                if distance < 1.5 {
                    melee_msg
                        .insert(entity, MeleeMessage { target: *player })
                        .expect("Unable to insert melee message.");
                    return;
                }

                let path = rltk::a_star_search(
                    map.point2d_to_index(pos.pt),
                    map.point2d_to_index(player_pos),
                    &mut *map,
                );

                // println!("{}, {}", path.success, path.steps.len());

                if path.success && path.steps.len() > 1 {
                    let mut idx = map.point2d_to_index(pos.pt);
                    map.blocked_tiles[idx as usize] = false;

                    pos.pt.x = path.steps[1] % map.width;
                    pos.pt.y = path.steps[1] / map.width;

                    idx = map.point2d_to_index(pos.pt);
                    map.blocked_tiles[idx as usize] = true;
                    vs.dirty = true;
                }
            }
        }
    }
}
