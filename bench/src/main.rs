use proc_macro2::TokenStream;
use std::{fs, path::Path, str::FromStr, time::Instant};

const HELP: &str = "Please run with `--release` flag for accurate results.
Example:
    > cargo run --release --package bench
    > cargo r -rp bench
";

const ITERS: &[usize] = &[1, 10, 100, 1000];
const PROGRAMS: &[&str] = &["simple.rs", "user.rs"];

fn main() {
    if cfg!(debug_assertions) {
        println!("{HELP}");
    }

    let data_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("data");

    for iter in ITERS.iter().copied() {
        println!("\nIter: {iter}");

        for program in PROGRAMS {
            let content = fs::read_to_string(data_dir.join(program)).unwrap();
            let input = TokenStream::from_str(&content).unwrap();
            {
                let time = Instant::now();
                for _ in 0..iter {
                    let output = serde_derive_impl::derive_serialize(input.clone());
                    assert!(!output.is_empty());
                }
                let time_end = time.elapsed();
                println!("[serde_derive: {program}] serialize: {:?}", time_end);
            }

            // {
            //     let time = Instant::now();
            //     for _ in 0..iter {
            //         let output = serde_derive_impl::derive_deserialize(input.clone());
            //         assert!(!output.is_empty());
            //     }
            //     let time_end = time.elapsed();
            //     println!("[serde_derive: {program}] deserialize: {:#?}", time_end);
            // }

            // --------------------------------------------------------------------------

            {
                let time = Instant::now();
                for _ in 0..iter {
                    let output = serde_derive_next_impl::derive_serialize(input.clone());
                    assert!(!output.is_empty());
                }
                let time_end = time.elapsed();
                println!("[serde_derive_next: {program}] serialize: {:?}", time_end);
            }

            // {
            //     let time = Instant::now();
            //     for _ in 0..iter {
            //         let output = serde_derive_next_impl::derive_deserialize(input.clone());
            //         assert!(!output.is_empty());
            //     }
            //     let time_end = time.elapsed();
            //     println!(
            //         "[serde_derive_next: {program}] deserialize: {:#?}",
            //         time_end
            //     );
            // }
            println!();
        }
    }
}
