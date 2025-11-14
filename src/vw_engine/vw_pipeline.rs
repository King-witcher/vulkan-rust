use std::sync::Arc;

use vulkano::shader::{ShaderModule, ShaderModuleCreateInfo};

use crate::vw_engine::vw_device::VwDevice;

pub struct VwPipeline {
    shader_module: Arc<ShaderModule>,
}

impl VwPipeline {
    pub fn new(device: &VwDevice, shader_code: &[u8]) -> anyhow::Result<Self> {
        let words = vulkano::shader::spirv::bytes_to_words(shader_code)?.into_owned();
        let shader_create_info = ShaderModuleCreateInfo::new(&words);
        let shader_module =
            unsafe { ShaderModule::new(device.logical_device(), shader_create_info)? };

        Ok(VwPipeline { shader_module })
    }
}
