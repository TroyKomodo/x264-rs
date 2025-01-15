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
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn x264_8_copy_column8(dst: *mut pixel, src: *mut pixel);
    fn x264_8_mb_mc(h: *mut x264_t);
    fn x264_8_mb_mc_8x8(h: *mut x264_t, i8: libc::c_int);
    fn x264_8_quant_luma_dc_trellis(
        h: *mut x264_t,
        dct: *mut dctcoef,
        i_quant_cat: libc::c_int,
        i_qp: libc::c_int,
        ctx_block_cat: libc::c_int,
        b_intra: libc::c_int,
        idx: libc::c_int,
    ) -> libc::c_int;
    fn x264_8_quant_chroma_dc_trellis(
        h: *mut x264_t,
        dct: *mut dctcoef,
        i_qp: libc::c_int,
        b_intra: libc::c_int,
        idx: libc::c_int,
    ) -> libc::c_int;
    fn x264_8_quant_4x4_trellis(
        h: *mut x264_t,
        dct: *mut dctcoef,
        i_quant_cat: libc::c_int,
        i_qp: libc::c_int,
        ctx_block_cat: libc::c_int,
        b_intra: libc::c_int,
        b_chroma: libc::c_int,
        idx: libc::c_int,
    ) -> libc::c_int;
    fn x264_8_quant_8x8_trellis(
        h: *mut x264_t,
        dct: *mut dctcoef,
        i_quant_cat: libc::c_int,
        i_qp: libc::c_int,
        ctx_block_cat: libc::c_int,
        b_intra: libc::c_int,
        b_chroma: libc::c_int,
        idx: libc::c_int,
    ) -> libc::c_int;
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
unsafe extern "C" fn x264_quant_4x4(
    mut h: *mut x264_t,
    mut dct: *mut dctcoef,
    mut i_qp: libc::c_int,
    mut ctx_block_cat: libc::c_int,
    mut b_intra: libc::c_int,
    mut p: libc::c_int,
    mut idx: libc::c_int,
) -> libc::c_int {
    let mut i_quant_cat: libc::c_int = if b_intra != 0 {
        if p != 0 {
            CQM_4IC as libc::c_int
        } else {
            CQM_4IY as libc::c_int
        }
    } else if p != 0 {
        CQM_4PC as libc::c_int
    } else {
        CQM_4PY as libc::c_int
    };
    if (*h).mb.b_noise_reduction != 0 {
        ((*h).quantf.denoise_dct).expect("non-null function pointer")(
            dct,
            (*((*h).nr_residual_sum)
                .offset((0 as libc::c_int + (p != 0) as libc::c_int * 2 as libc::c_int) as isize))
            .as_mut_ptr(),
            (*((*h).nr_offset)
                .offset((0 as libc::c_int + (p != 0) as libc::c_int * 2 as libc::c_int) as isize))
            .as_mut_ptr(),
            16 as libc::c_int,
        );
    }
    if (*h).mb.b_trellis != 0 {
        x264_8_quant_4x4_trellis(
            h,
            dct,
            i_quant_cat,
            i_qp,
            ctx_block_cat,
            b_intra,
            (p != 0) as libc::c_int,
            idx + p * 16 as libc::c_int,
        )
    } else {
        ((*h).quantf.quant_4x4).expect("non-null function pointer")(
            dct,
            (*((*h).quant4_mf[i_quant_cat as usize]).offset(i_qp as isize)).as_mut_ptr(),
            (*((*h).quant4_bias[i_quant_cat as usize]).offset(i_qp as isize)).as_mut_ptr(),
        )
    }
}
#[inline(always)]
unsafe extern "C" fn x264_quant_8x8(
    mut h: *mut x264_t,
    mut dct: *mut dctcoef,
    mut i_qp: libc::c_int,
    mut ctx_block_cat: libc::c_int,
    mut b_intra: libc::c_int,
    mut p: libc::c_int,
    mut idx: libc::c_int,
) -> libc::c_int {
    let mut i_quant_cat: libc::c_int = if b_intra != 0 {
        if p != 0 {
            CQM_8IC as libc::c_int
        } else {
            CQM_8IY as libc::c_int
        }
    } else if p != 0 {
        CQM_8PC as libc::c_int
    } else {
        CQM_8PY as libc::c_int
    };
    if (*h).mb.b_noise_reduction != 0 {
        ((*h).quantf.denoise_dct).expect("non-null function pointer")(
            dct,
            (*((*h).nr_residual_sum)
                .offset((1 as libc::c_int + (p != 0) as libc::c_int * 2 as libc::c_int) as isize))
            .as_mut_ptr(),
            (*((*h).nr_offset)
                .offset((1 as libc::c_int + (p != 0) as libc::c_int * 2 as libc::c_int) as isize))
            .as_mut_ptr(),
            64 as libc::c_int,
        );
    }
    if (*h).mb.b_trellis != 0 {
        x264_8_quant_8x8_trellis(
            h,
            dct,
            i_quant_cat,
            i_qp,
            ctx_block_cat,
            b_intra,
            (p != 0) as libc::c_int,
            idx + p * 4 as libc::c_int,
        )
    } else {
        ((*h).quantf.quant_8x8).expect("non-null function pointer")(
            dct,
            (*((*h).quant8_mf[i_quant_cat as usize]).offset(i_qp as isize)).as_mut_ptr(),
            (*((*h).quant8_bias[i_quant_cat as usize]).offset(i_qp as isize)).as_mut_ptr(),
        )
    }
}
#[inline(always)]
unsafe extern "C" fn x264_mb_encode_i4x4(
    mut h: *mut x264_t,
    mut p: libc::c_int,
    mut idx: libc::c_int,
    mut i_qp: libc::c_int,
    mut i_mode: libc::c_int,
    mut b_predict: libc::c_int,
) {
    let mut nz: libc::c_int = 0;
    let mut p_src: *mut pixel = &mut *(*((*h).mb.pic.p_fenc).as_mut_ptr().offset(p as isize))
        .offset(*block_idx_xy_fenc.as_ptr().offset(idx as isize) as isize)
        as *mut pixel;
    let mut p_dst: *mut pixel = &mut *(*((*h).mb.pic.p_fdec).as_mut_ptr().offset(p as isize))
        .offset(*block_idx_xy_fdec.as_ptr().offset(idx as isize) as isize)
        as *mut pixel;
    let mut dct4x4: [dctcoef; 16] = [0; 16];
    if b_predict != 0 {
        if (*h).mb.b_lossless != 0 {
            x264_8_predict_lossless_4x4(h, p_dst, p, idx, i_mode);
        } else {
            ((*h).predict_4x4[i_mode as usize]).expect("non-null function pointer")(p_dst);
        }
    }
    if (*h).mb.b_lossless != 0 {
        nz = ((*h).zigzagf.sub_4x4).expect("non-null function pointer")(
            ((*h).dct.luma4x4[(p * 16 as libc::c_int + idx) as usize]).as_mut_ptr(),
            p_src,
            p_dst,
        );
        (*h).mb.cache.non_zero_count[x264_scan8[(p * 16 as libc::c_int + idx) as usize] as usize] =
            nz as uint8_t;
        (*h).mb.i_cbp_luma |= nz << (idx >> 2 as libc::c_int);
        return;
    }
    ((*h).dctf.sub4x4_dct).expect("non-null function pointer")(dct4x4.as_mut_ptr(), p_src, p_dst);
    nz = x264_quant_4x4(
        h,
        dct4x4.as_mut_ptr(),
        i_qp,
        ctx_cat_plane[DCT_LUMA_4x4 as libc::c_int as usize][p as usize] as libc::c_int,
        1 as libc::c_int,
        p,
        idx,
    );
    (*h).mb.cache.non_zero_count[x264_scan8[(p * 16 as libc::c_int + idx) as usize] as usize] =
        nz as uint8_t;
    if nz != 0 {
        (*h).mb.i_cbp_luma |= (1 as libc::c_int) << (idx >> 2 as libc::c_int);
        ((*h).zigzagf.scan_4x4).expect("non-null function pointer")(
            ((*h).dct.luma4x4[(p * 16 as libc::c_int + idx) as usize]).as_mut_ptr(),
            dct4x4.as_mut_ptr(),
        );
        ((*h).quantf.dequant_4x4).expect("non-null function pointer")(
            dct4x4.as_mut_ptr(),
            (*h).dequant4_mf[(if p != 0 {
                CQM_4IC as libc::c_int
            } else {
                CQM_4IY as libc::c_int
            }) as usize],
            i_qp,
        );
        ((*h).dctf.add4x4_idct).expect("non-null function pointer")(p_dst, dct4x4.as_mut_ptr());
    }
}
#[inline(always)]
unsafe extern "C" fn x264_mb_encode_i8x8(
    mut h: *mut x264_t,
    mut p: libc::c_int,
    mut idx: libc::c_int,
    mut i_qp: libc::c_int,
    mut i_mode: libc::c_int,
    mut edge: *mut pixel,
    mut b_predict: libc::c_int,
) {
    let mut x: libc::c_int = idx & 1 as libc::c_int;
    let mut y: libc::c_int = idx >> 1 as libc::c_int;
    let mut nz: libc::c_int = 0;
    let mut p_src: *mut pixel = &mut *(*((*h).mb.pic.p_fenc).as_mut_ptr().offset(p as isize))
        .offset((8 as libc::c_int * x + 8 as libc::c_int * y * 16 as libc::c_int) as isize)
        as *mut pixel;
    let mut p_dst: *mut pixel = &mut *(*((*h).mb.pic.p_fdec).as_mut_ptr().offset(p as isize))
        .offset((8 as libc::c_int * x + 8 as libc::c_int * y * 32 as libc::c_int) as isize)
        as *mut pixel;
    let mut dct8x8: [dctcoef; 64] = [0; 64];
    let mut edge_buf: [pixel; 36] = [0; 36];
    if b_predict != 0 {
        if edge.is_null() {
            ((*h).predict_8x8_filter).expect("non-null function pointer")(
                p_dst,
                edge_buf.as_mut_ptr(),
                (*h).mb.i_neighbour8[idx as usize] as libc::c_int,
                x264_pred_i4x4_neighbors[i_mode as usize] as libc::c_int,
            );
            edge = edge_buf.as_mut_ptr();
        }
        if (*h).mb.b_lossless != 0 {
            x264_8_predict_lossless_8x8(h, p_dst, p, idx, i_mode, edge);
        } else {
            ((*h).predict_8x8[i_mode as usize]).expect("non-null function pointer")(p_dst, edge);
        }
    }
    if (*h).mb.b_lossless != 0 {
        nz = ((*h).zigzagf.sub_8x8).expect("non-null function pointer")(
            ((*h).dct.luma8x8[(p * 4 as libc::c_int + idx) as usize]).as_mut_ptr(),
            p_src,
            p_dst,
        );
        (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
            (*x264_scan8
                .as_ptr()
                .offset((p * 16 as libc::c_int + idx * 4 as libc::c_int) as isize)
                as libc::c_int
                + 0 as libc::c_int) as isize,
        ) as *mut uint8_t as *mut x264_union16_t))
            .i = (nz * 0x101 as libc::c_int) as uint16_t;
        (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
            (*x264_scan8
                .as_ptr()
                .offset((p * 16 as libc::c_int + idx * 4 as libc::c_int) as isize)
                as libc::c_int
                + 8 as libc::c_int) as isize,
        ) as *mut uint8_t as *mut x264_union16_t))
            .i = (nz * 0x101 as libc::c_int) as uint16_t;
        (*h).mb.i_cbp_luma |= nz << idx;
        return;
    }
    ((*h).dctf.sub8x8_dct8).expect("non-null function pointer")(dct8x8.as_mut_ptr(), p_src, p_dst);
    nz = x264_quant_8x8(
        h,
        dct8x8.as_mut_ptr(),
        i_qp,
        ctx_cat_plane[DCT_LUMA_8x8 as libc::c_int as usize][p as usize] as libc::c_int,
        1 as libc::c_int,
        p,
        idx,
    );
    if nz != 0 {
        (*h).mb.i_cbp_luma |= (1 as libc::c_int) << idx;
        ((*h).zigzagf.scan_8x8).expect("non-null function pointer")(
            ((*h).dct.luma8x8[(p * 4 as libc::c_int + idx) as usize]).as_mut_ptr(),
            dct8x8.as_mut_ptr(),
        );
        ((*h).quantf.dequant_8x8).expect("non-null function pointer")(
            dct8x8.as_mut_ptr(),
            (*h).dequant8_mf[(if p != 0 {
                CQM_8IC as libc::c_int
            } else {
                CQM_8IY as libc::c_int
            }) as usize],
            i_qp,
        );
        ((*h).dctf.add8x8_idct8).expect("non-null function pointer")(p_dst, dct8x8.as_mut_ptr());
        (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
            (*x264_scan8
                .as_ptr()
                .offset((p * 16 as libc::c_int + idx * 4 as libc::c_int) as isize)
                as libc::c_int
                + 0 as libc::c_int) as isize,
        ) as *mut uint8_t as *mut x264_union16_t))
            .i = (1 as libc::c_int * 0x101 as libc::c_int) as uint16_t;
        (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
            (*x264_scan8
                .as_ptr()
                .offset((p * 16 as libc::c_int + idx * 4 as libc::c_int) as isize)
                as libc::c_int
                + 8 as libc::c_int) as isize,
        ) as *mut uint8_t as *mut x264_union16_t))
            .i = (1 as libc::c_int * 0x101 as libc::c_int) as uint16_t;
    } else {
        (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
            (*x264_scan8
                .as_ptr()
                .offset((p * 16 as libc::c_int + idx * 4 as libc::c_int) as isize)
                as libc::c_int
                + 0 as libc::c_int) as isize,
        ) as *mut uint8_t as *mut x264_union16_t))
            .i = (0 as libc::c_int * 0x101 as libc::c_int) as uint16_t;
        (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
            (*x264_scan8
                .as_ptr()
                .offset((p * 16 as libc::c_int + idx * 4 as libc::c_int) as isize)
                as libc::c_int
                + 8 as libc::c_int) as isize,
        ) as *mut uint8_t as *mut x264_union16_t))
            .i = (0 as libc::c_int * 0x101 as libc::c_int) as uint16_t;
    };
}
#[inline]
unsafe extern "C" fn zigzag_scan_2x2_dc(mut level: *mut dctcoef, mut dct: *mut dctcoef) {
    *level.offset(0 as libc::c_int as isize) =
        *dct.offset((0 as libc::c_int * 2 as libc::c_int + 0 as libc::c_int) as isize);
    *level.offset(1 as libc::c_int as isize) =
        *dct.offset((1 as libc::c_int * 2 as libc::c_int + 0 as libc::c_int) as isize);
    *level.offset(2 as libc::c_int as isize) =
        *dct.offset((0 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as isize);
    *level.offset(3 as libc::c_int as isize) =
        *dct.offset((1 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as isize);
}
#[inline]
unsafe extern "C" fn zigzag_scan_2x4_dc(mut level: *mut dctcoef, mut dct: *mut dctcoef) {
    *level.offset(0 as libc::c_int as isize) = *dct.offset(0 as libc::c_int as isize);
    *level.offset(1 as libc::c_int as isize) = *dct.offset(2 as libc::c_int as isize);
    *level.offset(2 as libc::c_int as isize) = *dct.offset(1 as libc::c_int as isize);
    *level.offset(3 as libc::c_int as isize) = *dct.offset(4 as libc::c_int as isize);
    *level.offset(4 as libc::c_int as isize) = *dct.offset(6 as libc::c_int as isize);
    *level.offset(5 as libc::c_int as isize) = *dct.offset(3 as libc::c_int as isize);
    *level.offset(6 as libc::c_int as isize) = *dct.offset(5 as libc::c_int as isize);
    *level.offset(7 as libc::c_int as isize) = *dct.offset(7 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn idct_dequant_2x2_dc(
    mut dct: *mut dctcoef,
    mut dct4x4: *mut [dctcoef; 16],
    mut dequant_mf: *mut [libc::c_int; 16],
    mut i_qp: libc::c_int,
) {
    let mut d0: libc::c_int = *dct.offset(0 as libc::c_int as isize) as libc::c_int
        + *dct.offset(1 as libc::c_int as isize) as libc::c_int;
    let mut d1: libc::c_int = *dct.offset(2 as libc::c_int as isize) as libc::c_int
        + *dct.offset(3 as libc::c_int as isize) as libc::c_int;
    let mut d2: libc::c_int = *dct.offset(0 as libc::c_int as isize) as libc::c_int
        - *dct.offset(1 as libc::c_int as isize) as libc::c_int;
    let mut d3: libc::c_int = *dct.offset(2 as libc::c_int as isize) as libc::c_int
        - *dct.offset(3 as libc::c_int as isize) as libc::c_int;
    let mut dmf: libc::c_int = (*dequant_mf.offset((i_qp % 6 as libc::c_int) as isize))
        [0 as libc::c_int as usize]
        << (i_qp / 6 as libc::c_int);
    (*dct4x4.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] =
        (((d0 + d1) * dmf) >> 5 as libc::c_int) as dctcoef;
    (*dct4x4.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] =
        (((d0 - d1) * dmf) >> 5 as libc::c_int) as dctcoef;
    (*dct4x4.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize] =
        (((d2 + d3) * dmf) >> 5 as libc::c_int) as dctcoef;
    (*dct4x4.offset(3 as libc::c_int as isize))[0 as libc::c_int as usize] =
        (((d2 - d3) * dmf) >> 5 as libc::c_int) as dctcoef;
}
#[inline]
unsafe extern "C" fn idct_dequant_2x2_dconly(
    mut dct: *mut dctcoef,
    mut dequant_mf: *mut [libc::c_int; 16],
    mut i_qp: libc::c_int,
) {
    let mut d0: libc::c_int = *dct.offset(0 as libc::c_int as isize) as libc::c_int
        + *dct.offset(1 as libc::c_int as isize) as libc::c_int;
    let mut d1: libc::c_int = *dct.offset(2 as libc::c_int as isize) as libc::c_int
        + *dct.offset(3 as libc::c_int as isize) as libc::c_int;
    let mut d2: libc::c_int = *dct.offset(0 as libc::c_int as isize) as libc::c_int
        - *dct.offset(1 as libc::c_int as isize) as libc::c_int;
    let mut d3: libc::c_int = *dct.offset(2 as libc::c_int as isize) as libc::c_int
        - *dct.offset(3 as libc::c_int as isize) as libc::c_int;
    let mut dmf: libc::c_int = (*dequant_mf.offset((i_qp % 6 as libc::c_int) as isize))
        [0 as libc::c_int as usize]
        << (i_qp / 6 as libc::c_int);
    *dct.offset(0 as libc::c_int as isize) = (((d0 + d1) * dmf) >> 5 as libc::c_int) as dctcoef;
    *dct.offset(1 as libc::c_int as isize) = (((d0 - d1) * dmf) >> 5 as libc::c_int) as dctcoef;
    *dct.offset(2 as libc::c_int as isize) = (((d2 + d3) * dmf) >> 5 as libc::c_int) as dctcoef;
    *dct.offset(3 as libc::c_int as isize) = (((d2 - d3) * dmf) >> 5 as libc::c_int) as dctcoef;
}
#[inline]
unsafe extern "C" fn dct2x2dc(mut d: *mut dctcoef, mut dct4x4: *mut [dctcoef; 16]) {
    let mut d0: libc::c_int = (*dct4x4.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
        as libc::c_int
        + (*dct4x4.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] as libc::c_int;
    let mut d1: libc::c_int = (*dct4x4.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize]
        as libc::c_int
        + (*dct4x4.offset(3 as libc::c_int as isize))[0 as libc::c_int as usize] as libc::c_int;
    let mut d2: libc::c_int = (*dct4x4.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
        as libc::c_int
        - (*dct4x4.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] as libc::c_int;
    let mut d3: libc::c_int = (*dct4x4.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize]
        as libc::c_int
        - (*dct4x4.offset(3 as libc::c_int as isize))[0 as libc::c_int as usize] as libc::c_int;
    *d.offset(0 as libc::c_int as isize) = (d0 + d1) as dctcoef;
    *d.offset(2 as libc::c_int as isize) = (d2 + d3) as dctcoef;
    *d.offset(1 as libc::c_int as isize) = (d0 - d1) as dctcoef;
    *d.offset(3 as libc::c_int as isize) = (d2 - d3) as dctcoef;
    (*dct4x4.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] =
        0 as libc::c_int as dctcoef;
    (*dct4x4.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] =
        0 as libc::c_int as dctcoef;
    (*dct4x4.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize] =
        0 as libc::c_int as dctcoef;
    (*dct4x4.offset(3 as libc::c_int as isize))[0 as libc::c_int as usize] =
        0 as libc::c_int as dctcoef;
}
#[inline(always)]
unsafe extern "C" fn array_non_zero(mut v: *mut dctcoef, mut i_count: libc::c_int) -> libc::c_int {
    if ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
        == 8 as libc::c_int as libc::c_ulong
    {
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < i_count {
            if (*(&mut *v.offset(i as isize) as *mut dctcoef as *mut x264_union64_t)).i != 0 {
                return 1 as libc::c_int;
            }
            i = (i as libc::c_ulong).wrapping_add(
                (8 as libc::c_int as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<dctcoef>() as libc::c_ulong),
            ) as libc::c_int as libc::c_int;
        }
    } else {
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < i_count {
            if (*(&mut *v.offset(i_0 as isize) as *mut dctcoef as *mut x264_union32_t)).i != 0 {
                return 1 as libc::c_int;
            }
            i_0 = (i_0 as libc::c_ulong).wrapping_add(
                (4 as libc::c_int as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<dctcoef>() as libc::c_ulong),
            ) as libc::c_int as libc::c_int;
        }
    }
    0 as libc::c_int
}
unsafe extern "C" fn mb_encode_i16x16(
    mut h: *mut x264_t,
    mut p: libc::c_int,
    mut i_qp: libc::c_int,
) {
    let mut p_src: *mut pixel = (*h).mb.pic.p_fenc[p as usize];
    let mut p_dst: *mut pixel = (*h).mb.pic.p_fdec[p as usize];
    let mut dct4x4: [[dctcoef; 16]; 16] = [[0; 16]; 16];
    let mut dct_dc4x4: [dctcoef; 16] = [0; 16];
    let mut nz: libc::c_int = 0;
    let mut block_cbp: libc::c_int = 0 as libc::c_int;
    let mut decimate_score: libc::c_int = if (*h).mb.b_dct_decimate != 0 {
        0 as libc::c_int
    } else {
        9 as libc::c_int
    };
    let mut i_quant_cat: libc::c_int = if p != 0 {
        CQM_4IC as libc::c_int
    } else {
        CQM_4IY as libc::c_int
    };
    let mut i_mode: libc::c_int = (*h).mb.i_intra16x16_pred_mode;
    if (*h).mb.b_lossless != 0 {
        x264_8_predict_lossless_16x16(h, p, i_mode);
    } else {
        ((*h).predict_16x16[i_mode as usize]).expect("non-null function pointer")(
            (*h).mb.pic.p_fdec[p as usize],
        );
    }
    if (*h).mb.b_lossless != 0 {
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < 16 as libc::c_int {
            let mut oe: libc::c_int = block_idx_xy_fenc[i as usize] as libc::c_int;
            let mut od: libc::c_int = block_idx_xy_fdec[i as usize] as libc::c_int;
            nz = ((*h).zigzagf.sub_4x4ac).expect("non-null function pointer")(
                ((*h).dct.luma4x4[(16 as libc::c_int * p + i) as usize]).as_mut_ptr(),
                p_src.offset(oe as isize),
                p_dst.offset(od as isize),
                &mut *dct_dc4x4
                    .as_mut_ptr()
                    .offset(*block_idx_yx_1d.as_ptr().offset(i as isize) as isize),
            );
            (*h).mb.cache.non_zero_count
                [x264_scan8[(16 as libc::c_int * p + i) as usize] as usize] = nz as uint8_t;
            block_cbp |= nz;
            i += 1;
            i;
        }
        (*h).mb.i_cbp_luma |= block_cbp * 0xf as libc::c_int;
        (*h).mb.cache.non_zero_count[x264_scan8[(48 as libc::c_int + p) as usize] as usize] =
            array_non_zero(dct_dc4x4.as_mut_ptr(), 16 as libc::c_int) as uint8_t;
        ((*h).zigzagf.scan_4x4).expect("non-null function pointer")(
            ((*h).dct.luma16x16_dc[p as usize]).as_mut_ptr(),
            dct_dc4x4.as_mut_ptr(),
        );
        return;
    }
    (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
        (*x264_scan8.as_ptr().offset((16 as libc::c_int * p) as isize) as libc::c_int
            + 0 as libc::c_int * 8 as libc::c_int) as isize,
    ) as *mut uint8_t as *mut x264_union32_t))
        .i = 0 as libc::c_int as uint32_t;
    (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
        (*x264_scan8.as_ptr().offset((16 as libc::c_int * p) as isize) as libc::c_int
            + 1 as libc::c_int * 8 as libc::c_int) as isize,
    ) as *mut uint8_t as *mut x264_union32_t))
        .i = 0 as libc::c_int as uint32_t;
    (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
        (*x264_scan8.as_ptr().offset((16 as libc::c_int * p) as isize) as libc::c_int
            + 2 as libc::c_int * 8 as libc::c_int) as isize,
    ) as *mut uint8_t as *mut x264_union32_t))
        .i = 0 as libc::c_int as uint32_t;
    (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
        (*x264_scan8.as_ptr().offset((16 as libc::c_int * p) as isize) as libc::c_int
            + 3 as libc::c_int * 8 as libc::c_int) as isize,
    ) as *mut uint8_t as *mut x264_union32_t))
        .i = 0 as libc::c_int as uint32_t;
    ((*h).dctf.sub16x16_dct).expect("non-null function pointer")(dct4x4.as_mut_ptr(), p_src, p_dst);
    if (*h).mb.b_noise_reduction != 0 {
        let mut idx: libc::c_int = 0 as libc::c_int;
        while idx < 16 as libc::c_int {
            ((*h).quantf.denoise_dct).expect("non-null function pointer")(
                (dct4x4[idx as usize]).as_mut_ptr(),
                (*((*h).nr_residual_sum).offset(0 as libc::c_int as isize)).as_mut_ptr(),
                (*((*h).nr_offset).offset(0 as libc::c_int as isize)).as_mut_ptr(),
                16 as libc::c_int,
            );
            idx += 1;
            idx;
        }
    }
    let mut idx_0: libc::c_int = 0 as libc::c_int;
    while idx_0 < 16 as libc::c_int {
        dct_dc4x4[block_idx_xy_1d[idx_0 as usize] as usize] =
            dct4x4[idx_0 as usize][0 as libc::c_int as usize];
        dct4x4[idx_0 as usize][0 as libc::c_int as usize] = 0 as libc::c_int as dctcoef;
        idx_0 += 1;
        idx_0;
    }
    if (*h).mb.b_trellis != 0 {
        let mut idx_1: libc::c_int = 0 as libc::c_int;
        while idx_1 < 16 as libc::c_int {
            if x264_8_quant_4x4_trellis(
                h,
                (dct4x4[idx_1 as usize]).as_mut_ptr(),
                i_quant_cat,
                i_qp,
                ctx_cat_plane[DCT_LUMA_AC as libc::c_int as usize][p as usize] as libc::c_int,
                1 as libc::c_int,
                (p != 0) as libc::c_int,
                idx_1,
            ) != 0
            {
                block_cbp = 0xf as libc::c_int;
                ((*h).zigzagf.scan_4x4).expect("non-null function pointer")(
                    ((*h).dct.luma4x4[(16 as libc::c_int * p + idx_1) as usize]).as_mut_ptr(),
                    (dct4x4[idx_1 as usize]).as_mut_ptr(),
                );
                ((*h).quantf.dequant_4x4).expect("non-null function pointer")(
                    (dct4x4[idx_1 as usize]).as_mut_ptr(),
                    (*h).dequant4_mf[i_quant_cat as usize],
                    i_qp,
                );
                if decimate_score < 6 as libc::c_int {
                    decimate_score += ((*h).quantf.decimate_score15)
                        .expect("non-null function pointer")(
                        ((*h).dct.luma4x4[(16 as libc::c_int * p + idx_1) as usize]).as_mut_ptr(),
                    );
                }
                (*h).mb.cache.non_zero_count
                    [x264_scan8[(16 as libc::c_int * p + idx_1) as usize] as usize] =
                    1 as libc::c_int as uint8_t;
            }
            idx_1 += 1;
            idx_1;
        }
    } else {
        let mut i8x8: libc::c_int = 0 as libc::c_int;
        while i8x8 < 4 as libc::c_int {
            nz = ((*h).quantf.quant_4x4x4).expect("non-null function pointer")(
                &mut *dct4x4
                    .as_mut_ptr()
                    .offset((i8x8 * 4 as libc::c_int) as isize),
                (*((*h).quant4_mf[i_quant_cat as usize]).offset(i_qp as isize)).as_mut_ptr(),
                (*((*h).quant4_bias[i_quant_cat as usize]).offset(i_qp as isize)).as_mut_ptr(),
            );
            if nz != 0 {
                block_cbp = 0xf as libc::c_int;
                let mut idx_2: libc::c_int = i8x8 * 4 as libc::c_int;
                let mut msk: libc::c_int = nz;
                let mut skip: libc::c_int = 0;
                while msk != 0 && {
                    skip = x264_ctz_4bit(msk as uint32_t);
                    idx_2 += skip;
                    msk >>= skip + 1 as libc::c_int;
                    1 as libc::c_int != 0
                } {
                    ((*h).zigzagf.scan_4x4).expect("non-null function pointer")(
                        ((*h).dct.luma4x4[(16 as libc::c_int * p + idx_2) as usize]).as_mut_ptr(),
                        (dct4x4[idx_2 as usize]).as_mut_ptr(),
                    );
                    ((*h).quantf.dequant_4x4).expect("non-null function pointer")(
                        (dct4x4[idx_2 as usize]).as_mut_ptr(),
                        (*h).dequant4_mf[i_quant_cat as usize],
                        i_qp,
                    );
                    if decimate_score < 6 as libc::c_int {
                        decimate_score += ((*h).quantf.decimate_score15)
                            .expect("non-null function pointer")(
                            ((*h).dct.luma4x4[(16 as libc::c_int * p + idx_2) as usize])
                                .as_mut_ptr(),
                        );
                    }
                    (*h).mb.cache.non_zero_count
                        [x264_scan8[(16 as libc::c_int * p + idx_2) as usize] as usize] =
                        1 as libc::c_int as uint8_t;
                    idx_2 += 1;
                    idx_2;
                }
            }
            i8x8 += 1;
            i8x8;
        }
    }
    if decimate_score < 6 as libc::c_int {
        (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
            (*x264_scan8.as_ptr().offset((16 as libc::c_int * p) as isize) as libc::c_int
                + 0 as libc::c_int * 8 as libc::c_int) as isize,
        ) as *mut uint8_t as *mut x264_union32_t))
            .i = 0 as libc::c_int as uint32_t;
        (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
            (*x264_scan8.as_ptr().offset((16 as libc::c_int * p) as isize) as libc::c_int
                + 1 as libc::c_int * 8 as libc::c_int) as isize,
        ) as *mut uint8_t as *mut x264_union32_t))
            .i = 0 as libc::c_int as uint32_t;
        (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
            (*x264_scan8.as_ptr().offset((16 as libc::c_int * p) as isize) as libc::c_int
                + 2 as libc::c_int * 8 as libc::c_int) as isize,
        ) as *mut uint8_t as *mut x264_union32_t))
            .i = 0 as libc::c_int as uint32_t;
        (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
            (*x264_scan8.as_ptr().offset((16 as libc::c_int * p) as isize) as libc::c_int
                + 3 as libc::c_int * 8 as libc::c_int) as isize,
        ) as *mut uint8_t as *mut x264_union32_t))
            .i = 0 as libc::c_int as uint32_t;
        block_cbp = 0 as libc::c_int;
    } else {
        (*h).mb.i_cbp_luma |= block_cbp;
    }
    ((*h).dctf.dct4x4dc).expect("non-null function pointer")(dct_dc4x4.as_mut_ptr());
    if (*h).mb.b_trellis != 0 {
        nz = x264_8_quant_luma_dc_trellis(
            h,
            dct_dc4x4.as_mut_ptr(),
            i_quant_cat,
            i_qp,
            ctx_cat_plane[DCT_LUMA_DC as libc::c_int as usize][p as usize] as libc::c_int,
            1 as libc::c_int,
            48 as libc::c_int + p,
        );
    } else {
        nz = ((*h).quantf.quant_4x4_dc).expect("non-null function pointer")(
            dct_dc4x4.as_mut_ptr(),
            (*((*h).quant4_mf[i_quant_cat as usize]).offset(i_qp as isize))
                [0 as libc::c_int as usize] as libc::c_int
                >> 1 as libc::c_int,
            ((*((*h).quant4_bias[i_quant_cat as usize]).offset(i_qp as isize))
                [0 as libc::c_int as usize] as libc::c_int)
                << 1 as libc::c_int,
        );
    }
    (*h).mb.cache.non_zero_count[x264_scan8[(48 as libc::c_int + p) as usize] as usize] =
        nz as uint8_t;
    if nz != 0 {
        ((*h).zigzagf.scan_4x4).expect("non-null function pointer")(
            ((*h).dct.luma16x16_dc[p as usize]).as_mut_ptr(),
            dct_dc4x4.as_mut_ptr(),
        );
        ((*h).dctf.idct4x4dc).expect("non-null function pointer")(dct_dc4x4.as_mut_ptr());
        ((*h).quantf.dequant_4x4_dc).expect("non-null function pointer")(
            dct_dc4x4.as_mut_ptr(),
            (*h).dequant4_mf[i_quant_cat as usize],
            i_qp,
        );
        if block_cbp != 0 {
            let mut i_0: libc::c_int = 0 as libc::c_int;
            while i_0 < 16 as libc::c_int {
                dct4x4[i_0 as usize][0 as libc::c_int as usize] =
                    dct_dc4x4[block_idx_xy_1d[i_0 as usize] as usize];
                i_0 += 1;
                i_0;
            }
        }
    }
    if block_cbp != 0 {
        ((*h).dctf.add16x16_idct).expect("non-null function pointer")(p_dst, dct4x4.as_mut_ptr());
    } else if nz != 0 {
        ((*h).dctf.add16x16_idct_dc).expect("non-null function pointer")(
            p_dst,
            dct_dc4x4.as_mut_ptr(),
        );
    }
}
#[inline(always)]
unsafe extern "C" fn mb_optimize_chroma_dc(
    mut h: *mut x264_t,
    mut dct_dc: *mut dctcoef,
    mut dequant_mf: *mut [libc::c_int; 16],
    mut i_qp: libc::c_int,
    mut chroma422: libc::c_int,
) -> libc::c_int {
    let mut dmf: libc::c_int = (*dequant_mf.offset((i_qp % 6 as libc::c_int) as isize))
        [0 as libc::c_int as usize]
        << (i_qp / 6 as libc::c_int);
    if dmf > 32 as libc::c_int * 64 as libc::c_int {
        return 1 as libc::c_int;
    }
    if chroma422 != 0 {
        ((*h).quantf.optimize_chroma_2x4_dc).expect("non-null function pointer")(dct_dc, dmf)
    } else {
        ((*h).quantf.optimize_chroma_2x2_dc).expect("non-null function pointer")(dct_dc, dmf)
    }
}
#[inline(always)]
unsafe extern "C" fn mb_encode_chroma_internal(
    mut h: *mut x264_t,
    mut b_inter: libc::c_int,
    mut i_qp: libc::c_int,
    mut chroma422: libc::c_int,
) {
    let mut nz: libc::c_int = 0;
    let mut nz_dc: libc::c_int = 0;
    let mut b_decimate: libc::c_int = (b_inter != 0 && (*h).mb.b_dct_decimate != 0) as libc::c_int;
    let mut dequant_mf: *mut [libc::c_int; 16] =
        (*h).dequant4_mf[(CQM_4IC as libc::c_int + b_inter) as usize];
    let mut dct_dc: [dctcoef; 8] = [0; 8];
    (*h).mb.i_cbp_chroma = 0 as libc::c_int;
    let fresh0 = &mut (*((*h).nr_count).offset(2 as libc::c_int as isize));
    *fresh0 = (*fresh0).wrapping_add(((*h).mb.b_noise_reduction * 4 as libc::c_int) as uint32_t);
    (*(&mut *((*h).mb.cache.non_zero_count)
        .as_mut_ptr()
        .offset(*x264_scan8.as_ptr().offset(16 as libc::c_int as isize) as isize)
        as *mut uint8_t as *mut x264_union16_t))
        .i = 0 as libc::c_int as uint16_t;
    (*(&mut *((*h).mb.cache.non_zero_count)
        .as_mut_ptr()
        .offset(*x264_scan8.as_ptr().offset(18 as libc::c_int as isize) as isize)
        as *mut uint8_t as *mut x264_union16_t))
        .i = 0 as libc::c_int as uint16_t;
    (*(&mut *((*h).mb.cache.non_zero_count)
        .as_mut_ptr()
        .offset(*x264_scan8.as_ptr().offset(32 as libc::c_int as isize) as isize)
        as *mut uint8_t as *mut x264_union16_t))
        .i = 0 as libc::c_int as uint16_t;
    (*(&mut *((*h).mb.cache.non_zero_count)
        .as_mut_ptr()
        .offset(*x264_scan8.as_ptr().offset(34 as libc::c_int as isize) as isize)
        as *mut uint8_t as *mut x264_union16_t))
        .i = 0 as libc::c_int as uint16_t;
    if chroma422 != 0 {
        (*(&mut *((*h).mb.cache.non_zero_count)
            .as_mut_ptr()
            .offset(*x264_scan8.as_ptr().offset(24 as libc::c_int as isize) as isize)
            as *mut uint8_t as *mut x264_union16_t))
            .i = 0 as libc::c_int as uint16_t;
        (*(&mut *((*h).mb.cache.non_zero_count)
            .as_mut_ptr()
            .offset(*x264_scan8.as_ptr().offset(26 as libc::c_int as isize) as isize)
            as *mut uint8_t as *mut x264_union16_t))
            .i = 0 as libc::c_int as uint16_t;
        (*(&mut *((*h).mb.cache.non_zero_count)
            .as_mut_ptr()
            .offset(*x264_scan8.as_ptr().offset(40 as libc::c_int as isize) as isize)
            as *mut uint8_t as *mut x264_union16_t))
            .i = 0 as libc::c_int as uint16_t;
        (*(&mut *((*h).mb.cache.non_zero_count)
            .as_mut_ptr()
            .offset(*x264_scan8.as_ptr().offset(42 as libc::c_int as isize) as isize)
            as *mut uint8_t as *mut x264_union16_t))
            .i = 0 as libc::c_int as uint16_t;
    }
    if b_decimate != 0
        && i_qp
            >= (if (*h).mb.b_trellis != 0 {
                12 as libc::c_int
            } else {
                18 as libc::c_int
            })
        && (*h).mb.b_noise_reduction == 0
    {
        let mut thresh: libc::c_int = if chroma422 != 0 {
            (x264_lambda2_tab[i_qp as usize] + 16 as libc::c_int) >> 5 as libc::c_int
        } else {
            (x264_lambda2_tab[i_qp as usize] + 32 as libc::c_int) >> 6 as libc::c_int
        };
        let mut ssd: [libc::c_int; 2] = [0; 2];
        let mut chromapix: libc::c_int = if chroma422 != 0 {
            PIXEL_8x16 as libc::c_int
        } else {
            PIXEL_8x8 as libc::c_int
        };
        if ((*h).pixf.var2[chromapix as usize]).expect("non-null function pointer")(
            (*h).mb.pic.p_fenc[1 as libc::c_int as usize],
            (*h).mb.pic.p_fdec[1 as libc::c_int as usize],
            ssd.as_mut_ptr(),
        ) < thresh * 4 as libc::c_int
        {
            (*h).mb.cache.non_zero_count
                [x264_scan8[(49 as libc::c_int + 0 as libc::c_int) as usize] as usize] =
                0 as libc::c_int as uint8_t;
            (*h).mb.cache.non_zero_count
                [x264_scan8[(49 as libc::c_int + 1 as libc::c_int) as usize] as usize] =
                0 as libc::c_int as uint8_t;
            let mut ch: libc::c_int = 0 as libc::c_int;
            while ch < 2 as libc::c_int {
                if ssd[ch as usize] > thresh {
                    let mut p_src: *mut pixel =
                        (*h).mb.pic.p_fenc[(1 as libc::c_int + ch) as usize];
                    let mut p_dst: *mut pixel =
                        (*h).mb.pic.p_fdec[(1 as libc::c_int + ch) as usize];
                    if chroma422 != 0 {
                        ((*h).dctf.sub8x16_dct_dc).expect("non-null function pointer")(
                            dct_dc.as_mut_ptr(),
                            p_src,
                            p_dst,
                        );
                    } else {
                        ((*h).dctf.sub8x8_dct_dc).expect("non-null function pointer")(
                            dct_dc.as_mut_ptr(),
                            p_src,
                            p_dst,
                        );
                    }
                    if (*h).mb.b_trellis != 0 {
                        nz_dc = x264_8_quant_chroma_dc_trellis(
                            h,
                            dct_dc.as_mut_ptr(),
                            i_qp + 3 as libc::c_int * chroma422,
                            (b_inter == 0) as libc::c_int,
                            49 as libc::c_int + ch,
                        );
                    } else {
                        nz_dc = 0 as libc::c_int;
                        let mut i: libc::c_int = 0 as libc::c_int;
                        while i <= chroma422 {
                            nz_dc |= ((*h).quantf.quant_2x2_dc).expect("non-null function pointer")(
                                &mut *dct_dc.as_mut_ptr().offset((4 as libc::c_int * i) as isize),
                                (*((*h).quant4_mf[(CQM_4IC as libc::c_int + b_inter) as usize])
                                    .offset((i_qp + 3 as libc::c_int * chroma422) as isize))
                                    [0 as libc::c_int as usize]
                                    as libc::c_int
                                    >> 1 as libc::c_int,
                                ((*((*h).quant4_bias[(CQM_4IC as libc::c_int + b_inter) as usize])
                                    .offset((i_qp + 3 as libc::c_int * chroma422) as isize))
                                    [0 as libc::c_int as usize]
                                    as libc::c_int)
                                    << 1 as libc::c_int,
                            );
                            i += 1;
                            i;
                        }
                    }
                    if nz_dc != 0
                        && mb_optimize_chroma_dc(
                            h,
                            dct_dc.as_mut_ptr(),
                            dequant_mf,
                            i_qp + 3 as libc::c_int * chroma422,
                            chroma422,
                        ) != 0
                    {
                        (*h).mb.cache.non_zero_count
                            [x264_scan8[(49 as libc::c_int + ch) as usize] as usize] =
                            1 as libc::c_int as uint8_t;
                        if chroma422 != 0 {
                            zigzag_scan_2x4_dc(
                                ((*h).dct.chroma_dc[ch as usize]).as_mut_ptr(),
                                dct_dc.as_mut_ptr(),
                            );
                            ((*h).quantf.idct_dequant_2x4_dconly)
                                .expect("non-null function pointer")(
                                dct_dc.as_mut_ptr(),
                                dequant_mf,
                                i_qp + 3 as libc::c_int,
                            );
                        } else {
                            zigzag_scan_2x2_dc(
                                ((*h).dct.chroma_dc[ch as usize]).as_mut_ptr(),
                                dct_dc.as_mut_ptr(),
                            );
                            idct_dequant_2x2_dconly(dct_dc.as_mut_ptr(), dequant_mf, i_qp);
                        }
                        let mut i_0: libc::c_int = 0 as libc::c_int;
                        while i_0 <= chroma422 {
                            ((*h).dctf.add8x8_idct_dc).expect("non-null function pointer")(
                                p_dst.offset((8 as libc::c_int * i_0 * 32 as libc::c_int) as isize),
                                &mut *dct_dc
                                    .as_mut_ptr()
                                    .offset((4 as libc::c_int * i_0) as isize),
                            );
                            i_0 += 1;
                            i_0;
                        }
                        (*h).mb.i_cbp_chroma = 1 as libc::c_int;
                    }
                }
                ch += 1;
                ch;
            }
            return;
        }
    }
    let mut ch_0: libc::c_int = 0 as libc::c_int;
    while ch_0 < 2 as libc::c_int {
        let mut p_src_0: *mut pixel = (*h).mb.pic.p_fenc[(1 as libc::c_int + ch_0) as usize];
        let mut p_dst_0: *mut pixel = (*h).mb.pic.p_fdec[(1 as libc::c_int + ch_0) as usize];
        let mut i_decimate_score: libc::c_int = if b_decimate != 0 {
            0 as libc::c_int
        } else {
            7 as libc::c_int
        };
        let mut nz_ac: libc::c_int = 0 as libc::c_int;
        let mut dct4x4: [[dctcoef; 16]; 8] = [[0; 16]; 8];
        if (*h).mb.b_lossless != 0 {
            static mut chroma422_scan: [uint8_t; 8] = [
                0 as libc::c_int as uint8_t,
                2 as libc::c_int as uint8_t,
                1 as libc::c_int as uint8_t,
                5 as libc::c_int as uint8_t,
                3 as libc::c_int as uint8_t,
                6 as libc::c_int as uint8_t,
                4 as libc::c_int as uint8_t,
                7 as libc::c_int as uint8_t,
            ];
            let mut i_1: libc::c_int = 0 as libc::c_int;
            while i_1
                < (if chroma422 != 0 {
                    8 as libc::c_int
                } else {
                    4 as libc::c_int
                })
            {
                let mut oe: libc::c_int = 4 as libc::c_int * (i_1 & 1 as libc::c_int)
                    + 4 as libc::c_int * (i_1 >> 1 as libc::c_int) * 16 as libc::c_int;
                let mut od: libc::c_int = 4 as libc::c_int * (i_1 & 1 as libc::c_int)
                    + 4 as libc::c_int * (i_1 >> 1 as libc::c_int) * 32 as libc::c_int;
                nz = ((*h).zigzagf.sub_4x4ac).expect("non-null function pointer")(
                    ((*h).dct.luma4x4[(16 as libc::c_int
                        + i_1
                        + (if chroma422 != 0 {
                            i_1 & 4 as libc::c_int
                        } else {
                            0 as libc::c_int
                        })
                        + ch_0 * 16 as libc::c_int)
                        as usize])
                        .as_mut_ptr(),
                    p_src_0.offset(oe as isize),
                    p_dst_0.offset(od as isize),
                    &mut *(*((*h).dct.chroma_dc).as_mut_ptr().offset(ch_0 as isize))
                        .as_mut_ptr()
                        .offset(
                            (if chroma422 != 0 {
                                *chroma422_scan.as_ptr().offset(i_1 as isize) as libc::c_int
                            } else {
                                i_1
                            }) as isize,
                        ),
                );
                (*h).mb.cache.non_zero_count[x264_scan8[(16 as libc::c_int
                    + i_1
                    + (if chroma422 != 0 {
                        i_1 & 4 as libc::c_int
                    } else {
                        0 as libc::c_int
                    })
                    + ch_0 * 16 as libc::c_int)
                    as usize] as usize] = nz as uint8_t;
                (*h).mb.i_cbp_chroma |= nz;
                i_1 += 1;
                i_1;
            }
            (*h).mb.cache.non_zero_count
                [x264_scan8[(49 as libc::c_int + ch_0) as usize] as usize] = array_non_zero(
                ((*h).dct.chroma_dc[ch_0 as usize]).as_mut_ptr(),
                if chroma422 != 0 {
                    8 as libc::c_int
                } else {
                    4 as libc::c_int
                },
            )
                as uint8_t;
        } else {
            let mut i_2: libc::c_int = 0 as libc::c_int;
            while i_2 <= chroma422 {
                ((*h).dctf.sub8x8_dct).expect("non-null function pointer")(
                    &mut *dct4x4
                        .as_mut_ptr()
                        .offset((4 as libc::c_int * i_2) as isize),
                    p_src_0.offset((8 as libc::c_int * i_2 * 16 as libc::c_int) as isize),
                    p_dst_0.offset((8 as libc::c_int * i_2 * 32 as libc::c_int) as isize),
                );
                i_2 += 1;
                i_2;
            }
            if (*h).mb.b_noise_reduction != 0 {
                let mut i_3: libc::c_int = 0 as libc::c_int;
                while i_3
                    < (if chroma422 != 0 {
                        8 as libc::c_int
                    } else {
                        4 as libc::c_int
                    })
                {
                    ((*h).quantf.denoise_dct).expect("non-null function pointer")(
                        (dct4x4[i_3 as usize]).as_mut_ptr(),
                        (*((*h).nr_residual_sum).offset(2 as libc::c_int as isize)).as_mut_ptr(),
                        (*((*h).nr_offset).offset(2 as libc::c_int as isize)).as_mut_ptr(),
                        16 as libc::c_int,
                    );
                    i_3 += 1;
                    i_3;
                }
            }
            if chroma422 != 0 {
                ((*h).dctf.dct2x4dc).expect("non-null function pointer")(
                    dct_dc.as_mut_ptr(),
                    dct4x4.as_mut_ptr(),
                );
            } else {
                dct2x2dc(dct_dc.as_mut_ptr(), dct4x4.as_mut_ptr());
            }
            let mut i8x8: libc::c_int = 0 as libc::c_int;
            while i8x8
                < (if chroma422 != 0 {
                    2 as libc::c_int
                } else {
                    1 as libc::c_int
                })
            {
                if (*h).mb.b_trellis != 0 {
                    let mut i4x4: libc::c_int = 0 as libc::c_int;
                    while i4x4 < 4 as libc::c_int {
                        if x264_8_quant_4x4_trellis(
                            h,
                            (dct4x4[(i8x8 * 4 as libc::c_int + i4x4) as usize]).as_mut_ptr(),
                            CQM_4IC as libc::c_int + b_inter,
                            i_qp,
                            DCT_CHROMA_AC as libc::c_int,
                            (b_inter == 0) as libc::c_int,
                            1 as libc::c_int,
                            0 as libc::c_int,
                        ) != 0
                        {
                            let mut idx: libc::c_int = 16 as libc::c_int
                                + ch_0 * 16 as libc::c_int
                                + i8x8 * 8 as libc::c_int
                                + i4x4;
                            ((*h).zigzagf.scan_4x4).expect("non-null function pointer")(
                                ((*h).dct.luma4x4[idx as usize]).as_mut_ptr(),
                                (dct4x4[(i8x8 * 4 as libc::c_int + i4x4) as usize]).as_mut_ptr(),
                            );
                            ((*h).quantf.dequant_4x4).expect("non-null function pointer")(
                                (dct4x4[(i8x8 * 4 as libc::c_int + i4x4) as usize]).as_mut_ptr(),
                                dequant_mf,
                                i_qp,
                            );
                            if i_decimate_score < 7 as libc::c_int {
                                i_decimate_score += ((*h).quantf.decimate_score15)
                                    .expect("non-null function pointer")(
                                    ((*h).dct.luma4x4[idx as usize]).as_mut_ptr(),
                                );
                            }
                            (*h).mb.cache.non_zero_count[x264_scan8[idx as usize] as usize] =
                                1 as libc::c_int as uint8_t;
                            nz_ac = 1 as libc::c_int;
                        }
                        i4x4 += 1;
                        i4x4;
                    }
                } else {
                    nz = ((*h).quantf.quant_4x4x4).expect("non-null function pointer")(
                        &mut *dct4x4
                            .as_mut_ptr()
                            .offset((i8x8 * 4 as libc::c_int) as isize),
                        (*((*h).quant4_mf[(CQM_4IC as libc::c_int + b_inter) as usize])
                            .offset(i_qp as isize))
                        .as_mut_ptr(),
                        (*((*h).quant4_bias[(CQM_4IC as libc::c_int + b_inter) as usize])
                            .offset(i_qp as isize))
                        .as_mut_ptr(),
                    );
                    nz_ac |= nz;
                    let mut i4x4_0: libc::c_int = 0 as libc::c_int;
                    let mut msk: libc::c_int = nz;
                    let mut skip: libc::c_int = 0;
                    while msk != 0 && {
                        skip = x264_ctz_4bit(msk as uint32_t);
                        i4x4_0 += skip;
                        msk >>= skip + 1 as libc::c_int;
                        1 as libc::c_int != 0
                    } {
                        let mut idx_0: libc::c_int = 16 as libc::c_int
                            + ch_0 * 16 as libc::c_int
                            + i8x8 * 8 as libc::c_int
                            + i4x4_0;
                        ((*h).zigzagf.scan_4x4).expect("non-null function pointer")(
                            ((*h).dct.luma4x4[idx_0 as usize]).as_mut_ptr(),
                            (dct4x4[(i8x8 * 4 as libc::c_int + i4x4_0) as usize]).as_mut_ptr(),
                        );
                        ((*h).quantf.dequant_4x4).expect("non-null function pointer")(
                            (dct4x4[(i8x8 * 4 as libc::c_int + i4x4_0) as usize]).as_mut_ptr(),
                            dequant_mf,
                            i_qp,
                        );
                        if i_decimate_score < 7 as libc::c_int {
                            i_decimate_score += ((*h).quantf.decimate_score15)
                                .expect("non-null function pointer")(
                                ((*h).dct.luma4x4[idx_0 as usize]).as_mut_ptr(),
                            );
                        }
                        (*h).mb.cache.non_zero_count[x264_scan8[idx_0 as usize] as usize] =
                            1 as libc::c_int as uint8_t;
                        i4x4_0 += 1;
                        i4x4_0;
                    }
                }
                i8x8 += 1;
                i8x8;
            }
            if (*h).mb.b_trellis != 0 {
                nz_dc = x264_8_quant_chroma_dc_trellis(
                    h,
                    dct_dc.as_mut_ptr(),
                    i_qp + 3 as libc::c_int * chroma422,
                    (b_inter == 0) as libc::c_int,
                    49 as libc::c_int + ch_0,
                );
            } else {
                nz_dc = 0 as libc::c_int;
                let mut i_4: libc::c_int = 0 as libc::c_int;
                while i_4 <= chroma422 {
                    nz_dc |= ((*h).quantf.quant_2x2_dc).expect("non-null function pointer")(
                        &mut *dct_dc
                            .as_mut_ptr()
                            .offset((4 as libc::c_int * i_4) as isize),
                        (*((*h).quant4_mf[(CQM_4IC as libc::c_int + b_inter) as usize])
                            .offset((i_qp + 3 as libc::c_int * chroma422) as isize))
                            [0 as libc::c_int as usize] as libc::c_int
                            >> 1 as libc::c_int,
                        ((*((*h).quant4_bias[(CQM_4IC as libc::c_int + b_inter) as usize])
                            .offset((i_qp + 3 as libc::c_int * chroma422) as isize))
                            [0 as libc::c_int as usize] as libc::c_int)
                            << 1 as libc::c_int,
                    );
                    i_4 += 1;
                    i_4;
                }
            }
            (*h).mb.cache.non_zero_count
                [x264_scan8[(49 as libc::c_int + ch_0) as usize] as usize] = nz_dc as uint8_t;
            if i_decimate_score < 7 as libc::c_int || nz_ac == 0 {
                (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
                    *x264_scan8
                        .as_ptr()
                        .offset((16 as libc::c_int + 16 as libc::c_int * ch_0) as isize)
                        as isize,
                ) as *mut uint8_t as *mut x264_union16_t))
                    .i = 0 as libc::c_int as uint16_t;
                (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
                    *x264_scan8
                        .as_ptr()
                        .offset((18 as libc::c_int + 16 as libc::c_int * ch_0) as isize)
                        as isize,
                ) as *mut uint8_t as *mut x264_union16_t))
                    .i = 0 as libc::c_int as uint16_t;
                if chroma422 != 0 {
                    (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
                        *x264_scan8
                            .as_ptr()
                            .offset((24 as libc::c_int + 16 as libc::c_int * ch_0) as isize)
                            as isize,
                    ) as *mut uint8_t as *mut x264_union16_t))
                        .i = 0 as libc::c_int as uint16_t;
                    (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
                        *x264_scan8
                            .as_ptr()
                            .offset((26 as libc::c_int + 16 as libc::c_int * ch_0) as isize)
                            as isize,
                    ) as *mut uint8_t as *mut x264_union16_t))
                        .i = 0 as libc::c_int as uint16_t;
                }
                if nz_dc != 0 {
                    if mb_optimize_chroma_dc(
                        h,
                        dct_dc.as_mut_ptr(),
                        dequant_mf,
                        i_qp + 3 as libc::c_int * chroma422,
                        chroma422,
                    ) == 0
                    {
                        (*h).mb.cache.non_zero_count
                            [x264_scan8[(49 as libc::c_int + ch_0) as usize] as usize] =
                            0 as libc::c_int as uint8_t;
                    } else {
                        if chroma422 != 0 {
                            zigzag_scan_2x4_dc(
                                ((*h).dct.chroma_dc[ch_0 as usize]).as_mut_ptr(),
                                dct_dc.as_mut_ptr(),
                            );
                            ((*h).quantf.idct_dequant_2x4_dconly)
                                .expect("non-null function pointer")(
                                dct_dc.as_mut_ptr(),
                                dequant_mf,
                                i_qp + 3 as libc::c_int,
                            );
                        } else {
                            zigzag_scan_2x2_dc(
                                ((*h).dct.chroma_dc[ch_0 as usize]).as_mut_ptr(),
                                dct_dc.as_mut_ptr(),
                            );
                            idct_dequant_2x2_dconly(dct_dc.as_mut_ptr(), dequant_mf, i_qp);
                        }
                        let mut i_5: libc::c_int = 0 as libc::c_int;
                        while i_5 <= chroma422 {
                            ((*h).dctf.add8x8_idct_dc).expect("non-null function pointer")(
                                p_dst_0
                                    .offset((8 as libc::c_int * i_5 * 32 as libc::c_int) as isize),
                                &mut *dct_dc
                                    .as_mut_ptr()
                                    .offset((4 as libc::c_int * i_5) as isize),
                            );
                            i_5 += 1;
                            i_5;
                        }
                    }
                }
            } else {
                (*h).mb.i_cbp_chroma = 1 as libc::c_int;
                if nz_dc != 0 {
                    if chroma422 != 0 {
                        zigzag_scan_2x4_dc(
                            ((*h).dct.chroma_dc[ch_0 as usize]).as_mut_ptr(),
                            dct_dc.as_mut_ptr(),
                        );
                        ((*h).quantf.idct_dequant_2x4_dc).expect("non-null function pointer")(
                            dct_dc.as_mut_ptr(),
                            dct4x4.as_mut_ptr(),
                            dequant_mf,
                            i_qp + 3 as libc::c_int,
                        );
                    } else {
                        zigzag_scan_2x2_dc(
                            ((*h).dct.chroma_dc[ch_0 as usize]).as_mut_ptr(),
                            dct_dc.as_mut_ptr(),
                        );
                        idct_dequant_2x2_dc(
                            dct_dc.as_mut_ptr(),
                            dct4x4.as_mut_ptr(),
                            dequant_mf,
                            i_qp,
                        );
                    }
                }
                let mut i_6: libc::c_int = 0 as libc::c_int;
                while i_6 <= chroma422 {
                    ((*h).dctf.add8x8_idct).expect("non-null function pointer")(
                        p_dst_0.offset((8 as libc::c_int * i_6 * 32 as libc::c_int) as isize),
                        &mut *dct4x4
                            .as_mut_ptr()
                            .offset((4 as libc::c_int * i_6) as isize),
                    );
                    i_6 += 1;
                    i_6;
                }
            }
        }
        ch_0 += 1;
        ch_0;
    }
    (*h).mb.i_cbp_chroma += (*h).mb.cache.non_zero_count
        [x264_scan8[(49 as libc::c_int + 0 as libc::c_int) as usize] as usize]
        as libc::c_int
        | (*h).mb.cache.non_zero_count
            [x264_scan8[(49 as libc::c_int + 1 as libc::c_int) as usize] as usize]
            as libc::c_int
        | (*h).mb.i_cbp_chroma;
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_mb_encode_chroma(
    mut h: *mut x264_t,
    mut b_inter: libc::c_int,
    mut i_qp: libc::c_int,
) {
    if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_420 as libc::c_int {
        mb_encode_chroma_internal(h, b_inter, i_qp, 0 as libc::c_int);
    } else {
        mb_encode_chroma_internal(h, b_inter, i_qp, 1 as libc::c_int);
    };
}
unsafe extern "C" fn macroblock_encode_skip(mut h: *mut x264_t) {
    (*(&mut *((*h).mb.cache.non_zero_count)
        .as_mut_ptr()
        .offset(*x264_scan8.as_ptr().offset(0 as libc::c_int as isize) as isize)
        as *mut uint8_t as *mut x264_union32_t))
        .i = 0 as libc::c_int as uint32_t;
    (*(&mut *((*h).mb.cache.non_zero_count)
        .as_mut_ptr()
        .offset(*x264_scan8.as_ptr().offset(2 as libc::c_int as isize) as isize)
        as *mut uint8_t as *mut x264_union32_t))
        .i = 0 as libc::c_int as uint32_t;
    (*(&mut *((*h).mb.cache.non_zero_count)
        .as_mut_ptr()
        .offset(*x264_scan8.as_ptr().offset(8 as libc::c_int as isize) as isize)
        as *mut uint8_t as *mut x264_union32_t))
        .i = 0 as libc::c_int as uint32_t;
    (*(&mut *((*h).mb.cache.non_zero_count)
        .as_mut_ptr()
        .offset(*x264_scan8.as_ptr().offset(10 as libc::c_int as isize) as isize)
        as *mut uint8_t as *mut x264_union32_t))
        .i = 0 as libc::c_int as uint32_t;
    (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
        *x264_scan8
            .as_ptr()
            .offset((16 as libc::c_int + 0 as libc::c_int) as isize) as isize,
    ) as *mut uint8_t as *mut x264_union32_t))
        .i = 0 as libc::c_int as uint32_t;
    (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
        *x264_scan8
            .as_ptr()
            .offset((16 as libc::c_int + 2 as libc::c_int) as isize) as isize,
    ) as *mut uint8_t as *mut x264_union32_t))
        .i = 0 as libc::c_int as uint32_t;
    (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
        *x264_scan8
            .as_ptr()
            .offset((32 as libc::c_int + 0 as libc::c_int) as isize) as isize,
    ) as *mut uint8_t as *mut x264_union32_t))
        .i = 0 as libc::c_int as uint32_t;
    (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
        *x264_scan8
            .as_ptr()
            .offset((32 as libc::c_int + 2 as libc::c_int) as isize) as isize,
    ) as *mut uint8_t as *mut x264_union32_t))
        .i = 0 as libc::c_int as uint32_t;
    if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc >= CHROMA_422 as libc::c_int {
        (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
            *x264_scan8
                .as_ptr()
                .offset((16 as libc::c_int + 8 as libc::c_int) as isize) as isize,
        ) as *mut uint8_t as *mut x264_union32_t))
            .i = 0 as libc::c_int as uint32_t;
        (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
            *x264_scan8
                .as_ptr()
                .offset((16 as libc::c_int + 10 as libc::c_int) as isize) as isize,
        ) as *mut uint8_t as *mut x264_union32_t))
            .i = 0 as libc::c_int as uint32_t;
        (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
            *x264_scan8
                .as_ptr()
                .offset((32 as libc::c_int + 8 as libc::c_int) as isize) as isize,
        ) as *mut uint8_t as *mut x264_union32_t))
            .i = 0 as libc::c_int as uint32_t;
        (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
            *x264_scan8
                .as_ptr()
                .offset((32 as libc::c_int + 10 as libc::c_int) as isize) as isize,
        ) as *mut uint8_t as *mut x264_union32_t))
            .i = 0 as libc::c_int as uint32_t;
    }
    (*h).mb.i_cbp_luma = 0 as libc::c_int;
    (*h).mb.i_cbp_chroma = 0 as libc::c_int;
    *((*h).mb.cbp).offset((*h).mb.i_mb_xy as isize) = 0 as libc::c_int as int16_t;
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_predict_lossless_chroma(
    mut h: *mut x264_t,
    mut i_mode: libc::c_int,
) {
    let mut height: libc::c_int = 16 as libc::c_int >> (*h).mb.chroma_v_shift;
    if i_mode == I_PRED_CHROMA_V as libc::c_int {
        ((*h).mc.copy[PIXEL_8x8 as libc::c_int as usize]).expect("non-null function pointer")(
            (*h).mb.pic.p_fdec[1 as libc::c_int as usize],
            32 as libc::c_int as intptr_t,
            ((*h).mb.pic.p_fenc[1 as libc::c_int as usize]).offset(-(16 as libc::c_int as isize)),
            16 as libc::c_int as intptr_t,
            height,
        );
        ((*h).mc.copy[PIXEL_8x8 as libc::c_int as usize]).expect("non-null function pointer")(
            (*h).mb.pic.p_fdec[2 as libc::c_int as usize],
            32 as libc::c_int as intptr_t,
            ((*h).mb.pic.p_fenc[2 as libc::c_int as usize]).offset(-(16 as libc::c_int as isize)),
            16 as libc::c_int as intptr_t,
            height,
        );
        memcpy(
            (*h).mb.pic.p_fdec[1 as libc::c_int as usize] as *mut libc::c_void,
            ((*h).mb.pic.p_fdec[1 as libc::c_int as usize]).offset(-(32 as libc::c_int as isize))
                as *const libc::c_void,
            (8 as libc::c_int * ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int)
                as libc::c_ulong,
        );
        memcpy(
            (*h).mb.pic.p_fdec[2 as libc::c_int as usize] as *mut libc::c_void,
            ((*h).mb.pic.p_fdec[2 as libc::c_int as usize]).offset(-(32 as libc::c_int as isize))
                as *const libc::c_void,
            (8 as libc::c_int * ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int)
                as libc::c_ulong,
        );
    } else if i_mode == I_PRED_CHROMA_H as libc::c_int {
        ((*h).mc.copy[PIXEL_8x8 as libc::c_int as usize]).expect("non-null function pointer")(
            (*h).mb.pic.p_fdec[1 as libc::c_int as usize],
            32 as libc::c_int as intptr_t,
            ((*h).mb.pic.p_fenc[1 as libc::c_int as usize]).offset(-(1 as libc::c_int as isize)),
            16 as libc::c_int as intptr_t,
            height,
        );
        ((*h).mc.copy[PIXEL_8x8 as libc::c_int as usize]).expect("non-null function pointer")(
            (*h).mb.pic.p_fdec[2 as libc::c_int as usize],
            32 as libc::c_int as intptr_t,
            ((*h).mb.pic.p_fenc[2 as libc::c_int as usize]).offset(-(1 as libc::c_int as isize)),
            16 as libc::c_int as intptr_t,
            height,
        );
        x264_8_copy_column8(
            ((*h).mb.pic.p_fdec[1 as libc::c_int as usize])
                .offset((4 as libc::c_int * 32 as libc::c_int) as isize),
            ((*h).mb.pic.p_fdec[1 as libc::c_int as usize])
                .offset((4 as libc::c_int * 32 as libc::c_int) as isize)
                .offset(-(1 as libc::c_int as isize)),
        );
        x264_8_copy_column8(
            ((*h).mb.pic.p_fdec[2 as libc::c_int as usize])
                .offset((4 as libc::c_int * 32 as libc::c_int) as isize),
            ((*h).mb.pic.p_fdec[2 as libc::c_int as usize])
                .offset((4 as libc::c_int * 32 as libc::c_int) as isize)
                .offset(-(1 as libc::c_int as isize)),
        );
        if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_422 as libc::c_int {
            x264_8_copy_column8(
                ((*h).mb.pic.p_fdec[1 as libc::c_int as usize])
                    .offset((12 as libc::c_int * 32 as libc::c_int) as isize),
                ((*h).mb.pic.p_fdec[1 as libc::c_int as usize])
                    .offset((12 as libc::c_int * 32 as libc::c_int) as isize)
                    .offset(-(1 as libc::c_int as isize)),
            );
            x264_8_copy_column8(
                ((*h).mb.pic.p_fdec[2 as libc::c_int as usize])
                    .offset((12 as libc::c_int * 32 as libc::c_int) as isize),
                ((*h).mb.pic.p_fdec[2 as libc::c_int as usize])
                    .offset((12 as libc::c_int * 32 as libc::c_int) as isize)
                    .offset(-(1 as libc::c_int as isize)),
            );
        }
    } else {
        ((*h).predict_chroma[i_mode as usize]).expect("non-null function pointer")(
            (*h).mb.pic.p_fdec[1 as libc::c_int as usize],
        );
        ((*h).predict_chroma[i_mode as usize]).expect("non-null function pointer")(
            (*h).mb.pic.p_fdec[2 as libc::c_int as usize],
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_predict_lossless_4x4(
    mut h: *mut x264_t,
    mut p_dst: *mut pixel,
    mut p: libc::c_int,
    mut idx: libc::c_int,
    mut i_mode: libc::c_int,
) {
    let mut stride: libc::c_int = (*(*h).fenc).i_stride[p as usize] << (*h).mb.b_interlaced;
    let mut p_src: *mut pixel = ((*h).mb.pic.p_fenc_plane[p as usize])
        .offset((block_idx_x[idx as usize] as libc::c_int * 4 as libc::c_int) as isize)
        .offset((block_idx_y[idx as usize] as libc::c_int * 4 as libc::c_int * stride) as isize);
    if i_mode == I_PRED_4x4_V as libc::c_int {
        ((*h).mc.copy[PIXEL_4x4 as libc::c_int as usize]).expect("non-null function pointer")(
            p_dst,
            32 as libc::c_int as intptr_t,
            p_src.offset(-(stride as isize)),
            stride as intptr_t,
            4 as libc::c_int,
        );
        memcpy(
            p_dst as *mut libc::c_void,
            p_dst.offset(-(32 as libc::c_int as isize)) as *const libc::c_void,
            (4 as libc::c_int * ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int)
                as libc::c_ulong,
        );
    } else if i_mode == I_PRED_4x4_H as libc::c_int {
        ((*h).mc.copy[PIXEL_4x4 as libc::c_int as usize]).expect("non-null function pointer")(
            p_dst,
            32 as libc::c_int as intptr_t,
            p_src.offset(-(1 as libc::c_int as isize)),
            stride as intptr_t,
            4 as libc::c_int,
        );
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            *p_dst.offset((i * 32 as libc::c_int) as isize) =
                *p_dst.offset((i * 32 as libc::c_int - 1 as libc::c_int) as isize);
            i += 1;
            i;
        }
    } else {
        ((*h).predict_4x4[i_mode as usize]).expect("non-null function pointer")(p_dst);
    };
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_predict_lossless_8x8(
    mut h: *mut x264_t,
    mut p_dst: *mut pixel,
    mut p: libc::c_int,
    mut idx: libc::c_int,
    mut i_mode: libc::c_int,
    mut edge: *mut pixel,
) {
    let mut stride: libc::c_int = (*(*h).fenc).i_stride[p as usize] << (*h).mb.b_interlaced;
    let mut p_src: *mut pixel = ((*h).mb.pic.p_fenc_plane[p as usize])
        .offset(((idx & 1 as libc::c_int) * 8 as libc::c_int) as isize)
        .offset(((idx >> 1 as libc::c_int) * 8 as libc::c_int * stride) as isize);
    if i_mode == I_PRED_8x8_V as libc::c_int {
        ((*h).mc.copy[PIXEL_8x8 as libc::c_int as usize]).expect("non-null function pointer")(
            p_dst,
            32 as libc::c_int as intptr_t,
            p_src.offset(-(stride as isize)),
            stride as intptr_t,
            8 as libc::c_int,
        );
        memcpy(
            p_dst as *mut libc::c_void,
            &mut *edge.offset(16 as libc::c_int as isize) as *mut pixel as *const libc::c_void,
            (8 as libc::c_int * ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int)
                as libc::c_ulong,
        );
    } else if i_mode == I_PRED_8x8_H as libc::c_int {
        ((*h).mc.copy[PIXEL_8x8 as libc::c_int as usize]).expect("non-null function pointer")(
            p_dst,
            32 as libc::c_int as intptr_t,
            p_src.offset(-(1 as libc::c_int as isize)),
            stride as intptr_t,
            8 as libc::c_int,
        );
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < 8 as libc::c_int {
            *p_dst.offset((i * 32 as libc::c_int) as isize) =
                *edge.offset((14 as libc::c_int - i) as isize);
            i += 1;
            i;
        }
    } else {
        ((*h).predict_8x8[i_mode as usize]).expect("non-null function pointer")(p_dst, edge);
    };
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_predict_lossless_16x16(
    mut h: *mut x264_t,
    mut p: libc::c_int,
    mut i_mode: libc::c_int,
) {
    let mut stride: libc::c_int = (*(*h).fenc).i_stride[p as usize] << (*h).mb.b_interlaced;
    let mut p_dst: *mut pixel = (*h).mb.pic.p_fdec[p as usize];
    if i_mode == I_PRED_16x16_V as libc::c_int {
        ((*h).mc.copy[PIXEL_16x16 as libc::c_int as usize]).expect("non-null function pointer")(
            p_dst,
            32 as libc::c_int as intptr_t,
            ((*h).mb.pic.p_fenc_plane[p as usize]).offset(-(stride as isize)),
            stride as intptr_t,
            16 as libc::c_int,
        );
        memcpy(
            p_dst as *mut libc::c_void,
            p_dst.offset(-(32 as libc::c_int as isize)) as *const libc::c_void,
            (16 as libc::c_int * ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int)
                as libc::c_ulong,
        );
    } else if i_mode == I_PRED_16x16_H as libc::c_int {
        ((*h).mc.copy_16x16_unaligned).expect("non-null function pointer")(
            p_dst,
            32 as libc::c_int as intptr_t,
            ((*h).mb.pic.p_fenc_plane[p as usize]).offset(-(1 as libc::c_int as isize)),
            stride as intptr_t,
            16 as libc::c_int,
        );
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < 16 as libc::c_int {
            *p_dst.offset((i * 32 as libc::c_int) as isize) =
                *p_dst.offset((i * 32 as libc::c_int - 1 as libc::c_int) as isize);
            i += 1;
            i;
        }
    } else {
        ((*h).predict_16x16[i_mode as usize]).expect("non-null function pointer")(p_dst);
    };
}
#[inline(always)]
unsafe extern "C" fn macroblock_encode_internal(
    mut h: *mut x264_t,
    mut plane_count: libc::c_int,
    mut chroma: libc::c_int,
) {
    let mut i_qp: libc::c_int = (*h).mb.i_qp;
    let mut b_decimate: libc::c_int = (*h).mb.b_dct_decimate;
    let mut b_force_no_skip: libc::c_int = 0 as libc::c_int;
    let mut nz: libc::c_int = 0;
    (*h).mb.i_cbp_luma = 0 as libc::c_int;
    let mut p: libc::c_int = 0 as libc::c_int;
    while p < plane_count {
        (*h).mb.cache.non_zero_count[x264_scan8[(48 as libc::c_int + p) as usize] as usize] =
            0 as libc::c_int as uint8_t;
        p += 1;
        p;
    }
    if (*h).mb.i_type == I_PCM as libc::c_int {
        let mut p_0: libc::c_int = 0 as libc::c_int;
        while p_0 < plane_count {
            ((*h).mc.copy[PIXEL_16x16 as libc::c_int as usize]).expect("non-null function pointer")(
                (*h).mb.pic.p_fdec[p_0 as usize],
                32 as libc::c_int as intptr_t,
                (*h).mb.pic.p_fenc[p_0 as usize],
                16 as libc::c_int as intptr_t,
                16 as libc::c_int,
            );
            p_0 += 1;
            p_0;
        }
        if chroma != 0 {
            let mut height: libc::c_int = 16 as libc::c_int >> (*h).mb.chroma_v_shift;
            ((*h).mc.copy[PIXEL_8x8 as libc::c_int as usize]).expect("non-null function pointer")(
                (*h).mb.pic.p_fdec[1 as libc::c_int as usize],
                32 as libc::c_int as intptr_t,
                (*h).mb.pic.p_fenc[1 as libc::c_int as usize],
                16 as libc::c_int as intptr_t,
                height,
            );
            ((*h).mc.copy[PIXEL_8x8 as libc::c_int as usize]).expect("non-null function pointer")(
                (*h).mb.pic.p_fdec[2 as libc::c_int as usize],
                32 as libc::c_int as intptr_t,
                (*h).mb.pic.p_fenc[2 as libc::c_int as usize],
                16 as libc::c_int as intptr_t,
                height,
            );
        }
        return;
    }
    if (*h).mb.b_allow_skip == 0 {
        b_force_no_skip = 1 as libc::c_int;
        if (*h).mb.i_type == P_SKIP as libc::c_int || (*h).mb.i_type == B_SKIP as libc::c_int {
            if (*h).mb.i_type == P_SKIP as libc::c_int {
                (*h).mb.i_type = P_L0 as libc::c_int;
            } else if (*h).mb.i_type == B_SKIP as libc::c_int {
                (*h).mb.i_type = B_DIRECT as libc::c_int;
            }
        }
    }
    if (*h).mb.i_type == P_SKIP as libc::c_int {
        if (*h).mb.b_skip_mc == 0 {
            let mut mvx: libc::c_int = x264_clip3(
                (*h).mb.cache.mv[0 as libc::c_int as usize]
                    [x264_scan8[0 as libc::c_int as usize] as usize]
                    [0 as libc::c_int as usize] as libc::c_int,
                (*h).mb.mv_min[0 as libc::c_int as usize],
                (*h).mb.mv_max[0 as libc::c_int as usize],
            );
            let mut mvy: libc::c_int = x264_clip3(
                (*h).mb.cache.mv[0 as libc::c_int as usize]
                    [x264_scan8[0 as libc::c_int as usize] as usize]
                    [1 as libc::c_int as usize] as libc::c_int,
                (*h).mb.mv_min[1 as libc::c_int as usize],
                (*h).mb.mv_max[1 as libc::c_int as usize],
            );
            let mut p_1: libc::c_int = 0 as libc::c_int;
            while p_1 < plane_count {
                ((*h).mc.mc_luma).expect("non-null function pointer")(
                    (*h).mb.pic.p_fdec[p_1 as usize],
                    32 as libc::c_int as intptr_t,
                    &mut *(*(*((*h).mb.pic.p_fref)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize))
                    .as_mut_ptr()
                    .offset((p_1 * 4 as libc::c_int) as isize),
                    (*h).mb.pic.i_stride[p_1 as usize] as intptr_t,
                    mvx,
                    mvy,
                    16 as libc::c_int,
                    16 as libc::c_int,
                    &mut *(*((*h).sh.weight)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                    .as_mut_ptr()
                    .offset(p_1 as isize),
                );
                p_1 += 1;
                p_1;
            }
            if chroma != 0 {
                let mut v_shift: libc::c_int = (*h).mb.chroma_v_shift;
                let mut height_0: libc::c_int = 16 as libc::c_int >> v_shift;
                if mvx | mvy != 0 {
                    ((*h).mc.mc_chroma).expect("non-null function pointer")(
                        (*h).mb.pic.p_fdec[1 as libc::c_int as usize],
                        (*h).mb.pic.p_fdec[2 as libc::c_int as usize],
                        32 as libc::c_int as intptr_t,
                        (*h).mb.pic.p_fref[0 as libc::c_int as usize][0 as libc::c_int as usize]
                            [4 as libc::c_int as usize],
                        (*h).mb.pic.i_stride[1 as libc::c_int as usize] as intptr_t,
                        mvx,
                        (2 as libc::c_int * mvy) >> v_shift,
                        8 as libc::c_int,
                        height_0,
                    );
                } else {
                    ((*h).mc.load_deinterleave_chroma_fdec).expect("non-null function pointer")(
                        (*h).mb.pic.p_fdec[1 as libc::c_int as usize],
                        (*h).mb.pic.p_fref[0 as libc::c_int as usize][0 as libc::c_int as usize]
                            [4 as libc::c_int as usize],
                        (*h).mb.pic.i_stride[1 as libc::c_int as usize] as intptr_t,
                        height_0,
                    );
                }
                if !((*h).sh.weight[0 as libc::c_int as usize][1 as libc::c_int as usize].weightfn)
                    .is_null()
                {
                    (*((*h).sh.weight[0 as libc::c_int as usize][1 as libc::c_int as usize]
                        .weightfn)
                        .offset((8 as libc::c_int >> 2 as libc::c_int) as isize))
                    .expect("non-null function pointer")(
                        (*h).mb.pic.p_fdec[1 as libc::c_int as usize],
                        32 as libc::c_int as intptr_t,
                        (*h).mb.pic.p_fdec[1 as libc::c_int as usize],
                        32 as libc::c_int as intptr_t,
                        &mut *(*((*h).sh.weight)
                            .as_mut_ptr()
                            .offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize),
                        height_0,
                    );
                }
                if !((*h).sh.weight[0 as libc::c_int as usize][2 as libc::c_int as usize].weightfn)
                    .is_null()
                {
                    (*((*h).sh.weight[0 as libc::c_int as usize][2 as libc::c_int as usize]
                        .weightfn)
                        .offset((8 as libc::c_int >> 2 as libc::c_int) as isize))
                    .expect("non-null function pointer")(
                        (*h).mb.pic.p_fdec[2 as libc::c_int as usize],
                        32 as libc::c_int as intptr_t,
                        (*h).mb.pic.p_fdec[2 as libc::c_int as usize],
                        32 as libc::c_int as intptr_t,
                        &mut *(*((*h).sh.weight)
                            .as_mut_ptr()
                            .offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(2 as libc::c_int as isize),
                        height_0,
                    );
                }
            }
        }
        macroblock_encode_skip(h);
        return;
    }
    if (*h).mb.i_type == B_SKIP as libc::c_int {
        if (*h).mb.b_skip_mc == 0 {
            x264_8_mb_mc(h);
        }
        macroblock_encode_skip(h);
        return;
    }
    if (*h).mb.i_type == I_16x16 as libc::c_int {
        (*h).mb.b_transform_8x8 = 0 as libc::c_int;
        let mut p_2: libc::c_int = 0 as libc::c_int;
        while p_2 < plane_count {
            mb_encode_i16x16(h, p_2, i_qp);
            p_2 += 1;
            p_2;
            i_qp = (*h).mb.i_chroma_qp;
        }
    } else if (*h).mb.i_type == I_8x8 as libc::c_int {
        (*h).mb.b_transform_8x8 = 1 as libc::c_int;
        if (*h).mb.i_skip_intra != 0 {
            ((*h).mc.copy[PIXEL_16x16 as libc::c_int as usize]).expect("non-null function pointer")(
                (*h).mb.pic.p_fdec[0 as libc::c_int as usize],
                32 as libc::c_int as intptr_t,
                ((*h).mb.pic.i8x8_fdec_buf).as_mut_ptr(),
                16 as libc::c_int as intptr_t,
                16 as libc::c_int,
            );
            (*(&mut *((*h).mb.cache.non_zero_count)
                .as_mut_ptr()
                .offset(*x264_scan8.as_ptr().offset(0 as libc::c_int as isize) as isize)
                as *mut uint8_t as *mut x264_union32_t))
                .i = (*h).mb.pic.i8x8_nnz_buf[0 as libc::c_int as usize];
            (*(&mut *((*h).mb.cache.non_zero_count)
                .as_mut_ptr()
                .offset(*x264_scan8.as_ptr().offset(2 as libc::c_int as isize) as isize)
                as *mut uint8_t as *mut x264_union32_t))
                .i = (*h).mb.pic.i8x8_nnz_buf[1 as libc::c_int as usize];
            (*(&mut *((*h).mb.cache.non_zero_count)
                .as_mut_ptr()
                .offset(*x264_scan8.as_ptr().offset(8 as libc::c_int as isize) as isize)
                as *mut uint8_t as *mut x264_union32_t))
                .i = (*h).mb.pic.i8x8_nnz_buf[2 as libc::c_int as usize];
            (*(&mut *((*h).mb.cache.non_zero_count)
                .as_mut_ptr()
                .offset(*x264_scan8.as_ptr().offset(10 as libc::c_int as isize) as isize)
                as *mut uint8_t as *mut x264_union32_t))
                .i = (*h).mb.pic.i8x8_nnz_buf[3 as libc::c_int as usize];
            (*h).mb.i_cbp_luma = (*h).mb.pic.i8x8_cbp;
            if (*h).mb.i_skip_intra == 2 as libc::c_int {
                ((*h).mc.memcpy_aligned).expect("non-null function pointer")(
                    ((*h).dct.luma8x8).as_mut_ptr() as *mut libc::c_void,
                    ((*h).mb.pic.i8x8_dct_buf).as_mut_ptr() as *const libc::c_void,
                    ::core::mem::size_of::<[[dctcoef; 64]; 3]>() as libc::c_ulong,
                );
            }
        }
        let mut p_3: libc::c_int = 0 as libc::c_int;
        while p_3 < plane_count {
            let mut i: libc::c_int = if p_3 == 0 as libc::c_int && (*h).mb.i_skip_intra != 0 {
                3 as libc::c_int
            } else {
                0 as libc::c_int
            };
            while i < 4 as libc::c_int {
                let mut i_mode: libc::c_int = (*h).mb.cache.intra4x4_pred_mode
                    [x264_scan8[(4 as libc::c_int * i) as usize] as usize]
                    as libc::c_int;
                x264_mb_encode_i8x8(
                    h,
                    p_3,
                    i,
                    i_qp,
                    i_mode,
                    std::ptr::null_mut::<pixel>(),
                    1 as libc::c_int,
                );
                i += 1;
                i;
            }
            p_3 += 1;
            p_3;
            i_qp = (*h).mb.i_chroma_qp;
        }
    } else if (*h).mb.i_type == I_4x4 as libc::c_int {
        (*h).mb.b_transform_8x8 = 0 as libc::c_int;
        if (*h).mb.i_skip_intra != 0 {
            ((*h).mc.copy[PIXEL_16x16 as libc::c_int as usize]).expect("non-null function pointer")(
                (*h).mb.pic.p_fdec[0 as libc::c_int as usize],
                32 as libc::c_int as intptr_t,
                ((*h).mb.pic.i4x4_fdec_buf).as_mut_ptr(),
                16 as libc::c_int as intptr_t,
                16 as libc::c_int,
            );
            (*(&mut *((*h).mb.cache.non_zero_count)
                .as_mut_ptr()
                .offset(*x264_scan8.as_ptr().offset(0 as libc::c_int as isize) as isize)
                as *mut uint8_t as *mut x264_union32_t))
                .i = (*h).mb.pic.i4x4_nnz_buf[0 as libc::c_int as usize];
            (*(&mut *((*h).mb.cache.non_zero_count)
                .as_mut_ptr()
                .offset(*x264_scan8.as_ptr().offset(2 as libc::c_int as isize) as isize)
                as *mut uint8_t as *mut x264_union32_t))
                .i = (*h).mb.pic.i4x4_nnz_buf[1 as libc::c_int as usize];
            (*(&mut *((*h).mb.cache.non_zero_count)
                .as_mut_ptr()
                .offset(*x264_scan8.as_ptr().offset(8 as libc::c_int as isize) as isize)
                as *mut uint8_t as *mut x264_union32_t))
                .i = (*h).mb.pic.i4x4_nnz_buf[2 as libc::c_int as usize];
            (*(&mut *((*h).mb.cache.non_zero_count)
                .as_mut_ptr()
                .offset(*x264_scan8.as_ptr().offset(10 as libc::c_int as isize) as isize)
                as *mut uint8_t as *mut x264_union32_t))
                .i = (*h).mb.pic.i4x4_nnz_buf[3 as libc::c_int as usize];
            (*h).mb.i_cbp_luma = (*h).mb.pic.i4x4_cbp;
            if (*h).mb.i_skip_intra == 2 as libc::c_int {
                ((*h).mc.memcpy_aligned).expect("non-null function pointer")(
                    ((*h).dct.luma4x4).as_mut_ptr() as *mut libc::c_void,
                    ((*h).mb.pic.i4x4_dct_buf).as_mut_ptr() as *const libc::c_void,
                    ::core::mem::size_of::<[[dctcoef; 16]; 15]>() as libc::c_ulong,
                );
            }
        }
        let mut p_4: libc::c_int = 0 as libc::c_int;
        while p_4 < plane_count {
            let mut i_0: libc::c_int = if p_4 == 0 as libc::c_int && (*h).mb.i_skip_intra != 0 {
                15 as libc::c_int
            } else {
                0 as libc::c_int
            };
            while i_0 < 16 as libc::c_int {
                let mut p_dst: *mut pixel =
                    &mut *(*((*h).mb.pic.p_fdec).as_mut_ptr().offset(p_4 as isize))
                        .offset(*block_idx_xy_fdec.as_ptr().offset(i_0 as isize) as isize)
                        as *mut pixel;
                let mut i_mode_0: libc::c_int = (*h).mb.cache.intra4x4_pred_mode
                    [x264_scan8[i_0 as usize] as usize]
                    as libc::c_int;
                if (*h).mb.i_neighbour4[i_0 as usize]
                    & (MB_TOPRIGHT as libc::c_int | MB_TOP as libc::c_int) as libc::c_uint
                    == MB_TOP as libc::c_int as libc::c_uint
                {
                    (*(&mut *p_dst.offset((4 as libc::c_int - 32 as libc::c_int) as isize)
                        as *mut pixel as *mut x264_union32_t))
                        .i = (*p_dst.offset((3 as libc::c_int - 32 as libc::c_int) as isize)
                        as libc::c_uint)
                        .wrapping_mul(0x1010101 as libc::c_uint);
                }
                x264_mb_encode_i4x4(h, p_4, i_0, i_qp, i_mode_0, 1 as libc::c_int);
                i_0 += 1;
                i_0;
            }
            p_4 += 1;
            p_4;
            i_qp = (*h).mb.i_chroma_qp;
        }
    } else {
        let mut i_decimate_mb: libc::c_int = 0 as libc::c_int;
        if (*h).mb.b_skip_mc == 0 {
            x264_8_mb_mc(h);
        }
        if (*h).mb.b_lossless != 0 {
            if (*h).mb.b_transform_8x8 != 0 {
                let mut p_5: libc::c_int = 0 as libc::c_int;
                while p_5 < plane_count {
                    let mut i8x8: libc::c_int = 0 as libc::c_int;
                    while i8x8 < 4 as libc::c_int {
                        let mut x: libc::c_int = i8x8 & 1 as libc::c_int;
                        let mut y: libc::c_int = i8x8 >> 1 as libc::c_int;
                        nz = ((*h).zigzagf.sub_8x8).expect("non-null function pointer")(
                            ((*h).dct.luma8x8[(p_5 * 4 as libc::c_int + i8x8) as usize])
                                .as_mut_ptr(),
                            ((*h).mb.pic.p_fenc[p_5 as usize])
                                .offset((8 as libc::c_int * x) as isize)
                                .offset((8 as libc::c_int * y * 16 as libc::c_int) as isize),
                            ((*h).mb.pic.p_fdec[p_5 as usize])
                                .offset((8 as libc::c_int * x) as isize)
                                .offset((8 as libc::c_int * y * 32 as libc::c_int) as isize),
                        );
                        (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
                            (*x264_scan8.as_ptr().offset(
                                (p_5 * 16 as libc::c_int + i8x8 * 4 as libc::c_int) as isize,
                            ) as libc::c_int
                                + 0 as libc::c_int) as isize,
                        ) as *mut uint8_t as *mut x264_union16_t))
                            .i = (nz * 0x101 as libc::c_int) as uint16_t;
                        (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
                            (*x264_scan8.as_ptr().offset(
                                (p_5 * 16 as libc::c_int + i8x8 * 4 as libc::c_int) as isize,
                            ) as libc::c_int
                                + 8 as libc::c_int) as isize,
                        ) as *mut uint8_t as *mut x264_union16_t))
                            .i = (nz * 0x101 as libc::c_int) as uint16_t;
                        (*h).mb.i_cbp_luma |= nz << i8x8;
                        i8x8 += 1;
                        i8x8;
                    }
                    p_5 += 1;
                    p_5;
                }
            } else {
                let mut p_6: libc::c_int = 0 as libc::c_int;
                while p_6 < plane_count {
                    let mut i4x4: libc::c_int = 0 as libc::c_int;
                    while i4x4 < 16 as libc::c_int {
                        nz = ((*h).zigzagf.sub_4x4).expect("non-null function pointer")(
                            ((*h).dct.luma4x4[(p_6 * 16 as libc::c_int + i4x4) as usize])
                                .as_mut_ptr(),
                            ((*h).mb.pic.p_fenc[p_6 as usize])
                                .offset(block_idx_xy_fenc[i4x4 as usize] as libc::c_int as isize),
                            ((*h).mb.pic.p_fdec[p_6 as usize])
                                .offset(block_idx_xy_fdec[i4x4 as usize] as libc::c_int as isize),
                        );
                        (*h).mb.cache.non_zero_count
                            [x264_scan8[(p_6 * 16 as libc::c_int + i4x4) as usize] as usize] =
                            nz as uint8_t;
                        (*h).mb.i_cbp_luma |= nz << (i4x4 >> 2 as libc::c_int);
                        i4x4 += 1;
                        i4x4;
                    }
                    p_6 += 1;
                    p_6;
                }
            }
        } else if (*h).mb.b_transform_8x8 != 0 {
            let mut dct8x8: [[dctcoef; 64]; 4] = [[0; 64]; 4];
            b_decimate &= ((*h).mb.b_trellis == 0 || (*h).param.b_cabac == 0) as libc::c_int;
            let mut p_7: libc::c_int = 0 as libc::c_int;
            while p_7 < plane_count {
                let mut quant_cat: libc::c_int = if p_7 != 0 {
                    CQM_8PC as libc::c_int
                } else {
                    CQM_8PY as libc::c_int
                };
                (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
                    (*x264_scan8
                        .as_ptr()
                        .offset((16 as libc::c_int * p_7) as isize)
                        as libc::c_int
                        + 0 as libc::c_int * 8 as libc::c_int) as isize,
                ) as *mut uint8_t as *mut x264_union32_t))
                    .i = 0 as libc::c_int as uint32_t;
                (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
                    (*x264_scan8
                        .as_ptr()
                        .offset((16 as libc::c_int * p_7) as isize)
                        as libc::c_int
                        + 1 as libc::c_int * 8 as libc::c_int) as isize,
                ) as *mut uint8_t as *mut x264_union32_t))
                    .i = 0 as libc::c_int as uint32_t;
                (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
                    (*x264_scan8
                        .as_ptr()
                        .offset((16 as libc::c_int * p_7) as isize)
                        as libc::c_int
                        + 2 as libc::c_int * 8 as libc::c_int) as isize,
                ) as *mut uint8_t as *mut x264_union32_t))
                    .i = 0 as libc::c_int as uint32_t;
                (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
                    (*x264_scan8
                        .as_ptr()
                        .offset((16 as libc::c_int * p_7) as isize)
                        as libc::c_int
                        + 3 as libc::c_int * 8 as libc::c_int) as isize,
                ) as *mut uint8_t as *mut x264_union32_t))
                    .i = 0 as libc::c_int as uint32_t;
                ((*h).dctf.sub16x16_dct8).expect("non-null function pointer")(
                    dct8x8.as_mut_ptr(),
                    (*h).mb.pic.p_fenc[p_7 as usize],
                    (*h).mb.pic.p_fdec[p_7 as usize],
                );
                let fresh1 = &mut (*((*h).nr_count).offset(
                    (1 as libc::c_int + (p_7 != 0) as libc::c_int * 2 as libc::c_int) as isize,
                ));
                *fresh1 = (*fresh1)
                    .wrapping_add(((*h).mb.b_noise_reduction * 4 as libc::c_int) as uint32_t);
                let mut plane_cbp: libc::c_int = 0 as libc::c_int;
                let mut idx: libc::c_int = 0 as libc::c_int;
                while idx < 4 as libc::c_int {
                    nz = x264_quant_8x8(
                        h,
                        (dct8x8[idx as usize]).as_mut_ptr(),
                        i_qp,
                        ctx_cat_plane[DCT_LUMA_8x8 as libc::c_int as usize][p_7 as usize]
                            as libc::c_int,
                        0 as libc::c_int,
                        p_7,
                        idx,
                    );
                    if nz != 0 {
                        ((*h).zigzagf.scan_8x8).expect("non-null function pointer")(
                            ((*h).dct.luma8x8[(p_7 * 4 as libc::c_int + idx) as usize])
                                .as_mut_ptr(),
                            (dct8x8[idx as usize]).as_mut_ptr(),
                        );
                        if b_decimate != 0 {
                            let mut i_decimate_8x8: libc::c_int = ((*h).quantf.decimate_score64)
                                .expect("non-null function pointer")(
                                ((*h).dct.luma8x8[(p_7 * 4 as libc::c_int + idx) as usize])
                                    .as_mut_ptr(),
                            );
                            i_decimate_mb += i_decimate_8x8;
                            if i_decimate_8x8 >= 4 as libc::c_int {
                                plane_cbp |= (1 as libc::c_int) << idx;
                            }
                        } else {
                            plane_cbp |= (1 as libc::c_int) << idx;
                        }
                    }
                    idx += 1;
                    idx;
                }
                if i_decimate_mb >= 6 as libc::c_int || b_decimate == 0 {
                    (*h).mb.i_cbp_luma |= plane_cbp;
                    let mut idx_0: libc::c_int = 0 as libc::c_int;
                    let mut msk: libc::c_int = plane_cbp;
                    let mut skip: libc::c_int = 0;
                    while msk != 0 && {
                        skip = x264_ctz_4bit(msk as uint32_t);
                        idx_0 += skip;
                        msk >>= skip + 1 as libc::c_int;
                        1 as libc::c_int != 0
                    } {
                        ((*h).quantf.dequant_8x8).expect("non-null function pointer")(
                            (dct8x8[idx_0 as usize]).as_mut_ptr(),
                            (*h).dequant8_mf[quant_cat as usize],
                            i_qp,
                        );
                        ((*h).dctf.add8x8_idct8).expect("non-null function pointer")(
                            &mut *(*((*h).mb.pic.p_fdec).as_mut_ptr().offset(p_7 as isize)).offset(
                                (8 as libc::c_int * (idx_0 & 1 as libc::c_int)
                                    + 8 as libc::c_int
                                        * (idx_0 >> 1 as libc::c_int)
                                        * 32 as libc::c_int)
                                    as isize,
                            ),
                            (dct8x8[idx_0 as usize]).as_mut_ptr(),
                        );
                        (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
                            (*x264_scan8.as_ptr().offset(
                                (p_7 * 16 as libc::c_int + idx_0 * 4 as libc::c_int) as isize,
                            ) as libc::c_int
                                + 0 as libc::c_int) as isize,
                        ) as *mut uint8_t as *mut x264_union16_t))
                            .i = (1 as libc::c_int * 0x101 as libc::c_int) as uint16_t;
                        (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
                            (*x264_scan8.as_ptr().offset(
                                (p_7 * 16 as libc::c_int + idx_0 * 4 as libc::c_int) as isize,
                            ) as libc::c_int
                                + 8 as libc::c_int) as isize,
                        ) as *mut uint8_t as *mut x264_union16_t))
                            .i = (1 as libc::c_int * 0x101 as libc::c_int) as uint16_t;
                        idx_0 += 1;
                        idx_0;
                    }
                }
                p_7 += 1;
                p_7;
                i_qp = (*h).mb.i_chroma_qp;
            }
        } else {
            let mut dct4x4: [[dctcoef; 16]; 16] = [[0; 16]; 16];
            let mut p_8: libc::c_int = 0 as libc::c_int;
            while p_8 < plane_count {
                let mut quant_cat_0: libc::c_int = if p_8 != 0 {
                    CQM_4PC as libc::c_int
                } else {
                    CQM_4PY as libc::c_int
                };
                (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
                    (*x264_scan8
                        .as_ptr()
                        .offset((16 as libc::c_int * p_8) as isize)
                        as libc::c_int
                        + 0 as libc::c_int * 8 as libc::c_int) as isize,
                ) as *mut uint8_t as *mut x264_union32_t))
                    .i = 0 as libc::c_int as uint32_t;
                (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
                    (*x264_scan8
                        .as_ptr()
                        .offset((16 as libc::c_int * p_8) as isize)
                        as libc::c_int
                        + 1 as libc::c_int * 8 as libc::c_int) as isize,
                ) as *mut uint8_t as *mut x264_union32_t))
                    .i = 0 as libc::c_int as uint32_t;
                (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
                    (*x264_scan8
                        .as_ptr()
                        .offset((16 as libc::c_int * p_8) as isize)
                        as libc::c_int
                        + 2 as libc::c_int * 8 as libc::c_int) as isize,
                ) as *mut uint8_t as *mut x264_union32_t))
                    .i = 0 as libc::c_int as uint32_t;
                (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
                    (*x264_scan8
                        .as_ptr()
                        .offset((16 as libc::c_int * p_8) as isize)
                        as libc::c_int
                        + 3 as libc::c_int * 8 as libc::c_int) as isize,
                ) as *mut uint8_t as *mut x264_union32_t))
                    .i = 0 as libc::c_int as uint32_t;
                ((*h).dctf.sub16x16_dct).expect("non-null function pointer")(
                    dct4x4.as_mut_ptr(),
                    (*h).mb.pic.p_fenc[p_8 as usize],
                    (*h).mb.pic.p_fdec[p_8 as usize],
                );
                if (*h).mb.b_noise_reduction != 0 {
                    let fresh2 = &mut (*((*h).nr_count).offset(
                        (0 as libc::c_int + (p_8 != 0) as libc::c_int * 2 as libc::c_int) as isize,
                    ));
                    *fresh2 = (*fresh2).wrapping_add(16 as libc::c_int as uint32_t);
                    let mut idx_1: libc::c_int = 0 as libc::c_int;
                    while idx_1 < 16 as libc::c_int {
                        ((*h).quantf.denoise_dct).expect("non-null function pointer")(
                            (dct4x4[idx_1 as usize]).as_mut_ptr(),
                            (*((*h).nr_residual_sum).offset(
                                (0 as libc::c_int + (p_8 != 0) as libc::c_int * 2 as libc::c_int)
                                    as isize,
                            ))
                            .as_mut_ptr(),
                            (*((*h).nr_offset).offset(
                                (0 as libc::c_int + (p_8 != 0) as libc::c_int * 2 as libc::c_int)
                                    as isize,
                            ))
                            .as_mut_ptr(),
                            16 as libc::c_int,
                        );
                        idx_1 += 1;
                        idx_1;
                    }
                }
                let mut plane_cbp_0: libc::c_int = 0 as libc::c_int;
                let mut i8x8_0: libc::c_int = 0 as libc::c_int;
                while i8x8_0 < 4 as libc::c_int {
                    let mut i_decimate_8x8_0: libc::c_int = if b_decimate != 0 {
                        0 as libc::c_int
                    } else {
                        6 as libc::c_int
                    };
                    let mut nnz8x8: libc::c_int = 0 as libc::c_int;
                    if (*h).mb.b_trellis != 0 {
                        let mut i4x4_0: libc::c_int = 0 as libc::c_int;
                        while i4x4_0 < 4 as libc::c_int {
                            let mut idx_2: libc::c_int = i8x8_0 * 4 as libc::c_int + i4x4_0;
                            if x264_8_quant_4x4_trellis(
                                h,
                                (dct4x4[idx_2 as usize]).as_mut_ptr(),
                                quant_cat_0,
                                i_qp,
                                ctx_cat_plane[DCT_LUMA_4x4 as libc::c_int as usize][p_8 as usize]
                                    as libc::c_int,
                                0 as libc::c_int,
                                (p_8 != 0) as libc::c_int,
                                p_8 * 16 as libc::c_int + idx_2,
                            ) != 0
                            {
                                ((*h).zigzagf.scan_4x4).expect("non-null function pointer")(
                                    ((*h).dct.luma4x4[(p_8 * 16 as libc::c_int + idx_2) as usize])
                                        .as_mut_ptr(),
                                    (dct4x4[idx_2 as usize]).as_mut_ptr(),
                                );
                                ((*h).quantf.dequant_4x4).expect("non-null function pointer")(
                                    (dct4x4[idx_2 as usize]).as_mut_ptr(),
                                    (*h).dequant4_mf[quant_cat_0 as usize],
                                    i_qp,
                                );
                                if i_decimate_8x8_0 < 6 as libc::c_int {
                                    i_decimate_8x8_0 += ((*h).quantf.decimate_score16)
                                        .expect("non-null function pointer")(
                                        ((*h).dct.luma4x4
                                            [(p_8 * 16 as libc::c_int + idx_2) as usize])
                                            .as_mut_ptr(),
                                    );
                                }
                                (*h).mb.cache.non_zero_count[x264_scan8
                                    [(p_8 * 16 as libc::c_int + idx_2) as usize]
                                    as usize] = 1 as libc::c_int as uint8_t;
                                nnz8x8 = 1 as libc::c_int;
                            }
                            i4x4_0 += 1;
                            i4x4_0;
                        }
                    } else {
                        nz = ((*h).quantf.quant_4x4x4).expect("non-null function pointer")(
                            &mut *dct4x4
                                .as_mut_ptr()
                                .offset((i8x8_0 * 4 as libc::c_int) as isize),
                            (*((*h).quant4_mf[quant_cat_0 as usize]).offset(i_qp as isize))
                                .as_mut_ptr(),
                            (*((*h).quant4_bias[quant_cat_0 as usize]).offset(i_qp as isize))
                                .as_mut_ptr(),
                        );
                        nnz8x8 = nz;
                        if nz != 0 {
                            let mut idx_3: libc::c_int = i8x8_0 * 4 as libc::c_int;
                            let mut msk_0: libc::c_int = nz;
                            let mut skip_0: libc::c_int = 0;
                            while msk_0 != 0 && {
                                skip_0 = x264_ctz_4bit(msk_0 as uint32_t);
                                idx_3 += skip_0;
                                msk_0 >>= skip_0 + 1 as libc::c_int;
                                1 as libc::c_int != 0
                            } {
                                ((*h).zigzagf.scan_4x4).expect("non-null function pointer")(
                                    ((*h).dct.luma4x4[(p_8 * 16 as libc::c_int + idx_3) as usize])
                                        .as_mut_ptr(),
                                    (dct4x4[idx_3 as usize]).as_mut_ptr(),
                                );
                                ((*h).quantf.dequant_4x4).expect("non-null function pointer")(
                                    (dct4x4[idx_3 as usize]).as_mut_ptr(),
                                    (*h).dequant4_mf[quant_cat_0 as usize],
                                    i_qp,
                                );
                                if i_decimate_8x8_0 < 6 as libc::c_int {
                                    i_decimate_8x8_0 += ((*h).quantf.decimate_score16)
                                        .expect("non-null function pointer")(
                                        ((*h).dct.luma4x4
                                            [(p_8 * 16 as libc::c_int + idx_3) as usize])
                                            .as_mut_ptr(),
                                    );
                                }
                                (*h).mb.cache.non_zero_count[x264_scan8
                                    [(p_8 * 16 as libc::c_int + idx_3) as usize]
                                    as usize] = 1 as libc::c_int as uint8_t;
                                idx_3 += 1;
                                idx_3;
                            }
                        }
                    }
                    if nnz8x8 != 0 {
                        i_decimate_mb += i_decimate_8x8_0;
                        if i_decimate_8x8_0 < 4 as libc::c_int {
                            (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
                                (*x264_scan8.as_ptr().offset(
                                    (p_8 * 16 as libc::c_int + i8x8_0 * 4 as libc::c_int) as isize,
                                ) as libc::c_int
                                    + 0 as libc::c_int) as isize,
                            ) as *mut uint8_t
                                as *mut x264_union16_t))
                                .i = (0 as libc::c_int * 0x101 as libc::c_int) as uint16_t;
                            (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
                                (*x264_scan8.as_ptr().offset(
                                    (p_8 * 16 as libc::c_int + i8x8_0 * 4 as libc::c_int) as isize,
                                ) as libc::c_int
                                    + 8 as libc::c_int) as isize,
                            ) as *mut uint8_t
                                as *mut x264_union16_t))
                                .i = (0 as libc::c_int * 0x101 as libc::c_int) as uint16_t;
                        } else {
                            plane_cbp_0 |= (1 as libc::c_int) << i8x8_0;
                        }
                    }
                    i8x8_0 += 1;
                    i8x8_0;
                }
                if i_decimate_mb < 6 as libc::c_int {
                    plane_cbp_0 = 0 as libc::c_int;
                    (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
                        (*x264_scan8
                            .as_ptr()
                            .offset((16 as libc::c_int * p_8) as isize)
                            as libc::c_int
                            + 0 as libc::c_int * 8 as libc::c_int) as isize,
                    ) as *mut uint8_t as *mut x264_union32_t))
                        .i = 0 as libc::c_int as uint32_t;
                    (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
                        (*x264_scan8
                            .as_ptr()
                            .offset((16 as libc::c_int * p_8) as isize)
                            as libc::c_int
                            + 1 as libc::c_int * 8 as libc::c_int) as isize,
                    ) as *mut uint8_t as *mut x264_union32_t))
                        .i = 0 as libc::c_int as uint32_t;
                    (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
                        (*x264_scan8
                            .as_ptr()
                            .offset((16 as libc::c_int * p_8) as isize)
                            as libc::c_int
                            + 2 as libc::c_int * 8 as libc::c_int) as isize,
                    ) as *mut uint8_t as *mut x264_union32_t))
                        .i = 0 as libc::c_int as uint32_t;
                    (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
                        (*x264_scan8
                            .as_ptr()
                            .offset((16 as libc::c_int * p_8) as isize)
                            as libc::c_int
                            + 3 as libc::c_int * 8 as libc::c_int) as isize,
                    ) as *mut uint8_t as *mut x264_union32_t))
                        .i = 0 as libc::c_int as uint32_t;
                } else {
                    (*h).mb.i_cbp_luma |= plane_cbp_0;
                    let mut i8x8_1: libc::c_int = 0 as libc::c_int;
                    let mut msk_1: libc::c_int = plane_cbp_0;
                    let mut skip_1: libc::c_int = 0;
                    while msk_1 != 0 && {
                        skip_1 = x264_ctz_4bit(msk_1 as uint32_t);
                        i8x8_1 += skip_1;
                        msk_1 >>= skip_1 + 1 as libc::c_int;
                        1 as libc::c_int != 0
                    } {
                        ((*h).dctf.add8x8_idct).expect("non-null function pointer")(
                            &mut *(*((*h).mb.pic.p_fdec).as_mut_ptr().offset(p_8 as isize)).offset(
                                ((i8x8_1 & 1 as libc::c_int) * 8 as libc::c_int
                                    + (i8x8_1 >> 1 as libc::c_int)
                                        * 8 as libc::c_int
                                        * 32 as libc::c_int)
                                    as isize,
                            ),
                            &mut *dct4x4
                                .as_mut_ptr()
                                .offset((i8x8_1 * 4 as libc::c_int) as isize),
                        );
                        i8x8_1 += 1;
                        i8x8_1;
                    }
                }
                p_8 += 1;
                p_8;
                i_qp = (*h).mb.i_chroma_qp;
            }
        }
    }
    if chroma != 0 {
        if (*h).mb.i_type == I_4x4 as libc::c_int
            || (*h).mb.i_type == I_8x8 as libc::c_int
            || (*h).mb.i_type == I_16x16 as libc::c_int
            || (*h).mb.i_type == I_PCM as libc::c_int
        {
            let mut i_mode_1: libc::c_int = (*h).mb.i_chroma_pred_mode;
            if (*h).mb.b_lossless != 0 {
                x264_8_predict_lossless_chroma(h, i_mode_1);
            } else {
                ((*h).predict_chroma[i_mode_1 as usize]).expect("non-null function pointer")(
                    (*h).mb.pic.p_fdec[1 as libc::c_int as usize],
                );
                ((*h).predict_chroma[i_mode_1 as usize]).expect("non-null function pointer")(
                    (*h).mb.pic.p_fdec[2 as libc::c_int as usize],
                );
            }
        }
        x264_8_mb_encode_chroma(
            h,
            !((*h).mb.i_type == I_4x4 as libc::c_int
                || (*h).mb.i_type == I_8x8 as libc::c_int
                || (*h).mb.i_type == I_16x16 as libc::c_int
                || (*h).mb.i_type == I_PCM as libc::c_int) as libc::c_int,
            (*h).mb.i_chroma_qp,
        );
    } else {
        (*h).mb.i_cbp_chroma = 0 as libc::c_int;
    }
    let mut cbp: libc::c_int = ((*h).mb.i_cbp_chroma << 4 as libc::c_int) | (*h).mb.i_cbp_luma;
    if (*h).param.b_cabac != 0 {
        cbp |= (((*h).mb.cache.non_zero_count[x264_scan8[48 as libc::c_int as usize] as usize]
            as libc::c_int)
            << 8 as libc::c_int)
            | (((*h).mb.cache.non_zero_count
                [x264_scan8[(49 as libc::c_int + 0 as libc::c_int) as usize] as usize]
                as libc::c_int)
                << 9 as libc::c_int)
            | (((*h).mb.cache.non_zero_count
                [x264_scan8[(49 as libc::c_int + 1 as libc::c_int) as usize] as usize]
                as libc::c_int)
                << 10 as libc::c_int);
    }
    *((*h).mb.cbp).offset((*h).mb.i_mb_xy as isize) = cbp as int16_t;
    if b_force_no_skip == 0 {
        if (*h).mb.i_type == P_L0 as libc::c_int
            && (*h).mb.i_partition == D_16x16 as libc::c_int
            && (*h).mb.i_cbp_luma | (*h).mb.i_cbp_chroma == 0
            && (*(((*h).mb.cache.mv[0 as libc::c_int as usize]
                [x264_scan8[0 as libc::c_int as usize] as usize])
                .as_mut_ptr() as *mut x264_union32_t))
                .i
                == (*(((*h).mb.cache.pskip_mv).as_mut_ptr() as *mut x264_union32_t)).i
            && (*h).mb.cache.ref_0[0 as libc::c_int as usize]
                [x264_scan8[0 as libc::c_int as usize] as usize] as libc::c_int
                == 0 as libc::c_int
        {
            (*h).mb.i_type = P_SKIP as libc::c_int;
        }
        if (*h).mb.i_type == B_DIRECT as libc::c_int
            && (*h).mb.i_cbp_luma | (*h).mb.i_cbp_chroma == 0
        {
            (*h).mb.i_type = B_SKIP as libc::c_int;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_macroblock_encode(mut h: *mut x264_t) {
    if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as libc::c_int {
        macroblock_encode_internal(h, 3 as libc::c_int, 0 as libc::c_int);
    } else if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc != 0 {
        macroblock_encode_internal(h, 1 as libc::c_int, 1 as libc::c_int);
    } else {
        macroblock_encode_internal(h, 1 as libc::c_int, 0 as libc::c_int);
    };
}
#[inline(always)]
unsafe extern "C" fn macroblock_probe_skip_internal(
    mut h: *mut x264_t,
    mut b_bidir: libc::c_int,
    mut plane_count: libc::c_int,
    mut chroma: libc::c_int,
) -> libc::c_int {
    let mut dct4x4: [[dctcoef; 16]; 8] = [[0; 16]; 8];
    let mut dctscan: [dctcoef; 16] = [0; 16];
    let mut mvp: [int16_t; 2] = [0; 2];
    let mut i_qp: libc::c_int = (*h).mb.i_qp;
    let mut p: libc::c_int = 0 as libc::c_int;
    while p < plane_count {
        let mut quant_cat: libc::c_int = if p != 0 {
            CQM_4PC as libc::c_int
        } else {
            CQM_4PY as libc::c_int
        };
        if b_bidir == 0 {
            mvp[0 as libc::c_int as usize] = x264_clip3(
                (*h).mb.cache.pskip_mv[0 as libc::c_int as usize] as libc::c_int,
                (*h).mb.mv_min[0 as libc::c_int as usize],
                (*h).mb.mv_max[0 as libc::c_int as usize],
            ) as int16_t;
            mvp[1 as libc::c_int as usize] = x264_clip3(
                (*h).mb.cache.pskip_mv[1 as libc::c_int as usize] as libc::c_int,
                (*h).mb.mv_min[1 as libc::c_int as usize],
                (*h).mb.mv_max[1 as libc::c_int as usize],
            ) as int16_t;
            ((*h).mc.mc_luma).expect("non-null function pointer")(
                (*h).mb.pic.p_fdec[p as usize],
                32 as libc::c_int as intptr_t,
                &mut *(*(*((*h).mb.pic.p_fref)
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize))
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize))
                .as_mut_ptr()
                .offset((p * 4 as libc::c_int) as isize),
                (*h).mb.pic.i_stride[p as usize] as intptr_t,
                mvp[0 as libc::c_int as usize] as libc::c_int,
                mvp[1 as libc::c_int as usize] as libc::c_int,
                16 as libc::c_int,
                16 as libc::c_int,
                &mut *(*((*h).sh.weight)
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize))
                .as_mut_ptr()
                .offset(p as isize),
            );
        }
        let mut i8x8: libc::c_int = 0 as libc::c_int;
        let mut i_decimate_mb: libc::c_int = 0 as libc::c_int;
        while i8x8 < 4 as libc::c_int {
            let mut fenc_offset: libc::c_int = (i8x8 & 1 as libc::c_int) * 8 as libc::c_int
                + (i8x8 >> 1 as libc::c_int) * 16 as libc::c_int * 8 as libc::c_int;
            let mut fdec_offset: libc::c_int = (i8x8 & 1 as libc::c_int) * 8 as libc::c_int
                + (i8x8 >> 1 as libc::c_int) * 32 as libc::c_int * 8 as libc::c_int;
            ((*h).dctf.sub8x8_dct).expect("non-null function pointer")(
                dct4x4.as_mut_ptr(),
                ((*h).mb.pic.p_fenc[p as usize]).offset(fenc_offset as isize),
                ((*h).mb.pic.p_fdec[p as usize]).offset(fdec_offset as isize),
            );
            if (*h).mb.b_noise_reduction != 0 {
                let mut i4x4: libc::c_int = 0 as libc::c_int;
                while i4x4 < 4 as libc::c_int {
                    ((*h).quantf.denoise_dct).expect("non-null function pointer")(
                        (dct4x4[i4x4 as usize]).as_mut_ptr(),
                        (*((*h).nr_residual_sum).offset(
                            (0 as libc::c_int + (p != 0) as libc::c_int * 2 as libc::c_int)
                                as isize,
                        ))
                        .as_mut_ptr(),
                        (*((*h).nr_offset).offset(
                            (0 as libc::c_int + (p != 0) as libc::c_int * 2 as libc::c_int)
                                as isize,
                        ))
                        .as_mut_ptr(),
                        16 as libc::c_int,
                    );
                    i4x4 += 1;
                    i4x4;
                }
            }
            let mut nz: libc::c_int = ((*h).quantf.quant_4x4x4).expect("non-null function pointer")(
                dct4x4.as_mut_ptr(),
                (*((*h).quant4_mf[quant_cat as usize]).offset(i_qp as isize)).as_mut_ptr(),
                (*((*h).quant4_bias[quant_cat as usize]).offset(i_qp as isize)).as_mut_ptr(),
            );
            let mut idx: libc::c_int = 0 as libc::c_int;
            let mut msk: libc::c_int = nz;
            let mut skip: libc::c_int = 0;
            while msk != 0 && {
                skip = x264_ctz_4bit(msk as uint32_t);
                idx += skip;
                msk >>= skip + 1 as libc::c_int;
                1 as libc::c_int != 0
            } {
                ((*h).zigzagf.scan_4x4).expect("non-null function pointer")(
                    dctscan.as_mut_ptr(),
                    (dct4x4[idx as usize]).as_mut_ptr(),
                );
                i_decimate_mb += ((*h).quantf.decimate_score16).expect("non-null function pointer")(
                    dctscan.as_mut_ptr(),
                );
                if i_decimate_mb >= 6 as libc::c_int {
                    return 0 as libc::c_int;
                }
                idx += 1;
                idx;
            }
            i8x8 += 1;
            i8x8;
        }
        p += 1;
        p;
        i_qp = (*h).mb.i_chroma_qp;
    }
    if chroma == CHROMA_420 as libc::c_int || chroma == CHROMA_422 as libc::c_int {
        i_qp = (*h).mb.i_chroma_qp;
        let mut chroma422: libc::c_int = (chroma == CHROMA_422 as libc::c_int) as libc::c_int;
        let mut thresh: libc::c_int = if chroma422 != 0 {
            (x264_lambda2_tab[i_qp as usize] + 16 as libc::c_int) >> 5 as libc::c_int
        } else {
            (x264_lambda2_tab[i_qp as usize] + 32 as libc::c_int) >> 6 as libc::c_int
        };
        let mut ssd: libc::c_int = 0;
        let mut dct_dc: [dctcoef; 8] = [0; 8];
        if b_bidir == 0 {
            if (*(mvp.as_mut_ptr() as *mut x264_union32_t)).i != 0 {
                ((*h).mc.mc_chroma).expect("non-null function pointer")(
                    (*h).mb.pic.p_fdec[1 as libc::c_int as usize],
                    (*h).mb.pic.p_fdec[2 as libc::c_int as usize],
                    32 as libc::c_int as intptr_t,
                    (*h).mb.pic.p_fref[0 as libc::c_int as usize][0 as libc::c_int as usize]
                        [4 as libc::c_int as usize],
                    (*h).mb.pic.i_stride[1 as libc::c_int as usize] as intptr_t,
                    mvp[0 as libc::c_int as usize] as libc::c_int,
                    mvp[1 as libc::c_int as usize] as libc::c_int
                        * ((1 as libc::c_int) << chroma422),
                    8 as libc::c_int,
                    if chroma422 != 0 {
                        16 as libc::c_int
                    } else {
                        8 as libc::c_int
                    },
                );
            } else {
                ((*h).mc.load_deinterleave_chroma_fdec).expect("non-null function pointer")(
                    (*h).mb.pic.p_fdec[1 as libc::c_int as usize],
                    (*h).mb.pic.p_fref[0 as libc::c_int as usize][0 as libc::c_int as usize]
                        [4 as libc::c_int as usize],
                    (*h).mb.pic.i_stride[1 as libc::c_int as usize] as intptr_t,
                    if chroma422 != 0 {
                        16 as libc::c_int
                    } else {
                        8 as libc::c_int
                    },
                );
            }
        }
        let mut ch: libc::c_int = 0 as libc::c_int;
        while ch < 2 as libc::c_int {
            let mut p_src: *mut pixel = (*h).mb.pic.p_fenc[(1 as libc::c_int + ch) as usize];
            let mut p_dst: *mut pixel = (*h).mb.pic.p_fdec[(1 as libc::c_int + ch) as usize];
            if b_bidir == 0
                && !((*h).sh.weight[0 as libc::c_int as usize][(1 as libc::c_int + ch) as usize]
                    .weightfn)
                    .is_null()
            {
                (*((*h).sh.weight[0 as libc::c_int as usize][(1 as libc::c_int + ch) as usize]
                    .weightfn)
                    .offset((8 as libc::c_int >> 2 as libc::c_int) as isize))
                .expect("non-null function pointer")(
                    (*h).mb.pic.p_fdec[(1 as libc::c_int + ch) as usize],
                    32 as libc::c_int as intptr_t,
                    (*h).mb.pic.p_fdec[(1 as libc::c_int + ch) as usize],
                    32 as libc::c_int as intptr_t,
                    &mut *(*((*h).sh.weight)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                    .as_mut_ptr()
                    .offset((1 as libc::c_int + ch) as isize),
                    if chroma422 != 0 {
                        16 as libc::c_int
                    } else {
                        8 as libc::c_int
                    },
                );
            }
            ssd = ((*h).pixf.ssd[(if chroma422 != 0 {
                PIXEL_8x16 as libc::c_int
            } else {
                PIXEL_8x8 as libc::c_int
            }) as usize])
                .expect("non-null function pointer")(
                p_dst,
                32 as libc::c_int as intptr_t,
                p_src,
                16 as libc::c_int as intptr_t,
            );
            if ssd >= thresh {
                if (*h).mb.b_noise_reduction != 0 {
                    let mut i: libc::c_int = 0 as libc::c_int;
                    while i <= chroma422 {
                        ((*h).dctf.sub8x8_dct).expect("non-null function pointer")(
                            &mut *dct4x4.as_mut_ptr().offset((4 as libc::c_int * i) as isize),
                            p_src.offset((8 as libc::c_int * i * 16 as libc::c_int) as isize),
                            p_dst.offset((8 as libc::c_int * i * 32 as libc::c_int) as isize),
                        );
                        i += 1;
                        i;
                    }
                    let mut i4x4_0: libc::c_int = 0 as libc::c_int;
                    while i4x4_0
                        < (if chroma422 != 0 {
                            8 as libc::c_int
                        } else {
                            4 as libc::c_int
                        })
                    {
                        ((*h).quantf.denoise_dct).expect("non-null function pointer")(
                            (dct4x4[i4x4_0 as usize]).as_mut_ptr(),
                            (*((*h).nr_residual_sum).offset(2 as libc::c_int as isize))
                                .as_mut_ptr(),
                            (*((*h).nr_offset).offset(2 as libc::c_int as isize)).as_mut_ptr(),
                            16 as libc::c_int,
                        );
                        dct_dc[i4x4_0 as usize] =
                            dct4x4[i4x4_0 as usize][0 as libc::c_int as usize];
                        dct4x4[i4x4_0 as usize][0 as libc::c_int as usize] =
                            0 as libc::c_int as dctcoef;
                        i4x4_0 += 1;
                        i4x4_0;
                    }
                } else if chroma422 != 0 {
                    ((*h).dctf.sub8x16_dct_dc).expect("non-null function pointer")(
                        dct_dc.as_mut_ptr(),
                        p_src,
                        p_dst,
                    );
                } else {
                    ((*h).dctf.sub8x8_dct_dc).expect("non-null function pointer")(
                        dct_dc.as_mut_ptr(),
                        p_src,
                        p_dst,
                    );
                }
                let mut i_0: libc::c_int = 0 as libc::c_int;
                while i_0 <= chroma422 {
                    if ((*h).quantf.quant_2x2_dc).expect("non-null function pointer")(
                        &mut *dct_dc
                            .as_mut_ptr()
                            .offset((4 as libc::c_int * i_0) as isize),
                        (*((*h).quant4_mf[CQM_4PC as libc::c_int as usize])
                            .offset((i_qp + 3 as libc::c_int * chroma422) as isize))
                            [0 as libc::c_int as usize] as libc::c_int
                            >> 1 as libc::c_int,
                        ((*((*h).quant4_bias[CQM_4PC as libc::c_int as usize])
                            .offset((i_qp + 3 as libc::c_int * chroma422) as isize))
                            [0 as libc::c_int as usize] as libc::c_int)
                            << 1 as libc::c_int,
                    ) != 0
                    {
                        return 0 as libc::c_int;
                    }
                    i_0 += 1;
                    i_0;
                }
                if ssd >= thresh * 4 as libc::c_int {
                    if (*h).mb.b_noise_reduction == 0 {
                        let mut i_1: libc::c_int = 0 as libc::c_int;
                        while i_1 <= chroma422 {
                            ((*h).dctf.sub8x8_dct).expect("non-null function pointer")(
                                &mut *dct4x4
                                    .as_mut_ptr()
                                    .offset((4 as libc::c_int * i_1) as isize),
                                p_src.offset((8 as libc::c_int * i_1 * 16 as libc::c_int) as isize),
                                p_dst.offset((8 as libc::c_int * i_1 * 32 as libc::c_int) as isize),
                            );
                            dct4x4[(i_1 * 4 as libc::c_int + 0 as libc::c_int) as usize]
                                [0 as libc::c_int as usize] = 0 as libc::c_int as dctcoef;
                            dct4x4[(i_1 * 4 as libc::c_int + 1 as libc::c_int) as usize]
                                [0 as libc::c_int as usize] = 0 as libc::c_int as dctcoef;
                            dct4x4[(i_1 * 4 as libc::c_int + 2 as libc::c_int) as usize]
                                [0 as libc::c_int as usize] = 0 as libc::c_int as dctcoef;
                            dct4x4[(i_1 * 4 as libc::c_int + 3 as libc::c_int) as usize]
                                [0 as libc::c_int as usize] = 0 as libc::c_int as dctcoef;
                            i_1 += 1;
                            i_1;
                        }
                    }
                    let mut i8x8_0: libc::c_int = 0 as libc::c_int;
                    let mut i_decimate_mb_0: libc::c_int = 0 as libc::c_int;
                    while i8x8_0
                        < (if chroma422 != 0 {
                            2 as libc::c_int
                        } else {
                            1 as libc::c_int
                        })
                    {
                        let mut nz_0: libc::c_int = ((*h).quantf.quant_4x4x4)
                            .expect("non-null function pointer")(
                            &mut *dct4x4
                                .as_mut_ptr()
                                .offset((i8x8_0 * 4 as libc::c_int) as isize),
                            (*((*h).quant4_mf[CQM_4PC as libc::c_int as usize])
                                .offset(i_qp as isize))
                            .as_mut_ptr(),
                            (*((*h).quant4_bias[CQM_4PC as libc::c_int as usize])
                                .offset(i_qp as isize))
                            .as_mut_ptr(),
                        );
                        let mut idx_0: libc::c_int = i8x8_0 * 4 as libc::c_int;
                        let mut msk_0: libc::c_int = nz_0;
                        let mut skip_0: libc::c_int = 0;
                        while msk_0 != 0 && {
                            skip_0 = x264_ctz_4bit(msk_0 as uint32_t);
                            idx_0 += skip_0;
                            msk_0 >>= skip_0 + 1 as libc::c_int;
                            1 as libc::c_int != 0
                        } {
                            ((*h).zigzagf.scan_4x4).expect("non-null function pointer")(
                                dctscan.as_mut_ptr(),
                                (dct4x4[idx_0 as usize]).as_mut_ptr(),
                            );
                            i_decimate_mb_0 += ((*h).quantf.decimate_score15)
                                .expect("non-null function pointer")(
                                dctscan.as_mut_ptr()
                            );
                            if i_decimate_mb_0 >= 7 as libc::c_int {
                                return 0 as libc::c_int;
                            }
                            idx_0 += 1;
                            idx_0;
                        }
                        i8x8_0 += 1;
                        i8x8_0;
                    }
                }
            }
            ch += 1;
            ch;
        }
    }
    (*h).mb.b_skip_mc = 1 as libc::c_int;
    1 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_macroblock_probe_skip(
    mut h: *mut x264_t,
    mut b_bidir: libc::c_int,
) -> libc::c_int {
    if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_420 as libc::c_int {
        macroblock_probe_skip_internal(h, b_bidir, 1 as libc::c_int, CHROMA_420 as libc::c_int)
    } else if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_422 as libc::c_int {
        return macroblock_probe_skip_internal(
            h,
            b_bidir,
            1 as libc::c_int,
            CHROMA_422 as libc::c_int,
        );
    } else if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as libc::c_int {
        return macroblock_probe_skip_internal(
            h,
            b_bidir,
            3 as libc::c_int,
            CHROMA_444 as libc::c_int,
        );
    } else {
        return macroblock_probe_skip_internal(
            h,
            b_bidir,
            1 as libc::c_int,
            CHROMA_400 as libc::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_noise_reduction_update(mut h: *mut x264_t) {
    (*h).nr_offset = ((*h).nr_offset_denoise).as_mut_ptr();
    (*h).nr_residual_sum = ((*h).nr_residual_sum_buf[0 as libc::c_int as usize]).as_mut_ptr();
    (*h).nr_count = ((*h).nr_count_buf[0 as libc::c_int as usize]).as_mut_ptr();
    let mut cat: libc::c_int = 0 as libc::c_int;
    while cat
        < 3 as libc::c_int
            + ((*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as libc::c_int)
                as libc::c_int
    {
        let mut dct8x8: libc::c_int = cat & 1 as libc::c_int;
        let mut size: libc::c_int = if dct8x8 != 0 {
            64 as libc::c_int
        } else {
            16 as libc::c_int
        };
        let mut weight: *const uint32_t = if dct8x8 != 0 {
            x264_dct8_weight2_tab.as_ptr()
        } else {
            x264_dct4_weight2_tab.as_ptr()
        };
        if *((*h).nr_count).offset(cat as isize)
            > (if dct8x8 != 0 {
                (1 as libc::c_int) << 16 as libc::c_int
            } else {
                (1 as libc::c_int) << 18 as libc::c_int
            }) as uint32_t
        {
            let mut i: libc::c_int = 0 as libc::c_int;
            while i < size {
                (*((*h).nr_residual_sum).offset(cat as isize))[i as usize] >>= 1 as libc::c_int;
                i += 1;
                i;
            }
            *((*h).nr_count).offset(cat as isize) >>= 1 as libc::c_int;
        }
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < size {
            (*((*h).nr_offset).offset(cat as isize))[i_0 as usize] =
                (((*h).param.analyse.i_noise_reduction as uint64_t
                    * *((*h).nr_count).offset(cat as isize) as uint64_t)
                    .wrapping_add(
                        ((*((*h).nr_residual_sum).offset(cat as isize))[i_0 as usize]
                            / 2 as libc::c_int as uint32_t) as uint64_t,
                    )
                    / ((*((*h).nr_residual_sum).offset(cat as isize))[i_0 as usize] as uint64_t
                        * *weight.offset(i_0 as isize) as uint64_t
                        / 256 as libc::c_int as uint64_t)
                        .wrapping_add(1 as libc::c_int as uint64_t)) as udctcoef;
            i_0 += 1;
            i_0;
        }
        (*((*h).nr_offset).offset(cat as isize))[0 as libc::c_int as usize] =
            0 as libc::c_int as udctcoef;
        cat += 1;
        cat;
    }
}
#[inline(always)]
unsafe extern "C" fn macroblock_encode_p8x8_internal(
    mut h: *mut x264_t,
    mut i8: libc::c_int,
    mut plane_count: libc::c_int,
    mut chroma: libc::c_int,
) {
    let mut b_decimate: libc::c_int = (*h).mb.b_dct_decimate;
    let mut i_qp: libc::c_int = (*h).mb.i_qp;
    let mut x: libc::c_int = i8 & 1 as libc::c_int;
    let mut y: libc::c_int = i8 >> 1 as libc::c_int;
    let mut nz: libc::c_int = 0;
    let mut chroma422: libc::c_int = (chroma == CHROMA_422 as libc::c_int) as libc::c_int;
    (*h).mb.i_cbp_chroma = 0 as libc::c_int;
    (*h).mb.i_cbp_luma &= !((1 as libc::c_int) << i8);
    if (*h).mb.b_skip_mc == 0 {
        x264_8_mb_mc_8x8(h, i8);
    }
    if (*h).mb.b_lossless != 0 {
        let mut p: libc::c_int = 0 as libc::c_int;
        while p < plane_count {
            let mut p_fenc: *mut pixel = ((*h).mb.pic.p_fenc[p as usize])
                .offset((8 as libc::c_int * x) as isize)
                .offset((8 as libc::c_int * y * 16 as libc::c_int) as isize);
            let mut p_fdec: *mut pixel = ((*h).mb.pic.p_fdec[p as usize])
                .offset((8 as libc::c_int * x) as isize)
                .offset((8 as libc::c_int * y * 32 as libc::c_int) as isize);
            let mut nnz8x8: libc::c_int = 0 as libc::c_int;
            if (*h).mb.b_transform_8x8 != 0 {
                nnz8x8 = ((*h).zigzagf.sub_8x8).expect("non-null function pointer")(
                    ((*h).dct.luma8x8[(4 as libc::c_int * p + i8) as usize]).as_mut_ptr(),
                    p_fenc,
                    p_fdec,
                );
                (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
                    (*x264_scan8
                        .as_ptr()
                        .offset((p * 16 as libc::c_int + i8 * 4 as libc::c_int) as isize)
                        as libc::c_int
                        + 0 as libc::c_int) as isize,
                ) as *mut uint8_t as *mut x264_union16_t))
                    .i = (nnz8x8 * 0x101 as libc::c_int) as uint16_t;
                (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
                    (*x264_scan8
                        .as_ptr()
                        .offset((p * 16 as libc::c_int + i8 * 4 as libc::c_int) as isize)
                        as libc::c_int
                        + 8 as libc::c_int) as isize,
                ) as *mut uint8_t as *mut x264_union16_t))
                    .i = (nnz8x8 * 0x101 as libc::c_int) as uint16_t;
            } else {
                let mut i4: libc::c_int = i8 * 4 as libc::c_int;
                while i4 < i8 * 4 as libc::c_int + 4 as libc::c_int {
                    nz = ((*h).zigzagf.sub_4x4).expect("non-null function pointer")(
                        ((*h).dct.luma4x4[(16 as libc::c_int * p + i4) as usize]).as_mut_ptr(),
                        ((*h).mb.pic.p_fenc[p as usize])
                            .offset(block_idx_xy_fenc[i4 as usize] as libc::c_int as isize),
                        ((*h).mb.pic.p_fdec[p as usize])
                            .offset(block_idx_xy_fdec[i4 as usize] as libc::c_int as isize),
                    );
                    (*h).mb.cache.non_zero_count
                        [x264_scan8[(16 as libc::c_int * p + i4) as usize] as usize] =
                        nz as uint8_t;
                    nnz8x8 |= nz;
                    i4 += 1;
                    i4;
                }
            }
            (*h).mb.i_cbp_luma |= nnz8x8 << i8;
            p += 1;
            p;
        }
        if chroma == CHROMA_420 as libc::c_int || chroma == CHROMA_422 as libc::c_int {
            let mut ch: libc::c_int = 0 as libc::c_int;
            while ch < 2 as libc::c_int {
                let mut dc: dctcoef = 0;
                let mut p_fenc_0: *mut pixel = ((*h).mb.pic.p_fenc
                    [(1 as libc::c_int + ch) as usize])
                    .offset((4 as libc::c_int * x) as isize)
                    .offset(
                        ((if chroma422 != 0 {
                            8 as libc::c_int
                        } else {
                            4 as libc::c_int
                        }) * y
                            * 16 as libc::c_int) as isize,
                    );
                let mut p_fdec_0: *mut pixel = ((*h).mb.pic.p_fdec
                    [(1 as libc::c_int + ch) as usize])
                    .offset((4 as libc::c_int * x) as isize)
                    .offset(
                        ((if chroma422 != 0 {
                            8 as libc::c_int
                        } else {
                            4 as libc::c_int
                        }) * y
                            * 32 as libc::c_int) as isize,
                    );
                let mut i4x4: libc::c_int = 0 as libc::c_int;
                while i4x4 <= chroma422 {
                    let mut offset: libc::c_int = if chroma422 != 0 {
                        8 as libc::c_int * y + 2 as libc::c_int * i4x4 + x
                    } else {
                        i8
                    };
                    nz = ((*h).zigzagf.sub_4x4ac).expect("non-null function pointer")(
                        ((*h).dct.luma4x4
                            [(16 as libc::c_int + offset + ch * 16 as libc::c_int) as usize])
                            .as_mut_ptr(),
                        p_fenc_0.offset((4 as libc::c_int * i4x4 * 16 as libc::c_int) as isize),
                        p_fdec_0.offset((4 as libc::c_int * i4x4 * 32 as libc::c_int) as isize),
                        &mut dc,
                    );
                    (*h).mb.cache.non_zero_count[x264_scan8
                        [(16 as libc::c_int + offset + ch * 16 as libc::c_int) as usize]
                        as usize] = nz as uint8_t;
                    i4x4 += 1;
                    i4x4;
                }
                ch += 1;
                ch;
            }
            (*h).mb.i_cbp_chroma = 0x2 as libc::c_int;
        }
    } else {
        if (*h).mb.b_transform_8x8 != 0 {
            let mut p_0: libc::c_int = 0 as libc::c_int;
            while p_0 < plane_count {
                let mut quant_cat: libc::c_int = if p_0 != 0 {
                    CQM_8PC as libc::c_int
                } else {
                    CQM_8PY as libc::c_int
                };
                let mut p_fenc_1: *mut pixel = ((*h).mb.pic.p_fenc[p_0 as usize])
                    .offset((8 as libc::c_int * x) as isize)
                    .offset((8 as libc::c_int * y * 16 as libc::c_int) as isize);
                let mut p_fdec_1: *mut pixel = ((*h).mb.pic.p_fdec[p_0 as usize])
                    .offset((8 as libc::c_int * x) as isize)
                    .offset((8 as libc::c_int * y * 32 as libc::c_int) as isize);
                let mut dct8x8: [dctcoef; 64] = [0; 64];
                ((*h).dctf.sub8x8_dct8).expect("non-null function pointer")(
                    dct8x8.as_mut_ptr(),
                    p_fenc_1,
                    p_fdec_1,
                );
                let mut nnz8x8_0: libc::c_int = x264_quant_8x8(
                    h,
                    dct8x8.as_mut_ptr(),
                    i_qp,
                    ctx_cat_plane[DCT_LUMA_8x8 as libc::c_int as usize][p_0 as usize]
                        as libc::c_int,
                    0 as libc::c_int,
                    p_0,
                    i8,
                );
                if nnz8x8_0 != 0 {
                    ((*h).zigzagf.scan_8x8).expect("non-null function pointer")(
                        ((*h).dct.luma8x8[(4 as libc::c_int * p_0 + i8) as usize]).as_mut_ptr(),
                        dct8x8.as_mut_ptr(),
                    );
                    if b_decimate != 0 && (*h).mb.b_trellis == 0 {
                        nnz8x8_0 = (4 as libc::c_int
                            <= ((*h).quantf.decimate_score64).expect("non-null function pointer")(
                                ((*h).dct.luma8x8[(4 as libc::c_int * p_0 + i8) as usize])
                                    .as_mut_ptr(),
                            )) as libc::c_int;
                    }
                    if nnz8x8_0 != 0 {
                        ((*h).quantf.dequant_8x8).expect("non-null function pointer")(
                            dct8x8.as_mut_ptr(),
                            (*h).dequant8_mf[quant_cat as usize],
                            i_qp,
                        );
                        ((*h).dctf.add8x8_idct8).expect("non-null function pointer")(
                            p_fdec_1,
                            dct8x8.as_mut_ptr(),
                        );
                        (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
                            (*x264_scan8
                                .as_ptr()
                                .offset((p_0 * 16 as libc::c_int + i8 * 4 as libc::c_int) as isize)
                                as libc::c_int
                                + 0 as libc::c_int) as isize,
                        ) as *mut uint8_t as *mut x264_union16_t))
                            .i = (1 as libc::c_int * 0x101 as libc::c_int) as uint16_t;
                        (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
                            (*x264_scan8
                                .as_ptr()
                                .offset((p_0 * 16 as libc::c_int + i8 * 4 as libc::c_int) as isize)
                                as libc::c_int
                                + 8 as libc::c_int) as isize,
                        ) as *mut uint8_t as *mut x264_union16_t))
                            .i = (1 as libc::c_int * 0x101 as libc::c_int) as uint16_t;
                        (*h).mb.i_cbp_luma |= (1 as libc::c_int) << i8;
                    } else {
                        (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
                            (*x264_scan8
                                .as_ptr()
                                .offset((p_0 * 16 as libc::c_int + i8 * 4 as libc::c_int) as isize)
                                as libc::c_int
                                + 0 as libc::c_int) as isize,
                        ) as *mut uint8_t as *mut x264_union16_t))
                            .i = (0 as libc::c_int * 0x101 as libc::c_int) as uint16_t;
                        (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
                            (*x264_scan8
                                .as_ptr()
                                .offset((p_0 * 16 as libc::c_int + i8 * 4 as libc::c_int) as isize)
                                as libc::c_int
                                + 8 as libc::c_int) as isize,
                        ) as *mut uint8_t as *mut x264_union16_t))
                            .i = (0 as libc::c_int * 0x101 as libc::c_int) as uint16_t;
                    }
                } else {
                    (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
                        (*x264_scan8
                            .as_ptr()
                            .offset((p_0 * 16 as libc::c_int + i8 * 4 as libc::c_int) as isize)
                            as libc::c_int
                            + 0 as libc::c_int) as isize,
                    ) as *mut uint8_t as *mut x264_union16_t))
                        .i = (0 as libc::c_int * 0x101 as libc::c_int) as uint16_t;
                    (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
                        (*x264_scan8
                            .as_ptr()
                            .offset((p_0 * 16 as libc::c_int + i8 * 4 as libc::c_int) as isize)
                            as libc::c_int
                            + 8 as libc::c_int) as isize,
                    ) as *mut uint8_t as *mut x264_union16_t))
                        .i = (0 as libc::c_int * 0x101 as libc::c_int) as uint16_t;
                }
                p_0 += 1;
                p_0;
                i_qp = (*h).mb.i_chroma_qp;
            }
        } else {
            let mut p_1: libc::c_int = 0 as libc::c_int;
            while p_1 < plane_count {
                let mut quant_cat_0: libc::c_int = if p_1 != 0 {
                    CQM_4PC as libc::c_int
                } else {
                    CQM_4PY as libc::c_int
                };
                let mut p_fenc_2: *mut pixel = ((*h).mb.pic.p_fenc[p_1 as usize])
                    .offset((8 as libc::c_int * x) as isize)
                    .offset((8 as libc::c_int * y * 16 as libc::c_int) as isize);
                let mut p_fdec_2: *mut pixel = ((*h).mb.pic.p_fdec[p_1 as usize])
                    .offset((8 as libc::c_int * x) as isize)
                    .offset((8 as libc::c_int * y * 32 as libc::c_int) as isize);
                let mut i_decimate_8x8: libc::c_int = if b_decimate != 0 {
                    0 as libc::c_int
                } else {
                    4 as libc::c_int
                };
                let mut dct4x4: [[dctcoef; 16]; 4] = [[0; 16]; 4];
                let mut nnz8x8_1: libc::c_int = 0 as libc::c_int;
                ((*h).dctf.sub8x8_dct).expect("non-null function pointer")(
                    dct4x4.as_mut_ptr(),
                    p_fenc_2,
                    p_fdec_2,
                );
                (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
                    (*x264_scan8
                        .as_ptr()
                        .offset((p_1 * 16 as libc::c_int + i8 * 4 as libc::c_int) as isize)
                        as libc::c_int
                        + 0 as libc::c_int) as isize,
                ) as *mut uint8_t as *mut x264_union16_t))
                    .i = (0 as libc::c_int * 0x101 as libc::c_int) as uint16_t;
                (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
                    (*x264_scan8
                        .as_ptr()
                        .offset((p_1 * 16 as libc::c_int + i8 * 4 as libc::c_int) as isize)
                        as libc::c_int
                        + 8 as libc::c_int) as isize,
                ) as *mut uint8_t as *mut x264_union16_t))
                    .i = (0 as libc::c_int * 0x101 as libc::c_int) as uint16_t;
                if (*h).mb.b_noise_reduction != 0 {
                    let mut idx: libc::c_int = 0 as libc::c_int;
                    while idx < 4 as libc::c_int {
                        ((*h).quantf.denoise_dct).expect("non-null function pointer")(
                            (dct4x4[idx as usize]).as_mut_ptr(),
                            (*((*h).nr_residual_sum).offset(
                                (0 as libc::c_int + (p_1 != 0) as libc::c_int * 2 as libc::c_int)
                                    as isize,
                            ))
                            .as_mut_ptr(),
                            (*((*h).nr_offset).offset(
                                (0 as libc::c_int + (p_1 != 0) as libc::c_int * 2 as libc::c_int)
                                    as isize,
                            ))
                            .as_mut_ptr(),
                            16 as libc::c_int,
                        );
                        idx += 1;
                        idx;
                    }
                }
                if (*h).mb.b_trellis != 0 {
                    let mut i4x4_0: libc::c_int = 0 as libc::c_int;
                    while i4x4_0 < 4 as libc::c_int {
                        if x264_8_quant_4x4_trellis(
                            h,
                            (dct4x4[i4x4_0 as usize]).as_mut_ptr(),
                            quant_cat_0,
                            i_qp,
                            ctx_cat_plane[DCT_LUMA_4x4 as libc::c_int as usize][p_1 as usize]
                                as libc::c_int,
                            0 as libc::c_int,
                            (p_1 != 0) as libc::c_int,
                            i8 * 4 as libc::c_int + i4x4_0 + p_1 * 16 as libc::c_int,
                        ) != 0
                        {
                            ((*h).zigzagf.scan_4x4).expect("non-null function pointer")(
                                ((*h).dct.luma4x4[(p_1 * 16 as libc::c_int
                                    + i8 * 4 as libc::c_int
                                    + i4x4_0)
                                    as usize])
                                    .as_mut_ptr(),
                                (dct4x4[i4x4_0 as usize]).as_mut_ptr(),
                            );
                            ((*h).quantf.dequant_4x4).expect("non-null function pointer")(
                                (dct4x4[i4x4_0 as usize]).as_mut_ptr(),
                                (*h).dequant4_mf[quant_cat_0 as usize],
                                i_qp,
                            );
                            if i_decimate_8x8 < 4 as libc::c_int {
                                i_decimate_8x8 += ((*h).quantf.decimate_score16)
                                    .expect("non-null function pointer")(
                                    ((*h).dct.luma4x4[(p_1 * 16 as libc::c_int
                                        + i8 * 4 as libc::c_int
                                        + i4x4_0)
                                        as usize])
                                        .as_mut_ptr(),
                                );
                            }
                            (*h).mb.cache.non_zero_count[x264_scan8[(p_1 * 16 as libc::c_int
                                + i8 * 4 as libc::c_int
                                + i4x4_0)
                                as usize]
                                as usize] = 1 as libc::c_int as uint8_t;
                            nnz8x8_1 = 1 as libc::c_int;
                        }
                        i4x4_0 += 1;
                        i4x4_0;
                    }
                } else {
                    nz = ((*h).quantf.quant_4x4x4).expect("non-null function pointer")(
                        dct4x4.as_mut_ptr(),
                        (*((*h).quant4_mf[quant_cat_0 as usize]).offset(i_qp as isize))
                            .as_mut_ptr(),
                        (*((*h).quant4_bias[quant_cat_0 as usize]).offset(i_qp as isize))
                            .as_mut_ptr(),
                    );
                    nnz8x8_1 = nz;
                    if nz != 0 {
                        let mut i4x4_1: libc::c_int = 0 as libc::c_int;
                        let mut msk: libc::c_int = nz;
                        let mut skip: libc::c_int = 0;
                        while msk != 0 && {
                            skip = x264_ctz_4bit(msk as uint32_t);
                            i4x4_1 += skip;
                            msk >>= skip + 1 as libc::c_int;
                            1 as libc::c_int != 0
                        } {
                            ((*h).zigzagf.scan_4x4).expect("non-null function pointer")(
                                ((*h).dct.luma4x4[(p_1 * 16 as libc::c_int
                                    + i8 * 4 as libc::c_int
                                    + i4x4_1)
                                    as usize])
                                    .as_mut_ptr(),
                                (dct4x4[i4x4_1 as usize]).as_mut_ptr(),
                            );
                            ((*h).quantf.dequant_4x4).expect("non-null function pointer")(
                                (dct4x4[i4x4_1 as usize]).as_mut_ptr(),
                                (*h).dequant4_mf[quant_cat_0 as usize],
                                i_qp,
                            );
                            if i_decimate_8x8 < 4 as libc::c_int {
                                i_decimate_8x8 += ((*h).quantf.decimate_score16)
                                    .expect("non-null function pointer")(
                                    ((*h).dct.luma4x4[(p_1 * 16 as libc::c_int
                                        + i8 * 4 as libc::c_int
                                        + i4x4_1)
                                        as usize])
                                        .as_mut_ptr(),
                                );
                            }
                            (*h).mb.cache.non_zero_count[x264_scan8[(p_1 * 16 as libc::c_int
                                + i8 * 4 as libc::c_int
                                + i4x4_1)
                                as usize]
                                as usize] = 1 as libc::c_int as uint8_t;
                            i4x4_1 += 1;
                            i4x4_1;
                        }
                    }
                }
                if nnz8x8_1 != 0 {
                    if i_decimate_8x8 < 4 as libc::c_int {
                        (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
                            (*x264_scan8
                                .as_ptr()
                                .offset((p_1 * 16 as libc::c_int + i8 * 4 as libc::c_int) as isize)
                                as libc::c_int
                                + 0 as libc::c_int) as isize,
                        ) as *mut uint8_t as *mut x264_union16_t))
                            .i = (0 as libc::c_int * 0x101 as libc::c_int) as uint16_t;
                        (*(&mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
                            (*x264_scan8
                                .as_ptr()
                                .offset((p_1 * 16 as libc::c_int + i8 * 4 as libc::c_int) as isize)
                                as libc::c_int
                                + 8 as libc::c_int) as isize,
                        ) as *mut uint8_t as *mut x264_union16_t))
                            .i = (0 as libc::c_int * 0x101 as libc::c_int) as uint16_t;
                    } else {
                        ((*h).dctf.add8x8_idct).expect("non-null function pointer")(
                            p_fdec_2,
                            dct4x4.as_mut_ptr(),
                        );
                        (*h).mb.i_cbp_luma |= (1 as libc::c_int) << i8;
                    }
                }
                p_1 += 1;
                p_1;
                i_qp = (*h).mb.i_chroma_qp;
            }
        }
        if chroma == CHROMA_420 as libc::c_int || chroma == CHROMA_422 as libc::c_int {
            i_qp = (*h).mb.i_chroma_qp;
            let mut ch_0: libc::c_int = 0 as libc::c_int;
            while ch_0 < 2 as libc::c_int {
                let mut dct4x4_0: [[dctcoef; 16]; 2] = [[0; 16]; 2];
                let mut p_fenc_3: *mut pixel = ((*h).mb.pic.p_fenc
                    [(1 as libc::c_int + ch_0) as usize])
                    .offset((4 as libc::c_int * x) as isize)
                    .offset(
                        ((if chroma422 != 0 {
                            8 as libc::c_int
                        } else {
                            4 as libc::c_int
                        }) * y
                            * 16 as libc::c_int) as isize,
                    );
                let mut p_fdec_3: *mut pixel = ((*h).mb.pic.p_fdec
                    [(1 as libc::c_int + ch_0) as usize])
                    .offset((4 as libc::c_int * x) as isize)
                    .offset(
                        ((if chroma422 != 0 {
                            8 as libc::c_int
                        } else {
                            4 as libc::c_int
                        }) * y
                            * 32 as libc::c_int) as isize,
                    );
                let mut i4x4_2: libc::c_int = 0 as libc::c_int;
                while i4x4_2 <= chroma422 {
                    ((*h).dctf.sub4x4_dct).expect("non-null function pointer")(
                        (dct4x4_0[i4x4_2 as usize]).as_mut_ptr(),
                        p_fenc_3.offset((4 as libc::c_int * i4x4_2 * 16 as libc::c_int) as isize),
                        p_fdec_3.offset((4 as libc::c_int * i4x4_2 * 32 as libc::c_int) as isize),
                    );
                    if (*h).mb.b_noise_reduction != 0 {
                        ((*h).quantf.denoise_dct).expect("non-null function pointer")(
                            (dct4x4_0[i4x4_2 as usize]).as_mut_ptr(),
                            (*((*h).nr_residual_sum).offset(2 as libc::c_int as isize))
                                .as_mut_ptr(),
                            (*((*h).nr_offset).offset(2 as libc::c_int as isize)).as_mut_ptr(),
                            16 as libc::c_int,
                        );
                    }
                    dct4x4_0[i4x4_2 as usize][0 as libc::c_int as usize] =
                        0 as libc::c_int as dctcoef;
                    if (*h).mb.b_trellis != 0 {
                        nz = x264_8_quant_4x4_trellis(
                            h,
                            (dct4x4_0[i4x4_2 as usize]).as_mut_ptr(),
                            CQM_4PC as libc::c_int,
                            i_qp,
                            DCT_CHROMA_AC as libc::c_int,
                            0 as libc::c_int,
                            1 as libc::c_int,
                            0 as libc::c_int,
                        );
                    } else {
                        nz = ((*h).quantf.quant_4x4).expect("non-null function pointer")(
                            (dct4x4_0[i4x4_2 as usize]).as_mut_ptr(),
                            (*((*h).quant4_mf[CQM_4PC as libc::c_int as usize])
                                .offset(i_qp as isize))
                            .as_mut_ptr(),
                            (*((*h).quant4_bias[CQM_4PC as libc::c_int as usize])
                                .offset(i_qp as isize))
                            .as_mut_ptr(),
                        );
                    }
                    let mut offset_0: libc::c_int = if chroma422 != 0 {
                        ((5 as libc::c_int * i8) & 0x9 as libc::c_int) + 2 as libc::c_int * i4x4_2
                    } else {
                        i8
                    };
                    (*h).mb.cache.non_zero_count[x264_scan8
                        [(16 as libc::c_int + offset_0 + ch_0 * 16 as libc::c_int) as usize]
                        as usize] = nz as uint8_t;
                    if nz != 0 {
                        ((*h).zigzagf.scan_4x4).expect("non-null function pointer")(
                            ((*h).dct.luma4x4[(16 as libc::c_int
                                + offset_0
                                + ch_0 * 16 as libc::c_int)
                                as usize])
                                .as_mut_ptr(),
                            (dct4x4_0[i4x4_2 as usize]).as_mut_ptr(),
                        );
                        ((*h).quantf.dequant_4x4).expect("non-null function pointer")(
                            (dct4x4_0[i4x4_2 as usize]).as_mut_ptr(),
                            (*h).dequant4_mf[CQM_4PC as libc::c_int as usize],
                            i_qp,
                        );
                        ((*h).dctf.add4x4_idct).expect("non-null function pointer")(
                            p_fdec_3
                                .offset((4 as libc::c_int * i4x4_2 * 32 as libc::c_int) as isize),
                            (dct4x4_0[i4x4_2 as usize]).as_mut_ptr(),
                        );
                    }
                    i4x4_2 += 1;
                    i4x4_2;
                }
                ch_0 += 1;
                ch_0;
            }
            (*h).mb.i_cbp_chroma = 0x2 as libc::c_int;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_macroblock_encode_p8x8(mut h: *mut x264_t, mut i8: libc::c_int) {
    if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_420 as libc::c_int {
        macroblock_encode_p8x8_internal(h, i8, 1 as libc::c_int, CHROMA_420 as libc::c_int);
    } else if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_422 as libc::c_int {
        macroblock_encode_p8x8_internal(h, i8, 1 as libc::c_int, CHROMA_422 as libc::c_int);
    } else if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as libc::c_int {
        macroblock_encode_p8x8_internal(h, i8, 3 as libc::c_int, CHROMA_444 as libc::c_int);
    } else {
        macroblock_encode_p8x8_internal(h, i8, 1 as libc::c_int, CHROMA_400 as libc::c_int);
    };
}
#[inline(always)]
unsafe extern "C" fn macroblock_encode_p4x4_internal(
    mut h: *mut x264_t,
    mut i4: libc::c_int,
    mut plane_count: libc::c_int,
) {
    let mut i_qp: libc::c_int = (*h).mb.i_qp;
    let mut p: libc::c_int = 0 as libc::c_int;
    while p < plane_count {
        let mut quant_cat: libc::c_int = if p != 0 {
            CQM_4PC as libc::c_int
        } else {
            CQM_4PY as libc::c_int
        };
        let mut p_fenc: *mut pixel = &mut *(*((*h).mb.pic.p_fenc).as_mut_ptr().offset(p as isize))
            .offset(*block_idx_xy_fenc.as_ptr().offset(i4 as isize) as isize)
            as *mut pixel;
        let mut p_fdec: *mut pixel = &mut *(*((*h).mb.pic.p_fdec).as_mut_ptr().offset(p as isize))
            .offset(*block_idx_xy_fdec.as_ptr().offset(i4 as isize) as isize)
            as *mut pixel;
        let mut nz: libc::c_int = 0;
        if (*h).mb.b_lossless != 0 {
            nz = ((*h).zigzagf.sub_4x4).expect("non-null function pointer")(
                ((*h).dct.luma4x4[(p * 16 as libc::c_int + i4) as usize]).as_mut_ptr(),
                p_fenc,
                p_fdec,
            );
            (*h).mb.cache.non_zero_count
                [x264_scan8[(p * 16 as libc::c_int + i4) as usize] as usize] = nz as uint8_t;
        } else {
            let mut dct4x4: [dctcoef; 16] = [0; 16];
            ((*h).dctf.sub4x4_dct).expect("non-null function pointer")(
                dct4x4.as_mut_ptr(),
                p_fenc,
                p_fdec,
            );
            nz = x264_quant_4x4(
                h,
                dct4x4.as_mut_ptr(),
                i_qp,
                ctx_cat_plane[DCT_LUMA_4x4 as libc::c_int as usize][p as usize] as libc::c_int,
                0 as libc::c_int,
                p,
                i4,
            );
            (*h).mb.cache.non_zero_count
                [x264_scan8[(p * 16 as libc::c_int + i4) as usize] as usize] = nz as uint8_t;
            if nz != 0 {
                ((*h).zigzagf.scan_4x4).expect("non-null function pointer")(
                    ((*h).dct.luma4x4[(p * 16 as libc::c_int + i4) as usize]).as_mut_ptr(),
                    dct4x4.as_mut_ptr(),
                );
                ((*h).quantf.dequant_4x4).expect("non-null function pointer")(
                    dct4x4.as_mut_ptr(),
                    (*h).dequant4_mf[quant_cat as usize],
                    i_qp,
                );
                ((*h).dctf.add4x4_idct).expect("non-null function pointer")(
                    p_fdec,
                    dct4x4.as_mut_ptr(),
                );
            }
        }
        p += 1;
        p;
        i_qp = (*h).mb.i_chroma_qp;
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_macroblock_encode_p4x4(mut h: *mut x264_t, mut i8: libc::c_int) {
    if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as libc::c_int {
        macroblock_encode_p4x4_internal(h, i8, 3 as libc::c_int);
    } else {
        macroblock_encode_p4x4_internal(h, i8, 1 as libc::c_int);
    };
}
