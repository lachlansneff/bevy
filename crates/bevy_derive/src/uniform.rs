use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Error, Fields, Result};

pub fn derive_uniform(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
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

    let mut aligned_fields = Vec::new();
    let mut created_fields = Vec::new();

    for field in fields.named {
        let name = field.ident.unwrap();
        let ty = field.ty;

        let aligned_name = format_ident!("_align_{}", name);
        aligned_fields.push(quote! {
            #aligned_name: <#ty as _bevy_render::shader_resources::IntoStd140>::Align,
            #name: <#ty as _bevy_render::shader_resources::IntoStd140>::Ty,
        });

        created_fields.push(quote! {
            #aligned_name: <#ty as _bevy_render::shader_resources::IntoStd140>::Align::default(),
            #name: self.#name.into_std140(),
        });
    }

    let code = quote! {
        #[allow(bad_style)]
        const _: () = {
            extern crate bevy_render_prototype as _bevy_render;
            use _bevy_render::shader_resources::IntoStd140 as _;
            #[repr(C, align(16))]
            struct X {#(
                #aligned_fields
            )*}

            impl _bevy_render::shader_resources::Uniform for #struct_name {
                fn copy_padded_to_slice(&self, s: &mut [u8]) {
                    let x = X {#(
                        #created_fields
                    )*};

                    let size = core::mem::size_of::<X>();
                    let bytes = unsafe {
                        core::slice::from_raw_parts(
                            &x as *const X as *const u8,
                            size,
                        )
                    };

                    s[..size].copy_from_slice(bytes);
                }
                fn padded_size(&self) -> usize {
                    core::mem::size_of::<X>()
                }
            }
        };
    };

    Ok(code)
}
