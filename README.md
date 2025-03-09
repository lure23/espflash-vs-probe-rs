# espflash vs. probe-rs

There are implications to selecting the logging strategy for embedded ESP32 Rust projects. The strategies are not clearly defined, or documented. It's kind of like a Wild West.

Below the author places them in four categories.

The repo examines the compatibility of each of the strategies with the ST.com [VL53L5CX](https://www.st.com/en/imaging-and-photonics-solutions/vl53l5cx.html) time-of-flight sensor.

>Why VL53L5CX??? Because the sensor('s use of I2C bus) has some compatibility issues with `probe-rs`, which lead the author down this 🐰🕳️ to begin with.
 
## Four logging strategies

### `espflash-println`

Uses `println!` macros; run using `espflash --monitor`; depends on: [`esp-println`]()

### `espflash-log`

Uses `{debug|info|warn|...}!` macros; run using `espflash --monitor`; depends on: [`log`](), [`esp-println`]()

### `espflash-defmt`

Uses `{debug|info|warn|...}!` macros; run using `espflash monitor --log-format defmt`; depends on: [`defmt`](), [`esp-println`]()

### `probe_rs-defmt`

Uses `{debug|info|warn|...}!` macros; run using `probe-rs`; depends on: [`defmt`](), [`defmt-rtt`]()


## Requirements

- ESP32-C3 or ESP32-C6 devkit
- SATEL evaluation board
- patience!

For getting the results below, the setup was:

- Raspberry Pi acting as host

	```
	$ espflash --version
	espflash 3.3.0
	```

	```
	$ probe-rs --version
	probe-rs 0.26.0 (git commit: 4fd36e2)
	```

	```
	$ uname -a
	Linux rpi 6.1.21-v7+ #1642 SMP Mon Apr  3 17:20:52 BST 2023 armv7l GNU/Linux
	```

	```
	$ lsb_release -a
	[...]
	Description:   Raspbian GNU/Linux 11 (bullseye)
	```

## Wiring

|ESP32-C3 pins<sup>`**`</sup>|Satel|other/comments|
|---|---|---|
|GND|1. GND|
|3v3|2. 3v3|
|5V|3. 5V|
|GPIO6 <sub>(GPIO21)</sub>|4. PWR_EN|Alternatively, via ~47kΩ to +3v3|
|--|5. --|*pulled up by Satel*|
|GPIO5 <sub>(GPIO19)</sub>|6. SCL|
|GPIO4 <sub>(GPIO18)</sub>|7. SDA|
|--|8. --||
|--|9. --||

>`|**|`: ESP32-C6 pins are denoted in parantheses. Note that the order of `SDA` and `SCL` is different than in the code...


## Run

The code is set up for targetting ESP32-C3. If you are using ESP32-C6, run this first:

```
$ sh/set-c6.sh
```


### `espflash-println`: `esp-println` and `println!`

- [x] connect the devkit to either USB/UART or USB/JTAG port
- [x] run

   ```
   $ cargo run --release --features esp-hal-beta0 --example basic
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

>On ESP32-C6:
>
>```
>[...]
>SATEL board powered off and on again.
>alive = 0 1
>init 0
>alive = 0 1
>start ranging 255
>```
>
>This means the scanning did not start. `255` is a generic error code (tbd. **look closer to it**)

### `espflash-log`: `esp-println` and `{debug|info|warn|...}!`

- [x] connect the devkit to either USB/UART or USB/JTAG port
- [x] run

   ```
   $ cargo run --release --no-default-features --features=esp-hal-beta0,espflash-log --example basic
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

>Note: Coloring is by the line, e.g. <font color=green>all `INFO` lines are green</font>.

<p />

>Note: The `ESP_LOG` env.var. in `.cargo/config.toml` *should* now be usable for setting the logging level (but this didn't work for the author). tbd.

<p />

>On ESP32-C6:
>
>```
>INFO - SATEL board powered off and on again.
>INFO - alive = 0 1
>INFO - init 0
>INFO - alive = 0 1
>
>
>====================== PANIC ======================
>panicked at examples/basic.rs:209:46:
>called `Result::unwrap()` on an `Err` value: ArbitrationLost
>
>
>```
<!-- empty lines left on purpose -->


### `espflash-defmt`

- [x] connect the devkit to either USB/UART or USB/JTAG port
- [x] build

   ```
   $ cargo build --release --no-default-features --features=esp-hal-beta0,espflash-defmt --example basic
   ```

- [x] run

	```
	$ espflash flash --log-format defmt --monitor $(sh/target-dir.sh)/riscv32imc-unknown-none-elf/release/examples/basic
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

We get time stamps, log levels (colored only on the <font color=green>`INFO`</font>, not the whole lines).

>On ESP32-C6:
>
>```
>[...]
>0.169209 INFO SATEL board powered off and on again.
>0.173741 INFO alive = 0 1
>1.400483 INFO init 0
>1.404769 INFO alive = 0 1
>1.460476 ERROR ====================== PANIC ======================
>1.461334 ERROR panicked at examples/basic.rs:209:46
>1.461492 ERROR Backtrace:
>1.461596 ERROR 0x42000372
>0x42000372 - core::result::Result<T,E>::unwrap
>    at /rustc/e71f9a9a98b0faf423844bf0ba7438f29dc27d58/library/core/src/result.rs:1104
>1.461725 ERROR 0x42003f56
>0x42003f56 - vl53l5::_vl53l5cx_poll_for_answer
>    at /home/ubuntu/espflash-vs-probe_rs/src/lib.rs:123
>1.461852 ERROR 0x4200097a
>0x4200097a - main
>    at /home/ubuntu/espflash-vs-probe_rs/examples/basic.rs:105
>1.461977 ERROR 0x420068a0
>0x420068a0 - hal_main
>    at /home/ubuntu/.cargo/git/checkouts/esp-hal-42ec44e8c6943228/392d5cc/esp-hal/src/lib.rs:422
>```
>
>i.e. panics at `_vl53l5cx_poll_for_answer()`

### `probe_rs-defmt`

- [x] connect the devkit to the USB/JTAG port
- [x] build

   ```
   $ cargo build --release --no-default-features --features=esp-hal-beta0,probe_rs-defmt --example basic
   ```

- [x] run

   ```
   $ probe-rs run "--log-format={{t:dimmed} [{L:bold}]} {s}  {{c} {ff}:{l:1}%dimmed}" $(sh/target-dir.sh)/riscv32imc-unknown-none-elf/release/examples/basic
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

Unfortunately, `probe-rs` (0.26.0) does not work with `esp-hal` I2C access (long story, documented [elsewhere](https://github.com/probe-rs/probe-rs/issues/2818#issuecomment-2358791448)).

>On ESP32-C6:
>
>```
>0.202341 [INFO ] SATEL board powered off and on again.  basic examples/fmt.rs:150
>0.206864 [INFO ] alive = 0 1  basic examples/fmt.rs:150
>1.433658 [INFO ] init 0  basic examples/fmt.rs:150
>1.437950 [INFO ] alive = 0 1  basic examples/fmt.rs:150
>1.493739 [ERROR] ====================== PANIC ======================  esp_backtrace src/lib.rs:25
>1.493758 [ERROR] panicked at examples/basic.rs:209:46  esp_backtrace src/lib.rs:25
>```


## Summary

At the moment, the VL53L5CX device is usable only with a narrow combination of devkits, logging choices, and MCUs:


|devkit|`esp-hal`|`espflash-println`|`espflash-log`|`espflash-defmt`|`probe_rs-defmt`|
|---|---|---|---|---|---|
|**<nobr>ESP32-C3-DevkitC-02</nobr>**|
||`main` (latest; moving target)|✅|✅|✅ + nice logging|❌|
|**ESP32-C6-Devkit-M1**|
||`main` (latest; moving target)|❌ scanning does not start; error 255|❌ panic: `ArbitrationLost `|❌ panics at `_vl53l5cx_poll_for_answer`|❌ panic|
||`0.23.1`|❌ *as above*|||❌ Does not start; no output|
||`0.23.0`|❌ *as above*|||❌ Does not start; no output|

<!-- Using:

```
$ espflash --version
espflash 3.3.0

$ probe-rs --version
probe-rs 0.26.0 (git commit: 4fd36e2)
```
-->

<!-- hidden
### Next steps

- [ ] Repeat the ESP32-C6 results with another devkit / breadboard.
-->

<!--
### Footnotes

- `bjoernQ` reports things [work for him](https://github.com/bjoernQ/vl53l5-c2rust/issues/1#issuecomment-2635855632) on ESP32-C6
-->

## Footnotes...

Getting "no error" (`0`) from the `init` DOES NOT seem to mean things went well.

```
alive = 0 1
init 0
alive = 0 1
```

- When successful, getting to `init 0` takes some time
- When things fail, it's pretty instantaneous
