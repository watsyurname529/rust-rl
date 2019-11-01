use super::components::*;
use super::util::*;
use rltk::{Point, RandomNumberGenerator, RGB};
use specs::prelude::*;

pub fn player(ecs: &mut World, player_x: i32, player_y: i32) -> Entity {
    return ecs
        .create_entity()
        .with(Position {
            pt: Point::new(player_x, player_y),
        })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Name {
            name: "Player".to_string(),
        })
        .with(Player {})
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .with(CombatStats {
            max_hp: 30,
            cur_hp: 30,
            atk: 5,
            def: 3,
        })
        .build();
}

pub fn monster(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position {
            pt: Point::new(x, y),
        })
        .with(Renderable {
            glyph: rltk::to_cp437('g'),
            fg: RGB::named(rltk::RED),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .with(Name {
            name: "Goblin".to_string(),
        })
        .with(Monster {})
        .with(CombatStats {
            max_hp: 10,
            cur_hp: 10,
            atk: 4,
            def: 1,
        })
        .with(BlocksTile {})
        .with(BlocksSight {})
        .build();
}

pub fn potion(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position {
            pt: Point::new(x, y),
        })
        .with(Renderable {
            glyph: rltk::to_cp437('!'),
            fg: RGB::named(rltk::MAGENTA),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Name {
            name: "Health Potion".to_string(),
        })
        .with(Item {})
        .with(Potion {})
        .build();
}

pub fn populate_room(ecs: &mut World, room: &Rect) {
    let mut spawn_points_mob: Vec<Point> = Vec::new();
    let mut spawn_points_item: Vec<Point> = Vec::new();

    let mut rng = ecs.write_resource::<RandomNumberGenerator>();
    let num_mob = rng.range(0, 5);
    let num_item = rng.range(0, 2);

    for _i in 0..num_mob {
        let mut ok = false;
        while !ok {
            let x = rng.range(0, room.x2 - room.x1);
            let y = rng.range(0, room.y2 - room.y1);
            let pt = Point::new(room.x1 + x, room.y1 + y);

            if !spawn_points_mob.contains(&pt) {
                spawn_points_mob.push(pt);
                ok = true;
            }
        }
    }

    for _i in 0..num_item {
        let mut ok = false;
        while !ok {
            let x = rng.range(0, room.x2 - room.x1);
            let y = rng.range(0, room.y2 - room.y1);
            let pt = Point::new(room.x1 + x, room.y1 + y);

            if !spawn_points_item.contains(&pt) {
                spawn_points_item.push(pt);
                ok = true;
            }
        }
    }
    std::mem::drop(rng);

    for pt in spawn_points_mob.iter() {
        monster(ecs, pt.x, pt.y);
    }
    for pt in spawn_points_item.iter() {
        potion(ecs, pt.x, pt.y);
    }
}
