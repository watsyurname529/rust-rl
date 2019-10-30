use super::{BlocksTile, Map, Position};
use rltk::Algorithm2D;
use specs::prelude::*;

pub struct MapIndexingSystem {}

impl<'a> System<'a> for MapIndexingSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, BlocksTile>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, position, blockers, entities) = data;

        map.populate_blocked();
        map.clear_content_index();
        for (pos, ent) in (&position, &entities).join() {
            let idx = map.point2d_to_index(pos.pt) as usize;

            let _p: Option<&BlocksTile> = blockers.get(ent);
            if let Some(_p) = _p {
                map.blocked_tiles[idx] = true;
            }

            map.tile_content[idx].push(ent);
        }
    }
}
