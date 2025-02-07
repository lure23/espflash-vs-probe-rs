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

    #[cfg(all(feature="esp-hal-next", feature="esp-hal-0_22"))]
    compile_error!("Cannot have both 'esp-hal-next' and 'esp-hal-0_22' enabled.");

    // Link arguments
    {
        #[cfg(feature="defmt")]
        println!("cargo::rustc-link-arg={}", "-Tdefmt.x");
    }
}
