use crate::types::*;
extern "C" {
    fn abs(_: libc::c_int) -> libc::c_int;
    fn x264_8_prefetch_fenc(
        h: *mut x264_t,
        fenc: *mut x264_frame_t,
        i_mb_x: libc::c_int,
        i_mb_y: libc::c_int,
    );
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
unsafe extern "C" fn x264_clip_pixel(mut x: libc::c_int) -> pixel {
    (if x & !(((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int) != 0 {
        (-x >> 31 as libc::c_int) & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
    } else {
        x
    }) as pixel
}
#[inline(always)]
unsafe extern "C" fn deblock_edge_luma_c(
    mut pix: *mut pixel,
    mut xstride: intptr_t,
    mut alpha: libc::c_int,
    mut beta: libc::c_int,
    mut tc0: int8_t,
) {
    let mut p2: libc::c_int =
        *pix.offset((-(3 as libc::c_int) as intptr_t * xstride) as isize) as libc::c_int;
    let mut p1: libc::c_int =
        *pix.offset((-(2 as libc::c_int) as intptr_t * xstride) as isize) as libc::c_int;
    let mut p0: libc::c_int =
        *pix.offset((-(1 as libc::c_int) as intptr_t * xstride) as isize) as libc::c_int;
    let mut q0: libc::c_int =
        *pix.offset((0 as libc::c_int as intptr_t * xstride) as isize) as libc::c_int;
    let mut q1: libc::c_int =
        *pix.offset((1 as libc::c_int as intptr_t * xstride) as isize) as libc::c_int;
    let mut q2: libc::c_int =
        *pix.offset((2 as libc::c_int as intptr_t * xstride) as isize) as libc::c_int;
    if abs(p0 - q0) < alpha && abs(p1 - p0) < beta && abs(q1 - q0) < beta {
        let mut tc: libc::c_int = tc0 as libc::c_int;
        let mut delta: libc::c_int = 0;
        if abs(p2 - p0) < beta {
            if tc0 != 0 {
                *pix.offset((-(2 as libc::c_int) as intptr_t * xstride) as isize) =
                    (p1 + x264_clip3(
                        ((p2 + ((p0 + q0 + 1 as libc::c_int) >> 1 as libc::c_int))
                            >> 1 as libc::c_int)
                            - p1,
                        -(tc0 as libc::c_int),
                        tc0 as libc::c_int,
                    )) as pixel;
            }
            tc += 1;
            tc;
        }
        if abs(q2 - q0) < beta {
            if tc0 != 0 {
                *pix.offset((1 as libc::c_int as intptr_t * xstride) as isize) =
                    (q1 + x264_clip3(
                        ((q2 + ((p0 + q0 + 1 as libc::c_int) >> 1 as libc::c_int))
                            >> 1 as libc::c_int)
                            - q1,
                        -(tc0 as libc::c_int),
                        tc0 as libc::c_int,
                    )) as pixel;
            }
            tc += 1;
            tc;
        }
        delta = x264_clip3(
            ((q0 - p0) * 4 as libc::c_int + (p1 - q1) + 4 as libc::c_int) >> 3 as libc::c_int,
            -tc,
            tc,
        );
        *pix.offset((-(1 as libc::c_int) as intptr_t * xstride) as isize) =
            x264_clip_pixel(p0 + delta);
        *pix.offset((0 as libc::c_int as intptr_t * xstride) as isize) =
            x264_clip_pixel(q0 - delta);
    }
}
#[inline]
unsafe extern "C" fn deblock_luma_c(
    mut pix: *mut pixel,
    mut xstride: intptr_t,
    mut ystride: intptr_t,
    mut alpha: libc::c_int,
    mut beta: libc::c_int,
    mut tc0: *mut int8_t,
) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        if (*tc0.offset(i as isize) as libc::c_int) < 0 as libc::c_int {
            pix = pix.offset((4 as libc::c_int as intptr_t * ystride) as isize);
        } else {
            let mut d: libc::c_int = 0 as libc::c_int;
            while d < 4 as libc::c_int {
                deblock_edge_luma_c(pix, xstride, alpha, beta, *tc0.offset(i as isize));
                d += 1;
                d;
                pix = pix.offset(ystride as isize);
            }
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn deblock_h_luma_mbaff_c(
    mut pix: *mut pixel,
    mut stride: intptr_t,
    mut alpha: libc::c_int,
    mut beta: libc::c_int,
    mut tc0: *mut int8_t,
) {
    let mut d: libc::c_int = 0 as libc::c_int;
    while d < 8 as libc::c_int {
        deblock_edge_luma_c(
            pix,
            1 as libc::c_int as intptr_t,
            alpha,
            beta,
            *tc0.offset((d >> 1 as libc::c_int) as isize),
        );
        d += 1;
        d;
        pix = pix.offset(stride as isize);
    }
}
unsafe extern "C" fn deblock_v_luma_c(
    mut pix: *mut pixel,
    mut stride: intptr_t,
    mut alpha: libc::c_int,
    mut beta: libc::c_int,
    mut tc0: *mut int8_t,
) {
    deblock_luma_c(pix, stride, 1 as libc::c_int as intptr_t, alpha, beta, tc0);
}
unsafe extern "C" fn deblock_h_luma_c(
    mut pix: *mut pixel,
    mut stride: intptr_t,
    mut alpha: libc::c_int,
    mut beta: libc::c_int,
    mut tc0: *mut int8_t,
) {
    deblock_luma_c(pix, 1 as libc::c_int as intptr_t, stride, alpha, beta, tc0);
}
#[inline(always)]
unsafe extern "C" fn deblock_edge_chroma_c(
    mut pix: *mut pixel,
    mut xstride: intptr_t,
    mut alpha: libc::c_int,
    mut beta: libc::c_int,
    mut tc: int8_t,
) {
    let mut p1: libc::c_int =
        *pix.offset((-(2 as libc::c_int) as intptr_t * xstride) as isize) as libc::c_int;
    let mut p0: libc::c_int =
        *pix.offset((-(1 as libc::c_int) as intptr_t * xstride) as isize) as libc::c_int;
    let mut q0: libc::c_int =
        *pix.offset((0 as libc::c_int as intptr_t * xstride) as isize) as libc::c_int;
    let mut q1: libc::c_int =
        *pix.offset((1 as libc::c_int as intptr_t * xstride) as isize) as libc::c_int;
    if abs(p0 - q0) < alpha && abs(p1 - p0) < beta && abs(q1 - q0) < beta {
        let mut delta: libc::c_int = x264_clip3(
            ((q0 - p0) * 4 as libc::c_int + (p1 - q1) + 4 as libc::c_int) >> 3 as libc::c_int,
            -(tc as libc::c_int),
            tc as libc::c_int,
        );
        *pix.offset((-(1 as libc::c_int) as intptr_t * xstride) as isize) =
            x264_clip_pixel(p0 + delta);
        *pix.offset((0 as libc::c_int as intptr_t * xstride) as isize) =
            x264_clip_pixel(q0 - delta);
    }
}
#[inline(always)]
unsafe extern "C" fn deblock_chroma_c(
    mut pix: *mut pixel,
    mut height: libc::c_int,
    mut xstride: intptr_t,
    mut ystride: intptr_t,
    mut alpha: libc::c_int,
    mut beta: libc::c_int,
    mut tc0: *mut int8_t,
) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        let mut tc: libc::c_int = *tc0.offset(i as isize) as libc::c_int;
        if tc <= 0 as libc::c_int {
            pix = pix.offset((height as intptr_t * ystride) as isize);
        } else {
            let mut d: libc::c_int = 0 as libc::c_int;
            while d < height {
                let mut e: libc::c_int = 0 as libc::c_int;
                while e < 2 as libc::c_int {
                    deblock_edge_chroma_c(pix, xstride, alpha, beta, *tc0.offset(i as isize));
                    e += 1;
                    e;
                    pix = pix.offset(1);
                    pix;
                }
                d += 1;
                d;
                pix = pix.offset((ystride - 2 as libc::c_int as intptr_t) as isize);
            }
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn deblock_h_chroma_mbaff_c(
    mut pix: *mut pixel,
    mut stride: intptr_t,
    mut alpha: libc::c_int,
    mut beta: libc::c_int,
    mut tc0: *mut int8_t,
) {
    deblock_chroma_c(
        pix,
        1 as libc::c_int,
        2 as libc::c_int as intptr_t,
        stride,
        alpha,
        beta,
        tc0,
    );
}
unsafe extern "C" fn deblock_v_chroma_c(
    mut pix: *mut pixel,
    mut stride: intptr_t,
    mut alpha: libc::c_int,
    mut beta: libc::c_int,
    mut tc0: *mut int8_t,
) {
    deblock_chroma_c(
        pix,
        2 as libc::c_int,
        stride,
        2 as libc::c_int as intptr_t,
        alpha,
        beta,
        tc0,
    );
}
unsafe extern "C" fn deblock_h_chroma_c(
    mut pix: *mut pixel,
    mut stride: intptr_t,
    mut alpha: libc::c_int,
    mut beta: libc::c_int,
    mut tc0: *mut int8_t,
) {
    deblock_chroma_c(
        pix,
        2 as libc::c_int,
        2 as libc::c_int as intptr_t,
        stride,
        alpha,
        beta,
        tc0,
    );
}
unsafe extern "C" fn deblock_h_chroma_422_c(
    mut pix: *mut pixel,
    mut stride: intptr_t,
    mut alpha: libc::c_int,
    mut beta: libc::c_int,
    mut tc0: *mut int8_t,
) {
    deblock_chroma_c(
        pix,
        4 as libc::c_int,
        2 as libc::c_int as intptr_t,
        stride,
        alpha,
        beta,
        tc0,
    );
}
#[inline(always)]
unsafe extern "C" fn deblock_edge_luma_intra_c(
    mut pix: *mut pixel,
    mut xstride: intptr_t,
    mut alpha: libc::c_int,
    mut beta: libc::c_int,
) {
    let mut p2: libc::c_int =
        *pix.offset((-(3 as libc::c_int) as intptr_t * xstride) as isize) as libc::c_int;
    let mut p1: libc::c_int =
        *pix.offset((-(2 as libc::c_int) as intptr_t * xstride) as isize) as libc::c_int;
    let mut p0: libc::c_int =
        *pix.offset((-(1 as libc::c_int) as intptr_t * xstride) as isize) as libc::c_int;
    let mut q0: libc::c_int =
        *pix.offset((0 as libc::c_int as intptr_t * xstride) as isize) as libc::c_int;
    let mut q1: libc::c_int =
        *pix.offset((1 as libc::c_int as intptr_t * xstride) as isize) as libc::c_int;
    let mut q2: libc::c_int =
        *pix.offset((2 as libc::c_int as intptr_t * xstride) as isize) as libc::c_int;
    if abs(p0 - q0) < alpha && abs(p1 - p0) < beta && abs(q1 - q0) < beta {
        if abs(p0 - q0) < (alpha >> 2 as libc::c_int) + 2 as libc::c_int {
            if abs(p2 - p0) < beta {
                let p3: libc::c_int = *pix
                    .offset((-(4 as libc::c_int) as intptr_t * xstride) as isize)
                    as libc::c_int;
                *pix.offset((-(1 as libc::c_int) as intptr_t * xstride) as isize) =
                    ((p2 + 2 as libc::c_int * p1
                        + 2 as libc::c_int * p0
                        + 2 as libc::c_int * q0
                        + q1
                        + 4 as libc::c_int)
                        >> 3 as libc::c_int) as pixel;
                *pix.offset((-(2 as libc::c_int) as intptr_t * xstride) as isize) =
                    ((p2 + p1 + p0 + q0 + 2 as libc::c_int) >> 2 as libc::c_int) as pixel;
                *pix.offset((-(3 as libc::c_int) as intptr_t * xstride) as isize) =
                    ((2 as libc::c_int * p3
                        + 3 as libc::c_int * p2
                        + p1
                        + p0
                        + q0
                        + 4 as libc::c_int)
                        >> 3 as libc::c_int) as pixel;
            } else {
                *pix.offset((-(1 as libc::c_int) as intptr_t * xstride) as isize) =
                    ((2 as libc::c_int * p1 + p0 + q1 + 2 as libc::c_int) >> 2 as libc::c_int)
                        as pixel;
            }
            if abs(q2 - q0) < beta {
                let q3: libc::c_int =
                    *pix.offset((3 as libc::c_int as intptr_t * xstride) as isize) as libc::c_int;
                *pix.offset((0 as libc::c_int as intptr_t * xstride) as isize) =
                    ((p1 + 2 as libc::c_int * p0
                        + 2 as libc::c_int * q0
                        + 2 as libc::c_int * q1
                        + q2
                        + 4 as libc::c_int)
                        >> 3 as libc::c_int) as pixel;
                *pix.offset((1 as libc::c_int as intptr_t * xstride) as isize) =
                    ((p0 + q0 + q1 + q2 + 2 as libc::c_int) >> 2 as libc::c_int) as pixel;
                *pix.offset((2 as libc::c_int as intptr_t * xstride) as isize) =
                    ((2 as libc::c_int * q3
                        + 3 as libc::c_int * q2
                        + q1
                        + q0
                        + p0
                        + 4 as libc::c_int)
                        >> 3 as libc::c_int) as pixel;
            } else {
                *pix.offset((0 as libc::c_int as intptr_t * xstride) as isize) =
                    ((2 as libc::c_int * q1 + q0 + p1 + 2 as libc::c_int) >> 2 as libc::c_int)
                        as pixel;
            }
        } else {
            *pix.offset((-(1 as libc::c_int) as intptr_t * xstride) as isize) =
                ((2 as libc::c_int * p1 + p0 + q1 + 2 as libc::c_int) >> 2 as libc::c_int) as pixel;
            *pix.offset((0 as libc::c_int as intptr_t * xstride) as isize) =
                ((2 as libc::c_int * q1 + q0 + p1 + 2 as libc::c_int) >> 2 as libc::c_int) as pixel;
        }
    }
}
#[inline]
unsafe extern "C" fn deblock_luma_intra_c(
    mut pix: *mut pixel,
    mut xstride: intptr_t,
    mut ystride: intptr_t,
    mut alpha: libc::c_int,
    mut beta: libc::c_int,
) {
    let mut d: libc::c_int = 0 as libc::c_int;
    while d < 16 as libc::c_int {
        deblock_edge_luma_intra_c(pix, xstride, alpha, beta);
        d += 1;
        d;
        pix = pix.offset(ystride as isize);
    }
}
unsafe extern "C" fn deblock_h_luma_intra_mbaff_c(
    mut pix: *mut pixel,
    mut ystride: intptr_t,
    mut alpha: libc::c_int,
    mut beta: libc::c_int,
) {
    let mut d: libc::c_int = 0 as libc::c_int;
    while d < 8 as libc::c_int {
        deblock_edge_luma_intra_c(pix, 1 as libc::c_int as intptr_t, alpha, beta);
        d += 1;
        d;
        pix = pix.offset(ystride as isize);
    }
}
unsafe extern "C" fn deblock_v_luma_intra_c(
    mut pix: *mut pixel,
    mut stride: intptr_t,
    mut alpha: libc::c_int,
    mut beta: libc::c_int,
) {
    deblock_luma_intra_c(pix, stride, 1 as libc::c_int as intptr_t, alpha, beta);
}
unsafe extern "C" fn deblock_h_luma_intra_c(
    mut pix: *mut pixel,
    mut stride: intptr_t,
    mut alpha: libc::c_int,
    mut beta: libc::c_int,
) {
    deblock_luma_intra_c(pix, 1 as libc::c_int as intptr_t, stride, alpha, beta);
}
#[inline(always)]
unsafe extern "C" fn deblock_edge_chroma_intra_c(
    mut pix: *mut pixel,
    mut xstride: intptr_t,
    mut alpha: libc::c_int,
    mut beta: libc::c_int,
) {
    let mut p1: libc::c_int =
        *pix.offset((-(2 as libc::c_int) as intptr_t * xstride) as isize) as libc::c_int;
    let mut p0: libc::c_int =
        *pix.offset((-(1 as libc::c_int) as intptr_t * xstride) as isize) as libc::c_int;
    let mut q0: libc::c_int =
        *pix.offset((0 as libc::c_int as intptr_t * xstride) as isize) as libc::c_int;
    let mut q1: libc::c_int =
        *pix.offset((1 as libc::c_int as intptr_t * xstride) as isize) as libc::c_int;
    if abs(p0 - q0) < alpha && abs(p1 - p0) < beta && abs(q1 - q0) < beta {
        *pix.offset((-(1 as libc::c_int) as intptr_t * xstride) as isize) =
            ((2 as libc::c_int * p1 + p0 + q1 + 2 as libc::c_int) >> 2 as libc::c_int) as pixel;
        *pix.offset((0 as libc::c_int as intptr_t * xstride) as isize) =
            ((2 as libc::c_int * q1 + q0 + p1 + 2 as libc::c_int) >> 2 as libc::c_int) as pixel;
    }
}
#[inline(always)]
unsafe extern "C" fn deblock_chroma_intra_c(
    mut pix: *mut pixel,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut xstride: intptr_t,
    mut ystride: intptr_t,
    mut alpha: libc::c_int,
    mut beta: libc::c_int,
) {
    let mut d: libc::c_int = 0 as libc::c_int;
    while d < height {
        let mut e: libc::c_int = 0 as libc::c_int;
        while e < width {
            deblock_edge_chroma_intra_c(pix, xstride, alpha, beta);
            e += 1;
            e;
            pix = pix.offset(1);
            pix;
        }
        d += 1;
        d;
        pix = pix.offset((ystride - 2 as libc::c_int as intptr_t) as isize);
    }
}
unsafe extern "C" fn deblock_h_chroma_intra_mbaff_c(
    mut pix: *mut pixel,
    mut stride: intptr_t,
    mut alpha: libc::c_int,
    mut beta: libc::c_int,
) {
    deblock_chroma_intra_c(
        pix,
        2 as libc::c_int,
        4 as libc::c_int,
        2 as libc::c_int as intptr_t,
        stride,
        alpha,
        beta,
    );
}
unsafe extern "C" fn deblock_v_chroma_intra_c(
    mut pix: *mut pixel,
    mut stride: intptr_t,
    mut alpha: libc::c_int,
    mut beta: libc::c_int,
) {
    deblock_chroma_intra_c(
        pix,
        1 as libc::c_int,
        16 as libc::c_int,
        stride,
        2 as libc::c_int as intptr_t,
        alpha,
        beta,
    );
}
unsafe extern "C" fn deblock_h_chroma_intra_c(
    mut pix: *mut pixel,
    mut stride: intptr_t,
    mut alpha: libc::c_int,
    mut beta: libc::c_int,
) {
    deblock_chroma_intra_c(
        pix,
        2 as libc::c_int,
        8 as libc::c_int,
        2 as libc::c_int as intptr_t,
        stride,
        alpha,
        beta,
    );
}
unsafe extern "C" fn deblock_h_chroma_422_intra_c(
    mut pix: *mut pixel,
    mut stride: intptr_t,
    mut alpha: libc::c_int,
    mut beta: libc::c_int,
) {
    deblock_chroma_intra_c(
        pix,
        2 as libc::c_int,
        16 as libc::c_int,
        2 as libc::c_int as intptr_t,
        stride,
        alpha,
        beta,
    );
}
unsafe extern "C" fn deblock_strength_c(
    mut nnz: *mut uint8_t,
    mut ref_0: *mut [int8_t; 40],
    mut mv: *mut [[int16_t; 2]; 40],
    mut bs: *mut [[uint8_t; 4]; 8],
    mut mvy_limit: libc::c_int,
    mut bframe: libc::c_int,
) {
    let mut dir: libc::c_int = 0 as libc::c_int;
    while dir < 2 as libc::c_int {
        let mut s1: libc::c_int = if dir != 0 {
            1 as libc::c_int
        } else {
            8 as libc::c_int
        };
        let mut s2: libc::c_int = if dir != 0 {
            8 as libc::c_int
        } else {
            1 as libc::c_int
        };
        let mut edge: libc::c_int = 0 as libc::c_int;
        while edge < 4 as libc::c_int {
            let mut i: libc::c_int = 0 as libc::c_int;
            let mut loc: libc::c_int =
                4 as libc::c_int + 1 as libc::c_int * 8 as libc::c_int + edge * s2;
            while i < 4 as libc::c_int {
                let mut locn: libc::c_int = loc - s2;
                if *nnz.offset(loc as isize) as libc::c_int != 0
                    || *nnz.offset(locn as isize) as libc::c_int != 0
                {
                    (*bs.offset(dir as isize))[edge as usize][i as usize] =
                        2 as libc::c_int as uint8_t;
                } else if (*ref_0.offset(0 as libc::c_int as isize))[loc as usize] as libc::c_int
                    != (*ref_0.offset(0 as libc::c_int as isize))[locn as usize] as libc::c_int
                    || abs((*mv.offset(0 as libc::c_int as isize))[loc as usize]
                        [0 as libc::c_int as usize] as libc::c_int
                        - (*mv.offset(0 as libc::c_int as isize))[locn as usize]
                            [0 as libc::c_int as usize] as libc::c_int)
                        >= 4 as libc::c_int
                    || abs((*mv.offset(0 as libc::c_int as isize))[loc as usize]
                        [1 as libc::c_int as usize] as libc::c_int
                        - (*mv.offset(0 as libc::c_int as isize))[locn as usize]
                            [1 as libc::c_int as usize] as libc::c_int)
                        >= mvy_limit
                    || bframe != 0
                        && ((*ref_0.offset(1 as libc::c_int as isize))[loc as usize] as libc::c_int
                            != (*ref_0.offset(1 as libc::c_int as isize))[locn as usize]
                                as libc::c_int
                            || abs((*mv.offset(1 as libc::c_int as isize))[loc as usize]
                                [0 as libc::c_int as usize]
                                as libc::c_int
                                - (*mv.offset(1 as libc::c_int as isize))[locn as usize]
                                    [0 as libc::c_int as usize]
                                    as libc::c_int)
                                >= 4 as libc::c_int
                            || abs((*mv.offset(1 as libc::c_int as isize))[loc as usize]
                                [1 as libc::c_int as usize]
                                as libc::c_int
                                - (*mv.offset(1 as libc::c_int as isize))[locn as usize]
                                    [1 as libc::c_int as usize]
                                    as libc::c_int)
                                >= mvy_limit)
                {
                    (*bs.offset(dir as isize))[edge as usize][i as usize] =
                        1 as libc::c_int as uint8_t;
                } else {
                    (*bs.offset(dir as isize))[edge as usize][i as usize] =
                        0 as libc::c_int as uint8_t;
                }
                i += 1;
                i;
                loc += s1;
            }
            edge += 1;
            edge;
        }
        dir += 1;
        dir;
    }
}
#[inline(always)]
unsafe extern "C" fn deblock_edge(
    mut h: *mut x264_t,
    mut pix: *mut pixel,
    mut i_stride: intptr_t,
    mut bS: *mut uint8_t,
    mut i_qp: libc::c_int,
    mut a: libc::c_int,
    mut b: libc::c_int,
    mut b_chroma: libc::c_int,
    mut pf_inter: x264_deblock_inter_t,
) {
    let mut index_a: libc::c_int = i_qp + a;
    let mut index_b: libc::c_int = i_qp + b;
    let mut alpha: libc::c_int = (i_alpha_table[(index_a + 24 as libc::c_int) as usize]
        as libc::c_int)
        << (8 as libc::c_int - 8 as libc::c_int);
    let mut beta: libc::c_int = (i_beta_table[(index_b + 24 as libc::c_int) as usize]
        as libc::c_int)
        << (8 as libc::c_int - 8 as libc::c_int);
    let mut tc: [int8_t; 4] = [0; 4];
    if (*(bS as *mut x264_union32_t)).i == 0 || alpha == 0 || beta == 0 {
        return;
    }
    tc[0 as libc::c_int as usize] = (i_tc0_table[(index_a + 24 as libc::c_int) as usize]
        [*bS.offset(0 as libc::c_int as isize) as usize]
        as libc::c_int
        * ((1 as libc::c_int) << (8 as libc::c_int - 8 as libc::c_int))
        + b_chroma) as int8_t;
    tc[1 as libc::c_int as usize] = (i_tc0_table[(index_a + 24 as libc::c_int) as usize]
        [*bS.offset(1 as libc::c_int as isize) as usize]
        as libc::c_int
        * ((1 as libc::c_int) << (8 as libc::c_int - 8 as libc::c_int))
        + b_chroma) as int8_t;
    tc[2 as libc::c_int as usize] = (i_tc0_table[(index_a + 24 as libc::c_int) as usize]
        [*bS.offset(2 as libc::c_int as isize) as usize]
        as libc::c_int
        * ((1 as libc::c_int) << (8 as libc::c_int - 8 as libc::c_int))
        + b_chroma) as int8_t;
    tc[3 as libc::c_int as usize] = (i_tc0_table[(index_a + 24 as libc::c_int) as usize]
        [*bS.offset(3 as libc::c_int as isize) as usize]
        as libc::c_int
        * ((1 as libc::c_int) << (8 as libc::c_int - 8 as libc::c_int))
        + b_chroma) as int8_t;
    pf_inter.expect("non-null function pointer")(pix, i_stride, alpha, beta, tc.as_mut_ptr());
}
#[inline(always)]
unsafe extern "C" fn deblock_edge_intra(
    mut h: *mut x264_t,
    mut pix: *mut pixel,
    mut i_stride: intptr_t,
    mut bS: *mut uint8_t,
    mut i_qp: libc::c_int,
    mut a: libc::c_int,
    mut b: libc::c_int,
    mut b_chroma: libc::c_int,
    mut pf_intra: x264_deblock_intra_t,
) {
    let mut index_a: libc::c_int = i_qp + a;
    let mut index_b: libc::c_int = i_qp + b;
    let mut alpha: libc::c_int = (i_alpha_table[(index_a + 24 as libc::c_int) as usize]
        as libc::c_int)
        << (8 as libc::c_int - 8 as libc::c_int);
    let mut beta: libc::c_int = (i_beta_table[(index_b + 24 as libc::c_int) as usize]
        as libc::c_int)
        << (8 as libc::c_int - 8 as libc::c_int);
    if alpha == 0 || beta == 0 {
        return;
    }
    pf_intra.expect("non-null function pointer")(pix, i_stride, alpha, beta);
}
#[inline(always)]
unsafe extern "C" fn macroblock_cache_load_neighbours_deblock(
    mut h: *mut x264_t,
    mut mb_x: libc::c_int,
    mut mb_y: libc::c_int,
) {
    let mut deblock_on_slice_edges: libc::c_int =
        ((*h).sh.i_disable_deblocking_filter_idc != 2 as libc::c_int) as libc::c_int;
    (*h).mb.i_neighbour = 0 as libc::c_int as libc::c_uint;
    (*h).mb.i_mb_xy = mb_y * (*h).mb.i_mb_stride + mb_x;
    (*h).mb.b_interlaced = ((*h).param.b_interlaced != 0
        && *((*h).mb.field).offset((*h).mb.i_mb_xy as isize) as libc::c_int != 0)
        as libc::c_int;
    (*h).mb.i_mb_top_y = mb_y - ((1 as libc::c_int) << (*h).mb.b_interlaced);
    (*h).mb.i_mb_top_xy = mb_x + (*h).mb.i_mb_stride * (*h).mb.i_mb_top_y;
    (*h).mb.i_mb_left_xy[0 as libc::c_int as usize] = (*h).mb.i_mb_xy - 1 as libc::c_int;
    (*h).mb.i_mb_left_xy[1 as libc::c_int as usize] =
        (*h).mb.i_mb_left_xy[0 as libc::c_int as usize];
    if (*h).sh.b_mbaff != 0 {
        if mb_y & 1 as libc::c_int != 0 {
            if mb_x != 0
                && *((*h).mb.field).offset(((*h).mb.i_mb_xy - 1 as libc::c_int) as isize)
                    as libc::c_int
                    != (*h).mb.b_interlaced
            {
                (*h).mb.i_mb_left_xy[0 as libc::c_int as usize] -= (*h).mb.i_mb_stride;
            }
        } else {
            if (*h).mb.i_mb_top_xy >= 0 as libc::c_int
                && (*h).mb.b_interlaced != 0
                && *((*h).mb.field).offset((*h).mb.i_mb_top_xy as isize) == 0
            {
                (*h).mb.i_mb_top_xy += (*h).mb.i_mb_stride;
                (*h).mb.i_mb_top_y += 1;
                (*h).mb.i_mb_top_y;
            }
            if mb_x != 0
                && *((*h).mb.field).offset(((*h).mb.i_mb_xy - 1 as libc::c_int) as isize)
                    as libc::c_int
                    != (*h).mb.b_interlaced
            {
                (*h).mb.i_mb_left_xy[1 as libc::c_int as usize] += (*h).mb.i_mb_stride;
            }
        }
    }
    if mb_x > 0 as libc::c_int
        && (deblock_on_slice_edges != 0
            || *((*h).mb.slice_table)
                .offset((*h).mb.i_mb_left_xy[0 as libc::c_int as usize] as isize)
                == *((*h).mb.slice_table).offset((*h).mb.i_mb_xy as isize))
    {
        (*h).mb.i_neighbour |= MB_LEFT as libc::c_int as libc::c_uint;
    }
    if mb_y > (*h).mb.b_interlaced
        && (deblock_on_slice_edges != 0
            || *((*h).mb.slice_table).offset((*h).mb.i_mb_top_xy as isize)
                == *((*h).mb.slice_table).offset((*h).mb.i_mb_xy as isize))
    {
        (*h).mb.i_neighbour |= MB_TOP as libc::c_int as libc::c_uint;
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_frame_deblock_row(mut h: *mut x264_t, mut mb_y: libc::c_int) {
    let mut b_interlaced: libc::c_int = (*h).sh.b_mbaff;
    let mut a: libc::c_int =
        (*h).sh.i_alpha_c0_offset - 6 as libc::c_int * (8 as libc::c_int - 8 as libc::c_int);
    let mut b: libc::c_int =
        (*h).sh.i_beta_offset - 6 as libc::c_int * (8 as libc::c_int - 8 as libc::c_int);
    let mut qp_thresh: libc::c_int = 15 as libc::c_int
        - (if a < b { a } else { b })
        - (if 0 as libc::c_int > (*((*h).pps).as_mut_ptr()).i_chroma_qp_index_offset {
            0 as libc::c_int
        } else {
            (*((*h).pps).as_mut_ptr()).i_chroma_qp_index_offset
        });
    let mut stridey: libc::c_int = (*(*h).fdec).i_stride[0 as libc::c_int as usize];
    let mut strideuv: libc::c_int = (*(*h).fdec).i_stride[1 as libc::c_int as usize];
    let mut chroma_format: libc::c_int = (*((*h).sps).as_mut_ptr()).i_chroma_format_idc;
    let mut chroma444: libc::c_int = ((*((*h).sps).as_mut_ptr()).i_chroma_format_idc
        == CHROMA_444 as libc::c_int) as libc::c_int;
    let mut chroma_height: libc::c_int = 16 as libc::c_int >> (*h).mb.chroma_v_shift;
    let mut uvdiff: intptr_t = if chroma444 != 0 {
        ((*(*h).fdec).plane[2 as libc::c_int as usize])
            .offset_from((*(*h).fdec).plane[1 as libc::c_int as usize]) as libc::c_long
    } else {
        1 as libc::c_int as libc::c_long
    };
    let mut mb_x: libc::c_int = 0 as libc::c_int;
    while mb_x < (*h).mb.i_mb_width {
        x264_8_prefetch_fenc(h, (*h).fdec, mb_x, mb_y);
        macroblock_cache_load_neighbours_deblock(h, mb_x, mb_y);
        let mut mb_xy: libc::c_int = (*h).mb.i_mb_xy;
        let mut transform_8x8: libc::c_int =
            *((*h).mb.mb_transform_size).offset(mb_xy as isize) as libc::c_int;
        let mut intra_cur: libc::c_int = (*((*h).mb.type_0).offset(mb_xy as isize) as libc::c_int
            == I_4x4 as libc::c_int
            || *((*h).mb.type_0).offset(mb_xy as isize) as libc::c_int == I_8x8 as libc::c_int
            || *((*h).mb.type_0).offset(mb_xy as isize) as libc::c_int == I_16x16 as libc::c_int
            || *((*h).mb.type_0).offset(mb_xy as isize) as libc::c_int == I_PCM as libc::c_int)
            as libc::c_int;
        let mut bs: *mut [[uint8_t; 4]; 8] =
            (*((*h).deblock_strength[(mb_y & 1 as libc::c_int) as usize]).offset(
                (if (*h).param.b_sliced_threads != 0 {
                    mb_xy
                } else {
                    mb_x
                }) as isize,
            ))
            .as_mut_ptr();
        let mut pixy: *mut pixel = ((*(*h).fdec).plane[0 as libc::c_int as usize])
            .offset((16 as libc::c_int * mb_y * stridey) as isize)
            .offset((16 as libc::c_int * mb_x) as isize);
        let mut pixuv: *mut pixel = if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc != 0 {
            ((*(*h).fdec).plane[1 as libc::c_int as usize])
                .offset((chroma_height * mb_y * strideuv) as isize)
                .offset((16 as libc::c_int * mb_x) as isize)
        } else {
            std::ptr::null_mut::<pixel>()
        };
        if mb_y & (*h).mb.b_interlaced != 0 {
            pixy = pixy.offset(-((15 as libc::c_int * stridey) as isize));
            if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc != 0 {
                pixuv = pixuv.offset(-(((chroma_height - 1 as libc::c_int) * strideuv) as isize));
            }
        }
        let mut stride2y: libc::c_int = stridey << (*h).mb.b_interlaced;
        let mut stride2uv: libc::c_int = strideuv << (*h).mb.b_interlaced;
        let mut qp: libc::c_int = *((*h).mb.qp).offset(mb_xy as isize) as libc::c_int;
        let mut qpc: libc::c_int = *((*h).chroma_qp_table).offset(qp as isize) as libc::c_int;
        let mut first_edge_only: libc::c_int =
            (*((*h).mb.partition).offset(mb_xy as isize) as libc::c_int == D_16x16 as libc::c_int
                && *((*h).mb.cbp).offset(mb_xy as isize) == 0
                && intra_cur == 0
                || qp <= qp_thresh) as libc::c_int;
        if (*h).mb.i_neighbour & MB_LEFT as libc::c_int as libc::c_uint != 0 {
            if b_interlaced != 0
                && *((*h).mb.field).offset((*h).mb.i_mb_left_xy[0 as libc::c_int as usize] as isize)
                    as libc::c_int
                    != (*h).mb.b_interlaced
            {
                let mut luma_qp: [libc::c_int; 2] = [0; 2];
                let mut chroma_qp: [libc::c_int; 2] = [0; 2];
                let mut left_qp: [libc::c_int; 2] = [0; 2];
                let mut luma_deblock: x264_deblock_inter_t = (*h).loopf.deblock_luma_mbaff;
                let mut chroma_deblock: x264_deblock_inter_t = (*h).loopf.deblock_chroma_mbaff;
                let mut luma_intra_deblock: x264_deblock_intra_t =
                    (*h).loopf.deblock_luma_intra_mbaff;
                let mut chroma_intra_deblock: x264_deblock_intra_t =
                    (*h).loopf.deblock_chroma_intra_mbaff;
                let mut c: libc::c_int = if chroma444 != 0 {
                    0 as libc::c_int
                } else {
                    1 as libc::c_int
                };
                left_qp[0 as libc::c_int as usize] = *((*h).mb.qp)
                    .offset((*h).mb.i_mb_left_xy[0 as libc::c_int as usize] as isize)
                    as libc::c_int;
                luma_qp[0 as libc::c_int as usize] =
                    (qp + left_qp[0 as libc::c_int as usize] + 1 as libc::c_int)
                        >> 1 as libc::c_int;
                chroma_qp[0 as libc::c_int as usize] = (qpc
                    + *((*h).chroma_qp_table).offset(left_qp[0 as libc::c_int as usize] as isize)
                        as libc::c_int
                    + 1 as libc::c_int)
                    >> 1 as libc::c_int;
                if intra_cur != 0
                    || (*((*h).mb.type_0)
                        .offset((*h).mb.i_mb_left_xy[0 as libc::c_int as usize] as isize)
                        as libc::c_int
                        == I_4x4 as libc::c_int
                        || *((*h).mb.type_0)
                            .offset((*h).mb.i_mb_left_xy[0 as libc::c_int as usize] as isize)
                            as libc::c_int
                            == I_8x8 as libc::c_int
                        || *((*h).mb.type_0)
                            .offset((*h).mb.i_mb_left_xy[0 as libc::c_int as usize] as isize)
                            as libc::c_int
                            == I_16x16 as libc::c_int
                        || *((*h).mb.type_0)
                            .offset((*h).mb.i_mb_left_xy[0 as libc::c_int as usize] as isize)
                            as libc::c_int
                            == I_PCM as libc::c_int)
                {
                    deblock_edge_intra(
                        h,
                        pixy,
                        (2 as libc::c_int * stridey) as intptr_t,
                        ((*bs.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize])
                            .as_mut_ptr(),
                        luma_qp[0 as libc::c_int as usize],
                        a,
                        b,
                        0 as libc::c_int,
                        luma_intra_deblock,
                    );
                    if chroma_format != 0 {
                        deblock_edge_intra(
                            h,
                            pixuv,
                            (2 as libc::c_int * strideuv) as intptr_t,
                            ((*bs.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize])
                                .as_mut_ptr(),
                            chroma_qp[0 as libc::c_int as usize],
                            a,
                            b,
                            c,
                            chroma_intra_deblock,
                        );
                        if chroma444 != 0 {
                            deblock_edge_intra(
                                h,
                                pixuv.offset(uvdiff as isize),
                                (2 as libc::c_int * strideuv) as intptr_t,
                                ((*bs.offset(0 as libc::c_int as isize))
                                    [0 as libc::c_int as usize])
                                    .as_mut_ptr(),
                                chroma_qp[0 as libc::c_int as usize],
                                a,
                                b,
                                c,
                                chroma_intra_deblock,
                            );
                        }
                    }
                } else {
                    deblock_edge(
                        h,
                        pixy,
                        (2 as libc::c_int * stridey) as intptr_t,
                        ((*bs.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize])
                            .as_mut_ptr(),
                        luma_qp[0 as libc::c_int as usize],
                        a,
                        b,
                        0 as libc::c_int,
                        luma_deblock,
                    );
                    if chroma_format != 0 {
                        deblock_edge(
                            h,
                            pixuv,
                            (2 as libc::c_int * strideuv) as intptr_t,
                            ((*bs.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize])
                                .as_mut_ptr(),
                            chroma_qp[0 as libc::c_int as usize],
                            a,
                            b,
                            c,
                            chroma_deblock,
                        );
                        if chroma444 != 0 {
                            deblock_edge(
                                h,
                                pixuv.offset(uvdiff as isize),
                                (2 as libc::c_int * strideuv) as intptr_t,
                                ((*bs.offset(0 as libc::c_int as isize))
                                    [0 as libc::c_int as usize])
                                    .as_mut_ptr(),
                                chroma_qp[0 as libc::c_int as usize],
                                a,
                                b,
                                c,
                                chroma_deblock,
                            );
                        }
                    }
                }
                let mut offy: libc::c_int = if (*h).mb.b_interlaced != 0 {
                    4 as libc::c_int
                } else {
                    0 as libc::c_int
                };
                let mut offuv: libc::c_int = if (*h).mb.b_interlaced != 0 {
                    4 as libc::c_int - (*h).mb.chroma_v_shift
                } else {
                    0 as libc::c_int
                };
                left_qp[1 as libc::c_int as usize] = *((*h).mb.qp)
                    .offset((*h).mb.i_mb_left_xy[1 as libc::c_int as usize] as isize)
                    as libc::c_int;
                luma_qp[1 as libc::c_int as usize] =
                    (qp + left_qp[1 as libc::c_int as usize] + 1 as libc::c_int)
                        >> 1 as libc::c_int;
                chroma_qp[1 as libc::c_int as usize] = (qpc
                    + *((*h).chroma_qp_table).offset(left_qp[1 as libc::c_int as usize] as isize)
                        as libc::c_int
                    + 1 as libc::c_int)
                    >> 1 as libc::c_int;
                if intra_cur != 0
                    || (*((*h).mb.type_0)
                        .offset((*h).mb.i_mb_left_xy[1 as libc::c_int as usize] as isize)
                        as libc::c_int
                        == I_4x4 as libc::c_int
                        || *((*h).mb.type_0)
                            .offset((*h).mb.i_mb_left_xy[1 as libc::c_int as usize] as isize)
                            as libc::c_int
                            == I_8x8 as libc::c_int
                        || *((*h).mb.type_0)
                            .offset((*h).mb.i_mb_left_xy[1 as libc::c_int as usize] as isize)
                            as libc::c_int
                            == I_16x16 as libc::c_int
                        || *((*h).mb.type_0)
                            .offset((*h).mb.i_mb_left_xy[1 as libc::c_int as usize] as isize)
                            as libc::c_int
                            == I_PCM as libc::c_int)
                {
                    deblock_edge_intra(
                        h,
                        pixy.offset((stridey << offy) as isize),
                        (2 as libc::c_int * stridey) as intptr_t,
                        ((*bs.offset(0 as libc::c_int as isize))[4 as libc::c_int as usize])
                            .as_mut_ptr(),
                        luma_qp[1 as libc::c_int as usize],
                        a,
                        b,
                        0 as libc::c_int,
                        luma_intra_deblock,
                    );
                    if chroma_format != 0 {
                        deblock_edge_intra(
                            h,
                            pixuv.offset((strideuv << offuv) as isize),
                            (2 as libc::c_int * strideuv) as intptr_t,
                            ((*bs.offset(0 as libc::c_int as isize))[4 as libc::c_int as usize])
                                .as_mut_ptr(),
                            chroma_qp[1 as libc::c_int as usize],
                            a,
                            b,
                            c,
                            chroma_intra_deblock,
                        );
                        if chroma444 != 0 {
                            deblock_edge_intra(
                                h,
                                pixuv
                                    .offset(uvdiff as isize)
                                    .offset((strideuv << offuv) as isize),
                                (2 as libc::c_int * strideuv) as intptr_t,
                                ((*bs.offset(0 as libc::c_int as isize))
                                    [4 as libc::c_int as usize])
                                    .as_mut_ptr(),
                                chroma_qp[1 as libc::c_int as usize],
                                a,
                                b,
                                c,
                                chroma_intra_deblock,
                            );
                        }
                    }
                } else {
                    deblock_edge(
                        h,
                        pixy.offset((stridey << offy) as isize),
                        (2 as libc::c_int * stridey) as intptr_t,
                        ((*bs.offset(0 as libc::c_int as isize))[4 as libc::c_int as usize])
                            .as_mut_ptr(),
                        luma_qp[1 as libc::c_int as usize],
                        a,
                        b,
                        0 as libc::c_int,
                        luma_deblock,
                    );
                    if chroma_format != 0 {
                        deblock_edge(
                            h,
                            pixuv.offset((strideuv << offuv) as isize),
                            (2 as libc::c_int * strideuv) as intptr_t,
                            ((*bs.offset(0 as libc::c_int as isize))[4 as libc::c_int as usize])
                                .as_mut_ptr(),
                            chroma_qp[1 as libc::c_int as usize],
                            a,
                            b,
                            c,
                            chroma_deblock,
                        );
                        if chroma444 != 0 {
                            deblock_edge(
                                h,
                                pixuv
                                    .offset(uvdiff as isize)
                                    .offset((strideuv << offuv) as isize),
                                (2 as libc::c_int * strideuv) as intptr_t,
                                ((*bs.offset(0 as libc::c_int as isize))
                                    [4 as libc::c_int as usize])
                                    .as_mut_ptr(),
                                chroma_qp[1 as libc::c_int as usize],
                                a,
                                b,
                                c,
                                chroma_deblock,
                            );
                        }
                    }
                }
            } else {
                let mut qpl: libc::c_int = *((*h).mb.qp)
                    .offset(((*h).mb.i_mb_xy - 1 as libc::c_int) as isize)
                    as libc::c_int;
                let mut qp_left: libc::c_int = (qp + qpl + 1 as libc::c_int) >> 1 as libc::c_int;
                let mut qpc_left: libc::c_int = (qpc
                    + *((*h).chroma_qp_table).offset(qpl as isize) as libc::c_int
                    + 1 as libc::c_int)
                    >> 1 as libc::c_int;
                let mut intra_left: libc::c_int =
                    (*((*h).mb.type_0).offset(((*h).mb.i_mb_xy - 1 as libc::c_int) as isize)
                        as libc::c_int
                        == I_4x4 as libc::c_int
                        || *((*h).mb.type_0).offset(((*h).mb.i_mb_xy - 1 as libc::c_int) as isize)
                            as libc::c_int
                            == I_8x8 as libc::c_int
                        || *((*h).mb.type_0).offset(((*h).mb.i_mb_xy - 1 as libc::c_int) as isize)
                            as libc::c_int
                            == I_16x16 as libc::c_int
                        || *((*h).mb.type_0).offset(((*h).mb.i_mb_xy - 1 as libc::c_int) as isize)
                            as libc::c_int
                            == I_PCM as libc::c_int) as libc::c_int;
                let mut intra_deblock: libc::c_int =
                    (intra_cur != 0 || intra_left != 0) as libc::c_int;
                if !((*(*h).fdec).mb_info).is_null()
                    && (*(((*bs.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize])
                        .as_mut_ptr() as *mut x264_union32_t))
                        .i
                        != 0
                {
                    let fresh0 = &mut (*((*(*h).fdec).effective_qp).offset(mb_xy as isize));
                    *fresh0 = (*fresh0 as libc::c_int
                        | (0xff as libc::c_int
                            * (*((*(*h).fdec).mb_info).offset(mb_xy as isize) as libc::c_uint
                                & ((1 as libc::c_uint) << 0 as libc::c_int)
                                != 0) as libc::c_int)) as uint8_t;
                    let fresh1 = &mut (*((*(*h).fdec).effective_qp)
                        .offset((*h).mb.i_mb_left_xy[0 as libc::c_int as usize] as isize));
                    *fresh1 = (*fresh1 as libc::c_int
                        | (0xff as libc::c_int
                            * (*((*(*h).fdec).mb_info)
                                .offset((*h).mb.i_mb_left_xy[0 as libc::c_int as usize] as isize)
                                as libc::c_uint
                                & ((1 as libc::c_uint) << 0 as libc::c_int)
                                != 0) as libc::c_int)) as uint8_t;
                }
                if intra_deblock != 0 {
                    if 0 as libc::c_int & 1 as libc::c_int == 0 || transform_8x8 == 0 {
                        deblock_edge_intra(
                            h,
                            pixy.offset(
                                (4 as libc::c_int
                                    * 0 as libc::c_int
                                    * (if 0 as libc::c_int != 0 {
                                        stride2y
                                    } else {
                                        1 as libc::c_int
                                    })) as isize,
                            ),
                            stride2y as intptr_t,
                            ((*bs.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize])
                                .as_mut_ptr(),
                            qp_left,
                            a,
                            b,
                            0 as libc::c_int,
                            (*h).loopf.deblock_luma_intra[0 as libc::c_int as usize],
                        );
                        if chroma_format == CHROMA_444 as libc::c_int {
                            deblock_edge_intra(
                                h,
                                pixuv.offset(
                                    (4 as libc::c_int
                                        * 0 as libc::c_int
                                        * (if 0 as libc::c_int != 0 {
                                            stride2uv
                                        } else {
                                            1 as libc::c_int
                                        })) as isize,
                                ),
                                stride2uv as intptr_t,
                                ((*bs.offset(0 as libc::c_int as isize))
                                    [0 as libc::c_int as usize])
                                    .as_mut_ptr(),
                                qpc_left,
                                a,
                                b,
                                0 as libc::c_int,
                                (*h).loopf.deblock_luma_intra[0 as libc::c_int as usize],
                            );
                            deblock_edge_intra(
                                h,
                                pixuv.offset(uvdiff as isize).offset(
                                    (4 as libc::c_int
                                        * 0 as libc::c_int
                                        * (if 0 as libc::c_int != 0 {
                                            stride2uv
                                        } else {
                                            1 as libc::c_int
                                        })) as isize,
                                ),
                                stride2uv as intptr_t,
                                ((*bs.offset(0 as libc::c_int as isize))
                                    [0 as libc::c_int as usize])
                                    .as_mut_ptr(),
                                qpc_left,
                                a,
                                b,
                                0 as libc::c_int,
                                (*h).loopf.deblock_luma_intra[0 as libc::c_int as usize],
                            );
                        } else if chroma_format == CHROMA_420 as libc::c_int
                            && 0 as libc::c_int & 1 as libc::c_int == 0
                        {
                            deblock_edge_intra(
                                h,
                                pixuv.offset(
                                    (0 as libc::c_int
                                        * (if 0 as libc::c_int != 0 {
                                            2 as libc::c_int * stride2uv
                                        } else {
                                            4 as libc::c_int
                                        })) as isize,
                                ),
                                stride2uv as intptr_t,
                                ((*bs.offset(0 as libc::c_int as isize))
                                    [0 as libc::c_int as usize])
                                    .as_mut_ptr(),
                                qpc_left,
                                a,
                                b,
                                1 as libc::c_int,
                                (*h).loopf.deblock_chroma_intra[0 as libc::c_int as usize],
                            );
                        }
                    }
                    if chroma_format == CHROMA_422 as libc::c_int
                        && (0 as libc::c_int != 0 || 0 as libc::c_int & 1 as libc::c_int == 0)
                    {
                        deblock_edge_intra(
                            h,
                            pixuv.offset(
                                (0 as libc::c_int
                                    * (if 0 as libc::c_int != 0 {
                                        4 as libc::c_int * stride2uv
                                    } else {
                                        4 as libc::c_int
                                    })) as isize,
                            ),
                            stride2uv as intptr_t,
                            ((*bs.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize])
                                .as_mut_ptr(),
                            qpc_left,
                            a,
                            b,
                            1 as libc::c_int,
                            (*h).loopf.deblock_chroma_intra[0 as libc::c_int as usize],
                        );
                    }
                } else {
                    if 0 as libc::c_int & 1 as libc::c_int == 0 || transform_8x8 == 0 {
                        deblock_edge(
                            h,
                            pixy.offset(
                                (4 as libc::c_int
                                    * 0 as libc::c_int
                                    * (if 0 as libc::c_int != 0 {
                                        stride2y
                                    } else {
                                        1 as libc::c_int
                                    })) as isize,
                            ),
                            stride2y as intptr_t,
                            ((*bs.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize])
                                .as_mut_ptr(),
                            qp_left,
                            a,
                            b,
                            0 as libc::c_int,
                            (*h).loopf.deblock_luma[0 as libc::c_int as usize],
                        );
                        if chroma_format == CHROMA_444 as libc::c_int {
                            deblock_edge(
                                h,
                                pixuv.offset(
                                    (4 as libc::c_int
                                        * 0 as libc::c_int
                                        * (if 0 as libc::c_int != 0 {
                                            stride2uv
                                        } else {
                                            1 as libc::c_int
                                        })) as isize,
                                ),
                                stride2uv as intptr_t,
                                ((*bs.offset(0 as libc::c_int as isize))
                                    [0 as libc::c_int as usize])
                                    .as_mut_ptr(),
                                qpc_left,
                                a,
                                b,
                                0 as libc::c_int,
                                (*h).loopf.deblock_luma[0 as libc::c_int as usize],
                            );
                            deblock_edge(
                                h,
                                pixuv.offset(uvdiff as isize).offset(
                                    (4 as libc::c_int
                                        * 0 as libc::c_int
                                        * (if 0 as libc::c_int != 0 {
                                            stride2uv
                                        } else {
                                            1 as libc::c_int
                                        })) as isize,
                                ),
                                stride2uv as intptr_t,
                                ((*bs.offset(0 as libc::c_int as isize))
                                    [0 as libc::c_int as usize])
                                    .as_mut_ptr(),
                                qpc_left,
                                a,
                                b,
                                0 as libc::c_int,
                                (*h).loopf.deblock_luma[0 as libc::c_int as usize],
                            );
                        } else if chroma_format == CHROMA_420 as libc::c_int
                            && 0 as libc::c_int & 1 as libc::c_int == 0
                        {
                            deblock_edge(
                                h,
                                pixuv.offset(
                                    (0 as libc::c_int
                                        * (if 0 as libc::c_int != 0 {
                                            2 as libc::c_int * stride2uv
                                        } else {
                                            4 as libc::c_int
                                        })) as isize,
                                ),
                                stride2uv as intptr_t,
                                ((*bs.offset(0 as libc::c_int as isize))
                                    [0 as libc::c_int as usize])
                                    .as_mut_ptr(),
                                qpc_left,
                                a,
                                b,
                                1 as libc::c_int,
                                (*h).loopf.deblock_chroma[0 as libc::c_int as usize],
                            );
                        }
                    }
                    if chroma_format == CHROMA_422 as libc::c_int
                        && (0 as libc::c_int != 0 || 0 as libc::c_int & 1 as libc::c_int == 0)
                    {
                        deblock_edge(
                            h,
                            pixuv.offset(
                                (0 as libc::c_int
                                    * (if 0 as libc::c_int != 0 {
                                        4 as libc::c_int * stride2uv
                                    } else {
                                        4 as libc::c_int
                                    })) as isize,
                            ),
                            stride2uv as intptr_t,
                            ((*bs.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize])
                                .as_mut_ptr(),
                            qpc_left,
                            a,
                            b,
                            1 as libc::c_int,
                            (*h).loopf.deblock_chroma[0 as libc::c_int as usize],
                        );
                    }
                }
            }
        }
        if first_edge_only == 0 {
            if 1 as libc::c_int & 1 as libc::c_int == 0 || transform_8x8 == 0 {
                deblock_edge(
                    h,
                    pixy.offset(
                        (4 as libc::c_int
                            * 1 as libc::c_int
                            * (if 0 as libc::c_int != 0 {
                                stride2y
                            } else {
                                1 as libc::c_int
                            })) as isize,
                    ),
                    stride2y as intptr_t,
                    ((*bs.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize])
                        .as_mut_ptr(),
                    qp,
                    a,
                    b,
                    0 as libc::c_int,
                    (*h).loopf.deblock_luma[0 as libc::c_int as usize],
                );
                if chroma_format == CHROMA_444 as libc::c_int {
                    deblock_edge(
                        h,
                        pixuv.offset(
                            (4 as libc::c_int
                                * 1 as libc::c_int
                                * (if 0 as libc::c_int != 0 {
                                    stride2uv
                                } else {
                                    1 as libc::c_int
                                })) as isize,
                        ),
                        stride2uv as intptr_t,
                        ((*bs.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize])
                            .as_mut_ptr(),
                        qpc,
                        a,
                        b,
                        0 as libc::c_int,
                        (*h).loopf.deblock_luma[0 as libc::c_int as usize],
                    );
                    deblock_edge(
                        h,
                        pixuv.offset(uvdiff as isize).offset(
                            (4 as libc::c_int
                                * 1 as libc::c_int
                                * (if 0 as libc::c_int != 0 {
                                    stride2uv
                                } else {
                                    1 as libc::c_int
                                })) as isize,
                        ),
                        stride2uv as intptr_t,
                        ((*bs.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize])
                            .as_mut_ptr(),
                        qpc,
                        a,
                        b,
                        0 as libc::c_int,
                        (*h).loopf.deblock_luma[0 as libc::c_int as usize],
                    );
                } else if chroma_format == CHROMA_420 as libc::c_int
                    && 1 as libc::c_int & 1 as libc::c_int == 0
                {
                    deblock_edge(
                        h,
                        pixuv.offset(
                            (1 as libc::c_int
                                * (if 0 as libc::c_int != 0 {
                                    2 as libc::c_int * stride2uv
                                } else {
                                    4 as libc::c_int
                                })) as isize,
                        ),
                        stride2uv as intptr_t,
                        ((*bs.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize])
                            .as_mut_ptr(),
                        qpc,
                        a,
                        b,
                        1 as libc::c_int,
                        (*h).loopf.deblock_chroma[0 as libc::c_int as usize],
                    );
                }
            }
            if chroma_format == CHROMA_422 as libc::c_int
                && (0 as libc::c_int != 0 || 1 as libc::c_int & 1 as libc::c_int == 0)
            {
                deblock_edge(
                    h,
                    pixuv.offset(
                        (1 as libc::c_int
                            * (if 0 as libc::c_int != 0 {
                                4 as libc::c_int * stride2uv
                            } else {
                                4 as libc::c_int
                            })) as isize,
                    ),
                    stride2uv as intptr_t,
                    ((*bs.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize])
                        .as_mut_ptr(),
                    qpc,
                    a,
                    b,
                    1 as libc::c_int,
                    (*h).loopf.deblock_chroma[0 as libc::c_int as usize],
                );
            }
            if 2 as libc::c_int & 1 as libc::c_int == 0 || transform_8x8 == 0 {
                deblock_edge(
                    h,
                    pixy.offset(
                        (4 as libc::c_int
                            * 2 as libc::c_int
                            * (if 0 as libc::c_int != 0 {
                                stride2y
                            } else {
                                1 as libc::c_int
                            })) as isize,
                    ),
                    stride2y as intptr_t,
                    ((*bs.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize])
                        .as_mut_ptr(),
                    qp,
                    a,
                    b,
                    0 as libc::c_int,
                    (*h).loopf.deblock_luma[0 as libc::c_int as usize],
                );
                if chroma_format == CHROMA_444 as libc::c_int {
                    deblock_edge(
                        h,
                        pixuv.offset(
                            (4 as libc::c_int
                                * 2 as libc::c_int
                                * (if 0 as libc::c_int != 0 {
                                    stride2uv
                                } else {
                                    1 as libc::c_int
                                })) as isize,
                        ),
                        stride2uv as intptr_t,
                        ((*bs.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize])
                            .as_mut_ptr(),
                        qpc,
                        a,
                        b,
                        0 as libc::c_int,
                        (*h).loopf.deblock_luma[0 as libc::c_int as usize],
                    );
                    deblock_edge(
                        h,
                        pixuv.offset(uvdiff as isize).offset(
                            (4 as libc::c_int
                                * 2 as libc::c_int
                                * (if 0 as libc::c_int != 0 {
                                    stride2uv
                                } else {
                                    1 as libc::c_int
                                })) as isize,
                        ),
                        stride2uv as intptr_t,
                        ((*bs.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize])
                            .as_mut_ptr(),
                        qpc,
                        a,
                        b,
                        0 as libc::c_int,
                        (*h).loopf.deblock_luma[0 as libc::c_int as usize],
                    );
                } else if chroma_format == CHROMA_420 as libc::c_int
                    && 2 as libc::c_int & 1 as libc::c_int == 0
                {
                    deblock_edge(
                        h,
                        pixuv.offset(
                            (2 as libc::c_int
                                * (if 0 as libc::c_int != 0 {
                                    2 as libc::c_int * stride2uv
                                } else {
                                    4 as libc::c_int
                                })) as isize,
                        ),
                        stride2uv as intptr_t,
                        ((*bs.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize])
                            .as_mut_ptr(),
                        qpc,
                        a,
                        b,
                        1 as libc::c_int,
                        (*h).loopf.deblock_chroma[0 as libc::c_int as usize],
                    );
                }
            }
            if chroma_format == CHROMA_422 as libc::c_int
                && (0 as libc::c_int != 0 || 2 as libc::c_int & 1 as libc::c_int == 0)
            {
                deblock_edge(
                    h,
                    pixuv.offset(
                        (2 as libc::c_int
                            * (if 0 as libc::c_int != 0 {
                                4 as libc::c_int * stride2uv
                            } else {
                                4 as libc::c_int
                            })) as isize,
                    ),
                    stride2uv as intptr_t,
                    ((*bs.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize])
                        .as_mut_ptr(),
                    qpc,
                    a,
                    b,
                    1 as libc::c_int,
                    (*h).loopf.deblock_chroma[0 as libc::c_int as usize],
                );
            }
            if 3 as libc::c_int & 1 as libc::c_int == 0 || transform_8x8 == 0 {
                deblock_edge(
                    h,
                    pixy.offset(
                        (4 as libc::c_int
                            * 3 as libc::c_int
                            * (if 0 as libc::c_int != 0 {
                                stride2y
                            } else {
                                1 as libc::c_int
                            })) as isize,
                    ),
                    stride2y as intptr_t,
                    ((*bs.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize])
                        .as_mut_ptr(),
                    qp,
                    a,
                    b,
                    0 as libc::c_int,
                    (*h).loopf.deblock_luma[0 as libc::c_int as usize],
                );
                if chroma_format == CHROMA_444 as libc::c_int {
                    deblock_edge(
                        h,
                        pixuv.offset(
                            (4 as libc::c_int
                                * 3 as libc::c_int
                                * (if 0 as libc::c_int != 0 {
                                    stride2uv
                                } else {
                                    1 as libc::c_int
                                })) as isize,
                        ),
                        stride2uv as intptr_t,
                        ((*bs.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize])
                            .as_mut_ptr(),
                        qpc,
                        a,
                        b,
                        0 as libc::c_int,
                        (*h).loopf.deblock_luma[0 as libc::c_int as usize],
                    );
                    deblock_edge(
                        h,
                        pixuv.offset(uvdiff as isize).offset(
                            (4 as libc::c_int
                                * 3 as libc::c_int
                                * (if 0 as libc::c_int != 0 {
                                    stride2uv
                                } else {
                                    1 as libc::c_int
                                })) as isize,
                        ),
                        stride2uv as intptr_t,
                        ((*bs.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize])
                            .as_mut_ptr(),
                        qpc,
                        a,
                        b,
                        0 as libc::c_int,
                        (*h).loopf.deblock_luma[0 as libc::c_int as usize],
                    );
                } else if chroma_format == CHROMA_420 as libc::c_int
                    && 3 as libc::c_int & 1 as libc::c_int == 0
                {
                    deblock_edge(
                        h,
                        pixuv.offset(
                            (3 as libc::c_int
                                * (if 0 as libc::c_int != 0 {
                                    2 as libc::c_int * stride2uv
                                } else {
                                    4 as libc::c_int
                                })) as isize,
                        ),
                        stride2uv as intptr_t,
                        ((*bs.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize])
                            .as_mut_ptr(),
                        qpc,
                        a,
                        b,
                        1 as libc::c_int,
                        (*h).loopf.deblock_chroma[0 as libc::c_int as usize],
                    );
                }
            }
            if chroma_format == CHROMA_422 as libc::c_int
                && (0 as libc::c_int != 0 || 3 as libc::c_int & 1 as libc::c_int == 0)
            {
                deblock_edge(
                    h,
                    pixuv.offset(
                        (3 as libc::c_int
                            * (if 0 as libc::c_int != 0 {
                                4 as libc::c_int * stride2uv
                            } else {
                                4 as libc::c_int
                            })) as isize,
                    ),
                    stride2uv as intptr_t,
                    ((*bs.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize])
                        .as_mut_ptr(),
                    qpc,
                    a,
                    b,
                    1 as libc::c_int,
                    (*h).loopf.deblock_chroma[0 as libc::c_int as usize],
                );
            }
        }
        if (*h).mb.i_neighbour & MB_TOP as libc::c_int as libc::c_uint != 0 {
            if b_interlaced != 0
                && mb_y & 1 as libc::c_int == 0
                && (*h).mb.b_interlaced == 0
                && *((*h).mb.field).offset((*h).mb.i_mb_top_xy as isize) as libc::c_int != 0
            {
                let mut mbn_xy: libc::c_int = mb_xy - 2 as libc::c_int * (*h).mb.i_mb_stride;
                let mut j: libc::c_int = 0 as libc::c_int;
                while j < 2 as libc::c_int {
                    let mut qpt: libc::c_int = *((*h).mb.qp).offset(mbn_xy as isize) as libc::c_int;
                    let mut qp_top: libc::c_int = (qp + qpt + 1 as libc::c_int) >> 1 as libc::c_int;
                    let mut qpc_top: libc::c_int = (qpc
                        + *((*h).chroma_qp_table).offset(qpt as isize) as libc::c_int
                        + 1 as libc::c_int)
                        >> 1 as libc::c_int;
                    let mut intra_top: libc::c_int =
                        (*((*h).mb.type_0).offset(mbn_xy as isize) as libc::c_int
                            == I_4x4 as libc::c_int
                            || *((*h).mb.type_0).offset(mbn_xy as isize) as libc::c_int
                                == I_8x8 as libc::c_int
                            || *((*h).mb.type_0).offset(mbn_xy as isize) as libc::c_int
                                == I_16x16 as libc::c_int
                            || *((*h).mb.type_0).offset(mbn_xy as isize) as libc::c_int
                                == I_PCM as libc::c_int) as libc::c_int;
                    if intra_cur != 0 || intra_top != 0 {
                        (*(((*bs.offset(1 as libc::c_int as isize))
                            [(4 as libc::c_int * j) as usize])
                            .as_mut_ptr() as *mut x264_union32_t))
                            .i = 0x3030303 as libc::c_int as uint32_t;
                    }
                    deblock_edge(
                        h,
                        pixy.offset((j * stridey) as isize),
                        (2 as libc::c_int * stridey) as intptr_t,
                        ((*bs.offset(1 as libc::c_int as isize))[(4 as libc::c_int * j) as usize])
                            .as_mut_ptr(),
                        qp_top,
                        a,
                        b,
                        0 as libc::c_int,
                        (*h).loopf.deblock_luma[1 as libc::c_int as usize],
                    );
                    if chroma444 != 0 {
                        deblock_edge(
                            h,
                            pixuv.offset((j * strideuv) as isize),
                            (2 as libc::c_int * strideuv) as intptr_t,
                            ((*bs.offset(1 as libc::c_int as isize))
                                [(4 as libc::c_int * j) as usize])
                                .as_mut_ptr(),
                            qpc_top,
                            a,
                            b,
                            0 as libc::c_int,
                            (*h).loopf.deblock_luma[1 as libc::c_int as usize],
                        );
                        deblock_edge(
                            h,
                            pixuv
                                .offset(uvdiff as isize)
                                .offset((j * strideuv) as isize),
                            (2 as libc::c_int * strideuv) as intptr_t,
                            ((*bs.offset(1 as libc::c_int as isize))
                                [(4 as libc::c_int * j) as usize])
                                .as_mut_ptr(),
                            qpc_top,
                            a,
                            b,
                            0 as libc::c_int,
                            (*h).loopf.deblock_luma[1 as libc::c_int as usize],
                        );
                    } else if chroma_format != 0 {
                        deblock_edge(
                            h,
                            pixuv.offset((j * strideuv) as isize),
                            (2 as libc::c_int * strideuv) as intptr_t,
                            ((*bs.offset(1 as libc::c_int as isize))
                                [(4 as libc::c_int * j) as usize])
                                .as_mut_ptr(),
                            qpc_top,
                            a,
                            b,
                            1 as libc::c_int,
                            (*h).loopf.deblock_chroma[1 as libc::c_int as usize],
                        );
                    }
                    j += 1;
                    j;
                    mbn_xy += (*h).mb.i_mb_stride;
                }
            } else {
                let mut qpt_0: libc::c_int =
                    *((*h).mb.qp).offset((*h).mb.i_mb_top_xy as isize) as libc::c_int;
                let mut qp_top_0: libc::c_int = (qp + qpt_0 + 1 as libc::c_int) >> 1 as libc::c_int;
                let mut qpc_top_0: libc::c_int = (qpc
                    + *((*h).chroma_qp_table).offset(qpt_0 as isize) as libc::c_int
                    + 1 as libc::c_int)
                    >> 1 as libc::c_int;
                let mut intra_top_0: libc::c_int =
                    (*((*h).mb.type_0).offset((*h).mb.i_mb_top_xy as isize) as libc::c_int
                        == I_4x4 as libc::c_int
                        || *((*h).mb.type_0).offset((*h).mb.i_mb_top_xy as isize) as libc::c_int
                            == I_8x8 as libc::c_int
                        || *((*h).mb.type_0).offset((*h).mb.i_mb_top_xy as isize) as libc::c_int
                            == I_16x16 as libc::c_int
                        || *((*h).mb.type_0).offset((*h).mb.i_mb_top_xy as isize) as libc::c_int
                            == I_PCM as libc::c_int) as libc::c_int;
                let mut intra_deblock_0: libc::c_int =
                    (intra_cur != 0 || intra_top_0 != 0) as libc::c_int;
                if !((*(*h).fdec).mb_info).is_null()
                    && (*(((*bs.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize])
                        .as_mut_ptr() as *mut x264_union32_t))
                        .i
                        != 0
                {
                    let fresh2 = &mut (*((*(*h).fdec).effective_qp).offset(mb_xy as isize));
                    *fresh2 = (*fresh2 as libc::c_int
                        | (0xff as libc::c_int
                            * (*((*(*h).fdec).mb_info).offset(mb_xy as isize) as libc::c_uint
                                & ((1 as libc::c_uint) << 0 as libc::c_int)
                                != 0) as libc::c_int)) as uint8_t;
                    let fresh3 =
                        &mut (*((*(*h).fdec).effective_qp).offset((*h).mb.i_mb_top_xy as isize));
                    *fresh3 = (*fresh3 as libc::c_int
                        | (0xff as libc::c_int
                            * (*((*(*h).fdec).mb_info).offset((*h).mb.i_mb_top_xy as isize)
                                as libc::c_uint
                                & ((1 as libc::c_uint) << 0 as libc::c_int)
                                != 0) as libc::c_int)) as uint8_t;
                }
                if (b_interlaced == 0
                    || (*h).mb.b_interlaced == 0
                        && *((*h).mb.field).offset((*h).mb.i_mb_top_xy as isize) == 0)
                    && intra_deblock_0 != 0
                {
                    if 0 as libc::c_int & 1 as libc::c_int == 0 || transform_8x8 == 0 {
                        deblock_edge_intra(
                            h,
                            pixy.offset(
                                (4 as libc::c_int
                                    * 0 as libc::c_int
                                    * (if 1 as libc::c_int != 0 {
                                        stride2y
                                    } else {
                                        1 as libc::c_int
                                    })) as isize,
                            ),
                            stride2y as intptr_t,
                            ((*bs.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize])
                                .as_mut_ptr(),
                            qp_top_0,
                            a,
                            b,
                            0 as libc::c_int,
                            (*h).loopf.deblock_luma_intra[1 as libc::c_int as usize],
                        );
                        if chroma_format == CHROMA_444 as libc::c_int {
                            deblock_edge_intra(
                                h,
                                pixuv.offset(
                                    (4 as libc::c_int
                                        * 0 as libc::c_int
                                        * (if 1 as libc::c_int != 0 {
                                            stride2uv
                                        } else {
                                            1 as libc::c_int
                                        })) as isize,
                                ),
                                stride2uv as intptr_t,
                                ((*bs.offset(1 as libc::c_int as isize))
                                    [0 as libc::c_int as usize])
                                    .as_mut_ptr(),
                                qpc_top_0,
                                a,
                                b,
                                0 as libc::c_int,
                                (*h).loopf.deblock_luma_intra[1 as libc::c_int as usize],
                            );
                            deblock_edge_intra(
                                h,
                                pixuv.offset(uvdiff as isize).offset(
                                    (4 as libc::c_int
                                        * 0 as libc::c_int
                                        * (if 1 as libc::c_int != 0 {
                                            stride2uv
                                        } else {
                                            1 as libc::c_int
                                        })) as isize,
                                ),
                                stride2uv as intptr_t,
                                ((*bs.offset(1 as libc::c_int as isize))
                                    [0 as libc::c_int as usize])
                                    .as_mut_ptr(),
                                qpc_top_0,
                                a,
                                b,
                                0 as libc::c_int,
                                (*h).loopf.deblock_luma_intra[1 as libc::c_int as usize],
                            );
                        } else if chroma_format == CHROMA_420 as libc::c_int
                            && 0 as libc::c_int & 1 as libc::c_int == 0
                        {
                            deblock_edge_intra(
                                h,
                                pixuv.offset(
                                    (0 as libc::c_int
                                        * (if 1 as libc::c_int != 0 {
                                            2 as libc::c_int * stride2uv
                                        } else {
                                            4 as libc::c_int
                                        })) as isize,
                                ),
                                stride2uv as intptr_t,
                                ((*bs.offset(1 as libc::c_int as isize))
                                    [0 as libc::c_int as usize])
                                    .as_mut_ptr(),
                                qpc_top_0,
                                a,
                                b,
                                1 as libc::c_int,
                                (*h).loopf.deblock_chroma_intra[1 as libc::c_int as usize],
                            );
                        }
                    }
                    if chroma_format == CHROMA_422 as libc::c_int
                        && (1 as libc::c_int != 0 || 0 as libc::c_int & 1 as libc::c_int == 0)
                    {
                        deblock_edge_intra(
                            h,
                            pixuv.offset(
                                (0 as libc::c_int
                                    * (if 1 as libc::c_int != 0 {
                                        4 as libc::c_int * stride2uv
                                    } else {
                                        4 as libc::c_int
                                    })) as isize,
                            ),
                            stride2uv as intptr_t,
                            ((*bs.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize])
                                .as_mut_ptr(),
                            qpc_top_0,
                            a,
                            b,
                            1 as libc::c_int,
                            (*h).loopf.deblock_chroma_intra[1 as libc::c_int as usize],
                        );
                    }
                } else {
                    if intra_deblock_0 != 0 {
                        (*(((*bs.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize])
                            .as_mut_ptr() as *mut x264_union32_t))
                            .i = 0x3030303 as libc::c_int as uint32_t;
                    }
                    if 0 as libc::c_int & 1 as libc::c_int == 0 || transform_8x8 == 0 {
                        deblock_edge(
                            h,
                            pixy.offset(
                                (4 as libc::c_int
                                    * 0 as libc::c_int
                                    * (if 1 as libc::c_int != 0 {
                                        stride2y
                                    } else {
                                        1 as libc::c_int
                                    })) as isize,
                            ),
                            stride2y as intptr_t,
                            ((*bs.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize])
                                .as_mut_ptr(),
                            qp_top_0,
                            a,
                            b,
                            0 as libc::c_int,
                            (*h).loopf.deblock_luma[1 as libc::c_int as usize],
                        );
                        if chroma_format == CHROMA_444 as libc::c_int {
                            deblock_edge(
                                h,
                                pixuv.offset(
                                    (4 as libc::c_int
                                        * 0 as libc::c_int
                                        * (if 1 as libc::c_int != 0 {
                                            stride2uv
                                        } else {
                                            1 as libc::c_int
                                        })) as isize,
                                ),
                                stride2uv as intptr_t,
                                ((*bs.offset(1 as libc::c_int as isize))
                                    [0 as libc::c_int as usize])
                                    .as_mut_ptr(),
                                qpc_top_0,
                                a,
                                b,
                                0 as libc::c_int,
                                (*h).loopf.deblock_luma[1 as libc::c_int as usize],
                            );
                            deblock_edge(
                                h,
                                pixuv.offset(uvdiff as isize).offset(
                                    (4 as libc::c_int
                                        * 0 as libc::c_int
                                        * (if 1 as libc::c_int != 0 {
                                            stride2uv
                                        } else {
                                            1 as libc::c_int
                                        })) as isize,
                                ),
                                stride2uv as intptr_t,
                                ((*bs.offset(1 as libc::c_int as isize))
                                    [0 as libc::c_int as usize])
                                    .as_mut_ptr(),
                                qpc_top_0,
                                a,
                                b,
                                0 as libc::c_int,
                                (*h).loopf.deblock_luma[1 as libc::c_int as usize],
                            );
                        } else if chroma_format == CHROMA_420 as libc::c_int
                            && 0 as libc::c_int & 1 as libc::c_int == 0
                        {
                            deblock_edge(
                                h,
                                pixuv.offset(
                                    (0 as libc::c_int
                                        * (if 1 as libc::c_int != 0 {
                                            2 as libc::c_int * stride2uv
                                        } else {
                                            4 as libc::c_int
                                        })) as isize,
                                ),
                                stride2uv as intptr_t,
                                ((*bs.offset(1 as libc::c_int as isize))
                                    [0 as libc::c_int as usize])
                                    .as_mut_ptr(),
                                qpc_top_0,
                                a,
                                b,
                                1 as libc::c_int,
                                (*h).loopf.deblock_chroma[1 as libc::c_int as usize],
                            );
                        }
                    }
                    if chroma_format == CHROMA_422 as libc::c_int
                        && (1 as libc::c_int != 0 || 0 as libc::c_int & 1 as libc::c_int == 0)
                    {
                        deblock_edge(
                            h,
                            pixuv.offset(
                                (0 as libc::c_int
                                    * (if 1 as libc::c_int != 0 {
                                        4 as libc::c_int * stride2uv
                                    } else {
                                        4 as libc::c_int
                                    })) as isize,
                            ),
                            stride2uv as intptr_t,
                            ((*bs.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize])
                                .as_mut_ptr(),
                            qpc_top_0,
                            a,
                            b,
                            1 as libc::c_int,
                            (*h).loopf.deblock_chroma[1 as libc::c_int as usize],
                        );
                    }
                }
            }
        }
        if first_edge_only == 0 {
            if 1 as libc::c_int & 1 as libc::c_int == 0 || transform_8x8 == 0 {
                deblock_edge(
                    h,
                    pixy.offset(
                        (4 as libc::c_int
                            * 1 as libc::c_int
                            * (if 1 as libc::c_int != 0 {
                                stride2y
                            } else {
                                1 as libc::c_int
                            })) as isize,
                    ),
                    stride2y as intptr_t,
                    ((*bs.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize])
                        .as_mut_ptr(),
                    qp,
                    a,
                    b,
                    0 as libc::c_int,
                    (*h).loopf.deblock_luma[1 as libc::c_int as usize],
                );
                if chroma_format == CHROMA_444 as libc::c_int {
                    deblock_edge(
                        h,
                        pixuv.offset(
                            (4 as libc::c_int
                                * 1 as libc::c_int
                                * (if 1 as libc::c_int != 0 {
                                    stride2uv
                                } else {
                                    1 as libc::c_int
                                })) as isize,
                        ),
                        stride2uv as intptr_t,
                        ((*bs.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize])
                            .as_mut_ptr(),
                        qpc,
                        a,
                        b,
                        0 as libc::c_int,
                        (*h).loopf.deblock_luma[1 as libc::c_int as usize],
                    );
                    deblock_edge(
                        h,
                        pixuv.offset(uvdiff as isize).offset(
                            (4 as libc::c_int
                                * 1 as libc::c_int
                                * (if 1 as libc::c_int != 0 {
                                    stride2uv
                                } else {
                                    1 as libc::c_int
                                })) as isize,
                        ),
                        stride2uv as intptr_t,
                        ((*bs.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize])
                            .as_mut_ptr(),
                        qpc,
                        a,
                        b,
                        0 as libc::c_int,
                        (*h).loopf.deblock_luma[1 as libc::c_int as usize],
                    );
                } else if chroma_format == CHROMA_420 as libc::c_int
                    && 1 as libc::c_int & 1 as libc::c_int == 0
                {
                    deblock_edge(
                        h,
                        pixuv.offset(
                            (1 as libc::c_int
                                * (if 1 as libc::c_int != 0 {
                                    2 as libc::c_int * stride2uv
                                } else {
                                    4 as libc::c_int
                                })) as isize,
                        ),
                        stride2uv as intptr_t,
                        ((*bs.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize])
                            .as_mut_ptr(),
                        qpc,
                        a,
                        b,
                        1 as libc::c_int,
                        (*h).loopf.deblock_chroma[1 as libc::c_int as usize],
                    );
                }
            }
            if chroma_format == CHROMA_422 as libc::c_int
                && (1 as libc::c_int != 0 || 1 as libc::c_int & 1 as libc::c_int == 0)
            {
                deblock_edge(
                    h,
                    pixuv.offset(
                        (1 as libc::c_int
                            * (if 1 as libc::c_int != 0 {
                                4 as libc::c_int * stride2uv
                            } else {
                                4 as libc::c_int
                            })) as isize,
                    ),
                    stride2uv as intptr_t,
                    ((*bs.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize])
                        .as_mut_ptr(),
                    qpc,
                    a,
                    b,
                    1 as libc::c_int,
                    (*h).loopf.deblock_chroma[1 as libc::c_int as usize],
                );
            }
            if 2 as libc::c_int & 1 as libc::c_int == 0 || transform_8x8 == 0 {
                deblock_edge(
                    h,
                    pixy.offset(
                        (4 as libc::c_int
                            * 2 as libc::c_int
                            * (if 1 as libc::c_int != 0 {
                                stride2y
                            } else {
                                1 as libc::c_int
                            })) as isize,
                    ),
                    stride2y as intptr_t,
                    ((*bs.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize])
                        .as_mut_ptr(),
                    qp,
                    a,
                    b,
                    0 as libc::c_int,
                    (*h).loopf.deblock_luma[1 as libc::c_int as usize],
                );
                if chroma_format == CHROMA_444 as libc::c_int {
                    deblock_edge(
                        h,
                        pixuv.offset(
                            (4 as libc::c_int
                                * 2 as libc::c_int
                                * (if 1 as libc::c_int != 0 {
                                    stride2uv
                                } else {
                                    1 as libc::c_int
                                })) as isize,
                        ),
                        stride2uv as intptr_t,
                        ((*bs.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize])
                            .as_mut_ptr(),
                        qpc,
                        a,
                        b,
                        0 as libc::c_int,
                        (*h).loopf.deblock_luma[1 as libc::c_int as usize],
                    );
                    deblock_edge(
                        h,
                        pixuv.offset(uvdiff as isize).offset(
                            (4 as libc::c_int
                                * 2 as libc::c_int
                                * (if 1 as libc::c_int != 0 {
                                    stride2uv
                                } else {
                                    1 as libc::c_int
                                })) as isize,
                        ),
                        stride2uv as intptr_t,
                        ((*bs.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize])
                            .as_mut_ptr(),
                        qpc,
                        a,
                        b,
                        0 as libc::c_int,
                        (*h).loopf.deblock_luma[1 as libc::c_int as usize],
                    );
                } else if chroma_format == CHROMA_420 as libc::c_int
                    && 2 as libc::c_int & 1 as libc::c_int == 0
                {
                    deblock_edge(
                        h,
                        pixuv.offset(
                            (2 as libc::c_int
                                * (if 1 as libc::c_int != 0 {
                                    2 as libc::c_int * stride2uv
                                } else {
                                    4 as libc::c_int
                                })) as isize,
                        ),
                        stride2uv as intptr_t,
                        ((*bs.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize])
                            .as_mut_ptr(),
                        qpc,
                        a,
                        b,
                        1 as libc::c_int,
                        (*h).loopf.deblock_chroma[1 as libc::c_int as usize],
                    );
                }
            }
            if chroma_format == CHROMA_422 as libc::c_int
                && (1 as libc::c_int != 0 || 2 as libc::c_int & 1 as libc::c_int == 0)
            {
                deblock_edge(
                    h,
                    pixuv.offset(
                        (2 as libc::c_int
                            * (if 1 as libc::c_int != 0 {
                                4 as libc::c_int * stride2uv
                            } else {
                                4 as libc::c_int
                            })) as isize,
                    ),
                    stride2uv as intptr_t,
                    ((*bs.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize])
                        .as_mut_ptr(),
                    qpc,
                    a,
                    b,
                    1 as libc::c_int,
                    (*h).loopf.deblock_chroma[1 as libc::c_int as usize],
                );
            }
            if 3 as libc::c_int & 1 as libc::c_int == 0 || transform_8x8 == 0 {
                deblock_edge(
                    h,
                    pixy.offset(
                        (4 as libc::c_int
                            * 3 as libc::c_int
                            * (if 1 as libc::c_int != 0 {
                                stride2y
                            } else {
                                1 as libc::c_int
                            })) as isize,
                    ),
                    stride2y as intptr_t,
                    ((*bs.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize])
                        .as_mut_ptr(),
                    qp,
                    a,
                    b,
                    0 as libc::c_int,
                    (*h).loopf.deblock_luma[1 as libc::c_int as usize],
                );
                if chroma_format == CHROMA_444 as libc::c_int {
                    deblock_edge(
                        h,
                        pixuv.offset(
                            (4 as libc::c_int
                                * 3 as libc::c_int
                                * (if 1 as libc::c_int != 0 {
                                    stride2uv
                                } else {
                                    1 as libc::c_int
                                })) as isize,
                        ),
                        stride2uv as intptr_t,
                        ((*bs.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize])
                            .as_mut_ptr(),
                        qpc,
                        a,
                        b,
                        0 as libc::c_int,
                        (*h).loopf.deblock_luma[1 as libc::c_int as usize],
                    );
                    deblock_edge(
                        h,
                        pixuv.offset(uvdiff as isize).offset(
                            (4 as libc::c_int
                                * 3 as libc::c_int
                                * (if 1 as libc::c_int != 0 {
                                    stride2uv
                                } else {
                                    1 as libc::c_int
                                })) as isize,
                        ),
                        stride2uv as intptr_t,
                        ((*bs.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize])
                            .as_mut_ptr(),
                        qpc,
                        a,
                        b,
                        0 as libc::c_int,
                        (*h).loopf.deblock_luma[1 as libc::c_int as usize],
                    );
                } else if chroma_format == CHROMA_420 as libc::c_int
                    && 3 as libc::c_int & 1 as libc::c_int == 0
                {
                    deblock_edge(
                        h,
                        pixuv.offset(
                            (3 as libc::c_int
                                * (if 1 as libc::c_int != 0 {
                                    2 as libc::c_int * stride2uv
                                } else {
                                    4 as libc::c_int
                                })) as isize,
                        ),
                        stride2uv as intptr_t,
                        ((*bs.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize])
                            .as_mut_ptr(),
                        qpc,
                        a,
                        b,
                        1 as libc::c_int,
                        (*h).loopf.deblock_chroma[1 as libc::c_int as usize],
                    );
                }
            }
            if chroma_format == CHROMA_422 as libc::c_int
                && (1 as libc::c_int != 0 || 3 as libc::c_int & 1 as libc::c_int == 0)
            {
                deblock_edge(
                    h,
                    pixuv.offset(
                        (3 as libc::c_int
                            * (if 1 as libc::c_int != 0 {
                                4 as libc::c_int * stride2uv
                            } else {
                                4 as libc::c_int
                            })) as isize,
                    ),
                    stride2uv as intptr_t,
                    ((*bs.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize])
                        .as_mut_ptr(),
                    qpc,
                    a,
                    b,
                    1 as libc::c_int,
                    (*h).loopf.deblock_chroma[1 as libc::c_int as usize],
                );
            }
        }
        mb_x += (!b_interlaced | mb_y) & 1 as libc::c_int;
        mb_y ^= b_interlaced;
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_macroblock_deblock(mut h: *mut x264_t) {
    let mut a: libc::c_int =
        (*h).sh.i_alpha_c0_offset - 6 as libc::c_int * (8 as libc::c_int - 8 as libc::c_int);
    let mut b: libc::c_int =
        (*h).sh.i_beta_offset - 6 as libc::c_int * (8 as libc::c_int - 8 as libc::c_int);
    let mut qp_thresh: libc::c_int = 15 as libc::c_int
        - (if a < b { a } else { b })
        - (if 0 as libc::c_int > (*((*h).pps).as_mut_ptr()).i_chroma_qp_index_offset {
            0 as libc::c_int
        } else {
            (*((*h).pps).as_mut_ptr()).i_chroma_qp_index_offset
        });
    let mut intra_cur: libc::c_int = ((*h).mb.i_type == I_4x4 as libc::c_int
        || (*h).mb.i_type == I_8x8 as libc::c_int
        || (*h).mb.i_type == I_16x16 as libc::c_int
        || (*h).mb.i_type == I_PCM as libc::c_int)
        as libc::c_int;
    let mut qp: libc::c_int = (*h).mb.i_qp;
    let mut qpc: libc::c_int = (*h).mb.i_chroma_qp;
    if (*h).mb.i_partition == D_16x16 as libc::c_int && (*h).mb.i_cbp_luma == 0 && intra_cur == 0
        || qp <= qp_thresh
    {
        return;
    }
    let mut bs: *mut [[uint8_t; 4]; 8] = (*h).mb.cache.deblock_strength;
    if intra_cur != 0 {
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
    } else {
        ((*h).loopf.deblock_strength).expect("non-null function pointer")(
            ((*h).mb.cache.non_zero_count).as_mut_ptr(),
            ((*h).mb.cache.ref_0).as_mut_ptr(),
            ((*h).mb.cache.mv).as_mut_ptr(),
            bs,
            4 as libc::c_int >> (*h).mb.b_interlaced,
            ((*h).sh.i_type == SLICE_TYPE_B as libc::c_int) as libc::c_int,
        );
    }
    let mut transform_8x8: libc::c_int = (*h).mb.b_transform_8x8;
    if transform_8x8 == 0 {
        deblock_edge(
            h,
            ((*h).mb.pic.p_fdec[0 as libc::c_int as usize]).offset(
                (4 as libc::c_int
                    * 1 as libc::c_int
                    * (if 0 as libc::c_int != 0 {
                        32 as libc::c_int
                    } else {
                        1 as libc::c_int
                    })) as isize,
            ),
            32 as libc::c_int as intptr_t,
            ((*bs.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize]).as_mut_ptr(),
            qp,
            a,
            b,
            0 as libc::c_int,
            (*h).loopf.deblock_luma[0 as libc::c_int as usize],
        );
        if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as libc::c_int {
            deblock_edge(
                h,
                ((*h).mb.pic.p_fdec[1 as libc::c_int as usize]).offset(
                    (4 as libc::c_int
                        * 1 as libc::c_int
                        * (if 0 as libc::c_int != 0 {
                            32 as libc::c_int
                        } else {
                            1 as libc::c_int
                        })) as isize,
                ),
                32 as libc::c_int as intptr_t,
                ((*bs.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize]).as_mut_ptr(),
                qpc,
                a,
                b,
                0 as libc::c_int,
                (*h).loopf.deblock_luma[0 as libc::c_int as usize],
            );
            deblock_edge(
                h,
                ((*h).mb.pic.p_fdec[2 as libc::c_int as usize]).offset(
                    (4 as libc::c_int
                        * 1 as libc::c_int
                        * (if 0 as libc::c_int != 0 {
                            32 as libc::c_int
                        } else {
                            1 as libc::c_int
                        })) as isize,
                ),
                32 as libc::c_int as intptr_t,
                ((*bs.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize]).as_mut_ptr(),
                qpc,
                a,
                b,
                0 as libc::c_int,
                (*h).loopf.deblock_luma[0 as libc::c_int as usize],
            );
        }
    }
    deblock_edge(
        h,
        ((*h).mb.pic.p_fdec[0 as libc::c_int as usize]).offset(
            (4 as libc::c_int
                * 2 as libc::c_int
                * (if 0 as libc::c_int != 0 {
                    32 as libc::c_int
                } else {
                    1 as libc::c_int
                })) as isize,
        ),
        32 as libc::c_int as intptr_t,
        ((*bs.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize]).as_mut_ptr(),
        qp,
        a,
        b,
        0 as libc::c_int,
        (*h).loopf.deblock_luma[0 as libc::c_int as usize],
    );
    if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as libc::c_int {
        deblock_edge(
            h,
            ((*h).mb.pic.p_fdec[1 as libc::c_int as usize]).offset(
                (4 as libc::c_int
                    * 2 as libc::c_int
                    * (if 0 as libc::c_int != 0 {
                        32 as libc::c_int
                    } else {
                        1 as libc::c_int
                    })) as isize,
            ),
            32 as libc::c_int as intptr_t,
            ((*bs.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize]).as_mut_ptr(),
            qpc,
            a,
            b,
            0 as libc::c_int,
            (*h).loopf.deblock_luma[0 as libc::c_int as usize],
        );
        deblock_edge(
            h,
            ((*h).mb.pic.p_fdec[2 as libc::c_int as usize]).offset(
                (4 as libc::c_int
                    * 2 as libc::c_int
                    * (if 0 as libc::c_int != 0 {
                        32 as libc::c_int
                    } else {
                        1 as libc::c_int
                    })) as isize,
            ),
            32 as libc::c_int as intptr_t,
            ((*bs.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize]).as_mut_ptr(),
            qpc,
            a,
            b,
            0 as libc::c_int,
            (*h).loopf.deblock_luma[0 as libc::c_int as usize],
        );
    }
    if transform_8x8 == 0 {
        deblock_edge(
            h,
            ((*h).mb.pic.p_fdec[0 as libc::c_int as usize]).offset(
                (4 as libc::c_int
                    * 3 as libc::c_int
                    * (if 0 as libc::c_int != 0 {
                        32 as libc::c_int
                    } else {
                        1 as libc::c_int
                    })) as isize,
            ),
            32 as libc::c_int as intptr_t,
            ((*bs.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize]).as_mut_ptr(),
            qp,
            a,
            b,
            0 as libc::c_int,
            (*h).loopf.deblock_luma[0 as libc::c_int as usize],
        );
        if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as libc::c_int {
            deblock_edge(
                h,
                ((*h).mb.pic.p_fdec[1 as libc::c_int as usize]).offset(
                    (4 as libc::c_int
                        * 3 as libc::c_int
                        * (if 0 as libc::c_int != 0 {
                            32 as libc::c_int
                        } else {
                            1 as libc::c_int
                        })) as isize,
                ),
                32 as libc::c_int as intptr_t,
                ((*bs.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize]).as_mut_ptr(),
                qpc,
                a,
                b,
                0 as libc::c_int,
                (*h).loopf.deblock_luma[0 as libc::c_int as usize],
            );
            deblock_edge(
                h,
                ((*h).mb.pic.p_fdec[2 as libc::c_int as usize]).offset(
                    (4 as libc::c_int
                        * 3 as libc::c_int
                        * (if 0 as libc::c_int != 0 {
                            32 as libc::c_int
                        } else {
                            1 as libc::c_int
                        })) as isize,
                ),
                32 as libc::c_int as intptr_t,
                ((*bs.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize]).as_mut_ptr(),
                qpc,
                a,
                b,
                0 as libc::c_int,
                (*h).loopf.deblock_luma[0 as libc::c_int as usize],
            );
        }
    }
    if transform_8x8 == 0 {
        deblock_edge(
            h,
            ((*h).mb.pic.p_fdec[0 as libc::c_int as usize]).offset(
                (4 as libc::c_int
                    * 1 as libc::c_int
                    * (if 1 as libc::c_int != 0 {
                        32 as libc::c_int
                    } else {
                        1 as libc::c_int
                    })) as isize,
            ),
            32 as libc::c_int as intptr_t,
            ((*bs.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize]).as_mut_ptr(),
            qp,
            a,
            b,
            0 as libc::c_int,
            (*h).loopf.deblock_luma[1 as libc::c_int as usize],
        );
        if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as libc::c_int {
            deblock_edge(
                h,
                ((*h).mb.pic.p_fdec[1 as libc::c_int as usize]).offset(
                    (4 as libc::c_int
                        * 1 as libc::c_int
                        * (if 1 as libc::c_int != 0 {
                            32 as libc::c_int
                        } else {
                            1 as libc::c_int
                        })) as isize,
                ),
                32 as libc::c_int as intptr_t,
                ((*bs.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize]).as_mut_ptr(),
                qpc,
                a,
                b,
                0 as libc::c_int,
                (*h).loopf.deblock_luma[1 as libc::c_int as usize],
            );
            deblock_edge(
                h,
                ((*h).mb.pic.p_fdec[2 as libc::c_int as usize]).offset(
                    (4 as libc::c_int
                        * 1 as libc::c_int
                        * (if 1 as libc::c_int != 0 {
                            32 as libc::c_int
                        } else {
                            1 as libc::c_int
                        })) as isize,
                ),
                32 as libc::c_int as intptr_t,
                ((*bs.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize]).as_mut_ptr(),
                qpc,
                a,
                b,
                0 as libc::c_int,
                (*h).loopf.deblock_luma[1 as libc::c_int as usize],
            );
        }
    }
    deblock_edge(
        h,
        ((*h).mb.pic.p_fdec[0 as libc::c_int as usize]).offset(
            (4 as libc::c_int
                * 2 as libc::c_int
                * (if 1 as libc::c_int != 0 {
                    32 as libc::c_int
                } else {
                    1 as libc::c_int
                })) as isize,
        ),
        32 as libc::c_int as intptr_t,
        ((*bs.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize]).as_mut_ptr(),
        qp,
        a,
        b,
        0 as libc::c_int,
        (*h).loopf.deblock_luma[1 as libc::c_int as usize],
    );
    if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as libc::c_int {
        deblock_edge(
            h,
            ((*h).mb.pic.p_fdec[1 as libc::c_int as usize]).offset(
                (4 as libc::c_int
                    * 2 as libc::c_int
                    * (if 1 as libc::c_int != 0 {
                        32 as libc::c_int
                    } else {
                        1 as libc::c_int
                    })) as isize,
            ),
            32 as libc::c_int as intptr_t,
            ((*bs.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize]).as_mut_ptr(),
            qpc,
            a,
            b,
            0 as libc::c_int,
            (*h).loopf.deblock_luma[1 as libc::c_int as usize],
        );
        deblock_edge(
            h,
            ((*h).mb.pic.p_fdec[2 as libc::c_int as usize]).offset(
                (4 as libc::c_int
                    * 2 as libc::c_int
                    * (if 1 as libc::c_int != 0 {
                        32 as libc::c_int
                    } else {
                        1 as libc::c_int
                    })) as isize,
            ),
            32 as libc::c_int as intptr_t,
            ((*bs.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize]).as_mut_ptr(),
            qpc,
            a,
            b,
            0 as libc::c_int,
            (*h).loopf.deblock_luma[1 as libc::c_int as usize],
        );
    }
    if transform_8x8 == 0 {
        deblock_edge(
            h,
            ((*h).mb.pic.p_fdec[0 as libc::c_int as usize]).offset(
                (4 as libc::c_int
                    * 3 as libc::c_int
                    * (if 1 as libc::c_int != 0 {
                        32 as libc::c_int
                    } else {
                        1 as libc::c_int
                    })) as isize,
            ),
            32 as libc::c_int as intptr_t,
            ((*bs.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize]).as_mut_ptr(),
            qp,
            a,
            b,
            0 as libc::c_int,
            (*h).loopf.deblock_luma[1 as libc::c_int as usize],
        );
        if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as libc::c_int {
            deblock_edge(
                h,
                ((*h).mb.pic.p_fdec[1 as libc::c_int as usize]).offset(
                    (4 as libc::c_int
                        * 3 as libc::c_int
                        * (if 1 as libc::c_int != 0 {
                            32 as libc::c_int
                        } else {
                            1 as libc::c_int
                        })) as isize,
                ),
                32 as libc::c_int as intptr_t,
                ((*bs.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize]).as_mut_ptr(),
                qpc,
                a,
                b,
                0 as libc::c_int,
                (*h).loopf.deblock_luma[1 as libc::c_int as usize],
            );
            deblock_edge(
                h,
                ((*h).mb.pic.p_fdec[2 as libc::c_int as usize]).offset(
                    (4 as libc::c_int
                        * 3 as libc::c_int
                        * (if 1 as libc::c_int != 0 {
                            32 as libc::c_int
                        } else {
                            1 as libc::c_int
                        })) as isize,
                ),
                32 as libc::c_int as intptr_t,
                ((*bs.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize]).as_mut_ptr(),
                qpc,
                a,
                b,
                0 as libc::c_int,
                (*h).loopf.deblock_luma[1 as libc::c_int as usize],
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_deblock_init(
    mut cpu: uint32_t,
    mut pf: *mut x264_deblock_function_t,
    mut b_mbaff: libc::c_int,
) {
    (*pf).deblock_luma[1 as libc::c_int as usize] = Some(
        deblock_v_luma_c
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                libc::c_int,
                libc::c_int,
                *mut int8_t,
            ) -> (),
    );
    (*pf).deblock_luma[0 as libc::c_int as usize] = Some(
        deblock_h_luma_c
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                libc::c_int,
                libc::c_int,
                *mut int8_t,
            ) -> (),
    );
    (*pf).deblock_chroma[1 as libc::c_int as usize] = Some(
        deblock_v_chroma_c
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                libc::c_int,
                libc::c_int,
                *mut int8_t,
            ) -> (),
    );
    (*pf).deblock_h_chroma_420 = Some(
        deblock_h_chroma_c
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                libc::c_int,
                libc::c_int,
                *mut int8_t,
            ) -> (),
    );
    (*pf).deblock_h_chroma_422 = Some(
        deblock_h_chroma_422_c
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                libc::c_int,
                libc::c_int,
                *mut int8_t,
            ) -> (),
    );
    (*pf).deblock_luma_intra[1 as libc::c_int as usize] = Some(
        deblock_v_luma_intra_c
            as unsafe extern "C" fn(*mut pixel, intptr_t, libc::c_int, libc::c_int) -> (),
    );
    (*pf).deblock_luma_intra[0 as libc::c_int as usize] = Some(
        deblock_h_luma_intra_c
            as unsafe extern "C" fn(*mut pixel, intptr_t, libc::c_int, libc::c_int) -> (),
    );
    (*pf).deblock_chroma_intra[1 as libc::c_int as usize] = Some(
        deblock_v_chroma_intra_c
            as unsafe extern "C" fn(*mut pixel, intptr_t, libc::c_int, libc::c_int) -> (),
    );
    (*pf).deblock_h_chroma_420_intra = Some(
        deblock_h_chroma_intra_c
            as unsafe extern "C" fn(*mut pixel, intptr_t, libc::c_int, libc::c_int) -> (),
    );
    (*pf).deblock_h_chroma_422_intra = Some(
        deblock_h_chroma_422_intra_c
            as unsafe extern "C" fn(*mut pixel, intptr_t, libc::c_int, libc::c_int) -> (),
    );
    (*pf).deblock_luma_mbaff = Some(
        deblock_h_luma_mbaff_c
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                libc::c_int,
                libc::c_int,
                *mut int8_t,
            ) -> (),
    );
    (*pf).deblock_chroma_420_mbaff = Some(
        deblock_h_chroma_mbaff_c
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                libc::c_int,
                libc::c_int,
                *mut int8_t,
            ) -> (),
    );
    (*pf).deblock_luma_intra_mbaff = Some(
        deblock_h_luma_intra_mbaff_c
            as unsafe extern "C" fn(*mut pixel, intptr_t, libc::c_int, libc::c_int) -> (),
    );
    (*pf).deblock_chroma_420_intra_mbaff = Some(
        deblock_h_chroma_intra_mbaff_c
            as unsafe extern "C" fn(*mut pixel, intptr_t, libc::c_int, libc::c_int) -> (),
    );
    (*pf).deblock_strength = Some(
        deblock_strength_c
            as unsafe extern "C" fn(
                *mut uint8_t,
                *mut [int8_t; 40],
                *mut [[int16_t; 2]; 40],
                *mut [[uint8_t; 4]; 8],
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*pf).deblock_chroma_422_mbaff = (*pf).deblock_h_chroma_420;
    (*pf).deblock_chroma_422_intra_mbaff = (*pf).deblock_h_chroma_420_intra;
}
