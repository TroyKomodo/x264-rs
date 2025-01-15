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
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn x264_cli_log(
        name: *const libc::c_char,
        i_level: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
}
unsafe extern "C" fn register_vid_filter(mut new_filter: *mut cli_vid_filter_t) {
    let mut filter_i: *mut cli_vid_filter_t = first_filter;
    while !((*filter_i).next).is_null() {
        filter_i = (*filter_i).next;
    }
    (*filter_i).next = new_filter;
    (*new_filter).next = std::ptr::null_mut::<cli_vid_filter_t>();
}
#[no_mangle]
pub unsafe extern "C" fn x264_register_vid_filters() {
    extern "C" {
        static mut source_filter: cli_vid_filter_t;
    }
    first_filter = &mut source_filter;
    extern "C" {
        static mut cache_8_filter: cli_vid_filter_t;
    }
    register_vid_filter(&mut cache_8_filter);
    extern "C" {
        static mut depth_8_filter: cli_vid_filter_t;
    }
    register_vid_filter(&mut depth_8_filter);
    extern "C" {
        static mut crop_filter: cli_vid_filter_t;
    }
    register_vid_filter(&mut crop_filter);
    extern "C" {
        static mut fix_vfr_pts_filter: cli_vid_filter_t;
    }
    register_vid_filter(&mut fix_vfr_pts_filter);
    extern "C" {
        static mut resize_filter: cli_vid_filter_t;
    }
    register_vid_filter(&mut resize_filter);
    extern "C" {
        static mut select_every_filter: cli_vid_filter_t;
    }
    register_vid_filter(&mut select_every_filter);
}
#[no_mangle]
pub unsafe extern "C" fn x264_init_vid_filter(
    mut name: *const libc::c_char,
    mut handle: *mut hnd_t,
    mut filter_v: *mut cli_vid_filter_t,
    mut info: *mut video_info_t,
    mut param: *mut x264_param_t,
    mut opt_string: *mut libc::c_char,
) -> libc::c_int {
    let mut filter_i: *mut cli_vid_filter_t = first_filter;
    while !filter_i.is_null() && strcasecmp(name, (*filter_i).name) != 0 {
        filter_i = (*filter_i).next;
    }
    if filter_i.is_null() {
        x264_cli_log(
            b"x264\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            b"invalid filter `%s'\n\0" as *const u8 as *const libc::c_char,
            name,
        );
        return -(1 as libc::c_int);
    }
    if ((*filter_i).init).expect("non-null function pointer")(
        handle, filter_v, info, param, opt_string,
    ) != 0
    {
        return -(1 as libc::c_int);
    }
    0 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn x264_vid_filter_help(mut longhelp: libc::c_int) {
    let mut filter_i: *mut cli_vid_filter_t = first_filter;
    while !filter_i.is_null() {
        if ((*filter_i).help).is_some() {
            ((*filter_i).help).expect("non-null function pointer")(longhelp);
        }
        filter_i = (*filter_i).next;
    }
}
