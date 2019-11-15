use super::components::*;
use super::game_log::*;
use specs::prelude::*;

pub struct ItemManageSyst {}

impl<'a> System<'a> for ItemManageSyst {
    type SystemData = (
        ReadExpect<'a, Entity>,
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, ItemPickupMessage>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, InBackpack>,
    );

    fn run(&mut self,  data: Self::SystemData) {
        let (player, mut log, mut pickup_msg, mut positions, names, mut backpack) = data;

        for pickup in pickup_msg.join() {
            positions.remove(pickup.item);
            backpack.insert(pickup.item, InBackpack {owner: pickup.collected_by}).expect("Unable to add item to backpack.");

            if pickup.collected_by == *player {
                log.add_message(format!("You pick up the {}.", names.get(pickup.item).unwrap().name))
            }
        }

        pickup_msg.clear();
    }
}
