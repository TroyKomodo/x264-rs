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
    fn x264_8_mb_predict_mv(
        h: *mut x264_t,
        i_list: libc::c_int,
        idx: libc::c_int,
        i_width: libc::c_int,
        mvp: *mut int16_t,
    );
    fn x264_8_rd_cost_part(
        h: *mut x264_t,
        i_lambda2: libc::c_int,
        i8: libc::c_int,
        i_pixel: libc::c_int,
    ) -> uint64_t;
}
#[inline(always)]
unsafe extern "C" fn x264_predictor_roundclip(
    mut dst: *mut [int16_t; 2],
    mut mvc: *mut [int16_t; 2],
    mut i_mvc: libc::c_int,
    mut mv_limit: *mut [int16_t; 2],
    mut pmv: uint32_t,
) -> libc::c_int {
    let mut cnt: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < i_mvc {
        let mut mx: libc::c_int = ((*mvc.offset(i as isize))[0 as libc::c_int as usize]
            as libc::c_int
            + 2 as libc::c_int)
            >> 2 as libc::c_int;
        let mut my: libc::c_int = ((*mvc.offset(i as isize))[1 as libc::c_int as usize]
            as libc::c_int
            + 2 as libc::c_int)
            >> 2 as libc::c_int;
        let mut mv: uint32_t = pack16to32_mask(mx, my);
        if !(mv == 0 || mv == pmv) {
            (*dst.offset(cnt as isize))[0 as libc::c_int as usize] = x264_clip3(
                mx,
                (*mv_limit.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
                    as libc::c_int,
                (*mv_limit.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize]
                    as libc::c_int,
            ) as int16_t;
            (*dst.offset(cnt as isize))[1 as libc::c_int as usize] = x264_clip3(
                my,
                (*mv_limit.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize]
                    as libc::c_int,
                (*mv_limit.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize]
                    as libc::c_int,
            ) as int16_t;
            cnt += 1;
            cnt;
        }
        i += 1;
        i;
    }
    cnt
}
#[inline(always)]
unsafe extern "C" fn x264_predictor_clip(
    mut dst: *mut [int16_t; 2],
    mut mvc: *mut [int16_t; 2],
    mut i_mvc: libc::c_int,
    mut mv_limit: *mut [int16_t; 2],
    mut pmv: uint32_t,
) -> libc::c_int {
    let mut cnt: libc::c_int = 0 as libc::c_int;
    let mut qpel_limit: [libc::c_int; 4] = [
        ((*mv_limit.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] as libc::c_int)
            << 2 as libc::c_int,
        ((*mv_limit.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize] as libc::c_int)
            << 2 as libc::c_int,
        ((*mv_limit.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] as libc::c_int)
            << 2 as libc::c_int,
        ((*mv_limit.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize] as libc::c_int)
            << 2 as libc::c_int,
    ];
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < i_mvc {
        let mut mv: uint32_t = (*((*mvc.offset(i as isize)).as_mut_ptr() as *mut x264_union32_t)).i;
        let mut mx: libc::c_int =
            (*mvc.offset(i as isize))[0 as libc::c_int as usize] as libc::c_int;
        let mut my: libc::c_int =
            (*mvc.offset(i as isize))[1 as libc::c_int as usize] as libc::c_int;
        if !(mv == 0 || mv == pmv) {
            (*dst.offset(cnt as isize))[0 as libc::c_int as usize] = x264_clip3(
                mx,
                qpel_limit[0 as libc::c_int as usize],
                qpel_limit[2 as libc::c_int as usize],
            ) as int16_t;
            (*dst.offset(cnt as isize))[1 as libc::c_int as usize] = x264_clip3(
                my,
                qpel_limit[1 as libc::c_int as usize],
                qpel_limit[3 as libc::c_int as usize],
            ) as int16_t;
            cnt += 1;
            cnt;
        }
        i += 1;
        i;
    }
    cnt
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
unsafe extern "C" fn x264_predictor_difference(
    mut mvc: *mut [int16_t; 2],
    mut i_mvc: intptr_t,
) -> libc::c_int {
    let mut sum: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while (i as intptr_t) < i_mvc - 1 as libc::c_int as intptr_t {
        sum += abs(
            (*mvc.offset(i as isize))[0 as libc::c_int as usize] as libc::c_int
                - (*mvc.offset((i + 1 as libc::c_int) as isize))[0 as libc::c_int as usize]
                    as libc::c_int,
        ) + abs(
            (*mvc.offset(i as isize))[1 as libc::c_int as usize] as libc::c_int
                - (*mvc.offset((i + 1 as libc::c_int) as isize))[1 as libc::c_int as usize]
                    as libc::c_int,
        );
        i += 1;
        i;
    }
    sum
}
#[inline(always)]
unsafe extern "C" fn pack8to16(mut a: uint32_t, mut b: uint32_t) -> uint32_t {
    a.wrapping_add(b << 8 as libc::c_int)
}
#[inline(always)]
unsafe extern "C" fn pack16to32_mask(mut a: libc::c_int, mut b: libc::c_int) -> uint32_t {
    ((a & 0xffff as libc::c_int) as uint32_t).wrapping_add((b as uint32_t) << 16 as libc::c_int)
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
#[no_mangle]
pub unsafe extern "C" fn x264_8_me_search_ref(
    mut h: *mut x264_t,
    mut m: *mut x264_me_t,
    mut mvc: *mut [int16_t; 2],
    mut i_mvc: libc::c_int,
    mut p_halfpel_thresh: *mut libc::c_int,
) {
    let mut current_block: u64;
    let bw: libc::c_int = x264_pixel_size[(*m).i_pixel as usize].w as libc::c_int;
    let bh: libc::c_int = x264_pixel_size[(*m).i_pixel as usize].h as libc::c_int;
    let i_pixel: libc::c_int = (*m).i_pixel;
    let stride: libc::c_int = (*m).i_stride[0 as libc::c_int as usize];
    let mut i_me_range: libc::c_int = (*h).param.analyse.i_me_range;
    let mut bmx: libc::c_int = 0;
    let mut bmy: libc::c_int = 0;
    let mut bcost: libc::c_int = (1 as libc::c_int) << 28 as libc::c_int;
    let mut bpred_cost: libc::c_int = (1 as libc::c_int) << 28 as libc::c_int;
    let mut omx: libc::c_int = 0;
    let mut omy: libc::c_int = 0;
    let mut pmx: libc::c_int = 0;
    let mut pmy: libc::c_int = 0;
    let mut p_fenc: *mut pixel = (*m).p_fenc[0 as libc::c_int as usize];
    let mut p_fref_w: *mut pixel = (*m).p_fref_w;
    let mut pix: [pixel; 256] = [0; 256];
    let mut mvc_temp: [[int16_t; 2]; 16] = [[0; 2]; 16];
    let mut costs: [libc::c_int; 16] = [0; 16];
    let mut mv_x_min: libc::c_int =
        (*h).mb.mv_limit_fpel[0 as libc::c_int as usize][0 as libc::c_int as usize] as libc::c_int;
    let mut mv_y_min: libc::c_int =
        (*h).mb.mv_limit_fpel[0 as libc::c_int as usize][1 as libc::c_int as usize] as libc::c_int;
    let mut mv_x_max: libc::c_int =
        (*h).mb.mv_limit_fpel[1 as libc::c_int as usize][0 as libc::c_int as usize] as libc::c_int;
    let mut mv_y_max: libc::c_int =
        (*h).mb.mv_limit_fpel[1 as libc::c_int as usize][1 as libc::c_int as usize] as libc::c_int;
    let mut mv_min: uint32_t = ((-mv_x_min as uint32_t) << 16 as libc::c_int)
        | -mv_y_min as uint32_t & 0x7fff as libc::c_int as uint32_t;
    let mut mv_max: uint32_t = ((mv_x_max as uint32_t) << 16 as libc::c_int)
        | mv_y_max as uint32_t & 0x7fff as libc::c_int as uint32_t
        | 0x8000 as libc::c_int as uint32_t;
    let mut pmv: uint32_t = 0;
    let mut bpred_mv: uint32_t = 0 as libc::c_int as uint32_t;
    let mut p_cost_mvx: *const uint16_t =
        ((*m).p_cost_mv).offset(-((*m).mvp[0 as libc::c_int as usize] as libc::c_int as isize));
    let mut p_cost_mvy: *const uint16_t =
        ((*m).p_cost_mv).offset(-((*m).mvp[1 as libc::c_int as usize] as libc::c_int as isize));
    if (*h).mb.i_subpel_refine >= 3 as libc::c_int {
        let mut bpred_mx: libc::c_int = x264_clip3(
            (*m).mvp[0 as libc::c_int as usize] as libc::c_int,
            mv_x_min * 4 as libc::c_int,
            mv_x_max * 4 as libc::c_int,
        );
        let mut bpred_my: libc::c_int = x264_clip3(
            (*m).mvp[1 as libc::c_int as usize] as libc::c_int,
            mv_y_min * 4 as libc::c_int,
            mv_y_max * 4 as libc::c_int,
        );
        pmv = pack16to32_mask(bpred_mx, bpred_my);
        pmx = (bpred_mx + 2 as libc::c_int) >> 2 as libc::c_int;
        pmy = (bpred_my + 2 as libc::c_int) >> 2 as libc::c_int;
        let mut stride2: intptr_t = 16 as libc::c_int as intptr_t;
        let mut src: *mut pixel = ((*h).mc.get_ref).expect("non-null function pointer")(
            pix.as_mut_ptr(),
            &mut stride2,
            ((*m).p_fref).as_mut_ptr(),
            stride as intptr_t,
            bpred_mx,
            bpred_my,
            bw,
            bh,
            &*((*m).weight).offset(0 as libc::c_int as isize),
        );
        bpred_cost = ((*h).pixf.fpelcmp[i_pixel as usize]).expect("non-null function pointer")(
            p_fenc,
            16 as libc::c_int as intptr_t,
            src,
            stride2,
        ) + *p_cost_mvx.offset(bpred_mx as isize) as libc::c_int
            + *p_cost_mvy.offset(bpred_my as isize) as libc::c_int;
        let mut pmv_cost: libc::c_int = bpred_cost;
        if i_mvc > 0 as libc::c_int {
            let mut valid_mvcs: libc::c_int = x264_predictor_clip(
                mvc_temp.as_mut_ptr().offset(2 as libc::c_int as isize),
                mvc,
                i_mvc,
                ((*h).mb.mv_limit_fpel).as_mut_ptr(),
                pmv,
            );
            if valid_mvcs > 0 as libc::c_int {
                let mut i: libc::c_int = 1 as libc::c_int;
                let mut cost: libc::c_int = 0;
                (*((mvc_temp[1 as libc::c_int as usize]).as_mut_ptr() as *mut x264_union32_t)).i =
                    pmv;
                bpred_cost <<= 4 as libc::c_int;
                loop {
                    let mut mx: libc::c_int = mvc_temp[(i + 1 as libc::c_int) as usize]
                        [0 as libc::c_int as usize]
                        as libc::c_int;
                    let mut my: libc::c_int = mvc_temp[(i + 1 as libc::c_int) as usize]
                        [1 as libc::c_int as usize]
                        as libc::c_int;
                    let mut stride2_0: intptr_t = 16 as libc::c_int as intptr_t;
                    let mut src_0: *mut pixel = ((*h).mc.get_ref)
                        .expect("non-null function pointer")(
                        pix.as_mut_ptr(),
                        &mut stride2_0,
                        ((*m).p_fref).as_mut_ptr(),
                        stride as intptr_t,
                        mx,
                        my,
                        bw,
                        bh,
                        &*((*m).weight).offset(0 as libc::c_int as isize),
                    );
                    cost = ((*h).pixf.fpelcmp[i_pixel as usize])
                        .expect("non-null function pointer")(
                        p_fenc,
                        16 as libc::c_int as intptr_t,
                        src_0,
                        stride2_0,
                    ) + *p_cost_mvx.offset(mx as isize) as libc::c_int
                        + *p_cost_mvy.offset(my as isize) as libc::c_int;
                    if (cost << 4 as libc::c_int) + i < bpred_cost {
                        bpred_cost = (cost << 4 as libc::c_int) + i;
                    }
                    i += 1;
                    if i > valid_mvcs {
                        break;
                    }
                }
                bpred_mx = mvc_temp[((bpred_cost & 15 as libc::c_int) + 1 as libc::c_int) as usize]
                    [0 as libc::c_int as usize] as libc::c_int;
                bpred_my = mvc_temp[((bpred_cost & 15 as libc::c_int) + 1 as libc::c_int) as usize]
                    [1 as libc::c_int as usize] as libc::c_int;
                bpred_cost >>= 4 as libc::c_int;
            }
        }
        bmx = (bpred_mx + 2 as libc::c_int) >> 2 as libc::c_int;
        bmy = (bpred_my + 2 as libc::c_int) >> 2 as libc::c_int;
        bpred_mv = pack16to32_mask(bpred_mx, bpred_my);
        if bpred_mv & 0x30003 as libc::c_int as uint32_t != 0 {
            let mut cost_0: libc::c_int =
                ((*h).pixf.fpelcmp[i_pixel as usize]).expect("non-null function pointer")(
                    p_fenc,
                    16 as libc::c_int as intptr_t,
                    &mut *p_fref_w.offset((bmy * stride + bmx) as isize),
                    stride as intptr_t,
                ) + (*p_cost_mvx.offset((bmx * 4 as libc::c_int) as isize) as libc::c_int
                    + *p_cost_mvy.offset((bmy * 4 as libc::c_int) as isize) as libc::c_int);
            if cost_0 < bcost {
                bcost = cost_0;
                bmx = bmx;
                bmy = bmy;
            }
        } else {
            bcost = bpred_cost;
        }
        if pmv != 0 {
            if bmx | bmy != 0 {
                let mut cost_1: libc::c_int = ((*h).pixf.fpelcmp[i_pixel as usize])
                    .expect("non-null function pointer")(
                    p_fenc,
                    16 as libc::c_int as intptr_t,
                    &mut *p_fref_w.offset((0 as libc::c_int * stride + 0 as libc::c_int) as isize),
                    stride as intptr_t,
                ) + (*p_cost_mvx
                    .offset((0 as libc::c_int * 4 as libc::c_int) as isize)
                    as libc::c_int
                    + *p_cost_mvy.offset((0 as libc::c_int * 4 as libc::c_int) as isize)
                        as libc::c_int);
                if cost_1 < bcost {
                    bcost = cost_1;
                    bmx = 0 as libc::c_int;
                    bmy = 0 as libc::c_int;
                }
            }
        } else if pmv_cost < bcost {
            bcost = pmv_cost;
            bmx = 0 as libc::c_int;
            bmy = 0 as libc::c_int;
        }
    } else {
        pmx = x264_clip3(
            ((*m).mvp[0 as libc::c_int as usize] as libc::c_int + 2 as libc::c_int)
                >> 2 as libc::c_int,
            mv_x_min,
            mv_x_max,
        );
        bmx = pmx;
        pmy = x264_clip3(
            ((*m).mvp[1 as libc::c_int as usize] as libc::c_int + 2 as libc::c_int)
                >> 2 as libc::c_int,
            mv_y_min,
            mv_y_max,
        );
        bmy = pmy;
        pmv = pack16to32_mask(bmx, bmy);
        bcost = ((*h).pixf.fpelcmp[i_pixel as usize]).expect("non-null function pointer")(
            p_fenc,
            16 as libc::c_int as intptr_t,
            &mut *p_fref_w.offset((bmy * stride + bmx) as isize),
            stride as intptr_t,
        );
        if i_mvc > 0 as libc::c_int {
            let mut valid_mvcs_0: libc::c_int = x264_predictor_roundclip(
                mvc_temp.as_mut_ptr().offset(2 as libc::c_int as isize),
                mvc,
                i_mvc,
                ((*h).mb.mv_limit_fpel).as_mut_ptr(),
                pmv,
            );
            if valid_mvcs_0 > 0 as libc::c_int {
                let mut i_0: libc::c_int = 1 as libc::c_int;
                let mut cost_2: libc::c_int = 0;
                (*((mvc_temp[1 as libc::c_int as usize]).as_mut_ptr() as *mut x264_union32_t)).i =
                    pmv;
                bcost <<= 4 as libc::c_int;
                loop {
                    let mut mx_0: libc::c_int = mvc_temp[(i_0 + 1 as libc::c_int) as usize]
                        [0 as libc::c_int as usize]
                        as libc::c_int;
                    let mut my_0: libc::c_int = mvc_temp[(i_0 + 1 as libc::c_int) as usize]
                        [1 as libc::c_int as usize]
                        as libc::c_int;
                    cost_2 = ((*h).pixf.fpelcmp[i_pixel as usize])
                        .expect("non-null function pointer")(
                        p_fenc,
                        16 as libc::c_int as intptr_t,
                        &mut *p_fref_w.offset((my_0 * stride + mx_0) as isize),
                        stride as intptr_t,
                    ) + (*p_cost_mvx.offset((mx_0 * 4 as libc::c_int) as isize)
                        as libc::c_int
                        + *p_cost_mvy.offset((my_0 * 4 as libc::c_int) as isize) as libc::c_int);
                    if (cost_2 << 4 as libc::c_int) + i_0 < bcost {
                        bcost = (cost_2 << 4 as libc::c_int) + i_0;
                    }
                    i_0 += 1;
                    if i_0 > valid_mvcs_0 {
                        break;
                    }
                }
                bmx = mvc_temp[((bcost & 15 as libc::c_int) + 1 as libc::c_int) as usize]
                    [0 as libc::c_int as usize] as libc::c_int;
                bmy = mvc_temp[((bcost & 15 as libc::c_int) + 1 as libc::c_int) as usize]
                    [1 as libc::c_int as usize] as libc::c_int;
                bcost >>= 4 as libc::c_int;
            }
        }
        if pmv != 0 {
            let mut cost_3: libc::c_int =
                ((*h).pixf.fpelcmp[i_pixel as usize]).expect("non-null function pointer")(
                    p_fenc,
                    16 as libc::c_int as intptr_t,
                    &mut *p_fref_w.offset((0 as libc::c_int * stride + 0 as libc::c_int) as isize),
                    stride as intptr_t,
                ) + (*p_cost_mvx.offset((0 as libc::c_int * 4 as libc::c_int) as isize)
                    as libc::c_int
                    + *p_cost_mvy.offset((0 as libc::c_int * 4 as libc::c_int) as isize)
                        as libc::c_int);
            if cost_3 < bcost {
                bcost = cost_3;
                bmx = 0 as libc::c_int;
                bmy = 0 as libc::c_int;
            }
        }
    }
    match (*h).mb.i_me_method {
        0 => {
            bcost <<= 4 as libc::c_int;
            let mut i_1: libc::c_int = i_me_range;
            loop {
                let mut pix_base: *mut pixel = p_fref_w
                    .offset(bmx as isize)
                    .offset((bmy * stride) as isize);
                ((*h).pixf.fpelcmp_x4[i_pixel as usize]).expect("non-null function pointer")(
                    p_fenc,
                    pix_base
                        .offset(0 as libc::c_int as isize)
                        .offset((-(1 as libc::c_int) * stride) as isize),
                    pix_base
                        .offset(0 as libc::c_int as isize)
                        .offset((1 as libc::c_int * stride) as isize),
                    pix_base
                        .offset(-(1 as libc::c_int) as isize)
                        .offset((0 as libc::c_int * stride) as isize),
                    pix_base
                        .offset(1 as libc::c_int as isize)
                        .offset((0 as libc::c_int * stride) as isize),
                    stride as intptr_t,
                    costs.as_mut_ptr(),
                );
                costs[0 as libc::c_int as usize] += *p_cost_mvx
                    .offset(((bmx + 0 as libc::c_int) * 4 as libc::c_int) as isize)
                    as libc::c_int
                    + *p_cost_mvy.offset(((bmy + -(1 as libc::c_int)) * 4 as libc::c_int) as isize)
                        as libc::c_int;
                costs[1 as libc::c_int as usize] += *p_cost_mvx
                    .offset(((bmx + 0 as libc::c_int) * 4 as libc::c_int) as isize)
                    as libc::c_int
                    + *p_cost_mvy.offset(((bmy + 1 as libc::c_int) * 4 as libc::c_int) as isize)
                        as libc::c_int;
                costs[2 as libc::c_int as usize] += *p_cost_mvx
                    .offset(((bmx + -(1 as libc::c_int)) * 4 as libc::c_int) as isize)
                    as libc::c_int
                    + *p_cost_mvy.offset(((bmy + 0 as libc::c_int) * 4 as libc::c_int) as isize)
                        as libc::c_int;
                costs[3 as libc::c_int as usize] += *p_cost_mvx
                    .offset(((bmx + 1 as libc::c_int) * 4 as libc::c_int) as isize)
                    as libc::c_int
                    + *p_cost_mvy.offset(((bmy + 0 as libc::c_int) * 4 as libc::c_int) as isize)
                        as libc::c_int;
                if ((costs[0 as libc::c_int as usize] << 4 as libc::c_int) + 1 as libc::c_int)
                    < bcost
                {
                    bcost =
                        (costs[0 as libc::c_int as usize] << 4 as libc::c_int) + 1 as libc::c_int;
                }
                if ((costs[1 as libc::c_int as usize] << 4 as libc::c_int) + 3 as libc::c_int)
                    < bcost
                {
                    bcost =
                        (costs[1 as libc::c_int as usize] << 4 as libc::c_int) + 3 as libc::c_int;
                }
                if ((costs[2 as libc::c_int as usize] << 4 as libc::c_int) + 4 as libc::c_int)
                    < bcost
                {
                    bcost =
                        (costs[2 as libc::c_int as usize] << 4 as libc::c_int) + 4 as libc::c_int;
                }
                if ((costs[3 as libc::c_int as usize] << 4 as libc::c_int) + 12 as libc::c_int)
                    < bcost
                {
                    bcost =
                        (costs[3 as libc::c_int as usize] << 4 as libc::c_int) + 12 as libc::c_int;
                }
                if bcost & 15 as libc::c_int == 0 {
                    break;
                }
                bmx -= ((bcost as uint32_t) << 28 as libc::c_int) as int32_t >> 30 as libc::c_int;
                bmy -= ((bcost as uint32_t) << 30 as libc::c_int) as int32_t >> 30 as libc::c_int;
                bcost &= !(15 as libc::c_int);
                i_1 -= 1;
                if !(i_1 != 0
                    && ((((bmx as uint32_t) << 16 as libc::c_int)
                        | bmy as uint32_t & 0x7fff as libc::c_int as uint32_t)
                        .wrapping_add(mv_min)
                        | mv_max.wrapping_sub(
                            ((bmx as uint32_t) << 16 as libc::c_int)
                                | bmy as uint32_t & 0x7fff as libc::c_int as uint32_t,
                        ))
                        & 0x80004000 as libc::c_uint
                        == 0)
                {
                    break;
                }
            }
            bcost >>= 4 as libc::c_int;
            current_block = 14127502640287082657;
        }
        1 => {
            current_block = 10359272191501572242;
        }
        2 => {
            static mut pixel_size_shift: [uint8_t; 7] = [
                0 as libc::c_int as uint8_t,
                1 as libc::c_int as uint8_t,
                1 as libc::c_int as uint8_t,
                2 as libc::c_int as uint8_t,
                3 as libc::c_int as uint8_t,
                3 as libc::c_int as uint8_t,
                4 as libc::c_int as uint8_t,
            ];
            let mut ucost1: libc::c_int = 0;
            let mut ucost2: libc::c_int = 0;
            let mut cross_start: libc::c_int = 1 as libc::c_int;
            ucost1 = bcost;
            omx = pmx;
            omy = pmy;
            let mut pix_base_5: *mut pixel = p_fref_w
                .offset(omx as isize)
                .offset((omy * stride) as isize);
            ((*h).pixf.fpelcmp_x4[i_pixel as usize]).expect("non-null function pointer")(
                p_fenc,
                pix_base_5
                    .offset(0 as libc::c_int as isize)
                    .offset((-(1 as libc::c_int) * stride) as isize),
                pix_base_5
                    .offset(0 as libc::c_int as isize)
                    .offset((1 as libc::c_int * stride) as isize),
                pix_base_5
                    .offset(-(1 as libc::c_int) as isize)
                    .offset((0 as libc::c_int * stride) as isize),
                pix_base_5
                    .offset(1 as libc::c_int as isize)
                    .offset((0 as libc::c_int * stride) as isize),
                stride as intptr_t,
                costs.as_mut_ptr(),
            );
            costs[0 as libc::c_int as usize] += *p_cost_mvx
                .offset(((omx + 0 as libc::c_int) * 4 as libc::c_int) as isize)
                as libc::c_int
                + *p_cost_mvy.offset(((omy + -(1 as libc::c_int)) * 4 as libc::c_int) as isize)
                    as libc::c_int;
            costs[1 as libc::c_int as usize] += *p_cost_mvx
                .offset(((omx + 0 as libc::c_int) * 4 as libc::c_int) as isize)
                as libc::c_int
                + *p_cost_mvy.offset(((omy + 1 as libc::c_int) * 4 as libc::c_int) as isize)
                    as libc::c_int;
            costs[2 as libc::c_int as usize] += *p_cost_mvx
                .offset(((omx + -(1 as libc::c_int)) * 4 as libc::c_int) as isize)
                as libc::c_int
                + *p_cost_mvy.offset(((omy + 0 as libc::c_int) * 4 as libc::c_int) as isize)
                    as libc::c_int;
            costs[3 as libc::c_int as usize] += *p_cost_mvx
                .offset(((omx + 1 as libc::c_int) * 4 as libc::c_int) as isize)
                as libc::c_int
                + *p_cost_mvy.offset(((omy + 0 as libc::c_int) * 4 as libc::c_int) as isize)
                    as libc::c_int;
            if costs[0 as libc::c_int as usize] < bcost {
                bcost = costs[0 as libc::c_int as usize];
                bmx = omx + 0 as libc::c_int;
                bmy = omy + -(1 as libc::c_int);
            }
            if costs[1 as libc::c_int as usize] < bcost {
                bcost = costs[1 as libc::c_int as usize];
                bmx = omx + 0 as libc::c_int;
                bmy = omy + 1 as libc::c_int;
            }
            if costs[2 as libc::c_int as usize] < bcost {
                bcost = costs[2 as libc::c_int as usize];
                bmx = omx + -(1 as libc::c_int);
                bmy = omy + 0 as libc::c_int;
            }
            if costs[3 as libc::c_int as usize] < bcost {
                bcost = costs[3 as libc::c_int as usize];
                bmx = omx + 1 as libc::c_int;
                bmy = omy + 0 as libc::c_int;
            }
            if pmx | pmy != 0 {
                omx = 0 as libc::c_int;
                omy = 0 as libc::c_int;
                let mut pix_base_6: *mut pixel = p_fref_w
                    .offset(omx as isize)
                    .offset((omy * stride) as isize);
                ((*h).pixf.fpelcmp_x4[i_pixel as usize]).expect("non-null function pointer")(
                    p_fenc,
                    pix_base_6
                        .offset(0 as libc::c_int as isize)
                        .offset((-(1 as libc::c_int) * stride) as isize),
                    pix_base_6
                        .offset(0 as libc::c_int as isize)
                        .offset((1 as libc::c_int * stride) as isize),
                    pix_base_6
                        .offset(-(1 as libc::c_int) as isize)
                        .offset((0 as libc::c_int * stride) as isize),
                    pix_base_6
                        .offset(1 as libc::c_int as isize)
                        .offset((0 as libc::c_int * stride) as isize),
                    stride as intptr_t,
                    costs.as_mut_ptr(),
                );
                costs[0 as libc::c_int as usize] += *p_cost_mvx
                    .offset(((omx + 0 as libc::c_int) * 4 as libc::c_int) as isize)
                    as libc::c_int
                    + *p_cost_mvy.offset(((omy + -(1 as libc::c_int)) * 4 as libc::c_int) as isize)
                        as libc::c_int;
                costs[1 as libc::c_int as usize] += *p_cost_mvx
                    .offset(((omx + 0 as libc::c_int) * 4 as libc::c_int) as isize)
                    as libc::c_int
                    + *p_cost_mvy.offset(((omy + 1 as libc::c_int) * 4 as libc::c_int) as isize)
                        as libc::c_int;
                costs[2 as libc::c_int as usize] += *p_cost_mvx
                    .offset(((omx + -(1 as libc::c_int)) * 4 as libc::c_int) as isize)
                    as libc::c_int
                    + *p_cost_mvy.offset(((omy + 0 as libc::c_int) * 4 as libc::c_int) as isize)
                        as libc::c_int;
                costs[3 as libc::c_int as usize] += *p_cost_mvx
                    .offset(((omx + 1 as libc::c_int) * 4 as libc::c_int) as isize)
                    as libc::c_int
                    + *p_cost_mvy.offset(((omy + 0 as libc::c_int) * 4 as libc::c_int) as isize)
                        as libc::c_int;
                if costs[0 as libc::c_int as usize] < bcost {
                    bcost = costs[0 as libc::c_int as usize];
                    bmx = omx + 0 as libc::c_int;
                    bmy = omy + -(1 as libc::c_int);
                }
                if costs[1 as libc::c_int as usize] < bcost {
                    bcost = costs[1 as libc::c_int as usize];
                    bmx = omx + 0 as libc::c_int;
                    bmy = omy + 1 as libc::c_int;
                }
                if costs[2 as libc::c_int as usize] < bcost {
                    bcost = costs[2 as libc::c_int as usize];
                    bmx = omx + -(1 as libc::c_int);
                    bmy = omy + 0 as libc::c_int;
                }
                if costs[3 as libc::c_int as usize] < bcost {
                    bcost = costs[3 as libc::c_int as usize];
                    bmx = omx + 1 as libc::c_int;
                    bmy = omy + 0 as libc::c_int;
                }
            }
            if i_pixel == PIXEL_4x4 as libc::c_int {
                current_block = 10359272191501572242;
            } else {
                ucost2 = bcost;
                if bmx | bmy != 0 && (bmx - pmx) | (bmy - pmy) != 0 {
                    omx = bmx;
                    omy = bmy;
                    let mut pix_base_7: *mut pixel = p_fref_w
                        .offset(omx as isize)
                        .offset((omy * stride) as isize);
                    ((*h).pixf.fpelcmp_x4[i_pixel as usize]).expect("non-null function pointer")(
                        p_fenc,
                        pix_base_7
                            .offset(0 as libc::c_int as isize)
                            .offset((-(1 as libc::c_int) * stride) as isize),
                        pix_base_7
                            .offset(0 as libc::c_int as isize)
                            .offset((1 as libc::c_int * stride) as isize),
                        pix_base_7
                            .offset(-(1 as libc::c_int) as isize)
                            .offset((0 as libc::c_int * stride) as isize),
                        pix_base_7
                            .offset(1 as libc::c_int as isize)
                            .offset((0 as libc::c_int * stride) as isize),
                        stride as intptr_t,
                        costs.as_mut_ptr(),
                    );
                    costs[0 as libc::c_int as usize] += *p_cost_mvx
                        .offset(((omx + 0 as libc::c_int) * 4 as libc::c_int) as isize)
                        as libc::c_int
                        + *p_cost_mvy
                            .offset(((omy + -(1 as libc::c_int)) * 4 as libc::c_int) as isize)
                            as libc::c_int;
                    costs[1 as libc::c_int as usize] += *p_cost_mvx
                        .offset(((omx + 0 as libc::c_int) * 4 as libc::c_int) as isize)
                        as libc::c_int
                        + *p_cost_mvy.offset(((omy + 1 as libc::c_int) * 4 as libc::c_int) as isize)
                            as libc::c_int;
                    costs[2 as libc::c_int as usize] += *p_cost_mvx
                        .offset(((omx + -(1 as libc::c_int)) * 4 as libc::c_int) as isize)
                        as libc::c_int
                        + *p_cost_mvy.offset(((omy + 0 as libc::c_int) * 4 as libc::c_int) as isize)
                            as libc::c_int;
                    costs[3 as libc::c_int as usize] += *p_cost_mvx
                        .offset(((omx + 1 as libc::c_int) * 4 as libc::c_int) as isize)
                        as libc::c_int
                        + *p_cost_mvy.offset(((omy + 0 as libc::c_int) * 4 as libc::c_int) as isize)
                            as libc::c_int;
                    if costs[0 as libc::c_int as usize] < bcost {
                        bcost = costs[0 as libc::c_int as usize];
                        bmx = omx + 0 as libc::c_int;
                        bmy = omy + -(1 as libc::c_int);
                    }
                    if costs[1 as libc::c_int as usize] < bcost {
                        bcost = costs[1 as libc::c_int as usize];
                        bmx = omx + 0 as libc::c_int;
                        bmy = omy + 1 as libc::c_int;
                    }
                    if costs[2 as libc::c_int as usize] < bcost {
                        bcost = costs[2 as libc::c_int as usize];
                        bmx = omx + -(1 as libc::c_int);
                        bmy = omy + 0 as libc::c_int;
                    }
                    if costs[3 as libc::c_int as usize] < bcost {
                        bcost = costs[3 as libc::c_int as usize];
                        bmx = omx + 1 as libc::c_int;
                        bmy = omy + 0 as libc::c_int;
                    }
                }
                if bcost == ucost2 {
                    cross_start = 3 as libc::c_int;
                }
                omx = bmx;
                omy = bmy;
                if bcost == ucost2
                    && bcost
                        < 2000 as libc::c_int >> pixel_size_shift[i_pixel as usize] as libc::c_int
                {
                    let mut pix_base_8: *mut pixel = p_fref_w
                        .offset(omx as isize)
                        .offset((omy * stride) as isize);
                    ((*h).pixf.fpelcmp_x4[i_pixel as usize]).expect("non-null function pointer")(
                        p_fenc,
                        pix_base_8
                            .offset(0 as libc::c_int as isize)
                            .offset((-(2 as libc::c_int) * stride) as isize),
                        pix_base_8
                            .offset(-(1 as libc::c_int) as isize)
                            .offset((-(1 as libc::c_int) * stride) as isize),
                        pix_base_8
                            .offset(1 as libc::c_int as isize)
                            .offset((-(1 as libc::c_int) * stride) as isize),
                        pix_base_8
                            .offset(-(2 as libc::c_int) as isize)
                            .offset((0 as libc::c_int * stride) as isize),
                        stride as intptr_t,
                        costs.as_mut_ptr(),
                    );
                    costs[0 as libc::c_int as usize] += *p_cost_mvx
                        .offset(((omx + 0 as libc::c_int) * 4 as libc::c_int) as isize)
                        as libc::c_int
                        + *p_cost_mvy
                            .offset(((omy + -(2 as libc::c_int)) * 4 as libc::c_int) as isize)
                            as libc::c_int;
                    costs[1 as libc::c_int as usize] += *p_cost_mvx
                        .offset(((omx + -(1 as libc::c_int)) * 4 as libc::c_int) as isize)
                        as libc::c_int
                        + *p_cost_mvy
                            .offset(((omy + -(1 as libc::c_int)) * 4 as libc::c_int) as isize)
                            as libc::c_int;
                    costs[2 as libc::c_int as usize] += *p_cost_mvx
                        .offset(((omx + 1 as libc::c_int) * 4 as libc::c_int) as isize)
                        as libc::c_int
                        + *p_cost_mvy
                            .offset(((omy + -(1 as libc::c_int)) * 4 as libc::c_int) as isize)
                            as libc::c_int;
                    costs[3 as libc::c_int as usize] += *p_cost_mvx
                        .offset(((omx + -(2 as libc::c_int)) * 4 as libc::c_int) as isize)
                        as libc::c_int
                        + *p_cost_mvy.offset(((omy + 0 as libc::c_int) * 4 as libc::c_int) as isize)
                            as libc::c_int;
                    if costs[0 as libc::c_int as usize] < bcost {
                        bcost = costs[0 as libc::c_int as usize];
                        bmx = omx + 0 as libc::c_int;
                        bmy = omy + -(2 as libc::c_int);
                    }
                    if costs[1 as libc::c_int as usize] < bcost {
                        bcost = costs[1 as libc::c_int as usize];
                        bmx = omx + -(1 as libc::c_int);
                        bmy = omy + -(1 as libc::c_int);
                    }
                    if costs[2 as libc::c_int as usize] < bcost {
                        bcost = costs[2 as libc::c_int as usize];
                        bmx = omx + 1 as libc::c_int;
                        bmy = omy + -(1 as libc::c_int);
                    }
                    if costs[3 as libc::c_int as usize] < bcost {
                        bcost = costs[3 as libc::c_int as usize];
                        bmx = omx + -(2 as libc::c_int);
                        bmy = omy + 0 as libc::c_int;
                    }
                    let mut pix_base_9: *mut pixel = p_fref_w
                        .offset(omx as isize)
                        .offset((omy * stride) as isize);
                    ((*h).pixf.fpelcmp_x4[i_pixel as usize]).expect("non-null function pointer")(
                        p_fenc,
                        pix_base_9
                            .offset(2 as libc::c_int as isize)
                            .offset((0 as libc::c_int * stride) as isize),
                        pix_base_9
                            .offset(-(1 as libc::c_int) as isize)
                            .offset((1 as libc::c_int * stride) as isize),
                        pix_base_9
                            .offset(1 as libc::c_int as isize)
                            .offset((1 as libc::c_int * stride) as isize),
                        pix_base_9
                            .offset(0 as libc::c_int as isize)
                            .offset((2 as libc::c_int * stride) as isize),
                        stride as intptr_t,
                        costs.as_mut_ptr(),
                    );
                    costs[0 as libc::c_int as usize] += *p_cost_mvx
                        .offset(((omx + 2 as libc::c_int) * 4 as libc::c_int) as isize)
                        as libc::c_int
                        + *p_cost_mvy.offset(((omy + 0 as libc::c_int) * 4 as libc::c_int) as isize)
                            as libc::c_int;
                    costs[1 as libc::c_int as usize] += *p_cost_mvx
                        .offset(((omx + -(1 as libc::c_int)) * 4 as libc::c_int) as isize)
                        as libc::c_int
                        + *p_cost_mvy.offset(((omy + 1 as libc::c_int) * 4 as libc::c_int) as isize)
                            as libc::c_int;
                    costs[2 as libc::c_int as usize] += *p_cost_mvx
                        .offset(((omx + 1 as libc::c_int) * 4 as libc::c_int) as isize)
                        as libc::c_int
                        + *p_cost_mvy.offset(((omy + 1 as libc::c_int) * 4 as libc::c_int) as isize)
                            as libc::c_int;
                    costs[3 as libc::c_int as usize] += *p_cost_mvx
                        .offset(((omx + 0 as libc::c_int) * 4 as libc::c_int) as isize)
                        as libc::c_int
                        + *p_cost_mvy.offset(((omy + 2 as libc::c_int) * 4 as libc::c_int) as isize)
                            as libc::c_int;
                    if costs[0 as libc::c_int as usize] < bcost {
                        bcost = costs[0 as libc::c_int as usize];
                        bmx = omx + 2 as libc::c_int;
                        bmy = omy + 0 as libc::c_int;
                    }
                    if costs[1 as libc::c_int as usize] < bcost {
                        bcost = costs[1 as libc::c_int as usize];
                        bmx = omx + -(1 as libc::c_int);
                        bmy = omy + 1 as libc::c_int;
                    }
                    if costs[2 as libc::c_int as usize] < bcost {
                        bcost = costs[2 as libc::c_int as usize];
                        bmx = omx + 1 as libc::c_int;
                        bmy = omy + 1 as libc::c_int;
                    }
                    if costs[3 as libc::c_int as usize] < bcost {
                        bcost = costs[3 as libc::c_int as usize];
                        bmx = omx + 0 as libc::c_int;
                        bmy = omy + 2 as libc::c_int;
                    }
                    if bcost == ucost1
                        && bcost
                            < 500 as libc::c_int
                                >> pixel_size_shift[i_pixel as usize] as libc::c_int
                    {
                        current_block = 14127502640287082657;
                    } else if bcost == ucost2 {
                        let mut range: libc::c_int =
                            (i_me_range >> 1 as libc::c_int) | 1 as libc::c_int;
                        let mut i_3: libc::c_int = 3 as libc::c_int;
                        if range
                            <= (if mv_x_max - omx < omx - mv_x_min {
                                mv_x_max - omx
                            } else {
                                omx - mv_x_min
                            })
                        {
                            while i_3 < range - 2 as libc::c_int {
                                let mut pix_base_10: *mut pixel = p_fref_w
                                    .offset(omx as isize)
                                    .offset((omy * stride) as isize);
                                ((*h).pixf.fpelcmp_x4[i_pixel as usize])
                                    .expect("non-null function pointer")(
                                    p_fenc,
                                    pix_base_10
                                        .offset(i_3 as isize)
                                        .offset((0 as libc::c_int * stride) as isize),
                                    pix_base_10
                                        .offset(-i_3 as isize)
                                        .offset((0 as libc::c_int * stride) as isize),
                                    pix_base_10
                                        .offset((i_3 + 2 as libc::c_int) as isize)
                                        .offset((0 as libc::c_int * stride) as isize),
                                    pix_base_10
                                        .offset((-i_3 - 2 as libc::c_int) as isize)
                                        .offset((0 as libc::c_int * stride) as isize),
                                    stride as intptr_t,
                                    costs.as_mut_ptr(),
                                );
                                costs[0 as libc::c_int as usize] += *p_cost_mvx
                                    .offset(((omx + i_3) * 4 as libc::c_int) as isize)
                                    as libc::c_int
                                    + *p_cost_mvy.offset(
                                        ((omy + 0 as libc::c_int) * 4 as libc::c_int) as isize,
                                    ) as libc::c_int;
                                costs[1 as libc::c_int as usize] += *p_cost_mvx
                                    .offset(((omx + -i_3) * 4 as libc::c_int) as isize)
                                    as libc::c_int
                                    + *p_cost_mvy.offset(
                                        ((omy + 0 as libc::c_int) * 4 as libc::c_int) as isize,
                                    ) as libc::c_int;
                                costs[2 as libc::c_int as usize] += *p_cost_mvx.offset(
                                    ((omx + (i_3 + 2 as libc::c_int)) * 4 as libc::c_int) as isize,
                                )
                                    as libc::c_int
                                    + *p_cost_mvy.offset(
                                        ((omy + 0 as libc::c_int) * 4 as libc::c_int) as isize,
                                    ) as libc::c_int;
                                costs[3 as libc::c_int as usize] += *p_cost_mvx.offset(
                                    ((omx + (-i_3 - 2 as libc::c_int)) * 4 as libc::c_int) as isize,
                                )
                                    as libc::c_int
                                    + *p_cost_mvy.offset(
                                        ((omy + 0 as libc::c_int) * 4 as libc::c_int) as isize,
                                    ) as libc::c_int;
                                if costs[0 as libc::c_int as usize] < bcost {
                                    bcost = costs[0 as libc::c_int as usize];
                                    bmx = omx + i_3;
                                    bmy = omy + 0 as libc::c_int;
                                }
                                if costs[1 as libc::c_int as usize] < bcost {
                                    bcost = costs[1 as libc::c_int as usize];
                                    bmx = omx + -i_3;
                                    bmy = omy + 0 as libc::c_int;
                                }
                                if costs[2 as libc::c_int as usize] < bcost {
                                    bcost = costs[2 as libc::c_int as usize];
                                    bmx = omx + (i_3 + 2 as libc::c_int);
                                    bmy = omy + 0 as libc::c_int;
                                }
                                if costs[3 as libc::c_int as usize] < bcost {
                                    bcost = costs[3 as libc::c_int as usize];
                                    bmx = omx + (-i_3 - 2 as libc::c_int);
                                    bmy = omy + 0 as libc::c_int;
                                }
                                i_3 += 4 as libc::c_int;
                            }
                        }
                        while i_3 < range {
                            if omx + i_3 <= mv_x_max {
                                let mut cost_4: libc::c_int = ((*h).pixf.fpelcmp[i_pixel as usize])
                                    .expect("non-null function pointer")(
                                    p_fenc,
                                    16 as libc::c_int as intptr_t,
                                    &mut *p_fref_w.offset((omy * stride + (omx + i_3)) as isize),
                                    stride as intptr_t,
                                ) + (*p_cost_mvx
                                    .offset(((omx + i_3) * 4 as libc::c_int) as isize)
                                    as libc::c_int
                                    + *p_cost_mvy.offset((omy * 4 as libc::c_int) as isize)
                                        as libc::c_int);
                                if cost_4 < bcost {
                                    bcost = cost_4;
                                    bmx = omx + i_3;
                                    bmy = omy;
                                }
                            }
                            if omx - i_3 >= mv_x_min {
                                let mut cost_5: libc::c_int = ((*h).pixf.fpelcmp[i_pixel as usize])
                                    .expect("non-null function pointer")(
                                    p_fenc,
                                    16 as libc::c_int as intptr_t,
                                    &mut *p_fref_w.offset((omy * stride + (omx - i_3)) as isize),
                                    stride as intptr_t,
                                ) + (*p_cost_mvx
                                    .offset(((omx - i_3) * 4 as libc::c_int) as isize)
                                    as libc::c_int
                                    + *p_cost_mvy.offset((omy * 4 as libc::c_int) as isize)
                                        as libc::c_int);
                                if cost_5 < bcost {
                                    bcost = cost_5;
                                    bmx = omx - i_3;
                                    bmy = omy;
                                }
                            }
                            i_3 += 2 as libc::c_int;
                        }
                        i_3 = 3 as libc::c_int;
                        if range
                            <= (if mv_y_max - omy < omy - mv_y_min {
                                mv_y_max - omy
                            } else {
                                omy - mv_y_min
                            })
                        {
                            while i_3 < range - 2 as libc::c_int {
                                let mut pix_base_11: *mut pixel = p_fref_w
                                    .offset(omx as isize)
                                    .offset((omy * stride) as isize);
                                ((*h).pixf.fpelcmp_x4[i_pixel as usize])
                                    .expect("non-null function pointer")(
                                    p_fenc,
                                    pix_base_11
                                        .offset(0 as libc::c_int as isize)
                                        .offset((i_3 * stride) as isize),
                                    pix_base_11
                                        .offset(0 as libc::c_int as isize)
                                        .offset((-i_3 * stride) as isize),
                                    pix_base_11
                                        .offset(0 as libc::c_int as isize)
                                        .offset(((i_3 + 2 as libc::c_int) * stride) as isize),
                                    pix_base_11
                                        .offset(0 as libc::c_int as isize)
                                        .offset(((-i_3 - 2 as libc::c_int) * stride) as isize),
                                    stride as intptr_t,
                                    costs.as_mut_ptr(),
                                );
                                costs[0 as libc::c_int as usize] += *p_cost_mvx
                                    .offset(((omx + 0 as libc::c_int) * 4 as libc::c_int) as isize)
                                    as libc::c_int
                                    + *p_cost_mvy.offset(((omy + i_3) * 4 as libc::c_int) as isize)
                                        as libc::c_int;
                                costs[1 as libc::c_int as usize] += *p_cost_mvx
                                    .offset(((omx + 0 as libc::c_int) * 4 as libc::c_int) as isize)
                                    as libc::c_int
                                    + *p_cost_mvy.offset(((omy + -i_3) * 4 as libc::c_int) as isize)
                                        as libc::c_int;
                                costs[2 as libc::c_int as usize] += *p_cost_mvx
                                    .offset(((omx + 0 as libc::c_int) * 4 as libc::c_int) as isize)
                                    as libc::c_int
                                    + *p_cost_mvy.offset(
                                        ((omy + (i_3 + 2 as libc::c_int)) * 4 as libc::c_int)
                                            as isize,
                                    ) as libc::c_int;
                                costs[3 as libc::c_int as usize] += *p_cost_mvx
                                    .offset(((omx + 0 as libc::c_int) * 4 as libc::c_int) as isize)
                                    as libc::c_int
                                    + *p_cost_mvy.offset(
                                        ((omy + (-i_3 - 2 as libc::c_int)) * 4 as libc::c_int)
                                            as isize,
                                    ) as libc::c_int;
                                if costs[0 as libc::c_int as usize] < bcost {
                                    bcost = costs[0 as libc::c_int as usize];
                                    bmx = omx + 0 as libc::c_int;
                                    bmy = omy + i_3;
                                }
                                if costs[1 as libc::c_int as usize] < bcost {
                                    bcost = costs[1 as libc::c_int as usize];
                                    bmx = omx + 0 as libc::c_int;
                                    bmy = omy + -i_3;
                                }
                                if costs[2 as libc::c_int as usize] < bcost {
                                    bcost = costs[2 as libc::c_int as usize];
                                    bmx = omx + 0 as libc::c_int;
                                    bmy = omy + (i_3 + 2 as libc::c_int);
                                }
                                if costs[3 as libc::c_int as usize] < bcost {
                                    bcost = costs[3 as libc::c_int as usize];
                                    bmx = omx + 0 as libc::c_int;
                                    bmy = omy + (-i_3 - 2 as libc::c_int);
                                }
                                i_3 += 4 as libc::c_int;
                            }
                        }
                        while i_3 < range {
                            if omy + i_3 <= mv_y_max {
                                let mut cost_6: libc::c_int = ((*h).pixf.fpelcmp[i_pixel as usize])
                                    .expect("non-null function pointer")(
                                    p_fenc,
                                    16 as libc::c_int as intptr_t,
                                    &mut *p_fref_w.offset(((omy + i_3) * stride + omx) as isize),
                                    stride as intptr_t,
                                ) + (*p_cost_mvx
                                    .offset((omx * 4 as libc::c_int) as isize)
                                    as libc::c_int
                                    + *p_cost_mvy.offset(((omy + i_3) * 4 as libc::c_int) as isize)
                                        as libc::c_int);
                                if cost_6 < bcost {
                                    bcost = cost_6;
                                    bmx = omx;
                                    bmy = omy + i_3;
                                }
                            }
                            if omy - i_3 >= mv_y_min {
                                let mut cost_7: libc::c_int = ((*h).pixf.fpelcmp[i_pixel as usize])
                                    .expect("non-null function pointer")(
                                    p_fenc,
                                    16 as libc::c_int as intptr_t,
                                    &mut *p_fref_w.offset(((omy - i_3) * stride + omx) as isize),
                                    stride as intptr_t,
                                ) + (*p_cost_mvx
                                    .offset((omx * 4 as libc::c_int) as isize)
                                    as libc::c_int
                                    + *p_cost_mvy.offset(((omy - i_3) * 4 as libc::c_int) as isize)
                                        as libc::c_int);
                                if cost_7 < bcost {
                                    bcost = cost_7;
                                    bmx = omx;
                                    bmy = omy - i_3;
                                }
                            }
                            i_3 += 2 as libc::c_int;
                        }
                        let mut pix_base_12: *mut pixel = p_fref_w
                            .offset(omx as isize)
                            .offset((omy * stride) as isize);
                        ((*h).pixf.fpelcmp_x4[i_pixel as usize])
                            .expect("non-null function pointer")(
                            p_fenc,
                            pix_base_12
                                .offset(-(1 as libc::c_int) as isize)
                                .offset((-(2 as libc::c_int) * stride) as isize),
                            pix_base_12
                                .offset(1 as libc::c_int as isize)
                                .offset((-(2 as libc::c_int) * stride) as isize),
                            pix_base_12
                                .offset(-(2 as libc::c_int) as isize)
                                .offset((-(1 as libc::c_int) * stride) as isize),
                            pix_base_12
                                .offset(2 as libc::c_int as isize)
                                .offset((-(1 as libc::c_int) * stride) as isize),
                            stride as intptr_t,
                            costs.as_mut_ptr(),
                        );
                        costs[0 as libc::c_int as usize] += *p_cost_mvx
                            .offset(((omx + -(1 as libc::c_int)) * 4 as libc::c_int) as isize)
                            as libc::c_int
                            + *p_cost_mvy
                                .offset(((omy + -(2 as libc::c_int)) * 4 as libc::c_int) as isize)
                                as libc::c_int;
                        costs[1 as libc::c_int as usize] += *p_cost_mvx
                            .offset(((omx + 1 as libc::c_int) * 4 as libc::c_int) as isize)
                            as libc::c_int
                            + *p_cost_mvy
                                .offset(((omy + -(2 as libc::c_int)) * 4 as libc::c_int) as isize)
                                as libc::c_int;
                        costs[2 as libc::c_int as usize] += *p_cost_mvx
                            .offset(((omx + -(2 as libc::c_int)) * 4 as libc::c_int) as isize)
                            as libc::c_int
                            + *p_cost_mvy
                                .offset(((omy + -(1 as libc::c_int)) * 4 as libc::c_int) as isize)
                                as libc::c_int;
                        costs[3 as libc::c_int as usize] += *p_cost_mvx
                            .offset(((omx + 2 as libc::c_int) * 4 as libc::c_int) as isize)
                            as libc::c_int
                            + *p_cost_mvy
                                .offset(((omy + -(1 as libc::c_int)) * 4 as libc::c_int) as isize)
                                as libc::c_int;
                        if costs[0 as libc::c_int as usize] < bcost {
                            bcost = costs[0 as libc::c_int as usize];
                            bmx = omx + -(1 as libc::c_int);
                            bmy = omy + -(2 as libc::c_int);
                        }
                        if costs[1 as libc::c_int as usize] < bcost {
                            bcost = costs[1 as libc::c_int as usize];
                            bmx = omx + 1 as libc::c_int;
                            bmy = omy + -(2 as libc::c_int);
                        }
                        if costs[2 as libc::c_int as usize] < bcost {
                            bcost = costs[2 as libc::c_int as usize];
                            bmx = omx + -(2 as libc::c_int);
                            bmy = omy + -(1 as libc::c_int);
                        }
                        if costs[3 as libc::c_int as usize] < bcost {
                            bcost = costs[3 as libc::c_int as usize];
                            bmx = omx + 2 as libc::c_int;
                            bmy = omy + -(1 as libc::c_int);
                        }
                        let mut pix_base_13: *mut pixel = p_fref_w
                            .offset(omx as isize)
                            .offset((omy * stride) as isize);
                        ((*h).pixf.fpelcmp_x4[i_pixel as usize])
                            .expect("non-null function pointer")(
                            p_fenc,
                            pix_base_13
                                .offset(-(2 as libc::c_int) as isize)
                                .offset((1 as libc::c_int * stride) as isize),
                            pix_base_13
                                .offset(2 as libc::c_int as isize)
                                .offset((1 as libc::c_int * stride) as isize),
                            pix_base_13
                                .offset(-(1 as libc::c_int) as isize)
                                .offset((2 as libc::c_int * stride) as isize),
                            pix_base_13
                                .offset(1 as libc::c_int as isize)
                                .offset((2 as libc::c_int * stride) as isize),
                            stride as intptr_t,
                            costs.as_mut_ptr(),
                        );
                        costs[0 as libc::c_int as usize] += *p_cost_mvx
                            .offset(((omx + -(2 as libc::c_int)) * 4 as libc::c_int) as isize)
                            as libc::c_int
                            + *p_cost_mvy
                                .offset(((omy + 1 as libc::c_int) * 4 as libc::c_int) as isize)
                                as libc::c_int;
                        costs[1 as libc::c_int as usize] += *p_cost_mvx
                            .offset(((omx + 2 as libc::c_int) * 4 as libc::c_int) as isize)
                            as libc::c_int
                            + *p_cost_mvy
                                .offset(((omy + 1 as libc::c_int) * 4 as libc::c_int) as isize)
                                as libc::c_int;
                        costs[2 as libc::c_int as usize] += *p_cost_mvx
                            .offset(((omx + -(1 as libc::c_int)) * 4 as libc::c_int) as isize)
                            as libc::c_int
                            + *p_cost_mvy
                                .offset(((omy + 2 as libc::c_int) * 4 as libc::c_int) as isize)
                                as libc::c_int;
                        costs[3 as libc::c_int as usize] += *p_cost_mvx
                            .offset(((omx + 1 as libc::c_int) * 4 as libc::c_int) as isize)
                            as libc::c_int
                            + *p_cost_mvy
                                .offset(((omy + 2 as libc::c_int) * 4 as libc::c_int) as isize)
                                as libc::c_int;
                        if costs[0 as libc::c_int as usize] < bcost {
                            bcost = costs[0 as libc::c_int as usize];
                            bmx = omx + -(2 as libc::c_int);
                            bmy = omy + 1 as libc::c_int;
                        }
                        if costs[1 as libc::c_int as usize] < bcost {
                            bcost = costs[1 as libc::c_int as usize];
                            bmx = omx + 2 as libc::c_int;
                            bmy = omy + 1 as libc::c_int;
                        }
                        if costs[2 as libc::c_int as usize] < bcost {
                            bcost = costs[2 as libc::c_int as usize];
                            bmx = omx + -(1 as libc::c_int);
                            bmy = omy + 2 as libc::c_int;
                        }
                        if costs[3 as libc::c_int as usize] < bcost {
                            bcost = costs[3 as libc::c_int as usize];
                            bmx = omx + 1 as libc::c_int;
                            bmy = omy + 2 as libc::c_int;
                        }
                        if bcost == ucost2 {
                            current_block = 14127502640287082657;
                        } else {
                            cross_start = range + 2 as libc::c_int;
                            current_block = 3988218931164863998;
                        }
                    } else {
                        current_block = 3988218931164863998;
                    }
                } else {
                    current_block = 3988218931164863998;
                }
                match current_block {
                    14127502640287082657 => {}
                    _ => {
                        if i_mvc != 0 {
                            static mut range_mul: [[uint8_t; 4]; 4] = [
                                [
                                    3 as libc::c_int as uint8_t,
                                    3 as libc::c_int as uint8_t,
                                    4 as libc::c_int as uint8_t,
                                    4 as libc::c_int as uint8_t,
                                ],
                                [
                                    3 as libc::c_int as uint8_t,
                                    4 as libc::c_int as uint8_t,
                                    4 as libc::c_int as uint8_t,
                                    4 as libc::c_int as uint8_t,
                                ],
                                [
                                    4 as libc::c_int as uint8_t,
                                    4 as libc::c_int as uint8_t,
                                    4 as libc::c_int as uint8_t,
                                    5 as libc::c_int as uint8_t,
                                ],
                                [
                                    4 as libc::c_int as uint8_t,
                                    4 as libc::c_int as uint8_t,
                                    5 as libc::c_int as uint8_t,
                                    6 as libc::c_int as uint8_t,
                                ],
                            ];
                            let mut mvd: libc::c_int = 0;
                            let mut sad_ctx: libc::c_int = 0;
                            let mut mvd_ctx: libc::c_int = 0;
                            let mut denom: libc::c_int = 1 as libc::c_int;
                            if i_mvc == 1 as libc::c_int {
                                if i_pixel == PIXEL_16x16 as libc::c_int {
                                    mvd = 25 as libc::c_int;
                                } else {
                                    mvd = abs((*m).mvp[0 as libc::c_int as usize] as libc::c_int
                                        - (*mvc.offset(0 as libc::c_int as isize))
                                            [0 as libc::c_int as usize]
                                            as libc::c_int)
                                        + abs((*m).mvp[1 as libc::c_int as usize] as libc::c_int
                                            - (*mvc.offset(0 as libc::c_int as isize))
                                                [1 as libc::c_int as usize]
                                                as libc::c_int);
                                }
                            } else {
                                denom = i_mvc - 1 as libc::c_int;
                                mvd = 0 as libc::c_int;
                                if i_pixel != PIXEL_16x16 as libc::c_int {
                                    mvd = abs((*m).mvp[0 as libc::c_int as usize] as libc::c_int
                                        - (*mvc.offset(0 as libc::c_int as isize))
                                            [0 as libc::c_int as usize]
                                            as libc::c_int)
                                        + abs((*m).mvp[1 as libc::c_int as usize] as libc::c_int
                                            - (*mvc.offset(0 as libc::c_int as isize))
                                                [1 as libc::c_int as usize]
                                                as libc::c_int);
                                    denom += 1;
                                    denom;
                                }
                                mvd += x264_predictor_difference(mvc, i_mvc as intptr_t);
                            }
                            sad_ctx = if bcost
                                < 1000 as libc::c_int
                                    >> pixel_size_shift[i_pixel as usize] as libc::c_int
                            {
                                0 as libc::c_int
                            } else if bcost
                                < 2000 as libc::c_int
                                    >> pixel_size_shift[i_pixel as usize] as libc::c_int
                            {
                                1 as libc::c_int
                            } else if bcost
                                < 4000 as libc::c_int
                                    >> pixel_size_shift[i_pixel as usize] as libc::c_int
                            {
                                2 as libc::c_int
                            } else {
                                3 as libc::c_int
                            };
                            mvd_ctx = if mvd < 10 as libc::c_int * denom {
                                0 as libc::c_int
                            } else if mvd < 20 as libc::c_int * denom {
                                1 as libc::c_int
                            } else if mvd < 40 as libc::c_int * denom {
                                2 as libc::c_int
                            } else {
                                3 as libc::c_int
                            };
                            i_me_range = (i_me_range
                                * range_mul[mvd_ctx as usize][sad_ctx as usize] as libc::c_int)
                                >> 2 as libc::c_int;
                        }
                        let mut i_4: libc::c_int = cross_start;
                        if i_me_range
                            <= (if mv_x_max - omx < omx - mv_x_min {
                                mv_x_max - omx
                            } else {
                                omx - mv_x_min
                            })
                        {
                            while i_4 < i_me_range - 2 as libc::c_int {
                                let mut pix_base_14: *mut pixel = p_fref_w
                                    .offset(omx as isize)
                                    .offset((omy * stride) as isize);
                                ((*h).pixf.fpelcmp_x4[i_pixel as usize])
                                    .expect("non-null function pointer")(
                                    p_fenc,
                                    pix_base_14
                                        .offset(i_4 as isize)
                                        .offset((0 as libc::c_int * stride) as isize),
                                    pix_base_14
                                        .offset(-i_4 as isize)
                                        .offset((0 as libc::c_int * stride) as isize),
                                    pix_base_14
                                        .offset((i_4 + 2 as libc::c_int) as isize)
                                        .offset((0 as libc::c_int * stride) as isize),
                                    pix_base_14
                                        .offset((-i_4 - 2 as libc::c_int) as isize)
                                        .offset((0 as libc::c_int * stride) as isize),
                                    stride as intptr_t,
                                    costs.as_mut_ptr(),
                                );
                                costs[0 as libc::c_int as usize] += *p_cost_mvx
                                    .offset(((omx + i_4) * 4 as libc::c_int) as isize)
                                    as libc::c_int
                                    + *p_cost_mvy.offset(
                                        ((omy + 0 as libc::c_int) * 4 as libc::c_int) as isize,
                                    ) as libc::c_int;
                                costs[1 as libc::c_int as usize] += *p_cost_mvx
                                    .offset(((omx + -i_4) * 4 as libc::c_int) as isize)
                                    as libc::c_int
                                    + *p_cost_mvy.offset(
                                        ((omy + 0 as libc::c_int) * 4 as libc::c_int) as isize,
                                    ) as libc::c_int;
                                costs[2 as libc::c_int as usize] += *p_cost_mvx.offset(
                                    ((omx + (i_4 + 2 as libc::c_int)) * 4 as libc::c_int) as isize,
                                )
                                    as libc::c_int
                                    + *p_cost_mvy.offset(
                                        ((omy + 0 as libc::c_int) * 4 as libc::c_int) as isize,
                                    ) as libc::c_int;
                                costs[3 as libc::c_int as usize] += *p_cost_mvx.offset(
                                    ((omx + (-i_4 - 2 as libc::c_int)) * 4 as libc::c_int) as isize,
                                )
                                    as libc::c_int
                                    + *p_cost_mvy.offset(
                                        ((omy + 0 as libc::c_int) * 4 as libc::c_int) as isize,
                                    ) as libc::c_int;
                                if costs[0 as libc::c_int as usize] < bcost {
                                    bcost = costs[0 as libc::c_int as usize];
                                    bmx = omx + i_4;
                                    bmy = omy + 0 as libc::c_int;
                                }
                                if costs[1 as libc::c_int as usize] < bcost {
                                    bcost = costs[1 as libc::c_int as usize];
                                    bmx = omx + -i_4;
                                    bmy = omy + 0 as libc::c_int;
                                }
                                if costs[2 as libc::c_int as usize] < bcost {
                                    bcost = costs[2 as libc::c_int as usize];
                                    bmx = omx + (i_4 + 2 as libc::c_int);
                                    bmy = omy + 0 as libc::c_int;
                                }
                                if costs[3 as libc::c_int as usize] < bcost {
                                    bcost = costs[3 as libc::c_int as usize];
                                    bmx = omx + (-i_4 - 2 as libc::c_int);
                                    bmy = omy + 0 as libc::c_int;
                                }
                                i_4 += 4 as libc::c_int;
                            }
                        }
                        while i_4 < i_me_range {
                            if omx + i_4 <= mv_x_max {
                                let mut cost_8: libc::c_int = ((*h).pixf.fpelcmp[i_pixel as usize])
                                    .expect("non-null function pointer")(
                                    p_fenc,
                                    16 as libc::c_int as intptr_t,
                                    &mut *p_fref_w.offset((omy * stride + (omx + i_4)) as isize),
                                    stride as intptr_t,
                                ) + (*p_cost_mvx
                                    .offset(((omx + i_4) * 4 as libc::c_int) as isize)
                                    as libc::c_int
                                    + *p_cost_mvy.offset((omy * 4 as libc::c_int) as isize)
                                        as libc::c_int);
                                if cost_8 < bcost {
                                    bcost = cost_8;
                                    bmx = omx + i_4;
                                    bmy = omy;
                                }
                            }
                            if omx - i_4 >= mv_x_min {
                                let mut cost_9: libc::c_int = ((*h).pixf.fpelcmp[i_pixel as usize])
                                    .expect("non-null function pointer")(
                                    p_fenc,
                                    16 as libc::c_int as intptr_t,
                                    &mut *p_fref_w.offset((omy * stride + (omx - i_4)) as isize),
                                    stride as intptr_t,
                                ) + (*p_cost_mvx
                                    .offset(((omx - i_4) * 4 as libc::c_int) as isize)
                                    as libc::c_int
                                    + *p_cost_mvy.offset((omy * 4 as libc::c_int) as isize)
                                        as libc::c_int);
                                if cost_9 < bcost {
                                    bcost = cost_9;
                                    bmx = omx - i_4;
                                    bmy = omy;
                                }
                            }
                            i_4 += 2 as libc::c_int;
                        }
                        i_4 = cross_start;
                        if i_me_range >> 1 as libc::c_int
                            <= (if mv_y_max - omy < omy - mv_y_min {
                                mv_y_max - omy
                            } else {
                                omy - mv_y_min
                            })
                        {
                            while i_4 < (i_me_range >> 1 as libc::c_int) - 2 as libc::c_int {
                                let mut pix_base_15: *mut pixel = p_fref_w
                                    .offset(omx as isize)
                                    .offset((omy * stride) as isize);
                                ((*h).pixf.fpelcmp_x4[i_pixel as usize])
                                    .expect("non-null function pointer")(
                                    p_fenc,
                                    pix_base_15
                                        .offset(0 as libc::c_int as isize)
                                        .offset((i_4 * stride) as isize),
                                    pix_base_15
                                        .offset(0 as libc::c_int as isize)
                                        .offset((-i_4 * stride) as isize),
                                    pix_base_15
                                        .offset(0 as libc::c_int as isize)
                                        .offset(((i_4 + 2 as libc::c_int) * stride) as isize),
                                    pix_base_15
                                        .offset(0 as libc::c_int as isize)
                                        .offset(((-i_4 - 2 as libc::c_int) * stride) as isize),
                                    stride as intptr_t,
                                    costs.as_mut_ptr(),
                                );
                                costs[0 as libc::c_int as usize] += *p_cost_mvx
                                    .offset(((omx + 0 as libc::c_int) * 4 as libc::c_int) as isize)
                                    as libc::c_int
                                    + *p_cost_mvy.offset(((omy + i_4) * 4 as libc::c_int) as isize)
                                        as libc::c_int;
                                costs[1 as libc::c_int as usize] += *p_cost_mvx
                                    .offset(((omx + 0 as libc::c_int) * 4 as libc::c_int) as isize)
                                    as libc::c_int
                                    + *p_cost_mvy.offset(((omy + -i_4) * 4 as libc::c_int) as isize)
                                        as libc::c_int;
                                costs[2 as libc::c_int as usize] += *p_cost_mvx
                                    .offset(((omx + 0 as libc::c_int) * 4 as libc::c_int) as isize)
                                    as libc::c_int
                                    + *p_cost_mvy.offset(
                                        ((omy + (i_4 + 2 as libc::c_int)) * 4 as libc::c_int)
                                            as isize,
                                    ) as libc::c_int;
                                costs[3 as libc::c_int as usize] += *p_cost_mvx
                                    .offset(((omx + 0 as libc::c_int) * 4 as libc::c_int) as isize)
                                    as libc::c_int
                                    + *p_cost_mvy.offset(
                                        ((omy + (-i_4 - 2 as libc::c_int)) * 4 as libc::c_int)
                                            as isize,
                                    ) as libc::c_int;
                                if costs[0 as libc::c_int as usize] < bcost {
                                    bcost = costs[0 as libc::c_int as usize];
                                    bmx = omx + 0 as libc::c_int;
                                    bmy = omy + i_4;
                                }
                                if costs[1 as libc::c_int as usize] < bcost {
                                    bcost = costs[1 as libc::c_int as usize];
                                    bmx = omx + 0 as libc::c_int;
                                    bmy = omy + -i_4;
                                }
                                if costs[2 as libc::c_int as usize] < bcost {
                                    bcost = costs[2 as libc::c_int as usize];
                                    bmx = omx + 0 as libc::c_int;
                                    bmy = omy + (i_4 + 2 as libc::c_int);
                                }
                                if costs[3 as libc::c_int as usize] < bcost {
                                    bcost = costs[3 as libc::c_int as usize];
                                    bmx = omx + 0 as libc::c_int;
                                    bmy = omy + (-i_4 - 2 as libc::c_int);
                                }
                                i_4 += 4 as libc::c_int;
                            }
                        }
                        while i_4 < i_me_range >> 1 as libc::c_int {
                            if omy + i_4 <= mv_y_max {
                                let mut cost_10: libc::c_int = ((*h).pixf.fpelcmp
                                    [i_pixel as usize])
                                    .expect("non-null function pointer")(
                                    p_fenc,
                                    16 as libc::c_int as intptr_t,
                                    &mut *p_fref_w.offset(((omy + i_4) * stride + omx) as isize),
                                    stride as intptr_t,
                                ) + (*p_cost_mvx
                                    .offset((omx * 4 as libc::c_int) as isize)
                                    as libc::c_int
                                    + *p_cost_mvy.offset(((omy + i_4) * 4 as libc::c_int) as isize)
                                        as libc::c_int);
                                if cost_10 < bcost {
                                    bcost = cost_10;
                                    bmx = omx;
                                    bmy = omy + i_4;
                                }
                            }
                            if omy - i_4 >= mv_y_min {
                                let mut cost_11: libc::c_int = ((*h).pixf.fpelcmp
                                    [i_pixel as usize])
                                    .expect("non-null function pointer")(
                                    p_fenc,
                                    16 as libc::c_int as intptr_t,
                                    &mut *p_fref_w.offset(((omy - i_4) * stride + omx) as isize),
                                    stride as intptr_t,
                                ) + (*p_cost_mvx
                                    .offset((omx * 4 as libc::c_int) as isize)
                                    as libc::c_int
                                    + *p_cost_mvy.offset(((omy - i_4) * 4 as libc::c_int) as isize)
                                        as libc::c_int);
                                if cost_11 < bcost {
                                    bcost = cost_11;
                                    bmx = omx;
                                    bmy = omy - i_4;
                                }
                            }
                            i_4 += 2 as libc::c_int;
                        }
                        let mut pix_base_16: *mut pixel = p_fref_w
                            .offset(omx as isize)
                            .offset((omy * stride) as isize);
                        ((*h).pixf.fpelcmp_x4[i_pixel as usize])
                            .expect("non-null function pointer")(
                            p_fenc,
                            pix_base_16
                                .offset(-(2 as libc::c_int) as isize)
                                .offset((-(2 as libc::c_int) * stride) as isize),
                            pix_base_16
                                .offset(-(2 as libc::c_int) as isize)
                                .offset((2 as libc::c_int * stride) as isize),
                            pix_base_16
                                .offset(2 as libc::c_int as isize)
                                .offset((-(2 as libc::c_int) * stride) as isize),
                            pix_base_16
                                .offset(2 as libc::c_int as isize)
                                .offset((2 as libc::c_int * stride) as isize),
                            stride as intptr_t,
                            costs.as_mut_ptr(),
                        );
                        costs[0 as libc::c_int as usize] += *p_cost_mvx
                            .offset(((omx + -(2 as libc::c_int)) * 4 as libc::c_int) as isize)
                            as libc::c_int
                            + *p_cost_mvy
                                .offset(((omy + -(2 as libc::c_int)) * 4 as libc::c_int) as isize)
                                as libc::c_int;
                        costs[1 as libc::c_int as usize] += *p_cost_mvx
                            .offset(((omx + -(2 as libc::c_int)) * 4 as libc::c_int) as isize)
                            as libc::c_int
                            + *p_cost_mvy
                                .offset(((omy + 2 as libc::c_int) * 4 as libc::c_int) as isize)
                                as libc::c_int;
                        costs[2 as libc::c_int as usize] += *p_cost_mvx
                            .offset(((omx + 2 as libc::c_int) * 4 as libc::c_int) as isize)
                            as libc::c_int
                            + *p_cost_mvy
                                .offset(((omy + -(2 as libc::c_int)) * 4 as libc::c_int) as isize)
                                as libc::c_int;
                        costs[3 as libc::c_int as usize] += *p_cost_mvx
                            .offset(((omx + 2 as libc::c_int) * 4 as libc::c_int) as isize)
                            as libc::c_int
                            + *p_cost_mvy
                                .offset(((omy + 2 as libc::c_int) * 4 as libc::c_int) as isize)
                                as libc::c_int;
                        if costs[0 as libc::c_int as usize] < bcost {
                            bcost = costs[0 as libc::c_int as usize];
                            bmx = omx + -(2 as libc::c_int);
                            bmy = omy + -(2 as libc::c_int);
                        }
                        if costs[1 as libc::c_int as usize] < bcost {
                            bcost = costs[1 as libc::c_int as usize];
                            bmx = omx + -(2 as libc::c_int);
                            bmy = omy + 2 as libc::c_int;
                        }
                        if costs[2 as libc::c_int as usize] < bcost {
                            bcost = costs[2 as libc::c_int as usize];
                            bmx = omx + 2 as libc::c_int;
                            bmy = omy + -(2 as libc::c_int);
                        }
                        if costs[3 as libc::c_int as usize] < bcost {
                            bcost = costs[3 as libc::c_int as usize];
                            bmx = omx + 2 as libc::c_int;
                            bmy = omy + 2 as libc::c_int;
                        }
                        omx = bmx;
                        omy = bmy;
                        let mut p_cost_omvx: *const uint16_t =
                            p_cost_mvx.offset((omx * 4 as libc::c_int) as isize);
                        let mut p_cost_omvy: *const uint16_t =
                            p_cost_mvy.offset((omy * 4 as libc::c_int) as isize);
                        let mut i_5: libc::c_int = 1 as libc::c_int;
                        loop {
                            static mut hex4: [[int8_t; 2]; 16] = [
                                [0 as libc::c_int as int8_t, -(4 as libc::c_int) as int8_t],
                                [0 as libc::c_int as int8_t, 4 as libc::c_int as int8_t],
                                [-(2 as libc::c_int) as int8_t, -(3 as libc::c_int) as int8_t],
                                [2 as libc::c_int as int8_t, -(3 as libc::c_int) as int8_t],
                                [-(4 as libc::c_int) as int8_t, -(2 as libc::c_int) as int8_t],
                                [4 as libc::c_int as int8_t, -(2 as libc::c_int) as int8_t],
                                [-(4 as libc::c_int) as int8_t, -(1 as libc::c_int) as int8_t],
                                [4 as libc::c_int as int8_t, -(1 as libc::c_int) as int8_t],
                                [-(4 as libc::c_int) as int8_t, 0 as libc::c_int as int8_t],
                                [4 as libc::c_int as int8_t, 0 as libc::c_int as int8_t],
                                [-(4 as libc::c_int) as int8_t, 1 as libc::c_int as int8_t],
                                [4 as libc::c_int as int8_t, 1 as libc::c_int as int8_t],
                                [-(4 as libc::c_int) as int8_t, 2 as libc::c_int as int8_t],
                                [4 as libc::c_int as int8_t, 2 as libc::c_int as int8_t],
                                [-(2 as libc::c_int) as int8_t, 3 as libc::c_int as int8_t],
                                [2 as libc::c_int as int8_t, 3 as libc::c_int as int8_t],
                            ];
                            if 4 as libc::c_int * i_5
                                > (if mv_x_max - omx
                                    < (if omx - mv_x_min
                                        < (if mv_y_max - omy < omy - mv_y_min {
                                            mv_y_max - omy
                                        } else {
                                            omy - mv_y_min
                                        })
                                    {
                                        omx - mv_x_min
                                    } else if mv_y_max - omy < omy - mv_y_min {
                                        mv_y_max - omy
                                    } else {
                                        omy - mv_y_min
                                    })
                                {
                                    mv_x_max - omx
                                } else if omx - mv_x_min
                                    < (if mv_y_max - omy < omy - mv_y_min {
                                        mv_y_max - omy
                                    } else {
                                        omy - mv_y_min
                                    })
                                {
                                    omx - mv_x_min
                                } else if mv_y_max - omy < omy - mv_y_min {
                                    mv_y_max - omy
                                } else {
                                    omy - mv_y_min
                                })
                            {
                                let mut j: libc::c_int = 0 as libc::c_int;
                                while j < 16 as libc::c_int {
                                    let mut mx_1: libc::c_int = omx
                                        + hex4[j as usize][0 as libc::c_int as usize]
                                            as libc::c_int
                                            * i_5;
                                    let mut my_1: libc::c_int = omy
                                        + hex4[j as usize][1 as libc::c_int as usize]
                                            as libc::c_int
                                            * i_5;
                                    if ((((mx_1 as uint32_t) << 16 as libc::c_int)
                                        | my_1 as uint32_t & 0x7fff as libc::c_int as uint32_t)
                                        .wrapping_add(mv_min)
                                        | mv_max.wrapping_sub(
                                            ((mx_1 as uint32_t) << 16 as libc::c_int)
                                                | my_1 as uint32_t
                                                    & 0x7fff as libc::c_int as uint32_t,
                                        ))
                                        & 0x80004000 as libc::c_uint
                                        == 0
                                    {
                                        let mut cost_12: libc::c_int = ((*h).pixf.fpelcmp
                                            [i_pixel as usize])
                                            .expect("non-null function pointer")(
                                            p_fenc,
                                            16 as libc::c_int as intptr_t,
                                            &mut *p_fref_w.offset((my_1 * stride + mx_1) as isize),
                                            stride as intptr_t,
                                        ) + (*p_cost_mvx
                                            .offset((mx_1 * 4 as libc::c_int) as isize)
                                            as libc::c_int
                                            + *p_cost_mvy.offset((my_1 * 4 as libc::c_int) as isize)
                                                as libc::c_int);
                                        if cost_12 < bcost {
                                            bcost = cost_12;
                                            bmx = mx_1;
                                            bmy = my_1;
                                        }
                                    }
                                    j += 1;
                                    j;
                                }
                            } else {
                                let mut dir_0: libc::c_int = 0 as libc::c_int;
                                let mut pix_base_17: *mut pixel = p_fref_w
                                    .offset(omx as isize)
                                    .offset(((omy - 4 as libc::c_int * i_5) * stride) as isize);
                                let mut dy: libc::c_int = i_5 * stride;
                                ((*h).pixf.fpelcmp_x4[i_pixel as usize])
                                    .expect("non-null function pointer")(
                                    p_fenc,
                                    pix_base_17
                                        .offset((0 as libc::c_int * i_5) as isize)
                                        .offset(
                                            ((-(4 as libc::c_int)
                                                - 2 as libc::c_int * 0 as libc::c_int
                                                + 4 as libc::c_int)
                                                * dy)
                                                as isize,
                                        ),
                                    pix_base_17
                                        .offset((0 as libc::c_int * i_5) as isize)
                                        .offset(
                                            ((4 as libc::c_int
                                                - 2 as libc::c_int * 0 as libc::c_int
                                                + 4 as libc::c_int)
                                                * dy)
                                                as isize,
                                        ),
                                    pix_base_17
                                        .offset(-((2 as libc::c_int * i_5) as isize))
                                        .offset(
                                            ((-(3 as libc::c_int)
                                                - 2 as libc::c_int * 0 as libc::c_int
                                                + 4 as libc::c_int)
                                                * dy)
                                                as isize,
                                        ),
                                    pix_base_17
                                        .offset((2 as libc::c_int * i_5) as isize)
                                        .offset(
                                            ((-(3 as libc::c_int)
                                                - 2 as libc::c_int * 0 as libc::c_int
                                                + 4 as libc::c_int)
                                                * dy)
                                                as isize,
                                        ),
                                    stride as intptr_t,
                                    costs
                                        .as_mut_ptr()
                                        .offset((4 as libc::c_int * 0 as libc::c_int) as isize),
                                );
                                pix_base_17 = pix_base_17.offset((2 as libc::c_int * dy) as isize);
                                ((*h).pixf.fpelcmp_x4[i_pixel as usize])
                                    .expect("non-null function pointer")(
                                    p_fenc,
                                    pix_base_17
                                        .offset(-((4 as libc::c_int * i_5) as isize))
                                        .offset(
                                            ((-(2 as libc::c_int)
                                                - 2 as libc::c_int * 1 as libc::c_int
                                                + 4 as libc::c_int)
                                                * dy)
                                                as isize,
                                        ),
                                    pix_base_17
                                        .offset((4 as libc::c_int * i_5) as isize)
                                        .offset(
                                            ((-(2 as libc::c_int)
                                                - 2 as libc::c_int * 1 as libc::c_int
                                                + 4 as libc::c_int)
                                                * dy)
                                                as isize,
                                        ),
                                    pix_base_17
                                        .offset(-((4 as libc::c_int * i_5) as isize))
                                        .offset(
                                            ((-(1 as libc::c_int)
                                                - 2 as libc::c_int * 1 as libc::c_int
                                                + 4 as libc::c_int)
                                                * dy)
                                                as isize,
                                        ),
                                    pix_base_17
                                        .offset((4 as libc::c_int * i_5) as isize)
                                        .offset(
                                            ((-(1 as libc::c_int)
                                                - 2 as libc::c_int * 1 as libc::c_int
                                                + 4 as libc::c_int)
                                                * dy)
                                                as isize,
                                        ),
                                    stride as intptr_t,
                                    costs
                                        .as_mut_ptr()
                                        .offset((4 as libc::c_int * 1 as libc::c_int) as isize),
                                );
                                pix_base_17 = pix_base_17.offset((2 as libc::c_int * dy) as isize);
                                ((*h).pixf.fpelcmp_x4[i_pixel as usize])
                                    .expect("non-null function pointer")(
                                    p_fenc,
                                    pix_base_17
                                        .offset(-((4 as libc::c_int * i_5) as isize))
                                        .offset(
                                            ((0 as libc::c_int
                                                - 2 as libc::c_int * 2 as libc::c_int
                                                + 4 as libc::c_int)
                                                * dy)
                                                as isize,
                                        ),
                                    pix_base_17
                                        .offset((4 as libc::c_int * i_5) as isize)
                                        .offset(
                                            ((0 as libc::c_int
                                                - 2 as libc::c_int * 2 as libc::c_int
                                                + 4 as libc::c_int)
                                                * dy)
                                                as isize,
                                        ),
                                    pix_base_17
                                        .offset(-((4 as libc::c_int * i_5) as isize))
                                        .offset(
                                            ((1 as libc::c_int
                                                - 2 as libc::c_int * 2 as libc::c_int
                                                + 4 as libc::c_int)
                                                * dy)
                                                as isize,
                                        ),
                                    pix_base_17
                                        .offset((4 as libc::c_int * i_5) as isize)
                                        .offset(
                                            ((1 as libc::c_int
                                                - 2 as libc::c_int * 2 as libc::c_int
                                                + 4 as libc::c_int)
                                                * dy)
                                                as isize,
                                        ),
                                    stride as intptr_t,
                                    costs
                                        .as_mut_ptr()
                                        .offset((4 as libc::c_int * 2 as libc::c_int) as isize),
                                );
                                pix_base_17 = pix_base_17.offset((2 as libc::c_int * dy) as isize);
                                ((*h).pixf.fpelcmp_x4[i_pixel as usize])
                                    .expect("non-null function pointer")(
                                    p_fenc,
                                    pix_base_17
                                        .offset(-((4 as libc::c_int * i_5) as isize))
                                        .offset(
                                            ((2 as libc::c_int
                                                - 2 as libc::c_int * 3 as libc::c_int
                                                + 4 as libc::c_int)
                                                * dy)
                                                as isize,
                                        ),
                                    pix_base_17
                                        .offset((4 as libc::c_int * i_5) as isize)
                                        .offset(
                                            ((2 as libc::c_int
                                                - 2 as libc::c_int * 3 as libc::c_int
                                                + 4 as libc::c_int)
                                                * dy)
                                                as isize,
                                        ),
                                    pix_base_17
                                        .offset(-((2 as libc::c_int * i_5) as isize))
                                        .offset(
                                            ((3 as libc::c_int
                                                - 2 as libc::c_int * 3 as libc::c_int
                                                + 4 as libc::c_int)
                                                * dy)
                                                as isize,
                                        ),
                                    pix_base_17
                                        .offset((2 as libc::c_int * i_5) as isize)
                                        .offset(
                                            ((3 as libc::c_int
                                                - 2 as libc::c_int * 3 as libc::c_int
                                                + 4 as libc::c_int)
                                                * dy)
                                                as isize,
                                        ),
                                    stride as intptr_t,
                                    costs
                                        .as_mut_ptr()
                                        .offset((4 as libc::c_int * 3 as libc::c_int) as isize),
                                );
                                pix_base_17 = pix_base_17.offset((2 as libc::c_int * dy) as isize);
                                costs[0 as libc::c_int as usize] += *p_cost_omvx
                                    .offset((0 as libc::c_int * 4 as libc::c_int * i_5) as isize)
                                    as libc::c_int
                                    + *p_cost_omvy.offset(
                                        (-(4 as libc::c_int) * 4 as libc::c_int * i_5) as isize,
                                    ) as libc::c_int;
                                costs[1 as libc::c_int as usize] += *p_cost_omvx
                                    .offset((0 as libc::c_int * 4 as libc::c_int * i_5) as isize)
                                    as libc::c_int
                                    + *p_cost_omvy.offset(
                                        (4 as libc::c_int * 4 as libc::c_int * i_5) as isize,
                                    ) as libc::c_int;
                                costs[2 as libc::c_int as usize] += *p_cost_omvx
                                    .offset((-(2 as libc::c_int) * 4 as libc::c_int * i_5) as isize)
                                    as libc::c_int
                                    + *p_cost_omvy.offset(
                                        (-(3 as libc::c_int) * 4 as libc::c_int * i_5) as isize,
                                    ) as libc::c_int;
                                costs[3 as libc::c_int as usize] += *p_cost_omvx
                                    .offset((2 as libc::c_int * 4 as libc::c_int * i_5) as isize)
                                    as libc::c_int
                                    + *p_cost_omvy.offset(
                                        (-(3 as libc::c_int) * 4 as libc::c_int * i_5) as isize,
                                    ) as libc::c_int;
                                costs[4 as libc::c_int as usize] += *p_cost_omvx
                                    .offset((-(4 as libc::c_int) * 4 as libc::c_int * i_5) as isize)
                                    as libc::c_int
                                    + *p_cost_omvy.offset(
                                        (-(2 as libc::c_int) * 4 as libc::c_int * i_5) as isize,
                                    ) as libc::c_int;
                                costs[5 as libc::c_int as usize] += *p_cost_omvx
                                    .offset((4 as libc::c_int * 4 as libc::c_int * i_5) as isize)
                                    as libc::c_int
                                    + *p_cost_omvy.offset(
                                        (-(2 as libc::c_int) * 4 as libc::c_int * i_5) as isize,
                                    ) as libc::c_int;
                                costs[6 as libc::c_int as usize] += *p_cost_omvx
                                    .offset((-(4 as libc::c_int) * 4 as libc::c_int * i_5) as isize)
                                    as libc::c_int
                                    + *p_cost_omvy.offset(
                                        (-(1 as libc::c_int) * 4 as libc::c_int * i_5) as isize,
                                    ) as libc::c_int;
                                costs[7 as libc::c_int as usize] += *p_cost_omvx
                                    .offset((4 as libc::c_int * 4 as libc::c_int * i_5) as isize)
                                    as libc::c_int
                                    + *p_cost_omvy.offset(
                                        (-(1 as libc::c_int) * 4 as libc::c_int * i_5) as isize,
                                    ) as libc::c_int;
                                costs[8 as libc::c_int as usize] += *p_cost_omvx
                                    .offset((-(4 as libc::c_int) * 4 as libc::c_int * i_5) as isize)
                                    as libc::c_int
                                    + *p_cost_omvy.offset(
                                        (0 as libc::c_int * 4 as libc::c_int * i_5) as isize,
                                    ) as libc::c_int;
                                costs[9 as libc::c_int as usize] += *p_cost_omvx
                                    .offset((4 as libc::c_int * 4 as libc::c_int * i_5) as isize)
                                    as libc::c_int
                                    + *p_cost_omvy.offset(
                                        (0 as libc::c_int * 4 as libc::c_int * i_5) as isize,
                                    ) as libc::c_int;
                                costs[10 as libc::c_int as usize] += *p_cost_omvx
                                    .offset((-(4 as libc::c_int) * 4 as libc::c_int * i_5) as isize)
                                    as libc::c_int
                                    + *p_cost_omvy.offset(
                                        (1 as libc::c_int * 4 as libc::c_int * i_5) as isize,
                                    ) as libc::c_int;
                                costs[11 as libc::c_int as usize] += *p_cost_omvx
                                    .offset((4 as libc::c_int * 4 as libc::c_int * i_5) as isize)
                                    as libc::c_int
                                    + *p_cost_omvy.offset(
                                        (1 as libc::c_int * 4 as libc::c_int * i_5) as isize,
                                    ) as libc::c_int;
                                costs[12 as libc::c_int as usize] += *p_cost_omvx
                                    .offset((-(4 as libc::c_int) * 4 as libc::c_int * i_5) as isize)
                                    as libc::c_int
                                    + *p_cost_omvy.offset(
                                        (2 as libc::c_int * 4 as libc::c_int * i_5) as isize,
                                    ) as libc::c_int;
                                costs[13 as libc::c_int as usize] += *p_cost_omvx
                                    .offset((4 as libc::c_int * 4 as libc::c_int * i_5) as isize)
                                    as libc::c_int
                                    + *p_cost_omvy.offset(
                                        (2 as libc::c_int * 4 as libc::c_int * i_5) as isize,
                                    ) as libc::c_int;
                                costs[14 as libc::c_int as usize] += *p_cost_omvx
                                    .offset((-(2 as libc::c_int) * 4 as libc::c_int * i_5) as isize)
                                    as libc::c_int
                                    + *p_cost_omvy.offset(
                                        (3 as libc::c_int * 4 as libc::c_int * i_5) as isize,
                                    ) as libc::c_int;
                                costs[15 as libc::c_int as usize] += *p_cost_omvx
                                    .offset((2 as libc::c_int * 4 as libc::c_int * i_5) as isize)
                                    as libc::c_int
                                    + *p_cost_omvy.offset(
                                        (3 as libc::c_int * 4 as libc::c_int * i_5) as isize,
                                    ) as libc::c_int;
                                if costs[0 as libc::c_int as usize] < bcost {
                                    bcost = costs[0 as libc::c_int as usize];
                                    dir_0 = 0 as libc::c_int * 16 as libc::c_int
                                        + (-(4 as libc::c_int) & 15 as libc::c_int);
                                }
                                if costs[1 as libc::c_int as usize] < bcost {
                                    bcost = costs[1 as libc::c_int as usize];
                                    dir_0 = 0 as libc::c_int * 16 as libc::c_int
                                        + (4 as libc::c_int & 15 as libc::c_int);
                                }
                                if costs[2 as libc::c_int as usize] < bcost {
                                    bcost = costs[2 as libc::c_int as usize];
                                    dir_0 = -(2 as libc::c_int) * 16 as libc::c_int
                                        + (-(3 as libc::c_int) & 15 as libc::c_int);
                                }
                                if costs[3 as libc::c_int as usize] < bcost {
                                    bcost = costs[3 as libc::c_int as usize];
                                    dir_0 = 2 as libc::c_int * 16 as libc::c_int
                                        + (-(3 as libc::c_int) & 15 as libc::c_int);
                                }
                                if costs[4 as libc::c_int as usize] < bcost {
                                    bcost = costs[4 as libc::c_int as usize];
                                    dir_0 = -(4 as libc::c_int) * 16 as libc::c_int
                                        + (-(2 as libc::c_int) & 15 as libc::c_int);
                                }
                                if costs[5 as libc::c_int as usize] < bcost {
                                    bcost = costs[5 as libc::c_int as usize];
                                    dir_0 = 4 as libc::c_int * 16 as libc::c_int
                                        + (-(2 as libc::c_int) & 15 as libc::c_int);
                                }
                                if costs[6 as libc::c_int as usize] < bcost {
                                    bcost = costs[6 as libc::c_int as usize];
                                    dir_0 = -(4 as libc::c_int) * 16 as libc::c_int
                                        + (-(1 as libc::c_int) & 15 as libc::c_int);
                                }
                                if costs[7 as libc::c_int as usize] < bcost {
                                    bcost = costs[7 as libc::c_int as usize];
                                    dir_0 = 4 as libc::c_int * 16 as libc::c_int
                                        + (-(1 as libc::c_int) & 15 as libc::c_int);
                                }
                                if costs[8 as libc::c_int as usize] < bcost {
                                    bcost = costs[8 as libc::c_int as usize];
                                    dir_0 = -(4 as libc::c_int) * 16 as libc::c_int
                                        + (0 as libc::c_int & 15 as libc::c_int);
                                }
                                if costs[9 as libc::c_int as usize] < bcost {
                                    bcost = costs[9 as libc::c_int as usize];
                                    dir_0 = 4 as libc::c_int * 16 as libc::c_int
                                        + (0 as libc::c_int & 15 as libc::c_int);
                                }
                                if costs[10 as libc::c_int as usize] < bcost {
                                    bcost = costs[10 as libc::c_int as usize];
                                    dir_0 = -(4 as libc::c_int) * 16 as libc::c_int
                                        + (1 as libc::c_int & 15 as libc::c_int);
                                }
                                if costs[11 as libc::c_int as usize] < bcost {
                                    bcost = costs[11 as libc::c_int as usize];
                                    dir_0 = 4 as libc::c_int * 16 as libc::c_int
                                        + (1 as libc::c_int & 15 as libc::c_int);
                                }
                                if costs[12 as libc::c_int as usize] < bcost {
                                    bcost = costs[12 as libc::c_int as usize];
                                    dir_0 = -(4 as libc::c_int) * 16 as libc::c_int
                                        + (2 as libc::c_int & 15 as libc::c_int);
                                }
                                if costs[13 as libc::c_int as usize] < bcost {
                                    bcost = costs[13 as libc::c_int as usize];
                                    dir_0 = 4 as libc::c_int * 16 as libc::c_int
                                        + (2 as libc::c_int & 15 as libc::c_int);
                                }
                                if costs[14 as libc::c_int as usize] < bcost {
                                    bcost = costs[14 as libc::c_int as usize];
                                    dir_0 = -(2 as libc::c_int) * 16 as libc::c_int
                                        + (3 as libc::c_int & 15 as libc::c_int);
                                }
                                if costs[15 as libc::c_int as usize] < bcost {
                                    bcost = costs[15 as libc::c_int as usize];
                                    dir_0 = 2 as libc::c_int * 16 as libc::c_int
                                        + (3 as libc::c_int & 15 as libc::c_int);
                                }
                                if dir_0 != 0 {
                                    bmx = omx + i_5 * (dir_0 >> 4 as libc::c_int);
                                    bmy = omy
                                        + i_5
                                            * (((dir_0 as uint32_t) << 28 as libc::c_int)
                                                as int32_t
                                                >> 28 as libc::c_int);
                                }
                            }
                            i_5 += 1;
                            if i_5 > i_me_range >> 2 as libc::c_int {
                                break;
                            }
                        }
                        if bmy <= mv_y_max && bmy >= mv_y_min && bmx <= mv_x_max && bmx >= mv_x_min
                        {
                            current_block = 10359272191501572242;
                        } else {
                            current_block = 14127502640287082657;
                        }
                    }
                }
            }
        }
        3 | 4 => {
            let min_x: libc::c_int = if bmx - i_me_range > mv_x_min {
                bmx - i_me_range
            } else {
                mv_x_min
            };
            let min_y: libc::c_int = if bmy - i_me_range > mv_y_min {
                bmy - i_me_range
            } else {
                mv_y_min
            };
            let max_x: libc::c_int = if bmx + i_me_range < mv_x_max {
                bmx + i_me_range
            } else {
                mv_x_max
            };
            let max_y: libc::c_int = if bmy + i_me_range < mv_y_max {
                bmy + i_me_range
            } else {
                mv_y_max
            };
            let width: libc::c_int = (max_x - min_x + 3 as libc::c_int) & !(3 as libc::c_int);
            let mut sums_base: *mut uint16_t = (*m).integral;
            let mut enc_dc: [libc::c_int; 4] = [0; 4];
            let mut sad_size: libc::c_int = if i_pixel <= PIXEL_8x8 as libc::c_int {
                PIXEL_8x8 as libc::c_int
            } else {
                PIXEL_4x4 as libc::c_int
            };
            let mut delta: libc::c_int = x264_pixel_size[sad_size as usize].w as libc::c_int;
            let mut xs: *mut int16_t = (*h).scratch_buffer as *mut int16_t;
            let mut xn: libc::c_int = 0;
            let mut cost_fpel_mvx: *mut uint16_t =
                ((*h).cost_mv_fpel[(*h).mb.i_qp as usize][(-((*m).mvp[0 as libc::c_int as usize]
                    as libc::c_int)
                    & 3 as libc::c_int)
                    as usize])
                    .offset(
                        (-((*m).mvp[0 as libc::c_int as usize] as libc::c_int) >> 2 as libc::c_int)
                            as isize,
                    );
            ((*h).pixf.sad_x4[sad_size as usize]).expect("non-null function pointer")(
                x264_zero.as_mut_ptr() as *mut pixel,
                p_fenc,
                p_fenc.offset(delta as isize),
                p_fenc.offset((delta * 16 as libc::c_int) as isize),
                p_fenc
                    .offset(delta as isize)
                    .offset((delta * 16 as libc::c_int) as isize),
                16 as libc::c_int as intptr_t,
                enc_dc.as_mut_ptr(),
            );
            if delta == 4 as libc::c_int {
                sums_base = sums_base.offset(
                    (stride
                        * ((*(*h).fenc).i_lines[0 as libc::c_int as usize]
                            + 32 as libc::c_int * 2 as libc::c_int)) as isize,
                );
            }
            if i_pixel == PIXEL_16x16 as libc::c_int
                || i_pixel == PIXEL_8x16 as libc::c_int
                || i_pixel == PIXEL_4x8 as libc::c_int
            {
                delta *= stride;
            }
            if i_pixel == PIXEL_8x16 as libc::c_int || i_pixel == PIXEL_4x8 as libc::c_int {
                enc_dc[1 as libc::c_int as usize] = enc_dc[2 as libc::c_int as usize];
            }
            if (*h).mb.i_me_method == 4 as libc::c_int {
                let mut mvsads: *mut mvsad_t =
                    xs.offset(((width + 31 as libc::c_int) & !(31 as libc::c_int)) as isize)
                        .offset(4 as libc::c_int as isize) as *mut mvsad_t;
                let mut nmvsad: libc::c_int = 0 as libc::c_int;
                let mut limit: libc::c_int = 0;
                let mut sad_thresh: libc::c_int = if i_me_range <= 16 as libc::c_int {
                    10 as libc::c_int
                } else if i_me_range <= 24 as libc::c_int {
                    11 as libc::c_int
                } else {
                    12 as libc::c_int
                };
                let mut bsad: libc::c_int =
                    ((*h).pixf.sad[i_pixel as usize]).expect("non-null function pointer")(
                        p_fenc,
                        16 as libc::c_int as intptr_t,
                        p_fref_w
                            .offset((bmy * stride) as isize)
                            .offset(bmx as isize),
                        stride as intptr_t,
                    ) + (*p_cost_mvx.offset((bmx * 4 as libc::c_int) as isize) as libc::c_int
                        + *p_cost_mvy.offset((bmy * 4 as libc::c_int) as isize) as libc::c_int);
                let mut my_2: libc::c_int = min_y;
                while my_2 <= max_y {
                    let mut i_6: libc::c_int = 0;
                    let mut ycost: libc::c_int =
                        *p_cost_mvy.offset((my_2 * 4 as libc::c_int) as isize) as libc::c_int;
                    if bsad > ycost {
                        bsad -= ycost;
                        xn = ((*h).pixf.ads[i_pixel as usize]).expect("non-null function pointer")(
                            enc_dc.as_mut_ptr(),
                            sums_base
                                .offset(min_x as isize)
                                .offset((my_2 * stride) as isize),
                            delta,
                            cost_fpel_mvx.offset(min_x as isize),
                            xs,
                            width,
                            (bsad * 17 as libc::c_int) >> 4 as libc::c_int,
                        );
                        i_6 = 0 as libc::c_int;
                        while i_6 < xn - 2 as libc::c_int {
                            let mut ref_0: *mut pixel = p_fref_w
                                .offset(min_x as isize)
                                .offset((my_2 * stride) as isize);
                            let mut sads: [libc::c_int; 4] = [0; 4];
                            ((*h).pixf.sad_x3[i_pixel as usize])
                                .expect("non-null function pointer")(
                                p_fenc,
                                ref_0.offset(*xs.offset(i_6 as isize) as libc::c_int as isize),
                                ref_0.offset(*xs.offset((i_6 + 1 as libc::c_int) as isize)
                                    as libc::c_int
                                    as isize),
                                ref_0.offset(*xs.offset((i_6 + 2 as libc::c_int) as isize)
                                    as libc::c_int
                                    as isize),
                                stride as intptr_t,
                                sads.as_mut_ptr(),
                            );
                            let mut j_0: libc::c_int = 0 as libc::c_int;
                            while j_0 < 3 as libc::c_int {
                                let mut sad: libc::c_int = sads[j_0 as usize]
                                    + *cost_fpel_mvx
                                        .offset(*xs.offset((i_6 + j_0) as isize) as isize)
                                        as libc::c_int;
                                if sad < (bsad * sad_thresh) >> 3 as libc::c_int {
                                    if sad < bsad {
                                        bsad = sad;
                                    }
                                    (*mvsads.offset(nmvsad as isize)).sad = sad + ycost;
                                    (*mvsads.offset(nmvsad as isize)).mv
                                        [0 as libc::c_int as usize] = (min_x
                                        + *xs.offset((i_6 + j_0) as isize) as libc::c_int)
                                        as int16_t;
                                    (*mvsads.offset(nmvsad as isize)).mv
                                        [1 as libc::c_int as usize] = my_2 as int16_t;
                                    nmvsad += 1;
                                    nmvsad;
                                }
                                j_0 += 1;
                                j_0;
                            }
                            i_6 += 3 as libc::c_int;
                        }
                        while i_6 < xn {
                            let mut mx_2: libc::c_int =
                                min_x + *xs.offset(i_6 as isize) as libc::c_int;
                            let mut sad_0: libc::c_int = ((*h).pixf.sad[i_pixel as usize])
                                .expect("non-null function pointer")(
                                p_fenc,
                                16 as libc::c_int as intptr_t,
                                p_fref_w
                                    .offset(mx_2 as isize)
                                    .offset((my_2 * stride) as isize),
                                stride as intptr_t,
                            ) + *cost_fpel_mvx
                                .offset(*xs.offset(i_6 as isize) as isize)
                                as libc::c_int;
                            if sad_0 < (bsad * sad_thresh) >> 3 as libc::c_int {
                                if sad_0 < bsad {
                                    bsad = sad_0;
                                }
                                (*mvsads.offset(nmvsad as isize)).sad = sad_0 + ycost;
                                (*mvsads.offset(nmvsad as isize)).mv[0 as libc::c_int as usize] =
                                    mx_2 as int16_t;
                                (*mvsads.offset(nmvsad as isize)).mv[1 as libc::c_int as usize] =
                                    my_2 as int16_t;
                                nmvsad += 1;
                                nmvsad;
                            }
                            i_6 += 1;
                            i_6;
                        }
                        bsad += ycost;
                    }
                    my_2 += 1;
                    my_2;
                }
                limit = i_me_range >> 1 as libc::c_int;
                sad_thresh = (bsad * sad_thresh) >> 3 as libc::c_int;
                while nmvsad > limit * 2 as libc::c_int && sad_thresh > bsad {
                    let mut i_7: libc::c_int = 0 as libc::c_int;
                    sad_thresh = (sad_thresh + bsad) >> 1 as libc::c_int;
                    while i_7 < nmvsad && (*mvsads.offset(i_7 as isize)).sad <= sad_thresh {
                        i_7 += 1;
                        i_7;
                    }
                    let mut j_1: libc::c_int = i_7;
                    while j_1 < nmvsad {
                        let mut sad_1: uint32_t = 0;
                        if ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                            == 8 as libc::c_int as libc::c_ulong
                            && ::core::mem::size_of::<mvsad_t>() as libc::c_ulong
                                == 8 as libc::c_int as libc::c_ulong
                        {
                            let fresh0 = &mut (*(&mut *mvsads.offset(i_7 as isize) as *mut mvsad_t
                                as *mut x264_union64_t))
                                .i;
                            *fresh0 = (*(&mut *mvsads.offset(j_1 as isize) as *mut mvsad_t
                                as *mut x264_union64_t))
                                .i;
                            let mut mvsad: uint64_t = *fresh0;
                            sad_1 = mvsad as uint32_t;
                        } else {
                            sad_1 = (*mvsads.offset(j_1 as isize)).sad as uint32_t;
                            (*(((*mvsads.offset(i_7 as isize)).mv).as_mut_ptr()
                                as *mut x264_union32_t))
                                .i = (*(((*mvsads.offset(j_1 as isize)).mv).as_mut_ptr()
                                as *mut x264_union32_t))
                                .i;
                            (*mvsads.offset(i_7 as isize)).sad = sad_1 as libc::c_int;
                        }
                        i_7 = (i_7 as uint32_t).wrapping_add(
                            sad_1.wrapping_sub((sad_thresh + 1 as libc::c_int) as uint32_t)
                                >> 31 as libc::c_int,
                        ) as libc::c_int as libc::c_int;
                        j_1 += 1;
                        j_1;
                    }
                    nmvsad = i_7;
                }
                while nmvsad > limit {
                    let mut bi: libc::c_int = 0 as libc::c_int;
                    let mut i_8: libc::c_int = 1 as libc::c_int;
                    while i_8 < nmvsad {
                        if (*mvsads.offset(i_8 as isize)).sad > (*mvsads.offset(bi as isize)).sad {
                            bi = i_8;
                        }
                        i_8 += 1;
                        i_8;
                    }
                    nmvsad -= 1;
                    nmvsad;
                    if ::core::mem::size_of::<mvsad_t>() as libc::c_ulong
                        == ::core::mem::size_of::<uint64_t>() as libc::c_ulong
                    {
                        (*(&mut *mvsads.offset(bi as isize) as *mut mvsad_t
                            as *mut x264_union64_t))
                            .i = (*(&mut *mvsads.offset(nmvsad as isize) as *mut mvsad_t
                            as *mut x264_union64_t))
                            .i;
                    } else {
                        *mvsads.offset(bi as isize) = *mvsads.offset(nmvsad as isize);
                    }
                }
                let mut i_9: libc::c_int = 0 as libc::c_int;
                while i_9 < nmvsad {
                    let mut cost_13: libc::c_int = ((*h).pixf.fpelcmp[i_pixel as usize])
                        .expect("non-null function pointer")(
                        p_fenc,
                        16 as libc::c_int as intptr_t,
                        &mut *p_fref_w.offset(
                            (*((*mvsads.offset(i_9 as isize)).mv)
                                .as_mut_ptr()
                                .offset(1 as libc::c_int as isize)
                                as libc::c_int
                                * stride
                                + *((*mvsads.offset(i_9 as isize)).mv)
                                    .as_mut_ptr()
                                    .offset(0 as libc::c_int as isize)
                                    as libc::c_int) as isize,
                        ),
                        stride as intptr_t,
                    ) + (*p_cost_mvx.offset(
                        ((*mvsads.offset(i_9 as isize)).mv[0 as libc::c_int as usize]
                            as libc::c_int
                            * 4 as libc::c_int) as isize,
                    ) as libc::c_int
                        + *p_cost_mvy.offset(
                            ((*mvsads.offset(i_9 as isize)).mv[1 as libc::c_int as usize]
                                as libc::c_int
                                * 4 as libc::c_int) as isize,
                        ) as libc::c_int);
                    if cost_13 < bcost {
                        bcost = cost_13;
                        bmx = (*mvsads.offset(i_9 as isize)).mv[0 as libc::c_int as usize]
                            as libc::c_int;
                        bmy = (*mvsads.offset(i_9 as isize)).mv[1 as libc::c_int as usize]
                            as libc::c_int;
                    }
                    i_9 += 1;
                    i_9;
                }
            } else {
                let mut my_3: libc::c_int = min_y;
                while my_3 <= max_y {
                    let mut i_10: libc::c_int = 0;
                    let mut ycost_0: libc::c_int =
                        *p_cost_mvy.offset((my_3 * 4 as libc::c_int) as isize) as libc::c_int;
                    if bcost > ycost_0 {
                        bcost -= ycost_0;
                        xn = ((*h).pixf.ads[i_pixel as usize]).expect("non-null function pointer")(
                            enc_dc.as_mut_ptr(),
                            sums_base
                                .offset(min_x as isize)
                                .offset((my_3 * stride) as isize),
                            delta,
                            cost_fpel_mvx.offset(min_x as isize),
                            xs,
                            width,
                            bcost,
                        );
                        i_10 = 0 as libc::c_int;
                        while i_10 < xn - 2 as libc::c_int {
                            ((*h).pixf.fpelcmp_x3[i_pixel as usize])
                                .expect("non-null function pointer")(
                                p_fenc,
                                p_fref_w
                                    .offset(
                                        (min_x + *xs.offset(i_10 as isize) as libc::c_int) as isize,
                                    )
                                    .offset((my_3 * stride) as isize),
                                p_fref_w
                                    .offset(
                                        (min_x
                                            + *xs.offset((i_10 + 1 as libc::c_int) as isize)
                                                as libc::c_int)
                                            as isize,
                                    )
                                    .offset((my_3 * stride) as isize),
                                p_fref_w
                                    .offset(
                                        (min_x
                                            + *xs.offset((i_10 + 2 as libc::c_int) as isize)
                                                as libc::c_int)
                                            as isize,
                                    )
                                    .offset((my_3 * stride) as isize),
                                stride as intptr_t,
                                costs.as_mut_ptr(),
                            );
                            costs[0 as libc::c_int as usize] += *p_cost_mvx.offset(
                                ((min_x + *xs.offset(i_10 as isize) as libc::c_int)
                                    * 4 as libc::c_int) as isize,
                            )
                                as libc::c_int;
                            costs[1 as libc::c_int as usize] += *p_cost_mvx.offset(
                                ((min_x
                                    + *xs.offset((i_10 + 1 as libc::c_int) as isize)
                                        as libc::c_int)
                                    * 4 as libc::c_int) as isize,
                            )
                                as libc::c_int;
                            costs[2 as libc::c_int as usize] += *p_cost_mvx.offset(
                                ((min_x
                                    + *xs.offset((i_10 + 2 as libc::c_int) as isize)
                                        as libc::c_int)
                                    * 4 as libc::c_int) as isize,
                            )
                                as libc::c_int;
                            if costs[0 as libc::c_int as usize] < bcost {
                                bcost = costs[0 as libc::c_int as usize];
                                bmx = min_x + *xs.offset(i_10 as isize) as libc::c_int;
                                bmy = my_3;
                            }
                            if costs[1 as libc::c_int as usize] < bcost {
                                bcost = costs[1 as libc::c_int as usize];
                                bmx = min_x
                                    + *xs.offset((i_10 + 1 as libc::c_int) as isize) as libc::c_int;
                                bmy = my_3;
                            }
                            if costs[2 as libc::c_int as usize] < bcost {
                                bcost = costs[2 as libc::c_int as usize];
                                bmx = min_x
                                    + *xs.offset((i_10 + 2 as libc::c_int) as isize) as libc::c_int;
                                bmy = my_3;
                            }
                            i_10 += 3 as libc::c_int;
                        }
                        bcost += ycost_0;
                        while i_10 < xn {
                            let mut cost_14: libc::c_int = ((*h).pixf.fpelcmp[i_pixel as usize])
                                .expect("non-null function pointer")(
                                p_fenc,
                                16 as libc::c_int as intptr_t,
                                &mut *p_fref_w.offset(
                                    (my_3 * stride
                                        + (min_x + *xs.offset(i_10 as isize) as libc::c_int))
                                        as isize,
                                ),
                                stride as intptr_t,
                            ) + (*p_cost_mvx.offset(
                                ((min_x + *xs.offset(i_10 as isize) as libc::c_int)
                                    * 4 as libc::c_int) as isize,
                            )
                                as libc::c_int
                                + *p_cost_mvy.offset((my_3 * 4 as libc::c_int) as isize)
                                    as libc::c_int);
                            if cost_14 < bcost {
                                bcost = cost_14;
                                bmx = min_x + *xs.offset(i_10 as isize) as libc::c_int;
                                bmy = my_3;
                            }
                            i_10 += 1;
                            i_10;
                        }
                    }
                    my_3 += 1;
                    my_3;
                }
            }
            current_block = 14127502640287082657;
        }
        _ => {
            current_block = 14127502640287082657;
        }
    }
    if current_block == 10359272191501572242 {
        let mut pix_base_0: *mut pixel = p_fref_w
            .offset(bmx as isize)
            .offset((bmy * stride) as isize);
        ((*h).pixf.fpelcmp_x3[i_pixel as usize]).expect("non-null function pointer")(
            p_fenc,
            pix_base_0
                .offset(-(2 as libc::c_int) as isize)
                .offset((0 as libc::c_int * stride) as isize),
            pix_base_0
                .offset(-(1 as libc::c_int) as isize)
                .offset((2 as libc::c_int * stride) as isize),
            pix_base_0
                .offset(1 as libc::c_int as isize)
                .offset((2 as libc::c_int * stride) as isize),
            stride as intptr_t,
            costs.as_mut_ptr(),
        );
        costs[0 as libc::c_int as usize] += *p_cost_mvx
            .offset(((bmx + -(2 as libc::c_int)) * 4 as libc::c_int) as isize)
            as libc::c_int
            + *p_cost_mvy.offset(((bmy + 0 as libc::c_int) * 4 as libc::c_int) as isize)
                as libc::c_int;
        costs[1 as libc::c_int as usize] += *p_cost_mvx
            .offset(((bmx + -(1 as libc::c_int)) * 4 as libc::c_int) as isize)
            as libc::c_int
            + *p_cost_mvy.offset(((bmy + 2 as libc::c_int) * 4 as libc::c_int) as isize)
                as libc::c_int;
        costs[2 as libc::c_int as usize] += *p_cost_mvx
            .offset(((bmx + 1 as libc::c_int) * 4 as libc::c_int) as isize)
            as libc::c_int
            + *p_cost_mvy.offset(((bmy + 2 as libc::c_int) * 4 as libc::c_int) as isize)
                as libc::c_int;
        let mut pix_base_1: *mut pixel = p_fref_w
            .offset(bmx as isize)
            .offset((bmy * stride) as isize);
        ((*h).pixf.fpelcmp_x3[i_pixel as usize]).expect("non-null function pointer")(
            p_fenc,
            pix_base_1
                .offset(2 as libc::c_int as isize)
                .offset((0 as libc::c_int * stride) as isize),
            pix_base_1
                .offset(1 as libc::c_int as isize)
                .offset((-(2 as libc::c_int) * stride) as isize),
            pix_base_1
                .offset(-(1 as libc::c_int) as isize)
                .offset((-(2 as libc::c_int) * stride) as isize),
            stride as intptr_t,
            costs.as_mut_ptr().offset(4 as libc::c_int as isize),
        );
        *costs
            .as_mut_ptr()
            .offset(4 as libc::c_int as isize)
            .offset(0 as libc::c_int as isize) += *p_cost_mvx
            .offset(((bmx + 2 as libc::c_int) * 4 as libc::c_int) as isize)
            as libc::c_int
            + *p_cost_mvy.offset(((bmy + 0 as libc::c_int) * 4 as libc::c_int) as isize)
                as libc::c_int;
        *costs
            .as_mut_ptr()
            .offset(4 as libc::c_int as isize)
            .offset(1 as libc::c_int as isize) += *p_cost_mvx
            .offset(((bmx + 1 as libc::c_int) * 4 as libc::c_int) as isize)
            as libc::c_int
            + *p_cost_mvy.offset(((bmy + -(2 as libc::c_int)) * 4 as libc::c_int) as isize)
                as libc::c_int;
        *costs
            .as_mut_ptr()
            .offset(4 as libc::c_int as isize)
            .offset(2 as libc::c_int as isize) += *p_cost_mvx
            .offset(((bmx + -(1 as libc::c_int)) * 4 as libc::c_int) as isize)
            as libc::c_int
            + *p_cost_mvy.offset(((bmy + -(2 as libc::c_int)) * 4 as libc::c_int) as isize)
                as libc::c_int;
        bcost <<= 3 as libc::c_int;
        if ((costs[0 as libc::c_int as usize] << 3 as libc::c_int) + 2 as libc::c_int) < bcost {
            bcost = (costs[0 as libc::c_int as usize] << 3 as libc::c_int) + 2 as libc::c_int;
        }
        if ((costs[1 as libc::c_int as usize] << 3 as libc::c_int) + 3 as libc::c_int) < bcost {
            bcost = (costs[1 as libc::c_int as usize] << 3 as libc::c_int) + 3 as libc::c_int;
        }
        if ((costs[2 as libc::c_int as usize] << 3 as libc::c_int) + 4 as libc::c_int) < bcost {
            bcost = (costs[2 as libc::c_int as usize] << 3 as libc::c_int) + 4 as libc::c_int;
        }
        if ((costs[4 as libc::c_int as usize] << 3 as libc::c_int) + 5 as libc::c_int) < bcost {
            bcost = (costs[4 as libc::c_int as usize] << 3 as libc::c_int) + 5 as libc::c_int;
        }
        if ((costs[5 as libc::c_int as usize] << 3 as libc::c_int) + 6 as libc::c_int) < bcost {
            bcost = (costs[5 as libc::c_int as usize] << 3 as libc::c_int) + 6 as libc::c_int;
        }
        if ((costs[6 as libc::c_int as usize] << 3 as libc::c_int) + 7 as libc::c_int) < bcost {
            bcost = (costs[6 as libc::c_int as usize] << 3 as libc::c_int) + 7 as libc::c_int;
        }
        if bcost & 7 as libc::c_int != 0 {
            let mut dir: libc::c_int = (bcost & 7 as libc::c_int) - 2 as libc::c_int;
            bmx +=
                hex2[(dir + 1 as libc::c_int) as usize][0 as libc::c_int as usize] as libc::c_int;
            bmy +=
                hex2[(dir + 1 as libc::c_int) as usize][1 as libc::c_int as usize] as libc::c_int;
            let mut i_2: libc::c_int = (i_me_range >> 1 as libc::c_int) - 1 as libc::c_int;
            while i_2 > 0 as libc::c_int
                && ((((bmx as uint32_t) << 16 as libc::c_int)
                    | bmy as uint32_t & 0x7fff as libc::c_int as uint32_t)
                    .wrapping_add(mv_min)
                    | mv_max.wrapping_sub(
                        ((bmx as uint32_t) << 16 as libc::c_int)
                            | bmy as uint32_t & 0x7fff as libc::c_int as uint32_t,
                    ))
                    & 0x80004000 as libc::c_uint
                    == 0
            {
                let mut pix_base_2: *mut pixel = p_fref_w
                    .offset(bmx as isize)
                    .offset((bmy * stride) as isize);
                ((*h).pixf.fpelcmp_x3[i_pixel as usize]).expect("non-null function pointer")(
                    p_fenc,
                    pix_base_2
                        .offset(
                            hex2[(dir + 0 as libc::c_int) as usize][0 as libc::c_int as usize]
                                as libc::c_int as isize,
                        )
                        .offset(
                            (hex2[(dir + 0 as libc::c_int) as usize][1 as libc::c_int as usize]
                                as libc::c_int
                                * stride) as isize,
                        ),
                    pix_base_2
                        .offset(
                            hex2[(dir + 1 as libc::c_int) as usize][0 as libc::c_int as usize]
                                as libc::c_int as isize,
                        )
                        .offset(
                            (hex2[(dir + 1 as libc::c_int) as usize][1 as libc::c_int as usize]
                                as libc::c_int
                                * stride) as isize,
                        ),
                    pix_base_2
                        .offset(
                            hex2[(dir + 2 as libc::c_int) as usize][0 as libc::c_int as usize]
                                as libc::c_int as isize,
                        )
                        .offset(
                            (hex2[(dir + 2 as libc::c_int) as usize][1 as libc::c_int as usize]
                                as libc::c_int
                                * stride) as isize,
                        ),
                    stride as intptr_t,
                    costs.as_mut_ptr(),
                );
                costs[0 as libc::c_int as usize] += *p_cost_mvx.offset(
                    ((bmx
                        + hex2[(dir + 0 as libc::c_int) as usize][0 as libc::c_int as usize]
                            as libc::c_int)
                        * 4 as libc::c_int) as isize,
                ) as libc::c_int
                    + *p_cost_mvy.offset(
                        ((bmy
                            + hex2[(dir + 0 as libc::c_int) as usize][1 as libc::c_int as usize]
                                as libc::c_int)
                            * 4 as libc::c_int) as isize,
                    ) as libc::c_int;
                costs[1 as libc::c_int as usize] += *p_cost_mvx.offset(
                    ((bmx
                        + hex2[(dir + 1 as libc::c_int) as usize][0 as libc::c_int as usize]
                            as libc::c_int)
                        * 4 as libc::c_int) as isize,
                ) as libc::c_int
                    + *p_cost_mvy.offset(
                        ((bmy
                            + hex2[(dir + 1 as libc::c_int) as usize][1 as libc::c_int as usize]
                                as libc::c_int)
                            * 4 as libc::c_int) as isize,
                    ) as libc::c_int;
                costs[2 as libc::c_int as usize] += *p_cost_mvx.offset(
                    ((bmx
                        + hex2[(dir + 2 as libc::c_int) as usize][0 as libc::c_int as usize]
                            as libc::c_int)
                        * 4 as libc::c_int) as isize,
                ) as libc::c_int
                    + *p_cost_mvy.offset(
                        ((bmy
                            + hex2[(dir + 2 as libc::c_int) as usize][1 as libc::c_int as usize]
                                as libc::c_int)
                            * 4 as libc::c_int) as isize,
                    ) as libc::c_int;
                bcost &= !(7 as libc::c_int);
                if ((costs[0 as libc::c_int as usize] << 3 as libc::c_int) + 1 as libc::c_int)
                    < bcost
                {
                    bcost =
                        (costs[0 as libc::c_int as usize] << 3 as libc::c_int) + 1 as libc::c_int;
                }
                if ((costs[1 as libc::c_int as usize] << 3 as libc::c_int) + 2 as libc::c_int)
                    < bcost
                {
                    bcost =
                        (costs[1 as libc::c_int as usize] << 3 as libc::c_int) + 2 as libc::c_int;
                }
                if ((costs[2 as libc::c_int as usize] << 3 as libc::c_int) + 3 as libc::c_int)
                    < bcost
                {
                    bcost =
                        (costs[2 as libc::c_int as usize] << 3 as libc::c_int) + 3 as libc::c_int;
                }
                if bcost & 7 as libc::c_int == 0 {
                    break;
                }
                dir += (bcost & 7 as libc::c_int) - 2 as libc::c_int;
                dir = mod6m1[(dir + 1 as libc::c_int) as usize] as libc::c_int;
                bmx += hex2[(dir + 1 as libc::c_int) as usize][0 as libc::c_int as usize]
                    as libc::c_int;
                bmy += hex2[(dir + 1 as libc::c_int) as usize][1 as libc::c_int as usize]
                    as libc::c_int;
                i_2 -= 1;
                i_2;
            }
        }
        bcost >>= 3 as libc::c_int;
        bcost <<= 4 as libc::c_int;
        let mut pix_base_3: *mut pixel = p_fref_w
            .offset(bmx as isize)
            .offset((bmy * stride) as isize);
        ((*h).pixf.fpelcmp_x4[i_pixel as usize]).expect("non-null function pointer")(
            p_fenc,
            pix_base_3
                .offset(0 as libc::c_int as isize)
                .offset((-(1 as libc::c_int) * stride) as isize),
            pix_base_3
                .offset(0 as libc::c_int as isize)
                .offset((1 as libc::c_int * stride) as isize),
            pix_base_3
                .offset(-(1 as libc::c_int) as isize)
                .offset((0 as libc::c_int * stride) as isize),
            pix_base_3
                .offset(1 as libc::c_int as isize)
                .offset((0 as libc::c_int * stride) as isize),
            stride as intptr_t,
            costs.as_mut_ptr(),
        );
        costs[0 as libc::c_int as usize] += *p_cost_mvx
            .offset(((bmx + 0 as libc::c_int) * 4 as libc::c_int) as isize)
            as libc::c_int
            + *p_cost_mvy.offset(((bmy + -(1 as libc::c_int)) * 4 as libc::c_int) as isize)
                as libc::c_int;
        costs[1 as libc::c_int as usize] += *p_cost_mvx
            .offset(((bmx + 0 as libc::c_int) * 4 as libc::c_int) as isize)
            as libc::c_int
            + *p_cost_mvy.offset(((bmy + 1 as libc::c_int) * 4 as libc::c_int) as isize)
                as libc::c_int;
        costs[2 as libc::c_int as usize] += *p_cost_mvx
            .offset(((bmx + -(1 as libc::c_int)) * 4 as libc::c_int) as isize)
            as libc::c_int
            + *p_cost_mvy.offset(((bmy + 0 as libc::c_int) * 4 as libc::c_int) as isize)
                as libc::c_int;
        costs[3 as libc::c_int as usize] += *p_cost_mvx
            .offset(((bmx + 1 as libc::c_int) * 4 as libc::c_int) as isize)
            as libc::c_int
            + *p_cost_mvy.offset(((bmy + 0 as libc::c_int) * 4 as libc::c_int) as isize)
                as libc::c_int;
        if ((costs[0 as libc::c_int as usize] << 4 as libc::c_int) + 1 as libc::c_int) < bcost {
            bcost = (costs[0 as libc::c_int as usize] << 4 as libc::c_int) + 1 as libc::c_int;
        }
        if ((costs[1 as libc::c_int as usize] << 4 as libc::c_int) + 2 as libc::c_int) < bcost {
            bcost = (costs[1 as libc::c_int as usize] << 4 as libc::c_int) + 2 as libc::c_int;
        }
        if ((costs[2 as libc::c_int as usize] << 4 as libc::c_int) + 3 as libc::c_int) < bcost {
            bcost = (costs[2 as libc::c_int as usize] << 4 as libc::c_int) + 3 as libc::c_int;
        }
        if ((costs[3 as libc::c_int as usize] << 4 as libc::c_int) + 4 as libc::c_int) < bcost {
            bcost = (costs[3 as libc::c_int as usize] << 4 as libc::c_int) + 4 as libc::c_int;
        }
        let mut pix_base_4: *mut pixel = p_fref_w
            .offset(bmx as isize)
            .offset((bmy * stride) as isize);
        ((*h).pixf.fpelcmp_x4[i_pixel as usize]).expect("non-null function pointer")(
            p_fenc,
            pix_base_4
                .offset(-(1 as libc::c_int) as isize)
                .offset((-(1 as libc::c_int) * stride) as isize),
            pix_base_4
                .offset(-(1 as libc::c_int) as isize)
                .offset((1 as libc::c_int * stride) as isize),
            pix_base_4
                .offset(1 as libc::c_int as isize)
                .offset((-(1 as libc::c_int) * stride) as isize),
            pix_base_4
                .offset(1 as libc::c_int as isize)
                .offset((1 as libc::c_int * stride) as isize),
            stride as intptr_t,
            costs.as_mut_ptr(),
        );
        costs[0 as libc::c_int as usize] += *p_cost_mvx
            .offset(((bmx + -(1 as libc::c_int)) * 4 as libc::c_int) as isize)
            as libc::c_int
            + *p_cost_mvy.offset(((bmy + -(1 as libc::c_int)) * 4 as libc::c_int) as isize)
                as libc::c_int;
        costs[1 as libc::c_int as usize] += *p_cost_mvx
            .offset(((bmx + -(1 as libc::c_int)) * 4 as libc::c_int) as isize)
            as libc::c_int
            + *p_cost_mvy.offset(((bmy + 1 as libc::c_int) * 4 as libc::c_int) as isize)
                as libc::c_int;
        costs[2 as libc::c_int as usize] += *p_cost_mvx
            .offset(((bmx + 1 as libc::c_int) * 4 as libc::c_int) as isize)
            as libc::c_int
            + *p_cost_mvy.offset(((bmy + -(1 as libc::c_int)) * 4 as libc::c_int) as isize)
                as libc::c_int;
        costs[3 as libc::c_int as usize] += *p_cost_mvx
            .offset(((bmx + 1 as libc::c_int) * 4 as libc::c_int) as isize)
            as libc::c_int
            + *p_cost_mvy.offset(((bmy + 1 as libc::c_int) * 4 as libc::c_int) as isize)
                as libc::c_int;
        if ((costs[0 as libc::c_int as usize] << 4 as libc::c_int) + 5 as libc::c_int) < bcost {
            bcost = (costs[0 as libc::c_int as usize] << 4 as libc::c_int) + 5 as libc::c_int;
        }
        if ((costs[1 as libc::c_int as usize] << 4 as libc::c_int) + 6 as libc::c_int) < bcost {
            bcost = (costs[1 as libc::c_int as usize] << 4 as libc::c_int) + 6 as libc::c_int;
        }
        if ((costs[2 as libc::c_int as usize] << 4 as libc::c_int) + 7 as libc::c_int) < bcost {
            bcost = (costs[2 as libc::c_int as usize] << 4 as libc::c_int) + 7 as libc::c_int;
        }
        if ((costs[3 as libc::c_int as usize] << 4 as libc::c_int) + 8 as libc::c_int) < bcost {
            bcost = (costs[3 as libc::c_int as usize] << 4 as libc::c_int) + 8 as libc::c_int;
        }
        bmx +=
            square1[(bcost & 15 as libc::c_int) as usize][0 as libc::c_int as usize] as libc::c_int;
        bmy +=
            square1[(bcost & 15 as libc::c_int) as usize][1 as libc::c_int as usize] as libc::c_int;
        bcost >>= 4 as libc::c_int;
    }
    let mut bmv: uint32_t = pack16to32_mask(bmx, bmy);
    let mut bmv_spel: uint32_t = (bmv * 4 as libc::c_int as uint32_t) & 0xfffcfffc as libc::c_uint;
    if (*h).mb.i_subpel_refine < 3 as libc::c_int {
        (*m).cost_mv = *p_cost_mvx.offset((bmx * 4 as libc::c_int) as isize) as libc::c_int
            + *p_cost_mvy.offset((bmy * 4 as libc::c_int) as isize) as libc::c_int;
        (*m).cost = bcost;
        if bmv == pmv {
            (*m).cost += (*m).cost_mv;
        }
        (*(((*m).mv).as_mut_ptr() as *mut x264_union32_t)).i = bmv_spel;
    } else {
        (*(((*m).mv).as_mut_ptr() as *mut x264_union32_t)).i = if bpred_cost < bcost {
            bpred_mv
        } else {
            bmv_spel
        };
        (*m).cost = if bpred_cost < bcost {
            bpred_cost
        } else {
            bcost
        };
    }
    if (*h).mb.i_subpel_refine >= 2 as libc::c_int {
        let mut hpel: libc::c_int = subpel_iterations[(*h).mb.i_subpel_refine as usize]
            [2 as libc::c_int as usize] as libc::c_int;
        let mut qpel: libc::c_int = subpel_iterations[(*h).mb.i_subpel_refine as usize]
            [3 as libc::c_int as usize] as libc::c_int;
        refine_subpel(h, m, hpel, qpel, p_halfpel_thresh, 0 as libc::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_me_refine_qpel(mut h: *mut x264_t, mut m: *mut x264_me_t) {
    let mut hpel: libc::c_int = subpel_iterations[(*h).mb.i_subpel_refine as usize]
        [0 as libc::c_int as usize] as libc::c_int;
    let mut qpel: libc::c_int = subpel_iterations[(*h).mb.i_subpel_refine as usize]
        [1 as libc::c_int as usize] as libc::c_int;
    if (*m).i_pixel <= PIXEL_8x8 as libc::c_int {
        (*m).cost -= (*m).i_ref_cost;
    }
    refine_subpel(
        h,
        m,
        hpel,
        qpel,
        std::ptr::null_mut::<libc::c_int>(),
        1 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_me_refine_qpel_refdupe(
    mut h: *mut x264_t,
    mut m: *mut x264_me_t,
    mut p_halfpel_thresh: *mut libc::c_int,
) {
    refine_subpel(
        h,
        m,
        0 as libc::c_int,
        if (2 as libc::c_int)
            < subpel_iterations[(*h).mb.i_subpel_refine as usize][3 as libc::c_int as usize]
                as libc::c_int
        {
            2 as libc::c_int
        } else {
            subpel_iterations[(*h).mb.i_subpel_refine as usize][3 as libc::c_int as usize]
                as libc::c_int
        },
        p_halfpel_thresh,
        0 as libc::c_int,
    );
}
unsafe extern "C" fn refine_subpel(
    mut h: *mut x264_t,
    mut m: *mut x264_me_t,
    mut hpel_iters: libc::c_int,
    mut qpel_iters: libc::c_int,
    mut p_halfpel_thresh: *mut libc::c_int,
    mut b_refine_qpel: libc::c_int,
) {
    let bw: libc::c_int = x264_pixel_size[(*m).i_pixel as usize].w as libc::c_int;
    let bh: libc::c_int = x264_pixel_size[(*m).i_pixel as usize].h as libc::c_int;
    let mut p_cost_mvx: *const uint16_t =
        ((*m).p_cost_mv).offset(-((*m).mvp[0 as libc::c_int as usize] as libc::c_int as isize));
    let mut p_cost_mvy: *const uint16_t =
        ((*m).p_cost_mv).offset(-((*m).mvp[1 as libc::c_int as usize] as libc::c_int as isize));
    let i_pixel: libc::c_int = (*m).i_pixel;
    let b_chroma_me: libc::c_int = ((*h).mb.b_chroma_me != 0
        && (i_pixel <= PIXEL_8x8 as libc::c_int
            || (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as libc::c_int))
        as libc::c_int;
    let mut chromapix: libc::c_int = (*h).luma2chroma_pixel[i_pixel as usize] as libc::c_int;
    let mut chroma_v_shift: libc::c_int = (*h).mb.chroma_v_shift;
    let mut mvy_offset: libc::c_int = if chroma_v_shift & (*h).mb.b_interlaced & (*m).i_ref != 0 {
        ((*h).mb.i_mb_y & 1 as libc::c_int) * 4 as libc::c_int - 2 as libc::c_int
    } else {
        0 as libc::c_int
    };
    let mut pix: [pixel; 1152] = [0; 1152];
    let mut costs: [libc::c_int; 4] = [0; 4];
    let mut bmx: libc::c_int = (*m).mv[0 as libc::c_int as usize] as libc::c_int;
    let mut bmy: libc::c_int = (*m).mv[1 as libc::c_int as usize] as libc::c_int;
    let mut bcost: libc::c_int = (*m).cost;
    let mut odir: libc::c_int = -(1 as libc::c_int);
    let mut bdir: libc::c_int = 0;
    if hpel_iters != 0 {
        if (*h).mb.i_subpel_refine < 3 as libc::c_int {
            let mut mx: libc::c_int = x264_clip3(
                (*m).mvp[0 as libc::c_int as usize] as libc::c_int,
                (*h).mb.mv_min_spel[0 as libc::c_int as usize] + 2 as libc::c_int,
                (*h).mb.mv_max_spel[0 as libc::c_int as usize] - 2 as libc::c_int,
            );
            let mut my: libc::c_int = x264_clip3(
                (*m).mvp[1 as libc::c_int as usize] as libc::c_int,
                (*h).mb.mv_min_spel[1 as libc::c_int as usize] + 2 as libc::c_int,
                (*h).mb.mv_max_spel[1 as libc::c_int as usize] - 2 as libc::c_int,
            );
            if (mx - bmx) | (my - bmy) != 0 {
                let mut stride: intptr_t = 16 as libc::c_int as intptr_t;
                let mut src: *mut pixel = ((*h).mc.get_ref).expect("non-null function pointer")(
                    pix.as_mut_ptr(),
                    &mut stride,
                    ((*m).p_fref).as_mut_ptr(),
                    (*m).i_stride[0 as libc::c_int as usize] as intptr_t,
                    mx,
                    my,
                    bw,
                    bh,
                    &*((*m).weight).offset(0 as libc::c_int as isize),
                );
                let mut cost: libc::c_int = ((*h).pixf.fpelcmp[i_pixel as usize])
                    .expect("non-null function pointer")(
                    (*m).p_fenc[0 as libc::c_int as usize],
                    16 as libc::c_int as intptr_t,
                    src,
                    stride,
                ) + *p_cost_mvx.offset(mx as isize) as libc::c_int
                    + *p_cost_mvy.offset(my as isize) as libc::c_int;
                if cost < bcost {
                    bcost = cost;
                    bmx = mx;
                    bmy = my;
                }
            }
        }
        bcost <<= 6 as libc::c_int;
        let mut i: libc::c_int = hpel_iters;
        while i > 0 as libc::c_int {
            let mut omx: libc::c_int = bmx;
            let mut omy: libc::c_int = bmy;
            let mut stride_0: intptr_t = 64 as libc::c_int as intptr_t;
            let mut src0: *mut pixel = std::ptr::null_mut::<pixel>();
            let mut src1: *mut pixel = std::ptr::null_mut::<pixel>();
            let mut src2: *mut pixel = std::ptr::null_mut::<pixel>();
            let mut src3: *mut pixel = std::ptr::null_mut::<pixel>();
            src0 = ((*h).mc.get_ref).expect("non-null function pointer")(
                pix.as_mut_ptr(),
                &mut stride_0,
                ((*m).p_fref).as_mut_ptr(),
                (*m).i_stride[0 as libc::c_int as usize] as intptr_t,
                omx,
                omy - 2 as libc::c_int,
                bw,
                bh + 1 as libc::c_int,
                &*((*m).weight).offset(0 as libc::c_int as isize),
            );
            src2 = ((*h).mc.get_ref).expect("non-null function pointer")(
                pix.as_mut_ptr().offset(32 as libc::c_int as isize),
                &mut stride_0,
                ((*m).p_fref).as_mut_ptr(),
                (*m).i_stride[0 as libc::c_int as usize] as intptr_t,
                omx - 2 as libc::c_int,
                omy,
                bw + 4 as libc::c_int,
                bh,
                &*((*m).weight).offset(0 as libc::c_int as isize),
            );
            src1 = src0.offset(stride_0 as isize);
            src3 = src2.offset(1 as libc::c_int as isize);
            ((*h).pixf.fpelcmp_x4[i_pixel as usize]).expect("non-null function pointer")(
                (*m).p_fenc[0 as libc::c_int as usize],
                src0,
                src1,
                src2,
                src3,
                stride_0,
                costs.as_mut_ptr(),
            );
            costs[0 as libc::c_int as usize] += *p_cost_mvx.offset(omx as isize) as libc::c_int
                + *p_cost_mvy.offset((omy - 2 as libc::c_int) as isize) as libc::c_int;
            costs[1 as libc::c_int as usize] += *p_cost_mvx.offset(omx as isize) as libc::c_int
                + *p_cost_mvy.offset((omy + 2 as libc::c_int) as isize) as libc::c_int;
            costs[2 as libc::c_int as usize] +=
                *p_cost_mvx.offset((omx - 2 as libc::c_int) as isize) as libc::c_int
                    + *p_cost_mvy.offset(omy as isize) as libc::c_int;
            costs[3 as libc::c_int as usize] +=
                *p_cost_mvx.offset((omx + 2 as libc::c_int) as isize) as libc::c_int
                    + *p_cost_mvy.offset(omy as isize) as libc::c_int;
            if ((costs[0 as libc::c_int as usize] << 6 as libc::c_int) + 2 as libc::c_int) < bcost {
                bcost = (costs[0 as libc::c_int as usize] << 6 as libc::c_int) + 2 as libc::c_int;
            }
            if ((costs[1 as libc::c_int as usize] << 6 as libc::c_int) + 6 as libc::c_int) < bcost {
                bcost = (costs[1 as libc::c_int as usize] << 6 as libc::c_int) + 6 as libc::c_int;
            }
            if ((costs[2 as libc::c_int as usize] << 6 as libc::c_int) + 16 as libc::c_int) < bcost
            {
                bcost = (costs[2 as libc::c_int as usize] << 6 as libc::c_int) + 16 as libc::c_int;
            }
            if ((costs[3 as libc::c_int as usize] << 6 as libc::c_int) + 48 as libc::c_int) < bcost
            {
                bcost = (costs[3 as libc::c_int as usize] << 6 as libc::c_int) + 48 as libc::c_int;
            }
            if bcost & 63 as libc::c_int == 0 {
                break;
            }
            bmx -= ((bcost as uint32_t) << 26 as libc::c_int) as int32_t >> 29 as libc::c_int;
            bmy -= ((bcost as uint32_t) << 29 as libc::c_int) as int32_t >> 29 as libc::c_int;
            bcost &= !(63 as libc::c_int);
            i -= 1;
            i;
        }
        bcost >>= 6 as libc::c_int;
    }
    if b_refine_qpel == 0
        && ((*h).pixf.mbcmp_unaligned[0 as libc::c_int as usize]
            != (*h).pixf.fpelcmp[0 as libc::c_int as usize]
            || b_chroma_me != 0)
    {
        bcost = (1 as libc::c_int) << 28 as libc::c_int;
        if b_refine_qpel != 0 || -(1 as libc::c_int) ^ 1 as libc::c_int != odir {
            let mut stride_1: intptr_t = 16 as libc::c_int as intptr_t;
            let mut src_0: *mut pixel = ((*h).mc.get_ref).expect("non-null function pointer")(
                pix.as_mut_ptr(),
                &mut stride_1,
                &mut *((*m).p_fref).as_mut_ptr().offset(0 as libc::c_int as isize),
                (*m).i_stride[0 as libc::c_int as usize] as intptr_t,
                bmx,
                bmy,
                bw,
                bh,
                &*((*m).weight).offset(0 as libc::c_int as isize),
            );
            let mut cost_0: libc::c_int = ((*h).pixf.mbcmp_unaligned[i_pixel as usize])
                .expect("non-null function pointer")(
                (*m).p_fenc[0 as libc::c_int as usize],
                16 as libc::c_int as intptr_t,
                src_0,
                stride_1,
            ) + *p_cost_mvx.offset(bmx as isize) as libc::c_int
                + *p_cost_mvy.offset(bmy as isize) as libc::c_int;
            if b_chroma_me != 0 && cost_0 < bcost {
                if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as libc::c_int {
                    stride_1 = 16 as libc::c_int as intptr_t;
                    src_0 = ((*h).mc.get_ref).expect("non-null function pointer")(
                        pix.as_mut_ptr(),
                        &mut stride_1,
                        &mut *((*m).p_fref).as_mut_ptr().offset(4 as libc::c_int as isize),
                        (*m).i_stride[1 as libc::c_int as usize] as intptr_t,
                        bmx,
                        bmy,
                        bw,
                        bh,
                        &*((*m).weight).offset(1 as libc::c_int as isize),
                    );
                    cost_0 += ((*h).pixf.mbcmp_unaligned[i_pixel as usize])
                        .expect("non-null function pointer")(
                        (*m).p_fenc[1 as libc::c_int as usize],
                        16 as libc::c_int as intptr_t,
                        src_0,
                        stride_1,
                    );
                    if cost_0 < bcost {
                        stride_1 = 16 as libc::c_int as intptr_t;
                        src_0 = ((*h).mc.get_ref).expect("non-null function pointer")(
                            pix.as_mut_ptr(),
                            &mut stride_1,
                            &mut *((*m).p_fref).as_mut_ptr().offset(8 as libc::c_int as isize),
                            (*m).i_stride[2 as libc::c_int as usize] as intptr_t,
                            bmx,
                            bmy,
                            bw,
                            bh,
                            &*((*m).weight).offset(2 as libc::c_int as isize),
                        );
                        cost_0 += ((*h).pixf.mbcmp_unaligned[i_pixel as usize])
                            .expect("non-null function pointer")(
                            (*m).p_fenc[2 as libc::c_int as usize],
                            16 as libc::c_int as intptr_t,
                            src_0,
                            stride_1,
                        );
                    }
                } else {
                    ((*h).mc.mc_chroma).expect("non-null function pointer")(
                        pix.as_mut_ptr(),
                        pix.as_mut_ptr().offset(8 as libc::c_int as isize),
                        16 as libc::c_int as intptr_t,
                        (*m).p_fref[4 as libc::c_int as usize],
                        (*m).i_stride[1 as libc::c_int as usize] as intptr_t,
                        bmx,
                        (2 as libc::c_int * (bmy + mvy_offset)) >> chroma_v_shift,
                        bw >> 1 as libc::c_int,
                        bh >> chroma_v_shift,
                    );
                    if !((*((*m).weight).offset(1 as libc::c_int as isize)).weightfn).is_null() {
                        (*((*((*m).weight).offset(1 as libc::c_int as isize)).weightfn)
                            .offset((bw >> 3 as libc::c_int) as isize))
                        .expect("non-null function pointer")(
                            pix.as_mut_ptr(),
                            16 as libc::c_int as intptr_t,
                            pix.as_mut_ptr(),
                            16 as libc::c_int as intptr_t,
                            &*((*m).weight).offset(1 as libc::c_int as isize),
                            bh >> chroma_v_shift,
                        );
                    }
                    cost_0 += ((*h).pixf.mbcmp[chromapix as usize])
                        .expect("non-null function pointer")(
                        (*m).p_fenc[1 as libc::c_int as usize],
                        16 as libc::c_int as intptr_t,
                        pix.as_mut_ptr(),
                        16 as libc::c_int as intptr_t,
                    );
                    if cost_0 < bcost {
                        if !((*((*m).weight).offset(2 as libc::c_int as isize)).weightfn).is_null()
                        {
                            (*((*((*m).weight).offset(2 as libc::c_int as isize)).weightfn)
                                .offset((bw >> 3 as libc::c_int) as isize))
                            .expect("non-null function pointer")(
                                pix.as_mut_ptr().offset(8 as libc::c_int as isize),
                                16 as libc::c_int as intptr_t,
                                pix.as_mut_ptr().offset(8 as libc::c_int as isize),
                                16 as libc::c_int as intptr_t,
                                &*((*m).weight).offset(2 as libc::c_int as isize),
                                bh >> chroma_v_shift,
                            );
                        }
                        cost_0 += ((*h).pixf.mbcmp[chromapix as usize])
                            .expect("non-null function pointer")(
                            (*m).p_fenc[2 as libc::c_int as usize],
                            16 as libc::c_int as intptr_t,
                            pix.as_mut_ptr().offset(8 as libc::c_int as isize),
                            16 as libc::c_int as intptr_t,
                        );
                    }
                }
            }
            if cost_0 < bcost {
                bcost = cost_0;
                bmx = bmx;
                bmy = bmy;
                bdir = -(1 as libc::c_int);
            }
        }
    }
    if !p_halfpel_thresh.is_null() {
        if (bcost * 7 as libc::c_int) >> 3 as libc::c_int > *p_halfpel_thresh {
            (*m).cost = bcost;
            (*m).mv[0 as libc::c_int as usize] = bmx as int16_t;
            (*m).mv[1 as libc::c_int as usize] = bmy as int16_t;
            return;
        } else if bcost < *p_halfpel_thresh {
            *p_halfpel_thresh = bcost;
        }
    }
    if (*h).mb.i_subpel_refine != 1 as libc::c_int {
        bdir = -(1 as libc::c_int);
        let mut i_0: libc::c_int = qpel_iters;
        while i_0 > 0 as libc::c_int {
            if bmy <= (*h).mb.mv_min_spel[1 as libc::c_int as usize]
                || bmy >= (*h).mb.mv_max_spel[1 as libc::c_int as usize]
                || bmx <= (*h).mb.mv_min_spel[0 as libc::c_int as usize]
                || bmx >= (*h).mb.mv_max_spel[0 as libc::c_int as usize]
            {
                break;
            }
            odir = bdir;
            let mut omx_0: libc::c_int = bmx;
            let mut omy_0: libc::c_int = bmy;
            if b_refine_qpel != 0 || 0 as libc::c_int ^ 1 as libc::c_int != odir {
                let mut stride_2: intptr_t = 16 as libc::c_int as intptr_t;
                let mut src_1: *mut pixel = ((*h).mc.get_ref).expect("non-null function pointer")(
                    pix.as_mut_ptr(),
                    &mut stride_2,
                    &mut *((*m).p_fref).as_mut_ptr().offset(0 as libc::c_int as isize),
                    (*m).i_stride[0 as libc::c_int as usize] as intptr_t,
                    omx_0,
                    omy_0 - 1 as libc::c_int,
                    bw,
                    bh,
                    &*((*m).weight).offset(0 as libc::c_int as isize),
                );
                let mut cost_1: libc::c_int = ((*h).pixf.mbcmp_unaligned[i_pixel as usize])
                    .expect("non-null function pointer")(
                    (*m).p_fenc[0 as libc::c_int as usize],
                    16 as libc::c_int as intptr_t,
                    src_1,
                    stride_2,
                ) + *p_cost_mvx.offset(omx_0 as isize) as libc::c_int
                    + *p_cost_mvy.offset((omy_0 - 1 as libc::c_int) as isize) as libc::c_int;
                if b_chroma_me != 0 && cost_1 < bcost {
                    if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as libc::c_int {
                        stride_2 = 16 as libc::c_int as intptr_t;
                        src_1 = ((*h).mc.get_ref).expect("non-null function pointer")(
                            pix.as_mut_ptr(),
                            &mut stride_2,
                            &mut *((*m).p_fref).as_mut_ptr().offset(4 as libc::c_int as isize),
                            (*m).i_stride[1 as libc::c_int as usize] as intptr_t,
                            omx_0,
                            omy_0 - 1 as libc::c_int,
                            bw,
                            bh,
                            &*((*m).weight).offset(1 as libc::c_int as isize),
                        );
                        cost_1 += ((*h).pixf.mbcmp_unaligned[i_pixel as usize])
                            .expect("non-null function pointer")(
                            (*m).p_fenc[1 as libc::c_int as usize],
                            16 as libc::c_int as intptr_t,
                            src_1,
                            stride_2,
                        );
                        if cost_1 < bcost {
                            stride_2 = 16 as libc::c_int as intptr_t;
                            src_1 = ((*h).mc.get_ref).expect("non-null function pointer")(
                                pix.as_mut_ptr(),
                                &mut stride_2,
                                &mut *((*m).p_fref).as_mut_ptr().offset(8 as libc::c_int as isize),
                                (*m).i_stride[2 as libc::c_int as usize] as intptr_t,
                                omx_0,
                                omy_0 - 1 as libc::c_int,
                                bw,
                                bh,
                                &*((*m).weight).offset(2 as libc::c_int as isize),
                            );
                            cost_1 += ((*h).pixf.mbcmp_unaligned[i_pixel as usize])
                                .expect("non-null function pointer")(
                                (*m).p_fenc[2 as libc::c_int as usize],
                                16 as libc::c_int as intptr_t,
                                src_1,
                                stride_2,
                            );
                        }
                    } else {
                        ((*h).mc.mc_chroma).expect("non-null function pointer")(
                            pix.as_mut_ptr(),
                            pix.as_mut_ptr().offset(8 as libc::c_int as isize),
                            16 as libc::c_int as intptr_t,
                            (*m).p_fref[4 as libc::c_int as usize],
                            (*m).i_stride[1 as libc::c_int as usize] as intptr_t,
                            omx_0,
                            (2 as libc::c_int * (omy_0 - 1 as libc::c_int + mvy_offset))
                                >> chroma_v_shift,
                            bw >> 1 as libc::c_int,
                            bh >> chroma_v_shift,
                        );
                        if !((*((*m).weight).offset(1 as libc::c_int as isize)).weightfn).is_null()
                        {
                            (*((*((*m).weight).offset(1 as libc::c_int as isize)).weightfn)
                                .offset((bw >> 3 as libc::c_int) as isize))
                            .expect("non-null function pointer")(
                                pix.as_mut_ptr(),
                                16 as libc::c_int as intptr_t,
                                pix.as_mut_ptr(),
                                16 as libc::c_int as intptr_t,
                                &*((*m).weight).offset(1 as libc::c_int as isize),
                                bh >> chroma_v_shift,
                            );
                        }
                        cost_1 += ((*h).pixf.mbcmp[chromapix as usize])
                            .expect("non-null function pointer")(
                            (*m).p_fenc[1 as libc::c_int as usize],
                            16 as libc::c_int as intptr_t,
                            pix.as_mut_ptr(),
                            16 as libc::c_int as intptr_t,
                        );
                        if cost_1 < bcost {
                            if !((*((*m).weight).offset(2 as libc::c_int as isize)).weightfn)
                                .is_null()
                            {
                                (*((*((*m).weight).offset(2 as libc::c_int as isize)).weightfn)
                                    .offset((bw >> 3 as libc::c_int) as isize))
                                .expect("non-null function pointer")(
                                    pix.as_mut_ptr().offset(8 as libc::c_int as isize),
                                    16 as libc::c_int as intptr_t,
                                    pix.as_mut_ptr().offset(8 as libc::c_int as isize),
                                    16 as libc::c_int as intptr_t,
                                    &*((*m).weight).offset(2 as libc::c_int as isize),
                                    bh >> chroma_v_shift,
                                );
                            }
                            cost_1 += ((*h).pixf.mbcmp[chromapix as usize])
                                .expect("non-null function pointer")(
                                (*m).p_fenc[2 as libc::c_int as usize],
                                16 as libc::c_int as intptr_t,
                                pix.as_mut_ptr().offset(8 as libc::c_int as isize),
                                16 as libc::c_int as intptr_t,
                            );
                        }
                    }
                }
                if cost_1 < bcost {
                    bcost = cost_1;
                    bmx = omx_0;
                    bmy = omy_0 - 1 as libc::c_int;
                    bdir = 0 as libc::c_int;
                }
            }
            if b_refine_qpel != 0 || 1 as libc::c_int ^ 1 as libc::c_int != odir {
                let mut stride_3: intptr_t = 16 as libc::c_int as intptr_t;
                let mut src_2: *mut pixel = ((*h).mc.get_ref).expect("non-null function pointer")(
                    pix.as_mut_ptr(),
                    &mut stride_3,
                    &mut *((*m).p_fref).as_mut_ptr().offset(0 as libc::c_int as isize),
                    (*m).i_stride[0 as libc::c_int as usize] as intptr_t,
                    omx_0,
                    omy_0 + 1 as libc::c_int,
                    bw,
                    bh,
                    &*((*m).weight).offset(0 as libc::c_int as isize),
                );
                let mut cost_2: libc::c_int = ((*h).pixf.mbcmp_unaligned[i_pixel as usize])
                    .expect("non-null function pointer")(
                    (*m).p_fenc[0 as libc::c_int as usize],
                    16 as libc::c_int as intptr_t,
                    src_2,
                    stride_3,
                ) + *p_cost_mvx.offset(omx_0 as isize) as libc::c_int
                    + *p_cost_mvy.offset((omy_0 + 1 as libc::c_int) as isize) as libc::c_int;
                if b_chroma_me != 0 && cost_2 < bcost {
                    if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as libc::c_int {
                        stride_3 = 16 as libc::c_int as intptr_t;
                        src_2 = ((*h).mc.get_ref).expect("non-null function pointer")(
                            pix.as_mut_ptr(),
                            &mut stride_3,
                            &mut *((*m).p_fref).as_mut_ptr().offset(4 as libc::c_int as isize),
                            (*m).i_stride[1 as libc::c_int as usize] as intptr_t,
                            omx_0,
                            omy_0 + 1 as libc::c_int,
                            bw,
                            bh,
                            &*((*m).weight).offset(1 as libc::c_int as isize),
                        );
                        cost_2 += ((*h).pixf.mbcmp_unaligned[i_pixel as usize])
                            .expect("non-null function pointer")(
                            (*m).p_fenc[1 as libc::c_int as usize],
                            16 as libc::c_int as intptr_t,
                            src_2,
                            stride_3,
                        );
                        if cost_2 < bcost {
                            stride_3 = 16 as libc::c_int as intptr_t;
                            src_2 = ((*h).mc.get_ref).expect("non-null function pointer")(
                                pix.as_mut_ptr(),
                                &mut stride_3,
                                &mut *((*m).p_fref).as_mut_ptr().offset(8 as libc::c_int as isize),
                                (*m).i_stride[2 as libc::c_int as usize] as intptr_t,
                                omx_0,
                                omy_0 + 1 as libc::c_int,
                                bw,
                                bh,
                                &*((*m).weight).offset(2 as libc::c_int as isize),
                            );
                            cost_2 += ((*h).pixf.mbcmp_unaligned[i_pixel as usize])
                                .expect("non-null function pointer")(
                                (*m).p_fenc[2 as libc::c_int as usize],
                                16 as libc::c_int as intptr_t,
                                src_2,
                                stride_3,
                            );
                        }
                    } else {
                        ((*h).mc.mc_chroma).expect("non-null function pointer")(
                            pix.as_mut_ptr(),
                            pix.as_mut_ptr().offset(8 as libc::c_int as isize),
                            16 as libc::c_int as intptr_t,
                            (*m).p_fref[4 as libc::c_int as usize],
                            (*m).i_stride[1 as libc::c_int as usize] as intptr_t,
                            omx_0,
                            (2 as libc::c_int * (omy_0 + 1 as libc::c_int + mvy_offset))
                                >> chroma_v_shift,
                            bw >> 1 as libc::c_int,
                            bh >> chroma_v_shift,
                        );
                        if !((*((*m).weight).offset(1 as libc::c_int as isize)).weightfn).is_null()
                        {
                            (*((*((*m).weight).offset(1 as libc::c_int as isize)).weightfn)
                                .offset((bw >> 3 as libc::c_int) as isize))
                            .expect("non-null function pointer")(
                                pix.as_mut_ptr(),
                                16 as libc::c_int as intptr_t,
                                pix.as_mut_ptr(),
                                16 as libc::c_int as intptr_t,
                                &*((*m).weight).offset(1 as libc::c_int as isize),
                                bh >> chroma_v_shift,
                            );
                        }
                        cost_2 += ((*h).pixf.mbcmp[chromapix as usize])
                            .expect("non-null function pointer")(
                            (*m).p_fenc[1 as libc::c_int as usize],
                            16 as libc::c_int as intptr_t,
                            pix.as_mut_ptr(),
                            16 as libc::c_int as intptr_t,
                        );
                        if cost_2 < bcost {
                            if !((*((*m).weight).offset(2 as libc::c_int as isize)).weightfn)
                                .is_null()
                            {
                                (*((*((*m).weight).offset(2 as libc::c_int as isize)).weightfn)
                                    .offset((bw >> 3 as libc::c_int) as isize))
                                .expect("non-null function pointer")(
                                    pix.as_mut_ptr().offset(8 as libc::c_int as isize),
                                    16 as libc::c_int as intptr_t,
                                    pix.as_mut_ptr().offset(8 as libc::c_int as isize),
                                    16 as libc::c_int as intptr_t,
                                    &*((*m).weight).offset(2 as libc::c_int as isize),
                                    bh >> chroma_v_shift,
                                );
                            }
                            cost_2 += ((*h).pixf.mbcmp[chromapix as usize])
                                .expect("non-null function pointer")(
                                (*m).p_fenc[2 as libc::c_int as usize],
                                16 as libc::c_int as intptr_t,
                                pix.as_mut_ptr().offset(8 as libc::c_int as isize),
                                16 as libc::c_int as intptr_t,
                            );
                        }
                    }
                }
                if cost_2 < bcost {
                    bcost = cost_2;
                    bmx = omx_0;
                    bmy = omy_0 + 1 as libc::c_int;
                    bdir = 1 as libc::c_int;
                }
            }
            if b_refine_qpel != 0 || 2 as libc::c_int ^ 1 as libc::c_int != odir {
                let mut stride_4: intptr_t = 16 as libc::c_int as intptr_t;
                let mut src_3: *mut pixel = ((*h).mc.get_ref).expect("non-null function pointer")(
                    pix.as_mut_ptr(),
                    &mut stride_4,
                    &mut *((*m).p_fref).as_mut_ptr().offset(0 as libc::c_int as isize),
                    (*m).i_stride[0 as libc::c_int as usize] as intptr_t,
                    omx_0 - 1 as libc::c_int,
                    omy_0,
                    bw,
                    bh,
                    &*((*m).weight).offset(0 as libc::c_int as isize),
                );
                let mut cost_3: libc::c_int =
                    ((*h).pixf.mbcmp_unaligned[i_pixel as usize])
                        .expect("non-null function pointer")(
                        (*m).p_fenc[0 as libc::c_int as usize],
                        16 as libc::c_int as intptr_t,
                        src_3,
                        stride_4,
                    ) + *p_cost_mvx.offset((omx_0 - 1 as libc::c_int) as isize) as libc::c_int
                        + *p_cost_mvy.offset(omy_0 as isize) as libc::c_int;
                if b_chroma_me != 0 && cost_3 < bcost {
                    if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as libc::c_int {
                        stride_4 = 16 as libc::c_int as intptr_t;
                        src_3 = ((*h).mc.get_ref).expect("non-null function pointer")(
                            pix.as_mut_ptr(),
                            &mut stride_4,
                            &mut *((*m).p_fref).as_mut_ptr().offset(4 as libc::c_int as isize),
                            (*m).i_stride[1 as libc::c_int as usize] as intptr_t,
                            omx_0 - 1 as libc::c_int,
                            omy_0,
                            bw,
                            bh,
                            &*((*m).weight).offset(1 as libc::c_int as isize),
                        );
                        cost_3 += ((*h).pixf.mbcmp_unaligned[i_pixel as usize])
                            .expect("non-null function pointer")(
                            (*m).p_fenc[1 as libc::c_int as usize],
                            16 as libc::c_int as intptr_t,
                            src_3,
                            stride_4,
                        );
                        if cost_3 < bcost {
                            stride_4 = 16 as libc::c_int as intptr_t;
                            src_3 = ((*h).mc.get_ref).expect("non-null function pointer")(
                                pix.as_mut_ptr(),
                                &mut stride_4,
                                &mut *((*m).p_fref).as_mut_ptr().offset(8 as libc::c_int as isize),
                                (*m).i_stride[2 as libc::c_int as usize] as intptr_t,
                                omx_0 - 1 as libc::c_int,
                                omy_0,
                                bw,
                                bh,
                                &*((*m).weight).offset(2 as libc::c_int as isize),
                            );
                            cost_3 += ((*h).pixf.mbcmp_unaligned[i_pixel as usize])
                                .expect("non-null function pointer")(
                                (*m).p_fenc[2 as libc::c_int as usize],
                                16 as libc::c_int as intptr_t,
                                src_3,
                                stride_4,
                            );
                        }
                    } else {
                        ((*h).mc.mc_chroma).expect("non-null function pointer")(
                            pix.as_mut_ptr(),
                            pix.as_mut_ptr().offset(8 as libc::c_int as isize),
                            16 as libc::c_int as intptr_t,
                            (*m).p_fref[4 as libc::c_int as usize],
                            (*m).i_stride[1 as libc::c_int as usize] as intptr_t,
                            omx_0 - 1 as libc::c_int,
                            (2 as libc::c_int * (omy_0 + mvy_offset)) >> chroma_v_shift,
                            bw >> 1 as libc::c_int,
                            bh >> chroma_v_shift,
                        );
                        if !((*((*m).weight).offset(1 as libc::c_int as isize)).weightfn).is_null()
                        {
                            (*((*((*m).weight).offset(1 as libc::c_int as isize)).weightfn)
                                .offset((bw >> 3 as libc::c_int) as isize))
                            .expect("non-null function pointer")(
                                pix.as_mut_ptr(),
                                16 as libc::c_int as intptr_t,
                                pix.as_mut_ptr(),
                                16 as libc::c_int as intptr_t,
                                &*((*m).weight).offset(1 as libc::c_int as isize),
                                bh >> chroma_v_shift,
                            );
                        }
                        cost_3 += ((*h).pixf.mbcmp[chromapix as usize])
                            .expect("non-null function pointer")(
                            (*m).p_fenc[1 as libc::c_int as usize],
                            16 as libc::c_int as intptr_t,
                            pix.as_mut_ptr(),
                            16 as libc::c_int as intptr_t,
                        );
                        if cost_3 < bcost {
                            if !((*((*m).weight).offset(2 as libc::c_int as isize)).weightfn)
                                .is_null()
                            {
                                (*((*((*m).weight).offset(2 as libc::c_int as isize)).weightfn)
                                    .offset((bw >> 3 as libc::c_int) as isize))
                                .expect("non-null function pointer")(
                                    pix.as_mut_ptr().offset(8 as libc::c_int as isize),
                                    16 as libc::c_int as intptr_t,
                                    pix.as_mut_ptr().offset(8 as libc::c_int as isize),
                                    16 as libc::c_int as intptr_t,
                                    &*((*m).weight).offset(2 as libc::c_int as isize),
                                    bh >> chroma_v_shift,
                                );
                            }
                            cost_3 += ((*h).pixf.mbcmp[chromapix as usize])
                                .expect("non-null function pointer")(
                                (*m).p_fenc[2 as libc::c_int as usize],
                                16 as libc::c_int as intptr_t,
                                pix.as_mut_ptr().offset(8 as libc::c_int as isize),
                                16 as libc::c_int as intptr_t,
                            );
                        }
                    }
                }
                if cost_3 < bcost {
                    bcost = cost_3;
                    bmx = omx_0 - 1 as libc::c_int;
                    bmy = omy_0;
                    bdir = 2 as libc::c_int;
                }
            }
            if b_refine_qpel != 0 || 3 as libc::c_int ^ 1 as libc::c_int != odir {
                let mut stride_5: intptr_t = 16 as libc::c_int as intptr_t;
                let mut src_4: *mut pixel = ((*h).mc.get_ref).expect("non-null function pointer")(
                    pix.as_mut_ptr(),
                    &mut stride_5,
                    &mut *((*m).p_fref).as_mut_ptr().offset(0 as libc::c_int as isize),
                    (*m).i_stride[0 as libc::c_int as usize] as intptr_t,
                    omx_0 + 1 as libc::c_int,
                    omy_0,
                    bw,
                    bh,
                    &*((*m).weight).offset(0 as libc::c_int as isize),
                );
                let mut cost_4: libc::c_int =
                    ((*h).pixf.mbcmp_unaligned[i_pixel as usize])
                        .expect("non-null function pointer")(
                        (*m).p_fenc[0 as libc::c_int as usize],
                        16 as libc::c_int as intptr_t,
                        src_4,
                        stride_5,
                    ) + *p_cost_mvx.offset((omx_0 + 1 as libc::c_int) as isize) as libc::c_int
                        + *p_cost_mvy.offset(omy_0 as isize) as libc::c_int;
                if b_chroma_me != 0 && cost_4 < bcost {
                    if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as libc::c_int {
                        stride_5 = 16 as libc::c_int as intptr_t;
                        src_4 = ((*h).mc.get_ref).expect("non-null function pointer")(
                            pix.as_mut_ptr(),
                            &mut stride_5,
                            &mut *((*m).p_fref).as_mut_ptr().offset(4 as libc::c_int as isize),
                            (*m).i_stride[1 as libc::c_int as usize] as intptr_t,
                            omx_0 + 1 as libc::c_int,
                            omy_0,
                            bw,
                            bh,
                            &*((*m).weight).offset(1 as libc::c_int as isize),
                        );
                        cost_4 += ((*h).pixf.mbcmp_unaligned[i_pixel as usize])
                            .expect("non-null function pointer")(
                            (*m).p_fenc[1 as libc::c_int as usize],
                            16 as libc::c_int as intptr_t,
                            src_4,
                            stride_5,
                        );
                        if cost_4 < bcost {
                            stride_5 = 16 as libc::c_int as intptr_t;
                            src_4 = ((*h).mc.get_ref).expect("non-null function pointer")(
                                pix.as_mut_ptr(),
                                &mut stride_5,
                                &mut *((*m).p_fref).as_mut_ptr().offset(8 as libc::c_int as isize),
                                (*m).i_stride[2 as libc::c_int as usize] as intptr_t,
                                omx_0 + 1 as libc::c_int,
                                omy_0,
                                bw,
                                bh,
                                &*((*m).weight).offset(2 as libc::c_int as isize),
                            );
                            cost_4 += ((*h).pixf.mbcmp_unaligned[i_pixel as usize])
                                .expect("non-null function pointer")(
                                (*m).p_fenc[2 as libc::c_int as usize],
                                16 as libc::c_int as intptr_t,
                                src_4,
                                stride_5,
                            );
                        }
                    } else {
                        ((*h).mc.mc_chroma).expect("non-null function pointer")(
                            pix.as_mut_ptr(),
                            pix.as_mut_ptr().offset(8 as libc::c_int as isize),
                            16 as libc::c_int as intptr_t,
                            (*m).p_fref[4 as libc::c_int as usize],
                            (*m).i_stride[1 as libc::c_int as usize] as intptr_t,
                            omx_0 + 1 as libc::c_int,
                            (2 as libc::c_int * (omy_0 + mvy_offset)) >> chroma_v_shift,
                            bw >> 1 as libc::c_int,
                            bh >> chroma_v_shift,
                        );
                        if !((*((*m).weight).offset(1 as libc::c_int as isize)).weightfn).is_null()
                        {
                            (*((*((*m).weight).offset(1 as libc::c_int as isize)).weightfn)
                                .offset((bw >> 3 as libc::c_int) as isize))
                            .expect("non-null function pointer")(
                                pix.as_mut_ptr(),
                                16 as libc::c_int as intptr_t,
                                pix.as_mut_ptr(),
                                16 as libc::c_int as intptr_t,
                                &*((*m).weight).offset(1 as libc::c_int as isize),
                                bh >> chroma_v_shift,
                            );
                        }
                        cost_4 += ((*h).pixf.mbcmp[chromapix as usize])
                            .expect("non-null function pointer")(
                            (*m).p_fenc[1 as libc::c_int as usize],
                            16 as libc::c_int as intptr_t,
                            pix.as_mut_ptr(),
                            16 as libc::c_int as intptr_t,
                        );
                        if cost_4 < bcost {
                            if !((*((*m).weight).offset(2 as libc::c_int as isize)).weightfn)
                                .is_null()
                            {
                                (*((*((*m).weight).offset(2 as libc::c_int as isize)).weightfn)
                                    .offset((bw >> 3 as libc::c_int) as isize))
                                .expect("non-null function pointer")(
                                    pix.as_mut_ptr().offset(8 as libc::c_int as isize),
                                    16 as libc::c_int as intptr_t,
                                    pix.as_mut_ptr().offset(8 as libc::c_int as isize),
                                    16 as libc::c_int as intptr_t,
                                    &*((*m).weight).offset(2 as libc::c_int as isize),
                                    bh >> chroma_v_shift,
                                );
                            }
                            cost_4 += ((*h).pixf.mbcmp[chromapix as usize])
                                .expect("non-null function pointer")(
                                (*m).p_fenc[2 as libc::c_int as usize],
                                16 as libc::c_int as intptr_t,
                                pix.as_mut_ptr().offset(8 as libc::c_int as isize),
                                16 as libc::c_int as intptr_t,
                            );
                        }
                    }
                }
                if cost_4 < bcost {
                    bcost = cost_4;
                    bmx = omx_0 + 1 as libc::c_int;
                    bmy = omy_0;
                    bdir = 3 as libc::c_int;
                }
            }
            if (bmx == omx_0) as libc::c_int & (bmy == omy_0) as libc::c_int != 0 {
                break;
            }
            i_0 -= 1;
            i_0;
        }
    } else if bmy > (*h).mb.mv_min_spel[1 as libc::c_int as usize]
        && bmy < (*h).mb.mv_max_spel[1 as libc::c_int as usize]
        && bmx > (*h).mb.mv_min_spel[0 as libc::c_int as usize]
        && bmx < (*h).mb.mv_max_spel[0 as libc::c_int as usize]
    {
        let mut omx_1: libc::c_int = bmx;
        let mut omy_1: libc::c_int = bmy;
        ((*h).mc.mc_luma).expect("non-null function pointer")(
            pix.as_mut_ptr(),
            64 as libc::c_int as intptr_t,
            ((*m).p_fref).as_mut_ptr(),
            (*m).i_stride[0 as libc::c_int as usize] as intptr_t,
            omx_1,
            omy_1 - 1 as libc::c_int,
            bw,
            bh,
            &*((*m).weight).offset(0 as libc::c_int as isize),
        );
        ((*h).mc.mc_luma).expect("non-null function pointer")(
            pix.as_mut_ptr().offset(16 as libc::c_int as isize),
            64 as libc::c_int as intptr_t,
            ((*m).p_fref).as_mut_ptr(),
            (*m).i_stride[0 as libc::c_int as usize] as intptr_t,
            omx_1,
            omy_1 + 1 as libc::c_int,
            bw,
            bh,
            &*((*m).weight).offset(0 as libc::c_int as isize),
        );
        ((*h).mc.mc_luma).expect("non-null function pointer")(
            pix.as_mut_ptr().offset(32 as libc::c_int as isize),
            64 as libc::c_int as intptr_t,
            ((*m).p_fref).as_mut_ptr(),
            (*m).i_stride[0 as libc::c_int as usize] as intptr_t,
            omx_1 - 1 as libc::c_int,
            omy_1,
            bw,
            bh,
            &*((*m).weight).offset(0 as libc::c_int as isize),
        );
        ((*h).mc.mc_luma).expect("non-null function pointer")(
            pix.as_mut_ptr().offset(48 as libc::c_int as isize),
            64 as libc::c_int as intptr_t,
            ((*m).p_fref).as_mut_ptr(),
            (*m).i_stride[0 as libc::c_int as usize] as intptr_t,
            omx_1 + 1 as libc::c_int,
            omy_1,
            bw,
            bh,
            &*((*m).weight).offset(0 as libc::c_int as isize),
        );
        ((*h).pixf.fpelcmp_x4[i_pixel as usize]).expect("non-null function pointer")(
            (*m).p_fenc[0 as libc::c_int as usize],
            pix.as_mut_ptr(),
            pix.as_mut_ptr().offset(16 as libc::c_int as isize),
            pix.as_mut_ptr().offset(32 as libc::c_int as isize),
            pix.as_mut_ptr().offset(48 as libc::c_int as isize),
            64 as libc::c_int as intptr_t,
            costs.as_mut_ptr(),
        );
        costs[0 as libc::c_int as usize] += *p_cost_mvx.offset(omx_1 as isize) as libc::c_int
            + *p_cost_mvy.offset((omy_1 - 1 as libc::c_int) as isize) as libc::c_int;
        costs[1 as libc::c_int as usize] += *p_cost_mvx.offset(omx_1 as isize) as libc::c_int
            + *p_cost_mvy.offset((omy_1 + 1 as libc::c_int) as isize) as libc::c_int;
        costs[2 as libc::c_int as usize] += *p_cost_mvx.offset((omx_1 - 1 as libc::c_int) as isize)
            as libc::c_int
            + *p_cost_mvy.offset(omy_1 as isize) as libc::c_int;
        costs[3 as libc::c_int as usize] += *p_cost_mvx.offset((omx_1 + 1 as libc::c_int) as isize)
            as libc::c_int
            + *p_cost_mvy.offset(omy_1 as isize) as libc::c_int;
        bcost <<= 4 as libc::c_int;
        if ((costs[0 as libc::c_int as usize] << 4 as libc::c_int) + 1 as libc::c_int) < bcost {
            bcost = (costs[0 as libc::c_int as usize] << 4 as libc::c_int) + 1 as libc::c_int;
        }
        if ((costs[1 as libc::c_int as usize] << 4 as libc::c_int) + 3 as libc::c_int) < bcost {
            bcost = (costs[1 as libc::c_int as usize] << 4 as libc::c_int) + 3 as libc::c_int;
        }
        if ((costs[2 as libc::c_int as usize] << 4 as libc::c_int) + 4 as libc::c_int) < bcost {
            bcost = (costs[2 as libc::c_int as usize] << 4 as libc::c_int) + 4 as libc::c_int;
        }
        if ((costs[3 as libc::c_int as usize] << 4 as libc::c_int) + 12 as libc::c_int) < bcost {
            bcost = (costs[3 as libc::c_int as usize] << 4 as libc::c_int) + 12 as libc::c_int;
        }
        bmx -= ((bcost as uint32_t) << 28 as libc::c_int) as int32_t >> 30 as libc::c_int;
        bmy -= ((bcost as uint32_t) << 30 as libc::c_int) as int32_t >> 30 as libc::c_int;
        bcost >>= 4 as libc::c_int;
    }
    (*m).cost = bcost;
    (*m).mv[0 as libc::c_int as usize] = bmx as int16_t;
    (*m).mv[1 as libc::c_int as usize] = bmy as int16_t;
    (*m).cost_mv = *p_cost_mvx.offset(bmx as isize) as libc::c_int
        + *p_cost_mvy.offset(bmy as isize) as libc::c_int;
}
#[inline(always)]
unsafe extern "C" fn me_refine_bidir(
    mut h: *mut x264_t,
    mut m0: *mut x264_me_t,
    mut m1: *mut x264_me_t,
    mut i_weight: libc::c_int,
    mut i8: libc::c_int,
    mut i_lambda2: libc::c_int,
    mut rd: libc::c_int,
) {
    let mut x: libc::c_int = i8 & 1 as libc::c_int;
    let mut y: libc::c_int = i8 >> 1 as libc::c_int;
    let mut s8: libc::c_int = 4 as libc::c_int
        + 1 as libc::c_int * 8 as libc::c_int
        + 2 as libc::c_int * x
        + 16 as libc::c_int * y;
    let mut cache0_mv: *mut int16_t =
        ((*h).mb.cache.mv[0 as libc::c_int as usize][s8 as usize]).as_mut_ptr();
    let mut cache1_mv: *mut int16_t =
        ((*h).mb.cache.mv[1 as libc::c_int as usize][s8 as usize]).as_mut_ptr();
    let i_pixel: libc::c_int = (*m0).i_pixel;
    let bw: libc::c_int = x264_pixel_size[i_pixel as usize].w as libc::c_int;
    let bh: libc::c_int = x264_pixel_size[i_pixel as usize].h as libc::c_int;
    let mut pixy_buf: [[[pixel; 256]; 9]; 2] = [[[0; 256]; 9]; 2];
    let mut pixu_buf: [[[pixel; 256]; 9]; 2] = [[[0; 256]; 9]; 2];
    let mut pixv_buf: [[[pixel; 256]; 9]; 2] = [[[0; 256]; 9]; 2];
    let mut src: [[[*mut pixel; 9]; 2]; 3] = [[[std::ptr::null_mut::<pixel>(); 9]; 2]; 3];
    let mut chromapix: libc::c_int = (*h).luma2chroma_pixel[i_pixel as usize] as libc::c_int;
    let mut chroma_v_shift: libc::c_int = (*h).mb.chroma_v_shift;
    let mut chroma_x: libc::c_int = (8 as libc::c_int >> (*h).mb.chroma_h_shift) * x;
    let mut chroma_y: libc::c_int = (8 as libc::c_int >> chroma_v_shift) * y;
    let mut pix: *mut pixel = &mut *(*((*h).mb.pic.p_fdec)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize))
    .offset((8 as libc::c_int * x + 8 as libc::c_int * y * 32 as libc::c_int) as isize)
        as *mut pixel;
    let mut pixu: *mut pixel = if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc != 0 {
        &mut *(*((*h).mb.pic.p_fdec)
            .as_mut_ptr()
            .offset(1 as libc::c_int as isize))
        .offset((chroma_x + chroma_y * 32 as libc::c_int) as isize) as *mut pixel
    } else {
        std::ptr::null_mut::<pixel>()
    };
    let mut pixv: *mut pixel = if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc != 0 {
        &mut *(*((*h).mb.pic.p_fdec)
            .as_mut_ptr()
            .offset(2 as libc::c_int as isize))
        .offset((chroma_x + chroma_y * 32 as libc::c_int) as isize) as *mut pixel
    } else {
        std::ptr::null_mut::<pixel>()
    };
    let mut ref0: libc::c_int =
        (*h).mb.cache.ref_0[0 as libc::c_int as usize][s8 as usize] as libc::c_int;
    let mut ref1: libc::c_int =
        (*h).mb.cache.ref_0[1 as libc::c_int as usize][s8 as usize] as libc::c_int;
    let mv0y_offset: libc::c_int = if chroma_v_shift & (*h).mb.b_interlaced & ref0 != 0 {
        ((*h).mb.i_mb_y & 1 as libc::c_int) * 4 as libc::c_int - 2 as libc::c_int
    } else {
        0 as libc::c_int
    };
    let mv1y_offset: libc::c_int = if chroma_v_shift & (*h).mb.b_interlaced & ref1 != 0 {
        ((*h).mb.i_mb_y & 1 as libc::c_int) * 4 as libc::c_int - 2 as libc::c_int
    } else {
        0 as libc::c_int
    };
    let mut stride: [[[intptr_t; 9]; 2]; 3] = [[[0; 9]; 2]; 3];
    let mut bm0x: libc::c_int = (*m0).mv[0 as libc::c_int as usize] as libc::c_int;
    let mut bm0y: libc::c_int = (*m0).mv[1 as libc::c_int as usize] as libc::c_int;
    let mut bm1x: libc::c_int = (*m1).mv[0 as libc::c_int as usize] as libc::c_int;
    let mut bm1y: libc::c_int = (*m1).mv[1 as libc::c_int as usize] as libc::c_int;
    let mut bcost: libc::c_int = (1 as libc::c_int) << 28 as libc::c_int;
    let mut mc_list0: libc::c_int = 1 as libc::c_int;
    let mut mc_list1: libc::c_int = 1 as libc::c_int;
    let mut bcostrd: uint64_t = ((1 as libc::c_ulonglong) << 60 as libc::c_int) as uint64_t;
    let mut amvd: uint16_t = 0;
    let mut visited: [[[uint8_t; 8]; 8]; 8] = [[[0; 8]; 8]; 8];
    static mut dia4d: [[int8_t; 4]; 33] = [
        [
            0 as libc::c_int as int8_t,
            0 as libc::c_int as int8_t,
            0 as libc::c_int as int8_t,
            0 as libc::c_int as int8_t,
        ],
        [
            0 as libc::c_int as int8_t,
            0 as libc::c_int as int8_t,
            0 as libc::c_int as int8_t,
            1 as libc::c_int as int8_t,
        ],
        [
            0 as libc::c_int as int8_t,
            0 as libc::c_int as int8_t,
            0 as libc::c_int as int8_t,
            -(1 as libc::c_int) as int8_t,
        ],
        [
            0 as libc::c_int as int8_t,
            0 as libc::c_int as int8_t,
            1 as libc::c_int as int8_t,
            0 as libc::c_int as int8_t,
        ],
        [
            0 as libc::c_int as int8_t,
            0 as libc::c_int as int8_t,
            -(1 as libc::c_int) as int8_t,
            0 as libc::c_int as int8_t,
        ],
        [
            0 as libc::c_int as int8_t,
            1 as libc::c_int as int8_t,
            0 as libc::c_int as int8_t,
            0 as libc::c_int as int8_t,
        ],
        [
            0 as libc::c_int as int8_t,
            -(1 as libc::c_int) as int8_t,
            0 as libc::c_int as int8_t,
            0 as libc::c_int as int8_t,
        ],
        [
            1 as libc::c_int as int8_t,
            0 as libc::c_int as int8_t,
            0 as libc::c_int as int8_t,
            0 as libc::c_int as int8_t,
        ],
        [
            -(1 as libc::c_int) as int8_t,
            0 as libc::c_int as int8_t,
            0 as libc::c_int as int8_t,
            0 as libc::c_int as int8_t,
        ],
        [
            0 as libc::c_int as int8_t,
            0 as libc::c_int as int8_t,
            1 as libc::c_int as int8_t,
            1 as libc::c_int as int8_t,
        ],
        [
            0 as libc::c_int as int8_t,
            0 as libc::c_int as int8_t,
            -(1 as libc::c_int) as int8_t,
            -(1 as libc::c_int) as int8_t,
        ],
        [
            0 as libc::c_int as int8_t,
            1 as libc::c_int as int8_t,
            1 as libc::c_int as int8_t,
            0 as libc::c_int as int8_t,
        ],
        [
            0 as libc::c_int as int8_t,
            -(1 as libc::c_int) as int8_t,
            -(1 as libc::c_int) as int8_t,
            0 as libc::c_int as int8_t,
        ],
        [
            1 as libc::c_int as int8_t,
            1 as libc::c_int as int8_t,
            0 as libc::c_int as int8_t,
            0 as libc::c_int as int8_t,
        ],
        [
            -(1 as libc::c_int) as int8_t,
            -(1 as libc::c_int) as int8_t,
            0 as libc::c_int as int8_t,
            0 as libc::c_int as int8_t,
        ],
        [
            1 as libc::c_int as int8_t,
            0 as libc::c_int as int8_t,
            0 as libc::c_int as int8_t,
            1 as libc::c_int as int8_t,
        ],
        [
            -(1 as libc::c_int) as int8_t,
            0 as libc::c_int as int8_t,
            0 as libc::c_int as int8_t,
            -(1 as libc::c_int) as int8_t,
        ],
        [
            0 as libc::c_int as int8_t,
            1 as libc::c_int as int8_t,
            0 as libc::c_int as int8_t,
            1 as libc::c_int as int8_t,
        ],
        [
            0 as libc::c_int as int8_t,
            -(1 as libc::c_int) as int8_t,
            0 as libc::c_int as int8_t,
            -(1 as libc::c_int) as int8_t,
        ],
        [
            1 as libc::c_int as int8_t,
            0 as libc::c_int as int8_t,
            1 as libc::c_int as int8_t,
            0 as libc::c_int as int8_t,
        ],
        [
            -(1 as libc::c_int) as int8_t,
            0 as libc::c_int as int8_t,
            -(1 as libc::c_int) as int8_t,
            0 as libc::c_int as int8_t,
        ],
        [
            0 as libc::c_int as int8_t,
            0 as libc::c_int as int8_t,
            -(1 as libc::c_int) as int8_t,
            1 as libc::c_int as int8_t,
        ],
        [
            0 as libc::c_int as int8_t,
            0 as libc::c_int as int8_t,
            1 as libc::c_int as int8_t,
            -(1 as libc::c_int) as int8_t,
        ],
        [
            0 as libc::c_int as int8_t,
            -(1 as libc::c_int) as int8_t,
            1 as libc::c_int as int8_t,
            0 as libc::c_int as int8_t,
        ],
        [
            0 as libc::c_int as int8_t,
            1 as libc::c_int as int8_t,
            -(1 as libc::c_int) as int8_t,
            0 as libc::c_int as int8_t,
        ],
        [
            -(1 as libc::c_int) as int8_t,
            1 as libc::c_int as int8_t,
            0 as libc::c_int as int8_t,
            0 as libc::c_int as int8_t,
        ],
        [
            1 as libc::c_int as int8_t,
            -(1 as libc::c_int) as int8_t,
            0 as libc::c_int as int8_t,
            0 as libc::c_int as int8_t,
        ],
        [
            1 as libc::c_int as int8_t,
            0 as libc::c_int as int8_t,
            0 as libc::c_int as int8_t,
            -(1 as libc::c_int) as int8_t,
        ],
        [
            -(1 as libc::c_int) as int8_t,
            0 as libc::c_int as int8_t,
            0 as libc::c_int as int8_t,
            1 as libc::c_int as int8_t,
        ],
        [
            0 as libc::c_int as int8_t,
            -(1 as libc::c_int) as int8_t,
            0 as libc::c_int as int8_t,
            1 as libc::c_int as int8_t,
        ],
        [
            0 as libc::c_int as int8_t,
            1 as libc::c_int as int8_t,
            0 as libc::c_int as int8_t,
            -(1 as libc::c_int) as int8_t,
        ],
        [
            -(1 as libc::c_int) as int8_t,
            0 as libc::c_int as int8_t,
            1 as libc::c_int as int8_t,
            0 as libc::c_int as int8_t,
        ],
        [
            1 as libc::c_int as int8_t,
            0 as libc::c_int as int8_t,
            -(1 as libc::c_int) as int8_t,
            0 as libc::c_int as int8_t,
        ],
    ];
    if bm0y < (*h).mb.mv_min_spel[1 as libc::c_int as usize] + 8 as libc::c_int
        || bm1y < (*h).mb.mv_min_spel[1 as libc::c_int as usize] + 8 as libc::c_int
        || bm0y > (*h).mb.mv_max_spel[1 as libc::c_int as usize] - 8 as libc::c_int
        || bm1y > (*h).mb.mv_max_spel[1 as libc::c_int as usize] - 8 as libc::c_int
        || bm0x < (*h).mb.mv_min_spel[0 as libc::c_int as usize] + 8 as libc::c_int
        || bm1x < (*h).mb.mv_min_spel[0 as libc::c_int as usize] + 8 as libc::c_int
        || bm0x > (*h).mb.mv_max_spel[0 as libc::c_int as usize] - 8 as libc::c_int
        || bm1x > (*h).mb.mv_max_spel[0 as libc::c_int as usize] - 8 as libc::c_int
    {
        return;
    }
    if rd != 0 && (*m0).i_pixel != PIXEL_16x16 as libc::c_int && i8 != 0 as libc::c_int {
        x264_8_mb_predict_mv(
            h,
            0 as libc::c_int,
            i8 << 2 as libc::c_int,
            bw >> 2 as libc::c_int,
            ((*m0).mvp).as_mut_ptr(),
        );
        x264_8_mb_predict_mv(
            h,
            1 as libc::c_int,
            i8 << 2 as libc::c_int,
            bw >> 2 as libc::c_int,
            ((*m1).mvp).as_mut_ptr(),
        );
    }
    let mut p_cost_m0x: *const uint16_t =
        ((*m0).p_cost_mv).offset(-((*m0).mvp[0 as libc::c_int as usize] as libc::c_int as isize));
    let mut p_cost_m0y: *const uint16_t =
        ((*m0).p_cost_mv).offset(-((*m0).mvp[1 as libc::c_int as usize] as libc::c_int as isize));
    let mut p_cost_m1x: *const uint16_t =
        ((*m1).p_cost_mv).offset(-((*m1).mvp[0 as libc::c_int as usize] as libc::c_int as isize));
    let mut p_cost_m1y: *const uint16_t =
        ((*m1).p_cost_mv).offset(-((*m1).mvp[1 as libc::c_int as usize] as libc::c_int as isize));
    ((*h).mc.memzero_aligned).expect("non-null function pointer")(
        visited.as_mut_ptr() as *mut libc::c_void,
        ::core::mem::size_of::<[[[uint8_t; 8]; 8]; 8]>() as libc::c_ulong,
    );
    let mut pass: libc::c_int = 0 as libc::c_int;
    while pass < 8 as libc::c_int {
        let mut bestj: libc::c_int = 0 as libc::c_int;
        if mc_list0 != 0 {
            let mut j: libc::c_int = x264_8_iter_kludge;
            while j < 9 as libc::c_int {
                let mut m: *mut x264_me_t = m0;
                let mut i: libc::c_int = 4 as libc::c_int
                    + 3 as libc::c_int
                        * square1[j as usize][0 as libc::c_int as usize] as libc::c_int
                    + square1[j as usize][1 as libc::c_int as usize] as libc::c_int;
                let mut mvx: libc::c_int =
                    bm0x + square1[j as usize][0 as libc::c_int as usize] as libc::c_int;
                let mut mvy: libc::c_int =
                    bm0y + square1[j as usize][1 as libc::c_int as usize] as libc::c_int;
                stride[0 as libc::c_int as usize][0 as libc::c_int as usize][i as usize] =
                    bw as intptr_t;
                src[0 as libc::c_int as usize][0 as libc::c_int as usize][i as usize] =
                    ((*h).mc.get_ref).expect("non-null function pointer")(
                        (pixy_buf[0 as libc::c_int as usize][i as usize]).as_mut_ptr(),
                        &mut *(*(*stride.as_mut_ptr().offset(0 as libc::c_int as isize))
                            .as_mut_ptr()
                            .offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(i as isize),
                        &mut *((*m).p_fref).as_mut_ptr().offset(0 as libc::c_int as isize),
                        (*m).i_stride[0 as libc::c_int as usize] as intptr_t,
                        mvx,
                        mvy,
                        bw,
                        bh,
                        x264_zero.as_mut_ptr() as *const x264_weight_t,
                    );
                if rd != 0 {
                    if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as libc::c_int {
                        stride[1 as libc::c_int as usize][0 as libc::c_int as usize][i as usize] =
                            bw as intptr_t;
                        src[1 as libc::c_int as usize][0 as libc::c_int as usize][i as usize] =
                            ((*h).mc.get_ref).expect("non-null function pointer")(
                                (pixu_buf[0 as libc::c_int as usize][i as usize]).as_mut_ptr(),
                                &mut *(*(*stride.as_mut_ptr().offset(1 as libc::c_int as isize))
                                    .as_mut_ptr()
                                    .offset(0 as libc::c_int as isize))
                                .as_mut_ptr()
                                .offset(i as isize),
                                &mut *((*m).p_fref).as_mut_ptr().offset(4 as libc::c_int as isize),
                                (*m).i_stride[1 as libc::c_int as usize] as intptr_t,
                                mvx,
                                mvy,
                                bw,
                                bh,
                                x264_zero.as_mut_ptr() as *const x264_weight_t,
                            );
                        stride[2 as libc::c_int as usize][0 as libc::c_int as usize][i as usize] =
                            bw as intptr_t;
                        src[2 as libc::c_int as usize][0 as libc::c_int as usize][i as usize] =
                            ((*h).mc.get_ref).expect("non-null function pointer")(
                                (pixv_buf[0 as libc::c_int as usize][i as usize]).as_mut_ptr(),
                                &mut *(*(*stride.as_mut_ptr().offset(2 as libc::c_int as isize))
                                    .as_mut_ptr()
                                    .offset(0 as libc::c_int as isize))
                                .as_mut_ptr()
                                .offset(i as isize),
                                &mut *((*m).p_fref).as_mut_ptr().offset(8 as libc::c_int as isize),
                                (*m).i_stride[2 as libc::c_int as usize] as intptr_t,
                                mvx,
                                mvy,
                                bw,
                                bh,
                                x264_zero.as_mut_ptr() as *const x264_weight_t,
                            );
                    } else if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc != 0 {
                        ((*h).mc.mc_chroma).expect("non-null function pointer")(
                            (pixu_buf[0 as libc::c_int as usize][i as usize]).as_mut_ptr(),
                            (pixv_buf[0 as libc::c_int as usize][i as usize]).as_mut_ptr(),
                            8 as libc::c_int as intptr_t,
                            (*m).p_fref[4 as libc::c_int as usize],
                            (*m).i_stride[1 as libc::c_int as usize] as intptr_t,
                            mvx,
                            (2 as libc::c_int * (mvy + mv0y_offset)) >> chroma_v_shift,
                            bw >> 1 as libc::c_int,
                            bh >> chroma_v_shift,
                        );
                    }
                }
                j += 1;
                j;
            }
        }
        if mc_list1 != 0 {
            let mut j_0: libc::c_int = x264_8_iter_kludge;
            while j_0 < 9 as libc::c_int {
                let mut m_0: *mut x264_me_t = m1;
                let mut i_0: libc::c_int = 4 as libc::c_int
                    + 3 as libc::c_int
                        * square1[j_0 as usize][0 as libc::c_int as usize] as libc::c_int
                    + square1[j_0 as usize][1 as libc::c_int as usize] as libc::c_int;
                let mut mvx_0: libc::c_int =
                    bm1x + square1[j_0 as usize][0 as libc::c_int as usize] as libc::c_int;
                let mut mvy_0: libc::c_int =
                    bm1y + square1[j_0 as usize][1 as libc::c_int as usize] as libc::c_int;
                stride[0 as libc::c_int as usize][1 as libc::c_int as usize][i_0 as usize] =
                    bw as intptr_t;
                src[0 as libc::c_int as usize][1 as libc::c_int as usize][i_0 as usize] =
                    ((*h).mc.get_ref).expect("non-null function pointer")(
                        (pixy_buf[1 as libc::c_int as usize][i_0 as usize]).as_mut_ptr(),
                        &mut *(*(*stride.as_mut_ptr().offset(0 as libc::c_int as isize))
                            .as_mut_ptr()
                            .offset(1 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(i_0 as isize),
                        &mut *((*m_0).p_fref)
                            .as_mut_ptr()
                            .offset(0 as libc::c_int as isize),
                        (*m_0).i_stride[0 as libc::c_int as usize] as intptr_t,
                        mvx_0,
                        mvy_0,
                        bw,
                        bh,
                        x264_zero.as_mut_ptr() as *const x264_weight_t,
                    );
                if rd != 0 {
                    if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as libc::c_int {
                        stride[1 as libc::c_int as usize][1 as libc::c_int as usize]
                            [i_0 as usize] = bw as intptr_t;
                        src[1 as libc::c_int as usize][1 as libc::c_int as usize][i_0 as usize] =
                            ((*h).mc.get_ref).expect("non-null function pointer")(
                                (pixu_buf[1 as libc::c_int as usize][i_0 as usize]).as_mut_ptr(),
                                &mut *(*(*stride.as_mut_ptr().offset(1 as libc::c_int as isize))
                                    .as_mut_ptr()
                                    .offset(1 as libc::c_int as isize))
                                .as_mut_ptr()
                                .offset(i_0 as isize),
                                &mut *((*m_0).p_fref)
                                    .as_mut_ptr()
                                    .offset(4 as libc::c_int as isize),
                                (*m_0).i_stride[1 as libc::c_int as usize] as intptr_t,
                                mvx_0,
                                mvy_0,
                                bw,
                                bh,
                                x264_zero.as_mut_ptr() as *const x264_weight_t,
                            );
                        stride[2 as libc::c_int as usize][1 as libc::c_int as usize]
                            [i_0 as usize] = bw as intptr_t;
                        src[2 as libc::c_int as usize][1 as libc::c_int as usize][i_0 as usize] =
                            ((*h).mc.get_ref).expect("non-null function pointer")(
                                (pixv_buf[1 as libc::c_int as usize][i_0 as usize]).as_mut_ptr(),
                                &mut *(*(*stride.as_mut_ptr().offset(2 as libc::c_int as isize))
                                    .as_mut_ptr()
                                    .offset(1 as libc::c_int as isize))
                                .as_mut_ptr()
                                .offset(i_0 as isize),
                                &mut *((*m_0).p_fref)
                                    .as_mut_ptr()
                                    .offset(8 as libc::c_int as isize),
                                (*m_0).i_stride[2 as libc::c_int as usize] as intptr_t,
                                mvx_0,
                                mvy_0,
                                bw,
                                bh,
                                x264_zero.as_mut_ptr() as *const x264_weight_t,
                            );
                    } else if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc != 0 {
                        ((*h).mc.mc_chroma).expect("non-null function pointer")(
                            (pixu_buf[1 as libc::c_int as usize][i_0 as usize]).as_mut_ptr(),
                            (pixv_buf[1 as libc::c_int as usize][i_0 as usize]).as_mut_ptr(),
                            8 as libc::c_int as intptr_t,
                            (*m_0).p_fref[4 as libc::c_int as usize],
                            (*m_0).i_stride[1 as libc::c_int as usize] as intptr_t,
                            mvx_0,
                            (2 as libc::c_int * (mvy_0 + mv1y_offset)) >> chroma_v_shift,
                            bw >> 1 as libc::c_int,
                            bh >> chroma_v_shift,
                        );
                    }
                }
                j_0 += 1;
                j_0;
            }
        }
        let mut j_1: libc::c_int = (pass != 0) as libc::c_int;
        while j_1 < 33 as libc::c_int {
            let mut m0x: libc::c_int =
                dia4d[j_1 as usize][0 as libc::c_int as usize] as libc::c_int + bm0x;
            let mut m0y: libc::c_int =
                dia4d[j_1 as usize][1 as libc::c_int as usize] as libc::c_int + bm0y;
            let mut m1x: libc::c_int =
                dia4d[j_1 as usize][2 as libc::c_int as usize] as libc::c_int + bm1x;
            let mut m1y: libc::c_int =
                dia4d[j_1 as usize][3 as libc::c_int as usize] as libc::c_int + bm1y;
            if pass == 0
                || visited[(m0x & 7 as libc::c_int) as usize][(m0y & 7 as libc::c_int) as usize]
                    [(m1x & 7 as libc::c_int) as usize] as libc::c_int
                    & ((1 as libc::c_int) << (m1y & 7 as libc::c_int))
                    == 0
            {
                let mut i0: libc::c_int = 4 as libc::c_int
                    + 3 as libc::c_int
                        * dia4d[j_1 as usize][0 as libc::c_int as usize] as libc::c_int
                    + dia4d[j_1 as usize][1 as libc::c_int as usize] as libc::c_int;
                let mut i1: libc::c_int = 4 as libc::c_int
                    + 3 as libc::c_int
                        * dia4d[j_1 as usize][2 as libc::c_int as usize] as libc::c_int
                    + dia4d[j_1 as usize][3 as libc::c_int as usize] as libc::c_int;
                visited[(m0x & 7 as libc::c_int) as usize][(m0y & 7 as libc::c_int) as usize]
                    [(m1x & 7 as libc::c_int) as usize] =
                    (visited[(m0x & 7 as libc::c_int) as usize][(m0y & 7 as libc::c_int) as usize]
                        [(m1x & 7 as libc::c_int) as usize] as libc::c_int
                        | ((1 as libc::c_int) << (m1y & 7 as libc::c_int)))
                        as uint8_t;
                ((*h).mc.avg[i_pixel as usize]).expect("non-null function pointer")(
                    pix,
                    32 as libc::c_int as intptr_t,
                    src[0 as libc::c_int as usize][0 as libc::c_int as usize][i0 as usize],
                    stride[0 as libc::c_int as usize][0 as libc::c_int as usize][i0 as usize],
                    src[0 as libc::c_int as usize][1 as libc::c_int as usize][i1 as usize],
                    stride[0 as libc::c_int as usize][1 as libc::c_int as usize][i1 as usize],
                    i_weight,
                );
                let mut cost: libc::c_int = ((*h).pixf.mbcmp[i_pixel as usize])
                    .expect("non-null function pointer")(
                    (*m0).p_fenc[0 as libc::c_int as usize],
                    16 as libc::c_int as intptr_t,
                    pix,
                    32 as libc::c_int as intptr_t,
                ) + *p_cost_m0x.offset(m0x as isize) as libc::c_int
                    + *p_cost_m0y.offset(m0y as isize) as libc::c_int
                    + *p_cost_m1x.offset(m1x as isize) as libc::c_int
                    + *p_cost_m1y.offset(m1y as isize) as libc::c_int;
                if rd != 0 {
                    if cost < bcost + (bcost >> 4 as libc::c_int) {
                        bcost = if cost < bcost { cost } else { bcost };
                        (*(cache0_mv as *mut x264_union32_t)).i = pack16to32_mask(m0x, m0y);
                        (*(cache1_mv as *mut x264_union32_t)).i = pack16to32_mask(m1x, m1y);
                        if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc
                            == CHROMA_444 as libc::c_int
                        {
                            ((*h).mc.avg[i_pixel as usize]).expect("non-null function pointer")(
                                pixu,
                                32 as libc::c_int as intptr_t,
                                src[1 as libc::c_int as usize][0 as libc::c_int as usize]
                                    [i0 as usize],
                                stride[1 as libc::c_int as usize][0 as libc::c_int as usize]
                                    [i0 as usize],
                                src[1 as libc::c_int as usize][1 as libc::c_int as usize]
                                    [i1 as usize],
                                stride[1 as libc::c_int as usize][1 as libc::c_int as usize]
                                    [i1 as usize],
                                i_weight,
                            );
                            ((*h).mc.avg[i_pixel as usize]).expect("non-null function pointer")(
                                pixv,
                                32 as libc::c_int as intptr_t,
                                src[2 as libc::c_int as usize][0 as libc::c_int as usize]
                                    [i0 as usize],
                                stride[2 as libc::c_int as usize][0 as libc::c_int as usize]
                                    [i0 as usize],
                                src[2 as libc::c_int as usize][1 as libc::c_int as usize]
                                    [i1 as usize],
                                stride[2 as libc::c_int as usize][1 as libc::c_int as usize]
                                    [i1 as usize],
                                i_weight,
                            );
                        } else if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc != 0 {
                            ((*h).mc.avg[chromapix as usize]).expect("non-null function pointer")(
                                pixu,
                                32 as libc::c_int as intptr_t,
                                (pixu_buf[0 as libc::c_int as usize][i0 as usize]).as_mut_ptr(),
                                8 as libc::c_int as intptr_t,
                                (pixu_buf[1 as libc::c_int as usize][i1 as usize]).as_mut_ptr(),
                                8 as libc::c_int as intptr_t,
                                i_weight,
                            );
                            ((*h).mc.avg[chromapix as usize]).expect("non-null function pointer")(
                                pixv,
                                32 as libc::c_int as intptr_t,
                                (pixv_buf[0 as libc::c_int as usize][i0 as usize]).as_mut_ptr(),
                                8 as libc::c_int as intptr_t,
                                (pixv_buf[1 as libc::c_int as usize][i1 as usize]).as_mut_ptr(),
                                8 as libc::c_int as intptr_t,
                                i_weight,
                            );
                        }
                        let mut costrd: uint64_t =
                            x264_8_rd_cost_part(h, i_lambda2, i8 * 4 as libc::c_int, (*m0).i_pixel);
                        if costrd < bcostrd {
                            bcostrd = costrd;
                            bestj = j_1;
                        }
                    }
                } else if cost < bcost {
                    bcost = cost;
                    bestj = j_1;
                }
            }
            j_1 += 1;
            j_1;
        }
        if bestj == 0 {
            break;
        }
        bm0x += dia4d[bestj as usize][0 as libc::c_int as usize] as libc::c_int;
        bm0y += dia4d[bestj as usize][1 as libc::c_int as usize] as libc::c_int;
        bm1x += dia4d[bestj as usize][2 as libc::c_int as usize] as libc::c_int;
        bm1y += dia4d[bestj as usize][3 as libc::c_int as usize] as libc::c_int;
        mc_list0 = (*(&*(*dia4d.as_ptr().offset(bestj as isize))
            .as_ptr()
            .offset(0 as libc::c_int as isize) as *const int8_t
            as *mut x264_union16_t))
            .i as libc::c_int;
        mc_list1 = (*(&*(*dia4d.as_ptr().offset(bestj as isize))
            .as_ptr()
            .offset(2 as libc::c_int as isize) as *const int8_t
            as *mut x264_union16_t))
            .i as libc::c_int;
        pass += 1;
        pass;
    }
    if rd != 0 {
        x264_macroblock_cache_mv(
            h,
            2 as libc::c_int * x,
            2 as libc::c_int * y,
            bw >> 2 as libc::c_int,
            bh >> 2 as libc::c_int,
            0 as libc::c_int,
            pack16to32_mask(bm0x, bm0y),
        );
        amvd = pack8to16(
            (if abs(bm0x - (*m0).mvp[0 as libc::c_int as usize] as libc::c_int) < 33 as libc::c_int
            {
                abs(bm0x - (*m0).mvp[0 as libc::c_int as usize] as libc::c_int)
            } else {
                33 as libc::c_int
            }) as uint32_t,
            (if abs(bm0y - (*m0).mvp[1 as libc::c_int as usize] as libc::c_int) < 33 as libc::c_int
            {
                abs(bm0y - (*m0).mvp[1 as libc::c_int as usize] as libc::c_int)
            } else {
                33 as libc::c_int
            }) as uint32_t,
        ) as uint16_t;
        x264_macroblock_cache_mvd(
            h,
            2 as libc::c_int * x,
            2 as libc::c_int * y,
            bw >> 2 as libc::c_int,
            bh >> 2 as libc::c_int,
            0 as libc::c_int,
            amvd,
        );
        x264_macroblock_cache_mv(
            h,
            2 as libc::c_int * x,
            2 as libc::c_int * y,
            bw >> 2 as libc::c_int,
            bh >> 2 as libc::c_int,
            1 as libc::c_int,
            pack16to32_mask(bm1x, bm1y),
        );
        amvd = pack8to16(
            (if abs(bm1x - (*m1).mvp[0 as libc::c_int as usize] as libc::c_int) < 33 as libc::c_int
            {
                abs(bm1x - (*m1).mvp[0 as libc::c_int as usize] as libc::c_int)
            } else {
                33 as libc::c_int
            }) as uint32_t,
            (if abs(bm1y - (*m1).mvp[1 as libc::c_int as usize] as libc::c_int) < 33 as libc::c_int
            {
                abs(bm1y - (*m1).mvp[1 as libc::c_int as usize] as libc::c_int)
            } else {
                33 as libc::c_int
            }) as uint32_t,
        ) as uint16_t;
        x264_macroblock_cache_mvd(
            h,
            2 as libc::c_int * x,
            2 as libc::c_int * y,
            bw >> 2 as libc::c_int,
            bh >> 2 as libc::c_int,
            1 as libc::c_int,
            amvd,
        );
    }
    (*m0).mv[0 as libc::c_int as usize] = bm0x as int16_t;
    (*m0).mv[1 as libc::c_int as usize] = bm0y as int16_t;
    (*m1).mv[0 as libc::c_int as usize] = bm1x as int16_t;
    (*m1).mv[1 as libc::c_int as usize] = bm1y as int16_t;
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_me_refine_bidir_satd(
    mut h: *mut x264_t,
    mut m0: *mut x264_me_t,
    mut m1: *mut x264_me_t,
    mut i_weight: libc::c_int,
) {
    me_refine_bidir(
        h,
        m0,
        m1,
        i_weight,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_me_refine_bidir_rd(
    mut h: *mut x264_t,
    mut m0: *mut x264_me_t,
    mut m1: *mut x264_me_t,
    mut i_weight: libc::c_int,
    mut i8: libc::c_int,
    mut i_lambda2: libc::c_int,
) {
    (*h).mb.b_skip_mc = 1 as libc::c_int;
    me_refine_bidir(h, m0, m1, i_weight, i8, i_lambda2, 1 as libc::c_int);
    (*h).mb.b_skip_mc = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_me_refine_qpel_rd(
    mut h: *mut x264_t,
    mut m: *mut x264_me_t,
    mut i_lambda2: libc::c_int,
    mut i4: libc::c_int,
    mut i_list: libc::c_int,
) {
    let mut cache_mv: *mut int16_t =
        ((*h).mb.cache.mv[i_list as usize][x264_scan8[i4 as usize] as usize]).as_mut_ptr();
    let mut p_cost_mvx: *const uint16_t = std::ptr::null::<uint16_t>();
    let mut p_cost_mvy: *const uint16_t = std::ptr::null::<uint16_t>();
    let bw: libc::c_int = x264_pixel_size[(*m).i_pixel as usize].w as libc::c_int;
    let bh: libc::c_int = x264_pixel_size[(*m).i_pixel as usize].h as libc::c_int;
    let i_pixel: libc::c_int = (*m).i_pixel;
    let mut chroma_v_shift: libc::c_int = (*h).mb.chroma_v_shift;
    let mut mvy_offset: libc::c_int = if chroma_v_shift & (*h).mb.b_interlaced & (*m).i_ref != 0 {
        ((*h).mb.i_mb_y & 1 as libc::c_int) * 4 as libc::c_int - 2 as libc::c_int
    } else {
        0 as libc::c_int
    };
    let mut bcost: uint64_t = ((1 as libc::c_ulonglong) << 60 as libc::c_int) as uint64_t;
    let mut bmx: libc::c_int = (*m).mv[0 as libc::c_int as usize] as libc::c_int;
    let mut bmy: libc::c_int = (*m).mv[1 as libc::c_int as usize] as libc::c_int;
    let mut omx: libc::c_int = 0;
    let mut omy: libc::c_int = 0;
    let mut pmx: libc::c_int = 0;
    let mut pmy: libc::c_int = 0;
    let mut satd: libc::c_int = 0;
    let mut bsatd: libc::c_int = 0;
    let mut dir: libc::c_int = -(2 as libc::c_int);
    let mut i8: libc::c_int = i4 >> 2 as libc::c_int;
    let mut amvd: uint16_t = 0;
    let mut pix: *mut pixel = &mut *(*((*h).mb.pic.p_fdec)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize))
    .offset(*block_idx_xy_fdec.as_ptr().offset(i4 as isize) as isize)
        as *mut pixel;
    let mut pixu: *mut pixel = std::ptr::null_mut::<pixel>();
    let mut pixv: *mut pixel = std::ptr::null_mut::<pixel>();
    if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as libc::c_int {
        pixu = &mut *(*((*h).mb.pic.p_fdec)
            .as_mut_ptr()
            .offset(1 as libc::c_int as isize))
        .offset(*block_idx_xy_fdec.as_ptr().offset(i4 as isize) as isize)
            as *mut pixel;
        pixv = &mut *(*((*h).mb.pic.p_fdec)
            .as_mut_ptr()
            .offset(2 as libc::c_int as isize))
        .offset(*block_idx_xy_fdec.as_ptr().offset(i4 as isize) as isize)
            as *mut pixel;
    } else if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc != 0 {
        pixu = &mut *(*((*h).mb.pic.p_fdec)
            .as_mut_ptr()
            .offset(1 as libc::c_int as isize))
        .offset(
            ((i8 >> 1 as libc::c_int) * ((8 as libc::c_int * 32 as libc::c_int) >> chroma_v_shift)
                + (i8 & 1 as libc::c_int) * 4 as libc::c_int) as isize,
        ) as *mut pixel;
        pixv = &mut *(*((*h).mb.pic.p_fdec)
            .as_mut_ptr()
            .offset(2 as libc::c_int as isize))
        .offset(
            ((i8 >> 1 as libc::c_int) * ((8 as libc::c_int * 32 as libc::c_int) >> chroma_v_shift)
                + (i8 & 1 as libc::c_int) * 4 as libc::c_int) as isize,
        ) as *mut pixel;
    } else {
        pixu = std::ptr::null_mut::<pixel>();
        pixv = std::ptr::null_mut::<pixel>();
    }
    (*h).mb.b_skip_mc = 1 as libc::c_int;
    if (*m).i_pixel != PIXEL_16x16 as libc::c_int && i4 != 0 as libc::c_int {
        x264_8_mb_predict_mv(
            h,
            i_list,
            i4,
            bw >> 2 as libc::c_int,
            ((*m).mvp).as_mut_ptr(),
        );
    }
    pmx = (*m).mvp[0 as libc::c_int as usize] as libc::c_int;
    pmy = (*m).mvp[1 as libc::c_int as usize] as libc::c_int;
    p_cost_mvx = ((*m).p_cost_mv).offset(-(pmx as isize));
    p_cost_mvy = ((*m).p_cost_mv).offset(-(pmy as isize));
    if 0 as libc::c_int == 0 || !(bmx == pmx && bmy == pmy) {
        ((*h).mc.mc_luma).expect("non-null function pointer")(
            pix,
            32 as libc::c_int as intptr_t,
            ((*m).p_fref).as_mut_ptr(),
            (*m).i_stride[0 as libc::c_int as usize] as intptr_t,
            bmx,
            bmy,
            bw,
            bh,
            &*((*m).weight).offset(0 as libc::c_int as isize),
        );
        bsatd = ((*h).pixf.mbcmp[i_pixel as usize]).expect("non-null function pointer")(
            (*m).p_fenc[0 as libc::c_int as usize],
            16 as libc::c_int as intptr_t,
            pix,
            32 as libc::c_int as intptr_t,
        ) + *p_cost_mvx.offset(bmx as isize) as libc::c_int
            + *p_cost_mvy.offset(bmy as isize) as libc::c_int;
        if bsatd < bsatd {
            bsatd = bsatd;
        }
    } else {
        bsatd = (1 as libc::c_int) << 28 as libc::c_int;
    }
    if (*m).i_pixel != PIXEL_16x16 as libc::c_int {
        if 0 as libc::c_int <= bsatd + (bsatd >> 4 as libc::c_int) {
            let mut cost: uint64_t = 0;
            (*(cache_mv as *mut x264_union32_t)).i = pack16to32_mask(bmx, bmy);
            if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as libc::c_int {
                ((*h).mc.mc_luma).expect("non-null function pointer")(
                    pixu,
                    32 as libc::c_int as intptr_t,
                    &mut *((*m).p_fref).as_mut_ptr().offset(4 as libc::c_int as isize),
                    (*m).i_stride[1 as libc::c_int as usize] as intptr_t,
                    bmx,
                    bmy,
                    bw,
                    bh,
                    &*((*m).weight).offset(1 as libc::c_int as isize),
                );
                ((*h).mc.mc_luma).expect("non-null function pointer")(
                    pixv,
                    32 as libc::c_int as intptr_t,
                    &mut *((*m).p_fref).as_mut_ptr().offset(8 as libc::c_int as isize),
                    (*m).i_stride[2 as libc::c_int as usize] as intptr_t,
                    bmx,
                    bmy,
                    bw,
                    bh,
                    &*((*m).weight).offset(2 as libc::c_int as isize),
                );
            } else if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc != 0
                && (*m).i_pixel <= PIXEL_8x8 as libc::c_int
            {
                ((*h).mc.mc_chroma).expect("non-null function pointer")(
                    pixu,
                    pixv,
                    32 as libc::c_int as intptr_t,
                    (*m).p_fref[4 as libc::c_int as usize],
                    (*m).i_stride[1 as libc::c_int as usize] as intptr_t,
                    bmx,
                    (2 as libc::c_int * (bmy + mvy_offset)) >> chroma_v_shift,
                    bw >> 1 as libc::c_int,
                    bh >> chroma_v_shift,
                );
                if !((*((*m).weight).offset(1 as libc::c_int as isize)).weightfn).is_null() {
                    (*((*((*m).weight).offset(1 as libc::c_int as isize)).weightfn)
                        .offset((bw >> 3 as libc::c_int) as isize))
                    .expect("non-null function pointer")(
                        pixu,
                        32 as libc::c_int as intptr_t,
                        pixu,
                        32 as libc::c_int as intptr_t,
                        &*((*m).weight).offset(1 as libc::c_int as isize),
                        bh >> chroma_v_shift,
                    );
                }
                if !((*((*m).weight).offset(2 as libc::c_int as isize)).weightfn).is_null() {
                    (*((*((*m).weight).offset(2 as libc::c_int as isize)).weightfn)
                        .offset((bw >> 3 as libc::c_int) as isize))
                    .expect("non-null function pointer")(
                        pixv,
                        32 as libc::c_int as intptr_t,
                        pixv,
                        32 as libc::c_int as intptr_t,
                        &*((*m).weight).offset(2 as libc::c_int as isize),
                        bh >> chroma_v_shift,
                    );
                }
            }
            cost = x264_8_rd_cost_part(h, i_lambda2, i4, (*m).i_pixel);
            if cost < bcost {
                bcost = cost;
                bmx = bmx;
                bmy = bmy;
                dir = if 0 as libc::c_int != 0 {
                    0 as libc::c_int
                } else {
                    dir
                };
            }
        }
    } else {
        bcost = (*m).cost as uint64_t;
    }
    if (bmx != pmx || bmy != pmy)
        && pmx >= (*h).mb.mv_min_spel[0 as libc::c_int as usize]
        && pmx <= (*h).mb.mv_max_spel[0 as libc::c_int as usize]
        && pmy >= (*h).mb.mv_min_spel[1 as libc::c_int as usize]
        && pmy <= (*h).mb.mv_max_spel[1 as libc::c_int as usize]
    {
        if 0 as libc::c_int == 0 || !(pmx == pmx && pmy == pmy) {
            ((*h).mc.mc_luma).expect("non-null function pointer")(
                pix,
                32 as libc::c_int as intptr_t,
                ((*m).p_fref).as_mut_ptr(),
                (*m).i_stride[0 as libc::c_int as usize] as intptr_t,
                pmx,
                pmy,
                bw,
                bh,
                &*((*m).weight).offset(0 as libc::c_int as isize),
            );
            satd = ((*h).pixf.mbcmp[i_pixel as usize]).expect("non-null function pointer")(
                (*m).p_fenc[0 as libc::c_int as usize],
                16 as libc::c_int as intptr_t,
                pix,
                32 as libc::c_int as intptr_t,
            ) + *p_cost_mvx.offset(pmx as isize) as libc::c_int
                + *p_cost_mvy.offset(pmy as isize) as libc::c_int;
            if satd < bsatd {
                bsatd = satd;
            }
        } else {
            satd = (1 as libc::c_int) << 28 as libc::c_int;
        }
        if satd <= bsatd + (bsatd >> 4 as libc::c_int) {
            let mut cost_0: uint64_t = 0;
            (*(cache_mv as *mut x264_union32_t)).i = pack16to32_mask(pmx, pmy);
            if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as libc::c_int {
                ((*h).mc.mc_luma).expect("non-null function pointer")(
                    pixu,
                    32 as libc::c_int as intptr_t,
                    &mut *((*m).p_fref).as_mut_ptr().offset(4 as libc::c_int as isize),
                    (*m).i_stride[1 as libc::c_int as usize] as intptr_t,
                    pmx,
                    pmy,
                    bw,
                    bh,
                    &*((*m).weight).offset(1 as libc::c_int as isize),
                );
                ((*h).mc.mc_luma).expect("non-null function pointer")(
                    pixv,
                    32 as libc::c_int as intptr_t,
                    &mut *((*m).p_fref).as_mut_ptr().offset(8 as libc::c_int as isize),
                    (*m).i_stride[2 as libc::c_int as usize] as intptr_t,
                    pmx,
                    pmy,
                    bw,
                    bh,
                    &*((*m).weight).offset(2 as libc::c_int as isize),
                );
            } else if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc != 0
                && (*m).i_pixel <= PIXEL_8x8 as libc::c_int
            {
                ((*h).mc.mc_chroma).expect("non-null function pointer")(
                    pixu,
                    pixv,
                    32 as libc::c_int as intptr_t,
                    (*m).p_fref[4 as libc::c_int as usize],
                    (*m).i_stride[1 as libc::c_int as usize] as intptr_t,
                    pmx,
                    (2 as libc::c_int * (pmy + mvy_offset)) >> chroma_v_shift,
                    bw >> 1 as libc::c_int,
                    bh >> chroma_v_shift,
                );
                if !((*((*m).weight).offset(1 as libc::c_int as isize)).weightfn).is_null() {
                    (*((*((*m).weight).offset(1 as libc::c_int as isize)).weightfn)
                        .offset((bw >> 3 as libc::c_int) as isize))
                    .expect("non-null function pointer")(
                        pixu,
                        32 as libc::c_int as intptr_t,
                        pixu,
                        32 as libc::c_int as intptr_t,
                        &*((*m).weight).offset(1 as libc::c_int as isize),
                        bh >> chroma_v_shift,
                    );
                }
                if !((*((*m).weight).offset(2 as libc::c_int as isize)).weightfn).is_null() {
                    (*((*((*m).weight).offset(2 as libc::c_int as isize)).weightfn)
                        .offset((bw >> 3 as libc::c_int) as isize))
                    .expect("non-null function pointer")(
                        pixv,
                        32 as libc::c_int as intptr_t,
                        pixv,
                        32 as libc::c_int as intptr_t,
                        &*((*m).weight).offset(2 as libc::c_int as isize),
                        bh >> chroma_v_shift,
                    );
                }
            }
            cost_0 = x264_8_rd_cost_part(h, i_lambda2, i4, (*m).i_pixel);
            if cost_0 < bcost {
                bcost = cost_0;
                bmx = pmx;
                bmy = pmy;
                dir = if 0 as libc::c_int != 0 {
                    0 as libc::c_int
                } else {
                    dir
                };
            }
        }
        if bmx == pmx && bmy == pmy {
            pmx = (*m).mv[0 as libc::c_int as usize] as libc::c_int;
            pmy = (*m).mv[1 as libc::c_int as usize] as libc::c_int;
        }
    }
    if bmy < (*h).mb.mv_min_spel[1 as libc::c_int as usize] + 3 as libc::c_int
        || bmy > (*h).mb.mv_max_spel[1 as libc::c_int as usize] - 3 as libc::c_int
        || bmx < (*h).mb.mv_min_spel[0 as libc::c_int as usize] + 3 as libc::c_int
        || bmx > (*h).mb.mv_max_spel[0 as libc::c_int as usize] - 3 as libc::c_int
    {
        (*h).mb.b_skip_mc = 0 as libc::c_int;
        return;
    }
    dir = -(2 as libc::c_int);
    omx = bmx;
    omy = bmy;
    let mut j: libc::c_int = 0 as libc::c_int;
    while j < 6 as libc::c_int {
        if 1 as libc::c_int == 0
            || !(omx
                + hex2[(j + 1 as libc::c_int) as usize][0 as libc::c_int as usize] as libc::c_int
                == pmx
                && omy
                    + hex2[(j + 1 as libc::c_int) as usize][1 as libc::c_int as usize]
                        as libc::c_int
                    == pmy)
        {
            ((*h).mc.mc_luma).expect("non-null function pointer")(
                pix,
                32 as libc::c_int as intptr_t,
                ((*m).p_fref).as_mut_ptr(),
                (*m).i_stride[0 as libc::c_int as usize] as intptr_t,
                omx + hex2[(j + 1 as libc::c_int) as usize][0 as libc::c_int as usize]
                    as libc::c_int,
                omy + hex2[(j + 1 as libc::c_int) as usize][1 as libc::c_int as usize]
                    as libc::c_int,
                bw,
                bh,
                &*((*m).weight).offset(0 as libc::c_int as isize),
            );
            satd = ((*h).pixf.mbcmp[i_pixel as usize]).expect("non-null function pointer")(
                (*m).p_fenc[0 as libc::c_int as usize],
                16 as libc::c_int as intptr_t,
                pix,
                32 as libc::c_int as intptr_t,
            ) + *p_cost_mvx.offset(
                (omx + hex2[(j + 1 as libc::c_int) as usize][0 as libc::c_int as usize]
                    as libc::c_int) as isize,
            ) as libc::c_int
                + *p_cost_mvy.offset(
                    (omy + hex2[(j + 1 as libc::c_int) as usize][1 as libc::c_int as usize]
                        as libc::c_int) as isize,
                ) as libc::c_int;
            if satd < bsatd {
                bsatd = satd;
            }
        } else {
            satd = (1 as libc::c_int) << 28 as libc::c_int;
        }
        if satd <= bsatd + (bsatd >> 4 as libc::c_int) {
            let mut cost_1: uint64_t = 0;
            (*(cache_mv as *mut x264_union32_t)).i = pack16to32_mask(
                omx + hex2[(j + 1 as libc::c_int) as usize][0 as libc::c_int as usize]
                    as libc::c_int,
                omy + hex2[(j + 1 as libc::c_int) as usize][1 as libc::c_int as usize]
                    as libc::c_int,
            );
            if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as libc::c_int {
                ((*h).mc.mc_luma).expect("non-null function pointer")(
                    pixu,
                    32 as libc::c_int as intptr_t,
                    &mut *((*m).p_fref).as_mut_ptr().offset(4 as libc::c_int as isize),
                    (*m).i_stride[1 as libc::c_int as usize] as intptr_t,
                    omx + hex2[(j + 1 as libc::c_int) as usize][0 as libc::c_int as usize]
                        as libc::c_int,
                    omy + hex2[(j + 1 as libc::c_int) as usize][1 as libc::c_int as usize]
                        as libc::c_int,
                    bw,
                    bh,
                    &*((*m).weight).offset(1 as libc::c_int as isize),
                );
                ((*h).mc.mc_luma).expect("non-null function pointer")(
                    pixv,
                    32 as libc::c_int as intptr_t,
                    &mut *((*m).p_fref).as_mut_ptr().offset(8 as libc::c_int as isize),
                    (*m).i_stride[2 as libc::c_int as usize] as intptr_t,
                    omx + hex2[(j + 1 as libc::c_int) as usize][0 as libc::c_int as usize]
                        as libc::c_int,
                    omy + hex2[(j + 1 as libc::c_int) as usize][1 as libc::c_int as usize]
                        as libc::c_int,
                    bw,
                    bh,
                    &*((*m).weight).offset(2 as libc::c_int as isize),
                );
            } else if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc != 0
                && (*m).i_pixel <= PIXEL_8x8 as libc::c_int
            {
                ((*h).mc.mc_chroma).expect("non-null function pointer")(
                    pixu,
                    pixv,
                    32 as libc::c_int as intptr_t,
                    (*m).p_fref[4 as libc::c_int as usize],
                    (*m).i_stride[1 as libc::c_int as usize] as intptr_t,
                    omx + hex2[(j + 1 as libc::c_int) as usize][0 as libc::c_int as usize]
                        as libc::c_int,
                    (2 as libc::c_int
                        * (omy
                            + hex2[(j + 1 as libc::c_int) as usize][1 as libc::c_int as usize]
                                as libc::c_int
                            + mvy_offset))
                        >> chroma_v_shift,
                    bw >> 1 as libc::c_int,
                    bh >> chroma_v_shift,
                );
                if !((*((*m).weight).offset(1 as libc::c_int as isize)).weightfn).is_null() {
                    (*((*((*m).weight).offset(1 as libc::c_int as isize)).weightfn)
                        .offset((bw >> 3 as libc::c_int) as isize))
                    .expect("non-null function pointer")(
                        pixu,
                        32 as libc::c_int as intptr_t,
                        pixu,
                        32 as libc::c_int as intptr_t,
                        &*((*m).weight).offset(1 as libc::c_int as isize),
                        bh >> chroma_v_shift,
                    );
                }
                if !((*((*m).weight).offset(2 as libc::c_int as isize)).weightfn).is_null() {
                    (*((*((*m).weight).offset(2 as libc::c_int as isize)).weightfn)
                        .offset((bw >> 3 as libc::c_int) as isize))
                    .expect("non-null function pointer")(
                        pixv,
                        32 as libc::c_int as intptr_t,
                        pixv,
                        32 as libc::c_int as intptr_t,
                        &*((*m).weight).offset(2 as libc::c_int as isize),
                        bh >> chroma_v_shift,
                    );
                }
            }
            cost_1 = x264_8_rd_cost_part(h, i_lambda2, i4, (*m).i_pixel);
            if cost_1 < bcost {
                bcost = cost_1;
                bmx = omx
                    + hex2[(j + 1 as libc::c_int) as usize][0 as libc::c_int as usize]
                        as libc::c_int;
                bmy = omy
                    + hex2[(j + 1 as libc::c_int) as usize][1 as libc::c_int as usize]
                        as libc::c_int;
                dir = if 1 as libc::c_int != 0 { j } else { dir };
            }
        }
        j += 1;
        j;
    }
    if dir != -(2 as libc::c_int) {
        let mut i: libc::c_int = 1 as libc::c_int;
        while i < 10 as libc::c_int {
            let odir: libc::c_int = mod6m1[(dir + 1 as libc::c_int) as usize] as libc::c_int;
            if bmy < (*h).mb.mv_min_spel[1 as libc::c_int as usize] + 3 as libc::c_int
                || bmy > (*h).mb.mv_max_spel[1 as libc::c_int as usize] - 3 as libc::c_int
            {
                break;
            }
            dir = -(2 as libc::c_int);
            omx = bmx;
            omy = bmy;
            let mut j_0: libc::c_int = 0 as libc::c_int;
            while j_0 < 3 as libc::c_int {
                if 1 as libc::c_int == 0
                    || !(omx
                        + hex2[(odir + j_0) as usize][0 as libc::c_int as usize] as libc::c_int
                        == pmx
                        && omy
                            + hex2[(odir + j_0) as usize][1 as libc::c_int as usize] as libc::c_int
                            == pmy)
                {
                    ((*h).mc.mc_luma).expect("non-null function pointer")(
                        pix,
                        32 as libc::c_int as intptr_t,
                        ((*m).p_fref).as_mut_ptr(),
                        (*m).i_stride[0 as libc::c_int as usize] as intptr_t,
                        omx + hex2[(odir + j_0) as usize][0 as libc::c_int as usize] as libc::c_int,
                        omy + hex2[(odir + j_0) as usize][1 as libc::c_int as usize] as libc::c_int,
                        bw,
                        bh,
                        &*((*m).weight).offset(0 as libc::c_int as isize),
                    );
                    satd = ((*h).pixf.mbcmp[i_pixel as usize]).expect("non-null function pointer")(
                        (*m).p_fenc[0 as libc::c_int as usize],
                        16 as libc::c_int as intptr_t,
                        pix,
                        32 as libc::c_int as intptr_t,
                    ) + *p_cost_mvx.offset(
                        (omx + hex2[(odir + j_0) as usize][0 as libc::c_int as usize]
                            as libc::c_int) as isize,
                    ) as libc::c_int
                        + *p_cost_mvy.offset(
                            (omy + hex2[(odir + j_0) as usize][1 as libc::c_int as usize]
                                as libc::c_int) as isize,
                        ) as libc::c_int;
                    if satd < bsatd {
                        bsatd = satd;
                    }
                } else {
                    satd = (1 as libc::c_int) << 28 as libc::c_int;
                }
                if satd <= bsatd + (bsatd >> 4 as libc::c_int) {
                    let mut cost_2: uint64_t = 0;
                    (*(cache_mv as *mut x264_union32_t)).i = pack16to32_mask(
                        omx + hex2[(odir + j_0) as usize][0 as libc::c_int as usize] as libc::c_int,
                        omy + hex2[(odir + j_0) as usize][1 as libc::c_int as usize] as libc::c_int,
                    );
                    if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as libc::c_int {
                        ((*h).mc.mc_luma).expect("non-null function pointer")(
                            pixu,
                            32 as libc::c_int as intptr_t,
                            &mut *((*m).p_fref).as_mut_ptr().offset(4 as libc::c_int as isize),
                            (*m).i_stride[1 as libc::c_int as usize] as intptr_t,
                            omx + hex2[(odir + j_0) as usize][0 as libc::c_int as usize]
                                as libc::c_int,
                            omy + hex2[(odir + j_0) as usize][1 as libc::c_int as usize]
                                as libc::c_int,
                            bw,
                            bh,
                            &*((*m).weight).offset(1 as libc::c_int as isize),
                        );
                        ((*h).mc.mc_luma).expect("non-null function pointer")(
                            pixv,
                            32 as libc::c_int as intptr_t,
                            &mut *((*m).p_fref).as_mut_ptr().offset(8 as libc::c_int as isize),
                            (*m).i_stride[2 as libc::c_int as usize] as intptr_t,
                            omx + hex2[(odir + j_0) as usize][0 as libc::c_int as usize]
                                as libc::c_int,
                            omy + hex2[(odir + j_0) as usize][1 as libc::c_int as usize]
                                as libc::c_int,
                            bw,
                            bh,
                            &*((*m).weight).offset(2 as libc::c_int as isize),
                        );
                    } else if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc != 0
                        && (*m).i_pixel <= PIXEL_8x8 as libc::c_int
                    {
                        ((*h).mc.mc_chroma).expect("non-null function pointer")(
                            pixu,
                            pixv,
                            32 as libc::c_int as intptr_t,
                            (*m).p_fref[4 as libc::c_int as usize],
                            (*m).i_stride[1 as libc::c_int as usize] as intptr_t,
                            omx + hex2[(odir + j_0) as usize][0 as libc::c_int as usize]
                                as libc::c_int,
                            (2 as libc::c_int
                                * (omy
                                    + hex2[(odir + j_0) as usize][1 as libc::c_int as usize]
                                        as libc::c_int
                                    + mvy_offset))
                                >> chroma_v_shift,
                            bw >> 1 as libc::c_int,
                            bh >> chroma_v_shift,
                        );
                        if !((*((*m).weight).offset(1 as libc::c_int as isize)).weightfn).is_null()
                        {
                            (*((*((*m).weight).offset(1 as libc::c_int as isize)).weightfn)
                                .offset((bw >> 3 as libc::c_int) as isize))
                            .expect("non-null function pointer")(
                                pixu,
                                32 as libc::c_int as intptr_t,
                                pixu,
                                32 as libc::c_int as intptr_t,
                                &*((*m).weight).offset(1 as libc::c_int as isize),
                                bh >> chroma_v_shift,
                            );
                        }
                        if !((*((*m).weight).offset(2 as libc::c_int as isize)).weightfn).is_null()
                        {
                            (*((*((*m).weight).offset(2 as libc::c_int as isize)).weightfn)
                                .offset((bw >> 3 as libc::c_int) as isize))
                            .expect("non-null function pointer")(
                                pixv,
                                32 as libc::c_int as intptr_t,
                                pixv,
                                32 as libc::c_int as intptr_t,
                                &*((*m).weight).offset(2 as libc::c_int as isize),
                                bh >> chroma_v_shift,
                            );
                        }
                    }
                    cost_2 = x264_8_rd_cost_part(h, i_lambda2, i4, (*m).i_pixel);
                    if cost_2 < bcost {
                        bcost = cost_2;
                        bmx = omx
                            + hex2[(odir + j_0) as usize][0 as libc::c_int as usize] as libc::c_int;
                        bmy = omy
                            + hex2[(odir + j_0) as usize][1 as libc::c_int as usize] as libc::c_int;
                        dir = if 1 as libc::c_int != 0 {
                            odir - 1 as libc::c_int + j_0
                        } else {
                            dir
                        };
                    }
                }
                j_0 += 1;
                j_0;
            }
            if dir == -(2 as libc::c_int) {
                break;
            }
            i += 1;
            i;
        }
    }
    omx = bmx;
    omy = bmy;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < 8 as libc::c_int {
        if 1 as libc::c_int == 0
            || !(omx
                + square1[(i_0 + 1 as libc::c_int) as usize][0 as libc::c_int as usize]
                    as libc::c_int
                == pmx
                && omy
                    + square1[(i_0 + 1 as libc::c_int) as usize][1 as libc::c_int as usize]
                        as libc::c_int
                    == pmy)
        {
            ((*h).mc.mc_luma).expect("non-null function pointer")(
                pix,
                32 as libc::c_int as intptr_t,
                ((*m).p_fref).as_mut_ptr(),
                (*m).i_stride[0 as libc::c_int as usize] as intptr_t,
                omx + square1[(i_0 + 1 as libc::c_int) as usize][0 as libc::c_int as usize]
                    as libc::c_int,
                omy + square1[(i_0 + 1 as libc::c_int) as usize][1 as libc::c_int as usize]
                    as libc::c_int,
                bw,
                bh,
                &*((*m).weight).offset(0 as libc::c_int as isize),
            );
            satd = ((*h).pixf.mbcmp[i_pixel as usize]).expect("non-null function pointer")(
                (*m).p_fenc[0 as libc::c_int as usize],
                16 as libc::c_int as intptr_t,
                pix,
                32 as libc::c_int as intptr_t,
            ) + *p_cost_mvx.offset(
                (omx + square1[(i_0 + 1 as libc::c_int) as usize][0 as libc::c_int as usize]
                    as libc::c_int) as isize,
            ) as libc::c_int
                + *p_cost_mvy.offset(
                    (omy + square1[(i_0 + 1 as libc::c_int) as usize][1 as libc::c_int as usize]
                        as libc::c_int) as isize,
                ) as libc::c_int;
            if satd < bsatd {
                bsatd = satd;
            }
        } else {
            satd = (1 as libc::c_int) << 28 as libc::c_int;
        }
        if satd <= bsatd + (bsatd >> 4 as libc::c_int) {
            let mut cost_3: uint64_t = 0;
            (*(cache_mv as *mut x264_union32_t)).i = pack16to32_mask(
                omx + square1[(i_0 + 1 as libc::c_int) as usize][0 as libc::c_int as usize]
                    as libc::c_int,
                omy + square1[(i_0 + 1 as libc::c_int) as usize][1 as libc::c_int as usize]
                    as libc::c_int,
            );
            if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as libc::c_int {
                ((*h).mc.mc_luma).expect("non-null function pointer")(
                    pixu,
                    32 as libc::c_int as intptr_t,
                    &mut *((*m).p_fref).as_mut_ptr().offset(4 as libc::c_int as isize),
                    (*m).i_stride[1 as libc::c_int as usize] as intptr_t,
                    omx + square1[(i_0 + 1 as libc::c_int) as usize][0 as libc::c_int as usize]
                        as libc::c_int,
                    omy + square1[(i_0 + 1 as libc::c_int) as usize][1 as libc::c_int as usize]
                        as libc::c_int,
                    bw,
                    bh,
                    &*((*m).weight).offset(1 as libc::c_int as isize),
                );
                ((*h).mc.mc_luma).expect("non-null function pointer")(
                    pixv,
                    32 as libc::c_int as intptr_t,
                    &mut *((*m).p_fref).as_mut_ptr().offset(8 as libc::c_int as isize),
                    (*m).i_stride[2 as libc::c_int as usize] as intptr_t,
                    omx + square1[(i_0 + 1 as libc::c_int) as usize][0 as libc::c_int as usize]
                        as libc::c_int,
                    omy + square1[(i_0 + 1 as libc::c_int) as usize][1 as libc::c_int as usize]
                        as libc::c_int,
                    bw,
                    bh,
                    &*((*m).weight).offset(2 as libc::c_int as isize),
                );
            } else if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc != 0
                && (*m).i_pixel <= PIXEL_8x8 as libc::c_int
            {
                ((*h).mc.mc_chroma).expect("non-null function pointer")(
                    pixu,
                    pixv,
                    32 as libc::c_int as intptr_t,
                    (*m).p_fref[4 as libc::c_int as usize],
                    (*m).i_stride[1 as libc::c_int as usize] as intptr_t,
                    omx + square1[(i_0 + 1 as libc::c_int) as usize][0 as libc::c_int as usize]
                        as libc::c_int,
                    (2 as libc::c_int
                        * (omy
                            + square1[(i_0 + 1 as libc::c_int) as usize][1 as libc::c_int as usize]
                                as libc::c_int
                            + mvy_offset))
                        >> chroma_v_shift,
                    bw >> 1 as libc::c_int,
                    bh >> chroma_v_shift,
                );
                if !((*((*m).weight).offset(1 as libc::c_int as isize)).weightfn).is_null() {
                    (*((*((*m).weight).offset(1 as libc::c_int as isize)).weightfn)
                        .offset((bw >> 3 as libc::c_int) as isize))
                    .expect("non-null function pointer")(
                        pixu,
                        32 as libc::c_int as intptr_t,
                        pixu,
                        32 as libc::c_int as intptr_t,
                        &*((*m).weight).offset(1 as libc::c_int as isize),
                        bh >> chroma_v_shift,
                    );
                }
                if !((*((*m).weight).offset(2 as libc::c_int as isize)).weightfn).is_null() {
                    (*((*((*m).weight).offset(2 as libc::c_int as isize)).weightfn)
                        .offset((bw >> 3 as libc::c_int) as isize))
                    .expect("non-null function pointer")(
                        pixv,
                        32 as libc::c_int as intptr_t,
                        pixv,
                        32 as libc::c_int as intptr_t,
                        &*((*m).weight).offset(2 as libc::c_int as isize),
                        bh >> chroma_v_shift,
                    );
                }
            }
            cost_3 = x264_8_rd_cost_part(h, i_lambda2, i4, (*m).i_pixel);
            if cost_3 < bcost {
                bcost = cost_3;
                bmx = omx
                    + square1[(i_0 + 1 as libc::c_int) as usize][0 as libc::c_int as usize]
                        as libc::c_int;
                bmy = omy
                    + square1[(i_0 + 1 as libc::c_int) as usize][1 as libc::c_int as usize]
                        as libc::c_int;
                dir = if 0 as libc::c_int != 0 {
                    0 as libc::c_int
                } else {
                    dir
                };
            }
        }
        i_0 += 1;
        i_0;
    }
    (*m).cost = bcost as libc::c_int;
    (*m).mv[0 as libc::c_int as usize] = bmx as int16_t;
    (*m).mv[1 as libc::c_int as usize] = bmy as int16_t;
    x264_macroblock_cache_mv(
        h,
        block_idx_x[i4 as usize] as libc::c_int,
        block_idx_y[i4 as usize] as libc::c_int,
        bw >> 2 as libc::c_int,
        bh >> 2 as libc::c_int,
        i_list,
        pack16to32_mask(bmx, bmy),
    );
    amvd = pack8to16(
        (if abs(bmx - (*m).mvp[0 as libc::c_int as usize] as libc::c_int) < 66 as libc::c_int {
            abs(bmx - (*m).mvp[0 as libc::c_int as usize] as libc::c_int)
        } else {
            66 as libc::c_int
        }) as uint32_t,
        (if abs(bmy - (*m).mvp[1 as libc::c_int as usize] as libc::c_int) < 66 as libc::c_int {
            abs(bmy - (*m).mvp[1 as libc::c_int as usize] as libc::c_int)
        } else {
            66 as libc::c_int
        }) as uint32_t,
    ) as uint16_t;
    x264_macroblock_cache_mvd(
        h,
        block_idx_x[i4 as usize] as libc::c_int,
        block_idx_y[i4 as usize] as libc::c_int,
        bw >> 2 as libc::c_int,
        bh >> 2 as libc::c_int,
        i_list,
        amvd,
    );
    (*h).mb.b_skip_mc = 0 as libc::c_int;
}
