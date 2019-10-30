use super::components::*;
use super::{GameLog, Map};

use rltk::{Console, Rltk, RGB, Algorithm2D, Point};
use specs::prelude::*;

pub fn draw_ui(ecs: &World, ctx: &mut Rltk) {
    ctx.draw_box(
        0,
        43,
        79,
        6,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
    );

    let player_entity = *ecs.fetch::<Entity>();
    let combat_stats = ecs.read_storage::<CombatStats>();
    let names = ecs.read_storage::<Name>();

    let stats = combat_stats.get(player_entity).unwrap();
    let name = names.get(player_entity).unwrap();
    let health = format!("{} | HP: {} / {} ", name.name, stats.cur_hp, stats.max_hp);
    ctx.print_color(
        1,
        43,
        RGB::named(rltk::YELLOW),
        RGB::named(rltk::BLACK),
        &health,
    );
    ctx.draw_bar_horizontal(
        28,
        43,
        51,
        stats.cur_hp,
        stats.max_hp,
        RGB::named(rltk::RED),
        RGB::named(rltk::BLACK),
    );

    let (m_x, m_y) = ctx.mouse_pos();
    if m_x < 80 && m_y < 43 {
        ctx.set_bg(m_x, m_y, RGB::named(rltk::MAGENTA));
    }

    let log = ecs.fetch::<GameLog>();

    let mut y = 44;
    // for s in log.entries[..5].iter() {
    for s in log.entries.iter() {
        if y < 49 {
            ctx.print(2, y, &s);
        }
        y += 1;
    }
}

pub fn draw_tooltips(ecs: &World, ctx: &mut Rltk) {
    let map = &*ecs.fetch::<Map>();
    let names = ecs.read_storage::<Name>();
    // let positions = ecs.read_storage::<Position>();

    let (m_x, m_y) = ctx.mouse_pos();
    if m_x >= map.width || m_y >= map.height {
        return;
    }

    let mut tooltip : Vec<&String> = Vec::new();
    let idx = map.point2d_to_index(Point::new(m_x, m_y)) as usize;
    for entity in map.tile_content[idx].iter() {
        if let Some(name) = names.get(*entity) {
            tooltip.push(&name.name);
        }
    }

    if tooltip.is_empty() {
        return;
    } else {
        // let width = tooltip.max_by();
    }
}
