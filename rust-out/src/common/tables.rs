#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use crate::types::*;

#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
pub static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];

unsafe extern "C" fn run_static_initializers() {
    x264_dct8_weight_tab = [
        (if 0 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 0 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 0 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 0 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 0 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 0 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 3 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 4 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 3 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 0 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 0 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 0 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 0 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 0 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 0 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 3 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 4 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 3 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 3 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 1 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 5 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 1 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 3 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 1 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 5 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 1 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 4 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 5 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 2 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 2 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 2 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 2 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 2 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 2 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 5 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 4 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 5 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 2 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 2 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 2 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 2 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 2 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 2 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 5 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 3 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 1 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 5 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 1 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 3 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 1 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 5 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 1 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 0 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 0 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 0 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 0 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 0 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 0 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 3 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 4 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 3 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 0 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 0 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 0 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 0 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 0 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 0 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 3 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 4 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 3 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 3 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 1 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 5 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 1 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 3 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 1 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 5 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 1 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 4 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 5 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 2 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 2 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 2 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 2 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 2 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 2 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 5 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 4 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 5 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 2 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 2 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 2 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 2 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 2 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 2 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 5 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 3 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 1 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 5 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 1 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 3 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 1 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 5 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 1 as libc::c_int == 0 as libc::c_int {
            (1.0000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 1 as libc::c_int {
            (0.8859f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 2 as libc::c_int {
            (1.6000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 3 as libc::c_int {
            (0.9415f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 4 as libc::c_int {
            (1.2651f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 5 as libc::c_int {
            (1.1910f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
    ];
    x264_dct4_weight_tab = [
        (if 0 as libc::c_int == 0 as libc::c_int {
            (1.76777f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 0 as libc::c_int == 1 as libc::c_int {
            (1.11803f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 0 as libc::c_int == 2 as libc::c_int {
            (0.70711f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 1 as libc::c_int == 0 as libc::c_int {
            (1.76777f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 1 as libc::c_int {
            (1.11803f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 2 as libc::c_int {
            (0.70711f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 0 as libc::c_int == 0 as libc::c_int {
            (1.76777f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 0 as libc::c_int == 1 as libc::c_int {
            (1.11803f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 0 as libc::c_int == 2 as libc::c_int {
            (0.70711f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 1 as libc::c_int == 0 as libc::c_int {
            (1.76777f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 1 as libc::c_int {
            (1.11803f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 2 as libc::c_int {
            (0.70711f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 1 as libc::c_int == 0 as libc::c_int {
            (1.76777f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 1 as libc::c_int {
            (1.11803f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 2 as libc::c_int {
            (0.70711f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 2 as libc::c_int == 0 as libc::c_int {
            (1.76777f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 2 as libc::c_int == 1 as libc::c_int {
            (1.11803f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 2 as libc::c_int == 2 as libc::c_int {
            (0.70711f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 1 as libc::c_int == 0 as libc::c_int {
            (1.76777f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 1 as libc::c_int {
            (1.11803f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 2 as libc::c_int {
            (0.70711f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 2 as libc::c_int == 0 as libc::c_int {
            (1.76777f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 2 as libc::c_int == 1 as libc::c_int {
            (1.11803f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 2 as libc::c_int == 2 as libc::c_int {
            (0.70711f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 0 as libc::c_int == 0 as libc::c_int {
            (1.76777f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 0 as libc::c_int == 1 as libc::c_int {
            (1.11803f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 0 as libc::c_int == 2 as libc::c_int {
            (0.70711f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 1 as libc::c_int == 0 as libc::c_int {
            (1.76777f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 1 as libc::c_int {
            (1.11803f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 2 as libc::c_int {
            (0.70711f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 0 as libc::c_int == 0 as libc::c_int {
            (1.76777f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 0 as libc::c_int == 1 as libc::c_int {
            (1.11803f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 0 as libc::c_int == 2 as libc::c_int {
            (0.70711f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 1 as libc::c_int == 0 as libc::c_int {
            (1.76777f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 1 as libc::c_int {
            (1.11803f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 2 as libc::c_int {
            (0.70711f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 1 as libc::c_int == 0 as libc::c_int {
            (1.76777f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 1 as libc::c_int {
            (1.11803f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 2 as libc::c_int {
            (0.70711f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 2 as libc::c_int == 0 as libc::c_int {
            (1.76777f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 2 as libc::c_int == 1 as libc::c_int {
            (1.11803f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 2 as libc::c_int == 2 as libc::c_int {
            (0.70711f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 1 as libc::c_int == 0 as libc::c_int {
            (1.76777f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 1 as libc::c_int {
            (1.11803f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 2 as libc::c_int {
            (0.70711f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 2 as libc::c_int == 0 as libc::c_int {
            (1.76777f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 2 as libc::c_int == 1 as libc::c_int {
            (1.11803f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 2 as libc::c_int == 2 as libc::c_int {
            (0.70711f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
    ];
    x264_dct4_weight2_tab = [
        (if 0 as libc::c_int == 0 as libc::c_int {
            (3.125f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 0 as libc::c_int == 1 as libc::c_int {
            (1.25f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 0 as libc::c_int == 2 as libc::c_int {
            (0.5f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 1 as libc::c_int == 0 as libc::c_int {
            (3.125f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 1 as libc::c_int {
            (1.25f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 2 as libc::c_int {
            (0.5f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 0 as libc::c_int == 0 as libc::c_int {
            (3.125f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 0 as libc::c_int == 1 as libc::c_int {
            (1.25f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 0 as libc::c_int == 2 as libc::c_int {
            (0.5f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 1 as libc::c_int == 0 as libc::c_int {
            (3.125f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 1 as libc::c_int {
            (1.25f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 2 as libc::c_int {
            (0.5f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 1 as libc::c_int == 0 as libc::c_int {
            (3.125f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 1 as libc::c_int {
            (1.25f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 2 as libc::c_int {
            (0.5f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 2 as libc::c_int == 0 as libc::c_int {
            (3.125f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 2 as libc::c_int == 1 as libc::c_int {
            (1.25f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 2 as libc::c_int == 2 as libc::c_int {
            (0.5f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 1 as libc::c_int == 0 as libc::c_int {
            (3.125f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 1 as libc::c_int {
            (1.25f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 2 as libc::c_int {
            (0.5f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 2 as libc::c_int == 0 as libc::c_int {
            (3.125f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 2 as libc::c_int == 1 as libc::c_int {
            (1.25f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 2 as libc::c_int == 2 as libc::c_int {
            (0.5f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 0 as libc::c_int == 0 as libc::c_int {
            (3.125f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 0 as libc::c_int == 1 as libc::c_int {
            (1.25f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 0 as libc::c_int == 2 as libc::c_int {
            (0.5f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 1 as libc::c_int == 0 as libc::c_int {
            (3.125f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 1 as libc::c_int {
            (1.25f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 2 as libc::c_int {
            (0.5f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 0 as libc::c_int == 0 as libc::c_int {
            (3.125f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 0 as libc::c_int == 1 as libc::c_int {
            (1.25f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 0 as libc::c_int == 2 as libc::c_int {
            (0.5f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 1 as libc::c_int == 0 as libc::c_int {
            (3.125f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 1 as libc::c_int {
            (1.25f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 2 as libc::c_int {
            (0.5f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 1 as libc::c_int == 0 as libc::c_int {
            (3.125f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 1 as libc::c_int {
            (1.25f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 2 as libc::c_int {
            (0.5f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 2 as libc::c_int == 0 as libc::c_int {
            (3.125f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 2 as libc::c_int == 1 as libc::c_int {
            (1.25f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 2 as libc::c_int == 2 as libc::c_int {
            (0.5f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 1 as libc::c_int == 0 as libc::c_int {
            (3.125f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 1 as libc::c_int {
            (1.25f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 2 as libc::c_int {
            (0.5f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 2 as libc::c_int == 0 as libc::c_int {
            (3.125f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 2 as libc::c_int == 1 as libc::c_int {
            (1.25f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 2 as libc::c_int == 2 as libc::c_int {
            (0.5f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
    ];
    x264_dct8_weight2_tab = [
        (if 0 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 0 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 0 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 0 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 0 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 0 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 3 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 4 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 3 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 0 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 0 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 0 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 0 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 0 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 0 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 3 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 4 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 3 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 3 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 1 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 5 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 1 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 3 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 1 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 5 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 1 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 4 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 5 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 2 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 2 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 2 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 2 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 2 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 2 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 5 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 4 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 5 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 2 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 2 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 2 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 2 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 2 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 2 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 5 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 3 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 1 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 5 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 1 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 3 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 1 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 5 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 1 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 0 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 0 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 0 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 0 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 0 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 0 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 3 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 4 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 3 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 0 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 0 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 0 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 0 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 0 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 0 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 3 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 4 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 3 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 3 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 1 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 5 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 1 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 3 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 1 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 5 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 1 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 4 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 5 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 2 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 2 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 2 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 2 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 2 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 2 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 5 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 4 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 4 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 5 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 2 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 2 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 2 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 2 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 2 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 2 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 5 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 3 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 1 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 5 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 1 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 3 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 3 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 1 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 5 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 5 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
        (if 1 as libc::c_int == 0 as libc::c_int {
            (1.00000f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 1 as libc::c_int {
            (0.78487f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 2 as libc::c_int {
            (2.56132f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 3 as libc::c_int {
            (0.88637f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 4 as libc::c_int {
            (1.60040f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else if 1 as libc::c_int == 5 as libc::c_int {
            (1.41850f64 * ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_double + 0.5f64)
                as libc::c_int
        } else {
            0 as libc::c_int
        }) as uint32_t,
    ];
}
