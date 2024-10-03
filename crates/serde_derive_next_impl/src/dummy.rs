use proc_macro2::TokenStream;
use quote::quote;

pub fn wrap_in_const(serde_path: Option<&syn::Path>, code: TokenStream) -> TokenStream {
    let use_serde = match serde_path {
        Some(path) => quote! {
            use #path as _serde;
        },
        None => quote! {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
        },
    };

    quote! {
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #use_serde
            #code
        };
    }
}

mod __ {
    #![allow(dead_code)]
    use proc_macro2::TokenStream;
    use quote2::{quote, Quote};

    pub fn wrap_in_const(serde_path: Option<&syn::Path>, code: TokenStream) -> TokenStream {
        let use_serde = quote(|t| match serde_path {
            Some(path) => {
                quote!(t, {
                    use #path as _serde;
                });
            }
            None => {
                quote!(t, {
                    #[allow(unused_extern_crates, clippy::useless_attribute)]
                    extern crate serde as _serde;
                });
            }
        });

        let mut t = TokenStream::new();
        quote!(t, {
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                #use_serde
                #code
            };
        });
        t
    }
}
