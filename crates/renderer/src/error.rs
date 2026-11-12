use std::fmt;

pub type RendererResult<T> = Result<T, RendererError>;

#[derive(Debug)]
pub enum RendererError {
    InvalidRadius,
    InvalidFramebuffer,
}

impl fmt::Display for RendererError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RendererError::InvalidRadius => write!(f, "Radius should be finite and greater than zero"),
            RendererError::InvalidFramebuffer => write!(f, "Framebuffer isnt large enough to draw this")
        }
    }
}

impl std::error::Error for RendererError {}