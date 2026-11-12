use std::sync::Arc;

use physics::math::vec2::Vec2;
use pixels::{Pixels, SurfaceTexture};
use renderer::{
    renderer::Renderer,
    scene::{RenderObject, Scene},
    shapes::circle::Circle,
};
use winit::{
    application::ApplicationHandler, dpi::LogicalSize, event::WindowEvent, window::Window,
};

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

pub struct App {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
    renderer: Renderer,
    scene: Scene,
}

impl App {
    pub fn new() -> Self {
        let mut scene = Scene::new();

        scene.add_object(RenderObject {
            position: Vec2::new(200.0, 150.0),
            shape: Box::new(Circle::new(50.0).expect("hard-coded radius is valid")),
        });

        scene.add_object(RenderObject {
            position: Vec2::new(400.0, 250.0),
            shape: Box::new(Circle::new(30.0).expect("hard-coded radius is valid")),
        });

        Self {
            window: None,
            pixels: None,
            renderer: Renderer,
            scene,
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
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
                if size.width > 0
                    && size.height > 0
                    && let Some(window) = &self.window
                    && let Some(pixels) = &mut self.pixels
                {
                    let logical = size.to_logical::<u32>(window.scale_factor());
                    if let Err(err) = pixels.resize_surface(size.width, size.height) {
                        log::error!("Pixels resize error: {}", err);
                        event_loop.exit();
                        return;
                    }
                    if let Err(err) = pixels.resize_buffer(logical.width, logical.height) {
                        log::error!("Pixels resize error: {}", err);
                        event_loop.exit();
                        return;
                    }

                    window.request_redraw();
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(pixels) = &mut self.pixels {
                    let width = pixels.texture().width();
                    let height = pixels.texture().height();

                    if let Err(err) =
                        self.renderer
                            .render(pixels.frame_mut(), width, height, &self.scene)
                    {
                        log::error!("Rendering Error: {}", err);
                        event_loop.exit();
                        return;
                    }

                    if let Some(window) = &self.window {
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
