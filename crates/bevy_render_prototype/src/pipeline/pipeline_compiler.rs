use super::{state_descriptors::PrimitiveTopology, PipelineDescriptor, VertexBufferDescriptors};
use crate::{
    shader::{Shader, ShaderSource},
};
use bevy_asset::{Assets, Handle};
use bevy_property::{Properties, Property};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Default)]
pub struct PipelineCompiler;

impl PipelineCompiler {
    fn compile_shader(
        &mut self,
        shaders: &mut Assets<Shader>,
        shader_handle: &Handle<Shader>,
    ) -> Handle<Shader> {
        let shader = shaders.get(shader_handle).unwrap();

        // don't produce new shader if the input source is already spirv
        if let ShaderSource::Spirv(_) = shader.source {
            return *shader_handle;
        }

        // Compile shader
        let shader = shader.get_spirv_shader(None);
        shaders.add(shader)
    }

    pub fn compile_pipeline(
        &mut self,
        pipelines: &mut Assets<PipelineDescriptor>,
        shaders: &mut Assets<Shader>,
        source_pipeline: Handle<PipelineDescriptor>,
        vertex_buffer_descriptors: Option<&VertexBufferDescriptors>,
    ) -> Handle<PipelineDescriptor> {
        todo!();
    }
}
