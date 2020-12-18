use ggez::{graphics::{Rect}};
use ggez::nalgebra::Vector2;
use ggez::{Context, GameResult};

use crate::draw_system::DrawSystem;
use crate::drawables::Drawables;
use crate::physics_system::PhysicsSystem;

#[derive(Debug)]
pub struct Entity {
    pub location: Rect,
    extrapolation_active: bool,
    draw_system: Option<Box<dyn DrawSystem>>,
    affected_by_gravity: bool,
    physics_system: Option<Box<dyn PhysicsSystem>>,
}

impl Entity {
    pub fn set_location(mut self, x: f32, y: f32) -> Self {
        self.location.x = x;
        self.location.y = y;

        self
    }

    pub fn set_extrapolation_active(mut self, is_active: bool) -> Self {
        self.extrapolation_active = is_active;
        self
    }

    pub fn set_draw_system(mut self, draw_system: Box<dyn DrawSystem>) -> Self {
        self.draw_system = Some(draw_system);

        self
    }

    pub fn set_affected_by_gravity(mut self) -> Self {
        self.affected_by_gravity = true;
        self
    }

    pub fn set_physics_system(mut self, physics_system: Box<dyn PhysicsSystem>) -> Self {
        self.physics_system = Some(physics_system);
        self
    }

    pub fn get_velocity(&self) -> Vector2<f32>{
        let mut ret_vel = Vector2::new(0.0, 0.0);
        if let Some(physics_system) = &self.physics_system {
            ret_vel.copy_from(physics_system.get_velocity());
        }
        ret_vel
    }
    
    pub fn set_velocity(&mut self, x:f32, y:f32){
        if let Some(physics_system) = &mut self.physics_system {
            physics_system.set_velocity(x, y);
        }        
    }

    pub fn get_extrapolated_position(&self, lag:f32) -> Vector2<f32>{
        let mut ret_position = Vector2::new(self.location.x, self.location.y);
        if let Some(physics_system) = &self.physics_system {
            let part_velocity = physics_system.get_velocity() * lag;
            ret_position += part_velocity;
        }
        ret_position
    }

    pub fn draw(&self, context: &mut Context, drawables: &Drawables, lag: f32) -> GameResult {
        if let Some(draw_system) = &self.draw_system {
            let pos = if self.extrapolation_active{ self.get_extrapolated_position(lag) } else { Vector2::new(self.location.x, self.location.y) };
            draw_system.draw(
                drawables,
                context,
                &pos,
                lag,
                &self.physics_system,
            )?;
        }

        Ok(())
    }

    pub fn update(&mut self, gravity: &Vector2<f32>, dt:f32) {
        if let Some(physics_system) = &mut self.physics_system {
            if self.affected_by_gravity {
                physics_system.apply_force(gravity);
            }

            physics_system.update(&mut self.location, dt);
        }
    }
}

impl Default for Entity {
    fn default() -> Self {
        let location = Rect::new(0.0, 0.0, 0.0, 0.0);
        let extrapolation_active = true;
        let draw_system = None;
        let affected_by_gravity = false;
        let physics_system = None;

        Self {
            location,
            extrapolation_active,
            draw_system,
            affected_by_gravity,
            physics_system,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::draw_system::player_draw_system::PlayerDrawSystem;
    use crate::physics_system::player_physics_system::PlayerPhysicsSystem;

    use super::*;

    #[test]
    fn ci_test_set_location() {
        let mut entity = Entity::default();
        let mut location = Rect::new(0.0, 0.0, 0.0, 0.0);

        assert_eq!(entity.location, location);
        entity = entity.set_location(1.0, 2.0);
        location.x = 1.0;
        location.y = 2.0;
        assert_eq!(entity.location, location);
    }

    #[test]
    fn test_add_player_draw_system() {
        let mut entity = Entity::default();
        assert!(matches!(entity.draw_system, None));
        let player_draw_system = Box::new(PlayerDrawSystem{});
        entity = entity.set_draw_system(player_draw_system);
        assert!(!matches!(entity.draw_system, None));
    }

    #[test]
    fn ci_test_making_entity_affected_by_gravity() {
        let mut entity = Entity::default();
        assert_eq!(entity.affected_by_gravity, false);
        entity = entity.set_affected_by_gravity();
        assert_eq!(entity.affected_by_gravity, true);
    }

    #[test]
    fn ci_test_add_player_physics_system() {
        let mut entity = Entity::default();
        assert!(matches!(entity.physics_system, None));
        entity = entity.set_physics_system(Box::new(PlayerPhysicsSystem::default()));
        assert!(!matches!(entity.physics_system, None));
    }
}
