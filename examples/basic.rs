#![no_std]
#![no_main]

use core::cell::RefCell;

use critical_section::Mutex;
use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Output, Level, /*OutputConfig*/}
    //main
};
#[cfg(feature = "esp-hal-next")]
use esp_hal::{gpio::OutputConfig, time::Rate};
#[cfg(not(feature = "esp-hal-0_22"))]
use esp_hal::main;
#[cfg(feature = "esp-hal-0_22")]
use esp_hal::{entry as main};

#[cfg(feature = "esp-println")]
use esp_println::println;
#[cfg(feature = "defmt")]
use {
    defmt::{info as println, assert},
    defmt_rtt as _
};

static I2C: Mutex<RefCell<Option<esp_hal::i2c::master::I2c<esp_hal::Blocking>>>> =
    Mutex::new(RefCell::new(None));
static DELAY: Mutex<RefCell<Option<Delay>>> = Mutex::new(RefCell::new(None));

const ESP32_C3: bool = ! cfg!(target_has_atomic = "8");

#[main]
fn main() -> ! {
    //R #[cfg(feature="esp-println")]
    //R esp_println::logger::init_logger_from_env();

    let peripherals = esp_hal::init(esp_hal::Config::default());

    let delay = Delay::new();

    let i2c = {
        let xx = esp_hal::i2c::master::I2c::new(
            peripherals.I2C0, {
                let x = esp_hal::i2c::master::Config::default();
                #[cfg(feature="esp-hal-next")]
                let x = x.with_frequency( Rate::from_khz(1000) );     // Note: ESP32-C{36} only run up to 400 kHz (right?)
                x
            });

        #[cfg(not(feature = "esp-hal-0_22"))]
        let xx = xx.unwrap();

        if ESP32_C3 {
            xx.with_sda(peripherals.GPIO4)
                .with_scl(peripherals.GPIO5)
        } else {
            xx.with_sda(peripherals.GPIO18)
                .with_scl(peripherals.GPIO19)
        }
    };

    // SATEL board: reset by bringing 'PWR_EN' momentarily down
    {
        use esp_hal::gpio::AnyPin;
        let pin: AnyPin = if ESP32_C3 { peripherals.GPIO6.into() } else { peripherals.GPIO21.into() };

        #[allow(non_snake_case)]
        #[cfg(feature="esp-hal-next")]
        let mut PWR_EN = Output::new(pin, Level::Low, OutputConfig::default());
        #[allow(non_snake_case)]
        #[cfg(not(feature="esp-hal-next"))]
        let mut PWR_EN = Output::new(pin, Level::Low);

        PWR_EN.set_low();
        blocking_delay_ms(10);      // 10ms based on UM2884 (PDF; 18pp) Rev. 6, Chapter 4.2
        PWR_EN.set_high();
        println!("SATEL board powered off and on again.");
    }

    critical_section::with(|cs| {
        I2C.borrow_ref_mut(cs).replace(i2c);
        DELAY.borrow_ref_mut(cs).replace(delay);
    });

    unsafe {
        let mut p_dev = vl53l5::VL53L5CX_Configuration {
            platform: vl53l5::VL53L5CX_Platform { foo: 0 },
            streamcount: 0,
            data_read_size: 0,
            default_configuration: core::ptr::null_mut(),
            default_xtalk: core::ptr::null_mut(),
            offset_data: [0u8; 488],
            xtalk_data: [0u8; 776],
            temp_buffer: [0u8; 1452],
            is_auto_stop_enabled: 0,
        };

        let mut alive = 0u8;
        let status = vl53l5::vl53l5cx_is_alive(&mut p_dev as *mut _, &mut alive as *mut _);
        println!("alive = {} {}", status, alive);

        let status = vl53l5::vl53l5cx_init(&mut p_dev as *mut _);
        println!("init {}", status);

        let status = vl53l5::vl53l5cx_is_alive(&mut p_dev as *mut _, &mut alive as *mut _);
        println!("alive = {} {}", status, alive);

        let status = vl53l5::vl53l5cx_start_ranging(&mut p_dev as *mut _);
        println!("start ranging {}", status);

        let mut _loop = 0;
        let mut isReady = 0u8;
        let mut Results = vl53l5::VL53L5CX_ResultsData {
            silicon_temp_degc: 0i8,
            ambient_per_spad: [0u32; 64],
            nb_target_detected: [0u8; 64],
            nb_spads_enabled: [0u32; 64],
            signal_per_spad: [0u32; 64],
            range_sigma_mm: [0u16; 64],
            distance_mm: [0i16; 64],
            reflectance: [0u8; 64],
            target_status: [0u8; 64],
            motion_indicator: vl53l5::C2RustUnnamed {
                global_indicator_1: 0,
                global_indicator_2: 0,
                status: 0,
                nb_of_detected_aggregates: 0,
                nb_of_aggregates: 0,
                spare: 0,
                motion: [0u32; 32],
            },
        };

        while _loop < 10 {
            /* Use polling function to know when a new measurement is ready.
             * Another way can be to wait for HW interrupt raised on PIN A3
             * (GPIO 1) when a new measurement is ready */

            let _status =
                vl53l5::vl53l5cx_check_data_ready(&mut p_dev as *mut _, &mut isReady as *mut _);
            //println!("polling: {} {}", _status, isReady);

            const VL53L5CX_NB_TARGET_PER_ZONE: usize = 1;

            if isReady != 0 {
                vl53l5::vl53l5cx_get_ranging_data(&mut p_dev as *mut _, &mut Results as *mut _);

                /* As the sensor is set in 4x4 mode by default, we have a total
                 * of 16 zones to print. For this example, only the data of first zone are
                 * print */
                println!("Print data no : {}", p_dev.streamcount);
                for i in 0..16 {
                    println!(
                        "Zone : {}, Status : {}, Distance : {} mm\n",
                        i,
                        Results.target_status[VL53L5CX_NB_TARGET_PER_ZONE * i],
                        Results.distance_mm[VL53L5CX_NB_TARGET_PER_ZONE * i]
                    );
                }
                println!("\n");
                // _loop += 1;
            }

            /* Wait a few ms to avoid too high polling (function in platform
             * file, not in API) */
            WaitMs(&mut p_dev.platform as *mut _, 5);
        }

        let _status = vl53l5::vl53l5cx_stop_ranging(&mut p_dev as *mut _);

        println!("End of ULD demo");
    }

    loop {}
}

const ADDRESS: u8 = 0x29;

#[no_mangle]
extern "C" fn RdByte(
    p_platform: *mut vl53l5::VL53L5CX_Platform,
    register_adress: u16,
    p_value: *mut u8,
) -> u8 {
    critical_section::with(|cs| {
        let reg = register_adress.to_be_bytes();

        let mut i2c = I2C.borrow_ref_mut(cs);
        let i2c = i2c.as_mut().unwrap();

        let mut buffer = [0u8; 1];
        i2c.write(ADDRESS, &[reg[0], reg[1]]).unwrap();
        i2c.read(ADDRESS, &mut buffer).unwrap();

        WaitMs(p_platform, 1);

        unsafe {
            *p_value = buffer[0];
        }
        0
    })
}

#[no_mangle]
extern "C" fn WrByte(
    p_platform: *mut vl53l5::VL53L5CX_Platform,
    register_adress: u16,
    value: u8,
) -> u8 {
    critical_section::with(|cs| {
        let reg = register_adress.to_be_bytes();

        let mut i2c = I2C.borrow_ref_mut(cs);
        let i2c = i2c.as_mut().unwrap();

        let buffer = [reg[0], reg[1], value];

        i2c.write(ADDRESS, &buffer).unwrap();
        WaitMs(p_platform, 1);

        //     log::info!("wrote reg {} -> {}", RegisterAdress, value);
        0
    })
}

#[no_mangle]
extern "C" fn RdMulti(
    _p_platform: *mut vl53l5::VL53L5CX_Platform,
    register_adress: u16,
    p_values: *mut u8,
    size: u32,
) -> u8 {
    critical_section::with(|cs| {
        let reg = register_adress.to_be_bytes();

        let mut i2c = I2C.borrow_ref_mut(cs);
        let i2c = i2c.as_mut().unwrap();

        let rdata = unsafe { core::slice::from_raw_parts_mut(p_values, size as usize) };
        i2c.write_read(ADDRESS, &reg, rdata).unwrap();
    });
    0
}

#[no_mangle]
extern "C" fn WrMulti(
    _p_platform: *mut vl53l5::VL53L5CX_Platform,
    register_adress: u16,
    p_values: *mut u8,
    size: u32,
) -> u8 {
    critical_section::with(|cs| {
        let reg = register_adress.to_be_bytes();

        let mut i2c = I2C.borrow_ref_mut(cs);
        let i2c = i2c.as_mut().unwrap();

        let data = unsafe { core::slice::from_raw_parts_mut(p_values, size as usize) };

        #[cfg(not(all()))]  // original; using a buffer
        {
            let mut wdata = [0u8; 32770];
            wdata[0..][..2].copy_from_slice(&reg);
            wdata[2..][..data.len()].copy_from_slice(data);
            i2c.write(ADDRESS, &wdata[..(2 + data.len())]).unwrap();
        }
        #[cfg(all())]   // write-write transaction (not directly supported by 'esp-hal' API)
                        // based on esp-hal 'hil-test/tests/i2c.rs'
        {
            use esp_hal::i2c::master::Operation;
            i2c.transaction(ADDRESS,
        &mut [Operation::Write(&reg), Operation::Write(&data)]
            ).unwrap();
        }
    });

    0
}

// NOTE: Vendor docs don't really describe what the "4-byte grouping" means, but their 'protocol.c'
//      comments provide the details.
//
/// @brief Swap each 4-byte grouping, pointed to by 'buffer', so that ABCD becomes DCBA.
/// @param (uint8_t*) buf : Buffer to swap
/// @param (uint16_t) size : Buffer size in bytes; always multiple of 4.
#[no_mangle]
pub extern "C" fn SwapBuffer(buf: *mut u8, size: u16 /*size in bytes; not words*/) {
    // Note: Since we don't actually _know_, whether 'buffer' is 4-byte aligned (to be used as '*mut u32'),
    // The original doc mentions a blurry "generally uint32_t" (not very helpful).
    //
    assert!(
        buf as usize % 4 <= 0,
        "Buffer to swap byte order not 'u32' aligned"
    ); // '<= 0' to avoid an IDE warning

    let words: usize = (size as usize) / 4;
    let s: &mut [u32] = unsafe { core::slice::from_raw_parts_mut(buf as *mut u32, words) };

    for i in 0..words {
        s[i] = u32::swap_bytes(s[i]);
    }
}

#[no_mangle]
extern "C" fn WaitMs(_p_platform: *mut vl53l5::VL53L5CX_Platform, time_ms: u32) -> u8 {
    critical_section::with(|cs| {
        DELAY
            .borrow_ref_mut(cs)
            .as_mut()
            .unwrap()
            .delay_millis(time_ms);
    });
    0
}

// There should not be a reason to keep 'DELAY' a mutex-protected shared, is there?
const D_PROVIDER: Delay = Delay::new();

fn blocking_delay_ms(ms: u32) {
    D_PROVIDER.delay_millis(ms);
}
