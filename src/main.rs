use std::sync::Arc;

use sdl::keyboard::Scancode;
use sdl2::event::Event;
use vulkano::{
    Handle, Version, VulkanLibrary, VulkanObject,
    instance::{Instance, InstanceCreateFlags, InstanceCreateInfo, InstanceExtensions},
};

extern crate sdl2 as sdl;
extern crate vulkano;

mod vk_instance;

fn main() {
    let vk_lib = VulkanLibrary::new().unwrap();
    let sdl = sdl::init().unwrap();

    let video = sdl.video().unwrap();
    let window = video.window("VkWizard", 800, 600).vulkan().build().unwrap();
    let extensions = window.vulkan_instance_extensions().unwrap();
    println!("Vulkan extensions required by SDL2: {:?}", extensions);

    let vk_instance = create_vulkan_instance(vk_lib.clone());

    let vk_surface = window
        .vulkan_create_surface(vk_instance.handle().as_raw() as _)
        .unwrap();

    loop {
        for event in sdl.event_pump().unwrap().poll_iter() {
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
        enabled_layers: vec![],
        enabled_extensions,
        debug_utils_messengers: vec![],
        enabled_validation_features: vec![],
        disabled_validation_features: vec![],

        ..Default::default()
    };

    let vk_instance = Instance::new(vk_lib, instance_create_info).unwrap();

    vk_instance
}
