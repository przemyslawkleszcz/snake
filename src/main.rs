use std::path;

use entities::create_tail;
use ggez::{event, GameResult};
use specs::RunNow;
use specs::{World, WorldExt};

mod components;
mod constants;
mod entities;
mod map;
mod resources;
mod systems;

use crate::components::*;
use crate::map::*;
use crate::resources::*;
use crate::systems::*;

fn main() -> GameResult {
    let mut world = World::new();
    register_components(&mut world);
    register_resources(&mut world);
    initialize_level(&mut world);

    let context_builder = ggez::ContextBuilder::new("Snake", "Przemyslaw Kleszcz")
        .window_setup(ggez::conf::WindowSetup::default().title("Snake"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(1024.0, 768.0))
        .add_resource_path(path::PathBuf::from("./resources"));

    let (context, event_loop) = context_builder.build()?;
    let game = Game { world };
    event::run(context, event_loop, game)
}

struct Game {
    world: World,
}

impl event::EventHandler<ggez::GameError> for Game {
    fn update(&mut self, _ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        {
            let mut is = InputSystem { context: _ctx };
            is.run_now(&self.world);
        }

        {
            let mut cs = CollisionSystem { context: _ctx };
            cs.run_now(&self.world);
        }

        let mut item_added = false;
        {
            let mut player_info = self.world.write_resource::<PlayerInfo>();
            if player_info.previous_state < player_info.items {
                player_info.previous_state += 1;
                item_added = true;
            }
        }

        if item_added {
            let position = Position {
                x: 5 as u8,
                y: 5 as u8,
                z: 0,
            };

            create_tail(&mut self.world, position);
        }

        Ok(())
    }

    fn draw(&mut self, _ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        {
            let mut rs = RenderingSystem { context: _ctx };
            rs.run_now(&self.world);
        }

        Ok(())
    }

    fn key_down_event(
        &mut self,
        _context: &mut ggez::Context,
        keycode: ggez::event::KeyCode,
        _keymod: ggez::event::KeyMods,
        _repeat: bool,
    ) {
        let mut input_queue = self.world.write_resource::<InputQueue>();
        if input_queue.checked == CheckedState::Started {
            assign_key_pressed(&mut input_queue, keycode);
        } else if input_queue.checked == CheckedState::Released {
            if input_queue.key_pressed != Some(keycode) {
                assign_key_pressed(&mut input_queue, keycode);
            } else {
                input_queue.checked = CheckedState::Blocked;
            }
        }
    }
}

fn assign_key_pressed(
    input_queue: &mut specs::shred::FetchMut<InputQueue>,
    keycode: event::KeyCode,
) {
    input_queue.previous_key_pressed = input_queue.key_pressed;
    input_queue.key_pressed = Some(keycode);
    input_queue.checked = CheckedState::Blocked;
}
