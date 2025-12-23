use std::sync::Arc;

use vulkano::{
    pipeline::{
        PipelineLayout, PipelineShaderStageCreateFlags, PipelineShaderStageCreateInfo,
        graphics::GraphicsPipelineCreateInfo,
    },
    shader::{EntryPoint, ShaderModule, ShaderModuleCreateInfo, ShaderStage},
};

use crate::vw_engine::vw_device::VwDevice;

pub struct VwPipeline {}

impl VwPipeline {
    pub fn new(device: &VwDevice, shader_code: &[u8]) -> anyhow::Result<Self> {
        let words = vulkano::shader::spirv::bytes_to_words(shader_code)?.into_owned();
        let shader_create_info = ShaderModuleCreateInfo::new(&words);

        // The safety of this block depends on the validity of the provided SPIR-V code.
        let shader_module =
            unsafe { ShaderModule::new(device.logical_device(), shader_create_info)? };

        let vert_entry_point = shader_module
            .entry_point("vertex")
            .expect("Couldn't find vertex entry point");
        let frag_entry_point = shader_module
            .entry_point("fragment")
            .expect("Couldn't find fragment entry point");

        let vert_stage_info = PipelineShaderStageCreateInfo::new(vert_entry_point);
        let frag_stage_info = PipelineShaderStageCreateInfo::new(frag_entry_point);

        let test: PipelineShaderStageCreateInfo = PipelineShaderStageCreateInfo {
            ..PipelineShaderStageCreateInfo::new(frag_entry_point)
        };

        let layout = PipelineLayout::new(device.logical_device(), Default::default())?;

        let createInfo = GraphicsPipelineCreateInfo::layout(layout);

        Ok(VwPipeline {})
    }
}
