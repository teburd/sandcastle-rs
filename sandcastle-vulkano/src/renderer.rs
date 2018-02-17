use sandcastle;
use vulkano;
use vulkano::device::{Device, DeviceExtensions, Queue, QueuesIter};
use vulkano::format::Format;
use vulkano::image::{ImmutableImage, ImageUsage, ImageLayout};
use vulkano::image::Dimensions;
use vulkano::instance;
use vulkano::instance::{Instance, InstanceExtensions, PhysicalDevice};
use vulkano::instance::debug::{DebugCallback, MessageTypes};
use vulkano::swapchain::Surface;
use vulkano::swapchain::PresentMode;
use vulkano::swapchain::SurfaceTransform;
use vulkano::swapchain::Swapchain;
use vulkano::swapchain::AcquireError;
use vulkano::swapchain::SwapchainCreationError;


/// `RendererBuilder` builds a Renderer from any number of predefined information given
/// with a bare minimum provided by the constructor
pub struct RendererBuilder<'a> {
    instance: Instance,
    surface: Surface,
    physical: Option<PhysicalDevice<'a>>,
    swapchain: Option<Swapchain>,
}

impl<'a> RendererBuilder<'a> {
    /// Create a RendererBuilder with the absolute requirements of a vulkan instance and surface
    fn new(instance: Instance, surface: Surface) -> RendererBuilder<'a> {
        Renderer {
            instance: instance,
            surface: surface,
            physical: None,
            swapchain: None,
        }
    }

    pub fn set_physical(&mut self, physical: PhysicalDevice) {
       self.physical = Some(physical);
    }

    pub fn set_swapchain(&mut self, swapchain: Swapchain) {
        self.swapchain = Some(swapchain);
    }

    pub fn build(self) -> Renderer<'a> {
        // if physical does not exist, pick one
        let physical = if let Some(physical) = self.physical {
            physical
        } else {
            self.create_physical(&self.instance)
        };

        let (device, queues) = self.create_device(&physical, &self.surface);
        let draw_queue = queues.next().expect("Expected a draw queue");

        let swapchain =  if let Some(swapchain) = self.swapchain {
            swapchain
        }  else {
            self.create_swapchain(&physical, &device, &self.surface)
        };

        Renderer {
            instance: self.instance,
            surface: self.surface,
            physical: physical,
            swapchain: swapchain,
            device: device,
            draw_queue: queues.next().expect("Draw Queue"),
        }
    }

    fn create_physical(&self, instance: &Instance) -> PhysicalDevice {
        PhysicalDevice::enumerate(&instance)
            .next().expect("Vulkano no device available");
    }

    fn create_device(&self, physical: &PhysicalDevice, surface: &Surface) -> (Device, QueuesIter) {
        let queue = physical.queue_families().find(|&q| {
            // We take the first queue that supports drawing to our window.
            q.supports_graphics() && surface.is_supported(q).unwrap_or(false)
        }).expect("couldn't find a graphical queue family");
        let device_ext = vulkano::device::DeviceExtensions {
            khr_swapchain: true,
            .. vulkano::device::DeviceExtensions::none()

        };
        Device::new(physical, physical.supported_features(), &device_ext, [(queue, 0.5)].iter().cloned()).expect("Vulkano Device Creation")
    }

    fn create_swapchain(&self, physical: &PhysicalDevice, device: &Device, surface: &Surface, draw_queue: &Queue) -> Swapchain {
        let caps = surface.capabilities(physical)
                         .expect("failed to get surface capabilities");
        let dimensions = caps.current_extent.unwrap_or([1024, 768]);
        let alpha = caps.supported_composite_alpha.iter().next().unwrap();
        let format = caps.supported_formats[0].0;
        Swapchain::new(device.clone(), surface.clone(), caps.min_image_count, format,
                       dimensions, 1, caps.supported_usage_flags, draw_queue,
                       SurfaceTransform::Identity, alpha, PresentMode::Fifo, true,
                       None).expect("failed to create swapchain")
    }
}

/// A vulkano implementation of a Sandcastle renderer
pub struct Renderer<'a> {
    instance: Instance,
    physical: PhysicalDevice<'a>,
    device: Device,
    surface: Surface,
    //TODO dimensions: ...
    swapchain: Swapchain,
    draw_queue: Queue,
    //TODO transfer_queue: ...
    //TODO compute_queue: ...
}

impl<'a> Renderer<'a> {
    pub fn builder(instance: Instance, surface: Surface) -> RendererBuilder<'a> {
        RendererBuilder::new(instance, surface)
    }
}



impl<'a> sandcastle::Renderer for Renderer<'a> {
    fn render(&mut self, node: Box<sandcastle::RenderNode>) -> sandcastle::RenderResult<()> {
       Ok(())
    }
}
