# vl53l5-c2rust

Setting up logging while using the VL53L5 time-of-flight sensor.

Four logging variations on two ESP32 MCUs.

- `espflash-println`: using `println!` macros; run using `espflash --monitor`
- `espflash-log`: using `{debug|info|warn|...}!` macros; run using `espflash --monitor`
- `espflash-defmt`: using `{debug|info|warn|...}!` macros; run using `espflash monitor --log-format defmt`
- `probe_rs-defmt`: using `{debug|info|warn|...}!` macros; run using `probe-rs`


## Requirements

- ESP32-C3 devkit
- SATEL evaluation board
- patience!

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

## Selection of the MCU

Run either:

```
$ sh/set-c3.sh
$ sh/set-c6.sh
```


## Run

Contents of the `main` branch are set up for targetting ESP32-C3.

### `espflash-println`: `esp-println` and `println!`

- [ ] connect the devkit to either USB/UART or USB/JTAG port
- [ ] run

   ```
   $ cargo run --release --features=esp-hal-next --example basic
   ```

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

### `espflash-log`: `esp-println` and `{debug|info|warn|...}!`

- [ ] connect the devkit to either USB/UART or USB/JTAG port
- [ ] run

   ```
   $ cargo run --release --no-default-features --features=esp-hal-next,espflash-log --example basic
   ```

Sample output:

```
[...]
INFO - Print data no : 4
INFO - Zone : 0, Status : 5, Distance : 1777 mm

INFO - Zone : 1, Status : 5, Distance : 1776 mm

INFO - Zone : 2, Status : 5, Distance : 1776 mm

INFO - Zone : 3, Status : 5, Distance : 1778 mm

INFO - Zone : 4, Status : 5, Distance : 1782 mm

INFO - Zone : 5, Status : 5, Distance : 1778 mm

INFO - Zone : 6, Status : 5, Distance : 1780 mm
[...]
```

>Note: Coloring is by the line, e.g. all `INFO` lines are <font color=green>green</font>.

<p />

>Note: The `ESP_LOG` env.var. in `.cargo/config.toml` *should* now be usable for setting the logging level (but this didn't work for the author). tbd.


### `espflash-defmt`

- [ ] connect the devkit to either USB/UART or USB/JTAG port
- [ ] build

   ```
   $ cargo build --release --no-default-features --features=esp-hal-next,espflash-defmt --example basic
   ```

- [ ] run

	```
	$ espflash flash --log-format defmt --monitor target/riscv32imc-unknown-none-elf/release/examples/basic
	```

Output:

```
[...]
9.304684 INFO Zone : 0, Status : 4, Distance : 1796 mm

9.304828 INFO Zone : 1, Status : 4, Distance : 1777 mm

9.304955 INFO Zone : 2, Status : 255, Distance : 83 mm

9.305079 INFO Zone : 3, Status : 255, Distance : 82 mm

9.305206 INFO Zone : 4, Status : 4, Distance : 1781 mm

9.305332 INFO Zone : 5, Status : 4, Distance : 1774 mm

9.305461 INFO Zone : 6, Status : 255, Distance : 90 mm

9.305580 INFO Zone : 7, Status : 255, Distance : 90 mm

[...]
```

This is pretty nice. We get time stamps, log levels (colored only on the `INFO`, not the whole lines).


### `probe_rs-defmt`

- [ ] connect the devkit to the USB/JTAG port
- [ ] build

   ```
   $ cargo build --release --no-default-features --features=esp-hal-next,probe_rs-defmt --example basic
   ```

- [ ] run

   ```
   $ probe-rs run "--log-format={{t:dimmed} [{L:bold}]} {s}  {{c} {ff}:{l:1}%dimmed}" target/riscv32imc-unknown-none-elf/release/examples/basic
	```

Running fails with:

```
Finished in 5.05s
0.223258 [INFO ] SATEL board powered off and on again.  basic examples/fmt.rs:150
0.227887 [INFO ] alive = 0 1  basic examples/fmt.rs:150
0.490474 [ERROR] ====================== PANIC ======================  esp_backtrace src/lib.rs:25
0.490497 [ERROR] panicked at examples/basic.rs:270:14  esp_backtrace src/lib.rs:25
0.490601 [ERROR] Backtrace:  esp_backtrace src/lib.rs:25
0.490671 [ERROR] 0x42000c8e  esp_backtrace src/lib.rs:25
```

>Note: `probe-rs` provides nicer coloring than `espflash`, and the log format can be fine tuned.

Unfortunately, `probe-rs` (0.26.0) does not work with `esp-hal` I2C access (likely the reason behind the panic; founded guess, not proven here..).



## Same with ESP32-C6...?

If you wish to try on another MCU:

```
$ sed -i '' -e s/riscv32imc-unknown-none-elf/riscv32imac-unknown-none-elf/ .cargo/config.toml 
```

```
$ sed -i '' -e s/esp32c3/esp32c6/ Cargo.toml
```


## Summary

At the moment, the VL53L5 device is usable only with a narrow combination of devkits, logging choices, and MCUs:


|devkit|`esp-hal`|`espflash-println`|`espflash-log`|`espflash-defmt`|`probe_rs-defmt`|
|---|---|---|---|---|---|
|**<nobr>ESP32-C3-DevkitC-02</nobr>**|
||`main` (latest; moving target)|✅|✅|✅|❌|
||`0.23.1`|✅|||*not tested*|
||`0.23.0`|✅|||*not tested*|
||`0.22.0`|✅|||*not tested*|
|**ESP32-C6-Devkit-M1**|
||`main` (latest; moving target)|❌ scanning does not start; error 255|||❌|
||`0.23.1`|❌ <!--was: Scanning starts (0), but never reaches `data_ready` state;-->`init 0`, `alive=0 1`, then `AcknowledgeCheckFailed`|||❌ Does not start; no output|
||`0.23.0`|❌ *as 0.23.1*|||❌ Does not start; no output|
||`0.22.0`|❌ *as 0.23.1*|||❌ Does not start; no output|

<!-- Using:

$ espflash --version
espflash 3.3.0

$ probe-rs --version
probe-rs 0.26.0 (git commit: 4fd36e2)
-->

### Next steps

- [ ] Fix the build problems; complete the table

- [ ] Repeat the ESP32-C6 results with another devkit / breadboard.

<!--
	`bjoernQ` reports things [work for him](https://github.com/bjoernQ/vl53l5-c2rust/issues/1#issuecomment-2635855632)
-->
