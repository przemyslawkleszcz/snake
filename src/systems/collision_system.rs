use ggez::Context;
use specs::{
    shred::Fetch, shred::FetchMut, storage::MaskedStorage, ReadStorage, Storage, System,
    WriteStorage,
};
use specs::{Entities, Join};

use crate::components::*;

pub struct CollisionSystem<'a> {
    pub context: &'a mut Context,
}

impl<'a> System<'a> for CollisionSystem<'a> {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Immovable>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, players, mut positions, immovables) = data;

        let collision_happened = was_there_a_collision(&players, &positions, &immovables);
        if !collision_happened {
            return;
        }

        reset_player(&players, &entities, &mut positions);
    }
}

fn reset_player(
    players: &Storage<Player, Fetch<MaskedStorage<Player>>>,
    entities: &specs::Read<specs::world::EntitiesRes>,
    positions: &mut Storage<Position, FetchMut<MaskedStorage<Position>>>,
) {
    for (_player, entity) in (players, entities).join() {
        let position = positions.get_mut(entity);
        if let Some(position) = position {
            position.x = 16;
            position.y = 11;
        }
    }
}

fn was_there_a_collision(
    players: &Storage<Player, Fetch<MaskedStorage<Player>>>,
    positions: &Storage<Position, FetchMut<MaskedStorage<Position>>>,
    immovables: &Storage<Immovable, Fetch<MaskedStorage<Immovable>>>,
) -> bool {
    for (_player, player_position) in (players, positions).join() {
        for (_immovable, immovable_position) in (immovables, positions).join() {
            if player_position == immovable_position {
                return true;
            }
        }
    }

    return false;
}
