extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_derive(Serialize, attributes(serde))]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
    serde_derive_impl::derive_serialize(input.into()).into()
}

#[proc_macro_derive(Deserialize, attributes(serde))]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {
    serde_derive_impl::derive_deserialize(input.into()).into()
}
