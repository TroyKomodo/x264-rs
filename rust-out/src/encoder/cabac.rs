#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types, label_break_value)]
use crate::types::*;
extern "C" {
    fn abs(_: libc::c_int) -> libc::c_int;
    fn x264_8_cabac_encode_init_core(cb: *mut x264_cabac_t);
    fn x264_8_cabac_encode_decision_c(cb: *mut x264_cabac_t, i_ctx: libc::c_int, b: libc::c_int);
    fn x264_8_cabac_encode_bypass_c(cb: *mut x264_cabac_t, b: libc::c_int);
    fn x264_8_cabac_encode_terminal_c(cb: *mut x264_cabac_t);
    fn x264_8_cabac_encode_ue_bypass(
        cb: *mut x264_cabac_t,
        exp_bits: libc::c_int,
        val: libc::c_int,
    );
    fn x264_8_cabac_encode_flush(h: *mut x264_t, cb: *mut x264_cabac_t);
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn x264_8_mb_predict_mv(
        h: *mut x264_t,
        i_list: libc::c_int,
        idx: libc::c_int,
        i_width: libc::c_int,
        mvp: *mut int16_t,
    );
}
#[inline]
unsafe extern "C" fn bs_init(
    mut s: *mut bs_t,
    mut p_data: *mut libc::c_void,
    mut i_data: libc::c_int,
) {
    let mut offset: libc::c_int =
        (p_data as intptr_t & 3 as libc::c_int as intptr_t) as libc::c_int;
    (*s).p_start = (p_data as *mut uint8_t).offset(-(offset as isize));
    (*s).p = (*s).p_start;
    (*s).p_end = (p_data as *mut uint8_t).offset(i_data as isize);
    (*s).i_left = (::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
        .wrapping_sub(offset as libc::c_ulong)
        .wrapping_mul(8 as libc::c_int as libc::c_ulong) as libc::c_int;
    if offset != 0 {
        (*s).cur_bits = endian_fix32((*((*s).p as *mut x264_union32_t)).i) as uintptr_t;
        (*s).cur_bits >>= (4 as libc::c_int - offset) * 8 as libc::c_int;
    } else {
        (*s).cur_bits = 0 as libc::c_int as uintptr_t;
    };
}
#[inline]
unsafe extern "C" fn bs_flush(mut s: *mut bs_t) {
    (*((*s).p as *mut x264_union32_t)).i =
        endian_fix32(((*s).cur_bits << ((*s).i_left & 31 as libc::c_int)) as uint32_t);
    (*s).p = ((*s).p).offset(
        (::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
            .wrapping_sub(((*s).i_left >> 3 as libc::c_int) as libc::c_ulong) as isize,
    );
    (*s).i_left = (::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
        .wrapping_mul(8 as libc::c_int as libc::c_ulong) as libc::c_int;
}
#[inline]
unsafe extern "C" fn bs_write(mut s: *mut bs_t, mut i_count: libc::c_int, mut i_bits: uint32_t) {
    if ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
        == 8 as libc::c_int as libc::c_ulong
    {
        (*s).cur_bits = ((*s).cur_bits << i_count) | i_bits as uintptr_t;
        (*s).i_left -= i_count;
        if (*s).i_left <= 32 as libc::c_int {
            (*((*s).p as *mut x264_union32_t)).i =
                endian_fix((*s).cur_bits << (*s).i_left) as uint32_t;
            (*s).i_left += 32 as libc::c_int;
            (*s).p = ((*s).p).offset(4 as libc::c_int as isize);
        }
    } else if i_count < (*s).i_left {
        (*s).cur_bits = ((*s).cur_bits << i_count) | i_bits as uintptr_t;
        (*s).i_left -= i_count;
    } else {
        i_count -= (*s).i_left;
        (*s).cur_bits = ((*s).cur_bits << (*s).i_left) | (i_bits >> i_count) as uintptr_t;
        (*((*s).p as *mut x264_union32_t)).i = endian_fix((*s).cur_bits) as uint32_t;
        (*s).p = ((*s).p).offset(4 as libc::c_int as isize);
        (*s).cur_bits = i_bits as uintptr_t;
        (*s).i_left = 32 as libc::c_int - i_count;
    };
}
#[inline(always)]
unsafe extern "C" fn x264_cabac_pos(mut cb: *mut x264_cabac_t) -> libc::c_int {
    ((((*cb).p).offset_from((*cb).p_start) as libc::c_long
        + (*cb).i_bytes_outstanding as libc::c_long)
        * 8 as libc::c_int as libc::c_long
        + (*cb).i_queue as libc::c_long) as libc::c_int
}
#[inline(always)]
unsafe extern "C" fn endian_fix32(mut x: uint32_t) -> uint32_t {
    (x << 24 as libc::c_int)
        .wrapping_add((x << 8 as libc::c_int) & 0xff0000 as libc::c_int as uint32_t)
        .wrapping_add((x >> 8 as libc::c_int) & 0xff00 as libc::c_int as uint32_t)
        .wrapping_add(x >> 24 as libc::c_int)
}
#[inline(always)]
unsafe extern "C" fn endian_fix64(mut x: uint64_t) -> uint64_t {
    (endian_fix32((x >> 32 as libc::c_int) as uint32_t) as uint64_t)
        .wrapping_add((endian_fix32(x as uint32_t) as uint64_t) << 32 as libc::c_int)
}
#[inline(always)]
unsafe extern "C" fn endian_fix(mut x: uintptr_t) -> uintptr_t {
    if ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
        == 8 as libc::c_int as libc::c_ulong
    {
        endian_fix64(x)
    } else {
        endian_fix32(x as uint32_t) as uint64_t
    }
}
#[inline(always)]
unsafe extern "C" fn x264_ctz_4bit(mut x: uint32_t) -> libc::c_int {
    static mut lut: [uint8_t; 16] = [
        4 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        1 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        2 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        1 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        3 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        1 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        2 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        1 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
    ];
    lut[x as usize] as libc::c_int
}
#[inline(always)]
unsafe extern "C" fn x264_cabac_mvd_sum(
    mut mvdleft: *mut uint8_t,
    mut mvdtop: *mut uint8_t,
) -> uint16_t {
    let mut amvd0: libc::c_int = *mvdleft.offset(0 as libc::c_int as isize) as libc::c_int
        + *mvdtop.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut amvd1: libc::c_int = *mvdleft.offset(1 as libc::c_int as isize) as libc::c_int
        + *mvdtop.offset(1 as libc::c_int as isize) as libc::c_int;
    amvd0 = (amvd0 > 2 as libc::c_int) as libc::c_int + (amvd0 > 32 as libc::c_int) as libc::c_int;
    amvd1 = (amvd1 > 2 as libc::c_int) as libc::c_int + (amvd1 > 32 as libc::c_int) as libc::c_int;
    (amvd0 + (amvd1 << 8 as libc::c_int)) as uint16_t
}
#[inline(always)]
unsafe extern "C" fn pack8to16(mut a: uint32_t, mut b: uint32_t) -> uint32_t {
    a.wrapping_add(b << 8 as libc::c_int)
}
#[inline(always)]
unsafe extern "C" fn x264_mb_predict_intra4x4_mode(
    mut h: *mut x264_t,
    mut idx: libc::c_int,
) -> libc::c_int {
    let ma: libc::c_int = (*h).mb.cache.intra4x4_pred_mode
        [(x264_scan8[idx as usize] as libc::c_int - 1 as libc::c_int) as usize]
        as libc::c_int;
    let mb: libc::c_int = (*h).mb.cache.intra4x4_pred_mode
        [(x264_scan8[idx as usize] as libc::c_int - 8 as libc::c_int) as usize]
        as libc::c_int;
    let m: libc::c_int = if (x264_mb_pred_mode4x4_fix[(ma + 1 as libc::c_int) as usize]
        as libc::c_int)
        < x264_mb_pred_mode4x4_fix[(mb + 1 as libc::c_int) as usize] as libc::c_int
    {
        x264_mb_pred_mode4x4_fix[(ma + 1 as libc::c_int) as usize] as libc::c_int
    } else {
        x264_mb_pred_mode4x4_fix[(mb + 1 as libc::c_int) as usize] as libc::c_int
    };
    if m < 0 as libc::c_int {
        return I_PRED_4x4_DC as libc::c_int;
    }
    m
}
#[inline(always)]
unsafe extern "C" fn x264_mb_transform_8x8_allowed(mut h: *mut x264_t) -> libc::c_int {
    if (*((*h).pps).as_mut_ptr()).b_transform_8x8_mode == 0 {
        return 0 as libc::c_int;
    }
    if (*h).mb.i_type != P_8x8 as libc::c_int {
        return x264_transform_allowed[(*h).mb.i_type as usize] as libc::c_int;
    }
    ((*(((*h).mb.i_sub_partition).as_mut_ptr() as *mut x264_union32_t)).i
        == (D_L0_8x8 as libc::c_int * 0x1010101 as libc::c_int) as uint32_t) as libc::c_int
}
#[inline(always)]
unsafe extern "C" fn x264_macroblock_cache_rect(
    mut dst: *mut libc::c_void,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut s: libc::c_int,
    mut v: uint32_t,
) {
    let mut d: *mut uint8_t = dst as *mut uint8_t;
    let mut v2: uint16_t = (if s >= 2 as libc::c_int {
        v
    } else {
        v * 0x101 as libc::c_int as uint32_t
    }) as uint16_t;
    let mut v4: uint32_t = if s >= 4 as libc::c_int {
        v
    } else if s >= 2 as libc::c_int {
        v * 0x10001 as libc::c_int as uint32_t
    } else {
        v * 0x1010101 as libc::c_int as uint32_t
    };
    let mut v8: uint64_t = (v4 as uint64_t).wrapping_add((v4 as uint64_t) << 32 as libc::c_int);
    s *= 8 as libc::c_int;
    if w == 2 as libc::c_int {
        (*(d.offset((s * 0 as libc::c_int) as isize) as *mut x264_union16_t)).i = v2;
        if h == 1 as libc::c_int {
            return;
        }
        (*(d.offset((s * 1 as libc::c_int) as isize) as *mut x264_union16_t)).i = v2;
        if h == 2 as libc::c_int {
            return;
        }
        (*(d.offset((s * 2 as libc::c_int) as isize) as *mut x264_union16_t)).i = v2;
        (*(d.offset((s * 3 as libc::c_int) as isize) as *mut x264_union16_t)).i = v2;
    } else if w == 4 as libc::c_int {
        (*(d.offset((s * 0 as libc::c_int) as isize) as *mut x264_union32_t)).i = v4;
        if h == 1 as libc::c_int {
            return;
        }
        (*(d.offset((s * 1 as libc::c_int) as isize) as *mut x264_union32_t)).i = v4;
        if h == 2 as libc::c_int {
            return;
        }
        (*(d.offset((s * 2 as libc::c_int) as isize) as *mut x264_union32_t)).i = v4;
        (*(d.offset((s * 3 as libc::c_int) as isize) as *mut x264_union32_t)).i = v4;
    } else if w == 8 as libc::c_int {
        if ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
            == 8 as libc::c_int as libc::c_ulong
        {
            (*(d.offset((s * 0 as libc::c_int) as isize) as *mut x264_union64_t)).i = v8;
            if h == 1 as libc::c_int {
                return;
            }
            (*(d.offset((s * 1 as libc::c_int) as isize) as *mut x264_union64_t)).i = v8;
            if h == 2 as libc::c_int {
                return;
            }
            (*(d.offset((s * 2 as libc::c_int) as isize) as *mut x264_union64_t)).i = v8;
            (*(d.offset((s * 3 as libc::c_int) as isize) as *mut x264_union64_t)).i = v8;
        } else {
            (*(d.offset((s * 0 as libc::c_int) as isize)
                .offset(0 as libc::c_int as isize) as *mut x264_union32_t))
                .i = v4;
            (*(d.offset((s * 0 as libc::c_int) as isize)
                .offset(4 as libc::c_int as isize) as *mut x264_union32_t))
                .i = v4;
            if h == 1 as libc::c_int {
                return;
            }
            (*(d.offset((s * 1 as libc::c_int) as isize)
                .offset(0 as libc::c_int as isize) as *mut x264_union32_t))
                .i = v4;
            (*(d.offset((s * 1 as libc::c_int) as isize)
                .offset(4 as libc::c_int as isize) as *mut x264_union32_t))
                .i = v4;
            if h == 2 as libc::c_int {
                return;
            }
            (*(d.offset((s * 2 as libc::c_int) as isize)
                .offset(0 as libc::c_int as isize) as *mut x264_union32_t))
                .i = v4;
            (*(d.offset((s * 2 as libc::c_int) as isize)
                .offset(4 as libc::c_int as isize) as *mut x264_union32_t))
                .i = v4;
            (*(d.offset((s * 3 as libc::c_int) as isize)
                .offset(0 as libc::c_int as isize) as *mut x264_union32_t))
                .i = v4;
            (*(d.offset((s * 3 as libc::c_int) as isize)
                .offset(4 as libc::c_int as isize) as *mut x264_union32_t))
                .i = v4;
        }
    } else if w == 16 as libc::c_int {
        if h != 1 as libc::c_int {
        } else {
            __assert_fail(
                b"h != 1\0" as *const u8 as *const libc::c_char,
                b"./common/rectangle.h\0" as *const u8 as *const libc::c_char,
                82 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 65], &[libc::c_char; 65]>(
                    b"void x264_macroblock_cache_rect(void *, int, int, int, uint32_t)\0",
                ))
                .as_ptr(),
            );
        }
        'c_27091: {
            if h != 1 as libc::c_int {
            } else {
                __assert_fail(
                    b"h != 1\0" as *const u8 as *const libc::c_char,
                    b"./common/rectangle.h\0" as *const u8 as *const libc::c_char,
                    82 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<&[u8; 65], &[libc::c_char; 65]>(
                        b"void x264_macroblock_cache_rect(void *, int, int, int, uint32_t)\0",
                    ))
                    .as_ptr(),
                );
            }
        };
        if ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
            == 8 as libc::c_int as libc::c_ulong
        {
            loop {
                (*(d.offset((s * 0 as libc::c_int) as isize)
                    .offset(0 as libc::c_int as isize)
                    as *mut x264_union64_t))
                    .i = v8;
                (*(d.offset((s * 0 as libc::c_int) as isize)
                    .offset(8 as libc::c_int as isize)
                    as *mut x264_union64_t))
                    .i = v8;
                (*(d.offset((s * 1 as libc::c_int) as isize)
                    .offset(0 as libc::c_int as isize)
                    as *mut x264_union64_t))
                    .i = v8;
                (*(d.offset((s * 1 as libc::c_int) as isize)
                    .offset(8 as libc::c_int as isize)
                    as *mut x264_union64_t))
                    .i = v8;
                h -= 2 as libc::c_int;
                d = d.offset((s * 2 as libc::c_int) as isize);
                if h == 0 {
                    break;
                }
            }
        } else {
            loop {
                (*(d.offset(0 as libc::c_int as isize) as *mut x264_union32_t)).i = v4;
                (*(d.offset(4 as libc::c_int as isize) as *mut x264_union32_t)).i = v4;
                (*(d.offset(8 as libc::c_int as isize) as *mut x264_union32_t)).i = v4;
                (*(d.offset(12 as libc::c_int as isize) as *mut x264_union32_t)).i = v4;
                d = d.offset(s as isize);
                h -= 1;
                if h == 0 {
                    break;
                }
            }
        }
    } else {
        __assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"./common/rectangle.h\0" as *const u8 as *const libc::c_char,
            118 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 65], &[libc::c_char; 65]>(
                b"void x264_macroblock_cache_rect(void *, int, int, int, uint32_t)\0",
            ))
            .as_ptr(),
        );
        'c_26858: {
            __assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"./common/rectangle.h\0" as *const u8 as *const libc::c_char,
                118 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 65], &[libc::c_char; 65]>(
                    b"void x264_macroblock_cache_rect(void *, int, int, int, uint32_t)\0",
                ))
                .as_ptr(),
            );
        };
    };
}
#[inline(always)]
unsafe extern "C" fn x264_macroblock_cache_mvd(
    mut h: *mut x264_t,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut i_list: libc::c_int,
    mut mvd: uint16_t,
) {
    let mut mvd_cache: *mut libc::c_void =
        &mut *(*((*h).mb.cache.mvd).as_mut_ptr().offset(i_list as isize))
            .as_mut_ptr()
            .offset(
                (4 as libc::c_int + 1 as libc::c_int * 8 as libc::c_int + x + 8 as libc::c_int * y)
                    as isize,
            ) as *mut [uint8_t; 2] as *mut libc::c_void;
    if 0 == 0 || 0 == 0 {
        (x264_8_cache_mvd_func_table
            [(width + (height << 1 as libc::c_int) - 3 as libc::c_int) as usize])
            .expect("non-null function pointer")(mvd_cache, mvd as uint32_t);
    } else {
        x264_macroblock_cache_rect(
            mvd_cache,
            width * 2 as libc::c_int,
            height,
            2 as libc::c_int,
            mvd as uint32_t,
        );
    };
}
#[inline]
unsafe extern "C" fn cabac_mb_type_intra(
    mut h: *mut x264_t,
    mut cb: *mut x264_cabac_t,
    mut i_mb_type: libc::c_int,
    mut ctx0: libc::c_int,
    mut ctx1: libc::c_int,
    mut ctx2: libc::c_int,
    mut ctx3: libc::c_int,
    mut ctx4: libc::c_int,
    mut ctx5: libc::c_int,
) {
    if i_mb_type == I_4x4 as libc::c_int || i_mb_type == I_8x8 as libc::c_int {
        x264_8_cabac_encode_decision_c(cb, ctx0, 0 as libc::c_int);
    } else if i_mb_type == I_PCM as libc::c_int {
        x264_8_cabac_encode_decision_c(cb, ctx0, 1 as libc::c_int);
        x264_8_cabac_encode_flush(h, cb);
    } else {
        let mut i_pred: libc::c_int =
            x264_mb_pred_mode16x16_fix[(*h).mb.i_intra16x16_pred_mode as usize] as libc::c_int;
        x264_8_cabac_encode_decision_c(cb, ctx0, 1 as libc::c_int);
        x264_8_cabac_encode_terminal_c(cb);
        x264_8_cabac_encode_decision_c(cb, ctx1, ((*h).mb.i_cbp_luma != 0) as libc::c_int);
        if (*h).mb.i_cbp_chroma == 0 as libc::c_int {
            x264_8_cabac_encode_decision_c(cb, ctx2, 0 as libc::c_int);
        } else {
            x264_8_cabac_encode_decision_c(cb, ctx2, 1 as libc::c_int);
            x264_8_cabac_encode_decision_c(cb, ctx3, (*h).mb.i_cbp_chroma >> 1 as libc::c_int);
        }
        x264_8_cabac_encode_decision_c(cb, ctx4, i_pred >> 1 as libc::c_int);
        x264_8_cabac_encode_decision_c(cb, ctx5, i_pred & 1 as libc::c_int);
    };
}
unsafe extern "C" fn cabac_field_decoding_flag(mut h: *mut x264_t, mut cb: *mut x264_cabac_t) {
    let mut ctx: libc::c_int = 0 as libc::c_int;
    ctx += (*h).mb.field_decoding_flag & ((*h).mb.i_mb_x != 0) as libc::c_int;
    ctx += ((*h).mb.i_mb_top_mbpair_xy >= 0 as libc::c_int
        && *((*h).mb.slice_table).offset((*h).mb.i_mb_top_mbpair_xy as isize) == (*h).sh.i_first_mb
        && *((*h).mb.field).offset((*h).mb.i_mb_top_mbpair_xy as isize) as libc::c_int != 0)
        as libc::c_int;
    x264_8_cabac_encode_decision_c(cb, 70 as libc::c_int + ctx, (*h).mb.b_interlaced);
    (*h).mb.field_decoding_flag = (*h).mb.b_interlaced;
}
unsafe extern "C" fn cabac_intra4x4_pred_mode(
    mut cb: *mut x264_cabac_t,
    mut i_pred: libc::c_int,
    mut i_mode: libc::c_int,
) {
    if i_pred == i_mode {
        x264_8_cabac_encode_decision_c(cb, 68 as libc::c_int, 1 as libc::c_int);
    } else {
        x264_8_cabac_encode_decision_c(cb, 68 as libc::c_int, 0 as libc::c_int);
        if i_mode > i_pred {
            i_mode -= 1;
            i_mode;
        }
        x264_8_cabac_encode_decision_c(cb, 69 as libc::c_int, i_mode & 0x1 as libc::c_int);
        x264_8_cabac_encode_decision_c(
            cb,
            69 as libc::c_int,
            (i_mode >> 1 as libc::c_int) & 0x1 as libc::c_int,
        );
        x264_8_cabac_encode_decision_c(cb, 69 as libc::c_int, i_mode >> 2 as libc::c_int);
    };
}
unsafe extern "C" fn cabac_intra_chroma_pred_mode(mut h: *mut x264_t, mut cb: *mut x264_cabac_t) {
    let mut i_mode: libc::c_int =
        x264_mb_chroma_pred_mode_fix[(*h).mb.i_chroma_pred_mode as usize] as libc::c_int;
    let mut ctx: libc::c_int = 0 as libc::c_int;
    if (*h).mb.i_neighbour & MB_LEFT as libc::c_int as libc::c_uint != 0
        && *((*h).mb.chroma_pred_mode)
            .offset((*h).mb.i_mb_left_xy[0 as libc::c_int as usize] as isize)
            as libc::c_int
            != 0 as libc::c_int
    {
        ctx += 1;
        ctx;
    }
    if (*h).mb.i_neighbour & MB_TOP as libc::c_int as libc::c_uint != 0
        && *((*h).mb.chroma_pred_mode).offset((*h).mb.i_mb_top_xy as isize) as libc::c_int
            != 0 as libc::c_int
    {
        ctx += 1;
        ctx;
    }
    x264_8_cabac_encode_decision_c(
        cb,
        64 as libc::c_int + ctx,
        (i_mode > 0 as libc::c_int) as libc::c_int,
    );
    if i_mode > 0 as libc::c_int {
        x264_8_cabac_encode_decision_c(
            cb,
            64 as libc::c_int + 3 as libc::c_int,
            (i_mode > 1 as libc::c_int) as libc::c_int,
        );
        if i_mode > 1 as libc::c_int {
            x264_8_cabac_encode_decision_c(
                cb,
                64 as libc::c_int + 3 as libc::c_int,
                (i_mode > 2 as libc::c_int) as libc::c_int,
            );
        }
    }
}
unsafe extern "C" fn cabac_cbp_luma(mut h: *mut x264_t, mut cb: *mut x264_cabac_t) {
    let mut cbp: libc::c_int = (*h).mb.i_cbp_luma;
    let mut cbp_l: libc::c_int = (*h).mb.cache.i_cbp_left;
    let mut cbp_t: libc::c_int = (*h).mb.cache.i_cbp_top;
    x264_8_cabac_encode_decision_c(
        cb,
        76 as libc::c_int
            - ((cbp_l >> 1 as libc::c_int) & 1 as libc::c_int)
            - ((cbp_t >> 1 as libc::c_int) & 2 as libc::c_int),
        (cbp >> 0 as libc::c_int) & 1 as libc::c_int,
    );
    x264_8_cabac_encode_decision_c(
        cb,
        76 as libc::c_int
            - ((cbp >> 0 as libc::c_int) & 1 as libc::c_int)
            - ((cbp_t >> 2 as libc::c_int) & 2 as libc::c_int),
        (cbp >> 1 as libc::c_int) & 1 as libc::c_int,
    );
    x264_8_cabac_encode_decision_c(
        cb,
        76 as libc::c_int
            - ((cbp_l >> 3 as libc::c_int) & 1 as libc::c_int)
            - ((cbp << 1 as libc::c_int) & 2 as libc::c_int),
        (cbp >> 2 as libc::c_int) & 1 as libc::c_int,
    );
    x264_8_cabac_encode_decision_c(
        cb,
        76 as libc::c_int
            - ((cbp >> 2 as libc::c_int) & 1 as libc::c_int)
            - ((cbp >> 0 as libc::c_int) & 2 as libc::c_int),
        (cbp >> 3 as libc::c_int) & 1 as libc::c_int,
    );
}
unsafe extern "C" fn cabac_cbp_chroma(mut h: *mut x264_t, mut cb: *mut x264_cabac_t) {
    let mut cbp_a: libc::c_int = (*h).mb.cache.i_cbp_left & 0x30 as libc::c_int;
    let mut cbp_b: libc::c_int = (*h).mb.cache.i_cbp_top & 0x30 as libc::c_int;
    let mut ctx: libc::c_int = 0 as libc::c_int;
    if cbp_a != 0 && (*h).mb.cache.i_cbp_left != -(1 as libc::c_int) {
        ctx += 1;
        ctx;
    }
    if cbp_b != 0 && (*h).mb.cache.i_cbp_top != -(1 as libc::c_int) {
        ctx += 2 as libc::c_int;
    }
    if (*h).mb.i_cbp_chroma == 0 as libc::c_int {
        x264_8_cabac_encode_decision_c(cb, 77 as libc::c_int + ctx, 0 as libc::c_int);
    } else {
        x264_8_cabac_encode_decision_c(cb, 77 as libc::c_int + ctx, 1 as libc::c_int);
        ctx = 4 as libc::c_int;
        if cbp_a == 0x20 as libc::c_int {
            ctx += 1;
            ctx;
        }
        if cbp_b == 0x20 as libc::c_int {
            ctx += 2 as libc::c_int;
        }
        x264_8_cabac_encode_decision_c(
            cb,
            77 as libc::c_int + ctx,
            (*h).mb.i_cbp_chroma >> 1 as libc::c_int,
        );
    };
}
unsafe extern "C" fn cabac_qp_delta(mut h: *mut x264_t, mut cb: *mut x264_cabac_t) {
    let mut i_dqp: libc::c_int = (*h).mb.i_qp - (*h).mb.i_last_qp;
    let mut ctx: libc::c_int = 0;
    if (*h).mb.i_type == I_16x16 as libc::c_int
        && *((*h).mb.cbp).offset((*h).mb.i_mb_xy as isize) == 0
        && (*h).mb.i_qp > (*h).mb.i_last_qp
    {
        (*h).mb.i_qp = (*h).mb.i_last_qp;
        i_dqp = 0 as libc::c_int;
    }
    ctx = ((*h).mb.i_last_dqp != 0
        && (*((*h).mb.type_0).offset((*h).mb.i_mb_prev_xy as isize) as libc::c_int
            == I_16x16 as libc::c_int
            || *((*h).mb.cbp).offset((*h).mb.i_mb_prev_xy as isize) as libc::c_int
                & 0x3f as libc::c_int
                != 0)) as libc::c_int;
    if i_dqp != 0 as libc::c_int {
        i_dqp *= 2 as libc::c_int;
        let mut val: libc::c_int = 1 as libc::c_int - i_dqp;
        if val < 0 as libc::c_int {
            val = i_dqp;
        }
        val -= 1;
        val;
        if val >= 51 as libc::c_int + 6 as libc::c_int * (8 as libc::c_int - 8 as libc::c_int)
            && val
                != 51 as libc::c_int
                    + 6 as libc::c_int * (8 as libc::c_int - 8 as libc::c_int)
                    + 1 as libc::c_int
        {
            val = 2 as libc::c_int
                * (51 as libc::c_int + 6 as libc::c_int * (8 as libc::c_int - 8 as libc::c_int))
                + 1 as libc::c_int
                - val;
        }
        loop {
            x264_8_cabac_encode_decision_c(cb, 60 as libc::c_int + ctx, 1 as libc::c_int);
            ctx = 2 as libc::c_int + (ctx >> 1 as libc::c_int);
            val -= 1;
            if val == 0 {
                break;
            }
        }
    }
    x264_8_cabac_encode_decision_c(cb, 60 as libc::c_int + ctx, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_cabac_mb_skip(mut h: *mut x264_t, mut b_skip: libc::c_int) {
    let mut ctx: libc::c_int = (*h).mb.cache.i_neighbour_skip + 11 as libc::c_int;
    if (*h).sh.i_type != SLICE_TYPE_P as libc::c_int {
        ctx += 13 as libc::c_int;
    }
    x264_8_cabac_encode_decision_c(&mut (*h).cabac, ctx, b_skip);
}
#[inline]
unsafe extern "C" fn cabac_subpartition_p(mut cb: *mut x264_cabac_t, mut i_sub: libc::c_int) {
    if i_sub == D_L0_8x8 as libc::c_int {
        x264_8_cabac_encode_decision_c(cb, 21 as libc::c_int, 1 as libc::c_int);
        return;
    }
    x264_8_cabac_encode_decision_c(cb, 21 as libc::c_int, 0 as libc::c_int);
    if i_sub == D_L0_8x4 as libc::c_int {
        x264_8_cabac_encode_decision_c(cb, 22 as libc::c_int, 0 as libc::c_int);
    } else {
        x264_8_cabac_encode_decision_c(cb, 22 as libc::c_int, 1 as libc::c_int);
        x264_8_cabac_encode_decision_c(
            cb,
            23 as libc::c_int,
            (i_sub == D_L0_4x8 as libc::c_int) as libc::c_int,
        );
    };
}
#[inline(always)]
unsafe extern "C" fn cabac_subpartition_b(mut cb: *mut x264_cabac_t, mut i_sub: libc::c_int) {
    if i_sub == D_DIRECT_8x8 as libc::c_int {
        x264_8_cabac_encode_decision_c(cb, 36 as libc::c_int, 0 as libc::c_int);
        return;
    }
    x264_8_cabac_encode_decision_c(cb, 36 as libc::c_int, 1 as libc::c_int);
    if i_sub == D_BI_8x8 as libc::c_int {
        x264_8_cabac_encode_decision_c(cb, 37 as libc::c_int, 1 as libc::c_int);
        x264_8_cabac_encode_decision_c(cb, 38 as libc::c_int, 0 as libc::c_int);
        x264_8_cabac_encode_decision_c(cb, 39 as libc::c_int, 0 as libc::c_int);
        x264_8_cabac_encode_decision_c(cb, 39 as libc::c_int, 0 as libc::c_int);
        return;
    }
    x264_8_cabac_encode_decision_c(cb, 37 as libc::c_int, 0 as libc::c_int);
    x264_8_cabac_encode_decision_c(
        cb,
        39 as libc::c_int,
        (i_sub == D_L1_8x8 as libc::c_int) as libc::c_int,
    );
}
#[inline(always)]
unsafe extern "C" fn cabac_transform_size(mut h: *mut x264_t, mut cb: *mut x264_cabac_t) {
    let mut ctx: libc::c_int = 399 as libc::c_int + (*h).mb.cache.i_neighbour_transform_size;
    x264_8_cabac_encode_decision_c(cb, ctx, (*h).mb.b_transform_8x8);
}
#[inline(always)]
unsafe extern "C" fn cabac_ref_internal(
    mut h: *mut x264_t,
    mut cb: *mut x264_cabac_t,
    mut i_list: libc::c_int,
    mut idx: libc::c_int,
    mut bframe: libc::c_int,
) {
    let i8: libc::c_int = x264_scan8[idx as usize] as libc::c_int;
    let i_refa: libc::c_int =
        (*h).mb.cache.ref_0[i_list as usize][(i8 - 1 as libc::c_int) as usize] as libc::c_int;
    let i_refb: libc::c_int =
        (*h).mb.cache.ref_0[i_list as usize][(i8 - 8 as libc::c_int) as usize] as libc::c_int;
    let mut ctx: libc::c_int = 0 as libc::c_int;
    if i_refa > 0 as libc::c_int
        && (bframe == 0 || (*h).mb.cache.skip[(i8 - 1 as libc::c_int) as usize] == 0)
    {
        ctx += 1;
        ctx;
    }
    if i_refb > 0 as libc::c_int
        && (bframe == 0 || (*h).mb.cache.skip[(i8 - 8 as libc::c_int) as usize] == 0)
    {
        ctx += 2 as libc::c_int;
    }
    let mut i_ref: libc::c_int = (*h).mb.cache.ref_0[i_list as usize][i8 as usize] as libc::c_int;
    while i_ref > 0 as libc::c_int {
        x264_8_cabac_encode_decision_c(cb, 54 as libc::c_int + ctx, 1 as libc::c_int);
        ctx = (ctx >> 2 as libc::c_int) + 4 as libc::c_int;
        i_ref -= 1;
        i_ref;
    }
    x264_8_cabac_encode_decision_c(cb, 54 as libc::c_int + ctx, 0 as libc::c_int);
}
#[inline(never)]
unsafe extern "C" fn cabac_ref_p(
    mut h: *mut x264_t,
    mut cb: *mut x264_cabac_t,
    mut idx: libc::c_int,
) {
    cabac_ref_internal(h, cb, 0 as libc::c_int, idx, 0 as libc::c_int);
}
#[inline(never)]
unsafe extern "C" fn cabac_ref_b(
    mut h: *mut x264_t,
    mut cb: *mut x264_cabac_t,
    mut i_list: libc::c_int,
    mut idx: libc::c_int,
) {
    cabac_ref_internal(h, cb, i_list, idx, 1 as libc::c_int);
}
#[inline(always)]
unsafe extern "C" fn cabac_mvd_cpn(
    mut h: *mut x264_t,
    mut cb: *mut x264_cabac_t,
    mut i_list: libc::c_int,
    mut idx: libc::c_int,
    mut l: libc::c_int,
    mut mvd: libc::c_int,
    mut ctx: libc::c_int,
) -> libc::c_int {
    let mut ctxbase: libc::c_int = if l != 0 {
        47 as libc::c_int
    } else {
        40 as libc::c_int
    };
    if mvd == 0 as libc::c_int {
        x264_8_cabac_encode_decision_c(cb, ctxbase + ctx, 0 as libc::c_int);
        return 0 as libc::c_int;
    }
    let mut i_abs: libc::c_int = abs(mvd);
    x264_8_cabac_encode_decision_c(cb, ctxbase + ctx, 1 as libc::c_int);
    static mut ctxes: [uint8_t; 8] = [
        3 as libc::c_int as uint8_t,
        4 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
    ];
    if i_abs < 9 as libc::c_int {
        let mut i: libc::c_int = 1 as libc::c_int;
        while i < i_abs {
            x264_8_cabac_encode_decision_c(
                cb,
                ctxbase + ctxes[(i - 1 as libc::c_int) as usize] as libc::c_int,
                1 as libc::c_int,
            );
            i += 1;
            i;
        }
        x264_8_cabac_encode_decision_c(
            cb,
            ctxbase + ctxes[(i_abs - 1 as libc::c_int) as usize] as libc::c_int,
            0 as libc::c_int,
        );
    } else {
        let mut i_0: libc::c_int = 1 as libc::c_int;
        while i_0 < 9 as libc::c_int {
            x264_8_cabac_encode_decision_c(
                cb,
                ctxbase + ctxes[(i_0 - 1 as libc::c_int) as usize] as libc::c_int,
                1 as libc::c_int,
            );
            i_0 += 1;
            i_0;
        }
        x264_8_cabac_encode_ue_bypass(cb, 3 as libc::c_int, i_abs - 9 as libc::c_int);
    }
    x264_8_cabac_encode_bypass_c(cb, mvd >> 31 as libc::c_int);
    if i_abs < 66 as libc::c_int {
        i_abs
    } else {
        66 as libc::c_int
    }
}
#[inline(never)]
unsafe extern "C" fn cabac_mvd(
    mut h: *mut x264_t,
    mut cb: *mut x264_cabac_t,
    mut i_list: libc::c_int,
    mut idx: libc::c_int,
    mut width: libc::c_int,
) -> uint16_t {
    let mut mvp: [int16_t; 2] = [0; 2];
    let mut mdx: libc::c_int = 0;
    let mut mdy: libc::c_int = 0;
    x264_8_mb_predict_mv(h, i_list, idx, width, mvp.as_mut_ptr());
    mdx = (*h).mb.cache.mv[i_list as usize][x264_scan8[idx as usize] as usize]
        [0 as libc::c_int as usize] as libc::c_int
        - mvp[0 as libc::c_int as usize] as libc::c_int;
    mdy = (*h).mb.cache.mv[i_list as usize][x264_scan8[idx as usize] as usize]
        [1 as libc::c_int as usize] as libc::c_int
        - mvp[1 as libc::c_int as usize] as libc::c_int;
    let mut amvd: uint16_t = x264_cabac_mvd_sum(
        ((*h).mb.cache.mvd[i_list as usize]
            [(x264_scan8[idx as usize] as libc::c_int - 1 as libc::c_int) as usize])
            .as_mut_ptr(),
        ((*h).mb.cache.mvd[i_list as usize]
            [(x264_scan8[idx as usize] as libc::c_int - 8 as libc::c_int) as usize])
            .as_mut_ptr(),
    );
    mdx = cabac_mvd_cpn(
        h,
        cb,
        i_list,
        idx,
        0 as libc::c_int,
        mdx,
        amvd as libc::c_int & 0xff as libc::c_int,
    );
    mdy = cabac_mvd_cpn(
        h,
        cb,
        i_list,
        idx,
        1 as libc::c_int,
        mdy,
        amvd as libc::c_int >> 8 as libc::c_int,
    );
    pack8to16(mdx as uint32_t, mdy as uint32_t) as uint16_t
}
#[inline]
unsafe extern "C" fn cabac_8x8_mvd(
    mut h: *mut x264_t,
    mut cb: *mut x264_cabac_t,
    mut i: libc::c_int,
) {
    match (*h).mb.i_sub_partition[i as usize] as libc::c_int {
        3 => {
            let mut mvd: uint16_t = cabac_mvd(
                h,
                cb,
                0 as libc::c_int,
                4 as libc::c_int * i,
                2 as libc::c_int,
            );
            x264_macroblock_cache_mvd(
                h,
                block_idx_x[(4 as libc::c_int * i) as usize] as libc::c_int,
                block_idx_y[(4 as libc::c_int * i) as usize] as libc::c_int,
                2 as libc::c_int,
                2 as libc::c_int,
                0 as libc::c_int,
                mvd,
            );
        }
        1 => {
            let mut mvd_0: uint16_t = cabac_mvd(
                h,
                cb,
                0 as libc::c_int,
                4 as libc::c_int * i + 0 as libc::c_int,
                2 as libc::c_int,
            );
            x264_macroblock_cache_mvd(
                h,
                block_idx_x[(4 as libc::c_int * i + 0 as libc::c_int) as usize] as libc::c_int,
                block_idx_y[(4 as libc::c_int * i + 0 as libc::c_int) as usize] as libc::c_int,
                2 as libc::c_int,
                1 as libc::c_int,
                0 as libc::c_int,
                mvd_0,
            );
            let mut mvd_1: uint16_t = cabac_mvd(
                h,
                cb,
                0 as libc::c_int,
                4 as libc::c_int * i + 2 as libc::c_int,
                2 as libc::c_int,
            );
            x264_macroblock_cache_mvd(
                h,
                block_idx_x[(4 as libc::c_int * i + 2 as libc::c_int) as usize] as libc::c_int,
                block_idx_y[(4 as libc::c_int * i + 2 as libc::c_int) as usize] as libc::c_int,
                2 as libc::c_int,
                1 as libc::c_int,
                0 as libc::c_int,
                mvd_1,
            );
        }
        2 => {
            let mut mvd_2: uint16_t = cabac_mvd(
                h,
                cb,
                0 as libc::c_int,
                4 as libc::c_int * i + 0 as libc::c_int,
                1 as libc::c_int,
            );
            x264_macroblock_cache_mvd(
                h,
                block_idx_x[(4 as libc::c_int * i + 0 as libc::c_int) as usize] as libc::c_int,
                block_idx_y[(4 as libc::c_int * i + 0 as libc::c_int) as usize] as libc::c_int,
                1 as libc::c_int,
                2 as libc::c_int,
                0 as libc::c_int,
                mvd_2,
            );
            let mut mvd_3: uint16_t = cabac_mvd(
                h,
                cb,
                0 as libc::c_int,
                4 as libc::c_int * i + 1 as libc::c_int,
                1 as libc::c_int,
            );
            x264_macroblock_cache_mvd(
                h,
                block_idx_x[(4 as libc::c_int * i + 1 as libc::c_int) as usize] as libc::c_int,
                block_idx_y[(4 as libc::c_int * i + 1 as libc::c_int) as usize] as libc::c_int,
                1 as libc::c_int,
                2 as libc::c_int,
                0 as libc::c_int,
                mvd_3,
            );
        }
        0 => {
            let mut mvd_4: uint16_t = cabac_mvd(
                h,
                cb,
                0 as libc::c_int,
                4 as libc::c_int * i + 0 as libc::c_int,
                1 as libc::c_int,
            );
            x264_macroblock_cache_mvd(
                h,
                block_idx_x[(4 as libc::c_int * i + 0 as libc::c_int) as usize] as libc::c_int,
                block_idx_y[(4 as libc::c_int * i + 0 as libc::c_int) as usize] as libc::c_int,
                1 as libc::c_int,
                1 as libc::c_int,
                0 as libc::c_int,
                mvd_4,
            );
            let mut mvd_5: uint16_t = cabac_mvd(
                h,
                cb,
                0 as libc::c_int,
                4 as libc::c_int * i + 1 as libc::c_int,
                1 as libc::c_int,
            );
            x264_macroblock_cache_mvd(
                h,
                block_idx_x[(4 as libc::c_int * i + 1 as libc::c_int) as usize] as libc::c_int,
                block_idx_y[(4 as libc::c_int * i + 1 as libc::c_int) as usize] as libc::c_int,
                1 as libc::c_int,
                1 as libc::c_int,
                0 as libc::c_int,
                mvd_5,
            );
            let mut mvd_6: uint16_t = cabac_mvd(
                h,
                cb,
                0 as libc::c_int,
                4 as libc::c_int * i + 2 as libc::c_int,
                1 as libc::c_int,
            );
            x264_macroblock_cache_mvd(
                h,
                block_idx_x[(4 as libc::c_int * i + 2 as libc::c_int) as usize] as libc::c_int,
                block_idx_y[(4 as libc::c_int * i + 2 as libc::c_int) as usize] as libc::c_int,
                1 as libc::c_int,
                1 as libc::c_int,
                0 as libc::c_int,
                mvd_6,
            );
            let mut mvd_7: uint16_t = cabac_mvd(
                h,
                cb,
                0 as libc::c_int,
                4 as libc::c_int * i + 3 as libc::c_int,
                1 as libc::c_int,
            );
            x264_macroblock_cache_mvd(
                h,
                block_idx_x[(4 as libc::c_int * i + 3 as libc::c_int) as usize] as libc::c_int,
                block_idx_y[(4 as libc::c_int * i + 3 as libc::c_int) as usize] as libc::c_int,
                1 as libc::c_int,
                1 as libc::c_int,
                0 as libc::c_int,
                mvd_7,
            );
        }
        _ => {
            __assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"encoder/cabac.c\0" as *const u8 as *const libc::c_char,
                377 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                    b"void cabac_8x8_mvd(x264_t *, x264_cabac_t *, int)\0",
                ))
                .as_ptr(),
            );
            'c_35133: {
                __assert_fail(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"encoder/cabac.c\0" as *const u8 as *const libc::c_char,
                    377 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                        b"void cabac_8x8_mvd(x264_t *, x264_cabac_t *, int)\0",
                    ))
                    .as_ptr(),
                );
            };
        }
    };
}
#[inline(always)]
unsafe extern "C" fn cabac_mb_header_i(
    mut h: *mut x264_t,
    mut cb: *mut x264_cabac_t,
    mut i_mb_type: libc::c_int,
    mut slice_type: libc::c_int,
    mut chroma: libc::c_int,
) {
    if slice_type == SLICE_TYPE_I as libc::c_int {
        let mut ctx: libc::c_int = 0 as libc::c_int;
        if (*h).mb.i_neighbour & MB_LEFT as libc::c_int as libc::c_uint != 0
            && (*h).mb.i_mb_type_left[0 as libc::c_int as usize] != I_4x4 as libc::c_int
        {
            ctx += 1;
            ctx;
        }
        if (*h).mb.i_neighbour & MB_TOP as libc::c_int as libc::c_uint != 0
            && (*h).mb.i_mb_type_top != I_4x4 as libc::c_int
        {
            ctx += 1;
            ctx;
        }
        cabac_mb_type_intra(
            h,
            cb,
            i_mb_type,
            3 as libc::c_int + ctx,
            3 as libc::c_int + 3 as libc::c_int,
            3 as libc::c_int + 4 as libc::c_int,
            3 as libc::c_int + 5 as libc::c_int,
            3 as libc::c_int + 6 as libc::c_int,
            3 as libc::c_int + 7 as libc::c_int,
        );
    } else if slice_type == SLICE_TYPE_P as libc::c_int {
        x264_8_cabac_encode_decision_c(cb, 14 as libc::c_int, 1 as libc::c_int);
        cabac_mb_type_intra(
            h,
            cb,
            i_mb_type,
            17 as libc::c_int + 0 as libc::c_int,
            17 as libc::c_int + 1 as libc::c_int,
            17 as libc::c_int + 2 as libc::c_int,
            17 as libc::c_int + 2 as libc::c_int,
            17 as libc::c_int + 3 as libc::c_int,
            17 as libc::c_int + 3 as libc::c_int,
        );
    } else if slice_type == SLICE_TYPE_B as libc::c_int {
        x264_8_cabac_encode_decision_c(cb, 27 as libc::c_int + 3 as libc::c_int, 1 as libc::c_int);
        x264_8_cabac_encode_decision_c(cb, 27 as libc::c_int + 4 as libc::c_int, 1 as libc::c_int);
        x264_8_cabac_encode_decision_c(cb, 27 as libc::c_int + 5 as libc::c_int, 1 as libc::c_int);
        x264_8_cabac_encode_decision_c(cb, 27 as libc::c_int + 5 as libc::c_int, 0 as libc::c_int);
        x264_8_cabac_encode_decision_c(cb, 27 as libc::c_int + 5 as libc::c_int, 1 as libc::c_int);
        cabac_mb_type_intra(
            h,
            cb,
            i_mb_type,
            32 as libc::c_int + 0 as libc::c_int,
            32 as libc::c_int + 1 as libc::c_int,
            32 as libc::c_int + 2 as libc::c_int,
            32 as libc::c_int + 2 as libc::c_int,
            32 as libc::c_int + 3 as libc::c_int,
            32 as libc::c_int + 3 as libc::c_int,
        );
    }
    if i_mb_type == I_PCM as libc::c_int {
        return;
    }
    if i_mb_type != I_16x16 as libc::c_int {
        if (*((*h).pps).as_mut_ptr()).b_transform_8x8_mode != 0 {
            cabac_transform_size(h, cb);
        }
        let mut di: libc::c_int = if (*h).mb.b_transform_8x8 != 0 {
            4 as libc::c_int
        } else {
            1 as libc::c_int
        };
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < 16 as libc::c_int {
            let i_pred: libc::c_int = x264_mb_predict_intra4x4_mode(h, i);
            let i_mode: libc::c_int = x264_mb_pred_mode4x4_fix[((*h).mb.cache.intra4x4_pred_mode
                [x264_scan8[i as usize] as usize]
                as libc::c_int
                + 1 as libc::c_int)
                as usize] as libc::c_int;
            cabac_intra4x4_pred_mode(cb, i_pred, i_mode);
            i += di;
        }
    }
    if chroma != 0 {
        cabac_intra_chroma_pred_mode(h, cb);
    }
}
#[inline(always)]
unsafe extern "C" fn cabac_mb_header_p(
    mut h: *mut x264_t,
    mut cb: *mut x264_cabac_t,
    mut i_mb_type: libc::c_int,
    mut chroma: libc::c_int,
) {
    if i_mb_type == P_L0 as libc::c_int {
        x264_8_cabac_encode_decision_c(cb, 14 as libc::c_int, 0 as libc::c_int);
        if (*h).mb.i_partition == D_16x16 as libc::c_int {
            x264_8_cabac_encode_decision_c(cb, 15 as libc::c_int, 0 as libc::c_int);
            x264_8_cabac_encode_decision_c(cb, 16 as libc::c_int, 0 as libc::c_int);
            if (*h).mb.pic.i_fref[0 as libc::c_int as usize] > 1 as libc::c_int {
                cabac_ref_p(h, cb, 0 as libc::c_int);
            }
            let mut mvd: uint16_t =
                cabac_mvd(h, cb, 0 as libc::c_int, 0 as libc::c_int, 4 as libc::c_int);
            x264_macroblock_cache_mvd(
                h,
                block_idx_x[0 as libc::c_int as usize] as libc::c_int,
                block_idx_y[0 as libc::c_int as usize] as libc::c_int,
                4 as libc::c_int,
                4 as libc::c_int,
                0 as libc::c_int,
                mvd,
            );
        } else if (*h).mb.i_partition == D_16x8 as libc::c_int {
            x264_8_cabac_encode_decision_c(cb, 15 as libc::c_int, 1 as libc::c_int);
            x264_8_cabac_encode_decision_c(cb, 17 as libc::c_int, 1 as libc::c_int);
            if (*h).mb.pic.i_fref[0 as libc::c_int as usize] > 1 as libc::c_int {
                cabac_ref_p(h, cb, 0 as libc::c_int);
                cabac_ref_p(h, cb, 8 as libc::c_int);
            }
            let mut mvd_0: uint16_t =
                cabac_mvd(h, cb, 0 as libc::c_int, 0 as libc::c_int, 4 as libc::c_int);
            x264_macroblock_cache_mvd(
                h,
                block_idx_x[0 as libc::c_int as usize] as libc::c_int,
                block_idx_y[0 as libc::c_int as usize] as libc::c_int,
                4 as libc::c_int,
                2 as libc::c_int,
                0 as libc::c_int,
                mvd_0,
            );
            let mut mvd_1: uint16_t =
                cabac_mvd(h, cb, 0 as libc::c_int, 8 as libc::c_int, 4 as libc::c_int);
            x264_macroblock_cache_mvd(
                h,
                block_idx_x[8 as libc::c_int as usize] as libc::c_int,
                block_idx_y[8 as libc::c_int as usize] as libc::c_int,
                4 as libc::c_int,
                2 as libc::c_int,
                0 as libc::c_int,
                mvd_1,
            );
        } else {
            x264_8_cabac_encode_decision_c(cb, 15 as libc::c_int, 1 as libc::c_int);
            x264_8_cabac_encode_decision_c(cb, 17 as libc::c_int, 0 as libc::c_int);
            if (*h).mb.pic.i_fref[0 as libc::c_int as usize] > 1 as libc::c_int {
                cabac_ref_p(h, cb, 0 as libc::c_int);
                cabac_ref_p(h, cb, 4 as libc::c_int);
            }
            let mut mvd_2: uint16_t =
                cabac_mvd(h, cb, 0 as libc::c_int, 0 as libc::c_int, 2 as libc::c_int);
            x264_macroblock_cache_mvd(
                h,
                block_idx_x[0 as libc::c_int as usize] as libc::c_int,
                block_idx_y[0 as libc::c_int as usize] as libc::c_int,
                2 as libc::c_int,
                4 as libc::c_int,
                0 as libc::c_int,
                mvd_2,
            );
            let mut mvd_3: uint16_t =
                cabac_mvd(h, cb, 0 as libc::c_int, 4 as libc::c_int, 2 as libc::c_int);
            x264_macroblock_cache_mvd(
                h,
                block_idx_x[4 as libc::c_int as usize] as libc::c_int,
                block_idx_y[4 as libc::c_int as usize] as libc::c_int,
                2 as libc::c_int,
                4 as libc::c_int,
                0 as libc::c_int,
                mvd_3,
            );
        }
    } else if i_mb_type == P_8x8 as libc::c_int {
        x264_8_cabac_encode_decision_c(cb, 14 as libc::c_int, 0 as libc::c_int);
        x264_8_cabac_encode_decision_c(cb, 15 as libc::c_int, 0 as libc::c_int);
        x264_8_cabac_encode_decision_c(cb, 16 as libc::c_int, 1 as libc::c_int);
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            cabac_subpartition_p(cb, (*h).mb.i_sub_partition[i as usize] as libc::c_int);
            i += 1;
            i;
        }
        if (*h).mb.pic.i_fref[0 as libc::c_int as usize] > 1 as libc::c_int {
            cabac_ref_p(h, cb, 0 as libc::c_int);
            cabac_ref_p(h, cb, 4 as libc::c_int);
            cabac_ref_p(h, cb, 8 as libc::c_int);
            cabac_ref_p(h, cb, 12 as libc::c_int);
        }
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < 4 as libc::c_int {
            cabac_8x8_mvd(h, cb, i_0);
            i_0 += 1;
            i_0;
        }
    } else {
        cabac_mb_header_i(h, cb, i_mb_type, SLICE_TYPE_P as libc::c_int, chroma);
    };
}
#[inline(always)]
unsafe extern "C" fn cabac_mb_header_b(
    mut h: *mut x264_t,
    mut cb: *mut x264_cabac_t,
    mut i_mb_type: libc::c_int,
    mut chroma: libc::c_int,
) {
    let mut ctx: libc::c_int = 0 as libc::c_int;
    if (*h).mb.i_neighbour & MB_LEFT as libc::c_int as libc::c_uint != 0
        && (*h).mb.i_mb_type_left[0 as libc::c_int as usize] != B_SKIP as libc::c_int
        && (*h).mb.i_mb_type_left[0 as libc::c_int as usize] != B_DIRECT as libc::c_int
    {
        ctx += 1;
        ctx;
    }
    if (*h).mb.i_neighbour & MB_TOP as libc::c_int as libc::c_uint != 0
        && (*h).mb.i_mb_type_top != B_SKIP as libc::c_int
        && (*h).mb.i_mb_type_top != B_DIRECT as libc::c_int
    {
        ctx += 1;
        ctx;
    }
    if i_mb_type == B_DIRECT as libc::c_int {
        x264_8_cabac_encode_decision_c(cb, 27 as libc::c_int + ctx, 0 as libc::c_int);
        return;
    }
    x264_8_cabac_encode_decision_c(cb, 27 as libc::c_int + ctx, 1 as libc::c_int);
    if i_mb_type == B_8x8 as libc::c_int {
        x264_8_cabac_encode_decision_c(cb, 27 as libc::c_int + 3 as libc::c_int, 1 as libc::c_int);
        x264_8_cabac_encode_decision_c(cb, 27 as libc::c_int + 4 as libc::c_int, 1 as libc::c_int);
        x264_8_cabac_encode_decision_c(cb, 27 as libc::c_int + 5 as libc::c_int, 1 as libc::c_int);
        x264_8_cabac_encode_decision_c(cb, 27 as libc::c_int + 5 as libc::c_int, 1 as libc::c_int);
        x264_8_cabac_encode_decision_c(cb, 27 as libc::c_int + 5 as libc::c_int, 1 as libc::c_int);
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            cabac_subpartition_b(cb, (*h).mb.i_sub_partition[i as usize] as libc::c_int);
            i += 1;
            i;
        }
        if (*h).mb.pic.i_fref[0 as libc::c_int as usize] > 1 as libc::c_int {
            let mut i_0: libc::c_int = 0 as libc::c_int;
            while i_0 < 4 as libc::c_int {
                if x264_mb_partition_listX_table[0 as libc::c_int as usize]
                    [(*h).mb.i_sub_partition[i_0 as usize] as usize]
                    != 0
                {
                    cabac_ref_b(h, cb, 0 as libc::c_int, 4 as libc::c_int * i_0);
                }
                i_0 += 1;
                i_0;
            }
        }
        if (*h).mb.pic.i_fref[1 as libc::c_int as usize] > 1 as libc::c_int {
            let mut i_1: libc::c_int = 0 as libc::c_int;
            while i_1 < 4 as libc::c_int {
                if x264_mb_partition_listX_table[1 as libc::c_int as usize]
                    [(*h).mb.i_sub_partition[i_1 as usize] as usize]
                    != 0
                {
                    cabac_ref_b(h, cb, 1 as libc::c_int, 4 as libc::c_int * i_1);
                }
                i_1 += 1;
                i_1;
            }
        }
        let mut i_2: libc::c_int = 0 as libc::c_int;
        while i_2 < 4 as libc::c_int {
            if x264_mb_partition_listX_table[0 as libc::c_int as usize]
                [(*h).mb.i_sub_partition[i_2 as usize] as usize]
                != 0
            {
                let mut mvd: uint16_t = cabac_mvd(
                    h,
                    cb,
                    0 as libc::c_int,
                    4 as libc::c_int * i_2,
                    2 as libc::c_int,
                );
                x264_macroblock_cache_mvd(
                    h,
                    block_idx_x[(4 as libc::c_int * i_2) as usize] as libc::c_int,
                    block_idx_y[(4 as libc::c_int * i_2) as usize] as libc::c_int,
                    2 as libc::c_int,
                    2 as libc::c_int,
                    0 as libc::c_int,
                    mvd,
                );
            }
            i_2 += 1;
            i_2;
        }
        let mut i_3: libc::c_int = 0 as libc::c_int;
        while i_3 < 4 as libc::c_int {
            if x264_mb_partition_listX_table[1 as libc::c_int as usize]
                [(*h).mb.i_sub_partition[i_3 as usize] as usize]
                != 0
            {
                let mut mvd_0: uint16_t = cabac_mvd(
                    h,
                    cb,
                    1 as libc::c_int,
                    4 as libc::c_int * i_3,
                    2 as libc::c_int,
                );
                x264_macroblock_cache_mvd(
                    h,
                    block_idx_x[(4 as libc::c_int * i_3) as usize] as libc::c_int,
                    block_idx_y[(4 as libc::c_int * i_3) as usize] as libc::c_int,
                    2 as libc::c_int,
                    2 as libc::c_int,
                    1 as libc::c_int,
                    mvd_0,
                );
            }
            i_3 += 1;
            i_3;
        }
    } else if i_mb_type >= B_L0_L0 as libc::c_int && i_mb_type <= B_BI_BI as libc::c_int {
        static mut i_mb_bits: [uint8_t; 27] = [
            0x31 as libc::c_int as uint8_t,
            0x29 as libc::c_int as uint8_t,
            0x4 as libc::c_int as uint8_t,
            0x35 as libc::c_int as uint8_t,
            0x2d as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0x43 as libc::c_int as uint8_t,
            0x63 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0x3d as libc::c_int as uint8_t,
            0x2f as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0x39 as libc::c_int as uint8_t,
            0x25 as libc::c_int as uint8_t,
            0x6 as libc::c_int as uint8_t,
            0x53 as libc::c_int as uint8_t,
            0x73 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0x4b as libc::c_int as uint8_t,
            0x6b as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0x5b as libc::c_int as uint8_t,
            0x7b as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0x47 as libc::c_int as uint8_t,
            0x67 as libc::c_int as uint8_t,
            0x21 as libc::c_int as uint8_t,
        ];
        let idx: libc::c_int = (i_mb_type - B_L0_L0 as libc::c_int) * 3 as libc::c_int
            + ((*h).mb.i_partition - D_16x8 as libc::c_int);
        let mut bits: libc::c_int = i_mb_bits[idx as usize] as libc::c_int;
        x264_8_cabac_encode_decision_c(
            cb,
            27 as libc::c_int + 3 as libc::c_int,
            bits & 1 as libc::c_int,
        );
        x264_8_cabac_encode_decision_c(
            cb,
            27 as libc::c_int + 5 as libc::c_int - (bits & 1 as libc::c_int),
            (bits >> 1 as libc::c_int) & 1 as libc::c_int,
        );
        bits >>= 2 as libc::c_int;
        if bits != 1 as libc::c_int {
            x264_8_cabac_encode_decision_c(
                cb,
                27 as libc::c_int + 5 as libc::c_int,
                bits & 1 as libc::c_int,
            );
            bits >>= 1 as libc::c_int;
            x264_8_cabac_encode_decision_c(
                cb,
                27 as libc::c_int + 5 as libc::c_int,
                bits & 1 as libc::c_int,
            );
            bits >>= 1 as libc::c_int;
            x264_8_cabac_encode_decision_c(
                cb,
                27 as libc::c_int + 5 as libc::c_int,
                bits & 1 as libc::c_int,
            );
            bits >>= 1 as libc::c_int;
            if bits != 1 as libc::c_int {
                x264_8_cabac_encode_decision_c(
                    cb,
                    27 as libc::c_int + 5 as libc::c_int,
                    bits & 1 as libc::c_int,
                );
            }
        }
        let mut b_list: *const [uint8_t; 2] =
            (x264_mb_type_list_table[i_mb_type as usize]).as_ptr();
        if (*h).mb.pic.i_fref[0 as libc::c_int as usize] > 1 as libc::c_int {
            if (*b_list.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] != 0 {
                cabac_ref_b(h, cb, 0 as libc::c_int, 0 as libc::c_int);
            }
            if (*b_list.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize] as libc::c_int
                != 0
                && (*h).mb.i_partition != D_16x16 as libc::c_int
            {
                cabac_ref_b(
                    h,
                    cb,
                    0 as libc::c_int,
                    8 as libc::c_int
                        >> ((*h).mb.i_partition == D_8x16 as libc::c_int) as libc::c_int,
                );
            }
        }
        if (*h).mb.pic.i_fref[1 as libc::c_int as usize] > 1 as libc::c_int {
            if (*b_list.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] != 0 {
                cabac_ref_b(h, cb, 1 as libc::c_int, 0 as libc::c_int);
            }
            if (*b_list.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize] as libc::c_int
                != 0
                && (*h).mb.i_partition != D_16x16 as libc::c_int
            {
                cabac_ref_b(
                    h,
                    cb,
                    1 as libc::c_int,
                    8 as libc::c_int
                        >> ((*h).mb.i_partition == D_8x16 as libc::c_int) as libc::c_int,
                );
            }
        }
        let mut i_list: libc::c_int = 0 as libc::c_int;
        while i_list < 2 as libc::c_int {
            if (*h).mb.i_partition == D_16x16 as libc::c_int {
                if (*b_list.offset(i_list as isize))[0 as libc::c_int as usize] != 0 {
                    let mut mvd_1: uint16_t =
                        cabac_mvd(h, cb, i_list, 0 as libc::c_int, 4 as libc::c_int);
                    x264_macroblock_cache_mvd(
                        h,
                        block_idx_x[0 as libc::c_int as usize] as libc::c_int,
                        block_idx_y[0 as libc::c_int as usize] as libc::c_int,
                        4 as libc::c_int,
                        4 as libc::c_int,
                        i_list,
                        mvd_1,
                    );
                }
            } else if (*h).mb.i_partition == D_16x8 as libc::c_int {
                if (*b_list.offset(i_list as isize))[0 as libc::c_int as usize] != 0 {
                    let mut mvd_2: uint16_t =
                        cabac_mvd(h, cb, i_list, 0 as libc::c_int, 4 as libc::c_int);
                    x264_macroblock_cache_mvd(
                        h,
                        block_idx_x[0 as libc::c_int as usize] as libc::c_int,
                        block_idx_y[0 as libc::c_int as usize] as libc::c_int,
                        4 as libc::c_int,
                        2 as libc::c_int,
                        i_list,
                        mvd_2,
                    );
                }
                if (*b_list.offset(i_list as isize))[1 as libc::c_int as usize] != 0 {
                    let mut mvd_3: uint16_t =
                        cabac_mvd(h, cb, i_list, 8 as libc::c_int, 4 as libc::c_int);
                    x264_macroblock_cache_mvd(
                        h,
                        block_idx_x[8 as libc::c_int as usize] as libc::c_int,
                        block_idx_y[8 as libc::c_int as usize] as libc::c_int,
                        4 as libc::c_int,
                        2 as libc::c_int,
                        i_list,
                        mvd_3,
                    );
                }
            } else {
                if (*b_list.offset(i_list as isize))[0 as libc::c_int as usize] != 0 {
                    let mut mvd_4: uint16_t =
                        cabac_mvd(h, cb, i_list, 0 as libc::c_int, 2 as libc::c_int);
                    x264_macroblock_cache_mvd(
                        h,
                        block_idx_x[0 as libc::c_int as usize] as libc::c_int,
                        block_idx_y[0 as libc::c_int as usize] as libc::c_int,
                        2 as libc::c_int,
                        4 as libc::c_int,
                        i_list,
                        mvd_4,
                    );
                }
                if (*b_list.offset(i_list as isize))[1 as libc::c_int as usize] != 0 {
                    let mut mvd_5: uint16_t =
                        cabac_mvd(h, cb, i_list, 4 as libc::c_int, 2 as libc::c_int);
                    x264_macroblock_cache_mvd(
                        h,
                        block_idx_x[4 as libc::c_int as usize] as libc::c_int,
                        block_idx_y[4 as libc::c_int as usize] as libc::c_int,
                        2 as libc::c_int,
                        4 as libc::c_int,
                        i_list,
                        mvd_5,
                    );
                }
            }
            i_list += 1;
            i_list;
        }
    } else {
        cabac_mb_header_i(h, cb, i_mb_type, SLICE_TYPE_B as libc::c_int, chroma);
    };
}
#[inline(always)]
unsafe extern "C" fn cabac_cbf_ctxidxinc(
    mut h: *mut x264_t,
    mut i_cat: libc::c_int,
    mut i_idx: libc::c_int,
    mut b_intra: libc::c_int,
    mut b_dc: libc::c_int,
) -> libc::c_int {
    static mut base_ctx: [uint16_t; 14] = [
        85 as libc::c_int as uint16_t,
        89 as libc::c_int as uint16_t,
        93 as libc::c_int as uint16_t,
        97 as libc::c_int as uint16_t,
        101 as libc::c_int as uint16_t,
        1012 as libc::c_int as uint16_t,
        460 as libc::c_int as uint16_t,
        464 as libc::c_int as uint16_t,
        468 as libc::c_int as uint16_t,
        1016 as libc::c_int as uint16_t,
        472 as libc::c_int as uint16_t,
        476 as libc::c_int as uint16_t,
        480 as libc::c_int as uint16_t,
        1020 as libc::c_int as uint16_t,
    ];
    if b_dc != 0 {
        i_idx -= 48 as libc::c_int;
        if i_cat == DCT_CHROMA_DC as libc::c_int {
            let mut i_nza: libc::c_int = if (*h).mb.cache.i_cbp_left != -(1 as libc::c_int) {
                ((*h).mb.cache.i_cbp_left >> (8 as libc::c_int + i_idx)) & 1 as libc::c_int
            } else {
                b_intra
            };
            let mut i_nzb: libc::c_int = if (*h).mb.cache.i_cbp_top != -(1 as libc::c_int) {
                ((*h).mb.cache.i_cbp_top >> (8 as libc::c_int + i_idx)) & 1 as libc::c_int
            } else {
                b_intra
            };
            base_ctx[i_cat as usize] as libc::c_int + 2 as libc::c_int * i_nzb + i_nza
        } else {
            let mut i_nza_0: libc::c_int =
                ((*h).mb.cache.i_cbp_left >> (8 as libc::c_int + i_idx)) & 1 as libc::c_int;
            let mut i_nzb_0: libc::c_int =
                ((*h).mb.cache.i_cbp_top >> (8 as libc::c_int + i_idx)) & 1 as libc::c_int;
            base_ctx[i_cat as usize] as libc::c_int + 2 as libc::c_int * i_nzb_0 + i_nza_0
        }
    } else {
        let mut i_nza_1: libc::c_int = (*h).mb.cache.non_zero_count
            [(x264_scan8[i_idx as usize] as libc::c_int - 1 as libc::c_int) as usize]
            as libc::c_int;
        let mut i_nzb_1: libc::c_int = (*h).mb.cache.non_zero_count
            [(x264_scan8[i_idx as usize] as libc::c_int - 8 as libc::c_int) as usize]
            as libc::c_int;
        if 0 != 0 && b_intra == 0 {
            base_ctx[i_cat as usize] as libc::c_int
                + ((2 as libc::c_int * i_nzb_1 + i_nza_1) & 0x7f as libc::c_int)
        } else {
            i_nza_1 &= 0x7f as libc::c_int + (b_intra << 7 as libc::c_int);
            i_nzb_1 &= 0x7f as libc::c_int + (b_intra << 7 as libc::c_int);
            base_ctx[i_cat as usize] as libc::c_int
                + 2 as libc::c_int * (i_nzb_1 != 0) as libc::c_int
                + (i_nza_1 != 0) as libc::c_int
        }
    }
}
#[inline(always)]
unsafe extern "C" fn cabac_block_residual_internal(
    mut h: *mut x264_t,
    mut cb: *mut x264_cabac_t,
    mut ctx_block_cat: libc::c_int,
    mut l: *mut dctcoef,
    mut chroma422dc: libc::c_int,
) {
    let mut ctx_sig: libc::c_int = x264_significant_coeff_flag_offset[(*h).mb.b_interlaced as usize]
        [ctx_block_cat as usize] as libc::c_int;
    let mut ctx_last: libc::c_int = x264_last_coeff_flag_offset[(*h).mb.b_interlaced as usize]
        [ctx_block_cat as usize] as libc::c_int;
    let mut ctx_level: libc::c_int =
        x264_coeff_abs_level_m1_offset[ctx_block_cat as usize] as libc::c_int;
    let mut coeff_idx: libc::c_int = -(1 as libc::c_int);
    let mut node_ctx: libc::c_int = 0 as libc::c_int;
    let mut last: libc::c_int =
        ((*h).quantf.coeff_last[ctx_block_cat as usize]).expect("non-null function pointer")(l);
    let mut levelgt1_ctx: *const uint8_t = if chroma422dc != 0 {
        coeff_abs_levelgt1_ctx_chroma_dc.as_ptr()
    } else {
        coeff_abs_levelgt1_ctx.as_ptr()
    };
    let mut coeffs: [dctcoef; 64] = [0; 64];
    if chroma422dc != 0 {
        let mut count_m1: libc::c_int = 7 as libc::c_int;
        let mut i: libc::c_int = 0 as libc::c_int;
        loop {
            if *l.offset(i as isize) != 0 {
                coeff_idx += 1;
                coeffs[coeff_idx as usize] = *l.offset(i as isize);
                x264_8_cabac_encode_decision_c(
                    cb,
                    ctx_sig + x264_coeff_flag_offset_chroma_422_dc[i as usize] as libc::c_int,
                    1 as libc::c_int,
                );
                if i == last {
                    x264_8_cabac_encode_decision_c(
                        cb,
                        ctx_last + x264_coeff_flag_offset_chroma_422_dc[i as usize] as libc::c_int,
                        1 as libc::c_int,
                    );
                    break;
                } else {
                    x264_8_cabac_encode_decision_c(
                        cb,
                        ctx_last + x264_coeff_flag_offset_chroma_422_dc[i as usize] as libc::c_int,
                        0 as libc::c_int,
                    );
                }
            } else {
                x264_8_cabac_encode_decision_c(
                    cb,
                    ctx_sig + x264_coeff_flag_offset_chroma_422_dc[i as usize] as libc::c_int,
                    0 as libc::c_int,
                );
            }
            i += 1;
            if i != count_m1 {
                continue;
            }
            coeff_idx += 1;
            coeffs[coeff_idx as usize] = *l.offset(i as isize);
            break;
        }
    } else {
        let mut count_m1_0: libc::c_int = x264_count_cat_m1[ctx_block_cat as usize] as libc::c_int;
        if count_m1_0 == 63 as libc::c_int {
            let mut sig_offset: *const uint8_t =
                (x264_significant_coeff_flag_offset_8x8[(*h).mb.b_interlaced as usize]).as_ptr();
            let mut i_0: libc::c_int = 0 as libc::c_int;
            loop {
                if *l.offset(i_0 as isize) != 0 {
                    coeff_idx += 1;
                    coeffs[coeff_idx as usize] = *l.offset(i_0 as isize);
                    x264_8_cabac_encode_decision_c(
                        cb,
                        ctx_sig + *sig_offset.offset(i_0 as isize) as libc::c_int,
                        1 as libc::c_int,
                    );
                    if i_0 == last {
                        x264_8_cabac_encode_decision_c(
                            cb,
                            ctx_last + x264_last_coeff_flag_offset_8x8[i_0 as usize] as libc::c_int,
                            1 as libc::c_int,
                        );
                        break;
                    } else {
                        x264_8_cabac_encode_decision_c(
                            cb,
                            ctx_last + x264_last_coeff_flag_offset_8x8[i_0 as usize] as libc::c_int,
                            0 as libc::c_int,
                        );
                    }
                } else {
                    x264_8_cabac_encode_decision_c(
                        cb,
                        ctx_sig + *sig_offset.offset(i_0 as isize) as libc::c_int,
                        0 as libc::c_int,
                    );
                }
                i_0 += 1;
                if i_0 != count_m1_0 {
                    continue;
                }
                coeff_idx += 1;
                coeffs[coeff_idx as usize] = *l.offset(i_0 as isize);
                break;
            }
        } else {
            let mut i_1: libc::c_int = 0 as libc::c_int;
            loop {
                if *l.offset(i_1 as isize) != 0 {
                    coeff_idx += 1;
                    coeffs[coeff_idx as usize] = *l.offset(i_1 as isize);
                    x264_8_cabac_encode_decision_c(cb, ctx_sig + i_1, 1 as libc::c_int);
                    if i_1 == last {
                        x264_8_cabac_encode_decision_c(cb, ctx_last + i_1, 1 as libc::c_int);
                        break;
                    } else {
                        x264_8_cabac_encode_decision_c(cb, ctx_last + i_1, 0 as libc::c_int);
                    }
                } else {
                    x264_8_cabac_encode_decision_c(cb, ctx_sig + i_1, 0 as libc::c_int);
                }
                i_1 += 1;
                if i_1 != count_m1_0 {
                    continue;
                }
                coeff_idx += 1;
                coeffs[coeff_idx as usize] = *l.offset(i_1 as isize);
                break;
            }
        }
    }
    loop {
        let mut coeff: libc::c_int = coeffs[coeff_idx as usize] as libc::c_int;
        let mut abs_coeff: libc::c_int = abs(coeff);
        let mut coeff_sign: libc::c_int = coeff >> 31 as libc::c_int;
        let mut ctx: libc::c_int =
            coeff_abs_level1_ctx[node_ctx as usize] as libc::c_int + ctx_level;
        if abs_coeff > 1 as libc::c_int {
            x264_8_cabac_encode_decision_c(cb, ctx, 1 as libc::c_int);
            ctx = *levelgt1_ctx.offset(node_ctx as isize) as libc::c_int + ctx_level;
            let mut i_2: libc::c_int = (if abs_coeff < 15 as libc::c_int {
                abs_coeff
            } else {
                15 as libc::c_int
            }) - 2 as libc::c_int;
            while i_2 > 0 as libc::c_int {
                x264_8_cabac_encode_decision_c(cb, ctx, 1 as libc::c_int);
                i_2 -= 1;
                i_2;
            }
            if abs_coeff < 15 as libc::c_int {
                x264_8_cabac_encode_decision_c(cb, ctx, 0 as libc::c_int);
            } else {
                x264_8_cabac_encode_ue_bypass(cb, 0 as libc::c_int, abs_coeff - 15 as libc::c_int);
            }
            node_ctx = coeff_abs_level_transition[1 as libc::c_int as usize][node_ctx as usize]
                as libc::c_int;
        } else {
            x264_8_cabac_encode_decision_c(cb, ctx, 0 as libc::c_int);
            node_ctx = coeff_abs_level_transition[0 as libc::c_int as usize][node_ctx as usize]
                as libc::c_int;
        }
        x264_8_cabac_encode_bypass_c(cb, coeff_sign);
        coeff_idx -= 1;
        if coeff_idx < 0 as libc::c_int {
            break;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_cabac_block_residual_c(
    mut h: *mut x264_t,
    mut cb: *mut x264_cabac_t,
    mut ctx_block_cat: libc::c_int,
    mut l: *mut dctcoef,
) {
    cabac_block_residual_internal(h, cb, ctx_block_cat, l, 0 as libc::c_int);
}
#[inline(always)]
unsafe extern "C" fn cabac_block_residual(
    mut h: *mut x264_t,
    mut cb: *mut x264_cabac_t,
    mut ctx_block_cat: libc::c_int,
    mut l: *mut dctcoef,
) {
    x264_8_cabac_block_residual_c(h, cb, ctx_block_cat, l);
}
unsafe extern "C" fn cabac_block_residual_422_dc(
    mut h: *mut x264_t,
    mut cb: *mut x264_cabac_t,
    mut ctx_block_cat: libc::c_int,
    mut l: *mut dctcoef,
) {
    cabac_block_residual_internal(h, cb, DCT_CHROMA_DC as libc::c_int, l, 1 as libc::c_int);
}
#[inline(always)]
unsafe extern "C" fn macroblock_write_cabac_internal(
    mut h: *mut x264_t,
    mut cb: *mut x264_cabac_t,
    mut plane_count: libc::c_int,
    mut chroma: libc::c_int,
) {
    let i_mb_type: libc::c_int = (*h).mb.i_type;
    let i_mb_pos_start: libc::c_int = x264_cabac_pos(cb);
    let mut i_mb_pos_tex: libc::c_int = 0;
    if (*h).sh.b_mbaff != 0
        && ((*h).mb.i_mb_y & 1 as libc::c_int == 0
            || (*((*h).mb.type_0).offset(((*h).mb.i_mb_xy - (*h).mb.i_mb_stride) as isize)
                as libc::c_int
                == P_SKIP as libc::c_int
                || *((*h).mb.type_0).offset(((*h).mb.i_mb_xy - (*h).mb.i_mb_stride) as isize)
                    as libc::c_int
                    == B_SKIP as libc::c_int))
    {
        cabac_field_decoding_flag(h, cb);
    }
    if (*h).sh.i_type == SLICE_TYPE_P as libc::c_int {
        cabac_mb_header_p(h, cb, i_mb_type, chroma);
    } else if (*h).sh.i_type == SLICE_TYPE_B as libc::c_int {
        cabac_mb_header_b(h, cb, i_mb_type, chroma);
    } else {
        cabac_mb_header_i(h, cb, i_mb_type, SLICE_TYPE_I as libc::c_int, chroma);
    }
    i_mb_pos_tex = x264_cabac_pos(cb);
    (*h).stat.frame.i_mv_bits += i_mb_pos_tex - i_mb_pos_start;
    if i_mb_type == I_PCM as libc::c_int {
        let mut s: bs_t = bs_s {
            p_start: std::ptr::null_mut::<uint8_t>(),
            p: std::ptr::null_mut::<uint8_t>(),
            p_end: std::ptr::null_mut::<uint8_t>(),
            cur_bits: 0,
            i_left: 0,
            i_bits_encoded: 0,
        };
        bs_init(
            &mut s,
            (*cb).p as *mut libc::c_void,
            ((*cb).p_end).offset_from((*cb).p) as libc::c_long as libc::c_int,
        );
        let mut p: libc::c_int = 0 as libc::c_int;
        while p < plane_count {
            let mut i: libc::c_int = 0 as libc::c_int;
            while i < 256 as libc::c_int {
                bs_write(
                    &mut s,
                    8 as libc::c_int,
                    *((*h).mb.pic.p_fenc[p as usize]).offset(i as isize) as uint32_t,
                );
                i += 1;
                i;
            }
            p += 1;
            p;
        }
        if chroma != 0 {
            let mut ch: libc::c_int = 1 as libc::c_int;
            while ch < 3 as libc::c_int {
                let mut i_0: libc::c_int = 0 as libc::c_int;
                while i_0 < 16 as libc::c_int >> (*h).mb.chroma_v_shift {
                    let mut j: libc::c_int = 0 as libc::c_int;
                    while j < 8 as libc::c_int {
                        bs_write(
                            &mut s,
                            8 as libc::c_int,
                            *((*h).mb.pic.p_fenc[ch as usize])
                                .offset((i_0 * 16 as libc::c_int + j) as isize)
                                as uint32_t,
                        );
                        j += 1;
                        j;
                    }
                    i_0 += 1;
                    i_0;
                }
                ch += 1;
                ch;
            }
        }
        bs_flush(&mut s);
        (*cb).p = s.p;
        x264_8_cabac_encode_init_core(cb);
        (*h).stat.frame.i_tex_bits += x264_cabac_pos(cb) - i_mb_pos_tex;
        return;
    }
    if i_mb_type != I_16x16 as libc::c_int {
        cabac_cbp_luma(h, cb);
        if chroma != 0 {
            cabac_cbp_chroma(h, cb);
        }
    }
    if x264_mb_transform_8x8_allowed(h) != 0 && (*h).mb.i_cbp_luma != 0 {
        cabac_transform_size(h, cb);
    }
    if (*h).mb.i_cbp_luma != 0
        || chroma != 0 && (*h).mb.i_cbp_chroma != 0
        || i_mb_type == I_16x16 as libc::c_int
    {
        let b_intra: libc::c_int = (i_mb_type == I_4x4 as libc::c_int
            || i_mb_type == I_8x8 as libc::c_int
            || i_mb_type == I_16x16 as libc::c_int
            || i_mb_type == I_PCM as libc::c_int) as libc::c_int;
        cabac_qp_delta(h, cb);
        if i_mb_type == I_16x16 as libc::c_int {
            let mut p_0: libc::c_int = 0 as libc::c_int;
            while p_0 < plane_count {
                let mut ctxidxinc: libc::c_int = cabac_cbf_ctxidxinc(
                    h,
                    ctx_cat_plane[DCT_LUMA_DC as libc::c_int as usize][p_0 as usize] as libc::c_int,
                    48 as libc::c_int + p_0,
                    1 as libc::c_int,
                    1 as libc::c_int,
                );
                if (*h).mb.cache.non_zero_count
                    [x264_scan8[(48 as libc::c_int + p_0) as usize] as usize]
                    != 0
                {
                    x264_8_cabac_encode_decision_c(cb, ctxidxinc, 1 as libc::c_int);
                    cabac_block_residual(
                        h,
                        cb,
                        ctx_cat_plane[DCT_LUMA_DC as libc::c_int as usize][p_0 as usize]
                            as libc::c_int,
                        ((*h).dct.luma16x16_dc[p_0 as usize]).as_mut_ptr(),
                    );
                } else {
                    x264_8_cabac_encode_decision_c(cb, ctxidxinc, 0 as libc::c_int);
                }
                if (*h).mb.i_cbp_luma != 0 {
                    let mut i_1: libc::c_int = p_0 * 16 as libc::c_int;
                    while i_1 < p_0 * 16 as libc::c_int + 16 as libc::c_int {
                        let mut ctxidxinc_0: libc::c_int = cabac_cbf_ctxidxinc(
                            h,
                            ctx_cat_plane[DCT_LUMA_AC as libc::c_int as usize][p_0 as usize]
                                as libc::c_int,
                            i_1,
                            1 as libc::c_int,
                            0 as libc::c_int,
                        );
                        if (*h).mb.cache.non_zero_count[x264_scan8[i_1 as usize] as usize] != 0 {
                            x264_8_cabac_encode_decision_c(cb, ctxidxinc_0, 1 as libc::c_int);
                            cabac_block_residual(
                                h,
                                cb,
                                ctx_cat_plane[DCT_LUMA_AC as libc::c_int as usize][p_0 as usize]
                                    as libc::c_int,
                                ((*h).dct.luma4x4[i_1 as usize])
                                    .as_mut_ptr()
                                    .offset(1 as libc::c_int as isize),
                            );
                        } else {
                            x264_8_cabac_encode_decision_c(cb, ctxidxinc_0, 0 as libc::c_int);
                        }
                        i_1 += 1;
                        i_1;
                    }
                }
                p_0 += 1;
                p_0;
            }
        } else if (*h).mb.b_transform_8x8 != 0 {
            if plane_count == 3 as libc::c_int {
                let mut nnzbak: [[uint8_t; 8]; 3] = [[0; 8]; 3];
                if (*h).mb.i_neighbour & MB_LEFT as libc::c_int as libc::c_uint != 0
                    && *((*h).mb.mb_transform_size)
                        .offset((*h).mb.i_mb_left_xy[0 as libc::c_int as usize] as isize)
                        == 0
                    && *((*h).mb.cbp)
                        .offset((*h).mb.i_mb_left_xy[0 as libc::c_int as usize] as isize)
                        as libc::c_int
                        & 0x1000 as libc::c_int
                        == 0
                {
                    nnzbak[0 as libc::c_int as usize][0 as libc::c_int as usize] =
                        (*h).mb.cache.non_zero_count[(x264_scan8
                            [(16 as libc::c_int * 0 as libc::c_int + 0 as libc::c_int) as usize]
                            as libc::c_int
                            - 1 as libc::c_int)
                            as usize];
                    (*h).mb.cache.non_zero_count[(x264_scan8
                        [(16 as libc::c_int * 0 as libc::c_int + 0 as libc::c_int) as usize]
                        as libc::c_int
                        - 1 as libc::c_int)
                        as usize] = 0 as libc::c_int as uint8_t;
                    nnzbak[0 as libc::c_int as usize][1 as libc::c_int as usize] =
                        (*h).mb.cache.non_zero_count[(x264_scan8
                            [(16 as libc::c_int * 0 as libc::c_int + 2 as libc::c_int) as usize]
                            as libc::c_int
                            - 1 as libc::c_int)
                            as usize];
                    (*h).mb.cache.non_zero_count[(x264_scan8
                        [(16 as libc::c_int * 0 as libc::c_int + 2 as libc::c_int) as usize]
                        as libc::c_int
                        - 1 as libc::c_int)
                        as usize] = 0 as libc::c_int as uint8_t;
                    nnzbak[1 as libc::c_int as usize][0 as libc::c_int as usize] =
                        (*h).mb.cache.non_zero_count[(x264_scan8
                            [(16 as libc::c_int * 1 as libc::c_int + 0 as libc::c_int) as usize]
                            as libc::c_int
                            - 1 as libc::c_int)
                            as usize];
                    (*h).mb.cache.non_zero_count[(x264_scan8
                        [(16 as libc::c_int * 1 as libc::c_int + 0 as libc::c_int) as usize]
                        as libc::c_int
                        - 1 as libc::c_int)
                        as usize] = 0 as libc::c_int as uint8_t;
                    nnzbak[1 as libc::c_int as usize][1 as libc::c_int as usize] =
                        (*h).mb.cache.non_zero_count[(x264_scan8
                            [(16 as libc::c_int * 1 as libc::c_int + 2 as libc::c_int) as usize]
                            as libc::c_int
                            - 1 as libc::c_int)
                            as usize];
                    (*h).mb.cache.non_zero_count[(x264_scan8
                        [(16 as libc::c_int * 1 as libc::c_int + 2 as libc::c_int) as usize]
                        as libc::c_int
                        - 1 as libc::c_int)
                        as usize] = 0 as libc::c_int as uint8_t;
                    nnzbak[2 as libc::c_int as usize][0 as libc::c_int as usize] =
                        (*h).mb.cache.non_zero_count[(x264_scan8
                            [(16 as libc::c_int * 2 as libc::c_int + 0 as libc::c_int) as usize]
                            as libc::c_int
                            - 1 as libc::c_int)
                            as usize];
                    (*h).mb.cache.non_zero_count[(x264_scan8
                        [(16 as libc::c_int * 2 as libc::c_int + 0 as libc::c_int) as usize]
                        as libc::c_int
                        - 1 as libc::c_int)
                        as usize] = 0 as libc::c_int as uint8_t;
                    nnzbak[2 as libc::c_int as usize][1 as libc::c_int as usize] =
                        (*h).mb.cache.non_zero_count[(x264_scan8
                            [(16 as libc::c_int * 2 as libc::c_int + 2 as libc::c_int) as usize]
                            as libc::c_int
                            - 1 as libc::c_int)
                            as usize];
                    (*h).mb.cache.non_zero_count[(x264_scan8
                        [(16 as libc::c_int * 2 as libc::c_int + 2 as libc::c_int) as usize]
                        as libc::c_int
                        - 1 as libc::c_int)
                        as usize] = 0 as libc::c_int as uint8_t;
                }
                if (*h).mb.i_neighbour & MB_LEFT as libc::c_int as libc::c_uint != 0
                    && *((*h).mb.mb_transform_size)
                        .offset((*h).mb.i_mb_left_xy[1 as libc::c_int as usize] as isize)
                        == 0
                    && *((*h).mb.cbp)
                        .offset((*h).mb.i_mb_left_xy[1 as libc::c_int as usize] as isize)
                        as libc::c_int
                        & 0x1000 as libc::c_int
                        == 0
                {
                    nnzbak[0 as libc::c_int as usize][2 as libc::c_int as usize] =
                        (*h).mb.cache.non_zero_count[(x264_scan8
                            [(16 as libc::c_int * 0 as libc::c_int + 8 as libc::c_int) as usize]
                            as libc::c_int
                            - 1 as libc::c_int)
                            as usize];
                    (*h).mb.cache.non_zero_count[(x264_scan8
                        [(16 as libc::c_int * 0 as libc::c_int + 8 as libc::c_int) as usize]
                        as libc::c_int
                        - 1 as libc::c_int)
                        as usize] = 0 as libc::c_int as uint8_t;
                    nnzbak[0 as libc::c_int as usize][3 as libc::c_int as usize] =
                        (*h).mb.cache.non_zero_count[(x264_scan8
                            [(16 as libc::c_int * 0 as libc::c_int + 10 as libc::c_int) as usize]
                            as libc::c_int
                            - 1 as libc::c_int)
                            as usize];
                    (*h).mb.cache.non_zero_count[(x264_scan8
                        [(16 as libc::c_int * 0 as libc::c_int + 10 as libc::c_int) as usize]
                        as libc::c_int
                        - 1 as libc::c_int)
                        as usize] = 0 as libc::c_int as uint8_t;
                    nnzbak[1 as libc::c_int as usize][2 as libc::c_int as usize] =
                        (*h).mb.cache.non_zero_count[(x264_scan8
                            [(16 as libc::c_int * 1 as libc::c_int + 8 as libc::c_int) as usize]
                            as libc::c_int
                            - 1 as libc::c_int)
                            as usize];
                    (*h).mb.cache.non_zero_count[(x264_scan8
                        [(16 as libc::c_int * 1 as libc::c_int + 8 as libc::c_int) as usize]
                        as libc::c_int
                        - 1 as libc::c_int)
                        as usize] = 0 as libc::c_int as uint8_t;
                    nnzbak[1 as libc::c_int as usize][3 as libc::c_int as usize] =
                        (*h).mb.cache.non_zero_count[(x264_scan8
                            [(16 as libc::c_int * 1 as libc::c_int + 10 as libc::c_int) as usize]
                            as libc::c_int
                            - 1 as libc::c_int)
                            as usize];
                    (*h).mb.cache.non_zero_count[(x264_scan8
                        [(16 as libc::c_int * 1 as libc::c_int + 10 as libc::c_int) as usize]
                        as libc::c_int
                        - 1 as libc::c_int)
                        as usize] = 0 as libc::c_int as uint8_t;
                    nnzbak[2 as libc::c_int as usize][2 as libc::c_int as usize] =
                        (*h).mb.cache.non_zero_count[(x264_scan8
                            [(16 as libc::c_int * 2 as libc::c_int + 8 as libc::c_int) as usize]
                            as libc::c_int
                            - 1 as libc::c_int)
                            as usize];
                    (*h).mb.cache.non_zero_count[(x264_scan8
                        [(16 as libc::c_int * 2 as libc::c_int + 8 as libc::c_int) as usize]
                        as libc::c_int
                        - 1 as libc::c_int)
                        as usize] = 0 as libc::c_int as uint8_t;
                    nnzbak[2 as libc::c_int as usize][3 as libc::c_int as usize] =
                        (*h).mb.cache.non_zero_count[(x264_scan8
                            [(16 as libc::c_int * 2 as libc::c_int + 10 as libc::c_int) as usize]
                            as libc::c_int
                            - 1 as libc::c_int)
                            as usize];
                    (*h).mb.cache.non_zero_count[(x264_scan8
                        [(16 as libc::c_int * 2 as libc::c_int + 10 as libc::c_int) as usize]
                        as libc::c_int
                        - 1 as libc::c_int)
                        as usize] = 0 as libc::c_int as uint8_t;
                }
                if (*h).mb.i_neighbour & MB_TOP as libc::c_int as libc::c_uint != 0
                    && *((*h).mb.mb_transform_size).offset((*h).mb.i_mb_top_xy as isize) == 0
                    && *((*h).mb.cbp).offset((*h).mb.i_mb_top_xy as isize) as libc::c_int
                        & 0x1000 as libc::c_int
                        == 0
                {
                    (*(&mut *(*nnzbak.as_mut_ptr().offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(4 as libc::c_int as isize) as *mut uint8_t
                        as *mut x264_union32_t))
                        .i = (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
                        (*x264_scan8
                            .as_ptr()
                            .offset((16 as libc::c_int * 0 as libc::c_int) as isize)
                            as libc::c_int
                            - 8 as libc::c_int) as isize,
                    ) as *mut uint8_t as *mut x264_union32_t))
                        .i;
                    (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
                        (*x264_scan8
                            .as_ptr()
                            .offset((16 as libc::c_int * 0 as libc::c_int) as isize)
                            as libc::c_int
                            - 8 as libc::c_int) as isize,
                    ) as *mut uint8_t as *mut x264_union32_t))
                        .i = 0 as libc::c_uint;
                    (*(&mut *(*nnzbak.as_mut_ptr().offset(1 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(4 as libc::c_int as isize) as *mut uint8_t
                        as *mut x264_union32_t))
                        .i = (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
                        (*x264_scan8
                            .as_ptr()
                            .offset((16 as libc::c_int * 1 as libc::c_int) as isize)
                            as libc::c_int
                            - 8 as libc::c_int) as isize,
                    ) as *mut uint8_t as *mut x264_union32_t))
                        .i;
                    (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
                        (*x264_scan8
                            .as_ptr()
                            .offset((16 as libc::c_int * 1 as libc::c_int) as isize)
                            as libc::c_int
                            - 8 as libc::c_int) as isize,
                    ) as *mut uint8_t as *mut x264_union32_t))
                        .i = 0 as libc::c_uint;
                    (*(&mut *(*nnzbak.as_mut_ptr().offset(2 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(4 as libc::c_int as isize) as *mut uint8_t
                        as *mut x264_union32_t))
                        .i = (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
                        (*x264_scan8
                            .as_ptr()
                            .offset((16 as libc::c_int * 2 as libc::c_int) as isize)
                            as libc::c_int
                            - 8 as libc::c_int) as isize,
                    ) as *mut uint8_t as *mut x264_union32_t))
                        .i;
                    (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
                        (*x264_scan8
                            .as_ptr()
                            .offset((16 as libc::c_int * 2 as libc::c_int) as isize)
                            as libc::c_int
                            - 8 as libc::c_int) as isize,
                    ) as *mut uint8_t as *mut x264_union32_t))
                        .i = 0 as libc::c_uint;
                }
                let mut p_1: libc::c_int = 0 as libc::c_int;
                while p_1 < 3 as libc::c_int {
                    let mut i_2: libc::c_int = 0 as libc::c_int;
                    let mut msk: libc::c_int = (*h).mb.i_cbp_luma;
                    let mut skip: libc::c_int = 0;
                    while msk != 0 && {
                        skip = x264_ctz_4bit(msk as uint32_t);
                        i_2 += skip;
                        msk >>= skip + 1 as libc::c_int;
                        1 as libc::c_int != 0
                    } {
                        let mut ctxidxinc_1: libc::c_int = cabac_cbf_ctxidxinc(
                            h,
                            ctx_cat_plane[DCT_LUMA_8x8 as libc::c_int as usize][p_1 as usize]
                                as libc::c_int,
                            i_2 * 4 as libc::c_int + p_1 * 16 as libc::c_int,
                            b_intra,
                            0 as libc::c_int,
                        );
                        if (*h).mb.cache.non_zero_count[x264_scan8
                            [(i_2 * 4 as libc::c_int + p_1 * 16 as libc::c_int) as usize]
                            as usize]
                            != 0
                        {
                            x264_8_cabac_encode_decision_c(cb, ctxidxinc_1, 1 as libc::c_int);
                            cabac_block_residual(
                                h,
                                cb,
                                ctx_cat_plane[DCT_LUMA_8x8 as libc::c_int as usize][p_1 as usize]
                                    as libc::c_int,
                                ((*h).dct.luma8x8[(i_2 + p_1 * 4 as libc::c_int) as usize])
                                    .as_mut_ptr(),
                            );
                        } else {
                            x264_8_cabac_encode_decision_c(cb, ctxidxinc_1, 0 as libc::c_int);
                        }
                        i_2 += 1;
                        i_2;
                    }
                    p_1 += 1;
                    p_1;
                }
                if (*h).mb.i_neighbour & MB_LEFT as libc::c_int as libc::c_uint != 0
                    && *((*h).mb.mb_transform_size)
                        .offset((*h).mb.i_mb_left_xy[0 as libc::c_int as usize] as isize)
                        == 0
                    && *((*h).mb.cbp)
                        .offset((*h).mb.i_mb_left_xy[0 as libc::c_int as usize] as isize)
                        as libc::c_int
                        & 0x1000 as libc::c_int
                        == 0
                {
                    (*h).mb.cache.non_zero_count[(x264_scan8
                        [(16 as libc::c_int * 0 as libc::c_int + 0 as libc::c_int) as usize]
                        as libc::c_int
                        - 1 as libc::c_int)
                        as usize] = nnzbak[0 as libc::c_int as usize][0 as libc::c_int as usize];
                    (*h).mb.cache.non_zero_count[(x264_scan8
                        [(16 as libc::c_int * 0 as libc::c_int + 2 as libc::c_int) as usize]
                        as libc::c_int
                        - 1 as libc::c_int)
                        as usize] = nnzbak[0 as libc::c_int as usize][1 as libc::c_int as usize];
                    (*h).mb.cache.non_zero_count[(x264_scan8
                        [(16 as libc::c_int * 1 as libc::c_int + 0 as libc::c_int) as usize]
                        as libc::c_int
                        - 1 as libc::c_int)
                        as usize] = nnzbak[1 as libc::c_int as usize][0 as libc::c_int as usize];
                    (*h).mb.cache.non_zero_count[(x264_scan8
                        [(16 as libc::c_int * 1 as libc::c_int + 2 as libc::c_int) as usize]
                        as libc::c_int
                        - 1 as libc::c_int)
                        as usize] = nnzbak[1 as libc::c_int as usize][1 as libc::c_int as usize];
                    (*h).mb.cache.non_zero_count[(x264_scan8
                        [(16 as libc::c_int * 2 as libc::c_int + 0 as libc::c_int) as usize]
                        as libc::c_int
                        - 1 as libc::c_int)
                        as usize] = nnzbak[2 as libc::c_int as usize][0 as libc::c_int as usize];
                    (*h).mb.cache.non_zero_count[(x264_scan8
                        [(16 as libc::c_int * 2 as libc::c_int + 2 as libc::c_int) as usize]
                        as libc::c_int
                        - 1 as libc::c_int)
                        as usize] = nnzbak[2 as libc::c_int as usize][1 as libc::c_int as usize];
                }
                if (*h).mb.i_neighbour & MB_LEFT as libc::c_int as libc::c_uint != 0
                    && *((*h).mb.mb_transform_size)
                        .offset((*h).mb.i_mb_left_xy[1 as libc::c_int as usize] as isize)
                        == 0
                    && *((*h).mb.cbp)
                        .offset((*h).mb.i_mb_left_xy[1 as libc::c_int as usize] as isize)
                        as libc::c_int
                        & 0x1000 as libc::c_int
                        == 0
                {
                    (*h).mb.cache.non_zero_count[(x264_scan8
                        [(16 as libc::c_int * 0 as libc::c_int + 8 as libc::c_int) as usize]
                        as libc::c_int
                        - 1 as libc::c_int)
                        as usize] = nnzbak[0 as libc::c_int as usize][2 as libc::c_int as usize];
                    (*h).mb.cache.non_zero_count[(x264_scan8
                        [(16 as libc::c_int * 0 as libc::c_int + 10 as libc::c_int) as usize]
                        as libc::c_int
                        - 1 as libc::c_int)
                        as usize] = nnzbak[0 as libc::c_int as usize][3 as libc::c_int as usize];
                    (*h).mb.cache.non_zero_count[(x264_scan8
                        [(16 as libc::c_int * 1 as libc::c_int + 8 as libc::c_int) as usize]
                        as libc::c_int
                        - 1 as libc::c_int)
                        as usize] = nnzbak[1 as libc::c_int as usize][2 as libc::c_int as usize];
                    (*h).mb.cache.non_zero_count[(x264_scan8
                        [(16 as libc::c_int * 1 as libc::c_int + 10 as libc::c_int) as usize]
                        as libc::c_int
                        - 1 as libc::c_int)
                        as usize] = nnzbak[1 as libc::c_int as usize][3 as libc::c_int as usize];
                    (*h).mb.cache.non_zero_count[(x264_scan8
                        [(16 as libc::c_int * 2 as libc::c_int + 8 as libc::c_int) as usize]
                        as libc::c_int
                        - 1 as libc::c_int)
                        as usize] = nnzbak[2 as libc::c_int as usize][2 as libc::c_int as usize];
                    (*h).mb.cache.non_zero_count[(x264_scan8
                        [(16 as libc::c_int * 2 as libc::c_int + 10 as libc::c_int) as usize]
                        as libc::c_int
                        - 1 as libc::c_int)
                        as usize] = nnzbak[2 as libc::c_int as usize][3 as libc::c_int as usize];
                }
                if (*h).mb.i_neighbour & MB_TOP as libc::c_int as libc::c_uint != 0
                    && *((*h).mb.mb_transform_size).offset((*h).mb.i_mb_top_xy as isize) == 0
                    && *((*h).mb.cbp).offset((*h).mb.i_mb_top_xy as isize) as libc::c_int
                        & 0x1000 as libc::c_int
                        == 0
                {
                    (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
                        (*x264_scan8
                            .as_ptr()
                            .offset((16 as libc::c_int * 0 as libc::c_int) as isize)
                            as libc::c_int
                            - 8 as libc::c_int) as isize,
                    ) as *mut uint8_t as *mut x264_union32_t))
                        .i = (*(&mut *(*nnzbak.as_mut_ptr().offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(4 as libc::c_int as isize)
                        as *mut uint8_t as *mut x264_union32_t))
                        .i;
                    (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
                        (*x264_scan8
                            .as_ptr()
                            .offset((16 as libc::c_int * 1 as libc::c_int) as isize)
                            as libc::c_int
                            - 8 as libc::c_int) as isize,
                    ) as *mut uint8_t as *mut x264_union32_t))
                        .i = (*(&mut *(*nnzbak.as_mut_ptr().offset(1 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(4 as libc::c_int as isize)
                        as *mut uint8_t as *mut x264_union32_t))
                        .i;
                    (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
                        (*x264_scan8
                            .as_ptr()
                            .offset((16 as libc::c_int * 2 as libc::c_int) as isize)
                            as libc::c_int
                            - 8 as libc::c_int) as isize,
                    ) as *mut uint8_t as *mut x264_union32_t))
                        .i = (*(&mut *(*nnzbak.as_mut_ptr().offset(2 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(4 as libc::c_int as isize)
                        as *mut uint8_t as *mut x264_union32_t))
                        .i;
                }
            } else {
                let mut i_3: libc::c_int = 0 as libc::c_int;
                let mut msk_0: libc::c_int = (*h).mb.i_cbp_luma;
                let mut skip_0: libc::c_int = 0;
                while msk_0 != 0 && {
                    skip_0 = x264_ctz_4bit(msk_0 as uint32_t);
                    i_3 += skip_0;
                    msk_0 >>= skip_0 + 1 as libc::c_int;
                    1 as libc::c_int != 0
                } {
                    cabac_block_residual(
                        h,
                        cb,
                        DCT_LUMA_8x8 as libc::c_int,
                        ((*h).dct.luma8x8[i_3 as usize]).as_mut_ptr(),
                    );
                    i_3 += 1;
                    i_3;
                }
            }
        } else {
            let mut p_2: libc::c_int = 0 as libc::c_int;
            while p_2 < plane_count {
                let mut i8x8: libc::c_int = 0 as libc::c_int;
                let mut msk_1: libc::c_int = (*h).mb.i_cbp_luma;
                let mut skip_1: libc::c_int = 0;
                while msk_1 != 0 && {
                    skip_1 = x264_ctz_4bit(msk_1 as uint32_t);
                    i8x8 += skip_1;
                    msk_1 >>= skip_1 + 1 as libc::c_int;
                    1 as libc::c_int != 0
                } {
                    let mut i_4: libc::c_int = 0 as libc::c_int;
                    while i_4 < 4 as libc::c_int {
                        let mut ctxidxinc_2: libc::c_int = cabac_cbf_ctxidxinc(
                            h,
                            ctx_cat_plane[DCT_LUMA_4x4 as libc::c_int as usize][p_2 as usize]
                                as libc::c_int,
                            i_4 + i8x8 * 4 as libc::c_int + p_2 * 16 as libc::c_int,
                            b_intra,
                            0 as libc::c_int,
                        );
                        if (*h).mb.cache.non_zero_count[x264_scan8
                            [(i_4 + i8x8 * 4 as libc::c_int + p_2 * 16 as libc::c_int) as usize]
                            as usize]
                            != 0
                        {
                            x264_8_cabac_encode_decision_c(cb, ctxidxinc_2, 1 as libc::c_int);
                            cabac_block_residual(
                                h,
                                cb,
                                ctx_cat_plane[DCT_LUMA_4x4 as libc::c_int as usize][p_2 as usize]
                                    as libc::c_int,
                                ((*h).dct.luma4x4[(i_4
                                    + i8x8 * 4 as libc::c_int
                                    + p_2 * 16 as libc::c_int)
                                    as usize])
                                    .as_mut_ptr(),
                            );
                        } else {
                            x264_8_cabac_encode_decision_c(cb, ctxidxinc_2, 0 as libc::c_int);
                        }
                        i_4 += 1;
                        i_4;
                    }
                    i8x8 += 1;
                    i8x8;
                }
                p_2 += 1;
                p_2;
            }
        }
        if chroma != 0 && (*h).mb.i_cbp_chroma != 0 {
            if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_422 as libc::c_int {
                let mut ctxidxinc_3: libc::c_int = cabac_cbf_ctxidxinc(
                    h,
                    DCT_CHROMA_DC as libc::c_int,
                    49 as libc::c_int + 0 as libc::c_int,
                    b_intra,
                    1 as libc::c_int,
                );
                if (*h).mb.cache.non_zero_count
                    [x264_scan8[(49 as libc::c_int + 0 as libc::c_int) as usize] as usize]
                    != 0
                {
                    x264_8_cabac_encode_decision_c(cb, ctxidxinc_3, 1 as libc::c_int);
                    cabac_block_residual_422_dc(
                        h,
                        cb,
                        DCT_CHROMA_DC as libc::c_int,
                        ((*h).dct.chroma_dc[0 as libc::c_int as usize]).as_mut_ptr(),
                    );
                } else {
                    x264_8_cabac_encode_decision_c(cb, ctxidxinc_3, 0 as libc::c_int);
                }
                let mut ctxidxinc_4: libc::c_int = cabac_cbf_ctxidxinc(
                    h,
                    DCT_CHROMA_DC as libc::c_int,
                    49 as libc::c_int + 1 as libc::c_int,
                    b_intra,
                    1 as libc::c_int,
                );
                if (*h).mb.cache.non_zero_count
                    [x264_scan8[(49 as libc::c_int + 1 as libc::c_int) as usize] as usize]
                    != 0
                {
                    x264_8_cabac_encode_decision_c(cb, ctxidxinc_4, 1 as libc::c_int);
                    cabac_block_residual_422_dc(
                        h,
                        cb,
                        DCT_CHROMA_DC as libc::c_int,
                        ((*h).dct.chroma_dc[1 as libc::c_int as usize]).as_mut_ptr(),
                    );
                } else {
                    x264_8_cabac_encode_decision_c(cb, ctxidxinc_4, 0 as libc::c_int);
                }
            } else {
                let mut ctxidxinc_5: libc::c_int = cabac_cbf_ctxidxinc(
                    h,
                    DCT_CHROMA_DC as libc::c_int,
                    49 as libc::c_int + 0 as libc::c_int,
                    b_intra,
                    1 as libc::c_int,
                );
                if (*h).mb.cache.non_zero_count
                    [x264_scan8[(49 as libc::c_int + 0 as libc::c_int) as usize] as usize]
                    != 0
                {
                    x264_8_cabac_encode_decision_c(cb, ctxidxinc_5, 1 as libc::c_int);
                    cabac_block_residual(
                        h,
                        cb,
                        DCT_CHROMA_DC as libc::c_int,
                        ((*h).dct.chroma_dc[0 as libc::c_int as usize]).as_mut_ptr(),
                    );
                } else {
                    x264_8_cabac_encode_decision_c(cb, ctxidxinc_5, 0 as libc::c_int);
                }
                let mut ctxidxinc_6: libc::c_int = cabac_cbf_ctxidxinc(
                    h,
                    DCT_CHROMA_DC as libc::c_int,
                    49 as libc::c_int + 1 as libc::c_int,
                    b_intra,
                    1 as libc::c_int,
                );
                if (*h).mb.cache.non_zero_count
                    [x264_scan8[(49 as libc::c_int + 1 as libc::c_int) as usize] as usize]
                    != 0
                {
                    x264_8_cabac_encode_decision_c(cb, ctxidxinc_6, 1 as libc::c_int);
                    cabac_block_residual(
                        h,
                        cb,
                        DCT_CHROMA_DC as libc::c_int,
                        ((*h).dct.chroma_dc[1 as libc::c_int as usize]).as_mut_ptr(),
                    );
                } else {
                    x264_8_cabac_encode_decision_c(cb, ctxidxinc_6, 0 as libc::c_int);
                }
            }
            if (*h).mb.i_cbp_chroma == 2 as libc::c_int {
                let mut step: libc::c_int = (8 as libc::c_int) << (*h).mb.chroma_v_shift;
                let mut i_5: libc::c_int = 16 as libc::c_int;
                while i_5 < 3 as libc::c_int * 16 as libc::c_int {
                    let mut j_0: libc::c_int = i_5;
                    while j_0 < i_5 + 4 as libc::c_int {
                        let mut ctxidxinc_7: libc::c_int = cabac_cbf_ctxidxinc(
                            h,
                            DCT_CHROMA_AC as libc::c_int,
                            j_0,
                            b_intra,
                            0 as libc::c_int,
                        );
                        if (*h).mb.cache.non_zero_count[x264_scan8[j_0 as usize] as usize] != 0 {
                            x264_8_cabac_encode_decision_c(cb, ctxidxinc_7, 1 as libc::c_int);
                            cabac_block_residual(
                                h,
                                cb,
                                DCT_CHROMA_AC as libc::c_int,
                                ((*h).dct.luma4x4[j_0 as usize])
                                    .as_mut_ptr()
                                    .offset(1 as libc::c_int as isize),
                            );
                        } else {
                            x264_8_cabac_encode_decision_c(cb, ctxidxinc_7, 0 as libc::c_int);
                        }
                        j_0 += 1;
                        j_0;
                    }
                    i_5 += step;
                }
            }
        }
    }
    (*h).stat.frame.i_tex_bits += x264_cabac_pos(cb) - i_mb_pos_tex;
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_macroblock_write_cabac(
    mut h: *mut x264_t,
    mut cb: *mut x264_cabac_t,
) {
    if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as libc::c_int {
        macroblock_write_cabac_internal(h, cb, 3 as libc::c_int, 0 as libc::c_int);
    } else if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc != 0 {
        macroblock_write_cabac_internal(h, cb, 1 as libc::c_int, 1 as libc::c_int);
    } else {
        macroblock_write_cabac_internal(h, cb, 1 as libc::c_int, 0 as libc::c_int);
    };
}
