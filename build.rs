/*
* build.rs
*/

fn main() {
    // DEBUG: Show what we know about the compilation.
    //
    //  <<
    //   CARGO_CFG_TARGET_FEATURE=c,m
    //   CARGO_FEATURE_{..feature..}=1
    //   RUSTUP_TOOLCHAIN=stable-x86_64-unknown-linux-gnu
    //   TARGET=riscv32imc-unknown-none-elf
    //  <<
    //
    #[cfg(not(all()))]
    {
        std::env::vars().for_each(|(a, b)| { eprintln!("{a}={b}"); });
        std::process::exit(1);
    }

    {
        let count= 0;
        #[cfg(feature="espflash-println")]
        let count = count + 1;
        #[cfg(feature="espflash-log")]
        let count = count + 1;
        #[cfg(feature="espflash-defmt")]
        let count = count + 1;
        #[cfg(feature="probe_rs-defmt")]
        let count = count + 1;

        assert!(count > 0, "One 'espflash-*' or 'probe_rs-*' feature must be enabled");
        assert!(count == 1, "Only one 'espflash-*' or 'probe_rs-*' feature must be enabled");
    }

    // Expose the target path up to (excluding) '/build'
    //
    // 'OUT_DIR' is e.g. "/home/ubuntu/target/riscv32imac-unknown-none-elf/release/build/espflash-vs-d8e98ffe9238a676/out"
    //      - we don't need the tail
    //      - we _can_ get the head ('/home/ubuntu/target') from 'cargo metadata' (but not the target type)
    //
    // Thus the easiest is simply to pre-prepare the whole path, as it will be needed, here.
    // Only 'defmt' uses need this, because there we have separate build and run steps.
    //
    #[cfg(feature="_defmt")]
    {
        use std::{env, fs};
        const TMP: &str = ".OUT_DIR";

        let out_dir = env::var("OUT_DIR")
            .expect("OUT_DIR to have a value");

        // Cut at "/build/..."; user adds 'examples/basic'
        //
        use regex::Regex;

        #[allow(non_snake_case)]
        let RE_TAIL: Regex = Regex::new(r"/build/.+").unwrap();
        assert_eq!(RE_TAIL.replace("abc/build/def", ""), "abc");

        let s = RE_TAIL.replace(out_dir.as_str(), "") .to_string();

        fs::write(TMP, s)
            .expect(format!("Unable to write {TMP}").as_str());
    }

    // Link arguments
    {
        #[cfg(feature="_defmt")]
        println!("cargo::rustc-link-arg={}", "-Tdefmt.x");
    }
}
