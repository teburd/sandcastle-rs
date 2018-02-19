extern crate sandcastle;
extern crate sandcastle_vulkano;

// The `vulkano` crate is the main crate that you must use to use Vulkan.
#[macro_use]
extern crate vulkano;
// The `vulkano_shader_derive` crate allows us to use the `VulkanoShader` custom derive that we use
// in this example.
#[macro_use]
extern crate vulkano_shader_derive;
// However the Vulkan library doesn't provide any functionality to create and handle windows, as
// this would be out of scope. In order to open a window, we are going to use the `winit` crate.
extern crate winit;
// The `vulkano_win` crate is the link between `vulkano` and `winit`. Vulkano doesn't know about
// winit, and winit doesn't know about vulkano, so import a crate that will provide a link between
// the two.
extern crate vulkano_win;

use std::sync::Arc;
use vulkano_win::VkSurfaceBuild;

fn main() {
    let extensions = vulkano_win::required_extensions();
    let instance = vulkano::instance::Instance::new(None, &extensions, None).expect("failed to create instance");


    let mut events_loop = winit::EventsLoop::new();
    let window = winit::WindowBuilder::new().build_vk_surface(&events_loop, instance.clone()).unwrap();
    let surface = window.surface();
    let renderer = sandcastle_vulkano::Renderer::new(instance.clone(), surface.clone());

    println!("Renderer: {:?}", renderer);
    loop {
        let mut recreate_swapchain = false;
        let mut done = false;
        events_loop.poll_events(|ev| {
            match ev {
                winit::Event::WindowEvent { event: winit::WindowEvent::Closed, .. } => done = true,
                winit::Event::WindowEvent { event: winit::WindowEvent::Resized(_, _), .. } => recreate_swapchain = true,
                _ => ()
            }
        });
        if done { return; }
    }
}
