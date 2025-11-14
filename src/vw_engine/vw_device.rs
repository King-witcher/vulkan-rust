use crate::vk;
use anyhow::bail;
use std::sync::Arc;

#[derive(Clone)]
pub struct VwDevice {
    vk_physical_device: Arc<vk::PhysicalDevice>,
    vk_surface: Arc<vk::Surface>,
    vk_logical_device: Arc<vk::Device>,
    vk_graphics_queue: Arc<vk::Queue>,
    vk_present_queue: Arc<vk::Queue>,
}

pub struct VwSwapChainSupportDetails {
    pub surface_capabilities: vk::SurfaceCapabilities,
    pub surface_formats: Vec<(vk::Format, vk::ColorSpace)>,
    pub present_modes: Vec<vk::PresentMode>,
}

impl VwDevice {
    pub fn new(vk_instance: Arc<vk::Instance>, surface: Arc<vk::Surface>) -> anyhow::Result<Self> {
        let vk_physical_device = pick_physical_device(vk_instance.clone())?;

        println!(
            "Selected Vulkan physical device: {:?}",
            vk_physical_device.properties().device_name
        );

        let (vk_logical_device, vk_graphics_queue, vk_present_queue) =
            create_logical_device(vk_physical_device.clone(), &surface)?;

        Ok(VwDevice {
            vk_physical_device,
            vk_surface: surface,
            vk_logical_device,
            vk_graphics_queue,
            vk_present_queue,
        })
    }

    pub fn swap_chain_support(&self) -> anyhow::Result<VwSwapChainSupportDetails> {
        let surface_capabilities = self
            .vk_physical_device
            .surface_capabilities(&self.vk_surface, Default::default())?;

        let surface_formats = self
            .vk_physical_device
            .surface_formats(&self.vk_surface, Default::default())?;

        let present_modes = self
            .vk_physical_device
            .surface_present_modes(&self.vk_surface, Default::default())?;

        Ok(VwSwapChainSupportDetails {
            surface_capabilities,
            surface_formats,
            present_modes,
        })
    }

    pub fn logical_device(&self) -> Arc<vk::Device> {
        self.vk_logical_device.clone()
    }

    pub fn surface(&self) -> Arc<vk::Surface> {
        self.vk_surface.clone()
    }

    pub fn graphics_queue(&self) -> Arc<vk::Queue> {
        self.vk_graphics_queue.clone()
    }

    pub fn present_queue(&self) -> Arc<vk::Queue> {
        self.vk_present_queue.clone()
    }
}

const REQUIRED_DEVICE_EXTENSIONS: vk::DeviceExtensions = vk::DeviceExtensions {
    khr_swapchain: true,
    khr_spirv_1_4: false,
    khr_synchronization2: false,
    khr_create_renderpass2: false,
    ..vk::DeviceExtensions::empty()
};

const REQUIRED_FEATURES: vk::DeviceFeatures = vk::DeviceFeatures {
    geometry_shader: true,
    dynamic_rendering: true,
    shader_draw_parameters: true,
    extended_dynamic_state: true,
    ..vk::DeviceFeatures::empty()
};

fn is_device_suitable(device: &Arc<vk::PhysicalDevice>) -> bool {
    let queue_families = device.queue_family_properties();
    let features = device.supported_features();
    let properties = device.properties();
    let extensions = device.supported_extensions();

    if !queue_families
        .iter()
        .any(|qfp| qfp.queue_flags.intersects(vk::QueueFlags::GRAPHICS))
    {
        return false;
    }
    if !features.contains(&REQUIRED_FEATURES) {
        return false;
    }
    if properties.api_version < vk::Version::V1_3 {
        return false;
    }
    if !extensions.contains(&REQUIRED_DEVICE_EXTENSIONS) {
        return false;
    }

    true
}

fn pick_physical_device(instance: Arc<vk::Instance>) -> anyhow::Result<Arc<vk::PhysicalDevice>> {
    let physical_devices = instance.enumerate_physical_devices()?;

    let mut best_score = 0;
    let mut best_device = None;

    for device in physical_devices.filter(is_device_suitable) {
        let mut score = 0;
        let properties = device.properties();

        if properties.device_type == vk::PhysicalDeviceType::DiscreteGpu {
            score += 1000;
        }

        score += properties.max_image_dimension2_d;

        if score > best_score {
            best_score = score;
            best_device = Some(device);
        }
    }

    if let Some(device) = best_device {
        Ok(device)
    } else {
        bail!("No Vulkan physical device found");
    }
}

fn pick_graphics_present_queues(
    physical_device: Arc<vk::PhysicalDevice>,
    surface: &vk::Surface,
) -> (u32, u32) {
    let queue_families = physical_device.queue_family_properties();

    // Find a queue family that supports both graphics and presentation to the given surface
    let graphics_present_index = queue_families.iter().enumerate().find_map(|(index, qfp)| {
        if qfp.queue_flags.intersects(vk::QueueFlags::GRAPHICS)
            && physical_device
                .surface_support(index as u32, surface)
                .unwrap()
        {
            Some(index as u32)
        } else {
            None
        }
    });

    if let Some(index) = graphics_present_index {
        return (index, index);
    }

    let graphics_index = queue_families
        .iter()
        .enumerate()
        .find_map(|(index, qfp)| {
            if qfp.queue_flags.intersects(vk::QueueFlags::GRAPHICS) {
                Some(index as u32)
            } else {
                None
            }
        })
        .expect("Couldn't find queue family that supports graphics");

    let present_index = queue_families
        .iter()
        .enumerate()
        .find_map(|(index, _)| {
            if physical_device
                .surface_support(index as u32, surface)
                .unwrap()
            {
                Some(index as u32)
            } else {
                None
            }
        })
        .expect("Couldn't find queue family that supports presentation to the surface");

    (graphics_index, present_index)
}

fn create_logical_device(
    physical_device: Arc<vk::PhysicalDevice>,
    surface: &vk::Surface,
) -> anyhow::Result<(Arc<vk::Device>, Arc<vk::Queue>, Arc<vk::Queue>)> {
    // List all queue families in the device
    let (graphics_index, present_index) =
        pick_graphics_present_queues(physical_device.clone(), surface);

    let device_queue_create_info = vk::QueueCreateInfo {
        queue_family_index: graphics_index,
        queues: vec![1.0], // Queue priorities
        ..Default::default()
    };

    let device_create_info = vk::DeviceCreateInfo {
        queue_create_infos: vec![device_queue_create_info],
        enabled_extensions: REQUIRED_DEVICE_EXTENSIONS,
        enabled_features: REQUIRED_FEATURES,
        ..Default::default()
    };

    let (device, queues) = vk::Device::new(physical_device, device_create_info)?;
    let queues = queues.collect::<Vec<_>>();

    let graphics_queue = queues
        .iter()
        .find_map(|q| {
            if q.queue_family_index() == graphics_index {
                Some(q.clone())
            } else {
                None
            }
        })
        .expect("Failed to find graphics queue");

    let present_queue = queues
        .iter()
        .find_map(|q| {
            if q.queue_family_index() == present_index {
                Some(q.clone())
            } else {
                None
            }
        })
        .expect("Failed to find present queue");

    Ok((device, graphics_queue, present_queue))
}
