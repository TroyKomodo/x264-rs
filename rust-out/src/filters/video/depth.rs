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
    fn free(_: *mut libc::c_void);
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn x264_malloc(_: int64_t) -> *mut libc::c_void;
    fn x264_free(_: *mut libc::c_void);
    fn x264_cli_log(
        name: *const libc::c_char,
        i_level: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    fn x264_cli_csp_depth_factor(csp: libc::c_int) -> libc::c_int;
    fn x264_cli_pic_alloc(
        pic: *mut cli_pic_t,
        csp: libc::c_int,
        width: libc::c_int,
        height: libc::c_int,
    ) -> libc::c_int;
    fn x264_cli_pic_clean(pic: *mut cli_pic_t);
    fn x264_split_options(
        opt_str: *const libc::c_char,
        options: *const *const libc::c_char,
    ) -> *mut *mut libc::c_char;
    fn x264_get_option(
        name: *const libc::c_char,
        split_options: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn x264_otoi(str: *const libc::c_char, def: libc::c_int) -> libc::c_int;
}
#[inline(always)]
unsafe extern "C" fn x264_clip3(
    mut v: libc::c_int,
    mut i_min: libc::c_int,
    mut i_max: libc::c_int,
) -> libc::c_int {
    if v < i_min {
        i_min
    } else if v > i_max {
        i_max
    } else {
        v
    }
}
unsafe extern "C" fn depth_filter_csp_is_supported(mut csp: libc::c_int) -> libc::c_int {
    let mut csp_mask: libc::c_int = csp & 0xff as libc::c_int;
    (csp_mask == 0x1 as libc::c_int
        || csp_mask == 0x2 as libc::c_int
        || csp_mask == 0x6 as libc::c_int
        || csp_mask == 0xc as libc::c_int
        || csp_mask == 0x3 as libc::c_int
        || csp_mask == 0x7 as libc::c_int
        || csp_mask == 0xd as libc::c_int
        || csp_mask == 0x4 as libc::c_int
        || csp_mask == 0x5 as libc::c_int
        || csp_mask == 0x8 as libc::c_int
        || csp_mask == 0xe as libc::c_int
        || csp_mask == 0x10 as libc::c_int
        || csp_mask == 0xf as libc::c_int) as libc::c_int
}
unsafe extern "C" fn csp_num_interleaved(
    mut csp: libc::c_int,
    mut plane: libc::c_int,
) -> libc::c_int {
    let mut csp_mask: libc::c_int = csp & 0xff as libc::c_int;
    if (csp_mask == 0x4 as libc::c_int
        || csp_mask == 0x5 as libc::c_int
        || csp_mask == 0x8 as libc::c_int)
        && plane == 1 as libc::c_int
    {
        2 as libc::c_int
    } else if csp_mask == 0xe as libc::c_int || csp_mask == 0x10 as libc::c_int {
        3 as libc::c_int
    } else if csp_mask == 0xf as libc::c_int {
        4 as libc::c_int
    } else {
        1 as libc::c_int
    }
}
unsafe extern "C" fn dither_plane_1(
    mut dst: *mut pixel,
    mut dst_stride: libc::c_int,
    mut src: *mut uint16_t,
    mut src_stride: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut errors: *mut int16_t,
) {
    let lshift: libc::c_int = 16 as libc::c_int - 8 as libc::c_int;
    let rshift: libc::c_int = 16 as libc::c_int - 8 as libc::c_int + 2 as libc::c_int;
    let half: libc::c_int =
        (1 as libc::c_int) << (16 as libc::c_int - 8 as libc::c_int + 1 as libc::c_int);
    let pixel_max: libc::c_int = ((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int;
    memset(
        errors as *mut libc::c_void,
        0 as libc::c_int,
        ((width + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong),
    );
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < height {
        let mut err: libc::c_int = 0 as libc::c_int;
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < width {
            err = err * 2 as libc::c_int
                + *errors.offset(x as isize) as libc::c_int
                + *errors.offset((x + 1 as libc::c_int) as isize) as libc::c_int;
            *dst.offset((x * 1 as libc::c_int) as isize) = x264_clip3(
                (((*src.offset((x * 1 as libc::c_int) as isize) as libc::c_int)
                    << 2 as libc::c_int)
                    + err
                    + half)
                    >> rshift,
                0 as libc::c_int,
                pixel_max,
            ) as pixel;
            err = *src.offset((x * 1 as libc::c_int) as isize) as libc::c_int
                - ((*dst.offset((x * 1 as libc::c_int) as isize) as libc::c_int) << lshift);
            *errors.offset(x as isize) = err as int16_t;
            x += 1;
            x;
        }
        y += 1;
        y;
        src = src.offset(src_stride as isize);
        dst = dst.offset(dst_stride as isize);
    }
}
unsafe extern "C" fn dither_plane_2(
    mut dst: *mut pixel,
    mut dst_stride: libc::c_int,
    mut src: *mut uint16_t,
    mut src_stride: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut errors: *mut int16_t,
) {
    let lshift: libc::c_int = 16 as libc::c_int - 8 as libc::c_int;
    let rshift: libc::c_int = 16 as libc::c_int - 8 as libc::c_int + 2 as libc::c_int;
    let half: libc::c_int =
        (1 as libc::c_int) << (16 as libc::c_int - 8 as libc::c_int + 1 as libc::c_int);
    let pixel_max: libc::c_int = ((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int;
    memset(
        errors as *mut libc::c_void,
        0 as libc::c_int,
        ((width + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong),
    );
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < height {
        let mut err: libc::c_int = 0 as libc::c_int;
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < width {
            err = err * 2 as libc::c_int
                + *errors.offset(x as isize) as libc::c_int
                + *errors.offset((x + 1 as libc::c_int) as isize) as libc::c_int;
            *dst.offset((x * 2 as libc::c_int) as isize) = x264_clip3(
                (((*src.offset((x * 2 as libc::c_int) as isize) as libc::c_int)
                    << 2 as libc::c_int)
                    + err
                    + half)
                    >> rshift,
                0 as libc::c_int,
                pixel_max,
            ) as pixel;
            err = *src.offset((x * 2 as libc::c_int) as isize) as libc::c_int
                - ((*dst.offset((x * 2 as libc::c_int) as isize) as libc::c_int) << lshift);
            *errors.offset(x as isize) = err as int16_t;
            x += 1;
            x;
        }
        y += 1;
        y;
        src = src.offset(src_stride as isize);
        dst = dst.offset(dst_stride as isize);
    }
}
unsafe extern "C" fn dither_plane_3(
    mut dst: *mut pixel,
    mut dst_stride: libc::c_int,
    mut src: *mut uint16_t,
    mut src_stride: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut errors: *mut int16_t,
) {
    let lshift: libc::c_int = 16 as libc::c_int - 8 as libc::c_int;
    let rshift: libc::c_int = 16 as libc::c_int - 8 as libc::c_int + 2 as libc::c_int;
    let half: libc::c_int =
        (1 as libc::c_int) << (16 as libc::c_int - 8 as libc::c_int + 1 as libc::c_int);
    let pixel_max: libc::c_int = ((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int;
    memset(
        errors as *mut libc::c_void,
        0 as libc::c_int,
        ((width + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong),
    );
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < height {
        let mut err: libc::c_int = 0 as libc::c_int;
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < width {
            err = err * 2 as libc::c_int
                + *errors.offset(x as isize) as libc::c_int
                + *errors.offset((x + 1 as libc::c_int) as isize) as libc::c_int;
            *dst.offset((x * 3 as libc::c_int) as isize) = x264_clip3(
                (((*src.offset((x * 3 as libc::c_int) as isize) as libc::c_int)
                    << 2 as libc::c_int)
                    + err
                    + half)
                    >> rshift,
                0 as libc::c_int,
                pixel_max,
            ) as pixel;
            err = *src.offset((x * 3 as libc::c_int) as isize) as libc::c_int
                - ((*dst.offset((x * 3 as libc::c_int) as isize) as libc::c_int) << lshift);
            *errors.offset(x as isize) = err as int16_t;
            x += 1;
            x;
        }
        y += 1;
        y;
        src = src.offset(src_stride as isize);
        dst = dst.offset(dst_stride as isize);
    }
}
unsafe extern "C" fn dither_plane_4(
    mut dst: *mut pixel,
    mut dst_stride: libc::c_int,
    mut src: *mut uint16_t,
    mut src_stride: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut errors: *mut int16_t,
) {
    let lshift: libc::c_int = 16 as libc::c_int - 8 as libc::c_int;
    let rshift: libc::c_int = 16 as libc::c_int - 8 as libc::c_int + 2 as libc::c_int;
    let half: libc::c_int =
        (1 as libc::c_int) << (16 as libc::c_int - 8 as libc::c_int + 1 as libc::c_int);
    let pixel_max: libc::c_int = ((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int;
    memset(
        errors as *mut libc::c_void,
        0 as libc::c_int,
        ((width + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong),
    );
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < height {
        let mut err: libc::c_int = 0 as libc::c_int;
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < width {
            err = err * 2 as libc::c_int
                + *errors.offset(x as isize) as libc::c_int
                + *errors.offset((x + 1 as libc::c_int) as isize) as libc::c_int;
            *dst.offset((x * 4 as libc::c_int) as isize) = x264_clip3(
                (((*src.offset((x * 4 as libc::c_int) as isize) as libc::c_int)
                    << 2 as libc::c_int)
                    + err
                    + half)
                    >> rshift,
                0 as libc::c_int,
                pixel_max,
            ) as pixel;
            err = *src.offset((x * 4 as libc::c_int) as isize) as libc::c_int
                - ((*dst.offset((x * 4 as libc::c_int) as isize) as libc::c_int) << lshift);
            *errors.offset(x as isize) = err as int16_t;
            x += 1;
            x;
        }
        y += 1;
        y;
        src = src.offset(src_stride as isize);
        dst = dst.offset(dst_stride as isize);
    }
}
unsafe extern "C" fn dither_image(
    mut out: *mut cli_image_t,
    mut img: *mut cli_image_t,
    mut error_buf: *mut int16_t,
) {
    let mut csp_mask: libc::c_int = (*img).csp & 0xff as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*img).planes {
        let mut num_interleaved: libc::c_int = csp_num_interleaved((*img).csp, i);
        let mut height: libc::c_int = ((*x264_cli_csps.as_ptr().offset(csp_mask as isize)).height
            [i as usize]
            * (*img).height as libc::c_float) as libc::c_int;
        let mut width: libc::c_int = ((*x264_cli_csps.as_ptr().offset(csp_mask as isize)).width
            [i as usize]
            * (*img).width as libc::c_float
            / num_interleaved as libc::c_float) as libc::c_int;
        if num_interleaved == 4 as libc::c_int {
            dither_plane_4(
                ((*out).plane[i as usize] as *mut pixel).offset(0 as libc::c_int as isize),
                (*out).stride[i as usize]
                    / ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int,
                ((*img).plane[i as usize] as *mut uint16_t).offset(0 as libc::c_int as isize),
                (*img).stride[i as usize] / 2 as libc::c_int,
                width,
                height,
                error_buf,
            );
            dither_plane_4(
                ((*out).plane[i as usize] as *mut pixel).offset(1 as libc::c_int as isize),
                (*out).stride[i as usize]
                    / ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int,
                ((*img).plane[i as usize] as *mut uint16_t).offset(1 as libc::c_int as isize),
                (*img).stride[i as usize] / 2 as libc::c_int,
                width,
                height,
                error_buf,
            );
            dither_plane_4(
                ((*out).plane[i as usize] as *mut pixel).offset(2 as libc::c_int as isize),
                (*out).stride[i as usize]
                    / ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int,
                ((*img).plane[i as usize] as *mut uint16_t).offset(2 as libc::c_int as isize),
                (*img).stride[i as usize] / 2 as libc::c_int,
                width,
                height,
                error_buf,
            );
            dither_plane_4(
                ((*out).plane[i as usize] as *mut pixel).offset(3 as libc::c_int as isize),
                (*out).stride[i as usize]
                    / ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int,
                ((*img).plane[i as usize] as *mut uint16_t).offset(3 as libc::c_int as isize),
                (*img).stride[i as usize] / 2 as libc::c_int,
                width,
                height,
                error_buf,
            );
        } else if num_interleaved == 3 as libc::c_int {
            dither_plane_3(
                ((*out).plane[i as usize] as *mut pixel).offset(0 as libc::c_int as isize),
                (*out).stride[i as usize]
                    / ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int,
                ((*img).plane[i as usize] as *mut uint16_t).offset(0 as libc::c_int as isize),
                (*img).stride[i as usize] / 2 as libc::c_int,
                width,
                height,
                error_buf,
            );
            dither_plane_3(
                ((*out).plane[i as usize] as *mut pixel).offset(1 as libc::c_int as isize),
                (*out).stride[i as usize]
                    / ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int,
                ((*img).plane[i as usize] as *mut uint16_t).offset(1 as libc::c_int as isize),
                (*img).stride[i as usize] / 2 as libc::c_int,
                width,
                height,
                error_buf,
            );
            dither_plane_3(
                ((*out).plane[i as usize] as *mut pixel).offset(2 as libc::c_int as isize),
                (*out).stride[i as usize]
                    / ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int,
                ((*img).plane[i as usize] as *mut uint16_t).offset(2 as libc::c_int as isize),
                (*img).stride[i as usize] / 2 as libc::c_int,
                width,
                height,
                error_buf,
            );
        } else if num_interleaved == 2 as libc::c_int {
            dither_plane_2(
                ((*out).plane[i as usize] as *mut pixel).offset(0 as libc::c_int as isize),
                (*out).stride[i as usize]
                    / ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int,
                ((*img).plane[i as usize] as *mut uint16_t).offset(0 as libc::c_int as isize),
                (*img).stride[i as usize] / 2 as libc::c_int,
                width,
                height,
                error_buf,
            );
            dither_plane_2(
                ((*out).plane[i as usize] as *mut pixel).offset(1 as libc::c_int as isize),
                (*out).stride[i as usize]
                    / ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int,
                ((*img).plane[i as usize] as *mut uint16_t).offset(1 as libc::c_int as isize),
                (*img).stride[i as usize] / 2 as libc::c_int,
                width,
                height,
                error_buf,
            );
        } else {
            dither_plane_1(
                ((*out).plane[i as usize] as *mut pixel).offset(0 as libc::c_int as isize),
                (*out).stride[i as usize]
                    / ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int,
                ((*img).plane[i as usize] as *mut uint16_t).offset(0 as libc::c_int as isize),
                (*img).stride[i as usize] / 2 as libc::c_int,
                width,
                height,
                error_buf,
            );
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn scale_image(mut output: *mut cli_image_t, mut img: *mut cli_image_t) {
    let mut csp_mask: libc::c_int = (*img).csp & 0xff as libc::c_int;
    let shift: libc::c_int = 8 as libc::c_int - 8 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*img).planes {
        let mut src: *mut uint8_t = (*img).plane[i as usize];
        let mut dst: *mut uint16_t = (*output).plane[i as usize] as *mut uint16_t;
        let mut height: libc::c_int = ((*x264_cli_csps.as_ptr().offset(csp_mask as isize)).height
            [i as usize]
            * (*img).height as libc::c_float) as libc::c_int;
        let mut width: libc::c_int = ((*x264_cli_csps.as_ptr().offset(csp_mask as isize)).width
            [i as usize]
            * (*img).width as libc::c_float) as libc::c_int;
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < height {
            let mut k: libc::c_int = 0 as libc::c_int;
            while k < width {
                *dst.offset(k as isize) =
                    ((*src.offset(k as isize) as libc::c_int) << shift) as uint16_t;
                k += 1;
                k;
            }
            src = src.offset((*img).stride[i as usize] as isize);
            dst = dst.offset(((*output).stride[i as usize] / 2 as libc::c_int) as isize);
            j += 1;
            j;
        }
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn get_frame(
    mut handle: hnd_t,
    mut output: *mut cli_pic_t,
    mut frame: libc::c_int,
) -> libc::c_int {
    let mut h: *mut depth_hnd_t = handle as *mut depth_hnd_t;
    if ((*h).prev_filter.get_frame).expect("non-null function pointer")(
        (*h).prev_hnd,
        output,
        frame,
    ) != 0
    {
        return -(1 as libc::c_int);
    }
    if (*h).bit_depth < 16 as libc::c_int && (*output).img.csp & 0x2000 as libc::c_int != 0 {
        dither_image(&mut (*h).buffer.img, &mut (*output).img, (*h).error_buf);
        (*output).img = (*h).buffer.img;
    } else if (*h).bit_depth > 8 as libc::c_int && (*output).img.csp & 0x2000 as libc::c_int == 0 {
        scale_image(&mut (*h).buffer.img, &mut (*output).img);
        (*output).img = (*h).buffer.img;
    }
    0 as libc::c_int
}
pub unsafe extern "C" fn release_frame(
    mut handle: hnd_t,
    mut pic: *mut cli_pic_t,
    mut frame: libc::c_int,
) -> libc::c_int {
    let mut h: *mut depth_hnd_t = handle as *mut depth_hnd_t;
    ((*h).prev_filter.release_frame).expect("non-null function pointer")((*h).prev_hnd, pic, frame)
}
pub unsafe extern "C" fn free_filter(mut handle: hnd_t) {
    let mut h: *mut depth_hnd_t = handle as *mut depth_hnd_t;
    ((*h).prev_filter.free).expect("non-null function pointer")((*h).prev_hnd);
    x264_cli_pic_clean(&mut (*h).buffer);
    x264_free(h as *mut libc::c_void);
}
pub unsafe extern "C" fn init(
    mut handle: *mut hnd_t,
    mut filter_v: *mut cli_vid_filter_t,
    mut info: *mut video_info_t,
    mut param: *mut x264_param_t,
    mut opt_string: *mut libc::c_char,
) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut change_fmt: libc::c_int = ((*info).csp ^ (*param).i_csp) & 0x2000 as libc::c_int;
    let mut csp: libc::c_int = !(!(*info).csp ^ change_fmt);
    let mut bit_depth: libc::c_int = 8 as libc::c_int * x264_cli_csp_depth_factor(csp);
    if !opt_string.is_null() {
        static mut optlist: [*const libc::c_char; 2] = [
            b"bit_depth\0" as *const u8 as *const libc::c_char,
            0 as *const libc::c_char,
        ];
        let mut opts: *mut *mut libc::c_char = x264_split_options(opt_string, optlist.as_ptr());
        if !opts.is_null() {
            let mut str_bit_depth: *mut libc::c_char =
                x264_get_option(b"bit_depth\0" as *const u8 as *const libc::c_char, opts);
            bit_depth = x264_otoi(str_bit_depth, -(1 as libc::c_int));
            ret = (bit_depth < 8 as libc::c_int || bit_depth > 16 as libc::c_int) as libc::c_int;
            csp = if bit_depth > 8 as libc::c_int {
                csp | 0x2000 as libc::c_int
            } else {
                csp & !(0x2000 as libc::c_int)
            };
            change_fmt = ((*info).csp ^ csp) & 0x2000 as libc::c_int;
            free(opts as *mut libc::c_void);
        } else {
            ret = 1 as libc::c_int;
        }
    }
    if bit_depth != 8 as libc::c_int {
        x264_cli_log(
            b"depth_8\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            b"this filter supports only bit depth %d\n\0" as *const u8 as *const libc::c_char,
            8 as libc::c_int,
        );
        return -(1 as libc::c_int);
    }
    if ret != 0 {
        x264_cli_log(
            b"depth_8\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            b"unsupported bit depth conversion.\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if change_fmt != 0 || bit_depth != 8 as libc::c_int * x264_cli_csp_depth_factor(csp) {
        if depth_filter_csp_is_supported(csp) == 0 {
            x264_cli_log(
                b"depth_8\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
                b"unsupported colorspace.\n\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        let mut h: *mut depth_hnd_t = x264_malloc(
            (::core::mem::size_of::<depth_hnd_t>() as libc::c_ulong).wrapping_add(
                (((*info).width + 1 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong),
            ) as int64_t,
        ) as *mut depth_hnd_t;
        if h.is_null() {
            return -(1 as libc::c_int);
        }
        (*h).error_buf = h.offset(1 as libc::c_int as isize) as *mut int16_t;
        (*h).dst_csp = csp;
        (*h).bit_depth = bit_depth;
        (*h).prev_hnd = *handle;
        (*h).prev_filter = *filter_v;
        if x264_cli_pic_alloc(
            &mut (*h).buffer,
            (*h).dst_csp,
            (*info).width,
            (*info).height,
        ) != 0
        {
            x264_free(h as *mut libc::c_void);
            return -(1 as libc::c_int);
        }
        *handle = h as hnd_t;
        *filter_v = depth_8_filter;
        (*info).csp = (*h).dst_csp;
    }
    0 as libc::c_int
}
