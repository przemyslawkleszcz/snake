use std::path;

use ggez::{event, GameResult};
use specs::RunNow;
use specs::{World, WorldExt};

mod components;
mod constants;
mod entities;
mod systems;

use crate::components::*;
use crate::constants::*;
use crate::entities::*;
use crate::systems::*;

fn main() -> GameResult {
    let mut world = World::new();
    register_components(&mut world);
    create_player(&mut world, Position { x: 0, y: 0, z: 0 });
    create_wall(&mut world, Position { x: 1, y: 0, z: 0 });

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
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        {
            let mut rs = RenderingSystem { context: _ctx };
            rs.run_now(&self.world);
        }

        Ok(())
    }
}
