use std::path;

use ggez::{event, GameResult};
use specs::{World, WorldExt};

fn main() -> GameResult {
    let mut world = World::new();

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
        Ok(())
    }
}
