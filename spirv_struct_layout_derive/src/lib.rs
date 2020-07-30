extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Fields};

#[proc_macro_derive(SpirvLayout)]
pub fn spirv_layout_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let repr_check = ensure_repr(&input.attrs);
    let body = build_function_body(&input.data);

    let expanded = quote! {
        impl #impl_generics spirv_struct_layout::CheckSpirvStruct for #name
            #ty_generics #where_clause {

            fn check_spirv_layout(name: &str, spirv: Vec<u32>) {
                #repr_check

                let spv: spirq::SpirvBinary = spirv.into();
                let entries = spv.reflect().expect("Failed to parse SPIR-V");

                let buffer_desc = entries[0].resolve_desc(spirq::sym::Sym::new(name)).expect(format!("Failed to find symbol with name \"{}\"", name));

                let mut _rust_offset = 0;

                #body
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

fn ensure_repr(attrs: &Vec<Attribute>) -> TokenStream {
    for attr in attrs {
        if let Ok(meta) = attr.parse_meta() {
            if meta.path().is_ident("repr") {
                return quote! {};
            }
        }
    }

    quote! { compile_error!("structs exposed to SPIRV must have a declared repr"); }
}

fn build_function_body(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let inner = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    let ty = &f.ty;
                    quote_spanned! {
                        f.span() => {
                            {
                                let symbol = stringify!(#name);
                                let rust_size = std::mem::size_of::<#ty>();

                                if let Some(desc) = buffer_desc.desc_ty.resolve(spirq::sym::Sym::new(&symbol)) {
                                    let spirv_offset = desc.offset;
                                    let spirv_size = desc.ty.nbyte().expect(format!("Rust struct field named \"{}\" does not have a basic data type (float, vec, mat, array, struct) as a SPIR-V counterpart"));

                                    assert_eq!(
                                        spirv_size, rust_size,
                                        "field {} should be {} bytes, but was {} bytes",
                                        symbol, spirv_size, rust_size
                                    );
                                    assert_eq!(
                                        spirv_offset, _rust_offset,
                                        "field {} should have an offset of {} bytes, but was {} bytes",
                                        symbol, spirv_offset, _rust_offset
                                    );
                                }

                                _rust_offset += rust_size;
                            }
                        }
                    }
                });
                quote! {
                    #(#inner)*
                }
            }
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}
