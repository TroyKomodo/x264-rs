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
#[no_mangle]
pub unsafe extern "C" fn x264_8_cabac_init(mut h: *mut x264_t) {
    let mut ctx_count: libc::c_int =
        if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as libc::c_int {
            1024 as libc::c_int
        } else {
            460 as libc::c_int
        };
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        let mut cabac_context_init: *const [[int8_t; 2]; 1024] = if i == 0 as libc::c_int {
            &x264_cabac_context_init_I
        } else {
            &*x264_cabac_context_init_PB
                .as_ptr()
                .offset((i - 1 as libc::c_int) as isize) as *const [[int8_t; 2]; 1024]
        };
        let mut qp: libc::c_int = 0 as libc::c_int;
        while qp <= 51 as libc::c_int + 6 as libc::c_int * (8 as libc::c_int - 8 as libc::c_int) {
            let mut j: libc::c_int = 0 as libc::c_int;
            while j < ctx_count {
                let mut state: libc::c_int = x264_clip3(
                    (((*cabac_context_init)[j as usize][0 as libc::c_int as usize] as libc::c_int
                        * qp)
                        >> 4 as libc::c_int)
                        + (*cabac_context_init)[j as usize][1 as libc::c_int as usize]
                            as libc::c_int,
                    1 as libc::c_int,
                    126 as libc::c_int,
                );
                cabac_contexts[i as usize][qp as usize][j as usize] =
                    (((if state < 127 as libc::c_int - state {
                        state
                    } else {
                        127 as libc::c_int - state
                    }) << 1 as libc::c_int)
                        | (state >> 6 as libc::c_int)) as uint8_t;
                j += 1;
                j;
            }
            qp += 1;
            qp;
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_cabac_context_init(
    mut h: *mut x264_t,
    mut cb: *mut x264_cabac_t,
    mut i_slice_type: libc::c_int,
    mut i_qp: libc::c_int,
    mut i_model: libc::c_int,
) {
    memcpy(
        ((*cb).state).as_mut_ptr() as *mut libc::c_void,
        (cabac_contexts[(if i_slice_type == SLICE_TYPE_I as libc::c_int {
            0 as libc::c_int
        } else {
            i_model + 1 as libc::c_int
        }) as usize][i_qp as usize])
            .as_mut_ptr() as *const libc::c_void,
        (if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as libc::c_int {
            1024 as libc::c_int
        } else {
            460 as libc::c_int
        }) as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_cabac_encode_init_core(mut cb: *mut x264_cabac_t) {
    (*cb).i_low = 0 as libc::c_int;
    (*cb).i_range = 0x1fe as libc::c_int;
    (*cb).i_queue = -(9 as libc::c_int);
    (*cb).i_bytes_outstanding = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_cabac_encode_init(
    mut cb: *mut x264_cabac_t,
    mut p_data: *mut uint8_t,
    mut p_end: *mut uint8_t,
) {
    x264_8_cabac_encode_init_core(cb);
    (*cb).p_start = p_data;
    (*cb).p = p_data;
    (*cb).p_end = p_end;
}
#[inline]
unsafe extern "C" fn cabac_putbyte(mut cb: *mut x264_cabac_t) {
    if (*cb).i_queue >= 0 as libc::c_int {
        let mut out: libc::c_int = (*cb).i_low >> ((*cb).i_queue + 10 as libc::c_int);
        (*cb).i_low &= ((0x400 as libc::c_int) << (*cb).i_queue) - 1 as libc::c_int;
        (*cb).i_queue -= 8 as libc::c_int;
        if out & 0xff as libc::c_int == 0xff as libc::c_int {
            (*cb).i_bytes_outstanding += 1;
            (*cb).i_bytes_outstanding;
        } else {
            let mut carry: libc::c_int = out >> 8 as libc::c_int;
            let mut bytes_outstanding: libc::c_int = (*cb).i_bytes_outstanding;
            let fresh0 = &mut (*((*cb).p).offset(-(1 as libc::c_int) as isize));
            *fresh0 = (*fresh0 as libc::c_int + carry) as uint8_t;
            while bytes_outstanding > 0 as libc::c_int {
                let fresh1 = (*cb).p;
                (*cb).p = ((*cb).p).offset(1);
                *fresh1 = (carry - 1 as libc::c_int) as uint8_t;
                bytes_outstanding -= 1;
                bytes_outstanding;
            }
            let fresh2 = (*cb).p;
            (*cb).p = ((*cb).p).offset(1);
            *fresh2 = out as uint8_t;
            (*cb).i_bytes_outstanding = 0 as libc::c_int;
        }
    }
}
#[inline]
unsafe extern "C" fn cabac_encode_renorm(mut cb: *mut x264_cabac_t) {
    let mut shift: libc::c_int =
        x264_cabac_renorm_shift[((*cb).i_range >> 3 as libc::c_int) as usize] as libc::c_int;
    (*cb).i_range <<= shift;
    (*cb).i_low <<= shift;
    (*cb).i_queue += shift;
    cabac_putbyte(cb);
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_cabac_encode_decision_c(
    mut cb: *mut x264_cabac_t,
    mut i_ctx: libc::c_int,
    mut b: libc::c_int,
) {
    let mut i_state: libc::c_int = (*cb).state[i_ctx as usize] as libc::c_int;
    let mut i_range_lps: libc::c_int = x264_cabac_range_lps[(i_state >> 1 as libc::c_int) as usize]
        [(((*cb).i_range >> 6 as libc::c_int) - 4 as libc::c_int) as usize]
        as libc::c_int;
    (*cb).i_range -= i_range_lps;
    if b != i_state & 1 as libc::c_int {
        (*cb).i_low += (*cb).i_range;
        (*cb).i_range = i_range_lps;
    }
    (*cb).state[i_ctx as usize] = x264_cabac_transition[i_state as usize][b as usize];
    cabac_encode_renorm(cb);
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_cabac_encode_bypass_c(
    mut cb: *mut x264_cabac_t,
    mut b: libc::c_int,
) {
    (*cb).i_low <<= 1 as libc::c_int;
    (*cb).i_low += b & (*cb).i_range;
    (*cb).i_queue += 1 as libc::c_int;
    cabac_putbyte(cb);
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_cabac_encode_ue_bypass(
    mut cb: *mut x264_cabac_t,
    mut exp_bits: libc::c_int,
    mut val: libc::c_int,
) {
    let mut v: uint32_t = (val + ((1 as libc::c_int) << exp_bits)) as uint32_t;
    let mut k: libc::c_int = 31 as libc::c_int - v.leading_zeros() as i32;
    let mut x: uint32_t =
        ((bypass_lut[(k - exp_bits) as usize] as uint32_t) << exp_bits).wrapping_add(v);
    k = 2 as libc::c_int * k + 1 as libc::c_int - exp_bits;
    let mut i: libc::c_int = ((k - 1 as libc::c_int) & 7 as libc::c_int) + 1 as libc::c_int;
    loop {
        k -= i;
        (*cb).i_low <<= i;
        (*cb).i_low = ((*cb).i_low as uint32_t)
            .wrapping_add(((x >> k) & 0xff as libc::c_int as uint32_t) * (*cb).i_range as uint32_t)
            as libc::c_int as libc::c_int;
        (*cb).i_queue += i;
        cabac_putbyte(cb);
        i = 8 as libc::c_int;
        if k <= 0 as libc::c_int {
            break;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_cabac_encode_terminal_c(mut cb: *mut x264_cabac_t) {
    (*cb).i_range -= 2 as libc::c_int;
    cabac_encode_renorm(cb);
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_cabac_encode_flush(mut h: *mut x264_t, mut cb: *mut x264_cabac_t) {
    (*cb).i_low += (*cb).i_range - 2 as libc::c_int;
    (*cb).i_low |= 1 as libc::c_int;
    (*cb).i_low <<= 9 as libc::c_int;
    (*cb).i_queue += 9 as libc::c_int;
    cabac_putbyte(cb);
    cabac_putbyte(cb);
    (*cb).i_low <<= -(*cb).i_queue;
    (*cb).i_low |= ((0x35a4e4f5 as libc::c_int >> ((*h).i_frame & 31 as libc::c_int))
        & 1 as libc::c_int)
        << 10 as libc::c_int;
    (*cb).i_queue = 0 as libc::c_int;
    cabac_putbyte(cb);
    while (*cb).i_bytes_outstanding > 0 as libc::c_int {
        let fresh3 = (*cb).p;
        (*cb).p = ((*cb).p).offset(1);
        *fresh3 = 0xff as libc::c_int as uint8_t;
        (*cb).i_bytes_outstanding -= 1;
        (*cb).i_bytes_outstanding;
    }
}
