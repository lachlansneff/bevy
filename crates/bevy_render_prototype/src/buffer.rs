use crate::texture::TextureDataLayout;
#[cfg(feature = "wgpu")]
use crate::wgpu::UnwrapWgpu;
use std::{fmt, ops::RangeBounds};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MapMode {
    Read,
    Write,
}

#[cfg(feature = "wgpu")]
impl From<MapMode> for wgpu::MapMode {
    fn from(mode: MapMode) -> Self {
        match mode {
            MapMode::Read => wgpu::MapMode::Read,
            MapMode::Write => wgpu::MapMode::Write,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct BufferMappingError;

impl fmt::Display for BufferMappingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error occurred when trying to async map a buffer")
    }
}

bitflags::bitflags! {
    #[repr(transparent)]
    pub struct BufferUsage: u32 {
        const MAP_READ = 1;
        const MAP_WRITE = 2;
        const COPY_SRC = 4;
        const COPY_DST = 8;
        const INDEX = 16;
        const VERTEX = 32;
        const UNIFORM = 64;
        const STORAGE = 128;
        const INDIRECT = 256;
    }
}

#[cfg(feature = "wgpu")]
impl From<BufferUsage> for wgpu::BufferUsage {
    fn from(usage: BufferUsage) -> Self {
        wgpu::BufferUsage::from_bits(usage.bits()).expect("failed to convert buffer usage")
    }
}

pub struct BufferDescriptor<'a> {
    pub label: Option<&'a str>,
    pub size: u64,
    pub usage: BufferUsage,
    pub mapped_at_creation: bool,
}

#[cfg(feature = "wgpu")]
impl<'a> From<&BufferDescriptor<'a>> for wgpu::BufferDescriptor<'a> {
    fn from(desc: &BufferDescriptor<'a>) -> Self {
        wgpu::BufferDescriptor {
            label: desc.label,
            size: desc.size,
            usage: desc.usage.into(),
            mapped_at_creation: desc.mapped_at_creation,
        }
    }
}

pub struct BufferCopyView<'a> {
    pub buffer: &'a Buffer,
    pub layout: TextureDataLayout,
}

#[cfg(feature = "wgpu")]
impl<'a> From<BufferCopyView<'a>> for wgpu::BufferCopyView<'a> {
    fn from(view: BufferCopyView<'a>) -> Self {
        Self {
            buffer: view.buffer.unwrap_wgpu().0,
            layout: view.layout.into(),
        }
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
            Self::Wgpu { buffer, .. } => buffer
                .slice(range)
                .map_async(mode.into())
                .await
                .map_err(|_| BufferMappingError),
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

    // ... https://wgpu.rs/doc/wgpu/struct.Buffer.html
}

#[cfg(feature = "wgpu")]
impl<'a> crate::wgpu::UnwrapWgpu for &'a Buffer {
    type WgpuType = (&'a wgpu::Buffer, u64);

    #[inline]
    fn try_unwrap_wgpu(self) -> Result<Self::WgpuType, ()> {
        match *self {
            Buffer::Wgpu { ref buffer, size } => Ok((buffer, size)),
            _ => Err(()),
        }
    }
}
