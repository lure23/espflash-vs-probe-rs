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

    // Link arguments
    {
        #[cfg(feature="_defmt")]
        println!("cargo::rustc-link-arg={}", "-Tdefmt.x");
    }
}
