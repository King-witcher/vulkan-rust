use std::{sync::Arc, vec};

use sdl::{event::Event, keyboard::Scancode};

use crate::{
    vk,
    vw_engine::{vw_device::VwDevice, vw_pipeline::VwPipeline, vw_swapchain::VwSwapchain},
    vw_window::{VwWindow, VwWindowCreateInfo},
};

mod vw_device;
mod vw_pipeline;
mod vw_swapchain;

pub struct VkWizardEngine {
    vk_library: Arc<vk::VulkanLibrary>,
    vk_instance: Arc<vk::Instance>,

    vw_device: VwDevice,
    vw_swapchain: VwSwapchain,
    vw_pipeline: VwPipeline,
    vw_window: VwWindow,
}

impl VkWizardEngine {
    pub fn new() -> anyhow::Result<Self> {
        let vk_library = vk::VulkanLibrary::new()?;
        let vk_instance = create_vulkan_instance(vk_library.clone())?;

        let vw_window = VwWindow::new(VwWindowCreateInfo {
            title: "VkWizard Window",
            position: (-1400, 100),
            ..Default::default()
        });

        let surface = vw_window.create_vk_surface(vk_instance.clone());

        let vw_device = VwDevice::new(vk_instance.clone(), surface)?;
        let vw_swapchain = VwSwapchain::new(&vw_device)?;

        let shader_code = include_bytes!("../shaders/shader.slang.spv");
        let vw_pipeline = VwPipeline::new(&vw_device, shader_code)?;

        Ok(VkWizardEngine {
            vk_library,
            vk_instance,

            vw_device,
            vw_swapchain,
            vw_pipeline,
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
            self.vw_window.present();
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
        println!("Attaching validation layers, please wait...");
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
