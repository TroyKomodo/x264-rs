use crate::types::*;
extern "C" {
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
unsafe extern "C" fn bs_pos(mut s: *mut bs_t) -> libc::c_int {
    ((8 as libc::c_int as libc::c_long * ((*s).p).offset_from((*s).p_start) as libc::c_long)
        as libc::c_ulong)
        .wrapping_add(
            (::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        )
        .wrapping_sub((*s).i_left as libc::c_ulong) as libc::c_int
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
#[inline]
unsafe extern "C" fn bs_write1(mut s: *mut bs_t, mut i_bit: uint32_t) {
    (*s).cur_bits <<= 1 as libc::c_int;
    (*s).cur_bits |= i_bit as uintptr_t;
    (*s).i_left -= 1;
    (*s).i_left;
    if (*s).i_left as libc::c_ulong
        == (::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(32 as libc::c_int as libc::c_ulong)
    {
        (*((*s).p as *mut x264_union32_t)).i = endian_fix32((*s).cur_bits as uint32_t);
        (*s).p = ((*s).p).offset(4 as libc::c_int as isize);
        (*s).i_left = (::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong) as libc::c_int;
    }
}
#[inline]
unsafe extern "C" fn bs_align_0(mut s: *mut bs_t) {
    bs_write(
        s,
        (*s).i_left & 7 as libc::c_int,
        0 as libc::c_int as uint32_t,
    );
    bs_flush(s);
}
#[inline]
unsafe extern "C" fn bs_write_ue(mut s: *mut bs_t, mut val: libc::c_int) {
    bs_write(
        s,
        x264_ue_size_tab[(val + 1 as libc::c_int) as usize] as libc::c_int,
        (val + 1 as libc::c_int) as uint32_t,
    );
}
#[inline]
unsafe extern "C" fn bs_write_se(mut s: *mut bs_t, mut val: libc::c_int) {
    let mut size: libc::c_int = 0 as libc::c_int;
    let mut tmp: libc::c_int = 1 as libc::c_int - val * 2 as libc::c_int;
    if tmp < 0 as libc::c_int {
        tmp = val * 2 as libc::c_int;
    }
    val = tmp;
    if tmp >= 0x100 as libc::c_int {
        size = 16 as libc::c_int;
        tmp >>= 8 as libc::c_int;
    }
    size += x264_ue_size_tab[tmp as usize] as libc::c_int;
    bs_write(s, size, val as uint32_t);
}
#[inline]
unsafe extern "C" fn bs_write_te(mut s: *mut bs_t, mut x: libc::c_int, mut val: libc::c_int) {
    if x == 1 as libc::c_int {
        bs_write1(s, (1 as libc::c_int ^ val) as uint32_t);
    } else {
        bs_write_ue(s, val);
    };
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
unsafe extern "C" fn x264_mb_predict_non_zero_code(
    mut h: *mut x264_t,
    mut idx: libc::c_int,
) -> libc::c_int {
    let za: libc::c_int = (*h).mb.cache.non_zero_count
        [(x264_scan8[idx as usize] as libc::c_int - 1 as libc::c_int) as usize]
        as libc::c_int;
    let zb: libc::c_int = (*h).mb.cache.non_zero_count
        [(x264_scan8[idx as usize] as libc::c_int - 8 as libc::c_int) as usize]
        as libc::c_int;
    let mut i_ret: libc::c_int = za + zb;
    if i_ret < 0x80 as libc::c_int {
        i_ret = (i_ret + 1 as libc::c_int) >> 1 as libc::c_int;
    }
    i_ret & 0x7f as libc::c_int
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
#[inline]
unsafe extern "C" fn cavlc_block_residual_escape(
    mut h: *mut x264_t,
    mut i_suffix_length: libc::c_int,
    mut level: libc::c_int,
) -> libc::c_int {
    let mut s: *mut bs_t = &mut (*h).out.bs;
    static mut next_suffix: [uint16_t; 7] = [
        0 as libc::c_int as uint16_t,
        3 as libc::c_int as uint16_t,
        6 as libc::c_int as uint16_t,
        12 as libc::c_int as uint16_t,
        24 as libc::c_int as uint16_t,
        48 as libc::c_int as uint16_t,
        0xffff as libc::c_int as uint16_t,
    ];
    let mut i_level_prefix: libc::c_int = 15 as libc::c_int;
    let mut mask: libc::c_int = level >> 31 as libc::c_int;
    let mut abs_level: libc::c_int = (level ^ mask) - mask;
    let mut i_level_code: libc::c_int = abs_level * 2 as libc::c_int - mask - 2 as libc::c_int;
    if i_level_code >> i_suffix_length < 15 as libc::c_int {
        bs_write(
            s,
            (i_level_code >> i_suffix_length) + 1 as libc::c_int + i_suffix_length,
            (((1 as libc::c_int) << i_suffix_length)
                + (i_level_code & (((1 as libc::c_int) << i_suffix_length) - 1 as libc::c_int)))
                as uint32_t,
        );
    } else {
        i_level_code -= (15 as libc::c_int) << i_suffix_length;
        if i_suffix_length == 0 as libc::c_int {
            i_level_code -= 15 as libc::c_int;
        }
        if i_level_code >= (1 as libc::c_int) << 12 as libc::c_int {
            if (*((*h).sps).as_mut_ptr()).i_profile_idc >= PROFILE_HIGH as libc::c_int {
                while i_level_code >= (1 as libc::c_int) << (i_level_prefix - 3 as libc::c_int) {
                    i_level_code -= (1 as libc::c_int) << (i_level_prefix - 3 as libc::c_int);
                    i_level_prefix += 1;
                }
            } else {
                (*h).mb.b_overflow = 1 as libc::c_int;
            }
        }
        bs_write(
            s,
            i_level_prefix + 1 as libc::c_int,
            1 as libc::c_int as uint32_t,
        );
        bs_write(
            s,
            i_level_prefix - 3 as libc::c_int,
            (i_level_code
                & (((1 as libc::c_int) << (i_level_prefix - 3 as libc::c_int)) - 1 as libc::c_int))
                as uint32_t,
        );
    }
    if i_suffix_length == 0 as libc::c_int {
        i_suffix_length += 1;
    }
    if abs_level > next_suffix[i_suffix_length as usize] as libc::c_int {
        i_suffix_length += 1;
    }
    i_suffix_length
}
unsafe extern "C" fn cavlc_block_residual_internal(
    mut h: *mut x264_t,
    mut ctx_block_cat: libc::c_int,
    mut l: *mut dctcoef,
    mut nC: libc::c_int,
) -> libc::c_int {
    let mut s: *mut bs_t = &mut (*h).out.bs;
    static mut ctz_index: [uint8_t; 8] = [
        3 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        1 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        2 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        1 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
    ];
    static mut count_cat: [uint8_t; 14] = [
        16 as libc::c_int as uint8_t,
        15 as libc::c_int as uint8_t,
        16 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        15 as libc::c_int as uint8_t,
        64 as libc::c_int as uint8_t,
        16 as libc::c_int as uint8_t,
        15 as libc::c_int as uint8_t,
        16 as libc::c_int as uint8_t,
        64 as libc::c_int as uint8_t,
        16 as libc::c_int as uint8_t,
        15 as libc::c_int as uint8_t,
        16 as libc::c_int as uint8_t,
        64 as libc::c_int as uint8_t,
    ];
    let mut runlevel: x264_run_level_t = x264_run_level_t {
        last: 0,
        mask: 0,
        level: [0; 18],
    };
    let mut i_total: libc::c_int = 0;
    let mut i_trailing: libc::c_int = 0;
    let mut i_total_zero: libc::c_int = 0;
    let mut i_suffix_length: libc::c_int = 0;
    let mut i_sign: libc::c_uint = 0;
    i_total = ((*h).quantf.coeff_level_run[ctx_block_cat as usize])
        .expect("non-null function pointer")(l, &mut runlevel);
    x264_8_run_before
        .as_mut_ptr()
        .offset(runlevel.mask as isize);
    i_total_zero = runlevel.last + 1 as libc::c_int - i_total;
    runlevel.level[(i_total + 0 as libc::c_int) as usize] = 2 as libc::c_int as dctcoef;
    runlevel.level[(i_total + 1 as libc::c_int) as usize] = 2 as libc::c_int as dctcoef;
    i_trailing = (((runlevel.level[0 as libc::c_int as usize] as libc::c_int + 1 as libc::c_int)
        | (1 as libc::c_int - runlevel.level[0 as libc::c_int as usize] as libc::c_int))
        >> 31 as libc::c_int)
        & 1 as libc::c_int
        | (((runlevel.level[1 as libc::c_int as usize] as libc::c_int + 1 as libc::c_int)
            | (1 as libc::c_int - runlevel.level[1 as libc::c_int as usize] as libc::c_int))
            >> 31 as libc::c_int)
            & 2 as libc::c_int
        | (((runlevel.level[2 as libc::c_int as usize] as libc::c_int + 1 as libc::c_int)
            | (1 as libc::c_int - runlevel.level[2 as libc::c_int as usize] as libc::c_int))
            >> 31 as libc::c_int)
            & 4 as libc::c_int;
    i_trailing = ctz_index[i_trailing as usize] as libc::c_int;
    i_sign = ((runlevel.level[2 as libc::c_int as usize] as libc::c_int >> 31 as libc::c_int)
        & 1 as libc::c_int
        | (runlevel.level[1 as libc::c_int as usize] as libc::c_int >> 31 as libc::c_int)
            & 2 as libc::c_int
        | (runlevel.level[0 as libc::c_int as usize] as libc::c_int >> 31 as libc::c_int)
            & 4 as libc::c_int) as libc::c_uint;
    i_sign >>= 3 as libc::c_int - i_trailing;
    bs_write(
        s,
        x264_coeff_token[nC as usize][(i_total - 1 as libc::c_int) as usize][i_trailing as usize]
            .i_size as libc::c_int,
        x264_coeff_token[nC as usize][(i_total - 1 as libc::c_int) as usize][i_trailing as usize]
            .i_bits as uint32_t,
    );
    i_suffix_length = (i_total > 10 as libc::c_int && i_trailing < 3 as libc::c_int) as libc::c_int;
    bs_write(s, i_trailing, i_sign);
    if i_trailing < i_total {
        let mut val: libc::c_int = runlevel.level[i_trailing as usize] as libc::c_int;
        let mut val_original: libc::c_int = runlevel.level[i_trailing as usize] as libc::c_int
            + 128 as libc::c_int / 2 as libc::c_int;
        val -= ((val >> 31 as libc::c_int) | 1 as libc::c_int)
            & -((i_trailing < 3 as libc::c_int) as libc::c_int);
        val += 128 as libc::c_int / 2 as libc::c_int;
        if (val_original as libc::c_uint) < 128 as libc::c_int as libc::c_uint {
            bs_write(
                s,
                x264_8_level_token[i_suffix_length as usize][val as usize].i_size as libc::c_int,
                x264_8_level_token[i_suffix_length as usize][val as usize].i_bits as uint32_t,
            );
            i_suffix_length = x264_8_level_token[i_suffix_length as usize][val_original as usize]
                .i_next as libc::c_int;
        } else {
            i_suffix_length = cavlc_block_residual_escape(
                h,
                i_suffix_length,
                val - 128 as libc::c_int / 2 as libc::c_int,
            );
        }
        let mut i: libc::c_int = i_trailing + 1 as libc::c_int;
        while i < i_total {
            val = runlevel.level[i as usize] as libc::c_int + 128 as libc::c_int / 2 as libc::c_int;
            if (val as libc::c_uint) < 128 as libc::c_int as libc::c_uint {
                bs_write(
                    s,
                    x264_8_level_token[i_suffix_length as usize][val as usize].i_size
                        as libc::c_int,
                    x264_8_level_token[i_suffix_length as usize][val as usize].i_bits as uint32_t,
                );
                i_suffix_length = x264_8_level_token[i_suffix_length as usize][val as usize].i_next
                    as libc::c_int;
            } else {
                i_suffix_length = cavlc_block_residual_escape(
                    h,
                    i_suffix_length,
                    val - 128 as libc::c_int / 2 as libc::c_int,
                );
            }
            i += 1;
        }
    }
    if ctx_block_cat == DCT_CHROMA_DC as libc::c_int {
        if i_total < 8 as libc::c_int >> (*h).mb.chroma_v_shift {
            let mut total_zeros: vlc_t =
                if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_420 as libc::c_int {
                    x264_total_zeros_2x2_dc[(i_total - 1 as libc::c_int) as usize]
                        [i_total_zero as usize]
                } else {
                    x264_total_zeros_2x4_dc[(i_total - 1 as libc::c_int) as usize]
                        [i_total_zero as usize]
                };
            bs_write(
                s,
                total_zeros.i_size as libc::c_int,
                total_zeros.i_bits as uint32_t,
            );
        }
    } else if (i_total as uint8_t as libc::c_int) < count_cat[ctx_block_cat as usize] as libc::c_int
    {
        bs_write(
            s,
            x264_total_zeros[(i_total - 1 as libc::c_int) as usize][i_total_zero as usize].i_size
                as libc::c_int,
            x264_total_zeros[(i_total - 1 as libc::c_int) as usize][i_total_zero as usize].i_bits
                as uint32_t,
        );
    }
    let mut zero_run_code: libc::c_int = x264_8_run_before[runlevel.mask as usize] as libc::c_int;
    bs_write(
        s,
        zero_run_code & 0x1f as libc::c_int,
        (zero_run_code >> 5 as libc::c_int) as uint32_t,
    );
    i_total
}
unsafe extern "C" fn cavlc_qp_delta(mut h: *mut x264_t) {
    let mut s: *mut bs_t = &mut (*h).out.bs;
    let mut i_dqp: libc::c_int = (*h).mb.i_qp - (*h).mb.i_last_qp;
    if (*h).mb.i_type == I_16x16 as libc::c_int
        && (*h).mb.i_cbp_luma | (*h).mb.i_cbp_chroma == 0
        && (*h).mb.cache.non_zero_count[x264_scan8[48 as libc::c_int as usize] as usize] == 0
        && (*h).mb.cache.non_zero_count
            [x264_scan8[(49 as libc::c_int + 0 as libc::c_int) as usize] as usize]
            == 0
        && (*h).mb.cache.non_zero_count
            [x264_scan8[(49 as libc::c_int + 1 as libc::c_int) as usize] as usize]
            == 0
        && (*h).mb.i_qp > (*h).mb.i_last_qp
    {
        (*h).mb.i_qp = (*h).mb.i_last_qp;
        i_dqp = 0 as libc::c_int;
    }
    if i_dqp != 0 {
        if i_dqp
            < -(51 as libc::c_int
                + 6 as libc::c_int * (8 as libc::c_int - 8 as libc::c_int)
                + 1 as libc::c_int)
                / 2 as libc::c_int
        {
            i_dqp += 51 as libc::c_int
                + 6 as libc::c_int * (8 as libc::c_int - 8 as libc::c_int)
                + 1 as libc::c_int;
        } else if i_dqp
            > (51 as libc::c_int + 6 as libc::c_int * (8 as libc::c_int - 8 as libc::c_int))
                / 2 as libc::c_int
        {
            i_dqp -= 51 as libc::c_int
                + 6 as libc::c_int * (8 as libc::c_int - 8 as libc::c_int)
                + 1 as libc::c_int;
        }
    }
    bs_write_se(s, i_dqp);
}
unsafe extern "C" fn cavlc_mvd(
    mut h: *mut x264_t,
    mut i_list: libc::c_int,
    mut idx: libc::c_int,
    mut width: libc::c_int,
) {
    let mut s: *mut bs_t = &mut (*h).out.bs;
    let mut mvp: [int16_t; 2] = [0; 2];
    x264_8_mb_predict_mv(h, i_list, idx, width, mvp.as_mut_ptr());
    bs_write_se(
        s,
        (*h).mb.cache.mv[i_list as usize][x264_scan8[idx as usize] as usize]
            [0 as libc::c_int as usize] as libc::c_int
            - mvp[0 as libc::c_int as usize] as libc::c_int,
    );
    bs_write_se(
        s,
        (*h).mb.cache.mv[i_list as usize][x264_scan8[idx as usize] as usize]
            [1 as libc::c_int as usize] as libc::c_int
            - mvp[1 as libc::c_int as usize] as libc::c_int,
    );
}
#[inline]
unsafe extern "C" fn cavlc_8x8_mvd(mut h: *mut x264_t, mut i: libc::c_int) {
    match (*h).mb.i_sub_partition[i as usize] as libc::c_int {
        3 => {
            cavlc_mvd(h, 0 as libc::c_int, 4 as libc::c_int * i, 2 as libc::c_int);
        }
        1 => {
            cavlc_mvd(
                h,
                0 as libc::c_int,
                4 as libc::c_int * i + 0 as libc::c_int,
                2 as libc::c_int,
            );
            cavlc_mvd(
                h,
                0 as libc::c_int,
                4 as libc::c_int * i + 2 as libc::c_int,
                2 as libc::c_int,
            );
        }
        2 => {
            cavlc_mvd(
                h,
                0 as libc::c_int,
                4 as libc::c_int * i + 0 as libc::c_int,
                1 as libc::c_int,
            );
            cavlc_mvd(
                h,
                0 as libc::c_int,
                4 as libc::c_int * i + 1 as libc::c_int,
                1 as libc::c_int,
            );
        }
        0 => {
            cavlc_mvd(
                h,
                0 as libc::c_int,
                4 as libc::c_int * i + 0 as libc::c_int,
                1 as libc::c_int,
            );
            cavlc_mvd(
                h,
                0 as libc::c_int,
                4 as libc::c_int * i + 1 as libc::c_int,
                1 as libc::c_int,
            );
            cavlc_mvd(
                h,
                0 as libc::c_int,
                4 as libc::c_int * i + 2 as libc::c_int,
                1 as libc::c_int,
            );
            cavlc_mvd(
                h,
                0 as libc::c_int,
                4 as libc::c_int * i + 3 as libc::c_int,
                1 as libc::c_int,
            );
        }
        _ => {}
    };
}
#[inline(always)]
unsafe extern "C" fn cavlc_macroblock_luma_residual(
    mut h: *mut x264_t,
    mut plane_count: libc::c_int,
) {
    if (*h).mb.b_transform_8x8 != 0 {
        let mut p: libc::c_int = 0 as libc::c_int;
        while p < plane_count {
            let mut i8: libc::c_int = 0 as libc::c_int;
            while i8 < 4 as libc::c_int {
                if (*h).mb.cache.non_zero_count
                    [x264_scan8[(p * 16 as libc::c_int + i8 * 4 as libc::c_int) as usize] as usize]
                    != 0
                {
                    ((*h).zigzagf.interleave_8x8_cavlc).expect("non-null function pointer")(
                        ((*h).dct.luma4x4
                            [(p * 16 as libc::c_int + i8 * 4 as libc::c_int) as usize])
                            .as_mut_ptr(),
                        ((*h).dct.luma8x8[(p * 4 as libc::c_int + i8) as usize]).as_mut_ptr(),
                        &mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
                            *x264_scan8
                                .as_ptr()
                                .offset((p * 16 as libc::c_int + i8 * 4 as libc::c_int) as isize)
                                as isize,
                        ),
                    );
                }
                i8 += 1;
                i8;
            }
            p += 1;
        }
    }
    let mut p_0: libc::c_int = 0 as libc::c_int;
    while p_0 < plane_count {
        let mut i8_0: libc::c_int = 0 as libc::c_int;
        let mut msk: libc::c_int = (*h).mb.i_cbp_luma;
        let mut skip: libc::c_int = 0;
        while msk != 0 && {
            skip = x264_ctz_4bit(msk as uint32_t);
            i8_0 += skip;
            msk >>= skip + 1 as libc::c_int;
            1 as libc::c_int != 0
        } {
            let mut i4: libc::c_int = 0 as libc::c_int;
            while i4 < 4 as libc::c_int {
                let mut nC: libc::c_int =
                    if DCT_LUMA_4x4 as libc::c_int == DCT_CHROMA_DC as libc::c_int {
                        5 as libc::c_int - (*h).mb.chroma_v_shift
                    } else {
                        ct_index[x264_mb_predict_non_zero_code(
                            h,
                            if DCT_LUMA_4x4 as libc::c_int == DCT_LUMA_DC as libc::c_int {
                                (i4 + i8_0 * 4 as libc::c_int + p_0 * 16 as libc::c_int
                                    - 48 as libc::c_int)
                                    * 16 as libc::c_int
                            } else {
                                i4 + i8_0 * 4 as libc::c_int + p_0 * 16 as libc::c_int
                            },
                        ) as usize] as libc::c_int
                    };
                let mut nnz: *mut uint8_t =
                    &mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
                        *x264_scan8.as_ptr().offset(
                            (i4 + i8_0 * 4 as libc::c_int + p_0 * 16 as libc::c_int) as isize,
                        ) as isize,
                    ) as *mut uint8_t;
                if *nnz == 0 {
                    bs_write(
                        &mut (*h).out.bs,
                        x264_coeff0_token[nC as usize].i_size as libc::c_int,
                        x264_coeff0_token[nC as usize].i_bits as uint32_t,
                    );
                } else {
                    *nnz = cavlc_block_residual_internal(
                        h,
                        DCT_LUMA_4x4 as libc::c_int,
                        ((*h).dct.luma4x4
                            [(i4 + i8_0 * 4 as libc::c_int + p_0 * 16 as libc::c_int) as usize])
                            .as_mut_ptr(),
                        nC,
                    ) as uint8_t;
                }
                i4 += 1;
            }
            i8_0 += 1;
        }
        p_0 += 1;
    }
}
unsafe extern "C" fn cavlc_mb_header_i(
    mut h: *mut x264_t,
    mut i_mb_type: libc::c_int,
    mut i_mb_i_offset: libc::c_int,
    mut chroma: libc::c_int,
) {
    let mut s: *mut bs_t = &mut (*h).out.bs;
    if i_mb_type == I_16x16 as libc::c_int {
        bs_write_ue(
            s,
            i_mb_i_offset
                + 1 as libc::c_int
                + x264_mb_pred_mode16x16_fix[(*h).mb.i_intra16x16_pred_mode as usize]
                    as libc::c_int
                + (*h).mb.i_cbp_chroma * 4 as libc::c_int
                + (if (*h).mb.i_cbp_luma == 0 as libc::c_int {
                    0 as libc::c_int
                } else {
                    12 as libc::c_int
                }),
        );
    } else {
        let mut di: libc::c_int = if i_mb_type == I_8x8 as libc::c_int {
            4 as libc::c_int
        } else {
            1 as libc::c_int
        };
        bs_write_ue(s, i_mb_i_offset + 0 as libc::c_int);
        if (*((*h).pps).as_mut_ptr()).b_transform_8x8_mode != 0 {
            bs_write1(s, (*h).mb.b_transform_8x8 as uint32_t);
        }
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < 16 as libc::c_int {
            let mut i_pred: libc::c_int = x264_mb_predict_intra4x4_mode(h, i);
            let mut i_mode: libc::c_int = x264_mb_pred_mode4x4_fix[((*h).mb.cache.intra4x4_pred_mode
                [x264_scan8[i as usize] as usize]
                as libc::c_int
                + 1 as libc::c_int)
                as usize] as libc::c_int;
            if i_pred == i_mode {
                bs_write1(s, 1 as libc::c_int as uint32_t);
            } else {
                bs_write(
                    s,
                    4 as libc::c_int,
                    (i_mode - (i_mode > i_pred) as libc::c_int) as uint32_t,
                );
            }
            i += di;
        }
    }
    if chroma != 0 {
        bs_write_ue(
            s,
            x264_mb_chroma_pred_mode_fix[(*h).mb.i_chroma_pred_mode as usize] as libc::c_int,
        );
    }
}
#[inline(always)]
unsafe extern "C" fn cavlc_mb_header_p(
    mut h: *mut x264_t,
    mut i_mb_type: libc::c_int,
    mut chroma: libc::c_int,
) {
    let mut s: *mut bs_t = &mut (*h).out.bs;
    if i_mb_type == P_L0 as libc::c_int {
        if (*h).mb.i_partition == D_16x16 as libc::c_int {
            bs_write1(s, 1 as libc::c_int as uint32_t);
            if (*h).mb.pic.i_fref[0 as libc::c_int as usize] > 1 as libc::c_int {
                bs_write_te(
                    s,
                    (*h).mb.pic.i_fref[0 as libc::c_int as usize] - 1 as libc::c_int,
                    (*h).mb.cache.ref_0[0 as libc::c_int as usize]
                        [x264_scan8[0 as libc::c_int as usize] as usize]
                        as libc::c_int,
                );
            }
            cavlc_mvd(h, 0 as libc::c_int, 0 as libc::c_int, 4 as libc::c_int);
        } else if (*h).mb.i_partition == D_16x8 as libc::c_int {
            bs_write_ue(s, 1 as libc::c_int);
            if (*h).mb.pic.i_fref[0 as libc::c_int as usize] > 1 as libc::c_int {
                bs_write_te(
                    s,
                    (*h).mb.pic.i_fref[0 as libc::c_int as usize] - 1 as libc::c_int,
                    (*h).mb.cache.ref_0[0 as libc::c_int as usize]
                        [x264_scan8[0 as libc::c_int as usize] as usize]
                        as libc::c_int,
                );
                bs_write_te(
                    s,
                    (*h).mb.pic.i_fref[0 as libc::c_int as usize] - 1 as libc::c_int,
                    (*h).mb.cache.ref_0[0 as libc::c_int as usize]
                        [x264_scan8[8 as libc::c_int as usize] as usize]
                        as libc::c_int,
                );
            }
            cavlc_mvd(h, 0 as libc::c_int, 0 as libc::c_int, 4 as libc::c_int);
            cavlc_mvd(h, 0 as libc::c_int, 8 as libc::c_int, 4 as libc::c_int);
        } else if (*h).mb.i_partition == D_8x16 as libc::c_int {
            bs_write_ue(s, 2 as libc::c_int);
            if (*h).mb.pic.i_fref[0 as libc::c_int as usize] > 1 as libc::c_int {
                bs_write_te(
                    s,
                    (*h).mb.pic.i_fref[0 as libc::c_int as usize] - 1 as libc::c_int,
                    (*h).mb.cache.ref_0[0 as libc::c_int as usize]
                        [x264_scan8[0 as libc::c_int as usize] as usize]
                        as libc::c_int,
                );
                bs_write_te(
                    s,
                    (*h).mb.pic.i_fref[0 as libc::c_int as usize] - 1 as libc::c_int,
                    (*h).mb.cache.ref_0[0 as libc::c_int as usize]
                        [x264_scan8[4 as libc::c_int as usize] as usize]
                        as libc::c_int,
                );
            }
            cavlc_mvd(h, 0 as libc::c_int, 0 as libc::c_int, 2 as libc::c_int);
            cavlc_mvd(h, 0 as libc::c_int, 4 as libc::c_int, 2 as libc::c_int);
        }
    } else if i_mb_type == P_8x8 as libc::c_int {
        let mut b_sub_ref: libc::c_int = 0;
        if (*h).mb.cache.ref_0[0 as libc::c_int as usize]
            [x264_scan8[0 as libc::c_int as usize] as usize] as libc::c_int
            | (*h).mb.cache.ref_0[0 as libc::c_int as usize]
                [x264_scan8[4 as libc::c_int as usize] as usize] as libc::c_int
            | (*h).mb.cache.ref_0[0 as libc::c_int as usize]
                [x264_scan8[8 as libc::c_int as usize] as usize] as libc::c_int
            | (*h).mb.cache.ref_0[0 as libc::c_int as usize]
                [x264_scan8[12 as libc::c_int as usize] as usize] as libc::c_int
            == 0 as libc::c_int
        {
            bs_write_ue(s, 4 as libc::c_int);
            b_sub_ref = 0 as libc::c_int;
        } else {
            bs_write_ue(s, 3 as libc::c_int);
            b_sub_ref = 1 as libc::c_int;
        }
        if (*h).param.analyse.inter & 0x20 as libc::c_uint != 0 {
            let mut i: libc::c_int = 0 as libc::c_int;
            while i < 4 as libc::c_int {
                bs_write_ue(
                    s,
                    subpartition_p_to_golomb[(*h).mb.i_sub_partition[i as usize] as usize]
                        as libc::c_int,
                );
                i += 1;
            }
        } else {
            bs_write(s, 4 as libc::c_int, 0xf as libc::c_int as uint32_t);
        }
        if b_sub_ref != 0 {
            bs_write_te(
                s,
                (*h).mb.pic.i_fref[0 as libc::c_int as usize] - 1 as libc::c_int,
                (*h).mb.cache.ref_0[0 as libc::c_int as usize]
                    [x264_scan8[0 as libc::c_int as usize] as usize] as libc::c_int,
            );
            bs_write_te(
                s,
                (*h).mb.pic.i_fref[0 as libc::c_int as usize] - 1 as libc::c_int,
                (*h).mb.cache.ref_0[0 as libc::c_int as usize]
                    [x264_scan8[4 as libc::c_int as usize] as usize] as libc::c_int,
            );
            bs_write_te(
                s,
                (*h).mb.pic.i_fref[0 as libc::c_int as usize] - 1 as libc::c_int,
                (*h).mb.cache.ref_0[0 as libc::c_int as usize]
                    [x264_scan8[8 as libc::c_int as usize] as usize] as libc::c_int,
            );
            bs_write_te(
                s,
                (*h).mb.pic.i_fref[0 as libc::c_int as usize] - 1 as libc::c_int,
                (*h).mb.cache.ref_0[0 as libc::c_int as usize]
                    [x264_scan8[12 as libc::c_int as usize] as usize]
                    as libc::c_int,
            );
        }
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < 4 as libc::c_int {
            cavlc_8x8_mvd(h, i_0);
            i_0 += 1;
        }
    } else {
        cavlc_mb_header_i(h, i_mb_type, 5 as libc::c_int, chroma);
    };
}
#[inline(always)]
unsafe extern "C" fn cavlc_mb_header_b(
    mut h: *mut x264_t,
    mut i_mb_type: libc::c_int,
    mut chroma: libc::c_int,
) {
    let mut s: *mut bs_t = &mut (*h).out.bs;
    if i_mb_type == B_8x8 as libc::c_int {
        bs_write_ue(s, 22 as libc::c_int);
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            bs_write_ue(
                s,
                subpartition_b_to_golomb[(*h).mb.i_sub_partition[i as usize] as usize]
                    as libc::c_int,
            );
            i += 1;
        }
        if (*h).mb.pic.i_fref[0 as libc::c_int as usize] > 1 as libc::c_int {
            let mut i_0: libc::c_int = 0 as libc::c_int;
            while i_0 < 4 as libc::c_int {
                if x264_mb_partition_listX_table[0 as libc::c_int as usize]
                    [(*h).mb.i_sub_partition[i_0 as usize] as usize]
                    != 0
                {
                    bs_write_te(
                        s,
                        (*h).mb.pic.i_fref[0 as libc::c_int as usize] - 1 as libc::c_int,
                        (*h).mb.cache.ref_0[0 as libc::c_int as usize]
                            [x264_scan8[(i_0 * 4 as libc::c_int) as usize] as usize]
                            as libc::c_int,
                    );
                }
                i_0 += 1;
            }
        }
        if (*h).mb.pic.i_fref[1 as libc::c_int as usize] > 1 as libc::c_int {
            let mut i_1: libc::c_int = 0 as libc::c_int;
            while i_1 < 4 as libc::c_int {
                if x264_mb_partition_listX_table[1 as libc::c_int as usize]
                    [(*h).mb.i_sub_partition[i_1 as usize] as usize]
                    != 0
                {
                    bs_write_te(
                        s,
                        (*h).mb.pic.i_fref[1 as libc::c_int as usize] - 1 as libc::c_int,
                        (*h).mb.cache.ref_0[1 as libc::c_int as usize]
                            [x264_scan8[(i_1 * 4 as libc::c_int) as usize] as usize]
                            as libc::c_int,
                    );
                }
                i_1 += 1;
            }
        }
        let mut i_2: libc::c_int = 0 as libc::c_int;
        while i_2 < 4 as libc::c_int {
            if x264_mb_partition_listX_table[0 as libc::c_int as usize]
                [(*h).mb.i_sub_partition[i_2 as usize] as usize]
                != 0
            {
                cavlc_mvd(
                    h,
                    0 as libc::c_int,
                    4 as libc::c_int * i_2,
                    2 as libc::c_int,
                );
            }
            i_2 += 1;
        }
        let mut i_3: libc::c_int = 0 as libc::c_int;
        while i_3 < 4 as libc::c_int {
            if x264_mb_partition_listX_table[1 as libc::c_int as usize]
                [(*h).mb.i_sub_partition[i_3 as usize] as usize]
                != 0
            {
                cavlc_mvd(
                    h,
                    1 as libc::c_int,
                    4 as libc::c_int * i_3,
                    2 as libc::c_int,
                );
            }
            i_3 += 1;
        }
    } else if i_mb_type >= B_L0_L0 as libc::c_int && i_mb_type <= B_BI_BI as libc::c_int {
        let mut b_list: *const [uint8_t; 2] =
            (x264_mb_type_list_table[i_mb_type as usize]).as_ptr();
        let i_ref0_max: libc::c_int =
            (*h).mb.pic.i_fref[0 as libc::c_int as usize] - 1 as libc::c_int;
        let i_ref1_max: libc::c_int =
            (*h).mb.pic.i_fref[1 as libc::c_int as usize] - 1 as libc::c_int;
        bs_write_ue(
            s,
            mb_type_b_to_golomb[((*h).mb.i_partition - D_16x8 as libc::c_int) as usize]
                [(i_mb_type - B_L0_L0 as libc::c_int) as usize] as libc::c_int,
        );
        if (*h).mb.i_partition == D_16x16 as libc::c_int {
            if i_ref0_max != 0
                && (*b_list.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
                    as libc::c_int
                    != 0
            {
                bs_write_te(
                    s,
                    i_ref0_max,
                    (*h).mb.cache.ref_0[0 as libc::c_int as usize]
                        [x264_scan8[0 as libc::c_int as usize] as usize]
                        as libc::c_int,
                );
            }
            if i_ref1_max != 0
                && (*b_list.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize]
                    as libc::c_int
                    != 0
            {
                bs_write_te(
                    s,
                    i_ref1_max,
                    (*h).mb.cache.ref_0[1 as libc::c_int as usize]
                        [x264_scan8[0 as libc::c_int as usize] as usize]
                        as libc::c_int,
                );
            }
            if (*b_list.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] != 0 {
                cavlc_mvd(h, 0 as libc::c_int, 0 as libc::c_int, 4 as libc::c_int);
            }
            if (*b_list.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] != 0 {
                cavlc_mvd(h, 1 as libc::c_int, 0 as libc::c_int, 4 as libc::c_int);
            }
        } else {
            if i_ref0_max != 0
                && (*b_list.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
                    as libc::c_int
                    != 0
            {
                bs_write_te(
                    s,
                    i_ref0_max,
                    (*h).mb.cache.ref_0[0 as libc::c_int as usize]
                        [x264_scan8[0 as libc::c_int as usize] as usize]
                        as libc::c_int,
                );
            }
            if i_ref0_max != 0
                && (*b_list.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize]
                    as libc::c_int
                    != 0
            {
                bs_write_te(
                    s,
                    i_ref0_max,
                    (*h).mb.cache.ref_0[0 as libc::c_int as usize]
                        [x264_scan8[12 as libc::c_int as usize] as usize]
                        as libc::c_int,
                );
            }
            if i_ref1_max != 0
                && (*b_list.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize]
                    as libc::c_int
                    != 0
            {
                bs_write_te(
                    s,
                    i_ref1_max,
                    (*h).mb.cache.ref_0[1 as libc::c_int as usize]
                        [x264_scan8[0 as libc::c_int as usize] as usize]
                        as libc::c_int,
                );
            }
            if i_ref1_max != 0
                && (*b_list.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize]
                    as libc::c_int
                    != 0
            {
                bs_write_te(
                    s,
                    i_ref1_max,
                    (*h).mb.cache.ref_0[1 as libc::c_int as usize]
                        [x264_scan8[12 as libc::c_int as usize] as usize]
                        as libc::c_int,
                );
            }
            if (*h).mb.i_partition == D_16x8 as libc::c_int {
                if (*b_list.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] != 0 {
                    cavlc_mvd(h, 0 as libc::c_int, 0 as libc::c_int, 4 as libc::c_int);
                }
                if (*b_list.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize] != 0 {
                    cavlc_mvd(h, 0 as libc::c_int, 8 as libc::c_int, 4 as libc::c_int);
                }
                if (*b_list.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] != 0 {
                    cavlc_mvd(h, 1 as libc::c_int, 0 as libc::c_int, 4 as libc::c_int);
                }
                if (*b_list.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize] != 0 {
                    cavlc_mvd(h, 1 as libc::c_int, 8 as libc::c_int, 4 as libc::c_int);
                }
            } else {
                if (*b_list.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] != 0 {
                    cavlc_mvd(h, 0 as libc::c_int, 0 as libc::c_int, 2 as libc::c_int);
                }
                if (*b_list.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize] != 0 {
                    cavlc_mvd(h, 0 as libc::c_int, 4 as libc::c_int, 2 as libc::c_int);
                }
                if (*b_list.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] != 0 {
                    cavlc_mvd(h, 1 as libc::c_int, 0 as libc::c_int, 2 as libc::c_int);
                }
                if (*b_list.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize] != 0 {
                    cavlc_mvd(h, 1 as libc::c_int, 4 as libc::c_int, 2 as libc::c_int);
                }
            }
        }
    } else if i_mb_type == B_DIRECT as libc::c_int {
        bs_write1(s, 1 as libc::c_int as uint32_t);
    } else {
        cavlc_mb_header_i(h, i_mb_type, 23 as libc::c_int, chroma);
    };
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_macroblock_write_cavlc(mut h: *mut x264_t) {
    let mut s: *mut bs_t = &mut (*h).out.bs;
    let i_mb_type: libc::c_int = (*h).mb.i_type;
    let mut plane_count: libc::c_int =
        if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as libc::c_int {
            3 as libc::c_int
        } else {
            1 as libc::c_int
        };
    let mut chroma: libc::c_int = ((*((*h).sps).as_mut_ptr()).i_chroma_format_idc
        == CHROMA_420 as libc::c_int
        || (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_422 as libc::c_int)
        as libc::c_int;
    let i_mb_pos_start: libc::c_int = bs_pos(s);
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
        bs_write1(s, (*h).mb.b_interlaced as uint32_t);
        (*h).mb.field_decoding_flag = (*h).mb.b_interlaced;
    }
    if i_mb_type == I_PCM as libc::c_int {
        static mut i_offsets: [uint8_t; 3] = [
            5 as libc::c_int as uint8_t,
            23 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
        ];
        let mut p_start: *mut uint8_t = (*s).p_start;
        bs_write_ue(
            s,
            i_offsets[(*h).sh.i_type as usize] as libc::c_int + 25 as libc::c_int,
        );
        i_mb_pos_tex = bs_pos(s);
        (*h).stat.frame.i_mv_bits += i_mb_pos_tex - i_mb_pos_start;
        bs_align_0(s);
        let mut p: libc::c_int = 0 as libc::c_int;
        while p < plane_count {
            let mut i: libc::c_int = 0 as libc::c_int;
            while i < 256 as libc::c_int {
                bs_write(
                    s,
                    8 as libc::c_int,
                    *((*h).mb.pic.p_fenc[p as usize]).offset(i as isize) as uint32_t,
                );
                i += 1;
            }
            p += 1;
        }
        if chroma != 0 {
            let mut ch: libc::c_int = 1 as libc::c_int;
            while ch < 3 as libc::c_int {
                let mut i_0: libc::c_int = 0 as libc::c_int;
                while i_0 < 16 as libc::c_int >> (*h).mb.chroma_v_shift {
                    let mut j: libc::c_int = 0 as libc::c_int;
                    while j < 8 as libc::c_int {
                        bs_write(
                            s,
                            8 as libc::c_int,
                            *((*h).mb.pic.p_fenc[ch as usize])
                                .offset((i_0 * 16 as libc::c_int + j) as isize)
                                as uint32_t,
                        );
                        j += 1;
                    }
                    i_0 += 1;
                }
                ch += 1;
            }
        }
        bs_init(
            s,
            (*s).p as *mut libc::c_void,
            ((*s).p_end).offset_from((*s).p) as libc::c_long as libc::c_int,
        );
        (*s).p_start = p_start;
        (*h).stat.frame.i_tex_bits += bs_pos(s) - i_mb_pos_tex;
        return;
    }
    if (*h).sh.i_type == SLICE_TYPE_P as libc::c_int {
        cavlc_mb_header_p(h, i_mb_type, chroma);
    } else if (*h).sh.i_type == SLICE_TYPE_B as libc::c_int {
        cavlc_mb_header_b(h, i_mb_type, chroma);
    } else {
        cavlc_mb_header_i(h, i_mb_type, 0 as libc::c_int, chroma);
    }
    i_mb_pos_tex = bs_pos(s);
    (*h).stat.frame.i_mv_bits += i_mb_pos_tex - i_mb_pos_start;
    if i_mb_type != I_16x16 as libc::c_int {
        bs_write_ue(
            s,
            cbp_to_golomb[chroma as usize][(i_mb_type == I_4x4 as libc::c_int
                || i_mb_type == I_8x8 as libc::c_int
                || i_mb_type == I_16x16 as libc::c_int
                || i_mb_type == I_PCM as libc::c_int)
                as libc::c_int as usize]
                [(((*h).mb.i_cbp_chroma << 4 as libc::c_int) | (*h).mb.i_cbp_luma) as usize]
                as libc::c_int,
        );
    }
    if x264_mb_transform_8x8_allowed(h) != 0 && (*h).mb.i_cbp_luma != 0 {
        bs_write1(s, (*h).mb.b_transform_8x8 as uint32_t);
    }
    if i_mb_type == I_16x16 as libc::c_int {
        cavlc_qp_delta(h);
        let mut p_0: libc::c_int = 0 as libc::c_int;
        while p_0 < plane_count {
            let mut nC: libc::c_int = if DCT_LUMA_DC as libc::c_int == DCT_CHROMA_DC as libc::c_int
            {
                5 as libc::c_int - (*h).mb.chroma_v_shift
            } else {
                ct_index[x264_mb_predict_non_zero_code(
                    h,
                    if DCT_LUMA_DC as libc::c_int == DCT_LUMA_DC as libc::c_int {
                        (48 as libc::c_int + p_0 - 48 as libc::c_int) * 16 as libc::c_int
                    } else {
                        48 as libc::c_int + p_0
                    },
                ) as usize] as libc::c_int
            };
            let mut nnz: *mut uint8_t = &mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
                *x264_scan8
                    .as_ptr()
                    .offset((48 as libc::c_int + p_0) as isize) as isize,
            ) as *mut uint8_t;
            if *nnz == 0 {
                bs_write(
                    &mut (*h).out.bs,
                    x264_coeff0_token[nC as usize].i_size as libc::c_int,
                    x264_coeff0_token[nC as usize].i_bits as uint32_t,
                );
            } else {
                *nnz = cavlc_block_residual_internal(
                    h,
                    DCT_LUMA_DC as libc::c_int,
                    ((*h).dct.luma16x16_dc[p_0 as usize]).as_mut_ptr(),
                    nC,
                ) as uint8_t;
            }
            if (*h).mb.i_cbp_luma != 0 {
                let mut i_1: libc::c_int = p_0 * 16 as libc::c_int;
                while i_1 < p_0 * 16 as libc::c_int + 16 as libc::c_int {
                    let mut nC_0: libc::c_int =
                        if DCT_LUMA_AC as libc::c_int == DCT_CHROMA_DC as libc::c_int {
                            5 as libc::c_int - (*h).mb.chroma_v_shift
                        } else {
                            ct_index[x264_mb_predict_non_zero_code(
                                h,
                                if DCT_LUMA_AC as libc::c_int == DCT_LUMA_DC as libc::c_int {
                                    (i_1 - 48 as libc::c_int) * 16 as libc::c_int
                                } else {
                                    i_1
                                },
                            ) as usize] as libc::c_int
                        };
                    let mut nnz_0: *mut uint8_t = &mut *((*h).mb.cache.non_zero_count)
                        .as_mut_ptr()
                        .offset(*x264_scan8.as_ptr().offset(i_1 as isize) as isize)
                        as *mut uint8_t;
                    if *nnz_0 == 0 {
                        bs_write(
                            &mut (*h).out.bs,
                            x264_coeff0_token[nC_0 as usize].i_size as libc::c_int,
                            x264_coeff0_token[nC_0 as usize].i_bits as uint32_t,
                        );
                    } else {
                        *nnz_0 = cavlc_block_residual_internal(
                            h,
                            DCT_LUMA_AC as libc::c_int,
                            ((*h).dct.luma4x4[i_1 as usize])
                                .as_mut_ptr()
                                .offset(1 as libc::c_int as isize),
                            nC_0,
                        ) as uint8_t;
                    }
                    i_1 += 1;
                }
            }
            p_0 += 1;
        }
    } else if (*h).mb.i_cbp_luma | (*h).mb.i_cbp_chroma != 0 {
        cavlc_qp_delta(h);
        cavlc_macroblock_luma_residual(h, plane_count);
    }
    if (*h).mb.i_cbp_chroma != 0 {
        let mut nC_1: libc::c_int = if DCT_CHROMA_DC as libc::c_int == DCT_CHROMA_DC as libc::c_int
        {
            5 as libc::c_int - (*h).mb.chroma_v_shift
        } else {
            ct_index[x264_mb_predict_non_zero_code(
                h,
                if DCT_CHROMA_DC as libc::c_int == DCT_LUMA_DC as libc::c_int {
                    (49 as libc::c_int + 0 as libc::c_int - 48 as libc::c_int) * 16 as libc::c_int
                } else {
                    49 as libc::c_int + 0 as libc::c_int
                },
            ) as usize] as libc::c_int
        };
        let mut nnz_1: *mut uint8_t = &mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
            *x264_scan8
                .as_ptr()
                .offset((49 as libc::c_int + 0 as libc::c_int) as isize) as isize,
        ) as *mut uint8_t;
        if *nnz_1 == 0 {
            bs_write(
                &mut (*h).out.bs,
                x264_coeff0_token[nC_1 as usize].i_size as libc::c_int,
                x264_coeff0_token[nC_1 as usize].i_bits as uint32_t,
            );
        } else {
            *nnz_1 = cavlc_block_residual_internal(
                h,
                DCT_CHROMA_DC as libc::c_int,
                ((*h).dct.chroma_dc[0 as libc::c_int as usize]).as_mut_ptr(),
                nC_1,
            ) as uint8_t;
        }
        let mut nC_2: libc::c_int = if DCT_CHROMA_DC as libc::c_int == DCT_CHROMA_DC as libc::c_int
        {
            5 as libc::c_int - (*h).mb.chroma_v_shift
        } else {
            ct_index[x264_mb_predict_non_zero_code(
                h,
                if DCT_CHROMA_DC as libc::c_int == DCT_LUMA_DC as libc::c_int {
                    (49 as libc::c_int + 1 as libc::c_int - 48 as libc::c_int) * 16 as libc::c_int
                } else {
                    49 as libc::c_int + 1 as libc::c_int
                },
            ) as usize] as libc::c_int
        };
        let mut nnz_2: *mut uint8_t = &mut *((*h).mb.cache.non_zero_count).as_mut_ptr().offset(
            *x264_scan8
                .as_ptr()
                .offset((49 as libc::c_int + 1 as libc::c_int) as isize) as isize,
        ) as *mut uint8_t;
        if *nnz_2 == 0 {
            bs_write(
                &mut (*h).out.bs,
                x264_coeff0_token[nC_2 as usize].i_size as libc::c_int,
                x264_coeff0_token[nC_2 as usize].i_bits as uint32_t,
            );
        } else {
            *nnz_2 = cavlc_block_residual_internal(
                h,
                DCT_CHROMA_DC as libc::c_int,
                ((*h).dct.chroma_dc[1 as libc::c_int as usize]).as_mut_ptr(),
                nC_2,
            ) as uint8_t;
        }
        if (*h).mb.i_cbp_chroma == 2 as libc::c_int {
            let mut step: libc::c_int = (8 as libc::c_int) << (*h).mb.chroma_v_shift;
            let mut i_2: libc::c_int = 16 as libc::c_int;
            while i_2 < 3 as libc::c_int * 16 as libc::c_int {
                let mut j_0: libc::c_int = i_2;
                while j_0 < i_2 + 4 as libc::c_int {
                    let mut nC_3: libc::c_int =
                        if DCT_CHROMA_AC as libc::c_int == DCT_CHROMA_DC as libc::c_int {
                            5 as libc::c_int - (*h).mb.chroma_v_shift
                        } else {
                            ct_index[x264_mb_predict_non_zero_code(
                                h,
                                if DCT_CHROMA_AC as libc::c_int == DCT_LUMA_DC as libc::c_int {
                                    (j_0 - 48 as libc::c_int) * 16 as libc::c_int
                                } else {
                                    j_0
                                },
                            ) as usize] as libc::c_int
                        };
                    let mut nnz_3: *mut uint8_t = &mut *((*h).mb.cache.non_zero_count)
                        .as_mut_ptr()
                        .offset(*x264_scan8.as_ptr().offset(j_0 as isize) as isize)
                        as *mut uint8_t;
                    if *nnz_3 == 0 {
                        bs_write(
                            &mut (*h).out.bs,
                            x264_coeff0_token[nC_3 as usize].i_size as libc::c_int,
                            x264_coeff0_token[nC_3 as usize].i_bits as uint32_t,
                        );
                    } else {
                        *nnz_3 = cavlc_block_residual_internal(
                            h,
                            DCT_CHROMA_AC as libc::c_int,
                            ((*h).dct.luma4x4[j_0 as usize])
                                .as_mut_ptr()
                                .offset(1 as libc::c_int as isize),
                            nC_3,
                        ) as uint8_t;
                    }
                    j_0 += 1;
                }
                i_2 += step;
            }
        }
    }
    (*h).stat.frame.i_tex_bits += bs_pos(s) - i_mb_pos_tex;
}
