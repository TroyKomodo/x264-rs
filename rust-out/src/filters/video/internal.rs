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
extern "C" {
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn x264_cli_log(
        name: *const libc::c_char,
        i_level: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    fn x264_cli_csp_is_invalid(csp: libc::c_int) -> libc::c_int;
    fn x264_cli_csp_depth_factor(csp: libc::c_int) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn x264_cli_plane_copy(
    mut dst: *mut uint8_t,
    mut i_dst: libc::c_int,
    mut src: *mut uint8_t,
    mut i_src: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
) {
    loop {
        let fresh0 = h;
        h -= 1;
        if fresh0 == 0 {
            break;
        }
        memcpy(
            dst as *mut libc::c_void,
            src as *const libc::c_void,
            w as libc::c_ulong,
        );
        dst = dst.offset(i_dst as isize);
        src = src.offset(i_src as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_cli_pic_copy(
    mut out: *mut cli_pic_t,
    mut in_0: *mut cli_pic_t,
) -> libc::c_int {
    let mut csp: libc::c_int = (*in_0).img.csp & 0xff as libc::c_int;
    if x264_cli_csp_is_invalid((*in_0).img.csp) != 0 {
        x264_cli_log(
            b"x264\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            b"invalid colorspace arg %d\n\0" as *const u8 as *const libc::c_char,
            (*in_0).img.csp,
        );
        return -(1 as libc::c_int);
    }
    if (*in_0).img.csp != (*out).img.csp
        || (*in_0).img.height != (*out).img.height
        || (*in_0).img.width != (*out).img.width
    {
        x264_cli_log(
            b"x264\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            b"incompatible frame properties\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    (*out).duration = (*in_0).duration;
    (*out).pts = (*in_0).pts;
    (*out).opaque = (*in_0).opaque;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*out).img.planes {
        let mut height: libc::c_int = ((*in_0).img.height as libc::c_float
            * (*x264_cli_csps.as_ptr().offset(csp as isize)).height[i as usize])
            as libc::c_int;
        let mut width: libc::c_int = ((*in_0).img.width as libc::c_float
            * (*x264_cli_csps.as_ptr().offset(csp as isize)).width[i as usize])
            as libc::c_int;
        width *= x264_cli_csp_depth_factor((*in_0).img.csp);
        x264_cli_plane_copy(
            (*out).img.plane[i as usize],
            (*out).img.stride[i as usize],
            (*in_0).img.plane[i as usize],
            (*in_0).img.stride[i as usize],
            width,
            height,
        );
        i += 1;
        i;
    }
    0 as libc::c_int
}
