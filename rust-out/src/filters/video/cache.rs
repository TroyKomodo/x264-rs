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
    fn x264_cli_log(
        name: *const libc::c_char,
        i_level: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    fn x264_cli_pic_alloc(
        pic: *mut cli_pic_t,
        csp: libc::c_int,
        width: libc::c_int,
        height: libc::c_int,
    ) -> libc::c_int;
    fn x264_cli_pic_clean(pic: *mut cli_pic_t);
    fn x264_cli_pic_copy(out: *mut cli_pic_t, in_0: *mut cli_pic_t) -> libc::c_int;
    fn x264_8_frame_push(list: *mut *mut x264_frame_t, frame: *mut x264_frame_t);
    fn x264_8_frame_shift(list: *mut *mut x264_frame_t) -> *mut x264_frame_t;
}
pub unsafe extern "C" fn init(
    mut handle: *mut hnd_t,
    mut filter_v: *mut cli_vid_filter_t,
    mut info: *mut video_info_t,
    mut param: *mut x264_param_t,
    mut opt_string: *mut libc::c_char,
) -> libc::c_int {
    let mut size: intptr_t = opt_string as intptr_t;
    if size <= 0 as libc::c_int as intptr_t {
        return 0 as libc::c_int;
    }
    let mut h: *mut cache_hnd_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<cache_hnd_t>() as libc::c_ulong,
    ) as *mut cache_hnd_t;
    if h.is_null() {
        return -(1 as libc::c_int);
    }
    (*h).max_size = size as libc::c_int;
    (*h).cache = malloc(
        (((*h).max_size + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut cli_pic_t>() as libc::c_ulong),
    ) as *mut *mut cli_pic_t;
    if ((*h).cache).is_null() {
        return -(1 as libc::c_int);
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*h).max_size {
        let fresh0 = &mut (*((*h).cache).offset(i as isize));
        *fresh0 = malloc(::core::mem::size_of::<cli_pic_t>() as libc::c_ulong) as *mut cli_pic_t;
        if (*((*h).cache).offset(i as isize)).is_null()
            || x264_cli_pic_alloc(
                *((*h).cache).offset(i as isize),
                (*info).csp,
                (*info).width,
                (*info).height,
            ) != 0
        {
            return -(1 as libc::c_int);
        }
        i += 1;
        i;
    }
    let fresh1 = &mut (*((*h).cache).offset((*h).max_size as isize));
    *fresh1 = std::ptr::null_mut::<cli_pic_t>();
    (*h).prev_filter = *filter_v;
    (*h).prev_hnd = *handle;
    *handle = h as hnd_t;
    *filter_v = cache_8_filter;
    0 as libc::c_int
}
unsafe extern "C" fn fill_cache(mut h: *mut cache_hnd_t, mut frame: libc::c_int) {
    let mut shift: libc::c_int = frame - ((*h).first_frame + (*h).cur_size - 1 as libc::c_int);
    if shift <= 0 as libc::c_int || (*h).eof != 0 {
        return;
    }
    let mut cur_frame: libc::c_int =
        if (*h).first_frame + (*h).cur_size > frame - (*h).max_size + 1 as libc::c_int {
            (*h).first_frame + (*h).cur_size
        } else {
            frame - (*h).max_size + 1 as libc::c_int
        };
    (*h).first_frame = if (*h).first_frame + shift < cur_frame {
        (*h).first_frame + shift
    } else {
        cur_frame
    };
    (*h).cur_size = if (*h).cur_size - shift > 0 as libc::c_int {
        (*h).cur_size - shift
    } else {
        0 as libc::c_int
    };
    while (*h).cur_size < (*h).max_size {
        let mut temp: cli_pic_t = cli_pic_t {
            img: cli_image_t {
                csp: 0,
                width: 0,
                height: 0,
                planes: 0,
                plane: [std::ptr::null_mut::<uint8_t>(); 4],
                stride: [0; 4],
            },
            pts: 0,
            duration: 0,
            opaque: std::ptr::null_mut::<libc::c_void>(),
        };
        let mut cache: *mut cli_pic_t = *((*h).cache).offset(0 as libc::c_int as isize);
        if ((*h).prev_filter.get_frame).expect("non-null function pointer")(
            (*h).prev_hnd,
            &mut temp,
            cur_frame,
        ) != 0
            || x264_cli_pic_copy(cache, &mut temp) != 0
            || ((*h).prev_filter.release_frame).expect("non-null function pointer")(
                (*h).prev_hnd,
                &mut temp,
                cur_frame,
            ) != 0
        {
            (*h).eof = cur_frame;
            return;
        }
        x264_8_frame_push(
            (*h).cache as *mut libc::c_void as *mut *mut x264_frame_t,
            x264_8_frame_shift((*h).cache as *mut libc::c_void as *mut *mut x264_frame_t),
        );
        cur_frame += 1;
        cur_frame;
        (*h).cur_size += 1;
        (*h).cur_size;
    }
}
pub unsafe extern "C" fn get_frame(
    mut handle: hnd_t,
    mut output: *mut cli_pic_t,
    mut frame: libc::c_int,
) -> libc::c_int {
    let mut h: *mut cache_hnd_t = handle as *mut cache_hnd_t;
    if frame < (*h).first_frame {
        x264_cli_log(
            b"cache_8\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            b"frame %d is before first cached frame %d \n\0" as *const u8 as *const libc::c_char,
            frame,
            (*h).first_frame,
        );
        return -(1 as libc::c_int);
    }
    fill_cache(h, frame);
    if frame > (*h).first_frame + (*h).cur_size - 1 as libc::c_int {
        return -(1 as libc::c_int);
    }
    let mut idx: libc::c_int = frame
        - (if (*h).eof != 0 {
            (*h).eof - (*h).max_size
        } else {
            (*h).first_frame
        });
    *output = **((*h).cache).offset(idx as isize);
    0 as libc::c_int
}
pub unsafe extern "C" fn release_frame(
    mut handle: hnd_t,
    mut pic: *mut cli_pic_t,
    mut frame: libc::c_int,
) -> libc::c_int {
    0 as libc::c_int
}
pub unsafe extern "C" fn free_filter(mut handle: hnd_t) {
    let mut h: *mut cache_hnd_t = handle as *mut cache_hnd_t;
    ((*h).prev_filter.free).expect("non-null function pointer")((*h).prev_hnd);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*h).max_size {
        x264_cli_pic_clean(*((*h).cache).offset(i as isize));
        free(*((*h).cache).offset(i as isize) as *mut libc::c_void);
        i += 1;
        i;
    }
    free((*h).cache as *mut libc::c_void);
    free(h as *mut libc::c_void);
}
