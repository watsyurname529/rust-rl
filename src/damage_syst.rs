use super::components::*;
use super::GameLog;
use specs::prelude::*;

pub struct DamageSyst {}

impl DamageSyst {
    pub fn delete_the_dead(ecs: &mut World) {
        let mut dead_vec: Vec<Entity> = Vec::new();

        let mut log = ecs.write_resource::<GameLog>();
        let combat_stats = ecs.read_storage::<CombatStats>();
        let names = ecs.read_storage::<Name>();
        let players = ecs.read_storage::<Player>();
        let entities = ecs.entities();
        for (entity, stats) in (&entities, &combat_stats).join() {
            if stats.cur_hp < 1 {
                let player = players.get(entity);
                match player {
                    None => {
                        let victim_name = names.get(entity).unwrap();
                        log.add_message(format!("{} is dead!", &victim_name.name));
                        dead_vec.push(entity);
                    }
                    Some(_) => {
                        log.add_message(format!("You are dead!"));
                    }
                }
            }
        }
        std::mem::drop(entities);
        std::mem::drop(players);
        std::mem::drop(combat_stats);
        std::mem::drop(names);
        std::mem::drop(log);

        for victim in dead_vec {
            ecs.delete_entity(victim).expect("Unable to delete");
        }
    }
}

impl<'a> System<'a> for DamageSyst {
    type SystemData = (
        WriteStorage<'a, CombatStats>,
        WriteStorage<'a, DamageMessage>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut combat_stats, mut damage_msg) = data;

        for (mut stats, damage) in (&mut combat_stats, &damage_msg).join() {
            stats.cur_hp -= damage.val;
        }

        damage_msg.clear();
    }
}
