use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashSet;
use syn::{
    parse_macro_input, Attribute, Data, DeriveInput, Error, Fields, Lit, LitStr, Meta, NestedMeta,
    Result,
};

enum Kind {
    Resource(TokenStream),
    SpecializeDefine(TokenStream),
}

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
    let mut define_specializations = Vec::new();

    for field in fields.named {
        let field_name = field.ident.unwrap();
        let field_name_string = format!("{}", field_name);

        let mut parse_set_binding = |attr: &Attribute| {
            let meta = attr.parse_meta().unwrap();

            let meta_list = match meta {
                Meta::List(meta_list) => meta_list,
                _ => panic!("attribute must have arguments"),
            };

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

        let mut kinds = field
            .attrs
            .iter()
            .filter_map(|attr| {
                if attr.path.is_ident("uniform") {
                    let (set, binding) = parse_set_binding(attr);

                    Some(Kind::Resource(quote! {(
                        Cow::from(#field_name_string),
                        ShaderBinding { set: #set, binding: #binding },
                        ShaderResource::Uniform(&self.#field_name),
                    )}))
                } else if attr.path.is_ident("buffer") {
                    let (set, binding) = parse_set_binding(attr);

                    Some(Kind::Resource(quote! {(
                        Cow::from(#field_name_string),
                        ShaderBinding { set: #set, binding: #binding },
                        ShaderResource::Buffer(self.#field_name),
                    )}))
                } else if attr.path.is_ident("texture") {
                    let (set, binding) = parse_set_binding(attr);

                    Some(Kind::Resource(quote! {(
                        Cow::from(#field_name_string),
                        ShaderBinding { set: #set, binding: #binding },
                        ShaderResource::Texture(self.#field_name),
                    )}))
                } else if attr.path.is_ident("specialize_define") {
                    if let Ok(lit_str) = attr.parse_args::<LitStr>() {
                        Some(Kind::SpecializeDefine(quote! {(
                            Cow::from(#lit_str),
                            self.#field_name,
                        )}))
                    } else if attr.tokens.is_empty() {
                        Some(Kind::SpecializeDefine(quote! {(
                            Cow::from(#field_name_string),
                            self.#field_name,
                        )}))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        let kind = if kinds.is_empty() {
            return Err(Error::new(
                field_name.span(),
                &format!("`{}` must have a ShaderResource attribute", field_name),
            ));
        } else if kinds.len() > 1 {
            return Err(Error::new(
                field_name.span(),
                &format!("`{}` can only have a single attribute", field_name),
            ));
        } else {
            kinds.remove(0)
        };

        match kind {
            Kind::Resource(tokens) => {
                shader_resources.push(tokens);
            }
            Kind::SpecializeDefine(tokens) => {
                define_specializations.push(tokens);
            }
        }
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
                    vec![#(
                        #define_specializations,
                    )*]
                }
            }
        };
    };

    Ok(code)
}
