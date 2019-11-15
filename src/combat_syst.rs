use super::components::*;
use super::GameLog;
use specs::prelude::*;
use std::cmp::max;

pub struct CombatSyst {}

impl<'a> System<'a> for CombatSyst {
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, MeleeMessage>,
        WriteStorage<'a, DamageMessage>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, CombatStats>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut log, mut melee_msg, mut damage_msg, names, combat_stats) = data;

        for (_entity, melee, name, stats) in (&entities, &melee_msg, &names, &combat_stats).join() {
            if stats.cur_hp > 0 {
                let target_stats = combat_stats.get(melee.target).unwrap();
                if target_stats.cur_hp > 0 {
                    let target_name = names.get(melee.target).unwrap();
                    let damage = max(0, stats.atk - target_stats.def);

                    if damage == 0 {
                        log.add_message(format!(
                            "{} is unable to hurt {}.",
                            &name.name, &target_name.name
                        ));
                    } else {
                        log.add_message(format!(
                            "{} deals {} hp damage to {}.",
                            &name.name, damage, &target_name.name
                        ));

                        let temp = damage_msg.get_mut(melee.target);
                        match temp {
                            None => {
                                damage_msg
                                    .insert(melee.target, DamageMessage { val: damage })
                                    .expect("Unable to insert damage message.");
                            }

                            Some(msg) => {
                                msg.val += damage;
                            }
                        };
                    }
                }
            }
        }
        melee_msg.clear();
    }
}
