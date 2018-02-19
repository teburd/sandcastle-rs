use std::sync::Arc;
use std::fmt::{Debug, Formatter, Error as FmtError};
use sandcastle;
use vulkano;
use vulkano::device::{Device, DeviceExtensions, Queue, QueuesIter};
use vulkano::format::Format;
use vulkano::image::{ImmutableImage, ImageUsage, ImageLayout, SwapchainImage};
use vulkano::image::Dimensions;
use vulkano::instance;
use vulkano::instance::{Instance, InstanceExtensions, PhysicalDevice, PhysicalDeviceType};
use vulkano::instance::debug::{DebugCallback, MessageTypes};
use vulkano::swapchain::Surface;
use vulkano::swapchain::PresentMode;
use vulkano::swapchain::SurfaceTransform;
use vulkano::swapchain::Swapchain;
use vulkano::swapchain::AcquireError;
use vulkano::swapchain::SwapchainCreationError;

/// A vulkano implementation of a Sandcastle renderer
pub struct Renderer {
    instance: Arc<Instance>,
    surface: Arc<Surface>,
    description: (String, PhysicalDeviceType),
    device: Arc<Device>,
    dimensions: [u32; 2],
    swapchain: Arc<Swapchain>,
    images: Vec<Arc<SwapchainImage>>,
    draw_queue: Arc<Queue>,
    //TODO transfer_queue: ...
}

impl Debug for Renderer {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
        let debug = format!("Renderer using device: {} (type: {:?}), dimensions: ({}, {})", self.description.0, self.description.1, self.dimensions[0], self.dimensions[1]);
        fmt.write_str(&debug)
    }
}

impl Renderer {
    pub fn new(instance: Arc<Instance>, surface: Arc<Surface>) -> Renderer {
        // if physical does not exist, pick one
        let physical = PhysicalDevice::enumerate(&instance)
            .next().expect("Vulkano no device available");

        let (device, mut queues) = Renderer::create_device(&physical, &surface);
        let draw_queue = queues.next().expect("Expected a draw queue");
        let (swapchain, images, dimensions) = Renderer::create_swapchain(&physical, &device, &surface, &draw_queue);

        Renderer {
            instance: instance.clone(),
            surface: surface,
            description: (physical.name().clone(), physical.ty().clone()),
            device: device,
            draw_queue: draw_queue,
            swapchain: swapchain,
            images: images,
            dimensions: dimensions,
        }
    }

    fn create_device(physical: &PhysicalDevice, surface: &Arc<Surface>) -> (Arc<Device>, QueuesIter) {
        let queue = physical.queue_families().find(|&q| {
            // We take the first queue that supports drawing to our window.
            q.supports_graphics() && surface.is_supported(q).unwrap_or(false)
        }).expect("couldn't find a graphical queue family");
        let device_ext = vulkano::device::DeviceExtensions {
            khr_swapchain: true,
            .. vulkano::device::DeviceExtensions::none()

        };
        Device::new(*physical, physical.supported_features(), &device_ext, [(queue, 0.5)].iter().cloned()).expect("Vulkano Device Creation")
    }

    fn create_swapchain(physical: &PhysicalDevice, device: &Arc<Device>, surface: &Arc<Surface>, draw_queue: &Arc<Queue>) -> (Arc<Swapchain>, Vec<Arc<SwapchainImage>>, [u32; 2]) {
        let caps = surface.capabilities(*physical)
                         .expect("failed to get surface capabilities");
        let dimensions = caps.current_extent.unwrap_or([1024, 768]);
        let alpha = caps.supported_composite_alpha.iter().next().unwrap();
        let format = caps.supported_formats[0].0;
        let (swapchain, images) = Swapchain::new(device.clone(), surface.clone(), caps.min_image_count, format,
                       dimensions, 1, caps.supported_usage_flags, draw_queue,
                       SurfaceTransform::Identity, alpha, PresentMode::Fifo, true,
                                                 None).expect("failed to create swapchain");
        (swapchain, images, dimensions)
    }
}


impl<'a> sandcastle::Renderer for Renderer {
    fn render(&mut self, node: Box<sandcastle::RenderNode>) -> sandcastle::RenderResult<()> {
       Ok(())
    }
}
