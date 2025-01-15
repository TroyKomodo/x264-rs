#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(c_variadic, extern_types)]
use crate::types::*;
extern "C" {
    fn x264_log_default(
        p_unused: *mut libc::c_void,
        i_level: libc::c_int,
        psz_fmt: *const libc::c_char,
        arg: ::core::ffi::VaList,
    );
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_log(
    mut h: *mut x264_t,
    mut i_level: libc::c_int,
    mut psz_fmt: *const libc::c_char,
    mut args: ...
) {
    if h.is_null() || i_level <= (*h).param.i_log_level {
        let mut arg: ::core::ffi::VaListImpl;
        arg = args.clone();
        if h.is_null() {
            x264_log_default(
                std::ptr::null_mut::<libc::c_void>(),
                i_level,
                psz_fmt,
                arg.as_va_list(),
            );
        } else {
            ((*h).param.pf_log).expect("non-null function pointer")(
                (*h).param.p_log_private,
                i_level,
                psz_fmt,
                arg.as_va_list(),
            );
        }
    }
}
