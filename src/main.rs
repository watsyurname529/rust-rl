use rltk::{Algorithm2D, Console, GameState, Point, Rltk};
use specs::prelude::*;
#[macro_use]
extern crate specs_derive;

mod components;
pub use components::*;

mod player;
pub use player::*;

mod map;
pub use map::*;

mod visibility_syst;
pub use visibility_syst::*;

mod monster_syst;
pub use monster_syst::*;

mod mapindex_syst;
pub use mapindex_syst::*;

mod combat_syst;
pub use combat_syst::*;

mod damage_syst;
pub use damage_syst::*;

mod gui;
pub use gui::*;

mod game_log;
pub use game_log::*;

mod entity_spawn;
pub use entity_spawn::*;

mod util;
pub use util::*;

const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 43;

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    AwaitingInput,
    PreRun,
    PlayerTurn,
    MonsterTurn,
    Exit,
}

pub struct State {
    pub ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        let mut map = MapIndexingSystem {};
        map.run_now(&self.ecs);
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);
        let mut mon = MonsterAI {};
        mon.run_now(&self.ecs);
        let mut combat = CombatSyst {};
        combat.run_now(&self.ecs);
        let mut damage = DamageSyst {};
        damage.run_now(&self.ecs);
        self.ecs.maintain();
    }

    fn update_map(&mut self) {
        let mut map = MapIndexingSystem {};
        map.run_now(&self.ecs);
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        let mut new_runstate = *self.ecs.fetch::<RunState>();
        match new_runstate {
            RunState::PreRun => {
                self.run_systems();
                new_runstate = RunState::AwaitingInput;
            }
            RunState::AwaitingInput => {
                self.update_map();
                new_runstate = player_input(self, ctx);
            }
            RunState::PlayerTurn => {
                self.run_systems();
                new_runstate = RunState::MonsterTurn;
            }
            RunState::MonsterTurn => {
                self.run_systems();
                new_runstate = RunState::AwaitingInput;
            }
            RunState::Exit => {
                ctx.quit();
            }
        }

        let mut runstate_write = self.ecs.write_resource::<RunState>();
        *runstate_write = new_runstate;
        std::mem::drop(runstate_write);

        if new_runstate != RunState::AwaitingInput {
            DamageSyst::delete_the_dead(&mut self.ecs);
        }

        let map = self.ecs.fetch::<Map>();
        draw_map(&map, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            let idx = map.point2d_to_index(pos.pt) as usize;
            if map.visible_tiles[idx] {
                ctx.set(pos.pt.x, pos.pt.y, render.fg, render.bg, render.glyph);
            }
        }

        draw_ui(&self.ecs, ctx);
    }
}

fn main() {
    let context = Rltk::init_simple8x8(80, 50, "RL", "resources");
    // context.with_post_scanlines(true);

    let mut gs = State { ecs: World::new() };
    gs.ecs.insert(RunState::PreRun);
    gs.ecs.insert(GameLog::new(10));
    gs.ecs.insert(rltk::RandomNumberGenerator::new());

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Name>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<CombatStats>();
    gs.ecs.register::<MeleeMessage>();
    gs.ecs.register::<DamageMessage>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<BlocksTile>();
    gs.ecs.register::<BlocksSight>();
    gs.ecs.register::<Item>();
    gs.ecs.register::<Potion>();

    let map = Map::new_map_rooms(MAP_WIDTH, MAP_HEIGHT);
    let (player_x, player_y) = map.rooms[0].center();
    for room in map.rooms.iter().skip(1) {
        // let (x, y) = room.center();
        // monster(&mut gs.ecs, x, y);
        entity_spawn::populate_room(&mut gs.ecs, room);
    }

    let player_entity = entity_spawn::player(&mut gs.ecs, player_x, player_y);

    gs.ecs.insert(map);
    gs.ecs.insert(player_entity);
    gs.ecs.insert(Point::new(player_x, player_y));
    rltk::main_loop(context, gs);
}
