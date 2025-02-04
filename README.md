# vl53l5-c2rust

To help debug VL53L5 time-of-flight sensor use, in Rust.

## Requirements

- ESP32-C3 devkit
- SATEL evaluation board

## Wiring

|MCU|Satel|other/comments|
|---|---|---|---|
|GND|1. GND|
|3v3|2. 3v3|
|5V|3. 5V|
|--|4. PWR_EN|via ~47kΩ to +3v3|
|--|5. --|*pulled up by Satel*|
|GPIO2|6. SCL|
|GPIO1|7. SDA|
|--|8. --||
|--|9. --||

## Run

### default (ESP32-C3; esp-println)

```
$ cargo run --release --example basic
```

- Flashes and shows output via `espflash flash`
- Connect a USB cable to the `USB/UART` port of your ESP32 devkit

Sample output:

```
[...]
Print data no : 23
Zone : 0, Status : 5, Distance : 1741 mm

Zone : 1, Status : 5, Distance : 1785 mm

Zone : 2, Status : 4, Distance : 1785 mm

Zone : 3, Status : 5, Distance : 1850 mm

Zone : 4, Status : 5, Distance : 1748 mm

Zone : 5, Status : 4, Distance : 1762 mm
[...]
```

### `defmt`

For `defmt`, we do separate build and run steps. 

```
$ cargo build --release --no-default-features --features=defmt --example basic
$ probe-rs run target/riscv32imc-unknown-none-elf/release/examples/basic
```

- Connect a USB cable to the `USB/JTAG` port of your ESP32 devkit (optional for ESP32-C3 devkit).

Sample output:

```
      Erasing ✔ 100% [####################] 256.00 KiB @ 270.13 KiB/s (took 1s)
  Programming ✔ 100% [####################] 106.66 KiB @  27.81 KiB/s (took 4s)                                                                                                                    Finished in 4.90s
ERROR ====================== PANIC ======================
└─ esp_backtrace::panic_handler @ /home/ubuntu/.cargo/git/checkouts/esp-hal-42ec44e8c6943228/fe53061/esp-backtrace/src/lib.rs:25  
ERROR panicked at examples/basic.rs:223:56:
called `Result::unwrap()` on an `Err` value: Timeout
└─ esp_backtrace::panic_handler @ /home/ubuntu/.cargo/git/checkouts/esp-hal-42ec44e8c6943228/fe53061/esp-backtrace/src/lib.rs:25  
ERROR Backtrace:
└─ esp_backtrace::panic_handler @ /home/ubuntu/.cargo/git/checkouts/esp-hal-42ec44e8c6943228/fe53061/esp-backtrace/src/lib.rs:25  
ERROR 0x4200098c
└─ esp_backtrace::panic_handler @ /home/ubuntu/.cargo/git/checkouts/esp-hal-42ec44e8c6943228/fe53061/esp-backtrace/src/lib.rs:25  
ERROR 0x4200366e
└─ esp_backtrace::panic_handler @ /home/ubuntu/.cargo/git/checkouts/esp-hal-42ec44e8c6943228/fe53061/esp-backtrace/src/lib.rs:25  
ERROR 0x42000f5c
└─ esp_backtrace::panic_handler @ /home/ubuntu/.cargo/git/checkouts/esp-hal-42ec44e8c6943228/fe53061/esp-backtrace/src/lib.rs:25  
ERROR 0x42006b6a
└─ esp_backtrace::panic_handler @ /home/ubuntu/.cargo/git/checkouts/esp-hal-42ec44e8c6943228/fe53061/esp-backtrace/src/lib.rs:25  
ERROR 0x42000132
└─ esp_backtrace::panic_handler @ /home/ubuntu/.cargo/git/checkouts/esp-hal-42ec44e8c6943228/fe53061/esp-backtrace/src/lib.rs:25  
```

