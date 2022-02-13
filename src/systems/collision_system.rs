use ggez::Context;
use rand::Rng;
use specs::{
    shred::Fetch, shred::FetchMut, storage::MaskedStorage, ReadStorage, Storage, System,
    WriteStorage,
};
use specs::{Entities, Entity, Join};

use crate::components::*;

pub struct CollisionSystem<'a> {
    pub context: &'a mut Context,
}

impl<'a> System<'a> for CollisionSystem<'a> {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Immovable>,
        ReadStorage<'a, Item>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut players, mut positions, immovables, items) = data;

        let collision_happened = was_there_a_collision(&players, &positions, &immovables);
        if collision_happened {
            reset_player(&mut players, &entities, &mut positions);
        }

        let item_obtained = was_item_obained(&players, &positions, &items);
        if item_obtained {
            reset_item(&items, &entities, &mut positions);
            assign_item_to_player(&mut players, &entities);
        }
    }
}

fn assign_item_to_player(
    players: &mut Storage<Player, FetchMut<MaskedStorage<Player>>>,
    entities: &specs::Read<specs::world::EntitiesRes>,
) {
    if let Some(entity) = get_player_entity(players, entities) {
        if let Some(player) = players.get_mut(entity) {
            player.items += 1;
        }
    }
}

fn get_player_entity(
    players: &Storage<Player, FetchMut<MaskedStorage<Player>>>,
    entities: &specs::Read<specs::world::EntitiesRes>,
) -> Option<Entity> {
    for (_player, entity) in (players, entities).join() {
        return Some(entity);
    }

    return None;
}

fn reset_item(
    items: &Storage<Item, Fetch<MaskedStorage<Item>>>,
    entities: &specs::Read<specs::world::EntitiesRes>,
    positions: &mut Storage<Position, FetchMut<MaskedStorage<Position>>>,
) {
    let mut rng = rand::thread_rng();
    for (_item, entity) in (items, entities).join() {
        let position = positions.get_mut(entity);
        if let Some(position) = position {
            position.x = rng.gen_range(1..31);
            position.y = rng.gen_range(1..22);
        }
    }
}

fn reset_player(
    players: &mut Storage<Player, FetchMut<MaskedStorage<Player>>>,
    entities: &specs::Read<specs::world::EntitiesRes>,
    positions: &mut Storage<Position, FetchMut<MaskedStorage<Position>>>,
) {
    for (player, entity) in (players, entities).join() {
        let position = positions.get_mut(entity);
        if let Some(position) = position {
            position.x = 16;
            position.y = 11;
            player.items = 0;
        }
    }
}

fn was_there_a_collision(
    players: &Storage<Player, FetchMut<MaskedStorage<Player>>>,
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

fn was_item_obained(
    players: &Storage<Player, FetchMut<MaskedStorage<Player>>>,
    positions: &Storage<Position, FetchMut<MaskedStorage<Position>>>,
    items: &Storage<Item, Fetch<MaskedStorage<Item>>>,
) -> bool {
    for (_player, player_position) in (players, positions).join() {
        for (_immovable, item_position) in (items, positions).join() {
            if player_position == item_position {
                return true;
            }
        }
    }

    return false;
}
