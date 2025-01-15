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
use encoder_file::set_file::c2_defs::C2RustUnnamed_20;

use crate::types::*;
extern "C" {
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn log2f(_: libc::c_float) -> libc::c_float;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn x264_malloc(_: int64_t) -> *mut libc::c_void;
    fn x264_free(_: *mut libc::c_void);
    fn x264_param2string(p: *mut x264_param_t, b_res: libc::c_int) -> *mut libc::c_char;
    fn x264_8_log(h: *mut x264_t, i_level: libc::c_int, psz_fmt: *const libc::c_char, _: ...);
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
unsafe extern "C" fn bs_realign(mut s: *mut bs_t) {
    let mut offset: libc::c_int =
        ((*s).p as intptr_t & 3 as libc::c_int as intptr_t) as libc::c_int;
    if offset != 0 {
        (*s).p = ((*s).p).offset(-(offset as isize));
        (*s).i_left = (::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
            .wrapping_sub(offset as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong) as libc::c_int;
        (*s).cur_bits = endian_fix32((*((*s).p as *mut x264_union32_t)).i) as uintptr_t;
        (*s).cur_bits >>= (4 as libc::c_int - offset) * 8 as libc::c_int;
    }
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
unsafe extern "C" fn bs_write32(mut s: *mut bs_t, mut i_bits: uint32_t) {
    bs_write(s, 16 as libc::c_int, i_bits >> 16 as libc::c_int);
    bs_write(s, 16 as libc::c_int, i_bits);
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
unsafe extern "C" fn bs_align_10(mut s: *mut bs_t) {
    if (*s).i_left & 7 as libc::c_int != 0 {
        bs_write(
            s,
            (*s).i_left & 7 as libc::c_int,
            ((1 as libc::c_int) << (((*s).i_left & 7 as libc::c_int) - 1 as libc::c_int))
                as uint32_t,
        );
    }
    bs_flush(s);
}
#[inline]
unsafe extern "C" fn bs_write_ue_big(mut s: *mut bs_t, mut val: libc::c_uint) {
    let mut size: libc::c_int = 0 as libc::c_int;
    val = val.wrapping_add(1);
    let mut tmp: libc::c_int = val as libc::c_int;
    if tmp >= 0x10000 as libc::c_int {
        size = 32 as libc::c_int;
        tmp >>= 16 as libc::c_int;
    }
    if tmp >= 0x100 as libc::c_int {
        size += 16 as libc::c_int;
        tmp >>= 8 as libc::c_int;
    }
    size += x264_ue_size_tab[tmp as usize] as libc::c_int;
    bs_write(s, size >> 1 as libc::c_int, 0 as libc::c_int as uint32_t);
    bs_write(s, (size >> 1 as libc::c_int) + 1 as libc::c_int, val);
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
unsafe extern "C" fn bs_rbsp_trailing(mut s: *mut bs_t) {
    bs_write1(s, 1 as libc::c_int as uint32_t);
    bs_write(
        s,
        (*s).i_left & 7 as libc::c_int,
        0 as libc::c_int as uint32_t,
    );
}
#[inline(always)]
unsafe extern "C" fn bs_size_se(mut val: libc::c_int) -> libc::c_int {
    let mut tmp: libc::c_int = 1 as libc::c_int - val * 2 as libc::c_int;
    if tmp < 0 as libc::c_int {
        tmp = val * 2 as libc::c_int;
    }
    if tmp < 256 as libc::c_int {
        x264_ue_size_tab[tmp as usize] as libc::c_int
    } else {
        x264_ue_size_tab[(tmp >> 8 as libc::c_int) as usize] as libc::c_int + 16 as libc::c_int
    }
}
unsafe extern "C" fn transpose(mut buf: *mut uint8_t, mut w: libc::c_int) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < w {
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < i {
            let mut t: uint8_t = *buf.offset((w * i + j) as isize);
            *buf.offset((w * i + j) as isize) = *buf.offset((w * j + i) as isize);
            *buf.offset((w * j + i) as isize) = t;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn scaling_list_write(
    mut s: *mut bs_t,
    mut sps: *mut x264_sps_t,
    mut idx: libc::c_int,
) {
    let len: libc::c_int = if idx < 4 as libc::c_int {
        16 as libc::c_int
    } else {
        64 as libc::c_int
    };
    let mut zigzag: *const uint8_t = if idx < 4 as libc::c_int {
        (x264_zigzag_scan4[0 as libc::c_int as usize]).as_ptr()
    } else {
        (x264_zigzag_scan8[0 as libc::c_int as usize]).as_ptr()
    };
    let mut list: *const uint8_t = (*sps).scaling_list[idx as usize];
    let mut def_list: *const uint8_t = if idx == CQM_4IC as libc::c_int {
        (*sps).scaling_list[CQM_4IY as libc::c_int as usize]
    } else if idx == CQM_4PC as libc::c_int {
        (*sps).scaling_list[CQM_4PY as libc::c_int as usize]
    } else if idx == CQM_8IC as libc::c_int + 4 as libc::c_int {
        (*sps).scaling_list[(CQM_8IY as libc::c_int + 4 as libc::c_int) as usize]
    } else if idx == CQM_8PC as libc::c_int + 4 as libc::c_int {
        (*sps).scaling_list[(CQM_8PY as libc::c_int + 4 as libc::c_int) as usize]
    } else {
        x264_cqm_jvt[idx as usize]
    };
    if memcmp(
        list as *const libc::c_void,
        def_list as *const libc::c_void,
        len as libc::c_ulong,
    ) == 0
    {
        bs_write1(s, 0 as libc::c_int as uint32_t);
    } else if memcmp(
        list as *const libc::c_void,
        x264_cqm_jvt[idx as usize] as *const libc::c_void,
        len as libc::c_ulong,
    ) == 0
    {
        bs_write1(s, 1 as libc::c_int as uint32_t);
        bs_write_se(s, -(8 as libc::c_int));
    } else {
        let mut run: libc::c_int = 0;
        bs_write1(s, 1 as libc::c_int as uint32_t);
        run = len;
        while run > 1 as libc::c_int {
            if *list.offset(*zigzag.offset((run - 1 as libc::c_int) as isize) as isize)
                as libc::c_int
                != *list.offset(*zigzag.offset((run - 2 as libc::c_int) as isize) as isize)
                    as libc::c_int
            {
                break;
            }
            run -= 1;
            run;
        }
        if run < len
            && len - run
                < bs_size_se(
                    -(*list.offset(*zigzag.offset(run as isize) as isize) as libc::c_int) as int8_t
                        as libc::c_int,
                )
        {
            run = len;
        }
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < run {
            bs_write_se(
                s,
                (*list.offset(*zigzag.offset(j as isize) as isize) as libc::c_int
                    - (if j > 0 as libc::c_int {
                        *list.offset(*zigzag.offset((j - 1 as libc::c_int) as isize) as isize)
                            as libc::c_int
                    } else {
                        8 as libc::c_int
                    })) as int8_t as libc::c_int,
            );
            j += 1;
            j;
        }
        if run < len {
            bs_write_se(
                s,
                -(*list.offset(*zigzag.offset(run as isize) as isize) as libc::c_int) as int8_t
                    as libc::c_int,
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_sei_write(
    mut s: *mut bs_t,
    mut payload: *mut uint8_t,
    mut payload_size: libc::c_int,
    mut payload_type: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    bs_realign(s);
    i = 0 as libc::c_int;
    while i <= payload_type - 255 as libc::c_int {
        bs_write(s, 8 as libc::c_int, 255 as libc::c_int as uint32_t);
        i += 255 as libc::c_int;
    }
    bs_write(s, 8 as libc::c_int, (payload_type - i) as uint32_t);
    i = 0 as libc::c_int;
    while i <= payload_size - 255 as libc::c_int {
        bs_write(s, 8 as libc::c_int, 255 as libc::c_int as uint32_t);
        i += 255 as libc::c_int;
    }
    bs_write(s, 8 as libc::c_int, (payload_size - i) as uint32_t);
    i = 0 as libc::c_int;
    while i < payload_size {
        bs_write(s, 8 as libc::c_int, *payload.offset(i as isize) as uint32_t);
        i += 1;
        i;
    }
    bs_rbsp_trailing(s);
    bs_flush(s);
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_sps_init(
    mut sps: *mut x264_sps_t,
    mut i_id: libc::c_int,
    mut param: *mut x264_param_t,
) {
    let mut csp: libc::c_int = (*param).i_csp & 0xff as libc::c_int;
    (*sps).i_id = i_id;
    (*sps).i_mb_width = ((*param).i_width + 15 as libc::c_int) / 16 as libc::c_int;
    (*sps).i_mb_height = ((*param).i_height + 15 as libc::c_int) / 16 as libc::c_int;
    (*sps).b_frame_mbs_only =
        !((*param).b_interlaced != 0 || (*param).b_fake_interlaced != 0) as libc::c_int;
    if (*sps).b_frame_mbs_only == 0 {
        (*sps).i_mb_height = ((*sps).i_mb_height + 1 as libc::c_int) & !(1 as libc::c_int);
    }
    (*sps).i_chroma_format_idc = if csp >= 0xc as libc::c_int {
        CHROMA_444 as libc::c_int
    } else if csp >= 0x6 as libc::c_int {
        CHROMA_422 as libc::c_int
    } else if csp >= 0x2 as libc::c_int {
        CHROMA_420 as libc::c_int
    } else {
        CHROMA_400 as libc::c_int
    };
    (*sps).b_qpprime_y_zero_transform_bypass = ((*param).rc.i_rc_method == 0 as libc::c_int
        && (*param).rc.i_qp_constant == 0 as libc::c_int)
        as libc::c_int;
    if (*sps).b_qpprime_y_zero_transform_bypass != 0
        || (*sps).i_chroma_format_idc == CHROMA_444 as libc::c_int
    {
        (*sps).i_profile_idc = PROFILE_HIGH444_PREDICTIVE as libc::c_int;
    } else if (*sps).i_chroma_format_idc == CHROMA_422 as libc::c_int {
        (*sps).i_profile_idc = PROFILE_HIGH422 as libc::c_int;
    } else if 8 as libc::c_int > 8 as libc::c_int {
        (*sps).i_profile_idc = PROFILE_HIGH10 as libc::c_int;
    } else if (*param).analyse.b_transform_8x8 != 0
        || (*param).i_cqm_preset != 0 as libc::c_int
        || (*sps).i_chroma_format_idc == CHROMA_400 as libc::c_int
    {
        (*sps).i_profile_idc = PROFILE_HIGH as libc::c_int;
    } else if (*param).b_cabac != 0
        || (*param).i_bframe > 0 as libc::c_int
        || (*param).b_interlaced != 0
        || (*param).b_fake_interlaced != 0
        || (*param).analyse.i_weighted_pred > 0 as libc::c_int
    {
        (*sps).i_profile_idc = PROFILE_MAIN as libc::c_int;
    } else {
        (*sps).i_profile_idc = PROFILE_BASELINE as libc::c_int;
    }
    (*sps).b_constraint_set0 =
        ((*sps).i_profile_idc == PROFILE_BASELINE as libc::c_int) as libc::c_int;
    (*sps).b_constraint_set1 = ((*sps).i_profile_idc <= PROFILE_MAIN as libc::c_int) as libc::c_int;
    (*sps).b_constraint_set2 = 0 as libc::c_int;
    (*sps).b_constraint_set3 = 0 as libc::c_int;
    (*sps).i_level_idc = (*param).i_level_idc;
    if (*param).i_level_idc == 9 as libc::c_int
        && ((*sps).i_profile_idc == PROFILE_BASELINE as libc::c_int
            || (*sps).i_profile_idc == PROFILE_MAIN as libc::c_int)
    {
        (*sps).b_constraint_set3 = 1 as libc::c_int;
        (*sps).i_level_idc = 11 as libc::c_int;
    }
    if (*param).i_keyint_max == 1 as libc::c_int
        && (*sps).i_profile_idc >= PROFILE_HIGH as libc::c_int
    {
        (*sps).b_constraint_set3 = 1 as libc::c_int;
    }
    (*sps).vui.i_num_reorder_frames = if (*param).i_bframe_pyramid != 0 {
        2 as libc::c_int
    } else if (*param).i_bframe != 0 {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    (*sps).i_num_ref_frames = if (16 as libc::c_int)
        < (if (*param).i_frame_reference
            > (if 1 as libc::c_int + (*sps).vui.i_num_reorder_frames
                > (if (if (*param).i_bframe_pyramid != 0 {
                    4 as libc::c_int
                } else {
                    1 as libc::c_int
                }) > (*param).i_dpb_size
                {
                    if (*param).i_bframe_pyramid != 0 {
                        4 as libc::c_int
                    } else {
                        1 as libc::c_int
                    }
                } else {
                    (*param).i_dpb_size
                })
            {
                1 as libc::c_int + (*sps).vui.i_num_reorder_frames
            } else if (if (*param).i_bframe_pyramid != 0 {
                4 as libc::c_int
            } else {
                1 as libc::c_int
            }) > (*param).i_dpb_size
            {
                if (*param).i_bframe_pyramid != 0 {
                    4 as libc::c_int
                } else {
                    1 as libc::c_int
                }
            } else {
                (*param).i_dpb_size
            })
        {
            (*param).i_frame_reference
        } else if 1 as libc::c_int + (*sps).vui.i_num_reorder_frames
            > (if (if (*param).i_bframe_pyramid != 0 {
                4 as libc::c_int
            } else {
                1 as libc::c_int
            }) > (*param).i_dpb_size
            {
                if (*param).i_bframe_pyramid != 0 {
                    4 as libc::c_int
                } else {
                    1 as libc::c_int
                }
            } else {
                (*param).i_dpb_size
            })
        {
            1 as libc::c_int + (*sps).vui.i_num_reorder_frames
        } else if (if (*param).i_bframe_pyramid != 0 {
            4 as libc::c_int
        } else {
            1 as libc::c_int
        }) > (*param).i_dpb_size
        {
            if (*param).i_bframe_pyramid != 0 {
                4 as libc::c_int
            } else {
                1 as libc::c_int
            }
        } else {
            (*param).i_dpb_size
        }) {
        16 as libc::c_int
    } else if (*param).i_frame_reference
        > (if 1 as libc::c_int + (*sps).vui.i_num_reorder_frames
            > (if (if (*param).i_bframe_pyramid != 0 {
                4 as libc::c_int
            } else {
                1 as libc::c_int
            }) > (*param).i_dpb_size
            {
                if (*param).i_bframe_pyramid != 0 {
                    4 as libc::c_int
                } else {
                    1 as libc::c_int
                }
            } else {
                (*param).i_dpb_size
            })
        {
            1 as libc::c_int + (*sps).vui.i_num_reorder_frames
        } else if (if (*param).i_bframe_pyramid != 0 {
            4 as libc::c_int
        } else {
            1 as libc::c_int
        }) > (*param).i_dpb_size
        {
            if (*param).i_bframe_pyramid != 0 {
                4 as libc::c_int
            } else {
                1 as libc::c_int
            }
        } else {
            (*param).i_dpb_size
        })
    {
        (*param).i_frame_reference
    } else if 1 as libc::c_int + (*sps).vui.i_num_reorder_frames
        > (if (if (*param).i_bframe_pyramid != 0 {
            4 as libc::c_int
        } else {
            1 as libc::c_int
        }) > (*param).i_dpb_size
        {
            if (*param).i_bframe_pyramid != 0 {
                4 as libc::c_int
            } else {
                1 as libc::c_int
            }
        } else {
            (*param).i_dpb_size
        })
    {
        1 as libc::c_int + (*sps).vui.i_num_reorder_frames
    } else if (if (*param).i_bframe_pyramid != 0 {
        4 as libc::c_int
    } else {
        1 as libc::c_int
    }) > (*param).i_dpb_size
    {
        if (*param).i_bframe_pyramid != 0 {
            4 as libc::c_int
        } else {
            1 as libc::c_int
        }
    } else {
        (*param).i_dpb_size
    };
    (*sps).vui.i_max_dec_frame_buffering = (*sps).i_num_ref_frames;
    (*sps).i_num_ref_frames -= ((*param).i_bframe_pyramid == 1 as libc::c_int) as libc::c_int;
    if (*param).i_keyint_max == 1 as libc::c_int {
        (*sps).i_num_ref_frames = 0 as libc::c_int;
        (*sps).vui.i_max_dec_frame_buffering = 0 as libc::c_int;
    }
    let mut max_frame_num: libc::c_int = (*sps).vui.i_max_dec_frame_buffering
        * (((*param).i_bframe_pyramid != 0) as libc::c_int + 1 as libc::c_int)
        + 1 as libc::c_int;
    if (*param).b_intra_refresh != 0 {
        let mut time_to_recovery: libc::c_int =
            (if ((*sps).i_mb_width - 1 as libc::c_int) < (*param).i_keyint_max {
                (*sps).i_mb_width - 1 as libc::c_int
            } else {
                (*param).i_keyint_max
            }) + (*param).i_bframe
                - 1 as libc::c_int;
        max_frame_num = if max_frame_num > time_to_recovery + 1 as libc::c_int {
            max_frame_num
        } else {
            time_to_recovery + 1 as libc::c_int
        };
    }
    (*sps).i_log2_max_frame_num = 4 as libc::c_int;
    while (1 as libc::c_int) << (*sps).i_log2_max_frame_num <= max_frame_num {
        (*sps).i_log2_max_frame_num += 1;
        (*sps).i_log2_max_frame_num;
    }
    (*sps).i_poc_type =
        if (*param).i_bframe != 0 || (*param).b_interlaced != 0 || (*param).i_avcintra_class != 0 {
            0 as libc::c_int
        } else {
            2 as libc::c_int
        };
    if (*sps).i_poc_type == 0 as libc::c_int {
        let mut max_delta_poc: libc::c_int = ((*param).i_bframe + 2 as libc::c_int)
            * (((*param).i_bframe_pyramid != 0) as libc::c_int + 1 as libc::c_int)
            * 2 as libc::c_int;
        (*sps).i_log2_max_poc_lsb = 4 as libc::c_int;
        while (1 as libc::c_int) << (*sps).i_log2_max_poc_lsb <= max_delta_poc * 2 as libc::c_int {
            (*sps).i_log2_max_poc_lsb += 1;
            (*sps).i_log2_max_poc_lsb;
        }
    }
    (*sps).b_vui = 1 as libc::c_int;
    (*sps).b_gaps_in_frame_num_value_allowed = 0 as libc::c_int;
    (*sps).b_mb_adaptive_frame_field = (*param).b_interlaced;
    (*sps).b_direct8x8_inference = 1 as libc::c_int;
    x264_8_sps_init_reconfigurable(sps, param);
    (*sps).vui.b_overscan_info_present = ((*param).vui.i_overscan > 0 as libc::c_int
        && (*param).vui.i_overscan <= 2 as libc::c_int)
        as libc::c_int;
    if (*sps).vui.b_overscan_info_present != 0 {
        (*sps).vui.b_overscan_info = if (*param).vui.i_overscan == 2 as libc::c_int {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        };
    }
    (*sps).vui.b_signal_type_present = 0 as libc::c_int;
    (*sps).vui.i_vidformat = if (*param).vui.i_vidformat >= 0 as libc::c_int
        && (*param).vui.i_vidformat <= 5 as libc::c_int
    {
        (*param).vui.i_vidformat
    } else {
        5 as libc::c_int
    };
    (*sps).vui.b_fullrange = if (*param).vui.b_fullrange >= 0 as libc::c_int
        && (*param).vui.b_fullrange <= 1 as libc::c_int
    {
        (*param).vui.b_fullrange
    } else if csp >= 0xe as libc::c_int {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    (*sps).vui.b_color_description_present = 0 as libc::c_int;
    (*sps).vui.i_colorprim = if (*param).vui.i_colorprim >= 0 as libc::c_int
        && (*param).vui.i_colorprim <= 12 as libc::c_int
    {
        (*param).vui.i_colorprim
    } else {
        2 as libc::c_int
    };
    (*sps).vui.i_transfer = if (*param).vui.i_transfer >= 0 as libc::c_int
        && (*param).vui.i_transfer <= 18 as libc::c_int
    {
        (*param).vui.i_transfer
    } else {
        2 as libc::c_int
    };
    (*sps).vui.i_colmatrix = if (*param).vui.i_colmatrix >= 0 as libc::c_int
        && (*param).vui.i_colmatrix <= 14 as libc::c_int
    {
        (*param).vui.i_colmatrix
    } else if csp >= 0xe as libc::c_int {
        0 as libc::c_int
    } else {
        2 as libc::c_int
    };
    if (*sps).vui.i_colorprim != 2 as libc::c_int
        || (*sps).vui.i_transfer != 2 as libc::c_int
        || (*sps).vui.i_colmatrix != 2 as libc::c_int
    {
        (*sps).vui.b_color_description_present = 1 as libc::c_int;
    }
    if (*sps).vui.i_vidformat != 5 as libc::c_int
        || (*sps).vui.b_fullrange != 0
        || (*sps).vui.b_color_description_present != 0
    {
        (*sps).vui.b_signal_type_present = 1 as libc::c_int;
    }
    (*sps).vui.b_chroma_loc_info_present = ((*param).vui.i_chroma_loc > 0 as libc::c_int
        && (*param).vui.i_chroma_loc <= 5 as libc::c_int
        && (*sps).i_chroma_format_idc == CHROMA_420 as libc::c_int)
        as libc::c_int;
    if (*sps).vui.b_chroma_loc_info_present != 0 {
        (*sps).vui.i_chroma_loc_top = (*param).vui.i_chroma_loc;
        (*sps).vui.i_chroma_loc_bottom = (*param).vui.i_chroma_loc;
    }
    (*sps).vui.b_timing_info_present = ((*param).i_timebase_num > 0 as libc::c_int as uint32_t
        && (*param).i_timebase_den > 0 as libc::c_int as uint32_t)
        as libc::c_int;
    if (*sps).vui.b_timing_info_present != 0 {
        (*sps).vui.i_num_units_in_tick = (*param).i_timebase_num;
        (*sps).vui.i_time_scale = (*param).i_timebase_den * 2 as libc::c_int as uint32_t;
        (*sps).vui.b_fixed_frame_rate = ((*param).b_vfr_input == 0) as libc::c_int;
    }
    (*sps).vui.b_vcl_hrd_parameters_present = 0 as libc::c_int;
    (*sps).vui.b_nal_hrd_parameters_present = ((*param).i_nal_hrd != 0) as libc::c_int;
    (*sps).vui.b_pic_struct_present = (*param).b_pic_struct;
    (*sps).vui.b_bitstream_restriction = !((*sps).b_constraint_set3 != 0
        && (*sps).i_profile_idc >= PROFILE_HIGH as libc::c_int)
        as libc::c_int;
    if (*sps).vui.b_bitstream_restriction != 0 {
        (*sps).vui.b_motion_vectors_over_pic_boundaries = 1 as libc::c_int;
        (*sps).vui.i_max_bytes_per_pic_denom = 0 as libc::c_int;
        (*sps).vui.i_max_bits_per_mb_denom = 0 as libc::c_int;
        (*sps).vui.i_log2_max_mv_length_vertical = log2f(
            (if 1 as libc::c_int > (*param).analyse.i_mv_range * 4 as libc::c_int - 1 as libc::c_int
            {
                1 as libc::c_int
            } else {
                (*param).analyse.i_mv_range * 4 as libc::c_int - 1 as libc::c_int
            }) as libc::c_float,
        ) as libc::c_int
            + 1 as libc::c_int;
        (*sps).vui.i_log2_max_mv_length_horizontal = (*sps).vui.i_log2_max_mv_length_vertical;
    }
    (*sps).b_avcintra_hd = ((*param).i_avcintra_class != 0
        && (*param).i_avcintra_class <= 200 as libc::c_int)
        as libc::c_int;
    (*sps).b_avcintra_4k = ((*param).i_avcintra_class > 200 as libc::c_int) as libc::c_int;
    (*sps).i_cqm_preset = (*param).i_cqm_preset;
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_sps_init_reconfigurable(
    mut sps: *mut x264_sps_t,
    mut param: *mut x264_param_t,
) {
    (*sps).crop.i_left = (*param).crop_rect.i_left;
    (*sps).crop.i_top = (*param).crop_rect.i_top;
    (*sps).crop.i_right =
        (*param).crop_rect.i_right + (*sps).i_mb_width * 16 as libc::c_int - (*param).i_width;
    (*sps).crop.i_bottom =
        (*param).crop_rect.i_bottom + (*sps).i_mb_height * 16 as libc::c_int - (*param).i_height;
    (*sps).b_crop = ((*sps).crop.i_left != 0
        || (*sps).crop.i_top != 0
        || (*sps).crop.i_right != 0
        || (*sps).crop.i_bottom != 0) as libc::c_int;
    (*sps).vui.b_aspect_ratio_info_present = 0 as libc::c_int;
    if (*param).vui.i_sar_width > 0 as libc::c_int && (*param).vui.i_sar_height > 0 as libc::c_int {
        (*sps).vui.b_aspect_ratio_info_present = 1 as libc::c_int;
        (*sps).vui.i_sar_width = (*param).vui.i_sar_width;
        (*sps).vui.i_sar_height = (*param).vui.i_sar_height;
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_sps_init_scaling_list(
    mut sps: *mut x264_sps_t,
    mut param: *mut x264_param_t,
) {
    match (*sps).i_cqm_preset {
        0 => {
            let mut i: libc::c_int = 0 as libc::c_int;
            while i < 8 as libc::c_int {
                (*sps).scaling_list[i as usize] = x264_cqm_flat16.as_ptr();
                i += 1;
                i;
            }
        }
        1 => {
            let mut i_0: libc::c_int = 0 as libc::c_int;
            while i_0 < 8 as libc::c_int {
                (*sps).scaling_list[i_0 as usize] = x264_cqm_jvt[i_0 as usize];
                i_0 += 1;
                i_0;
            }
        }
        2 => {
            transpose(((*param).cqm_4iy).as_mut_ptr(), 4 as libc::c_int);
            transpose(((*param).cqm_4py).as_mut_ptr(), 4 as libc::c_int);
            transpose(((*param).cqm_4ic).as_mut_ptr(), 4 as libc::c_int);
            transpose(((*param).cqm_4pc).as_mut_ptr(), 4 as libc::c_int);
            transpose(((*param).cqm_8iy).as_mut_ptr(), 8 as libc::c_int);
            transpose(((*param).cqm_8py).as_mut_ptr(), 8 as libc::c_int);
            transpose(((*param).cqm_8ic).as_mut_ptr(), 8 as libc::c_int);
            transpose(((*param).cqm_8pc).as_mut_ptr(), 8 as libc::c_int);
            (*sps).scaling_list[CQM_4IY as libc::c_int as usize] = ((*param).cqm_4iy).as_mut_ptr();
            (*sps).scaling_list[CQM_4PY as libc::c_int as usize] = ((*param).cqm_4py).as_mut_ptr();
            (*sps).scaling_list[CQM_4IC as libc::c_int as usize] = ((*param).cqm_4ic).as_mut_ptr();
            (*sps).scaling_list[CQM_4PC as libc::c_int as usize] = ((*param).cqm_4pc).as_mut_ptr();
            (*sps).scaling_list[(CQM_8IY as libc::c_int + 4 as libc::c_int) as usize] =
                ((*param).cqm_8iy).as_mut_ptr();
            (*sps).scaling_list[(CQM_8PY as libc::c_int + 4 as libc::c_int) as usize] =
                ((*param).cqm_8py).as_mut_ptr();
            (*sps).scaling_list[(CQM_8IC as libc::c_int + 4 as libc::c_int) as usize] =
                ((*param).cqm_8ic).as_mut_ptr();
            (*sps).scaling_list[(CQM_8PC as libc::c_int + 4 as libc::c_int) as usize] =
                ((*param).cqm_8pc).as_mut_ptr();
            let mut i_1: libc::c_int = 0 as libc::c_int;
            while i_1 < 8 as libc::c_int {
                let mut j: libc::c_int = 0 as libc::c_int;
                while j
                    < (if i_1 < 4 as libc::c_int {
                        16 as libc::c_int
                    } else {
                        64 as libc::c_int
                    })
                {
                    if *((*sps).scaling_list[i_1 as usize]).offset(j as isize) as libc::c_int
                        == 0 as libc::c_int
                    {
                        (*sps).scaling_list[i_1 as usize] = x264_cqm_jvt[i_1 as usize];
                    }
                    j += 1;
                    j;
                }
                i_1 += 1;
                i_1;
            }
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_sps_write(mut s: *mut bs_t, mut sps: *mut x264_sps_t) {
    bs_realign(s);
    bs_write(s, 8 as libc::c_int, (*sps).i_profile_idc as uint32_t);
    bs_write1(s, (*sps).b_constraint_set0 as uint32_t);
    bs_write1(s, (*sps).b_constraint_set1 as uint32_t);
    bs_write1(s, (*sps).b_constraint_set2 as uint32_t);
    bs_write1(s, (*sps).b_constraint_set3 as uint32_t);
    bs_write(s, 4 as libc::c_int, 0 as libc::c_int as uint32_t);
    bs_write(s, 8 as libc::c_int, (*sps).i_level_idc as uint32_t);
    bs_write_ue_big(s, (*sps).i_id as libc::c_uint);
    if (*sps).i_profile_idc >= PROFILE_HIGH as libc::c_int {
        bs_write_ue_big(s, (*sps).i_chroma_format_idc as libc::c_uint);
        if (*sps).i_chroma_format_idc == CHROMA_444 as libc::c_int {
            bs_write1(s, 0 as libc::c_int as uint32_t);
        }
        bs_write_ue_big(s, (8 as libc::c_int - 8 as libc::c_int) as libc::c_uint);
        bs_write_ue_big(s, (8 as libc::c_int - 8 as libc::c_int) as libc::c_uint);
        bs_write1(s, (*sps).b_qpprime_y_zero_transform_bypass as uint32_t);
        bs_write1(s, (*sps).b_avcintra_hd as uint32_t);
        if (*sps).b_avcintra_hd != 0 {
            scaling_list_write(s, sps, CQM_4IY as libc::c_int);
            scaling_list_write(s, sps, CQM_4IC as libc::c_int);
            scaling_list_write(s, sps, CQM_4IC as libc::c_int);
            bs_write1(s, 0 as libc::c_int as uint32_t);
            bs_write1(s, 0 as libc::c_int as uint32_t);
            bs_write1(s, 0 as libc::c_int as uint32_t);
            scaling_list_write(s, sps, CQM_8IY as libc::c_int + 4 as libc::c_int);
            bs_write1(s, 0 as libc::c_int as uint32_t);
            if (*sps).i_chroma_format_idc == CHROMA_444 as libc::c_int {
                scaling_list_write(s, sps, CQM_8IC as libc::c_int + 4 as libc::c_int);
                bs_write1(s, 0 as libc::c_int as uint32_t);
                scaling_list_write(s, sps, CQM_8IC as libc::c_int + 4 as libc::c_int);
                bs_write1(s, 0 as libc::c_int as uint32_t);
            }
        }
    }
    bs_write_ue_big(
        s,
        ((*sps).i_log2_max_frame_num - 4 as libc::c_int) as libc::c_uint,
    );
    bs_write_ue_big(s, (*sps).i_poc_type as libc::c_uint);
    if (*sps).i_poc_type == 0 as libc::c_int {
        bs_write_ue_big(
            s,
            ((*sps).i_log2_max_poc_lsb - 4 as libc::c_int) as libc::c_uint,
        );
    }
    bs_write_ue_big(s, (*sps).i_num_ref_frames as libc::c_uint);
    bs_write1(s, (*sps).b_gaps_in_frame_num_value_allowed as uint32_t);
    bs_write_ue_big(s, ((*sps).i_mb_width - 1 as libc::c_int) as libc::c_uint);
    bs_write_ue_big(
        s,
        (((*sps).i_mb_height >> ((*sps).b_frame_mbs_only == 0) as libc::c_int) - 1 as libc::c_int)
            as libc::c_uint,
    );
    bs_write1(s, (*sps).b_frame_mbs_only as uint32_t);
    if (*sps).b_frame_mbs_only == 0 {
        bs_write1(s, (*sps).b_mb_adaptive_frame_field as uint32_t);
    }
    bs_write1(s, (*sps).b_direct8x8_inference as uint32_t);
    bs_write1(s, (*sps).b_crop as uint32_t);
    if (*sps).b_crop != 0 {
        let mut h_shift: libc::c_int = ((*sps).i_chroma_format_idc == CHROMA_420 as libc::c_int
            || (*sps).i_chroma_format_idc == CHROMA_422 as libc::c_int)
            as libc::c_int;
        let mut v_shift: libc::c_int = ((*sps).i_chroma_format_idc == CHROMA_420 as libc::c_int)
            as libc::c_int
            + ((*sps).b_frame_mbs_only == 0) as libc::c_int;
        bs_write_ue_big(s, ((*sps).crop.i_left >> h_shift) as libc::c_uint);
        bs_write_ue_big(s, ((*sps).crop.i_right >> h_shift) as libc::c_uint);
        bs_write_ue_big(s, ((*sps).crop.i_top >> v_shift) as libc::c_uint);
        bs_write_ue_big(s, ((*sps).crop.i_bottom >> v_shift) as libc::c_uint);
    }
    bs_write1(s, (*sps).b_vui as uint32_t);
    if (*sps).b_vui != 0 {
        bs_write1(s, (*sps).vui.b_aspect_ratio_info_present as uint32_t);
        if (*sps).vui.b_aspect_ratio_info_present != 0 {
            let mut i: libc::c_int = 0;
            static mut sar: [C2RustUnnamed_20; 17] = [
                {
                    C2RustUnnamed_20 {
                        w: 1 as libc::c_int as uint8_t,
                        h: 1 as libc::c_int as uint8_t,
                        sar: 1 as libc::c_int as uint8_t,
                    }
                },
                {
                    C2RustUnnamed_20 {
                        w: 12 as libc::c_int as uint8_t,
                        h: 11 as libc::c_int as uint8_t,
                        sar: 2 as libc::c_int as uint8_t,
                    }
                },
                {
                    C2RustUnnamed_20 {
                        w: 10 as libc::c_int as uint8_t,
                        h: 11 as libc::c_int as uint8_t,
                        sar: 3 as libc::c_int as uint8_t,
                    }
                },
                {
                    C2RustUnnamed_20 {
                        w: 16 as libc::c_int as uint8_t,
                        h: 11 as libc::c_int as uint8_t,
                        sar: 4 as libc::c_int as uint8_t,
                    }
                },
                {
                    C2RustUnnamed_20 {
                        w: 40 as libc::c_int as uint8_t,
                        h: 33 as libc::c_int as uint8_t,
                        sar: 5 as libc::c_int as uint8_t,
                    }
                },
                {
                    C2RustUnnamed_20 {
                        w: 24 as libc::c_int as uint8_t,
                        h: 11 as libc::c_int as uint8_t,
                        sar: 6 as libc::c_int as uint8_t,
                    }
                },
                {
                    C2RustUnnamed_20 {
                        w: 20 as libc::c_int as uint8_t,
                        h: 11 as libc::c_int as uint8_t,
                        sar: 7 as libc::c_int as uint8_t,
                    }
                },
                {
                    C2RustUnnamed_20 {
                        w: 32 as libc::c_int as uint8_t,
                        h: 11 as libc::c_int as uint8_t,
                        sar: 8 as libc::c_int as uint8_t,
                    }
                },
                {
                    C2RustUnnamed_20 {
                        w: 80 as libc::c_int as uint8_t,
                        h: 33 as libc::c_int as uint8_t,
                        sar: 9 as libc::c_int as uint8_t,
                    }
                },
                {
                    C2RustUnnamed_20 {
                        w: 18 as libc::c_int as uint8_t,
                        h: 11 as libc::c_int as uint8_t,
                        sar: 10 as libc::c_int as uint8_t,
                    }
                },
                {
                    C2RustUnnamed_20 {
                        w: 15 as libc::c_int as uint8_t,
                        h: 11 as libc::c_int as uint8_t,
                        sar: 11 as libc::c_int as uint8_t,
                    }
                },
                {
                    C2RustUnnamed_20 {
                        w: 64 as libc::c_int as uint8_t,
                        h: 33 as libc::c_int as uint8_t,
                        sar: 12 as libc::c_int as uint8_t,
                    }
                },
                {
                    C2RustUnnamed_20 {
                        w: 160 as libc::c_int as uint8_t,
                        h: 99 as libc::c_int as uint8_t,
                        sar: 13 as libc::c_int as uint8_t,
                    }
                },
                {
                    C2RustUnnamed_20 {
                        w: 4 as libc::c_int as uint8_t,
                        h: 3 as libc::c_int as uint8_t,
                        sar: 14 as libc::c_int as uint8_t,
                    }
                },
                {
                    C2RustUnnamed_20 {
                        w: 3 as libc::c_int as uint8_t,
                        h: 2 as libc::c_int as uint8_t,
                        sar: 15 as libc::c_int as uint8_t,
                    }
                },
                {
                    C2RustUnnamed_20 {
                        w: 2 as libc::c_int as uint8_t,
                        h: 1 as libc::c_int as uint8_t,
                        sar: 16 as libc::c_int as uint8_t,
                    }
                },
                {
                    C2RustUnnamed_20 {
                        w: 0 as libc::c_int as uint8_t,
                        h: 0 as libc::c_int as uint8_t,
                        sar: 255 as libc::c_int as uint8_t,
                    }
                },
            ];
            i = 0 as libc::c_int;
            while sar[i as usize].sar as libc::c_int != 255 as libc::c_int {
                if sar[i as usize].w as libc::c_int == (*sps).vui.i_sar_width
                    && sar[i as usize].h as libc::c_int == (*sps).vui.i_sar_height
                {
                    break;
                }
                i += 1;
                i;
            }
            bs_write(s, 8 as libc::c_int, sar[i as usize].sar as uint32_t);
            if sar[i as usize].sar as libc::c_int == 255 as libc::c_int {
                bs_write(s, 16 as libc::c_int, (*sps).vui.i_sar_width as uint32_t);
                bs_write(s, 16 as libc::c_int, (*sps).vui.i_sar_height as uint32_t);
            }
        }
        bs_write1(s, (*sps).vui.b_overscan_info_present as uint32_t);
        if (*sps).vui.b_overscan_info_present != 0 {
            bs_write1(s, (*sps).vui.b_overscan_info as uint32_t);
        }
        bs_write1(s, (*sps).vui.b_signal_type_present as uint32_t);
        if (*sps).vui.b_signal_type_present != 0 {
            bs_write(s, 3 as libc::c_int, (*sps).vui.i_vidformat as uint32_t);
            bs_write1(s, (*sps).vui.b_fullrange as uint32_t);
            bs_write1(s, (*sps).vui.b_color_description_present as uint32_t);
            if (*sps).vui.b_color_description_present != 0 {
                bs_write(s, 8 as libc::c_int, (*sps).vui.i_colorprim as uint32_t);
                bs_write(s, 8 as libc::c_int, (*sps).vui.i_transfer as uint32_t);
                bs_write(s, 8 as libc::c_int, (*sps).vui.i_colmatrix as uint32_t);
            }
        }
        bs_write1(s, (*sps).vui.b_chroma_loc_info_present as uint32_t);
        if (*sps).vui.b_chroma_loc_info_present != 0 {
            bs_write_ue_big(s, (*sps).vui.i_chroma_loc_top as libc::c_uint);
            bs_write_ue_big(s, (*sps).vui.i_chroma_loc_bottom as libc::c_uint);
        }
        bs_write1(s, (*sps).vui.b_timing_info_present as uint32_t);
        if (*sps).vui.b_timing_info_present != 0 {
            bs_write32(s, (*sps).vui.i_num_units_in_tick);
            bs_write32(s, (*sps).vui.i_time_scale);
            bs_write1(s, (*sps).vui.b_fixed_frame_rate as uint32_t);
        }
        bs_write1(s, (*sps).vui.b_nal_hrd_parameters_present as uint32_t);
        if (*sps).vui.b_nal_hrd_parameters_present != 0 {
            bs_write_ue_big(
                s,
                ((*sps).vui.hrd.i_cpb_cnt - 1 as libc::c_int) as libc::c_uint,
            );
            bs_write(
                s,
                4 as libc::c_int,
                (*sps).vui.hrd.i_bit_rate_scale as uint32_t,
            );
            bs_write(
                s,
                4 as libc::c_int,
                (*sps).vui.hrd.i_cpb_size_scale as uint32_t,
            );
            bs_write_ue_big(
                s,
                ((*sps).vui.hrd.i_bit_rate_value - 1 as libc::c_int) as libc::c_uint,
            );
            bs_write_ue_big(
                s,
                ((*sps).vui.hrd.i_cpb_size_value - 1 as libc::c_int) as libc::c_uint,
            );
            bs_write1(s, (*sps).vui.hrd.b_cbr_hrd as uint32_t);
            bs_write(
                s,
                5 as libc::c_int,
                ((*sps).vui.hrd.i_initial_cpb_removal_delay_length - 1 as libc::c_int) as uint32_t,
            );
            bs_write(
                s,
                5 as libc::c_int,
                ((*sps).vui.hrd.i_cpb_removal_delay_length - 1 as libc::c_int) as uint32_t,
            );
            bs_write(
                s,
                5 as libc::c_int,
                ((*sps).vui.hrd.i_dpb_output_delay_length - 1 as libc::c_int) as uint32_t,
            );
            bs_write(
                s,
                5 as libc::c_int,
                (*sps).vui.hrd.i_time_offset_length as uint32_t,
            );
        }
        bs_write1(s, (*sps).vui.b_vcl_hrd_parameters_present as uint32_t);
        if (*sps).vui.b_nal_hrd_parameters_present != 0
            || (*sps).vui.b_vcl_hrd_parameters_present != 0
        {
            bs_write1(s, 0 as libc::c_int as uint32_t);
        }
        bs_write1(s, (*sps).vui.b_pic_struct_present as uint32_t);
        bs_write1(s, (*sps).vui.b_bitstream_restriction as uint32_t);
        if (*sps).vui.b_bitstream_restriction != 0 {
            bs_write1(
                s,
                (*sps).vui.b_motion_vectors_over_pic_boundaries as uint32_t,
            );
            bs_write_ue_big(s, (*sps).vui.i_max_bytes_per_pic_denom as libc::c_uint);
            bs_write_ue_big(s, (*sps).vui.i_max_bits_per_mb_denom as libc::c_uint);
            bs_write_ue_big(
                s,
                (*sps).vui.i_log2_max_mv_length_horizontal as libc::c_uint,
            );
            bs_write_ue_big(s, (*sps).vui.i_log2_max_mv_length_vertical as libc::c_uint);
            bs_write_ue_big(s, (*sps).vui.i_num_reorder_frames as libc::c_uint);
            bs_write_ue_big(s, (*sps).vui.i_max_dec_frame_buffering as libc::c_uint);
        }
    }
    bs_rbsp_trailing(s);
    bs_flush(s);
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_pps_init(
    mut pps: *mut x264_pps_t,
    mut i_id: libc::c_int,
    mut param: *mut x264_param_t,
    mut sps: *mut x264_sps_t,
) {
    (*pps).i_id = i_id;
    (*pps).i_sps_id = (*sps).i_id;
    (*pps).b_cabac = (*param).b_cabac;
    (*pps).b_pic_order =
        ((*param).i_avcintra_class == 0 && (*param).b_interlaced != 0) as libc::c_int;
    (*pps).i_num_slice_groups = 1 as libc::c_int;
    (*pps).i_num_ref_idx_l0_default_active = (*param).i_frame_reference;
    (*pps).i_num_ref_idx_l1_default_active = 1 as libc::c_int;
    (*pps).b_weighted_pred = ((*param).analyse.i_weighted_pred > 0 as libc::c_int) as libc::c_int;
    (*pps).b_weighted_bipred = if (*param).analyse.b_weighted_bipred != 0 {
        2 as libc::c_int
    } else {
        0 as libc::c_int
    };
    (*pps).i_pic_init_qp =
        if (*param).rc.i_rc_method == 2 as libc::c_int || (*param).b_stitchable != 0 {
            26 as libc::c_int + 6 as libc::c_int * (8 as libc::c_int - 8 as libc::c_int)
        } else if (*param).rc.i_qp_constant
            < 51 as libc::c_int + 6 as libc::c_int * (8 as libc::c_int - 8 as libc::c_int)
        {
            (*param).rc.i_qp_constant
        } else {
            51 as libc::c_int + 6 as libc::c_int * (8 as libc::c_int - 8 as libc::c_int)
        };
    (*pps).i_pic_init_qs =
        26 as libc::c_int + 6 as libc::c_int * (8 as libc::c_int - 8 as libc::c_int);
    (*pps).i_chroma_qp_index_offset = (*param).analyse.i_chroma_qp_offset;
    (*pps).b_deblocking_filter_control = 1 as libc::c_int;
    (*pps).b_constrained_intra_pred = (*param).b_constrained_intra;
    (*pps).b_redundant_pic_cnt = 0 as libc::c_int;
    (*pps).b_transform_8x8_mode = if (*param).analyse.b_transform_8x8 != 0 {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_pps_write(
    mut s: *mut bs_t,
    mut sps: *mut x264_sps_t,
    mut pps: *mut x264_pps_t,
) {
    bs_realign(s);
    bs_write_ue_big(s, (*pps).i_id as libc::c_uint);
    bs_write_ue_big(s, (*pps).i_sps_id as libc::c_uint);
    bs_write1(s, (*pps).b_cabac as uint32_t);
    bs_write1(s, (*pps).b_pic_order as uint32_t);
    bs_write_ue_big(
        s,
        ((*pps).i_num_slice_groups - 1 as libc::c_int) as libc::c_uint,
    );
    bs_write_ue_big(
        s,
        ((*pps).i_num_ref_idx_l0_default_active - 1 as libc::c_int) as libc::c_uint,
    );
    bs_write_ue_big(
        s,
        ((*pps).i_num_ref_idx_l1_default_active - 1 as libc::c_int) as libc::c_uint,
    );
    bs_write1(s, (*pps).b_weighted_pred as uint32_t);
    bs_write(s, 2 as libc::c_int, (*pps).b_weighted_bipred as uint32_t);
    bs_write_se(
        s,
        (*pps).i_pic_init_qp
            - 26 as libc::c_int
            - 6 as libc::c_int * (8 as libc::c_int - 8 as libc::c_int),
    );
    bs_write_se(
        s,
        (*pps).i_pic_init_qs
            - 26 as libc::c_int
            - 6 as libc::c_int * (8 as libc::c_int - 8 as libc::c_int),
    );
    bs_write_se(s, (*pps).i_chroma_qp_index_offset);
    bs_write1(s, (*pps).b_deblocking_filter_control as uint32_t);
    bs_write1(s, (*pps).b_constrained_intra_pred as uint32_t);
    bs_write1(s, (*pps).b_redundant_pic_cnt as uint32_t);
    let mut b_scaling_list: libc::c_int =
        ((*sps).b_avcintra_hd == 0 && (*sps).i_cqm_preset != 0 as libc::c_int) as libc::c_int;
    if (*pps).b_transform_8x8_mode != 0 || b_scaling_list != 0 {
        bs_write1(s, (*pps).b_transform_8x8_mode as uint32_t);
        bs_write1(s, b_scaling_list as uint32_t);
        if b_scaling_list != 0 {
            scaling_list_write(s, sps, CQM_4IY as libc::c_int);
            scaling_list_write(s, sps, CQM_4IC as libc::c_int);
            if (*sps).b_avcintra_4k != 0 {
                scaling_list_write(s, sps, CQM_4IC as libc::c_int);
                bs_write1(s, 0 as libc::c_int as uint32_t);
                bs_write1(s, 0 as libc::c_int as uint32_t);
                bs_write1(s, 0 as libc::c_int as uint32_t);
            } else {
                bs_write1(s, 0 as libc::c_int as uint32_t);
                scaling_list_write(s, sps, CQM_4PY as libc::c_int);
                scaling_list_write(s, sps, CQM_4PC as libc::c_int);
                bs_write1(s, 0 as libc::c_int as uint32_t);
            }
            if (*pps).b_transform_8x8_mode != 0 {
                scaling_list_write(s, sps, CQM_8IY as libc::c_int + 4 as libc::c_int);
                if (*sps).b_avcintra_4k != 0 {
                    bs_write1(s, 0 as libc::c_int as uint32_t);
                } else {
                    scaling_list_write(s, sps, CQM_8PY as libc::c_int + 4 as libc::c_int);
                }
                if (*sps).i_chroma_format_idc == CHROMA_444 as libc::c_int {
                    scaling_list_write(s, sps, CQM_8IC as libc::c_int + 4 as libc::c_int);
                    scaling_list_write(s, sps, CQM_8PC as libc::c_int + 4 as libc::c_int);
                    bs_write1(s, 0 as libc::c_int as uint32_t);
                    bs_write1(s, 0 as libc::c_int as uint32_t);
                }
            }
        }
        bs_write_se(s, (*pps).i_chroma_qp_index_offset);
    }
    bs_rbsp_trailing(s);
    bs_flush(s);
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_sei_recovery_point_write(
    mut h: *mut x264_t,
    mut s: *mut bs_t,
    mut recovery_frame_cnt: libc::c_int,
) {
    let mut q: bs_t = bs_s {
        p_start: std::ptr::null_mut::<uint8_t>(),
        p: std::ptr::null_mut::<uint8_t>(),
        p_end: std::ptr::null_mut::<uint8_t>(),
        cur_bits: 0,
        i_left: 0,
        i_bits_encoded: 0,
    };
    let mut tmp_buf: [uint8_t; 100] = [0; 100];
    (*(tmp_buf.as_mut_ptr() as *mut x264_union32_t)).i = 0 as libc::c_int as uint32_t;
    bs_init(
        &mut q,
        tmp_buf.as_mut_ptr() as *mut libc::c_void,
        100 as libc::c_int,
    );
    bs_realign(&mut q);
    bs_write_ue_big(&mut q, recovery_frame_cnt as libc::c_uint);
    bs_write1(&mut q, 1 as libc::c_int as uint32_t);
    bs_write1(&mut q, 0 as libc::c_int as uint32_t);
    bs_write(&mut q, 2 as libc::c_int, 0 as libc::c_int as uint32_t);
    bs_align_10(&mut q);
    x264_8_sei_write(
        s,
        tmp_buf.as_mut_ptr(),
        bs_pos(&mut q) / 8 as libc::c_int,
        SEI_RECOVERY_POINT as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_sei_version_write(
    mut h: *mut x264_t,
    mut s: *mut bs_t,
) -> libc::c_int {
    static mut uuid: [uint8_t; 16] = [
        0xdc as libc::c_int as uint8_t,
        0x45 as libc::c_int as uint8_t,
        0xe9 as libc::c_int as uint8_t,
        0xbd as libc::c_int as uint8_t,
        0xe6 as libc::c_int as uint8_t,
        0xd9 as libc::c_int as uint8_t,
        0x48 as libc::c_int as uint8_t,
        0xb7 as libc::c_int as uint8_t,
        0x96 as libc::c_int as uint8_t,
        0x2c as libc::c_int as uint8_t,
        0xd8 as libc::c_int as uint8_t,
        0x20 as libc::c_int as uint8_t,
        0xd9 as libc::c_int as uint8_t,
        0x23 as libc::c_int as uint8_t,
        0xee as libc::c_int as uint8_t,
        0xef as libc::c_int as uint8_t,
    ];
    let mut opts: *mut libc::c_char = x264_param2string(&mut (*h).param, 0 as libc::c_int);
    let mut payload: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    let mut length: libc::c_int = 0;
    if opts.is_null() {
        return -(1 as libc::c_int);
    }
    payload =
        x264_malloc((200 as libc::c_int as libc::c_ulong).wrapping_add(strlen(opts)) as int64_t)
            as *mut libc::c_char;
    if payload.is_null() {
        x264_free(opts as *mut libc::c_void);
        -(1 as libc::c_int)
    } else {
        memcpy(
            payload as *mut libc::c_void,
            uuid.as_ptr() as *const libc::c_void,
            16 as libc::c_int as libc::c_ulong,
        );
        sprintf(
            payload.offset(16 as libc::c_int as isize),
            b"x264 - core %d%s - H.264/MPEG-4 AVC codec - Copy%s 2003-2025 - http://www.videolan.org/x264.html - options: %s\0"
                as *const u8 as *const libc::c_char,
            164 as libc::c_int,
            b" r3204 373697b\0" as *const u8 as *const libc::c_char,
            if 1 as libc::c_int != 0 {
                b"left\0" as *const u8 as *const libc::c_char
            } else {
                b"right\0" as *const u8 as *const libc::c_char
            },
            opts,
        );
        length = (strlen(payload)).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
        x264_8_sei_write(
            s,
            payload as *mut uint8_t,
            length,
            SEI_USER_DATA_UNREGISTERED as libc::c_int,
        );
        x264_free(opts as *mut libc::c_void);
        x264_free(payload as *mut libc::c_void);
        0 as libc::c_int
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_sei_buffering_period_write(mut h: *mut x264_t, mut s: *mut bs_t) {
    let mut sps: *mut x264_sps_t = ((*h).sps).as_mut_ptr();
    let mut q: bs_t = bs_s {
        p_start: std::ptr::null_mut::<uint8_t>(),
        p: std::ptr::null_mut::<uint8_t>(),
        p_end: std::ptr::null_mut::<uint8_t>(),
        cur_bits: 0,
        i_left: 0,
        i_bits_encoded: 0,
    };
    let mut tmp_buf: [uint8_t; 100] = [0; 100];
    (*(tmp_buf.as_mut_ptr() as *mut x264_union32_t)).i = 0 as libc::c_int as uint32_t;
    bs_init(
        &mut q,
        tmp_buf.as_mut_ptr() as *mut libc::c_void,
        100 as libc::c_int,
    );
    bs_realign(&mut q);
    bs_write_ue_big(&mut q, (*sps).i_id as libc::c_uint);
    if (*sps).vui.b_nal_hrd_parameters_present != 0 {
        bs_write(
            &mut q,
            (*sps).vui.hrd.i_initial_cpb_removal_delay_length,
            (*h).initial_cpb_removal_delay as uint32_t,
        );
        bs_write(
            &mut q,
            (*sps).vui.hrd.i_initial_cpb_removal_delay_length,
            (*h).initial_cpb_removal_delay_offset as uint32_t,
        );
    }
    bs_align_10(&mut q);
    x264_8_sei_write(
        s,
        tmp_buf.as_mut_ptr(),
        bs_pos(&mut q) / 8 as libc::c_int,
        SEI_BUFFERING_PERIOD as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_sei_pic_timing_write(mut h: *mut x264_t, mut s: *mut bs_t) {
    let mut sps: *mut x264_sps_t = ((*h).sps).as_mut_ptr();
    let mut q: bs_t = bs_s {
        p_start: std::ptr::null_mut::<uint8_t>(),
        p: std::ptr::null_mut::<uint8_t>(),
        p_end: std::ptr::null_mut::<uint8_t>(),
        cur_bits: 0,
        i_left: 0,
        i_bits_encoded: 0,
    };
    let mut tmp_buf: [uint8_t; 100] = [0; 100];
    (*(tmp_buf.as_mut_ptr() as *mut x264_union32_t)).i = 0 as libc::c_int as uint32_t;
    bs_init(
        &mut q,
        tmp_buf.as_mut_ptr() as *mut libc::c_void,
        100 as libc::c_int,
    );
    bs_realign(&mut q);
    if (*sps).vui.b_nal_hrd_parameters_present != 0 || (*sps).vui.b_vcl_hrd_parameters_present != 0
    {
        bs_write(
            &mut q,
            (*sps).vui.hrd.i_cpb_removal_delay_length,
            ((*(*h).fenc).i_cpb_delay - (*h).i_cpb_delay_pir_offset) as uint32_t,
        );
        bs_write(
            &mut q,
            (*sps).vui.hrd.i_dpb_output_delay_length,
            (*(*h).fenc).i_dpb_output_delay as uint32_t,
        );
    }
    if (*sps).vui.b_pic_struct_present != 0 {
        bs_write(
            &mut q,
            4 as libc::c_int,
            ((*(*h).fenc).i_pic_struct - 1 as libc::c_int) as uint32_t,
        );
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < num_clock_ts[(*(*h).fenc).i_pic_struct as usize] as libc::c_int {
            bs_write1(&mut q, 0 as libc::c_int as uint32_t);
            i += 1;
            i;
        }
    }
    bs_align_10(&mut q);
    x264_8_sei_write(
        s,
        tmp_buf.as_mut_ptr(),
        bs_pos(&mut q) / 8 as libc::c_int,
        SEI_PIC_TIMING as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_sei_frame_packing_write(mut h: *mut x264_t, mut s: *mut bs_t) {
    let mut quincunx_sampling_flag: libc::c_int =
        ((*h).param.i_frame_packing == 0 as libc::c_int) as libc::c_int;
    let mut q: bs_t = bs_s {
        p_start: std::ptr::null_mut::<uint8_t>(),
        p: std::ptr::null_mut::<uint8_t>(),
        p_end: std::ptr::null_mut::<uint8_t>(),
        cur_bits: 0,
        i_left: 0,
        i_bits_encoded: 0,
    };
    let mut tmp_buf: [uint8_t; 100] = [0; 100];
    (*(tmp_buf.as_mut_ptr() as *mut x264_union32_t)).i = 0 as libc::c_int as uint32_t;
    bs_init(
        &mut q,
        tmp_buf.as_mut_ptr() as *mut libc::c_void,
        100 as libc::c_int,
    );
    bs_realign(&mut q);
    bs_write_ue_big(&mut q, 0 as libc::c_int as libc::c_uint);
    bs_write1(&mut q, 0 as libc::c_int as uint32_t);
    bs_write(
        &mut q,
        7 as libc::c_int,
        (*h).param.i_frame_packing as uint32_t,
    );
    bs_write1(&mut q, quincunx_sampling_flag as uint32_t);
    bs_write(
        &mut q,
        6 as libc::c_int,
        ((*h).param.i_frame_packing != 6 as libc::c_int) as libc::c_int as uint32_t,
    );
    bs_write1(&mut q, 0 as libc::c_int as uint32_t);
    bs_write1(&mut q, 0 as libc::c_int as uint32_t);
    bs_write1(&mut q, 0 as libc::c_int as uint32_t);
    bs_write1(
        &mut q,
        ((*h).param.i_frame_packing == 5 as libc::c_int
            && (*(*h).fenc).i_frame & 1 as libc::c_int == 0) as libc::c_int as uint32_t,
    );
    bs_write1(&mut q, 0 as libc::c_int as uint32_t);
    bs_write1(&mut q, 0 as libc::c_int as uint32_t);
    if quincunx_sampling_flag == 0 as libc::c_int && (*h).param.i_frame_packing != 5 as libc::c_int
    {
        bs_write(&mut q, 4 as libc::c_int, 0 as libc::c_int as uint32_t);
        bs_write(&mut q, 4 as libc::c_int, 0 as libc::c_int as uint32_t);
        bs_write(&mut q, 4 as libc::c_int, 0 as libc::c_int as uint32_t);
        bs_write(&mut q, 4 as libc::c_int, 0 as libc::c_int as uint32_t);
    }
    bs_write(&mut q, 8 as libc::c_int, 0 as libc::c_int as uint32_t);
    bs_write_ue_big(
        &mut q,
        ((*h).param.i_frame_packing != 5 as libc::c_int) as libc::c_int as libc::c_uint,
    );
    bs_write1(&mut q, 0 as libc::c_int as uint32_t);
    bs_align_10(&mut q);
    x264_8_sei_write(
        s,
        tmp_buf.as_mut_ptr(),
        bs_pos(&mut q) / 8 as libc::c_int,
        SEI_FRAME_PACKING as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_sei_mastering_display_write(mut h: *mut x264_t, mut s: *mut bs_t) {
    let mut q: bs_t = bs_s {
        p_start: std::ptr::null_mut::<uint8_t>(),
        p: std::ptr::null_mut::<uint8_t>(),
        p_end: std::ptr::null_mut::<uint8_t>(),
        cur_bits: 0,
        i_left: 0,
        i_bits_encoded: 0,
    };
    let mut tmp_buf: [uint8_t; 100] = [0; 100];
    (*(tmp_buf.as_mut_ptr() as *mut x264_union32_t)).i = 0 as libc::c_int as uint32_t;
    bs_init(
        &mut q,
        tmp_buf.as_mut_ptr() as *mut libc::c_void,
        100 as libc::c_int,
    );
    bs_realign(&mut q);
    bs_write(
        &mut q,
        16 as libc::c_int,
        (*h).param.mastering_display.i_green_x as uint32_t,
    );
    bs_write(
        &mut q,
        16 as libc::c_int,
        (*h).param.mastering_display.i_green_y as uint32_t,
    );
    bs_write(
        &mut q,
        16 as libc::c_int,
        (*h).param.mastering_display.i_blue_x as uint32_t,
    );
    bs_write(
        &mut q,
        16 as libc::c_int,
        (*h).param.mastering_display.i_blue_y as uint32_t,
    );
    bs_write(
        &mut q,
        16 as libc::c_int,
        (*h).param.mastering_display.i_red_x as uint32_t,
    );
    bs_write(
        &mut q,
        16 as libc::c_int,
        (*h).param.mastering_display.i_red_y as uint32_t,
    );
    bs_write(
        &mut q,
        16 as libc::c_int,
        (*h).param.mastering_display.i_white_x as uint32_t,
    );
    bs_write(
        &mut q,
        16 as libc::c_int,
        (*h).param.mastering_display.i_white_y as uint32_t,
    );
    bs_write32(
        &mut q,
        (*h).param.mastering_display.i_display_max as uint32_t,
    );
    bs_write32(
        &mut q,
        (*h).param.mastering_display.i_display_min as uint32_t,
    );
    bs_align_10(&mut q);
    x264_8_sei_write(
        s,
        tmp_buf.as_mut_ptr(),
        bs_pos(&mut q) / 8 as libc::c_int,
        SEI_MASTERING_DISPLAY as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_sei_content_light_level_write(
    mut h: *mut x264_t,
    mut s: *mut bs_t,
) {
    let mut q: bs_t = bs_s {
        p_start: std::ptr::null_mut::<uint8_t>(),
        p: std::ptr::null_mut::<uint8_t>(),
        p_end: std::ptr::null_mut::<uint8_t>(),
        cur_bits: 0,
        i_left: 0,
        i_bits_encoded: 0,
    };
    let mut tmp_buf: [uint8_t; 100] = [0; 100];
    (*(tmp_buf.as_mut_ptr() as *mut x264_union32_t)).i = 0 as libc::c_int as uint32_t;
    bs_init(
        &mut q,
        tmp_buf.as_mut_ptr() as *mut libc::c_void,
        100 as libc::c_int,
    );
    bs_realign(&mut q);
    bs_write(
        &mut q,
        16 as libc::c_int,
        (*h).param.content_light_level.i_max_cll as uint32_t,
    );
    bs_write(
        &mut q,
        16 as libc::c_int,
        (*h).param.content_light_level.i_max_fall as uint32_t,
    );
    bs_align_10(&mut q);
    x264_8_sei_write(
        s,
        tmp_buf.as_mut_ptr(),
        bs_pos(&mut q) / 8 as libc::c_int,
        SEI_CONTENT_LIGHT_LEVEL as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_sei_alternative_transfer_write(
    mut h: *mut x264_t,
    mut s: *mut bs_t,
) {
    let mut q: bs_t = bs_s {
        p_start: std::ptr::null_mut::<uint8_t>(),
        p: std::ptr::null_mut::<uint8_t>(),
        p_end: std::ptr::null_mut::<uint8_t>(),
        cur_bits: 0,
        i_left: 0,
        i_bits_encoded: 0,
    };
    let mut tmp_buf: [uint8_t; 100] = [0; 100];
    (*(tmp_buf.as_mut_ptr() as *mut x264_union32_t)).i = 0 as libc::c_int as uint32_t;
    bs_init(
        &mut q,
        tmp_buf.as_mut_ptr() as *mut libc::c_void,
        100 as libc::c_int,
    );
    bs_realign(&mut q);
    bs_write(
        &mut q,
        8 as libc::c_int,
        (*h).param.i_alternative_transfer as uint32_t,
    );
    bs_align_10(&mut q);
    x264_8_sei_write(
        s,
        tmp_buf.as_mut_ptr(),
        bs_pos(&mut q) / 8 as libc::c_int,
        SEI_ALTERNATIVE_TRANSFER as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_filler_write(
    mut h: *mut x264_t,
    mut s: *mut bs_t,
    mut filler: libc::c_int,
) {
    bs_realign(s);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < filler {
        bs_write(s, 8 as libc::c_int, 0xff as libc::c_int as uint32_t);
        i += 1;
        i;
    }
    bs_rbsp_trailing(s);
    bs_flush(s);
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_sei_dec_ref_pic_marking_write(
    mut h: *mut x264_t,
    mut s: *mut bs_t,
) {
    let mut sh: *mut x264_slice_header_t = &mut (*h).sh_backup;
    let mut q: bs_t = bs_s {
        p_start: std::ptr::null_mut::<uint8_t>(),
        p: std::ptr::null_mut::<uint8_t>(),
        p_end: std::ptr::null_mut::<uint8_t>(),
        cur_bits: 0,
        i_left: 0,
        i_bits_encoded: 0,
    };
    let mut tmp_buf: [uint8_t; 100] = [0; 100];
    (*(tmp_buf.as_mut_ptr() as *mut x264_union32_t)).i = 0 as libc::c_int as uint32_t;
    bs_init(
        &mut q,
        tmp_buf.as_mut_ptr() as *mut libc::c_void,
        100 as libc::c_int,
    );
    bs_realign(&mut q);
    bs_write1(&mut q, 0 as libc::c_int as uint32_t);
    bs_write_ue_big(&mut q, (*sh).i_frame_num as libc::c_uint);
    if (*((*h).sps).as_mut_ptr()).b_frame_mbs_only == 0 {
        bs_write1(&mut q, 0 as libc::c_int as uint32_t);
    }
    bs_write1(
        &mut q,
        ((*sh).i_mmco_command_count > 0 as libc::c_int) as libc::c_int as uint32_t,
    );
    if (*sh).i_mmco_command_count > 0 as libc::c_int {
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < (*sh).i_mmco_command_count {
            bs_write_ue_big(&mut q, 1 as libc::c_int as libc::c_uint);
            bs_write_ue_big(
                &mut q,
                ((*sh).mmco[i as usize].i_difference_of_pic_nums - 1 as libc::c_int)
                    as libc::c_uint,
            );
            i += 1;
            i;
        }
        bs_write_ue_big(&mut q, 0 as libc::c_int as libc::c_uint);
    }
    bs_align_10(&mut q);
    x264_8_sei_write(
        s,
        tmp_buf.as_mut_ptr(),
        bs_pos(&mut q) / 8 as libc::c_int,
        SEI_DEC_REF_PIC_MARKING as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_sei_avcintra_umid_write(
    mut h: *mut x264_t,
    mut s: *mut bs_t,
) -> libc::c_int {
    let mut data: [uint8_t; 512] = [0; 512];
    let mut msg: *const libc::c_char = b"UMID\0" as *const u8 as *const libc::c_char;
    let len: libc::c_int = 497 as libc::c_int;
    memset(
        data.as_mut_ptr() as *mut libc::c_void,
        0xff as libc::c_int,
        len as libc::c_ulong,
    );
    memcpy(
        data.as_mut_ptr() as *mut libc::c_void,
        avcintra_uuid.as_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[uint8_t; 16]>() as libc::c_ulong,
    );
    memcpy(
        data.as_mut_ptr().offset(16 as libc::c_int as isize) as *mut libc::c_void,
        msg as *const libc::c_void,
        strlen(msg),
    );
    data[20 as libc::c_int as usize] = 0x13 as libc::c_int as uint8_t;
    data[26 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
    data[25 as libc::c_int as usize] = data[26 as libc::c_int as usize];
    data[23 as libc::c_int as usize] = data[25 as libc::c_int as usize];
    data[22 as libc::c_int as usize] = data[23 as libc::c_int as usize];
    data[28 as libc::c_int as usize] = 0x14 as libc::c_int as uint8_t;
    data[34 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
    data[33 as libc::c_int as usize] = data[34 as libc::c_int as usize];
    data[31 as libc::c_int as usize] = data[33 as libc::c_int as usize];
    data[30 as libc::c_int as usize] = data[31 as libc::c_int as usize];
    data[36 as libc::c_int as usize] = 0x60 as libc::c_int as uint8_t;
    data[41 as libc::c_int as usize] = 0x22 as libc::c_int as uint8_t;
    data[60 as libc::c_int as usize] = 0x62 as libc::c_int as uint8_t;
    data[66 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
    data[65 as libc::c_int as usize] = data[66 as libc::c_int as usize];
    data[63 as libc::c_int as usize] = data[65 as libc::c_int as usize];
    data[62 as libc::c_int as usize] = data[63 as libc::c_int as usize];
    data[68 as libc::c_int as usize] = 0x63 as libc::c_int as uint8_t;
    data[74 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
    data[73 as libc::c_int as usize] = data[74 as libc::c_int as usize];
    data[71 as libc::c_int as usize] = data[73 as libc::c_int as usize];
    data[70 as libc::c_int as usize] = data[71 as libc::c_int as usize];
    x264_8_sei_write(
        &mut (*h).out.bs,
        data.as_mut_ptr(),
        len,
        SEI_USER_DATA_UNREGISTERED as libc::c_int,
    );
    0 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_sei_avcintra_vanc_write(
    mut h: *mut x264_t,
    mut s: *mut bs_t,
    mut len: libc::c_int,
) -> libc::c_int {
    let mut data: [uint8_t; 6000] = [0; 6000];
    let mut msg: *const libc::c_char = b"VANC\0" as *const u8 as *const libc::c_char;
    if len < 0 as libc::c_int
        || len as libc::c_uint as libc::c_ulong
            > ::core::mem::size_of::<[uint8_t; 6000]>() as libc::c_ulong
    {
        x264_8_log(
            h,
            0 as libc::c_int,
            b"AVC-Intra SEI is too large (%d)\n\0" as *const u8 as *const libc::c_char,
            len,
        );
        return -(1 as libc::c_int);
    }
    memset(
        data.as_mut_ptr() as *mut libc::c_void,
        0xff as libc::c_int,
        len as libc::c_ulong,
    );
    memcpy(
        data.as_mut_ptr() as *mut libc::c_void,
        avcintra_uuid.as_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[uint8_t; 16]>() as libc::c_ulong,
    );
    memcpy(
        data.as_mut_ptr().offset(16 as libc::c_int as isize) as *mut libc::c_void,
        msg as *const libc::c_void,
        strlen(msg),
    );
    x264_8_sei_write(
        &mut (*h).out.bs,
        data.as_mut_ptr(),
        len,
        SEI_USER_DATA_UNREGISTERED as libc::c_int,
    );
    0 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_validate_levels(
    mut h: *mut x264_t,
    mut verbose: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut mbs: libc::c_int =
        (*((*h).sps).as_mut_ptr()).i_mb_width * (*((*h).sps).as_mut_ptr()).i_mb_height;
    let mut dpb: libc::c_int = mbs * (*((*h).sps).as_mut_ptr()).vui.i_max_dec_frame_buffering;
    let mut cbp_factor: libc::c_int =
        if (*((*h).sps).as_mut_ptr()).i_profile_idc >= PROFILE_HIGH422 as libc::c_int {
            16 as libc::c_int
        } else if (*((*h).sps).as_mut_ptr()).i_profile_idc == PROFILE_HIGH10 as libc::c_int {
            12 as libc::c_int
        } else if (*((*h).sps).as_mut_ptr()).i_profile_idc == PROFILE_HIGH as libc::c_int {
            5 as libc::c_int
        } else {
            4 as libc::c_int
        };
    let mut l: *const x264_level_t = x264_levels.as_ptr();
    while (*l).level_idc as libc::c_int != 0 as libc::c_int
        && (*l).level_idc as libc::c_int != (*h).param.i_level_idc
    {
        l = l.offset(1);
        l;
    }
    if (*l).frame_size < mbs
        || ((*l).frame_size * 8 as libc::c_int)
            < (*((*h).sps).as_mut_ptr()).i_mb_width * (*((*h).sps).as_mut_ptr()).i_mb_width
        || ((*l).frame_size * 8 as libc::c_int)
            < (*((*h).sps).as_mut_ptr()).i_mb_height * (*((*h).sps).as_mut_ptr()).i_mb_height
    {
        if verbose != 0 {
            x264_8_log(
                h,
                1 as libc::c_int,
                b"frame MB size (%dx%d) > level limit (%d)\n\0" as *const u8 as *const libc::c_char,
                (*((*h).sps).as_mut_ptr()).i_mb_width,
                (*((*h).sps).as_mut_ptr()).i_mb_height,
                (*l).frame_size,
            );
        }
        ret = 1 as libc::c_int;
    }
    if dpb > (*l).dpb {
        if verbose != 0 {
            x264_8_log(
                h,
                1 as libc::c_int,
                b"DPB size (%d frames, %d mbs) > level limit (%d frames, %d mbs)\n\0" as *const u8
                    as *const libc::c_char,
                (*((*h).sps).as_mut_ptr()).vui.i_max_dec_frame_buffering,
                dpb,
                (*l).dpb / mbs,
                (*l).dpb,
            );
        }
        ret = 1 as libc::c_int;
    }
    if (*h).param.rc.i_vbv_max_bitrate > (*l).bitrate * cbp_factor / 4 as libc::c_int {
        if verbose != 0 {
            x264_8_log(
                h,
                1 as libc::c_int,
                b"VBV bitrate (%ld) > level limit (%d)\n\0" as *const u8 as *const libc::c_char,
                (*h).param.rc.i_vbv_max_bitrate as int64_t,
                (*l).bitrate * cbp_factor / 4 as libc::c_int,
            );
        }
        ret = 1 as libc::c_int;
    }
    if (*h).param.rc.i_vbv_buffer_size > (*l).cpb * cbp_factor / 4 as libc::c_int {
        if verbose != 0 {
            x264_8_log(
                h,
                1 as libc::c_int,
                b"VBV buffer (%ld) > level limit (%d)\n\0" as *const u8 as *const libc::c_char,
                (*h).param.rc.i_vbv_buffer_size as int64_t,
                (*l).cpb * cbp_factor / 4 as libc::c_int,
            );
        }
        ret = 1 as libc::c_int;
    }
    if (*h).param.analyse.i_mv_range > (*l).mv_range as libc::c_int {
        if verbose != 0 {
            x264_8_log(
                h,
                1 as libc::c_int,
                b"MV range (%ld) > level limit (%d)\n\0" as *const u8 as *const libc::c_char,
                (*h).param.analyse.i_mv_range as int64_t,
                (*l).mv_range as libc::c_int,
            );
        }
        ret = 1 as libc::c_int;
    }
    if (*h).param.b_interlaced > ((*l).frame_only == 0) as libc::c_int {
        if verbose != 0 {
            x264_8_log(
                h,
                1 as libc::c_int,
                b"interlaced (%ld) > level limit (%d)\n\0" as *const u8 as *const libc::c_char,
                (*h).param.b_interlaced as int64_t,
                ((*l).frame_only == 0) as libc::c_int,
            );
        }
        ret = 1 as libc::c_int;
    }
    if (*h).param.b_fake_interlaced > ((*l).frame_only == 0) as libc::c_int {
        if verbose != 0 {
            x264_8_log(
                h,
                1 as libc::c_int,
                b"fake interlaced (%ld) > level limit (%d)\n\0" as *const u8 as *const libc::c_char,
                (*h).param.b_fake_interlaced as int64_t,
                ((*l).frame_only == 0) as libc::c_int,
            );
        }
        ret = 1 as libc::c_int;
    }
    if (*h).param.i_fps_den > 0 as libc::c_int as uint32_t
        && mbs as int64_t * (*h).param.i_fps_num as int64_t / (*h).param.i_fps_den as int64_t
            > (*l).mbps as int64_t
    {
        if verbose != 0 {
            x264_8_log(
                h,
                1 as libc::c_int,
                b"MB rate (%ld) > level limit (%d)\n\0" as *const u8 as *const libc::c_char,
                mbs as int64_t * (*h).param.i_fps_num as int64_t / (*h).param.i_fps_den as int64_t,
                (*l).mbps,
            );
        }
        ret = 1 as libc::c_int;
    }
    ret
}
