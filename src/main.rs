use std::sync::Arc;

use sdl::keyboard::Scancode;
use sdl2::event::Event;
use vulkano::{
    Handle, Version, VulkanLibrary, VulkanObject,
    instance::{Instance, InstanceCreateFlags, InstanceCreateInfo, InstanceExtensions},
};

use crate::window::{VwWindow, VwWindowCreateInfo};

extern crate sdl2 as sdl;
extern crate vulkano;

mod vk_instance;
mod window;

fn main() {
    let vk_lib = VulkanLibrary::new().unwrap();
    let vk_instance = create_vulkan_instance(vk_lib.clone());

    let window = VwWindow::from(VwWindowCreateInfo {
        title: "VkWizard Window",
        width: 1280,
        height: 720,
        fullscreen: false,
    });
    let vk_surface = window.create_vk_surface(vk_instance.handle().as_raw() as _);

    loop {
        for event in window.event_pump().poll_iter() {
            match event {
                Event::Quit { .. } => return,
                Event::KeyDown {
                    scancode: Some(Scancode::Escape),
                    ..
                } => {
                    return;
                }
                _ => {}
            }
        }
    }
}

fn create_vulkan_instance(vk_lib: Arc<VulkanLibrary>) -> Arc<Instance> {
    println!("Supported extensions:");
    for ext in vk_lib.supported_extensions().into_iter() {
        if let (ext, true) = ext {
            println!("\t{}", ext);
        }
    }

    println!("Supported layers:");
    for layer in vk_lib.layer_properties().unwrap().into_iter() {
        println!("\t{}, {}", layer.name(), layer.implementation_version());
    }

    let enabled_extensions = InstanceExtensions {
        khr_surface: true,
        khr_win32_surface: true,
        ..InstanceExtensions::empty()
    };

    let instance_create_info = InstanceCreateInfo {
        flags: InstanceCreateFlags::empty(),
        application_name: Some("VkWizard Game".into()),
        application_version: Version {
            major: 1,
            minor: 0,
            patch: 0,
        },
        engine_name: Some("VkWizard Engine".into()),
        engine_version: Version {
            major: 1,
            minor: 0,
            patch: 0,
        },
        max_api_version: Some(Version::V1_0),
        enabled_layers: vec!["VK_LAYER_KHRONOS_validation".into()],
        enabled_extensions,
        debug_utils_messengers: vec![],
        enabled_validation_features: vec![],
        disabled_validation_features: vec![],

        ..Default::default()
    };

    println!("Creating Vulkan instance and initializing validation layers...");
    println!("Please wait.");
    let vk_instance = Instance::new(vk_lib, instance_create_info).unwrap();

    vk_instance
}
