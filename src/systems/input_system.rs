use ggez::event::KeyCode;
use ggez::Context;
use specs::{Entities, Join};
use specs::{ReadStorage, System, Write, WriteStorage};

use crate::components::*;
use crate::resources::*;

pub struct InputSystem<'a> {
    pub context: &'a mut Context,
}

pub const MOVE_TICK_INTERVAL: u32 = 15;

impl<'a> System<'a> for InputSystem<'a> {
    type SystemData = (
        Write<'a, InputQueue>,
        Entities<'a>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut input_queue, entities, mut positions, players) = data;

        let ticks = ggez::timer::ticks(self.context) as u32;
        if ticks <= input_queue.next_move_at {
            return;
        }

        input_queue.next_move_at = ticks + MOVE_TICK_INTERVAL;

        for (_player, entity) in (&players, &entities).join() {
            if let Some(key) = input_queue.key_pressed {
                let position = positions.get_mut(entity);
                update_position(position, key, &mut input_queue);
                input_queue.checked = CheckedState::Released;
            }
        }
    }
}

fn update_position(
    position: Option<&mut Position>,
    key: KeyCode,
    input_queue: &mut Write<InputQueue>,
) {
    if let Some(position) = position {
        match key {
            KeyCode::Up => {
                if input_queue.previous_key_pressed.is_none()
                    || input_queue.previous_key_pressed.unwrap() != KeyCode::Down
                {
                    position.y -= 1
                } else {
                    input_queue.key_pressed = Some(KeyCode::Down);
                    position.y += 1
                }
            }
            KeyCode::Down => {
                if input_queue.previous_key_pressed.is_none()
                    || input_queue.previous_key_pressed.unwrap() != KeyCode::Up
                {
                    position.y += 1
                } else {
                    input_queue.key_pressed = Some(KeyCode::Up);
                    position.y -= 1
                }
            }
            KeyCode::Left => {
                if input_queue.previous_key_pressed.is_none()
                    || input_queue.previous_key_pressed.unwrap() != KeyCode::Right
                {
                    position.x -= 1
                } else {
                    input_queue.key_pressed = Some(KeyCode::Right);
                    position.x += 1
                }
            }
            KeyCode::Right => {
                if input_queue.previous_key_pressed.is_none()
                    || input_queue.previous_key_pressed.unwrap() != KeyCode::Left
                {
                    position.x += 1
                } else {
                    input_queue.key_pressed = Some(KeyCode::Left);
                    position.x -= 1
                }
            }
            _ => (),
        }
    }
}
