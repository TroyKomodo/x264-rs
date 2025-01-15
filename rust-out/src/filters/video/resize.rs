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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn x264_cli_log(
        name: *const libc::c_char,
        i_level: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
}
unsafe extern "C" fn full_check(
    mut info: *mut video_info_t,
    mut param: *mut x264_param_t,
) -> libc::c_int {
    let mut required: libc::c_int = 0 as libc::c_int;
    required |= ((*info).csp != (*param).i_csp) as libc::c_int;
    required |= ((*info).width != (*param).i_width) as libc::c_int;
    required |= ((*info).height != (*param).i_height) as libc::c_int;
    required |= ((*info).fullrange != (*param).vui.b_fullrange) as libc::c_int;
    required
}
pub unsafe extern "C" fn init(
    mut handle: *mut hnd_t,
    mut filter_v: *mut cli_vid_filter_t,
    mut info: *mut video_info_t,
    mut param: *mut x264_param_t,
    mut opt_string: *mut libc::c_char,
) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    if opt_string.is_null() {
        ret = full_check(info, param);
    } else if strcmp(opt_string, b"normcsp\0" as *const u8 as *const libc::c_char) == 0 {
        ret = (*info).csp & 0x4000 as libc::c_int;
    } else {
        ret = -(1 as libc::c_int);
    }
    if ret != 0 {
        x264_cli_log(
            b"resize\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            b"not compiled with swscale support\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    0 as libc::c_int
}
