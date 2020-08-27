use crate::buffer::Buffer;
#[cfg(feature = "wgpu")]
use crate::wgpu::UnwrapWgpu;

#[non_exhaustive]
pub enum CommandBuffer {
    #[cfg(feature = "wgpu")]
    Wgpu(wgpu::CommandBuffer),
    #[cfg(feature = "headless")]
    Headless,
}

#[non_exhaustive]
pub enum CommandEncoder {
    #[cfg(feature = "wgpu")]
    Wgpu(wgpu::CommandEncoder),
    #[cfg(feature = "headless")]
    Headless,
}

impl CommandEncoder {
    pub fn finish(self) -> CommandBuffer {
        match self {
            #[cfg(feature = "wgpu")]
            CommandEncoder::Wgpu(command_encoder) => CommandBuffer::Wgpu(command_encoder.finish()),
            #[cfg(feature = "headless")]
            CommandEncoder::Headless => CommandBuffer::Headless,
        }
    }

    // pub fn begin_render_pass
    // pub fn begin_compute_pass

    pub fn copy_buffer_to_buffer(
        &mut self,
        src: &Buffer,
        src_offset: u64,
        dest: &Buffer,
        dest_offset: u64,
        copy_size: u64,
    ) {
        match self {
            #[cfg(feature = "wgpu")]
            CommandEncoder::Wgpu(command_encoder) => {
                let (src, _) = src.unwrap_wgpu();
                let (dest, _) = dest.unwrap_wgpu();
                command_encoder.copy_buffer_to_buffer(
                    src,
                    src_offset,
                    dest,
                    dest_offset,
                    copy_size,
                );
            }
            #[cfg(feature = "headless")]
            CommandEncoder::Headless => {
                let _ = (src, src_offset, dest, dest_offset, copy_size);
            }
        }
    }

    // ... https://wgpu.rs/doc/wgpu/struct.CommandEncoder.html
}

#[cfg(feature = "wgpu")]
impl crate::wgpu::UnwrapWgpu for CommandBuffer {
    type WgpuType = wgpu::CommandBuffer;

    #[inline]
    fn try_unwrap_wgpu(self) -> Result<Self::WgpuType, ()> {
        match self {
            CommandBuffer::Wgpu(command_buffer) => Ok(command_buffer),
            _ => Err(()),
        }
    }
}
