use ash::vk;

pub struct VwDevice2 {
    physical_device: vk::PhysicalDevice,
}

impl VwDevice2 {
    pub fn new() -> anyhow::Result<Self> {
        Ok(VwDevice2 {
            physical_device: vk::PhysicalDevice::null(),
        })
    }
}
