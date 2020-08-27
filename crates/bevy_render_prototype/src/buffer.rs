use crate::shader_resources::{GpuShaderResource, ShaderResource};
use bevy_asset::Handle;
use std::{fmt, ops::RangeBounds};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MapMode {
    Read,
    Write,
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
    #[cfg(feature = "wgpu")]
    Wgpu { buffer: wgpu::Buffer, size: u64 },
    #[cfg(feature = "headless")]
    Headless,
}

impl Buffer {
    pub async fn map<S: RangeBounds<u64>>(
        &self,
        mode: MapMode,
        range: S,
    ) -> Result<(), BufferMappingError> {
        match self {
            #[cfg(feature = "wgpu")]
            Self::Wgpu { buffer, .. } => {
                let mode = match mode {
                    MapMode::Read => wgpu::MapMode::Read,
                    MapMode::Write => wgpu::MapMode::Write,
                };

                buffer
                    .slice(range)
                    .map_async(mode)
                    .await
                    .map_err(|_| BufferMappingError)
            }
            #[cfg(feature = "headless")]
            Self::Headless => {
                let _ = (mode, range);
                Ok(())
            }
        }
    }

    pub fn unmap(&self) {
        match self {
            #[cfg(feature = "wgpu")]
            Self::Wgpu { buffer, .. } => {
                buffer.unmap();
            }
            #[cfg(feature = "headless")]
            Self::Headless => {}
        }
    }
}

impl From<&Handle<Buffer>> for ShaderResource<'_> {
    fn from(handle: &Handle<Buffer>) -> Self {
        ShaderResource::Gpu(GpuShaderResource::Buffer(*handle))
    }
}
