use specs::{Component, NullStorage, VecStorage, World, WorldExt};

#[derive(Debug, Component, Clone, Copy)]
#[storage(VecStorage)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Movable;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Immovable;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Renderable {
    pub path: String,
}

pub fn register_components(world: &mut World) {
    world.register::<Position>();
    world.register::<Movable>();
    world.register::<Immovable>();
    world.register::<Renderable>();
}
