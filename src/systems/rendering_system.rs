use ggez::{
    graphics::{self, Color, DrawParam, Image},
    timer, Context,
};

use glam::Vec2;
use specs::{join::Join, ReadStorage, System};

use crate::components::*;
use crate::constants::*;

pub struct RenderingSystem<'a> {
    pub context: &'a mut Context,
}

impl RenderingSystem<'_> {
    pub fn draw_text(&mut self, text_string: &str, x: f32, y: f32) {
        let text = graphics::Text::new(text_string);
        let destination = Vec2::new(x, y);
        let color = Some(Color::new(0.0, 0.0, 0.0, 1.0));
        let dimensions = Vec2::new(0.0, 20.0);

        graphics::queue_text(self.context, &text, dimensions, color);
        graphics::draw_queued_text(
            self.context,
            graphics::DrawParam::new().dest(destination),
            None,
            graphics::FilterMode::Linear,
        )
        .expect("expected drawing queued text");
    }

    pub fn draw_fps(&mut self) {
        let fps = format!("FPS: {:.0}", timer::fps(self.context));
        self.draw_text(&fps, 5.0, 730.0);
    }
}

impl<'a> System<'a> for RenderingSystem<'a> {
    type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, Renderable>);

    fn run(&mut self, data: Self::SystemData) {
        let (positions, renderables) = data;
        graphics::clear(self.context, graphics::Color::new(0.95, 0.95, 0.95, 1.0));
        let rendering_data = (&positions, &renderables).join().collect::<Vec<_>>();

        for (position, renderable) in rendering_data.iter() {
            let image = Image::new(self.context, renderable.path.clone()).expect("expected image");
            let x = position.x as f32 * TILE_WIDTH;
            let y = position.y as f32 * TILE_WIDTH;

            let draw_params = DrawParam::new().dest(Vec2::new(x, y));
            graphics::draw(self.context, &image, draw_params).expect("expected render");
        }

        self.draw_fps();

        graphics::present(self.context).expect("expected to present");
    }
}
