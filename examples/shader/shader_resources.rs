use bevy::prelude::*;
use bevy_render_prototype::{
    buffer::Buffer,
    shader_resources::{ShaderResources, Uniform},
    texture::Texture,
};

#[derive(Default, Uniform)]
struct MyUniform {
    foo: Vec3,
    ints: [u32; 2],
}
#[derive(Default, ShaderResources)]
struct MyShaderResources {
    #[uniform(set = 0, binding = 0)]
    my_uniform: MyUniform,
    #[buffer(set = 0, binding = 1)]
    my_buffer: Handle<Buffer>,
    #[texture(set = 0, binding = 2)]
    my_texture: Handle<Texture>,
}

fn main() {
    let resources = MyShaderResources::default();

    println!("{:#?}", resources.shader_resources());
}
