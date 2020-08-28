use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashSet;
use syn::{
    parse_macro_input, Data, DeriveInput, Error, Fields, Lit, Meta, MetaList, NestedMeta, Result,
};

pub fn derive_shader_resources(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    do_derive(ast)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

pub fn do_derive(ast: DeriveInput) -> Result<TokenStream> {
    let data = match ast.data {
        Data::Struct(data) => data,
        _ => return Err(Error::new(ast.ident.span(), "uniform must be a struct")),
    };

    let struct_name = ast.ident;

    let fields = match data.fields {
        Fields::Named(fields) => fields,
        _ => {
            return Err(Error::new(
                struct_name.span(),
                "uniform cannot be a tuple struct",
            ))
        }
    };

    let mut set_binding_set = HashSet::new();
    let mut shader_resources = Vec::new();

    for field in fields.named {
        let field_name = field.ident.unwrap();
        let field_name_string = format!("{}", field_name);

        let mut parse_set_binding = |meta_list: MetaList| {
            let mut set: Option<u32> = None;
            let mut binding: Option<u32> = None;

            assert!(
                meta_list.nested.len() == 2,
                "there must only be two items in the `#[uniform(..)]` attribute"
            );

            for nested_meta in meta_list.nested {
                let name_value = match nested_meta {
                    NestedMeta::Meta(Meta::NameValue(name_value)) => name_value,
                    _ => panic!("attribute must be formatted as `#[uniform(set = <number>, binding = <number>)]`"),
                };

                if name_value.path.is_ident("set") {
                    assert!(
                        set.is_none(),
                        "`set = <number` can only exist once per attribute"
                    );
                    set = Some(match name_value.lit {
                        Lit::Int(int) => int.base10_parse().unwrap(),
                        _ => panic!("`set = <number>`"),
                    });
                } else if name_value.path.is_ident("binding") {
                    assert!(
                        binding.is_none(),
                        "`binding = <number` can only exist once per attribute"
                    );
                    binding = Some(match name_value.lit {
                        Lit::Int(int) => int.base10_parse().unwrap(),
                        _ => panic!("`binding = <number>`"),
                    });
                } else {
                    panic!("attribute must be formatted as `#[uniform(set = <number>, binding = <number>)]`")
                }
            }

            let set = set.unwrap();
            let binding = binding.unwrap();

            if !set_binding_set.insert((set, binding)) {
                panic!("you cannot have multiple items with the same `set` and `binding`.")
            }

            (set, binding)
        };

        let shader_resource = field
            .attrs
            .iter()
            .find_map(|attr| {
                let meta = attr.parse_meta().ok()?;

                let meta_list = match meta {
                    Meta::List(meta_list) => meta_list,
                    _ => return None,
                };

                if meta_list.path.is_ident("uniform") {
                    let (set, binding) = parse_set_binding(meta_list);

                    Some(quote! {(
                        Cow::from(#field_name_string),
                        ShaderBinding { set: #set, binding: #binding },
                        ShaderResource::Uniform(&self.#field_name),
                    )})
                } else if meta_list.path.is_ident("buffer") {
                    let (set, binding) = parse_set_binding(meta_list);

                    Some(quote! {(
                        Cow::from(#field_name_string),
                        ShaderBinding { set: #set, binding: #binding },
                        ShaderResource::Buffer(self.#field_name),
                    )})
                } else if meta_list.path.is_ident("texture") {
                    let (set, binding) = parse_set_binding(meta_list);

                    Some(quote! {(
                        Cow::from(#field_name_string),
                        ShaderBinding { set: #set, binding: #binding },
                        ShaderResource::Texture(self.#field_name),
                    )})
                } else {
                    None
                }
            })
            .expect(&format!(
                "field `{}` must have a shader resource attribute",
                field_name
            ));

        shader_resources.push(shader_resource);
    }

    let code = quote! {
        #[allow(bad_style)]
        const _: () = {
            extern crate bevy_render_prototype as _bevy_render;
            use std::borrow::Cow;
            use _bevy_render::shader_resources::{ShaderBinding, ShaderResource};

            impl _bevy_render::shader_resources::ShaderResources for #struct_name {
                fn shader_resources(&self) -> Vec<(Cow<str>, ShaderBinding, ShaderResource)> {
                    vec![#(
                        #shader_resources,
                    )*]
                }
                fn shader_specialization(&self) -> Vec<(Cow<str>, bool)> {
                    unimplemented!("shader specialization")
                }
            }
        };
    };

    Ok(code)
}
