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
    fn x264_cli_pic_alloc(
        pic: *mut cli_pic_t,
        csp: libc::c_int,
        width: libc::c_int,
        height: libc::c_int,
    ) -> libc::c_int;
    fn x264_cli_pic_clean(pic: *mut cli_pic_t);
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn x264_cli_pic_copy(out: *mut cli_pic_t, in_0: *mut cli_pic_t) -> libc::c_int;
}
pub unsafe extern "C" fn init(
    mut handle: *mut hnd_t,
    mut filter_v: *mut cli_vid_filter_t,
    mut info: *mut video_info_t,
    mut param: *mut x264_param_t,
    mut opt_string: *mut libc::c_char,
) -> libc::c_int {
    if (*info).vfr == 0 {
        return 0 as libc::c_int;
    }
    let mut h: *mut fix_vfr_pts_hnd_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<fix_vfr_pts_hnd_t>() as libc::c_ulong,
    ) as *mut fix_vfr_pts_hnd_t;
    if h.is_null() {
        return -(1 as libc::c_int);
    }
    (*h).holder_frame = -(1 as libc::c_int);
    (*h).prev_hnd = *handle;
    (*h).prev_filter = *filter_v;
    *handle = h as hnd_t;
    *filter_v = fix_vfr_pts_filter;
    0 as libc::c_int
}
pub unsafe extern "C" fn get_frame(
    mut handle: hnd_t,
    mut output: *mut cli_pic_t,
    mut frame: libc::c_int,
) -> libc::c_int {
    let mut h: *mut fix_vfr_pts_hnd_t = handle as *mut fix_vfr_pts_hnd_t;
    if frame == (*h).holder_frame {
        if (*h).holder_ret != 0 {
            return (*h).holder_ret;
        }
    } else {
        if (*h).holder_frame > 0 as libc::c_int
            && (*h).holder_frame < frame
            && ((*h).prev_filter.release_frame).expect("non-null function pointer")(
                (*h).prev_hnd,
                &mut (*h).holder,
                (*h).holder_frame,
            ) != 0
        {
            return -(1 as libc::c_int);
        }
        (*h).holder_frame = -(1 as libc::c_int);
        if ((*h).prev_filter.get_frame).expect("non-null function pointer")(
            (*h).prev_hnd,
            &mut (*h).holder,
            frame,
        ) != 0
        {
            return -(1 as libc::c_int);
        }
    }
    if (*h).holder.duration == 0 {
        if (*h).buffer_allocated == 0 {
            if x264_cli_pic_alloc(
                &mut (*h).buffer,
                (*h).holder.img.csp,
                (*h).holder.img.width,
                (*h).holder.img.height,
            ) != 0
            {
                return -(1 as libc::c_int);
            }
            (*h).buffer_allocated = 1 as libc::c_int;
        }
        (*h).holder_frame = frame + 1 as libc::c_int;
        if x264_cli_pic_copy(&mut (*h).buffer, &mut (*h).holder) != 0
            || ((*h).prev_filter.release_frame).expect("non-null function pointer")(
                (*h).prev_hnd,
                &mut (*h).holder,
                frame,
            ) != 0
        {
            return -(1 as libc::c_int);
        }
        (*h).holder_ret = ((*h).prev_filter.get_frame).expect("non-null function pointer")(
            (*h).prev_hnd,
            &mut (*h).holder,
            (*h).holder_frame,
        );
        if (*h).holder_ret == 0 {
            (*h).last_duration = if (*h).holder.pts - (*h).buffer.pts > 1 as libc::c_int as int64_t
            {
                (*h).holder.pts - (*h).buffer.pts
            } else {
                1 as libc::c_int as int64_t
            };
        }
        (*h).buffer.duration = (*h).last_duration;
        *output = (*h).buffer;
    } else {
        *output = (*h).holder;
    }
    (*output).pts = (*h).pts;
    (*h).pts += (*output).duration;
    0 as libc::c_int
}
pub unsafe extern "C" fn release_frame(
    mut handle: hnd_t,
    mut pic: *mut cli_pic_t,
    mut frame: libc::c_int,
) -> libc::c_int {
    let mut h: *mut fix_vfr_pts_hnd_t = handle as *mut fix_vfr_pts_hnd_t;
    if frame == (*h).holder_frame - 1 as libc::c_int {
        return 0 as libc::c_int;
    }
    ((*h).prev_filter.release_frame).expect("non-null function pointer")((*h).prev_hnd, pic, frame)
}
pub unsafe extern "C" fn free_filter(mut handle: hnd_t) {
    let mut h: *mut fix_vfr_pts_hnd_t = handle as *mut fix_vfr_pts_hnd_t;
    ((*h).prev_filter.free).expect("non-null function pointer")((*h).prev_hnd);
    if (*h).buffer_allocated != 0 {
        x264_cli_pic_clean(&mut (*h).buffer);
    }
    free(h as *mut libc::c_void);
}
