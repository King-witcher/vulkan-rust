use std::{sync::Arc, vec};

use anyhow::bail;
use sdl::{event::Event, keyboard::Scancode};
use vulkano::device::{Device, DeviceFeatures};

use crate::{
    vk,
    vw_window::{VwWindow, VwWindowCreateInfo},
};

pub struct VkWizardEngine {
    vk_library: Arc<vk::VulkanLibrary>,
    vk_instance: Arc<vk::Instance>,
    vk_physical_device: Arc<vk::PhysicalDevice>,
    vk_logical_device: Arc<vk::Device>,
    vw_window: VwWindow,
}

impl VkWizardEngine {
    pub fn new() -> anyhow::Result<Self> {
        let vk_library = vk::VulkanLibrary::new()?;
        let vk_instance = create_vulkan_instance(vk_library.clone())?;
        let vk_physical_device = pick_physical_device(vk_instance.clone())?;
        let vk_logical_device = create_logical_device(vk_physical_device.clone())?;

        let vw_window = VwWindow::new(VwWindowCreateInfo {
            title: "VkWizard Window",
            position: (-1400, 100),
            ..Default::default()
        });

        Ok(VkWizardEngine {
            vk_library,
            vk_instance,
            vk_physical_device,
            vk_logical_device,
            vw_window,
        })
    }

    pub fn run(&self) {
        let mut event_pump = self.vw_window.event_pump();
        loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => return,
                    Event::KeyDown {
                        scancode: Some(Scancode::Escape),
                        ..
                    } => return,
                    _ => {}
                }
            }
        }
    }
}

fn create_vulkan_instance(vk_lib: Arc<vk::VulkanLibrary>) -> anyhow::Result<Arc<vk::Instance>> {
    let supported_extensions = vk_lib.supported_extensions();
    if !supported_extensions.khr_surface {
        list_supported_extensions(&vk_lib);
        return Err(anyhow::anyhow!(
            "Required extension khr_surface is not supported"
        ));
    }
    if !supported_extensions.khr_win32_surface {
        list_supported_extensions(&vk_lib);
        return Err(anyhow::anyhow!(
            "Required extension khr_win32_surface is not supported"
        ));
    }

    let enabled_extensions = vk::InstanceExtensions {
        khr_surface: true,
        khr_win32_surface: true,
        ..vk::InstanceExtensions::empty()
    };

    let mut enabled_layers: Vec<String> = vec![];
    if cfg!(debug_assertions) {
        enabled_layers.push("VK_LAYER_KHRONOS_validation".into());
    }

    let instance_create_info = vk::InstanceCreateInfo {
        flags: vk::InstanceCreateFlags::empty(),
        application_name: Some("VkWizard Game".into()),
        application_version: vulkano::Version {
            major: 1,
            minor: 0,
            patch: 0,
        },
        engine_name: Some("VkWizard Engine".into()),
        engine_version: vulkano::Version {
            major: 1,
            minor: 0,
            patch: 0,
        },
        max_api_version: Some(vulkano::Version::V1_4),
        enabled_layers,
        enabled_extensions,
        debug_utils_messengers: vec![],
        enabled_validation_features: vec![],
        disabled_validation_features: vec![],

        ..Default::default()
    };

    if cfg!(debug_assertions) {
        println!("Creating Vulkan instance, please wait...");
    }
    let vk_instance = vk::Instance::new(vk_lib, instance_create_info).unwrap();

    Ok(vk_instance)
}

fn list_supported_extensions(vk_lib: &vk::VulkanLibrary) {
    println!("Supported extensions:");
    for ext in vk_lib.supported_extensions().into_iter() {
        if let (ext, true) = ext {
            println!("\t{}", ext);
        }
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

fn create_logical_device(
    physical_device: Arc<vk::PhysicalDevice>,
) -> anyhow::Result<Arc<vk::Device>> {
    let queue_family_properties = physical_device.queue_family_properties();
    let graphics_queue_family_index = queue_family_properties
        .iter()
        .enumerate()
        .find_map(|(index, qfp)| {
            if qfp.queue_flags.intersects(vk::QueueFlags::GRAPHICS) {
                Some(index as u32)
            } else {
                None
            }
        })
        .ok_or_else(|| anyhow::anyhow!("No suitable graphics queue family found"))?;

    let device_queue_create_info = vk::QueueCreateInfo {
        queue_family_index: graphics_queue_family_index,
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

    let (device, _) = vk::Device::new(physical_device, device_create_info)?;
    Ok(device)
}
