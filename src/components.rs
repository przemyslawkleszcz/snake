use specs::{Component, NullStorage, VecStorage, World, WorldExt};

#[derive(Debug, Component, Clone, Copy, PartialEq)]
#[storage(VecStorage)]
pub struct Position {
    pub x: u8,
    pub y: u8,
    pub z: u8,
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

//markup components

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Player {
    pub items: u32,
}


pub fn register_components(world: &mut World) {
    world.register::<Position>();
    world.register::<Movable>();
    world.register::<Immovable>();
    world.register::<Renderable>();

    //markup components
    world.register::<Player>();
}
