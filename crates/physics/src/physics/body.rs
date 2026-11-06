use crate::{
    math::vec2::Vec2,
    physics::error::{PhysicsError, PhysicsResult},
};

#[derive(Debug, Clone, Copy)]
pub struct RigidBody {
    pub position: Vec2,
    pub velocity: Vec2,
    pub accumulated_force: Vec2,
    pub mass: f32,
}

impl RigidBody {
    pub fn new(position: Vec2, mass: f32) -> PhysicsResult<Self> {
        if mass <= 0.0 || !mass.is_finite() {
            return Err(PhysicsError::InvalidMass);
        }
        Ok(Self {
            position,
            velocity: Vec2::zero(),
            accumulated_force: Vec2::zero(),
            mass,
        })
    }

    pub fn apply_force(&mut self, force: Vec2) {
        self.accumulated_force = self.accumulated_force + force;
    }

    pub fn clear_forces(&mut self) {
        self.accumulated_force = Vec2::zero();
    }

    pub fn acceleration(&self) -> Vec2 {
        self.accumulated_force / self.mass
    }

    pub fn integrate(&mut self, dt: f32) -> PhysicsResult<()> {
        if dt <= 0.0 || !dt.is_finite() {
            return Err(PhysicsError::InvalidTimeStep);
        }
        let acceleration = self.acceleration();
        self.velocity = self.velocity + acceleration * dt;
        self.position = self.position + self.velocity * dt;
        self.clear_forces();
        Ok(())
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_rigidbody_creation() {
        let rigid_body = RigidBody::new(Vec2::new(0.0, 2.0), 5.0).unwrap();
        assert_eq!(rigid_body.position, Vec2::new(0.0, 2.0));
        assert_eq!(rigid_body.velocity, Vec2::zero());
        assert_eq!(rigid_body.accumulated_force, Vec2::zero());
        assert_eq!(rigid_body.mass, 5.0);
    }

    #[test]
    fn test_rigidbody_invalid_mass() {
        let result = RigidBody::new(Vec2::new(0.0, 0.0), -1.0);
        assert!(matches!(result, Err(PhysicsError::InvalidMass)));
        
        let result = RigidBody::new(Vec2::new(0.0, 0.0), 0.0);
        assert!(matches!(result, Err(PhysicsError::InvalidMass)));

        let result = RigidBody::new(Vec2::new(0.0, 0.0), f32::INFINITY);
        assert!(matches!(result, Err(PhysicsError::InvalidMass)));

        let result = RigidBody::new(Vec2::new(0.0, 0.0), f32::NAN);
        assert!(matches!(result, Err(PhysicsError::InvalidMass)));
    }

    #[test]
    fn test_apply_force() {
        let mut rigid_body = RigidBody::new(Vec2::new(0.0, 0.0), 5.0).unwrap();
        rigid_body.apply_force(Vec2::new(1.0, 0.0));
        assert_eq!(rigid_body.accumulated_force, Vec2::new(1.0, 0.0));

        rigid_body.apply_force(Vec2::new(-1.0, 5.0));
        assert_eq!(rigid_body.accumulated_force, Vec2::new(0.0, 5.0));
    }

    #[test]
    fn test_clear_forces() {
        let mut rigid_body = RigidBody::new(Vec2::new(0.0, 0.0), 5.0).unwrap();
        rigid_body.apply_force(Vec2::new(1.0, 0.0));
        rigid_body.clear_forces();
        assert_eq!(rigid_body.accumulated_force, Vec2::new(0.0, 0.0));
    }

    #[test]
    fn test_acceleration() {
        let mut rigid_body = RigidBody::new(Vec2::new(0.0, 0.0), 2.0).unwrap();
        rigid_body.apply_force(Vec2::new(10.0, 0.0));
        let acceleration = rigid_body.acceleration();
        assert_eq!(acceleration, Vec2::new(5.0, 0.0));

        rigid_body.clear_forces();

        rigid_body.apply_force(Vec2::new(0.0, 4.0));
        let acceleration = rigid_body.acceleration();
        assert_eq!(acceleration, Vec2::new(0.0, 2.0));

        rigid_body.clear_forces();

        rigid_body.apply_force(Vec2::new(10.0, -10.0));
        let acceleration = rigid_body.acceleration();
        assert_eq!(acceleration, Vec2::new(5.0, -5.0));

        rigid_body.clear_forces();

        rigid_body.apply_force(Vec2::new(0.0, 0.0));
        let acceleration = rigid_body.acceleration();
        assert_eq!(acceleration, Vec2::new(0.0, 0.0));
    }

    #[test]
    fn test_integration() {
        let mut rigid_body = RigidBody::new(Vec2::new(0.0, 0.0), 2.0).unwrap();

        rigid_body.apply_force(Vec2::new(10.0, 0.0));

        assert_eq!(rigid_body.acceleration(), Vec2::new(5.0, 0.0));
        let _ = rigid_body.integrate(0.5);

        assert_eq!(rigid_body.velocity, Vec2::new(2.5, 0.0));
        assert_eq!(rigid_body.position, Vec2::new(1.25, 0.0));
        assert_eq!(rigid_body.accumulated_force, Vec2::new(0.0, 0.0));
    }
}
