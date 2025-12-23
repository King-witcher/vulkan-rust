use anyhow::bail;
use ash::khr;
use ash::vk;
use std::ffi::CStr;
use std::sync::Arc;

pub struct VwDevice2 {
    vk: Arc<ash::Instance>,
    physical_device: vk::PhysicalDevice,
    logical_device: ash::Device,
}

impl VwDevice2 {
    pub fn new(vk: Arc<ash::Instance>) -> anyhow::Result<Self> {
        let physical_device = pick_physical_device(&vk)?;
        let logical_device = create_logical_device(&vk, physical_device)?;
        vk::Device::destroy_device(&logical_device, None);

        Ok(VwDevice2 {
            vk,
            physical_device,
            logical_device,
        })
    }
}

impl Drop for VwDevice2 {
    fn drop(&mut self) {
        unsafe {}
    }
}

fn pick_physical_device(vk: &ash::Instance) -> anyhow::Result<vk::PhysicalDevice> {
    let physical_devices = unsafe { vk.enumerate_physical_devices() }?;
    let suitable_devices = physical_devices
        .into_iter()
        .filter(|device| is_device_suitable(vk, *device))
        .collect::<Vec<_>>();

    if suitable_devices.is_empty() {
        anyhow::bail!("No suitable Vulkan physical devices found.");
    }

    let mut best_score = 0;
    let mut best_device = None;

    for device in suitable_devices.iter() {
        let mut score = 0;
        let properties = unsafe { vk.get_physical_device_properties(*device) };
        // let features = unsafe { vk.get_physical_device_features(*device) };

        if properties.device_type == vk::PhysicalDeviceType::DISCRETE_GPU {
            score += 1000;
        }

        score += properties.limits.max_image_dimension2_d;
        if score > best_score {
            best_score = score;
            best_device = Some(*device);
        }
    }

    if let Some(device) = best_device {
        Ok(device)
    } else {
        bail!("No suitable Vulkan physical devices found.");
    }
}

const REQUIRED_DEVICE_EXTENSIONS: [&CStr; 1] = [khr::swapchain::NAME];

fn is_device_suitable(vk: &ash::Instance, device: vk::PhysicalDevice) -> bool {
    let queue_families = unsafe { vk.get_physical_device_queue_family_properties(device) };
    // let features = unsafe { vk.get_physical_device_features(device) };
    let properties = unsafe { vk.get_physical_device_properties(device) };
    let device_extensions = unsafe { vk.enumerate_device_extension_properties(device) }
        .expect("failed to enumerate extensions");

    if !queue_families
        .iter()
        .any(|qfp| qfp.queue_flags.intersects(vk::QueueFlags::GRAPHICS))
    {
        return false;
    }

    if properties.api_version < vk::API_VERSION_1_3 {
        return false;
    }

    if REQUIRED_DEVICE_EXTENSIONS.iter().any(|&required| {
        !device_extensions
            .iter()
            .any(|device_ext| device_ext.extension_name_as_c_str().unwrap() == required)
    }) {
        return false;
    }

    true
}

fn create_logical_device(
    vk: &ash::Instance,
    physical_device: vk::PhysicalDevice,
) -> anyhow::Result<vk::Device> {
    unimplemented!()
}
