use std::fmt::Debug;

use ggez::graphics::{Rect};
use ggez::nalgebra::Vector2;
use ggez::{Context, GameResult};

use crate::drawables::Drawables;
use crate::physics_system::PhysicsSystem;

pub mod player_draw_system;

pub trait DrawSystem
where
    Self: Debug,
{
    fn draw(
        &self,
        drawables: &Drawables,
        context: &mut Context,
        position: &Vector2<f32>,
        lag: f32,
        physics_system: &Option<Box<dyn PhysicsSystem>>,
    ) -> GameResult;
}
