use std::{os::raw::c_void, sync::Arc};

use ash::vk::SurfaceKHR;
use sdl::sys::VkSurfaceKHR;
use vulkano::{Handle, VulkanObject, swapchain::SurfaceApi};

use crate::vk;

pub struct VwWindow {
    sdl_context: sdl2::Sdl,
    // sdl_video: sdl2::VideoSubsystem,
    sdl_window: sdl2::video::Window,
}

pub struct VwWindowCreateInfo<'s> {
    pub title: &'s str,
    pub extent: (u32, u32),
    pub position: (i32, i32),
    pub fullscreen: bool,
}

impl VwWindow {
    pub fn new(create_info: VwWindowCreateInfo) -> Self {
        let sdl_context = sdl2::init().expect("Failed to initialize SDL2");
        let sdl_video = sdl_context
            .video()
            .expect("Failed to get SDL2 video subsystem");

        let sdl_window = {
            let title = if cfg!(debug_assertions) {
                format!("[DEBUG] {}", create_info.title)
            } else {
                create_info.title.to_string()
            };
            let (width, height) = create_info.extent;
            let (x_pos, y_pos) = create_info.position;

            let mut window_builder = sdl_video.window(&title, width, height);
            window_builder.vulkan();
            window_builder.position(x_pos, y_pos);
            if create_info.fullscreen {
                window_builder.fullscreen();
            }
            window_builder
                .build()
                .expect("Failed to create SDL2 window")
        };

        VwWindow {
            sdl_context,
            // sdl_video,
            sdl_window,
        }
    }

    pub fn present(&self) {
        self.sdl_window.gl_swap_window();
    }

    pub fn create_vk_surface(&self, instance: Arc<vk::Instance>) -> vk::Surface {
        let vk_surface = self
            .sdl_window
            .vulkan_create_surface(instance.handle().as_raw() as _)
            .expect("Failed to create Vulkan surface");

        let surface = SurfaceKHR::from_raw(vk_surface);
        unsafe { vk::Surface::from_handle(instance, surface, SurfaceApi::Win32, None) }
    }

    pub fn event_pump(&self) -> sdl2::EventPump {
        self.sdl_context
            .event_pump()
            .expect("Failed to get SDL2 event pump")
    }

    pub fn set_relative_mouse_mode(&self, enabled: bool) {
        self.sdl_context.mouse().set_relative_mouse_mode(enabled);
    }
}

impl Default for VwWindowCreateInfo<'_> {
    fn default() -> Self {
        VwWindowCreateInfo {
            title: "VkWizard Window",
            extent: (1280, 720),
            position: (100, 100),
            fullscreen: false,
        }
    }
}
