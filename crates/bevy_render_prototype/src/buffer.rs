use crate::shader_resources::{ShaderResource, GpuShaderResource};
use bevy_asset::Handle;
use std::{
    fmt,
    ops::RangeBounds,
};


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MapMode {
    Read,
    Write
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct BufferMappingError;

impl fmt::Display for BufferMappingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error occurred when trying to async map a buffer")
    }
}

#[non_exhaustive]
pub enum Buffer {
    Wgpu {
        buffer: wgpu::Buffer,
        size: u64,
    }
}

impl Buffer {
    pub async fn map_async<S: RangeBounds<u64>>(&self, mode: MapMode, range: S) -> Result<(), BufferMappingError> {
        match self {
            Self::Wgpu { buffer, .. } => {
                let mode = match mode {
                    MapMode::Read => wgpu::MapMode::Read,
                    MapMode::Write => wgpu::MapMode::Write,
                };

                let res = buffer.slice(range).map_async(mode).await;
                res.map_err(|_| BufferMappingError)
            }
        }
    }

    pub fn unmap(&self) {
        match self {
            Self::Wgpu { buffer, .. } => {
                buffer.unmap();
            },
        }
    }
}

impl From<&Handle<Buffer>> for ShaderResource<'_> {
    fn from(handle: &Handle<Buffer>) -> Self {
        ShaderResource::Gpu(GpuShaderResource::Buffer(*handle))
    }
}
