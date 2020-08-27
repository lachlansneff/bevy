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

#[cfg(not(any(feature = "headless", feature = "wgpu")))]
compile_error!("Enable at least one render backend.");

#[inline(never)]
#[cold]
fn wrong_backend() -> ! {
    panic!("render backend invariant broken, you have multiple render backend activated!")
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
