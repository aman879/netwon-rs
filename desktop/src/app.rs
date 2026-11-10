use std::sync::Arc;

use pixels::{Pixels, SurfaceTexture};
use winit::{
    application::ApplicationHandler, dpi::LogicalSize, event::WindowEvent, window::Window,
};

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
        let center_x = (width / 2) as i32;
        let center_y = (height / 2) as i32;
        let radius: i32 = 90;

        for pixels in frame.chunks_exact_mut(4) {
            pixels.copy_from_slice(&BACKGROUND_COLOR);
        }

        Self::draw_circle(frame, width, height, center_x, center_y, radius);
    }

    fn draw_circle(frame: &mut [u8], width: u32, height: u32, cx: i32, cy: i32, radius: i32) {
        const CIRCLE_COLOR: [u8; 4] = [255, 255, 255, 1];

        let mut x = 0;
        let mut y = radius;
        let mut decision = 1 - radius;

        while x <= y {
            draw_horizontal_line(frame, width, height, cx - x, cx + x, cy + y);
            draw_horizontal_line(frame, width, height, cx - x, cx + x, cy - y);

            draw_horizontal_line(frame, width, height, cx - y, cx + y, cy + x);
            draw_horizontal_line(frame, width, height, cx - y, cx + y, cy - x);

            x += 1;

            if decision < 0 {
                decision += 2 * x + 1;
            } else {
                y -= 1;
                decision += 2 * (x - y) + 1;
            }
        }

        fn draw_horizontal_line(
            frame: &mut [u8],
            width: u32,
            height: u32,
            x_start: i32,
            x_end: i32,
            y: i32,
        ) {
            if y < 0 || y >= height as i32 {
                return;
            }

            let left = x_start.max(0);
            let right = x_end.min(width as i32 - 1);

            if left > right {
                return;
            }

            for x in left..=right {
                let index = ((y as u32 * width + x as u32) * 4) as usize;
                frame[index..index + 4].copy_from_slice(&CIRCLE_COLOR);
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

        let window = Arc::new(
            event_loop
                .create_window(attr)
                .expect("Failed to create window"),
        );

        let physical_size = window.inner_size();
        log::info!(
            "Window created with size {}x{}",
            physical_size.width,
            physical_size.height
        );

        let surface_tex =
            SurfaceTexture::new(physical_size.width, physical_size.height, window.clone());
        let pixels =
            Pixels::new(WIDTH, HEIGHT, surface_tex).expect("Failed to create pixel framebuffer");

        self.window = Some(window);
        self.pixels = Some(pixels);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
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

            _ => (),
        }
    }
}
