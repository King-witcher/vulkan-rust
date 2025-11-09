use std::os::raw::c_void;

pub struct VwWindow {
    sdl_context: sdl2::Sdl,
    _sdl_video: sdl2::VideoSubsystem,
    sdl_window: sdl2::video::Window,
}

pub struct VwWindowCreateInfo<'s> {
    pub title: &'s str,
    pub width: u32,
    pub height: u32,
    pub fullscreen: bool,
}

impl VwWindow {
    pub fn create_vk_surface(&self, instance: *const c_void) -> *const c_void {
        self.sdl_window
            .vulkan_create_surface(instance as _)
            .expect("Failed to create Vulkan surface") as *const c_void
    }

    pub fn event_pump(&self) -> sdl2::EventPump {
        self.sdl_context
            .event_pump()
            .expect("Failed to get SDL2 event pump")
    }

    pub fn swap_window(&self) {
        self.sdl_window.gl_swap_window();
    }

    pub fn set_relative_mouse_mode(&self, enabled: bool) {
        self.sdl_context.mouse().set_relative_mouse_mode(enabled);
    }
}

impl From<VwWindowCreateInfo<'_>> for VwWindow {
    fn from(create_info: VwWindowCreateInfo) -> Self {
        let sdl_context = sdl2::init().expect("Failed to initialize SDL2");
        let _sdl_video = sdl_context
            .video()
            .expect("Failed to get SDL2 video subsystem");

        let sdl_window = _sdl_video
            .window(create_info.title, create_info.width, create_info.height)
            .position(-1800, 100)
            .vulkan()
            .build()
            .expect("Failed to create SDL2 window");

        VwWindow {
            sdl_context,
            _sdl_video,
            sdl_window,
        }
    }
}

impl Default for VwWindowCreateInfo<'_> {
    fn default() -> Self {
        VwWindowCreateInfo {
            title: "VkWizard Window",
            width: 800,
            height: 600,
            fullscreen: false,
        }
    }
}
