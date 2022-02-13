use std::collections::HashMap;

use ggez::{
    graphics::{self, spritebatch::SpriteBatch, Color, DrawParam, Image},
    timer, Context,
};

use glam::Vec2;
use itertools::Itertools;
use specs::{
    join::Join, shred::FetchMut, storage::MaskedStorage, Entities, Entity, ReadStorage, Storage,
    System, WriteStorage,
};

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

    pub fn draw_player_items(&mut self, player: &Player) {
        let items = format!("Player items: {}", player.items);
        self.draw_text(&items, 80.0, 730.0);
    }

    fn render_batches(&mut self, rendering_batches: HashMap<u8, HashMap<String, Vec<DrawParam>>>) {
        for (_z, group) in rendering_batches
            .iter()
            .sorted_by(|a, b| Ord::cmp(&a.0, &b.0))
        {
            for (image_path, draw_params) in group {
                let image = Image::new(self.context, image_path).expect("expected image");
                let mut sprite_batch = SpriteBatch::new(image);

                for draw_param in draw_params.iter() {
                    sprite_batch.add(*draw_param);
                }

                graphics::draw(self.context, &sprite_batch, graphics::DrawParam::new())
                    .expect("expected render");
            }
        }
    }
}

//need to create utility method
fn get_player_entity(
    players: &Storage<Player, FetchMut<MaskedStorage<Player>>>,
    entities: &specs::Read<specs::world::EntitiesRes>,
) -> Option<Entity> {
    for (_player, entity) in (players, entities).join() {
        return Some(entity);
    }

    return None;
}

impl<'a> System<'a> for RenderingSystem<'a> {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Renderable>,
        WriteStorage<'a, Player>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, positions, renderables, mut players) = data;
        graphics::clear(self.context, graphics::Color::new(0.95, 0.95, 0.95, 1.0));
        let mut rendering_batches: HashMap<u8, HashMap<String, Vec<DrawParam>>> = HashMap::new();
        let rendering_data = (&positions, &renderables).join().collect::<Vec<_>>();

        for (position, renderable) in rendering_data.iter() {
            let image_path = renderable.path.clone();
            let x = position.x as f32 * TILE_WIDTH;
            let y = position.y as f32 * TILE_WIDTH;
            let z = position.z;

            let draw_param = DrawParam::new().dest(Vec2::new(x, y));
            rendering_batches
                .entry(z)
                .or_default()
                .entry(image_path)
                .or_default()
                .push(draw_param);
        }

        if let Some(entity) = get_player_entity(&players, &entities) {
            if let Some(player) = players.get_mut(entity) {
                self.draw_player_items(player);
            }
        }

        self.render_batches(rendering_batches);
        self.draw_fps();

        graphics::present(self.context).expect("expected to present");
    }
}
