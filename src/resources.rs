use ggez::event::KeyCode;
use specs::World;

#[derive(Default)]
pub struct InputQueue {
    pub key_pressed: Option<KeyCode>,
    pub previous_key_pressed: Option<KeyCode>,
    pub next_move_at: u32,
}

pub fn register_resources(world: &mut World) {
    world.insert(InputQueue::default())
}
