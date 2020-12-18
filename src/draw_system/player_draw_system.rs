use ggez::graphics::{draw, DrawParam, Rect};
use ggez::{Context, GameResult};
use ggez::nalgebra::Vector2;

use crate::drawables::Drawables;
use crate::physics_system::PhysicsSystem;

use super::DrawSystem;

#[derive(Debug)]
pub struct PlayerDrawSystem;

impl DrawSystem for PlayerDrawSystem {
    fn draw(
        &self,
        drawables: &Drawables,
        context: &mut Context,
        position: &Vector2<f32>,
        lag: f32,
        physics_system: &Option<Box<dyn PhysicsSystem>>,
    ) -> GameResult {
        
        draw(
            context,
            &drawables.player,
            DrawParam::default().dest([position[0], position[1]]),
        )
    }
}
