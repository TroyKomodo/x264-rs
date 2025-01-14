#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type pixel = uint8_t;
pub type x264_predict_8x8_filter_t = Option::<
    unsafe extern "C" fn(*mut pixel, *mut pixel, libc::c_int, libc::c_int) -> (),
>;
pub type x264_predict_t = Option::<unsafe extern "C" fn(*mut pixel) -> ()>;
pub type x264_predict8x8_t = Option::<
    unsafe extern "C" fn(*mut pixel, *mut pixel) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union x264_union32_t {
    pub i: uint32_t,
    pub w: [uint16_t; 2],
    pub b: [uint8_t; 4],
}
pub type pixel4 = uint32_t;
pub type intra_chroma_pred_e = libc::c_uint;
pub const I_PRED_CHROMA_DC_128: intra_chroma_pred_e = 6;
pub const I_PRED_CHROMA_DC_TOP: intra_chroma_pred_e = 5;
pub const I_PRED_CHROMA_DC_LEFT: intra_chroma_pred_e = 4;
pub const I_PRED_CHROMA_P: intra_chroma_pred_e = 3;
pub const I_PRED_CHROMA_V: intra_chroma_pred_e = 2;
pub const I_PRED_CHROMA_H: intra_chroma_pred_e = 1;
pub const I_PRED_CHROMA_DC: intra_chroma_pred_e = 0;
pub type intra16x16_pred_e = libc::c_uint;
pub const I_PRED_16x16_DC_128: intra16x16_pred_e = 6;
pub const I_PRED_16x16_DC_TOP: intra16x16_pred_e = 5;
pub const I_PRED_16x16_DC_LEFT: intra16x16_pred_e = 4;
pub const I_PRED_16x16_P: intra16x16_pred_e = 3;
pub const I_PRED_16x16_DC: intra16x16_pred_e = 2;
pub const I_PRED_16x16_H: intra16x16_pred_e = 1;
pub const I_PRED_16x16_V: intra16x16_pred_e = 0;
pub type intra4x4_pred_e = libc::c_uint;
pub const I_PRED_4x4_DC_128: intra4x4_pred_e = 11;
pub const I_PRED_4x4_DC_TOP: intra4x4_pred_e = 10;
pub const I_PRED_4x4_DC_LEFT: intra4x4_pred_e = 9;
pub const I_PRED_4x4_HU: intra4x4_pred_e = 8;
pub const I_PRED_4x4_VL: intra4x4_pred_e = 7;
pub const I_PRED_4x4_HD: intra4x4_pred_e = 6;
pub const I_PRED_4x4_VR: intra4x4_pred_e = 5;
pub const I_PRED_4x4_DDR: intra4x4_pred_e = 4;
pub const I_PRED_4x4_DDL: intra4x4_pred_e = 3;
pub const I_PRED_4x4_DC: intra4x4_pred_e = 2;
pub const I_PRED_4x4_H: intra4x4_pred_e = 1;
pub const I_PRED_4x4_V: intra4x4_pred_e = 0;
pub type intra8x8_pred_e = libc::c_uint;
pub const I_PRED_8x8_DC_128: intra8x8_pred_e = 11;
pub const I_PRED_8x8_DC_TOP: intra8x8_pred_e = 10;
pub const I_PRED_8x8_DC_LEFT: intra8x8_pred_e = 9;
pub const I_PRED_8x8_HU: intra8x8_pred_e = 8;
pub const I_PRED_8x8_VL: intra8x8_pred_e = 7;
pub const I_PRED_8x8_HD: intra8x8_pred_e = 6;
pub const I_PRED_8x8_VR: intra8x8_pred_e = 5;
pub const I_PRED_8x8_DDR: intra8x8_pred_e = 4;
pub const I_PRED_8x8_DDL: intra8x8_pred_e = 3;
pub const I_PRED_8x8_DC: intra8x8_pred_e = 2;
pub const I_PRED_8x8_H: intra8x8_pred_e = 1;
pub const I_PRED_8x8_V: intra8x8_pred_e = 0;
pub const MB_TOPRIGHT: macroblock_position_e = 4;
pub const MB_TOPLEFT: macroblock_position_e = 8;
pub const MB_TOP: macroblock_position_e = 2;
pub const MB_LEFT: macroblock_position_e = 1;
pub type macroblock_position_e = libc::c_uint;
pub const ALL_NEIGHBORS: macroblock_position_e = 15;
pub const MB_PRIVATE: macroblock_position_e = 16;
#[inline(always)]
unsafe extern "C" fn x264_clip_pixel(mut x: libc::c_int) -> pixel {
    return (if x & !(((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int) != 0 {
        -x >> 31 as libc::c_int
            & ((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int
    } else {
        x
    }) as pixel;
}
#[inline(always)]
unsafe extern "C" fn pack8to16(mut a: uint32_t, mut b: uint32_t) -> uint32_t {
    return a.wrapping_add(b << 8 as libc::c_int);
}
#[inline(always)]
unsafe extern "C" fn pack16to32(mut a: uint32_t, mut b: uint32_t) -> uint32_t {
    return a.wrapping_add(b << 16 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_predict_16x16_dc_c(mut src: *mut pixel) {
    let mut dc: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        dc
            += *src.offset((-(1 as libc::c_int) + i * 32 as libc::c_int) as isize)
                as libc::c_int;
        dc += *src.offset((i - 32 as libc::c_int) as isize) as libc::c_int;
        i += 1;
        i;
    }
    let mut dcsplat: pixel4 = ((dc + 16 as libc::c_int >> 5 as libc::c_int)
        as libc::c_uint)
        .wrapping_mul(0x1010101 as libc::c_uint);
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < 16 as libc::c_int {
        (*(src.offset(0 as libc::c_int as isize) as *mut x264_union32_t)).i = dcsplat;
        (*(src.offset(4 as libc::c_int as isize) as *mut x264_union32_t)).i = dcsplat;
        (*(src.offset(8 as libc::c_int as isize) as *mut x264_union32_t)).i = dcsplat;
        (*(src.offset(12 as libc::c_int as isize) as *mut x264_union32_t)).i = dcsplat;
        src = src.offset(32 as libc::c_int as isize);
        i_0 += 1;
        i_0;
    }
}
unsafe extern "C" fn predict_16x16_dc_left_c(mut src: *mut pixel) {
    let mut dc: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        dc
            += *src.offset((-(1 as libc::c_int) + i * 32 as libc::c_int) as isize)
                as libc::c_int;
        i += 1;
        i;
    }
    let mut dcsplat: pixel4 = ((dc + 8 as libc::c_int >> 4 as libc::c_int)
        as libc::c_uint)
        .wrapping_mul(0x1010101 as libc::c_uint);
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < 16 as libc::c_int {
        (*(src.offset(0 as libc::c_int as isize) as *mut x264_union32_t)).i = dcsplat;
        (*(src.offset(4 as libc::c_int as isize) as *mut x264_union32_t)).i = dcsplat;
        (*(src.offset(8 as libc::c_int as isize) as *mut x264_union32_t)).i = dcsplat;
        (*(src.offset(12 as libc::c_int as isize) as *mut x264_union32_t)).i = dcsplat;
        src = src.offset(32 as libc::c_int as isize);
        i_0 += 1;
        i_0;
    }
}
unsafe extern "C" fn predict_16x16_dc_top_c(mut src: *mut pixel) {
    let mut dc: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        dc += *src.offset((i - 32 as libc::c_int) as isize) as libc::c_int;
        i += 1;
        i;
    }
    let mut dcsplat: pixel4 = ((dc + 8 as libc::c_int >> 4 as libc::c_int)
        as libc::c_uint)
        .wrapping_mul(0x1010101 as libc::c_uint);
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < 16 as libc::c_int {
        (*(src.offset(0 as libc::c_int as isize) as *mut x264_union32_t)).i = dcsplat;
        (*(src.offset(4 as libc::c_int as isize) as *mut x264_union32_t)).i = dcsplat;
        (*(src.offset(8 as libc::c_int as isize) as *mut x264_union32_t)).i = dcsplat;
        (*(src.offset(12 as libc::c_int as isize) as *mut x264_union32_t)).i = dcsplat;
        src = src.offset(32 as libc::c_int as isize);
        i_0 += 1;
        i_0;
    }
}
unsafe extern "C" fn predict_16x16_dc_128_c(mut src: *mut pixel) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        (*(src.offset(0 as libc::c_int as isize) as *mut x264_union32_t))
            .i = (((1 as libc::c_int) << 8 as libc::c_int - 1 as libc::c_int)
            as libc::c_uint)
            .wrapping_mul(0x1010101 as libc::c_uint);
        (*(src.offset(4 as libc::c_int as isize) as *mut x264_union32_t))
            .i = (((1 as libc::c_int) << 8 as libc::c_int - 1 as libc::c_int)
            as libc::c_uint)
            .wrapping_mul(0x1010101 as libc::c_uint);
        (*(src.offset(8 as libc::c_int as isize) as *mut x264_union32_t))
            .i = (((1 as libc::c_int) << 8 as libc::c_int - 1 as libc::c_int)
            as libc::c_uint)
            .wrapping_mul(0x1010101 as libc::c_uint);
        (*(src.offset(12 as libc::c_int as isize) as *mut x264_union32_t))
            .i = (((1 as libc::c_int) << 8 as libc::c_int - 1 as libc::c_int)
            as libc::c_uint)
            .wrapping_mul(0x1010101 as libc::c_uint);
        src = src.offset(32 as libc::c_int as isize);
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_predict_16x16_h_c(mut src: *mut pixel) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        let v: pixel4 = (*src.offset(-(1 as libc::c_int) as isize) as libc::c_uint)
            .wrapping_mul(0x1010101 as libc::c_uint);
        (*(src.offset(0 as libc::c_int as isize) as *mut x264_union32_t)).i = v;
        (*(src.offset(4 as libc::c_int as isize) as *mut x264_union32_t)).i = v;
        (*(src.offset(8 as libc::c_int as isize) as *mut x264_union32_t)).i = v;
        (*(src.offset(12 as libc::c_int as isize) as *mut x264_union32_t)).i = v;
        src = src.offset(32 as libc::c_int as isize);
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_predict_16x16_v_c(mut src: *mut pixel) {
    let mut v0: pixel4 = (*(&mut *src
        .offset((0 as libc::c_int - 32 as libc::c_int) as isize) as *mut pixel
        as *mut x264_union32_t))
        .i;
    let mut v1: pixel4 = (*(&mut *src
        .offset((4 as libc::c_int - 32 as libc::c_int) as isize) as *mut pixel
        as *mut x264_union32_t))
        .i;
    let mut v2: pixel4 = (*(&mut *src
        .offset((8 as libc::c_int - 32 as libc::c_int) as isize) as *mut pixel
        as *mut x264_union32_t))
        .i;
    let mut v3: pixel4 = (*(&mut *src
        .offset((12 as libc::c_int - 32 as libc::c_int) as isize) as *mut pixel
        as *mut x264_union32_t))
        .i;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        (*(src.offset(0 as libc::c_int as isize) as *mut x264_union32_t)).i = v0;
        (*(src.offset(4 as libc::c_int as isize) as *mut x264_union32_t)).i = v1;
        (*(src.offset(8 as libc::c_int as isize) as *mut x264_union32_t)).i = v2;
        (*(src.offset(12 as libc::c_int as isize) as *mut x264_union32_t)).i = v3;
        src = src.offset(32 as libc::c_int as isize);
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_predict_16x16_p_c(mut src: *mut pixel) {
    let mut H: libc::c_int = 0 as libc::c_int;
    let mut V: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i <= 7 as libc::c_int {
        H
            += (i + 1 as libc::c_int)
                * (*src.offset((8 as libc::c_int + i - 32 as libc::c_int) as isize)
                    as libc::c_int
                    - *src.offset((6 as libc::c_int - i - 32 as libc::c_int) as isize)
                        as libc::c_int);
        V
            += (i + 1 as libc::c_int)
                * (*src
                    .offset(
                        (-(1 as libc::c_int)
                            + (8 as libc::c_int + i) * 32 as libc::c_int) as isize,
                    ) as libc::c_int
                    - *src
                        .offset(
                            (-(1 as libc::c_int)
                                + (6 as libc::c_int - i) * 32 as libc::c_int) as isize,
                        ) as libc::c_int);
        i += 1;
        i;
    }
    let mut a: libc::c_int = 16 as libc::c_int
        * (*src
            .offset(
                (-(1 as libc::c_int) + 15 as libc::c_int * 32 as libc::c_int) as isize,
            ) as libc::c_int
            + *src.offset((15 as libc::c_int - 32 as libc::c_int) as isize)
                as libc::c_int);
    let mut b: libc::c_int = 5 as libc::c_int * H + 32 as libc::c_int
        >> 6 as libc::c_int;
    let mut c: libc::c_int = 5 as libc::c_int * V + 32 as libc::c_int
        >> 6 as libc::c_int;
    let mut i00: libc::c_int = a - b * 7 as libc::c_int - c * 7 as libc::c_int
        + 16 as libc::c_int;
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < 16 as libc::c_int {
        let mut pix: libc::c_int = i00;
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < 16 as libc::c_int {
            *src.offset(x as isize) = x264_clip_pixel(pix >> 5 as libc::c_int);
            pix += b;
            x += 1;
            x;
        }
        src = src.offset(32 as libc::c_int as isize);
        i00 += c;
        y += 1;
        y;
    }
}
unsafe extern "C" fn predict_8x8c_dc_128_c(mut src: *mut pixel) {
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < 8 as libc::c_int {
        (*(src.offset(0 as libc::c_int as isize) as *mut x264_union32_t))
            .i = (((1 as libc::c_int) << 8 as libc::c_int - 1 as libc::c_int)
            as libc::c_uint)
            .wrapping_mul(0x1010101 as libc::c_uint);
        (*(src.offset(4 as libc::c_int as isize) as *mut x264_union32_t))
            .i = (((1 as libc::c_int) << 8 as libc::c_int - 1 as libc::c_int)
            as libc::c_uint)
            .wrapping_mul(0x1010101 as libc::c_uint);
        src = src.offset(32 as libc::c_int as isize);
        y += 1;
        y;
    }
}
unsafe extern "C" fn predict_8x8c_dc_left_c(mut src: *mut pixel) {
    let mut dc0: libc::c_int = 0 as libc::c_int;
    let mut dc1: libc::c_int = 0 as libc::c_int;
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < 4 as libc::c_int {
        dc0
            += *src.offset((y * 32 as libc::c_int - 1 as libc::c_int) as isize)
                as libc::c_int;
        dc1
            += *src
                .offset(
                    ((y + 4 as libc::c_int) * 32 as libc::c_int - 1 as libc::c_int)
                        as isize,
                ) as libc::c_int;
        y += 1;
        y;
    }
    let mut dc0splat: pixel4 = ((dc0 + 2 as libc::c_int >> 2 as libc::c_int)
        as libc::c_uint)
        .wrapping_mul(0x1010101 as libc::c_uint);
    let mut dc1splat: pixel4 = ((dc1 + 2 as libc::c_int >> 2 as libc::c_int)
        as libc::c_uint)
        .wrapping_mul(0x1010101 as libc::c_uint);
    let mut y_0: libc::c_int = 0 as libc::c_int;
    while y_0 < 4 as libc::c_int {
        (*(src.offset(0 as libc::c_int as isize) as *mut x264_union32_t)).i = dc0splat;
        (*(src.offset(4 as libc::c_int as isize) as *mut x264_union32_t)).i = dc0splat;
        src = src.offset(32 as libc::c_int as isize);
        y_0 += 1;
        y_0;
    }
    let mut y_1: libc::c_int = 0 as libc::c_int;
    while y_1 < 4 as libc::c_int {
        (*(src.offset(0 as libc::c_int as isize) as *mut x264_union32_t)).i = dc1splat;
        (*(src.offset(4 as libc::c_int as isize) as *mut x264_union32_t)).i = dc1splat;
        src = src.offset(32 as libc::c_int as isize);
        y_1 += 1;
        y_1;
    }
}
unsafe extern "C" fn predict_8x8c_dc_top_c(mut src: *mut pixel) {
    let mut dc0: libc::c_int = 0 as libc::c_int;
    let mut dc1: libc::c_int = 0 as libc::c_int;
    let mut x: libc::c_int = 0 as libc::c_int;
    while x < 4 as libc::c_int {
        dc0 += *src.offset((x - 32 as libc::c_int) as isize) as libc::c_int;
        dc1
            += *src.offset((x + 4 as libc::c_int - 32 as libc::c_int) as isize)
                as libc::c_int;
        x += 1;
        x;
    }
    let mut dc0splat: pixel4 = ((dc0 + 2 as libc::c_int >> 2 as libc::c_int)
        as libc::c_uint)
        .wrapping_mul(0x1010101 as libc::c_uint);
    let mut dc1splat: pixel4 = ((dc1 + 2 as libc::c_int >> 2 as libc::c_int)
        as libc::c_uint)
        .wrapping_mul(0x1010101 as libc::c_uint);
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < 8 as libc::c_int {
        (*(src.offset(0 as libc::c_int as isize) as *mut x264_union32_t)).i = dc0splat;
        (*(src.offset(4 as libc::c_int as isize) as *mut x264_union32_t)).i = dc1splat;
        src = src.offset(32 as libc::c_int as isize);
        y += 1;
        y;
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_predict_8x8c_dc_c(mut src: *mut pixel) {
    let mut s0: libc::c_int = 0 as libc::c_int;
    let mut s1: libc::c_int = 0 as libc::c_int;
    let mut s2: libc::c_int = 0 as libc::c_int;
    let mut s3: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        s0 += *src.offset((i - 32 as libc::c_int) as isize) as libc::c_int;
        s1
            += *src.offset((i + 4 as libc::c_int - 32 as libc::c_int) as isize)
                as libc::c_int;
        s2
            += *src.offset((-(1 as libc::c_int) + i * 32 as libc::c_int) as isize)
                as libc::c_int;
        s3
            += *src
                .offset(
                    (-(1 as libc::c_int) + (i + 4 as libc::c_int) * 32 as libc::c_int)
                        as isize,
                ) as libc::c_int;
        i += 1;
        i;
    }
    let mut dc0: pixel4 = ((s0 + s2 + 4 as libc::c_int >> 3 as libc::c_int)
        as libc::c_uint)
        .wrapping_mul(0x1010101 as libc::c_uint);
    let mut dc1: pixel4 = ((s1 + 2 as libc::c_int >> 2 as libc::c_int) as libc::c_uint)
        .wrapping_mul(0x1010101 as libc::c_uint);
    let mut dc2: pixel4 = ((s3 + 2 as libc::c_int >> 2 as libc::c_int) as libc::c_uint)
        .wrapping_mul(0x1010101 as libc::c_uint);
    let mut dc3: pixel4 = ((s1 + s3 + 4 as libc::c_int >> 3 as libc::c_int)
        as libc::c_uint)
        .wrapping_mul(0x1010101 as libc::c_uint);
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < 4 as libc::c_int {
        (*(src.offset(0 as libc::c_int as isize) as *mut x264_union32_t)).i = dc0;
        (*(src.offset(4 as libc::c_int as isize) as *mut x264_union32_t)).i = dc1;
        src = src.offset(32 as libc::c_int as isize);
        y += 1;
        y;
    }
    let mut y_0: libc::c_int = 0 as libc::c_int;
    while y_0 < 4 as libc::c_int {
        (*(src.offset(0 as libc::c_int as isize) as *mut x264_union32_t)).i = dc2;
        (*(src.offset(4 as libc::c_int as isize) as *mut x264_union32_t)).i = dc3;
        src = src.offset(32 as libc::c_int as isize);
        y_0 += 1;
        y_0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_predict_8x8c_h_c(mut src: *mut pixel) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        let mut v: pixel4 = (*src.offset(-(1 as libc::c_int) as isize) as libc::c_uint)
            .wrapping_mul(0x1010101 as libc::c_uint);
        (*(src.offset(0 as libc::c_int as isize) as *mut x264_union32_t)).i = v;
        (*(src.offset(4 as libc::c_int as isize) as *mut x264_union32_t)).i = v;
        src = src.offset(32 as libc::c_int as isize);
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_predict_8x8c_v_c(mut src: *mut pixel) {
    let mut v0: pixel4 = (*(src
        .offset(0 as libc::c_int as isize)
        .offset(-(32 as libc::c_int as isize)) as *mut x264_union32_t))
        .i;
    let mut v1: pixel4 = (*(src
        .offset(4 as libc::c_int as isize)
        .offset(-(32 as libc::c_int as isize)) as *mut x264_union32_t))
        .i;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        (*(src.offset(0 as libc::c_int as isize) as *mut x264_union32_t)).i = v0;
        (*(src.offset(4 as libc::c_int as isize) as *mut x264_union32_t)).i = v1;
        src = src.offset(32 as libc::c_int as isize);
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_predict_8x8c_p_c(mut src: *mut pixel) {
    let mut H: libc::c_int = 0 as libc::c_int;
    let mut V: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        H
            += (i + 1 as libc::c_int)
                * (*src.offset((4 as libc::c_int + i - 32 as libc::c_int) as isize)
                    as libc::c_int
                    - *src.offset((2 as libc::c_int - i - 32 as libc::c_int) as isize)
                        as libc::c_int);
        V
            += (i + 1 as libc::c_int)
                * (*src
                    .offset(
                        (-(1 as libc::c_int)
                            + (i + 4 as libc::c_int) * 32 as libc::c_int) as isize,
                    ) as libc::c_int
                    - *src
                        .offset(
                            (-(1 as libc::c_int)
                                + (2 as libc::c_int - i) * 32 as libc::c_int) as isize,
                        ) as libc::c_int);
        i += 1;
        i;
    }
    let mut a: libc::c_int = 16 as libc::c_int
        * (*src
            .offset(
                (-(1 as libc::c_int) + 7 as libc::c_int * 32 as libc::c_int) as isize,
            ) as libc::c_int
            + *src.offset((7 as libc::c_int - 32 as libc::c_int) as isize)
                as libc::c_int);
    let mut b: libc::c_int = 17 as libc::c_int * H + 16 as libc::c_int
        >> 5 as libc::c_int;
    let mut c: libc::c_int = 17 as libc::c_int * V + 16 as libc::c_int
        >> 5 as libc::c_int;
    let mut i00: libc::c_int = a - 3 as libc::c_int * b - 3 as libc::c_int * c
        + 16 as libc::c_int;
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < 8 as libc::c_int {
        let mut pix: libc::c_int = i00;
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < 8 as libc::c_int {
            *src.offset(x as isize) = x264_clip_pixel(pix >> 5 as libc::c_int);
            pix += b;
            x += 1;
            x;
        }
        src = src.offset(32 as libc::c_int as isize);
        i00 += c;
        y += 1;
        y;
    }
}
unsafe extern "C" fn predict_8x16c_dc_128_c(mut src: *mut pixel) {
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < 16 as libc::c_int {
        (*(src.offset(0 as libc::c_int as isize) as *mut x264_union32_t))
            .i = (((1 as libc::c_int) << 8 as libc::c_int - 1 as libc::c_int)
            as libc::c_uint)
            .wrapping_mul(0x1010101 as libc::c_uint);
        (*(src.offset(4 as libc::c_int as isize) as *mut x264_union32_t))
            .i = (((1 as libc::c_int) << 8 as libc::c_int - 1 as libc::c_int)
            as libc::c_uint)
            .wrapping_mul(0x1010101 as libc::c_uint);
        src = src.offset(32 as libc::c_int as isize);
        y += 1;
        y;
    }
}
unsafe extern "C" fn predict_8x16c_dc_left_c(mut src: *mut pixel) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        let mut dc: libc::c_int = 0 as libc::c_int;
        let mut y: libc::c_int = 0 as libc::c_int;
        while y < 4 as libc::c_int {
            dc
                += *src.offset((y * 32 as libc::c_int - 1 as libc::c_int) as isize)
                    as libc::c_int;
            y += 1;
            y;
        }
        let mut dcsplat: pixel4 = ((dc + 2 as libc::c_int >> 2 as libc::c_int)
            as libc::c_uint)
            .wrapping_mul(0x1010101 as libc::c_uint);
        let mut y_0: libc::c_int = 0 as libc::c_int;
        while y_0 < 4 as libc::c_int {
            (*(src.offset(0 as libc::c_int as isize) as *mut x264_union32_t))
                .i = dcsplat;
            (*(src.offset(4 as libc::c_int as isize) as *mut x264_union32_t))
                .i = dcsplat;
            src = src.offset(32 as libc::c_int as isize);
            y_0 += 1;
            y_0;
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn predict_8x16c_dc_top_c(mut src: *mut pixel) {
    let mut dc0: libc::c_int = 0 as libc::c_int;
    let mut dc1: libc::c_int = 0 as libc::c_int;
    let mut x: libc::c_int = 0 as libc::c_int;
    while x < 4 as libc::c_int {
        dc0 += *src.offset((x - 32 as libc::c_int) as isize) as libc::c_int;
        dc1
            += *src.offset((x + 4 as libc::c_int - 32 as libc::c_int) as isize)
                as libc::c_int;
        x += 1;
        x;
    }
    let mut dc0splat: pixel4 = ((dc0 + 2 as libc::c_int >> 2 as libc::c_int)
        as libc::c_uint)
        .wrapping_mul(0x1010101 as libc::c_uint);
    let mut dc1splat: pixel4 = ((dc1 + 2 as libc::c_int >> 2 as libc::c_int)
        as libc::c_uint)
        .wrapping_mul(0x1010101 as libc::c_uint);
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < 16 as libc::c_int {
        (*(src.offset(0 as libc::c_int as isize) as *mut x264_union32_t)).i = dc0splat;
        (*(src.offset(4 as libc::c_int as isize) as *mut x264_union32_t)).i = dc1splat;
        src = src.offset(32 as libc::c_int as isize);
        y += 1;
        y;
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_predict_8x16c_dc_c(mut src: *mut pixel) {
    let mut s0: libc::c_int = 0 as libc::c_int;
    let mut s1: libc::c_int = 0 as libc::c_int;
    let mut s2: libc::c_int = 0 as libc::c_int;
    let mut s3: libc::c_int = 0 as libc::c_int;
    let mut s4: libc::c_int = 0 as libc::c_int;
    let mut s5: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        s0
            += *src.offset((i + 0 as libc::c_int - 32 as libc::c_int) as isize)
                as libc::c_int;
        s1
            += *src.offset((i + 4 as libc::c_int - 32 as libc::c_int) as isize)
                as libc::c_int;
        s2
            += *src
                .offset(
                    (-(1 as libc::c_int) + (i + 0 as libc::c_int) * 32 as libc::c_int)
                        as isize,
                ) as libc::c_int;
        s3
            += *src
                .offset(
                    (-(1 as libc::c_int) + (i + 4 as libc::c_int) * 32 as libc::c_int)
                        as isize,
                ) as libc::c_int;
        s4
            += *src
                .offset(
                    (-(1 as libc::c_int) + (i + 8 as libc::c_int) * 32 as libc::c_int)
                        as isize,
                ) as libc::c_int;
        s5
            += *src
                .offset(
                    (-(1 as libc::c_int) + (i + 12 as libc::c_int) * 32 as libc::c_int)
                        as isize,
                ) as libc::c_int;
        i += 1;
        i;
    }
    let mut dc0: pixel4 = ((s0 + s2 + 4 as libc::c_int >> 3 as libc::c_int)
        as libc::c_uint)
        .wrapping_mul(0x1010101 as libc::c_uint);
    let mut dc1: pixel4 = ((s1 + 2 as libc::c_int >> 2 as libc::c_int) as libc::c_uint)
        .wrapping_mul(0x1010101 as libc::c_uint);
    let mut dc2: pixel4 = ((s3 + 2 as libc::c_int >> 2 as libc::c_int) as libc::c_uint)
        .wrapping_mul(0x1010101 as libc::c_uint);
    let mut dc3: pixel4 = ((s1 + s3 + 4 as libc::c_int >> 3 as libc::c_int)
        as libc::c_uint)
        .wrapping_mul(0x1010101 as libc::c_uint);
    let mut dc4: pixel4 = ((s4 + 2 as libc::c_int >> 2 as libc::c_int) as libc::c_uint)
        .wrapping_mul(0x1010101 as libc::c_uint);
    let mut dc5: pixel4 = ((s1 + s4 + 4 as libc::c_int >> 3 as libc::c_int)
        as libc::c_uint)
        .wrapping_mul(0x1010101 as libc::c_uint);
    let mut dc6: pixel4 = ((s5 + 2 as libc::c_int >> 2 as libc::c_int) as libc::c_uint)
        .wrapping_mul(0x1010101 as libc::c_uint);
    let mut dc7: pixel4 = ((s1 + s5 + 4 as libc::c_int >> 3 as libc::c_int)
        as libc::c_uint)
        .wrapping_mul(0x1010101 as libc::c_uint);
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < 4 as libc::c_int {
        (*(src.offset(0 as libc::c_int as isize) as *mut x264_union32_t)).i = dc0;
        (*(src.offset(4 as libc::c_int as isize) as *mut x264_union32_t)).i = dc1;
        src = src.offset(32 as libc::c_int as isize);
        y += 1;
        y;
    }
    let mut y_0: libc::c_int = 0 as libc::c_int;
    while y_0 < 4 as libc::c_int {
        (*(src.offset(0 as libc::c_int as isize) as *mut x264_union32_t)).i = dc2;
        (*(src.offset(4 as libc::c_int as isize) as *mut x264_union32_t)).i = dc3;
        src = src.offset(32 as libc::c_int as isize);
        y_0 += 1;
        y_0;
    }
    let mut y_1: libc::c_int = 0 as libc::c_int;
    while y_1 < 4 as libc::c_int {
        (*(src.offset(0 as libc::c_int as isize) as *mut x264_union32_t)).i = dc4;
        (*(src.offset(4 as libc::c_int as isize) as *mut x264_union32_t)).i = dc5;
        src = src.offset(32 as libc::c_int as isize);
        y_1 += 1;
        y_1;
    }
    let mut y_2: libc::c_int = 0 as libc::c_int;
    while y_2 < 4 as libc::c_int {
        (*(src.offset(0 as libc::c_int as isize) as *mut x264_union32_t)).i = dc6;
        (*(src.offset(4 as libc::c_int as isize) as *mut x264_union32_t)).i = dc7;
        src = src.offset(32 as libc::c_int as isize);
        y_2 += 1;
        y_2;
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_predict_8x16c_h_c(mut src: *mut pixel) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        let mut v: pixel4 = (*src.offset(-(1 as libc::c_int) as isize) as libc::c_uint)
            .wrapping_mul(0x1010101 as libc::c_uint);
        (*(src.offset(0 as libc::c_int as isize) as *mut x264_union32_t)).i = v;
        (*(src.offset(4 as libc::c_int as isize) as *mut x264_union32_t)).i = v;
        src = src.offset(32 as libc::c_int as isize);
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_predict_8x16c_v_c(mut src: *mut pixel) {
    let mut v0: pixel4 = (*(src
        .offset(0 as libc::c_int as isize)
        .offset(-(32 as libc::c_int as isize)) as *mut x264_union32_t))
        .i;
    let mut v1: pixel4 = (*(src
        .offset(4 as libc::c_int as isize)
        .offset(-(32 as libc::c_int as isize)) as *mut x264_union32_t))
        .i;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        (*(src.offset(0 as libc::c_int as isize) as *mut x264_union32_t)).i = v0;
        (*(src.offset(4 as libc::c_int as isize) as *mut x264_union32_t)).i = v1;
        src = src.offset(32 as libc::c_int as isize);
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_predict_8x16c_p_c(mut src: *mut pixel) {
    let mut H: libc::c_int = 0 as libc::c_int;
    let mut V: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        H
            += (i + 1 as libc::c_int)
                * (*src.offset((4 as libc::c_int + i - 32 as libc::c_int) as isize)
                    as libc::c_int
                    - *src.offset((2 as libc::c_int - i - 32 as libc::c_int) as isize)
                        as libc::c_int);
        i += 1;
        i;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < 8 as libc::c_int {
        V
            += (i_0 + 1 as libc::c_int)
                * (*src
                    .offset(
                        (-(1 as libc::c_int)
                            + (i_0 + 8 as libc::c_int) * 32 as libc::c_int) as isize,
                    ) as libc::c_int
                    - *src
                        .offset(
                            (-(1 as libc::c_int)
                                + (6 as libc::c_int - i_0) * 32 as libc::c_int) as isize,
                        ) as libc::c_int);
        i_0 += 1;
        i_0;
    }
    let mut a: libc::c_int = 16 as libc::c_int
        * (*src
            .offset(
                (-(1 as libc::c_int) + 15 as libc::c_int * 32 as libc::c_int) as isize,
            ) as libc::c_int
            + *src.offset((7 as libc::c_int - 32 as libc::c_int) as isize)
                as libc::c_int);
    let mut b: libc::c_int = 17 as libc::c_int * H + 16 as libc::c_int
        >> 5 as libc::c_int;
    let mut c: libc::c_int = 5 as libc::c_int * V + 32 as libc::c_int
        >> 6 as libc::c_int;
    let mut i00: libc::c_int = a - 3 as libc::c_int * b - 7 as libc::c_int * c
        + 16 as libc::c_int;
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < 16 as libc::c_int {
        let mut pix: libc::c_int = i00;
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < 8 as libc::c_int {
            *src.offset(x as isize) = x264_clip_pixel(pix >> 5 as libc::c_int);
            pix += b;
            x += 1;
            x;
        }
        src = src.offset(32 as libc::c_int as isize);
        i00 += c;
        y += 1;
        y;
    }
}
unsafe extern "C" fn predict_4x4_dc_128_c(mut src: *mut pixel) {
    let ref mut fresh0 = (*(&mut *src
        .offset((0 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize)
        as *mut pixel as *mut x264_union32_t))
        .i;
    *fresh0 = (((1 as libc::c_int) << 8 as libc::c_int - 1 as libc::c_int)
        as libc::c_uint)
        .wrapping_mul(0x1010101 as libc::c_uint);
    let ref mut fresh1 = (*(&mut *src
        .offset((0 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize)
        as *mut pixel as *mut x264_union32_t))
        .i;
    *fresh1 = *fresh0;
    let ref mut fresh2 = (*(&mut *src
        .offset((0 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize)
        as *mut pixel as *mut x264_union32_t))
        .i;
    *fresh2 = *fresh1;
    (*(&mut *src
        .offset((0 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize)
        as *mut pixel as *mut x264_union32_t))
        .i = *fresh2;
}
unsafe extern "C" fn predict_4x4_dc_left_c(mut src: *mut pixel) {
    let mut dc: pixel4 = ((*src
        .offset((-(1 as libc::c_int) + 0 as libc::c_int * 32 as libc::c_int) as isize)
        as libc::c_int
        + *src
            .offset(
                (-(1 as libc::c_int) + 1 as libc::c_int * 32 as libc::c_int) as isize,
            ) as libc::c_int
        + *src
            .offset(
                (-(1 as libc::c_int) + 2 as libc::c_int * 32 as libc::c_int) as isize,
            ) as libc::c_int
        + *src
            .offset(
                (-(1 as libc::c_int) + 3 as libc::c_int * 32 as libc::c_int) as isize,
            ) as libc::c_int + 2 as libc::c_int >> 2 as libc::c_int) as libc::c_uint)
        .wrapping_mul(0x1010101 as libc::c_uint);
    let ref mut fresh3 = (*(&mut *src
        .offset((0 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize)
        as *mut pixel as *mut x264_union32_t))
        .i;
    *fresh3 = dc;
    let ref mut fresh4 = (*(&mut *src
        .offset((0 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize)
        as *mut pixel as *mut x264_union32_t))
        .i;
    *fresh4 = *fresh3;
    let ref mut fresh5 = (*(&mut *src
        .offset((0 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize)
        as *mut pixel as *mut x264_union32_t))
        .i;
    *fresh5 = *fresh4;
    (*(&mut *src
        .offset((0 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize)
        as *mut pixel as *mut x264_union32_t))
        .i = *fresh5;
}
unsafe extern "C" fn predict_4x4_dc_top_c(mut src: *mut pixel) {
    let mut dc: pixel4 = ((*src
        .offset((0 as libc::c_int + -(1 as libc::c_int) * 32 as libc::c_int) as isize)
        as libc::c_int
        + *src
            .offset(
                (1 as libc::c_int + -(1 as libc::c_int) * 32 as libc::c_int) as isize,
            ) as libc::c_int
        + *src
            .offset(
                (2 as libc::c_int + -(1 as libc::c_int) * 32 as libc::c_int) as isize,
            ) as libc::c_int
        + *src
            .offset(
                (3 as libc::c_int + -(1 as libc::c_int) * 32 as libc::c_int) as isize,
            ) as libc::c_int + 2 as libc::c_int >> 2 as libc::c_int) as libc::c_uint)
        .wrapping_mul(0x1010101 as libc::c_uint);
    let ref mut fresh6 = (*(&mut *src
        .offset((0 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize)
        as *mut pixel as *mut x264_union32_t))
        .i;
    *fresh6 = dc;
    let ref mut fresh7 = (*(&mut *src
        .offset((0 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize)
        as *mut pixel as *mut x264_union32_t))
        .i;
    *fresh7 = *fresh6;
    let ref mut fresh8 = (*(&mut *src
        .offset((0 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize)
        as *mut pixel as *mut x264_union32_t))
        .i;
    *fresh8 = *fresh7;
    (*(&mut *src
        .offset((0 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize)
        as *mut pixel as *mut x264_union32_t))
        .i = *fresh8;
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_predict_4x4_dc_c(mut src: *mut pixel) {
    let mut dc: pixel4 = ((*src
        .offset((-(1 as libc::c_int) + 0 as libc::c_int * 32 as libc::c_int) as isize)
        as libc::c_int
        + *src
            .offset(
                (-(1 as libc::c_int) + 1 as libc::c_int * 32 as libc::c_int) as isize,
            ) as libc::c_int
        + *src
            .offset(
                (-(1 as libc::c_int) + 2 as libc::c_int * 32 as libc::c_int) as isize,
            ) as libc::c_int
        + *src
            .offset(
                (-(1 as libc::c_int) + 3 as libc::c_int * 32 as libc::c_int) as isize,
            ) as libc::c_int
        + *src
            .offset(
                (0 as libc::c_int + -(1 as libc::c_int) * 32 as libc::c_int) as isize,
            ) as libc::c_int
        + *src
            .offset(
                (1 as libc::c_int + -(1 as libc::c_int) * 32 as libc::c_int) as isize,
            ) as libc::c_int
        + *src
            .offset(
                (2 as libc::c_int + -(1 as libc::c_int) * 32 as libc::c_int) as isize,
            ) as libc::c_int
        + *src
            .offset(
                (3 as libc::c_int + -(1 as libc::c_int) * 32 as libc::c_int) as isize,
            ) as libc::c_int + 4 as libc::c_int >> 3 as libc::c_int) as libc::c_uint)
        .wrapping_mul(0x1010101 as libc::c_uint);
    let ref mut fresh9 = (*(&mut *src
        .offset((0 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize)
        as *mut pixel as *mut x264_union32_t))
        .i;
    *fresh9 = dc;
    let ref mut fresh10 = (*(&mut *src
        .offset((0 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize)
        as *mut pixel as *mut x264_union32_t))
        .i;
    *fresh10 = *fresh9;
    let ref mut fresh11 = (*(&mut *src
        .offset((0 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize)
        as *mut pixel as *mut x264_union32_t))
        .i;
    *fresh11 = *fresh10;
    (*(&mut *src
        .offset((0 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize)
        as *mut pixel as *mut x264_union32_t))
        .i = *fresh11;
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_predict_4x4_h_c(mut src: *mut pixel) {
    (*(&mut *src
        .offset((0 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize)
        as *mut pixel as *mut x264_union32_t))
        .i = (*src
        .offset((-(1 as libc::c_int) + 0 as libc::c_int * 32 as libc::c_int) as isize)
        as libc::c_uint)
        .wrapping_mul(0x1010101 as libc::c_uint);
    (*(&mut *src
        .offset((0 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize)
        as *mut pixel as *mut x264_union32_t))
        .i = (*src
        .offset((-(1 as libc::c_int) + 1 as libc::c_int * 32 as libc::c_int) as isize)
        as libc::c_uint)
        .wrapping_mul(0x1010101 as libc::c_uint);
    (*(&mut *src
        .offset((0 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize)
        as *mut pixel as *mut x264_union32_t))
        .i = (*src
        .offset((-(1 as libc::c_int) + 2 as libc::c_int * 32 as libc::c_int) as isize)
        as libc::c_uint)
        .wrapping_mul(0x1010101 as libc::c_uint);
    (*(&mut *src
        .offset((0 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize)
        as *mut pixel as *mut x264_union32_t))
        .i = (*src
        .offset((-(1 as libc::c_int) + 3 as libc::c_int * 32 as libc::c_int) as isize)
        as libc::c_uint)
        .wrapping_mul(0x1010101 as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_predict_4x4_v_c(mut src: *mut pixel) {
    let ref mut fresh12 = (*(&mut *src
        .offset((0 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize)
        as *mut pixel as *mut x264_union32_t))
        .i;
    *fresh12 = (*(&mut *src
        .offset((0 as libc::c_int + -(1 as libc::c_int) * 32 as libc::c_int) as isize)
        as *mut pixel as *mut x264_union32_t))
        .i;
    let ref mut fresh13 = (*(&mut *src
        .offset((0 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize)
        as *mut pixel as *mut x264_union32_t))
        .i;
    *fresh13 = *fresh12;
    let ref mut fresh14 = (*(&mut *src
        .offset((0 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize)
        as *mut pixel as *mut x264_union32_t))
        .i;
    *fresh14 = *fresh13;
    (*(&mut *src
        .offset((0 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize)
        as *mut pixel as *mut x264_union32_t))
        .i = *fresh14;
}
unsafe extern "C" fn predict_4x4_ddl_c(mut src: *mut pixel) {
    let mut t0: libc::c_int = *src
        .offset((0 as libc::c_int + -(1 as libc::c_int) * 32 as libc::c_int) as isize)
        as libc::c_int;
    let mut t1: libc::c_int = *src
        .offset((1 as libc::c_int + -(1 as libc::c_int) * 32 as libc::c_int) as isize)
        as libc::c_int;
    let mut t2: libc::c_int = *src
        .offset((2 as libc::c_int + -(1 as libc::c_int) * 32 as libc::c_int) as isize)
        as libc::c_int;
    let mut t3: libc::c_int = *src
        .offset((3 as libc::c_int + -(1 as libc::c_int) * 32 as libc::c_int) as isize)
        as libc::c_int;
    let mut t4: libc::c_int = *src
        .offset((4 as libc::c_int + -(1 as libc::c_int) * 32 as libc::c_int) as isize)
        as libc::c_int;
    let mut t5: libc::c_int = *src
        .offset((5 as libc::c_int + -(1 as libc::c_int) * 32 as libc::c_int) as isize)
        as libc::c_int;
    let mut t6: libc::c_int = *src
        .offset((6 as libc::c_int + -(1 as libc::c_int) * 32 as libc::c_int) as isize)
        as libc::c_int;
    let mut t7: libc::c_int = *src
        .offset((7 as libc::c_int + -(1 as libc::c_int) * 32 as libc::c_int) as isize)
        as libc::c_int;
    *src
        .offset(
            (0 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = (t0 + 2 as libc::c_int * t1 + t2 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    let ref mut fresh15 = *src
        .offset((0 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh15 = (t1 + 2 as libc::c_int * t2 + t3 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    *src
        .offset(
            (1 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh15;
    let ref mut fresh16 = *src
        .offset((0 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh16 = (t2 + 2 as libc::c_int * t3 + t4 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    let ref mut fresh17 = *src
        .offset((1 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh17 = *fresh16;
    *src
        .offset(
            (2 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh17;
    let ref mut fresh18 = *src
        .offset((0 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh18 = (t3 + 2 as libc::c_int * t4 + t5 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    let ref mut fresh19 = *src
        .offset((1 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh19 = *fresh18;
    let ref mut fresh20 = *src
        .offset((2 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh20 = *fresh19;
    *src
        .offset(
            (3 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh20;
    let ref mut fresh21 = *src
        .offset((1 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh21 = (t4 + 2 as libc::c_int * t5 + t6 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    let ref mut fresh22 = *src
        .offset((2 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh22 = *fresh21;
    *src
        .offset(
            (3 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh22;
    let ref mut fresh23 = *src
        .offset((2 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh23 = (t5 + 2 as libc::c_int * t6 + t7 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    *src
        .offset(
            (3 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh23;
    *src
        .offset(
            (3 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize,
        ) = (t6 + 2 as libc::c_int * t7 + t7 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
}
unsafe extern "C" fn predict_4x4_ddr_c(mut src: *mut pixel) {
    let mut lt: libc::c_int = *src
        .offset((-(1 as libc::c_int) + -(1 as libc::c_int) * 32 as libc::c_int) as isize)
        as libc::c_int;
    let mut l0: libc::c_int = *src
        .offset((-(1 as libc::c_int) + 0 as libc::c_int * 32 as libc::c_int) as isize)
        as libc::c_int;
    let mut l1: libc::c_int = *src
        .offset((-(1 as libc::c_int) + 1 as libc::c_int * 32 as libc::c_int) as isize)
        as libc::c_int;
    let mut l2: libc::c_int = *src
        .offset((-(1 as libc::c_int) + 2 as libc::c_int * 32 as libc::c_int) as isize)
        as libc::c_int;
    let mut l3: libc::c_int = *src
        .offset((-(1 as libc::c_int) + 3 as libc::c_int * 32 as libc::c_int) as isize)
        as libc::c_int;
    let mut t0: libc::c_int = *src
        .offset((0 as libc::c_int + -(1 as libc::c_int) * 32 as libc::c_int) as isize)
        as libc::c_int;
    let mut t1: libc::c_int = *src
        .offset((1 as libc::c_int + -(1 as libc::c_int) * 32 as libc::c_int) as isize)
        as libc::c_int;
    let mut t2: libc::c_int = *src
        .offset((2 as libc::c_int + -(1 as libc::c_int) * 32 as libc::c_int) as isize)
        as libc::c_int;
    let mut t3: libc::c_int = *src
        .offset((3 as libc::c_int + -(1 as libc::c_int) * 32 as libc::c_int) as isize)
        as libc::c_int;
    *src
        .offset(
            (3 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = (t3 + 2 as libc::c_int * t2 + t1 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    let ref mut fresh24 = *src
        .offset((3 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh24 = (t2 + 2 as libc::c_int * t1 + t0 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    *src
        .offset(
            (2 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh24;
    let ref mut fresh25 = *src
        .offset((3 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh25 = (t1 + 2 as libc::c_int * t0 + lt + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    let ref mut fresh26 = *src
        .offset((2 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh26 = *fresh25;
    *src
        .offset(
            (1 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh26;
    let ref mut fresh27 = *src
        .offset((3 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh27 = (t0 + 2 as libc::c_int * lt + l0 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    let ref mut fresh28 = *src
        .offset((2 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh28 = *fresh27;
    let ref mut fresh29 = *src
        .offset((1 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh29 = *fresh28;
    *src
        .offset(
            (0 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh29;
    let ref mut fresh30 = *src
        .offset((2 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh30 = (lt + 2 as libc::c_int * l0 + l1 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    let ref mut fresh31 = *src
        .offset((1 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh31 = *fresh30;
    *src
        .offset(
            (0 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh31;
    let ref mut fresh32 = *src
        .offset((1 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh32 = (l0 + 2 as libc::c_int * l1 + l2 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    *src
        .offset(
            (0 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh32;
    *src
        .offset(
            (0 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize,
        ) = (l1 + 2 as libc::c_int * l2 + l3 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
}
unsafe extern "C" fn predict_4x4_vr_c(mut src: *mut pixel) {
    let mut lt: libc::c_int = *src
        .offset((-(1 as libc::c_int) + -(1 as libc::c_int) * 32 as libc::c_int) as isize)
        as libc::c_int;
    let mut l0: libc::c_int = *src
        .offset((-(1 as libc::c_int) + 0 as libc::c_int * 32 as libc::c_int) as isize)
        as libc::c_int;
    let mut l1: libc::c_int = *src
        .offset((-(1 as libc::c_int) + 1 as libc::c_int * 32 as libc::c_int) as isize)
        as libc::c_int;
    let mut l2: libc::c_int = *src
        .offset((-(1 as libc::c_int) + 2 as libc::c_int * 32 as libc::c_int) as isize)
        as libc::c_int;
    let mut l3: libc::c_int = *src
        .offset((-(1 as libc::c_int) + 3 as libc::c_int * 32 as libc::c_int) as isize)
        as libc::c_int;
    let mut t0: libc::c_int = *src
        .offset((0 as libc::c_int + -(1 as libc::c_int) * 32 as libc::c_int) as isize)
        as libc::c_int;
    let mut t1: libc::c_int = *src
        .offset((1 as libc::c_int + -(1 as libc::c_int) * 32 as libc::c_int) as isize)
        as libc::c_int;
    let mut t2: libc::c_int = *src
        .offset((2 as libc::c_int + -(1 as libc::c_int) * 32 as libc::c_int) as isize)
        as libc::c_int;
    let mut t3: libc::c_int = *src
        .offset((3 as libc::c_int + -(1 as libc::c_int) * 32 as libc::c_int) as isize)
        as libc::c_int;
    *src
        .offset(
            (0 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize,
        ) = (l2 + 2 as libc::c_int * l1 + l0 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    *src
        .offset(
            (0 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize,
        ) = (l1 + 2 as libc::c_int * l0 + lt + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    let ref mut fresh33 = *src
        .offset((1 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh33 = (l0 + 2 as libc::c_int * lt + t0 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    *src
        .offset(
            (0 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh33;
    let ref mut fresh34 = *src
        .offset((1 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh34 = (lt + t0 + 1 as libc::c_int >> 1 as libc::c_int) as pixel;
    *src
        .offset(
            (0 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh34;
    let ref mut fresh35 = *src
        .offset((2 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh35 = (lt + 2 as libc::c_int * t0 + t1 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    *src
        .offset(
            (1 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh35;
    let ref mut fresh36 = *src
        .offset((2 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh36 = (t0 + t1 + 1 as libc::c_int >> 1 as libc::c_int) as pixel;
    *src
        .offset(
            (1 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh36;
    let ref mut fresh37 = *src
        .offset((3 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh37 = (t0 + 2 as libc::c_int * t1 + t2 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    *src
        .offset(
            (2 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh37;
    let ref mut fresh38 = *src
        .offset((3 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh38 = (t1 + t2 + 1 as libc::c_int >> 1 as libc::c_int) as pixel;
    *src
        .offset(
            (2 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh38;
    *src
        .offset(
            (3 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize,
        ) = (t1 + 2 as libc::c_int * t2 + t3 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    *src
        .offset(
            (3 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = (t2 + t3 + 1 as libc::c_int >> 1 as libc::c_int) as pixel;
}
unsafe extern "C" fn predict_4x4_hd_c(mut src: *mut pixel) {
    let mut lt: libc::c_int = *src
        .offset((-(1 as libc::c_int) + -(1 as libc::c_int) * 32 as libc::c_int) as isize)
        as libc::c_int;
    let mut l0: libc::c_int = *src
        .offset((-(1 as libc::c_int) + 0 as libc::c_int * 32 as libc::c_int) as isize)
        as libc::c_int;
    let mut l1: libc::c_int = *src
        .offset((-(1 as libc::c_int) + 1 as libc::c_int * 32 as libc::c_int) as isize)
        as libc::c_int;
    let mut l2: libc::c_int = *src
        .offset((-(1 as libc::c_int) + 2 as libc::c_int * 32 as libc::c_int) as isize)
        as libc::c_int;
    let mut l3: libc::c_int = *src
        .offset((-(1 as libc::c_int) + 3 as libc::c_int * 32 as libc::c_int) as isize)
        as libc::c_int;
    let mut t0: libc::c_int = *src
        .offset((0 as libc::c_int + -(1 as libc::c_int) * 32 as libc::c_int) as isize)
        as libc::c_int;
    let mut t1: libc::c_int = *src
        .offset((1 as libc::c_int + -(1 as libc::c_int) * 32 as libc::c_int) as isize)
        as libc::c_int;
    let mut t2: libc::c_int = *src
        .offset((2 as libc::c_int + -(1 as libc::c_int) * 32 as libc::c_int) as isize)
        as libc::c_int;
    let mut t3: libc::c_int = *src
        .offset((3 as libc::c_int + -(1 as libc::c_int) * 32 as libc::c_int) as isize)
        as libc::c_int;
    *src
        .offset(
            (0 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize,
        ) = (l2 + l3 + 1 as libc::c_int >> 1 as libc::c_int) as pixel;
    *src
        .offset(
            (1 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize,
        ) = (l1 + 2 as libc::c_int * l2 + l3 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    let ref mut fresh39 = *src
        .offset((2 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh39 = (l1 + l2 + 1 as libc::c_int >> 1 as libc::c_int) as pixel;
    *src
        .offset(
            (0 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh39;
    let ref mut fresh40 = *src
        .offset((3 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh40 = (l0 + 2 as libc::c_int * l1 + l2 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    *src
        .offset(
            (1 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh40;
    let ref mut fresh41 = *src
        .offset((2 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh41 = (l0 + l1 + 1 as libc::c_int >> 1 as libc::c_int) as pixel;
    *src
        .offset(
            (0 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh41;
    let ref mut fresh42 = *src
        .offset((3 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh42 = (lt + 2 as libc::c_int * l0 + l1 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    *src
        .offset(
            (1 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh42;
    let ref mut fresh43 = *src
        .offset((2 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh43 = (lt + l0 + 1 as libc::c_int >> 1 as libc::c_int) as pixel;
    *src
        .offset(
            (0 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh43;
    let ref mut fresh44 = *src
        .offset((3 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh44 = (t0 + 2 as libc::c_int * lt + l0 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    *src
        .offset(
            (1 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh44;
    *src
        .offset(
            (2 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = (t1 + 2 as libc::c_int * t0 + lt + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    *src
        .offset(
            (3 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = (t2 + 2 as libc::c_int * t1 + t0 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
}
unsafe extern "C" fn predict_4x4_vl_c(mut src: *mut pixel) {
    let mut t0: libc::c_int = *src
        .offset((0 as libc::c_int + -(1 as libc::c_int) * 32 as libc::c_int) as isize)
        as libc::c_int;
    let mut t1: libc::c_int = *src
        .offset((1 as libc::c_int + -(1 as libc::c_int) * 32 as libc::c_int) as isize)
        as libc::c_int;
    let mut t2: libc::c_int = *src
        .offset((2 as libc::c_int + -(1 as libc::c_int) * 32 as libc::c_int) as isize)
        as libc::c_int;
    let mut t3: libc::c_int = *src
        .offset((3 as libc::c_int + -(1 as libc::c_int) * 32 as libc::c_int) as isize)
        as libc::c_int;
    let mut t4: libc::c_int = *src
        .offset((4 as libc::c_int + -(1 as libc::c_int) * 32 as libc::c_int) as isize)
        as libc::c_int;
    let mut t5: libc::c_int = *src
        .offset((5 as libc::c_int + -(1 as libc::c_int) * 32 as libc::c_int) as isize)
        as libc::c_int;
    let mut t6: libc::c_int = *src
        .offset((6 as libc::c_int + -(1 as libc::c_int) * 32 as libc::c_int) as isize)
        as libc::c_int;
    let mut t7: libc::c_int = *src
        .offset((7 as libc::c_int + -(1 as libc::c_int) * 32 as libc::c_int) as isize)
        as libc::c_int;
    *src
        .offset(
            (0 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = (t0 + t1 + 1 as libc::c_int >> 1 as libc::c_int) as pixel;
    *src
        .offset(
            (0 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize,
        ) = (t0 + 2 as libc::c_int * t1 + t2 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    let ref mut fresh45 = *src
        .offset((0 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh45 = (t1 + t2 + 1 as libc::c_int >> 1 as libc::c_int) as pixel;
    *src
        .offset(
            (1 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh45;
    let ref mut fresh46 = *src
        .offset((0 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh46 = (t1 + 2 as libc::c_int * t2 + t3 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    *src
        .offset(
            (1 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh46;
    let ref mut fresh47 = *src
        .offset((1 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh47 = (t2 + t3 + 1 as libc::c_int >> 1 as libc::c_int) as pixel;
    *src
        .offset(
            (2 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh47;
    let ref mut fresh48 = *src
        .offset((1 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh48 = (t2 + 2 as libc::c_int * t3 + t4 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    *src
        .offset(
            (2 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh48;
    let ref mut fresh49 = *src
        .offset((2 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh49 = (t3 + t4 + 1 as libc::c_int >> 1 as libc::c_int) as pixel;
    *src
        .offset(
            (3 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh49;
    let ref mut fresh50 = *src
        .offset((2 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh50 = (t3 + 2 as libc::c_int * t4 + t5 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    *src
        .offset(
            (3 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh50;
    *src
        .offset(
            (3 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize,
        ) = (t4 + t5 + 1 as libc::c_int >> 1 as libc::c_int) as pixel;
    *src
        .offset(
            (3 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize,
        ) = (t4 + 2 as libc::c_int * t5 + t6 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
}
unsafe extern "C" fn predict_4x4_hu_c(mut src: *mut pixel) {
    let mut l0: libc::c_int = *src
        .offset((-(1 as libc::c_int) + 0 as libc::c_int * 32 as libc::c_int) as isize)
        as libc::c_int;
    let mut l1: libc::c_int = *src
        .offset((-(1 as libc::c_int) + 1 as libc::c_int * 32 as libc::c_int) as isize)
        as libc::c_int;
    let mut l2: libc::c_int = *src
        .offset((-(1 as libc::c_int) + 2 as libc::c_int * 32 as libc::c_int) as isize)
        as libc::c_int;
    let mut l3: libc::c_int = *src
        .offset((-(1 as libc::c_int) + 3 as libc::c_int * 32 as libc::c_int) as isize)
        as libc::c_int;
    *src
        .offset(
            (0 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = (l0 + l1 + 1 as libc::c_int >> 1 as libc::c_int) as pixel;
    *src
        .offset(
            (1 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = (l0 + 2 as libc::c_int * l1 + l2 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    let ref mut fresh51 = *src
        .offset((0 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh51 = (l1 + l2 + 1 as libc::c_int >> 1 as libc::c_int) as pixel;
    *src
        .offset(
            (2 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh51;
    let ref mut fresh52 = *src
        .offset((1 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh52 = (l1 + 2 as libc::c_int * l2 + l3 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    *src
        .offset(
            (3 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh52;
    let ref mut fresh53 = *src
        .offset((0 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh53 = (l2 + l3 + 1 as libc::c_int >> 1 as libc::c_int) as pixel;
    *src
        .offset(
            (2 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh53;
    let ref mut fresh54 = *src
        .offset((1 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh54 = (l2 + 2 as libc::c_int * l3 + l3 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    *src
        .offset(
            (3 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh54;
    let ref mut fresh55 = *src
        .offset((3 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh55 = l3 as pixel;
    let ref mut fresh56 = *src
        .offset((2 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh56 = *fresh55;
    let ref mut fresh57 = *src
        .offset((2 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh57 = *fresh56;
    let ref mut fresh58 = *src
        .offset((0 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh58 = *fresh57;
    let ref mut fresh59 = *src
        .offset((1 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh59 = *fresh58;
    *src
        .offset(
            (3 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh59;
}
unsafe extern "C" fn predict_8x8_filter_c(
    mut src: *mut pixel,
    mut edge: *mut pixel,
    mut i_neighbor: libc::c_int,
    mut i_filters: libc::c_int,
) {
    let mut have_lt: libc::c_int = i_neighbor & MB_TOPLEFT as libc::c_int;
    if i_filters & MB_LEFT as libc::c_int != 0 {
        *edge
            .offset(
                15 as libc::c_int as isize,
            ) = (*src
            .offset(
                (0 as libc::c_int + -(1 as libc::c_int) * 32 as libc::c_int) as isize,
            ) as libc::c_int
            + 2 as libc::c_int
                * *src
                    .offset(
                        (-(1 as libc::c_int) + -(1 as libc::c_int) * 32 as libc::c_int)
                            as isize,
                    ) as libc::c_int
            + *src
                .offset(
                    (-(1 as libc::c_int) + 0 as libc::c_int * 32 as libc::c_int) as isize,
                ) as libc::c_int + 2 as libc::c_int >> 2 as libc::c_int) as pixel;
        *edge
            .offset(
                14 as libc::c_int as isize,
            ) = ((if have_lt != 0 {
            *src
                .offset(
                    (-(1 as libc::c_int) + -(1 as libc::c_int) * 32 as libc::c_int)
                        as isize,
                ) as libc::c_int
        } else {
            *src
                .offset(
                    (-(1 as libc::c_int) + 0 as libc::c_int * 32 as libc::c_int) as isize,
                ) as libc::c_int
        })
            + 2 as libc::c_int
                * *src
                    .offset(
                        (-(1 as libc::c_int) + 0 as libc::c_int * 32 as libc::c_int)
                            as isize,
                    ) as libc::c_int
            + *src
                .offset(
                    (-(1 as libc::c_int) + 1 as libc::c_int * 32 as libc::c_int) as isize,
                ) as libc::c_int + 2 as libc::c_int >> 2 as libc::c_int) as pixel;
        *edge
            .offset(
                (14 as libc::c_int - 1 as libc::c_int) as isize,
            ) = (*src
            .offset(
                (-(1 as libc::c_int)
                    + (1 as libc::c_int - 1 as libc::c_int) * 32 as libc::c_int) as isize,
            ) as libc::c_int
            + 2 as libc::c_int
                * *src
                    .offset(
                        (-(1 as libc::c_int) + 1 as libc::c_int * 32 as libc::c_int)
                            as isize,
                    ) as libc::c_int
            + *src
                .offset(
                    (-(1 as libc::c_int)
                        + (1 as libc::c_int + 1 as libc::c_int) * 32 as libc::c_int)
                        as isize,
                ) as libc::c_int + 2 as libc::c_int >> 2 as libc::c_int) as pixel;
        *edge
            .offset(
                (14 as libc::c_int - 2 as libc::c_int) as isize,
            ) = (*src
            .offset(
                (-(1 as libc::c_int)
                    + (2 as libc::c_int - 1 as libc::c_int) * 32 as libc::c_int) as isize,
            ) as libc::c_int
            + 2 as libc::c_int
                * *src
                    .offset(
                        (-(1 as libc::c_int) + 2 as libc::c_int * 32 as libc::c_int)
                            as isize,
                    ) as libc::c_int
            + *src
                .offset(
                    (-(1 as libc::c_int)
                        + (2 as libc::c_int + 1 as libc::c_int) * 32 as libc::c_int)
                        as isize,
                ) as libc::c_int + 2 as libc::c_int >> 2 as libc::c_int) as pixel;
        *edge
            .offset(
                (14 as libc::c_int - 3 as libc::c_int) as isize,
            ) = (*src
            .offset(
                (-(1 as libc::c_int)
                    + (3 as libc::c_int - 1 as libc::c_int) * 32 as libc::c_int) as isize,
            ) as libc::c_int
            + 2 as libc::c_int
                * *src
                    .offset(
                        (-(1 as libc::c_int) + 3 as libc::c_int * 32 as libc::c_int)
                            as isize,
                    ) as libc::c_int
            + *src
                .offset(
                    (-(1 as libc::c_int)
                        + (3 as libc::c_int + 1 as libc::c_int) * 32 as libc::c_int)
                        as isize,
                ) as libc::c_int + 2 as libc::c_int >> 2 as libc::c_int) as pixel;
        *edge
            .offset(
                (14 as libc::c_int - 4 as libc::c_int) as isize,
            ) = (*src
            .offset(
                (-(1 as libc::c_int)
                    + (4 as libc::c_int - 1 as libc::c_int) * 32 as libc::c_int) as isize,
            ) as libc::c_int
            + 2 as libc::c_int
                * *src
                    .offset(
                        (-(1 as libc::c_int) + 4 as libc::c_int * 32 as libc::c_int)
                            as isize,
                    ) as libc::c_int
            + *src
                .offset(
                    (-(1 as libc::c_int)
                        + (4 as libc::c_int + 1 as libc::c_int) * 32 as libc::c_int)
                        as isize,
                ) as libc::c_int + 2 as libc::c_int >> 2 as libc::c_int) as pixel;
        *edge
            .offset(
                (14 as libc::c_int - 5 as libc::c_int) as isize,
            ) = (*src
            .offset(
                (-(1 as libc::c_int)
                    + (5 as libc::c_int - 1 as libc::c_int) * 32 as libc::c_int) as isize,
            ) as libc::c_int
            + 2 as libc::c_int
                * *src
                    .offset(
                        (-(1 as libc::c_int) + 5 as libc::c_int * 32 as libc::c_int)
                            as isize,
                    ) as libc::c_int
            + *src
                .offset(
                    (-(1 as libc::c_int)
                        + (5 as libc::c_int + 1 as libc::c_int) * 32 as libc::c_int)
                        as isize,
                ) as libc::c_int + 2 as libc::c_int >> 2 as libc::c_int) as pixel;
        *edge
            .offset(
                (14 as libc::c_int - 6 as libc::c_int) as isize,
            ) = (*src
            .offset(
                (-(1 as libc::c_int)
                    + (6 as libc::c_int - 1 as libc::c_int) * 32 as libc::c_int) as isize,
            ) as libc::c_int
            + 2 as libc::c_int
                * *src
                    .offset(
                        (-(1 as libc::c_int) + 6 as libc::c_int * 32 as libc::c_int)
                            as isize,
                    ) as libc::c_int
            + *src
                .offset(
                    (-(1 as libc::c_int)
                        + (6 as libc::c_int + 1 as libc::c_int) * 32 as libc::c_int)
                        as isize,
                ) as libc::c_int + 2 as libc::c_int >> 2 as libc::c_int) as pixel;
        let ref mut fresh60 = *edge.offset(7 as libc::c_int as isize);
        *fresh60 = (*src
            .offset(
                (-(1 as libc::c_int) + 6 as libc::c_int * 32 as libc::c_int) as isize,
            ) as libc::c_int
            + 3 as libc::c_int
                * *src
                    .offset(
                        (-(1 as libc::c_int) + 7 as libc::c_int * 32 as libc::c_int)
                            as isize,
                    ) as libc::c_int + 2 as libc::c_int >> 2 as libc::c_int) as pixel;
        *edge.offset(6 as libc::c_int as isize) = *fresh60;
    }
    if i_filters & MB_TOP as libc::c_int != 0 {
        let mut have_tr: libc::c_int = i_neighbor & MB_TOPRIGHT as libc::c_int;
        *edge
            .offset(
                16 as libc::c_int as isize,
            ) = ((if have_lt != 0 {
            *src
                .offset(
                    (-(1 as libc::c_int) + -(1 as libc::c_int) * 32 as libc::c_int)
                        as isize,
                ) as libc::c_int
        } else {
            *src
                .offset(
                    (0 as libc::c_int + -(1 as libc::c_int) * 32 as libc::c_int) as isize,
                ) as libc::c_int
        })
            + 2 as libc::c_int
                * *src
                    .offset(
                        (0 as libc::c_int + -(1 as libc::c_int) * 32 as libc::c_int)
                            as isize,
                    ) as libc::c_int
            + *src
                .offset(
                    (1 as libc::c_int + -(1 as libc::c_int) * 32 as libc::c_int) as isize,
                ) as libc::c_int + 2 as libc::c_int >> 2 as libc::c_int) as pixel;
        *edge
            .offset(
                (16 as libc::c_int + 1 as libc::c_int) as isize,
            ) = (*src
            .offset(
                (1 as libc::c_int - 1 as libc::c_int
                    + -(1 as libc::c_int) * 32 as libc::c_int) as isize,
            ) as libc::c_int
            + 2 as libc::c_int
                * *src
                    .offset(
                        (1 as libc::c_int + -(1 as libc::c_int) * 32 as libc::c_int)
                            as isize,
                    ) as libc::c_int
            + *src
                .offset(
                    (1 as libc::c_int + 1 as libc::c_int
                        + -(1 as libc::c_int) * 32 as libc::c_int) as isize,
                ) as libc::c_int + 2 as libc::c_int >> 2 as libc::c_int) as pixel;
        *edge
            .offset(
                (16 as libc::c_int + 2 as libc::c_int) as isize,
            ) = (*src
            .offset(
                (2 as libc::c_int - 1 as libc::c_int
                    + -(1 as libc::c_int) * 32 as libc::c_int) as isize,
            ) as libc::c_int
            + 2 as libc::c_int
                * *src
                    .offset(
                        (2 as libc::c_int + -(1 as libc::c_int) * 32 as libc::c_int)
                            as isize,
                    ) as libc::c_int
            + *src
                .offset(
                    (2 as libc::c_int + 1 as libc::c_int
                        + -(1 as libc::c_int) * 32 as libc::c_int) as isize,
                ) as libc::c_int + 2 as libc::c_int >> 2 as libc::c_int) as pixel;
        *edge
            .offset(
                (16 as libc::c_int + 3 as libc::c_int) as isize,
            ) = (*src
            .offset(
                (3 as libc::c_int - 1 as libc::c_int
                    + -(1 as libc::c_int) * 32 as libc::c_int) as isize,
            ) as libc::c_int
            + 2 as libc::c_int
                * *src
                    .offset(
                        (3 as libc::c_int + -(1 as libc::c_int) * 32 as libc::c_int)
                            as isize,
                    ) as libc::c_int
            + *src
                .offset(
                    (3 as libc::c_int + 1 as libc::c_int
                        + -(1 as libc::c_int) * 32 as libc::c_int) as isize,
                ) as libc::c_int + 2 as libc::c_int >> 2 as libc::c_int) as pixel;
        *edge
            .offset(
                (16 as libc::c_int + 4 as libc::c_int) as isize,
            ) = (*src
            .offset(
                (4 as libc::c_int - 1 as libc::c_int
                    + -(1 as libc::c_int) * 32 as libc::c_int) as isize,
            ) as libc::c_int
            + 2 as libc::c_int
                * *src
                    .offset(
                        (4 as libc::c_int + -(1 as libc::c_int) * 32 as libc::c_int)
                            as isize,
                    ) as libc::c_int
            + *src
                .offset(
                    (4 as libc::c_int + 1 as libc::c_int
                        + -(1 as libc::c_int) * 32 as libc::c_int) as isize,
                ) as libc::c_int + 2 as libc::c_int >> 2 as libc::c_int) as pixel;
        *edge
            .offset(
                (16 as libc::c_int + 5 as libc::c_int) as isize,
            ) = (*src
            .offset(
                (5 as libc::c_int - 1 as libc::c_int
                    + -(1 as libc::c_int) * 32 as libc::c_int) as isize,
            ) as libc::c_int
            + 2 as libc::c_int
                * *src
                    .offset(
                        (5 as libc::c_int + -(1 as libc::c_int) * 32 as libc::c_int)
                            as isize,
                    ) as libc::c_int
            + *src
                .offset(
                    (5 as libc::c_int + 1 as libc::c_int
                        + -(1 as libc::c_int) * 32 as libc::c_int) as isize,
                ) as libc::c_int + 2 as libc::c_int >> 2 as libc::c_int) as pixel;
        *edge
            .offset(
                (16 as libc::c_int + 6 as libc::c_int) as isize,
            ) = (*src
            .offset(
                (6 as libc::c_int - 1 as libc::c_int
                    + -(1 as libc::c_int) * 32 as libc::c_int) as isize,
            ) as libc::c_int
            + 2 as libc::c_int
                * *src
                    .offset(
                        (6 as libc::c_int + -(1 as libc::c_int) * 32 as libc::c_int)
                            as isize,
                    ) as libc::c_int
            + *src
                .offset(
                    (6 as libc::c_int + 1 as libc::c_int
                        + -(1 as libc::c_int) * 32 as libc::c_int) as isize,
                ) as libc::c_int + 2 as libc::c_int >> 2 as libc::c_int) as pixel;
        *edge
            .offset(
                23 as libc::c_int as isize,
            ) = (*src
            .offset(
                (6 as libc::c_int + -(1 as libc::c_int) * 32 as libc::c_int) as isize,
            ) as libc::c_int
            + 2 as libc::c_int
                * *src
                    .offset(
                        (7 as libc::c_int + -(1 as libc::c_int) * 32 as libc::c_int)
                            as isize,
                    ) as libc::c_int
            + (if have_tr != 0 {
                *src
                    .offset(
                        (8 as libc::c_int + -(1 as libc::c_int) * 32 as libc::c_int)
                            as isize,
                    ) as libc::c_int
            } else {
                *src
                    .offset(
                        (7 as libc::c_int + -(1 as libc::c_int) * 32 as libc::c_int)
                            as isize,
                    ) as libc::c_int
            }) + 2 as libc::c_int >> 2 as libc::c_int) as pixel;
        if i_filters & MB_TOPRIGHT as libc::c_int != 0 {
            if have_tr != 0 {
                *edge
                    .offset(
                        (16 as libc::c_int + 8 as libc::c_int) as isize,
                    ) = (*src
                    .offset(
                        (8 as libc::c_int - 1 as libc::c_int
                            + -(1 as libc::c_int) * 32 as libc::c_int) as isize,
                    ) as libc::c_int
                    + 2 as libc::c_int
                        * *src
                            .offset(
                                (8 as libc::c_int + -(1 as libc::c_int) * 32 as libc::c_int)
                                    as isize,
                            ) as libc::c_int
                    + *src
                        .offset(
                            (8 as libc::c_int + 1 as libc::c_int
                                + -(1 as libc::c_int) * 32 as libc::c_int) as isize,
                        ) as libc::c_int + 2 as libc::c_int >> 2 as libc::c_int)
                    as pixel;
                *edge
                    .offset(
                        (16 as libc::c_int + 9 as libc::c_int) as isize,
                    ) = (*src
                    .offset(
                        (9 as libc::c_int - 1 as libc::c_int
                            + -(1 as libc::c_int) * 32 as libc::c_int) as isize,
                    ) as libc::c_int
                    + 2 as libc::c_int
                        * *src
                            .offset(
                                (9 as libc::c_int + -(1 as libc::c_int) * 32 as libc::c_int)
                                    as isize,
                            ) as libc::c_int
                    + *src
                        .offset(
                            (9 as libc::c_int + 1 as libc::c_int
                                + -(1 as libc::c_int) * 32 as libc::c_int) as isize,
                        ) as libc::c_int + 2 as libc::c_int >> 2 as libc::c_int)
                    as pixel;
                *edge
                    .offset(
                        (16 as libc::c_int + 10 as libc::c_int) as isize,
                    ) = (*src
                    .offset(
                        (10 as libc::c_int - 1 as libc::c_int
                            + -(1 as libc::c_int) * 32 as libc::c_int) as isize,
                    ) as libc::c_int
                    + 2 as libc::c_int
                        * *src
                            .offset(
                                (10 as libc::c_int
                                    + -(1 as libc::c_int) * 32 as libc::c_int) as isize,
                            ) as libc::c_int
                    + *src
                        .offset(
                            (10 as libc::c_int + 1 as libc::c_int
                                + -(1 as libc::c_int) * 32 as libc::c_int) as isize,
                        ) as libc::c_int + 2 as libc::c_int >> 2 as libc::c_int)
                    as pixel;
                *edge
                    .offset(
                        (16 as libc::c_int + 11 as libc::c_int) as isize,
                    ) = (*src
                    .offset(
                        (11 as libc::c_int - 1 as libc::c_int
                            + -(1 as libc::c_int) * 32 as libc::c_int) as isize,
                    ) as libc::c_int
                    + 2 as libc::c_int
                        * *src
                            .offset(
                                (11 as libc::c_int
                                    + -(1 as libc::c_int) * 32 as libc::c_int) as isize,
                            ) as libc::c_int
                    + *src
                        .offset(
                            (11 as libc::c_int + 1 as libc::c_int
                                + -(1 as libc::c_int) * 32 as libc::c_int) as isize,
                        ) as libc::c_int + 2 as libc::c_int >> 2 as libc::c_int)
                    as pixel;
                *edge
                    .offset(
                        (16 as libc::c_int + 12 as libc::c_int) as isize,
                    ) = (*src
                    .offset(
                        (12 as libc::c_int - 1 as libc::c_int
                            + -(1 as libc::c_int) * 32 as libc::c_int) as isize,
                    ) as libc::c_int
                    + 2 as libc::c_int
                        * *src
                            .offset(
                                (12 as libc::c_int
                                    + -(1 as libc::c_int) * 32 as libc::c_int) as isize,
                            ) as libc::c_int
                    + *src
                        .offset(
                            (12 as libc::c_int + 1 as libc::c_int
                                + -(1 as libc::c_int) * 32 as libc::c_int) as isize,
                        ) as libc::c_int + 2 as libc::c_int >> 2 as libc::c_int)
                    as pixel;
                *edge
                    .offset(
                        (16 as libc::c_int + 13 as libc::c_int) as isize,
                    ) = (*src
                    .offset(
                        (13 as libc::c_int - 1 as libc::c_int
                            + -(1 as libc::c_int) * 32 as libc::c_int) as isize,
                    ) as libc::c_int
                    + 2 as libc::c_int
                        * *src
                            .offset(
                                (13 as libc::c_int
                                    + -(1 as libc::c_int) * 32 as libc::c_int) as isize,
                            ) as libc::c_int
                    + *src
                        .offset(
                            (13 as libc::c_int + 1 as libc::c_int
                                + -(1 as libc::c_int) * 32 as libc::c_int) as isize,
                        ) as libc::c_int + 2 as libc::c_int >> 2 as libc::c_int)
                    as pixel;
                *edge
                    .offset(
                        (16 as libc::c_int + 14 as libc::c_int) as isize,
                    ) = (*src
                    .offset(
                        (14 as libc::c_int - 1 as libc::c_int
                            + -(1 as libc::c_int) * 32 as libc::c_int) as isize,
                    ) as libc::c_int
                    + 2 as libc::c_int
                        * *src
                            .offset(
                                (14 as libc::c_int
                                    + -(1 as libc::c_int) * 32 as libc::c_int) as isize,
                            ) as libc::c_int
                    + *src
                        .offset(
                            (14 as libc::c_int + 1 as libc::c_int
                                + -(1 as libc::c_int) * 32 as libc::c_int) as isize,
                        ) as libc::c_int + 2 as libc::c_int >> 2 as libc::c_int)
                    as pixel;
                let ref mut fresh61 = *edge.offset(32 as libc::c_int as isize);
                *fresh61 = (*src
                    .offset(
                        (14 as libc::c_int + -(1 as libc::c_int) * 32 as libc::c_int)
                            as isize,
                    ) as libc::c_int
                    + 3 as libc::c_int
                        * *src
                            .offset(
                                (15 as libc::c_int
                                    + -(1 as libc::c_int) * 32 as libc::c_int) as isize,
                            ) as libc::c_int + 2 as libc::c_int >> 2 as libc::c_int)
                    as pixel;
                *edge.offset(31 as libc::c_int as isize) = *fresh61;
            } else {
                (*(edge.offset(24 as libc::c_int as isize) as *mut x264_union32_t))
                    .i = (*src
                    .offset(
                        (7 as libc::c_int + -(1 as libc::c_int) * 32 as libc::c_int)
                            as isize,
                    ) as libc::c_uint)
                    .wrapping_mul(0x1010101 as libc::c_uint);
                (*(edge.offset(28 as libc::c_int as isize) as *mut x264_union32_t))
                    .i = (*src
                    .offset(
                        (7 as libc::c_int + -(1 as libc::c_int) * 32 as libc::c_int)
                            as isize,
                    ) as libc::c_uint)
                    .wrapping_mul(0x1010101 as libc::c_uint);
                *edge
                    .offset(
                        32 as libc::c_int as isize,
                    ) = *src
                    .offset(
                        (7 as libc::c_int + -(1 as libc::c_int) * 32 as libc::c_int)
                            as isize,
                    );
            }
        }
    }
}
unsafe extern "C" fn predict_8x8_dc_128_c(mut src: *mut pixel, mut edge: *mut pixel) {
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < 8 as libc::c_int {
        (*(src.offset(0 as libc::c_int as isize) as *mut x264_union32_t))
            .i = (((1 as libc::c_int) << 8 as libc::c_int - 1 as libc::c_int)
            as libc::c_uint)
            .wrapping_mul(0x1010101 as libc::c_uint);
        (*(src.offset(4 as libc::c_int as isize) as *mut x264_union32_t))
            .i = (((1 as libc::c_int) << 8 as libc::c_int - 1 as libc::c_int)
            as libc::c_uint)
            .wrapping_mul(0x1010101 as libc::c_uint);
        src = src.offset(32 as libc::c_int as isize);
        y += 1;
        y;
    }
}
unsafe extern "C" fn predict_8x8_dc_left_c(mut src: *mut pixel, mut edge: *mut pixel) {
    let mut l0: libc::c_int = *edge
        .offset((14 as libc::c_int - 0 as libc::c_int) as isize) as libc::c_int;
    let mut l1: libc::c_int = *edge
        .offset((14 as libc::c_int - 1 as libc::c_int) as isize) as libc::c_int;
    let mut l2: libc::c_int = *edge
        .offset((14 as libc::c_int - 2 as libc::c_int) as isize) as libc::c_int;
    let mut l3: libc::c_int = *edge
        .offset((14 as libc::c_int - 3 as libc::c_int) as isize) as libc::c_int;
    let mut l4: libc::c_int = *edge
        .offset((14 as libc::c_int - 4 as libc::c_int) as isize) as libc::c_int;
    let mut l5: libc::c_int = *edge
        .offset((14 as libc::c_int - 5 as libc::c_int) as isize) as libc::c_int;
    let mut l6: libc::c_int = *edge
        .offset((14 as libc::c_int - 6 as libc::c_int) as isize) as libc::c_int;
    let mut l7: libc::c_int = *edge
        .offset((14 as libc::c_int - 7 as libc::c_int) as isize) as libc::c_int;
    let mut dc: pixel4 = ((l0 + l1 + l2 + l3 + l4 + l5 + l6 + l7 + 4 as libc::c_int
        >> 3 as libc::c_int) as libc::c_uint)
        .wrapping_mul(0x1010101 as libc::c_uint);
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < 8 as libc::c_int {
        (*(src.offset(0 as libc::c_int as isize) as *mut x264_union32_t)).i = dc;
        (*(src.offset(4 as libc::c_int as isize) as *mut x264_union32_t)).i = dc;
        src = src.offset(32 as libc::c_int as isize);
        y += 1;
        y;
    }
}
unsafe extern "C" fn predict_8x8_dc_top_c(mut src: *mut pixel, mut edge: *mut pixel) {
    let mut t0: libc::c_int = *edge
        .offset((16 as libc::c_int + 0 as libc::c_int) as isize) as libc::c_int;
    let mut t1: libc::c_int = *edge
        .offset((16 as libc::c_int + 1 as libc::c_int) as isize) as libc::c_int;
    let mut t2: libc::c_int = *edge
        .offset((16 as libc::c_int + 2 as libc::c_int) as isize) as libc::c_int;
    let mut t3: libc::c_int = *edge
        .offset((16 as libc::c_int + 3 as libc::c_int) as isize) as libc::c_int;
    let mut t4: libc::c_int = *edge
        .offset((16 as libc::c_int + 4 as libc::c_int) as isize) as libc::c_int;
    let mut t5: libc::c_int = *edge
        .offset((16 as libc::c_int + 5 as libc::c_int) as isize) as libc::c_int;
    let mut t6: libc::c_int = *edge
        .offset((16 as libc::c_int + 6 as libc::c_int) as isize) as libc::c_int;
    let mut t7: libc::c_int = *edge
        .offset((16 as libc::c_int + 7 as libc::c_int) as isize) as libc::c_int;
    let mut dc: pixel4 = ((t0 + t1 + t2 + t3 + t4 + t5 + t6 + t7 + 4 as libc::c_int
        >> 3 as libc::c_int) as libc::c_uint)
        .wrapping_mul(0x1010101 as libc::c_uint);
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < 8 as libc::c_int {
        (*(src.offset(0 as libc::c_int as isize) as *mut x264_union32_t)).i = dc;
        (*(src.offset(4 as libc::c_int as isize) as *mut x264_union32_t)).i = dc;
        src = src.offset(32 as libc::c_int as isize);
        y += 1;
        y;
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_predict_8x8_dc_c(
    mut src: *mut pixel,
    mut edge: *mut pixel,
) {
    let mut l0: libc::c_int = *edge
        .offset((14 as libc::c_int - 0 as libc::c_int) as isize) as libc::c_int;
    let mut l1: libc::c_int = *edge
        .offset((14 as libc::c_int - 1 as libc::c_int) as isize) as libc::c_int;
    let mut l2: libc::c_int = *edge
        .offset((14 as libc::c_int - 2 as libc::c_int) as isize) as libc::c_int;
    let mut l3: libc::c_int = *edge
        .offset((14 as libc::c_int - 3 as libc::c_int) as isize) as libc::c_int;
    let mut l4: libc::c_int = *edge
        .offset((14 as libc::c_int - 4 as libc::c_int) as isize) as libc::c_int;
    let mut l5: libc::c_int = *edge
        .offset((14 as libc::c_int - 5 as libc::c_int) as isize) as libc::c_int;
    let mut l6: libc::c_int = *edge
        .offset((14 as libc::c_int - 6 as libc::c_int) as isize) as libc::c_int;
    let mut l7: libc::c_int = *edge
        .offset((14 as libc::c_int - 7 as libc::c_int) as isize) as libc::c_int;
    let mut t0: libc::c_int = *edge
        .offset((16 as libc::c_int + 0 as libc::c_int) as isize) as libc::c_int;
    let mut t1: libc::c_int = *edge
        .offset((16 as libc::c_int + 1 as libc::c_int) as isize) as libc::c_int;
    let mut t2: libc::c_int = *edge
        .offset((16 as libc::c_int + 2 as libc::c_int) as isize) as libc::c_int;
    let mut t3: libc::c_int = *edge
        .offset((16 as libc::c_int + 3 as libc::c_int) as isize) as libc::c_int;
    let mut t4: libc::c_int = *edge
        .offset((16 as libc::c_int + 4 as libc::c_int) as isize) as libc::c_int;
    let mut t5: libc::c_int = *edge
        .offset((16 as libc::c_int + 5 as libc::c_int) as isize) as libc::c_int;
    let mut t6: libc::c_int = *edge
        .offset((16 as libc::c_int + 6 as libc::c_int) as isize) as libc::c_int;
    let mut t7: libc::c_int = *edge
        .offset((16 as libc::c_int + 7 as libc::c_int) as isize) as libc::c_int;
    let mut dc: pixel4 = ((l0 + l1 + l2 + l3 + l4 + l5 + l6 + l7 + t0 + t1 + t2 + t3 + t4
        + t5 + t6 + t7 + 8 as libc::c_int >> 4 as libc::c_int) as libc::c_uint)
        .wrapping_mul(0x1010101 as libc::c_uint);
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < 8 as libc::c_int {
        (*(src.offset(0 as libc::c_int as isize) as *mut x264_union32_t)).i = dc;
        (*(src.offset(4 as libc::c_int as isize) as *mut x264_union32_t)).i = dc;
        src = src.offset(32 as libc::c_int as isize);
        y += 1;
        y;
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_predict_8x8_h_c(
    mut src: *mut pixel,
    mut edge: *mut pixel,
) {
    let mut l0: libc::c_int = *edge
        .offset((14 as libc::c_int - 0 as libc::c_int) as isize) as libc::c_int;
    let mut l1: libc::c_int = *edge
        .offset((14 as libc::c_int - 1 as libc::c_int) as isize) as libc::c_int;
    let mut l2: libc::c_int = *edge
        .offset((14 as libc::c_int - 2 as libc::c_int) as isize) as libc::c_int;
    let mut l3: libc::c_int = *edge
        .offset((14 as libc::c_int - 3 as libc::c_int) as isize) as libc::c_int;
    let mut l4: libc::c_int = *edge
        .offset((14 as libc::c_int - 4 as libc::c_int) as isize) as libc::c_int;
    let mut l5: libc::c_int = *edge
        .offset((14 as libc::c_int - 5 as libc::c_int) as isize) as libc::c_int;
    let mut l6: libc::c_int = *edge
        .offset((14 as libc::c_int - 6 as libc::c_int) as isize) as libc::c_int;
    let mut l7: libc::c_int = *edge
        .offset((14 as libc::c_int - 7 as libc::c_int) as isize) as libc::c_int;
    let ref mut fresh62 = (*(src
        .offset((0 as libc::c_int * 32 as libc::c_int) as isize)
        .offset(4 as libc::c_int as isize) as *mut x264_union32_t))
        .i;
    *fresh62 = (l0 as libc::c_uint).wrapping_mul(0x1010101 as libc::c_uint);
    (*(src
        .offset((0 as libc::c_int * 32 as libc::c_int) as isize)
        .offset(0 as libc::c_int as isize) as *mut x264_union32_t))
        .i = *fresh62;
    let ref mut fresh63 = (*(src
        .offset((1 as libc::c_int * 32 as libc::c_int) as isize)
        .offset(4 as libc::c_int as isize) as *mut x264_union32_t))
        .i;
    *fresh63 = (l1 as libc::c_uint).wrapping_mul(0x1010101 as libc::c_uint);
    (*(src
        .offset((1 as libc::c_int * 32 as libc::c_int) as isize)
        .offset(0 as libc::c_int as isize) as *mut x264_union32_t))
        .i = *fresh63;
    let ref mut fresh64 = (*(src
        .offset((2 as libc::c_int * 32 as libc::c_int) as isize)
        .offset(4 as libc::c_int as isize) as *mut x264_union32_t))
        .i;
    *fresh64 = (l2 as libc::c_uint).wrapping_mul(0x1010101 as libc::c_uint);
    (*(src
        .offset((2 as libc::c_int * 32 as libc::c_int) as isize)
        .offset(0 as libc::c_int as isize) as *mut x264_union32_t))
        .i = *fresh64;
    let ref mut fresh65 = (*(src
        .offset((3 as libc::c_int * 32 as libc::c_int) as isize)
        .offset(4 as libc::c_int as isize) as *mut x264_union32_t))
        .i;
    *fresh65 = (l3 as libc::c_uint).wrapping_mul(0x1010101 as libc::c_uint);
    (*(src
        .offset((3 as libc::c_int * 32 as libc::c_int) as isize)
        .offset(0 as libc::c_int as isize) as *mut x264_union32_t))
        .i = *fresh65;
    let ref mut fresh66 = (*(src
        .offset((4 as libc::c_int * 32 as libc::c_int) as isize)
        .offset(4 as libc::c_int as isize) as *mut x264_union32_t))
        .i;
    *fresh66 = (l4 as libc::c_uint).wrapping_mul(0x1010101 as libc::c_uint);
    (*(src
        .offset((4 as libc::c_int * 32 as libc::c_int) as isize)
        .offset(0 as libc::c_int as isize) as *mut x264_union32_t))
        .i = *fresh66;
    let ref mut fresh67 = (*(src
        .offset((5 as libc::c_int * 32 as libc::c_int) as isize)
        .offset(4 as libc::c_int as isize) as *mut x264_union32_t))
        .i;
    *fresh67 = (l5 as libc::c_uint).wrapping_mul(0x1010101 as libc::c_uint);
    (*(src
        .offset((5 as libc::c_int * 32 as libc::c_int) as isize)
        .offset(0 as libc::c_int as isize) as *mut x264_union32_t))
        .i = *fresh67;
    let ref mut fresh68 = (*(src
        .offset((6 as libc::c_int * 32 as libc::c_int) as isize)
        .offset(4 as libc::c_int as isize) as *mut x264_union32_t))
        .i;
    *fresh68 = (l6 as libc::c_uint).wrapping_mul(0x1010101 as libc::c_uint);
    (*(src
        .offset((6 as libc::c_int * 32 as libc::c_int) as isize)
        .offset(0 as libc::c_int as isize) as *mut x264_union32_t))
        .i = *fresh68;
    let ref mut fresh69 = (*(src
        .offset((7 as libc::c_int * 32 as libc::c_int) as isize)
        .offset(4 as libc::c_int as isize) as *mut x264_union32_t))
        .i;
    *fresh69 = (l7 as libc::c_uint).wrapping_mul(0x1010101 as libc::c_uint);
    (*(src
        .offset((7 as libc::c_int * 32 as libc::c_int) as isize)
        .offset(0 as libc::c_int as isize) as *mut x264_union32_t))
        .i = *fresh69;
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_predict_8x8_v_c(
    mut src: *mut pixel,
    mut edge: *mut pixel,
) {
    let mut top: [pixel4; 2] = [
        (*(edge.offset(16 as libc::c_int as isize) as *mut x264_union32_t)).i,
        (*(edge.offset(20 as libc::c_int as isize) as *mut x264_union32_t)).i,
    ];
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < 8 as libc::c_int {
        (*(src.offset((y * 32 as libc::c_int) as isize).offset(0 as libc::c_int as isize)
            as *mut x264_union32_t))
            .i = top[0 as libc::c_int as usize];
        (*(src.offset((y * 32 as libc::c_int) as isize).offset(4 as libc::c_int as isize)
            as *mut x264_union32_t))
            .i = top[1 as libc::c_int as usize];
        y += 1;
        y;
    }
}
unsafe extern "C" fn predict_8x8_ddl_c(mut src: *mut pixel, mut edge: *mut pixel) {
    let mut t0: libc::c_int = *edge
        .offset((16 as libc::c_int + 0 as libc::c_int) as isize) as libc::c_int;
    let mut t1: libc::c_int = *edge
        .offset((16 as libc::c_int + 1 as libc::c_int) as isize) as libc::c_int;
    let mut t2: libc::c_int = *edge
        .offset((16 as libc::c_int + 2 as libc::c_int) as isize) as libc::c_int;
    let mut t3: libc::c_int = *edge
        .offset((16 as libc::c_int + 3 as libc::c_int) as isize) as libc::c_int;
    let mut t4: libc::c_int = *edge
        .offset((16 as libc::c_int + 4 as libc::c_int) as isize) as libc::c_int;
    let mut t5: libc::c_int = *edge
        .offset((16 as libc::c_int + 5 as libc::c_int) as isize) as libc::c_int;
    let mut t6: libc::c_int = *edge
        .offset((16 as libc::c_int + 6 as libc::c_int) as isize) as libc::c_int;
    let mut t7: libc::c_int = *edge
        .offset((16 as libc::c_int + 7 as libc::c_int) as isize) as libc::c_int;
    let mut t8: libc::c_int = *edge
        .offset((16 as libc::c_int + 8 as libc::c_int) as isize) as libc::c_int;
    let mut t9: libc::c_int = *edge
        .offset((16 as libc::c_int + 9 as libc::c_int) as isize) as libc::c_int;
    let mut t10: libc::c_int = *edge
        .offset((16 as libc::c_int + 10 as libc::c_int) as isize) as libc::c_int;
    let mut t11: libc::c_int = *edge
        .offset((16 as libc::c_int + 11 as libc::c_int) as isize) as libc::c_int;
    let mut t12: libc::c_int = *edge
        .offset((16 as libc::c_int + 12 as libc::c_int) as isize) as libc::c_int;
    let mut t13: libc::c_int = *edge
        .offset((16 as libc::c_int + 13 as libc::c_int) as isize) as libc::c_int;
    let mut t14: libc::c_int = *edge
        .offset((16 as libc::c_int + 14 as libc::c_int) as isize) as libc::c_int;
    let mut t15: libc::c_int = *edge
        .offset((16 as libc::c_int + 15 as libc::c_int) as isize) as libc::c_int;
    *src
        .offset(
            (0 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = (t0 + 2 as libc::c_int * t1 + t2 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    let ref mut fresh70 = *src
        .offset((1 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh70 = (t1 + 2 as libc::c_int * t2 + t3 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    *src
        .offset(
            (0 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh70;
    let ref mut fresh71 = *src
        .offset((2 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh71 = (t2 + 2 as libc::c_int * t3 + t4 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    let ref mut fresh72 = *src
        .offset((1 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh72 = *fresh71;
    *src
        .offset(
            (0 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh72;
    let ref mut fresh73 = *src
        .offset((3 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh73 = (t3 + 2 as libc::c_int * t4 + t5 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    let ref mut fresh74 = *src
        .offset((2 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh74 = *fresh73;
    let ref mut fresh75 = *src
        .offset((1 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh75 = *fresh74;
    *src
        .offset(
            (0 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh75;
    let ref mut fresh76 = *src
        .offset((4 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh76 = (t4 + 2 as libc::c_int * t5 + t6 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    let ref mut fresh77 = *src
        .offset((3 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh77 = *fresh76;
    let ref mut fresh78 = *src
        .offset((2 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh78 = *fresh77;
    let ref mut fresh79 = *src
        .offset((1 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh79 = *fresh78;
    *src
        .offset(
            (0 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh79;
    let ref mut fresh80 = *src
        .offset((5 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh80 = (t5 + 2 as libc::c_int * t6 + t7 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    let ref mut fresh81 = *src
        .offset((4 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh81 = *fresh80;
    let ref mut fresh82 = *src
        .offset((3 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh82 = *fresh81;
    let ref mut fresh83 = *src
        .offset((2 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh83 = *fresh82;
    let ref mut fresh84 = *src
        .offset((1 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh84 = *fresh83;
    *src
        .offset(
            (0 as libc::c_int + 5 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh84;
    let ref mut fresh85 = *src
        .offset((6 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh85 = (t6 + 2 as libc::c_int * t7 + t8 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    let ref mut fresh86 = *src
        .offset((5 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh86 = *fresh85;
    let ref mut fresh87 = *src
        .offset((4 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh87 = *fresh86;
    let ref mut fresh88 = *src
        .offset((3 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh88 = *fresh87;
    let ref mut fresh89 = *src
        .offset((2 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh89 = *fresh88;
    let ref mut fresh90 = *src
        .offset((1 as libc::c_int + 5 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh90 = *fresh89;
    *src
        .offset(
            (0 as libc::c_int + 6 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh90;
    let ref mut fresh91 = *src
        .offset((7 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh91 = (t7 + 2 as libc::c_int * t8 + t9 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    let ref mut fresh92 = *src
        .offset((6 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh92 = *fresh91;
    let ref mut fresh93 = *src
        .offset((5 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh93 = *fresh92;
    let ref mut fresh94 = *src
        .offset((4 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh94 = *fresh93;
    let ref mut fresh95 = *src
        .offset((3 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh95 = *fresh94;
    let ref mut fresh96 = *src
        .offset((2 as libc::c_int + 5 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh96 = *fresh95;
    let ref mut fresh97 = *src
        .offset((1 as libc::c_int + 6 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh97 = *fresh96;
    *src
        .offset(
            (0 as libc::c_int + 7 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh97;
    let ref mut fresh98 = *src
        .offset((7 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh98 = (t8 + 2 as libc::c_int * t9 + t10 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    let ref mut fresh99 = *src
        .offset((6 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh99 = *fresh98;
    let ref mut fresh100 = *src
        .offset((5 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh100 = *fresh99;
    let ref mut fresh101 = *src
        .offset((4 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh101 = *fresh100;
    let ref mut fresh102 = *src
        .offset((3 as libc::c_int + 5 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh102 = *fresh101;
    let ref mut fresh103 = *src
        .offset((2 as libc::c_int + 6 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh103 = *fresh102;
    *src
        .offset(
            (1 as libc::c_int + 7 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh103;
    let ref mut fresh104 = *src
        .offset((7 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh104 = (t9 + 2 as libc::c_int * t10 + t11 + 2 as libc::c_int
        >> 2 as libc::c_int) as pixel;
    let ref mut fresh105 = *src
        .offset((6 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh105 = *fresh104;
    let ref mut fresh106 = *src
        .offset((5 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh106 = *fresh105;
    let ref mut fresh107 = *src
        .offset((4 as libc::c_int + 5 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh107 = *fresh106;
    let ref mut fresh108 = *src
        .offset((3 as libc::c_int + 6 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh108 = *fresh107;
    *src
        .offset(
            (2 as libc::c_int + 7 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh108;
    let ref mut fresh109 = *src
        .offset((7 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh109 = (t10 + 2 as libc::c_int * t11 + t12 + 2 as libc::c_int
        >> 2 as libc::c_int) as pixel;
    let ref mut fresh110 = *src
        .offset((6 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh110 = *fresh109;
    let ref mut fresh111 = *src
        .offset((5 as libc::c_int + 5 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh111 = *fresh110;
    let ref mut fresh112 = *src
        .offset((4 as libc::c_int + 6 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh112 = *fresh111;
    *src
        .offset(
            (3 as libc::c_int + 7 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh112;
    let ref mut fresh113 = *src
        .offset((7 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh113 = (t11 + 2 as libc::c_int * t12 + t13 + 2 as libc::c_int
        >> 2 as libc::c_int) as pixel;
    let ref mut fresh114 = *src
        .offset((6 as libc::c_int + 5 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh114 = *fresh113;
    let ref mut fresh115 = *src
        .offset((5 as libc::c_int + 6 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh115 = *fresh114;
    *src
        .offset(
            (4 as libc::c_int + 7 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh115;
    let ref mut fresh116 = *src
        .offset((7 as libc::c_int + 5 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh116 = (t12 + 2 as libc::c_int * t13 + t14 + 2 as libc::c_int
        >> 2 as libc::c_int) as pixel;
    let ref mut fresh117 = *src
        .offset((6 as libc::c_int + 6 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh117 = *fresh116;
    *src
        .offset(
            (5 as libc::c_int + 7 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh117;
    let ref mut fresh118 = *src
        .offset((7 as libc::c_int + 6 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh118 = (t13 + 2 as libc::c_int * t14 + t15 + 2 as libc::c_int
        >> 2 as libc::c_int) as pixel;
    *src
        .offset(
            (6 as libc::c_int + 7 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh118;
    *src
        .offset(
            (7 as libc::c_int + 7 as libc::c_int * 32 as libc::c_int) as isize,
        ) = (t14 + 2 as libc::c_int * t15 + t15 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
}
unsafe extern "C" fn predict_8x8_ddr_c(mut src: *mut pixel, mut edge: *mut pixel) {
    let mut t0: libc::c_int = *edge
        .offset((16 as libc::c_int + 0 as libc::c_int) as isize) as libc::c_int;
    let mut t1: libc::c_int = *edge
        .offset((16 as libc::c_int + 1 as libc::c_int) as isize) as libc::c_int;
    let mut t2: libc::c_int = *edge
        .offset((16 as libc::c_int + 2 as libc::c_int) as isize) as libc::c_int;
    let mut t3: libc::c_int = *edge
        .offset((16 as libc::c_int + 3 as libc::c_int) as isize) as libc::c_int;
    let mut t4: libc::c_int = *edge
        .offset((16 as libc::c_int + 4 as libc::c_int) as isize) as libc::c_int;
    let mut t5: libc::c_int = *edge
        .offset((16 as libc::c_int + 5 as libc::c_int) as isize) as libc::c_int;
    let mut t6: libc::c_int = *edge
        .offset((16 as libc::c_int + 6 as libc::c_int) as isize) as libc::c_int;
    let mut t7: libc::c_int = *edge
        .offset((16 as libc::c_int + 7 as libc::c_int) as isize) as libc::c_int;
    let mut l0: libc::c_int = *edge
        .offset((14 as libc::c_int - 0 as libc::c_int) as isize) as libc::c_int;
    let mut l1: libc::c_int = *edge
        .offset((14 as libc::c_int - 1 as libc::c_int) as isize) as libc::c_int;
    let mut l2: libc::c_int = *edge
        .offset((14 as libc::c_int - 2 as libc::c_int) as isize) as libc::c_int;
    let mut l3: libc::c_int = *edge
        .offset((14 as libc::c_int - 3 as libc::c_int) as isize) as libc::c_int;
    let mut l4: libc::c_int = *edge
        .offset((14 as libc::c_int - 4 as libc::c_int) as isize) as libc::c_int;
    let mut l5: libc::c_int = *edge
        .offset((14 as libc::c_int - 5 as libc::c_int) as isize) as libc::c_int;
    let mut l6: libc::c_int = *edge
        .offset((14 as libc::c_int - 6 as libc::c_int) as isize) as libc::c_int;
    let mut l7: libc::c_int = *edge
        .offset((14 as libc::c_int - 7 as libc::c_int) as isize) as libc::c_int;
    let mut lt: libc::c_int = *edge.offset(15 as libc::c_int as isize) as libc::c_int;
    *src
        .offset(
            (0 as libc::c_int + 7 as libc::c_int * 32 as libc::c_int) as isize,
        ) = (l7 + 2 as libc::c_int * l6 + l5 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    let ref mut fresh119 = *src
        .offset((1 as libc::c_int + 7 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh119 = (l6 + 2 as libc::c_int * l5 + l4 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    *src
        .offset(
            (0 as libc::c_int + 6 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh119;
    let ref mut fresh120 = *src
        .offset((2 as libc::c_int + 7 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh120 = (l5 + 2 as libc::c_int * l4 + l3 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    let ref mut fresh121 = *src
        .offset((1 as libc::c_int + 6 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh121 = *fresh120;
    *src
        .offset(
            (0 as libc::c_int + 5 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh121;
    let ref mut fresh122 = *src
        .offset((3 as libc::c_int + 7 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh122 = (l4 + 2 as libc::c_int * l3 + l2 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    let ref mut fresh123 = *src
        .offset((2 as libc::c_int + 6 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh123 = *fresh122;
    let ref mut fresh124 = *src
        .offset((1 as libc::c_int + 5 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh124 = *fresh123;
    *src
        .offset(
            (0 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh124;
    let ref mut fresh125 = *src
        .offset((4 as libc::c_int + 7 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh125 = (l3 + 2 as libc::c_int * l2 + l1 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    let ref mut fresh126 = *src
        .offset((3 as libc::c_int + 6 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh126 = *fresh125;
    let ref mut fresh127 = *src
        .offset((2 as libc::c_int + 5 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh127 = *fresh126;
    let ref mut fresh128 = *src
        .offset((1 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh128 = *fresh127;
    *src
        .offset(
            (0 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh128;
    let ref mut fresh129 = *src
        .offset((5 as libc::c_int + 7 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh129 = (l2 + 2 as libc::c_int * l1 + l0 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    let ref mut fresh130 = *src
        .offset((4 as libc::c_int + 6 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh130 = *fresh129;
    let ref mut fresh131 = *src
        .offset((3 as libc::c_int + 5 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh131 = *fresh130;
    let ref mut fresh132 = *src
        .offset((2 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh132 = *fresh131;
    let ref mut fresh133 = *src
        .offset((1 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh133 = *fresh132;
    *src
        .offset(
            (0 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh133;
    let ref mut fresh134 = *src
        .offset((6 as libc::c_int + 7 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh134 = (l1 + 2 as libc::c_int * l0 + lt + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    let ref mut fresh135 = *src
        .offset((5 as libc::c_int + 6 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh135 = *fresh134;
    let ref mut fresh136 = *src
        .offset((4 as libc::c_int + 5 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh136 = *fresh135;
    let ref mut fresh137 = *src
        .offset((3 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh137 = *fresh136;
    let ref mut fresh138 = *src
        .offset((2 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh138 = *fresh137;
    let ref mut fresh139 = *src
        .offset((1 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh139 = *fresh138;
    *src
        .offset(
            (0 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh139;
    let ref mut fresh140 = *src
        .offset((7 as libc::c_int + 7 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh140 = (l0 + 2 as libc::c_int * lt + t0 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    let ref mut fresh141 = *src
        .offset((6 as libc::c_int + 6 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh141 = *fresh140;
    let ref mut fresh142 = *src
        .offset((5 as libc::c_int + 5 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh142 = *fresh141;
    let ref mut fresh143 = *src
        .offset((4 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh143 = *fresh142;
    let ref mut fresh144 = *src
        .offset((3 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh144 = *fresh143;
    let ref mut fresh145 = *src
        .offset((2 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh145 = *fresh144;
    let ref mut fresh146 = *src
        .offset((1 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh146 = *fresh145;
    *src
        .offset(
            (0 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh146;
    let ref mut fresh147 = *src
        .offset((7 as libc::c_int + 6 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh147 = (lt + 2 as libc::c_int * t0 + t1 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    let ref mut fresh148 = *src
        .offset((6 as libc::c_int + 5 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh148 = *fresh147;
    let ref mut fresh149 = *src
        .offset((5 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh149 = *fresh148;
    let ref mut fresh150 = *src
        .offset((4 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh150 = *fresh149;
    let ref mut fresh151 = *src
        .offset((3 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh151 = *fresh150;
    let ref mut fresh152 = *src
        .offset((2 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh152 = *fresh151;
    *src
        .offset(
            (1 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh152;
    let ref mut fresh153 = *src
        .offset((7 as libc::c_int + 5 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh153 = (t0 + 2 as libc::c_int * t1 + t2 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    let ref mut fresh154 = *src
        .offset((6 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh154 = *fresh153;
    let ref mut fresh155 = *src
        .offset((5 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh155 = *fresh154;
    let ref mut fresh156 = *src
        .offset((4 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh156 = *fresh155;
    let ref mut fresh157 = *src
        .offset((3 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh157 = *fresh156;
    *src
        .offset(
            (2 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh157;
    let ref mut fresh158 = *src
        .offset((7 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh158 = (t1 + 2 as libc::c_int * t2 + t3 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    let ref mut fresh159 = *src
        .offset((6 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh159 = *fresh158;
    let ref mut fresh160 = *src
        .offset((5 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh160 = *fresh159;
    let ref mut fresh161 = *src
        .offset((4 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh161 = *fresh160;
    *src
        .offset(
            (3 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh161;
    let ref mut fresh162 = *src
        .offset((7 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh162 = (t2 + 2 as libc::c_int * t3 + t4 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    let ref mut fresh163 = *src
        .offset((6 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh163 = *fresh162;
    let ref mut fresh164 = *src
        .offset((5 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh164 = *fresh163;
    *src
        .offset(
            (4 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh164;
    let ref mut fresh165 = *src
        .offset((7 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh165 = (t3 + 2 as libc::c_int * t4 + t5 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    let ref mut fresh166 = *src
        .offset((6 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh166 = *fresh165;
    *src
        .offset(
            (5 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh166;
    let ref mut fresh167 = *src
        .offset((7 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh167 = (t4 + 2 as libc::c_int * t5 + t6 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    *src
        .offset(
            (6 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh167;
    *src
        .offset(
            (7 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = (t5 + 2 as libc::c_int * t6 + t7 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
}
unsafe extern "C" fn predict_8x8_vr_c(mut src: *mut pixel, mut edge: *mut pixel) {
    let mut t0: libc::c_int = *edge
        .offset((16 as libc::c_int + 0 as libc::c_int) as isize) as libc::c_int;
    let mut t1: libc::c_int = *edge
        .offset((16 as libc::c_int + 1 as libc::c_int) as isize) as libc::c_int;
    let mut t2: libc::c_int = *edge
        .offset((16 as libc::c_int + 2 as libc::c_int) as isize) as libc::c_int;
    let mut t3: libc::c_int = *edge
        .offset((16 as libc::c_int + 3 as libc::c_int) as isize) as libc::c_int;
    let mut t4: libc::c_int = *edge
        .offset((16 as libc::c_int + 4 as libc::c_int) as isize) as libc::c_int;
    let mut t5: libc::c_int = *edge
        .offset((16 as libc::c_int + 5 as libc::c_int) as isize) as libc::c_int;
    let mut t6: libc::c_int = *edge
        .offset((16 as libc::c_int + 6 as libc::c_int) as isize) as libc::c_int;
    let mut t7: libc::c_int = *edge
        .offset((16 as libc::c_int + 7 as libc::c_int) as isize) as libc::c_int;
    let mut l0: libc::c_int = *edge
        .offset((14 as libc::c_int - 0 as libc::c_int) as isize) as libc::c_int;
    let mut l1: libc::c_int = *edge
        .offset((14 as libc::c_int - 1 as libc::c_int) as isize) as libc::c_int;
    let mut l2: libc::c_int = *edge
        .offset((14 as libc::c_int - 2 as libc::c_int) as isize) as libc::c_int;
    let mut l3: libc::c_int = *edge
        .offset((14 as libc::c_int - 3 as libc::c_int) as isize) as libc::c_int;
    let mut l4: libc::c_int = *edge
        .offset((14 as libc::c_int - 4 as libc::c_int) as isize) as libc::c_int;
    let mut l5: libc::c_int = *edge
        .offset((14 as libc::c_int - 5 as libc::c_int) as isize) as libc::c_int;
    let mut l6: libc::c_int = *edge
        .offset((14 as libc::c_int - 6 as libc::c_int) as isize) as libc::c_int;
    let mut l7: libc::c_int = *edge
        .offset((14 as libc::c_int - 7 as libc::c_int) as isize) as libc::c_int;
    let mut lt: libc::c_int = *edge.offset(15 as libc::c_int as isize) as libc::c_int;
    *src
        .offset(
            (0 as libc::c_int + 6 as libc::c_int * 32 as libc::c_int) as isize,
        ) = (l5 + 2 as libc::c_int * l4 + l3 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    *src
        .offset(
            (0 as libc::c_int + 7 as libc::c_int * 32 as libc::c_int) as isize,
        ) = (l6 + 2 as libc::c_int * l5 + l4 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    let ref mut fresh168 = *src
        .offset((1 as libc::c_int + 6 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh168 = (l3 + 2 as libc::c_int * l2 + l1 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    *src
        .offset(
            (0 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh168;
    let ref mut fresh169 = *src
        .offset((1 as libc::c_int + 7 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh169 = (l4 + 2 as libc::c_int * l3 + l2 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    *src
        .offset(
            (0 as libc::c_int + 5 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh169;
    let ref mut fresh170 = *src
        .offset((2 as libc::c_int + 6 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh170 = (l1 + 2 as libc::c_int * l0 + lt + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    let ref mut fresh171 = *src
        .offset((1 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh171 = *fresh170;
    *src
        .offset(
            (0 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh171;
    let ref mut fresh172 = *src
        .offset((2 as libc::c_int + 7 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh172 = (l2 + 2 as libc::c_int * l1 + l0 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    let ref mut fresh173 = *src
        .offset((1 as libc::c_int + 5 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh173 = *fresh172;
    *src
        .offset(
            (0 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh173;
    let ref mut fresh174 = *src
        .offset((3 as libc::c_int + 7 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh174 = (l0 + 2 as libc::c_int * lt + t0 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    let ref mut fresh175 = *src
        .offset((2 as libc::c_int + 5 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh175 = *fresh174;
    let ref mut fresh176 = *src
        .offset((1 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh176 = *fresh175;
    *src
        .offset(
            (0 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh176;
    let ref mut fresh177 = *src
        .offset((3 as libc::c_int + 6 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh177 = (lt + t0 + 1 as libc::c_int >> 1 as libc::c_int) as pixel;
    let ref mut fresh178 = *src
        .offset((2 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh178 = *fresh177;
    let ref mut fresh179 = *src
        .offset((1 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh179 = *fresh178;
    *src
        .offset(
            (0 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh179;
    let ref mut fresh180 = *src
        .offset((4 as libc::c_int + 7 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh180 = (lt + 2 as libc::c_int * t0 + t1 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    let ref mut fresh181 = *src
        .offset((3 as libc::c_int + 5 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh181 = *fresh180;
    let ref mut fresh182 = *src
        .offset((2 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh182 = *fresh181;
    *src
        .offset(
            (1 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh182;
    let ref mut fresh183 = *src
        .offset((4 as libc::c_int + 6 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh183 = (t0 + t1 + 1 as libc::c_int >> 1 as libc::c_int) as pixel;
    let ref mut fresh184 = *src
        .offset((3 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh184 = *fresh183;
    let ref mut fresh185 = *src
        .offset((2 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh185 = *fresh184;
    *src
        .offset(
            (1 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh185;
    let ref mut fresh186 = *src
        .offset((5 as libc::c_int + 7 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh186 = (t0 + 2 as libc::c_int * t1 + t2 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    let ref mut fresh187 = *src
        .offset((4 as libc::c_int + 5 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh187 = *fresh186;
    let ref mut fresh188 = *src
        .offset((3 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh188 = *fresh187;
    *src
        .offset(
            (2 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh188;
    let ref mut fresh189 = *src
        .offset((5 as libc::c_int + 6 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh189 = (t1 + t2 + 1 as libc::c_int >> 1 as libc::c_int) as pixel;
    let ref mut fresh190 = *src
        .offset((4 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh190 = *fresh189;
    let ref mut fresh191 = *src
        .offset((3 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh191 = *fresh190;
    *src
        .offset(
            (2 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh191;
    let ref mut fresh192 = *src
        .offset((6 as libc::c_int + 7 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh192 = (t1 + 2 as libc::c_int * t2 + t3 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    let ref mut fresh193 = *src
        .offset((5 as libc::c_int + 5 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh193 = *fresh192;
    let ref mut fresh194 = *src
        .offset((4 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh194 = *fresh193;
    *src
        .offset(
            (3 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh194;
    let ref mut fresh195 = *src
        .offset((6 as libc::c_int + 6 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh195 = (t2 + t3 + 1 as libc::c_int >> 1 as libc::c_int) as pixel;
    let ref mut fresh196 = *src
        .offset((5 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh196 = *fresh195;
    let ref mut fresh197 = *src
        .offset((4 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh197 = *fresh196;
    *src
        .offset(
            (3 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh197;
    let ref mut fresh198 = *src
        .offset((7 as libc::c_int + 7 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh198 = (t2 + 2 as libc::c_int * t3 + t4 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    let ref mut fresh199 = *src
        .offset((6 as libc::c_int + 5 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh199 = *fresh198;
    let ref mut fresh200 = *src
        .offset((5 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh200 = *fresh199;
    *src
        .offset(
            (4 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh200;
    let ref mut fresh201 = *src
        .offset((7 as libc::c_int + 6 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh201 = (t3 + t4 + 1 as libc::c_int >> 1 as libc::c_int) as pixel;
    let ref mut fresh202 = *src
        .offset((6 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh202 = *fresh201;
    let ref mut fresh203 = *src
        .offset((5 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh203 = *fresh202;
    *src
        .offset(
            (4 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh203;
    let ref mut fresh204 = *src
        .offset((7 as libc::c_int + 5 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh204 = (t3 + 2 as libc::c_int * t4 + t5 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    let ref mut fresh205 = *src
        .offset((6 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh205 = *fresh204;
    *src
        .offset(
            (5 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh205;
    let ref mut fresh206 = *src
        .offset((7 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh206 = (t4 + t5 + 1 as libc::c_int >> 1 as libc::c_int) as pixel;
    let ref mut fresh207 = *src
        .offset((6 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh207 = *fresh206;
    *src
        .offset(
            (5 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh207;
    let ref mut fresh208 = *src
        .offset((7 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh208 = (t4 + 2 as libc::c_int * t5 + t6 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    *src
        .offset(
            (6 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh208;
    let ref mut fresh209 = *src
        .offset((7 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh209 = (t5 + t6 + 1 as libc::c_int >> 1 as libc::c_int) as pixel;
    *src
        .offset(
            (6 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh209;
    *src
        .offset(
            (7 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize,
        ) = (t5 + 2 as libc::c_int * t6 + t7 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    *src
        .offset(
            (7 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = (t6 + t7 + 1 as libc::c_int >> 1 as libc::c_int) as pixel;
}
unsafe extern "C" fn predict_8x8_hd_c(mut src: *mut pixel, mut edge: *mut pixel) {
    let mut t0: libc::c_int = *edge
        .offset((16 as libc::c_int + 0 as libc::c_int) as isize) as libc::c_int;
    let mut t1: libc::c_int = *edge
        .offset((16 as libc::c_int + 1 as libc::c_int) as isize) as libc::c_int;
    let mut t2: libc::c_int = *edge
        .offset((16 as libc::c_int + 2 as libc::c_int) as isize) as libc::c_int;
    let mut t3: libc::c_int = *edge
        .offset((16 as libc::c_int + 3 as libc::c_int) as isize) as libc::c_int;
    let mut t4: libc::c_int = *edge
        .offset((16 as libc::c_int + 4 as libc::c_int) as isize) as libc::c_int;
    let mut t5: libc::c_int = *edge
        .offset((16 as libc::c_int + 5 as libc::c_int) as isize) as libc::c_int;
    let mut t6: libc::c_int = *edge
        .offset((16 as libc::c_int + 6 as libc::c_int) as isize) as libc::c_int;
    let mut t7: libc::c_int = *edge
        .offset((16 as libc::c_int + 7 as libc::c_int) as isize) as libc::c_int;
    let mut l0: libc::c_int = *edge
        .offset((14 as libc::c_int - 0 as libc::c_int) as isize) as libc::c_int;
    let mut l1: libc::c_int = *edge
        .offset((14 as libc::c_int - 1 as libc::c_int) as isize) as libc::c_int;
    let mut l2: libc::c_int = *edge
        .offset((14 as libc::c_int - 2 as libc::c_int) as isize) as libc::c_int;
    let mut l3: libc::c_int = *edge
        .offset((14 as libc::c_int - 3 as libc::c_int) as isize) as libc::c_int;
    let mut l4: libc::c_int = *edge
        .offset((14 as libc::c_int - 4 as libc::c_int) as isize) as libc::c_int;
    let mut l5: libc::c_int = *edge
        .offset((14 as libc::c_int - 5 as libc::c_int) as isize) as libc::c_int;
    let mut l6: libc::c_int = *edge
        .offset((14 as libc::c_int - 6 as libc::c_int) as isize) as libc::c_int;
    let mut l7: libc::c_int = *edge
        .offset((14 as libc::c_int - 7 as libc::c_int) as isize) as libc::c_int;
    let mut lt: libc::c_int = *edge.offset(15 as libc::c_int as isize) as libc::c_int;
    let mut p1: libc::c_int = pack8to16(
        (l6 + l7 + 1 as libc::c_int >> 1 as libc::c_int) as uint32_t,
        (l5 + 2 as libc::c_int * l6 + l7 + 2 as libc::c_int >> 2 as libc::c_int)
            as uint32_t,
    ) as libc::c_int;
    let mut p2: libc::c_int = pack8to16(
        (l5 + l6 + 1 as libc::c_int >> 1 as libc::c_int) as uint32_t,
        (l4 + 2 as libc::c_int * l5 + l6 + 2 as libc::c_int >> 2 as libc::c_int)
            as uint32_t,
    ) as libc::c_int;
    let mut p3: libc::c_int = pack8to16(
        (l4 + l5 + 1 as libc::c_int >> 1 as libc::c_int) as uint32_t,
        (l3 + 2 as libc::c_int * l4 + l5 + 2 as libc::c_int >> 2 as libc::c_int)
            as uint32_t,
    ) as libc::c_int;
    let mut p4: libc::c_int = pack8to16(
        (l3 + l4 + 1 as libc::c_int >> 1 as libc::c_int) as uint32_t,
        (l2 + 2 as libc::c_int * l3 + l4 + 2 as libc::c_int >> 2 as libc::c_int)
            as uint32_t,
    ) as libc::c_int;
    let mut p5: libc::c_int = pack8to16(
        (l2 + l3 + 1 as libc::c_int >> 1 as libc::c_int) as uint32_t,
        (l1 + 2 as libc::c_int * l2 + l3 + 2 as libc::c_int >> 2 as libc::c_int)
            as uint32_t,
    ) as libc::c_int;
    let mut p6: libc::c_int = pack8to16(
        (l1 + l2 + 1 as libc::c_int >> 1 as libc::c_int) as uint32_t,
        (l0 + 2 as libc::c_int * l1 + l2 + 2 as libc::c_int >> 2 as libc::c_int)
            as uint32_t,
    ) as libc::c_int;
    let mut p7: libc::c_int = pack8to16(
        (l0 + l1 + 1 as libc::c_int >> 1 as libc::c_int) as uint32_t,
        (lt + 2 as libc::c_int * l0 + l1 + 2 as libc::c_int >> 2 as libc::c_int)
            as uint32_t,
    ) as libc::c_int;
    let mut p8: libc::c_int = pack8to16(
        (lt + l0 + 1 as libc::c_int >> 1 as libc::c_int) as uint32_t,
        (l0 + 2 as libc::c_int * lt + t0 + 2 as libc::c_int >> 2 as libc::c_int)
            as uint32_t,
    ) as libc::c_int;
    let mut p9: libc::c_int = pack8to16(
        (t1 + 2 as libc::c_int * t0 + lt + 2 as libc::c_int >> 2 as libc::c_int)
            as uint32_t,
        (t2 + 2 as libc::c_int * t1 + t0 + 2 as libc::c_int >> 2 as libc::c_int)
            as uint32_t,
    ) as libc::c_int;
    let mut p10: libc::c_int = pack8to16(
        (t3 + 2 as libc::c_int * t2 + t1 + 2 as libc::c_int >> 2 as libc::c_int)
            as uint32_t,
        (t4 + 2 as libc::c_int * t3 + t2 + 2 as libc::c_int >> 2 as libc::c_int)
            as uint32_t,
    ) as libc::c_int;
    let mut p11: libc::c_int = pack8to16(
        (t5 + 2 as libc::c_int * t4 + t3 + 2 as libc::c_int >> 2 as libc::c_int)
            as uint32_t,
        (t6 + 2 as libc::c_int * t5 + t4 + 2 as libc::c_int >> 2 as libc::c_int)
            as uint32_t,
    ) as libc::c_int;
    (*(&mut *src
        .offset((0 as libc::c_int + 7 as libc::c_int * 32 as libc::c_int) as isize)
        as *mut pixel as *mut x264_union32_t))
        .i = pack16to32(p1 as uint32_t, p2 as uint32_t);
    (*(&mut *src
        .offset((0 as libc::c_int + 6 as libc::c_int * 32 as libc::c_int) as isize)
        as *mut pixel as *mut x264_union32_t))
        .i = pack16to32(p2 as uint32_t, p3 as uint32_t);
    let ref mut fresh210 = (*(&mut *src
        .offset((0 as libc::c_int + 5 as libc::c_int * 32 as libc::c_int) as isize)
        as *mut pixel as *mut x264_union32_t))
        .i;
    *fresh210 = pack16to32(p3 as uint32_t, p4 as uint32_t);
    (*(&mut *src
        .offset((4 as libc::c_int + 7 as libc::c_int * 32 as libc::c_int) as isize)
        as *mut pixel as *mut x264_union32_t))
        .i = *fresh210;
    let ref mut fresh211 = (*(&mut *src
        .offset((0 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int) as isize)
        as *mut pixel as *mut x264_union32_t))
        .i;
    *fresh211 = pack16to32(p4 as uint32_t, p5 as uint32_t);
    (*(&mut *src
        .offset((4 as libc::c_int + 6 as libc::c_int * 32 as libc::c_int) as isize)
        as *mut pixel as *mut x264_union32_t))
        .i = *fresh211;
    let ref mut fresh212 = (*(&mut *src
        .offset((0 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize)
        as *mut pixel as *mut x264_union32_t))
        .i;
    *fresh212 = pack16to32(p5 as uint32_t, p6 as uint32_t);
    (*(&mut *src
        .offset((4 as libc::c_int + 5 as libc::c_int * 32 as libc::c_int) as isize)
        as *mut pixel as *mut x264_union32_t))
        .i = *fresh212;
    let ref mut fresh213 = (*(&mut *src
        .offset((0 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize)
        as *mut pixel as *mut x264_union32_t))
        .i;
    *fresh213 = pack16to32(p6 as uint32_t, p7 as uint32_t);
    (*(&mut *src
        .offset((4 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int) as isize)
        as *mut pixel as *mut x264_union32_t))
        .i = *fresh213;
    let ref mut fresh214 = (*(&mut *src
        .offset((0 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize)
        as *mut pixel as *mut x264_union32_t))
        .i;
    *fresh214 = pack16to32(p7 as uint32_t, p8 as uint32_t);
    (*(&mut *src
        .offset((4 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize)
        as *mut pixel as *mut x264_union32_t))
        .i = *fresh214;
    let ref mut fresh215 = (*(&mut *src
        .offset((0 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize)
        as *mut pixel as *mut x264_union32_t))
        .i;
    *fresh215 = pack16to32(p8 as uint32_t, p9 as uint32_t);
    (*(&mut *src
        .offset((4 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize)
        as *mut pixel as *mut x264_union32_t))
        .i = *fresh215;
    (*(&mut *src
        .offset((4 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize)
        as *mut pixel as *mut x264_union32_t))
        .i = pack16to32(p9 as uint32_t, p10 as uint32_t);
    (*(&mut *src
        .offset((4 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize)
        as *mut pixel as *mut x264_union32_t))
        .i = pack16to32(p10 as uint32_t, p11 as uint32_t);
}
unsafe extern "C" fn predict_8x8_vl_c(mut src: *mut pixel, mut edge: *mut pixel) {
    let mut t0: libc::c_int = *edge
        .offset((16 as libc::c_int + 0 as libc::c_int) as isize) as libc::c_int;
    let mut t1: libc::c_int = *edge
        .offset((16 as libc::c_int + 1 as libc::c_int) as isize) as libc::c_int;
    let mut t2: libc::c_int = *edge
        .offset((16 as libc::c_int + 2 as libc::c_int) as isize) as libc::c_int;
    let mut t3: libc::c_int = *edge
        .offset((16 as libc::c_int + 3 as libc::c_int) as isize) as libc::c_int;
    let mut t4: libc::c_int = *edge
        .offset((16 as libc::c_int + 4 as libc::c_int) as isize) as libc::c_int;
    let mut t5: libc::c_int = *edge
        .offset((16 as libc::c_int + 5 as libc::c_int) as isize) as libc::c_int;
    let mut t6: libc::c_int = *edge
        .offset((16 as libc::c_int + 6 as libc::c_int) as isize) as libc::c_int;
    let mut t7: libc::c_int = *edge
        .offset((16 as libc::c_int + 7 as libc::c_int) as isize) as libc::c_int;
    let mut t8: libc::c_int = *edge
        .offset((16 as libc::c_int + 8 as libc::c_int) as isize) as libc::c_int;
    let mut t9: libc::c_int = *edge
        .offset((16 as libc::c_int + 9 as libc::c_int) as isize) as libc::c_int;
    let mut t10: libc::c_int = *edge
        .offset((16 as libc::c_int + 10 as libc::c_int) as isize) as libc::c_int;
    let mut t11: libc::c_int = *edge
        .offset((16 as libc::c_int + 11 as libc::c_int) as isize) as libc::c_int;
    let mut t12: libc::c_int = *edge
        .offset((16 as libc::c_int + 12 as libc::c_int) as isize) as libc::c_int;
    let mut t13: libc::c_int = *edge
        .offset((16 as libc::c_int + 13 as libc::c_int) as isize) as libc::c_int;
    let mut t14: libc::c_int = *edge
        .offset((16 as libc::c_int + 14 as libc::c_int) as isize) as libc::c_int;
    let mut t15: libc::c_int = *edge
        .offset((16 as libc::c_int + 15 as libc::c_int) as isize) as libc::c_int;
    *src
        .offset(
            (0 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize,
        ) = (t0 + t1 + 1 as libc::c_int >> 1 as libc::c_int) as pixel;
    *src
        .offset(
            (0 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize,
        ) = (t0 + 2 as libc::c_int * t1 + t2 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    let ref mut fresh216 = *src
        .offset((1 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh216 = (t1 + t2 + 1 as libc::c_int >> 1 as libc::c_int) as pixel;
    *src
        .offset(
            (0 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh216;
    let ref mut fresh217 = *src
        .offset((1 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh217 = (t1 + 2 as libc::c_int * t2 + t3 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    *src
        .offset(
            (0 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh217;
    let ref mut fresh218 = *src
        .offset((2 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh218 = (t2 + t3 + 1 as libc::c_int >> 1 as libc::c_int) as pixel;
    let ref mut fresh219 = *src
        .offset((1 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh219 = *fresh218;
    *src
        .offset(
            (0 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh219;
    let ref mut fresh220 = *src
        .offset((2 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh220 = (t2 + 2 as libc::c_int * t3 + t4 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    let ref mut fresh221 = *src
        .offset((1 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh221 = *fresh220;
    *src
        .offset(
            (0 as libc::c_int + 5 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh221;
    let ref mut fresh222 = *src
        .offset((3 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh222 = (t3 + t4 + 1 as libc::c_int >> 1 as libc::c_int) as pixel;
    let ref mut fresh223 = *src
        .offset((2 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh223 = *fresh222;
    let ref mut fresh224 = *src
        .offset((1 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh224 = *fresh223;
    *src
        .offset(
            (0 as libc::c_int + 6 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh224;
    let ref mut fresh225 = *src
        .offset((3 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh225 = (t3 + 2 as libc::c_int * t4 + t5 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    let ref mut fresh226 = *src
        .offset((2 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh226 = *fresh225;
    let ref mut fresh227 = *src
        .offset((1 as libc::c_int + 5 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh227 = *fresh226;
    *src
        .offset(
            (0 as libc::c_int + 7 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh227;
    let ref mut fresh228 = *src
        .offset((4 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh228 = (t4 + t5 + 1 as libc::c_int >> 1 as libc::c_int) as pixel;
    let ref mut fresh229 = *src
        .offset((3 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh229 = *fresh228;
    let ref mut fresh230 = *src
        .offset((2 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh230 = *fresh229;
    *src
        .offset(
            (1 as libc::c_int + 6 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh230;
    let ref mut fresh231 = *src
        .offset((4 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh231 = (t4 + 2 as libc::c_int * t5 + t6 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    let ref mut fresh232 = *src
        .offset((3 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh232 = *fresh231;
    let ref mut fresh233 = *src
        .offset((2 as libc::c_int + 5 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh233 = *fresh232;
    *src
        .offset(
            (1 as libc::c_int + 7 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh233;
    let ref mut fresh234 = *src
        .offset((5 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh234 = (t5 + t6 + 1 as libc::c_int >> 1 as libc::c_int) as pixel;
    let ref mut fresh235 = *src
        .offset((4 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh235 = *fresh234;
    let ref mut fresh236 = *src
        .offset((3 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh236 = *fresh235;
    *src
        .offset(
            (2 as libc::c_int + 6 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh236;
    let ref mut fresh237 = *src
        .offset((5 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh237 = (t5 + 2 as libc::c_int * t6 + t7 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    let ref mut fresh238 = *src
        .offset((4 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh238 = *fresh237;
    let ref mut fresh239 = *src
        .offset((3 as libc::c_int + 5 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh239 = *fresh238;
    *src
        .offset(
            (2 as libc::c_int + 7 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh239;
    let ref mut fresh240 = *src
        .offset((6 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh240 = (t6 + t7 + 1 as libc::c_int >> 1 as libc::c_int) as pixel;
    let ref mut fresh241 = *src
        .offset((5 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh241 = *fresh240;
    let ref mut fresh242 = *src
        .offset((4 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh242 = *fresh241;
    *src
        .offset(
            (3 as libc::c_int + 6 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh242;
    let ref mut fresh243 = *src
        .offset((6 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh243 = (t6 + 2 as libc::c_int * t7 + t8 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    let ref mut fresh244 = *src
        .offset((5 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh244 = *fresh243;
    let ref mut fresh245 = *src
        .offset((4 as libc::c_int + 5 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh245 = *fresh244;
    *src
        .offset(
            (3 as libc::c_int + 7 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh245;
    let ref mut fresh246 = *src
        .offset((7 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh246 = (t7 + t8 + 1 as libc::c_int >> 1 as libc::c_int) as pixel;
    let ref mut fresh247 = *src
        .offset((6 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh247 = *fresh246;
    let ref mut fresh248 = *src
        .offset((5 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh248 = *fresh247;
    *src
        .offset(
            (4 as libc::c_int + 6 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh248;
    let ref mut fresh249 = *src
        .offset((7 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh249 = (t7 + 2 as libc::c_int * t8 + t9 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    let ref mut fresh250 = *src
        .offset((6 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh250 = *fresh249;
    let ref mut fresh251 = *src
        .offset((5 as libc::c_int + 5 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh251 = *fresh250;
    *src
        .offset(
            (4 as libc::c_int + 7 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh251;
    let ref mut fresh252 = *src
        .offset((7 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh252 = (t8 + t9 + 1 as libc::c_int >> 1 as libc::c_int) as pixel;
    let ref mut fresh253 = *src
        .offset((6 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh253 = *fresh252;
    *src
        .offset(
            (5 as libc::c_int + 6 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh253;
    let ref mut fresh254 = *src
        .offset((7 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh254 = (t8 + 2 as libc::c_int * t9 + t10 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
    let ref mut fresh255 = *src
        .offset((6 as libc::c_int + 5 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh255 = *fresh254;
    *src
        .offset(
            (5 as libc::c_int + 7 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh255;
    let ref mut fresh256 = *src
        .offset((7 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh256 = (t9 + t10 + 1 as libc::c_int >> 1 as libc::c_int) as pixel;
    *src
        .offset(
            (6 as libc::c_int + 6 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh256;
    let ref mut fresh257 = *src
        .offset((7 as libc::c_int + 5 as libc::c_int * 32 as libc::c_int) as isize);
    *fresh257 = (t9 + 2 as libc::c_int * t10 + t11 + 2 as libc::c_int
        >> 2 as libc::c_int) as pixel;
    *src
        .offset(
            (6 as libc::c_int + 7 as libc::c_int * 32 as libc::c_int) as isize,
        ) = *fresh257;
    *src
        .offset(
            (7 as libc::c_int + 6 as libc::c_int * 32 as libc::c_int) as isize,
        ) = (t10 + t11 + 1 as libc::c_int >> 1 as libc::c_int) as pixel;
    *src
        .offset(
            (7 as libc::c_int + 7 as libc::c_int * 32 as libc::c_int) as isize,
        ) = (t10 + 2 as libc::c_int * t11 + t12 + 2 as libc::c_int >> 2 as libc::c_int)
        as pixel;
}
unsafe extern "C" fn predict_8x8_hu_c(mut src: *mut pixel, mut edge: *mut pixel) {
    let mut l0: libc::c_int = *edge
        .offset((14 as libc::c_int - 0 as libc::c_int) as isize) as libc::c_int;
    let mut l1: libc::c_int = *edge
        .offset((14 as libc::c_int - 1 as libc::c_int) as isize) as libc::c_int;
    let mut l2: libc::c_int = *edge
        .offset((14 as libc::c_int - 2 as libc::c_int) as isize) as libc::c_int;
    let mut l3: libc::c_int = *edge
        .offset((14 as libc::c_int - 3 as libc::c_int) as isize) as libc::c_int;
    let mut l4: libc::c_int = *edge
        .offset((14 as libc::c_int - 4 as libc::c_int) as isize) as libc::c_int;
    let mut l5: libc::c_int = *edge
        .offset((14 as libc::c_int - 5 as libc::c_int) as isize) as libc::c_int;
    let mut l6: libc::c_int = *edge
        .offset((14 as libc::c_int - 6 as libc::c_int) as isize) as libc::c_int;
    let mut l7: libc::c_int = *edge
        .offset((14 as libc::c_int - 7 as libc::c_int) as isize) as libc::c_int;
    let mut p1: libc::c_int = pack8to16(
        (l0 + l1 + 1 as libc::c_int >> 1 as libc::c_int) as uint32_t,
        (l0 + 2 as libc::c_int * l1 + l2 + 2 as libc::c_int >> 2 as libc::c_int)
            as uint32_t,
    ) as libc::c_int;
    let mut p2: libc::c_int = pack8to16(
        (l1 + l2 + 1 as libc::c_int >> 1 as libc::c_int) as uint32_t,
        (l1 + 2 as libc::c_int * l2 + l3 + 2 as libc::c_int >> 2 as libc::c_int)
            as uint32_t,
    ) as libc::c_int;
    let mut p3: libc::c_int = pack8to16(
        (l2 + l3 + 1 as libc::c_int >> 1 as libc::c_int) as uint32_t,
        (l2 + 2 as libc::c_int * l3 + l4 + 2 as libc::c_int >> 2 as libc::c_int)
            as uint32_t,
    ) as libc::c_int;
    let mut p4: libc::c_int = pack8to16(
        (l3 + l4 + 1 as libc::c_int >> 1 as libc::c_int) as uint32_t,
        (l3 + 2 as libc::c_int * l4 + l5 + 2 as libc::c_int >> 2 as libc::c_int)
            as uint32_t,
    ) as libc::c_int;
    let mut p5: libc::c_int = pack8to16(
        (l4 + l5 + 1 as libc::c_int >> 1 as libc::c_int) as uint32_t,
        (l4 + 2 as libc::c_int * l5 + l6 + 2 as libc::c_int >> 2 as libc::c_int)
            as uint32_t,
    ) as libc::c_int;
    let mut p6: libc::c_int = pack8to16(
        (l5 + l6 + 1 as libc::c_int >> 1 as libc::c_int) as uint32_t,
        (l5 + 2 as libc::c_int * l6 + l7 + 2 as libc::c_int >> 2 as libc::c_int)
            as uint32_t,
    ) as libc::c_int;
    let mut p7: libc::c_int = pack8to16(
        (l6 + l7 + 1 as libc::c_int >> 1 as libc::c_int) as uint32_t,
        (l6 + 2 as libc::c_int * l7 + l7 + 2 as libc::c_int >> 2 as libc::c_int)
            as uint32_t,
    ) as libc::c_int;
    let mut p8: libc::c_int = pack8to16(l7 as uint32_t, l7 as uint32_t) as libc::c_int;
    (*(&mut *src
        .offset((0 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize)
        as *mut pixel as *mut x264_union32_t))
        .i = pack16to32(p1 as uint32_t, p2 as uint32_t);
    (*(&mut *src
        .offset((0 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize)
        as *mut pixel as *mut x264_union32_t))
        .i = pack16to32(p2 as uint32_t, p3 as uint32_t);
    let ref mut fresh258 = (*(&mut *src
        .offset((0 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize)
        as *mut pixel as *mut x264_union32_t))
        .i;
    *fresh258 = pack16to32(p3 as uint32_t, p4 as uint32_t);
    (*(&mut *src
        .offset((4 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int) as isize)
        as *mut pixel as *mut x264_union32_t))
        .i = *fresh258;
    let ref mut fresh259 = (*(&mut *src
        .offset((0 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize)
        as *mut pixel as *mut x264_union32_t))
        .i;
    *fresh259 = pack16to32(p4 as uint32_t, p5 as uint32_t);
    (*(&mut *src
        .offset((4 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int) as isize)
        as *mut pixel as *mut x264_union32_t))
        .i = *fresh259;
    let ref mut fresh260 = (*(&mut *src
        .offset((0 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int) as isize)
        as *mut pixel as *mut x264_union32_t))
        .i;
    *fresh260 = pack16to32(p5 as uint32_t, p6 as uint32_t);
    (*(&mut *src
        .offset((4 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int) as isize)
        as *mut pixel as *mut x264_union32_t))
        .i = *fresh260;
    let ref mut fresh261 = (*(&mut *src
        .offset((0 as libc::c_int + 5 as libc::c_int * 32 as libc::c_int) as isize)
        as *mut pixel as *mut x264_union32_t))
        .i;
    *fresh261 = pack16to32(p6 as uint32_t, p7 as uint32_t);
    (*(&mut *src
        .offset((4 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int) as isize)
        as *mut pixel as *mut x264_union32_t))
        .i = *fresh261;
    let ref mut fresh262 = (*(&mut *src
        .offset((0 as libc::c_int + 6 as libc::c_int * 32 as libc::c_int) as isize)
        as *mut pixel as *mut x264_union32_t))
        .i;
    *fresh262 = pack16to32(p7 as uint32_t, p8 as uint32_t);
    (*(&mut *src
        .offset((4 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int) as isize)
        as *mut pixel as *mut x264_union32_t))
        .i = *fresh262;
    let ref mut fresh263 = (*(&mut *src
        .offset((4 as libc::c_int + 7 as libc::c_int * 32 as libc::c_int) as isize)
        as *mut pixel as *mut x264_union32_t))
        .i;
    *fresh263 = pack16to32(p8 as uint32_t, p8 as uint32_t);
    let ref mut fresh264 = (*(&mut *src
        .offset((0 as libc::c_int + 7 as libc::c_int * 32 as libc::c_int) as isize)
        as *mut pixel as *mut x264_union32_t))
        .i;
    *fresh264 = *fresh263;
    let ref mut fresh265 = (*(&mut *src
        .offset((4 as libc::c_int + 6 as libc::c_int * 32 as libc::c_int) as isize)
        as *mut pixel as *mut x264_union32_t))
        .i;
    *fresh265 = *fresh264;
    (*(&mut *src
        .offset((4 as libc::c_int + 5 as libc::c_int * 32 as libc::c_int) as isize)
        as *mut pixel as *mut x264_union32_t))
        .i = *fresh265;
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_predict_16x16_init(
    mut cpu: uint32_t,
    mut pf: *mut x264_predict_t,
) {
    let ref mut fresh266 = *pf.offset(I_PRED_16x16_V as libc::c_int as isize);
    *fresh266 = Some(x264_8_predict_16x16_v_c as unsafe extern "C" fn(*mut pixel) -> ());
    let ref mut fresh267 = *pf.offset(I_PRED_16x16_H as libc::c_int as isize);
    *fresh267 = Some(x264_8_predict_16x16_h_c as unsafe extern "C" fn(*mut pixel) -> ());
    let ref mut fresh268 = *pf.offset(I_PRED_16x16_DC as libc::c_int as isize);
    *fresh268 = Some(
        x264_8_predict_16x16_dc_c as unsafe extern "C" fn(*mut pixel) -> (),
    );
    let ref mut fresh269 = *pf.offset(I_PRED_16x16_P as libc::c_int as isize);
    *fresh269 = Some(x264_8_predict_16x16_p_c as unsafe extern "C" fn(*mut pixel) -> ());
    let ref mut fresh270 = *pf.offset(I_PRED_16x16_DC_LEFT as libc::c_int as isize);
    *fresh270 = Some(predict_16x16_dc_left_c as unsafe extern "C" fn(*mut pixel) -> ());
    let ref mut fresh271 = *pf.offset(I_PRED_16x16_DC_TOP as libc::c_int as isize);
    *fresh271 = Some(predict_16x16_dc_top_c as unsafe extern "C" fn(*mut pixel) -> ());
    let ref mut fresh272 = *pf.offset(I_PRED_16x16_DC_128 as libc::c_int as isize);
    *fresh272 = Some(predict_16x16_dc_128_c as unsafe extern "C" fn(*mut pixel) -> ());
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_predict_8x8c_init(
    mut cpu: uint32_t,
    mut pf: *mut x264_predict_t,
) {
    let ref mut fresh273 = *pf.offset(I_PRED_CHROMA_V as libc::c_int as isize);
    *fresh273 = Some(x264_8_predict_8x8c_v_c as unsafe extern "C" fn(*mut pixel) -> ());
    let ref mut fresh274 = *pf.offset(I_PRED_CHROMA_H as libc::c_int as isize);
    *fresh274 = Some(x264_8_predict_8x8c_h_c as unsafe extern "C" fn(*mut pixel) -> ());
    let ref mut fresh275 = *pf.offset(I_PRED_CHROMA_DC as libc::c_int as isize);
    *fresh275 = Some(x264_8_predict_8x8c_dc_c as unsafe extern "C" fn(*mut pixel) -> ());
    let ref mut fresh276 = *pf.offset(I_PRED_CHROMA_P as libc::c_int as isize);
    *fresh276 = Some(x264_8_predict_8x8c_p_c as unsafe extern "C" fn(*mut pixel) -> ());
    let ref mut fresh277 = *pf.offset(I_PRED_CHROMA_DC_LEFT as libc::c_int as isize);
    *fresh277 = Some(predict_8x8c_dc_left_c as unsafe extern "C" fn(*mut pixel) -> ());
    let ref mut fresh278 = *pf.offset(I_PRED_CHROMA_DC_TOP as libc::c_int as isize);
    *fresh278 = Some(predict_8x8c_dc_top_c as unsafe extern "C" fn(*mut pixel) -> ());
    let ref mut fresh279 = *pf.offset(I_PRED_CHROMA_DC_128 as libc::c_int as isize);
    *fresh279 = Some(predict_8x8c_dc_128_c as unsafe extern "C" fn(*mut pixel) -> ());
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_predict_8x16c_init(
    mut cpu: uint32_t,
    mut pf: *mut x264_predict_t,
) {
    let ref mut fresh280 = *pf.offset(I_PRED_CHROMA_V as libc::c_int as isize);
    *fresh280 = Some(x264_8_predict_8x16c_v_c as unsafe extern "C" fn(*mut pixel) -> ());
    let ref mut fresh281 = *pf.offset(I_PRED_CHROMA_H as libc::c_int as isize);
    *fresh281 = Some(x264_8_predict_8x16c_h_c as unsafe extern "C" fn(*mut pixel) -> ());
    let ref mut fresh282 = *pf.offset(I_PRED_CHROMA_DC as libc::c_int as isize);
    *fresh282 = Some(
        x264_8_predict_8x16c_dc_c as unsafe extern "C" fn(*mut pixel) -> (),
    );
    let ref mut fresh283 = *pf.offset(I_PRED_CHROMA_P as libc::c_int as isize);
    *fresh283 = Some(x264_8_predict_8x16c_p_c as unsafe extern "C" fn(*mut pixel) -> ());
    let ref mut fresh284 = *pf.offset(I_PRED_CHROMA_DC_LEFT as libc::c_int as isize);
    *fresh284 = Some(predict_8x16c_dc_left_c as unsafe extern "C" fn(*mut pixel) -> ());
    let ref mut fresh285 = *pf.offset(I_PRED_CHROMA_DC_TOP as libc::c_int as isize);
    *fresh285 = Some(predict_8x16c_dc_top_c as unsafe extern "C" fn(*mut pixel) -> ());
    let ref mut fresh286 = *pf.offset(I_PRED_CHROMA_DC_128 as libc::c_int as isize);
    *fresh286 = Some(predict_8x16c_dc_128_c as unsafe extern "C" fn(*mut pixel) -> ());
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_predict_8x8_init(
    mut cpu: uint32_t,
    mut pf: *mut x264_predict8x8_t,
    mut predict_filter: *mut x264_predict_8x8_filter_t,
) {
    let ref mut fresh287 = *pf.offset(I_PRED_8x8_V as libc::c_int as isize);
    *fresh287 = Some(
        x264_8_predict_8x8_v_c as unsafe extern "C" fn(*mut pixel, *mut pixel) -> (),
    );
    let ref mut fresh288 = *pf.offset(I_PRED_8x8_H as libc::c_int as isize);
    *fresh288 = Some(
        x264_8_predict_8x8_h_c as unsafe extern "C" fn(*mut pixel, *mut pixel) -> (),
    );
    let ref mut fresh289 = *pf.offset(I_PRED_8x8_DC as libc::c_int as isize);
    *fresh289 = Some(
        x264_8_predict_8x8_dc_c as unsafe extern "C" fn(*mut pixel, *mut pixel) -> (),
    );
    let ref mut fresh290 = *pf.offset(I_PRED_8x8_DDL as libc::c_int as isize);
    *fresh290 = Some(
        predict_8x8_ddl_c as unsafe extern "C" fn(*mut pixel, *mut pixel) -> (),
    );
    let ref mut fresh291 = *pf.offset(I_PRED_8x8_DDR as libc::c_int as isize);
    *fresh291 = Some(
        predict_8x8_ddr_c as unsafe extern "C" fn(*mut pixel, *mut pixel) -> (),
    );
    let ref mut fresh292 = *pf.offset(I_PRED_8x8_VR as libc::c_int as isize);
    *fresh292 = Some(
        predict_8x8_vr_c as unsafe extern "C" fn(*mut pixel, *mut pixel) -> (),
    );
    let ref mut fresh293 = *pf.offset(I_PRED_8x8_HD as libc::c_int as isize);
    *fresh293 = Some(
        predict_8x8_hd_c as unsafe extern "C" fn(*mut pixel, *mut pixel) -> (),
    );
    let ref mut fresh294 = *pf.offset(I_PRED_8x8_VL as libc::c_int as isize);
    *fresh294 = Some(
        predict_8x8_vl_c as unsafe extern "C" fn(*mut pixel, *mut pixel) -> (),
    );
    let ref mut fresh295 = *pf.offset(I_PRED_8x8_HU as libc::c_int as isize);
    *fresh295 = Some(
        predict_8x8_hu_c as unsafe extern "C" fn(*mut pixel, *mut pixel) -> (),
    );
    let ref mut fresh296 = *pf.offset(I_PRED_8x8_DC_LEFT as libc::c_int as isize);
    *fresh296 = Some(
        predict_8x8_dc_left_c as unsafe extern "C" fn(*mut pixel, *mut pixel) -> (),
    );
    let ref mut fresh297 = *pf.offset(I_PRED_8x8_DC_TOP as libc::c_int as isize);
    *fresh297 = Some(
        predict_8x8_dc_top_c as unsafe extern "C" fn(*mut pixel, *mut pixel) -> (),
    );
    let ref mut fresh298 = *pf.offset(I_PRED_8x8_DC_128 as libc::c_int as isize);
    *fresh298 = Some(
        predict_8x8_dc_128_c as unsafe extern "C" fn(*mut pixel, *mut pixel) -> (),
    );
    *predict_filter = Some(
        predict_8x8_filter_c
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_predict_4x4_init(
    mut cpu: uint32_t,
    mut pf: *mut x264_predict_t,
) {
    let ref mut fresh299 = *pf.offset(I_PRED_4x4_V as libc::c_int as isize);
    *fresh299 = Some(x264_8_predict_4x4_v_c as unsafe extern "C" fn(*mut pixel) -> ());
    let ref mut fresh300 = *pf.offset(I_PRED_4x4_H as libc::c_int as isize);
    *fresh300 = Some(x264_8_predict_4x4_h_c as unsafe extern "C" fn(*mut pixel) -> ());
    let ref mut fresh301 = *pf.offset(I_PRED_4x4_DC as libc::c_int as isize);
    *fresh301 = Some(x264_8_predict_4x4_dc_c as unsafe extern "C" fn(*mut pixel) -> ());
    let ref mut fresh302 = *pf.offset(I_PRED_4x4_DDL as libc::c_int as isize);
    *fresh302 = Some(predict_4x4_ddl_c as unsafe extern "C" fn(*mut pixel) -> ());
    let ref mut fresh303 = *pf.offset(I_PRED_4x4_DDR as libc::c_int as isize);
    *fresh303 = Some(predict_4x4_ddr_c as unsafe extern "C" fn(*mut pixel) -> ());
    let ref mut fresh304 = *pf.offset(I_PRED_4x4_VR as libc::c_int as isize);
    *fresh304 = Some(predict_4x4_vr_c as unsafe extern "C" fn(*mut pixel) -> ());
    let ref mut fresh305 = *pf.offset(I_PRED_4x4_HD as libc::c_int as isize);
    *fresh305 = Some(predict_4x4_hd_c as unsafe extern "C" fn(*mut pixel) -> ());
    let ref mut fresh306 = *pf.offset(I_PRED_4x4_VL as libc::c_int as isize);
    *fresh306 = Some(predict_4x4_vl_c as unsafe extern "C" fn(*mut pixel) -> ());
    let ref mut fresh307 = *pf.offset(I_PRED_4x4_HU as libc::c_int as isize);
    *fresh307 = Some(predict_4x4_hu_c as unsafe extern "C" fn(*mut pixel) -> ());
    let ref mut fresh308 = *pf.offset(I_PRED_4x4_DC_LEFT as libc::c_int as isize);
    *fresh308 = Some(predict_4x4_dc_left_c as unsafe extern "C" fn(*mut pixel) -> ());
    let ref mut fresh309 = *pf.offset(I_PRED_4x4_DC_TOP as libc::c_int as isize);
    *fresh309 = Some(predict_4x4_dc_top_c as unsafe extern "C" fn(*mut pixel) -> ());
    let ref mut fresh310 = *pf.offset(I_PRED_4x4_DC_128 as libc::c_int as isize);
    *fresh310 = Some(predict_4x4_dc_128_c as unsafe extern "C" fn(*mut pixel) -> ());
}
