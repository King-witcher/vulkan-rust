use ash::vk;

pub struct VwEngine2 {
    _entry: ash::Entry,
    instance: ash::Instance,
}

impl VwEngine2 {
    pub fn new() -> anyhow::Result<Self> {
        use ash::Entry;
        let entry = Entry::linked();

        let instance = unsafe { create_instance(&entry)? };

        Ok(VwEngine2 {
            _entry: entry,
            instance,
        })
    }
}

unsafe fn create_instance(entry: &ash::Entry) -> anyhow::Result<ash::Instance> {
    use ash::khr;

    let app_info = vk::ApplicationInfo::default()
        .application_name(c"VkWizard Application")
        .application_version(vk::make_api_version(0, 1, 0, 0))
        .engine_name(c"VkWizard Engine")
        .engine_version(vk::make_api_version(0, 1, 0, 0))
        .api_version(vk::API_VERSION_1_3);

    // Required extensions by SDL2 for Vulkan surface creation on Windows
    let extension_names = vec![
        khr::surface::NAME.as_ptr(),
        khr::win32_surface::NAME.as_ptr(),
    ];

    let instance_create_info = vk::InstanceCreateInfo::default()
        .application_info(&app_info)
        .enabled_extension_names(&extension_names);

    let instance = unsafe { entry.create_instance(&instance_create_info, None) }?;

    Ok(instance)
}

impl Drop for VwEngine2 {
    fn drop(&mut self) {
        unsafe {
            self.instance.destroy_instance(None);
        }
    }
}
