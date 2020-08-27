use crate::{buffer::Buffer, texture::Texture};
use bevy_asset::Handle;
use bevy_math::{Mat4, Vec2, Vec3, Vec4};
use std::borrow::Cow;

#[derive(Clone)]
pub enum GpuShaderResource {
    Buffer(Handle<Buffer>),
    Texture(Handle<Texture>),
}

#[derive(Clone)]
pub enum ShaderResource<'a> {
    Gpu(GpuShaderResource),
    Cpu(&'a [u8]),
}

#[derive(Clone)]
pub struct ShaderBinding {
    pub set: u32,
    pub binding: u32,
}

pub trait ShaderResources {
    fn shader_resources(&self) -> Cow<[(Cow<str>, ShaderBinding, ShaderResource)]>;
}

// impl<T> From<T> for ShaderResource where T: AsBytes {
//     fn from(as_bytes: T) -> Self {

//     }
// }

#[macro_export]
macro_rules! impl_into_shader_resource {
    ($ty:ident) => {
        impl<'a> From<&'a $ty> for crate::shader_resources::ShaderResource<'a> {
            fn from(x: &'a $ty) -> Self {
                use bevy_core::AsBytes;
                Self::Cpu(x.as_bytes())
            }
        }
    };
}

// TODO: when specialization lands, replace these with impl<T> RenderResource for T where T: AsBytes
impl_into_shader_resource!(Vec2);
impl_into_shader_resource!(Vec3);
impl_into_shader_resource!(Vec4);
impl_into_shader_resource!(Mat4);
impl_into_shader_resource!(u8);
impl_into_shader_resource!(u16);
impl_into_shader_resource!(u32);
impl_into_shader_resource!(u64);
impl_into_shader_resource!(i8);
impl_into_shader_resource!(i16);
impl_into_shader_resource!(i32);
impl_into_shader_resource!(i64);
impl_into_shader_resource!(f32);
impl_into_shader_resource!(f64);

impl<'a> From<&'a Vec<u8>> for ShaderResource<'a> {
    fn from(vec: &'a Vec<u8>) -> Self {
        Self::Cpu(vec.as_slice())
    }
}

impl<'a> From<&'a Box<[u8]>> for ShaderResource<'a> {
    fn from(b: &'a Box<[u8]>) -> Self {
        Self::Cpu(&*b)
    }
}
