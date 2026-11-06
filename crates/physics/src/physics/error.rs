use std::fmt;

pub type PhysicsResult<T> = Result<T, PhysicsError>;

#[derive(Debug)]
pub enum PhysicsError {
    InvalidMass,
    InvalidTimeStep,
}

impl fmt::Display for PhysicsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PhysicsError::InvalidMass => write!(f, "Mass should be finite and greater than zero"),
            PhysicsError::InvalidTimeStep => write!(f, "Time step should be finite and greater than zero"),
        }
    }
}

impl std::error::Error for PhysicsError {}