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
