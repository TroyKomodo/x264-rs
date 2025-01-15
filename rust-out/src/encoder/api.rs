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
    fn x264_log_internal(i_level: libc::c_int, psz_fmt: *const libc::c_char, _: ...);
    fn x264_8_encoder_open(_: *mut x264_param_t, _: *mut libc::c_void) -> *mut x264_t;
    fn x264_8_nal_encode(h: *mut x264_t, dst: *mut uint8_t, nal: *mut x264_nal_t);
    fn x264_8_encoder_reconfig(_: *mut x264_t, _: *mut x264_param_t) -> libc::c_int;
    fn x264_8_encoder_parameters(_: *mut x264_t, _: *mut x264_param_t);
    fn x264_8_encoder_headers(
        _: *mut x264_t,
        pp_nal: *mut *mut x264_nal_t,
        pi_nal: *mut libc::c_int,
    ) -> libc::c_int;
    fn x264_8_encoder_encode(
        _: *mut x264_t,
        pp_nal: *mut *mut x264_nal_t,
        pi_nal: *mut libc::c_int,
        pic_in: *mut x264_picture_t,
        pic_out: *mut x264_picture_t,
    ) -> libc::c_int;
    fn x264_8_encoder_close(_: *mut x264_t);
    fn x264_8_encoder_delayed_frames(_: *mut x264_t) -> libc::c_int;
    fn x264_8_encoder_maximum_delayed_frames(_: *mut x264_t) -> libc::c_int;
    fn x264_8_encoder_intra_refresh(_: *mut x264_t);
    fn x264_8_encoder_invalidate_reference(_: *mut x264_t, pts: int64_t) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn x264_encoder_open_164(mut param: *mut x264_param_t) -> *mut x264_t {
    let mut api: *mut x264_api_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<x264_api_t>() as libc::c_ulong,
    ) as *mut x264_api_t;
    if api.is_null() {
        return std::ptr::null_mut::<x264_t>();
    }
    if (*param).i_bitdepth == 8 as libc::c_int {
        (*api).nal_encode = Some(
            x264_8_nal_encode
                as unsafe extern "C" fn(*mut x264_t, *mut uint8_t, *mut x264_nal_t) -> (),
        );
        (*api).encoder_reconfig = Some(
            x264_8_encoder_reconfig
                as unsafe extern "C" fn(*mut x264_t, *mut x264_param_t) -> libc::c_int,
        );
        (*api).encoder_parameters = Some(
            x264_8_encoder_parameters as unsafe extern "C" fn(*mut x264_t, *mut x264_param_t) -> (),
        );
        (*api).encoder_headers = Some(
            x264_8_encoder_headers
                as unsafe extern "C" fn(
                    *mut x264_t,
                    *mut *mut x264_nal_t,
                    *mut libc::c_int,
                ) -> libc::c_int,
        );
        (*api).encoder_encode = Some(
            x264_8_encoder_encode
                as unsafe extern "C" fn(
                    *mut x264_t,
                    *mut *mut x264_nal_t,
                    *mut libc::c_int,
                    *mut x264_picture_t,
                    *mut x264_picture_t,
                ) -> libc::c_int,
        );
        (*api).encoder_close =
            Some(x264_8_encoder_close as unsafe extern "C" fn(*mut x264_t) -> ());
        (*api).encoder_delayed_frames =
            Some(x264_8_encoder_delayed_frames as unsafe extern "C" fn(*mut x264_t) -> libc::c_int);
        (*api).encoder_maximum_delayed_frames = Some(
            x264_8_encoder_maximum_delayed_frames
                as unsafe extern "C" fn(*mut x264_t) -> libc::c_int,
        );
        (*api).encoder_intra_refresh =
            Some(x264_8_encoder_intra_refresh as unsafe extern "C" fn(*mut x264_t) -> ());
        (*api).encoder_invalidate_reference = Some(
            x264_8_encoder_invalidate_reference
                as unsafe extern "C" fn(*mut x264_t, int64_t) -> libc::c_int,
        );
        (*api).x264 = x264_8_encoder_open(param, api as *mut libc::c_void);
    } else {
        x264_log_internal(
            0 as libc::c_int,
            b"not compiled with %d bit depth support\n\0" as *const u8 as *const libc::c_char,
            (*param).i_bitdepth,
        );
    }
    if ((*api).x264).is_null() {
        free(api as *mut libc::c_void);
        return std::ptr::null_mut::<x264_t>();
    }
    api as *mut x264_t
}
#[no_mangle]
pub unsafe extern "C" fn x264_encoder_close(mut h: *mut x264_t) {
    let mut api: *mut x264_api_t = h as *mut x264_api_t;
    ((*api).encoder_close).expect("non-null function pointer")((*api).x264);
    free(api as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn x264_nal_encode(
    mut h: *mut x264_t,
    mut dst: *mut uint8_t,
    mut nal: *mut x264_nal_t,
) {
    let mut api: *mut x264_api_t = h as *mut x264_api_t;
    ((*api).nal_encode).expect("non-null function pointer")((*api).x264, dst, nal);
}
#[no_mangle]
pub unsafe extern "C" fn x264_encoder_reconfig(
    mut h: *mut x264_t,
    mut param: *mut x264_param_t,
) -> libc::c_int {
    let mut api: *mut x264_api_t = h as *mut x264_api_t;
    ((*api).encoder_reconfig).expect("non-null function pointer")((*api).x264, param)
}
#[no_mangle]
pub unsafe extern "C" fn x264_encoder_parameters(mut h: *mut x264_t, mut param: *mut x264_param_t) {
    let mut api: *mut x264_api_t = h as *mut x264_api_t;
    ((*api).encoder_parameters).expect("non-null function pointer")((*api).x264, param);
}
#[no_mangle]
pub unsafe extern "C" fn x264_encoder_headers(
    mut h: *mut x264_t,
    mut pp_nal: *mut *mut x264_nal_t,
    mut pi_nal: *mut libc::c_int,
) -> libc::c_int {
    let mut api: *mut x264_api_t = h as *mut x264_api_t;
    ((*api).encoder_headers).expect("non-null function pointer")((*api).x264, pp_nal, pi_nal)
}
#[no_mangle]
pub unsafe extern "C" fn x264_encoder_encode(
    mut h: *mut x264_t,
    mut pp_nal: *mut *mut x264_nal_t,
    mut pi_nal: *mut libc::c_int,
    mut pic_in: *mut x264_picture_t,
    mut pic_out: *mut x264_picture_t,
) -> libc::c_int {
    let mut api: *mut x264_api_t = h as *mut x264_api_t;
    ((*api).encoder_encode).expect("non-null function pointer")(
        (*api).x264,
        pp_nal,
        pi_nal,
        pic_in,
        pic_out,
    )
}
#[no_mangle]
pub unsafe extern "C" fn x264_encoder_delayed_frames(mut h: *mut x264_t) -> libc::c_int {
    let mut api: *mut x264_api_t = h as *mut x264_api_t;
    ((*api).encoder_delayed_frames).expect("non-null function pointer")((*api).x264)
}
#[no_mangle]
pub unsafe extern "C" fn x264_encoder_maximum_delayed_frames(mut h: *mut x264_t) -> libc::c_int {
    let mut api: *mut x264_api_t = h as *mut x264_api_t;
    ((*api).encoder_maximum_delayed_frames).expect("non-null function pointer")((*api).x264)
}
#[no_mangle]
pub unsafe extern "C" fn x264_encoder_intra_refresh(mut h: *mut x264_t) {
    let mut api: *mut x264_api_t = h as *mut x264_api_t;
    ((*api).encoder_intra_refresh).expect("non-null function pointer")((*api).x264);
}
#[no_mangle]
pub unsafe extern "C" fn x264_encoder_invalidate_reference(
    mut h: *mut x264_t,
    mut pts: int64_t,
) -> libc::c_int {
    let mut api: *mut x264_api_t = h as *mut x264_api_t;
    ((*api).encoder_invalidate_reference).expect("non-null function pointer")((*api).x264, pts)
}
