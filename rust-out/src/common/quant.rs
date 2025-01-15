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
extern "C" {}
unsafe extern "C" fn quant_8x8(
    mut dct: *mut dctcoef,
    mut mf: *mut udctcoef,
    mut bias: *mut udctcoef,
) -> libc::c_int {
    let mut nz: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 64 as libc::c_int {
        if *dct.offset(i as isize) as libc::c_int > 0 as libc::c_int {
            *dct.offset(i as isize) = (((*bias.offset(i as isize) as uint32_t)
                .wrapping_add(*dct.offset(i as isize) as uint32_t)
                * *mf.offset(i as isize) as uint32_t)
                >> 16 as libc::c_int) as dctcoef;
        } else {
            *dct.offset(i as isize) = -((((*bias.offset(i as isize) as uint32_t)
                .wrapping_add(-(*dct.offset(i as isize) as libc::c_int) as uint32_t)
                * *mf.offset(i as isize) as uint32_t)
                >> 16 as libc::c_int) as int32_t) as dctcoef;
        }
        nz |= *dct.offset(i as isize) as libc::c_int;
        i += 1;
        i;
    }
    (nz != 0) as libc::c_int
}
unsafe extern "C" fn quant_4x4(
    mut dct: *mut dctcoef,
    mut mf: *mut udctcoef,
    mut bias: *mut udctcoef,
) -> libc::c_int {
    let mut nz: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        if *dct.offset(i as isize) as libc::c_int > 0 as libc::c_int {
            *dct.offset(i as isize) = (((*bias.offset(i as isize) as uint32_t)
                .wrapping_add(*dct.offset(i as isize) as uint32_t)
                * *mf.offset(i as isize) as uint32_t)
                >> 16 as libc::c_int) as dctcoef;
        } else {
            *dct.offset(i as isize) = -((((*bias.offset(i as isize) as uint32_t)
                .wrapping_add(-(*dct.offset(i as isize) as libc::c_int) as uint32_t)
                * *mf.offset(i as isize) as uint32_t)
                >> 16 as libc::c_int) as int32_t) as dctcoef;
        }
        nz |= *dct.offset(i as isize) as libc::c_int;
        i += 1;
        i;
    }
    (nz != 0) as libc::c_int
}
unsafe extern "C" fn quant_4x4x4(
    mut dct: *mut [dctcoef; 16],
    mut mf: *mut udctcoef,
    mut bias: *mut udctcoef,
) -> libc::c_int {
    let mut nza: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0 as libc::c_int;
    while j < 4 as libc::c_int {
        let mut nz: libc::c_int = 0 as libc::c_int;
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < 16 as libc::c_int {
            if (*dct.offset(j as isize))[i as usize] as libc::c_int > 0 as libc::c_int {
                (*dct.offset(j as isize))[i as usize] = (((*bias.offset(i as isize) as uint32_t)
                    .wrapping_add((*dct.offset(j as isize))[i as usize] as uint32_t)
                    * *mf.offset(i as isize) as uint32_t)
                    >> 16 as libc::c_int)
                    as dctcoef;
            } else {
                (*dct.offset(j as isize))[i as usize] =
                    -((((*bias.offset(i as isize) as uint32_t).wrapping_add(
                        -((*dct.offset(j as isize))[i as usize] as libc::c_int) as uint32_t,
                    ) * *mf.offset(i as isize) as uint32_t)
                        >> 16 as libc::c_int) as int32_t) as dctcoef;
            }
            nz |= (*dct.offset(j as isize))[i as usize] as libc::c_int;
            i += 1;
            i;
        }
        nza |= ((nz != 0) as libc::c_int) << j;
        j += 1;
        j;
    }
    nza
}
unsafe extern "C" fn quant_4x4_dc(
    mut dct: *mut dctcoef,
    mut mf: libc::c_int,
    mut bias: libc::c_int,
) -> libc::c_int {
    let mut nz: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        if *dct.offset(i as isize) as libc::c_int > 0 as libc::c_int {
            *dct.offset(i as isize) = (((bias as uint32_t)
                .wrapping_add(*dct.offset(i as isize) as uint32_t)
                * mf as uint32_t)
                >> 16 as libc::c_int) as dctcoef;
        } else {
            *dct.offset(i as isize) = -((((bias as uint32_t)
                .wrapping_add(-(*dct.offset(i as isize) as libc::c_int) as uint32_t)
                * mf as uint32_t)
                >> 16 as libc::c_int) as int32_t) as dctcoef;
        }
        nz |= *dct.offset(i as isize) as libc::c_int;
        i += 1;
        i;
    }
    (nz != 0) as libc::c_int
}
unsafe extern "C" fn quant_2x2_dc(
    mut dct: *mut dctcoef,
    mut mf: libc::c_int,
    mut bias: libc::c_int,
) -> libc::c_int {
    let mut nz: libc::c_int = 0 as libc::c_int;
    if *dct.offset(0 as libc::c_int as isize) as libc::c_int > 0 as libc::c_int {
        *dct.offset(0 as libc::c_int as isize) = (((bias as uint32_t)
            .wrapping_add(*dct.offset(0 as libc::c_int as isize) as uint32_t)
            * mf as uint32_t)
            >> 16 as libc::c_int) as dctcoef;
    } else {
        *dct.offset(0 as libc::c_int as isize) = -((((bias as uint32_t)
            .wrapping_add(-(*dct.offset(0 as libc::c_int as isize) as libc::c_int) as uint32_t)
            * mf as uint32_t)
            >> 16 as libc::c_int) as int32_t)
            as dctcoef;
    }
    nz |= *dct.offset(0 as libc::c_int as isize) as libc::c_int;
    if *dct.offset(1 as libc::c_int as isize) as libc::c_int > 0 as libc::c_int {
        *dct.offset(1 as libc::c_int as isize) = (((bias as uint32_t)
            .wrapping_add(*dct.offset(1 as libc::c_int as isize) as uint32_t)
            * mf as uint32_t)
            >> 16 as libc::c_int) as dctcoef;
    } else {
        *dct.offset(1 as libc::c_int as isize) = -((((bias as uint32_t)
            .wrapping_add(-(*dct.offset(1 as libc::c_int as isize) as libc::c_int) as uint32_t)
            * mf as uint32_t)
            >> 16 as libc::c_int) as int32_t)
            as dctcoef;
    }
    nz |= *dct.offset(1 as libc::c_int as isize) as libc::c_int;
    if *dct.offset(2 as libc::c_int as isize) as libc::c_int > 0 as libc::c_int {
        *dct.offset(2 as libc::c_int as isize) = (((bias as uint32_t)
            .wrapping_add(*dct.offset(2 as libc::c_int as isize) as uint32_t)
            * mf as uint32_t)
            >> 16 as libc::c_int) as dctcoef;
    } else {
        *dct.offset(2 as libc::c_int as isize) = -((((bias as uint32_t)
            .wrapping_add(-(*dct.offset(2 as libc::c_int as isize) as libc::c_int) as uint32_t)
            * mf as uint32_t)
            >> 16 as libc::c_int) as int32_t)
            as dctcoef;
    }
    nz |= *dct.offset(2 as libc::c_int as isize) as libc::c_int;
    if *dct.offset(3 as libc::c_int as isize) as libc::c_int > 0 as libc::c_int {
        *dct.offset(3 as libc::c_int as isize) = (((bias as uint32_t)
            .wrapping_add(*dct.offset(3 as libc::c_int as isize) as uint32_t)
            * mf as uint32_t)
            >> 16 as libc::c_int) as dctcoef;
    } else {
        *dct.offset(3 as libc::c_int as isize) = -((((bias as uint32_t)
            .wrapping_add(-(*dct.offset(3 as libc::c_int as isize) as libc::c_int) as uint32_t)
            * mf as uint32_t)
            >> 16 as libc::c_int) as int32_t)
            as dctcoef;
    }
    nz |= *dct.offset(3 as libc::c_int as isize) as libc::c_int;
    (nz != 0) as libc::c_int
}
unsafe extern "C" fn dequant_4x4(
    mut dct: *mut dctcoef,
    mut dequant_mf: *mut [libc::c_int; 16],
    mut i_qp: libc::c_int,
) {
    let i_mf: libc::c_int = i_qp % 6 as libc::c_int;
    let i_qbits: libc::c_int = i_qp / 6 as libc::c_int - 4 as libc::c_int;
    if i_qbits >= 0 as libc::c_int {
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < 16 as libc::c_int {
            *dct.offset(i as isize) = (*dct.offset(i as isize) as libc::c_int
                * (*dequant_mf.offset(i_mf as isize))[i as usize]
                * ((1 as libc::c_int) << i_qbits)) as dctcoef;
            i += 1;
            i;
        }
    } else {
        let f: libc::c_int = (1 as libc::c_int) << (-i_qbits - 1 as libc::c_int);
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < 16 as libc::c_int {
            *dct.offset(i_0 as isize) = ((*dct.offset(i_0 as isize) as libc::c_int
                * (*dequant_mf.offset(i_mf as isize))[i_0 as usize]
                + f)
                >> -i_qbits) as dctcoef;
            i_0 += 1;
            i_0;
        }
    };
}
unsafe extern "C" fn dequant_8x8(
    mut dct: *mut dctcoef,
    mut dequant_mf: *mut [libc::c_int; 64],
    mut i_qp: libc::c_int,
) {
    let i_mf: libc::c_int = i_qp % 6 as libc::c_int;
    let i_qbits: libc::c_int = i_qp / 6 as libc::c_int - 6 as libc::c_int;
    if i_qbits >= 0 as libc::c_int {
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < 64 as libc::c_int {
            *dct.offset(i as isize) = (*dct.offset(i as isize) as libc::c_int
                * (*dequant_mf.offset(i_mf as isize))[i as usize]
                * ((1 as libc::c_int) << i_qbits)) as dctcoef;
            i += 1;
            i;
        }
    } else {
        let f: libc::c_int = (1 as libc::c_int) << (-i_qbits - 1 as libc::c_int);
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < 64 as libc::c_int {
            *dct.offset(i_0 as isize) = ((*dct.offset(i_0 as isize) as libc::c_int
                * (*dequant_mf.offset(i_mf as isize))[i_0 as usize]
                + f)
                >> -i_qbits) as dctcoef;
            i_0 += 1;
            i_0;
        }
    };
}
unsafe extern "C" fn dequant_4x4_dc(
    mut dct: *mut dctcoef,
    mut dequant_mf: *mut [libc::c_int; 16],
    mut i_qp: libc::c_int,
) {
    let i_qbits: libc::c_int = i_qp / 6 as libc::c_int - 6 as libc::c_int;
    if i_qbits >= 0 as libc::c_int {
        let i_dmf: libc::c_int = (*dequant_mf.offset((i_qp % 6 as libc::c_int) as isize))
            [0 as libc::c_int as usize]
            << i_qbits;
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < 16 as libc::c_int {
            let fresh0 = &mut (*dct.offset(i as isize));
            *fresh0 = (*fresh0 as libc::c_int * i_dmf) as dctcoef;
            i += 1;
            i;
        }
    } else {
        let i_dmf_0: libc::c_int =
            (*dequant_mf.offset((i_qp % 6 as libc::c_int) as isize))[0 as libc::c_int as usize];
        let f: libc::c_int = (1 as libc::c_int) << (-i_qbits - 1 as libc::c_int);
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < 16 as libc::c_int {
            *dct.offset(i_0 as isize) =
                ((*dct.offset(i_0 as isize) as libc::c_int * i_dmf_0 + f) >> -i_qbits) as dctcoef;
            i_0 += 1;
            i_0;
        }
    };
}
unsafe extern "C" fn idct_dequant_2x4_dc(
    mut dct: *mut dctcoef,
    mut dct4x4: *mut [dctcoef; 16],
    mut dequant_mf: *mut [libc::c_int; 16],
    mut i_qp: libc::c_int,
) {
    let mut a0: libc::c_int = *dct.offset(0 as libc::c_int as isize) as libc::c_int
        + *dct.offset(1 as libc::c_int as isize) as libc::c_int;
    let mut a1: libc::c_int = *dct.offset(2 as libc::c_int as isize) as libc::c_int
        + *dct.offset(3 as libc::c_int as isize) as libc::c_int;
    let mut a2: libc::c_int = *dct.offset(4 as libc::c_int as isize) as libc::c_int
        + *dct.offset(5 as libc::c_int as isize) as libc::c_int;
    let mut a3: libc::c_int = *dct.offset(6 as libc::c_int as isize) as libc::c_int
        + *dct.offset(7 as libc::c_int as isize) as libc::c_int;
    let mut a4: libc::c_int = *dct.offset(0 as libc::c_int as isize) as libc::c_int
        - *dct.offset(1 as libc::c_int as isize) as libc::c_int;
    let mut a5: libc::c_int = *dct.offset(2 as libc::c_int as isize) as libc::c_int
        - *dct.offset(3 as libc::c_int as isize) as libc::c_int;
    let mut a6: libc::c_int = *dct.offset(4 as libc::c_int as isize) as libc::c_int
        - *dct.offset(5 as libc::c_int as isize) as libc::c_int;
    let mut a7: libc::c_int = *dct.offset(6 as libc::c_int as isize) as libc::c_int
        - *dct.offset(7 as libc::c_int as isize) as libc::c_int;
    let mut b0: libc::c_int = a0 + a1;
    let mut b1: libc::c_int = a2 + a3;
    let mut b2: libc::c_int = a4 + a5;
    let mut b3: libc::c_int = a6 + a7;
    let mut b4: libc::c_int = a0 - a1;
    let mut b5: libc::c_int = a2 - a3;
    let mut b6: libc::c_int = a4 - a5;
    let mut b7: libc::c_int = a6 - a7;
    let mut dmf: libc::c_int = (*dequant_mf.offset((i_qp % 6 as libc::c_int) as isize))
        [0 as libc::c_int as usize]
        << (i_qp / 6 as libc::c_int);
    (*dct4x4.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] =
        (((b0 + b1) * dmf + 32 as libc::c_int) >> 6 as libc::c_int) as dctcoef;
    (*dct4x4.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] =
        (((b2 + b3) * dmf + 32 as libc::c_int) >> 6 as libc::c_int) as dctcoef;
    (*dct4x4.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize] =
        (((b0 - b1) * dmf + 32 as libc::c_int) >> 6 as libc::c_int) as dctcoef;
    (*dct4x4.offset(3 as libc::c_int as isize))[0 as libc::c_int as usize] =
        (((b2 - b3) * dmf + 32 as libc::c_int) >> 6 as libc::c_int) as dctcoef;
    (*dct4x4.offset(4 as libc::c_int as isize))[0 as libc::c_int as usize] =
        (((b4 - b5) * dmf + 32 as libc::c_int) >> 6 as libc::c_int) as dctcoef;
    (*dct4x4.offset(5 as libc::c_int as isize))[0 as libc::c_int as usize] =
        (((b6 - b7) * dmf + 32 as libc::c_int) >> 6 as libc::c_int) as dctcoef;
    (*dct4x4.offset(6 as libc::c_int as isize))[0 as libc::c_int as usize] =
        (((b4 + b5) * dmf + 32 as libc::c_int) >> 6 as libc::c_int) as dctcoef;
    (*dct4x4.offset(7 as libc::c_int as isize))[0 as libc::c_int as usize] =
        (((b6 + b7) * dmf + 32 as libc::c_int) >> 6 as libc::c_int) as dctcoef;
}
unsafe extern "C" fn idct_dequant_2x4_dconly(
    mut dct: *mut dctcoef,
    mut dequant_mf: *mut [libc::c_int; 16],
    mut i_qp: libc::c_int,
) {
    let mut a0: libc::c_int = *dct.offset(0 as libc::c_int as isize) as libc::c_int
        + *dct.offset(1 as libc::c_int as isize) as libc::c_int;
    let mut a1: libc::c_int = *dct.offset(2 as libc::c_int as isize) as libc::c_int
        + *dct.offset(3 as libc::c_int as isize) as libc::c_int;
    let mut a2: libc::c_int = *dct.offset(4 as libc::c_int as isize) as libc::c_int
        + *dct.offset(5 as libc::c_int as isize) as libc::c_int;
    let mut a3: libc::c_int = *dct.offset(6 as libc::c_int as isize) as libc::c_int
        + *dct.offset(7 as libc::c_int as isize) as libc::c_int;
    let mut a4: libc::c_int = *dct.offset(0 as libc::c_int as isize) as libc::c_int
        - *dct.offset(1 as libc::c_int as isize) as libc::c_int;
    let mut a5: libc::c_int = *dct.offset(2 as libc::c_int as isize) as libc::c_int
        - *dct.offset(3 as libc::c_int as isize) as libc::c_int;
    let mut a6: libc::c_int = *dct.offset(4 as libc::c_int as isize) as libc::c_int
        - *dct.offset(5 as libc::c_int as isize) as libc::c_int;
    let mut a7: libc::c_int = *dct.offset(6 as libc::c_int as isize) as libc::c_int
        - *dct.offset(7 as libc::c_int as isize) as libc::c_int;
    let mut b0: libc::c_int = a0 + a1;
    let mut b1: libc::c_int = a2 + a3;
    let mut b2: libc::c_int = a4 + a5;
    let mut b3: libc::c_int = a6 + a7;
    let mut b4: libc::c_int = a0 - a1;
    let mut b5: libc::c_int = a2 - a3;
    let mut b6: libc::c_int = a4 - a5;
    let mut b7: libc::c_int = a6 - a7;
    let mut dmf: libc::c_int = (*dequant_mf.offset((i_qp % 6 as libc::c_int) as isize))
        [0 as libc::c_int as usize]
        << (i_qp / 6 as libc::c_int);
    *dct.offset(0 as libc::c_int as isize) =
        (((b0 + b1) * dmf + 32 as libc::c_int) >> 6 as libc::c_int) as dctcoef;
    *dct.offset(1 as libc::c_int as isize) =
        (((b2 + b3) * dmf + 32 as libc::c_int) >> 6 as libc::c_int) as dctcoef;
    *dct.offset(2 as libc::c_int as isize) =
        (((b0 - b1) * dmf + 32 as libc::c_int) >> 6 as libc::c_int) as dctcoef;
    *dct.offset(3 as libc::c_int as isize) =
        (((b2 - b3) * dmf + 32 as libc::c_int) >> 6 as libc::c_int) as dctcoef;
    *dct.offset(4 as libc::c_int as isize) =
        (((b4 - b5) * dmf + 32 as libc::c_int) >> 6 as libc::c_int) as dctcoef;
    *dct.offset(5 as libc::c_int as isize) =
        (((b6 - b7) * dmf + 32 as libc::c_int) >> 6 as libc::c_int) as dctcoef;
    *dct.offset(6 as libc::c_int as isize) =
        (((b4 + b5) * dmf + 32 as libc::c_int) >> 6 as libc::c_int) as dctcoef;
    *dct.offset(7 as libc::c_int as isize) =
        (((b6 + b7) * dmf + 32 as libc::c_int) >> 6 as libc::c_int) as dctcoef;
}
#[inline(always)]
unsafe extern "C" fn optimize_chroma_idct_dequant_2x4(
    mut out: *mut dctcoef,
    mut dct: *mut dctcoef,
    mut dmf: libc::c_int,
) {
    let mut a0: libc::c_int = *dct.offset(0 as libc::c_int as isize) as libc::c_int
        + *dct.offset(1 as libc::c_int as isize) as libc::c_int;
    let mut a1: libc::c_int = *dct.offset(2 as libc::c_int as isize) as libc::c_int
        + *dct.offset(3 as libc::c_int as isize) as libc::c_int;
    let mut a2: libc::c_int = *dct.offset(4 as libc::c_int as isize) as libc::c_int
        + *dct.offset(5 as libc::c_int as isize) as libc::c_int;
    let mut a3: libc::c_int = *dct.offset(6 as libc::c_int as isize) as libc::c_int
        + *dct.offset(7 as libc::c_int as isize) as libc::c_int;
    let mut a4: libc::c_int = *dct.offset(0 as libc::c_int as isize) as libc::c_int
        - *dct.offset(1 as libc::c_int as isize) as libc::c_int;
    let mut a5: libc::c_int = *dct.offset(2 as libc::c_int as isize) as libc::c_int
        - *dct.offset(3 as libc::c_int as isize) as libc::c_int;
    let mut a6: libc::c_int = *dct.offset(4 as libc::c_int as isize) as libc::c_int
        - *dct.offset(5 as libc::c_int as isize) as libc::c_int;
    let mut a7: libc::c_int = *dct.offset(6 as libc::c_int as isize) as libc::c_int
        - *dct.offset(7 as libc::c_int as isize) as libc::c_int;
    let mut b0: libc::c_int = a0 + a1;
    let mut b1: libc::c_int = a2 + a3;
    let mut b2: libc::c_int = a4 + a5;
    let mut b3: libc::c_int = a6 + a7;
    let mut b4: libc::c_int = a0 - a1;
    let mut b5: libc::c_int = a2 - a3;
    let mut b6: libc::c_int = a4 - a5;
    let mut b7: libc::c_int = a6 - a7;
    *out.offset(0 as libc::c_int as isize) =
        (((b0 + b1) * dmf + 2080 as libc::c_int) >> 6 as libc::c_int) as dctcoef;
    *out.offset(1 as libc::c_int as isize) =
        (((b2 + b3) * dmf + 2080 as libc::c_int) >> 6 as libc::c_int) as dctcoef;
    *out.offset(2 as libc::c_int as isize) =
        (((b0 - b1) * dmf + 2080 as libc::c_int) >> 6 as libc::c_int) as dctcoef;
    *out.offset(3 as libc::c_int as isize) =
        (((b2 - b3) * dmf + 2080 as libc::c_int) >> 6 as libc::c_int) as dctcoef;
    *out.offset(4 as libc::c_int as isize) =
        (((b4 - b5) * dmf + 2080 as libc::c_int) >> 6 as libc::c_int) as dctcoef;
    *out.offset(5 as libc::c_int as isize) =
        (((b6 - b7) * dmf + 2080 as libc::c_int) >> 6 as libc::c_int) as dctcoef;
    *out.offset(6 as libc::c_int as isize) =
        (((b4 + b5) * dmf + 2080 as libc::c_int) >> 6 as libc::c_int) as dctcoef;
    *out.offset(7 as libc::c_int as isize) =
        (((b6 + b7) * dmf + 2080 as libc::c_int) >> 6 as libc::c_int) as dctcoef;
}
#[inline(always)]
unsafe extern "C" fn optimize_chroma_idct_dequant_2x2(
    mut out: *mut dctcoef,
    mut dct: *mut dctcoef,
    mut dmf: libc::c_int,
) {
    let mut d0: libc::c_int = *dct.offset(0 as libc::c_int as isize) as libc::c_int
        + *dct.offset(1 as libc::c_int as isize) as libc::c_int;
    let mut d1: libc::c_int = *dct.offset(2 as libc::c_int as isize) as libc::c_int
        + *dct.offset(3 as libc::c_int as isize) as libc::c_int;
    let mut d2: libc::c_int = *dct.offset(0 as libc::c_int as isize) as libc::c_int
        - *dct.offset(1 as libc::c_int as isize) as libc::c_int;
    let mut d3: libc::c_int = *dct.offset(2 as libc::c_int as isize) as libc::c_int
        - *dct.offset(3 as libc::c_int as isize) as libc::c_int;
    *out.offset(0 as libc::c_int as isize) =
        ((((d0 + d1) * dmf) >> 5 as libc::c_int) + 32 as libc::c_int) as dctcoef;
    *out.offset(1 as libc::c_int as isize) =
        ((((d0 - d1) * dmf) >> 5 as libc::c_int) + 32 as libc::c_int) as dctcoef;
    *out.offset(2 as libc::c_int as isize) =
        ((((d2 + d3) * dmf) >> 5 as libc::c_int) + 32 as libc::c_int) as dctcoef;
    *out.offset(3 as libc::c_int as isize) =
        ((((d2 - d3) * dmf) >> 5 as libc::c_int) + 32 as libc::c_int) as dctcoef;
}
#[inline(always)]
unsafe extern "C" fn optimize_chroma_round(
    mut ref_0: *mut dctcoef,
    mut dct: *mut dctcoef,
    mut dequant_mf: libc::c_int,
    mut chroma422: libc::c_int,
) -> libc::c_int {
    let mut out: [dctcoef; 8] = [0; 8];
    if chroma422 != 0 {
        optimize_chroma_idct_dequant_2x4(out.as_mut_ptr(), dct, dequant_mf);
    } else {
        optimize_chroma_idct_dequant_2x2(out.as_mut_ptr(), dct, dequant_mf);
    }
    let mut sum: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i
        < (if chroma422 != 0 {
            8 as libc::c_int
        } else {
            4 as libc::c_int
        })
    {
        sum |= *ref_0.offset(i as isize) as libc::c_int ^ out[i as usize] as libc::c_int;
        i += 1;
        i;
    }
    sum >> 6 as libc::c_int
}
#[inline(always)]
unsafe extern "C" fn optimize_chroma_dc_internal(
    mut dct: *mut dctcoef,
    mut dequant_mf: libc::c_int,
    mut chroma422: libc::c_int,
) -> libc::c_int {
    let mut dct_orig: [dctcoef; 8] = [0; 8];
    let mut coeff: libc::c_int = 0;
    let mut nz: libc::c_int = 0;
    if chroma422 != 0 {
        optimize_chroma_idct_dequant_2x4(dct_orig.as_mut_ptr(), dct, dequant_mf);
    } else {
        optimize_chroma_idct_dequant_2x2(dct_orig.as_mut_ptr(), dct, dequant_mf);
    }
    let mut sum: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i
        < (if chroma422 != 0 {
            8 as libc::c_int
        } else {
            4 as libc::c_int
        })
    {
        sum |= dct_orig[i as usize] as libc::c_int;
        i += 1;
        i;
    }
    if sum >> 6 as libc::c_int == 0 {
        return 0 as libc::c_int;
    }
    nz = 0 as libc::c_int;
    coeff = if chroma422 != 0 {
        7 as libc::c_int
    } else {
        3 as libc::c_int
    };
    while coeff >= 0 as libc::c_int {
        let mut level: libc::c_int = *dct.offset(coeff as isize) as libc::c_int;
        let mut sign: libc::c_int = (level >> 31 as libc::c_int) | 1 as libc::c_int;
        while level != 0 {
            *dct.offset(coeff as isize) = (level - sign) as dctcoef;
            if optimize_chroma_round(dct_orig.as_mut_ptr(), dct, dequant_mf, chroma422) != 0 {
                nz = 1 as libc::c_int;
                *dct.offset(coeff as isize) = level as dctcoef;
                break;
            } else {
                level -= sign;
            }
        }
        coeff -= 1;
        coeff;
    }
    nz
}
unsafe extern "C" fn optimize_chroma_2x2_dc(
    mut dct: *mut dctcoef,
    mut dequant_mf: libc::c_int,
) -> libc::c_int {
    optimize_chroma_dc_internal(dct, dequant_mf, 0 as libc::c_int)
}
unsafe extern "C" fn optimize_chroma_2x4_dc(
    mut dct: *mut dctcoef,
    mut dequant_mf: libc::c_int,
) -> libc::c_int {
    optimize_chroma_dc_internal(dct, dequant_mf, 1 as libc::c_int)
}
unsafe extern "C" fn denoise_dct(
    mut dct: *mut dctcoef,
    mut sum: *mut uint32_t,
    mut offset: *mut udctcoef,
    mut size: libc::c_int,
) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < size {
        let mut level: libc::c_int = *dct.offset(i as isize) as libc::c_int;
        let mut sign: libc::c_int = level >> 31 as libc::c_int;
        level = (level + sign) ^ sign;
        let fresh1 = &mut (*sum.offset(i as isize));
        *fresh1 = (*fresh1).wrapping_add(level as uint32_t);
        level -= *offset.offset(i as isize) as libc::c_int;
        *dct.offset(i as isize) = (if level < 0 as libc::c_int {
            0 as libc::c_int
        } else {
            (level ^ sign) - sign
        }) as dctcoef;
        i += 1;
        i;
    }
}
#[inline(always)]
unsafe extern "C" fn decimate_score_internal(
    mut dct: *mut dctcoef,
    mut i_max: libc::c_int,
) -> libc::c_int {
    let mut ds_table: *const uint8_t = if i_max == 64 as libc::c_int {
        x264_decimate_table8.as_ptr()
    } else {
        x264_decimate_table4.as_ptr()
    };
    let mut i_score: libc::c_int = 0 as libc::c_int;
    let mut idx: libc::c_int = i_max - 1 as libc::c_int;
    while idx >= 0 as libc::c_int && *dct.offset(idx as isize) as libc::c_int == 0 as libc::c_int {
        idx -= 1;
        idx;
    }
    while idx >= 0 as libc::c_int {
        let mut i_run: libc::c_int = 0;
        let fresh2 = idx;
        idx -= 1;
        if (*dct.offset(fresh2 as isize) as libc::c_int + 1 as libc::c_int) as libc::c_uint
            > 2 as libc::c_int as libc::c_uint
        {
            return 9 as libc::c_int;
        }
        i_run = 0 as libc::c_int;
        while idx >= 0 as libc::c_int
            && *dct.offset(idx as isize) as libc::c_int == 0 as libc::c_int
        {
            idx -= 1;
            idx;
            i_run += 1;
            i_run;
        }
        i_score += *ds_table.offset(i_run as isize) as libc::c_int;
    }
    i_score
}
unsafe extern "C" fn decimate_score15(mut dct: *mut dctcoef) -> libc::c_int {
    decimate_score_internal(dct.offset(1 as libc::c_int as isize), 15 as libc::c_int)
}
unsafe extern "C" fn decimate_score16(mut dct: *mut dctcoef) -> libc::c_int {
    decimate_score_internal(dct, 16 as libc::c_int)
}
unsafe extern "C" fn decimate_score64(mut dct: *mut dctcoef) -> libc::c_int {
    decimate_score_internal(dct, 64 as libc::c_int)
}
unsafe extern "C" fn coeff_last4(mut l: *mut dctcoef) -> libc::c_int {
    let mut i_last: libc::c_int = 4 as libc::c_int - 1 as libc::c_int;
    while i_last >= 0 as libc::c_int
        && *l.offset(i_last as isize) as libc::c_int == 0 as libc::c_int
    {
        i_last -= 1;
        i_last;
    }
    i_last
}
unsafe extern "C" fn coeff_last8(mut l: *mut dctcoef) -> libc::c_int {
    let mut i_last: libc::c_int = 8 as libc::c_int - 1 as libc::c_int;
    while i_last >= 0 as libc::c_int
        && *l.offset(i_last as isize) as libc::c_int == 0 as libc::c_int
    {
        i_last -= 1;
        i_last;
    }
    i_last
}
unsafe extern "C" fn coeff_last15(mut l: *mut dctcoef) -> libc::c_int {
    let mut i_last: libc::c_int = 15 as libc::c_int - 1 as libc::c_int;
    while i_last >= 0 as libc::c_int
        && *l.offset(i_last as isize) as libc::c_int == 0 as libc::c_int
    {
        i_last -= 1;
        i_last;
    }
    i_last
}
unsafe extern "C" fn coeff_last16(mut l: *mut dctcoef) -> libc::c_int {
    let mut i_last: libc::c_int = 16 as libc::c_int - 1 as libc::c_int;
    while i_last >= 0 as libc::c_int
        && *l.offset(i_last as isize) as libc::c_int == 0 as libc::c_int
    {
        i_last -= 1;
        i_last;
    }
    i_last
}
unsafe extern "C" fn coeff_last64(mut l: *mut dctcoef) -> libc::c_int {
    let mut i_last: libc::c_int = 64 as libc::c_int - 1 as libc::c_int;
    while i_last >= 0 as libc::c_int
        && *l.offset(i_last as isize) as libc::c_int == 0 as libc::c_int
    {
        i_last -= 1;
        i_last;
    }
    i_last
}
unsafe extern "C" fn coeff_level_run4(
    mut dct: *mut dctcoef,
    mut runlevel: *mut x264_run_level_t,
) -> libc::c_int {
    (*runlevel).last = coeff_last4(dct);
    let mut i_last: libc::c_int = (*runlevel).last;
    let mut i_total: libc::c_int = 0 as libc::c_int;
    let mut mask: libc::c_int = 0 as libc::c_int;
    loop {
        let fresh3 = i_total;
        i_total += 1;
        (*runlevel).level[fresh3 as usize] = *dct.offset(i_last as isize);
        mask |= (1 as libc::c_int) << i_last;
        loop {
            i_last -= 1;
            if !(i_last >= 0 as libc::c_int
                && *dct.offset(i_last as isize) as libc::c_int == 0 as libc::c_int)
            {
                break;
            }
        }
        if i_last < 0 as libc::c_int {
            break;
        }
    }
    (*runlevel).mask = mask;
    i_total
}
unsafe extern "C" fn coeff_level_run8(
    mut dct: *mut dctcoef,
    mut runlevel: *mut x264_run_level_t,
) -> libc::c_int {
    (*runlevel).last = coeff_last8(dct);
    let mut i_last: libc::c_int = (*runlevel).last;
    let mut i_total: libc::c_int = 0 as libc::c_int;
    let mut mask: libc::c_int = 0 as libc::c_int;
    loop {
        let fresh4 = i_total;
        i_total += 1;
        (*runlevel).level[fresh4 as usize] = *dct.offset(i_last as isize);
        mask |= (1 as libc::c_int) << i_last;
        loop {
            i_last -= 1;
            if !(i_last >= 0 as libc::c_int
                && *dct.offset(i_last as isize) as libc::c_int == 0 as libc::c_int)
            {
                break;
            }
        }
        if i_last < 0 as libc::c_int {
            break;
        }
    }
    (*runlevel).mask = mask;
    i_total
}
unsafe extern "C" fn coeff_level_run15(
    mut dct: *mut dctcoef,
    mut runlevel: *mut x264_run_level_t,
) -> libc::c_int {
    (*runlevel).last = coeff_last15(dct);
    let mut i_last: libc::c_int = (*runlevel).last;
    let mut i_total: libc::c_int = 0 as libc::c_int;
    let mut mask: libc::c_int = 0 as libc::c_int;
    loop {
        let fresh5 = i_total;
        i_total += 1;
        (*runlevel).level[fresh5 as usize] = *dct.offset(i_last as isize);
        mask |= (1 as libc::c_int) << i_last;
        loop {
            i_last -= 1;
            if !(i_last >= 0 as libc::c_int
                && *dct.offset(i_last as isize) as libc::c_int == 0 as libc::c_int)
            {
                break;
            }
        }
        if i_last < 0 as libc::c_int {
            break;
        }
    }
    (*runlevel).mask = mask;
    i_total
}
unsafe extern "C" fn coeff_level_run16(
    mut dct: *mut dctcoef,
    mut runlevel: *mut x264_run_level_t,
) -> libc::c_int {
    (*runlevel).last = coeff_last16(dct);
    let mut i_last: libc::c_int = (*runlevel).last;
    let mut i_total: libc::c_int = 0 as libc::c_int;
    let mut mask: libc::c_int = 0 as libc::c_int;
    loop {
        let fresh6 = i_total;
        i_total += 1;
        (*runlevel).level[fresh6 as usize] = *dct.offset(i_last as isize);
        mask |= (1 as libc::c_int) << i_last;
        loop {
            i_last -= 1;
            if !(i_last >= 0 as libc::c_int
                && *dct.offset(i_last as isize) as libc::c_int == 0 as libc::c_int)
            {
                break;
            }
        }
        if i_last < 0 as libc::c_int {
            break;
        }
    }
    (*runlevel).mask = mask;
    i_total
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_quant_init(
    mut h: *mut x264_t,
    mut cpu: uint32_t,
    mut pf: *mut x264_quant_function_t,
) {
    (*pf).quant_8x8 = Some(
        quant_8x8
            as unsafe extern "C" fn(*mut dctcoef, *mut udctcoef, *mut udctcoef) -> libc::c_int,
    );
    (*pf).quant_4x4 = Some(
        quant_4x4
            as unsafe extern "C" fn(*mut dctcoef, *mut udctcoef, *mut udctcoef) -> libc::c_int,
    );
    (*pf).quant_4x4x4 = Some(
        quant_4x4x4
            as unsafe extern "C" fn(
                *mut [dctcoef; 16],
                *mut udctcoef,
                *mut udctcoef,
            ) -> libc::c_int,
    );
    (*pf).quant_4x4_dc = Some(
        quant_4x4_dc as unsafe extern "C" fn(*mut dctcoef, libc::c_int, libc::c_int) -> libc::c_int,
    );
    (*pf).quant_2x2_dc = Some(
        quant_2x2_dc as unsafe extern "C" fn(*mut dctcoef, libc::c_int, libc::c_int) -> libc::c_int,
    );
    (*pf).dequant_4x4 = Some(
        dequant_4x4
            as unsafe extern "C" fn(*mut dctcoef, *mut [libc::c_int; 16], libc::c_int) -> (),
    );
    (*pf).dequant_4x4_dc = Some(
        dequant_4x4_dc
            as unsafe extern "C" fn(*mut dctcoef, *mut [libc::c_int; 16], libc::c_int) -> (),
    );
    (*pf).dequant_8x8 = Some(
        dequant_8x8
            as unsafe extern "C" fn(*mut dctcoef, *mut [libc::c_int; 64], libc::c_int) -> (),
    );
    (*pf).idct_dequant_2x4_dc = Some(
        idct_dequant_2x4_dc
            as unsafe extern "C" fn(
                *mut dctcoef,
                *mut [dctcoef; 16],
                *mut [libc::c_int; 16],
                libc::c_int,
            ) -> (),
    );
    (*pf).idct_dequant_2x4_dconly = Some(
        idct_dequant_2x4_dconly
            as unsafe extern "C" fn(*mut dctcoef, *mut [libc::c_int; 16], libc::c_int) -> (),
    );
    (*pf).optimize_chroma_2x2_dc = Some(
        optimize_chroma_2x2_dc as unsafe extern "C" fn(*mut dctcoef, libc::c_int) -> libc::c_int,
    );
    (*pf).optimize_chroma_2x4_dc = Some(
        optimize_chroma_2x4_dc as unsafe extern "C" fn(*mut dctcoef, libc::c_int) -> libc::c_int,
    );
    (*pf).denoise_dct = Some(
        denoise_dct
            as unsafe extern "C" fn(*mut dctcoef, *mut uint32_t, *mut udctcoef, libc::c_int) -> (),
    );
    (*pf).decimate_score15 =
        Some(decimate_score15 as unsafe extern "C" fn(*mut dctcoef) -> libc::c_int);
    (*pf).decimate_score16 =
        Some(decimate_score16 as unsafe extern "C" fn(*mut dctcoef) -> libc::c_int);
    (*pf).decimate_score64 =
        Some(decimate_score64 as unsafe extern "C" fn(*mut dctcoef) -> libc::c_int);
    (*pf).coeff_last4 = Some(coeff_last4 as unsafe extern "C" fn(*mut dctcoef) -> libc::c_int);
    (*pf).coeff_last8 = Some(coeff_last8 as unsafe extern "C" fn(*mut dctcoef) -> libc::c_int);
    (*pf).coeff_last[DCT_LUMA_AC as libc::c_int as usize] =
        Some(coeff_last15 as unsafe extern "C" fn(*mut dctcoef) -> libc::c_int);
    (*pf).coeff_last[DCT_LUMA_4x4 as libc::c_int as usize] =
        Some(coeff_last16 as unsafe extern "C" fn(*mut dctcoef) -> libc::c_int);
    (*pf).coeff_last[DCT_LUMA_8x8 as libc::c_int as usize] =
        Some(coeff_last64 as unsafe extern "C" fn(*mut dctcoef) -> libc::c_int);
    (*pf).coeff_level_run4 = Some(
        coeff_level_run4
            as unsafe extern "C" fn(*mut dctcoef, *mut x264_run_level_t) -> libc::c_int,
    );
    (*pf).coeff_level_run8 = Some(
        coeff_level_run8
            as unsafe extern "C" fn(*mut dctcoef, *mut x264_run_level_t) -> libc::c_int,
    );
    (*pf).coeff_level_run[DCT_LUMA_AC as libc::c_int as usize] = Some(
        coeff_level_run15
            as unsafe extern "C" fn(*mut dctcoef, *mut x264_run_level_t) -> libc::c_int,
    );
    (*pf).coeff_level_run[DCT_LUMA_4x4 as libc::c_int as usize] = Some(
        coeff_level_run16
            as unsafe extern "C" fn(*mut dctcoef, *mut x264_run_level_t) -> libc::c_int,
    );
    (*pf).coeff_last[DCT_CHROMAV_4x4 as libc::c_int as usize] =
        (*pf).coeff_last[DCT_LUMA_4x4 as libc::c_int as usize];
    (*pf).coeff_last[DCT_CHROMAU_4x4 as libc::c_int as usize] =
        (*pf).coeff_last[DCT_CHROMAV_4x4 as libc::c_int as usize];
    (*pf).coeff_last[DCT_CHROMAV_DC as libc::c_int as usize] =
        (*pf).coeff_last[DCT_CHROMAU_4x4 as libc::c_int as usize];
    (*pf).coeff_last[DCT_CHROMAU_DC as libc::c_int as usize] =
        (*pf).coeff_last[DCT_CHROMAV_DC as libc::c_int as usize];
    (*pf).coeff_last[DCT_LUMA_DC as libc::c_int as usize] =
        (*pf).coeff_last[DCT_CHROMAU_DC as libc::c_int as usize];
    (*pf).coeff_last[DCT_CHROMAV_AC as libc::c_int as usize] =
        (*pf).coeff_last[DCT_LUMA_AC as libc::c_int as usize];
    (*pf).coeff_last[DCT_CHROMAU_AC as libc::c_int as usize] =
        (*pf).coeff_last[DCT_CHROMAV_AC as libc::c_int as usize];
    (*pf).coeff_last[DCT_CHROMA_AC as libc::c_int as usize] =
        (*pf).coeff_last[DCT_CHROMAU_AC as libc::c_int as usize];
    (*pf).coeff_last[DCT_CHROMAV_8x8 as libc::c_int as usize] =
        (*pf).coeff_last[DCT_LUMA_8x8 as libc::c_int as usize];
    (*pf).coeff_last[DCT_CHROMAU_8x8 as libc::c_int as usize] =
        (*pf).coeff_last[DCT_CHROMAV_8x8 as libc::c_int as usize];
    (*pf).coeff_level_run[DCT_CHROMAV_4x4 as libc::c_int as usize] =
        (*pf).coeff_level_run[DCT_LUMA_4x4 as libc::c_int as usize];
    (*pf).coeff_level_run[DCT_CHROMAU_4x4 as libc::c_int as usize] =
        (*pf).coeff_level_run[DCT_CHROMAV_4x4 as libc::c_int as usize];
    (*pf).coeff_level_run[DCT_CHROMAV_DC as libc::c_int as usize] =
        (*pf).coeff_level_run[DCT_CHROMAU_4x4 as libc::c_int as usize];
    (*pf).coeff_level_run[DCT_CHROMAU_DC as libc::c_int as usize] =
        (*pf).coeff_level_run[DCT_CHROMAV_DC as libc::c_int as usize];
    (*pf).coeff_level_run[DCT_LUMA_DC as libc::c_int as usize] =
        (*pf).coeff_level_run[DCT_CHROMAU_DC as libc::c_int as usize];
    (*pf).coeff_level_run[DCT_CHROMAV_AC as libc::c_int as usize] =
        (*pf).coeff_level_run[DCT_LUMA_AC as libc::c_int as usize];
    (*pf).coeff_level_run[DCT_CHROMAU_AC as libc::c_int as usize] =
        (*pf).coeff_level_run[DCT_CHROMAV_AC as libc::c_int as usize];
    (*pf).coeff_level_run[DCT_CHROMA_AC as libc::c_int as usize] =
        (*pf).coeff_level_run[DCT_CHROMAU_AC as libc::c_int as usize];
}
