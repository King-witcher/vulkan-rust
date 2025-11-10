use crate::{vk, vw_window::VwWindow};
use anyhow::bail;
use std::sync::Arc;

pub struct VwDevice {
    vk_instance: Arc<vk::Instance>,
    vk_physical_device: Arc<vk::PhysicalDevice>,
    vk_logical_device: Arc<vk::Device>,
    vk_graphics_queue: Arc<vk::Queue>,
    vk_present_queue: Arc<vk::Queue>,
}

impl VwDevice {
    pub fn new(vk_instance: Arc<vk::Instance>, surface: vk::Surface) -> anyhow::Result<Self> {
        let vk_physical_device = pick_physical_device(vk_instance.clone())?;
        let (vk_logical_device, vk_graphics_queue, vk_present_queue) =
            create_logical_device(vk_physical_device.clone(), surface)?;

        Ok(VwDevice {
            vk_instance,
            vk_physical_device,
            vk_logical_device,
            vk_graphics_queue,
            vk_present_queue,
        })
    }
}

fn pick_physical_device(instance: Arc<vk::Instance>) -> anyhow::Result<Arc<vk::PhysicalDevice>> {
    let physical_devices = instance.enumerate_physical_devices()?;

    let mut best_score = 0;
    let mut best_device = None;
    for device in physical_devices {
        if !is_device_suitable(&device) {
            continue;
        }

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

const REQUIRED_DEVICE_EXTENSIONS: vk::DeviceExtensions = vk::DeviceExtensions {
    khr_swapchain: true,
    khr_spirv_1_4: true,
    khr_synchronization2: true,
    khr_create_renderpass2: true,
    ..vk::DeviceExtensions::empty()
};

fn is_device_suitable(device: &Arc<vk::PhysicalDevice>) -> bool {
    let features = device.supported_features();
    let properties = device.properties();
    let queue_families = device.queue_family_properties();
    let extensions = device.supported_extensions();

    if !queue_families
        .iter()
        .any(|qfp| qfp.queue_flags.intersects(vk::QueueFlags::GRAPHICS))
    {
        return false;
    }
    if !features.geometry_shader {
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

fn find_queue_family_index(
    physical_device: &Arc<vk::PhysicalDevice>,
    required_flags: vk::QueueFlags,
) -> anyhow::Result<u32> {
    let queue_family_properties = physical_device.queue_family_properties();
    for (index, qfp) in queue_family_properties.iter().enumerate() {
        if qfp.queue_flags.intersects(required_flags) {
            return Ok(index as u32);
        }
    }
    bail!("No suitable queue family found");
}

fn create_logical_device(
    physical_device: Arc<vk::PhysicalDevice>,
    surface: vk::Surface,
) -> anyhow::Result<(Arc<vk::Device>, Arc<vk::Queue>, Arc<vk::Queue>)> {
    let queue_family_properties = physical_device.queue_family_properties();

    let graphics_index = queue_family_properties
        .iter()
        .position(|qfp| qfp.queue_flags.intersects(vk::QueueFlags::GRAPHICS))
        .expect("Couldn't find queue family that supports graphics")
        as u32;

    let present_index = queue_family_properties
        .iter()
        .enumerate()
        .find_map(|(index, _)| {
            if physical_device
                .surface_support(index as u32, &surface)
                .unwrap()
            {
                Some(index as u32)
            } else {
                None
            }
        })
        .expect("Couldn't find queue family that supports presentation")
        as u32;

    let device_queue_create_info = vk::QueueCreateInfo {
        queue_family_index: graphics_index,
        queues: vec![1.0], // Queue priorities
        ..Default::default()
    };

    let device_features = vk::DeviceFeatures {
        dynamic_rendering: true,
        extended_dynamic_state: true,
        ..Default::default()
    };

    let device_create_info = vk::DeviceCreateInfo {
        queue_create_infos: vec![device_queue_create_info],
        enabled_extensions: REQUIRED_DEVICE_EXTENSIONS,
        enabled_features: device_features,
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
