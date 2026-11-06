use std::sync::Arc;

use pixels::{Pixels, SurfaceTexture};
use winit::{application::ApplicationHandler, dpi::LogicalSize, event::WindowEvent, window::Window};

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

const BACKGROUND_COLOR: [u8; 4] = [255, 255, 255, 255];


#[derive(Default)]
pub struct App {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
}

impl App {
    fn draw(frame: &mut [u8], width: u32, height: u32) {
        const STRIP_COLOR: [u8; 4] = [255, 255, 255, 1];

        let center_x = (width / 2) as i32;
        let center_y = (height / 2) as i32;
        let radius: i32 = 90;
        let radius_sq = radius * radius;

        let min_y = (center_y - radius).max(0) as u32;
        let max_y = (center_y + radius).min(height as i32 - 1) as u32;

        for pixels in frame.chunks_exact_mut(4) {
            pixels.copy_from_slice(&BACKGROUND_COLOR);
            
        }

        for y in min_y..=max_y {

            let dy = y as i32 - center_y;
            let dy_squred = dy * dy;
            
            let horizontal_radius = ((radius_sq - dy_squred) as f32).sqrt() as i32;

            let left_x = (center_x - horizontal_radius).max(0);
            let right_x = (center_x + horizontal_radius).min(width as i32 - 1);

            for x in left_x..=right_x {
                let index = ((y * width + x as u32) * 4) as usize;
                frame[index..index + 4].copy_from_slice(&STRIP_COLOR);
            }
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        let attr = Window::default_attributes()
            .with_title("Newton-rs | 2D Physics Simulator")
            .with_inner_size(LogicalSize::new(WIDTH, HEIGHT));

        let window = Arc::new(event_loop.create_window(attr).expect("Failed to create window"));

        let physical_size = window.inner_size();
        log::info!(
            "Window created with size {}x{}",
            physical_size.width,
            physical_size.height
        );

        let surface_tex = SurfaceTexture::new(physical_size.width, physical_size.height, window.clone());
        let pixels = Pixels::new(WIDTH, HEIGHT, surface_tex).expect("Failed to create pixel framebuffer");

        self.window = Some(window);
        self.pixels = Some(pixels);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    )
    {
        match event {
            WindowEvent::CloseRequested => {

                self.window = None;
                self.pixels = None;
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                if size.width > 0 && size.height > 0 {
                    if let Some(pixels) = &mut self.pixels {
                        if let Err(err) = pixels.resize_surface(size.width, size.height) {
                            log::error!("Pixels resize error: {}", err);
                            event_loop.exit();
                            return;
                        }
                        if let Err(err) = pixels.resize_buffer(size.width, size.height) {
                            log::error!("Pixels resize error: {}", err);
                            event_loop.exit();
                            return;
                        }
                    } 

                    if let Some(window) = &self.window {
                        window.request_redraw();
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(pixels) = &mut self.pixels {
                    

                    if let Some(window) = &self.window {
                        let width = pixels.texture().width();
                        let height = pixels.texture().height();
                        Self::draw(pixels.frame_mut(), width, height);
                        window.pre_present_notify();
                    }

                    if let Err(err) = pixels.render() {
                        log::error!("Pixels render error: {}", err);
                        event_loop.exit();
                    }
                }
            }

            _ => ()
        }
    }
}