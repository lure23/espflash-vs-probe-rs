mod vl53l5;

fn main() {
    println!("Hello, world!");
}

#[no_mangle]
extern "C" fn RdByte(
    p_platform: *mut vl53l5::VL53L5CX_Platform,
    RegisterAdress: libc::uint16_t,
    p_value: *mut libc::uint8_t,
) -> libc::uint8_t {
    todo!()
}

#[no_mangle]
extern "C" fn WrByte(
    p_platform: *mut vl53l5::VL53L5CX_Platform,
    RegisterAdress: libc::uint16_t,
    value: libc::uint8_t,
) -> libc::uint8_t {
    todo!()
}

#[no_mangle]
extern "C" fn RdMulti(
    p_platform: *mut vl53l5::VL53L5CX_Platform,
    RegisterAdress: libc::uint16_t,
    p_values: *mut libc::uint8_t,
    size: libc::uint32_t,
) -> libc::uint8_t {
    todo!()
}

#[no_mangle]
extern "C" fn WrMulti(
    p_platform: *mut vl53l5::VL53L5CX_Platform,
    RegisterAdress: libc::uint16_t,
    p_values: *mut libc::uint8_t,
    size: libc::uint32_t,
) -> libc::uint8_t {
    todo!()
}

#[no_mangle]
extern "C" fn SwapBuffer(buffer: *mut libc::uint8_t, size: libc::uint16_t) {
    todo!()
}

#[no_mangle]
extern "C" fn WaitMs(p_platform: *mut vl53l5::VL53L5CX_Platform, TimeMs: libc::uint32_t) -> libc::uint8_t {
    todo!()
}

