use crate::{error::RendererResult, frame::Frame};


pub trait Drawable {
    fn draw(
        &self,
        frame: &mut Frame,
        cx: i64,
        cy: i64,
    ) -> RendererResult<()>;
}