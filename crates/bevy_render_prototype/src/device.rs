use crate::buffer::Buffer;

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
    #[cfg(feature = "wgpu")]
    pub(crate) fn as_wgpu_backend(&self) -> &wgpu::Device {
        match self {
            Self::Wgpu(device) => device,
            _ => crate::wrong_backend(),
        }
    }

    pub fn create_buffer(&self, desc: &BufferDescriptor) -> Buffer {
        match self {
            #[cfg(feature = "wgpu")]
            Self::Wgpu(device) => Buffer::Wgpu {
                buffer: device.create_buffer(desc.into()),
                size: desc.size,
            },
            #[cfg(feature = "headless")]
            Self::Headless => Buffer::Headless,
        }
    }

    // ... https://wgpu.rs/doc/wgpu/struct.Device.html
}

impl Queue {
    #[cfg(feature = "wgpu")]
    pub(crate) fn as_wgpu_backend(&self) -> &wgpu::Queue {
        match self {
            Self::Wgpu(queue) => queue,
            _ => crate::wrong_backend(),
        }
    }

    pub fn write_buffer(&self, buffer: &Buffer, offset: u64, data: &[u8]) {
        match self {
            #[cfg(feature = "wgpu")]
            Self::Wgpu(queue) => {
                let (buffer, _) = buffer.as_wgpu_backend();
                queue.write_buffer(buffer, offset, data);
            }
            #[cfg(feature = "headless")]
            Self::Headless => {}
        }
    }

    // ... https://wgpu.rs/doc/wgpu/struct.Queue.html
}
