use crate::device_queue::{Device, Queue};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
pub struct WgpuConfig {
    pub shader_validation: bool,
    // Add limits and features
}

impl Default for WgpuConfig {
    fn default() -> Self {
        Self {
            shader_validation: true,
        }
    }
}

pub trait UnwrapWgpu: Sized {
    type WgpuType;

    fn try_unwrap_wgpu(self) -> Result<Self::WgpuType, ()>;
    #[inline(always)]
    fn unwrap_wgpu(self) -> Self::WgpuType {
        self.try_unwrap_wgpu()
            .unwrap_or_else(|_| crate::wrong_backend())
    }
}

pub async fn create_device_and_queue(
    render_config: &super::RenderPluginConfig,
    wgpu_config: &WgpuConfig,
) -> (Device, Queue) {
    let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: match render_config.power_preference {
                Some(super::PowerPreference::HighPerformance) | None => {
                    wgpu::PowerPreference::HighPerformance
                }
                Some(super::PowerPreference::LowPower) => wgpu::PowerPreference::LowPower,
            },
            compatible_surface: None,
        })
        .await
        .expect("Unable to find a GPU! Make sure you have installed required drivers!");

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
                shader_validation: wgpu_config.shader_validation,
            },
            None,
        )
        .await
        .unwrap();

    let device = Arc::new(device);
    let queue = Arc::new(queue);

    (Device::Wgpu(device), Queue::Wgpu(queue))
}
