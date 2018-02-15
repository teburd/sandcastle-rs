use sandcastle;
use vulkano::device::{Device, DeviceExtensions, Queue};
use vulkano::format::Format;
use vulkano::image::{ImmutableImage, ImageUsage, ImageLayout};
use vulkano::image::Dimensions;
use vulkano::instance;
use vulkano::instance::{Instance, InstanceExtensions, PhysicalDevice};
use vulkano::instance::debug::{DebugCallback, MessageTypes};
use vulkano::swapchain::Surface;

/// `RendererBuilder` builds a Renderer from any number of predefined information given
/// with a bare minimum provided by the constructor
pub struct RendererBuilder {
    instance: Instance,
    surface: Surface,
    physical: Option<PhysicalDevice>,
    swap_chain: Option<SwapChain>,
}

impl RendererBuilder {
    /// Create a RendererBuilder with the absolute requirements of a vulkan instance and surface
    fn new(instance: Instance, surface: Surface) -> RendererBuilder {
        Renderer {
            instance: instance,
            surface: surface,
            physical: None,
            swap_chain: None,
        }
    }

    pub fn set_physical(&mut self, physical: PhysicalDevice) {
       self.physical = Some(physical);
    }

    pub fn set_swap_chain(&mut self, swap_chain: SwapChain) {
        self.swap_chain = Some(swap_chain);
    }

    pub fn build(self) -> Result<Renderer> {
        // if physical does not exist, pick one
        let physical = if let Some(physical) = self.physical {
            physical
        } else {
            self.create_physical()?
        };

        let swap_chain =  if let Some(swap_chain) = self.swap_chain {
            swap_chain
        }  else {
            self.create_swap_chain()?
        }

        let (device, queues) = self.create_device(&physical)?;
        let draw_queue = queues.next().expect("Expected a draw queue");

        Renderer {
            instance: self.instance,
            surface: self.surface,
            physical: physical,
            swap_chain: swap_chain,
            device: device,
            draw_queue: draw_queue,
        }
    }
}

/// A vulkano implementation of a Sandcastle renderer
pub struct Renderer {
    instance: Instance,
    physical: PhysicalDevice,
    device: Device,
    surface: Surface,
    //TODO dimensions: ...
    swap_chain: SwapChain,
    draw_queue: Queue,
    //TODO transfer_queue: ...
    //TODO compute_queue: ...
}

impl Renderer {
    pub fn builder(instance: Instance, surface: Surface) -> RendererBuilder {
        RendererBuilder::new(instance, surface);
    }
}



impl sandcastle::Renderer for Renderer {
}
