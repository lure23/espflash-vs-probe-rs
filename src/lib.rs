#![no_std]
#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use c2rust_bitfields::BitfieldStruct;

pub mod consts;

type ulong = u32;

extern "C" {
    fn memcpy(_: *mut (), _: *const (), _: ulong) -> *mut ();
    fn memset(_: *mut (), _: i32, _: ulong) -> *mut ();
    fn RdByte(
        p_platform: *mut VL53L5CX_Platform,
        RegisterAdress: uint16_t,
        p_value: *mut uint8_t,
    ) -> uint8_t;
    fn WrByte(
        p_platform: *mut VL53L5CX_Platform,
        RegisterAdress: uint16_t,
        value: uint8_t,
    ) -> uint8_t;
    fn RdMulti(
        p_platform: *mut VL53L5CX_Platform,
        RegisterAdress: uint16_t,
        p_values: *mut uint8_t,
        size: uint32_t,
    ) -> uint8_t;
    fn WrMulti(
        p_platform: *mut VL53L5CX_Platform,
        RegisterAdress: uint16_t,
        p_values: *mut uint8_t,
        size: uint32_t,
    ) -> uint8_t;
    fn SwapBuffer(buffer: *mut uint8_t, size: uint16_t);
    fn WaitMs(p_platform: *mut VL53L5CX_Platform, TimeMs: uint32_t) -> uint8_t;
}
pub type int8_t = i8;
pub type int16_t = i16;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VL53L5CX_Platform {
    pub foo: u32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VL53L5CX_Configuration {
    pub platform: VL53L5CX_Platform,
    pub streamcount: uint8_t,
    pub data_read_size: uint32_t,
    pub default_configuration: *mut uint8_t,
    pub default_xtalk: *mut uint8_t,
    pub offset_data: [uint8_t; 488],
    pub xtalk_data: [uint8_t; 776],
    pub temp_buffer: [uint8_t; 1452],
    pub is_auto_stop_enabled: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VL53L5CX_ResultsData {
    pub silicon_temp_degc: int8_t,
    pub ambient_per_spad: [uint32_t; 64],
    pub nb_target_detected: [uint8_t; 64],
    pub nb_spads_enabled: [uint32_t; 64],
    pub signal_per_spad: [uint32_t; 64],
    pub range_sigma_mm: [uint16_t; 64],
    pub distance_mm: [int16_t; 64],
    pub reflectance: [uint8_t; 64],
    pub target_status: [uint8_t; 64],
    pub motion_indicator: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub global_indicator_1: uint32_t,
    pub global_indicator_2: uint32_t,
    pub status: uint8_t,
    pub nb_of_detected_aggregates: uint8_t,
    pub nb_of_aggregates: uint8_t,
    pub spare: uint8_t,
    pub motion: [uint32_t; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union Block_header {
    pub bytes: uint32_t,
    pub c2rust_unnamed: C2RustUnnamed_0,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    #[bitfield(name = "type_0", ty = "uint32_t", bits = "0..=3")]
    #[bitfield(name = "size", ty = "uint32_t", bits = "4..=15")]
    #[bitfield(name = "idx", ty = "uint32_t", bits = "16..=31")]
    pub type_0_size_idx: [u8; 4],
}

unsafe extern "C" fn _vl53l5cx_poll_for_answer(
    mut p_dev: *mut VL53L5CX_Configuration,
    mut size: uint8_t,
    mut pos: uint8_t,
    mut address: uint16_t,
    mut mask: uint8_t,
    mut expected_value: uint8_t,
) -> uint8_t {
    let mut status: uint8_t = 0u8;
    let mut timeout: uint8_t = 0u8;
    loop {
        status = (status as i32
            | RdMulti(
                &mut (*p_dev).platform,
                address,
                ((*p_dev).temp_buffer).as_mut_ptr(),
                size as uint32_t,
            ) as i32) as uint8_t;
        status = (status as i32 | WaitMs(&mut (*p_dev).platform, 10 as i32 as uint32_t) as i32)
            as uint8_t;
        if timeout as i32 >= 200u8 as i32 {
            status = (status as i32 | 1u8 as i32) as uint8_t;
            break;
        } else if size as i32 >= 4u8 as i32
            && (*p_dev).temp_buffer[2 as i32 as usize] as i32 >= 0x7fu8 as i32
        {
            status = (status as i32 | 66u8 as i32) as uint8_t;
            break;
        } else {
            timeout = timeout.wrapping_add(1);
            if !((*p_dev).temp_buffer[pos as usize] as i32 & mask as i32 != expected_value as i32) {
                break;
            }
        }
    }
    return status;
}
unsafe extern "C" fn _vl53l5cx_poll_for_mcu_boot(
    mut p_dev: *mut VL53L5CX_Configuration,
) -> uint8_t {
    let mut go2_status0: uint8_t = 0;
    let mut go2_status1: uint8_t = 0;
    let mut status: uint8_t = 0u8;
    let mut timeout: uint16_t = 0 as i32 as uint16_t;
    loop {
        status = (status as i32
            | RdByte(
                &mut (*p_dev).platform,
                0x6 as i32 as uint16_t,
                &mut go2_status0,
            ) as i32) as uint8_t;
        if go2_status0 as i32 & 0x80u8 as i32 != 0u8 as i32 {
            status = (status as i32
                | RdByte(
                    &mut (*p_dev).platform,
                    0x7 as i32 as uint16_t,
                    &mut go2_status1,
                ) as i32) as uint8_t;
            status = (status as i32 | go2_status1 as i32) as uint8_t;
            break;
        } else {
            WaitMs(&mut (*p_dev).platform, 1 as i32 as uint32_t);
            timeout = timeout.wrapping_add(1);
            if go2_status0 as i32 & 0x1u8 as i32 != 0u8 as i32 {
                break;
            }
            if !((timeout as i32) < 500 as i32 as uint16_t as i32) {
                break;
            }
        }
    }
    return status;
}
unsafe extern "C" fn _vl53l5cx_send_offset_data(
    mut p_dev: *mut VL53L5CX_Configuration,
    mut resolution: uint8_t,
) -> uint8_t {
    let mut status: uint8_t = 0u8;
    let mut signal_grid: [uint32_t; 64] = [0; 64];
    let mut range_grid: [int16_t; 64] = [0; 64];
    let mut dss_4x4: [uint8_t; 8] = [0xfu8, 0x4u8, 0x4u8, 0u8, 0x8u8, 0x10u8, 0x10u8, 0x7u8];
    let mut footer: [uint8_t; 8] = [0u8, 0u8, 0u8, 0xfu8, 0x3u8, 0x1u8, 0x1u8, 0xe4u8];
    let mut i: int8_t = 0;
    let mut j: int8_t = 0;
    let mut k: uint16_t = 0;
    memcpy(
        ((*p_dev).temp_buffer).as_mut_ptr() as *mut (),
        ((*p_dev).offset_data).as_mut_ptr() as *const (),
        488 as u32 as uint16_t as ulong,
    );
    if resolution as i32 == 16u8 as i32 {
        memcpy(
            &mut *((*p_dev).temp_buffer)
                .as_mut_ptr()
                .offset(0x10 as i32 as isize) as *mut uint8_t as *mut (),
            dss_4x4.as_mut_ptr() as *const (),
            ::core::mem::size_of::<[uint8_t; 8]>() as ulong,
        );
        SwapBuffer(((*p_dev).temp_buffer).as_mut_ptr(), 488 as u32 as uint16_t);
        memcpy(
            signal_grid.as_mut_ptr() as *mut (),
            &mut *((*p_dev).temp_buffer)
                .as_mut_ptr()
                .offset(0x3c as i32 as isize) as *mut uint8_t as *const (),
            ::core::mem::size_of::<[uint32_t; 64]>() as ulong,
        );
        memcpy(
            range_grid.as_mut_ptr() as *mut (),
            &mut *((*p_dev).temp_buffer)
                .as_mut_ptr()
                .offset(0x140 as i32 as isize) as *mut uint8_t as *const (),
            ::core::mem::size_of::<[int16_t; 64]>() as ulong,
        );
        j = 0 as i32 as int8_t;
        while (j as i32) < 4 as i32 as int8_t as i32 {
            i = 0 as i32 as int8_t;
            while (i as i32) < 4 as i32 as int8_t as i32 {
                signal_grid[(i as i32 + 4 as i32 * j as i32) as usize] =
                    (signal_grid[(2 as i32 * i as i32
                        + 16 as i32 * j as i32
                        + 0 as i32 as int8_t as i32) as usize])
                        .wrapping_add(
                            signal_grid[(2 as i32 * i as i32
                                + 16 as i32 * j as i32
                                + 1 as i32 as int8_t as i32)
                                as usize],
                        )
                        .wrapping_add(
                            signal_grid[(2 as i32 * i as i32
                                + 16 as i32 * j as i32
                                + 8 as i32 as int8_t as i32)
                                as usize],
                        )
                        .wrapping_add(
                            signal_grid[(2 as i32 * i as i32
                                + 16 as i32 * j as i32
                                + 9 as i32 as int8_t as i32)
                                as usize],
                        )
                        .wrapping_div(4 as i32 as uint32_t);
                range_grid[(i as i32 + 4 as i32 * j as i32) as usize] = ((range_grid
                    [(2 as i32 * i as i32 + 16 as i32 * j as i32) as usize]
                    as i32
                    + range_grid[(2 as i32 * i as i32 + 16 as i32 * j as i32 + 1 as i32) as usize]
                        as i32
                    + range_grid[(2 as i32 * i as i32 + 16 as i32 * j as i32 + 8 as i32) as usize]
                        as i32
                    + range_grid[(2 as i32 * i as i32 + 16 as i32 * j as i32 + 9 as i32) as usize]
                        as i32)
                    / 4 as i32 as int16_t as i32)
                    as int16_t;
                i += 1;
            }
            j += 1;
        }
        memset(
            &mut *range_grid.as_mut_ptr().offset(0x10 as i32 as isize) as *mut int16_t as *mut (),
            0 as i32,
            96 as i32 as uint16_t as ulong,
        );
        memset(
            &mut *signal_grid.as_mut_ptr().offset(0x10 as i32 as isize) as *mut uint32_t as *mut (),
            0 as i32,
            192 as i32 as uint16_t as ulong,
        );
        memcpy(
            &mut *((*p_dev).temp_buffer)
                .as_mut_ptr()
                .offset(0x3c as i32 as isize) as *mut uint8_t as *mut (),
            signal_grid.as_mut_ptr() as *const (),
            ::core::mem::size_of::<[uint32_t; 64]>() as ulong,
        );
        memcpy(
            &mut *((*p_dev).temp_buffer)
                .as_mut_ptr()
                .offset(0x140 as i32 as isize) as *mut uint8_t as *mut (),
            range_grid.as_mut_ptr() as *const (),
            ::core::mem::size_of::<[int16_t; 64]>() as ulong,
        );
        SwapBuffer(((*p_dev).temp_buffer).as_mut_ptr(), 488 as u32 as uint16_t);
    }
    k = 0 as i32 as uint16_t;
    while (k as i32) < 488 as u32 as uint16_t as i32 - 4 as i32 as uint16_t as i32 {
        (*p_dev).temp_buffer[k as usize] =
            (*p_dev).temp_buffer[(k as i32 + 8 as i32 as uint16_t as i32) as usize];
        k = k.wrapping_add(1);
    }
    memcpy(
        &mut *((*p_dev).temp_buffer)
            .as_mut_ptr()
            .offset(0x1e0 as i32 as isize) as *mut uint8_t as *mut (),
        footer.as_mut_ptr() as *const (),
        8 as i32 as ulong,
    );
    status = (status as i32
        | WrMulti(
            &mut (*p_dev).platform,
            0x2e18 as i32 as uint16_t,
            ((*p_dev).temp_buffer).as_mut_ptr(),
            488 as u32 as uint16_t as uint32_t,
        ) as i32) as uint8_t;
    status = (status as i32
        | _vl53l5cx_poll_for_answer(p_dev, 4u8, 1u8, 0x2c00 as u32 as uint16_t, 0xffu8, 0x3u8)
            as i32) as uint8_t;
    return status;
}
unsafe extern "C" fn _vl53l5cx_send_xtalk_data(
    mut p_dev: *mut VL53L5CX_Configuration,
    mut resolution: uint8_t,
) -> uint8_t {
    let mut status: uint8_t = 0u8;
    let mut res4x4: [uint8_t; 8] = [0xfu8, 0x4u8, 0x4u8, 0x17u8, 0x8u8, 0x10u8, 0x10u8, 0x7u8];
    let mut dss_4x4: [uint8_t; 8] = [0u8, 0x78u8, 0u8, 0x8u8, 0u8, 0u8, 0u8, 0x8u8];
    let mut profile_4x4: [uint8_t; 4] = [0xa0u8, 0xfcu8, 0x1u8, 0u8];
    let mut signal_grid: [uint32_t; 64] = [0; 64];
    let mut i: int8_t = 0;
    let mut j: int8_t = 0;
    memcpy(
        ((*p_dev).temp_buffer).as_mut_ptr() as *mut (),
        &mut *((*p_dev).xtalk_data).as_mut_ptr().offset(0 as i32 as isize) as *mut uint8_t
            as *const (),
        776 as u32 as uint16_t as ulong,
    );
    if resolution as i32 == 16u8 as i32 {
        memcpy(
            &mut *((*p_dev).temp_buffer)
                .as_mut_ptr()
                .offset(0x8 as i32 as isize) as *mut uint8_t as *mut (),
            res4x4.as_mut_ptr() as *const (),
            ::core::mem::size_of::<[uint8_t; 8]>() as ulong,
        );
        memcpy(
            &mut *((*p_dev).temp_buffer)
                .as_mut_ptr()
                .offset(0x20 as i32 as isize) as *mut uint8_t as *mut (),
            dss_4x4.as_mut_ptr() as *const (),
            ::core::mem::size_of::<[uint8_t; 8]>() as ulong,
        );
        SwapBuffer(((*p_dev).temp_buffer).as_mut_ptr(), 776 as u32 as uint16_t);
        memcpy(
            signal_grid.as_mut_ptr() as *mut (),
            &mut *((*p_dev).temp_buffer)
                .as_mut_ptr()
                .offset(0x34 as i32 as isize) as *mut uint8_t as *const (),
            ::core::mem::size_of::<[uint32_t; 64]>() as ulong,
        );
        j = 0 as i32 as int8_t;
        while (j as i32) < 4 as i32 as int8_t as i32 {
            i = 0 as i32 as int8_t;
            while (i as i32) < 4 as i32 as int8_t as i32 {
                signal_grid[(i as i32 + 4 as i32 * j as i32) as usize] = (signal_grid
                    [(2 as i32 * i as i32 + 16 as i32 * j as i32 + 0 as i32) as usize])
                    .wrapping_add(
                        signal_grid
                            [(2 as i32 * i as i32 + 16 as i32 * j as i32 + 1 as i32) as usize],
                    )
                    .wrapping_add(
                        signal_grid
                            [(2 as i32 * i as i32 + 16 as i32 * j as i32 + 8 as i32) as usize],
                    )
                    .wrapping_add(
                        signal_grid
                            [(2 as i32 * i as i32 + 16 as i32 * j as i32 + 9 as i32) as usize],
                    )
                    .wrapping_div(4 as i32 as uint32_t);
                i += 1;
            }
            j += 1;
        }
        memset(
            &mut *signal_grid.as_mut_ptr().offset(0x10 as i32 as isize) as *mut uint32_t as *mut (),
            0 as i32,
            192 as i32 as uint32_t as ulong,
        );
        memcpy(
            &mut *((*p_dev).temp_buffer)
                .as_mut_ptr()
                .offset(0x34 as i32 as isize) as *mut uint8_t as *mut (),
            signal_grid.as_mut_ptr() as *const (),
            ::core::mem::size_of::<[uint32_t; 64]>() as ulong,
        );
        SwapBuffer(((*p_dev).temp_buffer).as_mut_ptr(), 776 as u32 as uint16_t);
        memcpy(
            &mut *((*p_dev).temp_buffer)
                .as_mut_ptr()
                .offset(0x134 as i32 as isize) as *mut uint8_t as *mut (),
            profile_4x4.as_mut_ptr() as *const (),
            ::core::mem::size_of::<[uint8_t; 4]>() as ulong,
        );
        memset(
            &mut *((*p_dev).temp_buffer)
                .as_mut_ptr()
                .offset(0x78 as i32 as isize) as *mut uint8_t as *mut (),
            0 as i32,
            (4 as i32 as uint32_t as ulong)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as ulong),
        );
    }
    status = (status as i32
        | WrMulti(
            &mut (*p_dev).platform,
            0x2cf8 as i32 as uint16_t,
            ((*p_dev).temp_buffer).as_mut_ptr(),
            776 as u32 as uint16_t as uint32_t,
        ) as i32) as uint8_t;
    status = (status as i32
        | _vl53l5cx_poll_for_answer(p_dev, 4u8, 1u8, 0x2c00 as u32 as uint16_t, 0xffu8, 0x3u8)
            as i32) as uint8_t;
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn vl53l5cx_is_alive(
    mut p_dev: *mut VL53L5CX_Configuration,
    mut p_is_alive: *mut uint8_t,
) -> uint8_t {
    let mut status: uint8_t = 0u8;
    let mut device_id: uint8_t = 0;
    let mut revision_id: uint8_t = 0;
    status = (status as i32 | WrByte(&mut (*p_dev).platform, 0x7fff as i32 as uint16_t, 0u8) as i32)
        as uint8_t;
    status = (status as i32
        | RdByte(&mut (*p_dev).platform, 0 as i32 as uint16_t, &mut device_id) as i32)
        as uint8_t;
    status = (status as i32
        | RdByte(
            &mut (*p_dev).platform,
            1 as i32 as uint16_t,
            &mut revision_id,
        ) as i32) as uint8_t;
    status = (status as i32
        | WrByte(&mut (*p_dev).platform, 0x7fff as i32 as uint16_t, 0x2u8) as i32)
        as uint8_t;
    if device_id as i32 == 0xf0u8 as i32 && revision_id as i32 == 0x2u8 as i32 {
        *p_is_alive = 1u8;
    } else {
        *p_is_alive = 0u8;
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn vl53l5cx_init(mut p_dev: *mut VL53L5CX_Configuration) -> uint8_t {
    let mut tmp: uint8_t = 0;
    let mut status: uint8_t = 0u8;
    let mut pipe_ctrl: [uint8_t; 4] = [1u8, 0u8, 0x1u8, 0u8];
    let mut single_range: uint32_t = 0x1 as i32 as uint32_t;
    let ref mut fresh0 = (*p_dev).default_xtalk;
    *fresh0 = consts::VL53L5CX_DEFAULT_XTALK.as_ptr() as *mut uint8_t;
    let ref mut fresh1 = (*p_dev).default_configuration;
    *fresh1 = consts::VL53L5CX_DEFAULT_CONFIGURATION.as_ptr() as *mut uint8_t;
    (*p_dev).is_auto_stop_enabled = 0u8;
    status = (status as i32 | WrByte(&mut (*p_dev).platform, 0x7fff as i32 as uint16_t, 0u8) as i32)
        as uint8_t;
    status = (status as i32 | WrByte(&mut (*p_dev).platform, 0x9 as i32 as uint16_t, 0x4u8) as i32)
        as uint8_t;
    status = (status as i32 | WrByte(&mut (*p_dev).platform, 0xf as i32 as uint16_t, 0x40u8) as i32)
        as uint8_t;
    status = (status as i32 | WrByte(&mut (*p_dev).platform, 0xa as i32 as uint16_t, 0x3u8) as i32)
        as uint8_t;
    status = (status as i32
        | RdByte(&mut (*p_dev).platform, 0x7fff as i32 as uint16_t, &mut tmp) as i32)
        as uint8_t;
    status = (status as i32 | WrByte(&mut (*p_dev).platform, 0xc as i32 as uint16_t, 0x1u8) as i32)
        as uint8_t;
    status = (status as i32 | WrByte(&mut (*p_dev).platform, 0x101 as i32 as uint16_t, 0u8) as i32)
        as uint8_t;
    status = (status as i32 | WrByte(&mut (*p_dev).platform, 0x102 as i32 as uint16_t, 0u8) as i32)
        as uint8_t;
    status = (status as i32
        | WrByte(&mut (*p_dev).platform, 0x10a as i32 as uint16_t, 0x1u8) as i32)
        as uint8_t;
    status = (status as i32
        | WrByte(&mut (*p_dev).platform, 0x4002 as i32 as uint16_t, 0x1u8) as i32)
        as uint8_t;
    status = (status as i32 | WrByte(&mut (*p_dev).platform, 0x4002 as i32 as uint16_t, 0u8) as i32)
        as uint8_t;
    status = (status as i32
        | WrByte(&mut (*p_dev).platform, 0x10a as i32 as uint16_t, 0x3u8) as i32)
        as uint8_t;
    status = (status as i32
        | WrByte(&mut (*p_dev).platform, 0x103 as i32 as uint16_t, 0x1u8) as i32)
        as uint8_t;
    status = (status as i32 | WrByte(&mut (*p_dev).platform, 0xc as i32 as uint16_t, 0u8) as i32)
        as uint8_t;
    status = (status as i32 | WrByte(&mut (*p_dev).platform, 0xf as i32 as uint16_t, 0x43u8) as i32)
        as uint8_t;
    status =
        (status as i32 | WaitMs(&mut (*p_dev).platform, 1 as i32 as uint32_t) as i32) as uint8_t;
    status = (status as i32 | WrByte(&mut (*p_dev).platform, 0xf as i32 as uint16_t, 0x40u8) as i32)
        as uint8_t;
    status = (status as i32 | WrByte(&mut (*p_dev).platform, 0xa as i32 as uint16_t, 0x1u8) as i32)
        as uint8_t;
    status =
        (status as i32 | WaitMs(&mut (*p_dev).platform, 100 as i32 as uint32_t) as i32) as uint8_t;
    status = (status as i32 | WrByte(&mut (*p_dev).platform, 0x7fff as i32 as uint16_t, 0u8) as i32)
        as uint8_t;
    status = (status as i32
        | _vl53l5cx_poll_for_answer(p_dev, 1u8, 0u8, 0x6 as i32 as uint16_t, 0xffu8, 1u8) as i32)
        as uint8_t;
    if !(status as i32 != 0u8 as i32) {
        status = (status as i32
            | WrByte(&mut (*p_dev).platform, 0xe as i32 as uint16_t, 0x1u8) as i32)
            as uint8_t;
        status = (status as i32
            | WrByte(&mut (*p_dev).platform, 0x7fff as i32 as uint16_t, 0x2u8) as i32)
            as uint8_t;
        status = (status as i32
            | WrByte(&mut (*p_dev).platform, 0x3 as i32 as uint16_t, 0xdu8) as i32)
            as uint8_t;
        status = (status as i32
            | WrByte(&mut (*p_dev).platform, 0x7fff as i32 as uint16_t, 0x1u8) as i32)
            as uint8_t;
        status = (status as i32
            | _vl53l5cx_poll_for_answer(p_dev, 1u8, 0u8, 0x21 as i32 as uint16_t, 0x10u8, 0x10u8)
                as i32) as uint8_t;
        status = (status as i32
            | WrByte(&mut (*p_dev).platform, 0x7fff as i32 as uint16_t, 0u8) as i32)
            as uint8_t;
        status = (status as i32
            | RdByte(&mut (*p_dev).platform, 0x7fff as i32 as uint16_t, &mut tmp) as i32)
            as uint8_t;
        status = (status as i32
            | WrByte(&mut (*p_dev).platform, 0xc as i32 as uint16_t, 0x1u8) as i32)
            as uint8_t;
        status = (status as i32
            | WrByte(&mut (*p_dev).platform, 0x7fff as i32 as uint16_t, 0u8) as i32)
            as uint8_t;
        status = (status as i32
            | WrByte(&mut (*p_dev).platform, 0x101 as i32 as uint16_t, 0u8) as i32)
            as uint8_t;
        status = (status as i32
            | WrByte(&mut (*p_dev).platform, 0x102 as i32 as uint16_t, 0u8) as i32)
            as uint8_t;
        status = (status as i32
            | WrByte(&mut (*p_dev).platform, 0x10a as i32 as uint16_t, 0x1u8) as i32)
            as uint8_t;
        status = (status as i32
            | WrByte(&mut (*p_dev).platform, 0x4002 as i32 as uint16_t, 0x1u8) as i32)
            as uint8_t;
        status = (status as i32
            | WrByte(&mut (*p_dev).platform, 0x4002 as i32 as uint16_t, 0u8) as i32)
            as uint8_t;
        status = (status as i32
            | WrByte(&mut (*p_dev).platform, 0x10a as i32 as uint16_t, 0x3u8) as i32)
            as uint8_t;
        status = (status as i32
            | WrByte(&mut (*p_dev).platform, 0x103 as i32 as uint16_t, 0x1u8) as i32)
            as uint8_t;
        status = (status as i32
            | WrByte(&mut (*p_dev).platform, 0x400f as i32 as uint16_t, 0u8) as i32)
            as uint8_t;
        status = (status as i32
            | WrByte(&mut (*p_dev).platform, 0x21a as i32 as uint16_t, 0x43u8) as i32)
            as uint8_t;
        status = (status as i32
            | WrByte(&mut (*p_dev).platform, 0x21a as i32 as uint16_t, 0x3u8) as i32)
            as uint8_t;
        status = (status as i32
            | WrByte(&mut (*p_dev).platform, 0x21a as i32 as uint16_t, 0x1u8) as i32)
            as uint8_t;
        status = (status as i32
            | WrByte(&mut (*p_dev).platform, 0x21a as i32 as uint16_t, 0u8) as i32)
            as uint8_t;
        status = (status as i32
            | WrByte(&mut (*p_dev).platform, 0x219 as i32 as uint16_t, 0u8) as i32)
            as uint8_t;
        status = (status as i32
            | WrByte(&mut (*p_dev).platform, 0x21b as i32 as uint16_t, 0u8) as i32)
            as uint8_t;
        status = (status as i32
            | WrByte(&mut (*p_dev).platform, 0x7fff as i32 as uint16_t, 0u8) as i32)
            as uint8_t;
        status = (status as i32
            | RdByte(&mut (*p_dev).platform, 0x7fff as i32 as uint16_t, &mut tmp) as i32)
            as uint8_t;
        status = (status as i32
            | WrByte(&mut (*p_dev).platform, 0xc as i32 as uint16_t, 0u8) as i32)
            as uint8_t;
        status = (status as i32
            | WrByte(&mut (*p_dev).platform, 0x7fff as i32 as uint16_t, 0x1u8) as i32)
            as uint8_t;
        status = (status as i32
            | WrByte(&mut (*p_dev).platform, 0x20 as i32 as uint16_t, 0x7u8) as i32)
            as uint8_t;
        status = (status as i32
            | WrByte(&mut (*p_dev).platform, 0x20 as i32 as uint16_t, 0x6u8) as i32)
            as uint8_t;
        status = (status as i32
            | WrByte(&mut (*p_dev).platform, 0x7fff as i32 as uint16_t, 0x9u8) as i32)
            as uint8_t;
        status = (status as i32
            | WrMulti(
                &mut (*p_dev).platform,
                0 as i32 as uint16_t,
                &*consts::VL53L5CX_FIRMWARE.as_ptr().offset(0 as i32 as isize) as *const uint8_t
                    as *mut uint8_t,
                0x8000 as i32 as uint32_t,
            ) as i32) as uint8_t;
        status = (status as i32
            | WrByte(&mut (*p_dev).platform, 0x7fff as i32 as uint16_t, 0xau8) as i32)
            as uint8_t;
        status = (status as i32
            | WrMulti(
                &mut (*p_dev).platform,
                0 as i32 as uint16_t,
                &*consts::VL53L5CX_FIRMWARE
                    .as_ptr()
                    .offset(0x8000 as i32 as isize) as *const uint8_t
                    as *mut uint8_t,
                0x8000 as i32 as uint32_t,
            ) as i32) as uint8_t;
        status = (status as i32
            | WrByte(&mut (*p_dev).platform, 0x7fff as i32 as uint16_t, 0xbu8) as i32)
            as uint8_t;
        status = (status as i32
            | WrMulti(
                &mut (*p_dev).platform,
                0 as i32 as uint16_t,
                &*consts::VL53L5CX_FIRMWARE
                    .as_ptr()
                    .offset(0x10000 as i32 as isize) as *const uint8_t
                    as *mut uint8_t,
                0x5000 as i32 as uint32_t,
            ) as i32) as uint8_t;
        status = (status as i32
            | WrByte(&mut (*p_dev).platform, 0x7fff as i32 as uint16_t, 0x1u8) as i32)
            as uint8_t;
        status = (status as i32
            | WrByte(&mut (*p_dev).platform, 0x7fff as i32 as uint16_t, 0x2u8) as i32)
            as uint8_t;
        status = (status as i32
            | WrByte(&mut (*p_dev).platform, 0x3 as i32 as uint16_t, 0xdu8) as i32)
            as uint8_t;
        status = (status as i32
            | WrByte(&mut (*p_dev).platform, 0x7fff as i32 as uint16_t, 0x1u8) as i32)
            as uint8_t;
        status = (status as i32
            | _vl53l5cx_poll_for_answer(p_dev, 1u8, 0u8, 0x21 as i32 as uint16_t, 0x10u8, 0x10u8)
                as i32) as uint8_t;
        if !(status as i32 != 0u8 as i32) {
            status = (status as i32
                | WrByte(&mut (*p_dev).platform, 0x7fff as i32 as uint16_t, 0u8) as i32)
                as uint8_t;
            status = (status as i32
                | RdByte(&mut (*p_dev).platform, 0x7fff as i32 as uint16_t, &mut tmp) as i32)
                as uint8_t;
            status = (status as i32
                | WrByte(&mut (*p_dev).platform, 0xc as i32 as uint16_t, 0x1u8) as i32)
                as uint8_t;
            status = (status as i32
                | WrByte(&mut (*p_dev).platform, 0x7fff as i32 as uint16_t, 0u8) as i32)
                as uint8_t;
            status = (status as i32
                | WrByte(&mut (*p_dev).platform, 0x114 as i32 as uint16_t, 0u8) as i32)
                as uint8_t;
            status = (status as i32
                | WrByte(&mut (*p_dev).platform, 0x115 as i32 as uint16_t, 0u8) as i32)
                as uint8_t;
            status = (status as i32
                | WrByte(&mut (*p_dev).platform, 0x116 as i32 as uint16_t, 0x42u8) as i32)
                as uint8_t;
            status = (status as i32
                | WrByte(&mut (*p_dev).platform, 0x117 as i32 as uint16_t, 0u8) as i32)
                as uint8_t;
            status = (status as i32
                | WrByte(&mut (*p_dev).platform, 0xb as i32 as uint16_t, 0u8) as i32)
                as uint8_t;
            status = (status as i32
                | RdByte(&mut (*p_dev).platform, 0x7fff as i32 as uint16_t, &mut tmp) as i32)
                as uint8_t;
            status = (status as i32
                | WrByte(&mut (*p_dev).platform, 0xc as i32 as uint16_t, 0u8) as i32)
                as uint8_t;
            status = (status as i32
                | WrByte(&mut (*p_dev).platform, 0xb as i32 as uint16_t, 0x1u8) as i32)
                as uint8_t;
            status = (status as i32 | _vl53l5cx_poll_for_mcu_boot(p_dev) as i32) as uint8_t;
            if !(status as i32 != 0u8 as i32) {
                status = (status as i32
                    | WrByte(&mut (*p_dev).platform, 0x7fff as i32 as uint16_t, 0x2u8) as i32)
                    as uint8_t;
                status = (status as i32
                    | WrMulti(
                        &mut (*p_dev).platform,
                        0x2fd8 as i32 as uint16_t,
                        consts::VL53L5CX_GET_NVM_CMD.as_ptr() as *mut uint8_t,
                        ::core::mem::size_of::<[uint8_t; 40]>() as ulong as uint32_t,
                    ) as i32) as uint8_t;
                status = (status as i32
                    | _vl53l5cx_poll_for_answer(
                        p_dev,
                        4u8,
                        0u8,
                        0x2c00 as u32 as uint16_t,
                        0xffu8,
                        2u8,
                    ) as i32) as uint8_t;
                status = (status as i32
                    | RdMulti(
                        &mut (*p_dev).platform,
                        0x2c04 as u32 as uint16_t,
                        ((*p_dev).temp_buffer).as_mut_ptr(),
                        492 as u32 as uint16_t as uint32_t,
                    ) as i32) as uint8_t;
                memcpy(
                    ((*p_dev).offset_data).as_mut_ptr() as *mut (),
                    ((*p_dev).temp_buffer).as_mut_ptr() as *const (),
                    488 as u32 as uint16_t as ulong,
                );
                status =
                    (status as i32 | _vl53l5cx_send_offset_data(p_dev, 16u8) as i32) as uint8_t;
                memcpy(
                    ((*p_dev).xtalk_data).as_mut_ptr() as *mut (),
                    consts::VL53L5CX_DEFAULT_XTALK.as_ptr() as *mut uint8_t as *const (),
                    776 as u32 as uint16_t as ulong,
                );
                status = (status as i32 | _vl53l5cx_send_xtalk_data(p_dev, 16u8) as i32) as uint8_t;
                status = (status as i32
                    | WrMulti(
                        &mut (*p_dev).platform,
                        0x2c34 as i32 as uint16_t,
                        (*p_dev).default_configuration,
                        ::core::mem::size_of::<[uint8_t; 972]>() as ulong as uint32_t,
                    ) as i32) as uint8_t;
                status = (status as i32
                    | _vl53l5cx_poll_for_answer(
                        p_dev,
                        4u8,
                        1u8,
                        0x2c00 as u32 as uint16_t,
                        0xffu8,
                        0x3u8,
                    ) as i32) as uint8_t;
                status = (status as i32
                    | vl53l5cx_dci_write_data(
                        p_dev,
                        &mut pipe_ctrl as *mut [uint8_t; 4] as *mut uint8_t,
                        0xdb80 as u32 as uint16_t as uint32_t,
                        ::core::mem::size_of::<[uint8_t; 4]>() as ulong as uint16_t,
                    ) as i32) as uint8_t;
                status = (status as i32
                    | vl53l5cx_dci_write_data(
                        p_dev,
                        &mut single_range as *mut uint32_t as *mut uint8_t,
                        0xd964 as u32 as uint16_t as uint32_t,
                        ::core::mem::size_of::<uint32_t>() as ulong as uint16_t,
                    ) as i32) as uint8_t;
                tmp = 1u8;
                status = (status as i32
                    | vl53l5cx_dci_replace_data(
                        p_dev,
                        ((*p_dev).temp_buffer).as_mut_ptr(),
                        0xe108 as u32 as uint16_t as uint32_t,
                        40 as i32 as uint16_t,
                        &mut tmp as *mut uint8_t,
                        1 as i32 as uint16_t,
                        0x26 as i32 as uint16_t,
                    ) as i32) as uint8_t;
                status = (status as i32
                    | vl53l5cx_dci_replace_data(
                        p_dev,
                        ((*p_dev).temp_buffer).as_mut_ptr(),
                        0xe108 as u32 as uint16_t as uint32_t,
                        40 as i32 as uint16_t,
                        &mut tmp as *mut uint8_t,
                        1 as i32 as uint16_t,
                        0x25 as i32 as uint16_t,
                    ) as i32) as uint8_t;
            }
        }
    }
    return status;
}

#[no_mangle]
pub unsafe extern "C" fn vl53l5cx_get_power_mode(
    mut p_dev: *mut VL53L5CX_Configuration,
    mut p_power_mode: *mut uint8_t,
) -> uint8_t {
    let mut tmp: uint8_t = 0;
    let mut status: uint8_t = 0u8;
    status = (status as i32 | WrByte(&mut (*p_dev).platform, 0x7fff as i32 as uint16_t, 0u8) as i32)
        as uint8_t;
    status = (status as i32
        | RdByte(&mut (*p_dev).platform, 0x9 as i32 as uint16_t, &mut tmp) as i32)
        as uint8_t;
    match tmp as i32 {
        4 => {
            *p_power_mode = 1u8;
        }
        2 => {
            *p_power_mode = 0u8;
        }
        _ => {
            *p_power_mode = 0u8;
            status = 255u8;
        }
    }
    status = (status as i32
        | WrByte(&mut (*p_dev).platform, 0x7fff as i32 as uint16_t, 0x2u8) as i32)
        as uint8_t;
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn vl53l5cx_set_power_mode(
    mut p_dev: *mut VL53L5CX_Configuration,
    mut power_mode: uint8_t,
) -> uint8_t {
    let mut current_power_mode: uint8_t = 0;
    let mut status: uint8_t = 0u8;
    status =
        (status as i32 | vl53l5cx_get_power_mode(p_dev, &mut current_power_mode) as i32) as uint8_t;
    if power_mode as i32 != current_power_mode as i32 {
        match power_mode as i32 {
            1 => {
                status = (status as i32
                    | WrByte(&mut (*p_dev).platform, 0x7fff as i32 as uint16_t, 0u8) as i32)
                    as uint8_t;
                status = (status as i32
                    | WrByte(&mut (*p_dev).platform, 0x9 as i32 as uint16_t, 0x4u8) as i32)
                    as uint8_t;
                status = (status as i32
                    | _vl53l5cx_poll_for_answer(p_dev, 1u8, 0u8, 0x6 as i32 as uint16_t, 0x1u8, 1u8)
                        as i32) as uint8_t;
            }
            0 => {
                status = (status as i32
                    | WrByte(&mut (*p_dev).platform, 0x7fff as i32 as uint16_t, 0u8) as i32)
                    as uint8_t;
                status = (status as i32
                    | WrByte(&mut (*p_dev).platform, 0x9 as i32 as uint16_t, 0x2u8) as i32)
                    as uint8_t;
                status = (status as i32
                    | _vl53l5cx_poll_for_answer(p_dev, 1u8, 0u8, 0x6 as i32 as uint16_t, 0x1u8, 0u8)
                        as i32) as uint8_t;
            }
            _ => {
                status = 255u8;
            }
        }
        status = (status as i32
            | WrByte(&mut (*p_dev).platform, 0x7fff as i32 as uint16_t, 0x2u8) as i32)
            as uint8_t;
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn vl53l5cx_start_ranging(mut p_dev: *mut VL53L5CX_Configuration) -> uint8_t {
    let mut resolution: uint8_t = 0;
    let mut status: uint8_t = 0u8;
    let mut tmp: uint16_t = 0;
    let mut i: uint32_t = 0;
    let mut header_config: [uint32_t; 2] = [0 as i32 as uint32_t, 0 as i32 as uint32_t];
    let mut bh_ptr: *mut Block_header = 0 as *mut Block_header;
    let mut cmd: [uint8_t; 4] = [0u8, 0x3u8, 0u8, 0u8];
    status = (status as i32 | vl53l5cx_get_resolution(p_dev, &mut resolution) as i32) as uint8_t;
    (*p_dev).data_read_size = 0 as i32 as uint32_t;
    (*p_dev).streamcount = 255u8;
    let mut output_bh_enable: [uint32_t; 4] = [0x7 as u32, 0 as u32, 0 as u32, 0xc0000000 as u32];
    let mut output: [uint32_t; 12] = [
        0xd as u32,
        0x54b400c0 as u32,
        0x54c00040 as u32,
        0x54d00104 as u32,
        0x55d00404 as u32,
        0xdb840401 as u32,
        0xdbc40404 as u32,
        0xdec40402 as u32,
        0xdf440402 as u32,
        0xe0440401 as u32,
        0xe0840401 as u32,
        0xd85808c0 as u32,
    ];
    output_bh_enable[0 as i32 as usize] = (output_bh_enable[0 as i32 as usize] as u32)
        .wrapping_add(8 as i32 as uint32_t) as uint32_t
        as uint32_t;
    output_bh_enable[0 as i32 as usize] = (output_bh_enable[0 as i32 as usize] as u32)
        .wrapping_add(16 as i32 as uint32_t) as uint32_t
        as uint32_t;
    output_bh_enable[0 as i32 as usize] = (output_bh_enable[0 as i32 as usize] as u32)
        .wrapping_add(32 as i32 as uint32_t) as uint32_t
        as uint32_t;
    output_bh_enable[0 as i32 as usize] = (output_bh_enable[0 as i32 as usize] as u32)
        .wrapping_add(64 as i32 as uint32_t) as uint32_t
        as uint32_t;
    output_bh_enable[0 as i32 as usize] = (output_bh_enable[0 as i32 as usize] as u32)
        .wrapping_add(128 as i32 as uint32_t) as uint32_t
        as uint32_t;
    output_bh_enable[0 as i32 as usize] = (output_bh_enable[0 as i32 as usize] as u32)
        .wrapping_add(256 as i32 as uint32_t) as uint32_t
        as uint32_t;
    output_bh_enable[0 as i32 as usize] = (output_bh_enable[0 as i32 as usize] as u32)
        .wrapping_add(512 as i32 as uint32_t) as uint32_t
        as uint32_t;
    output_bh_enable[0 as i32 as usize] = (output_bh_enable[0 as i32 as usize] as u32)
        .wrapping_add(1024 as i32 as uint32_t) as uint32_t
        as uint32_t;
    output_bh_enable[0 as i32 as usize] = (output_bh_enable[0 as i32 as usize] as u32)
        .wrapping_add(2048 as i32 as uint32_t) as uint32_t
        as uint32_t;
    i = 0 as i32 as uint32_t;
    while i
        < (::core::mem::size_of::<[uint32_t; 12]>() as ulong)
            .wrapping_div(::core::mem::size_of::<uint32_t>() as ulong) as uint32_t
    {
        if !(output[i as usize] == 0u8 as u32
            || output_bh_enable[i.wrapping_div(32 as i32 as uint32_t) as usize]
                & (1 as i32 as uint32_t) << i.wrapping_rem(32 as i32 as uint32_t)
                == 0 as i32 as uint32_t)
        {
            bh_ptr =
                &mut *output.as_mut_ptr().offset(i as isize) as *mut uint32_t as *mut Block_header;
            if ((*bh_ptr).c2rust_unnamed).type_0() as uint8_t as i32 >= 0x1u8 as i32
                && (((*bh_ptr).c2rust_unnamed).type_0() as uint8_t as i32) < 0xdu8 as i32
            {
                if ((*bh_ptr).c2rust_unnamed).idx() as i32 >= 0x54d0 as i32 as uint16_t as i32
                    && (((*bh_ptr).c2rust_unnamed).idx() as i32)
                        < (0x54d0 as i32 + 960 as i32) as uint16_t as i32
                {
                    let ref mut fresh2 = (*bh_ptr).c2rust_unnamed;
                    (*fresh2).set_size(resolution as uint32_t);
                } else {
                    let ref mut fresh3 = (*bh_ptr).c2rust_unnamed;
                    (*fresh3).set_size(
                        (resolution as uint16_t as i32 * 1 as u32 as uint16_t as i32) as uint16_t
                            as uint32_t,
                    );
                }
                let ref mut fresh4 = (*p_dev).data_read_size;
                *fresh4 = (*fresh4 as u32).wrapping_add(
                    (((*bh_ptr).c2rust_unnamed).type_0() as i32
                        * ((*bh_ptr).c2rust_unnamed).size() as i32) as u32,
                ) as uint32_t as uint32_t;
            } else {
                let ref mut fresh5 = (*p_dev).data_read_size;
                *fresh5 = (*fresh5 as u32).wrapping_add(((*bh_ptr).c2rust_unnamed).size())
                    as uint32_t as uint32_t;
            }
            let ref mut fresh6 = (*p_dev).data_read_size;
            *fresh6 = (*fresh6 as u32).wrapping_add(4 as i32 as uint32_t) as uint32_t as uint32_t;
        }
        i = i.wrapping_add(1);
    }
    let ref mut fresh7 = (*p_dev).data_read_size;
    *fresh7 = (*fresh7 as u32).wrapping_add(24 as i32 as uint32_t) as uint32_t as uint32_t;
    status = (status as i32
        | vl53l5cx_dci_write_data(
            p_dev,
            &mut output as *mut [uint32_t; 12] as *mut uint8_t,
            0xd980 as u32 as uint16_t as uint32_t,
            ::core::mem::size_of::<[uint32_t; 12]>() as ulong as uint16_t,
        ) as i32) as uint8_t;
    header_config[0 as i32 as usize] = (*p_dev).data_read_size;
    header_config[1 as i32 as usize] = i.wrapping_add(1 as i32 as uint32_t);
    status = (status as i32
        | vl53l5cx_dci_write_data(
            p_dev,
            &mut header_config as *mut [uint32_t; 2] as *mut uint8_t,
            0xd968 as u32 as uint16_t as uint32_t,
            ::core::mem::size_of::<[uint32_t; 2]>() as ulong as uint16_t,
        ) as i32) as uint8_t;
    status = (status as i32
        | vl53l5cx_dci_write_data(
            p_dev,
            &mut output_bh_enable as *mut [uint32_t; 4] as *mut uint8_t,
            0xd970 as u32 as uint16_t as uint32_t,
            ::core::mem::size_of::<[uint32_t; 4]>() as ulong as uint16_t,
        ) as i32) as uint8_t;
    status = (status as i32 | WrByte(&mut (*p_dev).platform, 0x7fff as i32 as uint16_t, 0u8) as i32)
        as uint8_t;
    status = (status as i32 | WrByte(&mut (*p_dev).platform, 0x9 as i32 as uint16_t, 0x5u8) as i32)
        as uint8_t;
    status = (status as i32
        | WrByte(&mut (*p_dev).platform, 0x7fff as i32 as uint16_t, 0x2u8) as i32)
        as uint8_t;
    status = (status as i32
        | WrMulti(
            &mut (*p_dev).platform,
            (0x2fff as u32 as uint16_t as i32 - (4 as i32 - 1 as i32) as uint16_t as i32)
                as uint16_t,
            cmd.as_mut_ptr(),
            ::core::mem::size_of::<[uint8_t; 4]>() as ulong as uint32_t,
        ) as i32) as uint8_t;
    status = (status as i32
        | _vl53l5cx_poll_for_answer(p_dev, 4u8, 1u8, 0x2c00 as u32 as uint16_t, 0xffu8, 0x3u8)
            as i32) as uint8_t;
    status = (status as i32
        | vl53l5cx_dci_read_data(
            p_dev,
            ((*p_dev).temp_buffer).as_mut_ptr(),
            0x5440 as i32 as uint32_t,
            12 as i32 as uint16_t,
        ) as i32) as uint8_t;
    memcpy(
        &mut tmp as *mut uint16_t as *mut (),
        &mut *((*p_dev).temp_buffer)
            .as_mut_ptr()
            .offset(0x8 as i32 as isize) as *mut uint8_t as *const (),
        ::core::mem::size_of::<uint16_t>() as ulong,
    );
    if tmp as u32 != (*p_dev).data_read_size {
        status = (status as i32 | 255u8 as i32) as uint8_t;
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn vl53l5cx_stop_ranging(mut p_dev: *mut VL53L5CX_Configuration) -> uint8_t {
    let mut tmp: uint8_t = 0u8;
    let mut status: uint8_t = 0u8;
    let mut timeout: uint16_t = 0 as i32 as uint16_t;
    let mut auto_stop_flag: uint32_t = 0 as i32 as uint32_t;
    status = (status as i32
        | RdMulti(
            &mut (*p_dev).platform,
            0x2ffc as i32 as uint16_t,
            &mut auto_stop_flag as *mut uint32_t as *mut uint8_t,
            4 as i32 as uint32_t,
        ) as i32) as uint8_t;
    if auto_stop_flag != 0x4ff as i32 as uint32_t
        && (*p_dev).is_auto_stop_enabled as i32 == 0u8 as i32
    {
        status = (status as i32
            | WrByte(&mut (*p_dev).platform, 0x7fff as i32 as uint16_t, 0u8) as i32)
            as uint8_t;
        status = (status as i32
            | WrByte(&mut (*p_dev).platform, 0x15 as i32 as uint16_t, 0x16u8) as i32)
            as uint8_t;
        status = (status as i32
            | WrByte(&mut (*p_dev).platform, 0x14 as i32 as uint16_t, 0x1u8) as i32)
            as uint8_t;
        while (tmp as i32 & 0x80u8 as i32) >> 7 as i32 == 0u8 as i32 {
            status = (status as i32
                | RdByte(&mut (*p_dev).platform, 0x6 as i32 as uint16_t, &mut tmp) as i32)
                as uint8_t;
            status = (status as i32 | WaitMs(&mut (*p_dev).platform, 10 as i32 as uint32_t) as i32)
                as uint8_t;
            timeout = timeout.wrapping_add(1);
            if !(timeout as i32 > 500 as i32 as uint16_t as i32) {
                continue;
            }
            status = (status as i32 | tmp as i32) as uint8_t;
            break;
        }
    }
    status = (status as i32
        | RdByte(&mut (*p_dev).platform, 0x6 as i32 as uint16_t, &mut tmp) as i32)
        as uint8_t;
    if tmp as i32 & 0x80u8 as i32 != 0u8 as i32 {
        status = (status as i32
            | RdByte(&mut (*p_dev).platform, 0x7 as i32 as uint16_t, &mut tmp) as i32)
            as uint8_t;
        if tmp as i32 != 0x84u8 as i32 && tmp as i32 != 0x85u8 as i32 {
            status = (status as i32 | tmp as i32) as uint8_t;
        }
    }
    status = (status as i32 | WrByte(&mut (*p_dev).platform, 0x7fff as i32 as uint16_t, 0u8) as i32)
        as uint8_t;
    status = (status as i32 | WrByte(&mut (*p_dev).platform, 0x14 as i32 as uint16_t, 0u8) as i32)
        as uint8_t;
    status = (status as i32 | WrByte(&mut (*p_dev).platform, 0x15 as i32 as uint16_t, 0u8) as i32)
        as uint8_t;
    status = (status as i32 | WrByte(&mut (*p_dev).platform, 0x9 as i32 as uint16_t, 0x4u8) as i32)
        as uint8_t;
    status = (status as i32
        | WrByte(&mut (*p_dev).platform, 0x7fff as i32 as uint16_t, 0x2u8) as i32)
        as uint8_t;
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn vl53l5cx_check_data_ready(
    mut p_dev: *mut VL53L5CX_Configuration,
    mut p_isReady: *mut uint8_t,
) -> uint8_t {
    let mut status: uint8_t = 0u8;
    status = (status as i32
        | RdMulti(
            &mut (*p_dev).platform,
            0 as i32 as uint16_t,
            ((*p_dev).temp_buffer).as_mut_ptr(),
            4 as i32 as uint32_t,
        ) as i32) as uint8_t;
    if (*p_dev).temp_buffer[0 as i32 as usize] as i32 != (*p_dev).streamcount as i32
        && (*p_dev).temp_buffer[0 as i32 as usize] as i32 != 255u8 as i32
        && (*p_dev).temp_buffer[1 as i32 as usize] as i32 == 0x5u8 as i32
        && (*p_dev).temp_buffer[2 as i32 as usize] as i32 & 0x5u8 as i32 == 0x5u8 as i32
        && (*p_dev).temp_buffer[3 as i32 as usize] as i32 & 0x10u8 as i32 == 0x10u8 as i32
    {
        *p_isReady = 1u8;
        (*p_dev).streamcount = (*p_dev).temp_buffer[0 as i32 as usize];
    } else {
        if (*p_dev).temp_buffer[3 as i32 as usize] as i32 & 0x80u8 as i32 != 0u8 as i32 {
            status = (status as i32 | (*p_dev).temp_buffer[2 as i32 as usize] as i32) as uint8_t;
        }
        *p_isReady = 0u8;
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn vl53l5cx_get_ranging_data(
    mut p_dev: *mut VL53L5CX_Configuration,
    mut p_results: *mut VL53L5CX_ResultsData,
) -> uint8_t {
    let mut status: uint8_t = 0u8;
    let mut bh_ptr: *mut Block_header = 0 as *mut Block_header;
    let mut header_id: uint16_t = 0;
    let mut footer_id: uint16_t = 0;
    let mut i: uint32_t = 0;
    let mut j: uint32_t = 0;
    let mut msize: uint32_t = 0;
    status = (status as i32
        | RdMulti(
            &mut (*p_dev).platform,
            0 as i32 as uint16_t,
            ((*p_dev).temp_buffer).as_mut_ptr(),
            (*p_dev).data_read_size,
        ) as i32) as uint8_t;
    (*p_dev).streamcount = (*p_dev).temp_buffer[0 as i32 as usize];
    SwapBuffer(
        ((*p_dev).temp_buffer).as_mut_ptr(),
        (*p_dev).data_read_size as uint16_t,
    );
    i = 16 as i32 as uint32_t;
    while i < (*p_dev).data_read_size {
        bh_ptr = &mut *((*p_dev).temp_buffer).as_mut_ptr().offset(i as isize) as *mut uint8_t
            as *mut Block_header;
        if ((*bh_ptr).c2rust_unnamed).type_0() > 0x1 as i32 as uint32_t
            && ((*bh_ptr).c2rust_unnamed).type_0() < 0xd as i32 as uint32_t
        {
            msize = (((*bh_ptr).c2rust_unnamed).type_0() as i32
                * ((*bh_ptr).c2rust_unnamed).size() as i32) as uint32_t;
        } else {
            msize = ((*bh_ptr).c2rust_unnamed).size();
        }
        match ((*bh_ptr).c2rust_unnamed).idx() as i32 {
            21684 => {
                (*p_results).silicon_temp_degc =
                    (*p_dev).temp_buffer[i.wrapping_add(12 as i32 as uint32_t) as usize] as int8_t;
            }
            21712 => {
                memcpy(
                    ((*p_results).ambient_per_spad).as_mut_ptr() as *mut (),
                    &mut *((*p_dev).temp_buffer)
                        .as_mut_ptr()
                        .offset(i.wrapping_add(4 as i32 as uint32_t) as isize)
                        as *mut uint8_t as *const (),
                    msize as ulong,
                );
            }
            21968 => {
                memcpy(
                    ((*p_results).nb_spads_enabled).as_mut_ptr() as *mut (),
                    &mut *((*p_dev).temp_buffer)
                        .as_mut_ptr()
                        .offset(i.wrapping_add(4 as i32 as uint32_t) as isize)
                        as *mut uint8_t as *const (),
                    msize as ulong,
                );
            }
            56196 => {
                memcpy(
                    ((*p_results).nb_target_detected).as_mut_ptr() as *mut (),
                    &mut *((*p_dev).temp_buffer)
                        .as_mut_ptr()
                        .offset(i.wrapping_add(4 as i32 as uint32_t) as isize)
                        as *mut uint8_t as *const (),
                    msize as ulong,
                );
            }
            56260 => {
                memcpy(
                    ((*p_results).signal_per_spad).as_mut_ptr() as *mut (),
                    &mut *((*p_dev).temp_buffer)
                        .as_mut_ptr()
                        .offset(i.wrapping_add(4 as i32 as uint32_t) as isize)
                        as *mut uint8_t as *const (),
                    msize as ulong,
                );
            }
            57028 => {
                memcpy(
                    ((*p_results).range_sigma_mm).as_mut_ptr() as *mut (),
                    &mut *((*p_dev).temp_buffer)
                        .as_mut_ptr()
                        .offset(i.wrapping_add(4 as i32 as uint32_t) as isize)
                        as *mut uint8_t as *const (),
                    msize as ulong,
                );
            }
            57156 => {
                memcpy(
                    ((*p_results).distance_mm).as_mut_ptr() as *mut (),
                    &mut *((*p_dev).temp_buffer)
                        .as_mut_ptr()
                        .offset(i.wrapping_add(4 as i32 as uint32_t) as isize)
                        as *mut uint8_t as *const (),
                    msize as ulong,
                );
            }
            57412 => {
                memcpy(
                    ((*p_results).reflectance).as_mut_ptr() as *mut (),
                    &mut *((*p_dev).temp_buffer)
                        .as_mut_ptr()
                        .offset(i.wrapping_add(4 as i32 as uint32_t) as isize)
                        as *mut uint8_t as *const (),
                    msize as ulong,
                );
            }
            57476 => {
                memcpy(
                    ((*p_results).target_status).as_mut_ptr() as *mut (),
                    &mut *((*p_dev).temp_buffer)
                        .as_mut_ptr()
                        .offset(i.wrapping_add(4 as i32 as uint32_t) as isize)
                        as *mut uint8_t as *const (),
                    msize as ulong,
                );
            }
            55384 => {
                memcpy(
                    &mut (*p_results).motion_indicator as *mut C2RustUnnamed as *mut (),
                    &mut *((*p_dev).temp_buffer)
                        .as_mut_ptr()
                        .offset(i.wrapping_add(4 as i32 as uint32_t) as isize)
                        as *mut uint8_t as *const (),
                    msize as ulong,
                );
            }
            _ => {}
        }
        i = (i as u32).wrapping_add(msize) as uint32_t as uint32_t;
        i = (i as u32).wrapping_add(4 as i32 as uint32_t) as uint32_t as uint32_t;
    }
    i = 0 as i32 as uint32_t;
    while i < 64u8 as uint32_t {
        let ref mut fresh8 = (*p_results).ambient_per_spad[i as usize];
        *fresh8 = (*fresh8 as u32).wrapping_div(2048 as i32 as uint32_t) as uint32_t as uint32_t;
        i = i.wrapping_add(1);
    }
    i = 0 as i32 as uint32_t;
    while i < (64u8 as u32).wrapping_mul(1 as u32) {
        let ref mut fresh9 = (*p_results).distance_mm[i as usize];
        *fresh9 = (*fresh9 as i32 / 4 as i32) as int16_t;
        if ((*p_results).distance_mm[i as usize] as i32) < 0 as i32 {
            (*p_results).distance_mm[i as usize] = 0 as i32 as int16_t;
        }
        let ref mut fresh10 = (*p_results).reflectance[i as usize];
        *fresh10 = (*fresh10 as i32 / 2u8 as i32) as uint8_t;
        let ref mut fresh11 = (*p_results).range_sigma_mm[i as usize];
        *fresh11 = (*fresh11 as i32 / 128 as i32 as uint16_t as i32) as uint16_t;
        let ref mut fresh12 = (*p_results).signal_per_spad[i as usize];
        *fresh12 = (*fresh12 as u32).wrapping_div(2048 as i32 as uint32_t) as uint32_t as uint32_t;
        i = i.wrapping_add(1);
    }
    i = 0 as i32 as uint32_t;
    while i < 64u8 as uint32_t {
        if (*p_results).nb_target_detected[i as usize] as i32 == 0u8 as i32 {
            j = 0 as i32 as uint32_t;
            while j < 1 as u32 {
                (*p_results).target_status[(1 as u32).wrapping_mul(i).wrapping_add(j) as usize] =
                    255u8;
                j = j.wrapping_add(1);
            }
        }
        i = i.wrapping_add(1);
    }
    i = 0 as i32 as uint32_t;
    while i < 32 as i32 as uint32_t {
        let ref mut fresh13 = (*p_results).motion_indicator.motion[i as usize];
        *fresh13 = (*fresh13 as u32).wrapping_div(65535 as i32 as uint32_t) as uint32_t as uint32_t;
        i = i.wrapping_add(1);
    }
    header_id = ((((*p_dev).temp_buffer[0x8 as i32 as usize] as uint16_t as i32) << 8 as i32)
        as u32
        & 0xff00 as u32) as uint16_t;
    header_id = (header_id as u32
        | (*p_dev).temp_buffer[0x9 as i32 as usize] as uint16_t as u32 & 0xff as u32)
        as uint16_t;
    footer_id = ((((*p_dev).temp_buffer
        [((*p_dev).data_read_size).wrapping_sub(4 as i32 as uint32_t) as usize]
        as uint16_t as i32)
        << 8 as i32) as u32
        & 0xff00 as u32) as uint16_t;
    footer_id = (footer_id as u32
        | (*p_dev).temp_buffer
            [((*p_dev).data_read_size).wrapping_sub(3 as i32 as uint32_t) as usize]
            as uint16_t as u32
            & 0xff as u32) as uint16_t;
    if header_id as i32 != footer_id as i32 {
        status = (status as i32 | 2u8 as i32) as uint8_t;
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn vl53l5cx_get_resolution(
    mut p_dev: *mut VL53L5CX_Configuration,
    mut p_resolution: *mut uint8_t,
) -> uint8_t {
    let mut status: uint8_t = 0u8;
    status = (status as i32
        | vl53l5cx_dci_read_data(
            p_dev,
            ((*p_dev).temp_buffer).as_mut_ptr(),
            0x5450 as u32 as uint16_t as uint32_t,
            8 as i32 as uint16_t,
        ) as i32) as uint8_t;
    *p_resolution = ((*p_dev).temp_buffer[0 as i32 as usize] as i32
        * (*p_dev).temp_buffer[0x1 as i32 as usize] as i32) as uint8_t;
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn vl53l5cx_set_resolution(
    mut p_dev: *mut VL53L5CX_Configuration,
    mut resolution: uint8_t,
) -> uint8_t {
    let mut status: uint8_t = 0u8;
    match resolution as i32 {
        16 => {
            status = (status as i32
                | vl53l5cx_dci_read_data(
                    p_dev,
                    ((*p_dev).temp_buffer).as_mut_ptr(),
                    0xad38 as u32 as uint16_t as uint32_t,
                    16 as i32 as uint16_t,
                ) as i32) as uint8_t;
            (*p_dev).temp_buffer[0x4 as i32 as usize] = 64u8;
            (*p_dev).temp_buffer[0x6 as i32 as usize] = 64u8;
            (*p_dev).temp_buffer[0x9 as i32 as usize] = 4u8;
            status = (status as i32
                | vl53l5cx_dci_write_data(
                    p_dev,
                    ((*p_dev).temp_buffer).as_mut_ptr(),
                    0xad38 as u32 as uint16_t as uint32_t,
                    16 as i32 as uint16_t,
                ) as i32) as uint8_t;
            status = (status as i32
                | vl53l5cx_dci_read_data(
                    p_dev,
                    ((*p_dev).temp_buffer).as_mut_ptr(),
                    0x5450 as u32 as uint16_t as uint32_t,
                    8 as i32 as uint16_t,
                ) as i32) as uint8_t;
            (*p_dev).temp_buffer[0 as i32 as usize] = 4u8;
            (*p_dev).temp_buffer[0x1 as i32 as usize] = 4u8;
            (*p_dev).temp_buffer[0x4 as i32 as usize] = 8u8;
            (*p_dev).temp_buffer[0x5 as i32 as usize] = 8u8;
            status = (status as i32
                | vl53l5cx_dci_write_data(
                    p_dev,
                    ((*p_dev).temp_buffer).as_mut_ptr(),
                    0x5450 as u32 as uint16_t as uint32_t,
                    8 as i32 as uint16_t,
                ) as i32) as uint8_t;
        }
        64 => {
            status = (status as i32
                | vl53l5cx_dci_read_data(
                    p_dev,
                    ((*p_dev).temp_buffer).as_mut_ptr(),
                    0xad38 as u32 as uint16_t as uint32_t,
                    16 as i32 as uint16_t,
                ) as i32) as uint8_t;
            (*p_dev).temp_buffer[0x4 as i32 as usize] = 16u8;
            (*p_dev).temp_buffer[0x6 as i32 as usize] = 16u8;
            (*p_dev).temp_buffer[0x9 as i32 as usize] = 1u8;
            status = (status as i32
                | vl53l5cx_dci_write_data(
                    p_dev,
                    ((*p_dev).temp_buffer).as_mut_ptr(),
                    0xad38 as u32 as uint16_t as uint32_t,
                    16 as i32 as uint16_t,
                ) as i32) as uint8_t;
            status = (status as i32
                | vl53l5cx_dci_read_data(
                    p_dev,
                    ((*p_dev).temp_buffer).as_mut_ptr(),
                    0x5450 as u32 as uint16_t as uint32_t,
                    8 as i32 as uint16_t,
                ) as i32) as uint8_t;
            (*p_dev).temp_buffer[0 as i32 as usize] = 8u8;
            (*p_dev).temp_buffer[0x1 as i32 as usize] = 8u8;
            (*p_dev).temp_buffer[0x4 as i32 as usize] = 4u8;
            (*p_dev).temp_buffer[0x5 as i32 as usize] = 4u8;
            status = (status as i32
                | vl53l5cx_dci_write_data(
                    p_dev,
                    ((*p_dev).temp_buffer).as_mut_ptr(),
                    0x5450 as u32 as uint16_t as uint32_t,
                    8 as i32 as uint16_t,
                ) as i32) as uint8_t;
        }
        _ => {
            status = 127u8;
        }
    }
    status = (status as i32 | _vl53l5cx_send_offset_data(p_dev, resolution) as i32) as uint8_t;
    status = (status as i32 | _vl53l5cx_send_xtalk_data(p_dev, resolution) as i32) as uint8_t;
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn vl53l5cx_get_ranging_frequency_hz(
    mut p_dev: *mut VL53L5CX_Configuration,
    mut p_frequency_hz: *mut uint8_t,
) -> uint8_t {
    let mut status: uint8_t = 0u8;
    status = (status as i32
        | vl53l5cx_dci_read_data(
            p_dev,
            ((*p_dev).temp_buffer).as_mut_ptr(),
            0x5458 as u32 as uint16_t as uint32_t,
            4 as i32 as uint16_t,
        ) as i32) as uint8_t;
    *p_frequency_hz = (*p_dev).temp_buffer[0x1 as i32 as usize];
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn vl53l5cx_set_ranging_frequency_hz(
    mut p_dev: *mut VL53L5CX_Configuration,
    mut frequency_hz: uint8_t,
) -> uint8_t {
    let mut status: uint8_t = 0u8;
    status = (status as i32
        | vl53l5cx_dci_replace_data(
            p_dev,
            ((*p_dev).temp_buffer).as_mut_ptr(),
            0x5458 as u32 as uint16_t as uint32_t,
            4 as i32 as uint16_t,
            &mut frequency_hz as *mut uint8_t,
            1 as i32 as uint16_t,
            0x1 as i32 as uint16_t,
        ) as i32) as uint8_t;
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn vl53l5cx_get_integration_time_ms(
    mut p_dev: *mut VL53L5CX_Configuration,
    mut p_time_ms: *mut uint32_t,
) -> uint8_t {
    let mut status: uint8_t = 0u8;
    status = (status as i32
        | vl53l5cx_dci_read_data(
            p_dev,
            ((*p_dev).temp_buffer).as_mut_ptr(),
            0x545c as u32 as uint16_t as uint32_t,
            20 as i32 as uint16_t,
        ) as i32) as uint8_t;
    memcpy(
        p_time_ms as *mut (),
        &mut *((*p_dev).temp_buffer)
            .as_mut_ptr()
            .offset(0 as i32 as isize) as *mut uint8_t as *const (),
        4 as i32 as ulong,
    );
    *p_time_ms = (*p_time_ms as u32).wrapping_div(1000 as i32 as uint32_t) as uint32_t as uint32_t;
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn vl53l5cx_set_integration_time_ms(
    mut p_dev: *mut VL53L5CX_Configuration,
    mut integration_time_ms: uint32_t,
) -> uint8_t {
    let mut status: uint8_t = 0u8;
    let mut integration: uint32_t = integration_time_ms;
    if integration < 2 as i32 as uint32_t || integration > 1000 as i32 as uint32_t {
        status = (status as i32 | 127u8 as i32) as uint8_t;
    } else {
        integration =
            (integration as u32).wrapping_mul(1000 as i32 as uint32_t) as uint32_t as uint32_t;
        status = (status as i32
            | vl53l5cx_dci_replace_data(
                p_dev,
                ((*p_dev).temp_buffer).as_mut_ptr(),
                0x545c as u32 as uint16_t as uint32_t,
                20 as i32 as uint16_t,
                &mut integration as *mut uint32_t as *mut uint8_t,
                4 as i32 as uint16_t,
                0 as i32 as uint16_t,
            ) as i32) as uint8_t;
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn vl53l5cx_get_sharpener_percent(
    mut p_dev: *mut VL53L5CX_Configuration,
    mut p_sharpener_percent: *mut uint8_t,
) -> uint8_t {
    let mut status: uint8_t = 0u8;
    status = (status as i32
        | vl53l5cx_dci_read_data(
            p_dev,
            ((*p_dev).temp_buffer).as_mut_ptr(),
            0xaed8 as u32 as uint16_t as uint32_t,
            16 as i32 as uint16_t,
        ) as i32) as uint8_t;
    *p_sharpener_percent =
        ((*p_dev).temp_buffer[0xd as i32 as usize] as i32 * 100u8 as i32 / 255u8 as i32) as uint8_t;
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn vl53l5cx_set_sharpener_percent(
    mut p_dev: *mut VL53L5CX_Configuration,
    mut sharpener_percent: uint8_t,
) -> uint8_t {
    let mut status: uint8_t = 0u8;
    let mut sharpener: uint8_t = 0;
    if sharpener_percent as i32 >= 100u8 as i32 {
        status = (status as i32 | 127u8 as i32) as uint8_t;
    } else {
        sharpener = (sharpener_percent as i32 * 255u8 as i32 / 100u8 as i32) as uint8_t;
        status = (status as i32
            | vl53l5cx_dci_replace_data(
                p_dev,
                ((*p_dev).temp_buffer).as_mut_ptr(),
                0xaed8 as u32 as uint16_t as uint32_t,
                16 as i32 as uint16_t,
                &mut sharpener as *mut uint8_t,
                1 as i32 as uint16_t,
                0xd as i32 as uint16_t,
            ) as i32) as uint8_t;
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn vl53l5cx_get_target_order(
    mut p_dev: *mut VL53L5CX_Configuration,
    mut p_target_order: *mut uint8_t,
) -> uint8_t {
    let mut status: uint8_t = 0u8;
    status = (status as i32
        | vl53l5cx_dci_read_data(
            p_dev,
            ((*p_dev).temp_buffer).as_mut_ptr(),
            0xae64 as u32 as uint16_t as uint32_t,
            4 as i32 as uint16_t,
        ) as i32) as uint8_t;
    *p_target_order = (*p_dev).temp_buffer[0 as i32 as usize];
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn vl53l5cx_set_target_order(
    mut p_dev: *mut VL53L5CX_Configuration,
    mut target_order: uint8_t,
) -> uint8_t {
    let mut status: uint8_t = 0u8;
    if target_order as i32 == 1u8 as i32 || target_order as i32 == 2u8 as i32 {
        status = (status as i32
            | vl53l5cx_dci_replace_data(
                p_dev,
                ((*p_dev).temp_buffer).as_mut_ptr(),
                0xae64 as u32 as uint16_t as uint32_t,
                4 as i32 as uint16_t,
                &mut target_order as *mut uint8_t,
                1 as i32 as uint16_t,
                0 as i32 as uint16_t,
            ) as i32) as uint8_t;
    } else {
        status = (status as i32 | 127u8 as i32) as uint8_t;
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn vl53l5cx_get_ranging_mode(
    mut p_dev: *mut VL53L5CX_Configuration,
    mut p_ranging_mode: *mut uint8_t,
) -> uint8_t {
    let mut status: uint8_t = 0u8;
    status = (status as i32
        | vl53l5cx_dci_read_data(
            p_dev,
            ((*p_dev).temp_buffer).as_mut_ptr(),
            0xad30 as u32 as uint16_t as uint32_t,
            8 as i32 as uint16_t,
        ) as i32) as uint8_t;
    if (*p_dev).temp_buffer[0x1 as i32 as usize] as i32 == 0x1u8 as i32 {
        *p_ranging_mode = 1u8;
    } else {
        *p_ranging_mode = 3u8;
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn vl53l5cx_set_ranging_mode(
    mut p_dev: *mut VL53L5CX_Configuration,
    mut ranging_mode: uint8_t,
) -> uint8_t {
    let mut status: uint8_t = 0u8;
    let mut single_range: uint32_t = 0 as i32 as uint32_t;
    status = (status as i32
        | vl53l5cx_dci_read_data(
            p_dev,
            ((*p_dev).temp_buffer).as_mut_ptr(),
            0xad30 as u32 as uint16_t as uint32_t,
            8 as i32 as uint16_t,
        ) as i32) as uint8_t;
    match ranging_mode as i32 {
        1 => {
            (*p_dev).temp_buffer[0x1 as i32 as usize] = 0x1u8;
            (*p_dev).temp_buffer[0x3 as i32 as usize] = 0x3u8;
            single_range = 0 as i32 as uint32_t;
        }
        3 => {
            (*p_dev).temp_buffer[0x1 as i32 as usize] = 0x3u8;
            (*p_dev).temp_buffer[0x3 as i32 as usize] = 0x2u8;
            single_range = 0x1 as i32 as uint32_t;
        }
        _ => {
            status = 127u8;
        }
    }
    status = (status as i32
        | vl53l5cx_dci_write_data(
            p_dev,
            ((*p_dev).temp_buffer).as_mut_ptr(),
            0xad30 as u32 as uint16_t as uint32_t,
            8 as i32 as uint16_t,
        ) as i32) as uint8_t;
    status = (status as i32
        | vl53l5cx_dci_write_data(
            p_dev,
            &mut single_range as *mut uint32_t as *mut uint8_t,
            0xd964 as u32 as uint16_t as uint32_t,
            ::core::mem::size_of::<uint32_t>() as ulong as uint16_t,
        ) as i32) as uint8_t;
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn vl53l5cx_enable_internal_cp(
    mut p_dev: *mut VL53L5CX_Configuration,
) -> uint8_t {
    let mut status: uint8_t = 0u8;
    let mut vcsel_bootup_fsm: uint8_t = 1u8;
    let mut analog_dynamic_pad_0: uint8_t = 0u8;
    status = (status as i32
        | vl53l5cx_dci_replace_data(
            p_dev,
            ((*p_dev).temp_buffer).as_mut_ptr(),
            0xb39c as u32 as uint16_t as uint32_t,
            16 as i32 as uint16_t,
            &mut vcsel_bootup_fsm as *mut uint8_t,
            1 as i32 as uint16_t,
            0xa as i32 as uint16_t,
        ) as i32) as uint8_t;
    status = (status as i32
        | vl53l5cx_dci_replace_data(
            p_dev,
            ((*p_dev).temp_buffer).as_mut_ptr(),
            0xb39c as u32 as uint16_t as uint32_t,
            16 as i32 as uint16_t,
            &mut analog_dynamic_pad_0 as *mut uint8_t,
            1 as i32 as uint16_t,
            0xe as i32 as uint16_t,
        ) as i32) as uint8_t;
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn vl53l5cx_disable_internal_cp(
    mut p_dev: *mut VL53L5CX_Configuration,
) -> uint8_t {
    let mut status: uint8_t = 0u8;
    let mut vcsel_bootup_fsm: uint8_t = 0u8;
    let mut analog_dynamic_pad_0: uint8_t = 1u8;
    status = (status as i32
        | vl53l5cx_dci_replace_data(
            p_dev,
            ((*p_dev).temp_buffer).as_mut_ptr(),
            0xb39c as u32 as uint16_t as uint32_t,
            16 as i32 as uint16_t,
            &mut vcsel_bootup_fsm as *mut uint8_t,
            1 as i32 as uint16_t,
            0xa as i32 as uint16_t,
        ) as i32) as uint8_t;
    status = (status as i32
        | vl53l5cx_dci_replace_data(
            p_dev,
            ((*p_dev).temp_buffer).as_mut_ptr(),
            0xb39c as u32 as uint16_t as uint32_t,
            16 as i32 as uint16_t,
            &mut analog_dynamic_pad_0 as *mut uint8_t,
            1 as i32 as uint16_t,
            0xe as i32 as uint16_t,
        ) as i32) as uint8_t;
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn vl53l5cx_get_VHV_repeat_count(
    mut p_dev: *mut VL53L5CX_Configuration,
    mut p_repeat_count: *mut uint32_t,
) -> uint8_t {
    let mut status: uint8_t = 0u8;
    status = (status as i32
        | vl53l5cx_dci_read_data(
            p_dev,
            ((*p_dev).temp_buffer).as_mut_ptr(),
            0xad60 as u32 as uint16_t as uint32_t,
            16 as i32 as uint16_t,
        ) as i32) as uint8_t;
    *p_repeat_count = ((*p_dev).temp_buffer[7 as i32 as usize] as uint32_t) << 24 as i32
        | ((*p_dev).temp_buffer[6 as i32 as usize] as uint32_t) << 16 as i32
        | ((*p_dev).temp_buffer[5 as i32 as usize] as uint32_t) << 8 as i32
        | (*p_dev).temp_buffer[4 as i32 as usize] as uint32_t;
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn vl53l5cx_set_VHV_repeat_count(
    mut p_dev: *mut VL53L5CX_Configuration,
    mut repeat_count: uint32_t,
) -> uint8_t {
    let mut status: uint8_t = 0u8;
    status = (status as i32
        | vl53l5cx_dci_replace_data(
            p_dev,
            ((*p_dev).temp_buffer).as_mut_ptr(),
            0xad60 as u32 as uint16_t as uint32_t,
            16 as i32 as uint16_t,
            &mut repeat_count as *mut uint32_t as *mut uint8_t,
            4 as i32 as uint16_t,
            0x4 as i32 as uint16_t,
        ) as i32) as uint8_t;
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn vl53l5cx_dci_read_data(
    mut p_dev: *mut VL53L5CX_Configuration,
    mut data: *mut uint8_t,
    mut index: uint32_t,
    mut data_size: uint16_t,
) -> uint8_t {
    let mut i: int16_t = 0;
    let mut status: uint8_t = 0u8;
    let mut rd_size: uint32_t = (data_size as uint32_t).wrapping_add(12 as i32 as uint32_t);
    let mut cmd: [uint8_t; 12] = [
        0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0xfu8, 0u8, 0x2u8, 0u8, 0x8u8,
    ];
    if data_size as i32 + 12 as i32 as uint16_t as i32
        > (40 as u32)
            .wrapping_add(260 as u32)
            .wrapping_add(260 as u32)
            .wrapping_add(68 as u32)
            .wrapping_add((256 as u32).wrapping_mul(1 as u32).wrapping_add(4 as u32))
            .wrapping_add((128 as u32).wrapping_mul(1 as u32).wrapping_add(4 as u32))
            .wrapping_add((128 as u32).wrapping_mul(1 as u32).wrapping_add(4 as u32))
            .wrapping_add((64 as u32).wrapping_mul(1 as u32).wrapping_add(4 as u32))
            .wrapping_add((64 as u32).wrapping_mul(1 as u32).wrapping_add(4 as u32))
            .wrapping_add(144 as u32)
            .wrapping_add(20 as u32) as uint16_t as i32
    {
        status = (status as i32 | 255u8 as i32) as uint8_t;
    } else {
        cmd[0 as i32 as usize] = (index >> 8 as i32) as uint8_t;
        cmd[1 as i32 as usize] = (index & 0xff as i32 as uint32_t) as uint8_t;
        cmd[2 as i32 as usize] =
            ((data_size as i32 & 0xff0 as i32 as uint16_t as i32) >> 4 as i32) as uint8_t;
        cmd[3 as i32 as usize] =
            ((data_size as i32 & 0xf as i32 as uint16_t as i32) << 4 as i32) as uint8_t;
        status = (status as i32
            | WrMulti(
                &mut (*p_dev).platform,
                (0x2fff as u32 as uint16_t as i32 - 11 as i32 as uint16_t as i32) as uint16_t,
                cmd.as_mut_ptr(),
                ::core::mem::size_of::<[uint8_t; 12]>() as ulong as uint32_t,
            ) as i32) as uint8_t;
        status = (status as i32
            | _vl53l5cx_poll_for_answer(p_dev, 4u8, 1u8, 0x2c00 as u32 as uint16_t, 0xffu8, 0x3u8)
                as i32) as uint8_t;
        status = (status as i32
            | RdMulti(
                &mut (*p_dev).platform,
                0x2c04 as u32 as uint16_t,
                ((*p_dev).temp_buffer).as_mut_ptr(),
                rd_size,
            ) as i32) as uint8_t;
        SwapBuffer(
            ((*p_dev).temp_buffer).as_mut_ptr(),
            (data_size as i32 + 12 as i32 as uint16_t as i32) as uint16_t,
        );
        i = 0 as i32 as int16_t;
        while (i as i32) < data_size as int16_t as i32 {
            *data.offset(i as isize) = (*p_dev).temp_buffer[(i as i32 + 4 as i32) as usize];
            i += 1;
        }
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn vl53l5cx_dci_write_data(
    mut p_dev: *mut VL53L5CX_Configuration,
    mut data: *mut uint8_t,
    mut index: uint32_t,
    mut data_size: uint16_t,
) -> uint8_t {
    let mut status: uint8_t = 0u8;
    let mut i: int16_t = 0;
    let mut headers: [uint8_t; 4] = [0u8, 0u8, 0u8, 0u8];
    let mut footer: [uint8_t; 8] = [
        0u8,
        0u8,
        0u8,
        0xfu8,
        0x5u8,
        0x1u8,
        (data_size as i32 + 8 as i32 as uint16_t as i32 >> 8 as i32) as uint8_t,
        (data_size as i32 + 8 as i32 as uint16_t as i32 & 0xffu8 as i32) as uint8_t,
    ];
    let mut address: uint16_t = (0x2fff as u32 as uint16_t as i32
        - (data_size as i32 + 12 as i32 as uint16_t as i32)
        + 1 as i32 as uint16_t as i32) as uint16_t;
    if data_size as i32 + 12 as i32 as uint16_t as i32
        > (40 as u32)
            .wrapping_add(260 as u32)
            .wrapping_add(260 as u32)
            .wrapping_add(68 as u32)
            .wrapping_add((256 as u32).wrapping_mul(1 as u32).wrapping_add(4 as u32))
            .wrapping_add((128 as u32).wrapping_mul(1 as u32).wrapping_add(4 as u32))
            .wrapping_add((128 as u32).wrapping_mul(1 as u32).wrapping_add(4 as u32))
            .wrapping_add((64 as u32).wrapping_mul(1 as u32).wrapping_add(4 as u32))
            .wrapping_add((64 as u32).wrapping_mul(1 as u32).wrapping_add(4 as u32))
            .wrapping_add(144 as u32)
            .wrapping_add(20 as u32) as uint16_t as i32
    {
        status = (status as i32 | 255u8 as i32) as uint8_t;
    } else {
        headers[0 as i32 as usize] = (index >> 8 as i32) as uint8_t;
        headers[1 as i32 as usize] = (index & 0xff as i32 as uint32_t) as uint8_t;
        headers[2 as i32 as usize] =
            ((data_size as i32 & 0xff0 as i32 as uint16_t as i32) >> 4 as i32) as uint8_t;
        headers[3 as i32 as usize] =
            ((data_size as i32 & 0xf as i32 as uint16_t as i32) << 4 as i32) as uint8_t;
        SwapBuffer(data, data_size);
        i = (data_size as int16_t as i32 - 1 as i32 as int16_t as i32) as int16_t;
        while i as i32 >= 0 as i32 {
            (*p_dev).temp_buffer[(i as i32 + 4 as i32) as usize] = *data.offset(i as isize);
            i -= 1;
        }
        memcpy(
            &mut *((*p_dev).temp_buffer)
                .as_mut_ptr()
                .offset(0 as i32 as isize) as *mut uint8_t as *mut (),
            headers.as_mut_ptr() as *const (),
            ::core::mem::size_of::<[uint8_t; 4]>() as ulong,
        );
        memcpy(
            &mut *((*p_dev).temp_buffer)
                .as_mut_ptr()
                .offset((data_size as i32 + 4 as i32 as uint16_t as i32) as isize)
                as *mut uint8_t as *mut (),
            footer.as_mut_ptr() as *const (),
            ::core::mem::size_of::<[uint8_t; 8]>() as ulong,
        );
        status = (status as i32
            | WrMulti(
                &mut (*p_dev).platform,
                address,
                ((*p_dev).temp_buffer).as_mut_ptr(),
                (data_size as uint32_t).wrapping_add(12 as i32 as uint32_t),
            ) as i32) as uint8_t;
        status = (status as i32
            | _vl53l5cx_poll_for_answer(p_dev, 4u8, 1u8, 0x2c00 as u32 as uint16_t, 0xffu8, 0x3u8)
                as i32) as uint8_t;
        SwapBuffer(data, data_size);
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn vl53l5cx_dci_replace_data(
    mut p_dev: *mut VL53L5CX_Configuration,
    mut data: *mut uint8_t,
    mut index: uint32_t,
    mut data_size: uint16_t,
    mut new_data: *mut uint8_t,
    mut new_data_size: uint16_t,
    mut new_data_pos: uint16_t,
) -> uint8_t {
    let mut status: uint8_t = 0u8;
    status =
        (status as i32 | vl53l5cx_dci_read_data(p_dev, data, index, data_size) as i32) as uint8_t;
    memcpy(
        &mut *data.offset(new_data_pos as isize) as *mut uint8_t as *mut (),
        new_data as *const (),
        new_data_size as ulong,
    );
    status =
        (status as i32 | vl53l5cx_dci_write_data(p_dev, data, index, data_size) as i32) as uint8_t;
    return status;
}
