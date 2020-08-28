#![feature(min_const_generics)] // for now
#![cfg_attr(not(any(feature = "headless", feature = "wgpu")), allow(dead_code))]

pub mod buffer;
pub mod color;
pub mod commands;
pub mod device_queue;
pub mod pipeline;
pub mod shader;
pub mod shader_resources;
pub mod texture;

#[cfg(feature = "wgpu")]
mod wgpu;
#[cfg(feature = "wgpu")]
pub use self::wgpu::WgpuConfig;

use bevy_app::{AppBuilder, Plugin};
use serde::{Deserialize, Serialize};

#[cfg(not(any(feature = "headless", feature = "wgpu")))]
compile_error!("Enable at least one render backend.");

#[inline(never)]
#[cold]
fn wrong_backend() -> ! {
    panic!("render backend invariant broken, you have multiple render backend activated!")
}

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PowerPreference {
    HighPerformance,
    LowPower,
}

#[derive(Serialize, Deserialize)]
pub enum BackendConfig {
    #[cfg(feature = "wgpu")]
    Wgpu(WgpuConfig),
    #[cfg(feature = "headless")]
    Headless,
}

impl Default for BackendConfig {
    fn default() -> Self {
        #[cfg(feature = "wgpu")]
        {
            Self::Wgpu(WgpuConfig::default())
        }
        #[cfg(all(not(feature = "wgpu"), feature = "headless"))]
        {
            Self::Headless
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct RenderPluginConfig {
    pub power_preference: Option<PowerPreference>,
    pub backend: BackendConfig,
}

#[derive(Default)]
pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut AppBuilder) {
        if app.resources().get::<RenderPluginConfig>().is_none() {
            app.init_resource::<RenderPluginConfig>();
        }

        let (device, queue) = {
            let config = app.resources().get::<RenderPluginConfig>().unwrap();
            match config.backend {
                #[cfg(feature = "wgpu")]
                BackendConfig::Wgpu(ref wgpu_config) => {
                    pollster::block_on(wgpu::create_device_and_queue(&config, wgpu_config))
                }
                #[cfg(feature = "headless")]
                BackendConfig::Headless => (
                    device_queue::Device::Headless,
                    device_queue::Queue::Headless,
                ),
            }
        };
        app.add_resource(device);
        app.add_resource(queue);

        // ...
    }
}
