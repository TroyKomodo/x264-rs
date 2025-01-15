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
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
pub unsafe extern "C" fn init(
    mut handle: *mut hnd_t,
    mut filter_v: *mut cli_vid_filter_t,
    mut info: *mut video_info_t,
    mut param: *mut x264_param_t,
    mut opt_string: *mut libc::c_char,
) -> libc::c_int {
    let mut h: *mut source_hnd_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<source_hnd_t>() as libc::c_ulong,
    ) as *mut source_hnd_t;
    if h.is_null() {
        return -(1 as libc::c_int);
    }
    (*h).cur_frame = -(1 as libc::c_int);
    if (cli_input.picture_alloc).expect("non-null function pointer")(
        &mut (*h).pic,
        *handle,
        (*info).csp,
        (*info).width,
        (*info).height,
    ) != 0
    {
        return -(1 as libc::c_int);
    }
    (*h).hin = *handle;
    *handle = h as hnd_t;
    *filter_v = source_filter;
    0 as libc::c_int
}
pub unsafe extern "C" fn get_frame(
    mut handle: hnd_t,
    mut output: *mut cli_pic_t,
    mut frame: libc::c_int,
) -> libc::c_int {
    let mut h: *mut source_hnd_t = handle as *mut source_hnd_t;
    if frame <= (*h).cur_frame
        || (cli_input.read_frame).expect("non-null function pointer")(
            &mut (*h).pic,
            (*h).hin,
            frame,
        ) != 0
    {
        return -(1 as libc::c_int);
    }
    (*h).cur_frame = frame;
    *output = (*h).pic;
    0 as libc::c_int
}
pub unsafe extern "C" fn release_frame(
    mut handle: hnd_t,
    mut pic: *mut cli_pic_t,
    mut frame: libc::c_int,
) -> libc::c_int {
    let mut h: *mut source_hnd_t = handle as *mut source_hnd_t;
    if (cli_input.release_frame).is_some()
        && (cli_input.release_frame).expect("non-null function pointer")(&mut (*h).pic, (*h).hin)
            != 0
    {
        return -(1 as libc::c_int);
    }
    0 as libc::c_int
}
pub unsafe extern "C" fn free_filter(mut handle: hnd_t) {
    let mut h: *mut source_hnd_t = handle as *mut source_hnd_t;
    (cli_input.picture_clean).expect("non-null function pointer")(&mut (*h).pic, (*h).hin);
    (cli_input.close_file).expect("non-null function pointer")((*h).hin);
    free(h as *mut libc::c_void);
}
