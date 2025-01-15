use crate::types::*;
extern "C" {
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn x264_8_frame_expand_border_lowres(frame: *mut x264_frame_t);
}
#[inline(always)]
unsafe extern "C" fn x264_clip_pixel(mut x: libc::c_int) -> pixel {
    (if x & !(((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int) != 0 {
        (-x >> 31 as libc::c_int) & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
    } else {
        x
    }) as pixel
}
#[inline(always)]
unsafe extern "C" fn endian_fix16(mut x: uint16_t) -> uint16_t {
    (((x as libc::c_int) << 8 as libc::c_int) | (x as libc::c_int >> 8 as libc::c_int)) as uint16_t
}
#[inline]
unsafe extern "C" fn pixel_avg(
    mut dst: *mut pixel,
    mut i_dst_stride: intptr_t,
    mut src1: *mut pixel,
    mut i_src1_stride: intptr_t,
    mut src2: *mut pixel,
    mut i_src2_stride: intptr_t,
    mut i_width: libc::c_int,
    mut i_height: libc::c_int,
) {
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < i_height {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < i_width {
            *dst.offset(x as isize) = ((*src1.offset(x as isize) as libc::c_int
                + *src2.offset(x as isize) as libc::c_int
                + 1 as libc::c_int)
                >> 1 as libc::c_int) as pixel;
            x += 1;
            x;
        }
        dst = dst.offset(i_dst_stride as isize);
        src1 = src1.offset(i_src1_stride as isize);
        src2 = src2.offset(i_src2_stride as isize);
        y += 1;
        y;
    }
}
#[inline]
unsafe extern "C" fn pixel_avg_wxh(
    mut dst: *mut pixel,
    mut i_dst: intptr_t,
    mut src1: *mut pixel,
    mut i_src1: intptr_t,
    mut src2: *mut pixel,
    mut i_src2: intptr_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
) {
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < height {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < width {
            *dst.offset(x as isize) = ((*src1.offset(x as isize) as libc::c_int
                + *src2.offset(x as isize) as libc::c_int
                + 1 as libc::c_int)
                >> 1 as libc::c_int) as pixel;
            x += 1;
            x;
        }
        src1 = src1.offset(i_src1 as isize);
        src2 = src2.offset(i_src2 as isize);
        dst = dst.offset(i_dst as isize);
        y += 1;
        y;
    }
}
#[inline]
unsafe extern "C" fn pixel_avg_weight_wxh(
    mut dst: *mut pixel,
    mut i_dst: intptr_t,
    mut src1: *mut pixel,
    mut i_src1: intptr_t,
    mut src2: *mut pixel,
    mut i_src2: intptr_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut i_weight1: libc::c_int,
) {
    let mut i_weight2: libc::c_int = 64 as libc::c_int - i_weight1;
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < height {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < width {
            *dst.offset(x as isize) = x264_clip_pixel(
                (*src1.offset(x as isize) as libc::c_int * i_weight1
                    + *src2.offset(x as isize) as libc::c_int * i_weight2
                    + ((1 as libc::c_int) << 5 as libc::c_int))
                    >> 6 as libc::c_int,
            );
            x += 1;
            x;
        }
        y += 1;
        y;
        dst = dst.offset(i_dst as isize);
        src1 = src1.offset(i_src1 as isize);
        src2 = src2.offset(i_src2 as isize);
    }
}
unsafe extern "C" fn pixel_avg_16x16(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
    mut pix3: *mut pixel,
    mut i_stride_pix3: intptr_t,
    mut weight: libc::c_int,
) {
    if weight == 32 as libc::c_int {
        pixel_avg_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            16 as libc::c_int,
            16 as libc::c_int,
        );
    } else {
        pixel_avg_weight_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            16 as libc::c_int,
            16 as libc::c_int,
            weight,
        );
    };
}
unsafe extern "C" fn pixel_avg_16x8(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
    mut pix3: *mut pixel,
    mut i_stride_pix3: intptr_t,
    mut weight: libc::c_int,
) {
    if weight == 32 as libc::c_int {
        pixel_avg_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            16 as libc::c_int,
            8 as libc::c_int,
        );
    } else {
        pixel_avg_weight_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            16 as libc::c_int,
            8 as libc::c_int,
            weight,
        );
    };
}
unsafe extern "C" fn pixel_avg_8x16(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
    mut pix3: *mut pixel,
    mut i_stride_pix3: intptr_t,
    mut weight: libc::c_int,
) {
    if weight == 32 as libc::c_int {
        pixel_avg_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            8 as libc::c_int,
            16 as libc::c_int,
        );
    } else {
        pixel_avg_weight_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            8 as libc::c_int,
            16 as libc::c_int,
            weight,
        );
    };
}
unsafe extern "C" fn pixel_avg_8x8(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
    mut pix3: *mut pixel,
    mut i_stride_pix3: intptr_t,
    mut weight: libc::c_int,
) {
    if weight == 32 as libc::c_int {
        pixel_avg_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            8 as libc::c_int,
            8 as libc::c_int,
        );
    } else {
        pixel_avg_weight_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            8 as libc::c_int,
            8 as libc::c_int,
            weight,
        );
    };
}
unsafe extern "C" fn pixel_avg_8x4(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
    mut pix3: *mut pixel,
    mut i_stride_pix3: intptr_t,
    mut weight: libc::c_int,
) {
    if weight == 32 as libc::c_int {
        pixel_avg_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            8 as libc::c_int,
            4 as libc::c_int,
        );
    } else {
        pixel_avg_weight_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            8 as libc::c_int,
            4 as libc::c_int,
            weight,
        );
    };
}
unsafe extern "C" fn pixel_avg_4x16(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
    mut pix3: *mut pixel,
    mut i_stride_pix3: intptr_t,
    mut weight: libc::c_int,
) {
    if weight == 32 as libc::c_int {
        pixel_avg_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            4 as libc::c_int,
            16 as libc::c_int,
        );
    } else {
        pixel_avg_weight_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            4 as libc::c_int,
            16 as libc::c_int,
            weight,
        );
    };
}
unsafe extern "C" fn pixel_avg_4x8(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
    mut pix3: *mut pixel,
    mut i_stride_pix3: intptr_t,
    mut weight: libc::c_int,
) {
    if weight == 32 as libc::c_int {
        pixel_avg_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            4 as libc::c_int,
            8 as libc::c_int,
        );
    } else {
        pixel_avg_weight_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            4 as libc::c_int,
            8 as libc::c_int,
            weight,
        );
    };
}
unsafe extern "C" fn pixel_avg_4x4(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
    mut pix3: *mut pixel,
    mut i_stride_pix3: intptr_t,
    mut weight: libc::c_int,
) {
    if weight == 32 as libc::c_int {
        pixel_avg_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            4 as libc::c_int,
            4 as libc::c_int,
        );
    } else {
        pixel_avg_weight_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            4 as libc::c_int,
            4 as libc::c_int,
            weight,
        );
    };
}
unsafe extern "C" fn pixel_avg_4x2(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
    mut pix3: *mut pixel,
    mut i_stride_pix3: intptr_t,
    mut weight: libc::c_int,
) {
    if weight == 32 as libc::c_int {
        pixel_avg_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            4 as libc::c_int,
            2 as libc::c_int,
        );
    } else {
        pixel_avg_weight_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            4 as libc::c_int,
            2 as libc::c_int,
            weight,
        );
    };
}
unsafe extern "C" fn pixel_avg_2x8(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
    mut pix3: *mut pixel,
    mut i_stride_pix3: intptr_t,
    mut weight: libc::c_int,
) {
    if weight == 32 as libc::c_int {
        pixel_avg_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            2 as libc::c_int,
            8 as libc::c_int,
        );
    } else {
        pixel_avg_weight_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            2 as libc::c_int,
            8 as libc::c_int,
            weight,
        );
    };
}
unsafe extern "C" fn pixel_avg_2x4(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
    mut pix3: *mut pixel,
    mut i_stride_pix3: intptr_t,
    mut weight: libc::c_int,
) {
    if weight == 32 as libc::c_int {
        pixel_avg_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            2 as libc::c_int,
            4 as libc::c_int,
        );
    } else {
        pixel_avg_weight_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            2 as libc::c_int,
            4 as libc::c_int,
            weight,
        );
    };
}
unsafe extern "C" fn pixel_avg_2x2(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
    mut pix3: *mut pixel,
    mut i_stride_pix3: intptr_t,
    mut weight: libc::c_int,
) {
    if weight == 32 as libc::c_int {
        pixel_avg_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            2 as libc::c_int,
            2 as libc::c_int,
        );
    } else {
        pixel_avg_weight_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            2 as libc::c_int,
            2 as libc::c_int,
            weight,
        );
    };
}
unsafe extern "C" fn weight_cache(mut h: *mut x264_t, mut w: *mut x264_weight_t) {
    (*w).weightfn = (*h).mc.weight;
}
unsafe extern "C" fn mc_weight(
    mut dst: *mut pixel,
    mut i_dst_stride: intptr_t,
    mut src: *mut pixel,
    mut i_src_stride: intptr_t,
    mut weight: *const x264_weight_t,
    mut i_width: libc::c_int,
    mut i_height: libc::c_int,
) {
    let mut offset: libc::c_int =
        (*weight).i_offset * ((1 as libc::c_int) << (8 as libc::c_int - 8 as libc::c_int));
    let mut scale: libc::c_int = (*weight).i_scale;
    let mut denom: libc::c_int = (*weight).i_denom;
    if denom >= 1 as libc::c_int {
        let mut y: libc::c_int = 0 as libc::c_int;
        while y < i_height {
            let mut x: libc::c_int = 0 as libc::c_int;
            while x < i_width {
                *dst.offset(x as isize) = x264_clip_pixel(
                    ((*src.offset(x as isize) as libc::c_int * scale
                        + ((1 as libc::c_int) << (denom - 1 as libc::c_int)))
                        >> denom)
                        + offset,
                );
                x += 1;
                x;
            }
            y += 1;
            y;
            dst = dst.offset(i_dst_stride as isize);
            src = src.offset(i_src_stride as isize);
        }
    } else {
        let mut y_0: libc::c_int = 0 as libc::c_int;
        while y_0 < i_height {
            let mut x_0: libc::c_int = 0 as libc::c_int;
            while x_0 < i_width {
                *dst.offset(x_0 as isize) =
                    x264_clip_pixel(*src.offset(x_0 as isize) as libc::c_int * scale + offset);
                x_0 += 1;
                x_0;
            }
            y_0 += 1;
            y_0;
            dst = dst.offset(i_dst_stride as isize);
            src = src.offset(i_src_stride as isize);
        }
    };
}
pub unsafe extern "C" fn mc_weight_w20(
    mut dst: *mut pixel,
    mut i_dst_stride: intptr_t,
    mut src: *mut pixel,
    mut i_src_stride: intptr_t,
    mut weight: *const x264_weight_t,
    mut height: libc::c_int,
) {
    mc_weight(
        dst,
        i_dst_stride,
        src,
        i_src_stride,
        weight,
        20 as libc::c_int,
        height,
    );
}
pub unsafe extern "C" fn mc_weight_w16(
    mut dst: *mut pixel,
    mut i_dst_stride: intptr_t,
    mut src: *mut pixel,
    mut i_src_stride: intptr_t,
    mut weight: *const x264_weight_t,
    mut height: libc::c_int,
) {
    mc_weight(
        dst,
        i_dst_stride,
        src,
        i_src_stride,
        weight,
        16 as libc::c_int,
        height,
    );
}
pub unsafe extern "C" fn mc_weight_w12(
    mut dst: *mut pixel,
    mut i_dst_stride: intptr_t,
    mut src: *mut pixel,
    mut i_src_stride: intptr_t,
    mut weight: *const x264_weight_t,
    mut height: libc::c_int,
) {
    mc_weight(
        dst,
        i_dst_stride,
        src,
        i_src_stride,
        weight,
        12 as libc::c_int,
        height,
    );
}
pub unsafe extern "C" fn mc_weight_w8(
    mut dst: *mut pixel,
    mut i_dst_stride: intptr_t,
    mut src: *mut pixel,
    mut i_src_stride: intptr_t,
    mut weight: *const x264_weight_t,
    mut height: libc::c_int,
) {
    mc_weight(
        dst,
        i_dst_stride,
        src,
        i_src_stride,
        weight,
        8 as libc::c_int,
        height,
    );
}
pub unsafe extern "C" fn mc_weight_w4(
    mut dst: *mut pixel,
    mut i_dst_stride: intptr_t,
    mut src: *mut pixel,
    mut i_src_stride: intptr_t,
    mut weight: *const x264_weight_t,
    mut height: libc::c_int,
) {
    mc_weight(
        dst,
        i_dst_stride,
        src,
        i_src_stride,
        weight,
        4 as libc::c_int,
        height,
    );
}
pub unsafe extern "C" fn mc_weight_w2(
    mut dst: *mut pixel,
    mut i_dst_stride: intptr_t,
    mut src: *mut pixel,
    mut i_src_stride: intptr_t,
    mut weight: *const x264_weight_t,
    mut height: libc::c_int,
) {
    mc_weight(
        dst,
        i_dst_stride,
        src,
        i_src_stride,
        weight,
        2 as libc::c_int,
        height,
    );
}
unsafe extern "C" fn mc_copy(
    mut src: *mut pixel,
    mut i_src_stride: intptr_t,
    mut dst: *mut pixel,
    mut i_dst_stride: intptr_t,
    mut i_width: libc::c_int,
    mut i_height: libc::c_int,
) {
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < i_height {
        memcpy(
            dst as *mut libc::c_void,
            src as *const libc::c_void,
            (i_width * ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int)
                as libc::c_ulong,
        );
        src = src.offset(i_src_stride as isize);
        dst = dst.offset(i_dst_stride as isize);
        y += 1;
        y;
    }
}
unsafe extern "C" fn hpel_filter(
    mut dsth: *mut pixel,
    mut dstv: *mut pixel,
    mut dstc: *mut pixel,
    mut src: *mut pixel,
    mut stride: intptr_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut buf: *mut int16_t,
) {
    let pad: libc::c_int = if 8 as libc::c_int > 9 as libc::c_int {
        -(10 as libc::c_int) * (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
    } else {
        0 as libc::c_int
    };
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < height {
        let mut x: libc::c_int = -(2 as libc::c_int);
        while x < width + 3 as libc::c_int {
            let mut v: libc::c_int = *src
                .offset((x as intptr_t - 2 as libc::c_int as intptr_t * stride) as isize)
                as libc::c_int
                + *src.offset((x as intptr_t + 3 as libc::c_int as intptr_t * stride) as isize)
                    as libc::c_int
                - 5 as libc::c_int
                    * (*src.offset((x as intptr_t - stride) as isize) as libc::c_int
                        + *src.offset(
                            (x as intptr_t + 2 as libc::c_int as intptr_t * stride) as isize,
                        ) as libc::c_int)
                + 20 as libc::c_int
                    * (*src.offset(x as isize) as libc::c_int
                        + *src.offset((x as intptr_t + stride) as isize) as libc::c_int);
            *dstv.offset(x as isize) = x264_clip_pixel((v + 16 as libc::c_int) >> 5 as libc::c_int);
            *buf.offset((x + 2 as libc::c_int) as isize) = (v + pad) as int16_t;
            x += 1;
            x;
        }
        let mut x_0: libc::c_int = 0 as libc::c_int;
        while x_0 < width {
            *dstc.offset(x_0 as isize) = x264_clip_pixel(
                (*buf
                    .offset(2 as libc::c_int as isize)
                    .offset((x_0 - 2 as libc::c_int * 1 as libc::c_int) as isize)
                    as libc::c_int
                    + *buf
                        .offset(2 as libc::c_int as isize)
                        .offset((x_0 + 3 as libc::c_int * 1 as libc::c_int) as isize)
                        as libc::c_int
                    - 5 as libc::c_int
                        * (*buf
                            .offset(2 as libc::c_int as isize)
                            .offset((x_0 - 1 as libc::c_int) as isize)
                            as libc::c_int
                            + *buf
                                .offset(2 as libc::c_int as isize)
                                .offset((x_0 + 2 as libc::c_int * 1 as libc::c_int) as isize)
                                as libc::c_int)
                    + 20 as libc::c_int
                        * (*buf.offset(2 as libc::c_int as isize).offset(x_0 as isize)
                            as libc::c_int
                            + *buf
                                .offset(2 as libc::c_int as isize)
                                .offset((x_0 + 1 as libc::c_int) as isize)
                                as libc::c_int)
                    - 32 as libc::c_int * pad
                    + 512 as libc::c_int)
                    >> 10 as libc::c_int,
            );
            x_0 += 1;
            x_0;
        }
        let mut x_1: libc::c_int = 0 as libc::c_int;
        while x_1 < width {
            *dsth.offset(x_1 as isize) = x264_clip_pixel(
                (*src.offset((x_1 - 2 as libc::c_int * 1 as libc::c_int) as isize) as libc::c_int
                    + *src.offset((x_1 + 3 as libc::c_int * 1 as libc::c_int) as isize)
                        as libc::c_int
                    - 5 as libc::c_int
                        * (*src.offset((x_1 - 1 as libc::c_int) as isize) as libc::c_int
                            + *src.offset((x_1 + 2 as libc::c_int * 1 as libc::c_int) as isize)
                                as libc::c_int)
                    + 20 as libc::c_int
                        * (*src.offset(x_1 as isize) as libc::c_int
                            + *src.offset((x_1 + 1 as libc::c_int) as isize) as libc::c_int)
                    + 16 as libc::c_int)
                    >> 5 as libc::c_int,
            );
            x_1 += 1;
            x_1;
        }
        dsth = dsth.offset(stride as isize);
        dstv = dstv.offset(stride as isize);
        dstc = dstc.offset(stride as isize);
        src = src.offset(stride as isize);
        y += 1;
        y;
    }
}
unsafe extern "C" fn mc_luma(
    mut dst: *mut pixel,
    mut i_dst_stride: intptr_t,
    mut src: *mut *mut pixel,
    mut i_src_stride: intptr_t,
    mut mvx: libc::c_int,
    mut mvy: libc::c_int,
    mut i_width: libc::c_int,
    mut i_height: libc::c_int,
    mut weight: *const x264_weight_t,
) {
    let mut qpel_idx: libc::c_int =
        ((mvy & 3 as libc::c_int) << 2 as libc::c_int) + (mvx & 3 as libc::c_int);
    let mut offset: libc::c_int = ((mvy >> 2 as libc::c_int) as intptr_t * i_src_stride
        + (mvx >> 2 as libc::c_int) as intptr_t) as libc::c_int;
    let mut src1: *mut pixel = (*src.offset(x264_hpel_ref0[qpel_idx as usize] as isize))
        .offset(offset as isize)
        .offset(
            ((mvy & 3 as libc::c_int == 3 as libc::c_int) as libc::c_int as intptr_t * i_src_stride)
                as isize,
        );
    if qpel_idx & 5 as libc::c_int != 0 {
        let mut src2: *mut pixel = (*src.offset(x264_hpel_ref1[qpel_idx as usize] as isize))
            .offset(offset as isize)
            .offset((mvx & 3 as libc::c_int == 3 as libc::c_int) as libc::c_int as isize);
        pixel_avg(
            dst,
            i_dst_stride,
            src1,
            i_src_stride,
            src2,
            i_src_stride,
            i_width,
            i_height,
        );
        if !((*weight).weightfn).is_null() {
            mc_weight(
                dst,
                i_dst_stride,
                dst,
                i_dst_stride,
                weight,
                i_width,
                i_height,
            );
        }
    } else if !((*weight).weightfn).is_null() {
        mc_weight(
            dst,
            i_dst_stride,
            src1,
            i_src_stride,
            weight,
            i_width,
            i_height,
        );
    } else {
        mc_copy(src1, i_src_stride, dst, i_dst_stride, i_width, i_height);
    };
}
unsafe extern "C" fn get_ref(
    mut dst: *mut pixel,
    mut i_dst_stride: *mut intptr_t,
    mut src: *mut *mut pixel,
    mut i_src_stride: intptr_t,
    mut mvx: libc::c_int,
    mut mvy: libc::c_int,
    mut i_width: libc::c_int,
    mut i_height: libc::c_int,
    mut weight: *const x264_weight_t,
) -> *mut pixel {
    let mut qpel_idx: libc::c_int =
        ((mvy & 3 as libc::c_int) << 2 as libc::c_int) + (mvx & 3 as libc::c_int);
    let mut offset: libc::c_int = ((mvy >> 2 as libc::c_int) as intptr_t * i_src_stride
        + (mvx >> 2 as libc::c_int) as intptr_t) as libc::c_int;
    let mut src1: *mut pixel = (*src.offset(x264_hpel_ref0[qpel_idx as usize] as isize))
        .offset(offset as isize)
        .offset(
            ((mvy & 3 as libc::c_int == 3 as libc::c_int) as libc::c_int as intptr_t * i_src_stride)
                as isize,
        );
    if qpel_idx & 5 as libc::c_int != 0 {
        let mut src2: *mut pixel = (*src.offset(x264_hpel_ref1[qpel_idx as usize] as isize))
            .offset(offset as isize)
            .offset((mvx & 3 as libc::c_int == 3 as libc::c_int) as libc::c_int as isize);
        pixel_avg(
            dst,
            *i_dst_stride,
            src1,
            i_src_stride,
            src2,
            i_src_stride,
            i_width,
            i_height,
        );
        if !((*weight).weightfn).is_null() {
            mc_weight(
                dst,
                *i_dst_stride,
                dst,
                *i_dst_stride,
                weight,
                i_width,
                i_height,
            );
        }
        dst
    } else if !((*weight).weightfn).is_null() {
        mc_weight(
            dst,
            *i_dst_stride,
            src1,
            i_src_stride,
            weight,
            i_width,
            i_height,
        );
        return dst;
    } else {
        *i_dst_stride = i_src_stride;
        return src1;
    }
}
unsafe extern "C" fn mc_chroma(
    mut dstu: *mut pixel,
    mut dstv: *mut pixel,
    mut i_dst_stride: intptr_t,
    mut src: *mut pixel,
    mut i_src_stride: intptr_t,
    mut mvx: libc::c_int,
    mut mvy: libc::c_int,
    mut i_width: libc::c_int,
    mut i_height: libc::c_int,
) {
    let mut srcp: *mut pixel = std::ptr::null_mut::<pixel>();
    let mut d8x: libc::c_int = mvx & 0x7 as libc::c_int;
    let mut d8y: libc::c_int = mvy & 0x7 as libc::c_int;
    let mut cA: libc::c_int = (8 as libc::c_int - d8x) * (8 as libc::c_int - d8y);
    let mut cB: libc::c_int = d8x * (8 as libc::c_int - d8y);
    let mut cC: libc::c_int = (8 as libc::c_int - d8x) * d8y;
    let mut cD: libc::c_int = d8x * d8y;
    src = src.offset(
        ((mvy >> 3 as libc::c_int) as intptr_t * i_src_stride
            + ((mvx >> 3 as libc::c_int) * 2 as libc::c_int) as intptr_t) as isize,
    );
    srcp = &mut *src.offset(i_src_stride as isize) as *mut pixel;
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < i_height {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < i_width {
            *dstu.offset(x as isize) = ((cA
                * *src.offset((2 as libc::c_int * x) as isize) as libc::c_int
                + cB * *src.offset((2 as libc::c_int * x + 2 as libc::c_int) as isize)
                    as libc::c_int
                + cC * *srcp.offset((2 as libc::c_int * x) as isize) as libc::c_int
                + cD * *srcp.offset((2 as libc::c_int * x + 2 as libc::c_int) as isize)
                    as libc::c_int
                + 32 as libc::c_int)
                >> 6 as libc::c_int) as pixel;
            *dstv.offset(x as isize) = ((cA
                * *src.offset((2 as libc::c_int * x + 1 as libc::c_int) as isize) as libc::c_int
                + cB * *src.offset((2 as libc::c_int * x + 3 as libc::c_int) as isize)
                    as libc::c_int
                + cC * *srcp.offset((2 as libc::c_int * x + 1 as libc::c_int) as isize)
                    as libc::c_int
                + cD * *srcp.offset((2 as libc::c_int * x + 3 as libc::c_int) as isize)
                    as libc::c_int
                + 32 as libc::c_int)
                >> 6 as libc::c_int) as pixel;
            x += 1;
            x;
        }
        dstu = dstu.offset(i_dst_stride as isize);
        dstv = dstv.offset(i_dst_stride as isize);
        src = srcp;
        srcp = srcp.offset(i_src_stride as isize);
        y += 1;
        y;
    }
}
unsafe extern "C" fn mc_copy_w16(
    mut dst: *mut pixel,
    mut i_dst: intptr_t,
    mut src: *mut pixel,
    mut i_src: intptr_t,
    mut i_height: libc::c_int,
) {
    mc_copy(src, i_src, dst, i_dst, 16 as libc::c_int, i_height);
}
unsafe extern "C" fn mc_copy_w8(
    mut dst: *mut pixel,
    mut i_dst: intptr_t,
    mut src: *mut pixel,
    mut i_src: intptr_t,
    mut i_height: libc::c_int,
) {
    mc_copy(src, i_src, dst, i_dst, 8 as libc::c_int, i_height);
}
unsafe extern "C" fn mc_copy_w4(
    mut dst: *mut pixel,
    mut i_dst: intptr_t,
    mut src: *mut pixel,
    mut i_src: intptr_t,
    mut i_height: libc::c_int,
) {
    mc_copy(src, i_src, dst, i_dst, 4 as libc::c_int, i_height);
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_plane_copy_c(
    mut dst: *mut pixel,
    mut i_dst: intptr_t,
    mut src: *mut pixel,
    mut i_src: intptr_t,
    mut w: libc::c_int,
    mut h: libc::c_int,
) {
    loop {
        let fresh0 = h;
        h -= 1;
        if fresh0 == 0 {
            break;
        }
        memcpy(
            dst as *mut libc::c_void,
            src as *const libc::c_void,
            (w * ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int) as libc::c_ulong,
        );
        dst = dst.offset(i_dst as isize);
        src = src.offset(i_src as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_plane_copy_swap_c(
    mut dst: *mut pixel,
    mut i_dst: intptr_t,
    mut src: *mut pixel,
    mut i_src: intptr_t,
    mut w: libc::c_int,
    mut h: libc::c_int,
) {
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < h {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < 2 as libc::c_int * w {
            *dst.offset(x as isize) = *src.offset((x + 1 as libc::c_int) as isize);
            *dst.offset((x + 1 as libc::c_int) as isize) = *src.offset(x as isize);
            x += 2 as libc::c_int;
        }
        y += 1;
        y;
        dst = dst.offset(i_dst as isize);
        src = src.offset(i_src as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_plane_copy_interleave_c(
    mut dst: *mut pixel,
    mut i_dst: intptr_t,
    mut srcu: *mut pixel,
    mut i_srcu: intptr_t,
    mut srcv: *mut pixel,
    mut i_srcv: intptr_t,
    mut w: libc::c_int,
    mut h: libc::c_int,
) {
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < h {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < w {
            *dst.offset((2 as libc::c_int * x) as isize) = *srcu.offset(x as isize);
            *dst.offset((2 as libc::c_int * x + 1 as libc::c_int) as isize) =
                *srcv.offset(x as isize);
            x += 1;
            x;
        }
        y += 1;
        y;
        dst = dst.offset(i_dst as isize);
        srcu = srcu.offset(i_srcu as isize);
        srcv = srcv.offset(i_srcv as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_plane_copy_deinterleave_c(
    mut dsta: *mut pixel,
    mut i_dsta: intptr_t,
    mut dstb: *mut pixel,
    mut i_dstb: intptr_t,
    mut src: *mut pixel,
    mut i_src: intptr_t,
    mut w: libc::c_int,
    mut h: libc::c_int,
) {
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < h {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < w {
            *dsta.offset(x as isize) = *src.offset((2 as libc::c_int * x) as isize);
            *dstb.offset(x as isize) =
                *src.offset((2 as libc::c_int * x + 1 as libc::c_int) as isize);
            x += 1;
            x;
        }
        y += 1;
        y;
        dsta = dsta.offset(i_dsta as isize);
        dstb = dstb.offset(i_dstb as isize);
        src = src.offset(i_src as isize);
    }
}
unsafe extern "C" fn plane_copy_deinterleave_rgb_c(
    mut dsta: *mut pixel,
    mut i_dsta: intptr_t,
    mut dstb: *mut pixel,
    mut i_dstb: intptr_t,
    mut dstc: *mut pixel,
    mut i_dstc: intptr_t,
    mut src: *mut pixel,
    mut i_src: intptr_t,
    mut pw: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
) {
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < h {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < w {
            *dsta.offset(x as isize) = *src.offset((x * pw) as isize);
            *dstb.offset(x as isize) = *src.offset((x * pw + 1 as libc::c_int) as isize);
            *dstc.offset(x as isize) = *src.offset((x * pw + 2 as libc::c_int) as isize);
            x += 1;
            x;
        }
        y += 1;
        y;
        dsta = dsta.offset(i_dsta as isize);
        dstb = dstb.offset(i_dstb as isize);
        dstc = dstc.offset(i_dstc as isize);
        src = src.offset(i_src as isize);
    }
}
unsafe extern "C" fn plane_copy_deinterleave_v210_c(
    mut dsty: *mut pixel,
    mut i_dsty: intptr_t,
    mut dstc: *mut pixel,
    mut i_dstc: intptr_t,
    mut src: *mut uint32_t,
    mut i_src: intptr_t,
    mut w: libc::c_int,
    mut h: libc::c_int,
) {
    let mut l: libc::c_int = 0 as libc::c_int;
    while l < h {
        let mut dsty0: *mut pixel = dsty;
        let mut dstc0: *mut pixel = dstc;
        let mut src0: *mut uint32_t = src;
        let mut n: libc::c_int = 0 as libc::c_int;
        while n < w {
            let fresh1 = src0;
            src0 = src0.offset(1);
            let mut s: uint32_t = *fresh1;
            let fresh2 = dstc0;
            dstc0 = dstc0.offset(1);
            *fresh2 = (s & 0x3ff as libc::c_int as uint32_t) as pixel;
            let fresh3 = dsty0;
            dsty0 = dsty0.offset(1);
            *fresh3 = ((s >> 10 as libc::c_int) & 0x3ff as libc::c_int as uint32_t) as pixel;
            let fresh4 = dstc0;
            dstc0 = dstc0.offset(1);
            *fresh4 = ((s >> 20 as libc::c_int) & 0x3ff as libc::c_int as uint32_t) as pixel;
            let fresh5 = src0;
            src0 = src0.offset(1);
            s = *fresh5;
            let fresh6 = dsty0;
            dsty0 = dsty0.offset(1);
            *fresh6 = (s & 0x3ff as libc::c_int as uint32_t) as pixel;
            let fresh7 = dstc0;
            dstc0 = dstc0.offset(1);
            *fresh7 = ((s >> 10 as libc::c_int) & 0x3ff as libc::c_int as uint32_t) as pixel;
            let fresh8 = dsty0;
            dsty0 = dsty0.offset(1);
            *fresh8 = ((s >> 20 as libc::c_int) & 0x3ff as libc::c_int as uint32_t) as pixel;
            n += 3 as libc::c_int;
        }
        dsty = dsty.offset(i_dsty as isize);
        dstc = dstc.offset(i_dstc as isize);
        src = src.offset(i_src as isize);
        l += 1;
        l;
    }
}
unsafe extern "C" fn store_interleave_chroma(
    mut dst: *mut pixel,
    mut i_dst: intptr_t,
    mut srcu: *mut pixel,
    mut srcv: *mut pixel,
    mut height: libc::c_int,
) {
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < height {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < 8 as libc::c_int {
            *dst.offset((2 as libc::c_int * x) as isize) = *srcu.offset(x as isize);
            *dst.offset((2 as libc::c_int * x + 1 as libc::c_int) as isize) =
                *srcv.offset(x as isize);
            x += 1;
            x;
        }
        y += 1;
        y;
        dst = dst.offset(i_dst as isize);
        srcu = srcu.offset(32 as libc::c_int as isize);
        srcv = srcv.offset(32 as libc::c_int as isize);
    }
}
unsafe extern "C" fn load_deinterleave_chroma_fenc(
    mut dst: *mut pixel,
    mut src: *mut pixel,
    mut i_src: intptr_t,
    mut height: libc::c_int,
) {
    x264_8_plane_copy_deinterleave_c(
        dst,
        16 as libc::c_int as intptr_t,
        dst.offset((16 as libc::c_int / 2 as libc::c_int) as isize),
        16 as libc::c_int as intptr_t,
        src,
        i_src,
        8 as libc::c_int,
        height,
    );
}
unsafe extern "C" fn load_deinterleave_chroma_fdec(
    mut dst: *mut pixel,
    mut src: *mut pixel,
    mut i_src: intptr_t,
    mut height: libc::c_int,
) {
    x264_8_plane_copy_deinterleave_c(
        dst,
        32 as libc::c_int as intptr_t,
        dst.offset((32 as libc::c_int / 2 as libc::c_int) as isize),
        32 as libc::c_int as intptr_t,
        src,
        i_src,
        8 as libc::c_int,
        height,
    );
}
unsafe extern "C" fn prefetch_fenc_null(
    mut pix_y: *mut pixel,
    mut stride_y: intptr_t,
    mut pix_uv: *mut pixel,
    mut stride_uv: intptr_t,
    mut mb_x: libc::c_int,
) {
}
unsafe extern "C" fn prefetch_ref_null(
    mut pix: *mut pixel,
    mut stride: intptr_t,
    mut parity: libc::c_int,
) {
}
unsafe extern "C" fn memzero_aligned(mut dst: *mut libc::c_void, mut n: size_t) {
    memset(dst, 0 as libc::c_int, n);
}
unsafe extern "C" fn integral_init4h(
    mut sum: *mut uint16_t,
    mut pix: *mut pixel,
    mut stride: intptr_t,
) {
    let mut v: libc::c_int = *pix.offset(0 as libc::c_int as isize) as libc::c_int
        + *pix.offset(1 as libc::c_int as isize) as libc::c_int
        + *pix.offset(2 as libc::c_int as isize) as libc::c_int
        + *pix.offset(3 as libc::c_int as isize) as libc::c_int;
    let mut x: libc::c_int = 0 as libc::c_int;
    while (x as intptr_t) < stride - 4 as libc::c_int as intptr_t {
        *sum.offset(x as isize) =
            (v + *sum.offset((x as intptr_t - stride) as isize) as libc::c_int) as uint16_t;
        v += *pix.offset((x + 4 as libc::c_int) as isize) as libc::c_int
            - *pix.offset(x as isize) as libc::c_int;
        x += 1;
        x;
    }
}
unsafe extern "C" fn integral_init8h(
    mut sum: *mut uint16_t,
    mut pix: *mut pixel,
    mut stride: intptr_t,
) {
    let mut v: libc::c_int = *pix.offset(0 as libc::c_int as isize) as libc::c_int
        + *pix.offset(1 as libc::c_int as isize) as libc::c_int
        + *pix.offset(2 as libc::c_int as isize) as libc::c_int
        + *pix.offset(3 as libc::c_int as isize) as libc::c_int
        + *pix.offset(4 as libc::c_int as isize) as libc::c_int
        + *pix.offset(5 as libc::c_int as isize) as libc::c_int
        + *pix.offset(6 as libc::c_int as isize) as libc::c_int
        + *pix.offset(7 as libc::c_int as isize) as libc::c_int;
    let mut x: libc::c_int = 0 as libc::c_int;
    while (x as intptr_t) < stride - 8 as libc::c_int as intptr_t {
        *sum.offset(x as isize) =
            (v + *sum.offset((x as intptr_t - stride) as isize) as libc::c_int) as uint16_t;
        v += *pix.offset((x + 8 as libc::c_int) as isize) as libc::c_int
            - *pix.offset(x as isize) as libc::c_int;
        x += 1;
        x;
    }
}
unsafe extern "C" fn integral_init4v(
    mut sum8: *mut uint16_t,
    mut sum4: *mut uint16_t,
    mut stride: intptr_t,
) {
    let mut x: libc::c_int = 0 as libc::c_int;
    while (x as intptr_t) < stride - 8 as libc::c_int as intptr_t {
        *sum4.offset(x as isize) =
            (*sum8.offset((x as intptr_t + 4 as libc::c_int as intptr_t * stride) as isize)
                as libc::c_int
                - *sum8.offset(x as isize) as libc::c_int) as uint16_t;
        x += 1;
        x;
    }
    let mut x_0: libc::c_int = 0 as libc::c_int;
    while (x_0 as intptr_t) < stride - 8 as libc::c_int as intptr_t {
        *sum8.offset(x_0 as isize) = (*sum8
            .offset((x_0 as intptr_t + 8 as libc::c_int as intptr_t * stride) as isize)
            as libc::c_int
            + *sum8.offset(
                (x_0 as intptr_t
                    + 8 as libc::c_int as intptr_t * stride
                    + 4 as libc::c_int as intptr_t) as isize,
            ) as libc::c_int
            - *sum8.offset(x_0 as isize) as libc::c_int
            - *sum8.offset((x_0 + 4 as libc::c_int) as isize) as libc::c_int)
            as uint16_t;
        x_0 += 1;
        x_0;
    }
}
unsafe extern "C" fn integral_init8v(mut sum8: *mut uint16_t, mut stride: intptr_t) {
    let mut x: libc::c_int = 0 as libc::c_int;
    while (x as intptr_t) < stride - 8 as libc::c_int as intptr_t {
        *sum8.offset(x as isize) =
            (*sum8.offset((x as intptr_t + 8 as libc::c_int as intptr_t * stride) as isize)
                as libc::c_int
                - *sum8.offset(x as isize) as libc::c_int) as uint16_t;
        x += 1;
        x;
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_frame_init_lowres(
    mut h: *mut x264_t,
    mut frame: *mut x264_frame_t,
) {
    let mut src: *mut pixel = (*frame).plane[0 as libc::c_int as usize];
    let mut i_stride: libc::c_int = (*frame).i_stride[0 as libc::c_int as usize];
    let mut i_height: libc::c_int = (*frame).i_lines[0 as libc::c_int as usize];
    let mut i_width: libc::c_int = (*frame).i_width[0 as libc::c_int as usize];
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < i_height {
        *src.offset((i_width + y * i_stride) as isize) =
            *src.offset((i_width - 1 as libc::c_int + y * i_stride) as isize);
        y += 1;
        y;
    }
    memcpy(
        src.offset((i_stride * i_height) as isize) as *mut libc::c_void,
        src.offset((i_stride * (i_height - 1 as libc::c_int)) as isize) as *const libc::c_void,
        ((i_width + 1 as libc::c_int)
            * ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int)
            as libc::c_ulong,
    );
    ((*h).mc.frame_init_lowres_core).expect("non-null function pointer")(
        src,
        (*frame).lowres[0 as libc::c_int as usize],
        (*frame).lowres[1 as libc::c_int as usize],
        (*frame).lowres[2 as libc::c_int as usize],
        (*frame).lowres[3 as libc::c_int as usize],
        i_stride as intptr_t,
        (*frame).i_stride_lowres as intptr_t,
        (*frame).i_width_lowres,
        (*frame).i_lines_lowres,
    );
    x264_8_frame_expand_border_lowres(frame);
    memset(
        ((*frame).i_cost_est).as_mut_ptr() as *mut libc::c_void,
        -(1 as libc::c_int),
        ::core::mem::size_of::<[[libc::c_int; 18]; 18]>() as libc::c_ulong,
    );
    let mut y_0: libc::c_int = 0 as libc::c_int;
    while y_0 < (*h).param.i_bframe + 2 as libc::c_int {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < (*h).param.i_bframe + 2 as libc::c_int {
            *((*frame).i_row_satds[y_0 as usize][x as usize]).offset(0 as libc::c_int as isize) =
                -(1 as libc::c_int);
            x += 1;
            x;
        }
        y_0 += 1;
        y_0;
    }
    let mut y_1: libc::c_int = 0 as libc::c_int;
    while y_1 <= ((*h).param.i_bframe != 0) as libc::c_int {
        let mut x_0: libc::c_int = 0 as libc::c_int;
        while x_0 <= (*h).param.i_bframe {
            (*((*frame).lowres_mvs[y_1 as usize][x_0 as usize])
                .offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] =
                0x7fff as libc::c_int as int16_t;
            x_0 += 1;
            x_0;
        }
        y_1 += 1;
        y_1;
    }
}
unsafe extern "C" fn frame_init_lowres_core(
    mut src0: *mut pixel,
    mut dst0: *mut pixel,
    mut dsth: *mut pixel,
    mut dstv: *mut pixel,
    mut dstc: *mut pixel,
    mut src_stride: intptr_t,
    mut dst_stride: intptr_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
) {
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < height {
        let mut src1: *mut pixel = src0.offset(src_stride as isize);
        let mut src2: *mut pixel = src1.offset(src_stride as isize);
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < width {
            *dst0.offset(x as isize) = ((((*src0.offset((2 as libc::c_int * x) as isize)
                as libc::c_int
                + *src1.offset((2 as libc::c_int * x) as isize) as libc::c_int
                + 1 as libc::c_int)
                >> 1 as libc::c_int)
                + ((*src0.offset((2 as libc::c_int * x + 1 as libc::c_int) as isize)
                    as libc::c_int
                    + *src1.offset((2 as libc::c_int * x + 1 as libc::c_int) as isize)
                        as libc::c_int
                    + 1 as libc::c_int)
                    >> 1 as libc::c_int)
                + 1 as libc::c_int)
                >> 1 as libc::c_int) as pixel;
            *dsth.offset(x as isize) = ((((*src0
                .offset((2 as libc::c_int * x + 1 as libc::c_int) as isize)
                as libc::c_int
                + *src1.offset((2 as libc::c_int * x + 1 as libc::c_int) as isize)
                    as libc::c_int
                + 1 as libc::c_int)
                >> 1 as libc::c_int)
                + ((*src0.offset((2 as libc::c_int * x + 2 as libc::c_int) as isize)
                    as libc::c_int
                    + *src1.offset((2 as libc::c_int * x + 2 as libc::c_int) as isize)
                        as libc::c_int
                    + 1 as libc::c_int)
                    >> 1 as libc::c_int)
                + 1 as libc::c_int)
                >> 1 as libc::c_int) as pixel;
            *dstv.offset(x as isize) = ((((*src1.offset((2 as libc::c_int * x) as isize)
                as libc::c_int
                + *src2.offset((2 as libc::c_int * x) as isize) as libc::c_int
                + 1 as libc::c_int)
                >> 1 as libc::c_int)
                + ((*src1.offset((2 as libc::c_int * x + 1 as libc::c_int) as isize)
                    as libc::c_int
                    + *src2.offset((2 as libc::c_int * x + 1 as libc::c_int) as isize)
                        as libc::c_int
                    + 1 as libc::c_int)
                    >> 1 as libc::c_int)
                + 1 as libc::c_int)
                >> 1 as libc::c_int) as pixel;
            *dstc.offset(x as isize) = ((((*src1
                .offset((2 as libc::c_int * x + 1 as libc::c_int) as isize)
                as libc::c_int
                + *src2.offset((2 as libc::c_int * x + 1 as libc::c_int) as isize)
                    as libc::c_int
                + 1 as libc::c_int)
                >> 1 as libc::c_int)
                + ((*src1.offset((2 as libc::c_int * x + 2 as libc::c_int) as isize)
                    as libc::c_int
                    + *src2.offset((2 as libc::c_int * x + 2 as libc::c_int) as isize)
                        as libc::c_int
                    + 1 as libc::c_int)
                    >> 1 as libc::c_int)
                + 1 as libc::c_int)
                >> 1 as libc::c_int) as pixel;
            x += 1;
            x;
        }
        src0 = src0.offset((src_stride * 2 as libc::c_int as intptr_t) as isize);
        dst0 = dst0.offset(dst_stride as isize);
        dsth = dsth.offset(dst_stride as isize);
        dstv = dstv.offset(dst_stride as isize);
        dstc = dstc.offset(dst_stride as isize);
        y += 1;
        y;
    }
}
unsafe extern "C" fn mbtree_propagate_cost(
    mut dst: *mut int16_t,
    mut propagate_in: *mut uint16_t,
    mut intra_costs: *mut uint16_t,
    mut inter_costs: *mut uint16_t,
    mut inv_qscales: *mut uint16_t,
    mut fps_factor: *mut libc::c_float,
    mut len: libc::c_int,
) {
    let mut fps: libc::c_float = *fps_factor;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < len {
        let mut intra_cost: libc::c_int = *intra_costs.offset(i as isize) as libc::c_int;
        let mut inter_cost: libc::c_int = if (*intra_costs.offset(i as isize) as libc::c_int)
            < *inter_costs.offset(i as isize) as libc::c_int
                & (((1 as libc::c_int) << 14 as libc::c_int) - 1 as libc::c_int)
        {
            *intra_costs.offset(i as isize) as libc::c_int
        } else {
            *inter_costs.offset(i as isize) as libc::c_int
                & (((1 as libc::c_int) << 14 as libc::c_int) - 1 as libc::c_int)
        };
        let mut propagate_intra: libc::c_float =
            (intra_cost * *inv_qscales.offset(i as isize) as libc::c_int) as libc::c_float;
        let mut propagate_amount: libc::c_float = *propagate_in.offset(i as isize) as libc::c_int
            as libc::c_float
            + propagate_intra * fps;
        let mut propagate_num: libc::c_float = (intra_cost - inter_cost) as libc::c_float;
        let mut propagate_denom: libc::c_float = intra_cost as libc::c_float;
        *dst.offset(i as isize) = (if ((propagate_amount * propagate_num / propagate_denom + 0.5f32)
            as libc::c_int)
            < 32767 as libc::c_int
        {
            (propagate_amount * propagate_num / propagate_denom + 0.5f32) as libc::c_int
        } else {
            32767 as libc::c_int
        }) as int16_t;
        i += 1;
        i;
    }
}
unsafe extern "C" fn mbtree_propagate_list(
    mut h: *mut x264_t,
    mut ref_costs: *mut uint16_t,
    mut mvs: *mut [int16_t; 2],
    mut propagate_amount: *mut int16_t,
    mut lowres_costs: *mut uint16_t,
    mut bipred_weight: libc::c_int,
    mut mb_y: libc::c_int,
    mut len: libc::c_int,
    mut list: libc::c_int,
) {
    let mut stride: libc::c_uint = (*h).mb.i_mb_stride as libc::c_uint;
    let mut width: libc::c_uint = (*h).mb.i_mb_width as libc::c_uint;
    let mut height: libc::c_uint = (*h).mb.i_mb_height as libc::c_uint;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < len {
        let mut lists_used: libc::c_int =
            *lowres_costs.offset(i as isize) as libc::c_int >> 14 as libc::c_int;
        if lists_used & ((1 as libc::c_int) << list) != 0 {
            let mut listamount: libc::c_int = *propagate_amount.offset(i as isize) as libc::c_int;
            if lists_used == 3 as libc::c_int {
                listamount = (listamount * bipred_weight + 32 as libc::c_int) >> 6 as libc::c_int;
            }
            if (*((*mvs.offset(i as isize)).as_mut_ptr() as *mut x264_union32_t)).i == 0 {
                *ref_costs.offset(
                    (mb_y as libc::c_uint)
                        .wrapping_mul(stride)
                        .wrapping_add(i as libc::c_uint) as isize,
                ) = (if *ref_costs.offset(
                    (mb_y as libc::c_uint)
                        .wrapping_mul(stride)
                        .wrapping_add(i as libc::c_uint) as isize,
                ) as libc::c_int
                    + listamount
                    < ((1 as libc::c_int) << 15 as libc::c_int) - 1 as libc::c_int
                {
                    *ref_costs.offset(
                        (mb_y as libc::c_uint)
                            .wrapping_mul(stride)
                            .wrapping_add(i as libc::c_uint) as isize,
                    ) as libc::c_int
                        + listamount
                } else {
                    ((1 as libc::c_int) << 15 as libc::c_int) - 1 as libc::c_int
                }) as uint16_t;
            } else {
                let mut x: libc::c_int =
                    (*mvs.offset(i as isize))[0 as libc::c_int as usize] as libc::c_int;
                let mut y: libc::c_int =
                    (*mvs.offset(i as isize))[1 as libc::c_int as usize] as libc::c_int;
                let mut mbx: libc::c_uint = ((x >> 5 as libc::c_int) + i) as libc::c_uint;
                let mut mby: libc::c_uint = ((y >> 5 as libc::c_int) + mb_y) as libc::c_uint;
                let mut idx0: libc::c_uint = mbx.wrapping_add(mby.wrapping_mul(stride));
                let mut idx2: libc::c_uint = idx0.wrapping_add(stride);
                x &= 31 as libc::c_int;
                y &= 31 as libc::c_int;
                let mut idx0weight: libc::c_int = (32 as libc::c_int - y) * (32 as libc::c_int - x);
                let mut idx1weight: libc::c_int = (32 as libc::c_int - y) * x;
                let mut idx2weight: libc::c_int = y * (32 as libc::c_int - x);
                let mut idx3weight: libc::c_int = y * x;
                idx0weight = (idx0weight * listamount + 512 as libc::c_int) >> 10 as libc::c_int;
                idx1weight = (idx1weight * listamount + 512 as libc::c_int) >> 10 as libc::c_int;
                idx2weight = (idx2weight * listamount + 512 as libc::c_int) >> 10 as libc::c_int;
                idx3weight = (idx3weight * listamount + 512 as libc::c_int) >> 10 as libc::c_int;
                if mbx < width.wrapping_sub(1 as libc::c_int as libc::c_uint)
                    && mby < height.wrapping_sub(1 as libc::c_int as libc::c_uint)
                {
                    *ref_costs
                        .offset(idx0.wrapping_add(0 as libc::c_int as libc::c_uint) as isize) =
                        (if *ref_costs
                            .offset(idx0.wrapping_add(0 as libc::c_int as libc::c_uint) as isize)
                            as libc::c_int
                            + idx0weight
                            < ((1 as libc::c_int) << 15 as libc::c_int) - 1 as libc::c_int
                        {
                            *ref_costs
                            .offset(
                                idx0.wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
                            ) as libc::c_int + idx0weight
                        } else {
                            ((1 as libc::c_int) << 15 as libc::c_int) - 1 as libc::c_int
                        }) as uint16_t;
                    *ref_costs
                        .offset(idx0.wrapping_add(1 as libc::c_int as libc::c_uint) as isize) =
                        (if *ref_costs
                            .offset(idx0.wrapping_add(1 as libc::c_int as libc::c_uint) as isize)
                            as libc::c_int
                            + idx1weight
                            < ((1 as libc::c_int) << 15 as libc::c_int) - 1 as libc::c_int
                        {
                            *ref_costs
                            .offset(
                                idx0.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                            ) as libc::c_int + idx1weight
                        } else {
                            ((1 as libc::c_int) << 15 as libc::c_int) - 1 as libc::c_int
                        }) as uint16_t;
                    *ref_costs
                        .offset(idx2.wrapping_add(0 as libc::c_int as libc::c_uint) as isize) =
                        (if *ref_costs
                            .offset(idx2.wrapping_add(0 as libc::c_int as libc::c_uint) as isize)
                            as libc::c_int
                            + idx2weight
                            < ((1 as libc::c_int) << 15 as libc::c_int) - 1 as libc::c_int
                        {
                            *ref_costs
                            .offset(
                                idx2.wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
                            ) as libc::c_int + idx2weight
                        } else {
                            ((1 as libc::c_int) << 15 as libc::c_int) - 1 as libc::c_int
                        }) as uint16_t;
                    *ref_costs
                        .offset(idx2.wrapping_add(1 as libc::c_int as libc::c_uint) as isize) =
                        (if *ref_costs
                            .offset(idx2.wrapping_add(1 as libc::c_int as libc::c_uint) as isize)
                            as libc::c_int
                            + idx3weight
                            < ((1 as libc::c_int) << 15 as libc::c_int) - 1 as libc::c_int
                        {
                            *ref_costs
                            .offset(
                                idx2.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                            ) as libc::c_int + idx3weight
                        } else {
                            ((1 as libc::c_int) << 15 as libc::c_int) - 1 as libc::c_int
                        }) as uint16_t;
                } else {
                    if mby < height {
                        if mbx < width {
                            *ref_costs.offset(
                                idx0.wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
                            ) =
                                (if *ref_costs
                                    .offset(idx0.wrapping_add(0 as libc::c_int as libc::c_uint)
                                        as isize) as libc::c_int
                                    + idx0weight
                                    < ((1 as libc::c_int) << 15 as libc::c_int) - 1 as libc::c_int
                                {
                                    *ref_costs
                                        .offset(idx0.wrapping_add(0 as libc::c_int as libc::c_uint)
                                            as isize)
                                        as libc::c_int
                                        + idx0weight
                                } else {
                                    ((1 as libc::c_int) << 15 as libc::c_int) - 1 as libc::c_int
                                }) as uint16_t;
                        }
                        if mbx.wrapping_add(1 as libc::c_int as libc::c_uint) < width {
                            *ref_costs.offset(
                                idx0.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                            ) =
                                (if *ref_costs
                                    .offset(idx0.wrapping_add(1 as libc::c_int as libc::c_uint)
                                        as isize) as libc::c_int
                                    + idx1weight
                                    < ((1 as libc::c_int) << 15 as libc::c_int) - 1 as libc::c_int
                                {
                                    *ref_costs
                                        .offset(idx0.wrapping_add(1 as libc::c_int as libc::c_uint)
                                            as isize)
                                        as libc::c_int
                                        + idx1weight
                                } else {
                                    ((1 as libc::c_int) << 15 as libc::c_int) - 1 as libc::c_int
                                }) as uint16_t;
                        }
                    }
                    if mby.wrapping_add(1 as libc::c_int as libc::c_uint) < height {
                        if mbx < width {
                            *ref_costs.offset(
                                idx2.wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
                            ) =
                                (if *ref_costs
                                    .offset(idx2.wrapping_add(0 as libc::c_int as libc::c_uint)
                                        as isize) as libc::c_int
                                    + idx2weight
                                    < ((1 as libc::c_int) << 15 as libc::c_int) - 1 as libc::c_int
                                {
                                    *ref_costs
                                        .offset(idx2.wrapping_add(0 as libc::c_int as libc::c_uint)
                                            as isize)
                                        as libc::c_int
                                        + idx2weight
                                } else {
                                    ((1 as libc::c_int) << 15 as libc::c_int) - 1 as libc::c_int
                                }) as uint16_t;
                        }
                        if mbx.wrapping_add(1 as libc::c_int as libc::c_uint) < width {
                            *ref_costs.offset(
                                idx2.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                            ) =
                                (if *ref_costs
                                    .offset(idx2.wrapping_add(1 as libc::c_int as libc::c_uint)
                                        as isize) as libc::c_int
                                    + idx3weight
                                    < ((1 as libc::c_int) << 15 as libc::c_int) - 1 as libc::c_int
                                {
                                    *ref_costs
                                        .offset(idx2.wrapping_add(1 as libc::c_int as libc::c_uint)
                                            as isize)
                                        as libc::c_int
                                        + idx3weight
                                } else {
                                    ((1 as libc::c_int) << 15 as libc::c_int) - 1 as libc::c_int
                                }) as uint16_t;
                        }
                    }
                }
            }
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn mbtree_fix8_pack(
    mut dst: *mut uint16_t,
    mut src: *mut libc::c_float,
    mut count: libc::c_int,
) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < count {
        *dst.offset(i as isize) =
            endian_fix16((*src.offset(i as isize) * 256.0f32) as int16_t as uint16_t);
        i += 1;
        i;
    }
}
unsafe extern "C" fn mbtree_fix8_unpack(
    mut dst: *mut libc::c_float,
    mut src: *mut uint16_t,
    mut count: libc::c_int,
) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < count {
        *dst.offset(i as isize) = endian_fix16(*src.offset(i as isize)) as int16_t as libc::c_int
            as libc::c_float
            * (1.0f32 / 256.0f32);
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_mc_init(
    mut cpu: uint32_t,
    mut pf: *mut x264_mc_functions_t,
    mut cpu_independent: libc::c_int,
) {
    (*pf).mc_luma = Some(
        mc_luma
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut *mut pixel,
                intptr_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                *const x264_weight_t,
            ) -> (),
    );
    (*pf).get_ref = Some(
        get_ref
            as unsafe extern "C" fn(
                *mut pixel,
                *mut intptr_t,
                *mut *mut pixel,
                intptr_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                *const x264_weight_t,
            ) -> *mut pixel,
    );
    (*pf).mc_chroma = Some(
        mc_chroma
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*pf).avg[PIXEL_16x16 as libc::c_int as usize] = Some(
        pixel_avg_16x16
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                libc::c_int,
            ) -> (),
    );
    (*pf).avg[PIXEL_16x8 as libc::c_int as usize] = Some(
        pixel_avg_16x8
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                libc::c_int,
            ) -> (),
    );
    (*pf).avg[PIXEL_8x16 as libc::c_int as usize] = Some(
        pixel_avg_8x16
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                libc::c_int,
            ) -> (),
    );
    (*pf).avg[PIXEL_8x8 as libc::c_int as usize] = Some(
        pixel_avg_8x8
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                libc::c_int,
            ) -> (),
    );
    (*pf).avg[PIXEL_8x4 as libc::c_int as usize] = Some(
        pixel_avg_8x4
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                libc::c_int,
            ) -> (),
    );
    (*pf).avg[PIXEL_4x16 as libc::c_int as usize] = Some(
        pixel_avg_4x16
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                libc::c_int,
            ) -> (),
    );
    (*pf).avg[PIXEL_4x8 as libc::c_int as usize] = Some(
        pixel_avg_4x8
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                libc::c_int,
            ) -> (),
    );
    (*pf).avg[PIXEL_4x4 as libc::c_int as usize] = Some(
        pixel_avg_4x4
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                libc::c_int,
            ) -> (),
    );
    (*pf).avg[PIXEL_4x2 as libc::c_int as usize] = Some(
        pixel_avg_4x2
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                libc::c_int,
            ) -> (),
    );
    (*pf).avg[PIXEL_2x8 as libc::c_int as usize] = Some(
        pixel_avg_2x8
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                libc::c_int,
            ) -> (),
    );
    (*pf).avg[PIXEL_2x4 as libc::c_int as usize] = Some(
        pixel_avg_2x4
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                libc::c_int,
            ) -> (),
    );
    (*pf).avg[PIXEL_2x2 as libc::c_int as usize] = Some(
        pixel_avg_2x2
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                libc::c_int,
            ) -> (),
    );
    (*pf).weight = mc_weight_wtab.as_mut_ptr();
    (*pf).offsetadd = mc_weight_wtab.as_mut_ptr();
    (*pf).offsetsub = mc_weight_wtab.as_mut_ptr();
    (*pf).weight_cache =
        Some(weight_cache as unsafe extern "C" fn(*mut x264_t, *mut x264_weight_t) -> ());
    (*pf).copy_16x16_unaligned = Some(
        mc_copy_w16
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t, libc::c_int) -> (),
    );
    (*pf).copy[PIXEL_16x16 as libc::c_int as usize] = Some(
        mc_copy_w16
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t, libc::c_int) -> (),
    );
    (*pf).copy[PIXEL_8x8 as libc::c_int as usize] = Some(
        mc_copy_w8
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t, libc::c_int) -> (),
    );
    (*pf).copy[PIXEL_4x4 as libc::c_int as usize] = Some(
        mc_copy_w4
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t, libc::c_int) -> (),
    );
    (*pf).store_interleave_chroma = Some(
        store_interleave_chroma
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                *mut pixel,
                libc::c_int,
            ) -> (),
    );
    (*pf).load_deinterleave_chroma_fenc = Some(
        load_deinterleave_chroma_fenc
            as unsafe extern "C" fn(*mut pixel, *mut pixel, intptr_t, libc::c_int) -> (),
    );
    (*pf).load_deinterleave_chroma_fdec = Some(
        load_deinterleave_chroma_fdec
            as unsafe extern "C" fn(*mut pixel, *mut pixel, intptr_t, libc::c_int) -> (),
    );
    (*pf).plane_copy = Some(
        x264_8_plane_copy_c
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*pf).plane_copy_swap = Some(
        x264_8_plane_copy_swap_c
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*pf).plane_copy_interleave = Some(
        x264_8_plane_copy_interleave_c
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*pf).plane_copy_deinterleave = Some(
        x264_8_plane_copy_deinterleave_c
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*pf).plane_copy_deinterleave_yuyv = Some(
        x264_8_plane_copy_deinterleave_c
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*pf).plane_copy_deinterleave_rgb = Some(
        plane_copy_deinterleave_rgb_c
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*pf).plane_copy_deinterleave_v210 = Some(
        plane_copy_deinterleave_v210_c
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut uint32_t,
                intptr_t,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*pf).hpel_filter = Some(
        hpel_filter
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                libc::c_int,
                libc::c_int,
                *mut int16_t,
            ) -> (),
    );
    (*pf).prefetch_fenc_400 = Some(
        prefetch_fenc_null
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t, libc::c_int) -> (),
    );
    (*pf).prefetch_fenc_420 = Some(
        prefetch_fenc_null
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t, libc::c_int) -> (),
    );
    (*pf).prefetch_fenc_422 = Some(
        prefetch_fenc_null
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t, libc::c_int) -> (),
    );
    (*pf).prefetch_ref =
        Some(prefetch_ref_null as unsafe extern "C" fn(*mut pixel, intptr_t, libc::c_int) -> ());
    (*pf).memcpy_aligned = Some(
        memcpy
            as unsafe extern "C" fn(
                *mut libc::c_void,
                *const libc::c_void,
                libc::c_ulong,
            ) -> *mut libc::c_void,
    );
    (*pf).memzero_aligned =
        Some(memzero_aligned as unsafe extern "C" fn(*mut libc::c_void, size_t) -> ());
    (*pf).frame_init_lowres_core = Some(
        frame_init_lowres_core
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                intptr_t,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*pf).integral_init4h =
        Some(integral_init4h as unsafe extern "C" fn(*mut uint16_t, *mut pixel, intptr_t) -> ());
    (*pf).integral_init8h =
        Some(integral_init8h as unsafe extern "C" fn(*mut uint16_t, *mut pixel, intptr_t) -> ());
    (*pf).integral_init4v =
        Some(integral_init4v as unsafe extern "C" fn(*mut uint16_t, *mut uint16_t, intptr_t) -> ());
    (*pf).integral_init8v =
        Some(integral_init8v as unsafe extern "C" fn(*mut uint16_t, intptr_t) -> ());
    (*pf).mbtree_propagate_cost = Some(
        mbtree_propagate_cost
            as unsafe extern "C" fn(
                *mut int16_t,
                *mut uint16_t,
                *mut uint16_t,
                *mut uint16_t,
                *mut uint16_t,
                *mut libc::c_float,
                libc::c_int,
            ) -> (),
    );
    (*pf).mbtree_propagate_list = Some(
        mbtree_propagate_list
            as unsafe extern "C" fn(
                *mut x264_t,
                *mut uint16_t,
                *mut [int16_t; 2],
                *mut int16_t,
                *mut uint16_t,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*pf).mbtree_fix8_pack = Some(
        mbtree_fix8_pack
            as unsafe extern "C" fn(*mut uint16_t, *mut libc::c_float, libc::c_int) -> (),
    );
    (*pf).mbtree_fix8_unpack = Some(
        mbtree_fix8_unpack
            as unsafe extern "C" fn(*mut libc::c_float, *mut uint16_t, libc::c_int) -> (),
    );
    if cpu_independent != 0 {
        (*pf).mbtree_propagate_cost = Some(
            mbtree_propagate_cost
                as unsafe extern "C" fn(
                    *mut int16_t,
                    *mut uint16_t,
                    *mut uint16_t,
                    *mut uint16_t,
                    *mut uint16_t,
                    *mut libc::c_float,
                    libc::c_int,
                ) -> (),
        );
        (*pf).mbtree_propagate_list = Some(
            mbtree_propagate_list
                as unsafe extern "C" fn(
                    *mut x264_t,
                    *mut uint16_t,
                    *mut [int16_t; 2],
                    *mut int16_t,
                    *mut uint16_t,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                ) -> (),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_frame_filter(
    mut h: *mut x264_t,
    mut frame: *mut x264_frame_t,
    mut mb_y: libc::c_int,
    mut b_end: libc::c_int,
) {
    let b_interlaced: libc::c_int = (*h).param.b_interlaced;
    let mut start: libc::c_int = mb_y * 16 as libc::c_int - 8 as libc::c_int;
    let mut height: libc::c_int = (if b_end != 0 {
        (*frame).i_lines[0 as libc::c_int as usize] + 16 as libc::c_int * (*h).param.b_interlaced
    } else {
        (mb_y + b_interlaced) * 16 as libc::c_int
    }) + 8 as libc::c_int;
    if mb_y & b_interlaced != 0 {
        return;
    }
    let mut p: libc::c_int = 0 as libc::c_int;
    while p
        < (if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as libc::c_int {
            3 as libc::c_int
        } else {
            1 as libc::c_int
        })
    {
        let mut stride: libc::c_int = (*frame).i_stride[p as usize];
        let width: libc::c_int = (*frame).i_width[p as usize];
        let mut offs: libc::c_int = start * stride - 8 as libc::c_int;
        if b_interlaced == 0 || (*h).mb.b_adaptive_mbaff != 0 {
            ((*h).mc.hpel_filter).expect("non-null function pointer")(
                ((*frame).filtered[p as usize][1 as libc::c_int as usize]).offset(offs as isize),
                ((*frame).filtered[p as usize][2 as libc::c_int as usize]).offset(offs as isize),
                ((*frame).filtered[p as usize][3 as libc::c_int as usize]).offset(offs as isize),
                ((*frame).plane[p as usize]).offset(offs as isize),
                stride as intptr_t,
                width + 16 as libc::c_int,
                height - start,
                (*h).scratch_buffer as *mut int16_t,
            );
        }
        if b_interlaced != 0 {
            stride = (*frame).i_stride[p as usize] << 1 as libc::c_int;
            start = ((mb_y * 16 as libc::c_int) >> 1 as libc::c_int) - 8 as libc::c_int;
            let mut height_fld: libc::c_int = ((if b_end != 0 {
                (*frame).i_lines[p as usize]
            } else {
                mb_y * 16 as libc::c_int
            }) >> 1 as libc::c_int)
                + 8 as libc::c_int;
            offs = start * stride - 8 as libc::c_int;
            let mut i: libc::c_int = 0 as libc::c_int;
            while i < 2 as libc::c_int {
                ((*h).mc.hpel_filter).expect("non-null function pointer")(
                    ((*frame).filtered_fld[p as usize][1 as libc::c_int as usize])
                        .offset(offs as isize),
                    ((*frame).filtered_fld[p as usize][2 as libc::c_int as usize])
                        .offset(offs as isize),
                    ((*frame).filtered_fld[p as usize][3 as libc::c_int as usize])
                        .offset(offs as isize),
                    ((*frame).plane_fld[p as usize]).offset(offs as isize),
                    stride as intptr_t,
                    width + 16 as libc::c_int,
                    height_fld - start,
                    (*h).scratch_buffer as *mut int16_t,
                );
                i += 1;
                i;
                offs += (*frame).i_stride[p as usize];
            }
        }
        p += 1;
        p;
    }
    if !((*frame).integral).is_null() {
        let mut stride_0: libc::c_int = (*frame).i_stride[0 as libc::c_int as usize];
        if start < 0 as libc::c_int {
            memset(
                ((*frame).integral)
                    .offset(-((32 as libc::c_int * stride_0) as isize))
                    .offset(
                        -((if 32 as libc::c_int
                            > 64 as libc::c_int
                                / ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int
                        {
                            32 as libc::c_int
                        } else {
                            64 as libc::c_int
                                / ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int
                        }) as isize),
                    ) as *mut libc::c_void,
                0 as libc::c_int,
                (stride_0 as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<uint16_t>() as libc::c_ulong),
            );
            start = -(32 as libc::c_int);
        }
        if b_end != 0 {
            height += 32 as libc::c_int - 9 as libc::c_int;
        }
        let mut y: libc::c_int = start;
        while y < height {
            let mut pix: *mut pixel = ((*frame).plane[0 as libc::c_int as usize])
                .offset((y * stride_0) as isize)
                .offset(
                    -((if 32 as libc::c_int
                        > 64 as libc::c_int
                            / ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int
                    {
                        32 as libc::c_int
                    } else {
                        64 as libc::c_int
                            / ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int
                    }) as isize),
                );
            let mut sum8: *mut uint16_t = ((*frame).integral)
                .offset(((y + 1 as libc::c_int) * stride_0) as isize)
                .offset(
                    -((if 32 as libc::c_int
                        > 64 as libc::c_int
                            / ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int
                    {
                        32 as libc::c_int
                    } else {
                        64 as libc::c_int
                            / ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int
                    }) as isize),
                );
            let mut sum4: *mut uint16_t = std::ptr::null_mut::<uint16_t>();
            if (*h).frames.b_have_sub8x8_esa != 0 {
                ((*h).mc.integral_init4h).expect("non-null function pointer")(
                    sum8,
                    pix,
                    stride_0 as intptr_t,
                );
                sum8 = sum8.offset(-((8 as libc::c_int * stride_0) as isize));
                sum4 = sum8.offset(
                    (stride_0
                        * ((*frame).i_lines[0 as libc::c_int as usize]
                            + 32 as libc::c_int * 2 as libc::c_int)) as isize,
                );
                if y >= 8 as libc::c_int - 32 as libc::c_int {
                    ((*h).mc.integral_init4v).expect("non-null function pointer")(
                        sum8,
                        sum4,
                        stride_0 as intptr_t,
                    );
                }
            } else {
                ((*h).mc.integral_init8h).expect("non-null function pointer")(
                    sum8,
                    pix,
                    stride_0 as intptr_t,
                );
                if y >= 8 as libc::c_int - 32 as libc::c_int {
                    ((*h).mc.integral_init8v).expect("non-null function pointer")(
                        sum8.offset(-((8 as libc::c_int * stride_0) as isize)),
                        stride_0 as intptr_t,
                    );
                }
            }
            y += 1;
            y;
        }
    }
}
