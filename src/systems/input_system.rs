use specs::{System, Write};

use crate::resources::*;

pub struct InputSystem {}

impl<'a> System<'a> for InputSystem {
    type SystemData = Write<'a, InputQueue>;

    fn run(&mut self, data: Self::SystemData) {
        let mut input_queue = data;
        if let Some(key) = input_queue.keys_pressed.pop() {
            println!("Steps in the queue: {:?}", key);
        }
    }
}
