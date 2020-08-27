use crate::{buffer::Buffer, texture::Texture};
use bevy_asset::Handle;
use bevy_math::{Vec2, Vec3, Vec4, Mat2, Mat3, Mat4, Quat};
use std::borrow::Cow;

#[derive(Clone)]
pub enum ShaderResource<'a> {
    Uniform(&'a dyn Uniform),
    GpuUniform(Handle<Buffer>),
    Buffer(Handle<Buffer>),
    Texture(Handle<Texture>),
}

#[derive(Clone)]
pub struct ShaderBinding {
    pub set: u32,
    pub binding: u32,
}

pub trait Uniform {
    /// Since the uniform is converted into a correctly padded version inline,
    /// we can't return it in a dynamic manner.
    fn copy_padded_to_slice(&self, s: &mut [u8]);
    fn size(&self) -> usize;
}

pub trait ShaderResources {
    fn shader_resources(&self) -> Cow<[(Option<Cow<str>>, ShaderBinding, ShaderResource)]>;
    fn shader_specialization(&self) -> Cow<[(Cow<str>, bool)]>;
}

// The way I'm visualing this:
//
// #[derive(Uniform)]
// struct MyUniform {
//     mvp: Mat4,
//     position: Vec3,
//     normal: Vec3,
//     uv: Vec2,
//     constants: [i32; 3],
// }

// #[derive(ShaderResources)]
// struct MyShaderResources {
//     #[uniform(set = 0, binding = 0)]
//     vert_uniform: MyUniform,
//     #[uniform(set = 1, binding = 0)]
//     frag_uniform: SomeOtherUniform,
//     #[buffer(set = 0, binding = 1)]
//     point_buffer: Handle<Buffer>,
//     #[texture(set = 0, binding = 2)]
//     my_texture: Handle<Texture>,
//     #[specialize]
//     shading_enabled: bool,
//     #[specialize("ENABLE_FOOBAR")]
//     foobar_enabled: bool,
// }

pub trait IntoUniformType {
    type Padded;
    fn into_uniform_type(self) -> Self::Padded;
}

#[macro_export]
macro_rules! impl_into_uniform_type {
    ($ty:ty) => {
        impl crate::shader_resources::IntoUniformType for $ty {
            type Padded = $ty;
            fn into_uniform_type(self) -> Self::Padded {
                self
            }
        }
    };
    ($ty:ty as $target:ty) => {
        impl crate::shader_resources::IntoUniformType for $ty {
            type Padded = $target;
            fn into_uniform_type(self) -> Self::Padded {
                self as $target
            }
        }
    };
    ($ty:ty = $target:ty) => {
        impl crate::shader_resources::IntoUniformType for $ty {
            type Padded = <$target as crate::shader_resources::IntoUniformType>::Padded;
            fn into_uniform_type(self) -> Self::Padded {
                let x: $target = self.into();
                x.into_uniform_type()
            }
        }
    };
    ($ty:ty => $target:ty) => {
        impl crate::shader_resources::IntoUniformType for $ty {
            type Padded = $target;
            fn into_uniform_type(self) -> Self::Padded {
                (*(self.as_ref())).into()
            }
        }
    };
    ($ty:ty ; ($target:ty ; $convert:expr)) => {
        impl crate::shader_resources::IntoUniformType for $ty {
            type Padded = $target;
            fn into_uniform_type(self) -> Self::Padded {
                (($convert)(self)).into()
            }
        }
    };
    ($($ty:ty),*) => {
        $(
            impl_into_uniform_type!($ty);
        )*
    };
    ($($ty:ty as $target:ty),*) => {
        $(
            impl_into_uniform_type!($ty as $target);
        )*
    };
    ($($ty:ty => $target:ty),*) => {
        $(
            impl_into_uniform_type!($ty => $target);
        )*
    };
}

impl_into_uniform_type!(u32, i32, f32, f64);
impl_into_uniform_type!(
    bool as u32,
    u8 as u32,
    u16 as u32,
    i8 as i32,
    i16 as i32
);
impl_into_uniform_type!(
    Vec2 => shader_types::Vec2,
    Vec3 => shader_types::Vec3,
    Vec4 => shader_types::Vec4,
    Mat2 => shader_types::Mat2x2,
    Mat4 => shader_types::Mat4x4,
    Quat => shader_types::Vec4
);
impl_into_uniform_type!(Mat3 ; (shader_types::Mat3x3 ; |x: Mat3| x.to_cols_array()));
