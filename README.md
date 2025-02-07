# vl53l5-c2rust

To help debug VL53L5 time-of-flight sensor use, in Rust.

## Requirements

- ESP32-C3 devkit
- SATEL evaluation board

## Wiring

|ESP32-C3 pins|Satel|other/comments|
|---|---|---|
|GND|1. GND|
|3v3|2. 3v3|
|5V|3. 5V|
|GPIO6|4. PWR_EN|Alternatively, via ~47kΩ to +3v3|
|--|5. --|*pulled up by Satel*|
|GPIO4|6. SCL|
|GPIO5|7. SDA|
|--|8. --||
|--|9. --||

>For ESP32-C6 pins, see the source.

## Run

Contents of the `main` branch are set up for targetting ESP32-C3.

### default (ESP32-C3; esp-println)

```
$ cargo run --release --features=esp-hal-next --example basic
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

### `defmt` (ESP32-C3)

For `defmt`, we do separate build and run steps. 

```
$ cargo build --release --no-default-features --features=defmt,esp-hal-next --example basic
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

### Change to ESP32-C6

>Commands on macOS `bash`.

```
$ sed -i '' -e s/riscv32imc-unknown-none-elf/riscv32imac-unknown-none-elf/ .cargo/config.toml 
```

```
$ sed -i '' -e s/esp32c3/esp32c6/ Cargo.toml
```

Now we can proceed building and running the code on ESP32-C6.


### `esp-println` (ESP32-C6)

```
$ cargo run --release --features=esp-hal-next --example basic
```

- Flashes and shows output via `espflash flash`
- Connect a USB cable to the `USB/UART` port of your ESP32 devkit

Sample output:

```
[...]
alive = 0 1
init 0
init done
alive = 0 1
start ranging 255

[...]
```

The **255** is an error code indicating the vendor driver could not start a scan. 

Tested on two separate sensors, one known to work on C3.


### `defmt` (ESP32-C6)

For `defmt`, we do separate build and run steps. 

```
$ cargo build --release --no-default-features --features=defmt,esp-hal-next --example basic
$ probe-rs run target/riscv32imac-unknown-none-elf/release/examples/basic
```

- Connect a USB cable to the `USB/JTAG` port of your ESP32 devkit (optional for ESP32-C3 devkit).

Sample output:

```
      Erasing ✔ 100% [####################] 256.00 KiB @ 237.80 KiB/s (took 1s)
  Programming ✔ 100% [####################] 108.13 KiB @  38.61 KiB/s (took 3s)                                                                                                                    Finished in 3.94s
ERROR ====================== PANIC ======================
└─ esp_backtrace::panic_handler @ /home/ubuntu/.cargo/git/checkouts/esp-hal-42ec44e8c6943228/fe53061/esp-backtrace/src/lib.rs:25
ERROR panicked at examples/basic.rs:178:37:
called `Result::unwrap()` on an `Err` value: ArbitrationLost
└─ esp_backtrace::panic_handler @ /home/ubuntu/.cargo/git/checkouts/esp-hal-42ec44e8c6943228/fe53061/esp-backtrace/src/lib.rs:25
```

## Summary

At the moment, the VL53L5 device is usable only with a narrow combination of devkits (or MCUs), and output features.


|devkit|`esp-hal`|`esp-println`|`defmt`<sup>`[2]`</sup>|comments|
|---|---|---|---|---|
|**<nobr>ESP32-C3-DevkitC-02</nobr>**|
||`main` (latest; moving target)|✅|❌|
||`0.23.1`|✅|*not tested*|
||`0.23.0`|✅|*not tested*|
||`0.22.0`|✅|*not tested*|
|**ESP32-C6-Devkit-M1**|
||`main` (latest; moving target)|❌ scanning does not start; error 255|❌|
||`0.23.1`|❌ <!--was: Scanning starts (0), but never reaches `data_ready` state;-->`init 0`, `alive=0 1`, then `AcknowledgeCheckFailed`|❌ Does not start; no output|
||`0.23.0`|❌ *as 0.23.1*|❌ Does not start; no output|
||`0.22.0`|❌ *as 0.23.1*|❌ Does not start; no output|

<small>`[2]`: `defmt` + `defmt-rtt`</small>

<!-- Using:

$ espflash --version
espflash 3.3.0

$ probe-rs --version
probe-rs 0.26.0 (git commit: 4fd36e2)
-->

### Next steps

- [x] One could see, how much the used `esp-hal` version (latest `main`, as of 4-Feb-25) affects this.

- [x] Repeat the ESP32-C6 results with another user / devkit / breadboard.

	`bjoernQ` reports things [work for him](https://github.com/bjoernQ/vl53l5-c2rust/issues/1#issuecomment-2635855632)

