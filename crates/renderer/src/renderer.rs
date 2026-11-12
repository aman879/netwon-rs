use crate::{error::RendererResult, frame::Frame, scene::Scene,};

const BACKGROUND_COLOR: [u8; 4] = [255, 255, 255, 255];

pub struct Renderer;

impl Renderer {

    pub fn new() -> Self {
        Self
    }

    pub fn render(
        &mut self,
        frame: &mut [u8],
        width: u32,
        height: u32,
        scene: &Scene
    ) -> RendererResult<()> {

        let mut frame = Frame::new(frame, width, height)?;

        frame.clear(BACKGROUND_COLOR);

        for object in &scene.objects {
            object.shape.draw(
                &mut frame,
                object.position.x.round() as i64, 
                object.position.y.round() as i64,
            )?;
        }

        Ok(())
    }
}

impl Default for Renderer {
    fn default() -> Self {
        Self::new()
    }
}