#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]
use crate::types::*;
extern "C" {
    static mut stdout: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
}
pub unsafe extern "C" fn open_file(
    mut psz_filename: *mut libc::c_char,
    mut p_handle: *mut hnd_t,
    mut opt: *mut cli_output_opt_t,
) -> libc::c_int {
    if strcmp(psz_filename, b"-\0" as *const u8 as *const libc::c_char) == 0 {
        *p_handle = stdout as hnd_t;
    } else {
        *p_handle = fopen(psz_filename, b"w+b\0" as *const u8 as *const libc::c_char) as hnd_t;
        if (*p_handle).is_null() {
            return -(1 as libc::c_int);
        }
    }
    0 as libc::c_int
}
pub unsafe extern "C" fn set_param(
    mut handle: hnd_t,
    mut p_param: *mut x264_param_t,
) -> libc::c_int {
    0 as libc::c_int
}
pub unsafe extern "C" fn write_headers(
    mut handle: hnd_t,
    mut p_nal: *mut x264_nal_t,
) -> libc::c_int {
    let mut size: libc::c_int = (*p_nal.offset(0 as libc::c_int as isize)).i_payload
        + (*p_nal.offset(1 as libc::c_int as isize)).i_payload
        + (*p_nal.offset(2 as libc::c_int as isize)).i_payload;
    if fwrite(
        (*p_nal.offset(0 as libc::c_int as isize)).p_payload as *const libc::c_void,
        size as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        handle as *mut FILE,
    ) != 0
    {
        return size;
    }
    -(1 as libc::c_int)
}
pub unsafe extern "C" fn write_frame(
    mut handle: hnd_t,
    mut p_nalu: *mut uint8_t,
    mut i_size: libc::c_int,
    mut p_picture: *mut x264_picture_t,
) -> libc::c_int {
    if fwrite(
        p_nalu as *const libc::c_void,
        i_size as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        handle as *mut FILE,
    ) != 0
    {
        return i_size;
    }
    -(1 as libc::c_int)
}
pub unsafe extern "C" fn close_file(
    mut handle: hnd_t,
    mut largest_pts: int64_t,
    mut second_largest_pts: int64_t,
) -> libc::c_int {
    if handle.is_null() || handle == stdout as hnd_t {
        return 0 as libc::c_int;
    }
    fclose(handle as *mut FILE)
}
