use crate::{math::vec2::Vec2, physics::{body::RigidBody, error::PhysicsResult}};

#[derive(Debug)]
pub struct PhysicsWorld {
    pub gravity: Vec2,
    pub bodies: Vec<RigidBody>,
}

impl PhysicsWorld {
    pub fn new() -> Self {
        Self {
            gravity: Vec2::new(0.0, 9.8),
            bodies: Vec::new(),
        }
    }

    pub fn add_body(&mut self, body: RigidBody) {
        self.bodies.push(body);
    }

    pub fn step(&mut self, dt: f32 /* time step in seconds */) -> PhysicsResult<()>{
        for b in self.bodies.iter_mut() {
            b.apply_force(self.gravity * b.mass);
            b.integrate(dt)?;
        }
        Ok(())
    }
}

impl Default for PhysicsWorld {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {


    use super::*;

    #[test]
    fn test_physics_world_step() {
        let mut world = PhysicsWorld::new();

        let body1 = RigidBody::new(Vec2::zero(), 2.0).unwrap();
        let body2 = RigidBody::new(Vec2::zero(), 5.0).unwrap();

        world.add_body(body1);
        world.add_body(body2);

        world.step(1.0).unwrap();

        // Body 1
        assert_eq!(world.bodies[0].velocity, Vec2::new(0.0, 9.8));
        assert_eq!(world.bodies[0].position, Vec2::new(0.0, 9.8));
        assert_eq!(world.bodies[0].accumulated_force, Vec2::zero());

        // Body 2
        assert_eq!(world.bodies[1].velocity, Vec2::new(0.0, 9.8));
        assert_eq!(world.bodies[1].position, Vec2::new(0.0, 9.8));
        assert_eq!(world.bodies[1].accumulated_force, Vec2::zero());
    }

    #[test]
    fn test_multiple_physics_steps() {
        let mut world = PhysicsWorld::new();

        let body = RigidBody::new(Vec2::zero(), 2.0).unwrap();
        world.add_body(body);

        world.step(1.0).unwrap();
        assert_eq!(world.bodies[0].velocity, Vec2::new(0.0, 9.8));
        assert_eq!(world.bodies[0].position, Vec2::new(0.0, 9.8));
        assert_eq!(world.bodies[0].accumulated_force, Vec2::zero());

        world.step(1.0).unwrap();
        assert_eq!(world.bodies[0].velocity, Vec2::new(0.0, 19.6));
        assert!(approx_eq(world.bodies[0].position.x, 0.0));
        assert!(approx_eq(world.bodies[0].position.y, 29.4));
        assert_eq!(world.bodies[0].accumulated_force, Vec2::zero());
    }


    fn approx_eq(a: f32, b: f32) -> bool {
        (a - b).abs() < 0.0001
    }
}