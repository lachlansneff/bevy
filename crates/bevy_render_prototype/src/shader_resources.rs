use crate::{buffer::Buffer, texture::Texture};
use bevy_asset::Handle;
use bevy_math::{Mat2, Mat3, Mat4, Quat, Vec2, Vec3, Vec4};
use std::{borrow::Cow, fmt};

pub use bevy_derive::{ShaderResources, Uniform};

#[derive(Clone)]
pub enum ShaderResource<'a> {
    Uniform(&'a dyn Uniform),
    GpuUniform(Handle<Buffer>),
    Buffer(Handle<Buffer>),
    Texture(Handle<Texture>),
}

impl fmt::Debug for ShaderResource<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Uniform(_) => write!(f, "ShaderResource::Uniform"),
            Self::GpuUniform(_) => write!(f, "ShaderResource::GpuUniform"),
            Self::Buffer(_) => write!(f, "ShaderResource::Buffer"),
            Self::Texture(_) => write!(f, "ShaderResource::Texture"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ShaderBinding {
    pub set: u32,
    pub binding: u32,
}

pub trait Uniform {
    /// Since the uniform is converted into a correctly padded version inline,
    /// we can't return it in a dynamic manner.
    fn copy_padded_to_slice(&self, s: &mut [u8]);
    fn padded_size(&self) -> usize;
}

pub trait ShaderResources {
    fn shader_resources(&self) -> Vec<(Cow<str>, ShaderBinding, ShaderResource)>;
    fn shader_specialization(&self) -> Vec<(Cow<str>, bool)>;
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
//
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

// Can we just recreate this in every instance of the proc macro
// to avoid exporting them?
#[doc(hidden)]
pub unsafe trait IntoStd140 {
    type Ty;
    type Align: Default;

    fn into_std140(&self) -> Self::Ty;
}

#[doc(hidden)]
#[repr(align(4))]
#[derive(Default)]
pub struct Align4;
#[doc(hidden)]
#[repr(align(8))]
#[derive(Default)]
pub struct Align8;
#[doc(hidden)]
#[repr(align(16))]
#[derive(Default)]
pub struct Align16;

macro_rules! impl_scalar {
    ($type:ty : $align:ty) => {
        #[doc(hidden)]
        unsafe impl IntoStd140 for $type {
            type Ty = $type;
            type Align = $align;
            fn into_std140(&self) -> $type {
                *self
            }
        }
    };
    ($type:ty as $as_type:ty : $align:ty) => {
        #[doc(hidden)]
        unsafe impl IntoStd140 for $type {
            type Ty = $as_type;
            type Align = $align;
            fn into_std140(&self) -> $as_type {
                *self as $as_type
            }
        }
    };
    ($($type:ty : $align:ty,)*) => {
        $(
            impl_scalar!($type : $align);
        )*
    };
    ($($type:ty as $as_type:ty : $align:ty,)*) => {
        $(
            impl_scalar!($type as $as_type: $align);
        )*
    };
}

impl_scalar!(
    // simple scalars
    u32: Align4,
    i32: Align4,
    f32: Align4,
    f64: Align8,
    // vectors
    Vec2: Align8,
    Vec3: Align16,
    Vec4: Align16,
    // matrices
    Mat2: Align16,
    Mat3: Align16,
    Mat4: Align16,
    Quat: Align16, // basically just a Vec4
    // some other types
    crate::color::Color: Align4,
);

impl_scalar!(
    // scalars that must be casted
    bool as u32: Align4,
    u8 as u32: Align4,
    u16 as u32: Align4,
    i8 as i32: Align4,
    i16 as i32: Align4,
);

#[doc(hidden)]
#[repr(C, align(16))]
pub struct Element<T: IntoStd140>(pub T::Ty, pub T::Align);

#[doc(hidden)]
unsafe impl<T, const N: usize> IntoStd140 for [T; N]
where
    T: IntoStd140,
{
    type Align = Align16;
    type Ty = [Element<T>; N];

    fn into_std140(&self) -> Self::Ty {
        use std::mem::{self, MaybeUninit};
        let mut array: [MaybeUninit<Element<T>>; N] =
            unsafe { MaybeUninit::uninit().assume_init() };

        for (from, into) in self.iter().zip(array.iter_mut()) {
            *into = MaybeUninit::new(Element(from.into_std140(), T::Align::default()));
        }

        let output = unsafe {
            (&array as *const [MaybeUninit<Element<T>>; N] as *const [Element<T>; N]).read()
        };
        mem::forget(array);

        output
    }
}
