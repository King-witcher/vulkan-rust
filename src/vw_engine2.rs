use std::sync::Arc;

use anyhow::bail;
use ash::vk;

use crate::vw_engine2::vw_device2::VwDevice2;

mod vw_device2;

pub struct VwEngine2 {
    _entry: ash::Entry,
    instance: Arc<ash::Instance>,
    device: VwDevice2,
}

impl VwEngine2 {
    pub fn new() -> anyhow::Result<Self> {
        use ash::Entry;
        let entry = Entry::linked();

        let instance = unsafe { create_instance(&entry)? };
        let instance = Arc::new(instance);
        let device = VwDevice2::new(instance.clone())?;

        Ok(VwEngine2 {
            _entry: entry,
            instance,
            device,
        })
    }
}

unsafe fn create_instance(entry: &ash::Entry) -> anyhow::Result<ash::Instance> {
    use ash::khr;

    check_extension_support(&entry)?;

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

    let mut enable_layers = vec![];
    if cfg!(debug_assertions) {
        enable_layers.push(c"VK_LAYER_KHRONOS_validation".as_ptr());
        println!("Enabling validation layers. Please wait...");
    }

    let instance_create_info = vk::InstanceCreateInfo::default()
        .application_info(&app_info)
        .enabled_extension_names(&extension_names)
        .enabled_layer_names(&enable_layers);

    let instance = unsafe { entry.create_instance(&instance_create_info, None) }?;

    Ok(instance)
}

fn check_extension_support(entry: &ash::Entry) -> anyhow::Result<()> {
    use ash::khr;

    let extensions = unsafe { entry.enumerate_instance_extension_properties(None) }?;
    let mut has_surface = false;
    let mut has_win32_surface = false;

    for ext in extensions {
        if ext.extension_name_as_c_str()? == khr::surface::NAME {
            has_surface = true;
        }
        if ext.extension_name_as_c_str()? == khr::win32_surface::NAME {
            has_win32_surface = true;
        }
    }

    if !has_surface {
        bail!("Required extension khr_surface is not supported");
    }
    if !has_win32_surface {
        bail!("Required extension khr_win32_surface is not supported");
    }

    Ok(())
}

impl Drop for VwEngine2 {
    fn drop(&mut self) {
        unsafe {
            self.instance.destroy_instance(None);
        }
    }
}
