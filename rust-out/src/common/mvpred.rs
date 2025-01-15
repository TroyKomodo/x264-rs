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
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
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
#[inline(always)]
unsafe extern "C" fn x264_median(
    mut a: libc::c_int,
    mut b: libc::c_int,
    mut c: libc::c_int,
) -> libc::c_int {
    let mut t: libc::c_int = (a - b) & ((a - b) >> 31 as libc::c_int);
    a -= t;
    b += t;
    b -= (b - c) & ((b - c) >> 31 as libc::c_int);
    b += (a - b) & ((a - b) >> 31 as libc::c_int);
    b
}
#[inline(always)]
unsafe extern "C" fn x264_median_mv(
    mut dst: *mut int16_t,
    mut a: *mut int16_t,
    mut b: *mut int16_t,
    mut c: *mut int16_t,
) {
    *dst.offset(0 as libc::c_int as isize) = x264_median(
        *a.offset(0 as libc::c_int as isize) as libc::c_int,
        *b.offset(0 as libc::c_int as isize) as libc::c_int,
        *c.offset(0 as libc::c_int as isize) as libc::c_int,
    ) as int16_t;
    *dst.offset(1 as libc::c_int as isize) = x264_median(
        *a.offset(1 as libc::c_int as isize) as libc::c_int,
        *b.offset(1 as libc::c_int as isize) as libc::c_int,
        *c.offset(1 as libc::c_int as isize) as libc::c_int,
    ) as int16_t;
}
#[inline(always)]
unsafe extern "C" fn pack16to32_mask(mut a: libc::c_int, mut b: libc::c_int) -> uint32_t {
    ((a & 0xffff as libc::c_int) as uint32_t).wrapping_add((b as uint32_t) << 16 as libc::c_int)
}
#[inline(always)]
unsafe extern "C" fn x264_macroblock_cache_mv(
    mut h: *mut x264_t,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut i_list: libc::c_int,
    mut mv: uint32_t,
) {
    let mut mv_cache: *mut libc::c_void =
        &mut *(*((*h).mb.cache.mv).as_mut_ptr().offset(i_list as isize))
            .as_mut_ptr()
            .offset(
                (4 as libc::c_int + 1 as libc::c_int * 8 as libc::c_int + x + 8 as libc::c_int * y)
                    as isize,
            ) as *mut [int16_t; 2] as *mut libc::c_void;
    if 0 == 0 || 0 == 0 {
        (x264_8_cache_mv_func_table
            [(width + (height << 1 as libc::c_int) - 3 as libc::c_int) as usize])
            .expect("non-null function pointer")(mv_cache, mv);
    } else {
        x264_macroblock_cache_rect(
            mv_cache,
            width * 4 as libc::c_int,
            height,
            4 as libc::c_int,
            mv,
        );
    };
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
                b"common/rectangle.h\0" as *const u8 as *const libc::c_char,
                82 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 65], &[libc::c_char; 65]>(
                    b"void x264_macroblock_cache_rect(void *, int, int, int, uint32_t)\0",
                ))
                .as_ptr(),
            );
        }
        'c_29086: {
            if h != 1 as libc::c_int {
            } else {
                __assert_fail(
                    b"h != 1\0" as *const u8 as *const libc::c_char,
                    b"common/rectangle.h\0" as *const u8 as *const libc::c_char,
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
            b"common/rectangle.h\0" as *const u8 as *const libc::c_char,
            118 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 65], &[libc::c_char; 65]>(
                b"void x264_macroblock_cache_rect(void *, int, int, int, uint32_t)\0",
            ))
            .as_ptr(),
        );
        'c_28853: {
            __assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"common/rectangle.h\0" as *const u8 as *const libc::c_char,
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
unsafe extern "C" fn x264_macroblock_cache_ref(
    mut h: *mut x264_t,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut i_list: libc::c_int,
    mut ref_0: int8_t,
) {
    let mut ref_cache: *mut libc::c_void =
        &mut *(*((*h).mb.cache.ref_0).as_mut_ptr().offset(i_list as isize))
            .as_mut_ptr()
            .offset(
                (4 as libc::c_int + 1 as libc::c_int * 8 as libc::c_int + x + 8 as libc::c_int * y)
                    as isize,
            ) as *mut int8_t as *mut libc::c_void;
    if 0 == 0 || 0 == 0 {
        (x264_8_cache_ref_func_table
            [(width + (height << 1 as libc::c_int) - 3 as libc::c_int) as usize])
            .expect("non-null function pointer")(ref_cache, ref_0 as uint8_t as uint32_t);
    } else {
        x264_macroblock_cache_rect(
            ref_cache,
            width,
            height,
            1 as libc::c_int,
            ref_0 as uint8_t as uint32_t,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_mb_predict_mv(
    mut h: *mut x264_t,
    mut i_list: libc::c_int,
    mut idx: libc::c_int,
    mut i_width: libc::c_int,
    mut mvp: *mut int16_t,
) {
    let i8: libc::c_int = x264_scan8[idx as usize] as libc::c_int;
    let i_ref: libc::c_int = (*h).mb.cache.ref_0[i_list as usize][i8 as usize] as libc::c_int;
    let mut i_refa: libc::c_int =
        (*h).mb.cache.ref_0[i_list as usize][(i8 - 1 as libc::c_int) as usize] as libc::c_int;
    let mut mv_a: *mut int16_t =
        ((*h).mb.cache.mv[i_list as usize][(i8 - 1 as libc::c_int) as usize]).as_mut_ptr();
    let mut i_refb: libc::c_int =
        (*h).mb.cache.ref_0[i_list as usize][(i8 - 8 as libc::c_int) as usize] as libc::c_int;
    let mut mv_b: *mut int16_t =
        ((*h).mb.cache.mv[i_list as usize][(i8 - 8 as libc::c_int) as usize]).as_mut_ptr();
    let mut i_refc: libc::c_int = (*h).mb.cache.ref_0[i_list as usize]
        [(i8 - 8 as libc::c_int + i_width) as usize]
        as libc::c_int;
    let mut mv_c: *mut int16_t = ((*h).mb.cache.mv[i_list as usize]
        [(i8 - 8 as libc::c_int + i_width) as usize])
        .as_mut_ptr();
    if idx & 3 as libc::c_int >= 2 as libc::c_int + (i_width & 1 as libc::c_int)
        || i_refc == -(2 as libc::c_int)
    {
        i_refc = (*h).mb.cache.ref_0[i_list as usize]
            [(i8 - 8 as libc::c_int - 1 as libc::c_int) as usize] as libc::c_int;
        mv_c = ((*h).mb.cache.mv[i_list as usize]
            [(i8 - 8 as libc::c_int - 1 as libc::c_int) as usize])
            .as_mut_ptr();
        if (*h).sh.b_mbaff != 0
            && (*h).mb.cache.ref_0[i_list as usize]
                [(x264_scan8[0 as libc::c_int as usize] as libc::c_int - 1 as libc::c_int) as usize]
                as libc::c_int
                != -(2 as libc::c_int)
            && (*h).mb.b_interlaced
                != *((*h).mb.field).offset((*h).mb.i_mb_left_xy[0 as libc::c_int as usize] as isize)
                    as libc::c_int
        {
            if idx == 2 as libc::c_int {
                mv_c = ((*h).mb.cache.topright_mv[i_list as usize][0 as libc::c_int as usize])
                    .as_mut_ptr();
                i_refc = (*h).mb.cache.topright_ref[i_list as usize][0 as libc::c_int as usize]
                    as libc::c_int;
            } else if idx == 8 as libc::c_int {
                mv_c = ((*h).mb.cache.topright_mv[i_list as usize][1 as libc::c_int as usize])
                    .as_mut_ptr();
                i_refc = (*h).mb.cache.topright_ref[i_list as usize][1 as libc::c_int as usize]
                    as libc::c_int;
            } else if idx == 10 as libc::c_int {
                mv_c = ((*h).mb.cache.topright_mv[i_list as usize][2 as libc::c_int as usize])
                    .as_mut_ptr();
                i_refc = (*h).mb.cache.topright_ref[i_list as usize][2 as libc::c_int as usize]
                    as libc::c_int;
            }
        }
    }
    if (*h).mb.i_partition == D_16x8 as libc::c_int {
        if idx == 0 as libc::c_int {
            if i_refb == i_ref {
                (*(mvp as *mut x264_union32_t)).i = (*(mv_b as *mut x264_union32_t)).i;
                return;
            }
        } else if i_refa == i_ref {
            (*(mvp as *mut x264_union32_t)).i = (*(mv_a as *mut x264_union32_t)).i;
            return;
        }
    } else if (*h).mb.i_partition == D_8x16 as libc::c_int {
        if idx == 0 as libc::c_int {
            if i_refa == i_ref {
                (*(mvp as *mut x264_union32_t)).i = (*(mv_a as *mut x264_union32_t)).i;
                return;
            }
        } else if i_refc == i_ref {
            (*(mvp as *mut x264_union32_t)).i = (*(mv_c as *mut x264_union32_t)).i;
            return;
        }
    }
    let mut i_count: libc::c_int = (i_refa == i_ref) as libc::c_int
        + (i_refb == i_ref) as libc::c_int
        + (i_refc == i_ref) as libc::c_int;
    let mut current_block_51: u64;
    if i_count > 1 as libc::c_int {
        current_block_51 = 7257199314785017154;
    } else if i_count == 1 as libc::c_int {
        if i_refa == i_ref {
            (*(mvp as *mut x264_union32_t)).i = (*(mv_a as *mut x264_union32_t)).i;
        } else if i_refb == i_ref {
            (*(mvp as *mut x264_union32_t)).i = (*(mv_b as *mut x264_union32_t)).i;
        } else {
            (*(mvp as *mut x264_union32_t)).i = (*(mv_c as *mut x264_union32_t)).i;
        }
        current_block_51 = 10150597327160359210;
    } else if i_refb == -(2 as libc::c_int)
        && i_refc == -(2 as libc::c_int)
        && i_refa != -(2 as libc::c_int)
    {
        (*(mvp as *mut x264_union32_t)).i = (*(mv_a as *mut x264_union32_t)).i;
        current_block_51 = 10150597327160359210;
    } else {
        current_block_51 = 7257199314785017154;
    }
    if current_block_51 == 7257199314785017154 {
        x264_median_mv(mvp, mv_a, mv_b, mv_c);
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_mb_predict_mv_16x16(
    mut h: *mut x264_t,
    mut i_list: libc::c_int,
    mut i_ref: libc::c_int,
    mut mvp: *mut int16_t,
) {
    let mut i_refa: libc::c_int = (*h).mb.cache.ref_0[i_list as usize]
        [(4 as libc::c_int + 1 as libc::c_int * 8 as libc::c_int - 1 as libc::c_int) as usize]
        as libc::c_int;
    let mut mv_a: *mut int16_t = ((*h).mb.cache.mv[i_list as usize]
        [(4 as libc::c_int + 1 as libc::c_int * 8 as libc::c_int - 1 as libc::c_int) as usize])
        .as_mut_ptr();
    let mut i_refb: libc::c_int = (*h).mb.cache.ref_0[i_list as usize]
        [(4 as libc::c_int + 1 as libc::c_int * 8 as libc::c_int - 8 as libc::c_int) as usize]
        as libc::c_int;
    let mut mv_b: *mut int16_t = ((*h).mb.cache.mv[i_list as usize]
        [(4 as libc::c_int + 1 as libc::c_int * 8 as libc::c_int - 8 as libc::c_int) as usize])
        .as_mut_ptr();
    let mut i_refc: libc::c_int = (*h).mb.cache.ref_0[i_list as usize][(4 as libc::c_int
        + 1 as libc::c_int * 8 as libc::c_int
        - 8 as libc::c_int
        + 4 as libc::c_int)
        as usize] as libc::c_int;
    let mut mv_c: *mut int16_t =
        ((*h).mb.cache.mv[i_list as usize][(4 as libc::c_int + 1 as libc::c_int * 8 as libc::c_int
            - 8 as libc::c_int
            + 4 as libc::c_int) as usize])
            .as_mut_ptr();
    if i_refc == -(2 as libc::c_int) {
        i_refc = (*h).mb.cache.ref_0[i_list as usize][(4 as libc::c_int
            + 1 as libc::c_int * 8 as libc::c_int
            - 8 as libc::c_int
            - 1 as libc::c_int) as usize] as libc::c_int;
        mv_c = ((*h).mb.cache.mv[i_list as usize][(4 as libc::c_int
            + 1 as libc::c_int * 8 as libc::c_int
            - 8 as libc::c_int
            - 1 as libc::c_int) as usize])
            .as_mut_ptr();
    }
    let mut i_count: libc::c_int = (i_refa == i_ref) as libc::c_int
        + (i_refb == i_ref) as libc::c_int
        + (i_refc == i_ref) as libc::c_int;
    let mut current_block_11: u64;
    if i_count > 1 as libc::c_int {
        current_block_11 = 10810905470696066182;
    } else if i_count == 1 as libc::c_int {
        if i_refa == i_ref {
            (*(mvp as *mut x264_union32_t)).i = (*(mv_a as *mut x264_union32_t)).i;
        } else if i_refb == i_ref {
            (*(mvp as *mut x264_union32_t)).i = (*(mv_b as *mut x264_union32_t)).i;
        } else {
            (*(mvp as *mut x264_union32_t)).i = (*(mv_c as *mut x264_union32_t)).i;
        }
        current_block_11 = 13056961889198038528;
    } else if i_refb == -(2 as libc::c_int)
        && i_refc == -(2 as libc::c_int)
        && i_refa != -(2 as libc::c_int)
    {
        (*(mvp as *mut x264_union32_t)).i = (*(mv_a as *mut x264_union32_t)).i;
        current_block_11 = 13056961889198038528;
    } else {
        current_block_11 = 10810905470696066182;
    }
    if current_block_11 == 10810905470696066182 {
        x264_median_mv(mvp, mv_a, mv_b, mv_c);
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_mb_predict_mv_pskip(mut h: *mut x264_t, mut mv: *mut int16_t) {
    let mut i_refa: libc::c_int = (*h).mb.cache.ref_0[0 as libc::c_int as usize]
        [(4 as libc::c_int + 1 as libc::c_int * 8 as libc::c_int - 1 as libc::c_int) as usize]
        as libc::c_int;
    let mut i_refb: libc::c_int = (*h).mb.cache.ref_0[0 as libc::c_int as usize]
        [(4 as libc::c_int + 1 as libc::c_int * 8 as libc::c_int - 8 as libc::c_int) as usize]
        as libc::c_int;
    let mut mv_a: *mut int16_t = ((*h).mb.cache.mv[0 as libc::c_int as usize]
        [(4 as libc::c_int + 1 as libc::c_int * 8 as libc::c_int - 1 as libc::c_int) as usize])
        .as_mut_ptr();
    let mut mv_b: *mut int16_t = ((*h).mb.cache.mv[0 as libc::c_int as usize]
        [(4 as libc::c_int + 1 as libc::c_int * 8 as libc::c_int - 8 as libc::c_int) as usize])
        .as_mut_ptr();
    if i_refa == -(2 as libc::c_int)
        || i_refb == -(2 as libc::c_int)
        || i_refa as uint32_t | (*(mv_a as *mut x264_union32_t)).i == 0
        || i_refb as uint32_t | (*(mv_b as *mut x264_union32_t)).i == 0
    {
        (*(mv as *mut x264_union32_t)).i = 0 as libc::c_int as uint32_t;
    } else {
        x264_8_mb_predict_mv_16x16(h, 0 as libc::c_int, 0 as libc::c_int, mv);
    };
}
unsafe extern "C" fn mb_predict_mv_direct16x16_temporal(mut h: *mut x264_t) -> libc::c_int {
    let mut mb_x: libc::c_int = (*h).mb.i_mb_x;
    let mut mb_y: libc::c_int = (*h).mb.i_mb_y;
    let mut mb_xy: libc::c_int = (*h).mb.i_mb_xy;
    let mut type_col: [libc::c_int; 2] = [
        *((*(*h).fref[1 as libc::c_int as usize][0 as libc::c_int as usize]).mb_type)
            .offset(mb_xy as isize) as libc::c_int,
        *((*(*h).fref[1 as libc::c_int as usize][0 as libc::c_int as usize]).mb_type)
            .offset(mb_xy as isize) as libc::c_int,
    ];
    let mut partition_col: [libc::c_int; 2] = [
        *((*(*h).fref[1 as libc::c_int as usize][0 as libc::c_int as usize]).mb_partition)
            .offset(mb_xy as isize) as libc::c_int,
        *((*(*h).fref[1 as libc::c_int as usize][0 as libc::c_int as usize]).mb_partition)
            .offset(mb_xy as isize) as libc::c_int,
    ];
    let mut preshift: libc::c_int = (*h).mb.b_interlaced;
    let mut postshift: libc::c_int = (*h).mb.b_interlaced;
    let mut offset: libc::c_int = 1 as libc::c_int;
    let mut yshift: libc::c_int = 1 as libc::c_int;
    (*h).mb.i_partition = partition_col[0 as libc::c_int as usize];
    if (*h).param.b_interlaced != 0
        && *((*(*h).fref[1 as libc::c_int as usize][0 as libc::c_int as usize]).field)
            .offset(mb_xy as isize) as libc::c_int
            != (*h).mb.b_interlaced
    {
        if (*h).mb.b_interlaced != 0 {
            mb_y = (*h).mb.i_mb_y & !(1 as libc::c_int);
            mb_xy = mb_x + (*h).mb.i_mb_stride * mb_y;
            type_col[0 as libc::c_int as usize] =
                *((*(*h).fref[1 as libc::c_int as usize][0 as libc::c_int as usize]).mb_type)
                    .offset(mb_xy as isize) as libc::c_int;
            type_col[1 as libc::c_int as usize] =
                *((*(*h).fref[1 as libc::c_int as usize][0 as libc::c_int as usize]).mb_type)
                    .offset((mb_xy + (*h).mb.i_mb_stride) as isize) as libc::c_int;
            partition_col[0 as libc::c_int as usize] =
                *((*(*h).fref[1 as libc::c_int as usize][0 as libc::c_int as usize]).mb_partition)
                    .offset(mb_xy as isize) as libc::c_int;
            partition_col[1 as libc::c_int as usize] =
                *((*(*h).fref[1 as libc::c_int as usize][0 as libc::c_int as usize]).mb_partition)
                    .offset((mb_xy + (*h).mb.i_mb_stride) as isize) as libc::c_int;
            preshift = 0 as libc::c_int;
            yshift = 0 as libc::c_int;
            if (type_col[0 as libc::c_int as usize] == I_4x4 as libc::c_int
                || type_col[0 as libc::c_int as usize] == I_8x8 as libc::c_int
                || type_col[0 as libc::c_int as usize] == I_16x16 as libc::c_int
                || type_col[0 as libc::c_int as usize] == I_PCM as libc::c_int
                || partition_col[0 as libc::c_int as usize] == D_16x16 as libc::c_int)
                && (type_col[1 as libc::c_int as usize] == I_4x4 as libc::c_int
                    || type_col[1 as libc::c_int as usize] == I_8x8 as libc::c_int
                    || type_col[1 as libc::c_int as usize] == I_16x16 as libc::c_int
                    || type_col[1 as libc::c_int as usize] == I_PCM as libc::c_int
                    || partition_col[1 as libc::c_int as usize] == D_16x16 as libc::c_int)
                && partition_col[0 as libc::c_int as usize] != D_8x8 as libc::c_int
            {
                (*h).mb.i_partition = D_16x8 as libc::c_int;
            } else {
                (*h).mb.i_partition = D_8x8 as libc::c_int;
            }
        } else {
            let mut cur_poc: libc::c_int = (*(*h).fdec).i_poc
                + (*(*h).fdec).i_delta_poc
                    [((*h).mb.b_interlaced & (*h).mb.i_mb_y & 1 as libc::c_int) as usize];
            let mut col_parity: libc::c_int = (abs((*(*h).fref[1 as libc::c_int as usize]
                [0 as libc::c_int as usize])
                .i_poc
                + (*(*h).fref[1 as libc::c_int as usize][0 as libc::c_int as usize]).i_delta_poc
                    [0 as libc::c_int as usize]
                - cur_poc)
                >= abs(
                    (*(*h).fref[1 as libc::c_int as usize][0 as libc::c_int as usize]).i_poc
                        + (*(*h).fref[1 as libc::c_int as usize][0 as libc::c_int as usize])
                            .i_delta_poc[1 as libc::c_int as usize]
                        - cur_poc,
                )) as libc::c_int;
            mb_y = ((*h).mb.i_mb_y & !(1 as libc::c_int)) + col_parity;
            mb_xy = mb_x + (*h).mb.i_mb_stride * mb_y;
            type_col[1 as libc::c_int as usize] =
                *((*(*h).fref[1 as libc::c_int as usize][0 as libc::c_int as usize]).mb_type)
                    .offset(mb_xy as isize) as libc::c_int;
            type_col[0 as libc::c_int as usize] = type_col[1 as libc::c_int as usize];
            partition_col[1 as libc::c_int as usize] =
                *((*(*h).fref[1 as libc::c_int as usize][0 as libc::c_int as usize]).mb_partition)
                    .offset(mb_xy as isize) as libc::c_int;
            partition_col[0 as libc::c_int as usize] = partition_col[1 as libc::c_int as usize];
            preshift = 1 as libc::c_int;
            yshift = 2 as libc::c_int;
            (*h).mb.i_partition = partition_col[0 as libc::c_int as usize];
        }
        offset = 0 as libc::c_int;
    }
    let mut i_mb_4x4: libc::c_int =
        16 as libc::c_int * (*h).mb.i_mb_stride * mb_y + 4 as libc::c_int * mb_x;
    let mut i_mb_8x8: libc::c_int =
        4 as libc::c_int * (*h).mb.i_mb_stride * mb_y + 2 as libc::c_int * mb_x;
    x264_macroblock_cache_ref(
        h,
        0 as libc::c_int,
        0 as libc::c_int,
        4 as libc::c_int,
        4 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int as int8_t,
    );
    let mut max_i8: libc::c_int = D_16x16 as libc::c_int - (*h).mb.i_partition + 1 as libc::c_int;
    let mut step: libc::c_int =
        ((*h).mb.i_partition == D_16x8 as libc::c_int) as libc::c_int + 1 as libc::c_int;
    let mut width: libc::c_int =
        4 as libc::c_int >> ((D_16x16 as libc::c_int - (*h).mb.i_partition) & 1 as libc::c_int);
    let mut height: libc::c_int =
        4 as libc::c_int >> ((D_16x16 as libc::c_int - (*h).mb.i_partition) >> 1 as libc::c_int);
    let mut i8: libc::c_int = 0 as libc::c_int;
    while i8 < max_i8 {
        let mut x8: libc::c_int = i8 & 1 as libc::c_int;
        let mut y8: libc::c_int = i8 >> 1 as libc::c_int;
        let mut ypart: libc::c_int = if (*h).sh.b_mbaff != 0
            && *((*(*h).fref[1 as libc::c_int as usize][0 as libc::c_int as usize]).field)
                .offset(mb_xy as isize) as libc::c_int
                != (*h).mb.b_interlaced
        {
            if (*h).mb.b_interlaced != 0 {
                y8 * 6 as libc::c_int
            } else {
                2 as libc::c_int * ((*h).mb.i_mb_y & 1 as libc::c_int) + y8
            }
        } else {
            3 as libc::c_int * y8
        };
        if type_col[y8 as usize] == I_4x4 as libc::c_int
            || type_col[y8 as usize] == I_8x8 as libc::c_int
            || type_col[y8 as usize] == I_16x16 as libc::c_int
            || type_col[y8 as usize] == I_PCM as libc::c_int
        {
            x264_macroblock_cache_ref(
                h,
                2 as libc::c_int * x8,
                2 as libc::c_int * y8,
                width,
                height,
                0 as libc::c_int,
                0 as libc::c_int as int8_t,
            );
            x264_macroblock_cache_mv(
                h,
                2 as libc::c_int * x8,
                2 as libc::c_int * y8,
                width,
                height,
                0 as libc::c_int,
                0 as libc::c_int as uint32_t,
            );
            x264_macroblock_cache_mv(
                h,
                2 as libc::c_int * x8,
                2 as libc::c_int * y8,
                width,
                height,
                1 as libc::c_int,
                0 as libc::c_int as uint32_t,
            );
        } else {
            let mut i_part_8x8: libc::c_int =
                i_mb_8x8 + x8 + (ypart >> 1 as libc::c_int) * (*h).mb.i_b8_stride;
            let mut i_ref1_ref: libc::c_int =
                *((*(*h).fref[1 as libc::c_int as usize][0 as libc::c_int as usize]).ref_0
                    [0 as libc::c_int as usize])
                    .offset(i_part_8x8 as isize) as libc::c_int;
            let mut i_ref: libc::c_int = (*h).mb.map_col_to_list0
                [((i_ref1_ref >> preshift) + 2 as libc::c_int) as usize]
                as libc::c_int
                * ((1 as libc::c_int) << postshift)
                + (offset & i_ref1_ref & (*h).mb.b_interlaced);
            if i_ref >= 0 as libc::c_int {
                let mut dist_scale_factor: libc::c_int = (*((*h).mb.dist_scale_factor)
                    .offset(i_ref as isize))[0 as libc::c_int as usize]
                    as libc::c_int;
                let mut mv_col: *mut int16_t = (*((*(*h).fref[1 as libc::c_int as usize]
                    [0 as libc::c_int as usize])
                    .mv[0 as libc::c_int as usize])
                    .offset(
                        (i_mb_4x4 + 3 as libc::c_int * x8 + ypart * (*h).mb.i_b4_stride) as isize,
                    ))
                .as_mut_ptr();
                let mut mv_y: int16_t = (*mv_col.offset(1 as libc::c_int as isize) as libc::c_int
                    * ((1 as libc::c_int) << yshift)
                    / 2 as libc::c_int) as int16_t;
                let mut l0x: libc::c_int = (dist_scale_factor
                    * *mv_col.offset(0 as libc::c_int as isize) as libc::c_int
                    + 128 as libc::c_int)
                    >> 8 as libc::c_int;
                let mut l0y: libc::c_int = (dist_scale_factor * mv_y as libc::c_int
                    + 128 as libc::c_int)
                    >> 8 as libc::c_int;
                if (*h).param.i_threads > 1 as libc::c_int
                    && (l0y > (*h).mb.mv_max_spel[1 as libc::c_int as usize]
                        || l0y - mv_y as libc::c_int
                            > (*h).mb.mv_max_spel[1 as libc::c_int as usize])
                {
                    return 0 as libc::c_int;
                }
                x264_macroblock_cache_ref(
                    h,
                    2 as libc::c_int * x8,
                    2 as libc::c_int * y8,
                    width,
                    height,
                    0 as libc::c_int,
                    i_ref as int8_t,
                );
                x264_macroblock_cache_mv(
                    h,
                    2 as libc::c_int * x8,
                    2 as libc::c_int * y8,
                    width,
                    height,
                    0 as libc::c_int,
                    pack16to32_mask(l0x, l0y),
                );
                x264_macroblock_cache_mv(
                    h,
                    2 as libc::c_int * x8,
                    2 as libc::c_int * y8,
                    width,
                    height,
                    1 as libc::c_int,
                    pack16to32_mask(
                        l0x - *mv_col.offset(0 as libc::c_int as isize) as libc::c_int,
                        l0y - mv_y as libc::c_int,
                    ),
                );
            } else {
                return 0 as libc::c_int;
            }
        }
        i8 += step;
    }
    1 as libc::c_int
}
#[inline(always)]
unsafe extern "C" fn mb_predict_mv_direct16x16_spatial(
    mut h: *mut x264_t,
    mut b_interlaced: libc::c_int,
) -> libc::c_int {
    let mut ref_0: [int8_t; 2] = [0; 2];
    let mut mv: [[int16_t; 2]; 2] = [[0; 2]; 2];
    let mut i_list: libc::c_int = 0 as libc::c_int;
    while i_list < 2 as libc::c_int {
        let mut i_refa: libc::c_int = (*h).mb.cache.ref_0[i_list as usize]
            [(4 as libc::c_int + 1 as libc::c_int * 8 as libc::c_int - 1 as libc::c_int) as usize]
            as libc::c_int;
        let mut mv_a: *mut int16_t = ((*h).mb.cache.mv[i_list as usize]
            [(4 as libc::c_int + 1 as libc::c_int * 8 as libc::c_int - 1 as libc::c_int) as usize])
            .as_mut_ptr();
        let mut i_refb: libc::c_int = (*h).mb.cache.ref_0[i_list as usize]
            [(4 as libc::c_int + 1 as libc::c_int * 8 as libc::c_int - 8 as libc::c_int) as usize]
            as libc::c_int;
        let mut mv_b: *mut int16_t = ((*h).mb.cache.mv[i_list as usize]
            [(4 as libc::c_int + 1 as libc::c_int * 8 as libc::c_int - 8 as libc::c_int) as usize])
            .as_mut_ptr();
        let mut i_refc: libc::c_int = (*h).mb.cache.ref_0[i_list as usize][(4 as libc::c_int
            + 1 as libc::c_int * 8 as libc::c_int
            - 8 as libc::c_int
            + 4 as libc::c_int)
            as usize] as libc::c_int;
        let mut mv_c: *mut int16_t = ((*h).mb.cache.mv[i_list as usize][(4 as libc::c_int
            + 1 as libc::c_int * 8 as libc::c_int
            - 8 as libc::c_int
            + 4 as libc::c_int)
            as usize])
            .as_mut_ptr();
        if i_refc == -(2 as libc::c_int) {
            i_refc = (*h).mb.cache.ref_0[i_list as usize][(4 as libc::c_int
                + 1 as libc::c_int * 8 as libc::c_int
                - 8 as libc::c_int
                - 1 as libc::c_int)
                as usize] as libc::c_int;
            mv_c = ((*h).mb.cache.mv[i_list as usize][(4 as libc::c_int
                + 1 as libc::c_int * 8 as libc::c_int
                - 8 as libc::c_int
                - 1 as libc::c_int)
                as usize])
                .as_mut_ptr();
        }
        let mut i_ref: libc::c_int = (if (i_refa as libc::c_uint)
            < (if (i_refb as libc::c_uint) < i_refc as libc::c_uint {
                i_refb as libc::c_uint
            } else {
                i_refc as libc::c_uint
            }) {
            i_refa as libc::c_uint
        } else if (i_refb as libc::c_uint) < i_refc as libc::c_uint {
            i_refb as libc::c_uint
        } else {
            i_refc as libc::c_uint
        }) as libc::c_int;
        if i_ref < 0 as libc::c_int {
            i_ref = -(1 as libc::c_int);
            (*((mv[i_list as usize]).as_mut_ptr() as *mut x264_union32_t)).i =
                0 as libc::c_int as uint32_t;
        } else {
            let mut i_count: libc::c_int = (i_refa == i_ref) as libc::c_int
                + (i_refb == i_ref) as libc::c_int
                + (i_refc == i_ref) as libc::c_int;
            if i_count > 1 as libc::c_int {
                x264_median_mv((mv[i_list as usize]).as_mut_ptr(), mv_a, mv_b, mv_c);
            } else if i_refa == i_ref {
                (*((mv[i_list as usize]).as_mut_ptr() as *mut x264_union32_t)).i =
                    (*(mv_a as *mut x264_union32_t)).i;
            } else if i_refb == i_ref {
                (*((mv[i_list as usize]).as_mut_ptr() as *mut x264_union32_t)).i =
                    (*(mv_b as *mut x264_union32_t)).i;
            } else {
                (*((mv[i_list as usize]).as_mut_ptr() as *mut x264_union32_t)).i =
                    (*(mv_c as *mut x264_union32_t)).i;
            }
        }
        x264_macroblock_cache_ref(
            h,
            0 as libc::c_int,
            0 as libc::c_int,
            4 as libc::c_int,
            4 as libc::c_int,
            i_list,
            i_ref as int8_t,
        );
        x264_macroblock_cache_mv(
            h,
            0 as libc::c_int,
            0 as libc::c_int,
            4 as libc::c_int,
            4 as libc::c_int,
            i_list,
            (*((mv[i_list as usize]).as_mut_ptr() as *mut x264_union32_t)).i,
        );
        ref_0[i_list as usize] = i_ref as int8_t;
        i_list += 1;
        i_list;
    }
    let mut mb_x: libc::c_int = (*h).mb.i_mb_x;
    let mut mb_y: libc::c_int = (*h).mb.i_mb_y;
    let mut mb_xy: libc::c_int = (*h).mb.i_mb_xy;
    let mut type_col: [libc::c_int; 2] = [
        *((*(*h).fref[1 as libc::c_int as usize][0 as libc::c_int as usize]).mb_type)
            .offset(mb_xy as isize) as libc::c_int,
        *((*(*h).fref[1 as libc::c_int as usize][0 as libc::c_int as usize]).mb_type)
            .offset(mb_xy as isize) as libc::c_int,
    ];
    let mut partition_col: [libc::c_int; 2] = [
        *((*(*h).fref[1 as libc::c_int as usize][0 as libc::c_int as usize]).mb_partition)
            .offset(mb_xy as isize) as libc::c_int,
        *((*(*h).fref[1 as libc::c_int as usize][0 as libc::c_int as usize]).mb_partition)
            .offset(mb_xy as isize) as libc::c_int,
    ];
    (*h).mb.i_partition = partition_col[0 as libc::c_int as usize];
    if b_interlaced != 0
        && *((*(*h).fref[1 as libc::c_int as usize][0 as libc::c_int as usize]).field)
            .offset(mb_xy as isize) as libc::c_int
            != (*h).mb.b_interlaced
    {
        if (*h).mb.b_interlaced != 0 {
            mb_y = (*h).mb.i_mb_y & !(1 as libc::c_int);
            mb_xy = mb_x + (*h).mb.i_mb_stride * mb_y;
            type_col[0 as libc::c_int as usize] =
                *((*(*h).fref[1 as libc::c_int as usize][0 as libc::c_int as usize]).mb_type)
                    .offset(mb_xy as isize) as libc::c_int;
            type_col[1 as libc::c_int as usize] =
                *((*(*h).fref[1 as libc::c_int as usize][0 as libc::c_int as usize]).mb_type)
                    .offset((mb_xy + (*h).mb.i_mb_stride) as isize) as libc::c_int;
            partition_col[0 as libc::c_int as usize] =
                *((*(*h).fref[1 as libc::c_int as usize][0 as libc::c_int as usize]).mb_partition)
                    .offset(mb_xy as isize) as libc::c_int;
            partition_col[1 as libc::c_int as usize] =
                *((*(*h).fref[1 as libc::c_int as usize][0 as libc::c_int as usize]).mb_partition)
                    .offset((mb_xy + (*h).mb.i_mb_stride) as isize) as libc::c_int;
            if (type_col[0 as libc::c_int as usize] == I_4x4 as libc::c_int
                || type_col[0 as libc::c_int as usize] == I_8x8 as libc::c_int
                || type_col[0 as libc::c_int as usize] == I_16x16 as libc::c_int
                || type_col[0 as libc::c_int as usize] == I_PCM as libc::c_int
                || partition_col[0 as libc::c_int as usize] == D_16x16 as libc::c_int)
                && (type_col[1 as libc::c_int as usize] == I_4x4 as libc::c_int
                    || type_col[1 as libc::c_int as usize] == I_8x8 as libc::c_int
                    || type_col[1 as libc::c_int as usize] == I_16x16 as libc::c_int
                    || type_col[1 as libc::c_int as usize] == I_PCM as libc::c_int
                    || partition_col[1 as libc::c_int as usize] == D_16x16 as libc::c_int)
                && partition_col[0 as libc::c_int as usize] != D_8x8 as libc::c_int
            {
                (*h).mb.i_partition = D_16x8 as libc::c_int;
            } else {
                (*h).mb.i_partition = D_8x8 as libc::c_int;
            }
        } else {
            let mut cur_poc: libc::c_int = (*(*h).fdec).i_poc
                + (*(*h).fdec).i_delta_poc
                    [((*h).mb.b_interlaced & (*h).mb.i_mb_y & 1 as libc::c_int) as usize];
            let mut col_parity: libc::c_int = (abs((*(*h).fref[1 as libc::c_int as usize]
                [0 as libc::c_int as usize])
                .i_poc
                + (*(*h).fref[1 as libc::c_int as usize][0 as libc::c_int as usize]).i_delta_poc
                    [0 as libc::c_int as usize]
                - cur_poc)
                >= abs(
                    (*(*h).fref[1 as libc::c_int as usize][0 as libc::c_int as usize]).i_poc
                        + (*(*h).fref[1 as libc::c_int as usize][0 as libc::c_int as usize])
                            .i_delta_poc[1 as libc::c_int as usize]
                        - cur_poc,
                )) as libc::c_int;
            mb_y = ((*h).mb.i_mb_y & !(1 as libc::c_int)) + col_parity;
            mb_xy = mb_x + (*h).mb.i_mb_stride * mb_y;
            type_col[1 as libc::c_int as usize] =
                *((*(*h).fref[1 as libc::c_int as usize][0 as libc::c_int as usize]).mb_type)
                    .offset(mb_xy as isize) as libc::c_int;
            type_col[0 as libc::c_int as usize] = type_col[1 as libc::c_int as usize];
            partition_col[1 as libc::c_int as usize] =
                *((*(*h).fref[1 as libc::c_int as usize][0 as libc::c_int as usize]).mb_partition)
                    .offset(mb_xy as isize) as libc::c_int;
            partition_col[0 as libc::c_int as usize] = partition_col[1 as libc::c_int as usize];
            (*h).mb.i_partition = partition_col[0 as libc::c_int as usize];
        }
    }
    let mut i_mb_4x4: libc::c_int = if b_interlaced != 0 {
        4 as libc::c_int * ((*h).mb.i_b4_stride * mb_y + mb_x)
    } else {
        (*h).mb.i_b4_xy
    };
    let mut i_mb_8x8: libc::c_int = if b_interlaced != 0 {
        2 as libc::c_int * ((*h).mb.i_b8_stride * mb_y + mb_x)
    } else {
        (*h).mb.i_b8_xy
    };
    let mut l1ref0: *mut int8_t =
        &mut *(*((**(*((*h).fref).as_mut_ptr().offset(1 as libc::c_int as isize))
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize))
        .ref_0)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize))
        .offset(i_mb_8x8 as isize) as *mut int8_t;
    let mut l1ref1: *mut int8_t =
        &mut *(*((**(*((*h).fref).as_mut_ptr().offset(1 as libc::c_int as isize))
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize))
        .ref_0)
            .as_mut_ptr()
            .offset(1 as libc::c_int as isize))
        .offset(i_mb_8x8 as isize) as *mut int8_t;
    let mut l1mv: [*mut [int16_t; 2]; 2] = [
        &mut *(*((**(*((*h).fref).as_mut_ptr().offset(1 as libc::c_int as isize))
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize))
        .mv)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize))
        .offset(i_mb_4x4 as isize) as *mut [int16_t; 2],
        &mut *(*((**(*((*h).fref).as_mut_ptr().offset(1 as libc::c_int as isize))
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize))
        .mv)
            .as_mut_ptr()
            .offset(1 as libc::c_int as isize))
        .offset(i_mb_4x4 as isize) as *mut [int16_t; 2],
    ];
    if (*(ref_0.as_mut_ptr() as *mut x264_union16_t)).i as libc::c_int & 0x8080 as libc::c_int
        == 0x8080 as libc::c_int
    {
        x264_macroblock_cache_ref(
            h,
            0 as libc::c_int,
            0 as libc::c_int,
            4 as libc::c_int,
            4 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int as int8_t,
        );
        x264_macroblock_cache_ref(
            h,
            0 as libc::c_int,
            0 as libc::c_int,
            4 as libc::c_int,
            4 as libc::c_int,
            1 as libc::c_int,
            0 as libc::c_int as int8_t,
        );
        return 1 as libc::c_int;
    }
    if (*h).param.i_threads > 1 as libc::c_int
        && (mv[0 as libc::c_int as usize][1 as libc::c_int as usize] as libc::c_int
            > (*h).mb.mv_max_spel[1 as libc::c_int as usize]
            || mv[1 as libc::c_int as usize][1 as libc::c_int as usize] as libc::c_int
                > (*h).mb.mv_max_spel[1 as libc::c_int as usize])
    {
        return 0 as libc::c_int;
    }
    if (*(mv.as_mut_ptr() as *mut x264_union64_t)).i == 0
        || b_interlaced == 0
            && (type_col[0 as libc::c_int as usize] == I_4x4 as libc::c_int
                || type_col[0 as libc::c_int as usize] == I_8x8 as libc::c_int
                || type_col[0 as libc::c_int as usize] == I_16x16 as libc::c_int
                || type_col[0 as libc::c_int as usize] == I_PCM as libc::c_int)
        || ref_0[0 as libc::c_int as usize] as libc::c_int != 0
            && ref_0[1 as libc::c_int as usize] as libc::c_int != 0
    {
        return 1 as libc::c_int;
    }
    let mut max_i8: libc::c_int = D_16x16 as libc::c_int - (*h).mb.i_partition + 1 as libc::c_int;
    let mut step: libc::c_int =
        ((*h).mb.i_partition == D_16x8 as libc::c_int) as libc::c_int + 1 as libc::c_int;
    let mut width: libc::c_int =
        4 as libc::c_int >> ((D_16x16 as libc::c_int - (*h).mb.i_partition) & 1 as libc::c_int);
    let mut height: libc::c_int =
        4 as libc::c_int >> ((D_16x16 as libc::c_int - (*h).mb.i_partition) >> 1 as libc::c_int);
    let mut current_block_59: u64;
    let mut i8: libc::c_int = 0 as libc::c_int;
    while i8 < max_i8 {
        let x8: libc::c_int = i8 & 1 as libc::c_int;
        let y8: libc::c_int = i8 >> 1 as libc::c_int;
        let mut ypart: libc::c_int = if b_interlaced != 0
            && *((*(*h).fref[1 as libc::c_int as usize][0 as libc::c_int as usize]).field)
                .offset(mb_xy as isize) as libc::c_int
                != (*h).mb.b_interlaced
        {
            if (*h).mb.b_interlaced != 0 {
                y8 * 6 as libc::c_int
            } else {
                2 as libc::c_int * ((*h).mb.i_mb_y & 1 as libc::c_int) + y8
            }
        } else {
            3 as libc::c_int * y8
        };
        let mut o8: libc::c_int = x8 + (ypart >> 1 as libc::c_int) * (*h).mb.i_b8_stride;
        let mut o4: libc::c_int = 3 as libc::c_int * x8 + ypart * (*h).mb.i_b4_stride;
        if !(b_interlaced != 0
            && (type_col[y8 as usize] == I_4x4 as libc::c_int
                || type_col[y8 as usize] == I_8x8 as libc::c_int
                || type_col[y8 as usize] == I_16x16 as libc::c_int
                || type_col[y8 as usize] == I_PCM as libc::c_int))
        {
            let mut idx: libc::c_int = 0;
            if *l1ref0.offset(o8 as isize) as libc::c_int == 0 as libc::c_int {
                idx = 0 as libc::c_int;
                current_block_59 = 6528285054092551010;
            } else if (*l1ref0.offset(o8 as isize) as libc::c_int) < 0 as libc::c_int
                && *l1ref1.offset(o8 as isize) as libc::c_int == 0 as libc::c_int
            {
                idx = 1 as libc::c_int;
                current_block_59 = 6528285054092551010;
            } else {
                current_block_59 = 1423531122933789233;
            }
            match current_block_59 {
                1423531122933789233 => {}
                _ => {
                    if abs(
                        (*(l1mv[idx as usize]).offset(o4 as isize))[0 as libc::c_int as usize]
                            as libc::c_int,
                    ) <= 1 as libc::c_int
                        && abs((*(l1mv[idx as usize]).offset(o4 as isize))
                            [1 as libc::c_int as usize]
                            as libc::c_int)
                            <= 1 as libc::c_int
                    {
                        if ref_0[0 as libc::c_int as usize] as libc::c_int == 0 as libc::c_int {
                            x264_macroblock_cache_mv(
                                h,
                                2 as libc::c_int * x8,
                                2 as libc::c_int * y8,
                                width,
                                height,
                                0 as libc::c_int,
                                0 as libc::c_int as uint32_t,
                            );
                        }
                        if ref_0[1 as libc::c_int as usize] as libc::c_int == 0 as libc::c_int {
                            x264_macroblock_cache_mv(
                                h,
                                2 as libc::c_int * x8,
                                2 as libc::c_int * y8,
                                width,
                                height,
                                1 as libc::c_int,
                                0 as libc::c_int as uint32_t,
                            );
                        }
                    }
                }
            }
        }
        i8 += step;
    }
    1 as libc::c_int
}
unsafe extern "C" fn mb_predict_mv_direct16x16_spatial_interlaced(
    mut h: *mut x264_t,
) -> libc::c_int {
    mb_predict_mv_direct16x16_spatial(h, 1 as libc::c_int)
}
unsafe extern "C" fn mb_predict_mv_direct16x16_spatial_progressive(
    mut h: *mut x264_t,
) -> libc::c_int {
    mb_predict_mv_direct16x16_spatial(h, 0 as libc::c_int)
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_mb_predict_mv_direct16x16(
    mut h: *mut x264_t,
    mut b_changed: *mut libc::c_int,
) -> libc::c_int {
    let mut b_available: libc::c_int = 0;
    if (*h).param.analyse.i_direct_mv_pred == 0 as libc::c_int {
        return 0 as libc::c_int;
    } else if (*h).sh.b_direct_spatial_mv_pred != 0 {
        if (*h).sh.b_mbaff != 0 {
            b_available = mb_predict_mv_direct16x16_spatial_interlaced(h);
        } else {
            b_available = mb_predict_mv_direct16x16_spatial_progressive(h);
        }
    } else {
        b_available = mb_predict_mv_direct16x16_temporal(h);
    }
    if !b_changed.is_null() && b_available != 0 {
        let mut changed: libc::c_int = 0;
        changed = ((*(((*h).mb.cache.direct_mv[0 as libc::c_int as usize]
            [0 as libc::c_int as usize])
            .as_mut_ptr() as *mut x264_union32_t))
            .i
            ^ (*(((*h).mb.cache.mv[0 as libc::c_int as usize]
                [x264_scan8[0 as libc::c_int as usize] as usize])
                .as_mut_ptr() as *mut x264_union32_t))
                .i) as libc::c_int;
        changed |= ((*(((*h).mb.cache.direct_mv[1 as libc::c_int as usize]
            [0 as libc::c_int as usize])
            .as_mut_ptr() as *mut x264_union32_t))
            .i
            ^ (*(((*h).mb.cache.mv[1 as libc::c_int as usize]
                [x264_scan8[0 as libc::c_int as usize] as usize])
                .as_mut_ptr() as *mut x264_union32_t))
                .i) as libc::c_int;
        changed |= (*h).mb.cache.direct_ref[0 as libc::c_int as usize][0 as libc::c_int as usize]
            as libc::c_int
            ^ (*h).mb.cache.ref_0[0 as libc::c_int as usize]
                [x264_scan8[0 as libc::c_int as usize] as usize] as libc::c_int;
        changed |= (*h).mb.cache.direct_ref[1 as libc::c_int as usize][0 as libc::c_int as usize]
            as libc::c_int
            ^ (*h).mb.cache.ref_0[1 as libc::c_int as usize]
                [x264_scan8[0 as libc::c_int as usize] as usize] as libc::c_int;
        if changed == 0 && (*h).mb.i_partition != D_16x16 as libc::c_int {
            changed |= ((*(((*h).mb.cache.direct_mv[0 as libc::c_int as usize]
                [3 as libc::c_int as usize])
                .as_mut_ptr() as *mut x264_union32_t))
                .i
                ^ (*(((*h).mb.cache.mv[0 as libc::c_int as usize]
                    [x264_scan8[12 as libc::c_int as usize] as usize])
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i) as libc::c_int;
            changed |= ((*(((*h).mb.cache.direct_mv[1 as libc::c_int as usize]
                [3 as libc::c_int as usize])
                .as_mut_ptr() as *mut x264_union32_t))
                .i
                ^ (*(((*h).mb.cache.mv[1 as libc::c_int as usize]
                    [x264_scan8[12 as libc::c_int as usize] as usize])
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i) as libc::c_int;
            changed |= (*h).mb.cache.direct_ref[0 as libc::c_int as usize]
                [3 as libc::c_int as usize] as libc::c_int
                ^ (*h).mb.cache.ref_0[0 as libc::c_int as usize]
                    [x264_scan8[12 as libc::c_int as usize] as usize]
                    as libc::c_int;
            changed |= (*h).mb.cache.direct_ref[1 as libc::c_int as usize]
                [3 as libc::c_int as usize] as libc::c_int
                ^ (*h).mb.cache.ref_0[1 as libc::c_int as usize]
                    [x264_scan8[12 as libc::c_int as usize] as usize]
                    as libc::c_int;
        }
        if changed == 0 && (*h).mb.i_partition == D_8x8 as libc::c_int {
            changed |= ((*(((*h).mb.cache.direct_mv[0 as libc::c_int as usize]
                [1 as libc::c_int as usize])
                .as_mut_ptr() as *mut x264_union32_t))
                .i
                ^ (*(((*h).mb.cache.mv[0 as libc::c_int as usize]
                    [x264_scan8[4 as libc::c_int as usize] as usize])
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i) as libc::c_int;
            changed |= ((*(((*h).mb.cache.direct_mv[1 as libc::c_int as usize]
                [1 as libc::c_int as usize])
                .as_mut_ptr() as *mut x264_union32_t))
                .i
                ^ (*(((*h).mb.cache.mv[1 as libc::c_int as usize]
                    [x264_scan8[4 as libc::c_int as usize] as usize])
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i) as libc::c_int;
            changed |= ((*(((*h).mb.cache.direct_mv[0 as libc::c_int as usize]
                [2 as libc::c_int as usize])
                .as_mut_ptr() as *mut x264_union32_t))
                .i
                ^ (*(((*h).mb.cache.mv[0 as libc::c_int as usize]
                    [x264_scan8[8 as libc::c_int as usize] as usize])
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i) as libc::c_int;
            changed |= ((*(((*h).mb.cache.direct_mv[1 as libc::c_int as usize]
                [2 as libc::c_int as usize])
                .as_mut_ptr() as *mut x264_union32_t))
                .i
                ^ (*(((*h).mb.cache.mv[1 as libc::c_int as usize]
                    [x264_scan8[8 as libc::c_int as usize] as usize])
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i) as libc::c_int;
            changed |= (*h).mb.cache.direct_ref[0 as libc::c_int as usize]
                [1 as libc::c_int as usize] as libc::c_int
                ^ (*h).mb.cache.ref_0[0 as libc::c_int as usize]
                    [x264_scan8[4 as libc::c_int as usize] as usize]
                    as libc::c_int;
            changed |= (*h).mb.cache.direct_ref[1 as libc::c_int as usize]
                [1 as libc::c_int as usize] as libc::c_int
                ^ (*h).mb.cache.ref_0[1 as libc::c_int as usize]
                    [x264_scan8[4 as libc::c_int as usize] as usize]
                    as libc::c_int;
            changed |= (*h).mb.cache.direct_ref[0 as libc::c_int as usize]
                [2 as libc::c_int as usize] as libc::c_int
                ^ (*h).mb.cache.ref_0[0 as libc::c_int as usize]
                    [x264_scan8[8 as libc::c_int as usize] as usize]
                    as libc::c_int;
            changed |= (*h).mb.cache.direct_ref[1 as libc::c_int as usize]
                [2 as libc::c_int as usize] as libc::c_int
                ^ (*h).mb.cache.ref_0[1 as libc::c_int as usize]
                    [x264_scan8[8 as libc::c_int as usize] as usize]
                    as libc::c_int;
        }
        *b_changed = changed;
        if changed == 0 {
            return b_available;
        }
    }
    if b_available != 0 {
        let mut l: libc::c_int = 0 as libc::c_int;
        while l < 2 as libc::c_int {
            (*(((*h).mb.cache.direct_mv[l as usize][0 as libc::c_int as usize]).as_mut_ptr()
                as *mut x264_union32_t))
                .i = (*(((*h).mb.cache.mv[l as usize]
                [x264_scan8[0 as libc::c_int as usize] as usize])
                .as_mut_ptr() as *mut x264_union32_t))
                .i;
            (*(((*h).mb.cache.direct_mv[l as usize][1 as libc::c_int as usize]).as_mut_ptr()
                as *mut x264_union32_t))
                .i = (*(((*h).mb.cache.mv[l as usize]
                [x264_scan8[4 as libc::c_int as usize] as usize])
                .as_mut_ptr() as *mut x264_union32_t))
                .i;
            (*(((*h).mb.cache.direct_mv[l as usize][2 as libc::c_int as usize]).as_mut_ptr()
                as *mut x264_union32_t))
                .i = (*(((*h).mb.cache.mv[l as usize]
                [x264_scan8[8 as libc::c_int as usize] as usize])
                .as_mut_ptr() as *mut x264_union32_t))
                .i;
            (*(((*h).mb.cache.direct_mv[l as usize][3 as libc::c_int as usize]).as_mut_ptr()
                as *mut x264_union32_t))
                .i = (*(((*h).mb.cache.mv[l as usize]
                [x264_scan8[12 as libc::c_int as usize] as usize])
                .as_mut_ptr() as *mut x264_union32_t))
                .i;
            (*h).mb.cache.direct_ref[l as usize][0 as libc::c_int as usize] =
                (*h).mb.cache.ref_0[l as usize][x264_scan8[0 as libc::c_int as usize] as usize];
            (*h).mb.cache.direct_ref[l as usize][1 as libc::c_int as usize] =
                (*h).mb.cache.ref_0[l as usize][x264_scan8[4 as libc::c_int as usize] as usize];
            (*h).mb.cache.direct_ref[l as usize][2 as libc::c_int as usize] =
                (*h).mb.cache.ref_0[l as usize][x264_scan8[8 as libc::c_int as usize] as usize];
            (*h).mb.cache.direct_ref[l as usize][3 as libc::c_int as usize] =
                (*h).mb.cache.ref_0[l as usize][x264_scan8[12 as libc::c_int as usize] as usize];
            (*h).mb.cache.direct_partition = (*h).mb.i_partition;
            l += 1;
            l;
        }
    }
    b_available
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_mb_predict_mv_ref16x16(
    mut h: *mut x264_t,
    mut i_list: libc::c_int,
    mut i_ref: libc::c_int,
    mut mvc: *mut [int16_t; 2],
    mut i_mvc: *mut libc::c_int,
) {
    let mut mvr: *mut [int16_t; 2] = (*h).mb.mvr[i_list as usize][i_ref as usize];
    let mut i: libc::c_int = 0 as libc::c_int;
    if (*h).sh.i_type == SLICE_TYPE_B as libc::c_int
        && (*h).mb.cache.ref_0[i_list as usize][x264_scan8[12 as libc::c_int as usize] as usize]
            as libc::c_int
            == i_ref
    {
        (*((*mvc.offset(i as isize)).as_mut_ptr() as *mut x264_union32_t)).i =
            (*(((*h).mb.cache.mv[i_list as usize][x264_scan8[12 as libc::c_int as usize] as usize])
                .as_mut_ptr() as *mut x264_union32_t))
                .i;
        i += 1;
        i;
    }
    if i_ref == 0 as libc::c_int && (*h).frames.b_have_lowres != 0 {
        let mut idx: libc::c_int = if i_list != 0 {
            (*(*h).fref[1 as libc::c_int as usize][0 as libc::c_int as usize]).i_frame
                - (*(*h).fenc).i_frame
                - 1 as libc::c_int
        } else {
            (*(*h).fenc).i_frame
                - (*(*h).fref[0 as libc::c_int as usize][0 as libc::c_int as usize]).i_frame
                - 1 as libc::c_int
        };
        if idx <= (*h).param.i_bframe {
            let mut lowres_mv: *mut [int16_t; 2] =
                (*(*h).fenc).lowres_mvs[i_list as usize][idx as usize];
            if (*lowres_mv.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
                as libc::c_int
                != 0x7fff as libc::c_int
            {
                (*((*mvc.offset(i as isize)).as_mut_ptr() as *mut x264_union32_t)).i =
                    ((*((*lowres_mv.offset((*h).mb.i_mb_xy as isize)).as_mut_ptr()
                        as *mut x264_union32_t))
                        .i
                        * 2 as libc::c_int as uint32_t)
                        & 0xfffeffff as libc::c_uint;
                i += 1;
                i;
            }
        }
    }
    if (*h).sh.b_mbaff != 0 {
        if (*h).mb.i_mb_left_xy[0 as libc::c_int as usize] >= 0 as libc::c_int {
            let mut shift: libc::c_int = 1 as libc::c_int + (*h).mb.b_interlaced
                - *((*h).mb.field).offset((*h).mb.i_mb_left_xy[0 as libc::c_int as usize] as isize)
                    as libc::c_int;
            let mut mvp: *mut int16_t = (*((*h).mb.mvr[i_list as usize]
                [(i_ref << 1 as libc::c_int >> shift) as usize])
                .offset((*h).mb.i_mb_left_xy[0 as libc::c_int as usize] as isize))
            .as_mut_ptr();
            (*mvc.offset(i as isize))[0 as libc::c_int as usize] =
                *mvp.offset(0 as libc::c_int as isize);
            (*mvc.offset(i as isize))[1 as libc::c_int as usize] =
                ((*mvp.offset(1 as libc::c_int as isize) as libc::c_int * 2 as libc::c_int)
                    >> shift) as int16_t;
            i += 1;
            i;
        }
        if (*h).mb.i_mb_top_xy >= 0 as libc::c_int {
            let mut shift_0: libc::c_int = 1 as libc::c_int + (*h).mb.b_interlaced
                - *((*h).mb.field).offset((*h).mb.i_mb_top_xy as isize) as libc::c_int;
            let mut mvp_0: *mut int16_t = (*((*h).mb.mvr[i_list as usize]
                [(i_ref << 1 as libc::c_int >> shift_0) as usize])
                .offset((*h).mb.i_mb_top_xy as isize))
            .as_mut_ptr();
            (*mvc.offset(i as isize))[0 as libc::c_int as usize] =
                *mvp_0.offset(0 as libc::c_int as isize);
            (*mvc.offset(i as isize))[1 as libc::c_int as usize] =
                ((*mvp_0.offset(1 as libc::c_int as isize) as libc::c_int * 2 as libc::c_int)
                    >> shift_0) as int16_t;
            i += 1;
            i;
        }
        if (*h).mb.i_mb_topleft_xy >= 0 as libc::c_int {
            let mut shift_1: libc::c_int = 1 as libc::c_int + (*h).mb.b_interlaced
                - *((*h).mb.field).offset((*h).mb.i_mb_topleft_xy as isize) as libc::c_int;
            let mut mvp_1: *mut int16_t = (*((*h).mb.mvr[i_list as usize]
                [(i_ref << 1 as libc::c_int >> shift_1) as usize])
                .offset((*h).mb.i_mb_topleft_xy as isize))
            .as_mut_ptr();
            (*mvc.offset(i as isize))[0 as libc::c_int as usize] =
                *mvp_1.offset(0 as libc::c_int as isize);
            (*mvc.offset(i as isize))[1 as libc::c_int as usize] =
                ((*mvp_1.offset(1 as libc::c_int as isize) as libc::c_int * 2 as libc::c_int)
                    >> shift_1) as int16_t;
            i += 1;
            i;
        }
        if (*h).mb.i_mb_topright_xy >= 0 as libc::c_int {
            let mut shift_2: libc::c_int = 1 as libc::c_int + (*h).mb.b_interlaced
                - *((*h).mb.field).offset((*h).mb.i_mb_topright_xy as isize) as libc::c_int;
            let mut mvp_2: *mut int16_t = (*((*h).mb.mvr[i_list as usize]
                [(i_ref << 1 as libc::c_int >> shift_2) as usize])
                .offset((*h).mb.i_mb_topright_xy as isize))
            .as_mut_ptr();
            (*mvc.offset(i as isize))[0 as libc::c_int as usize] =
                *mvp_2.offset(0 as libc::c_int as isize);
            (*mvc.offset(i as isize))[1 as libc::c_int as usize] =
                ((*mvp_2.offset(1 as libc::c_int as isize) as libc::c_int * 2 as libc::c_int)
                    >> shift_2) as int16_t;
            i += 1;
            i;
        }
    } else {
        (*((*mvc.offset(i as isize)).as_mut_ptr() as *mut x264_union32_t)).i =
            (*((*mvr.offset((*h).mb.i_mb_left_xy[0 as libc::c_int as usize] as isize)).as_mut_ptr()
                as *mut x264_union32_t))
                .i;
        i += 1;
        i;
        (*((*mvc.offset(i as isize)).as_mut_ptr() as *mut x264_union32_t)).i =
            (*((*mvr.offset((*h).mb.i_mb_top_xy as isize)).as_mut_ptr() as *mut x264_union32_t)).i;
        i += 1;
        i;
        (*((*mvc.offset(i as isize)).as_mut_ptr() as *mut x264_union32_t)).i =
            (*((*mvr.offset((*h).mb.i_mb_topleft_xy as isize)).as_mut_ptr()
                as *mut x264_union32_t))
                .i;
        i += 1;
        i;
        (*((*mvc.offset(i as isize)).as_mut_ptr() as *mut x264_union32_t)).i =
            (*((*mvr.offset((*h).mb.i_mb_topright_xy as isize)).as_mut_ptr()
                as *mut x264_union32_t))
                .i;
        i += 1;
        i;
    }
    if (*(*h).fref[0 as libc::c_int as usize][0 as libc::c_int as usize]).i_ref
        [0 as libc::c_int as usize]
        > 0 as libc::c_int
    {
        let mut l0: *mut x264_frame_t =
            (*h).fref[0 as libc::c_int as usize][0 as libc::c_int as usize];
        let mut field: libc::c_int = (*h).mb.i_mb_y & 1 as libc::c_int;
        let mut curpoc: libc::c_int = (*(*h).fdec).i_poc + (*(*h).fdec).i_delta_poc[field as usize];
        let mut refpoc: libc::c_int =
            (*(*h).fref[i_list as usize][(i_ref >> (*h).sh.b_mbaff) as usize]).i_poc;
        refpoc += (*l0).i_delta_poc[(field ^ i_ref & 1 as libc::c_int) as usize];
        let mut mb_index: libc::c_int =
            (*h).mb.i_mb_xy + 0 as libc::c_int + 0 as libc::c_int * (*h).mb.i_mb_stride;
        let mut scale: libc::c_int = (curpoc - refpoc)
            * (*l0).inv_ref_poc[((*h).mb.b_interlaced & field) as usize] as libc::c_int;
        (*mvc.offset(i as isize))[0 as libc::c_int as usize] = x264_clip3(
            ((*((*l0).mv16x16).offset(mb_index as isize))[0 as libc::c_int as usize]
                as libc::c_int
                * scale
                + 128 as libc::c_int)
                >> 8 as libc::c_int,
            -(32767 as libc::c_int) - 1 as libc::c_int,
            32767 as libc::c_int,
        ) as int16_t;
        (*mvc.offset(i as isize))[1 as libc::c_int as usize] = x264_clip3(
            ((*((*l0).mv16x16).offset(mb_index as isize))[1 as libc::c_int as usize]
                as libc::c_int
                * scale
                + 128 as libc::c_int)
                >> 8 as libc::c_int,
            -(32767 as libc::c_int) - 1 as libc::c_int,
            32767 as libc::c_int,
        ) as int16_t;
        i += 1;
        i;
        if (*h).mb.i_mb_x < (*h).mb.i_mb_width - 1 as libc::c_int {
            let mut mb_index_0: libc::c_int =
                (*h).mb.i_mb_xy + 1 as libc::c_int + 0 as libc::c_int * (*h).mb.i_mb_stride;
            let mut scale_0: libc::c_int = (curpoc - refpoc)
                * (*l0).inv_ref_poc[((*h).mb.b_interlaced & field) as usize] as libc::c_int;
            (*mvc.offset(i as isize))[0 as libc::c_int as usize] = x264_clip3(
                ((*((*l0).mv16x16).offset(mb_index_0 as isize))[0 as libc::c_int as usize]
                    as libc::c_int
                    * scale_0
                    + 128 as libc::c_int)
                    >> 8 as libc::c_int,
                -(32767 as libc::c_int) - 1 as libc::c_int,
                32767 as libc::c_int,
            ) as int16_t;
            (*mvc.offset(i as isize))[1 as libc::c_int as usize] = x264_clip3(
                ((*((*l0).mv16x16).offset(mb_index_0 as isize))[1 as libc::c_int as usize]
                    as libc::c_int
                    * scale_0
                    + 128 as libc::c_int)
                    >> 8 as libc::c_int,
                -(32767 as libc::c_int) - 1 as libc::c_int,
                32767 as libc::c_int,
            ) as int16_t;
            i += 1;
            i;
        }
        if (*h).mb.i_mb_y < (*h).mb.i_mb_height - 1 as libc::c_int {
            let mut mb_index_1: libc::c_int =
                (*h).mb.i_mb_xy + 0 as libc::c_int + 1 as libc::c_int * (*h).mb.i_mb_stride;
            let mut scale_1: libc::c_int = (curpoc - refpoc)
                * (*l0).inv_ref_poc[((*h).mb.b_interlaced & field) as usize] as libc::c_int;
            (*mvc.offset(i as isize))[0 as libc::c_int as usize] = x264_clip3(
                ((*((*l0).mv16x16).offset(mb_index_1 as isize))[0 as libc::c_int as usize]
                    as libc::c_int
                    * scale_1
                    + 128 as libc::c_int)
                    >> 8 as libc::c_int,
                -(32767 as libc::c_int) - 1 as libc::c_int,
                32767 as libc::c_int,
            ) as int16_t;
            (*mvc.offset(i as isize))[1 as libc::c_int as usize] = x264_clip3(
                ((*((*l0).mv16x16).offset(mb_index_1 as isize))[1 as libc::c_int as usize]
                    as libc::c_int
                    * scale_1
                    + 128 as libc::c_int)
                    >> 8 as libc::c_int,
                -(32767 as libc::c_int) - 1 as libc::c_int,
                32767 as libc::c_int,
            ) as int16_t;
            i += 1;
            i;
        }
    }
    *i_mvc = i;
}
