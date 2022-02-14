use ggez::event::KeyCode;
use specs::World;

#[derive(PartialEq)]
pub enum CheckedState {
    Started,
    Blocked,
    Released,
}

impl Default for CheckedState {
    fn default() -> Self {
        CheckedState::Started
    }
}

#[derive(Default)]
pub struct InputQueue {
    pub key_pressed: Option<KeyCode>,
    pub previous_key_pressed: Option<KeyCode>,
    pub checked: CheckedState,
    pub next_move_at: u32,
}

#[derive(Default)]
pub struct PlayerInfo {
    pub previous_state: u32,
    pub items: u32,
}

pub fn register_resources(world: &mut World) {
    world.insert(InputQueue::default());
    world.insert(PlayerInfo::default());
}
