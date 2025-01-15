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
    fn x264_cli_log(
        name: *const libc::c_char,
        i_level: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    fn x264_cli_csp_is_invalid(csp: libc::c_int) -> libc::c_int;
    fn x264_cli_csp_depth_factor(csp: libc::c_int) -> libc::c_int;
    fn x264_cli_get_csp(csp: libc::c_int) -> *const x264_cli_csp_t;
    fn x264_split_options(
        opt_str: *const libc::c_char,
        options: *const *const libc::c_char,
    ) -> *mut *mut libc::c_char;
    fn x264_get_option(
        name: *const libc::c_char,
        split_options: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn x264_otoi(str: *const libc::c_char, def: libc::c_int) -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
pub unsafe extern "C" fn help(mut longhelp: libc::c_int) {
    printf(b"      crop:left,top,right,bottom\n\0" as *const u8 as *const libc::c_char);
    if longhelp == 0 {
        return;
    }
    printf(
        b"            removes pixels from the edges of the frame\n\0" as *const u8
            as *const libc::c_char,
    );
}
pub unsafe extern "C" fn handle_opts(
    mut h: *mut crop_hnd_t,
    mut info: *mut video_info_t,
    mut opts: *mut *mut libc::c_char,
    mut optlist: *const *const libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        let mut opt: *mut libc::c_char = x264_get_option(*optlist.offset(i as isize), opts);
        if opt.is_null() {
            x264_cli_log(
                b"crop\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
                b"%s crop value not specified\n\0" as *const u8 as *const libc::c_char,
                *optlist.offset(i as isize),
            );
            return -(1 as libc::c_int);
        }
        (*h).dims[i as usize] = x264_otoi(opt, -(1 as libc::c_int));
        if (*h).dims[i as usize] < 0 as libc::c_int {
            x264_cli_log(
                b"crop\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
                b"%s crop value `%s' is less than 0\n\0" as *const u8 as *const libc::c_char,
                *optlist.offset(i as isize),
                opt,
            );
            return -(1 as libc::c_int);
        }
        let mut dim_mod: libc::c_int = if i & 1 as libc::c_int != 0 {
            (*(*h).csp).mod_height << (*info).interlaced
        } else {
            (*(*h).csp).mod_width
        };
        if (*h).dims[i as usize] % dim_mod != 0 {
            x264_cli_log(
                b"crop\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
                b"%s crop value `%s' is not a multiple of %d\n\0" as *const u8
                    as *const libc::c_char,
                *optlist.offset(i as isize),
                opt,
                dim_mod,
            );
            return -(1 as libc::c_int);
        }
        i += 1;
        i;
    }
    0 as libc::c_int
}
pub unsafe extern "C" fn init(
    mut handle: *mut hnd_t,
    mut filter_v: *mut cli_vid_filter_t,
    mut info: *mut video_info_t,
    mut param: *mut x264_param_t,
    mut opt_string: *mut libc::c_char,
) -> libc::c_int {
    if x264_cli_csp_is_invalid((*info).csp) != 0 {
        x264_cli_log(
            b"crop\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            b"invalid csp %d\n\0" as *const u8 as *const libc::c_char,
            (*info).csp,
        );
        return -(1 as libc::c_int);
    }
    let mut h: *mut crop_hnd_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<crop_hnd_t>() as libc::c_ulong,
    ) as *mut crop_hnd_t;
    if h.is_null() {
        return -(1 as libc::c_int);
    }
    (*h).csp = x264_cli_get_csp((*info).csp);
    static mut optlist: [*const libc::c_char; 5] = [
        b"left\0" as *const u8 as *const libc::c_char,
        b"top\0" as *const u8 as *const libc::c_char,
        b"right\0" as *const u8 as *const libc::c_char,
        b"bottom\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut opts: *mut *mut libc::c_char = x264_split_options(opt_string, optlist.as_ptr());
    if opts.is_null() {
        return -(1 as libc::c_int);
    }
    let mut err: libc::c_int = handle_opts(h, info, opts, optlist.as_ptr());
    free(opts as *mut libc::c_void);
    if err != 0 {
        return -(1 as libc::c_int);
    }
    (*h).dims[2 as libc::c_int as usize] =
        (*info).width - (*h).dims[0 as libc::c_int as usize] - (*h).dims[2 as libc::c_int as usize];
    (*h).dims[3 as libc::c_int as usize] = (*info).height
        - (*h).dims[1 as libc::c_int as usize]
        - (*h).dims[3 as libc::c_int as usize];
    if (*h).dims[2 as libc::c_int as usize] <= 0 as libc::c_int
        || (*h).dims[3 as libc::c_int as usize] <= 0 as libc::c_int
    {
        x264_cli_log(
            b"crop\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            b"invalid output resolution %dx%d\n\0" as *const u8 as *const libc::c_char,
            (*h).dims[2 as libc::c_int as usize],
            (*h).dims[3 as libc::c_int as usize],
        );
        return -(1 as libc::c_int);
    }
    if (*info).width != (*h).dims[2 as libc::c_int as usize]
        || (*info).height != (*h).dims[3 as libc::c_int as usize]
    {
        x264_cli_log(
            b"crop\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int,
            b"cropping to %dx%d\n\0" as *const u8 as *const libc::c_char,
            (*h).dims[2 as libc::c_int as usize],
            (*h).dims[3 as libc::c_int as usize],
        );
    } else {
        free(h as *mut libc::c_void);
        return 0 as libc::c_int;
    }
    (*info).width = (*h).dims[2 as libc::c_int as usize];
    (*info).height = (*h).dims[3 as libc::c_int as usize];
    (*h).prev_filter = *filter_v;
    (*h).prev_hnd = *handle;
    *handle = h as hnd_t;
    *filter_v = crop_filter;
    0 as libc::c_int
}
pub unsafe extern "C" fn get_frame(
    mut handle: hnd_t,
    mut output: *mut cli_pic_t,
    mut frame: libc::c_int,
) -> libc::c_int {
    let mut h: *mut crop_hnd_t = handle as *mut crop_hnd_t;
    if ((*h).prev_filter.get_frame).expect("non-null function pointer")(
        (*h).prev_hnd,
        output,
        frame,
    ) != 0
    {
        return -(1 as libc::c_int);
    }
    (*output).img.width = (*h).dims[2 as libc::c_int as usize];
    (*output).img.height = (*h).dims[3 as libc::c_int as usize];
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*output).img.planes {
        let mut offset: intptr_t = (((*output).img.stride[i as usize]
            * (*h).dims[1 as libc::c_int as usize])
            as libc::c_float
            * (*(*h).csp).height[i as usize]) as intptr_t;
        offset = (offset as libc::c_float
            + (*h).dims[0 as libc::c_int as usize] as libc::c_float
                * (*(*h).csp).width[i as usize]
                * x264_cli_csp_depth_factor((*output).img.csp) as libc::c_float)
            as intptr_t;
        (*output).img.plane[i as usize] = ((*output).img.plane[i as usize]).offset(offset as isize);
        i += 1;
        i;
    }
    0 as libc::c_int
}
pub unsafe extern "C" fn release_frame(
    mut handle: hnd_t,
    mut pic: *mut cli_pic_t,
    mut frame: libc::c_int,
) -> libc::c_int {
    let mut h: *mut crop_hnd_t = handle as *mut crop_hnd_t;
    ((*h).prev_filter.release_frame).expect("non-null function pointer")((*h).prev_hnd, pic, frame)
}
pub unsafe extern "C" fn free_filter(mut handle: hnd_t) {
    let mut h: *mut crop_hnd_t = handle as *mut crop_hnd_t;
    ((*h).prev_filter.free).expect("non-null function pointer")((*h).prev_hnd);
    free(h as *mut libc::c_void);
}
