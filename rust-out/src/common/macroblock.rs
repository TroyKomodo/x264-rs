use crate::types::*;
#[cfg(target_arch = "x86")]
pub use core::arch::x86::{__m128, _mm_setr_ps};
#[cfg(target_arch = "x86_64")]
pub use core::arch::x86_64::_mm_setr_ps;
extern "C" {
    fn abs(_: libc::c_int) -> libc::c_int;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn x264_malloc(_: int64_t) -> *mut libc::c_void;
    fn x264_free(_: *mut libc::c_void);
    fn x264_8_mb_predict_mv_pskip(h: *mut x264_t, mv: *mut int16_t);
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
unsafe extern "C" fn x264_macroblock_cache_skip(
    mut h: *mut x264_t,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut b_skip: libc::c_int,
) {
    x264_macroblock_cache_rect(
        &mut *((*h).mb.cache.skip).as_mut_ptr().offset(
            (4 as libc::c_int + 1 as libc::c_int * 8 as libc::c_int + x + 8 as libc::c_int * y)
                as isize,
        ) as *mut int8_t as *mut libc::c_void,
        width,
        height,
        1 as libc::c_int,
        b_skip as uint32_t,
    );
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
        'c_30189: {
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
        'c_29956: {
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
unsafe extern "C" fn pack16to32(mut a: uint32_t, mut b: uint32_t) -> uint32_t {
    a.wrapping_add(b << 16 as libc::c_int)
}
#[inline(always)]
unsafe extern "C" fn pack8to32(
    mut a: uint32_t,
    mut b: uint32_t,
    mut c: uint32_t,
    mut d: uint32_t,
) -> uint32_t {
    a.wrapping_add(b << 8 as libc::c_int)
        .wrapping_add(c << 16 as libc::c_int)
        .wrapping_add(d << 24 as libc::c_int)
}
#[inline(never)]
unsafe extern "C" fn mb_mc_0xywh(
    mut h: *mut x264_t,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
) {
    let mut i8: libc::c_int =
        x264_scan8[0 as libc::c_int as usize] as libc::c_int + x + 8 as libc::c_int * y;
    let mut i_ref: libc::c_int =
        (*h).mb.cache.ref_0[0 as libc::c_int as usize][i8 as usize] as libc::c_int;
    let mut mvx: libc::c_int = x264_clip3(
        (*h).mb.cache.mv[0 as libc::c_int as usize][i8 as usize][0 as libc::c_int as usize]
            as libc::c_int,
        (*h).mb.mv_min[0 as libc::c_int as usize],
        (*h).mb.mv_max[0 as libc::c_int as usize],
    ) + 4 as libc::c_int * 4 as libc::c_int * x;
    let mut mvy: libc::c_int = x264_clip3(
        (*h).mb.cache.mv[0 as libc::c_int as usize][i8 as usize][1 as libc::c_int as usize]
            as libc::c_int,
        (*h).mb.mv_min[1 as libc::c_int as usize],
        (*h).mb.mv_max[1 as libc::c_int as usize],
    ) + 4 as libc::c_int * 4 as libc::c_int * y;
    ((*h).mc.mc_luma).expect("non-null function pointer")(
        &mut *(*((*h).mb.pic.p_fdec)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize))
        .offset((4 as libc::c_int * y * 32 as libc::c_int + 4 as libc::c_int * x) as isize),
        32 as libc::c_int as intptr_t,
        &mut *(*(*((*h).mb.pic.p_fref)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize))
        .as_mut_ptr()
        .offset(i_ref as isize))
        .as_mut_ptr()
        .offset((0 as libc::c_int * 4 as libc::c_int) as isize),
        (*h).mb.pic.i_stride[0 as libc::c_int as usize] as intptr_t,
        mvx,
        mvy,
        4 as libc::c_int * width,
        4 as libc::c_int * height,
        if 0 as libc::c_int != 0 {
            x264_zero.as_mut_ptr() as *const x264_weight_t
        } else {
            &mut *(*((*h).sh.weight).as_mut_ptr().offset(i_ref as isize))
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as *mut x264_weight_t
                as *const x264_weight_t
        },
    );
    if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as libc::c_int {
        ((*h).mc.mc_luma).expect("non-null function pointer")(
            &mut *(*((*h).mb.pic.p_fdec)
                .as_mut_ptr()
                .offset(1 as libc::c_int as isize))
            .offset((4 as libc::c_int * y * 32 as libc::c_int + 4 as libc::c_int * x) as isize),
            32 as libc::c_int as intptr_t,
            &mut *(*(*((*h).mb.pic.p_fref)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize))
            .as_mut_ptr()
            .offset(i_ref as isize))
            .as_mut_ptr()
            .offset((1 as libc::c_int * 4 as libc::c_int) as isize),
            (*h).mb.pic.i_stride[1 as libc::c_int as usize] as intptr_t,
            mvx,
            mvy,
            4 as libc::c_int * width,
            4 as libc::c_int * height,
            if 0 as libc::c_int != 0 {
                x264_zero.as_mut_ptr() as *const x264_weight_t
            } else {
                &mut *(*((*h).sh.weight).as_mut_ptr().offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize) as *mut x264_weight_t
                    as *const x264_weight_t
            },
        );
        ((*h).mc.mc_luma).expect("non-null function pointer")(
            &mut *(*((*h).mb.pic.p_fdec)
                .as_mut_ptr()
                .offset(2 as libc::c_int as isize))
            .offset((4 as libc::c_int * y * 32 as libc::c_int + 4 as libc::c_int * x) as isize),
            32 as libc::c_int as intptr_t,
            &mut *(*(*((*h).mb.pic.p_fref)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize))
            .as_mut_ptr()
            .offset(i_ref as isize))
            .as_mut_ptr()
            .offset((2 as libc::c_int * 4 as libc::c_int) as isize),
            (*h).mb.pic.i_stride[2 as libc::c_int as usize] as intptr_t,
            mvx,
            mvy,
            4 as libc::c_int * width,
            4 as libc::c_int * height,
            if 0 as libc::c_int != 0 {
                x264_zero.as_mut_ptr() as *const x264_weight_t
            } else {
                &mut *(*((*h).sh.weight).as_mut_ptr().offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as isize) as *mut x264_weight_t
                    as *const x264_weight_t
            },
        );
    } else if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc != 0 {
        let mut v_shift: libc::c_int = (*h).mb.chroma_v_shift;
        if v_shift & (*h).mb.b_interlaced & i_ref != 0 {
            mvy += ((*h).mb.i_mb_y & 1 as libc::c_int) * 4 as libc::c_int - 2 as libc::c_int;
        }
        let mut offset: libc::c_int =
            ((4 as libc::c_int * 32 as libc::c_int) >> v_shift) * y + 2 as libc::c_int * x;
        height = (4 as libc::c_int * height) >> v_shift;
        ((*h).mc.mc_chroma).expect("non-null function pointer")(
            &mut *(*((*h).mb.pic.p_fdec)
                .as_mut_ptr()
                .offset(1 as libc::c_int as isize))
            .offset(offset as isize),
            &mut *(*((*h).mb.pic.p_fdec)
                .as_mut_ptr()
                .offset(2 as libc::c_int as isize))
            .offset(offset as isize),
            32 as libc::c_int as intptr_t,
            (*h).mb.pic.p_fref[0 as libc::c_int as usize][i_ref as usize]
                [4 as libc::c_int as usize],
            (*h).mb.pic.i_stride[1 as libc::c_int as usize] as intptr_t,
            mvx,
            (2 as libc::c_int * mvy) >> v_shift,
            2 as libc::c_int * width,
            height,
        );
        if !((*h).sh.weight[i_ref as usize][1 as libc::c_int as usize].weightfn).is_null() {
            (*((*h).sh.weight[i_ref as usize][1 as libc::c_int as usize].weightfn)
                .offset((width >> 1 as libc::c_int) as isize))
            .expect("non-null function pointer")(
                &mut *(*((*h).mb.pic.p_fdec)
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize))
                .offset(offset as isize),
                32 as libc::c_int as intptr_t,
                &mut *(*((*h).mb.pic.p_fdec)
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize))
                .offset(offset as isize),
                32 as libc::c_int as intptr_t,
                &mut *(*((*h).sh.weight).as_mut_ptr().offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize),
                height,
            );
        }
        if !((*h).sh.weight[i_ref as usize][2 as libc::c_int as usize].weightfn).is_null() {
            (*((*h).sh.weight[i_ref as usize][2 as libc::c_int as usize].weightfn)
                .offset((width >> 1 as libc::c_int) as isize))
            .expect("non-null function pointer")(
                &mut *(*((*h).mb.pic.p_fdec)
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as isize))
                .offset(offset as isize),
                32 as libc::c_int as intptr_t,
                &mut *(*((*h).mb.pic.p_fdec)
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as isize))
                .offset(offset as isize),
                32 as libc::c_int as intptr_t,
                &mut *(*((*h).sh.weight).as_mut_ptr().offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as isize),
                height,
            );
        }
    }
}
#[inline(never)]
unsafe extern "C" fn mb_mc_1xywh(
    mut h: *mut x264_t,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
) {
    let mut i8: libc::c_int =
        x264_scan8[0 as libc::c_int as usize] as libc::c_int + x + 8 as libc::c_int * y;
    let mut i_ref: libc::c_int =
        (*h).mb.cache.ref_0[1 as libc::c_int as usize][i8 as usize] as libc::c_int;
    let mut mvx: libc::c_int = x264_clip3(
        (*h).mb.cache.mv[1 as libc::c_int as usize][i8 as usize][0 as libc::c_int as usize]
            as libc::c_int,
        (*h).mb.mv_min[0 as libc::c_int as usize],
        (*h).mb.mv_max[0 as libc::c_int as usize],
    ) + 4 as libc::c_int * 4 as libc::c_int * x;
    let mut mvy: libc::c_int = x264_clip3(
        (*h).mb.cache.mv[1 as libc::c_int as usize][i8 as usize][1 as libc::c_int as usize]
            as libc::c_int,
        (*h).mb.mv_min[1 as libc::c_int as usize],
        (*h).mb.mv_max[1 as libc::c_int as usize],
    ) + 4 as libc::c_int * 4 as libc::c_int * y;
    ((*h).mc.mc_luma).expect("non-null function pointer")(
        &mut *(*((*h).mb.pic.p_fdec)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize))
        .offset((4 as libc::c_int * y * 32 as libc::c_int + 4 as libc::c_int * x) as isize),
        32 as libc::c_int as intptr_t,
        &mut *(*(*((*h).mb.pic.p_fref)
            .as_mut_ptr()
            .offset(1 as libc::c_int as isize))
        .as_mut_ptr()
        .offset(i_ref as isize))
        .as_mut_ptr()
        .offset((0 as libc::c_int * 4 as libc::c_int) as isize),
        (*h).mb.pic.i_stride[0 as libc::c_int as usize] as intptr_t,
        mvx,
        mvy,
        4 as libc::c_int * width,
        4 as libc::c_int * height,
        if 1 as libc::c_int != 0 {
            x264_zero.as_mut_ptr() as *const x264_weight_t
        } else {
            &mut *(*((*h).sh.weight).as_mut_ptr().offset(i_ref as isize))
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as *mut x264_weight_t
                as *const x264_weight_t
        },
    );
    if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as libc::c_int {
        ((*h).mc.mc_luma).expect("non-null function pointer")(
            &mut *(*((*h).mb.pic.p_fdec)
                .as_mut_ptr()
                .offset(1 as libc::c_int as isize))
            .offset((4 as libc::c_int * y * 32 as libc::c_int + 4 as libc::c_int * x) as isize),
            32 as libc::c_int as intptr_t,
            &mut *(*(*((*h).mb.pic.p_fref)
                .as_mut_ptr()
                .offset(1 as libc::c_int as isize))
            .as_mut_ptr()
            .offset(i_ref as isize))
            .as_mut_ptr()
            .offset((1 as libc::c_int * 4 as libc::c_int) as isize),
            (*h).mb.pic.i_stride[1 as libc::c_int as usize] as intptr_t,
            mvx,
            mvy,
            4 as libc::c_int * width,
            4 as libc::c_int * height,
            if 1 as libc::c_int != 0 {
                x264_zero.as_mut_ptr() as *const x264_weight_t
            } else {
                &mut *(*((*h).sh.weight).as_mut_ptr().offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize) as *mut x264_weight_t
                    as *const x264_weight_t
            },
        );
        ((*h).mc.mc_luma).expect("non-null function pointer")(
            &mut *(*((*h).mb.pic.p_fdec)
                .as_mut_ptr()
                .offset(2 as libc::c_int as isize))
            .offset((4 as libc::c_int * y * 32 as libc::c_int + 4 as libc::c_int * x) as isize),
            32 as libc::c_int as intptr_t,
            &mut *(*(*((*h).mb.pic.p_fref)
                .as_mut_ptr()
                .offset(1 as libc::c_int as isize))
            .as_mut_ptr()
            .offset(i_ref as isize))
            .as_mut_ptr()
            .offset((2 as libc::c_int * 4 as libc::c_int) as isize),
            (*h).mb.pic.i_stride[2 as libc::c_int as usize] as intptr_t,
            mvx,
            mvy,
            4 as libc::c_int * width,
            4 as libc::c_int * height,
            if 1 as libc::c_int != 0 {
                x264_zero.as_mut_ptr() as *const x264_weight_t
            } else {
                &mut *(*((*h).sh.weight).as_mut_ptr().offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as isize) as *mut x264_weight_t
                    as *const x264_weight_t
            },
        );
    } else if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc != 0 {
        let mut v_shift: libc::c_int = (*h).mb.chroma_v_shift;
        if v_shift & (*h).mb.b_interlaced & i_ref != 0 {
            mvy += ((*h).mb.i_mb_y & 1 as libc::c_int) * 4 as libc::c_int - 2 as libc::c_int;
        }
        let mut offset: libc::c_int =
            ((4 as libc::c_int * 32 as libc::c_int) >> v_shift) * y + 2 as libc::c_int * x;
        ((*h).mc.mc_chroma).expect("non-null function pointer")(
            &mut *(*((*h).mb.pic.p_fdec)
                .as_mut_ptr()
                .offset(1 as libc::c_int as isize))
            .offset(offset as isize),
            &mut *(*((*h).mb.pic.p_fdec)
                .as_mut_ptr()
                .offset(2 as libc::c_int as isize))
            .offset(offset as isize),
            32 as libc::c_int as intptr_t,
            (*h).mb.pic.p_fref[1 as libc::c_int as usize][i_ref as usize]
                [4 as libc::c_int as usize],
            (*h).mb.pic.i_stride[1 as libc::c_int as usize] as intptr_t,
            mvx,
            (2 as libc::c_int * mvy) >> v_shift,
            2 as libc::c_int * width,
            (4 as libc::c_int * height) >> v_shift,
        );
    }
}
#[inline(never)]
unsafe extern "C" fn mb_mc_01xywh(
    mut h: *mut x264_t,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
) {
    let mut i8: libc::c_int =
        x264_scan8[0 as libc::c_int as usize] as libc::c_int + x + 8 as libc::c_int * y;
    let mut i_ref0: libc::c_int =
        (*h).mb.cache.ref_0[0 as libc::c_int as usize][i8 as usize] as libc::c_int;
    let mut i_ref1: libc::c_int =
        (*h).mb.cache.ref_0[1 as libc::c_int as usize][i8 as usize] as libc::c_int;
    let mut weight: libc::c_int =
        (*((*h).mb.bipred_weight).offset(i_ref0 as isize))[i_ref1 as usize] as libc::c_int;
    let mut mvx0: libc::c_int = x264_clip3(
        (*h).mb.cache.mv[0 as libc::c_int as usize][i8 as usize][0 as libc::c_int as usize]
            as libc::c_int,
        (*h).mb.mv_min[0 as libc::c_int as usize],
        (*h).mb.mv_max[0 as libc::c_int as usize],
    ) + 4 as libc::c_int * 4 as libc::c_int * x;
    let mut mvx1: libc::c_int = x264_clip3(
        (*h).mb.cache.mv[1 as libc::c_int as usize][i8 as usize][0 as libc::c_int as usize]
            as libc::c_int,
        (*h).mb.mv_min[0 as libc::c_int as usize],
        (*h).mb.mv_max[0 as libc::c_int as usize],
    ) + 4 as libc::c_int * 4 as libc::c_int * x;
    let mut mvy0: libc::c_int = x264_clip3(
        (*h).mb.cache.mv[0 as libc::c_int as usize][i8 as usize][1 as libc::c_int as usize]
            as libc::c_int,
        (*h).mb.mv_min[1 as libc::c_int as usize],
        (*h).mb.mv_max[1 as libc::c_int as usize],
    ) + 4 as libc::c_int * 4 as libc::c_int * y;
    let mut mvy1: libc::c_int = x264_clip3(
        (*h).mb.cache.mv[1 as libc::c_int as usize][i8 as usize][1 as libc::c_int as usize]
            as libc::c_int,
        (*h).mb.mv_min[1 as libc::c_int as usize],
        (*h).mb.mv_max[1 as libc::c_int as usize],
    ) + 4 as libc::c_int * 4 as libc::c_int * y;
    let mut i_mode: libc::c_int = x264_size2pixel[height as usize][width as usize] as libc::c_int;
    let mut i_stride0: intptr_t = 16 as libc::c_int as intptr_t;
    let mut i_stride1: intptr_t = 16 as libc::c_int as intptr_t;
    let mut tmp0: [pixel; 256] = [0; 256];
    let mut tmp1: [pixel; 256] = [0; 256];
    let mut src0: *mut pixel = std::ptr::null_mut::<pixel>();
    let mut src1: *mut pixel = std::ptr::null_mut::<pixel>();
    src0 = ((*h).mc.get_ref).expect("non-null function pointer")(
        tmp0.as_mut_ptr(),
        &mut i_stride0,
        &mut *(*(*((*h).mb.pic.p_fref)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize))
        .as_mut_ptr()
        .offset(i_ref0 as isize))
        .as_mut_ptr()
        .offset((0 as libc::c_int * 4 as libc::c_int) as isize),
        (*h).mb.pic.i_stride[0 as libc::c_int as usize] as intptr_t,
        mvx0,
        mvy0,
        4 as libc::c_int * width,
        4 as libc::c_int * height,
        x264_zero.as_mut_ptr() as *const x264_weight_t,
    );
    src1 = ((*h).mc.get_ref).expect("non-null function pointer")(
        tmp1.as_mut_ptr(),
        &mut i_stride1,
        &mut *(*(*((*h).mb.pic.p_fref)
            .as_mut_ptr()
            .offset(1 as libc::c_int as isize))
        .as_mut_ptr()
        .offset(i_ref1 as isize))
        .as_mut_ptr()
        .offset((0 as libc::c_int * 4 as libc::c_int) as isize),
        (*h).mb.pic.i_stride[0 as libc::c_int as usize] as intptr_t,
        mvx1,
        mvy1,
        4 as libc::c_int * width,
        4 as libc::c_int * height,
        x264_zero.as_mut_ptr() as *const x264_weight_t,
    );
    ((*h).mc.avg[i_mode as usize]).expect("non-null function pointer")(
        &mut *(*((*h).mb.pic.p_fdec)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize))
        .offset((4 as libc::c_int * y * 32 as libc::c_int + 4 as libc::c_int * x) as isize),
        32 as libc::c_int as intptr_t,
        src0,
        i_stride0,
        src1,
        i_stride1,
        weight,
    );
    if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as libc::c_int {
        src0 = ((*h).mc.get_ref).expect("non-null function pointer")(
            tmp0.as_mut_ptr(),
            &mut i_stride0,
            &mut *(*(*((*h).mb.pic.p_fref)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize))
            .as_mut_ptr()
            .offset(i_ref0 as isize))
            .as_mut_ptr()
            .offset((1 as libc::c_int * 4 as libc::c_int) as isize),
            (*h).mb.pic.i_stride[1 as libc::c_int as usize] as intptr_t,
            mvx0,
            mvy0,
            4 as libc::c_int * width,
            4 as libc::c_int * height,
            x264_zero.as_mut_ptr() as *const x264_weight_t,
        );
        src1 = ((*h).mc.get_ref).expect("non-null function pointer")(
            tmp1.as_mut_ptr(),
            &mut i_stride1,
            &mut *(*(*((*h).mb.pic.p_fref)
                .as_mut_ptr()
                .offset(1 as libc::c_int as isize))
            .as_mut_ptr()
            .offset(i_ref1 as isize))
            .as_mut_ptr()
            .offset((1 as libc::c_int * 4 as libc::c_int) as isize),
            (*h).mb.pic.i_stride[1 as libc::c_int as usize] as intptr_t,
            mvx1,
            mvy1,
            4 as libc::c_int * width,
            4 as libc::c_int * height,
            x264_zero.as_mut_ptr() as *const x264_weight_t,
        );
        ((*h).mc.avg[i_mode as usize]).expect("non-null function pointer")(
            &mut *(*((*h).mb.pic.p_fdec)
                .as_mut_ptr()
                .offset(1 as libc::c_int as isize))
            .offset((4 as libc::c_int * y * 32 as libc::c_int + 4 as libc::c_int * x) as isize),
            32 as libc::c_int as intptr_t,
            src0,
            i_stride0,
            src1,
            i_stride1,
            weight,
        );
        src0 = ((*h).mc.get_ref).expect("non-null function pointer")(
            tmp0.as_mut_ptr(),
            &mut i_stride0,
            &mut *(*(*((*h).mb.pic.p_fref)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize))
            .as_mut_ptr()
            .offset(i_ref0 as isize))
            .as_mut_ptr()
            .offset((2 as libc::c_int * 4 as libc::c_int) as isize),
            (*h).mb.pic.i_stride[2 as libc::c_int as usize] as intptr_t,
            mvx0,
            mvy0,
            4 as libc::c_int * width,
            4 as libc::c_int * height,
            x264_zero.as_mut_ptr() as *const x264_weight_t,
        );
        src1 = ((*h).mc.get_ref).expect("non-null function pointer")(
            tmp1.as_mut_ptr(),
            &mut i_stride1,
            &mut *(*(*((*h).mb.pic.p_fref)
                .as_mut_ptr()
                .offset(1 as libc::c_int as isize))
            .as_mut_ptr()
            .offset(i_ref1 as isize))
            .as_mut_ptr()
            .offset((2 as libc::c_int * 4 as libc::c_int) as isize),
            (*h).mb.pic.i_stride[2 as libc::c_int as usize] as intptr_t,
            mvx1,
            mvy1,
            4 as libc::c_int * width,
            4 as libc::c_int * height,
            x264_zero.as_mut_ptr() as *const x264_weight_t,
        );
        ((*h).mc.avg[i_mode as usize]).expect("non-null function pointer")(
            &mut *(*((*h).mb.pic.p_fdec)
                .as_mut_ptr()
                .offset(2 as libc::c_int as isize))
            .offset((4 as libc::c_int * y * 32 as libc::c_int + 4 as libc::c_int * x) as isize),
            32 as libc::c_int as intptr_t,
            src0,
            i_stride0,
            src1,
            i_stride1,
            weight,
        );
    } else if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc != 0 {
        let mut v_shift: libc::c_int = (*h).mb.chroma_v_shift;
        if v_shift & (*h).mb.b_interlaced & i_ref0 != 0 {
            mvy0 += ((*h).mb.i_mb_y & 1 as libc::c_int) * 4 as libc::c_int - 2 as libc::c_int;
        }
        if v_shift & (*h).mb.b_interlaced & i_ref1 != 0 {
            mvy1 += ((*h).mb.i_mb_y & 1 as libc::c_int) * 4 as libc::c_int - 2 as libc::c_int;
        }
        ((*h).mc.mc_chroma).expect("non-null function pointer")(
            tmp0.as_mut_ptr(),
            tmp0.as_mut_ptr().offset(8 as libc::c_int as isize),
            16 as libc::c_int as intptr_t,
            (*h).mb.pic.p_fref[0 as libc::c_int as usize][i_ref0 as usize]
                [4 as libc::c_int as usize],
            (*h).mb.pic.i_stride[1 as libc::c_int as usize] as intptr_t,
            mvx0,
            (2 as libc::c_int * mvy0) >> v_shift,
            2 as libc::c_int * width,
            (4 as libc::c_int * height) >> v_shift,
        );
        ((*h).mc.mc_chroma).expect("non-null function pointer")(
            tmp1.as_mut_ptr(),
            tmp1.as_mut_ptr().offset(8 as libc::c_int as isize),
            16 as libc::c_int as intptr_t,
            (*h).mb.pic.p_fref[1 as libc::c_int as usize][i_ref1 as usize]
                [4 as libc::c_int as usize],
            (*h).mb.pic.i_stride[1 as libc::c_int as usize] as intptr_t,
            mvx1,
            (2 as libc::c_int * mvy1) >> v_shift,
            2 as libc::c_int * width,
            (4 as libc::c_int * height) >> v_shift,
        );
        let mut chromapix: libc::c_int = (*h).luma2chroma_pixel[i_mode as usize] as libc::c_int;
        let mut offset: libc::c_int =
            ((4 as libc::c_int * 32 as libc::c_int) >> v_shift) * y + 2 as libc::c_int * x;
        ((*h).mc.avg[chromapix as usize]).expect("non-null function pointer")(
            &mut *(*((*h).mb.pic.p_fdec)
                .as_mut_ptr()
                .offset(1 as libc::c_int as isize))
            .offset(offset as isize),
            32 as libc::c_int as intptr_t,
            tmp0.as_mut_ptr(),
            16 as libc::c_int as intptr_t,
            tmp1.as_mut_ptr(),
            16 as libc::c_int as intptr_t,
            weight,
        );
        ((*h).mc.avg[chromapix as usize]).expect("non-null function pointer")(
            &mut *(*((*h).mb.pic.p_fdec)
                .as_mut_ptr()
                .offset(2 as libc::c_int as isize))
            .offset(offset as isize),
            32 as libc::c_int as intptr_t,
            tmp0.as_mut_ptr().offset(8 as libc::c_int as isize),
            16 as libc::c_int as intptr_t,
            tmp1.as_mut_ptr().offset(8 as libc::c_int as isize),
            16 as libc::c_int as intptr_t,
            weight,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_mb_mc_8x8(mut h: *mut x264_t, mut i8: libc::c_int) {
    let mut x: libc::c_int = 2 as libc::c_int * (i8 & 1 as libc::c_int);
    let mut y: libc::c_int = 2 as libc::c_int * (i8 >> 1 as libc::c_int);
    if (*h).sh.i_type == SLICE_TYPE_P as libc::c_int {
        match (*h).mb.i_sub_partition[i8 as usize] as libc::c_int {
            3 => {
                mb_mc_0xywh(h, x, y, 2 as libc::c_int, 2 as libc::c_int);
            }
            1 => {
                mb_mc_0xywh(
                    h,
                    x,
                    y + 0 as libc::c_int,
                    2 as libc::c_int,
                    1 as libc::c_int,
                );
                mb_mc_0xywh(
                    h,
                    x,
                    y + 1 as libc::c_int,
                    2 as libc::c_int,
                    1 as libc::c_int,
                );
            }
            2 => {
                mb_mc_0xywh(
                    h,
                    x + 0 as libc::c_int,
                    y,
                    1 as libc::c_int,
                    2 as libc::c_int,
                );
                mb_mc_0xywh(
                    h,
                    x + 1 as libc::c_int,
                    y,
                    1 as libc::c_int,
                    2 as libc::c_int,
                );
            }
            0 => {
                mb_mc_0xywh(
                    h,
                    x + 0 as libc::c_int,
                    y + 0 as libc::c_int,
                    1 as libc::c_int,
                    1 as libc::c_int,
                );
                mb_mc_0xywh(
                    h,
                    x + 1 as libc::c_int,
                    y + 0 as libc::c_int,
                    1 as libc::c_int,
                    1 as libc::c_int,
                );
                mb_mc_0xywh(
                    h,
                    x + 0 as libc::c_int,
                    y + 1 as libc::c_int,
                    1 as libc::c_int,
                    1 as libc::c_int,
                );
                mb_mc_0xywh(
                    h,
                    x + 1 as libc::c_int,
                    y + 1 as libc::c_int,
                    1 as libc::c_int,
                    1 as libc::c_int,
                );
            }
            _ => {}
        }
    } else {
        let mut scan8: libc::c_int =
            x264_scan8[0 as libc::c_int as usize] as libc::c_int + x + 8 as libc::c_int * y;
        if (*h).mb.cache.ref_0[0 as libc::c_int as usize][scan8 as usize] as libc::c_int
            >= 0 as libc::c_int
        {
            if (*h).mb.cache.ref_0[1 as libc::c_int as usize][scan8 as usize] as libc::c_int
                >= 0 as libc::c_int
            {
                mb_mc_01xywh(h, x, y, 2 as libc::c_int, 2 as libc::c_int);
            } else {
                mb_mc_0xywh(h, x, y, 2 as libc::c_int, 2 as libc::c_int);
            }
        } else {
            mb_mc_1xywh(h, x, y, 2 as libc::c_int, 2 as libc::c_int);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_mb_mc(mut h: *mut x264_t) {
    if (*h).mb.i_partition == D_8x8 as libc::c_int {
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            x264_8_mb_mc_8x8(h, i);
            i += 1;
            i;
        }
    } else {
        let mut ref0a: libc::c_int = (*h).mb.cache.ref_0[0 as libc::c_int as usize]
            [x264_scan8[0 as libc::c_int as usize] as usize]
            as libc::c_int;
        let mut ref0b: libc::c_int = (*h).mb.cache.ref_0[0 as libc::c_int as usize]
            [x264_scan8[12 as libc::c_int as usize] as usize]
            as libc::c_int;
        let mut ref1a: libc::c_int = (*h).mb.cache.ref_0[1 as libc::c_int as usize]
            [x264_scan8[0 as libc::c_int as usize] as usize]
            as libc::c_int;
        let mut ref1b: libc::c_int = (*h).mb.cache.ref_0[1 as libc::c_int as usize]
            [x264_scan8[12 as libc::c_int as usize] as usize]
            as libc::c_int;
        if (*h).mb.i_partition == D_16x16 as libc::c_int {
            if ref0a >= 0 as libc::c_int {
                if ref1a >= 0 as libc::c_int {
                    mb_mc_01xywh(
                        h,
                        0 as libc::c_int,
                        0 as libc::c_int,
                        4 as libc::c_int,
                        4 as libc::c_int,
                    );
                } else {
                    mb_mc_0xywh(
                        h,
                        0 as libc::c_int,
                        0 as libc::c_int,
                        4 as libc::c_int,
                        4 as libc::c_int,
                    );
                }
            } else {
                mb_mc_1xywh(
                    h,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    4 as libc::c_int,
                    4 as libc::c_int,
                );
            }
        } else if (*h).mb.i_partition == D_16x8 as libc::c_int {
            if ref0a >= 0 as libc::c_int {
                if ref1a >= 0 as libc::c_int {
                    mb_mc_01xywh(
                        h,
                        0 as libc::c_int,
                        0 as libc::c_int,
                        4 as libc::c_int,
                        2 as libc::c_int,
                    );
                } else {
                    mb_mc_0xywh(
                        h,
                        0 as libc::c_int,
                        0 as libc::c_int,
                        4 as libc::c_int,
                        2 as libc::c_int,
                    );
                }
            } else {
                mb_mc_1xywh(
                    h,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    4 as libc::c_int,
                    2 as libc::c_int,
                );
            }
            if ref0b >= 0 as libc::c_int {
                if ref1b >= 0 as libc::c_int {
                    mb_mc_01xywh(
                        h,
                        0 as libc::c_int,
                        2 as libc::c_int,
                        4 as libc::c_int,
                        2 as libc::c_int,
                    );
                } else {
                    mb_mc_0xywh(
                        h,
                        0 as libc::c_int,
                        2 as libc::c_int,
                        4 as libc::c_int,
                        2 as libc::c_int,
                    );
                }
            } else {
                mb_mc_1xywh(
                    h,
                    0 as libc::c_int,
                    2 as libc::c_int,
                    4 as libc::c_int,
                    2 as libc::c_int,
                );
            }
        } else if (*h).mb.i_partition == D_8x16 as libc::c_int {
            if ref0a >= 0 as libc::c_int {
                if ref1a >= 0 as libc::c_int {
                    mb_mc_01xywh(
                        h,
                        0 as libc::c_int,
                        0 as libc::c_int,
                        2 as libc::c_int,
                        4 as libc::c_int,
                    );
                } else {
                    mb_mc_0xywh(
                        h,
                        0 as libc::c_int,
                        0 as libc::c_int,
                        2 as libc::c_int,
                        4 as libc::c_int,
                    );
                }
            } else {
                mb_mc_1xywh(
                    h,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    2 as libc::c_int,
                    4 as libc::c_int,
                );
            }
            if ref0b >= 0 as libc::c_int {
                if ref1b >= 0 as libc::c_int {
                    mb_mc_01xywh(
                        h,
                        2 as libc::c_int,
                        0 as libc::c_int,
                        2 as libc::c_int,
                        4 as libc::c_int,
                    );
                } else {
                    mb_mc_0xywh(
                        h,
                        2 as libc::c_int,
                        0 as libc::c_int,
                        2 as libc::c_int,
                        4 as libc::c_int,
                    );
                }
            } else {
                mb_mc_1xywh(
                    h,
                    2 as libc::c_int,
                    0 as libc::c_int,
                    2 as libc::c_int,
                    4 as libc::c_int,
                );
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_macroblock_cache_allocate(mut h: *mut x264_t) -> libc::c_int {
    let mut i_mb_count: libc::c_int = (*h).mb.i_mb_count;
    (*h).mb.i_mb_stride = (*h).mb.i_mb_width;
    (*h).mb.i_b8_stride = (*h).mb.i_mb_width * 2 as libc::c_int;
    (*h).mb.i_b4_stride = (*h).mb.i_mb_width * 4 as libc::c_int;
    (*h).mb.b_interlaced = (*h).param.b_interlaced;
    let mut prealloc_idx: libc::c_int = 0 as libc::c_int;
    let mut prealloc_size: int64_t = 0 as libc::c_int as int64_t;
    let mut preallocs: [*mut *mut uint8_t; 1024] = [std::ptr::null_mut::<*mut uint8_t>(); 1024];
    (*h).mb.qp = prealloc_size as *mut libc::c_void as *mut int8_t;
    let fresh0 = prealloc_idx;
    prealloc_idx += 1;
    preallocs[fresh0 as usize] = &mut (*h).mb.qp as *mut *mut int8_t as *mut *mut uint8_t;
    prealloc_size += ((i_mb_count as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<int8_t>() as libc::c_ulong)
        as int64_t
        + (64 as libc::c_int - 1 as libc::c_int) as int64_t)
        & !(64 as libc::c_int - 1 as libc::c_int) as int64_t;
    (*h).mb.cbp = prealloc_size as *mut libc::c_void as *mut int16_t;
    let fresh1 = prealloc_idx;
    prealloc_idx += 1;
    preallocs[fresh1 as usize] = &mut (*h).mb.cbp as *mut *mut int16_t as *mut *mut uint8_t;
    prealloc_size += ((i_mb_count as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong)
        as int64_t
        + (64 as libc::c_int - 1 as libc::c_int) as int64_t)
        & !(64 as libc::c_int - 1 as libc::c_int) as int64_t;
    (*h).mb.mb_transform_size = prealloc_size as *mut libc::c_void as *mut int8_t;
    let fresh2 = prealloc_idx;
    prealloc_idx += 1;
    preallocs[fresh2 as usize] =
        &mut (*h).mb.mb_transform_size as *mut *mut int8_t as *mut *mut uint8_t;
    prealloc_size += ((i_mb_count as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<int8_t>() as libc::c_ulong)
        as int64_t
        + (64 as libc::c_int - 1 as libc::c_int) as int64_t)
        & !(64 as libc::c_int - 1 as libc::c_int) as int64_t;
    (*h).mb.slice_table = prealloc_size as *mut libc::c_void as *mut int32_t;
    let fresh3 = prealloc_idx;
    prealloc_idx += 1;
    preallocs[fresh3 as usize] = &mut (*h).mb.slice_table as *mut *mut int32_t as *mut *mut uint8_t;
    prealloc_size += ((i_mb_count as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<int32_t>() as libc::c_ulong)
        as int64_t
        + (64 as libc::c_int - 1 as libc::c_int) as int64_t)
        & !(64 as libc::c_int - 1 as libc::c_int) as int64_t;
    (*h).mb.intra4x4_pred_mode = prealloc_size as *mut libc::c_void as *mut [int8_t; 8];
    let fresh4 = prealloc_idx;
    prealloc_idx += 1;
    preallocs[fresh4 as usize] =
        &mut (*h).mb.intra4x4_pred_mode as *mut *mut [int8_t; 8] as *mut *mut uint8_t;
    prealloc_size += (((i_mb_count * 8 as libc::c_int) as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<int8_t>() as libc::c_ulong)
        as int64_t
        + (64 as libc::c_int - 1 as libc::c_int) as int64_t)
        & !(64 as libc::c_int - 1 as libc::c_int) as int64_t;
    (*h).mb.non_zero_count = prealloc_size as *mut libc::c_void as *mut [uint8_t; 48];
    let fresh5 = prealloc_idx;
    prealloc_idx += 1;
    preallocs[fresh5 as usize] =
        &mut (*h).mb.non_zero_count as *mut *mut [uint8_t; 48] as *mut *mut uint8_t;
    prealloc_size += (((i_mb_count * 48 as libc::c_int) as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<uint8_t>() as libc::c_ulong)
        as int64_t
        + (64 as libc::c_int - 1 as libc::c_int) as int64_t)
        & !(64 as libc::c_int - 1 as libc::c_int) as int64_t;
    if (*h).param.b_cabac != 0 {
        (*h).mb.skipbp = prealloc_size as *mut libc::c_void as *mut int8_t;
        let fresh6 = prealloc_idx;
        prealloc_idx += 1;
        preallocs[fresh6 as usize] = &mut (*h).mb.skipbp as *mut *mut int8_t as *mut *mut uint8_t;
        prealloc_size += ((i_mb_count as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<int8_t>() as libc::c_ulong)
            as int64_t
            + (64 as libc::c_int - 1 as libc::c_int) as int64_t)
            & !(64 as libc::c_int - 1 as libc::c_int) as int64_t;
        (*h).mb.chroma_pred_mode = prealloc_size as *mut libc::c_void as *mut int8_t;
        let fresh7 = prealloc_idx;
        prealloc_idx += 1;
        preallocs[fresh7 as usize] =
            &mut (*h).mb.chroma_pred_mode as *mut *mut int8_t as *mut *mut uint8_t;
        prealloc_size += ((i_mb_count as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<int8_t>() as libc::c_ulong)
            as int64_t
            + (64 as libc::c_int - 1 as libc::c_int) as int64_t)
            & !(64 as libc::c_int - 1 as libc::c_int) as int64_t;
        (*h).mb.mvd[0 as libc::c_int as usize] =
            prealloc_size as *mut libc::c_void as *mut [[uint8_t; 2]; 8];
        let fresh8 = prealloc_idx;
        prealloc_idx += 1;
        preallocs[fresh8 as usize] =
            &mut *((*h).mb.mvd).as_mut_ptr().offset(0 as libc::c_int as isize)
                as *mut *mut [[uint8_t; 2]; 8] as *mut *mut uint8_t;
        prealloc_size += ((i_mb_count as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<[[uint8_t; 2]; 8]>() as libc::c_ulong)
            as int64_t
            + (64 as libc::c_int - 1 as libc::c_int) as int64_t)
            & !(64 as libc::c_int - 1 as libc::c_int) as int64_t;
        if (*h).param.i_bframe != 0 {
            (*h).mb.mvd[1 as libc::c_int as usize] =
                prealloc_size as *mut libc::c_void as *mut [[uint8_t; 2]; 8];
            let fresh9 = prealloc_idx;
            prealloc_idx += 1;
            preallocs[fresh9 as usize] =
                &mut *((*h).mb.mvd).as_mut_ptr().offset(1 as libc::c_int as isize)
                    as *mut *mut [[uint8_t; 2]; 8] as *mut *mut uint8_t;
            prealloc_size += ((i_mb_count as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<[[uint8_t; 2]; 8]>() as libc::c_ulong)
                as int64_t
                + (64 as libc::c_int - 1 as libc::c_int) as int64_t)
                & !(64 as libc::c_int - 1 as libc::c_int) as int64_t;
        }
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        let mut i_refs: libc::c_int = (if (16 as libc::c_int)
            < (if i != 0 {
                1 as libc::c_int + ((*h).param.i_bframe_pyramid != 0) as libc::c_int
            } else {
                (*h).param.i_frame_reference
            }) {
            16 as libc::c_int
        } else if i != 0 {
            1 as libc::c_int + ((*h).param.i_bframe_pyramid != 0) as libc::c_int
        } else {
            (*h).param.i_frame_reference
        }) << (*h).param.b_interlaced;
        if (*h).param.analyse.i_weighted_pred == 2 as libc::c_int {
            i_refs = if (16 as libc::c_int)
                < i_refs + 1 as libc::c_int + (8 as libc::c_int == 8 as libc::c_int) as libc::c_int
            {
                16 as libc::c_int
            } else {
                i_refs + 1 as libc::c_int + (8 as libc::c_int == 8 as libc::c_int) as libc::c_int
            };
        }
        let mut j: libc::c_int = (i == 0) as libc::c_int;
        while j < i_refs {
            (*h).mb.mvr[i as usize][j as usize] =
                prealloc_size as *mut libc::c_void as *mut [int16_t; 2];
            let fresh10 = prealloc_idx;
            prealloc_idx += 1;
            preallocs[fresh10 as usize] = &mut *(*((*h).mb.mvr).as_mut_ptr().offset(i as isize))
                .as_mut_ptr()
                .offset(j as isize)
                as *mut *mut [int16_t; 2]
                as *mut *mut uint8_t;
            prealloc_size += (((2 as libc::c_int * (i_mb_count + 1 as libc::c_int))
                as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong)
                as int64_t
                + (64 as libc::c_int - 1 as libc::c_int) as int64_t)
                & !(64 as libc::c_int - 1 as libc::c_int) as int64_t;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    if (*h).param.analyse.i_weighted_pred != 0 {
        let mut i_padv: libc::c_int = (32 as libc::c_int) << (*h).param.b_interlaced;
        let mut luma_plane_size: libc::c_int = 0 as libc::c_int;
        let mut numweightbuf: libc::c_int = 0;
        if (*h).param.analyse.i_weighted_pred == -(1 as libc::c_int) {
            if (*h).param.i_sync_lookahead == 0 || h == (*h).thread[(*h).param.i_threads as usize] {
                luma_plane_size = (*(*h).fdec).i_stride_lowres
                    * ((*h).mb.i_mb_height * 8 as libc::c_int + 2 as libc::c_int * i_padv);
                numweightbuf = 1 as libc::c_int;
            } else {
                numweightbuf = 0 as libc::c_int;
            }
        } else {
            luma_plane_size = (*(*h).fdec).i_stride[0 as libc::c_int as usize]
                * ((*h).mb.i_mb_height
                    * ((16 as libc::c_int)
                        << ((*((*h).sps).as_mut_ptr()).i_chroma_format_idc
                            == CHROMA_422 as libc::c_int)
                            as libc::c_int)
                    + 2 as libc::c_int * i_padv);
            if (*h).param.analyse.i_weighted_pred == 2 as libc::c_int {
                numweightbuf =
                    1 as libc::c_int + (8 as libc::c_int == 8 as libc::c_int) as libc::c_int;
            } else {
                numweightbuf = 1 as libc::c_int;
            }
        }
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < numweightbuf {
            (*h).mb.p_weight_buf[i_0 as usize] = prealloc_size as *mut libc::c_void as *mut pixel;
            let fresh11 = prealloc_idx;
            prealloc_idx += 1;
            preallocs[fresh11 as usize] =
                &mut *((*h).mb.p_weight_buf).as_mut_ptr().offset(i_0 as isize) as *mut *mut pixel
                    as *mut *mut uint8_t;
            prealloc_size += ((luma_plane_size
                * ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int)
                as int64_t
                + (64 as libc::c_int - 1 as libc::c_int) as int64_t)
                & !(64 as libc::c_int - 1 as libc::c_int) as int64_t;
            i_0 += 1;
            i_0;
        }
    }
    (*h).mb.base = x264_malloc(prealloc_size) as *mut uint8_t;
    if ((*h).mb.base).is_null() {
        -(1 as libc::c_int)
    } else {
        loop {
            let fresh12 = prealloc_idx;
            prealloc_idx -= 1;
            if fresh12 == 0 {
                break;
            }
            *preallocs[prealloc_idx as usize] = (*preallocs[prealloc_idx as usize] as intptr_t
                + (*h).mb.base as intptr_t)
                as *mut uint8_t;
        }
        memset(
            (*h).mb.slice_table as *mut libc::c_void,
            -(1 as libc::c_int),
            (i_mb_count as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<int32_t>() as libc::c_ulong),
        );
        let mut i_1: libc::c_int = 0 as libc::c_int;
        while i_1 < 2 as libc::c_int {
            let mut i_refs_0: libc::c_int = (if (16 as libc::c_int)
                < (if i_1 != 0 {
                    1 as libc::c_int + ((*h).param.i_bframe_pyramid != 0) as libc::c_int
                } else {
                    (*h).param.i_frame_reference
                }) {
                16 as libc::c_int
            } else if i_1 != 0 {
                1 as libc::c_int + ((*h).param.i_bframe_pyramid != 0) as libc::c_int
            } else {
                (*h).param.i_frame_reference
            }) << (*h).param.b_interlaced;
            if (*h).param.analyse.i_weighted_pred == 2 as libc::c_int {
                i_refs_0 = if (16 as libc::c_int)
                    < i_refs_0
                        + 1 as libc::c_int
                        + (8 as libc::c_int == 8 as libc::c_int) as libc::c_int
                {
                    16 as libc::c_int
                } else {
                    i_refs_0
                        + 1 as libc::c_int
                        + (8 as libc::c_int == 8 as libc::c_int) as libc::c_int
                };
            }
            let mut j_0: libc::c_int = (i_1 == 0) as libc::c_int;
            while j_0 < i_refs_0 {
                (*((*((*h).mb.mvr[i_1 as usize][j_0 as usize]).offset(0 as libc::c_int as isize))
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i = 0 as libc::c_int as uint32_t;
                (*h).mb.mvr[i_1 as usize][j_0 as usize] =
                    ((*h).mb.mvr[i_1 as usize][j_0 as usize]).offset(1);
                (*h).mb.mvr[i_1 as usize][j_0 as usize];
                j_0 += 1;
                j_0;
            }
            i_1 += 1;
            i_1;
        }
        0 as libc::c_int
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_macroblock_cache_free(mut h: *mut x264_t) {
    x264_free((*h).mb.base as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_macroblock_thread_allocate(
    mut h: *mut x264_t,
    mut b_lookahead: libc::c_int,
) -> libc::c_int {
    let mut scratch_size: libc::c_int = 0;
    let mut buf_mbtree: libc::c_int = 0;
    let mut buf_lookahead_threads: libc::c_int = 0;
    let mut buf_mbtree2: libc::c_int = 0;
    let mut current_block: u64;
    if b_lookahead == 0 {
        let mut i: libc::c_int = 0 as libc::c_int;
        's_5: loop {
            if i >= (if (*h).param.b_interlaced != 0 {
                5 as libc::c_int
            } else {
                2 as libc::c_int
            }) {
                current_block = 8515828400728868193;
                break;
            }
            let mut j: libc::c_int = 0 as libc::c_int;
            while j
                < (if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as libc::c_int {
                    3 as libc::c_int
                } else {
                    2 as libc::c_int
                })
            {
                (*h).intra_border_backup[i as usize][j as usize] = x264_malloc(
                    (((*((*h).sps).as_mut_ptr()).i_mb_width * 16 as libc::c_int
                        + 32 as libc::c_int)
                        * ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int)
                        as int64_t,
                ) as *mut pixel;
                if ((*h).intra_border_backup[i as usize][j as usize]).is_null() {
                    current_block = 16210657447058269181;
                    break 's_5;
                }
                (*h).intra_border_backup[i as usize][j as usize] = ((*h).intra_border_backup
                    [i as usize][j as usize])
                    .offset(16 as libc::c_int as isize);
                j += 1;
                j;
            }
            i += 1;
            i;
        }
        match current_block {
            16210657447058269181 => {}
            _ => {
                let mut i_0: libc::c_int = 0 as libc::c_int;
                loop {
                    if i_0 > (*h).param.b_interlaced {
                        current_block = 5783071609795492627;
                        break;
                    }
                    if (*h).param.b_sliced_threads != 0 {
                        if h == (*h).thread[0 as libc::c_int as usize] && i_0 == 0 {
                            (*h).deblock_strength[0 as libc::c_int as usize] = x264_malloc(
                                (::core::mem::size_of::<[[[uint8_t; 4]; 8]; 2]>() as libc::c_ulong)
                                    .wrapping_mul((*h).mb.i_mb_count as libc::c_ulong)
                                    as int64_t,
                            )
                                as *mut [[[uint8_t; 4]; 8]; 2];
                            if ((*h).deblock_strength[0 as libc::c_int as usize]).is_null() {
                                current_block = 16210657447058269181;
                                break;
                            }
                        } else {
                            (*h).deblock_strength[i_0 as usize] = (*(*h).thread
                                [0 as libc::c_int as usize])
                                .deblock_strength[0 as libc::c_int as usize];
                        }
                    } else {
                        (*h).deblock_strength[i_0 as usize] = x264_malloc(
                            (::core::mem::size_of::<[[[uint8_t; 4]; 8]; 2]>() as libc::c_ulong)
                                .wrapping_mul((*h).mb.i_mb_width as libc::c_ulong)
                                as int64_t,
                        )
                            as *mut [[[uint8_t; 4]; 8]; 2];
                        if ((*h).deblock_strength[i_0 as usize]).is_null() {
                            current_block = 16210657447058269181;
                            break;
                        }
                    }
                    (*h).deblock_strength[1 as libc::c_int as usize] =
                        (*h).deblock_strength[i_0 as usize];
                    i_0 += 1;
                    i_0;
                }
            }
        }
    } else {
        current_block = 5783071609795492627;
    }
    if current_block == 5783071609795492627 {
        scratch_size = 0 as libc::c_int;
        if b_lookahead == 0 {
            let mut buf_hpel: libc::c_int = (((*(*(*h).thread[0 as libc::c_int as usize]).fdec)
                .i_width[0 as libc::c_int as usize]
                + 48 as libc::c_int
                + 32 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong)
                as libc::c_int;
            let mut buf_ssim: libc::c_int = (((*h).param.analyse.b_ssim
                * 8 as libc::c_int
                * ((*h).param.i_width / 4 as libc::c_int + 3 as libc::c_int))
                as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                as libc::c_int;
            let mut me_range: libc::c_int =
                if (*h).param.analyse.i_me_range < (*h).param.analyse.i_mv_range {
                    (*h).param.analyse.i_me_range
                } else {
                    (*h).param.analyse.i_mv_range
                };
            let mut buf_tesa: libc::c_int = (((*h).param.analyse.i_me_method >= 3 as libc::c_int)
                as libc::c_int as libc::c_ulong)
                .wrapping_mul(
                    ((me_range * 2 as libc::c_int + 24 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong)
                        .wrapping_add(
                            (((me_range + 4 as libc::c_int)
                                * (me_range + 1 as libc::c_int)
                                * 4 as libc::c_int) as libc::c_ulong)
                                .wrapping_mul(::core::mem::size_of::<mvsad_t>() as libc::c_ulong),
                        ),
                ) as libc::c_int;
            scratch_size = if buf_hpel
                > (if buf_ssim > buf_tesa {
                    buf_ssim
                } else {
                    buf_tesa
                }) {
                buf_hpel
            } else if buf_ssim > buf_tesa {
                buf_ssim
            } else {
                buf_tesa
            };
        }
        buf_mbtree = ((*h).param.rc.b_mb_tree as libc::c_ulong).wrapping_mul(
            ((*h).mb.i_mb_width as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong)
                .wrapping_add((64 as libc::c_int - 1 as libc::c_int) as libc::c_ulong)
                & !(64 as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
        ) as libc::c_int;
        scratch_size = if scratch_size > buf_mbtree {
            scratch_size
        } else {
            buf_mbtree
        };
        if scratch_size != 0 {
            (*h).scratch_buffer = x264_malloc(scratch_size as int64_t);
            if ((*h).scratch_buffer).is_null() {
                current_block = 16210657447058269181;
            } else {
                current_block = 2891135413264362348;
            }
        } else {
            (*h).scratch_buffer = std::ptr::null_mut::<libc::c_void>();
            current_block = 2891135413264362348;
        }
        match current_block {
            16210657447058269181 => {}
            _ => {
                buf_lookahead_threads = (((*h).mb.i_mb_height
                    + (4 as libc::c_int + 32 as libc::c_int) * (*h).param.i_lookahead_threads)
                    as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                    as libc::c_int;
                buf_mbtree2 = buf_mbtree * 12 as libc::c_int;
                scratch_size = if buf_lookahead_threads > buf_mbtree2 {
                    buf_lookahead_threads
                } else {
                    buf_mbtree2
                };
                (*h).scratch_buffer2 = x264_malloc(scratch_size as int64_t);
                if !((*h).scratch_buffer2).is_null() {
                    return 0 as libc::c_int;
                }
            }
        }
    }
    -(1 as libc::c_int)
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_macroblock_thread_free(
    mut h: *mut x264_t,
    mut b_lookahead: libc::c_int,
) {
    if b_lookahead == 0 {
        let mut i: libc::c_int = 0 as libc::c_int;
        while i <= (*h).param.b_interlaced {
            if (*h).param.b_sliced_threads == 0
                || h == (*h).thread[0 as libc::c_int as usize] && i == 0
            {
                x264_free((*h).deblock_strength[i as usize] as *mut libc::c_void);
            }
            i += 1;
            i;
        }
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0
            < (if (*h).param.b_interlaced != 0 {
                5 as libc::c_int
            } else {
                2 as libc::c_int
            })
        {
            let mut j: libc::c_int = 0 as libc::c_int;
            while j
                < (if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as libc::c_int {
                    3 as libc::c_int
                } else {
                    2 as libc::c_int
                })
            {
                x264_free(
                    ((*h).intra_border_backup[i_0 as usize][j as usize])
                        .offset(-(16 as libc::c_int as isize))
                        as *mut libc::c_void,
                );
                j += 1;
                j;
            }
            i_0 += 1;
            i_0;
        }
    }
    x264_free((*h).scratch_buffer);
    x264_free((*h).scratch_buffer2);
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_macroblock_slice_init(mut h: *mut x264_t) {
    (*h).mb.mv[0 as libc::c_int as usize] = (*(*h).fdec).mv[0 as libc::c_int as usize];
    (*h).mb.mv[1 as libc::c_int as usize] = (*(*h).fdec).mv[1 as libc::c_int as usize];
    (*h).mb.mvr[0 as libc::c_int as usize][0 as libc::c_int as usize] = (*(*h).fdec).mv16x16;
    (*h).mb.ref_0[0 as libc::c_int as usize] = (*(*h).fdec).ref_0[0 as libc::c_int as usize];
    (*h).mb.ref_0[1 as libc::c_int as usize] = (*(*h).fdec).ref_0[1 as libc::c_int as usize];
    (*h).mb.type_0 = (*(*h).fdec).mb_type;
    (*h).mb.partition = (*(*h).fdec).mb_partition;
    (*h).mb.field = (*(*h).fdec).field;
    (*(*h).fdec).i_ref[0 as libc::c_int as usize] = (*h).i_ref[0 as libc::c_int as usize];
    (*(*h).fdec).i_ref[1 as libc::c_int as usize] = (*h).i_ref[1 as libc::c_int as usize];
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*h).i_ref[0 as libc::c_int as usize] {
        (*(*h).fdec).ref_poc[0 as libc::c_int as usize][i as usize] =
            (*(*h).fref[0 as libc::c_int as usize][i as usize]).i_poc;
        i += 1;
        i;
    }
    if (*h).sh.i_type == SLICE_TYPE_B as libc::c_int {
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < (*h).i_ref[1 as libc::c_int as usize] {
            (*(*h).fdec).ref_poc[1 as libc::c_int as usize][i_0 as usize] =
                (*(*h).fref[1 as libc::c_int as usize][i_0 as usize]).i_poc;
            i_0 += 1;
            i_0;
        }
        (*h).mb.map_col_to_list0[(-(1 as libc::c_int) + 2 as libc::c_int) as usize] =
            -(1 as libc::c_int) as int8_t;
        (*h).mb.map_col_to_list0[(-(2 as libc::c_int) + 2 as libc::c_int) as usize] =
            -(2 as libc::c_int) as int8_t;
        let mut i_1: libc::c_int = 0 as libc::c_int;
        while i_1
            < (*(*h).fref[1 as libc::c_int as usize][0 as libc::c_int as usize]).i_ref
                [0 as libc::c_int as usize]
        {
            let mut poc: libc::c_int = (*(*h).fref[1 as libc::c_int as usize]
                [0 as libc::c_int as usize])
                .ref_poc[0 as libc::c_int as usize][i_1 as usize];
            (*h).mb.map_col_to_list0[(i_1 + 2 as libc::c_int) as usize] =
                -(2 as libc::c_int) as int8_t;
            let mut j: libc::c_int = 0 as libc::c_int;
            while j < (*h).i_ref[0 as libc::c_int as usize] {
                if (*(*h).fref[0 as libc::c_int as usize][j as usize]).i_poc == poc {
                    (*h).mb.map_col_to_list0[(i_1 + 2 as libc::c_int) as usize] = j as int8_t;
                    break;
                } else {
                    j += 1;
                    j;
                }
            }
            i_1 += 1;
            i_1;
        }
    } else if (*h).sh.i_type == SLICE_TYPE_P as libc::c_int
        && (*h).sh.i_disable_deblocking_filter_idc != 1 as libc::c_int
        && (*h).param.analyse.i_weighted_pred == 2 as libc::c_int
    {
        (*h).mb.deblock_ref_table[(-(2 as libc::c_int) + 2 as libc::c_int) as usize] =
            -(2 as libc::c_int) as int8_t;
        (*h).mb.deblock_ref_table[(-(1 as libc::c_int) + 2 as libc::c_int) as usize] =
            -(1 as libc::c_int) as int8_t;
        let mut i_2: libc::c_int = 0 as libc::c_int;
        while i_2 < (*h).i_ref[0 as libc::c_int as usize] << (*h).sh.b_mbaff {
            if (*h).mb.b_interlaced == 0 {
                (*h).mb.deblock_ref_table[(i_2 + 2 as libc::c_int) as usize] =
                    ((*(*h).fref[0 as libc::c_int as usize][i_2 as usize]).i_frame_num
                        & 63 as libc::c_int) as int8_t;
            } else {
                (*h).mb.deblock_ref_table[(i_2 + 2 as libc::c_int) as usize] = ((((*(*h).fref
                    [0 as libc::c_int as usize][(i_2 >> 1 as libc::c_int) as usize])
                    .i_frame_num
                    & 63 as libc::c_int)
                    << 1 as libc::c_int)
                    + (i_2 & 1 as libc::c_int))
                    as int8_t;
            }
            i_2 += 1;
            i_2;
        }
    }
    memset(
        ((*h).mb.cache.ref_0).as_mut_ptr() as *mut libc::c_void,
        -(2 as libc::c_int),
        ::core::mem::size_of::<[[int8_t; 40]; 2]>() as libc::c_ulong,
    );
    if (*h).i_ref[0 as libc::c_int as usize] > 0 as libc::c_int {
        let mut field: libc::c_int = 0 as libc::c_int;
        while field <= (*h).sh.b_mbaff {
            let mut curpoc: libc::c_int =
                (*(*h).fdec).i_poc + (*(*h).fdec).i_delta_poc[field as usize];
            let mut refpoc: libc::c_int =
                (*(*h).fref[0 as libc::c_int as usize][0 as libc::c_int as usize]).i_poc
                    + (*(*h).fref[0 as libc::c_int as usize][0 as libc::c_int as usize])
                        .i_delta_poc[field as usize];
            let mut delta: libc::c_int = curpoc - refpoc;
            (*(*h).fdec).inv_ref_poc[field as usize] =
                ((256 as libc::c_int + delta / 2 as libc::c_int) / delta) as int16_t;
            field += 1;
            field;
        }
    }
    (*h).mb.i_neighbour4[14 as libc::c_int as usize] = (MB_LEFT as libc::c_int
        | MB_TOP as libc::c_int
        | MB_TOPLEFT as libc::c_int
        | MB_TOPRIGHT as libc::c_int)
        as libc::c_uint;
    (*h).mb.i_neighbour4[12 as libc::c_int as usize] =
        (*h).mb.i_neighbour4[14 as libc::c_int as usize];
    (*h).mb.i_neighbour4[9 as libc::c_int as usize] =
        (*h).mb.i_neighbour4[12 as libc::c_int as usize];
    (*h).mb.i_neighbour4[6 as libc::c_int as usize] =
        (*h).mb.i_neighbour4[9 as libc::c_int as usize];
    (*h).mb.i_neighbour8[3 as libc::c_int as usize] =
        (MB_LEFT as libc::c_int | MB_TOP as libc::c_int | MB_TOPLEFT as libc::c_int)
            as libc::c_uint;
    (*h).mb.i_neighbour4[15 as libc::c_int as usize] =
        (*h).mb.i_neighbour8[3 as libc::c_int as usize];
    (*h).mb.i_neighbour4[13 as libc::c_int as usize] =
        (*h).mb.i_neighbour4[15 as libc::c_int as usize];
    (*h).mb.i_neighbour4[11 as libc::c_int as usize] =
        (*h).mb.i_neighbour4[13 as libc::c_int as usize];
    (*h).mb.i_neighbour4[7 as libc::c_int as usize] =
        (*h).mb.i_neighbour4[11 as libc::c_int as usize];
    (*h).mb.i_neighbour4[3 as libc::c_int as usize] =
        (*h).mb.i_neighbour4[7 as libc::c_int as usize];
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_macroblock_thread_init(mut h: *mut x264_t) {
    (*h).mb.i_me_method = (*h).param.analyse.i_me_method;
    (*h).mb.i_subpel_refine = (*h).param.analyse.i_subpel_refine;
    if (*h).sh.i_type == SLICE_TYPE_B as libc::c_int
        && ((*h).mb.i_subpel_refine == 6 as libc::c_int
            || (*h).mb.i_subpel_refine == 8 as libc::c_int)
    {
        (*h).mb.i_subpel_refine -= 1;
        (*h).mb.i_subpel_refine;
    }
    (*h).mb.b_chroma_me = ((*h).param.analyse.b_chroma_me != 0
        && ((*h).sh.i_type == SLICE_TYPE_P as libc::c_int
            && (*h).mb.i_subpel_refine >= 5 as libc::c_int
            || (*h).sh.i_type == SLICE_TYPE_B as libc::c_int
                && (*h).mb.i_subpel_refine >= 9 as libc::c_int))
        as libc::c_int;
    (*h).mb.b_dct_decimate = ((*h).sh.i_type == SLICE_TYPE_B as libc::c_int
        || (*h).param.analyse.b_dct_decimate != 0 && (*h).sh.i_type != SLICE_TYPE_I as libc::c_int)
        as libc::c_int;
    (*h).mb.i_mb_prev_xy = -(1 as libc::c_int);
    (*h).mb.pic.p_fenc[0 as libc::c_int as usize] = ((*h).mb.pic.fenc_buf).as_mut_ptr();
    (*h).mb.pic.p_fdec[0 as libc::c_int as usize] = ((*h).mb.pic.fdec_buf)
        .as_mut_ptr()
        .offset((2 as libc::c_int * 32 as libc::c_int) as isize);
    if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc != 0 {
        (*h).mb.pic.p_fenc[1 as libc::c_int as usize] = ((*h).mb.pic.fenc_buf)
            .as_mut_ptr()
            .offset((16 as libc::c_int * 16 as libc::c_int) as isize);
        (*h).mb.pic.p_fdec[1 as libc::c_int as usize] = ((*h).mb.pic.fdec_buf)
            .as_mut_ptr()
            .offset((20 as libc::c_int * 32 as libc::c_int) as isize);
        if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as libc::c_int {
            (*h).mb.pic.p_fenc[2 as libc::c_int as usize] = ((*h).mb.pic.fenc_buf)
                .as_mut_ptr()
                .offset((32 as libc::c_int * 16 as libc::c_int) as isize);
            (*h).mb.pic.p_fdec[2 as libc::c_int as usize] = ((*h).mb.pic.fdec_buf)
                .as_mut_ptr()
                .offset((38 as libc::c_int * 32 as libc::c_int) as isize);
        } else {
            (*h).mb.pic.p_fenc[2 as libc::c_int as usize] = ((*h).mb.pic.fenc_buf)
                .as_mut_ptr()
                .offset((16 as libc::c_int * 16 as libc::c_int) as isize)
                .offset(8 as libc::c_int as isize);
            (*h).mb.pic.p_fdec[2 as libc::c_int as usize] = ((*h).mb.pic.fdec_buf)
                .as_mut_ptr()
                .offset((20 as libc::c_int * 32 as libc::c_int) as isize)
                .offset(16 as libc::c_int as isize);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_prefetch_fenc(
    mut h: *mut x264_t,
    mut fenc: *mut x264_frame_t,
    mut i_mb_x: libc::c_int,
    mut i_mb_y: libc::c_int,
) {
    let mut stride_y: libc::c_int = (*fenc).i_stride[0 as libc::c_int as usize];
    let mut stride_uv: libc::c_int = (*fenc).i_stride[1 as libc::c_int as usize];
    let mut off_y: libc::c_int = 16 as libc::c_int * i_mb_x + 16 as libc::c_int * i_mb_y * stride_y;
    let mut off_uv: libc::c_int = 16 as libc::c_int * i_mb_x
        + ((16 as libc::c_int * i_mb_y * stride_uv) >> (*h).mb.chroma_v_shift);
    ((*h).mc.prefetch_fenc).expect("non-null function pointer")(
        ((*fenc).plane[0 as libc::c_int as usize]).offset(off_y as isize),
        stride_y as intptr_t,
        if !((*fenc).plane[1 as libc::c_int as usize]).is_null() {
            ((*fenc).plane[1 as libc::c_int as usize]).offset(off_uv as isize)
        } else {
            std::ptr::null_mut::<pixel>()
        },
        stride_uv as intptr_t,
        i_mb_x,
    );
}
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn x264_8_copy_column8(mut dst: *mut pixel, mut src: *mut pixel) {
    let mut i: libc::c_int = -(4 as libc::c_int);
    while i < 4 as libc::c_int {
        *dst.offset((i * 32 as libc::c_int) as isize) =
            *src.offset((i * 32 as libc::c_int) as isize);
        i += 1;
        i;
    }
}
#[inline(always)]
unsafe extern "C" fn macroblock_load_pic_pointers(
    mut h: *mut x264_t,
    mut mb_x: libc::c_int,
    mut mb_y: libc::c_int,
    mut i: libc::c_int,
    mut b_chroma: libc::c_int,
    mut b_mbaff: libc::c_int,
) {
    let mut mb_interlaced: libc::c_int = (b_mbaff != 0 && (*h).mb.b_interlaced != 0) as libc::c_int;
    let mut height: libc::c_int = if b_chroma != 0 {
        16 as libc::c_int >> (*h).mb.chroma_v_shift
    } else {
        16 as libc::c_int
    };
    let mut i_stride: libc::c_int = (*(*h).fdec).i_stride[i as usize];
    let mut i_stride2: libc::c_int = i_stride << mb_interlaced;
    let mut i_pix_offset: libc::c_int = if mb_interlaced != 0 {
        16 as libc::c_int * mb_x
            + height * (mb_y & !(1 as libc::c_int)) * i_stride
            + (mb_y & 1 as libc::c_int) * i_stride
    } else {
        16 as libc::c_int * mb_x + height * mb_y * i_stride
    };
    let mut plane_fdec: *mut pixel = &mut *(*((*(*h).fdec).plane).as_mut_ptr().offset(i as isize))
        .offset(i_pix_offset as isize) as *mut pixel;
    let mut fdec_idx: libc::c_int = if b_mbaff != 0 {
        if mb_interlaced != 0 {
            3 as libc::c_int + (mb_y & 1 as libc::c_int)
        } else if mb_y & 1 as libc::c_int != 0 {
            2 as libc::c_int
        } else {
            4 as libc::c_int
        }
    } else {
        (mb_y & 1 as libc::c_int == 0) as libc::c_int
    };
    let mut intra_fdec: *mut pixel = &mut *(*(*((*h).intra_border_backup)
        .as_mut_ptr()
        .offset(fdec_idx as isize))
    .as_mut_ptr()
    .offset(i as isize))
    .offset((mb_x * 16 as libc::c_int) as isize) as *mut pixel;
    let mut ref_pix_offset: [libc::c_int; 2] = [i_pix_offset, i_pix_offset];
    if mb_interlaced != 0 {
        ref_pix_offset[1 as libc::c_int as usize] +=
            (1 as libc::c_int - 2 as libc::c_int * (mb_y & 1 as libc::c_int)) * i_stride;
    }
    (*h).mb.pic.i_stride[i as usize] = i_stride2;
    (*h).mb.pic.p_fenc_plane[i as usize] =
        &mut *(*((*(*h).fenc).plane).as_mut_ptr().offset(i as isize)).offset(i_pix_offset as isize)
            as *mut pixel;
    if b_chroma != 0 {
        ((*h).mc.load_deinterleave_chroma_fenc).expect("non-null function pointer")(
            (*h).mb.pic.p_fenc[1 as libc::c_int as usize],
            (*h).mb.pic.p_fenc_plane[1 as libc::c_int as usize],
            i_stride2 as intptr_t,
            height,
        );
        memcpy(
            ((*h).mb.pic.p_fdec[1 as libc::c_int as usize]).offset(-(32 as libc::c_int as isize))
                as *mut libc::c_void,
            intra_fdec as *const libc::c_void,
            (8 as libc::c_int * ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int)
                as libc::c_ulong,
        );
        memcpy(
            ((*h).mb.pic.p_fdec[2 as libc::c_int as usize]).offset(-(32 as libc::c_int as isize))
                as *mut libc::c_void,
            intra_fdec.offset(8 as libc::c_int as isize) as *const libc::c_void,
            (8 as libc::c_int * ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int)
                as libc::c_ulong,
        );
        *((*h).mb.pic.p_fdec[1 as libc::c_int as usize])
            .offset((-(32 as libc::c_int) - 1 as libc::c_int) as isize) =
            *intra_fdec.offset((-(1 as libc::c_int) - 8 as libc::c_int) as isize);
        *((*h).mb.pic.p_fdec[2 as libc::c_int as usize])
            .offset((-(32 as libc::c_int) - 1 as libc::c_int) as isize) =
            *intra_fdec.offset(-(1 as libc::c_int) as isize);
    } else {
        ((*h).mc.copy[PIXEL_16x16 as libc::c_int as usize]).expect("non-null function pointer")(
            (*h).mb.pic.p_fenc[i as usize],
            16 as libc::c_int as intptr_t,
            (*h).mb.pic.p_fenc_plane[i as usize],
            i_stride2 as intptr_t,
            16 as libc::c_int,
        );
        memcpy(
            ((*h).mb.pic.p_fdec[i as usize]).offset(-(32 as libc::c_int as isize))
                as *mut libc::c_void,
            intra_fdec as *const libc::c_void,
            (24 as libc::c_int * ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int)
                as libc::c_ulong,
        );
        *((*h).mb.pic.p_fdec[i as usize])
            .offset((-(32 as libc::c_int) - 1 as libc::c_int) as isize) =
            *intra_fdec.offset(-(1 as libc::c_int) as isize);
    }
    if b_mbaff != 0 || (*h).mb.b_reencode_mb != 0 {
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < height {
            if b_chroma != 0 {
                *((*h).mb.pic.p_fdec[1 as libc::c_int as usize])
                    .offset((-(1 as libc::c_int) + j * 32 as libc::c_int) as isize) =
                    *plane_fdec.offset((-(2 as libc::c_int) + j * i_stride2) as isize);
                *((*h).mb.pic.p_fdec[2 as libc::c_int as usize])
                    .offset((-(1 as libc::c_int) + j * 32 as libc::c_int) as isize) =
                    *plane_fdec.offset((-(1 as libc::c_int) + j * i_stride2) as isize);
            } else {
                *((*h).mb.pic.p_fdec[i as usize])
                    .offset((-(1 as libc::c_int) + j * 32 as libc::c_int) as isize) =
                    *plane_fdec.offset((-(1 as libc::c_int) + j * i_stride2) as isize);
            }
            j += 1;
            j;
        }
    }
    let mut plane_src: *mut pixel = std::ptr::null_mut::<pixel>();
    let mut filtered_src: *mut *mut pixel = std::ptr::null_mut::<*mut pixel>();
    let mut j_0: libc::c_int = 0 as libc::c_int;
    while j_0 < (*h).mb.pic.i_fref[0 as libc::c_int as usize] {
        if mb_interlaced != 0 {
            plane_src = (*(*h).fref[0 as libc::c_int as usize][(j_0 >> 1 as libc::c_int) as usize])
                .plane_fld[i as usize];
            filtered_src = ((*(*h).fref[0 as libc::c_int as usize]
                [(j_0 >> 1 as libc::c_int) as usize])
                .filtered_fld[i as usize])
                .as_mut_ptr();
        } else {
            plane_src = (*(*h).fref[0 as libc::c_int as usize][j_0 as usize]).plane[i as usize];
            filtered_src = ((*(*h).fref[0 as libc::c_int as usize][j_0 as usize]).filtered
                [i as usize])
                .as_mut_ptr();
        }
        (*h).mb.pic.p_fref[0 as libc::c_int as usize][j_0 as usize]
            [(i * 4 as libc::c_int) as usize] =
            plane_src.offset(ref_pix_offset[(j_0 & 1 as libc::c_int) as usize] as isize);
        if b_chroma == 0 {
            if (*h).param.analyse.i_subpel_refine != 0 {
                let mut k: libc::c_int = 1 as libc::c_int;
                while k < 4 as libc::c_int {
                    (*h).mb.pic.p_fref[0 as libc::c_int as usize][j_0 as usize]
                        [(i * 4 as libc::c_int + k) as usize] = (*filtered_src.offset(k as isize))
                        .offset(ref_pix_offset[(j_0 & 1 as libc::c_int) as usize] as isize);
                    k += 1;
                    k;
                }
            }
            if i == 0 {
                if !((*h).sh.weight[j_0 as usize][0 as libc::c_int as usize].weightfn).is_null() {
                    (*h).mb.pic.p_fref_w[j_0 as usize] = &mut *(*((*(*h).fenc).weighted)
                        .as_mut_ptr()
                        .offset((j_0 >> mb_interlaced) as isize))
                    .offset(
                        *ref_pix_offset
                            .as_mut_ptr()
                            .offset((j_0 & 1 as libc::c_int) as isize)
                            as isize,
                    ) as *mut pixel;
                } else {
                    (*h).mb.pic.p_fref_w[j_0 as usize] = (*h).mb.pic.p_fref
                        [0 as libc::c_int as usize][j_0 as usize]
                        [0 as libc::c_int as usize];
                }
            }
        }
        j_0 += 1;
        j_0;
    }
    if (*h).sh.i_type == SLICE_TYPE_B as libc::c_int {
        let mut j_1: libc::c_int = 0 as libc::c_int;
        while j_1 < (*h).mb.pic.i_fref[1 as libc::c_int as usize] {
            if mb_interlaced != 0 {
                plane_src = (*(*h).fref[1 as libc::c_int as usize]
                    [(j_1 >> 1 as libc::c_int) as usize])
                    .plane_fld[i as usize];
                filtered_src = ((*(*h).fref[1 as libc::c_int as usize]
                    [(j_1 >> 1 as libc::c_int) as usize])
                    .filtered_fld[i as usize])
                    .as_mut_ptr();
            } else {
                plane_src = (*(*h).fref[1 as libc::c_int as usize][j_1 as usize]).plane[i as usize];
                filtered_src = ((*(*h).fref[1 as libc::c_int as usize][j_1 as usize]).filtered
                    [i as usize])
                    .as_mut_ptr();
            }
            (*h).mb.pic.p_fref[1 as libc::c_int as usize][j_1 as usize]
                [(i * 4 as libc::c_int) as usize] =
                plane_src.offset(ref_pix_offset[(j_1 & 1 as libc::c_int) as usize] as isize);
            if b_chroma == 0 && (*h).param.analyse.i_subpel_refine != 0 {
                let mut k_0: libc::c_int = 1 as libc::c_int;
                while k_0 < 4 as libc::c_int {
                    (*h).mb.pic.p_fref[1 as libc::c_int as usize][j_1 as usize]
                        [(i * 4 as libc::c_int + k_0) as usize] = (*filtered_src
                        .offset(k_0 as isize))
                    .offset(ref_pix_offset[(j_1 & 1 as libc::c_int) as usize] as isize);
                    k_0 += 1;
                    k_0;
                }
            }
            j_1 += 1;
            j_1;
        }
    }
}
#[inline(always)]
unsafe extern "C" fn macroblock_cache_load_neighbours(
    mut h: *mut x264_t,
    mut mb_x: libc::c_int,
    mut mb_y: libc::c_int,
    mut b_interlaced: libc::c_int,
) {
    let mb_interlaced: libc::c_int =
        (b_interlaced != 0 && (*h).mb.b_interlaced != 0) as libc::c_int;
    let mut top_y: libc::c_int = mb_y - ((1 as libc::c_int) << mb_interlaced);
    let mut top: libc::c_int = top_y * (*h).mb.i_mb_stride + mb_x;
    (*h).mb.i_mb_x = mb_x;
    (*h).mb.i_mb_y = mb_y;
    (*h).mb.i_mb_xy = mb_y * (*h).mb.i_mb_stride + mb_x;
    (*h).mb.i_b8_xy = 2 as libc::c_int * (mb_y * (*h).mb.i_b8_stride + mb_x);
    (*h).mb.i_b4_xy = 4 as libc::c_int * (mb_y * (*h).mb.i_b4_stride + mb_x);
    (*h).mb.left_b8[1 as libc::c_int as usize] = -(1 as libc::c_int);
    (*h).mb.left_b8[0 as libc::c_int as usize] = (*h).mb.left_b8[1 as libc::c_int as usize];
    (*h).mb.left_b4[1 as libc::c_int as usize] = -(1 as libc::c_int);
    (*h).mb.left_b4[0 as libc::c_int as usize] = (*h).mb.left_b4[1 as libc::c_int as usize];
    (*h).mb.i_neighbour = 0 as libc::c_int as libc::c_uint;
    (*h).mb.i_neighbour_intra = 0 as libc::c_int as libc::c_uint;
    (*h).mb.i_neighbour_frame = 0 as libc::c_int as libc::c_uint;
    (*h).mb.i_mb_top_xy = -(1 as libc::c_int);
    (*h).mb.i_mb_top_y = -(1 as libc::c_int);
    (*h).mb.i_mb_left_xy[1 as libc::c_int as usize] = -(1 as libc::c_int);
    (*h).mb.i_mb_left_xy[0 as libc::c_int as usize] =
        (*h).mb.i_mb_left_xy[1 as libc::c_int as usize];
    (*h).mb.i_mb_topleft_xy = -(1 as libc::c_int);
    (*h).mb.i_mb_topright_xy = -(1 as libc::c_int);
    (*h).mb.i_mb_type_top = -(1 as libc::c_int);
    (*h).mb.i_mb_type_left[1 as libc::c_int as usize] = -(1 as libc::c_int);
    (*h).mb.i_mb_type_left[0 as libc::c_int as usize] =
        (*h).mb.i_mb_type_left[1 as libc::c_int as usize];
    (*h).mb.i_mb_type_topleft = -(1 as libc::c_int);
    (*h).mb.i_mb_type_topright = -(1 as libc::c_int);
    (*h).mb.left_index_table =
        &*left_indices.as_ptr().offset(3 as libc::c_int as isize) as *const x264_left_table_t;
    (*h).mb.topleft_partition = 0 as libc::c_int;
    let mut topleft_y: libc::c_int = top_y;
    let mut topright_y: libc::c_int = top_y;
    let mut left: [libc::c_int; 2] = [0; 2];
    left[1 as libc::c_int as usize] = (*h).mb.i_mb_xy - 1 as libc::c_int;
    left[0 as libc::c_int as usize] = left[1 as libc::c_int as usize];
    (*h).mb.left_b8[1 as libc::c_int as usize] = (*h).mb.i_b8_xy - 2 as libc::c_int;
    (*h).mb.left_b8[0 as libc::c_int as usize] = (*h).mb.left_b8[1 as libc::c_int as usize];
    (*h).mb.left_b4[1 as libc::c_int as usize] = (*h).mb.i_b4_xy - 4 as libc::c_int;
    (*h).mb.left_b4[0 as libc::c_int as usize] = (*h).mb.left_b4[1 as libc::c_int as usize];
    if b_interlaced != 0 {
        (*h).mb.i_mb_top_mbpair_xy = (*h).mb.i_mb_xy - 2 as libc::c_int * (*h).mb.i_mb_stride;
        (*h).mb.i_mb_topleft_y = -(1 as libc::c_int);
        (*h).mb.i_mb_topright_y = -(1 as libc::c_int);
        if mb_y & 1 as libc::c_int != 0 {
            if mb_x != 0
                && mb_interlaced
                    != *((*h).mb.field).offset(((*h).mb.i_mb_xy - 1 as libc::c_int) as isize)
                        as libc::c_int
            {
                left[1 as libc::c_int as usize] =
                    (*h).mb.i_mb_xy - 1 as libc::c_int - (*h).mb.i_mb_stride;
                left[0 as libc::c_int as usize] = left[1 as libc::c_int as usize];
                (*h).mb.left_b8[1 as libc::c_int as usize] =
                    (*h).mb.i_b8_xy - 2 as libc::c_int - 2 as libc::c_int * (*h).mb.i_b8_stride;
                (*h).mb.left_b8[0 as libc::c_int as usize] =
                    (*h).mb.left_b8[1 as libc::c_int as usize];
                (*h).mb.left_b4[1 as libc::c_int as usize] =
                    (*h).mb.i_b4_xy - 4 as libc::c_int - 4 as libc::c_int * (*h).mb.i_b4_stride;
                (*h).mb.left_b4[0 as libc::c_int as usize] =
                    (*h).mb.left_b4[1 as libc::c_int as usize];
                if mb_interlaced != 0 {
                    (*h).mb.left_index_table =
                        &*left_indices.as_ptr().offset(2 as libc::c_int as isize)
                            as *const x264_left_table_t;
                    left[1 as libc::c_int as usize] += (*h).mb.i_mb_stride;
                    (*h).mb.left_b8[1 as libc::c_int as usize] +=
                        2 as libc::c_int * (*h).mb.i_b8_stride;
                    (*h).mb.left_b4[1 as libc::c_int as usize] +=
                        4 as libc::c_int * (*h).mb.i_b4_stride;
                } else {
                    (*h).mb.left_index_table =
                        &*left_indices.as_ptr().offset(1 as libc::c_int as isize)
                            as *const x264_left_table_t;
                    topleft_y += 1;
                    topleft_y;
                    (*h).mb.topleft_partition = 1 as libc::c_int;
                }
            }
            if mb_interlaced == 0 {
                topright_y = -(1 as libc::c_int);
            }
        } else {
            if mb_interlaced != 0 && top >= 0 as libc::c_int {
                if *((*h).mb.field).offset(top as isize) == 0 {
                    top += (*h).mb.i_mb_stride;
                    top_y += 1;
                    top_y;
                }
                if mb_x != 0 {
                    topleft_y += (*((*h).mb.field).offset(
                        ((*h).mb.i_mb_stride * topleft_y + mb_x - 1 as libc::c_int) as isize,
                    ) == 0) as libc::c_int;
                }
                if mb_x < (*h).mb.i_mb_width - 1 as libc::c_int {
                    topright_y += (*((*h).mb.field).offset(
                        ((*h).mb.i_mb_stride * topright_y + mb_x + 1 as libc::c_int) as isize,
                    ) == 0) as libc::c_int;
                }
            }
            if mb_x != 0
                && mb_interlaced
                    != *((*h).mb.field).offset(((*h).mb.i_mb_xy - 1 as libc::c_int) as isize)
                        as libc::c_int
            {
                if mb_interlaced != 0 {
                    (*h).mb.left_index_table =
                        &*left_indices.as_ptr().offset(2 as libc::c_int as isize)
                            as *const x264_left_table_t;
                    left[1 as libc::c_int as usize] += (*h).mb.i_mb_stride;
                    (*h).mb.left_b8[1 as libc::c_int as usize] +=
                        2 as libc::c_int * (*h).mb.i_b8_stride;
                    (*h).mb.left_b4[1 as libc::c_int as usize] +=
                        4 as libc::c_int * (*h).mb.i_b4_stride;
                } else {
                    (*h).mb.left_index_table =
                        &*left_indices.as_ptr().offset(0 as libc::c_int as isize)
                            as *const x264_left_table_t;
                }
            }
        }
    }
    if mb_x > 0 as libc::c_int {
        (*h).mb.i_neighbour_frame |= MB_LEFT as libc::c_int as libc::c_uint;
        (*h).mb.i_mb_left_xy[0 as libc::c_int as usize] = left[0 as libc::c_int as usize];
        (*h).mb.i_mb_left_xy[1 as libc::c_int as usize] = left[1 as libc::c_int as usize];
        (*h).mb.i_mb_type_left[0 as libc::c_int as usize] = *((*h).mb.type_0)
            .offset((*h).mb.i_mb_left_xy[0 as libc::c_int as usize] as isize)
            as libc::c_int;
        (*h).mb.i_mb_type_left[1 as libc::c_int as usize] = *((*h).mb.type_0)
            .offset((*h).mb.i_mb_left_xy[1 as libc::c_int as usize] as isize)
            as libc::c_int;
        if *((*h).mb.slice_table).offset(left[0 as libc::c_int as usize] as isize)
            == (*h).sh.i_first_mb
        {
            (*h).mb.i_neighbour |= MB_LEFT as libc::c_int as libc::c_uint;
            if (*h).param.b_constrained_intra == 0
                || ((*h).mb.i_mb_type_left[0 as libc::c_int as usize] == I_4x4 as libc::c_int
                    || (*h).mb.i_mb_type_left[0 as libc::c_int as usize] == I_8x8 as libc::c_int
                    || (*h).mb.i_mb_type_left[0 as libc::c_int as usize] == I_16x16 as libc::c_int
                    || (*h).mb.i_mb_type_left[0 as libc::c_int as usize] == I_PCM as libc::c_int)
            {
                (*h).mb.i_neighbour_intra |= MB_LEFT as libc::c_int as libc::c_uint;
            }
        }
    }
    if (*h).i_threadslice_start >> mb_interlaced != mb_y >> mb_interlaced {
        if top >= 0 as libc::c_int {
            (*h).mb.i_neighbour_frame |= MB_TOP as libc::c_int as libc::c_uint;
            (*h).mb.i_mb_top_xy = top;
            (*h).mb.i_mb_top_y = top_y;
            (*h).mb.i_mb_type_top =
                *((*h).mb.type_0).offset((*h).mb.i_mb_top_xy as isize) as libc::c_int;
            if *((*h).mb.slice_table).offset(top as isize) == (*h).sh.i_first_mb {
                (*h).mb.i_neighbour |= MB_TOP as libc::c_int as libc::c_uint;
                if (*h).param.b_constrained_intra == 0
                    || ((*h).mb.i_mb_type_top == I_4x4 as libc::c_int
                        || (*h).mb.i_mb_type_top == I_8x8 as libc::c_int
                        || (*h).mb.i_mb_type_top == I_16x16 as libc::c_int
                        || (*h).mb.i_mb_type_top == I_PCM as libc::c_int)
                {
                    (*h).mb.i_neighbour_intra |= MB_TOP as libc::c_int as libc::c_uint;
                }
                ((*h).mb.cbp).offset(top as isize);
                (*((*h).mb.non_zero_count).offset(top as isize))
                    .as_mut_ptr()
                    .offset(12 as libc::c_int as isize);
                ((*h).mb.mb_transform_size).offset(top as isize);
                if (*h).param.b_cabac != 0 {
                    ((*h).mb.skipbp).offset(top as isize);
                }
            }
        }
        if mb_x > 0 as libc::c_int && topleft_y >= 0 as libc::c_int {
            (*h).mb.i_neighbour_frame |= MB_TOPLEFT as libc::c_int as libc::c_uint;
            (*h).mb.i_mb_topleft_xy = (*h).mb.i_mb_stride * topleft_y + mb_x - 1 as libc::c_int;
            (*h).mb.i_mb_topleft_y = topleft_y;
            (*h).mb.i_mb_type_topleft =
                *((*h).mb.type_0).offset((*h).mb.i_mb_topleft_xy as isize) as libc::c_int;
            if *((*h).mb.slice_table).offset((*h).mb.i_mb_topleft_xy as isize) == (*h).sh.i_first_mb
            {
                (*h).mb.i_neighbour |= MB_TOPLEFT as libc::c_int as libc::c_uint;
                if (*h).param.b_constrained_intra == 0
                    || ((*h).mb.i_mb_type_topleft == I_4x4 as libc::c_int
                        || (*h).mb.i_mb_type_topleft == I_8x8 as libc::c_int
                        || (*h).mb.i_mb_type_topleft == I_16x16 as libc::c_int
                        || (*h).mb.i_mb_type_topleft == I_PCM as libc::c_int)
                {
                    (*h).mb.i_neighbour_intra |= MB_TOPLEFT as libc::c_int as libc::c_uint;
                }
            }
        }
        if mb_x < (*h).mb.i_mb_width - 1 as libc::c_int && topright_y >= 0 as libc::c_int {
            (*h).mb.i_neighbour_frame |= MB_TOPRIGHT as libc::c_int as libc::c_uint;
            (*h).mb.i_mb_topright_xy = (*h).mb.i_mb_stride * topright_y + mb_x + 1 as libc::c_int;
            (*h).mb.i_mb_topright_y = topright_y;
            (*h).mb.i_mb_type_topright =
                *((*h).mb.type_0).offset((*h).mb.i_mb_topright_xy as isize) as libc::c_int;
            if *((*h).mb.slice_table).offset((*h).mb.i_mb_topright_xy as isize)
                == (*h).sh.i_first_mb
            {
                (*h).mb.i_neighbour |= MB_TOPRIGHT as libc::c_int as libc::c_uint;
                if (*h).param.b_constrained_intra == 0
                    || ((*h).mb.i_mb_type_topright == I_4x4 as libc::c_int
                        || (*h).mb.i_mb_type_topright == I_8x8 as libc::c_int
                        || (*h).mb.i_mb_type_topright == I_16x16 as libc::c_int
                        || (*h).mb.i_mb_type_topright == I_PCM as libc::c_int)
                {
                    (*h).mb.i_neighbour_intra |= MB_TOPRIGHT as libc::c_int as libc::c_uint;
                }
            }
        }
    }
}
#[inline(always)]
unsafe extern "C" fn macroblock_cache_load(
    mut h: *mut x264_t,
    mut mb_x: libc::c_int,
    mut mb_y: libc::c_int,
    mut b_mbaff: libc::c_int,
) {
    macroblock_cache_load_neighbours(h, mb_x, mb_y, b_mbaff);
    let mut left: *mut libc::c_int = ((*h).mb.i_mb_left_xy).as_mut_ptr();
    let mut top: libc::c_int = (*h).mb.i_mb_top_xy;
    let mut top_y: libc::c_int = (*h).mb.i_mb_top_y;
    let mut s8x8: libc::c_int = (*h).mb.i_b8_stride;
    let mut s4x4: libc::c_int = (*h).mb.i_b4_stride;
    let mut top_8x8: libc::c_int =
        (2 as libc::c_int * top_y + 1 as libc::c_int) * s8x8 + 2 as libc::c_int * mb_x;
    let mut top_4x4: libc::c_int =
        (4 as libc::c_int * top_y + 3 as libc::c_int) * s4x4 + 4 as libc::c_int * mb_x;
    let mut lists: libc::c_int = ((1 as libc::c_int) << (*h).sh.i_type) & 3 as libc::c_int;
    let mut i4x4: *mut [int8_t; 8] = (*h).mb.intra4x4_pred_mode;
    let mut nnz: *mut [uint8_t; 48] = (*h).mb.non_zero_count;
    let mut cbp: *mut int16_t = (*h).mb.cbp;
    let mut left_index_table: *const x264_left_table_t = (*h).mb.left_index_table;
    (*h).mb.cache.deblock_strength = (*((*h).deblock_strength[(mb_y & 1 as libc::c_int) as usize])
        .offset(
            (if (*h).param.b_sliced_threads != 0 {
                (*h).mb.i_mb_xy
            } else {
                mb_x
            }) as isize,
        ))
    .as_mut_ptr();
    if (*h).mb.i_neighbour & MB_TOP as libc::c_int as libc::c_uint != 0 {
        (*h).mb.cache.i_cbp_top = *cbp.offset(top as isize) as libc::c_int;
        (*(&mut *((*h).mb.cache.intra4x4_pred_mode).as_mut_ptr().offset(
            (*x264_scan8.as_ptr().offset(0 as libc::c_int as isize) as libc::c_int
                - 8 as libc::c_int) as isize,
        ) as *mut int8_t as *mut x264_union32_t))
            .i = (*(&mut *(*i4x4.offset(top as isize))
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) as *mut int8_t
            as *mut x264_union32_t))
            .i;
        (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
            (*x264_scan8.as_ptr().offset(0 as libc::c_int as isize) as libc::c_int
                - 8 as libc::c_int) as isize,
        ) as *mut uint8_t as *mut x264_union32_t))
            .i = (*(&mut *(*nnz.offset(top as isize))
            .as_mut_ptr()
            .offset(12 as libc::c_int as isize) as *mut uint8_t
            as *mut x264_union32_t))
            .i;
        (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
            (*x264_scan8.as_ptr().offset(16 as libc::c_int as isize) as libc::c_int
                - 8 as libc::c_int) as isize,
        ) as *mut uint8_t as *mut x264_union32_t))
            .i = (*(&mut *(*nnz.offset(top as isize)).as_mut_ptr().offset(
            (16 as libc::c_int - 4 as libc::c_int + (16 as libc::c_int >> (*h).mb.chroma_v_shift))
                as isize,
        ) as *mut uint8_t as *mut x264_union32_t))
            .i;
        (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
            (*x264_scan8.as_ptr().offset(32 as libc::c_int as isize) as libc::c_int
                - 8 as libc::c_int) as isize,
        ) as *mut uint8_t as *mut x264_union32_t))
            .i = (*(&mut *(*nnz.offset(top as isize)).as_mut_ptr().offset(
            (32 as libc::c_int - 4 as libc::c_int + (16 as libc::c_int >> (*h).mb.chroma_v_shift))
                as isize,
        ) as *mut uint8_t as *mut x264_union32_t))
            .i;
        let mut l: libc::c_int = 0 as libc::c_int;
        while l < lists {
            (*((*h).mb.mv).as_mut_ptr().offset(l as isize))
                .offset((top_4x4 - 1 as libc::c_int) as isize);
            (*((*h).mb.mv).as_mut_ptr().offset(l as isize))
                .offset((top_4x4 + 4 as libc::c_int) as isize);
            (*((*h).mb.ref_0).as_mut_ptr().offset(l as isize))
                .offset((top_8x8 - 1 as libc::c_int) as isize);
            if (*h).param.b_cabac != 0 {
                (*((*h).mb.mvd).as_mut_ptr().offset(l as isize)).offset(top as isize);
            }
            l += 1;
            l;
        }
    } else {
        (*h).mb.cache.i_cbp_top = -(1 as libc::c_int);
        (*(&mut *((*h).mb.cache.intra4x4_pred_mode).as_mut_ptr().offset(
            (*x264_scan8.as_ptr().offset(0 as libc::c_int as isize) as libc::c_int
                - 8 as libc::c_int) as isize,
        ) as *mut int8_t as *mut x264_union32_t))
            .i = 0xffffffff as libc::c_uint;
        (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
            (*x264_scan8.as_ptr().offset(0 as libc::c_int as isize) as libc::c_int
                - 8 as libc::c_int) as isize,
        ) as *mut uint8_t as *mut x264_union32_t))
            .i = 0x80808080 as libc::c_uint;
        (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
            (*x264_scan8.as_ptr().offset(16 as libc::c_int as isize) as libc::c_int
                - 8 as libc::c_int) as isize,
        ) as *mut uint8_t as *mut x264_union32_t))
            .i = 0x80808080 as libc::c_uint;
        (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
            (*x264_scan8.as_ptr().offset(32 as libc::c_int as isize) as libc::c_int
                - 8 as libc::c_int) as isize,
        ) as *mut uint8_t as *mut x264_union32_t))
            .i = 0x80808080 as libc::c_uint;
    }
    if (*h).mb.i_neighbour & MB_LEFT as libc::c_int as libc::c_uint != 0 {
        let mut ltop: libc::c_int = *left.offset(0 as libc::c_int as isize);
        let mut lbot: libc::c_int = if b_mbaff != 0 {
            *left.offset(1 as libc::c_int as isize)
        } else {
            ltop
        };
        if b_mbaff != 0 {
            let top_luma: int16_t = ((*cbp.offset(ltop as isize) as libc::c_int
                >> ((*left_index_table).mv[0 as libc::c_int as usize] as libc::c_int
                    & !(1 as libc::c_int)))
                & 2 as libc::c_int) as int16_t;
            let bot_luma: int16_t = ((*cbp.offset(lbot as isize) as libc::c_int
                >> ((*left_index_table).mv[2 as libc::c_int as usize] as libc::c_int
                    & !(1 as libc::c_int)))
                & 2 as libc::c_int) as int16_t;
            (*h).mb.cache.i_cbp_left = *cbp.offset(ltop as isize) as libc::c_int
                & 0xfff0 as libc::c_int
                | ((bot_luma as libc::c_int) << 2 as libc::c_int)
                | top_luma as libc::c_int;
        } else {
            (*h).mb.cache.i_cbp_left = *cbp.offset(ltop as isize) as libc::c_int;
        }
        (*h).mb.cache.intra4x4_pred_mode
            [(x264_scan8[0 as libc::c_int as usize] as libc::c_int - 1 as libc::c_int) as usize] =
            (*i4x4.offset(ltop as isize))
                [(*left_index_table).intra[0 as libc::c_int as usize] as usize];
        (*h).mb.cache.intra4x4_pred_mode
            [(x264_scan8[2 as libc::c_int as usize] as libc::c_int - 1 as libc::c_int) as usize] =
            (*i4x4.offset(ltop as isize))
                [(*left_index_table).intra[1 as libc::c_int as usize] as usize];
        (*h).mb.cache.intra4x4_pred_mode
            [(x264_scan8[8 as libc::c_int as usize] as libc::c_int - 1 as libc::c_int) as usize] =
            (*i4x4.offset(lbot as isize))
                [(*left_index_table).intra[2 as libc::c_int as usize] as usize];
        (*h).mb.cache.intra4x4_pred_mode
            [(x264_scan8[10 as libc::c_int as usize] as libc::c_int - 1 as libc::c_int) as usize] =
            (*i4x4.offset(lbot as isize))
                [(*left_index_table).intra[3 as libc::c_int as usize] as usize];
        (*h).mb.cache.non_zero_count
            [(x264_scan8[0 as libc::c_int as usize] as libc::c_int - 1 as libc::c_int) as usize] =
            (*nnz.offset(ltop as isize))
                [(*left_index_table).nnz[0 as libc::c_int as usize] as usize];
        (*h).mb.cache.non_zero_count
            [(x264_scan8[2 as libc::c_int as usize] as libc::c_int - 1 as libc::c_int) as usize] =
            (*nnz.offset(ltop as isize))
                [(*left_index_table).nnz[1 as libc::c_int as usize] as usize];
        (*h).mb.cache.non_zero_count
            [(x264_scan8[8 as libc::c_int as usize] as libc::c_int - 1 as libc::c_int) as usize] =
            (*nnz.offset(lbot as isize))
                [(*left_index_table).nnz[2 as libc::c_int as usize] as usize];
        (*h).mb.cache.non_zero_count
            [(x264_scan8[10 as libc::c_int as usize] as libc::c_int - 1 as libc::c_int) as usize] =
            (*nnz.offset(lbot as isize))
                [(*left_index_table).nnz[3 as libc::c_int as usize] as usize];
        if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc >= CHROMA_422 as libc::c_int {
            let mut offset: libc::c_int =
                (4 as libc::c_int >> (*h).mb.chroma_h_shift) - 4 as libc::c_int;
            (*h).mb.cache.non_zero_count[(x264_scan8
                [(16 as libc::c_int + 0 as libc::c_int) as usize]
                as libc::c_int
                - 1 as libc::c_int) as usize] =
                (*nnz.offset(ltop as isize))[((*left_index_table).nnz[0 as libc::c_int as usize]
                    as libc::c_int
                    + 16 as libc::c_int
                    + offset) as usize];
            (*h).mb.cache.non_zero_count[(x264_scan8
                [(16 as libc::c_int + 2 as libc::c_int) as usize]
                as libc::c_int
                - 1 as libc::c_int) as usize] =
                (*nnz.offset(ltop as isize))[((*left_index_table).nnz[1 as libc::c_int as usize]
                    as libc::c_int
                    + 16 as libc::c_int
                    + offset) as usize];
            (*h).mb.cache.non_zero_count[(x264_scan8
                [(16 as libc::c_int + 8 as libc::c_int) as usize]
                as libc::c_int
                - 1 as libc::c_int) as usize] =
                (*nnz.offset(lbot as isize))[((*left_index_table).nnz[2 as libc::c_int as usize]
                    as libc::c_int
                    + 16 as libc::c_int
                    + offset) as usize];
            (*h).mb.cache.non_zero_count[(x264_scan8
                [(16 as libc::c_int + 10 as libc::c_int) as usize]
                as libc::c_int
                - 1 as libc::c_int) as usize] =
                (*nnz.offset(lbot as isize))[((*left_index_table).nnz[3 as libc::c_int as usize]
                    as libc::c_int
                    + 16 as libc::c_int
                    + offset) as usize];
            (*h).mb.cache.non_zero_count[(x264_scan8
                [(32 as libc::c_int + 0 as libc::c_int) as usize]
                as libc::c_int
                - 1 as libc::c_int) as usize] =
                (*nnz.offset(ltop as isize))[((*left_index_table).nnz[0 as libc::c_int as usize]
                    as libc::c_int
                    + 32 as libc::c_int
                    + offset) as usize];
            (*h).mb.cache.non_zero_count[(x264_scan8
                [(32 as libc::c_int + 2 as libc::c_int) as usize]
                as libc::c_int
                - 1 as libc::c_int) as usize] =
                (*nnz.offset(ltop as isize))[((*left_index_table).nnz[1 as libc::c_int as usize]
                    as libc::c_int
                    + 32 as libc::c_int
                    + offset) as usize];
            (*h).mb.cache.non_zero_count[(x264_scan8
                [(32 as libc::c_int + 8 as libc::c_int) as usize]
                as libc::c_int
                - 1 as libc::c_int) as usize] =
                (*nnz.offset(lbot as isize))[((*left_index_table).nnz[2 as libc::c_int as usize]
                    as libc::c_int
                    + 32 as libc::c_int
                    + offset) as usize];
            (*h).mb.cache.non_zero_count[(x264_scan8
                [(32 as libc::c_int + 10 as libc::c_int) as usize]
                as libc::c_int
                - 1 as libc::c_int) as usize] =
                (*nnz.offset(lbot as isize))[((*left_index_table).nnz[3 as libc::c_int as usize]
                    as libc::c_int
                    + 32 as libc::c_int
                    + offset) as usize];
        } else {
            (*h).mb.cache.non_zero_count[(x264_scan8
                [(16 as libc::c_int + 0 as libc::c_int) as usize]
                as libc::c_int
                - 1 as libc::c_int) as usize] = (*nnz.offset(ltop as isize))
                [(*left_index_table).nnz_chroma[0 as libc::c_int as usize] as usize];
            (*h).mb.cache.non_zero_count[(x264_scan8
                [(16 as libc::c_int + 2 as libc::c_int) as usize]
                as libc::c_int
                - 1 as libc::c_int) as usize] = (*nnz.offset(lbot as isize))
                [(*left_index_table).nnz_chroma[1 as libc::c_int as usize] as usize];
            (*h).mb.cache.non_zero_count[(x264_scan8
                [(32 as libc::c_int + 0 as libc::c_int) as usize]
                as libc::c_int
                - 1 as libc::c_int) as usize] = (*nnz.offset(ltop as isize))
                [(*left_index_table).nnz_chroma[2 as libc::c_int as usize] as usize];
            (*h).mb.cache.non_zero_count[(x264_scan8
                [(32 as libc::c_int + 2 as libc::c_int) as usize]
                as libc::c_int
                - 1 as libc::c_int) as usize] = (*nnz.offset(lbot as isize))
                [(*left_index_table).nnz_chroma[3 as libc::c_int as usize] as usize];
        }
    } else {
        (*h).mb.cache.i_cbp_left = -(1 as libc::c_int);
        (*h).mb.cache.intra4x4_pred_mode
            [(x264_scan8[10 as libc::c_int as usize] as libc::c_int - 1 as libc::c_int) as usize] =
            -(1 as libc::c_int) as int8_t;
        (*h).mb.cache.intra4x4_pred_mode
            [(x264_scan8[8 as libc::c_int as usize] as libc::c_int - 1 as libc::c_int) as usize] =
            (*h).mb.cache.intra4x4_pred_mode[(x264_scan8[10 as libc::c_int as usize] as libc::c_int
                - 1 as libc::c_int) as usize];
        (*h).mb.cache.intra4x4_pred_mode
            [(x264_scan8[2 as libc::c_int as usize] as libc::c_int - 1 as libc::c_int) as usize] =
            (*h).mb.cache.intra4x4_pred_mode[(x264_scan8[8 as libc::c_int as usize] as libc::c_int
                - 1 as libc::c_int) as usize];
        (*h).mb.cache.intra4x4_pred_mode
            [(x264_scan8[0 as libc::c_int as usize] as libc::c_int - 1 as libc::c_int) as usize] =
            (*h).mb.cache.intra4x4_pred_mode[(x264_scan8[2 as libc::c_int as usize] as libc::c_int
                - 1 as libc::c_int) as usize];
        (*h).mb.cache.non_zero_count[(x264_scan8[(32 as libc::c_int + 2 as libc::c_int) as usize]
            as libc::c_int
            - 1 as libc::c_int) as usize] = 0x80 as libc::c_int as uint8_t;
        (*h).mb.cache.non_zero_count[(x264_scan8[(32 as libc::c_int + 0 as libc::c_int) as usize]
            as libc::c_int
            - 1 as libc::c_int) as usize] = (*h).mb.cache.non_zero_count[(x264_scan8
            [(32 as libc::c_int + 2 as libc::c_int) as usize]
            as libc::c_int
            - 1 as libc::c_int)
            as usize];
        (*h).mb.cache.non_zero_count[(x264_scan8[(16 as libc::c_int + 2 as libc::c_int) as usize]
            as libc::c_int
            - 1 as libc::c_int) as usize] = (*h).mb.cache.non_zero_count[(x264_scan8
            [(32 as libc::c_int + 0 as libc::c_int) as usize]
            as libc::c_int
            - 1 as libc::c_int)
            as usize];
        (*h).mb.cache.non_zero_count[(x264_scan8[(16 as libc::c_int + 0 as libc::c_int) as usize]
            as libc::c_int
            - 1 as libc::c_int) as usize] = (*h).mb.cache.non_zero_count[(x264_scan8
            [(16 as libc::c_int + 2 as libc::c_int) as usize]
            as libc::c_int
            - 1 as libc::c_int)
            as usize];
        (*h).mb.cache.non_zero_count
            [(x264_scan8[10 as libc::c_int as usize] as libc::c_int - 1 as libc::c_int) as usize] =
            (*h).mb.cache.non_zero_count[(x264_scan8
                [(16 as libc::c_int + 0 as libc::c_int) as usize]
                as libc::c_int
                - 1 as libc::c_int) as usize];
        (*h).mb.cache.non_zero_count
            [(x264_scan8[8 as libc::c_int as usize] as libc::c_int - 1 as libc::c_int) as usize] =
            (*h).mb.cache.non_zero_count[(x264_scan8[10 as libc::c_int as usize] as libc::c_int
                - 1 as libc::c_int) as usize];
        (*h).mb.cache.non_zero_count
            [(x264_scan8[2 as libc::c_int as usize] as libc::c_int - 1 as libc::c_int) as usize] =
            (*h).mb.cache.non_zero_count[(x264_scan8[8 as libc::c_int as usize] as libc::c_int
                - 1 as libc::c_int) as usize];
        (*h).mb.cache.non_zero_count
            [(x264_scan8[0 as libc::c_int as usize] as libc::c_int - 1 as libc::c_int) as usize] =
            (*h).mb.cache.non_zero_count[(x264_scan8[2 as libc::c_int as usize] as libc::c_int
                - 1 as libc::c_int) as usize];
        if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc >= CHROMA_422 as libc::c_int {
            (*h).mb.cache.non_zero_count[(x264_scan8
                [(32 as libc::c_int + 10 as libc::c_int) as usize]
                as libc::c_int
                - 1 as libc::c_int) as usize] = 0x80 as libc::c_int as uint8_t;
            (*h).mb.cache.non_zero_count[(x264_scan8
                [(32 as libc::c_int + 8 as libc::c_int) as usize]
                as libc::c_int
                - 1 as libc::c_int) as usize] = (*h).mb.cache.non_zero_count[(x264_scan8
                [(32 as libc::c_int + 10 as libc::c_int) as usize]
                as libc::c_int
                - 1 as libc::c_int)
                as usize];
            (*h).mb.cache.non_zero_count[(x264_scan8
                [(16 as libc::c_int + 10 as libc::c_int) as usize]
                as libc::c_int
                - 1 as libc::c_int) as usize] = (*h).mb.cache.non_zero_count[(x264_scan8
                [(32 as libc::c_int + 8 as libc::c_int) as usize]
                as libc::c_int
                - 1 as libc::c_int)
                as usize];
            (*h).mb.cache.non_zero_count[(x264_scan8
                [(16 as libc::c_int + 8 as libc::c_int) as usize]
                as libc::c_int
                - 1 as libc::c_int) as usize] = (*h).mb.cache.non_zero_count[(x264_scan8
                [(16 as libc::c_int + 10 as libc::c_int) as usize]
                as libc::c_int
                - 1 as libc::c_int)
                as usize];
        }
    }
    if (*((*h).pps).as_mut_ptr()).b_transform_8x8_mode != 0 {
        (*h).mb.cache.i_neighbour_transform_size = ((*h).mb.i_neighbour
            & MB_LEFT as libc::c_int as libc::c_uint
            != 0
            && *((*h).mb.mb_transform_size).offset(*left.offset(0 as libc::c_int as isize) as isize)
                as libc::c_int
                != 0) as libc::c_int
            + ((*h).mb.i_neighbour & MB_TOP as libc::c_int as libc::c_uint != 0
                && *((*h).mb.mb_transform_size).offset(top as isize) as libc::c_int != 0)
                as libc::c_int;
    }
    if b_mbaff != 0 {
        (*h).mb.pic.i_fref[0 as libc::c_int as usize] =
            (*h).i_ref[0 as libc::c_int as usize] << (*h).mb.b_interlaced;
        (*h).mb.pic.i_fref[1 as libc::c_int as usize] =
            (*h).i_ref[1 as libc::c_int as usize] << (*h).mb.b_interlaced;
    }
    if b_mbaff == 0 {
        x264_8_copy_column8(
            ((*h).mb.pic.p_fdec[0 as libc::c_int as usize])
                .offset(-(1 as libc::c_int as isize))
                .offset((4 as libc::c_int * 32 as libc::c_int) as isize),
            ((*h).mb.pic.p_fdec[0 as libc::c_int as usize])
                .offset(15 as libc::c_int as isize)
                .offset((4 as libc::c_int * 32 as libc::c_int) as isize),
        );
        x264_8_copy_column8(
            ((*h).mb.pic.p_fdec[0 as libc::c_int as usize])
                .offset(-(1 as libc::c_int as isize))
                .offset((12 as libc::c_int * 32 as libc::c_int) as isize),
            ((*h).mb.pic.p_fdec[0 as libc::c_int as usize])
                .offset(15 as libc::c_int as isize)
                .offset((12 as libc::c_int * 32 as libc::c_int) as isize),
        );
        macroblock_load_pic_pointers(
            h,
            mb_x,
            mb_y,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
        );
        if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as libc::c_int {
            x264_8_copy_column8(
                ((*h).mb.pic.p_fdec[1 as libc::c_int as usize])
                    .offset(-(1 as libc::c_int as isize))
                    .offset((4 as libc::c_int * 32 as libc::c_int) as isize),
                ((*h).mb.pic.p_fdec[1 as libc::c_int as usize])
                    .offset(15 as libc::c_int as isize)
                    .offset((4 as libc::c_int * 32 as libc::c_int) as isize),
            );
            x264_8_copy_column8(
                ((*h).mb.pic.p_fdec[1 as libc::c_int as usize])
                    .offset(-(1 as libc::c_int as isize))
                    .offset((12 as libc::c_int * 32 as libc::c_int) as isize),
                ((*h).mb.pic.p_fdec[1 as libc::c_int as usize])
                    .offset(15 as libc::c_int as isize)
                    .offset((12 as libc::c_int * 32 as libc::c_int) as isize),
            );
            x264_8_copy_column8(
                ((*h).mb.pic.p_fdec[2 as libc::c_int as usize])
                    .offset(-(1 as libc::c_int as isize))
                    .offset((4 as libc::c_int * 32 as libc::c_int) as isize),
                ((*h).mb.pic.p_fdec[2 as libc::c_int as usize])
                    .offset(15 as libc::c_int as isize)
                    .offset((4 as libc::c_int * 32 as libc::c_int) as isize),
            );
            x264_8_copy_column8(
                ((*h).mb.pic.p_fdec[2 as libc::c_int as usize])
                    .offset(-(1 as libc::c_int as isize))
                    .offset((12 as libc::c_int * 32 as libc::c_int) as isize),
                ((*h).mb.pic.p_fdec[2 as libc::c_int as usize])
                    .offset(15 as libc::c_int as isize)
                    .offset((12 as libc::c_int * 32 as libc::c_int) as isize),
            );
            macroblock_load_pic_pointers(
                h,
                mb_x,
                mb_y,
                1 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
            );
            macroblock_load_pic_pointers(
                h,
                mb_x,
                mb_y,
                2 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
            );
        } else if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc != 0 {
            x264_8_copy_column8(
                ((*h).mb.pic.p_fdec[1 as libc::c_int as usize])
                    .offset(-(1 as libc::c_int as isize))
                    .offset((4 as libc::c_int * 32 as libc::c_int) as isize),
                ((*h).mb.pic.p_fdec[1 as libc::c_int as usize])
                    .offset(7 as libc::c_int as isize)
                    .offset((4 as libc::c_int * 32 as libc::c_int) as isize),
            );
            x264_8_copy_column8(
                ((*h).mb.pic.p_fdec[2 as libc::c_int as usize])
                    .offset(-(1 as libc::c_int as isize))
                    .offset((4 as libc::c_int * 32 as libc::c_int) as isize),
                ((*h).mb.pic.p_fdec[2 as libc::c_int as usize])
                    .offset(7 as libc::c_int as isize)
                    .offset((4 as libc::c_int * 32 as libc::c_int) as isize),
            );
            if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_422 as libc::c_int {
                x264_8_copy_column8(
                    ((*h).mb.pic.p_fdec[1 as libc::c_int as usize])
                        .offset(-(1 as libc::c_int as isize))
                        .offset((12 as libc::c_int * 32 as libc::c_int) as isize),
                    ((*h).mb.pic.p_fdec[1 as libc::c_int as usize])
                        .offset(7 as libc::c_int as isize)
                        .offset((12 as libc::c_int * 32 as libc::c_int) as isize),
                );
                x264_8_copy_column8(
                    ((*h).mb.pic.p_fdec[2 as libc::c_int as usize])
                        .offset(-(1 as libc::c_int as isize))
                        .offset((12 as libc::c_int * 32 as libc::c_int) as isize),
                    ((*h).mb.pic.p_fdec[2 as libc::c_int as usize])
                        .offset(7 as libc::c_int as isize)
                        .offset((12 as libc::c_int * 32 as libc::c_int) as isize),
                );
            }
            macroblock_load_pic_pointers(
                h,
                mb_x,
                mb_y,
                1 as libc::c_int,
                1 as libc::c_int,
                0 as libc::c_int,
            );
        }
    } else {
        macroblock_load_pic_pointers(
            h,
            mb_x,
            mb_y,
            0 as libc::c_int,
            0 as libc::c_int,
            1 as libc::c_int,
        );
        if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as libc::c_int {
            macroblock_load_pic_pointers(
                h,
                mb_x,
                mb_y,
                1 as libc::c_int,
                0 as libc::c_int,
                1 as libc::c_int,
            );
            macroblock_load_pic_pointers(
                h,
                mb_x,
                mb_y,
                2 as libc::c_int,
                0 as libc::c_int,
                1 as libc::c_int,
            );
        } else if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc != 0 {
            macroblock_load_pic_pointers(
                h,
                mb_x,
                mb_y,
                1 as libc::c_int,
                1 as libc::c_int,
                1 as libc::c_int,
            );
        }
    }
    if !((*(*h).fdec).integral).is_null() {
        let mut offset_0: libc::c_int =
            16 as libc::c_int * (mb_x + mb_y * (*(*h).fdec).i_stride[0 as libc::c_int as usize]);
        let mut list: libc::c_int = 0 as libc::c_int;
        while list < 2 as libc::c_int {
            let mut i: libc::c_int = 0 as libc::c_int;
            while i < (*h).mb.pic.i_fref[list as usize] {
                (*h).mb.pic.p_integral[list as usize][i as usize] =
                    &mut *((**(*((*h).fref).as_mut_ptr().offset(list as isize))
                        .as_mut_ptr()
                        .offset(i as isize))
                    .integral)
                        .offset(offset_0 as isize) as *mut uint16_t;
                i += 1;
                i;
            }
            list += 1;
            list;
        }
    }
    x264_8_prefetch_fenc(h, (*h).fenc, mb_x, mb_y);
    let mut l_0: libc::c_int = 0 as libc::c_int;
    while l_0 < lists {
        let mut mv: *mut [int16_t; 2] = (*h).mb.mv[l_0 as usize];
        let mut ref_0: *mut int8_t = (*h).mb.ref_0[l_0 as usize];
        let mut i8: libc::c_int = x264_scan8[0 as libc::c_int as usize] as libc::c_int
            - 1 as libc::c_int
            - 1 as libc::c_int * 8 as libc::c_int;
        if (*h).mb.i_neighbour & MB_TOPLEFT as libc::c_int as libc::c_uint != 0 {
            let mut ir: libc::c_int = if b_mbaff != 0 {
                2 as libc::c_int * (s8x8 * (*h).mb.i_mb_topleft_y + mb_x - 1 as libc::c_int)
                    + 1 as libc::c_int
                    + s8x8
            } else {
                top_8x8 - 1 as libc::c_int
            };
            let mut iv: libc::c_int = if b_mbaff != 0 {
                4 as libc::c_int * (s4x4 * (*h).mb.i_mb_topleft_y + mb_x - 1 as libc::c_int)
                    + 3 as libc::c_int
                    + 3 as libc::c_int * s4x4
            } else {
                top_4x4 - 1 as libc::c_int
            };
            if b_mbaff != 0 && (*h).mb.topleft_partition != 0 {
                iv -= 2 as libc::c_int * s4x4;
                ir -= s8x8;
            }
            (*h).mb.cache.ref_0[l_0 as usize][i8 as usize] = *ref_0.offset(ir as isize);
            (*(((*h).mb.cache.mv[l_0 as usize][i8 as usize]).as_mut_ptr()
                as *mut x264_union32_t))
                .i = (*((*mv.offset(iv as isize)).as_mut_ptr() as *mut x264_union32_t)).i;
        } else {
            (*h).mb.cache.ref_0[l_0 as usize][i8 as usize] = -(2 as libc::c_int) as int8_t;
            (*(((*h).mb.cache.mv[l_0 as usize][i8 as usize]).as_mut_ptr()
                as *mut x264_union32_t))
                .i = 0 as libc::c_int as uint32_t;
        }
        i8 = x264_scan8[0 as libc::c_int as usize] as libc::c_int - 8 as libc::c_int;
        if (*h).mb.i_neighbour & MB_TOP as libc::c_int as libc::c_uint != 0 {
            (*h).mb.cache.ref_0[l_0 as usize][(i8 + 1 as libc::c_int) as usize] =
                *ref_0.offset((top_8x8 + 0 as libc::c_int) as isize);
            (*h).mb.cache.ref_0[l_0 as usize][(i8 + 0 as libc::c_int) as usize] =
                (*h).mb.cache.ref_0[l_0 as usize][(i8 + 1 as libc::c_int) as usize];
            (*h).mb.cache.ref_0[l_0 as usize][(i8 + 3 as libc::c_int) as usize] =
                *ref_0.offset((top_8x8 + 1 as libc::c_int) as isize);
            (*h).mb.cache.ref_0[l_0 as usize][(i8 + 2 as libc::c_int) as usize] =
                (*h).mb.cache.ref_0[l_0 as usize][(i8 + 3 as libc::c_int) as usize];
            (*(((*h).mb.cache.mv[l_0 as usize][i8 as usize]).as_mut_ptr()
                as *mut x264_union128_sse_t))
                .i = (*((*mv.offset(top_4x4 as isize)).as_mut_ptr() as *mut x264_union128_sse_t)).i;
        } else {
            (*(((*h).mb.cache.mv[l_0 as usize][i8 as usize]).as_mut_ptr()
                as *mut x264_union128_sse_t))
                .i = _mm_setr_ps(
                0 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
            (*(&mut *(*((*h).mb.cache.ref_0).as_mut_ptr().offset(l_0 as isize))
                .as_mut_ptr()
                .offset(i8 as isize) as *mut int8_t as *mut x264_union32_t))
                .i = (-(2 as libc::c_int) as uint8_t as libc::c_uint)
                .wrapping_mul(0x1010101 as libc::c_uint);
        }
        i8 = x264_scan8[0 as libc::c_int as usize] as libc::c_int + 4 as libc::c_int
            - 1 as libc::c_int * 8 as libc::c_int;
        if (*h).mb.i_neighbour & MB_TOPRIGHT as libc::c_int as libc::c_uint != 0 {
            let mut ir_0: libc::c_int = if b_mbaff != 0 {
                2 as libc::c_int * (s8x8 * (*h).mb.i_mb_topright_y + (mb_x + 1 as libc::c_int))
                    + s8x8
            } else {
                top_8x8 + 2 as libc::c_int
            };
            let mut iv_0: libc::c_int = if b_mbaff != 0 {
                4 as libc::c_int * (s4x4 * (*h).mb.i_mb_topright_y + (mb_x + 1 as libc::c_int))
                    + 3 as libc::c_int * s4x4
            } else {
                top_4x4 + 4 as libc::c_int
            };
            (*h).mb.cache.ref_0[l_0 as usize][i8 as usize] = *ref_0.offset(ir_0 as isize);
            (*(((*h).mb.cache.mv[l_0 as usize][i8 as usize]).as_mut_ptr()
                as *mut x264_union32_t))
                .i = (*((*mv.offset(iv_0 as isize)).as_mut_ptr() as *mut x264_union32_t)).i;
        } else {
            (*h).mb.cache.ref_0[l_0 as usize][i8 as usize] = -(2 as libc::c_int) as int8_t;
        }
        i8 = x264_scan8[0 as libc::c_int as usize] as libc::c_int - 1 as libc::c_int;
        if (*h).mb.i_neighbour & MB_LEFT as libc::c_int as libc::c_uint != 0 {
            if b_mbaff != 0 {
                (*h).mb.cache.ref_0[l_0 as usize]
                    [(i8 + 0 as libc::c_int * 8 as libc::c_int) as usize] = *ref_0.offset(
                    ((*h).mb.left_b8[0 as libc::c_int as usize]
                        + 1 as libc::c_int
                        + s8x8
                            * (*left_index_table).ref_0[0 as libc::c_int as usize] as libc::c_int)
                        as isize,
                );
                (*h).mb.cache.ref_0[l_0 as usize]
                    [(i8 + 1 as libc::c_int * 8 as libc::c_int) as usize] = *ref_0.offset(
                    ((*h).mb.left_b8[0 as libc::c_int as usize]
                        + 1 as libc::c_int
                        + s8x8
                            * (*left_index_table).ref_0[1 as libc::c_int as usize] as libc::c_int)
                        as isize,
                );
                (*h).mb.cache.ref_0[l_0 as usize]
                    [(i8 + 2 as libc::c_int * 8 as libc::c_int) as usize] = *ref_0.offset(
                    ((*h).mb.left_b8[1 as libc::c_int as usize]
                        + 1 as libc::c_int
                        + s8x8
                            * (*left_index_table).ref_0[2 as libc::c_int as usize] as libc::c_int)
                        as isize,
                );
                (*h).mb.cache.ref_0[l_0 as usize]
                    [(i8 + 3 as libc::c_int * 8 as libc::c_int) as usize] = *ref_0.offset(
                    ((*h).mb.left_b8[1 as libc::c_int as usize]
                        + 1 as libc::c_int
                        + s8x8
                            * (*left_index_table).ref_0[3 as libc::c_int as usize] as libc::c_int)
                        as isize,
                );
                (*(((*h).mb.cache.mv[l_0 as usize]
                    [(i8 + 0 as libc::c_int * 8 as libc::c_int) as usize])
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i = (*((*mv.offset(
                    ((*h).mb.left_b4[0 as libc::c_int as usize]
                        + 3 as libc::c_int
                        + s4x4 * (*left_index_table).mv[0 as libc::c_int as usize] as libc::c_int)
                        as isize,
                ))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i;
                (*(((*h).mb.cache.mv[l_0 as usize]
                    [(i8 + 1 as libc::c_int * 8 as libc::c_int) as usize])
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i = (*((*mv.offset(
                    ((*h).mb.left_b4[0 as libc::c_int as usize]
                        + 3 as libc::c_int
                        + s4x4 * (*left_index_table).mv[1 as libc::c_int as usize] as libc::c_int)
                        as isize,
                ))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i;
                (*(((*h).mb.cache.mv[l_0 as usize]
                    [(i8 + 2 as libc::c_int * 8 as libc::c_int) as usize])
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i = (*((*mv.offset(
                    ((*h).mb.left_b4[1 as libc::c_int as usize]
                        + 3 as libc::c_int
                        + s4x4 * (*left_index_table).mv[2 as libc::c_int as usize] as libc::c_int)
                        as isize,
                ))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i;
                (*(((*h).mb.cache.mv[l_0 as usize]
                    [(i8 + 3 as libc::c_int * 8 as libc::c_int) as usize])
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i = (*((*mv.offset(
                    ((*h).mb.left_b4[1 as libc::c_int as usize]
                        + 3 as libc::c_int
                        + s4x4 * (*left_index_table).mv[3 as libc::c_int as usize] as libc::c_int)
                        as isize,
                ))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i;
            } else {
                let ir_1: libc::c_int = (*h).mb.i_b8_xy - 1 as libc::c_int;
                let iv_1: libc::c_int = (*h).mb.i_b4_xy - 1 as libc::c_int;
                (*h).mb.cache.ref_0[l_0 as usize]
                    [(i8 + 1 as libc::c_int * 8 as libc::c_int) as usize] =
                    *ref_0.offset((ir_1 + 0 as libc::c_int * s8x8) as isize);
                (*h).mb.cache.ref_0[l_0 as usize]
                    [(i8 + 0 as libc::c_int * 8 as libc::c_int) as usize] = (*h).mb.cache.ref_0
                    [l_0 as usize][(i8 + 1 as libc::c_int * 8 as libc::c_int) as usize];
                (*h).mb.cache.ref_0[l_0 as usize]
                    [(i8 + 3 as libc::c_int * 8 as libc::c_int) as usize] =
                    *ref_0.offset((ir_1 + 1 as libc::c_int * s8x8) as isize);
                (*h).mb.cache.ref_0[l_0 as usize]
                    [(i8 + 2 as libc::c_int * 8 as libc::c_int) as usize] = (*h).mb.cache.ref_0
                    [l_0 as usize][(i8 + 3 as libc::c_int * 8 as libc::c_int) as usize];
                (*(((*h).mb.cache.mv[l_0 as usize]
                    [(i8 + 0 as libc::c_int * 8 as libc::c_int) as usize])
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i = (*((*mv.offset((iv_1 + 0 as libc::c_int * s4x4) as isize)).as_mut_ptr()
                    as *mut x264_union32_t))
                    .i;
                (*(((*h).mb.cache.mv[l_0 as usize]
                    [(i8 + 1 as libc::c_int * 8 as libc::c_int) as usize])
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i = (*((*mv.offset((iv_1 + 1 as libc::c_int * s4x4) as isize)).as_mut_ptr()
                    as *mut x264_union32_t))
                    .i;
                (*(((*h).mb.cache.mv[l_0 as usize]
                    [(i8 + 2 as libc::c_int * 8 as libc::c_int) as usize])
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i = (*((*mv.offset((iv_1 + 2 as libc::c_int * s4x4) as isize)).as_mut_ptr()
                    as *mut x264_union32_t))
                    .i;
                (*(((*h).mb.cache.mv[l_0 as usize]
                    [(i8 + 3 as libc::c_int * 8 as libc::c_int) as usize])
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i = (*((*mv.offset((iv_1 + 3 as libc::c_int * s4x4) as isize)).as_mut_ptr()
                    as *mut x264_union32_t))
                    .i;
            }
        } else {
            let mut i_0: libc::c_int = 0 as libc::c_int;
            while i_0 < 4 as libc::c_int {
                (*h).mb.cache.ref_0[l_0 as usize][(i8 + i_0 * 8 as libc::c_int) as usize] =
                    -(2 as libc::c_int) as int8_t;
                (*(((*h).mb.cache.mv[l_0 as usize][(i8 + i_0 * 8 as libc::c_int) as usize])
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i = 0 as libc::c_int as uint32_t;
                i_0 += 1;
                i_0;
            }
        }
        if b_mbaff != 0 && (*h).mb.i_neighbour & MB_LEFT as libc::c_int as libc::c_uint != 0 {
            if (*h).mb.b_interlaced != 0
                && *((*h).mb.field).offset(((*h).mb.i_mb_xy - 1 as libc::c_int) as isize) == 0
            {
                (*h).mb.cache.topright_ref[l_0 as usize][0 as libc::c_int as usize] = *ref_0
                    .offset(
                        ((*h).mb.left_b8[0 as libc::c_int as usize]
                            + 1 as libc::c_int
                            + s8x8 * 0 as libc::c_int) as isize,
                    );
                (*h).mb.cache.topright_ref[l_0 as usize][1 as libc::c_int as usize] = *ref_0
                    .offset(
                        ((*h).mb.left_b8[0 as libc::c_int as usize]
                            + 1 as libc::c_int
                            + s8x8 * 1 as libc::c_int) as isize,
                    );
                (*h).mb.cache.topright_ref[l_0 as usize][2 as libc::c_int as usize] = *ref_0
                    .offset(
                        ((*h).mb.left_b8[1 as libc::c_int as usize]
                            + 1 as libc::c_int
                            + s8x8 * 0 as libc::c_int) as isize,
                    );
                (*(((*h).mb.cache.topright_mv[l_0 as usize][0 as libc::c_int as usize])
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i = (*((*mv
                    .offset(
                        ((*h).mb.left_b4[0 as libc::c_int as usize] + 3 as libc::c_int
                            + s4x4
                                * ((*left_index_table).mv[0 as libc::c_int as usize]
                                    as libc::c_int + 1 as libc::c_int)) as isize,
                    ))
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i;
                (*(((*h).mb.cache.topright_mv[l_0 as usize][1 as libc::c_int as usize])
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i = (*((*mv
                    .offset(
                        ((*h).mb.left_b4[0 as libc::c_int as usize] + 3 as libc::c_int
                            + s4x4
                                * ((*left_index_table).mv[1 as libc::c_int as usize]
                                    as libc::c_int + 1 as libc::c_int)) as isize,
                    ))
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i;
                (*(((*h).mb.cache.topright_mv[l_0 as usize][2 as libc::c_int as usize])
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i = (*((*mv
                    .offset(
                        ((*h).mb.left_b4[1 as libc::c_int as usize] + 3 as libc::c_int
                            + s4x4
                                * ((*left_index_table).mv[2 as libc::c_int as usize]
                                    as libc::c_int + 1 as libc::c_int)) as isize,
                    ))
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i;
            } else if (*h).mb.b_interlaced == 0
                && *((*h).mb.field).offset(((*h).mb.i_mb_xy - 1 as libc::c_int) as isize)
                    as libc::c_int
                    != 0
            {
                (*h).mb.cache.topright_ref[l_0 as usize][0 as libc::c_int as usize] = *ref_0
                    .offset(
                        ((*h).mb.left_b8[0 as libc::c_int as usize]
                            + 1 as libc::c_int
                            + s8x8 * 2 as libc::c_int
                            + s8x8
                                * (*left_index_table).ref_0[0 as libc::c_int as usize]
                                    as libc::c_int) as isize,
                    );
                (*h).mb.cache.topright_ref[l_0 as usize][1 as libc::c_int as usize] = *ref_0
                    .offset(
                        ((*h).mb.left_b8[0 as libc::c_int as usize]
                            + 1 as libc::c_int
                            + s8x8 * 2 as libc::c_int
                            + s8x8
                                * (*left_index_table).ref_0[1 as libc::c_int as usize]
                                    as libc::c_int) as isize,
                    );
                (*h).mb.cache.topright_ref[l_0 as usize][2 as libc::c_int as usize] = *ref_0
                    .offset(
                        ((*h).mb.left_b8[0 as libc::c_int as usize]
                            + 1 as libc::c_int
                            + s8x8 * 2 as libc::c_int
                            + s8x8
                                * (*left_index_table).ref_0[2 as libc::c_int as usize]
                                    as libc::c_int) as isize,
                    );
                (*(((*h).mb.cache.topright_mv[l_0 as usize][0 as libc::c_int as usize])
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i = (*((*mv
                    .offset(
                        ((*h).mb.left_b4[0 as libc::c_int as usize] + 3 as libc::c_int
                            + s4x4 * 4 as libc::c_int
                            + s4x4
                                * (*left_index_table).mv[0 as libc::c_int as usize]
                                    as libc::c_int) as isize,
                    ))
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i;
                (*(((*h).mb.cache.topright_mv[l_0 as usize][1 as libc::c_int as usize])
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i = (*((*mv
                    .offset(
                        ((*h).mb.left_b4[0 as libc::c_int as usize] + 3 as libc::c_int
                            + s4x4 * 4 as libc::c_int
                            + s4x4
                                * (*left_index_table).mv[1 as libc::c_int as usize]
                                    as libc::c_int) as isize,
                    ))
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i;
                (*(((*h).mb.cache.topright_mv[l_0 as usize][2 as libc::c_int as usize])
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i = (*((*mv
                    .offset(
                        ((*h).mb.left_b4[0 as libc::c_int as usize] + 3 as libc::c_int
                            + s4x4 * 4 as libc::c_int
                            + s4x4
                                * (*left_index_table).mv[2 as libc::c_int as usize]
                                    as libc::c_int) as isize,
                    ))
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i;
            }
        }
        if (*h).param.b_cabac != 0 {
            let mut mvd: *mut [[uint8_t; 2]; 8] = (*h).mb.mvd[l_0 as usize];
            if (*h).mb.i_neighbour & MB_TOP as libc::c_int as libc::c_uint != 0 {
                (*(((*h).mb.cache.mvd[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                    as libc::c_int
                    - 8 as libc::c_int)
                    as usize])
                    .as_mut_ptr() as *mut x264_union64_t))
                    .i = (*(((*mvd.offset(top as isize))[0 as libc::c_int as usize]).as_mut_ptr()
                    as *mut x264_union64_t))
                    .i;
            } else {
                (*(((*h).mb.cache.mvd[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                    as libc::c_int
                    - 8 as libc::c_int)
                    as usize])
                    .as_mut_ptr() as *mut x264_union64_t))
                    .i = 0 as libc::c_int as uint64_t;
            }
            if (*h).mb.i_neighbour & MB_LEFT as libc::c_int as libc::c_uint != 0
                && (b_mbaff == 0
                    || (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                        as libc::c_int
                        - 1 as libc::c_int)
                        as usize] as libc::c_int
                        >= 0 as libc::c_int)
            {
                (*(((*h).mb.cache.mvd[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                    as libc::c_int
                    - 1 as libc::c_int)
                    as usize])
                    .as_mut_ptr() as *mut x264_union16_t))
                    .i = (*(((*mvd.offset(*left.offset(0 as libc::c_int as isize) as isize))
                    [(*left_index_table).intra[0 as libc::c_int as usize] as usize])
                    .as_mut_ptr() as *mut x264_union16_t))
                    .i;
                (*(((*h).mb.cache.mvd[l_0 as usize][(x264_scan8[2 as libc::c_int as usize]
                    as libc::c_int
                    - 1 as libc::c_int)
                    as usize])
                    .as_mut_ptr() as *mut x264_union16_t))
                    .i = (*(((*mvd.offset(*left.offset(0 as libc::c_int as isize) as isize))
                    [(*left_index_table).intra[1 as libc::c_int as usize] as usize])
                    .as_mut_ptr() as *mut x264_union16_t))
                    .i;
            } else {
                (*(((*h).mb.cache.mvd[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                    as libc::c_int
                    - 1 as libc::c_int
                    + 0 as libc::c_int * 8 as libc::c_int)
                    as usize])
                    .as_mut_ptr() as *mut x264_union16_t))
                    .i = 0 as libc::c_int as uint16_t;
                (*(((*h).mb.cache.mvd[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                    as libc::c_int
                    - 1 as libc::c_int
                    + 1 as libc::c_int * 8 as libc::c_int)
                    as usize])
                    .as_mut_ptr() as *mut x264_union16_t))
                    .i = 0 as libc::c_int as uint16_t;
            }
            if (*h).mb.i_neighbour & MB_LEFT as libc::c_int as libc::c_uint != 0
                && (b_mbaff == 0
                    || (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                        as libc::c_int
                        - 1 as libc::c_int
                        + 2 as libc::c_int * 8 as libc::c_int)
                        as usize] as libc::c_int
                        >= 0 as libc::c_int)
            {
                (*(((*h).mb.cache.mvd[l_0 as usize][(x264_scan8[8 as libc::c_int as usize]
                    as libc::c_int
                    - 1 as libc::c_int)
                    as usize])
                    .as_mut_ptr() as *mut x264_union16_t))
                    .i = (*(((*mvd.offset(*left.offset(1 as libc::c_int as isize) as isize))
                    [(*left_index_table).intra[2 as libc::c_int as usize] as usize])
                    .as_mut_ptr() as *mut x264_union16_t))
                    .i;
                (*(((*h).mb.cache.mvd[l_0 as usize][(x264_scan8[10 as libc::c_int as usize]
                    as libc::c_int
                    - 1 as libc::c_int)
                    as usize])
                    .as_mut_ptr() as *mut x264_union16_t))
                    .i = (*(((*mvd.offset(*left.offset(1 as libc::c_int as isize) as isize))
                    [(*left_index_table).intra[3 as libc::c_int as usize] as usize])
                    .as_mut_ptr() as *mut x264_union16_t))
                    .i;
            } else {
                (*(((*h).mb.cache.mvd[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                    as libc::c_int
                    - 1 as libc::c_int
                    + 2 as libc::c_int * 8 as libc::c_int)
                    as usize])
                    .as_mut_ptr() as *mut x264_union16_t))
                    .i = 0 as libc::c_int as uint16_t;
                (*(((*h).mb.cache.mvd[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                    as libc::c_int
                    - 1 as libc::c_int
                    + 3 as libc::c_int * 8 as libc::c_int)
                    as usize])
                    .as_mut_ptr() as *mut x264_union16_t))
                    .i = 0 as libc::c_int as uint16_t;
            }
        }
        if b_mbaff != 0 {
            if (*h).mb.b_interlaced != 0 {
                if (*h).mb.i_mb_topleft_xy >= 0 as libc::c_int
                    && *((*h).mb.field).offset((*h).mb.i_mb_topleft_xy as isize) == 0
                    && (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                        as libc::c_int
                        - 1 as libc::c_int
                        - 1 as libc::c_int * 8 as libc::c_int)
                        as usize] as libc::c_int
                        >= 0 as libc::c_int
                {
                    (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                        as libc::c_int
                        - 1 as libc::c_int
                        - 1 as libc::c_int * 8 as libc::c_int)
                        as usize] = (((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                        [0 as libc::c_int as usize]
                        as libc::c_int
                        - 1 as libc::c_int
                        - 1 as libc::c_int * 8 as libc::c_int)
                        as usize] as libc::c_int)
                        << 1 as libc::c_int) as int8_t;
                    (*h).mb.cache.mv[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                        as libc::c_int
                        - 1 as libc::c_int
                        - 1 as libc::c_int * 8 as libc::c_int)
                        as usize][1 as libc::c_int as usize] = ((*h).mb.cache.mv[l_0 as usize]
                        [(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                            - 1 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize]
                        as libc::c_int
                        / 2 as libc::c_int)
                        as int16_t;
                    (*h).mb.cache.mvd[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                        as libc::c_int
                        - 1 as libc::c_int
                        - 1 as libc::c_int * 8 as libc::c_int)
                        as usize][1 as libc::c_int as usize] = ((*h).mb.cache.mvd[l_0 as usize]
                        [(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                            - 1 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize]
                        as libc::c_int
                        >> 1 as libc::c_int)
                        as uint8_t;
                }
                if top >= 0 as libc::c_int && *((*h).mb.field).offset(top as isize) == 0 {
                    if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                        as libc::c_int
                        + 0 as libc::c_int
                        - 1 as libc::c_int * 8 as libc::c_int)
                        as usize] as libc::c_int
                        >= 0 as libc::c_int
                    {
                        (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int
                            + 0 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize] = (((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                            [0 as libc::c_int as usize]
                            as libc::c_int
                            + 0 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize] as libc::c_int)
                            << 1 as libc::c_int) as int8_t;
                        (*h).mb.cache.mv[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int
                            + 0 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] = ((*h).mb.cache.mv[l_0 as usize]
                            [(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                                + 0 as libc::c_int
                                - 1 as libc::c_int * 8 as libc::c_int)
                                as usize][1 as libc::c_int as usize]
                            as libc::c_int
                            / 2 as libc::c_int)
                            as int16_t;
                        (*h).mb.cache.mvd[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int
                            + 0 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] = ((*h).mb.cache.mvd[l_0 as usize]
                            [(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                                + 0 as libc::c_int
                                - 1 as libc::c_int * 8 as libc::c_int)
                                as usize][1 as libc::c_int as usize]
                            as libc::c_int
                            >> 1 as libc::c_int)
                            as uint8_t;
                    }
                    if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                        as libc::c_int
                        + 1 as libc::c_int
                        - 1 as libc::c_int * 8 as libc::c_int)
                        as usize] as libc::c_int
                        >= 0 as libc::c_int
                    {
                        (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int
                            + 1 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize] = (((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                            [0 as libc::c_int as usize]
                            as libc::c_int
                            + 1 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize] as libc::c_int)
                            << 1 as libc::c_int) as int8_t;
                        (*h).mb.cache.mv[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int
                            + 1 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] = ((*h).mb.cache.mv[l_0 as usize]
                            [(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                                + 1 as libc::c_int
                                - 1 as libc::c_int * 8 as libc::c_int)
                                as usize][1 as libc::c_int as usize]
                            as libc::c_int
                            / 2 as libc::c_int)
                            as int16_t;
                        (*h).mb.cache.mvd[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int
                            + 1 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] = ((*h).mb.cache.mvd[l_0 as usize]
                            [(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                                + 1 as libc::c_int
                                - 1 as libc::c_int * 8 as libc::c_int)
                                as usize][1 as libc::c_int as usize]
                            as libc::c_int
                            >> 1 as libc::c_int)
                            as uint8_t;
                    }
                    if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                        as libc::c_int
                        + 2 as libc::c_int
                        - 1 as libc::c_int * 8 as libc::c_int)
                        as usize] as libc::c_int
                        >= 0 as libc::c_int
                    {
                        (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int
                            + 2 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize] = (((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                            [0 as libc::c_int as usize]
                            as libc::c_int
                            + 2 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize] as libc::c_int)
                            << 1 as libc::c_int) as int8_t;
                        (*h).mb.cache.mv[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int
                            + 2 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] = ((*h).mb.cache.mv[l_0 as usize]
                            [(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                                + 2 as libc::c_int
                                - 1 as libc::c_int * 8 as libc::c_int)
                                as usize][1 as libc::c_int as usize]
                            as libc::c_int
                            / 2 as libc::c_int)
                            as int16_t;
                        (*h).mb.cache.mvd[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int
                            + 2 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] = ((*h).mb.cache.mvd[l_0 as usize]
                            [(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                                + 2 as libc::c_int
                                - 1 as libc::c_int * 8 as libc::c_int)
                                as usize][1 as libc::c_int as usize]
                            as libc::c_int
                            >> 1 as libc::c_int)
                            as uint8_t;
                    }
                    if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                        as libc::c_int
                        + 3 as libc::c_int
                        - 1 as libc::c_int * 8 as libc::c_int)
                        as usize] as libc::c_int
                        >= 0 as libc::c_int
                    {
                        (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int
                            + 3 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize] = (((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                            [0 as libc::c_int as usize]
                            as libc::c_int
                            + 3 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize] as libc::c_int)
                            << 1 as libc::c_int) as int8_t;
                        (*h).mb.cache.mv[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int
                            + 3 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] = ((*h).mb.cache.mv[l_0 as usize]
                            [(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                                + 3 as libc::c_int
                                - 1 as libc::c_int * 8 as libc::c_int)
                                as usize][1 as libc::c_int as usize]
                            as libc::c_int
                            / 2 as libc::c_int)
                            as int16_t;
                        (*h).mb.cache.mvd[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int
                            + 3 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] = ((*h).mb.cache.mvd[l_0 as usize]
                            [(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                                + 3 as libc::c_int
                                - 1 as libc::c_int * 8 as libc::c_int)
                                as usize][1 as libc::c_int as usize]
                            as libc::c_int
                            >> 1 as libc::c_int)
                            as uint8_t;
                    }
                }
                if (*h).mb.i_mb_topright_xy >= 0 as libc::c_int
                    && *((*h).mb.field).offset((*h).mb.i_mb_topright_xy as isize) == 0
                    && (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                        as libc::c_int
                        + 4 as libc::c_int
                        - 1 as libc::c_int * 8 as libc::c_int)
                        as usize] as libc::c_int
                        >= 0 as libc::c_int
                {
                    (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                        as libc::c_int
                        + 4 as libc::c_int
                        - 1 as libc::c_int * 8 as libc::c_int)
                        as usize] = (((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                        [0 as libc::c_int as usize]
                        as libc::c_int
                        + 4 as libc::c_int
                        - 1 as libc::c_int * 8 as libc::c_int)
                        as usize] as libc::c_int)
                        << 1 as libc::c_int) as int8_t;
                    (*h).mb.cache.mv[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                        as libc::c_int
                        + 4 as libc::c_int
                        - 1 as libc::c_int * 8 as libc::c_int)
                        as usize][1 as libc::c_int as usize] = ((*h).mb.cache.mv[l_0 as usize]
                        [(x264_scan8[0 as libc::c_int as usize] as libc::c_int + 4 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize]
                        as libc::c_int
                        / 2 as libc::c_int)
                        as int16_t;
                    (*h).mb.cache.mvd[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                        as libc::c_int
                        + 4 as libc::c_int
                        - 1 as libc::c_int * 8 as libc::c_int)
                        as usize][1 as libc::c_int as usize] = ((*h).mb.cache.mvd[l_0 as usize]
                        [(x264_scan8[0 as libc::c_int as usize] as libc::c_int + 4 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize]
                        as libc::c_int
                        >> 1 as libc::c_int)
                        as uint8_t;
                }
                if *left.offset(0 as libc::c_int as isize) >= 0 as libc::c_int
                    && *((*h).mb.field).offset(*left.offset(0 as libc::c_int as isize) as isize)
                        == 0
                {
                    if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                        as libc::c_int
                        - 1 as libc::c_int
                        + 0 as libc::c_int * 8 as libc::c_int)
                        as usize] as libc::c_int
                        >= 0 as libc::c_int
                    {
                        (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int
                            - 1 as libc::c_int
                            + 0 as libc::c_int * 8 as libc::c_int)
                            as usize] = (((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                            [0 as libc::c_int as usize]
                            as libc::c_int
                            - 1 as libc::c_int
                            + 0 as libc::c_int * 8 as libc::c_int)
                            as usize] as libc::c_int)
                            << 1 as libc::c_int) as int8_t;
                        (*h).mb.cache.mv[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int
                            - 1 as libc::c_int
                            + 0 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] = ((*h).mb.cache.mv[l_0 as usize]
                            [(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                                - 1 as libc::c_int
                                + 0 as libc::c_int * 8 as libc::c_int)
                                as usize][1 as libc::c_int as usize]
                            as libc::c_int
                            / 2 as libc::c_int)
                            as int16_t;
                        (*h).mb.cache.mvd[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int
                            - 1 as libc::c_int
                            + 0 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] = ((*h).mb.cache.mvd[l_0 as usize]
                            [(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                                - 1 as libc::c_int
                                + 0 as libc::c_int * 8 as libc::c_int)
                                as usize][1 as libc::c_int as usize]
                            as libc::c_int
                            >> 1 as libc::c_int)
                            as uint8_t;
                    }
                    if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                        as libc::c_int
                        - 1 as libc::c_int
                        + 1 as libc::c_int * 8 as libc::c_int)
                        as usize] as libc::c_int
                        >= 0 as libc::c_int
                    {
                        (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int
                            - 1 as libc::c_int
                            + 1 as libc::c_int * 8 as libc::c_int)
                            as usize] = (((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                            [0 as libc::c_int as usize]
                            as libc::c_int
                            - 1 as libc::c_int
                            + 1 as libc::c_int * 8 as libc::c_int)
                            as usize] as libc::c_int)
                            << 1 as libc::c_int) as int8_t;
                        (*h).mb.cache.mv[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int
                            - 1 as libc::c_int
                            + 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] = ((*h).mb.cache.mv[l_0 as usize]
                            [(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                                - 1 as libc::c_int
                                + 1 as libc::c_int * 8 as libc::c_int)
                                as usize][1 as libc::c_int as usize]
                            as libc::c_int
                            / 2 as libc::c_int)
                            as int16_t;
                        (*h).mb.cache.mvd[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int
                            - 1 as libc::c_int
                            + 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] = ((*h).mb.cache.mvd[l_0 as usize]
                            [(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                                - 1 as libc::c_int
                                + 1 as libc::c_int * 8 as libc::c_int)
                                as usize][1 as libc::c_int as usize]
                            as libc::c_int
                            >> 1 as libc::c_int)
                            as uint8_t;
                    }
                    if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                        as libc::c_int
                        - 1 as libc::c_int
                        + 2 as libc::c_int * 8 as libc::c_int)
                        as usize] as libc::c_int
                        >= 0 as libc::c_int
                    {
                        (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int
                            - 1 as libc::c_int
                            + 2 as libc::c_int * 8 as libc::c_int)
                            as usize] = (((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                            [0 as libc::c_int as usize]
                            as libc::c_int
                            - 1 as libc::c_int
                            + 2 as libc::c_int * 8 as libc::c_int)
                            as usize] as libc::c_int)
                            << 1 as libc::c_int) as int8_t;
                        (*h).mb.cache.mv[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int
                            - 1 as libc::c_int
                            + 2 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] = ((*h).mb.cache.mv[l_0 as usize]
                            [(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                                - 1 as libc::c_int
                                + 2 as libc::c_int * 8 as libc::c_int)
                                as usize][1 as libc::c_int as usize]
                            as libc::c_int
                            / 2 as libc::c_int)
                            as int16_t;
                        (*h).mb.cache.mvd[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int
                            - 1 as libc::c_int
                            + 2 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] = ((*h).mb.cache.mvd[l_0 as usize]
                            [(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                                - 1 as libc::c_int
                                + 2 as libc::c_int * 8 as libc::c_int)
                                as usize][1 as libc::c_int as usize]
                            as libc::c_int
                            >> 1 as libc::c_int)
                            as uint8_t;
                    }
                    if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                        as libc::c_int
                        - 1 as libc::c_int
                        + 3 as libc::c_int * 8 as libc::c_int)
                        as usize] as libc::c_int
                        >= 0 as libc::c_int
                    {
                        (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int
                            - 1 as libc::c_int
                            + 3 as libc::c_int * 8 as libc::c_int)
                            as usize] = (((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                            [0 as libc::c_int as usize]
                            as libc::c_int
                            - 1 as libc::c_int
                            + 3 as libc::c_int * 8 as libc::c_int)
                            as usize] as libc::c_int)
                            << 1 as libc::c_int) as int8_t;
                        (*h).mb.cache.mv[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int
                            - 1 as libc::c_int
                            + 3 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] = ((*h).mb.cache.mv[l_0 as usize]
                            [(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                                - 1 as libc::c_int
                                + 3 as libc::c_int * 8 as libc::c_int)
                                as usize][1 as libc::c_int as usize]
                            as libc::c_int
                            / 2 as libc::c_int)
                            as int16_t;
                        (*h).mb.cache.mvd[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int
                            - 1 as libc::c_int
                            + 3 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] = ((*h).mb.cache.mvd[l_0 as usize]
                            [(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                                - 1 as libc::c_int
                                + 3 as libc::c_int * 8 as libc::c_int)
                                as usize][1 as libc::c_int as usize]
                            as libc::c_int
                            >> 1 as libc::c_int)
                            as uint8_t;
                    }
                    if (*h).mb.cache.topright_ref[l_0 as usize][0 as libc::c_int as usize]
                        as libc::c_int
                        >= 0 as libc::c_int
                    {
                        (*h).mb.cache.topright_ref[l_0 as usize][0 as libc::c_int as usize] =
                            (((*h).mb.cache.topright_ref[l_0 as usize][0 as libc::c_int as usize]
                                as libc::c_int)
                                << 1 as libc::c_int) as int8_t;
                        (*h).mb.cache.topright_mv[l_0 as usize][0 as libc::c_int as usize]
                            [1 as libc::c_int as usize] = ((*h).mb.cache.topright_mv[l_0 as usize]
                            [0 as libc::c_int as usize][1 as libc::c_int as usize]
                            as libc::c_int
                            / 2 as libc::c_int)
                            as int16_t;
                        (*h).mb.cache.mvd[l_0 as usize][0 as libc::c_int as usize]
                            [1 as libc::c_int as usize] = ((*h).mb.cache.mvd[l_0 as usize]
                            [0 as libc::c_int as usize][1 as libc::c_int as usize]
                            as libc::c_int
                            >> 1 as libc::c_int)
                            as uint8_t;
                    }
                    if (*h).mb.cache.topright_ref[l_0 as usize][1 as libc::c_int as usize]
                        as libc::c_int
                        >= 0 as libc::c_int
                    {
                        (*h).mb.cache.topright_ref[l_0 as usize][1 as libc::c_int as usize] =
                            (((*h).mb.cache.topright_ref[l_0 as usize][1 as libc::c_int as usize]
                                as libc::c_int)
                                << 1 as libc::c_int) as int8_t;
                        (*h).mb.cache.topright_mv[l_0 as usize][1 as libc::c_int as usize]
                            [1 as libc::c_int as usize] = ((*h).mb.cache.topright_mv[l_0 as usize]
                            [1 as libc::c_int as usize][1 as libc::c_int as usize]
                            as libc::c_int
                            / 2 as libc::c_int)
                            as int16_t;
                        (*h).mb.cache.mvd[l_0 as usize][1 as libc::c_int as usize]
                            [1 as libc::c_int as usize] = ((*h).mb.cache.mvd[l_0 as usize]
                            [1 as libc::c_int as usize][1 as libc::c_int as usize]
                            as libc::c_int
                            >> 1 as libc::c_int)
                            as uint8_t;
                    }
                    if (*h).mb.cache.topright_ref[l_0 as usize][2 as libc::c_int as usize]
                        as libc::c_int
                        >= 0 as libc::c_int
                    {
                        (*h).mb.cache.topright_ref[l_0 as usize][2 as libc::c_int as usize] =
                            (((*h).mb.cache.topright_ref[l_0 as usize][2 as libc::c_int as usize]
                                as libc::c_int)
                                << 1 as libc::c_int) as int8_t;
                        (*h).mb.cache.topright_mv[l_0 as usize][2 as libc::c_int as usize]
                            [1 as libc::c_int as usize] = ((*h).mb.cache.topright_mv[l_0 as usize]
                            [2 as libc::c_int as usize][1 as libc::c_int as usize]
                            as libc::c_int
                            / 2 as libc::c_int)
                            as int16_t;
                        (*h).mb.cache.mvd[l_0 as usize][2 as libc::c_int as usize]
                            [1 as libc::c_int as usize] = ((*h).mb.cache.mvd[l_0 as usize]
                            [2 as libc::c_int as usize][1 as libc::c_int as usize]
                            as libc::c_int
                            >> 1 as libc::c_int)
                            as uint8_t;
                    }
                }
            } else {
                if (*h).mb.i_mb_topleft_xy >= 0 as libc::c_int
                    && *((*h).mb.field).offset((*h).mb.i_mb_topleft_xy as isize) as libc::c_int != 0
                    && (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                        as libc::c_int
                        - 1 as libc::c_int
                        - 1 as libc::c_int * 8 as libc::c_int)
                        as usize] as libc::c_int
                        >= 0 as libc::c_int
                {
                    (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                        as libc::c_int
                        - 1 as libc::c_int
                        - 1 as libc::c_int * 8 as libc::c_int)
                        as usize] = ((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                        [0 as libc::c_int as usize]
                        as libc::c_int
                        - 1 as libc::c_int
                        - 1 as libc::c_int * 8 as libc::c_int)
                        as usize] as libc::c_int
                        >> 1 as libc::c_int) as int8_t;
                    (*h).mb.cache.mv[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                        as libc::c_int
                        - 1 as libc::c_int
                        - 1 as libc::c_int * 8 as libc::c_int)
                        as usize][1 as libc::c_int as usize] = ((*h).mb.cache.mv[l_0 as usize]
                        [(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                            - 1 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize]
                        as libc::c_int
                        * 2 as libc::c_int)
                        as int16_t;
                    (*h).mb.cache.mvd[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                        as libc::c_int
                        - 1 as libc::c_int
                        - 1 as libc::c_int * 8 as libc::c_int)
                        as usize][1 as libc::c_int as usize] = (((*h).mb.cache.mvd[l_0 as usize]
                        [(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                            - 1 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize]
                        as libc::c_int)
                        << 1 as libc::c_int)
                        as uint8_t;
                }
                if top >= 0 as libc::c_int
                    && *((*h).mb.field).offset(top as isize) as libc::c_int != 0
                {
                    if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                        as libc::c_int
                        + 0 as libc::c_int
                        - 1 as libc::c_int * 8 as libc::c_int)
                        as usize] as libc::c_int
                        >= 0 as libc::c_int
                    {
                        (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int
                            + 0 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize] = ((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                            [0 as libc::c_int as usize]
                            as libc::c_int
                            + 0 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize] as libc::c_int
                            >> 1 as libc::c_int) as int8_t;
                        (*h).mb.cache.mv[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int
                            + 0 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] = ((*h).mb.cache.mv[l_0 as usize]
                            [(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                                + 0 as libc::c_int
                                - 1 as libc::c_int * 8 as libc::c_int)
                                as usize][1 as libc::c_int as usize]
                            as libc::c_int
                            * 2 as libc::c_int)
                            as int16_t;
                        (*h).mb.cache.mvd[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int
                            + 0 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] = (((*h).mb.cache.mvd[l_0 as usize]
                            [(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                                + 0 as libc::c_int
                                - 1 as libc::c_int * 8 as libc::c_int)
                                as usize][1 as libc::c_int as usize]
                            as libc::c_int)
                            << 1 as libc::c_int)
                            as uint8_t;
                    }
                    if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                        as libc::c_int
                        + 1 as libc::c_int
                        - 1 as libc::c_int * 8 as libc::c_int)
                        as usize] as libc::c_int
                        >= 0 as libc::c_int
                    {
                        (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int
                            + 1 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize] = ((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                            [0 as libc::c_int as usize]
                            as libc::c_int
                            + 1 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize] as libc::c_int
                            >> 1 as libc::c_int) as int8_t;
                        (*h).mb.cache.mv[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int
                            + 1 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] = ((*h).mb.cache.mv[l_0 as usize]
                            [(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                                + 1 as libc::c_int
                                - 1 as libc::c_int * 8 as libc::c_int)
                                as usize][1 as libc::c_int as usize]
                            as libc::c_int
                            * 2 as libc::c_int)
                            as int16_t;
                        (*h).mb.cache.mvd[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int
                            + 1 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] = (((*h).mb.cache.mvd[l_0 as usize]
                            [(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                                + 1 as libc::c_int
                                - 1 as libc::c_int * 8 as libc::c_int)
                                as usize][1 as libc::c_int as usize]
                            as libc::c_int)
                            << 1 as libc::c_int)
                            as uint8_t;
                    }
                    if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                        as libc::c_int
                        + 2 as libc::c_int
                        - 1 as libc::c_int * 8 as libc::c_int)
                        as usize] as libc::c_int
                        >= 0 as libc::c_int
                    {
                        (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int
                            + 2 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize] = ((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                            [0 as libc::c_int as usize]
                            as libc::c_int
                            + 2 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize] as libc::c_int
                            >> 1 as libc::c_int) as int8_t;
                        (*h).mb.cache.mv[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int
                            + 2 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] = ((*h).mb.cache.mv[l_0 as usize]
                            [(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                                + 2 as libc::c_int
                                - 1 as libc::c_int * 8 as libc::c_int)
                                as usize][1 as libc::c_int as usize]
                            as libc::c_int
                            * 2 as libc::c_int)
                            as int16_t;
                        (*h).mb.cache.mvd[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int
                            + 2 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] = (((*h).mb.cache.mvd[l_0 as usize]
                            [(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                                + 2 as libc::c_int
                                - 1 as libc::c_int * 8 as libc::c_int)
                                as usize][1 as libc::c_int as usize]
                            as libc::c_int)
                            << 1 as libc::c_int)
                            as uint8_t;
                    }
                    if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                        as libc::c_int
                        + 3 as libc::c_int
                        - 1 as libc::c_int * 8 as libc::c_int)
                        as usize] as libc::c_int
                        >= 0 as libc::c_int
                    {
                        (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int
                            + 3 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize] = ((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                            [0 as libc::c_int as usize]
                            as libc::c_int
                            + 3 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize] as libc::c_int
                            >> 1 as libc::c_int) as int8_t;
                        (*h).mb.cache.mv[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int
                            + 3 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] = ((*h).mb.cache.mv[l_0 as usize]
                            [(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                                + 3 as libc::c_int
                                - 1 as libc::c_int * 8 as libc::c_int)
                                as usize][1 as libc::c_int as usize]
                            as libc::c_int
                            * 2 as libc::c_int)
                            as int16_t;
                        (*h).mb.cache.mvd[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int
                            + 3 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] = (((*h).mb.cache.mvd[l_0 as usize]
                            [(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                                + 3 as libc::c_int
                                - 1 as libc::c_int * 8 as libc::c_int)
                                as usize][1 as libc::c_int as usize]
                            as libc::c_int)
                            << 1 as libc::c_int)
                            as uint8_t;
                    }
                }
                if (*h).mb.i_mb_topright_xy >= 0 as libc::c_int
                    && *((*h).mb.field).offset((*h).mb.i_mb_topright_xy as isize) as libc::c_int
                        != 0
                    && (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                        as libc::c_int
                        + 4 as libc::c_int
                        - 1 as libc::c_int * 8 as libc::c_int)
                        as usize] as libc::c_int
                        >= 0 as libc::c_int
                {
                    (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                        as libc::c_int
                        + 4 as libc::c_int
                        - 1 as libc::c_int * 8 as libc::c_int)
                        as usize] = ((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                        [0 as libc::c_int as usize]
                        as libc::c_int
                        + 4 as libc::c_int
                        - 1 as libc::c_int * 8 as libc::c_int)
                        as usize] as libc::c_int
                        >> 1 as libc::c_int) as int8_t;
                    (*h).mb.cache.mv[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                        as libc::c_int
                        + 4 as libc::c_int
                        - 1 as libc::c_int * 8 as libc::c_int)
                        as usize][1 as libc::c_int as usize] = ((*h).mb.cache.mv[l_0 as usize]
                        [(x264_scan8[0 as libc::c_int as usize] as libc::c_int + 4 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize]
                        as libc::c_int
                        * 2 as libc::c_int)
                        as int16_t;
                    (*h).mb.cache.mvd[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                        as libc::c_int
                        + 4 as libc::c_int
                        - 1 as libc::c_int * 8 as libc::c_int)
                        as usize][1 as libc::c_int as usize] = (((*h).mb.cache.mvd[l_0 as usize]
                        [(x264_scan8[0 as libc::c_int as usize] as libc::c_int + 4 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize]
                        as libc::c_int)
                        << 1 as libc::c_int)
                        as uint8_t;
                }
                if *left.offset(0 as libc::c_int as isize) >= 0 as libc::c_int
                    && *((*h).mb.field).offset(*left.offset(0 as libc::c_int as isize) as isize)
                        as libc::c_int
                        != 0
                {
                    if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                        as libc::c_int
                        - 1 as libc::c_int
                        + 0 as libc::c_int * 8 as libc::c_int)
                        as usize] as libc::c_int
                        >= 0 as libc::c_int
                    {
                        (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int
                            - 1 as libc::c_int
                            + 0 as libc::c_int * 8 as libc::c_int)
                            as usize] = ((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                            [0 as libc::c_int as usize]
                            as libc::c_int
                            - 1 as libc::c_int
                            + 0 as libc::c_int * 8 as libc::c_int)
                            as usize] as libc::c_int
                            >> 1 as libc::c_int) as int8_t;
                        (*h).mb.cache.mv[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int
                            - 1 as libc::c_int
                            + 0 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] = ((*h).mb.cache.mv[l_0 as usize]
                            [(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                                - 1 as libc::c_int
                                + 0 as libc::c_int * 8 as libc::c_int)
                                as usize][1 as libc::c_int as usize]
                            as libc::c_int
                            * 2 as libc::c_int)
                            as int16_t;
                        (*h).mb.cache.mvd[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int
                            - 1 as libc::c_int
                            + 0 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] = (((*h).mb.cache.mvd[l_0 as usize]
                            [(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                                - 1 as libc::c_int
                                + 0 as libc::c_int * 8 as libc::c_int)
                                as usize][1 as libc::c_int as usize]
                            as libc::c_int)
                            << 1 as libc::c_int)
                            as uint8_t;
                    }
                    if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                        as libc::c_int
                        - 1 as libc::c_int
                        + 1 as libc::c_int * 8 as libc::c_int)
                        as usize] as libc::c_int
                        >= 0 as libc::c_int
                    {
                        (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int
                            - 1 as libc::c_int
                            + 1 as libc::c_int * 8 as libc::c_int)
                            as usize] = ((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                            [0 as libc::c_int as usize]
                            as libc::c_int
                            - 1 as libc::c_int
                            + 1 as libc::c_int * 8 as libc::c_int)
                            as usize] as libc::c_int
                            >> 1 as libc::c_int) as int8_t;
                        (*h).mb.cache.mv[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int
                            - 1 as libc::c_int
                            + 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] = ((*h).mb.cache.mv[l_0 as usize]
                            [(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                                - 1 as libc::c_int
                                + 1 as libc::c_int * 8 as libc::c_int)
                                as usize][1 as libc::c_int as usize]
                            as libc::c_int
                            * 2 as libc::c_int)
                            as int16_t;
                        (*h).mb.cache.mvd[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int
                            - 1 as libc::c_int
                            + 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] = (((*h).mb.cache.mvd[l_0 as usize]
                            [(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                                - 1 as libc::c_int
                                + 1 as libc::c_int * 8 as libc::c_int)
                                as usize][1 as libc::c_int as usize]
                            as libc::c_int)
                            << 1 as libc::c_int)
                            as uint8_t;
                    }
                    if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                        as libc::c_int
                        - 1 as libc::c_int
                        + 2 as libc::c_int * 8 as libc::c_int)
                        as usize] as libc::c_int
                        >= 0 as libc::c_int
                    {
                        (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int
                            - 1 as libc::c_int
                            + 2 as libc::c_int * 8 as libc::c_int)
                            as usize] = ((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                            [0 as libc::c_int as usize]
                            as libc::c_int
                            - 1 as libc::c_int
                            + 2 as libc::c_int * 8 as libc::c_int)
                            as usize] as libc::c_int
                            >> 1 as libc::c_int) as int8_t;
                        (*h).mb.cache.mv[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int
                            - 1 as libc::c_int
                            + 2 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] = ((*h).mb.cache.mv[l_0 as usize]
                            [(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                                - 1 as libc::c_int
                                + 2 as libc::c_int * 8 as libc::c_int)
                                as usize][1 as libc::c_int as usize]
                            as libc::c_int
                            * 2 as libc::c_int)
                            as int16_t;
                        (*h).mb.cache.mvd[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int
                            - 1 as libc::c_int
                            + 2 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] = (((*h).mb.cache.mvd[l_0 as usize]
                            [(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                                - 1 as libc::c_int
                                + 2 as libc::c_int * 8 as libc::c_int)
                                as usize][1 as libc::c_int as usize]
                            as libc::c_int)
                            << 1 as libc::c_int)
                            as uint8_t;
                    }
                    if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                        as libc::c_int
                        - 1 as libc::c_int
                        + 3 as libc::c_int * 8 as libc::c_int)
                        as usize] as libc::c_int
                        >= 0 as libc::c_int
                    {
                        (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int
                            - 1 as libc::c_int
                            + 3 as libc::c_int * 8 as libc::c_int)
                            as usize] = ((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                            [0 as libc::c_int as usize]
                            as libc::c_int
                            - 1 as libc::c_int
                            + 3 as libc::c_int * 8 as libc::c_int)
                            as usize] as libc::c_int
                            >> 1 as libc::c_int) as int8_t;
                        (*h).mb.cache.mv[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int
                            - 1 as libc::c_int
                            + 3 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] = ((*h).mb.cache.mv[l_0 as usize]
                            [(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                                - 1 as libc::c_int
                                + 3 as libc::c_int * 8 as libc::c_int)
                                as usize][1 as libc::c_int as usize]
                            as libc::c_int
                            * 2 as libc::c_int)
                            as int16_t;
                        (*h).mb.cache.mvd[l_0 as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int
                            - 1 as libc::c_int
                            + 3 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] = (((*h).mb.cache.mvd[l_0 as usize]
                            [(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                                - 1 as libc::c_int
                                + 3 as libc::c_int * 8 as libc::c_int)
                                as usize][1 as libc::c_int as usize]
                            as libc::c_int)
                            << 1 as libc::c_int)
                            as uint8_t;
                    }
                    if (*h).mb.cache.topright_ref[l_0 as usize][0 as libc::c_int as usize]
                        as libc::c_int
                        >= 0 as libc::c_int
                    {
                        (*h).mb.cache.topright_ref[l_0 as usize][0 as libc::c_int as usize] =
                            ((*h).mb.cache.topright_ref[l_0 as usize][0 as libc::c_int as usize]
                                as libc::c_int
                                >> 1 as libc::c_int) as int8_t;
                        (*h).mb.cache.topright_mv[l_0 as usize][0 as libc::c_int as usize]
                            [1 as libc::c_int as usize] = ((*h).mb.cache.topright_mv[l_0 as usize]
                            [0 as libc::c_int as usize][1 as libc::c_int as usize]
                            as libc::c_int
                            * 2 as libc::c_int)
                            as int16_t;
                        (*h).mb.cache.mvd[l_0 as usize][0 as libc::c_int as usize]
                            [1 as libc::c_int as usize] = (((*h).mb.cache.mvd[l_0 as usize]
                            [0 as libc::c_int as usize][1 as libc::c_int as usize]
                            as libc::c_int)
                            << 1 as libc::c_int)
                            as uint8_t;
                    }
                    if (*h).mb.cache.topright_ref[l_0 as usize][1 as libc::c_int as usize]
                        as libc::c_int
                        >= 0 as libc::c_int
                    {
                        (*h).mb.cache.topright_ref[l_0 as usize][1 as libc::c_int as usize] =
                            ((*h).mb.cache.topright_ref[l_0 as usize][1 as libc::c_int as usize]
                                as libc::c_int
                                >> 1 as libc::c_int) as int8_t;
                        (*h).mb.cache.topright_mv[l_0 as usize][1 as libc::c_int as usize]
                            [1 as libc::c_int as usize] = ((*h).mb.cache.topright_mv[l_0 as usize]
                            [1 as libc::c_int as usize][1 as libc::c_int as usize]
                            as libc::c_int
                            * 2 as libc::c_int)
                            as int16_t;
                        (*h).mb.cache.mvd[l_0 as usize][1 as libc::c_int as usize]
                            [1 as libc::c_int as usize] = (((*h).mb.cache.mvd[l_0 as usize]
                            [1 as libc::c_int as usize][1 as libc::c_int as usize]
                            as libc::c_int)
                            << 1 as libc::c_int)
                            as uint8_t;
                    }
                    if (*h).mb.cache.topright_ref[l_0 as usize][2 as libc::c_int as usize]
                        as libc::c_int
                        >= 0 as libc::c_int
                    {
                        (*h).mb.cache.topright_ref[l_0 as usize][2 as libc::c_int as usize] =
                            ((*h).mb.cache.topright_ref[l_0 as usize][2 as libc::c_int as usize]
                                as libc::c_int
                                >> 1 as libc::c_int) as int8_t;
                        (*h).mb.cache.topright_mv[l_0 as usize][2 as libc::c_int as usize]
                            [1 as libc::c_int as usize] = ((*h).mb.cache.topright_mv[l_0 as usize]
                            [2 as libc::c_int as usize][1 as libc::c_int as usize]
                            as libc::c_int
                            * 2 as libc::c_int)
                            as int16_t;
                        (*h).mb.cache.mvd[l_0 as usize][2 as libc::c_int as usize]
                            [1 as libc::c_int as usize] = (((*h).mb.cache.mvd[l_0 as usize]
                            [2 as libc::c_int as usize][1 as libc::c_int as usize]
                            as libc::c_int)
                            << 1 as libc::c_int)
                            as uint8_t;
                    }
                }
            }
        }
        l_0 += 1;
        l_0;
    }
    if b_mbaff != 0 && mb_x == 0 as libc::c_int && mb_y & 1 as libc::c_int == 0 {
        if (*h).mb.i_mb_top_xy >= (*h).sh.i_first_mb {
            (*h).mb.field_decoding_flag =
                *((*h).mb.field).offset((*h).mb.i_mb_top_xy as isize) as libc::c_int;
        } else {
            (*h).mb.field_decoding_flag = 0 as libc::c_int;
        }
    }
    (*h).mb.b_allow_skip = 1 as libc::c_int;
    if b_mbaff != 0
        && (*h).mb.b_interlaced != (*h).mb.field_decoding_flag
        && mb_y & 1 as libc::c_int != 0
        && (*((*h).mb.type_0).offset(((*h).mb.i_mb_xy - (*h).mb.i_mb_stride) as isize)
            as libc::c_int
            == P_SKIP as libc::c_int
            || *((*h).mb.type_0).offset(((*h).mb.i_mb_xy - (*h).mb.i_mb_stride) as isize)
                as libc::c_int
                == B_SKIP as libc::c_int)
    {
        (*h).mb.b_allow_skip = 0 as libc::c_int;
    }
    if (*h).param.b_cabac != 0 {
        if b_mbaff != 0 {
            let mut left_xy: libc::c_int = 0;
            let mut top_xy: libc::c_int = 0;
            let mut mb_xy: libc::c_int = mb_x + (mb_y & !(1 as libc::c_int)) * (*h).mb.i_mb_stride;
            left_xy = mb_xy - 1 as libc::c_int;
            if mb_y & 1 as libc::c_int != 0
                && mb_x > 0 as libc::c_int
                && (*h).mb.field_decoding_flag
                    == *((*h).mb.field).offset(left_xy as isize) as libc::c_int
            {
                left_xy += (*h).mb.i_mb_stride;
            }
            if (*h).mb.field_decoding_flag != 0 {
                top_xy = mb_xy - (*h).mb.i_mb_stride;
                if mb_y & 1 as libc::c_int == 0
                    && top_xy >= 0 as libc::c_int
                    && *((*h).mb.slice_table).offset(top_xy as isize) == (*h).sh.i_first_mb
                    && *((*h).mb.field).offset(top_xy as isize) as libc::c_int != 0
                {
                    top_xy -= (*h).mb.i_mb_stride;
                }
            } else {
                top_xy = mb_x + (mb_y - 1 as libc::c_int) * (*h).mb.i_mb_stride;
            }
            (*h).mb.cache.i_neighbour_skip = (mb_x > 0 as libc::c_int
                && *((*h).mb.slice_table).offset(left_xy as isize) == (*h).sh.i_first_mb
                && !(*((*h).mb.type_0).offset(left_xy as isize) as libc::c_int
                    == P_SKIP as libc::c_int
                    || *((*h).mb.type_0).offset(left_xy as isize) as libc::c_int
                        == B_SKIP as libc::c_int))
                as libc::c_int
                + (top_xy >= 0 as libc::c_int
                    && *((*h).mb.slice_table).offset(top_xy as isize) == (*h).sh.i_first_mb
                    && !(*((*h).mb.type_0).offset(top_xy as isize) as libc::c_int
                        == P_SKIP as libc::c_int
                        || *((*h).mb.type_0).offset(top_xy as isize) as libc::c_int
                            == B_SKIP as libc::c_int)) as libc::c_int;
        } else {
            (*h).mb.cache.i_neighbour_skip = ((*h).mb.i_neighbour
                & MB_LEFT as libc::c_int as libc::c_uint
                != 0
                && !((*h).mb.i_mb_type_left[0 as libc::c_int as usize] == P_SKIP as libc::c_int
                    || (*h).mb.i_mb_type_left[0 as libc::c_int as usize] == B_SKIP as libc::c_int))
                as libc::c_int
                + ((*h).mb.i_neighbour & MB_TOP as libc::c_int as libc::c_uint != 0
                    && !((*h).mb.i_mb_type_top == P_SKIP as libc::c_int
                        || (*h).mb.i_mb_type_top == B_SKIP as libc::c_int))
                    as libc::c_int;
        }
    }
    if (*h).sh.i_type == SLICE_TYPE_B as libc::c_int {
        (*h).mb.bipred_weight = ((*h).mb.bipred_weight_buf[(*h).mb.b_interlaced as usize]
            [((*h).mb.b_interlaced & (mb_y & 1 as libc::c_int)) as usize])
            .as_mut_ptr();
        (*h).mb.dist_scale_factor = ((*h).mb.dist_scale_factor_buf[(*h).mb.b_interlaced as usize]
            [((*h).mb.b_interlaced & (mb_y & 1 as libc::c_int)) as usize])
            .as_mut_ptr();
        if (*h).param.b_cabac != 0 {
            let mut skipbp: uint8_t = 0;
            x264_macroblock_cache_skip(
                h,
                0 as libc::c_int,
                0 as libc::c_int,
                4 as libc::c_int,
                4 as libc::c_int,
                0 as libc::c_int,
            );
            if b_mbaff != 0 {
                skipbp = (if (*h).mb.i_neighbour & MB_LEFT as libc::c_int as libc::c_uint != 0 {
                    *((*h).mb.skipbp).offset(*left.offset(0 as libc::c_int as isize) as isize)
                        as libc::c_int
                } else {
                    0 as libc::c_int
                }) as uint8_t;
                (*h).mb.cache.skip[(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                    - 1 as libc::c_int) as usize] = ((skipbp as libc::c_int
                    >> (1 as libc::c_int
                        + ((*left_index_table).mv[0 as libc::c_int as usize] as libc::c_int
                            & !(1 as libc::c_int))))
                    & 1 as libc::c_int)
                    as int8_t;
                skipbp = (if (*h).mb.i_neighbour & MB_LEFT as libc::c_int as libc::c_uint != 0 {
                    *((*h).mb.skipbp).offset(*left.offset(1 as libc::c_int as isize) as isize)
                        as libc::c_int
                } else {
                    0 as libc::c_int
                }) as uint8_t;
                (*h).mb.cache.skip[(x264_scan8[8 as libc::c_int as usize] as libc::c_int
                    - 1 as libc::c_int) as usize] = ((skipbp as libc::c_int
                    >> (1 as libc::c_int
                        + ((*left_index_table).mv[2 as libc::c_int as usize] as libc::c_int
                            & !(1 as libc::c_int))))
                    & 1 as libc::c_int)
                    as int8_t;
            } else {
                skipbp = (if (*h).mb.i_neighbour & MB_LEFT as libc::c_int as libc::c_uint != 0 {
                    *((*h).mb.skipbp).offset(*left.offset(0 as libc::c_int as isize) as isize)
                        as libc::c_int
                } else {
                    0 as libc::c_int
                }) as uint8_t;
                (*h).mb.cache.skip[(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                    - 1 as libc::c_int) as usize] =
                    (skipbp as libc::c_int & 0x2 as libc::c_int) as int8_t;
                (*h).mb.cache.skip[(x264_scan8[8 as libc::c_int as usize] as libc::c_int
                    - 1 as libc::c_int) as usize] =
                    (skipbp as libc::c_int & 0x8 as libc::c_int) as int8_t;
            }
            skipbp = (if (*h).mb.i_neighbour & MB_TOP as libc::c_int as libc::c_uint != 0 {
                *((*h).mb.skipbp).offset(top as isize) as libc::c_int
            } else {
                0 as libc::c_int
            }) as uint8_t;
            (*h).mb.cache.skip[(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                - 8 as libc::c_int) as usize] =
                (skipbp as libc::c_int & 0x4 as libc::c_int) as int8_t;
            (*h).mb.cache.skip[(x264_scan8[4 as libc::c_int as usize] as libc::c_int
                - 8 as libc::c_int) as usize] =
                (skipbp as libc::c_int & 0x8 as libc::c_int) as int8_t;
        }
    }
    if (*h).sh.i_type == SLICE_TYPE_P as libc::c_int {
        x264_8_mb_predict_mv_pskip(h, ((*h).mb.cache.pskip_mv).as_mut_ptr());
    }
    (*h).mb.i_neighbour8[0 as libc::c_int as usize] = (*h).mb.i_neighbour_intra
        & (MB_TOP as libc::c_int | MB_LEFT as libc::c_int | MB_TOPLEFT as libc::c_int)
            as libc::c_uint
        | (if (*h).mb.i_neighbour_intra & MB_TOP as libc::c_int as libc::c_uint != 0 {
            MB_TOPRIGHT as libc::c_int
        } else {
            0 as libc::c_int
        }) as libc::c_uint;
    (*h).mb.i_neighbour4[0 as libc::c_int as usize] =
        (*h).mb.i_neighbour8[0 as libc::c_int as usize];
    (*h).mb.i_neighbour4[1 as libc::c_int as usize] = (MB_LEFT as libc::c_int
        | (if (*h).mb.i_neighbour_intra & MB_TOP as libc::c_int as libc::c_uint != 0 {
            MB_TOP as libc::c_int | MB_TOPLEFT as libc::c_int | MB_TOPRIGHT as libc::c_int
        } else {
            0 as libc::c_int
        })) as libc::c_uint;
    (*h).mb.i_neighbour4[4 as libc::c_int as usize] =
        (*h).mb.i_neighbour4[1 as libc::c_int as usize];
    (*h).mb.i_neighbour8[2 as libc::c_int as usize] = (MB_TOP as libc::c_int
        | MB_TOPRIGHT as libc::c_int
        | (if (*h).mb.i_neighbour_intra & MB_LEFT as libc::c_int as libc::c_uint != 0 {
            MB_LEFT as libc::c_int | MB_TOPLEFT as libc::c_int
        } else {
            0 as libc::c_int
        })) as libc::c_uint;
    (*h).mb.i_neighbour4[10 as libc::c_int as usize] =
        (*h).mb.i_neighbour8[2 as libc::c_int as usize];
    (*h).mb.i_neighbour4[8 as libc::c_int as usize] =
        (*h).mb.i_neighbour4[10 as libc::c_int as usize];
    (*h).mb.i_neighbour4[2 as libc::c_int as usize] =
        (*h).mb.i_neighbour4[8 as libc::c_int as usize];
    (*h).mb.i_neighbour8[1 as libc::c_int as usize] = MB_LEFT as libc::c_int as libc::c_uint
        | (*h).mb.i_neighbour_intra & MB_TOPRIGHT as libc::c_int as libc::c_uint
        | (if (*h).mb.i_neighbour_intra & MB_TOP as libc::c_int as libc::c_uint != 0 {
            MB_TOP as libc::c_int | MB_TOPLEFT as libc::c_int
        } else {
            0 as libc::c_int
        }) as libc::c_uint;
    (*h).mb.i_neighbour4[5 as libc::c_int as usize] =
        (*h).mb.i_neighbour8[1 as libc::c_int as usize];
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_macroblock_cache_load_progressive(
    mut h: *mut x264_t,
    mut mb_x: libc::c_int,
    mut mb_y: libc::c_int,
) {
    macroblock_cache_load(h, mb_x, mb_y, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_macroblock_cache_load_interlaced(
    mut h: *mut x264_t,
    mut mb_x: libc::c_int,
    mut mb_y: libc::c_int,
) {
    macroblock_cache_load(h, mb_x, mb_y, 1 as libc::c_int);
}
unsafe extern "C" fn macroblock_deblock_strength_mbaff(
    mut h: *mut x264_t,
    mut bs: *mut [[uint8_t; 4]; 8],
) {
    if (*h).mb.i_neighbour & MB_LEFT as libc::c_int as libc::c_uint != 0
        && *((*h).mb.field).offset((*h).mb.i_mb_left_xy[0 as libc::c_int as usize] as isize)
            as libc::c_int
            != (*h).mb.b_interlaced
    {
        static mut offset: [[[uint8_t; 8]; 2]; 2] = [
            [
                [
                    0 as libc::c_int as uint8_t,
                    0 as libc::c_int as uint8_t,
                    0 as libc::c_int as uint8_t,
                    0 as libc::c_int as uint8_t,
                    1 as libc::c_int as uint8_t,
                    1 as libc::c_int as uint8_t,
                    1 as libc::c_int as uint8_t,
                    1 as libc::c_int as uint8_t,
                ],
                [
                    2 as libc::c_int as uint8_t,
                    2 as libc::c_int as uint8_t,
                    2 as libc::c_int as uint8_t,
                    2 as libc::c_int as uint8_t,
                    3 as libc::c_int as uint8_t,
                    3 as libc::c_int as uint8_t,
                    3 as libc::c_int as uint8_t,
                    3 as libc::c_int as uint8_t,
                ],
            ],
            [
                [
                    0 as libc::c_int as uint8_t,
                    1 as libc::c_int as uint8_t,
                    2 as libc::c_int as uint8_t,
                    3 as libc::c_int as uint8_t,
                    0 as libc::c_int as uint8_t,
                    1 as libc::c_int as uint8_t,
                    2 as libc::c_int as uint8_t,
                    3 as libc::c_int as uint8_t,
                ],
                [
                    0 as libc::c_int as uint8_t,
                    1 as libc::c_int as uint8_t,
                    2 as libc::c_int as uint8_t,
                    3 as libc::c_int as uint8_t,
                    0 as libc::c_int as uint8_t,
                    1 as libc::c_int as uint8_t,
                    2 as libc::c_int as uint8_t,
                    3 as libc::c_int as uint8_t,
                ],
            ],
        ];
        let mut tmpbs: [uint8_t; 8] = [0; 8];
        let mut off: *const uint8_t = (offset[(*h).mb.b_interlaced as usize]
            [((*h).mb.i_mb_y & 1 as libc::c_int) as usize])
            .as_ptr();
        let mut nnz: *mut [uint8_t; 48] = (*h).mb.non_zero_count;
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < 8 as libc::c_int {
            let mut left: libc::c_int = (*h).mb.i_mb_left_xy[(if (*h).mb.b_interlaced != 0 {
                i >> 2 as libc::c_int
            } else {
                i & 1 as libc::c_int
            }) as usize];
            let mut nnz_this: libc::c_int =
                (*h).mb.cache.non_zero_count[(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                    + 8 as libc::c_int * (i >> 1 as libc::c_int))
                    as usize] as libc::c_int;
            let mut nnz_left: libc::c_int = (*nnz.offset(left as isize))[(3 as libc::c_int
                + 4 as libc::c_int * *off.offset(i as isize) as libc::c_int)
                as usize] as libc::c_int;
            if (*h).param.b_cabac == 0 && (*((*h).pps).as_mut_ptr()).b_transform_8x8_mode != 0 {
                let mut j: libc::c_int =
                    *off.offset(i as isize) as libc::c_int & !(1 as libc::c_int);
                if *((*h).mb.mb_transform_size).offset(left as isize) != 0 {
                    nnz_left = ((*(&mut *(*nnz.offset(left as isize))
                        .as_mut_ptr()
                        .offset((2 as libc::c_int + 4 as libc::c_int * j) as isize)
                        as *mut uint8_t as *mut x264_union16_t))
                        .i as libc::c_int
                        | (*(&mut *(*nnz.offset(left as isize)).as_mut_ptr().offset(
                            (2 as libc::c_int + 4 as libc::c_int * (1 as libc::c_int + j)) as isize,
                        ) as *mut uint8_t as *mut x264_union16_t))
                            .i as libc::c_int
                        != 0) as libc::c_int;
                }
            }
            tmpbs[i as usize] = (if nnz_left != 0 || nnz_this != 0 {
                2 as libc::c_int
            } else {
                1 as libc::c_int
            }) as uint8_t;
            i += 1;
            i;
        }
        if (*h).mb.b_interlaced != 0 {
            (*(((*bs.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]).as_mut_ptr()
                as *mut x264_union32_t))
                .i = (*(&mut *tmpbs.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut uint8_t
                as *mut x264_union32_t))
                .i;
            (*(((*bs.offset(0 as libc::c_int as isize))[4 as libc::c_int as usize]).as_mut_ptr()
                as *mut x264_union32_t))
                .i = (*(&mut *tmpbs.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut uint8_t
                as *mut x264_union32_t))
                .i;
        } else {
            let mut i_0: libc::c_int = 0 as libc::c_int;
            while i_0 < 4 as libc::c_int {
                (*bs.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize][i_0 as usize] =
                    tmpbs[(2 as libc::c_int * i_0) as usize];
                i_0 += 1;
                i_0;
            }
            let mut i_1: libc::c_int = 0 as libc::c_int;
            while i_1 < 4 as libc::c_int {
                (*bs.offset(0 as libc::c_int as isize))[4 as libc::c_int as usize][i_1 as usize] =
                    tmpbs[(1 as libc::c_int + 2 as libc::c_int * i_1) as usize];
                i_1 += 1;
                i_1;
            }
        }
    }
    if (*h).mb.i_neighbour & MB_TOP as libc::c_int as libc::c_uint != 0
        && (*h).mb.b_interlaced
            != *((*h).mb.field).offset((*h).mb.i_mb_top_xy as isize) as libc::c_int
    {
        if (*h).mb.i_mb_y & 1 as libc::c_int == 0 && (*h).mb.b_interlaced == 0 {
            let mut mbn_xy: libc::c_int = (*h).mb.i_mb_xy - 2 as libc::c_int * (*h).mb.i_mb_stride;
            let mut nnz_cur: *mut uint8_t = &mut *((*h).mb.cache.non_zero_count)
                .as_mut_ptr()
                .offset(*x264_scan8.as_ptr().offset(0 as libc::c_int as isize) as isize)
                as *mut uint8_t;
            let mut j_0: libc::c_int = 0 as libc::c_int;
            while j_0 < 2 as libc::c_int {
                let mut nnz_0: *mut [uint8_t; 48] = (*h).mb.non_zero_count;
                let mut nnz_top: [uint8_t; 4] = [0; 4];
                (*(nnz_top.as_mut_ptr() as *mut x264_union32_t)).i =
                    (*(&mut *(*nnz_0.offset(mbn_xy as isize))
                        .as_mut_ptr()
                        .offset((3 as libc::c_int * 4 as libc::c_int) as isize)
                        as *mut uint8_t as *mut x264_union32_t))
                        .i;
                if (*h).param.b_cabac == 0
                    && (*((*h).pps).as_mut_ptr()).b_transform_8x8_mode != 0
                    && *((*h).mb.mb_transform_size).offset(mbn_xy as isize) as libc::c_int != 0
                {
                    nnz_top[1 as libc::c_int as usize] =
                        ((*(&mut *(*nnz_0.offset(mbn_xy as isize))
                            .as_mut_ptr()
                            .offset(8 as libc::c_int as isize)
                            as *mut uint8_t as *mut x264_union16_t))
                            .i as libc::c_int
                            != 0
                            || (*(&mut *(*nnz_0.offset(mbn_xy as isize))
                                .as_mut_ptr()
                                .offset(12 as libc::c_int as isize)
                                as *mut uint8_t
                                as *mut x264_union16_t))
                                .i as libc::c_int
                                != 0) as libc::c_int as uint8_t;
                    nnz_top[0 as libc::c_int as usize] = nnz_top[1 as libc::c_int as usize];
                    nnz_top[3 as libc::c_int as usize] =
                        ((*(&mut *(*nnz_0.offset(mbn_xy as isize))
                            .as_mut_ptr()
                            .offset(10 as libc::c_int as isize)
                            as *mut uint8_t as *mut x264_union16_t))
                            .i as libc::c_int
                            != 0
                            || (*(&mut *(*nnz_0.offset(mbn_xy as isize))
                                .as_mut_ptr()
                                .offset(14 as libc::c_int as isize)
                                as *mut uint8_t
                                as *mut x264_union16_t))
                                .i as libc::c_int
                                != 0) as libc::c_int as uint8_t;
                    nnz_top[2 as libc::c_int as usize] = nnz_top[3 as libc::c_int as usize];
                }
                let mut i_2: libc::c_int = 0 as libc::c_int;
                while i_2 < 4 as libc::c_int {
                    (*bs.offset(1 as libc::c_int as isize))[(4 as libc::c_int * j_0) as usize]
                        [i_2 as usize] = (if *nnz_cur.offset(i_2 as isize) as libc::c_int != 0
                        || nnz_top[i_2 as usize] as libc::c_int != 0
                    {
                        2 as libc::c_int
                    } else {
                        1 as libc::c_int
                    }) as uint8_t;
                    i_2 += 1;
                    i_2;
                }
                j_0 += 1;
                j_0;
                mbn_xy += (*h).mb.i_mb_stride;
            }
        } else {
            let mut i_3: libc::c_int = 0 as libc::c_int;
            while i_3 < 4 as libc::c_int {
                (*bs.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize][i_3 as usize] =
                    (if (*bs.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize]
                        [i_3 as usize] as libc::c_int
                        > 1 as libc::c_int
                    {
                        (*bs.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize]
                            [i_3 as usize] as libc::c_int
                    } else {
                        1 as libc::c_int
                    }) as uint8_t;
                i_3 += 1;
                i_3;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_macroblock_deblock_strength(mut h: *mut x264_t) {
    let mut bs: *mut [[uint8_t; 4]; 8] = (*h).mb.cache.deblock_strength;
    if (*h).mb.i_type == I_4x4 as libc::c_int
        || (*h).mb.i_type == I_8x8 as libc::c_int
        || (*h).mb.i_type == I_16x16 as libc::c_int
        || (*h).mb.i_type == I_PCM as libc::c_int
    {
        (*(((*bs.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize]).as_mut_ptr()
            as *mut x264_union32_t))
            .i = 0x3030303 as libc::c_int as uint32_t;
        (*(((*bs.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize]).as_mut_ptr()
            as *mut x264_union64_t))
            .i = 0x303030303030303 as libc::c_ulonglong as uint64_t;
        (*(((*bs.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize]).as_mut_ptr()
            as *mut x264_union32_t))
            .i = 0x3030303 as libc::c_int as uint32_t;
        (*(((*bs.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize]).as_mut_ptr()
            as *mut x264_union64_t))
            .i = 0x303030303030303 as libc::c_ulonglong as uint64_t;
        return;
    }
    if (*h).mb.b_transform_8x8 != 0
        && ((*((*h).sps).as_mut_ptr()).i_chroma_format_idc != CHROMA_444 as libc::c_int)
    {
        let mut cbp_mask: libc::c_int = 0xf as libc::c_int >> (*h).mb.chroma_v_shift;
        if (*h).mb.i_cbp_luma & cbp_mask == cbp_mask {
            (*(((*bs.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]).as_mut_ptr()
                as *mut x264_union32_t))
                .i = 0x2020202 as libc::c_int as uint32_t;
            (*(((*bs.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize]).as_mut_ptr()
                as *mut x264_union32_t))
                .i = 0x2020202 as libc::c_int as uint32_t;
            (*(((*bs.offset(0 as libc::c_int as isize))[4 as libc::c_int as usize]).as_mut_ptr()
                as *mut x264_union32_t))
                .i = 0x2020202 as libc::c_int as uint32_t;
            (*(((*bs.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize]).as_mut_ptr()
                as *mut x264_union64_t))
                .i = 0x202020202020202 as libc::c_ulonglong as uint64_t;
            (*(((*bs.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize]).as_mut_ptr()
                as *mut x264_union64_t))
                .i = 0x202020202020202 as libc::c_ulonglong as uint64_t;
            (*(((*bs.offset(1 as libc::c_int as isize))[4 as libc::c_int as usize]).as_mut_ptr()
                as *mut x264_union32_t))
                .i = 0x2020202 as libc::c_int as uint32_t;
            return;
        }
    }
    let mut neighbour_changed: libc::c_int = 0 as libc::c_int;
    if (*h).sh.i_disable_deblocking_filter_idc != 2 as libc::c_int {
        neighbour_changed = ((*h).mb.i_neighbour_frame & !(*h).mb.i_neighbour) as libc::c_int;
        (*h).mb.i_neighbour = (*h).mb.i_neighbour_frame;
    }
    if (*h).sh.b_mbaff != 0
        && (*h).mb.i_neighbour & MB_LEFT as libc::c_int as libc::c_uint != 0
        && *((*h).mb.field).offset(((*h).mb.i_mb_xy - 1 as libc::c_int) as isize) as libc::c_int
            != (*h).mb.b_interlaced
    {
        (*h).mb.i_mb_left_xy[0 as libc::c_int as usize] = (*h).mb.i_mb_xy - 1 as libc::c_int;
        (*h).mb.i_mb_left_xy[1 as libc::c_int as usize] =
            (*h).mb.i_mb_left_xy[0 as libc::c_int as usize];
        if (*h).mb.i_mb_y & 1 as libc::c_int != 0 {
            (*h).mb.i_mb_left_xy[0 as libc::c_int as usize] -= (*h).mb.i_mb_stride;
        } else {
            (*h).mb.i_mb_left_xy[1 as libc::c_int as usize] += (*h).mb.i_mb_stride;
        }
    }
    if neighbour_changed != 0 {
        let mut top_y: libc::c_int = (*h).mb.i_mb_top_y;
        let mut top_8x8: libc::c_int = (2 as libc::c_int * top_y + 1 as libc::c_int)
            * (*h).mb.i_b8_stride
            + 2 as libc::c_int * (*h).mb.i_mb_x;
        let mut top_4x4: libc::c_int = (4 as libc::c_int * top_y + 3 as libc::c_int)
            * (*h).mb.i_b4_stride
            + 4 as libc::c_int * (*h).mb.i_mb_x;
        let mut s8x8: libc::c_int = (*h).mb.i_b8_stride;
        let mut s4x4: libc::c_int = (*h).mb.i_b4_stride;
        let mut nnz: *mut [uint8_t; 48] = (*h).mb.non_zero_count;
        let mut left_index_table: *const x264_left_table_t = if (*h).sh.b_mbaff != 0 {
            (*h).mb.left_index_table
        } else {
            &*left_indices.as_ptr().offset(3 as libc::c_int as isize) as *const x264_left_table_t
        };
        if neighbour_changed & MB_TOP as libc::c_int != 0 {
            (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
                (*x264_scan8.as_ptr().offset(0 as libc::c_int as isize) as libc::c_int
                    - 8 as libc::c_int) as isize,
            ) as *mut uint8_t as *mut x264_union32_t))
                .i = (*(&mut *(*nnz.offset((*h).mb.i_mb_top_xy as isize))
                .as_mut_ptr()
                .offset(12 as libc::c_int as isize) as *mut uint8_t
                as *mut x264_union32_t))
                .i;
        }
        if neighbour_changed & MB_LEFT as libc::c_int != 0 {
            let mut left: *mut libc::c_int = ((*h).mb.i_mb_left_xy).as_mut_ptr();
            (*h).mb.cache.non_zero_count[(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                - 1 as libc::c_int) as usize] = (*nnz
                .offset(*left.offset(0 as libc::c_int as isize) as isize))
                [(*left_index_table).nnz[0 as libc::c_int as usize] as usize];
            (*h).mb.cache.non_zero_count[(x264_scan8[2 as libc::c_int as usize] as libc::c_int
                - 1 as libc::c_int) as usize] = (*nnz
                .offset(*left.offset(0 as libc::c_int as isize) as isize))
                [(*left_index_table).nnz[1 as libc::c_int as usize] as usize];
            (*h).mb.cache.non_zero_count[(x264_scan8[8 as libc::c_int as usize] as libc::c_int
                - 1 as libc::c_int) as usize] = (*nnz
                .offset(*left.offset(1 as libc::c_int as isize) as isize))
                [(*left_index_table).nnz[2 as libc::c_int as usize] as usize];
            (*h).mb.cache.non_zero_count[(x264_scan8[10 as libc::c_int as usize] as libc::c_int
                - 1 as libc::c_int) as usize] = (*nnz
                .offset(*left.offset(1 as libc::c_int as isize) as isize))
                [(*left_index_table).nnz[3 as libc::c_int as usize] as usize];
        }
        let mut l: libc::c_int = 0 as libc::c_int;
        while l <= ((*h).sh.i_type == SLICE_TYPE_B as libc::c_int) as libc::c_int {
            let mut mv: *mut [int16_t; 2] = (*h).mb.mv[l as usize];
            let mut ref_0: *mut int8_t = (*h).mb.ref_0[l as usize];
            let mut i8: libc::c_int =
                x264_scan8[0 as libc::c_int as usize] as libc::c_int - 8 as libc::c_int;
            if neighbour_changed & MB_TOP as libc::c_int != 0 {
                (*h).mb.cache.ref_0[l as usize][(i8 + 1 as libc::c_int) as usize] =
                    *ref_0.offset((top_8x8 + 0 as libc::c_int) as isize);
                (*h).mb.cache.ref_0[l as usize][(i8 + 0 as libc::c_int) as usize] =
                    (*h).mb.cache.ref_0[l as usize][(i8 + 1 as libc::c_int) as usize];
                (*h).mb.cache.ref_0[l as usize][(i8 + 3 as libc::c_int) as usize] =
                    *ref_0.offset((top_8x8 + 1 as libc::c_int) as isize);
                (*h).mb.cache.ref_0[l as usize][(i8 + 2 as libc::c_int) as usize] =
                    (*h).mb.cache.ref_0[l as usize][(i8 + 3 as libc::c_int) as usize];
                (*(((*h).mb.cache.mv[l as usize][i8 as usize]).as_mut_ptr()
                    as *mut x264_union128_sse_t))
                    .i =
                    (*((*mv.offset(top_4x4 as isize)).as_mut_ptr() as *mut x264_union128_sse_t)).i;
            }
            i8 = x264_scan8[0 as libc::c_int as usize] as libc::c_int - 1 as libc::c_int;
            if neighbour_changed & MB_LEFT as libc::c_int != 0 {
                (*h).mb.cache.ref_0[l as usize]
                    [(i8 + 1 as libc::c_int * 8 as libc::c_int) as usize] = *ref_0.offset(
                    ((*h).mb.left_b8[0 as libc::c_int as usize]
                        + 1 as libc::c_int
                        + s8x8
                            * (*left_index_table).ref_0[0 as libc::c_int as usize] as libc::c_int)
                        as isize,
                );
                (*h).mb.cache.ref_0[l as usize]
                    [(i8 + 0 as libc::c_int * 8 as libc::c_int) as usize] = (*h).mb.cache.ref_0
                    [l as usize][(i8 + 1 as libc::c_int * 8 as libc::c_int) as usize];
                (*h).mb.cache.ref_0[l as usize]
                    [(i8 + 3 as libc::c_int * 8 as libc::c_int) as usize] = *ref_0.offset(
                    ((*h).mb.left_b8[1 as libc::c_int as usize]
                        + 1 as libc::c_int
                        + s8x8
                            * (*left_index_table).ref_0[2 as libc::c_int as usize] as libc::c_int)
                        as isize,
                );
                (*h).mb.cache.ref_0[l as usize]
                    [(i8 + 2 as libc::c_int * 8 as libc::c_int) as usize] = (*h).mb.cache.ref_0
                    [l as usize][(i8 + 3 as libc::c_int * 8 as libc::c_int) as usize];
                (*(((*h).mb.cache.mv[l as usize]
                    [(i8 + 0 as libc::c_int * 8 as libc::c_int) as usize])
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i = (*((*mv.offset(
                    ((*h).mb.left_b4[0 as libc::c_int as usize]
                        + 3 as libc::c_int
                        + s4x4 * (*left_index_table).mv[0 as libc::c_int as usize] as libc::c_int)
                        as isize,
                ))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i;
                (*(((*h).mb.cache.mv[l as usize]
                    [(i8 + 1 as libc::c_int * 8 as libc::c_int) as usize])
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i = (*((*mv.offset(
                    ((*h).mb.left_b4[0 as libc::c_int as usize]
                        + 3 as libc::c_int
                        + s4x4 * (*left_index_table).mv[1 as libc::c_int as usize] as libc::c_int)
                        as isize,
                ))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i;
                (*(((*h).mb.cache.mv[l as usize]
                    [(i8 + 2 as libc::c_int * 8 as libc::c_int) as usize])
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i = (*((*mv.offset(
                    ((*h).mb.left_b4[1 as libc::c_int as usize]
                        + 3 as libc::c_int
                        + s4x4 * (*left_index_table).mv[2 as libc::c_int as usize] as libc::c_int)
                        as isize,
                ))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i;
                (*(((*h).mb.cache.mv[l as usize]
                    [(i8 + 3 as libc::c_int * 8 as libc::c_int) as usize])
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i = (*((*mv.offset(
                    ((*h).mb.left_b4[1 as libc::c_int as usize]
                        + 3 as libc::c_int
                        + s4x4 * (*left_index_table).mv[3 as libc::c_int as usize] as libc::c_int)
                        as isize,
                ))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i;
            }
            l += 1;
            l;
        }
    }
    if (*h).param.analyse.i_weighted_pred == 2 as libc::c_int
        && (*h).sh.i_type == SLICE_TYPE_P as libc::c_int
    {
        let mut i8_0: libc::c_int =
            x264_scan8[0 as libc::c_int as usize] as libc::c_int - 8 as libc::c_int;
        (*h).mb.cache.ref_0[0 as libc::c_int as usize][(i8_0 + 1 as libc::c_int) as usize] =
            (*h).mb.deblock_ref_table[((*h).mb.cache.ref_0[0 as libc::c_int as usize]
                [(i8_0 + 0 as libc::c_int) as usize]
                as libc::c_int
                + 2 as libc::c_int) as usize];
        (*h).mb.cache.ref_0[0 as libc::c_int as usize][(i8_0 + 0 as libc::c_int) as usize] =
            (*h).mb.cache.ref_0[0 as libc::c_int as usize][(i8_0 + 1 as libc::c_int) as usize];
        (*h).mb.cache.ref_0[0 as libc::c_int as usize][(i8_0 + 3 as libc::c_int) as usize] =
            (*h).mb.deblock_ref_table[((*h).mb.cache.ref_0[0 as libc::c_int as usize]
                [(i8_0 + 2 as libc::c_int) as usize]
                as libc::c_int
                + 2 as libc::c_int) as usize];
        (*h).mb.cache.ref_0[0 as libc::c_int as usize][(i8_0 + 2 as libc::c_int) as usize] =
            (*h).mb.cache.ref_0[0 as libc::c_int as usize][(i8_0 + 3 as libc::c_int) as usize];
        i8_0 = x264_scan8[0 as libc::c_int as usize] as libc::c_int - 1 as libc::c_int;
        (*h).mb.cache.ref_0[0 as libc::c_int as usize]
            [(i8_0 + 1 as libc::c_int * 8 as libc::c_int) as usize] = (*h).mb.deblock_ref_table
            [((*h).mb.cache.ref_0[0 as libc::c_int as usize]
                [(i8_0 + 0 as libc::c_int * 8 as libc::c_int) as usize]
                as libc::c_int
                + 2 as libc::c_int) as usize];
        (*h).mb.cache.ref_0[0 as libc::c_int as usize]
            [(i8_0 + 0 as libc::c_int * 8 as libc::c_int) as usize] = (*h).mb.cache.ref_0
            [0 as libc::c_int as usize][(i8_0 + 1 as libc::c_int * 8 as libc::c_int) as usize];
        (*h).mb.cache.ref_0[0 as libc::c_int as usize]
            [(i8_0 + 3 as libc::c_int * 8 as libc::c_int) as usize] = (*h).mb.deblock_ref_table
            [((*h).mb.cache.ref_0[0 as libc::c_int as usize]
                [(i8_0 + 2 as libc::c_int * 8 as libc::c_int) as usize]
                as libc::c_int
                + 2 as libc::c_int) as usize];
        (*h).mb.cache.ref_0[0 as libc::c_int as usize]
            [(i8_0 + 2 as libc::c_int * 8 as libc::c_int) as usize] = (*h).mb.cache.ref_0
            [0 as libc::c_int as usize][(i8_0 + 3 as libc::c_int * 8 as libc::c_int) as usize];
        let mut ref0: libc::c_int = (*h).mb.deblock_ref_table[((*h).mb.cache.ref_0
            [0 as libc::c_int as usize][x264_scan8[0 as libc::c_int as usize] as usize]
            as libc::c_int
            + 2 as libc::c_int)
            as usize] as libc::c_int;
        let mut ref1: libc::c_int = (*h).mb.deblock_ref_table[((*h).mb.cache.ref_0
            [0 as libc::c_int as usize][x264_scan8[4 as libc::c_int as usize] as usize]
            as libc::c_int
            + 2 as libc::c_int)
            as usize] as libc::c_int;
        let mut ref2: libc::c_int = (*h).mb.deblock_ref_table[((*h).mb.cache.ref_0
            [0 as libc::c_int as usize][x264_scan8[8 as libc::c_int as usize] as usize]
            as libc::c_int
            + 2 as libc::c_int)
            as usize] as libc::c_int;
        let mut ref3: libc::c_int = (*h).mb.deblock_ref_table[((*h).mb.cache.ref_0
            [0 as libc::c_int as usize][x264_scan8[12 as libc::c_int as usize] as usize]
            as libc::c_int
            + 2 as libc::c_int)
            as usize] as libc::c_int;
        let mut reftop: uint32_t =
            pack16to32(ref0 as uint8_t as uint32_t, ref1 as uint8_t as uint32_t)
                * 0x101 as libc::c_int as uint32_t;
        let mut refbot: uint32_t =
            pack16to32(ref2 as uint8_t as uint32_t, ref3 as uint8_t as uint32_t)
                * 0x101 as libc::c_int as uint32_t;
        (*(&mut *(*((*h).mb.cache.ref_0)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize))
        .as_mut_ptr()
        .offset(
            (*x264_scan8.as_ptr().offset(0 as libc::c_int as isize) as libc::c_int
                + 8 as libc::c_int * 0 as libc::c_int) as isize,
        ) as *mut int8_t as *mut x264_union32_t))
            .i = reftop;
        (*(&mut *(*((*h).mb.cache.ref_0)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize))
        .as_mut_ptr()
        .offset(
            (*x264_scan8.as_ptr().offset(0 as libc::c_int as isize) as libc::c_int
                + 8 as libc::c_int * 1 as libc::c_int) as isize,
        ) as *mut int8_t as *mut x264_union32_t))
            .i = reftop;
        (*(&mut *(*((*h).mb.cache.ref_0)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize))
        .as_mut_ptr()
        .offset(
            (*x264_scan8.as_ptr().offset(0 as libc::c_int as isize) as libc::c_int
                + 8 as libc::c_int * 2 as libc::c_int) as isize,
        ) as *mut int8_t as *mut x264_union32_t))
            .i = refbot;
        (*(&mut *(*((*h).mb.cache.ref_0)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize))
        .as_mut_ptr()
        .offset(
            (*x264_scan8.as_ptr().offset(0 as libc::c_int as isize) as libc::c_int
                + 8 as libc::c_int * 3 as libc::c_int) as isize,
        ) as *mut int8_t as *mut x264_union32_t))
            .i = refbot;
    }
    if (*h).param.b_cabac == 0 && (*((*h).pps).as_mut_ptr()).b_transform_8x8_mode != 0 {
        let mut nnz_0: *mut [uint8_t; 48] = (*h).mb.non_zero_count;
        let mut top: libc::c_int = (*h).mb.i_mb_top_xy;
        let mut left_0: *mut libc::c_int = ((*h).mb.i_mb_left_xy).as_mut_ptr();
        if (*h).mb.i_neighbour & MB_TOP as libc::c_int as libc::c_uint != 0
            && *((*h).mb.mb_transform_size).offset(top as isize) as libc::c_int != 0
        {
            let mut i8_1: libc::c_int =
                x264_scan8[0 as libc::c_int as usize] as libc::c_int - 8 as libc::c_int;
            let mut nnz_top0: libc::c_int = (*(&mut *(*nnz_0.offset(top as isize))
                .as_mut_ptr()
                .offset(8 as libc::c_int as isize)
                as *mut uint8_t
                as *mut x264_union16_t))
                .i as libc::c_int
                | (*(&mut *(*nnz_0.offset(top as isize))
                    .as_mut_ptr()
                    .offset(12 as libc::c_int as isize) as *mut uint8_t
                    as *mut x264_union16_t))
                    .i as libc::c_int;
            let mut nnz_top1: libc::c_int = (*(&mut *(*nnz_0.offset(top as isize))
                .as_mut_ptr()
                .offset(10 as libc::c_int as isize)
                as *mut uint8_t
                as *mut x264_union16_t))
                .i as libc::c_int
                | (*(&mut *(*nnz_0.offset(top as isize))
                    .as_mut_ptr()
                    .offset(14 as libc::c_int as isize) as *mut uint8_t
                    as *mut x264_union16_t))
                    .i as libc::c_int;
            (*(&mut *((*h).mb.cache.non_zero_count)
                .as_mut_ptr()
                .offset((i8_1 + 0 as libc::c_int) as isize) as *mut uint8_t
                as *mut x264_union16_t))
                .i = (if nnz_top0 != 0 {
                0x101 as libc::c_int
            } else {
                0 as libc::c_int
            }) as uint16_t;
            (*(&mut *((*h).mb.cache.non_zero_count)
                .as_mut_ptr()
                .offset((i8_1 + 2 as libc::c_int) as isize) as *mut uint8_t
                as *mut x264_union16_t))
                .i = (if nnz_top1 != 0 {
                0x101 as libc::c_int
            } else {
                0 as libc::c_int
            }) as uint16_t;
        }
        if (*h).mb.i_neighbour & MB_LEFT as libc::c_int as libc::c_uint != 0 {
            let mut i8_2: libc::c_int =
                x264_scan8[0 as libc::c_int as usize] as libc::c_int - 1 as libc::c_int;
            if *((*h).mb.mb_transform_size)
                .offset(*left_0.offset(0 as libc::c_int as isize) as isize)
                != 0
            {
                let mut nnz_left0: libc::c_int = (*(&mut *(*nnz_0
                    .offset(*left_0.offset(0 as libc::c_int as isize) as isize))
                .as_mut_ptr()
                .offset(2 as libc::c_int as isize)
                    as *mut uint8_t
                    as *mut x264_union16_t))
                    .i as libc::c_int
                    | (*(&mut *(*nnz_0.offset(*left_0.offset(0 as libc::c_int as isize) as isize))
                        .as_mut_ptr()
                        .offset(6 as libc::c_int as isize) as *mut uint8_t
                        as *mut x264_union16_t))
                        .i as libc::c_int;
                (*h).mb.cache.non_zero_count
                    [(i8_2 + 8 as libc::c_int * 0 as libc::c_int) as usize] =
                    (nnz_left0 != 0) as libc::c_int as uint8_t;
                (*h).mb.cache.non_zero_count
                    [(i8_2 + 8 as libc::c_int * 1 as libc::c_int) as usize] =
                    (nnz_left0 != 0) as libc::c_int as uint8_t;
            }
            if *((*h).mb.mb_transform_size)
                .offset(*left_0.offset(1 as libc::c_int as isize) as isize)
                != 0
            {
                let mut nnz_left1: libc::c_int = (*(&mut *(*nnz_0
                    .offset(*left_0.offset(1 as libc::c_int as isize) as isize))
                .as_mut_ptr()
                .offset(10 as libc::c_int as isize)
                    as *mut uint8_t
                    as *mut x264_union16_t))
                    .i as libc::c_int
                    | (*(&mut *(*nnz_0.offset(*left_0.offset(1 as libc::c_int as isize) as isize))
                        .as_mut_ptr()
                        .offset(14 as libc::c_int as isize) as *mut uint8_t
                        as *mut x264_union16_t))
                        .i as libc::c_int;
                (*h).mb.cache.non_zero_count
                    [(i8_2 + 8 as libc::c_int * 2 as libc::c_int) as usize] =
                    (nnz_left1 != 0) as libc::c_int as uint8_t;
                (*h).mb.cache.non_zero_count
                    [(i8_2 + 8 as libc::c_int * 3 as libc::c_int) as usize] =
                    (nnz_left1 != 0) as libc::c_int as uint8_t;
            }
        }
        if (*h).mb.b_transform_8x8 != 0 {
            let mut nnz0: libc::c_int = (*(&mut *((*h).mb.cache.non_zero_count)
                .as_mut_ptr()
                .offset(*x264_scan8.as_ptr().offset(0 as libc::c_int as isize) as isize)
                as *mut uint8_t as *mut x264_union16_t))
                .i as libc::c_int
                | (*(&mut *((*h).mb.cache.non_zero_count)
                    .as_mut_ptr()
                    .offset(*x264_scan8.as_ptr().offset(2 as libc::c_int as isize) as isize)
                    as *mut uint8_t as *mut x264_union16_t))
                    .i as libc::c_int;
            let mut nnz1: libc::c_int = (*(&mut *((*h).mb.cache.non_zero_count)
                .as_mut_ptr()
                .offset(*x264_scan8.as_ptr().offset(4 as libc::c_int as isize) as isize)
                as *mut uint8_t as *mut x264_union16_t))
                .i as libc::c_int
                | (*(&mut *((*h).mb.cache.non_zero_count)
                    .as_mut_ptr()
                    .offset(*x264_scan8.as_ptr().offset(6 as libc::c_int as isize) as isize)
                    as *mut uint8_t as *mut x264_union16_t))
                    .i as libc::c_int;
            let mut nnz2: libc::c_int = (*(&mut *((*h).mb.cache.non_zero_count)
                .as_mut_ptr()
                .offset(*x264_scan8.as_ptr().offset(8 as libc::c_int as isize) as isize)
                as *mut uint8_t as *mut x264_union16_t))
                .i as libc::c_int
                | (*(&mut *((*h).mb.cache.non_zero_count)
                    .as_mut_ptr()
                    .offset(*x264_scan8.as_ptr().offset(10 as libc::c_int as isize) as isize)
                    as *mut uint8_t as *mut x264_union16_t))
                    .i as libc::c_int;
            let mut nnz3: libc::c_int = (*(&mut *((*h).mb.cache.non_zero_count)
                .as_mut_ptr()
                .offset(*x264_scan8.as_ptr().offset(12 as libc::c_int as isize) as isize)
                as *mut uint8_t as *mut x264_union16_t))
                .i as libc::c_int
                | (*(&mut *((*h).mb.cache.non_zero_count)
                    .as_mut_ptr()
                    .offset(*x264_scan8.as_ptr().offset(14 as libc::c_int as isize) as isize)
                    as *mut uint8_t as *mut x264_union16_t))
                    .i as libc::c_int;
            let mut nnztop: uint32_t = pack16to32(
                (nnz0 != 0) as libc::c_int as uint32_t,
                (nnz1 != 0) as libc::c_int as uint32_t,
            ) * 0x101 as libc::c_int as uint32_t;
            let mut nnzbot: uint32_t = pack16to32(
                (nnz2 != 0) as libc::c_int as uint32_t,
                (nnz3 != 0) as libc::c_int as uint32_t,
            ) * 0x101 as libc::c_int as uint32_t;
            (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
                (*x264_scan8.as_ptr().offset(0 as libc::c_int as isize) as libc::c_int
                    + 8 as libc::c_int * 0 as libc::c_int) as isize,
            ) as *mut uint8_t as *mut x264_union32_t))
                .i = nnztop;
            (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
                (*x264_scan8.as_ptr().offset(0 as libc::c_int as isize) as libc::c_int
                    + 8 as libc::c_int * 1 as libc::c_int) as isize,
            ) as *mut uint8_t as *mut x264_union32_t))
                .i = nnztop;
            (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
                (*x264_scan8.as_ptr().offset(0 as libc::c_int as isize) as libc::c_int
                    + 8 as libc::c_int * 2 as libc::c_int) as isize,
            ) as *mut uint8_t as *mut x264_union32_t))
                .i = nnzbot;
            (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
                (*x264_scan8.as_ptr().offset(0 as libc::c_int as isize) as libc::c_int
                    + 8 as libc::c_int * 3 as libc::c_int) as isize,
            ) as *mut uint8_t as *mut x264_union32_t))
                .i = nnzbot;
        }
    }
    ((*h).loopf.deblock_strength).expect("non-null function pointer")(
        ((*h).mb.cache.non_zero_count).as_mut_ptr(),
        ((*h).mb.cache.ref_0).as_mut_ptr(),
        ((*h).mb.cache.mv).as_mut_ptr(),
        bs,
        4 as libc::c_int >> (*h).mb.b_interlaced,
        ((*h).sh.i_type == SLICE_TYPE_B as libc::c_int) as libc::c_int,
    );
    if (*h).sh.b_mbaff != 0 {
        macroblock_deblock_strength_mbaff(h, bs);
    }
}
#[inline(always)]
unsafe extern "C" fn macroblock_store_pic(
    mut h: *mut x264_t,
    mut mb_x: libc::c_int,
    mut mb_y: libc::c_int,
    mut i: libc::c_int,
    mut b_chroma: libc::c_int,
    mut b_mbaff: libc::c_int,
) {
    let mut height: libc::c_int = if b_chroma != 0 {
        16 as libc::c_int >> (*h).mb.chroma_v_shift
    } else {
        16 as libc::c_int
    };
    let mut i_stride: libc::c_int = (*(*h).fdec).i_stride[i as usize];
    let mut i_stride2: libc::c_int =
        i_stride << (b_mbaff != 0 && (*h).mb.b_interlaced != 0) as libc::c_int;
    let mut i_pix_offset: libc::c_int = if b_mbaff != 0 && (*h).mb.b_interlaced != 0 {
        16 as libc::c_int * mb_x
            + height * (mb_y & !(1 as libc::c_int)) * i_stride
            + (mb_y & 1 as libc::c_int) * i_stride
    } else {
        16 as libc::c_int * mb_x + height * mb_y * i_stride
    };
    if b_chroma != 0 {
        ((*h).mc.store_interleave_chroma).expect("non-null function pointer")(
            &mut *(*((*(*h).fdec).plane)
                .as_mut_ptr()
                .offset(1 as libc::c_int as isize))
            .offset(i_pix_offset as isize),
            i_stride2 as intptr_t,
            (*h).mb.pic.p_fdec[1 as libc::c_int as usize],
            (*h).mb.pic.p_fdec[2 as libc::c_int as usize],
            height,
        );
    } else {
        ((*h).mc.copy[PIXEL_16x16 as libc::c_int as usize]).expect("non-null function pointer")(
            &mut *(*((*(*h).fdec).plane).as_mut_ptr().offset(i as isize))
                .offset(i_pix_offset as isize),
            i_stride2 as intptr_t,
            (*h).mb.pic.p_fdec[i as usize],
            32 as libc::c_int as intptr_t,
            16 as libc::c_int,
        );
    };
}
#[inline(always)]
unsafe extern "C" fn macroblock_backup_intra(
    mut h: *mut x264_t,
    mut mb_x: libc::c_int,
    mut mb_y: libc::c_int,
    mut b_mbaff: libc::c_int,
) {
    let mut backup_dst: libc::c_int = if b_mbaff == 0 {
        mb_y & 1 as libc::c_int
    } else if mb_y & 1 as libc::c_int != 0 {
        1 as libc::c_int
    } else if (*h).mb.b_interlaced != 0 {
        0 as libc::c_int
    } else {
        2 as libc::c_int
    };
    memcpy(
        &mut *(*(*((*h).intra_border_backup)
            .as_mut_ptr()
            .offset(backup_dst as isize))
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize))
        .offset((mb_x * 16 as libc::c_int) as isize) as *mut pixel as *mut libc::c_void,
        ((*h).mb.pic.p_fdec[0 as libc::c_int as usize])
            .offset((32 as libc::c_int * 15 as libc::c_int) as isize)
            as *const libc::c_void,
        (16 as libc::c_int * ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int)
            as libc::c_ulong,
    );
    if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as libc::c_int {
        memcpy(
            &mut *(*(*((*h).intra_border_backup)
                .as_mut_ptr()
                .offset(backup_dst as isize))
            .as_mut_ptr()
            .offset(1 as libc::c_int as isize))
            .offset((mb_x * 16 as libc::c_int) as isize) as *mut pixel
                as *mut libc::c_void,
            ((*h).mb.pic.p_fdec[1 as libc::c_int as usize])
                .offset((32 as libc::c_int * 15 as libc::c_int) as isize)
                as *const libc::c_void,
            (16 as libc::c_int * ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int)
                as libc::c_ulong,
        );
        memcpy(
            &mut *(*(*((*h).intra_border_backup)
                .as_mut_ptr()
                .offset(backup_dst as isize))
            .as_mut_ptr()
            .offset(2 as libc::c_int as isize))
            .offset((mb_x * 16 as libc::c_int) as isize) as *mut pixel
                as *mut libc::c_void,
            ((*h).mb.pic.p_fdec[2 as libc::c_int as usize])
                .offset((32 as libc::c_int * 15 as libc::c_int) as isize)
                as *const libc::c_void,
            (16 as libc::c_int * ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int)
                as libc::c_ulong,
        );
    } else if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc != 0 {
        let mut backup_src: libc::c_int =
            (15 as libc::c_int >> (*h).mb.chroma_v_shift) * 32 as libc::c_int;
        memcpy(
            &mut *(*(*((*h).intra_border_backup)
                .as_mut_ptr()
                .offset(backup_dst as isize))
            .as_mut_ptr()
            .offset(1 as libc::c_int as isize))
            .offset((mb_x * 16 as libc::c_int) as isize) as *mut pixel
                as *mut libc::c_void,
            ((*h).mb.pic.p_fdec[1 as libc::c_int as usize]).offset(backup_src as isize)
                as *const libc::c_void,
            (8 as libc::c_int * ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int)
                as libc::c_ulong,
        );
        memcpy(
            &mut *(*(*((*h).intra_border_backup)
                .as_mut_ptr()
                .offset(backup_dst as isize))
            .as_mut_ptr()
            .offset(1 as libc::c_int as isize))
            .offset((mb_x * 16 as libc::c_int + 8 as libc::c_int) as isize)
                as *mut pixel as *mut libc::c_void,
            ((*h).mb.pic.p_fdec[2 as libc::c_int as usize]).offset(backup_src as isize)
                as *const libc::c_void,
            (8 as libc::c_int * ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int)
                as libc::c_ulong,
        );
    }
    if b_mbaff != 0 && mb_y & 1 as libc::c_int != 0 {
        let mut backup_src_0: libc::c_int = (if (*h).mb.b_interlaced != 0 {
            7 as libc::c_int
        } else {
            14 as libc::c_int
        }) * 32 as libc::c_int;
        backup_dst = if (*h).mb.b_interlaced != 0 {
            2 as libc::c_int
        } else {
            0 as libc::c_int
        };
        memcpy(
            &mut *(*(*((*h).intra_border_backup)
                .as_mut_ptr()
                .offset(backup_dst as isize))
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize))
            .offset((mb_x * 16 as libc::c_int) as isize) as *mut pixel
                as *mut libc::c_void,
            ((*h).mb.pic.p_fdec[0 as libc::c_int as usize]).offset(backup_src_0 as isize)
                as *const libc::c_void,
            (16 as libc::c_int * ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int)
                as libc::c_ulong,
        );
        if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as libc::c_int {
            memcpy(
                &mut *(*(*((*h).intra_border_backup)
                    .as_mut_ptr()
                    .offset(backup_dst as isize))
                .as_mut_ptr()
                .offset(1 as libc::c_int as isize))
                .offset((mb_x * 16 as libc::c_int) as isize) as *mut pixel
                    as *mut libc::c_void,
                ((*h).mb.pic.p_fdec[1 as libc::c_int as usize]).offset(backup_src_0 as isize)
                    as *const libc::c_void,
                (16 as libc::c_int
                    * ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int)
                    as libc::c_ulong,
            );
            memcpy(
                &mut *(*(*((*h).intra_border_backup)
                    .as_mut_ptr()
                    .offset(backup_dst as isize))
                .as_mut_ptr()
                .offset(2 as libc::c_int as isize))
                .offset((mb_x * 16 as libc::c_int) as isize) as *mut pixel
                    as *mut libc::c_void,
                ((*h).mb.pic.p_fdec[2 as libc::c_int as usize]).offset(backup_src_0 as isize)
                    as *const libc::c_void,
                (16 as libc::c_int
                    * ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int)
                    as libc::c_ulong,
            );
        } else if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc != 0 {
            if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_420 as libc::c_int {
                backup_src_0 = (if (*h).mb.b_interlaced != 0 {
                    3 as libc::c_int
                } else {
                    6 as libc::c_int
                }) * 32 as libc::c_int;
            }
            memcpy(
                &mut *(*(*((*h).intra_border_backup)
                    .as_mut_ptr()
                    .offset(backup_dst as isize))
                .as_mut_ptr()
                .offset(1 as libc::c_int as isize))
                .offset((mb_x * 16 as libc::c_int) as isize) as *mut pixel
                    as *mut libc::c_void,
                ((*h).mb.pic.p_fdec[1 as libc::c_int as usize]).offset(backup_src_0 as isize)
                    as *const libc::c_void,
                (8 as libc::c_int * ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int)
                    as libc::c_ulong,
            );
            memcpy(
                &mut *(*(*((*h).intra_border_backup)
                    .as_mut_ptr()
                    .offset(backup_dst as isize))
                .as_mut_ptr()
                .offset(1 as libc::c_int as isize))
                .offset((mb_x * 16 as libc::c_int + 8 as libc::c_int) as isize)
                    as *mut pixel as *mut libc::c_void,
                ((*h).mb.pic.p_fdec[2 as libc::c_int as usize]).offset(backup_src_0 as isize)
                    as *const libc::c_void,
                (8 as libc::c_int * ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int)
                    as libc::c_ulong,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_macroblock_cache_save(mut h: *mut x264_t) {
    let i_mb_xy: libc::c_int = (*h).mb.i_mb_xy;
    let i_mb_type: libc::c_int = x264_mb_type_fix[(*h).mb.i_type as usize] as libc::c_int;
    let s8x8: libc::c_int = (*h).mb.i_b8_stride;
    let s4x4: libc::c_int = (*h).mb.i_b4_stride;
    let i_mb_4x4: libc::c_int = (*h).mb.i_b4_xy;
    let i_mb_8x8: libc::c_int = (*h).mb.i_b8_xy;
    let mut i4x4: *mut int8_t =
        (*((*h).mb.intra4x4_pred_mode).offset(i_mb_xy as isize)).as_mut_ptr();
    let mut nnz: *mut uint8_t = (*((*h).mb.non_zero_count).offset(i_mb_xy as isize)).as_mut_ptr();
    if (*h).sh.b_mbaff != 0 {
        macroblock_backup_intra(h, (*h).mb.i_mb_x, (*h).mb.i_mb_y, 1 as libc::c_int);
        macroblock_store_pic(
            h,
            (*h).mb.i_mb_x,
            (*h).mb.i_mb_y,
            0 as libc::c_int,
            0 as libc::c_int,
            1 as libc::c_int,
        );
        if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as libc::c_int {
            macroblock_store_pic(
                h,
                (*h).mb.i_mb_x,
                (*h).mb.i_mb_y,
                1 as libc::c_int,
                0 as libc::c_int,
                1 as libc::c_int,
            );
            macroblock_store_pic(
                h,
                (*h).mb.i_mb_x,
                (*h).mb.i_mb_y,
                2 as libc::c_int,
                0 as libc::c_int,
                1 as libc::c_int,
            );
        } else if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc != 0 {
            macroblock_store_pic(
                h,
                (*h).mb.i_mb_x,
                (*h).mb.i_mb_y,
                1 as libc::c_int,
                1 as libc::c_int,
                1 as libc::c_int,
            );
        }
    } else {
        macroblock_backup_intra(h, (*h).mb.i_mb_x, (*h).mb.i_mb_y, 0 as libc::c_int);
        macroblock_store_pic(
            h,
            (*h).mb.i_mb_x,
            (*h).mb.i_mb_y,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
        );
        if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as libc::c_int {
            macroblock_store_pic(
                h,
                (*h).mb.i_mb_x,
                (*h).mb.i_mb_y,
                1 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
            );
            macroblock_store_pic(
                h,
                (*h).mb.i_mb_x,
                (*h).mb.i_mb_y,
                2 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
            );
        } else if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc != 0 {
            macroblock_store_pic(
                h,
                (*h).mb.i_mb_x,
                (*h).mb.i_mb_y,
                1 as libc::c_int,
                1 as libc::c_int,
                0 as libc::c_int,
            );
        }
    }
    x264_8_prefetch_fenc(h, (*h).fdec, (*h).mb.i_mb_x, (*h).mb.i_mb_y);
    *((*h).mb.type_0).offset(i_mb_xy as isize) = i_mb_type as int8_t;
    *((*h).mb.slice_table).offset(i_mb_xy as isize) = (*h).sh.i_first_mb;
    *((*h).mb.partition).offset(i_mb_xy as isize) = (if i_mb_type == I_4x4 as libc::c_int
        || i_mb_type == I_8x8 as libc::c_int
        || i_mb_type == I_16x16 as libc::c_int
        || i_mb_type == I_PCM as libc::c_int
    {
        D_16x16 as libc::c_int
    } else {
        (*h).mb.i_partition
    }) as uint8_t;
    (*h).mb.i_mb_prev_xy = i_mb_xy;
    if i_mb_type == I_4x4 as libc::c_int {
        (*(&mut *i4x4.offset(0 as libc::c_int as isize) as *mut int8_t as *mut x264_union32_t)).i =
            (*(&mut *((*h).mb.cache.intra4x4_pred_mode)
                .as_mut_ptr()
                .offset(*x264_scan8.as_ptr().offset(10 as libc::c_int as isize) as isize)
                as *mut int8_t as *mut x264_union32_t))
                .i;
        (*(&mut *i4x4.offset(4 as libc::c_int as isize) as *mut int8_t as *mut x264_union32_t)).i =
            pack8to32(
                (*h).mb.cache.intra4x4_pred_mode[x264_scan8[5 as libc::c_int as usize] as usize]
                    as uint32_t,
                (*h).mb.cache.intra4x4_pred_mode[x264_scan8[7 as libc::c_int as usize] as usize]
                    as uint32_t,
                (*h).mb.cache.intra4x4_pred_mode[x264_scan8[13 as libc::c_int as usize] as usize]
                    as uint32_t,
                0 as libc::c_int as uint32_t,
            );
    } else if (*h).param.b_constrained_intra == 0
        || (i_mb_type == I_4x4 as libc::c_int
            || i_mb_type == I_8x8 as libc::c_int
            || i_mb_type == I_16x16 as libc::c_int
            || i_mb_type == I_PCM as libc::c_int)
    {
        (*(i4x4 as *mut x264_union64_t)).i = (I_PRED_4x4_DC as libc::c_int as libc::c_ulonglong)
            .wrapping_mul(0x101010101010101 as libc::c_ulonglong)
            as uint64_t;
    } else {
        (*(i4x4 as *mut x264_union64_t)).i = (-(1 as libc::c_int) as uint8_t as libc::c_ulonglong)
            .wrapping_mul(0x101010101010101 as libc::c_ulonglong)
            as uint64_t;
    }
    if i_mb_type == I_PCM as libc::c_int {
        *((*h).mb.qp).offset(i_mb_xy as isize) = 0 as libc::c_int as int8_t;
        (*h).mb.i_last_dqp = 0 as libc::c_int;
        (*h).mb.i_cbp_chroma =
            if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as libc::c_int {
                0 as libc::c_int
            } else {
                2 as libc::c_int
            };
        (*h).mb.i_cbp_luma = 0xf as libc::c_int;
        *((*h).mb.cbp).offset(i_mb_xy as isize) = (((*h).mb.i_cbp_chroma << 4 as libc::c_int)
            | (*h).mb.i_cbp_luma
            | 0x1700 as libc::c_int) as int16_t;
        (*h).mb.b_transform_8x8 = 0 as libc::c_int;
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < 48 as libc::c_int {
            (*h).mb.cache.non_zero_count[x264_scan8[i as usize] as usize] =
                (if (*h).param.b_cabac != 0 {
                    1 as libc::c_int
                } else {
                    16 as libc::c_int
                }) as uint8_t;
            i += 1;
            i;
        }
    } else {
        if (*h).mb.i_type != I_16x16 as libc::c_int
            && (*h).mb.i_cbp_luma == 0 as libc::c_int
            && (*h).mb.i_cbp_chroma == 0 as libc::c_int
        {
            (*h).mb.i_qp = (*h).mb.i_last_qp;
        }
        *((*h).mb.qp).offset(i_mb_xy as isize) = (*h).mb.i_qp as int8_t;
        (*h).mb.i_last_dqp = (*h).mb.i_qp - (*h).mb.i_last_qp;
        (*h).mb.i_last_qp = (*h).mb.i_qp;
    }
    (*(&mut *nnz.offset((0 as libc::c_int + 0 as libc::c_int * 4 as libc::c_int) as isize)
        as *mut uint8_t as *mut x264_union32_t))
        .i = (*(&mut *((*h).mb.cache.non_zero_count)
        .as_mut_ptr()
        .offset(*x264_scan8.as_ptr().offset(0 as libc::c_int as isize) as isize)
        as *mut uint8_t as *mut x264_union32_t))
        .i;
    (*(&mut *nnz.offset((0 as libc::c_int + 1 as libc::c_int * 4 as libc::c_int) as isize)
        as *mut uint8_t as *mut x264_union32_t))
        .i = (*(&mut *((*h).mb.cache.non_zero_count)
        .as_mut_ptr()
        .offset(*x264_scan8.as_ptr().offset(2 as libc::c_int as isize) as isize)
        as *mut uint8_t as *mut x264_union32_t))
        .i;
    (*(&mut *nnz.offset((0 as libc::c_int + 2 as libc::c_int * 4 as libc::c_int) as isize)
        as *mut uint8_t as *mut x264_union32_t))
        .i = (*(&mut *((*h).mb.cache.non_zero_count)
        .as_mut_ptr()
        .offset(*x264_scan8.as_ptr().offset(8 as libc::c_int as isize) as isize)
        as *mut uint8_t as *mut x264_union32_t))
        .i;
    (*(&mut *nnz.offset((0 as libc::c_int + 3 as libc::c_int * 4 as libc::c_int) as isize)
        as *mut uint8_t as *mut x264_union32_t))
        .i = (*(&mut *((*h).mb.cache.non_zero_count)
        .as_mut_ptr()
        .offset(*x264_scan8.as_ptr().offset(10 as libc::c_int as isize) as isize)
        as *mut uint8_t as *mut x264_union32_t))
        .i;
    (*(&mut *nnz.offset((16 as libc::c_int + 0 as libc::c_int * 4 as libc::c_int) as isize)
        as *mut uint8_t as *mut x264_union32_t))
        .i = (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
        *x264_scan8
            .as_ptr()
            .offset((16 as libc::c_int + 0 as libc::c_int) as isize) as isize,
    ) as *mut uint8_t as *mut x264_union32_t))
        .i;
    (*(&mut *nnz.offset((16 as libc::c_int + 1 as libc::c_int * 4 as libc::c_int) as isize)
        as *mut uint8_t as *mut x264_union32_t))
        .i = (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
        *x264_scan8
            .as_ptr()
            .offset((16 as libc::c_int + 2 as libc::c_int) as isize) as isize,
    ) as *mut uint8_t as *mut x264_union32_t))
        .i;
    (*(&mut *nnz.offset((32 as libc::c_int + 0 as libc::c_int * 4 as libc::c_int) as isize)
        as *mut uint8_t as *mut x264_union32_t))
        .i = (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
        *x264_scan8
            .as_ptr()
            .offset((32 as libc::c_int + 0 as libc::c_int) as isize) as isize,
    ) as *mut uint8_t as *mut x264_union32_t))
        .i;
    (*(&mut *nnz.offset((32 as libc::c_int + 1 as libc::c_int * 4 as libc::c_int) as isize)
        as *mut uint8_t as *mut x264_union32_t))
        .i = (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
        *x264_scan8
            .as_ptr()
            .offset((32 as libc::c_int + 2 as libc::c_int) as isize) as isize,
    ) as *mut uint8_t as *mut x264_union32_t))
        .i;
    if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc >= CHROMA_422 as libc::c_int {
        (*(&mut *nnz.offset((16 as libc::c_int + 2 as libc::c_int * 4 as libc::c_int) as isize)
            as *mut uint8_t as *mut x264_union32_t))
            .i = (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
            *x264_scan8
                .as_ptr()
                .offset((16 as libc::c_int + 8 as libc::c_int) as isize) as isize,
        ) as *mut uint8_t as *mut x264_union32_t))
            .i;
        (*(&mut *nnz.offset((16 as libc::c_int + 3 as libc::c_int * 4 as libc::c_int) as isize)
            as *mut uint8_t as *mut x264_union32_t))
            .i = (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
            *x264_scan8
                .as_ptr()
                .offset((16 as libc::c_int + 10 as libc::c_int) as isize) as isize,
        ) as *mut uint8_t as *mut x264_union32_t))
            .i;
        (*(&mut *nnz.offset((32 as libc::c_int + 2 as libc::c_int * 4 as libc::c_int) as isize)
            as *mut uint8_t as *mut x264_union32_t))
            .i = (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
            *x264_scan8
                .as_ptr()
                .offset((32 as libc::c_int + 8 as libc::c_int) as isize) as isize,
        ) as *mut uint8_t as *mut x264_union32_t))
            .i;
        (*(&mut *nnz.offset((32 as libc::c_int + 3 as libc::c_int * 4 as libc::c_int) as isize)
            as *mut uint8_t as *mut x264_union32_t))
            .i = (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
            *x264_scan8
                .as_ptr()
                .offset((32 as libc::c_int + 10 as libc::c_int) as isize) as isize,
        ) as *mut uint8_t as *mut x264_union32_t))
            .i;
    }
    if (*h).mb.i_cbp_luma == 0 as libc::c_int && (*h).mb.i_type != I_8x8 as libc::c_int {
        (*h).mb.b_transform_8x8 = 0 as libc::c_int;
    }
    *((*h).mb.mb_transform_size).offset(i_mb_xy as isize) = (*h).mb.b_transform_8x8 as int8_t;
    if (*h).sh.i_type != SLICE_TYPE_I as libc::c_int {
        let mut mv0: *mut [int16_t; 2] =
            &mut *(*((*h).mb.mv).as_mut_ptr().offset(0 as libc::c_int as isize))
                .offset(i_mb_4x4 as isize) as *mut [int16_t; 2];
        let mut ref0: *mut int8_t = &mut *(*((*h).mb.ref_0)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize))
        .offset(i_mb_8x8 as isize) as *mut int8_t;
        if !(i_mb_type == I_4x4 as libc::c_int
            || i_mb_type == I_8x8 as libc::c_int
            || i_mb_type == I_16x16 as libc::c_int
            || i_mb_type == I_PCM as libc::c_int)
        {
            *ref0.offset((0 as libc::c_int + 0 as libc::c_int * s8x8) as isize) =
                (*h).mb.cache.ref_0[0 as libc::c_int as usize]
                    [x264_scan8[0 as libc::c_int as usize] as usize];
            *ref0.offset((1 as libc::c_int + 0 as libc::c_int * s8x8) as isize) =
                (*h).mb.cache.ref_0[0 as libc::c_int as usize]
                    [x264_scan8[4 as libc::c_int as usize] as usize];
            *ref0.offset((0 as libc::c_int + 1 as libc::c_int * s8x8) as isize) =
                (*h).mb.cache.ref_0[0 as libc::c_int as usize]
                    [x264_scan8[8 as libc::c_int as usize] as usize];
            *ref0.offset((1 as libc::c_int + 1 as libc::c_int * s8x8) as isize) =
                (*h).mb.cache.ref_0[0 as libc::c_int as usize]
                    [x264_scan8[12 as libc::c_int as usize] as usize];
            (*(&mut *mv0.offset((0 as libc::c_int * s4x4) as isize) as *mut [int16_t; 2]
                as *mut x264_union128_sse_t))
                .i = (*(((*h).mb.cache.mv[0 as libc::c_int as usize][(x264_scan8
                [0 as libc::c_int as usize]
                as libc::c_int
                + 8 as libc::c_int * 0 as libc::c_int)
                as usize])
                .as_mut_ptr() as *mut x264_union128_sse_t))
                .i;
            (*(&mut *mv0.offset((1 as libc::c_int * s4x4) as isize) as *mut [int16_t; 2]
                as *mut x264_union128_sse_t))
                .i = (*(((*h).mb.cache.mv[0 as libc::c_int as usize][(x264_scan8
                [0 as libc::c_int as usize]
                as libc::c_int
                + 8 as libc::c_int * 1 as libc::c_int)
                as usize])
                .as_mut_ptr() as *mut x264_union128_sse_t))
                .i;
            (*(&mut *mv0.offset((2 as libc::c_int * s4x4) as isize) as *mut [int16_t; 2]
                as *mut x264_union128_sse_t))
                .i = (*(((*h).mb.cache.mv[0 as libc::c_int as usize][(x264_scan8
                [0 as libc::c_int as usize]
                as libc::c_int
                + 8 as libc::c_int * 2 as libc::c_int)
                as usize])
                .as_mut_ptr() as *mut x264_union128_sse_t))
                .i;
            (*(&mut *mv0.offset((3 as libc::c_int * s4x4) as isize) as *mut [int16_t; 2]
                as *mut x264_union128_sse_t))
                .i = (*(((*h).mb.cache.mv[0 as libc::c_int as usize][(x264_scan8
                [0 as libc::c_int as usize]
                as libc::c_int
                + 8 as libc::c_int * 3 as libc::c_int)
                as usize])
                .as_mut_ptr() as *mut x264_union128_sse_t))
                .i;
            if (*h).sh.i_type == SLICE_TYPE_B as libc::c_int {
                let mut mv1: *mut [int16_t; 2] =
                    &mut *(*((*h).mb.mv).as_mut_ptr().offset(1 as libc::c_int as isize))
                        .offset(i_mb_4x4 as isize) as *mut [int16_t; 2];
                let mut ref1: *mut int8_t = &mut *(*((*h).mb.ref_0)
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize))
                .offset(i_mb_8x8 as isize)
                    as *mut int8_t;
                *ref1.offset((0 as libc::c_int + 0 as libc::c_int * s8x8) as isize) =
                    (*h).mb.cache.ref_0[1 as libc::c_int as usize]
                        [x264_scan8[0 as libc::c_int as usize] as usize];
                *ref1.offset((1 as libc::c_int + 0 as libc::c_int * s8x8) as isize) =
                    (*h).mb.cache.ref_0[1 as libc::c_int as usize]
                        [x264_scan8[4 as libc::c_int as usize] as usize];
                *ref1.offset((0 as libc::c_int + 1 as libc::c_int * s8x8) as isize) =
                    (*h).mb.cache.ref_0[1 as libc::c_int as usize]
                        [x264_scan8[8 as libc::c_int as usize] as usize];
                *ref1.offset((1 as libc::c_int + 1 as libc::c_int * s8x8) as isize) =
                    (*h).mb.cache.ref_0[1 as libc::c_int as usize]
                        [x264_scan8[12 as libc::c_int as usize] as usize];
                (*(&mut *mv1.offset((0 as libc::c_int * s4x4) as isize) as *mut [int16_t; 2]
                    as *mut x264_union128_sse_t))
                    .i = (*(((*h).mb.cache.mv[1 as libc::c_int as usize][(x264_scan8
                    [0 as libc::c_int as usize]
                    as libc::c_int
                    + 8 as libc::c_int * 0 as libc::c_int)
                    as usize])
                    .as_mut_ptr() as *mut x264_union128_sse_t))
                    .i;
                (*(&mut *mv1.offset((1 as libc::c_int * s4x4) as isize) as *mut [int16_t; 2]
                    as *mut x264_union128_sse_t))
                    .i = (*(((*h).mb.cache.mv[1 as libc::c_int as usize][(x264_scan8
                    [0 as libc::c_int as usize]
                    as libc::c_int
                    + 8 as libc::c_int * 1 as libc::c_int)
                    as usize])
                    .as_mut_ptr() as *mut x264_union128_sse_t))
                    .i;
                (*(&mut *mv1.offset((2 as libc::c_int * s4x4) as isize) as *mut [int16_t; 2]
                    as *mut x264_union128_sse_t))
                    .i = (*(((*h).mb.cache.mv[1 as libc::c_int as usize][(x264_scan8
                    [0 as libc::c_int as usize]
                    as libc::c_int
                    + 8 as libc::c_int * 2 as libc::c_int)
                    as usize])
                    .as_mut_ptr() as *mut x264_union128_sse_t))
                    .i;
                (*(&mut *mv1.offset((3 as libc::c_int * s4x4) as isize) as *mut [int16_t; 2]
                    as *mut x264_union128_sse_t))
                    .i = (*(((*h).mb.cache.mv[1 as libc::c_int as usize][(x264_scan8
                    [0 as libc::c_int as usize]
                    as libc::c_int
                    + 8 as libc::c_int * 3 as libc::c_int)
                    as usize])
                    .as_mut_ptr() as *mut x264_union128_sse_t))
                    .i;
            }
        } else {
            (*(&mut *ref0.offset((0 as libc::c_int * s8x8) as isize) as *mut int8_t
                as *mut x264_union16_t))
                .i =
                (-(1 as libc::c_int) as uint8_t as libc::c_int * 0x101 as libc::c_int) as uint16_t;
            (*(&mut *ref0.offset((1 as libc::c_int * s8x8) as isize) as *mut int8_t
                as *mut x264_union16_t))
                .i =
                (-(1 as libc::c_int) as uint8_t as libc::c_int * 0x101 as libc::c_int) as uint16_t;
            (*(&mut *mv0.offset((0 as libc::c_int * s4x4) as isize) as *mut [int16_t; 2]
                as *mut x264_union128_sse_t))
                .i = _mm_setr_ps(
                0 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
            (*(&mut *mv0.offset((1 as libc::c_int * s4x4) as isize) as *mut [int16_t; 2]
                as *mut x264_union128_sse_t))
                .i = _mm_setr_ps(
                0 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
            (*(&mut *mv0.offset((2 as libc::c_int * s4x4) as isize) as *mut [int16_t; 2]
                as *mut x264_union128_sse_t))
                .i = _mm_setr_ps(
                0 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
            (*(&mut *mv0.offset((3 as libc::c_int * s4x4) as isize) as *mut [int16_t; 2]
                as *mut x264_union128_sse_t))
                .i = _mm_setr_ps(
                0 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
            if (*h).sh.i_type == SLICE_TYPE_B as libc::c_int {
                let mut mv1_0: *mut [int16_t; 2] =
                    &mut *(*((*h).mb.mv).as_mut_ptr().offset(1 as libc::c_int as isize))
                        .offset(i_mb_4x4 as isize) as *mut [int16_t; 2];
                let mut ref1_0: *mut int8_t = &mut *(*((*h).mb.ref_0)
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize))
                .offset(i_mb_8x8 as isize)
                    as *mut int8_t;
                (*(&mut *ref1_0.offset((0 as libc::c_int * s8x8) as isize) as *mut int8_t
                    as *mut x264_union16_t))
                    .i = (-(1 as libc::c_int) as uint8_t as libc::c_int * 0x101 as libc::c_int)
                    as uint16_t;
                (*(&mut *ref1_0.offset((1 as libc::c_int * s8x8) as isize) as *mut int8_t
                    as *mut x264_union16_t))
                    .i = (-(1 as libc::c_int) as uint8_t as libc::c_int * 0x101 as libc::c_int)
                    as uint16_t;
                (*(&mut *mv1_0.offset((0 as libc::c_int * s4x4) as isize) as *mut [int16_t; 2]
                    as *mut x264_union128_sse_t))
                    .i = _mm_setr_ps(
                    0 as libc::c_int as libc::c_float,
                    0 as libc::c_int as libc::c_float,
                    0 as libc::c_int as libc::c_float,
                    0 as libc::c_int as libc::c_float,
                );
                (*(&mut *mv1_0.offset((1 as libc::c_int * s4x4) as isize) as *mut [int16_t; 2]
                    as *mut x264_union128_sse_t))
                    .i = _mm_setr_ps(
                    0 as libc::c_int as libc::c_float,
                    0 as libc::c_int as libc::c_float,
                    0 as libc::c_int as libc::c_float,
                    0 as libc::c_int as libc::c_float,
                );
                (*(&mut *mv1_0.offset((2 as libc::c_int * s4x4) as isize) as *mut [int16_t; 2]
                    as *mut x264_union128_sse_t))
                    .i = _mm_setr_ps(
                    0 as libc::c_int as libc::c_float,
                    0 as libc::c_int as libc::c_float,
                    0 as libc::c_int as libc::c_float,
                    0 as libc::c_int as libc::c_float,
                );
                (*(&mut *mv1_0.offset((3 as libc::c_int * s4x4) as isize) as *mut [int16_t; 2]
                    as *mut x264_union128_sse_t))
                    .i = _mm_setr_ps(
                    0 as libc::c_int as libc::c_float,
                    0 as libc::c_int as libc::c_float,
                    0 as libc::c_int as libc::c_float,
                    0 as libc::c_int as libc::c_float,
                );
            }
        }
    }
    if (*h).param.b_cabac != 0 {
        let mut mvd0: *mut [uint8_t; 2] =
            (*((*h).mb.mvd[0 as libc::c_int as usize]).offset(i_mb_xy as isize)).as_mut_ptr();
        if (i_mb_type == I_4x4 as libc::c_int
            || i_mb_type == I_8x8 as libc::c_int
            || i_mb_type == I_16x16 as libc::c_int
            || i_mb_type == I_PCM as libc::c_int)
            && i_mb_type != I_PCM as libc::c_int
        {
            *((*h).mb.chroma_pred_mode).offset(i_mb_xy as isize) =
                x264_mb_chroma_pred_mode_fix[(*h).mb.i_chroma_pred_mode as usize] as int8_t;
        } else {
            *((*h).mb.chroma_pred_mode).offset(i_mb_xy as isize) =
                I_PRED_CHROMA_DC as libc::c_int as int8_t;
        }
        if (0x3ff30 as libc::c_int >> i_mb_type) & 1 as libc::c_int != 0 {
            (*((*mvd0.offset(0 as libc::c_int as isize)).as_mut_ptr() as *mut x264_union64_t)).i =
                (*(((*h).mb.cache.mvd[0 as libc::c_int as usize]
                    [x264_scan8[10 as libc::c_int as usize] as usize])
                    .as_mut_ptr() as *mut x264_union64_t))
                    .i;
            (*((*mvd0.offset(4 as libc::c_int as isize)).as_mut_ptr() as *mut x264_union16_t)).i =
                (*(((*h).mb.cache.mvd[0 as libc::c_int as usize]
                    [x264_scan8[5 as libc::c_int as usize] as usize])
                    .as_mut_ptr() as *mut x264_union16_t))
                    .i;
            (*((*mvd0.offset(5 as libc::c_int as isize)).as_mut_ptr() as *mut x264_union16_t)).i =
                (*(((*h).mb.cache.mvd[0 as libc::c_int as usize]
                    [x264_scan8[7 as libc::c_int as usize] as usize])
                    .as_mut_ptr() as *mut x264_union16_t))
                    .i;
            (*((*mvd0.offset(6 as libc::c_int as isize)).as_mut_ptr() as *mut x264_union16_t)).i =
                (*(((*h).mb.cache.mvd[0 as libc::c_int as usize]
                    [x264_scan8[13 as libc::c_int as usize] as usize])
                    .as_mut_ptr() as *mut x264_union16_t))
                    .i;
            if (*h).sh.i_type == SLICE_TYPE_B as libc::c_int {
                let mut mvd1: *mut [uint8_t; 2] = (*((*h).mb.mvd[1 as libc::c_int as usize])
                    .offset(i_mb_xy as isize))
                .as_mut_ptr();
                (*((*mvd1.offset(0 as libc::c_int as isize)).as_mut_ptr()
                    as *mut x264_union64_t))
                    .i = (*(((*h).mb.cache.mvd[1 as libc::c_int as usize]
                    [x264_scan8[10 as libc::c_int as usize] as usize])
                    .as_mut_ptr() as *mut x264_union64_t))
                    .i;
                (*((*mvd1.offset(4 as libc::c_int as isize)).as_mut_ptr()
                    as *mut x264_union16_t))
                    .i = (*(((*h).mb.cache.mvd[1 as libc::c_int as usize]
                    [x264_scan8[5 as libc::c_int as usize] as usize])
                    .as_mut_ptr() as *mut x264_union16_t))
                    .i;
                (*((*mvd1.offset(5 as libc::c_int as isize)).as_mut_ptr()
                    as *mut x264_union16_t))
                    .i = (*(((*h).mb.cache.mvd[1 as libc::c_int as usize]
                    [x264_scan8[7 as libc::c_int as usize] as usize])
                    .as_mut_ptr() as *mut x264_union16_t))
                    .i;
                (*((*mvd1.offset(6 as libc::c_int as isize)).as_mut_ptr()
                    as *mut x264_union16_t))
                    .i = (*(((*h).mb.cache.mvd[1 as libc::c_int as usize]
                    [x264_scan8[13 as libc::c_int as usize] as usize])
                    .as_mut_ptr() as *mut x264_union16_t))
                    .i;
            }
        } else {
            (*((*mvd0.offset(0 as libc::c_int as isize)).as_mut_ptr()
                as *mut x264_union128_sse_t))
                .i = _mm_setr_ps(
                0 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
            if (*h).sh.i_type == SLICE_TYPE_B as libc::c_int {
                let mut mvd1_0: *mut [uint8_t; 2] = (*((*h).mb.mvd[1 as libc::c_int as usize])
                    .offset(i_mb_xy as isize))
                .as_mut_ptr();
                (*((*mvd1_0.offset(0 as libc::c_int as isize)).as_mut_ptr()
                    as *mut x264_union128_sse_t))
                    .i = _mm_setr_ps(
                    0 as libc::c_int as libc::c_float,
                    0 as libc::c_int as libc::c_float,
                    0 as libc::c_int as libc::c_float,
                    0 as libc::c_int as libc::c_float,
                );
            }
        }
        if (*h).sh.i_type == SLICE_TYPE_B as libc::c_int {
            if i_mb_type == B_SKIP as libc::c_int || i_mb_type == B_DIRECT as libc::c_int {
                *((*h).mb.skipbp).offset(i_mb_xy as isize) = 0xf as libc::c_int as int8_t;
            } else if i_mb_type == B_8x8 as libc::c_int {
                let mut skipbp: libc::c_int =
                    (((*h).mb.i_sub_partition[0 as libc::c_int as usize] as libc::c_int
                        == D_DIRECT_8x8 as libc::c_int) as libc::c_int)
                        << 0 as libc::c_int;
                skipbp |= (((*h).mb.i_sub_partition[1 as libc::c_int as usize] as libc::c_int
                    == D_DIRECT_8x8 as libc::c_int) as libc::c_int)
                    << 1 as libc::c_int;
                skipbp |= (((*h).mb.i_sub_partition[2 as libc::c_int as usize] as libc::c_int
                    == D_DIRECT_8x8 as libc::c_int) as libc::c_int)
                    << 2 as libc::c_int;
                skipbp |= (((*h).mb.i_sub_partition[3 as libc::c_int as usize] as libc::c_int
                    == D_DIRECT_8x8 as libc::c_int) as libc::c_int)
                    << 3 as libc::c_int;
                *((*h).mb.skipbp).offset(i_mb_xy as isize) = skipbp as int8_t;
            } else {
                *((*h).mb.skipbp).offset(i_mb_xy as isize) = 0 as libc::c_int as int8_t;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_macroblock_bipred_init(mut h: *mut x264_t) {
    let mut mbfield: libc::c_int = 0 as libc::c_int;
    while mbfield <= (*h).sh.b_mbaff {
        let mut field: libc::c_int = 0 as libc::c_int;
        while field <= (*h).sh.b_mbaff {
            let mut i_ref0: libc::c_int = 0 as libc::c_int;
            while i_ref0 < (*h).i_ref[0 as libc::c_int as usize] << mbfield {
                let mut l0: *mut x264_frame_t =
                    (*h).fref[0 as libc::c_int as usize][(i_ref0 >> mbfield) as usize];
                let mut poc0: libc::c_int = (*l0).i_poc
                    + mbfield * (*l0).i_delta_poc[(field ^ i_ref0 & 1 as libc::c_int) as usize];
                let mut i_ref1: libc::c_int = 0 as libc::c_int;
                while i_ref1 < (*h).i_ref[1 as libc::c_int as usize] << mbfield {
                    let mut l1: *mut x264_frame_t =
                        (*h).fref[1 as libc::c_int as usize][(i_ref1 >> mbfield) as usize];
                    let mut cur_poc: libc::c_int =
                        (*(*h).fdec).i_poc + mbfield * (*(*h).fdec).i_delta_poc[field as usize];
                    let mut poc1: libc::c_int = (*l1).i_poc
                        + mbfield * (*l1).i_delta_poc[(field ^ i_ref1 & 1 as libc::c_int) as usize];
                    let mut td: libc::c_int =
                        x264_clip3(poc1 - poc0, -(128 as libc::c_int), 127 as libc::c_int);
                    if td == 0 as libc::c_int {
                        (*h).mb.dist_scale_factor_buf[mbfield as usize][field as usize]
                            [i_ref0 as usize][i_ref1 as usize] = 256 as libc::c_int as int16_t;
                        (*h).mb.bipred_weight_buf[mbfield as usize][field as usize]
                            [i_ref0 as usize][i_ref1 as usize] = 32 as libc::c_int as int8_t;
                    } else {
                        let mut tb: libc::c_int =
                            x264_clip3(cur_poc - poc0, -(128 as libc::c_int), 127 as libc::c_int);
                        let mut tx: libc::c_int =
                            (16384 as libc::c_int + (abs(td) >> 1 as libc::c_int)) / td;
                        let mut dist_scale_factor: libc::c_int = x264_clip3(
                            (tb * tx + 32 as libc::c_int) >> 6 as libc::c_int,
                            -(1024 as libc::c_int),
                            1023 as libc::c_int,
                        );
                        (*h).mb.dist_scale_factor_buf[mbfield as usize][field as usize]
                            [i_ref0 as usize][i_ref1 as usize] = dist_scale_factor as int16_t;
                        dist_scale_factor >>= 2 as libc::c_int;
                        if (*h).param.analyse.b_weighted_bipred != 0
                            && dist_scale_factor >= -(64 as libc::c_int)
                            && dist_scale_factor <= 128 as libc::c_int
                        {
                            (*h).mb.bipred_weight_buf[mbfield as usize][field as usize]
                                [i_ref0 as usize][i_ref1 as usize] =
                                (64 as libc::c_int - dist_scale_factor) as int8_t;
                            if dist_scale_factor >= -(63 as libc::c_int)
                                && dist_scale_factor <= 127 as libc::c_int
                            {
                            } else {
                                __assert_fail(
                                    b"dist_scale_factor >= -63 && dist_scale_factor <= 127\0"
                                        as *const u8
                                        as *const libc::c_char,
                                    b"common/macroblock.c\0" as *const u8 as *const libc::c_char,
                                    1918 as libc::c_int as libc::c_uint,
                                    (*::core::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                                        b"void x264_8_macroblock_bipred_init(x264_t *)\0",
                                    ))
                                    .as_ptr(),
                                );
                            }
                            'c_49035: {
                                if dist_scale_factor >= -(63 as libc::c_int)
                                    && dist_scale_factor <= 127 as libc::c_int
                                {
                                } else {
                                    __assert_fail(
                                        b"dist_scale_factor >= -63 && dist_scale_factor <= 127\0"
                                            as *const u8
                                            as *const libc::c_char,
                                        b"common/macroblock.c\0" as *const u8
                                            as *const libc::c_char,
                                        1918 as libc::c_int as libc::c_uint,
                                        (*::core::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                                            b"void x264_8_macroblock_bipred_init(x264_t *)\0",
                                        ))
                                        .as_ptr(),
                                    );
                                }
                            };
                        } else {
                            (*h).mb.bipred_weight_buf[mbfield as usize][field as usize]
                                [i_ref0 as usize][i_ref1 as usize] = 32 as libc::c_int as int8_t;
                        }
                    }
                    i_ref1 += 1;
                    i_ref1;
                }
                i_ref0 += 1;
                i_ref0;
            }
            field += 1;
            field;
        }
        mbfield += 1;
        mbfield;
    }
}
