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

pub mod __ {
    use crate::quote_into;
    use proc_macro2::TokenStream;
    use quote::ToTokens;
    use quote2::{quote, Quote};

    pub fn wrap_in_const(serde_path: Option<&syn::Path>, code: impl ToTokens) -> TokenStream {
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

        quote_into! {
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                #use_serde
                #code
            };
        }
    }
}
