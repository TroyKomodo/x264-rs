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
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn mk_create_writer(filename: *const libc::c_char) -> *mut mk_writer;
    fn mk_write_header(
        w: *mut mk_writer,
        writing_app: *const libc::c_char,
        codec_id: *const libc::c_char,
        codec_private: *const libc::c_void,
        codec_private_size: libc::c_uint,
        default_frame_duration: int64_t,
        timescale: int64_t,
        width: libc::c_uint,
        height: libc::c_uint,
        d_width: libc::c_uint,
        d_height: libc::c_uint,
        display_size_units: libc::c_int,
        stereo_mode: libc::c_int,
    ) -> libc::c_int;
    fn mk_start_frame(w: *mut mk_writer) -> libc::c_int;
    fn mk_add_frame_data(
        w: *mut mk_writer,
        data: *const libc::c_void,
        size: libc::c_uint,
    ) -> libc::c_int;
    fn mk_set_frame_flags(
        w: *mut mk_writer,
        timestamp: int64_t,
        keyframe: libc::c_int,
        skippable: libc::c_int,
    ) -> libc::c_int;
    fn mk_close(w: *mut mk_writer, last_delta: int64_t) -> libc::c_int;
}
pub unsafe extern "C" fn open_file(
    mut psz_filename: *mut libc::c_char,
    mut p_handle: *mut hnd_t,
    mut opt: *mut cli_output_opt_t,
) -> libc::c_int {
    *p_handle = std::ptr::null_mut::<libc::c_void>();
    let mut p_mkv: *mut mkv_hnd_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<mkv_hnd_t>() as libc::c_ulong,
    ) as *mut mkv_hnd_t;
    if p_mkv.is_null() {
        return -(1 as libc::c_int);
    }
    (*p_mkv).w = mk_create_writer(psz_filename);
    if ((*p_mkv).w).is_null() {
        free(p_mkv as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    *p_handle = p_mkv as hnd_t;
    0 as libc::c_int
}
pub unsafe extern "C" fn set_param(
    mut handle: hnd_t,
    mut p_param: *mut x264_param_t,
) -> libc::c_int {
    let mut p_mkv: *mut mkv_hnd_t = handle as *mut mkv_hnd_t;
    let mut dw: int64_t = 0;
    let mut dh: int64_t = 0;
    if (*p_param).i_fps_num > 0 as libc::c_int as uint32_t && (*p_param).b_vfr_input == 0 {
        (*p_mkv).frame_duration = (*p_param).i_fps_den as int64_t
            * 1000000000 as libc::c_int as int64_t
            / (*p_param).i_fps_num as int64_t;
    } else {
        (*p_mkv).frame_duration = 0 as libc::c_int as int64_t;
    }
    (*p_mkv).width = (*p_param).i_width;
    dw = (*p_mkv).width as int64_t;
    (*p_mkv).height = (*p_param).i_height;
    dh = (*p_mkv).height as int64_t;
    (*p_mkv).display_size_units = 0 as libc::c_int;
    (*p_mkv).stereo_mode = -(1 as libc::c_int);
    if (*p_param).i_frame_packing >= 0 as libc::c_int
        && (*p_param).i_frame_packing < 7 as libc::c_int
    {
        (*p_mkv).stereo_mode = stereo_modes[(*p_param).i_frame_packing as usize] as libc::c_int;
        dw /= stereo_w_div[(*p_param).i_frame_packing as usize] as int64_t;
        dh /= stereo_h_div[(*p_param).i_frame_packing as usize] as int64_t;
    }
    if (*p_param).vui.i_sar_width != 0
        && (*p_param).vui.i_sar_height != 0
        && (*p_param).vui.i_sar_width != (*p_param).vui.i_sar_height
    {
        if (*p_param).vui.i_sar_width > (*p_param).vui.i_sar_height {
            dw =
                dw * (*p_param).vui.i_sar_width as int64_t / (*p_param).vui.i_sar_height as int64_t;
        } else {
            dh =
                dh * (*p_param).vui.i_sar_height as int64_t / (*p_param).vui.i_sar_width as int64_t;
        }
    }
    (*p_mkv).d_width = dw as libc::c_int;
    (*p_mkv).d_height = dh as libc::c_int;
    (*p_mkv).i_timebase_num = (*p_param).i_timebase_num;
    (*p_mkv).i_timebase_den = (*p_param).i_timebase_den;
    0 as libc::c_int
}
pub unsafe extern "C" fn write_headers(
    mut handle: hnd_t,
    mut p_nal: *mut x264_nal_t,
) -> libc::c_int {
    let mut p_mkv: *mut mkv_hnd_t = handle as *mut mkv_hnd_t;
    let mut sps_size: libc::c_int =
        (*p_nal.offset(0 as libc::c_int as isize)).i_payload - 4 as libc::c_int;
    let mut pps_size: libc::c_int =
        (*p_nal.offset(1 as libc::c_int as isize)).i_payload - 4 as libc::c_int;
    let mut sei_size: libc::c_int = (*p_nal.offset(2 as libc::c_int as isize)).i_payload;
    let mut sps: *mut uint8_t =
        ((*p_nal.offset(0 as libc::c_int as isize)).p_payload).offset(4 as libc::c_int as isize);
    let mut pps: *mut uint8_t =
        ((*p_nal.offset(1 as libc::c_int as isize)).p_payload).offset(4 as libc::c_int as isize);
    let mut sei: *mut uint8_t = (*p_nal.offset(2 as libc::c_int as isize)).p_payload;
    let mut ret: libc::c_int = 0;
    let mut avcC: *mut uint8_t = std::ptr::null_mut::<uint8_t>();
    let mut avcC_len: libc::c_int = 0;
    if (*p_mkv).width == 0
        || (*p_mkv).height == 0
        || (*p_mkv).d_width == 0
        || (*p_mkv).d_height == 0
    {
        return -(1 as libc::c_int);
    }
    avcC_len = 5 as libc::c_int
        + 1 as libc::c_int
        + 2 as libc::c_int
        + sps_size
        + 1 as libc::c_int
        + 2 as libc::c_int
        + pps_size;
    avcC = malloc(avcC_len as libc::c_ulong) as *mut uint8_t;
    if avcC.is_null() {
        return -(1 as libc::c_int);
    }
    *avcC.offset(0 as libc::c_int as isize) = 1 as libc::c_int as uint8_t;
    *avcC.offset(1 as libc::c_int as isize) = *sps.offset(1 as libc::c_int as isize);
    *avcC.offset(2 as libc::c_int as isize) = *sps.offset(2 as libc::c_int as isize);
    *avcC.offset(3 as libc::c_int as isize) = *sps.offset(3 as libc::c_int as isize);
    *avcC.offset(4 as libc::c_int as isize) = 0xff as libc::c_int as uint8_t;
    *avcC.offset(5 as libc::c_int as isize) = 0xe1 as libc::c_int as uint8_t;
    *avcC.offset(6 as libc::c_int as isize) = (sps_size >> 8 as libc::c_int) as uint8_t;
    *avcC.offset(7 as libc::c_int as isize) = sps_size as uint8_t;
    memcpy(
        avcC.offset(8 as libc::c_int as isize) as *mut libc::c_void,
        sps as *const libc::c_void,
        sps_size as libc::c_ulong,
    );
    *avcC.offset((8 as libc::c_int + sps_size) as isize) = 1 as libc::c_int as uint8_t;
    *avcC.offset((9 as libc::c_int + sps_size) as isize) =
        (pps_size >> 8 as libc::c_int) as uint8_t;
    *avcC.offset((10 as libc::c_int + sps_size) as isize) = pps_size as uint8_t;
    memcpy(
        avcC.offset(11 as libc::c_int as isize)
            .offset(sps_size as isize) as *mut libc::c_void,
        pps as *const libc::c_void,
        pps_size as libc::c_ulong,
    );
    ret = mk_write_header(
        (*p_mkv).w,
        b"x264 r3204 373697b\0" as *const u8 as *const libc::c_char,
        b"V_MPEG4/ISO/AVC\0" as *const u8 as *const libc::c_char,
        avcC as *const libc::c_void,
        avcC_len as libc::c_uint,
        (*p_mkv).frame_duration,
        50000 as libc::c_int as int64_t,
        (*p_mkv).width as libc::c_uint,
        (*p_mkv).height as libc::c_uint,
        (*p_mkv).d_width as libc::c_uint,
        (*p_mkv).d_height as libc::c_uint,
        (*p_mkv).display_size_units,
        (*p_mkv).stereo_mode,
    );
    free(avcC as *mut libc::c_void);
    if ret < 0 as libc::c_int {
        return ret;
    }
    if (*p_mkv).b_writing_frame == 0 {
        if mk_start_frame((*p_mkv).w) < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        (*p_mkv).b_writing_frame = 1 as libc::c_int as libc::c_char;
    }
    if mk_add_frame_data(
        (*p_mkv).w,
        sei as *const libc::c_void,
        sei_size as libc::c_uint,
    ) < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    sei_size + sps_size + pps_size
}
pub unsafe extern "C" fn write_frame(
    mut handle: hnd_t,
    mut p_nalu: *mut uint8_t,
    mut i_size: libc::c_int,
    mut p_picture: *mut x264_picture_t,
) -> libc::c_int {
    let mut p_mkv: *mut mkv_hnd_t = handle as *mut mkv_hnd_t;
    if (*p_mkv).b_writing_frame == 0 {
        if mk_start_frame((*p_mkv).w) < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        (*p_mkv).b_writing_frame = 1 as libc::c_int as libc::c_char;
    }
    if mk_add_frame_data(
        (*p_mkv).w,
        p_nalu as *const libc::c_void,
        i_size as libc::c_uint,
    ) < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    let mut i_stamp: int64_t =
        ((*p_picture).i_pts as libc::c_double * 1e9f64 * (*p_mkv).i_timebase_num as libc::c_double
            / (*p_mkv).i_timebase_den as libc::c_double
            + 0.5f64) as int64_t;
    (*p_mkv).b_writing_frame = 0 as libc::c_int as libc::c_char;
    if mk_set_frame_flags(
        (*p_mkv).w,
        i_stamp,
        (*p_picture).b_keyframe,
        ((*p_picture).i_type == 0x5 as libc::c_int) as libc::c_int,
    ) < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    i_size
}
pub unsafe extern "C" fn close_file(
    mut handle: hnd_t,
    mut largest_pts: int64_t,
    mut second_largest_pts: int64_t,
) -> libc::c_int {
    let mut p_mkv: *mut mkv_hnd_t = handle as *mut mkv_hnd_t;
    let mut ret: libc::c_int = 0;
    let mut i_last_delta: int64_t = 0;
    i_last_delta = if (*p_mkv).i_timebase_den != 0 {
        (((largest_pts - second_largest_pts) * (*p_mkv).i_timebase_num as int64_t
            / (*p_mkv).i_timebase_den as int64_t) as libc::c_double
            + 0.5f64) as int64_t
    } else {
        0 as libc::c_int as int64_t
    };
    ret = mk_close((*p_mkv).w, i_last_delta);
    free(p_mkv as *mut libc::c_void);
    ret
}
