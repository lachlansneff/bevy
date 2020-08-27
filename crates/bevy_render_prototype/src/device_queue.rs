#[cfg(feature = "wgpu")]
use crate::wgpu::UnwrapWgpu;
use crate::{
    buffer::{Buffer, BufferDescriptor},
    commands::CommandBuffer,
    texture::{Extent3d, TextureCopyView, TextureDataLayout},
};

#[non_exhaustive]
pub enum Device {
    #[cfg(feature = "wgpu")]
    Wgpu(wgpu::Device),
    #[cfg(feature = "headless")]
    Headless,
}

#[non_exhaustive]
pub enum Queue {
    #[cfg(feature = "wgpu")]
    Wgpu(wgpu::Queue),
    #[cfg(feature = "headless")]
    Headless,
}

impl Device {
    pub fn create_buffer(&self, desc: &BufferDescriptor) -> Buffer {
        match self {
            #[cfg(feature = "wgpu")]
            Self::Wgpu(device) => Buffer::Wgpu {
                buffer: device.create_buffer(&desc.into()),
                size: desc.size,
            },
            #[cfg(feature = "headless")]
            Self::Headless => {
                let _ = desc;
                Buffer::Headless
            }
        }
    }

    // ... https://wgpu.rs/doc/wgpu/struct.Device.html
}

impl Queue {
    pub fn write_buffer(&self, buffer: &Buffer, offset: u64, data: &[u8]) {
        match self {
            #[cfg(feature = "wgpu")]
            Self::Wgpu(queue) => {
                let (buffer, _) = buffer.unwrap_wgpu();
                queue.write_buffer(buffer, offset, data);
            }
            #[cfg(feature = "headless")]
            Self::Headless => {
                let _ = (buffer, offset, data);
            }
        }
    }

    pub fn write_texture(
        &self,
        texture: TextureCopyView,
        data: &[u8],
        data_layout: TextureDataLayout,
        size: Extent3d,
    ) {
        match self {
            #[cfg(feature = "wgpu")]
            Self::Wgpu(queue) => {
                queue.write_texture(texture.into(), data, data_layout.into(), size.into())
            }
            #[cfg(feature = "headless")]
            Self::Headless => {
                let _ = (texture, data, data_layout, size);
            }
        }
    }

    pub fn submit<I>(&self, command_buffers: I)
    where
        I: IntoIterator<Item = CommandBuffer>,
    {
        match self {
            #[cfg(feature = "wgpu")]
            Self::Wgpu(queue) => {
                queue.submit(
                    command_buffers
                        .into_iter()
                        .map(|cmd_buf| cmd_buf.unwrap_wgpu()),
                );
            }
            #[cfg(feature = "headless")]
            Self::Headless => {
                let _ = command_buffers;
            }
        }
    }

    // ... https://wgpu.rs/doc/wgpu/struct.Queue.html
}

#[cfg(feature = "wgpu")]
impl<'a> crate::wgpu::UnwrapWgpu for &'a Device {
    type WgpuType = &'a wgpu::Device;

    #[inline]
    fn try_unwrap_wgpu(self) -> Result<Self::WgpuType, ()> {
        match self {
            Device::Wgpu(device) => Ok(device),
            _ => Err(()),
        }
    }
}

#[cfg(feature = "wgpu")]
impl<'a> crate::wgpu::UnwrapWgpu for &'a Queue {
    type WgpuType = &'a wgpu::Queue;

    #[inline]
    fn try_unwrap_wgpu(self) -> Result<Self::WgpuType, ()> {
        match self {
            Queue::Wgpu(queue) => Ok(queue),
            _ => Err(()),
        }
    }
}
