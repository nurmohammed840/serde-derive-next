use crate::internals::ast::{Container, Data, Field, Style, Variant};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

// Suppress dead_code warnings that would otherwise appear when using a remote
// derive. Other than this pretend code, a struct annotated with remote derive
// never has its fields referenced and an enum annotated with remote derive
// never has its variants constructed.
//
//     warning: field is never used: `i`
//      --> src/main.rs:4:20
//       |
//     4 | struct StructDef { i: i32 }
//       |                    ^^^^^^
//
//     warning: variant is never constructed: `V`
//      --> src/main.rs:8:16
//       |
//     8 | enum EnumDef { V }
//       |                ^
//
pub fn pretend_used(cont: &Container, is_packed: bool) -> TokenStream {
    let pretend_fields = pretend_fields_used(cont, is_packed);
    let pretend_variants = pretend_variants_used(cont);

    quote! {
        #pretend_fields
        #pretend_variants
    }
}

// For structs with named fields, expands to:
//
//     match None::<&T> {
//         Some(T { a: __v0, b: __v1 }) => {}
//         _ => {}
//     }
//
// For packed structs on sufficiently new rustc, expands to:
//
//     match None::<&T> {
//         Some(__v @ T { a: _, b: _ }) => {
//             let _ = addr_of!(__v.a);
//             let _ = addr_of!(__v.b);
//         }
//         _ => {}
//     }
//
// For packed structs on older rustc, we assume Sized and !Drop, and expand to:
//
//     match None::<T> {
//         Some(T { a: __v0, b: __v1 }) => {}
//         _ => {}
//     }
//
// For enums, expands to the following but only including struct variants:
//
//     match None::<&T> {
//         Some(T::A { a: __v0 }) => {}
//         Some(T::B { b: __v0 }) => {}
//         _ => {}
//     }
//
fn pretend_fields_used(cont: &Container, is_packed: bool) -> TokenStream {
    match &cont.data {
        Data::Enum(variants) => pretend_fields_used_enum(cont, variants),
        Data::Struct(Style::Struct | Style::Tuple | Style::Newtype, fields) => {
            if is_packed {
                pretend_fields_used_struct_packed(cont, fields)
            } else {
                pretend_fields_used_struct(cont, fields)
            }
        }
        Data::Struct(Style::Unit, _) => quote!(),
    }
}

fn pretend_fields_used_struct(cont: &Container, fields: &[Field]) -> TokenStream {
    let type_ident = &cont.ident;
    let (_, ty_generics, _) = cont.generics.split_for_impl();

    let members = fields.iter().map(|field| &field.member);
    let placeholders = (0usize..).map(|i| format_ident!("__v{}", i));

    quote! {
        match _serde::__private::None::<&#type_ident #ty_generics> {
            _serde::__private::Some(#type_ident { #(#members: #placeholders),* }) => {}
            _ => {}
        }
    }
}

fn pretend_fields_used_struct_packed(cont: &Container, fields: &[Field]) -> TokenStream {
    let type_ident = &cont.ident;
    let (_, ty_generics, _) = cont.generics.split_for_impl();

    let members = fields.iter().map(|field| &field.member).collect::<Vec<_>>();

    quote! {
        match _serde::__private::None::<&#type_ident #ty_generics> {
            _serde::__private::Some(__v @ #type_ident { #(#members: _),* }) => {
                #(
                    let _ = _serde::__private::ptr::addr_of!(__v.#members);
                )*
            }
            _ => {}
        }
    }
}

fn pretend_fields_used_enum(cont: &Container, variants: &[Variant]) -> TokenStream {
    let type_ident = &cont.ident;
    let (_, ty_generics, _) = cont.generics.split_for_impl();

    let patterns = variants
        .iter()
        .filter_map(|variant| match variant.style {
            Style::Struct | Style::Tuple | Style::Newtype => {
                let variant_ident = &variant.ident;
                let members = variant.fields.iter().map(|field| &field.member);
                let placeholders = (0usize..).map(|i| format_ident!("__v{}", i));
                Some(quote!(#type_ident::#variant_ident { #(#members: #placeholders),* }))
            }
            Style::Unit => None,
        })
        .collect::<Vec<_>>();

    quote! {
        match _serde::__private::None::<&#type_ident #ty_generics> {
            #(
                _serde::__private::Some(#patterns) => {}
            )*
            _ => {}
        }
    }
}

// Expands to one of these per enum variant:
//
//     match None {
//         Some((__v0, __v1,)) => {
//             let _ = E::V { a: __v0, b: __v1 };
//         }
//         _ => {}
//     }
//
fn pretend_variants_used(cont: &Container) -> TokenStream {
    let variants = match &cont.data {
        Data::Enum(variants) => variants,
        Data::Struct(_, _) => {
            return quote!();
        }
    };

    let type_ident = &cont.ident;
    let (_, ty_generics, _) = cont.generics.split_for_impl();
    let turbofish = ty_generics.as_turbofish();

    let cases = variants.iter().map(|variant| {
        let variant_ident = &variant.ident;
        let placeholders = &(0..variant.fields.len())
            .map(|i| format_ident!("__v{}", i))
            .collect::<Vec<_>>();

        let pat = match variant.style {
            Style::Struct => {
                let members = variant.fields.iter().map(|field| &field.member);
                quote!({ #(#members: #placeholders),* })
            }
            Style::Tuple | Style::Newtype => quote!(( #(#placeholders),* )),
            Style::Unit => quote!(),
        };

        quote! {
            match _serde::__private::None {
                _serde::__private::Some((#(#placeholders,)*)) => {
                    let _ = #type_ident::#variant_ident #turbofish #pat;
                }
                _ => {}
            }
        }
    });

    quote!(#(#cases)*)
}

pub mod __ {
    #![allow(dead_code)]

    use crate::{
        internals::ast::{Container, Data, Field, Style, Variant},
        quote_fn,
    };
    use proc_macro2::TokenStream;
    use quote::format_ident;
    use quote2::{quote, Quote};

    // Suppress dead_code warnings that would otherwise appear when using a remote
    // derive. Other than this pretend code, a struct annotated with remote derive
    // never has its fields referenced and an enum annotated with remote derive
    // never has its variants constructed.
    //
    //     warning: field is never used: `i`
    //      --> src/main.rs:4:20
    //       |
    //     4 | struct StructDef { i: i32 }
    //       |                    ^^^^^^
    //
    //     warning: variant is never constructed: `V`
    //      --> src/main.rs:8:16
    //       |
    //     8 | enum EnumDef { V }
    //       |                ^
    //
    pub fn pretend_used<'a>(cont: &'a Container, is_packed: bool) -> quote_fn!(type 'a) {
        quote(move |t| {
            pretend_fields_used(t, cont, is_packed);
            pretend_variants_used(t, cont);
        })
    }

    // For structs with named fields, expands to:
    //
    //     match None::<&T> {
    //         Some(T { a: __v0, b: __v1 }) => {}
    //         _ => {}
    //     }
    //
    // For packed structs on sufficiently new rustc, expands to:
    //
    //     match None::<&T> {
    //         Some(__v @ T { a: _, b: _ }) => {
    //             let _ = addr_of!(__v.a);
    //             let _ = addr_of!(__v.b);
    //         }
    //         _ => {}
    //     }
    //
    // For packed structs on older rustc, we assume Sized and !Drop, and expand to:
    //
    //     match None::<T> {
    //         Some(T { a: __v0, b: __v1 }) => {}
    //         _ => {}
    //     }
    //
    // For enums, expands to the following but only including struct variants:
    //
    //     match None::<&T> {
    //         Some(T::A { a: __v0 }) => {}
    //         Some(T::B { b: __v0 }) => {}
    //         _ => {}
    //     }
    //
    fn pretend_fields_used(t: &mut TokenStream, cont: &Container, is_packed: bool) {
        match &cont.data {
            Data::Enum(variants) => pretend_fields_used_enum(t, cont, variants),
            Data::Struct(Style::Struct | Style::Tuple | Style::Newtype, fields) => {
                if is_packed {
                    pretend_fields_used_struct_packed(t, cont, fields)
                } else {
                    pretend_fields_used_struct(t, cont, fields)
                }
            }
            Data::Struct(Style::Unit, _) => {}
        }
    }

    fn pretend_fields_used_struct(t: &mut TokenStream, cont: &Container, fields: &[Field]) {
        let type_ident = &cont.ident;
        let (_, ty_generics, _) = cont.generics.split_for_impl();

        let placeholders = placeholders(fields);

        quote!(t, {
            match _serde::__private::None::<&#type_ident #ty_generics> {
                _serde::__private::Some(#type_ident { #placeholders }) => {}
                _ => {}
            }
        });
    }

    fn pretend_fields_used_struct_packed(t: &mut TokenStream, cont: &Container, fields: &[Field]) {
        let type_ident = &cont.ident;
        let (_, ty_generics, _) = cont.generics.split_for_impl();

        let members = quote(|t| {
            for Field { member, .. } in fields {
                quote!(t, { #member: _ , });
            }
        });
        let members_body = quote(|t| {
            for Field { member, .. } in fields {
                quote!(t, {
                    let _ = _serde::__private::ptr::addr_of!(__v.#member);
                });
            }
        });

        quote!(t, {
            match _serde::__private::None::<& #type_ident #ty_generics> {
                _serde::__private::Some(__v @ #type_ident { #members }) => {
                    #members_body
                }
                _ => {}
            }
        });
    }

    fn pretend_fields_used_enum(t: &mut TokenStream, cont: &Container, variants: &[Variant]) {
        let type_ident = &cont.ident;
        let (_, ty_generics, _) = cont.generics.split_for_impl();

        let patterns = quote(|t| {
            for variant in variants {
                if let Style::Struct | Style::Tuple | Style::Newtype = variant.style {
                    let variant_ident = &variant.ident;
                    let placeholders = placeholders(&variant.fields);
                    quote!(t, {
                        _serde::__private::Some( #type_ident :: #variant_ident { #placeholders } ) => {}
                    });
                }
            }
        });

        quote!(t, {
            match _serde::__private::None::<& #type_ident #ty_generics> {
                #patterns
                _ => {}
            }
        });
    }

    // Expands to one of these per enum variant:
    //
    //     match None {
    //         Some((__v0, __v1,)) => {
    //             let _ = E::V { a: __v0, b: __v1 };
    //         }
    //         _ => {}
    //     }
    //
    fn pretend_variants_used(t: &mut TokenStream, cont: &Container) {
        let variants = match &cont.data {
            Data::Enum(variants) => variants,
            Data::Struct(_, _) => {
                return;
            }
        };

        let type_ident = &cont.ident;
        let (_, ty_generics, _) = cont.generics.split_for_impl();
        let turbofish = &ty_generics.as_turbofish();

        for variant in variants {
            let variant_ident = &variant.ident;
            let placeholders: Vec<_> = variant
                .fields
                .iter()
                .enumerate()
                .map(|(i, field)| (&field.member, format_ident!("__v{}", i)))
                .collect();

            let sep_placeholders = quote(|t| {
                for (_, placeholder) in &placeholders {
                    t.add_tokens(placeholder);
                    t.add_punct(',');
                }
            });

            let pat = quote(|t| match variant.style {
                Style::Struct => {
                    for (member, placeholder) in &placeholders {
                        quote!(t, { #member: #placeholder, });
                    }
                }
                Style::Tuple | Style::Newtype => {
                    quote!(t, { (#sep_placeholders) });
                }
                Style::Unit => {}
            });

            quote!(t, {
                match _serde::__private::None {
                    _serde::__private::Some((#sep_placeholders)) => {
                        let _ = #type_ident :: #variant_ident #turbofish #pat;
                    }
                    _ => {}
                }
            });
        }
    }

    fn placeholders<'a>(fields: &'a [Field]) -> quote_fn!(type 'a) {
        quote(move |t| {
            for (i, Field { member, .. }) in fields.iter().enumerate() {
                let placeholder = format_ident!("__v{}", i);
                quote!(t, { #member: #placeholder, });
            }
        })
    }
}
