use crate::types::*;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
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
        {
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
    };
}
pub unsafe extern "C" fn macroblock_cache_mv_2_1(mut target: *mut libc::c_void, mut val: uint32_t) {
    x264_macroblock_cache_rect(
        target,
        2 as libc::c_int * 4 as libc::c_int,
        1 as libc::c_int,
        4 as libc::c_int,
        val,
    );
}
pub unsafe extern "C" fn macroblock_cache_mv_1_1(mut target: *mut libc::c_void, mut val: uint32_t) {
    x264_macroblock_cache_rect(
        target,
        1 as libc::c_int * 4 as libc::c_int,
        1 as libc::c_int,
        4 as libc::c_int,
        val,
    );
}
pub unsafe extern "C" fn macroblock_cache_mv_2_4(mut target: *mut libc::c_void, mut val: uint32_t) {
    x264_macroblock_cache_rect(
        target,
        2 as libc::c_int * 4 as libc::c_int,
        4 as libc::c_int,
        4 as libc::c_int,
        val,
    );
}
pub unsafe extern "C" fn macroblock_cache_mv_4_4(mut target: *mut libc::c_void, mut val: uint32_t) {
    x264_macroblock_cache_rect(
        target,
        4 as libc::c_int * 4 as libc::c_int,
        4 as libc::c_int,
        4 as libc::c_int,
        val,
    );
}
pub unsafe extern "C" fn macroblock_cache_mv_2_2(mut target: *mut libc::c_void, mut val: uint32_t) {
    x264_macroblock_cache_rect(
        target,
        2 as libc::c_int * 4 as libc::c_int,
        2 as libc::c_int,
        4 as libc::c_int,
        val,
    );
}
pub unsafe extern "C" fn macroblock_cache_mv_1_2(mut target: *mut libc::c_void, mut val: uint32_t) {
    x264_macroblock_cache_rect(
        target,
        1 as libc::c_int * 4 as libc::c_int,
        2 as libc::c_int,
        4 as libc::c_int,
        val,
    );
}
pub unsafe extern "C" fn macroblock_cache_mv_4_2(mut target: *mut libc::c_void, mut val: uint32_t) {
    x264_macroblock_cache_rect(
        target,
        4 as libc::c_int * 4 as libc::c_int,
        2 as libc::c_int,
        4 as libc::c_int,
        val,
    );
}
pub unsafe extern "C" fn macroblock_cache_mvd_4_4(
    mut target: *mut libc::c_void,
    mut val: uint32_t,
) {
    x264_macroblock_cache_rect(
        target,
        4 as libc::c_int * 2 as libc::c_int,
        4 as libc::c_int,
        2 as libc::c_int,
        val,
    );
}
pub unsafe extern "C" fn macroblock_cache_mvd_2_4(
    mut target: *mut libc::c_void,
    mut val: uint32_t,
) {
    x264_macroblock_cache_rect(
        target,
        2 as libc::c_int * 2 as libc::c_int,
        4 as libc::c_int,
        2 as libc::c_int,
        val,
    );
}
pub unsafe extern "C" fn macroblock_cache_mvd_4_2(
    mut target: *mut libc::c_void,
    mut val: uint32_t,
) {
    x264_macroblock_cache_rect(
        target,
        4 as libc::c_int * 2 as libc::c_int,
        2 as libc::c_int,
        2 as libc::c_int,
        val,
    );
}
pub unsafe extern "C" fn macroblock_cache_mvd_2_2(
    mut target: *mut libc::c_void,
    mut val: uint32_t,
) {
    x264_macroblock_cache_rect(
        target,
        2 as libc::c_int * 2 as libc::c_int,
        2 as libc::c_int,
        2 as libc::c_int,
        val,
    );
}
pub unsafe extern "C" fn macroblock_cache_mvd_1_2(
    mut target: *mut libc::c_void,
    mut val: uint32_t,
) {
    x264_macroblock_cache_rect(
        target,
        1 as libc::c_int * 2 as libc::c_int,
        2 as libc::c_int,
        2 as libc::c_int,
        val,
    );
}
pub unsafe extern "C" fn macroblock_cache_mvd_2_1(
    mut target: *mut libc::c_void,
    mut val: uint32_t,
) {
    x264_macroblock_cache_rect(
        target,
        2 as libc::c_int * 2 as libc::c_int,
        1 as libc::c_int,
        2 as libc::c_int,
        val,
    );
}
pub unsafe extern "C" fn macroblock_cache_mvd_1_1(
    mut target: *mut libc::c_void,
    mut val: uint32_t,
) {
    x264_macroblock_cache_rect(
        target,
        1 as libc::c_int * 2 as libc::c_int,
        1 as libc::c_int,
        2 as libc::c_int,
        val,
    );
}
pub unsafe extern "C" fn macroblock_cache_ref_4_4(
    mut target: *mut libc::c_void,
    mut val: uint32_t,
) {
    x264_macroblock_cache_rect(
        target,
        4 as libc::c_int * 1 as libc::c_int,
        4 as libc::c_int,
        1 as libc::c_int,
        val,
    );
}
pub unsafe extern "C" fn macroblock_cache_ref_2_4(
    mut target: *mut libc::c_void,
    mut val: uint32_t,
) {
    x264_macroblock_cache_rect(
        target,
        2 as libc::c_int * 1 as libc::c_int,
        4 as libc::c_int,
        1 as libc::c_int,
        val,
    );
}
pub unsafe extern "C" fn macroblock_cache_ref_2_1(
    mut target: *mut libc::c_void,
    mut val: uint32_t,
) {
    x264_macroblock_cache_rect(
        target,
        2 as libc::c_int * 1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        val,
    );
}
pub unsafe extern "C" fn macroblock_cache_ref_1_2(
    mut target: *mut libc::c_void,
    mut val: uint32_t,
) {
    x264_macroblock_cache_rect(
        target,
        1 as libc::c_int * 1 as libc::c_int,
        2 as libc::c_int,
        1 as libc::c_int,
        val,
    );
}
pub unsafe extern "C" fn macroblock_cache_ref_2_2(
    mut target: *mut libc::c_void,
    mut val: uint32_t,
) {
    x264_macroblock_cache_rect(
        target,
        2 as libc::c_int * 1 as libc::c_int,
        2 as libc::c_int,
        1 as libc::c_int,
        val,
    );
}
pub unsafe extern "C" fn macroblock_cache_ref_1_1(
    mut target: *mut libc::c_void,
    mut val: uint32_t,
) {
    x264_macroblock_cache_rect(
        target,
        1 as libc::c_int * 1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        val,
    );
}
pub unsafe extern "C" fn macroblock_cache_ref_4_2(
    mut target: *mut libc::c_void,
    mut val: uint32_t,
) {
    x264_macroblock_cache_rect(
        target,
        4 as libc::c_int * 1 as libc::c_int,
        2 as libc::c_int,
        1 as libc::c_int,
        val,
    );
}
