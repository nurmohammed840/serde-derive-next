extern crate proc_macro;
use proc_macro::TokenStream;

use std::{fmt::Write, io, time::Instant};

fn is_real_bench() -> bool {
    std::env::var("REAL_BENCH").is_ok_and(|val| val == "1")
}

fn write_file(name: &str, data: &str) -> io::Result<()> {
    let path = std::env::current_dir()?.join(name);
    std::fs::write(path, data.as_bytes())
}

#[proc_macro_derive(Serialize, attributes(serde))]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
    if is_real_bench() {
        // let raw_input = input.to_string();

        let time = Instant::now();
        let out: TokenStream = serde_derive_next_impl::derive_serialize(input.into()).into();
        let elapsed = time.elapsed();

        let mut output = String::new();
        let _ = writeln!(output, "Time: {:?}", elapsed);
        // let _ = writeln!(output, "Input: {}", raw_input);
        // let _ = writeln!(output, "Output: {}", out.to_string());
        // let _ = writeln!(output, "--------------------------------");
        let _ = write_file("serde_derive_next.ser-bench.log", &output);
        out
    } else {
        serde_derive_next_impl::derive_serialize(input.into()).into()
    }
}

#[proc_macro_derive(Deserialize, attributes(serde))]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {
    serde_derive_next_impl::derive_deserialize(input.into()).into()
}
