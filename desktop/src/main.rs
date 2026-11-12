use winit::event_loop::{ControlFlow, EventLoop};

use crate::app::App;

pub mod app;

pub fn main() { 
    env_logger::init();

    let event_loop = EventLoop::new().expect("create event loop");
    event_loop.set_control_flow(ControlFlow::Wait);
    log::info!("Starting application");
    event_loop.run_app(&mut App::new()).expect("run app");  
    log::info!("Exiting application");
}