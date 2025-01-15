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
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn strtok_r(
        __s: *mut libc::c_char,
        __delim: *const libc::c_char,
        __save_ptr: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn x264_reduce_fraction(n: *mut uint32_t, d: *mut uint32_t);
    fn x264_cli_log(
        name: *const libc::c_char,
        i_level: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    fn x264_otoi(str: *const libc::c_char, def: libc::c_int) -> libc::c_int;
    fn x264_init_vid_filter(
        name: *const libc::c_char,
        handle: *mut hnd_t,
        filter: *mut cli_vid_filter_t,
        info: *mut video_info_t,
        param: *mut x264_param_t,
        opt_string: *mut libc::c_char,
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
pub unsafe extern "C" fn help(mut longhelp: libc::c_int) {
    printf(b"      select_every:step,offset1[,...]\n\0" as *const u8 as *const libc::c_char);
    if longhelp == 0 {
        return;
    }
    printf(
        b"            apply a selection pattern to input frames\n            step: the number of frames in the pattern\n            offsets: the offset into the step to select a frame\n            see: http://avisynth.nl/index.php/Select#SelectEvery\n\0"
            as *const u8 as *const libc::c_char,
    );
}
pub unsafe extern "C" fn init(
    mut handle: *mut hnd_t,
    mut filter_v: *mut cli_vid_filter_t,
    mut info: *mut video_info_t,
    mut param: *mut x264_param_t,
    mut opt_string: *mut libc::c_char,
) -> libc::c_int {
    let mut h: *mut selvry_hnd_t =
        malloc(::core::mem::size_of::<selvry_hnd_t>() as libc::c_ulong) as *mut selvry_hnd_t;
    if h.is_null() {
        return -(1 as libc::c_int);
    }
    (*h).pattern_len = 0 as libc::c_int;
    (*h).step_size = 0 as libc::c_int;
    let mut offsets: [libc::c_int; 100] = [0; 100];
    let mut tok: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    let mut p: *mut libc::c_char = opt_string;
    let mut saveptr: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    loop {
        tok = strtok_r(p, b",\0" as *const u8 as *const libc::c_char, &mut saveptr);
        if tok.is_null() {
            break;
        }
        let mut val: libc::c_int = x264_otoi(tok, -(1 as libc::c_int));
        if !p.is_null() {
            if val <= 0 as libc::c_int {
                x264_cli_log(
                    b"select_every\0" as *const u8 as *const libc::c_char,
                    0 as libc::c_int,
                    b"invalid step `%s'\n\0" as *const u8 as *const libc::c_char,
                    tok,
                );
                return -(1 as libc::c_int);
            }
            (*h).step_size = val;
        } else {
            if val < 0 as libc::c_int || val >= (*h).step_size {
                x264_cli_log(
                    b"select_every\0" as *const u8 as *const libc::c_char,
                    0 as libc::c_int,
                    b"invalid offset `%s'\n\0" as *const u8 as *const libc::c_char,
                    tok,
                );
                return -(1 as libc::c_int);
            }
            if (*h).pattern_len >= 100 as libc::c_int {
                x264_cli_log(
                    b"select_every\0" as *const u8 as *const libc::c_char,
                    0 as libc::c_int,
                    b"max pattern size %d reached\n\0" as *const u8 as *const libc::c_char,
                    100 as libc::c_int,
                );
                return -(1 as libc::c_int);
            }
            let fresh0 = (*h).pattern_len;
            (*h).pattern_len += 1;
            offsets[fresh0 as usize] = val;
        }
        p = std::ptr::null_mut::<libc::c_char>();
    }
    if (*h).step_size == 0 {
        x264_cli_log(
            b"select_every\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            b"no step size provided\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (*h).pattern_len == 0 {
        x264_cli_log(
            b"select_every\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            b"no offsets supplied\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    (*h).pattern = malloc(
        ((*h).pattern_len as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    if ((*h).pattern).is_null() {
        return -(1 as libc::c_int);
    }
    memcpy(
        (*h).pattern as *mut libc::c_void,
        offsets.as_mut_ptr() as *const libc::c_void,
        ((*h).pattern_len as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    let mut max_rewind: intptr_t = 0 as libc::c_int as intptr_t;
    let mut min: libc::c_int = (*h).step_size;
    let mut i: libc::c_int = (*h).pattern_len - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        min = if min < offsets[i as usize] {
            min
        } else {
            offsets[i as usize]
        };
        if i != 0 {
            max_rewind = if max_rewind
                > (offsets[(i - 1 as libc::c_int) as usize] - min + 1 as libc::c_int) as intptr_t
            {
                max_rewind
            } else {
                (offsets[(i - 1 as libc::c_int) as usize] - min + 1 as libc::c_int) as intptr_t
            };
        }
        if max_rewind == (*h).step_size as intptr_t {
            break;
        }
        i -= 1;
        i;
    }
    let mut name: [libc::c_char; 20] = [0; 20];
    sprintf(
        name.as_mut_ptr(),
        b"cache_%d\0" as *const u8 as *const libc::c_char,
        (*param).i_bitdepth,
    );
    if x264_init_vid_filter(
        name.as_mut_ptr(),
        handle,
        filter_v,
        info,
        param,
        max_rewind as *mut libc::c_void as *mut libc::c_char,
    ) != 0
    {
        return -(1 as libc::c_int);
    }
    if (*h).step_size != (*h).pattern_len {
        (*info).num_frames = ((*info).num_frames as uint64_t * (*h).pattern_len as uint64_t
            / (*h).step_size as uint64_t) as libc::c_int;
        (*info).fps_den *= (*h).step_size as uint32_t;
        (*info).fps_num *= (*h).pattern_len as uint32_t;
        x264_reduce_fraction(&mut (*info).fps_num, &mut (*info).fps_den);
        if (*info).vfr != 0 {
            (*info).timebase_den *= (*h).pattern_len as uint32_t;
            (*info).timebase_num *= (*h).step_size as uint32_t;
            x264_reduce_fraction(&mut (*info).timebase_num, &mut (*info).timebase_den);
        }
    }
    (*h).pts = 0 as libc::c_int as int64_t;
    (*h).vfr = (*info).vfr;
    (*h).prev_filter = *filter_v;
    (*h).prev_hnd = *handle;
    *filter_v = select_every_filter;
    *handle = h as hnd_t;
    0 as libc::c_int
}
pub unsafe extern "C" fn get_frame(
    mut handle: hnd_t,
    mut output: *mut cli_pic_t,
    mut frame: libc::c_int,
) -> libc::c_int {
    let mut h: *mut selvry_hnd_t = handle as *mut selvry_hnd_t;
    let mut pat_frame: libc::c_int = *((*h).pattern).offset((frame % (*h).pattern_len) as isize)
        + frame / (*h).pattern_len * (*h).step_size;
    if ((*h).prev_filter.get_frame).expect("non-null function pointer")(
        (*h).prev_hnd,
        output,
        pat_frame,
    ) != 0
    {
        return -(1 as libc::c_int);
    }
    if (*h).vfr != 0 {
        (*output).pts = (*h).pts;
        (*h).pts += (*output).duration;
    }
    0 as libc::c_int
}
pub unsafe extern "C" fn release_frame(
    mut handle: hnd_t,
    mut pic: *mut cli_pic_t,
    mut frame: libc::c_int,
) -> libc::c_int {
    let mut h: *mut selvry_hnd_t = handle as *mut selvry_hnd_t;
    let mut pat_frame: libc::c_int = *((*h).pattern).offset((frame % (*h).pattern_len) as isize)
        + frame / (*h).pattern_len * (*h).step_size;
    ((*h).prev_filter.release_frame).expect("non-null function pointer")(
        (*h).prev_hnd,
        pic,
        pat_frame,
    )
}
pub unsafe extern "C" fn free_filter(mut handle: hnd_t) {
    let mut h: *mut selvry_hnd_t = handle as *mut selvry_hnd_t;
    ((*h).prev_filter.free).expect("non-null function pointer")((*h).prev_hnd);
    free((*h).pattern as *mut libc::c_void);
    free(h as *mut libc::c_void);
}
