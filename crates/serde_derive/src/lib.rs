extern crate proc_macro;

use proc_macro::TokenStream;
use std::{str::FromStr, time::Instant};

fn is_real_bench() -> bool {
    matches!(std::env::var("REAL_BENCH").as_deref(), Ok("1"))
}

#[proc_macro_derive(Serialize, attributes(serde))]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
    if is_real_bench() {
        let time = Instant::now();
        let out: TokenStream = serde_derive_impl::derive_serialize(input.into()).into();
        let elapsed = time.elapsed();

        let mut res = TokenStream::from_str(&format!("///Time: {elapsed:?}")).unwrap();
        res.extend(out);
        res
    } else {
        serde_derive_impl::derive_serialize(input.into()).into()
    }
}

#[proc_macro_derive(Deserialize, attributes(serde))]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {
    serde_derive_impl::derive_deserialize(input.into()).into()
}
