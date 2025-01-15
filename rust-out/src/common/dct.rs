use crate::types::*;
extern "C" {
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
}
#[inline(always)]
unsafe extern "C" fn x264_clip_pixel(mut x: libc::c_int) -> pixel {
    (if x & !(((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int) != 0 {
        (-x >> 31 as libc::c_int) & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
    } else {
        x
    }) as pixel
}
unsafe extern "C" fn dct4x4dc(mut d: *mut dctcoef) {
    let mut tmp: [dctcoef; 16] = [0; 16];
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        let mut s01: libc::c_int = *d.offset((i * 4 as libc::c_int + 0 as libc::c_int) as isize)
            as libc::c_int
            + *d.offset((i * 4 as libc::c_int + 1 as libc::c_int) as isize) as libc::c_int;
        let mut d01: libc::c_int = *d.offset((i * 4 as libc::c_int + 0 as libc::c_int) as isize)
            as libc::c_int
            - *d.offset((i * 4 as libc::c_int + 1 as libc::c_int) as isize) as libc::c_int;
        let mut s23: libc::c_int = *d.offset((i * 4 as libc::c_int + 2 as libc::c_int) as isize)
            as libc::c_int
            + *d.offset((i * 4 as libc::c_int + 3 as libc::c_int) as isize) as libc::c_int;
        let mut d23: libc::c_int = *d.offset((i * 4 as libc::c_int + 2 as libc::c_int) as isize)
            as libc::c_int
            - *d.offset((i * 4 as libc::c_int + 3 as libc::c_int) as isize) as libc::c_int;
        tmp[(0 as libc::c_int * 4 as libc::c_int + i) as usize] = (s01 + s23) as dctcoef;
        tmp[(1 as libc::c_int * 4 as libc::c_int + i) as usize] = (s01 - s23) as dctcoef;
        tmp[(2 as libc::c_int * 4 as libc::c_int + i) as usize] = (d01 - d23) as dctcoef;
        tmp[(3 as libc::c_int * 4 as libc::c_int + i) as usize] = (d01 + d23) as dctcoef;
        i += 1;
        i;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < 4 as libc::c_int {
        let mut s01_0: libc::c_int = tmp[(i_0 * 4 as libc::c_int + 0 as libc::c_int) as usize]
            as libc::c_int
            + tmp[(i_0 * 4 as libc::c_int + 1 as libc::c_int) as usize] as libc::c_int;
        let mut d01_0: libc::c_int = tmp[(i_0 * 4 as libc::c_int + 0 as libc::c_int) as usize]
            as libc::c_int
            - tmp[(i_0 * 4 as libc::c_int + 1 as libc::c_int) as usize] as libc::c_int;
        let mut s23_0: libc::c_int = tmp[(i_0 * 4 as libc::c_int + 2 as libc::c_int) as usize]
            as libc::c_int
            + tmp[(i_0 * 4 as libc::c_int + 3 as libc::c_int) as usize] as libc::c_int;
        let mut d23_0: libc::c_int = tmp[(i_0 * 4 as libc::c_int + 2 as libc::c_int) as usize]
            as libc::c_int
            - tmp[(i_0 * 4 as libc::c_int + 3 as libc::c_int) as usize] as libc::c_int;
        *d.offset((i_0 * 4 as libc::c_int + 0 as libc::c_int) as isize) =
            ((s01_0 + s23_0 + 1 as libc::c_int) >> 1 as libc::c_int) as dctcoef;
        *d.offset((i_0 * 4 as libc::c_int + 1 as libc::c_int) as isize) =
            ((s01_0 - s23_0 + 1 as libc::c_int) >> 1 as libc::c_int) as dctcoef;
        *d.offset((i_0 * 4 as libc::c_int + 2 as libc::c_int) as isize) =
            ((d01_0 - d23_0 + 1 as libc::c_int) >> 1 as libc::c_int) as dctcoef;
        *d.offset((i_0 * 4 as libc::c_int + 3 as libc::c_int) as isize) =
            ((d01_0 + d23_0 + 1 as libc::c_int) >> 1 as libc::c_int) as dctcoef;
        i_0 += 1;
        i_0;
    }
}
unsafe extern "C" fn idct4x4dc(mut d: *mut dctcoef) {
    let mut tmp: [dctcoef; 16] = [0; 16];
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        let mut s01: libc::c_int = *d.offset((i * 4 as libc::c_int + 0 as libc::c_int) as isize)
            as libc::c_int
            + *d.offset((i * 4 as libc::c_int + 1 as libc::c_int) as isize) as libc::c_int;
        let mut d01: libc::c_int = *d.offset((i * 4 as libc::c_int + 0 as libc::c_int) as isize)
            as libc::c_int
            - *d.offset((i * 4 as libc::c_int + 1 as libc::c_int) as isize) as libc::c_int;
        let mut s23: libc::c_int = *d.offset((i * 4 as libc::c_int + 2 as libc::c_int) as isize)
            as libc::c_int
            + *d.offset((i * 4 as libc::c_int + 3 as libc::c_int) as isize) as libc::c_int;
        let mut d23: libc::c_int = *d.offset((i * 4 as libc::c_int + 2 as libc::c_int) as isize)
            as libc::c_int
            - *d.offset((i * 4 as libc::c_int + 3 as libc::c_int) as isize) as libc::c_int;
        tmp[(0 as libc::c_int * 4 as libc::c_int + i) as usize] = (s01 + s23) as dctcoef;
        tmp[(1 as libc::c_int * 4 as libc::c_int + i) as usize] = (s01 - s23) as dctcoef;
        tmp[(2 as libc::c_int * 4 as libc::c_int + i) as usize] = (d01 - d23) as dctcoef;
        tmp[(3 as libc::c_int * 4 as libc::c_int + i) as usize] = (d01 + d23) as dctcoef;
        i += 1;
        i;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < 4 as libc::c_int {
        let mut s01_0: libc::c_int = tmp[(i_0 * 4 as libc::c_int + 0 as libc::c_int) as usize]
            as libc::c_int
            + tmp[(i_0 * 4 as libc::c_int + 1 as libc::c_int) as usize] as libc::c_int;
        let mut d01_0: libc::c_int = tmp[(i_0 * 4 as libc::c_int + 0 as libc::c_int) as usize]
            as libc::c_int
            - tmp[(i_0 * 4 as libc::c_int + 1 as libc::c_int) as usize] as libc::c_int;
        let mut s23_0: libc::c_int = tmp[(i_0 * 4 as libc::c_int + 2 as libc::c_int) as usize]
            as libc::c_int
            + tmp[(i_0 * 4 as libc::c_int + 3 as libc::c_int) as usize] as libc::c_int;
        let mut d23_0: libc::c_int = tmp[(i_0 * 4 as libc::c_int + 2 as libc::c_int) as usize]
            as libc::c_int
            - tmp[(i_0 * 4 as libc::c_int + 3 as libc::c_int) as usize] as libc::c_int;
        *d.offset((i_0 * 4 as libc::c_int + 0 as libc::c_int) as isize) =
            (s01_0 + s23_0) as dctcoef;
        *d.offset((i_0 * 4 as libc::c_int + 1 as libc::c_int) as isize) =
            (s01_0 - s23_0) as dctcoef;
        *d.offset((i_0 * 4 as libc::c_int + 2 as libc::c_int) as isize) =
            (d01_0 - d23_0) as dctcoef;
        *d.offset((i_0 * 4 as libc::c_int + 3 as libc::c_int) as isize) =
            (d01_0 + d23_0) as dctcoef;
        i_0 += 1;
        i_0;
    }
}
unsafe extern "C" fn dct2x4dc(mut dct: *mut dctcoef, mut dct4x4: *mut [dctcoef; 16]) {
    let mut a0: libc::c_int = (*dct4x4.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
        as libc::c_int
        + (*dct4x4.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] as libc::c_int;
    let mut a1: libc::c_int = (*dct4x4.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize]
        as libc::c_int
        + (*dct4x4.offset(3 as libc::c_int as isize))[0 as libc::c_int as usize] as libc::c_int;
    let mut a2: libc::c_int = (*dct4x4.offset(4 as libc::c_int as isize))[0 as libc::c_int as usize]
        as libc::c_int
        + (*dct4x4.offset(5 as libc::c_int as isize))[0 as libc::c_int as usize] as libc::c_int;
    let mut a3: libc::c_int = (*dct4x4.offset(6 as libc::c_int as isize))[0 as libc::c_int as usize]
        as libc::c_int
        + (*dct4x4.offset(7 as libc::c_int as isize))[0 as libc::c_int as usize] as libc::c_int;
    let mut a4: libc::c_int = (*dct4x4.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
        as libc::c_int
        - (*dct4x4.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] as libc::c_int;
    let mut a5: libc::c_int = (*dct4x4.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize]
        as libc::c_int
        - (*dct4x4.offset(3 as libc::c_int as isize))[0 as libc::c_int as usize] as libc::c_int;
    let mut a6: libc::c_int = (*dct4x4.offset(4 as libc::c_int as isize))[0 as libc::c_int as usize]
        as libc::c_int
        - (*dct4x4.offset(5 as libc::c_int as isize))[0 as libc::c_int as usize] as libc::c_int;
    let mut a7: libc::c_int = (*dct4x4.offset(6 as libc::c_int as isize))[0 as libc::c_int as usize]
        as libc::c_int
        - (*dct4x4.offset(7 as libc::c_int as isize))[0 as libc::c_int as usize] as libc::c_int;
    let mut b0: libc::c_int = a0 + a1;
    let mut b1: libc::c_int = a2 + a3;
    let mut b2: libc::c_int = a4 + a5;
    let mut b3: libc::c_int = a6 + a7;
    let mut b4: libc::c_int = a0 - a1;
    let mut b5: libc::c_int = a2 - a3;
    let mut b6: libc::c_int = a4 - a5;
    let mut b7: libc::c_int = a6 - a7;
    *dct.offset(0 as libc::c_int as isize) = (b0 + b1) as dctcoef;
    *dct.offset(1 as libc::c_int as isize) = (b2 + b3) as dctcoef;
    *dct.offset(2 as libc::c_int as isize) = (b0 - b1) as dctcoef;
    *dct.offset(3 as libc::c_int as isize) = (b2 - b3) as dctcoef;
    *dct.offset(4 as libc::c_int as isize) = (b4 - b5) as dctcoef;
    *dct.offset(5 as libc::c_int as isize) = (b6 - b7) as dctcoef;
    *dct.offset(6 as libc::c_int as isize) = (b4 + b5) as dctcoef;
    *dct.offset(7 as libc::c_int as isize) = (b6 + b7) as dctcoef;
    (*dct4x4.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] =
        0 as libc::c_int as dctcoef;
    (*dct4x4.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] =
        0 as libc::c_int as dctcoef;
    (*dct4x4.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize] =
        0 as libc::c_int as dctcoef;
    (*dct4x4.offset(3 as libc::c_int as isize))[0 as libc::c_int as usize] =
        0 as libc::c_int as dctcoef;
    (*dct4x4.offset(4 as libc::c_int as isize))[0 as libc::c_int as usize] =
        0 as libc::c_int as dctcoef;
    (*dct4x4.offset(5 as libc::c_int as isize))[0 as libc::c_int as usize] =
        0 as libc::c_int as dctcoef;
    (*dct4x4.offset(6 as libc::c_int as isize))[0 as libc::c_int as usize] =
        0 as libc::c_int as dctcoef;
    (*dct4x4.offset(7 as libc::c_int as isize))[0 as libc::c_int as usize] =
        0 as libc::c_int as dctcoef;
}
#[inline]
unsafe extern "C" fn pixel_sub_wxh(
    mut diff: *mut dctcoef,
    mut i_size: libc::c_int,
    mut pix1: *mut pixel,
    mut i_pix1: libc::c_int,
    mut pix2: *mut pixel,
    mut i_pix2: libc::c_int,
) {
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < i_size {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < i_size {
            *diff.offset((x + y * i_size) as isize) = (*pix1.offset(x as isize) as libc::c_int
                - *pix2.offset(x as isize) as libc::c_int)
                as dctcoef;
            x += 1;
            x;
        }
        pix1 = pix1.offset(i_pix1 as isize);
        pix2 = pix2.offset(i_pix2 as isize);
        y += 1;
        y;
    }
}
unsafe extern "C" fn sub4x4_dct(mut dct: *mut dctcoef, mut pix1: *mut pixel, mut pix2: *mut pixel) {
    let mut d: [dctcoef; 16] = [0; 16];
    let mut tmp: [dctcoef; 16] = [0; 16];
    pixel_sub_wxh(
        d.as_mut_ptr(),
        4 as libc::c_int,
        pix1,
        16 as libc::c_int,
        pix2,
        32 as libc::c_int,
    );
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        let mut s03: libc::c_int = d[(i * 4 as libc::c_int + 0 as libc::c_int) as usize]
            as libc::c_int
            + d[(i * 4 as libc::c_int + 3 as libc::c_int) as usize] as libc::c_int;
        let mut s12: libc::c_int = d[(i * 4 as libc::c_int + 1 as libc::c_int) as usize]
            as libc::c_int
            + d[(i * 4 as libc::c_int + 2 as libc::c_int) as usize] as libc::c_int;
        let mut d03: libc::c_int = d[(i * 4 as libc::c_int + 0 as libc::c_int) as usize]
            as libc::c_int
            - d[(i * 4 as libc::c_int + 3 as libc::c_int) as usize] as libc::c_int;
        let mut d12: libc::c_int = d[(i * 4 as libc::c_int + 1 as libc::c_int) as usize]
            as libc::c_int
            - d[(i * 4 as libc::c_int + 2 as libc::c_int) as usize] as libc::c_int;
        tmp[(0 as libc::c_int * 4 as libc::c_int + i) as usize] = (s03 + s12) as dctcoef;
        tmp[(1 as libc::c_int * 4 as libc::c_int + i) as usize] =
            (2 as libc::c_int * d03 + d12) as dctcoef;
        tmp[(2 as libc::c_int * 4 as libc::c_int + i) as usize] = (s03 - s12) as dctcoef;
        tmp[(3 as libc::c_int * 4 as libc::c_int + i) as usize] =
            (d03 - 2 as libc::c_int * d12) as dctcoef;
        i += 1;
        i;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < 4 as libc::c_int {
        let mut s03_0: libc::c_int = tmp[(i_0 * 4 as libc::c_int + 0 as libc::c_int) as usize]
            as libc::c_int
            + tmp[(i_0 * 4 as libc::c_int + 3 as libc::c_int) as usize] as libc::c_int;
        let mut s12_0: libc::c_int = tmp[(i_0 * 4 as libc::c_int + 1 as libc::c_int) as usize]
            as libc::c_int
            + tmp[(i_0 * 4 as libc::c_int + 2 as libc::c_int) as usize] as libc::c_int;
        let mut d03_0: libc::c_int = tmp[(i_0 * 4 as libc::c_int + 0 as libc::c_int) as usize]
            as libc::c_int
            - tmp[(i_0 * 4 as libc::c_int + 3 as libc::c_int) as usize] as libc::c_int;
        let mut d12_0: libc::c_int = tmp[(i_0 * 4 as libc::c_int + 1 as libc::c_int) as usize]
            as libc::c_int
            - tmp[(i_0 * 4 as libc::c_int + 2 as libc::c_int) as usize] as libc::c_int;
        *dct.offset((i_0 * 4 as libc::c_int + 0 as libc::c_int) as isize) =
            (s03_0 + s12_0) as dctcoef;
        *dct.offset((i_0 * 4 as libc::c_int + 1 as libc::c_int) as isize) =
            (2 as libc::c_int * d03_0 + d12_0) as dctcoef;
        *dct.offset((i_0 * 4 as libc::c_int + 2 as libc::c_int) as isize) =
            (s03_0 - s12_0) as dctcoef;
        *dct.offset((i_0 * 4 as libc::c_int + 3 as libc::c_int) as isize) =
            (d03_0 - 2 as libc::c_int * d12_0) as dctcoef;
        i_0 += 1;
        i_0;
    }
}
unsafe extern "C" fn sub8x8_dct(
    mut dct: *mut [dctcoef; 16],
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
) {
    sub4x4_dct(
        (*dct.offset(0 as libc::c_int as isize)).as_mut_ptr(),
        &mut *pix1.offset(0 as libc::c_int as isize),
        &mut *pix2.offset(0 as libc::c_int as isize),
    );
    sub4x4_dct(
        (*dct.offset(1 as libc::c_int as isize)).as_mut_ptr(),
        &mut *pix1.offset(4 as libc::c_int as isize),
        &mut *pix2.offset(4 as libc::c_int as isize),
    );
    sub4x4_dct(
        (*dct.offset(2 as libc::c_int as isize)).as_mut_ptr(),
        &mut *pix1.offset((4 as libc::c_int * 16 as libc::c_int + 0 as libc::c_int) as isize),
        &mut *pix2.offset((4 as libc::c_int * 32 as libc::c_int + 0 as libc::c_int) as isize),
    );
    sub4x4_dct(
        (*dct.offset(3 as libc::c_int as isize)).as_mut_ptr(),
        &mut *pix1.offset((4 as libc::c_int * 16 as libc::c_int + 4 as libc::c_int) as isize),
        &mut *pix2.offset((4 as libc::c_int * 32 as libc::c_int + 4 as libc::c_int) as isize),
    );
}
unsafe extern "C" fn sub16x16_dct(
    mut dct: *mut [dctcoef; 16],
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
) {
    sub8x8_dct(
        &mut *dct.offset(0 as libc::c_int as isize),
        &mut *pix1.offset(0 as libc::c_int as isize),
        &mut *pix2.offset(0 as libc::c_int as isize),
    );
    sub8x8_dct(
        &mut *dct.offset(4 as libc::c_int as isize),
        &mut *pix1.offset(8 as libc::c_int as isize),
        &mut *pix2.offset(8 as libc::c_int as isize),
    );
    sub8x8_dct(
        &mut *dct.offset(8 as libc::c_int as isize),
        &mut *pix1.offset((8 as libc::c_int * 16 as libc::c_int + 0 as libc::c_int) as isize),
        &mut *pix2.offset((8 as libc::c_int * 32 as libc::c_int + 0 as libc::c_int) as isize),
    );
    sub8x8_dct(
        &mut *dct.offset(12 as libc::c_int as isize),
        &mut *pix1.offset((8 as libc::c_int * 16 as libc::c_int + 8 as libc::c_int) as isize),
        &mut *pix2.offset((8 as libc::c_int * 32 as libc::c_int + 8 as libc::c_int) as isize),
    );
}
unsafe extern "C" fn sub4x4_dct_dc(mut pix1: *mut pixel, mut pix2: *mut pixel) -> libc::c_int {
    let mut sum: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        sum += *pix1.offset(0 as libc::c_int as isize) as libc::c_int
            + *pix1.offset(1 as libc::c_int as isize) as libc::c_int
            + *pix1.offset(2 as libc::c_int as isize) as libc::c_int
            + *pix1.offset(3 as libc::c_int as isize) as libc::c_int
            - *pix2.offset(0 as libc::c_int as isize) as libc::c_int
            - *pix2.offset(1 as libc::c_int as isize) as libc::c_int
            - *pix2.offset(2 as libc::c_int as isize) as libc::c_int
            - *pix2.offset(3 as libc::c_int as isize) as libc::c_int;
        i += 1;
        i;
        pix1 = pix1.offset(16 as libc::c_int as isize);
        pix2 = pix2.offset(32 as libc::c_int as isize);
    }
    sum
}
unsafe extern "C" fn sub8x8_dct_dc(
    mut dct: *mut dctcoef,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
) {
    *dct.offset(0 as libc::c_int as isize) = sub4x4_dct_dc(
        &mut *pix1.offset(0 as libc::c_int as isize),
        &mut *pix2.offset(0 as libc::c_int as isize),
    ) as dctcoef;
    *dct.offset(1 as libc::c_int as isize) = sub4x4_dct_dc(
        &mut *pix1.offset(4 as libc::c_int as isize),
        &mut *pix2.offset(4 as libc::c_int as isize),
    ) as dctcoef;
    *dct.offset(2 as libc::c_int as isize) = sub4x4_dct_dc(
        &mut *pix1.offset((4 as libc::c_int * 16 as libc::c_int + 0 as libc::c_int) as isize),
        &mut *pix2.offset((4 as libc::c_int * 32 as libc::c_int + 0 as libc::c_int) as isize),
    ) as dctcoef;
    *dct.offset(3 as libc::c_int as isize) = sub4x4_dct_dc(
        &mut *pix1.offset((4 as libc::c_int * 16 as libc::c_int + 4 as libc::c_int) as isize),
        &mut *pix2.offset((4 as libc::c_int * 32 as libc::c_int + 4 as libc::c_int) as isize),
    ) as dctcoef;
    let mut d0: libc::c_int = *dct.offset(0 as libc::c_int as isize) as libc::c_int
        + *dct.offset(1 as libc::c_int as isize) as libc::c_int;
    let mut d1: libc::c_int = *dct.offset(2 as libc::c_int as isize) as libc::c_int
        + *dct.offset(3 as libc::c_int as isize) as libc::c_int;
    let mut d2: libc::c_int = *dct.offset(0 as libc::c_int as isize) as libc::c_int
        - *dct.offset(1 as libc::c_int as isize) as libc::c_int;
    let mut d3: libc::c_int = *dct.offset(2 as libc::c_int as isize) as libc::c_int
        - *dct.offset(3 as libc::c_int as isize) as libc::c_int;
    *dct.offset(0 as libc::c_int as isize) = (d0 + d1) as dctcoef;
    *dct.offset(1 as libc::c_int as isize) = (d0 - d1) as dctcoef;
    *dct.offset(2 as libc::c_int as isize) = (d2 + d3) as dctcoef;
    *dct.offset(3 as libc::c_int as isize) = (d2 - d3) as dctcoef;
}
unsafe extern "C" fn sub8x16_dct_dc(
    mut dct: *mut dctcoef,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
) {
    let mut a0: libc::c_int = sub4x4_dct_dc(
        &mut *pix1.offset((0 as libc::c_int * 16 as libc::c_int + 0 as libc::c_int) as isize),
        &mut *pix2.offset((0 as libc::c_int * 32 as libc::c_int + 0 as libc::c_int) as isize),
    );
    let mut a1: libc::c_int = sub4x4_dct_dc(
        &mut *pix1.offset((0 as libc::c_int * 16 as libc::c_int + 4 as libc::c_int) as isize),
        &mut *pix2.offset((0 as libc::c_int * 32 as libc::c_int + 4 as libc::c_int) as isize),
    );
    let mut a2: libc::c_int = sub4x4_dct_dc(
        &mut *pix1.offset((4 as libc::c_int * 16 as libc::c_int + 0 as libc::c_int) as isize),
        &mut *pix2.offset((4 as libc::c_int * 32 as libc::c_int + 0 as libc::c_int) as isize),
    );
    let mut a3: libc::c_int = sub4x4_dct_dc(
        &mut *pix1.offset((4 as libc::c_int * 16 as libc::c_int + 4 as libc::c_int) as isize),
        &mut *pix2.offset((4 as libc::c_int * 32 as libc::c_int + 4 as libc::c_int) as isize),
    );
    let mut a4: libc::c_int = sub4x4_dct_dc(
        &mut *pix1.offset((8 as libc::c_int * 16 as libc::c_int + 0 as libc::c_int) as isize),
        &mut *pix2.offset((8 as libc::c_int * 32 as libc::c_int + 0 as libc::c_int) as isize),
    );
    let mut a5: libc::c_int = sub4x4_dct_dc(
        &mut *pix1.offset((8 as libc::c_int * 16 as libc::c_int + 4 as libc::c_int) as isize),
        &mut *pix2.offset((8 as libc::c_int * 32 as libc::c_int + 4 as libc::c_int) as isize),
    );
    let mut a6: libc::c_int = sub4x4_dct_dc(
        &mut *pix1.offset((12 as libc::c_int * 16 as libc::c_int + 0 as libc::c_int) as isize),
        &mut *pix2.offset((12 as libc::c_int * 32 as libc::c_int + 0 as libc::c_int) as isize),
    );
    let mut a7: libc::c_int = sub4x4_dct_dc(
        &mut *pix1.offset((12 as libc::c_int * 16 as libc::c_int + 4 as libc::c_int) as isize),
        &mut *pix2.offset((12 as libc::c_int * 32 as libc::c_int + 4 as libc::c_int) as isize),
    );
    let mut b0: libc::c_int = a0 + a1;
    let mut b1: libc::c_int = a2 + a3;
    let mut b2: libc::c_int = a4 + a5;
    let mut b3: libc::c_int = a6 + a7;
    let mut b4: libc::c_int = a0 - a1;
    let mut b5: libc::c_int = a2 - a3;
    let mut b6: libc::c_int = a4 - a5;
    let mut b7: libc::c_int = a6 - a7;
    a0 = b0 + b1;
    a1 = b2 + b3;
    a2 = b4 + b5;
    a3 = b6 + b7;
    a4 = b0 - b1;
    a5 = b2 - b3;
    a6 = b4 - b5;
    a7 = b6 - b7;
    *dct.offset(0 as libc::c_int as isize) = (a0 + a1) as dctcoef;
    *dct.offset(1 as libc::c_int as isize) = (a2 + a3) as dctcoef;
    *dct.offset(2 as libc::c_int as isize) = (a0 - a1) as dctcoef;
    *dct.offset(3 as libc::c_int as isize) = (a2 - a3) as dctcoef;
    *dct.offset(4 as libc::c_int as isize) = (a4 - a5) as dctcoef;
    *dct.offset(5 as libc::c_int as isize) = (a6 - a7) as dctcoef;
    *dct.offset(6 as libc::c_int as isize) = (a4 + a5) as dctcoef;
    *dct.offset(7 as libc::c_int as isize) = (a6 + a7) as dctcoef;
}
unsafe extern "C" fn add4x4_idct(mut p_dst: *mut pixel, mut dct: *mut dctcoef) {
    let mut d: [dctcoef; 16] = [0; 16];
    let mut tmp: [dctcoef; 16] = [0; 16];
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        let mut s02: libc::c_int = *dct.offset((0 as libc::c_int * 4 as libc::c_int + i) as isize)
            as libc::c_int
            + *dct.offset((2 as libc::c_int * 4 as libc::c_int + i) as isize) as libc::c_int;
        let mut d02: libc::c_int = *dct.offset((0 as libc::c_int * 4 as libc::c_int + i) as isize)
            as libc::c_int
            - *dct.offset((2 as libc::c_int * 4 as libc::c_int + i) as isize) as libc::c_int;
        let mut s13: libc::c_int = *dct.offset((1 as libc::c_int * 4 as libc::c_int + i) as isize)
            as libc::c_int
            + (*dct.offset((3 as libc::c_int * 4 as libc::c_int + i) as isize) as libc::c_int
                >> 1 as libc::c_int);
        let mut d13: libc::c_int = (*dct.offset((1 as libc::c_int * 4 as libc::c_int + i) as isize)
            as libc::c_int
            >> 1 as libc::c_int)
            - *dct.offset((3 as libc::c_int * 4 as libc::c_int + i) as isize) as libc::c_int;
        tmp[(i * 4 as libc::c_int + 0 as libc::c_int) as usize] = (s02 + s13) as dctcoef;
        tmp[(i * 4 as libc::c_int + 1 as libc::c_int) as usize] = (d02 + d13) as dctcoef;
        tmp[(i * 4 as libc::c_int + 2 as libc::c_int) as usize] = (d02 - d13) as dctcoef;
        tmp[(i * 4 as libc::c_int + 3 as libc::c_int) as usize] = (s02 - s13) as dctcoef;
        i += 1;
        i;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < 4 as libc::c_int {
        let mut s02_0: libc::c_int = tmp[(0 as libc::c_int * 4 as libc::c_int + i_0) as usize]
            as libc::c_int
            + tmp[(2 as libc::c_int * 4 as libc::c_int + i_0) as usize] as libc::c_int;
        let mut d02_0: libc::c_int = tmp[(0 as libc::c_int * 4 as libc::c_int + i_0) as usize]
            as libc::c_int
            - tmp[(2 as libc::c_int * 4 as libc::c_int + i_0) as usize] as libc::c_int;
        let mut s13_0: libc::c_int = tmp[(1 as libc::c_int * 4 as libc::c_int + i_0) as usize]
            as libc::c_int
            + (tmp[(3 as libc::c_int * 4 as libc::c_int + i_0) as usize] as libc::c_int
                >> 1 as libc::c_int);
        let mut d13_0: libc::c_int = (tmp[(1 as libc::c_int * 4 as libc::c_int + i_0) as usize]
            as libc::c_int
            >> 1 as libc::c_int)
            - tmp[(3 as libc::c_int * 4 as libc::c_int + i_0) as usize] as libc::c_int;
        d[(0 as libc::c_int * 4 as libc::c_int + i_0) as usize] =
            ((s02_0 + s13_0 + 32 as libc::c_int) >> 6 as libc::c_int) as dctcoef;
        d[(1 as libc::c_int * 4 as libc::c_int + i_0) as usize] =
            ((d02_0 + d13_0 + 32 as libc::c_int) >> 6 as libc::c_int) as dctcoef;
        d[(2 as libc::c_int * 4 as libc::c_int + i_0) as usize] =
            ((d02_0 - d13_0 + 32 as libc::c_int) >> 6 as libc::c_int) as dctcoef;
        d[(3 as libc::c_int * 4 as libc::c_int + i_0) as usize] =
            ((s02_0 - s13_0 + 32 as libc::c_int) >> 6 as libc::c_int) as dctcoef;
        i_0 += 1;
        i_0;
    }
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < 4 as libc::c_int {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < 4 as libc::c_int {
            *p_dst.offset(x as isize) = x264_clip_pixel(
                *p_dst.offset(x as isize) as libc::c_int
                    + d[(y * 4 as libc::c_int + x) as usize] as libc::c_int,
            );
            x += 1;
            x;
        }
        p_dst = p_dst.offset(32 as libc::c_int as isize);
        y += 1;
        y;
    }
}
unsafe extern "C" fn add8x8_idct(mut p_dst: *mut pixel, mut dct: *mut [dctcoef; 16]) {
    add4x4_idct(
        &mut *p_dst.offset(0 as libc::c_int as isize),
        (*dct.offset(0 as libc::c_int as isize)).as_mut_ptr(),
    );
    add4x4_idct(
        &mut *p_dst.offset(4 as libc::c_int as isize),
        (*dct.offset(1 as libc::c_int as isize)).as_mut_ptr(),
    );
    add4x4_idct(
        &mut *p_dst.offset((4 as libc::c_int * 32 as libc::c_int + 0 as libc::c_int) as isize),
        (*dct.offset(2 as libc::c_int as isize)).as_mut_ptr(),
    );
    add4x4_idct(
        &mut *p_dst.offset((4 as libc::c_int * 32 as libc::c_int + 4 as libc::c_int) as isize),
        (*dct.offset(3 as libc::c_int as isize)).as_mut_ptr(),
    );
}
unsafe extern "C" fn add16x16_idct(mut p_dst: *mut pixel, mut dct: *mut [dctcoef; 16]) {
    add8x8_idct(
        &mut *p_dst.offset(0 as libc::c_int as isize),
        &mut *dct.offset(0 as libc::c_int as isize),
    );
    add8x8_idct(
        &mut *p_dst.offset(8 as libc::c_int as isize),
        &mut *dct.offset(4 as libc::c_int as isize),
    );
    add8x8_idct(
        &mut *p_dst.offset((8 as libc::c_int * 32 as libc::c_int + 0 as libc::c_int) as isize),
        &mut *dct.offset(8 as libc::c_int as isize),
    );
    add8x8_idct(
        &mut *p_dst.offset((8 as libc::c_int * 32 as libc::c_int + 8 as libc::c_int) as isize),
        &mut *dct.offset(12 as libc::c_int as isize),
    );
}
unsafe extern "C" fn sub8x8_dct8(
    mut dct: *mut dctcoef,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
) {
    let mut tmp: [dctcoef; 64] = [0; 64];
    pixel_sub_wxh(
        tmp.as_mut_ptr(),
        8 as libc::c_int,
        pix1,
        16 as libc::c_int,
        pix2,
        32 as libc::c_int,
    );
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        let mut s07: libc::c_int = tmp[(0 as libc::c_int * 8 as libc::c_int + i) as usize]
            as libc::c_int
            + tmp[(7 as libc::c_int * 8 as libc::c_int + i) as usize] as libc::c_int;
        let mut s16: libc::c_int = tmp[(1 as libc::c_int * 8 as libc::c_int + i) as usize]
            as libc::c_int
            + tmp[(6 as libc::c_int * 8 as libc::c_int + i) as usize] as libc::c_int;
        let mut s25: libc::c_int = tmp[(2 as libc::c_int * 8 as libc::c_int + i) as usize]
            as libc::c_int
            + tmp[(5 as libc::c_int * 8 as libc::c_int + i) as usize] as libc::c_int;
        let mut s34: libc::c_int = tmp[(3 as libc::c_int * 8 as libc::c_int + i) as usize]
            as libc::c_int
            + tmp[(4 as libc::c_int * 8 as libc::c_int + i) as usize] as libc::c_int;
        let mut a0: libc::c_int = s07 + s34;
        let mut a1: libc::c_int = s16 + s25;
        let mut a2: libc::c_int = s07 - s34;
        let mut a3: libc::c_int = s16 - s25;
        let mut d07: libc::c_int = tmp[(0 as libc::c_int * 8 as libc::c_int + i) as usize]
            as libc::c_int
            - tmp[(7 as libc::c_int * 8 as libc::c_int + i) as usize] as libc::c_int;
        let mut d16: libc::c_int = tmp[(1 as libc::c_int * 8 as libc::c_int + i) as usize]
            as libc::c_int
            - tmp[(6 as libc::c_int * 8 as libc::c_int + i) as usize] as libc::c_int;
        let mut d25: libc::c_int = tmp[(2 as libc::c_int * 8 as libc::c_int + i) as usize]
            as libc::c_int
            - tmp[(5 as libc::c_int * 8 as libc::c_int + i) as usize] as libc::c_int;
        let mut d34: libc::c_int = tmp[(3 as libc::c_int * 8 as libc::c_int + i) as usize]
            as libc::c_int
            - tmp[(4 as libc::c_int * 8 as libc::c_int + i) as usize] as libc::c_int;
        let mut a4: libc::c_int = d16 + d25 + (d07 + (d07 >> 1 as libc::c_int));
        let mut a5: libc::c_int = d07 - d34 - (d25 + (d25 >> 1 as libc::c_int));
        let mut a6: libc::c_int = d07 + d34 - (d16 + (d16 >> 1 as libc::c_int));
        let mut a7: libc::c_int = d16 - d25 + (d34 + (d34 >> 1 as libc::c_int));
        tmp[(0 as libc::c_int * 8 as libc::c_int + i) as usize] = (a0 + a1) as dctcoef;
        tmp[(1 as libc::c_int * 8 as libc::c_int + i) as usize] =
            (a4 + (a7 >> 2 as libc::c_int)) as dctcoef;
        tmp[(2 as libc::c_int * 8 as libc::c_int + i) as usize] =
            (a2 + (a3 >> 1 as libc::c_int)) as dctcoef;
        tmp[(3 as libc::c_int * 8 as libc::c_int + i) as usize] =
            (a5 + (a6 >> 2 as libc::c_int)) as dctcoef;
        tmp[(4 as libc::c_int * 8 as libc::c_int + i) as usize] = (a0 - a1) as dctcoef;
        tmp[(5 as libc::c_int * 8 as libc::c_int + i) as usize] =
            (a6 - (a5 >> 2 as libc::c_int)) as dctcoef;
        tmp[(6 as libc::c_int * 8 as libc::c_int + i) as usize] =
            ((a2 >> 1 as libc::c_int) - a3) as dctcoef;
        tmp[(7 as libc::c_int * 8 as libc::c_int + i) as usize] =
            ((a4 >> 2 as libc::c_int) - a7) as dctcoef;
        i += 1;
        i;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < 8 as libc::c_int {
        let mut s07_0: libc::c_int = tmp[(i_0 * 8 as libc::c_int + 0 as libc::c_int) as usize]
            as libc::c_int
            + tmp[(i_0 * 8 as libc::c_int + 7 as libc::c_int) as usize] as libc::c_int;
        let mut s16_0: libc::c_int = tmp[(i_0 * 8 as libc::c_int + 1 as libc::c_int) as usize]
            as libc::c_int
            + tmp[(i_0 * 8 as libc::c_int + 6 as libc::c_int) as usize] as libc::c_int;
        let mut s25_0: libc::c_int = tmp[(i_0 * 8 as libc::c_int + 2 as libc::c_int) as usize]
            as libc::c_int
            + tmp[(i_0 * 8 as libc::c_int + 5 as libc::c_int) as usize] as libc::c_int;
        let mut s34_0: libc::c_int = tmp[(i_0 * 8 as libc::c_int + 3 as libc::c_int) as usize]
            as libc::c_int
            + tmp[(i_0 * 8 as libc::c_int + 4 as libc::c_int) as usize] as libc::c_int;
        let mut a0_0: libc::c_int = s07_0 + s34_0;
        let mut a1_0: libc::c_int = s16_0 + s25_0;
        let mut a2_0: libc::c_int = s07_0 - s34_0;
        let mut a3_0: libc::c_int = s16_0 - s25_0;
        let mut d07_0: libc::c_int = tmp[(i_0 * 8 as libc::c_int + 0 as libc::c_int) as usize]
            as libc::c_int
            - tmp[(i_0 * 8 as libc::c_int + 7 as libc::c_int) as usize] as libc::c_int;
        let mut d16_0: libc::c_int = tmp[(i_0 * 8 as libc::c_int + 1 as libc::c_int) as usize]
            as libc::c_int
            - tmp[(i_0 * 8 as libc::c_int + 6 as libc::c_int) as usize] as libc::c_int;
        let mut d25_0: libc::c_int = tmp[(i_0 * 8 as libc::c_int + 2 as libc::c_int) as usize]
            as libc::c_int
            - tmp[(i_0 * 8 as libc::c_int + 5 as libc::c_int) as usize] as libc::c_int;
        let mut d34_0: libc::c_int = tmp[(i_0 * 8 as libc::c_int + 3 as libc::c_int) as usize]
            as libc::c_int
            - tmp[(i_0 * 8 as libc::c_int + 4 as libc::c_int) as usize] as libc::c_int;
        let mut a4_0: libc::c_int = d16_0 + d25_0 + (d07_0 + (d07_0 >> 1 as libc::c_int));
        let mut a5_0: libc::c_int = d07_0 - d34_0 - (d25_0 + (d25_0 >> 1 as libc::c_int));
        let mut a6_0: libc::c_int = d07_0 + d34_0 - (d16_0 + (d16_0 >> 1 as libc::c_int));
        let mut a7_0: libc::c_int = d16_0 - d25_0 + (d34_0 + (d34_0 >> 1 as libc::c_int));
        *dct.offset((0 as libc::c_int * 8 as libc::c_int + i_0) as isize) =
            (a0_0 + a1_0) as dctcoef;
        *dct.offset((1 as libc::c_int * 8 as libc::c_int + i_0) as isize) =
            (a4_0 + (a7_0 >> 2 as libc::c_int)) as dctcoef;
        *dct.offset((2 as libc::c_int * 8 as libc::c_int + i_0) as isize) =
            (a2_0 + (a3_0 >> 1 as libc::c_int)) as dctcoef;
        *dct.offset((3 as libc::c_int * 8 as libc::c_int + i_0) as isize) =
            (a5_0 + (a6_0 >> 2 as libc::c_int)) as dctcoef;
        *dct.offset((4 as libc::c_int * 8 as libc::c_int + i_0) as isize) =
            (a0_0 - a1_0) as dctcoef;
        *dct.offset((5 as libc::c_int * 8 as libc::c_int + i_0) as isize) =
            (a6_0 - (a5_0 >> 2 as libc::c_int)) as dctcoef;
        *dct.offset((6 as libc::c_int * 8 as libc::c_int + i_0) as isize) =
            ((a2_0 >> 1 as libc::c_int) - a3_0) as dctcoef;
        *dct.offset((7 as libc::c_int * 8 as libc::c_int + i_0) as isize) =
            ((a4_0 >> 2 as libc::c_int) - a7_0) as dctcoef;
        i_0 += 1;
        i_0;
    }
}
unsafe extern "C" fn sub16x16_dct8(
    mut dct: *mut [dctcoef; 64],
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
) {
    sub8x8_dct8(
        (*dct.offset(0 as libc::c_int as isize)).as_mut_ptr(),
        &mut *pix1.offset(0 as libc::c_int as isize),
        &mut *pix2.offset(0 as libc::c_int as isize),
    );
    sub8x8_dct8(
        (*dct.offset(1 as libc::c_int as isize)).as_mut_ptr(),
        &mut *pix1.offset(8 as libc::c_int as isize),
        &mut *pix2.offset(8 as libc::c_int as isize),
    );
    sub8x8_dct8(
        (*dct.offset(2 as libc::c_int as isize)).as_mut_ptr(),
        &mut *pix1.offset((8 as libc::c_int * 16 as libc::c_int + 0 as libc::c_int) as isize),
        &mut *pix2.offset((8 as libc::c_int * 32 as libc::c_int + 0 as libc::c_int) as isize),
    );
    sub8x8_dct8(
        (*dct.offset(3 as libc::c_int as isize)).as_mut_ptr(),
        &mut *pix1.offset((8 as libc::c_int * 16 as libc::c_int + 8 as libc::c_int) as isize),
        &mut *pix2.offset((8 as libc::c_int * 32 as libc::c_int + 8 as libc::c_int) as isize),
    );
}
unsafe extern "C" fn add8x8_idct8(mut dst: *mut pixel, mut dct: *mut dctcoef) {
    let fresh0 = &mut (*dct.offset(0 as libc::c_int as isize));
    *fresh0 = (*fresh0 as libc::c_int + 32 as libc::c_int) as dctcoef;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        let mut a0: libc::c_int = *dct.offset((0 as libc::c_int * 8 as libc::c_int + i) as isize)
            as libc::c_int
            + *dct.offset((4 as libc::c_int * 8 as libc::c_int + i) as isize) as libc::c_int;
        let mut a2: libc::c_int = *dct.offset((0 as libc::c_int * 8 as libc::c_int + i) as isize)
            as libc::c_int
            - *dct.offset((4 as libc::c_int * 8 as libc::c_int + i) as isize) as libc::c_int;
        let mut a4: libc::c_int = (*dct.offset((2 as libc::c_int * 8 as libc::c_int + i) as isize)
            as libc::c_int
            >> 1 as libc::c_int)
            - *dct.offset((6 as libc::c_int * 8 as libc::c_int + i) as isize) as libc::c_int;
        let mut a6: libc::c_int = (*dct.offset((6 as libc::c_int * 8 as libc::c_int + i) as isize)
            as libc::c_int
            >> 1 as libc::c_int)
            + *dct.offset((2 as libc::c_int * 8 as libc::c_int + i) as isize) as libc::c_int;
        let mut b0: libc::c_int = a0 + a6;
        let mut b2: libc::c_int = a2 + a4;
        let mut b4: libc::c_int = a2 - a4;
        let mut b6: libc::c_int = a0 - a6;
        let mut a1: libc::c_int = -(*dct.offset((3 as libc::c_int * 8 as libc::c_int + i) as isize)
            as libc::c_int)
            + *dct.offset((5 as libc::c_int * 8 as libc::c_int + i) as isize) as libc::c_int
            - *dct.offset((7 as libc::c_int * 8 as libc::c_int + i) as isize) as libc::c_int
            - (*dct.offset((7 as libc::c_int * 8 as libc::c_int + i) as isize) as libc::c_int
                >> 1 as libc::c_int);
        let mut a3: libc::c_int = *dct.offset((1 as libc::c_int * 8 as libc::c_int + i) as isize)
            as libc::c_int
            + *dct.offset((7 as libc::c_int * 8 as libc::c_int + i) as isize) as libc::c_int
            - *dct.offset((3 as libc::c_int * 8 as libc::c_int + i) as isize) as libc::c_int
            - (*dct.offset((3 as libc::c_int * 8 as libc::c_int + i) as isize) as libc::c_int
                >> 1 as libc::c_int);
        let mut a5: libc::c_int = -(*dct.offset((1 as libc::c_int * 8 as libc::c_int + i) as isize)
            as libc::c_int)
            + *dct.offset((7 as libc::c_int * 8 as libc::c_int + i) as isize) as libc::c_int
            + *dct.offset((5 as libc::c_int * 8 as libc::c_int + i) as isize) as libc::c_int
            + (*dct.offset((5 as libc::c_int * 8 as libc::c_int + i) as isize) as libc::c_int
                >> 1 as libc::c_int);
        let mut a7: libc::c_int = *dct.offset((3 as libc::c_int * 8 as libc::c_int + i) as isize)
            as libc::c_int
            + *dct.offset((5 as libc::c_int * 8 as libc::c_int + i) as isize) as libc::c_int
            + *dct.offset((1 as libc::c_int * 8 as libc::c_int + i) as isize) as libc::c_int
            + (*dct.offset((1 as libc::c_int * 8 as libc::c_int + i) as isize) as libc::c_int
                >> 1 as libc::c_int);
        let mut b1: libc::c_int = (a7 >> 2 as libc::c_int) + a1;
        let mut b3: libc::c_int = a3 + (a5 >> 2 as libc::c_int);
        let mut b5: libc::c_int = (a3 >> 2 as libc::c_int) - a5;
        let mut b7: libc::c_int = a7 - (a1 >> 2 as libc::c_int);
        *dct.offset((0 as libc::c_int * 8 as libc::c_int + i) as isize) = (b0 + b7) as dctcoef;
        *dct.offset((1 as libc::c_int * 8 as libc::c_int + i) as isize) = (b2 + b5) as dctcoef;
        *dct.offset((2 as libc::c_int * 8 as libc::c_int + i) as isize) = (b4 + b3) as dctcoef;
        *dct.offset((3 as libc::c_int * 8 as libc::c_int + i) as isize) = (b6 + b1) as dctcoef;
        *dct.offset((4 as libc::c_int * 8 as libc::c_int + i) as isize) = (b6 - b1) as dctcoef;
        *dct.offset((5 as libc::c_int * 8 as libc::c_int + i) as isize) = (b4 - b3) as dctcoef;
        *dct.offset((6 as libc::c_int * 8 as libc::c_int + i) as isize) = (b2 - b5) as dctcoef;
        *dct.offset((7 as libc::c_int * 8 as libc::c_int + i) as isize) = (b0 - b7) as dctcoef;
        i += 1;
        i;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < 8 as libc::c_int {
        let mut a0_0: libc::c_int =
            *dct.offset((i_0 * 8 as libc::c_int + 0 as libc::c_int) as isize) as libc::c_int
                + *dct.offset((i_0 * 8 as libc::c_int + 4 as libc::c_int) as isize) as libc::c_int;
        let mut a2_0: libc::c_int =
            *dct.offset((i_0 * 8 as libc::c_int + 0 as libc::c_int) as isize) as libc::c_int
                - *dct.offset((i_0 * 8 as libc::c_int + 4 as libc::c_int) as isize) as libc::c_int;
        let mut a4_0: libc::c_int =
            (*dct.offset((i_0 * 8 as libc::c_int + 2 as libc::c_int) as isize) as libc::c_int
                >> 1 as libc::c_int)
                - *dct.offset((i_0 * 8 as libc::c_int + 6 as libc::c_int) as isize) as libc::c_int;
        let mut a6_0: libc::c_int =
            (*dct.offset((i_0 * 8 as libc::c_int + 6 as libc::c_int) as isize) as libc::c_int
                >> 1 as libc::c_int)
                + *dct.offset((i_0 * 8 as libc::c_int + 2 as libc::c_int) as isize) as libc::c_int;
        let mut b0_0: libc::c_int = a0_0 + a6_0;
        let mut b2_0: libc::c_int = a2_0 + a4_0;
        let mut b4_0: libc::c_int = a2_0 - a4_0;
        let mut b6_0: libc::c_int = a0_0 - a6_0;
        let mut a1_0: libc::c_int = -(*dct
            .offset((i_0 * 8 as libc::c_int + 3 as libc::c_int) as isize)
            as libc::c_int)
            + *dct.offset((i_0 * 8 as libc::c_int + 5 as libc::c_int) as isize) as libc::c_int
            - *dct.offset((i_0 * 8 as libc::c_int + 7 as libc::c_int) as isize) as libc::c_int
            - (*dct.offset((i_0 * 8 as libc::c_int + 7 as libc::c_int) as isize) as libc::c_int
                >> 1 as libc::c_int);
        let mut a3_0: libc::c_int = *dct
            .offset((i_0 * 8 as libc::c_int + 1 as libc::c_int) as isize)
            as libc::c_int
            + *dct.offset((i_0 * 8 as libc::c_int + 7 as libc::c_int) as isize) as libc::c_int
            - *dct.offset((i_0 * 8 as libc::c_int + 3 as libc::c_int) as isize) as libc::c_int
            - (*dct.offset((i_0 * 8 as libc::c_int + 3 as libc::c_int) as isize) as libc::c_int
                >> 1 as libc::c_int);
        let mut a5_0: libc::c_int = -(*dct
            .offset((i_0 * 8 as libc::c_int + 1 as libc::c_int) as isize)
            as libc::c_int)
            + *dct.offset((i_0 * 8 as libc::c_int + 7 as libc::c_int) as isize) as libc::c_int
            + *dct.offset((i_0 * 8 as libc::c_int + 5 as libc::c_int) as isize) as libc::c_int
            + (*dct.offset((i_0 * 8 as libc::c_int + 5 as libc::c_int) as isize) as libc::c_int
                >> 1 as libc::c_int);
        let mut a7_0: libc::c_int = *dct
            .offset((i_0 * 8 as libc::c_int + 3 as libc::c_int) as isize)
            as libc::c_int
            + *dct.offset((i_0 * 8 as libc::c_int + 5 as libc::c_int) as isize) as libc::c_int
            + *dct.offset((i_0 * 8 as libc::c_int + 1 as libc::c_int) as isize) as libc::c_int
            + (*dct.offset((i_0 * 8 as libc::c_int + 1 as libc::c_int) as isize) as libc::c_int
                >> 1 as libc::c_int);
        let mut b1_0: libc::c_int = (a7_0 >> 2 as libc::c_int) + a1_0;
        let mut b3_0: libc::c_int = a3_0 + (a5_0 >> 2 as libc::c_int);
        let mut b5_0: libc::c_int = (a3_0 >> 2 as libc::c_int) - a5_0;
        let mut b7_0: libc::c_int = a7_0 - (a1_0 >> 2 as libc::c_int);
        *dst.offset((i_0 + 0 as libc::c_int * 32 as libc::c_int) as isize) = x264_clip_pixel(
            *dst.offset((i_0 + 0 as libc::c_int * 32 as libc::c_int) as isize) as libc::c_int
                + ((b0_0 + b7_0) >> 6 as libc::c_int),
        );
        *dst.offset((i_0 + 1 as libc::c_int * 32 as libc::c_int) as isize) = x264_clip_pixel(
            *dst.offset((i_0 + 1 as libc::c_int * 32 as libc::c_int) as isize) as libc::c_int
                + ((b2_0 + b5_0) >> 6 as libc::c_int),
        );
        *dst.offset((i_0 + 2 as libc::c_int * 32 as libc::c_int) as isize) = x264_clip_pixel(
            *dst.offset((i_0 + 2 as libc::c_int * 32 as libc::c_int) as isize) as libc::c_int
                + ((b4_0 + b3_0) >> 6 as libc::c_int),
        );
        *dst.offset((i_0 + 3 as libc::c_int * 32 as libc::c_int) as isize) = x264_clip_pixel(
            *dst.offset((i_0 + 3 as libc::c_int * 32 as libc::c_int) as isize) as libc::c_int
                + ((b6_0 + b1_0) >> 6 as libc::c_int),
        );
        *dst.offset((i_0 + 4 as libc::c_int * 32 as libc::c_int) as isize) = x264_clip_pixel(
            *dst.offset((i_0 + 4 as libc::c_int * 32 as libc::c_int) as isize) as libc::c_int
                + ((b6_0 - b1_0) >> 6 as libc::c_int),
        );
        *dst.offset((i_0 + 5 as libc::c_int * 32 as libc::c_int) as isize) = x264_clip_pixel(
            *dst.offset((i_0 + 5 as libc::c_int * 32 as libc::c_int) as isize) as libc::c_int
                + ((b4_0 - b3_0) >> 6 as libc::c_int),
        );
        *dst.offset((i_0 + 6 as libc::c_int * 32 as libc::c_int) as isize) = x264_clip_pixel(
            *dst.offset((i_0 + 6 as libc::c_int * 32 as libc::c_int) as isize) as libc::c_int
                + ((b2_0 - b5_0) >> 6 as libc::c_int),
        );
        *dst.offset((i_0 + 7 as libc::c_int * 32 as libc::c_int) as isize) = x264_clip_pixel(
            *dst.offset((i_0 + 7 as libc::c_int * 32 as libc::c_int) as isize) as libc::c_int
                + ((b0_0 - b7_0) >> 6 as libc::c_int),
        );
        i_0 += 1;
        i_0;
    }
}
unsafe extern "C" fn add16x16_idct8(mut dst: *mut pixel, mut dct: *mut [dctcoef; 64]) {
    add8x8_idct8(
        &mut *dst.offset(0 as libc::c_int as isize),
        (*dct.offset(0 as libc::c_int as isize)).as_mut_ptr(),
    );
    add8x8_idct8(
        &mut *dst.offset(8 as libc::c_int as isize),
        (*dct.offset(1 as libc::c_int as isize)).as_mut_ptr(),
    );
    add8x8_idct8(
        &mut *dst.offset((8 as libc::c_int * 32 as libc::c_int + 0 as libc::c_int) as isize),
        (*dct.offset(2 as libc::c_int as isize)).as_mut_ptr(),
    );
    add8x8_idct8(
        &mut *dst.offset((8 as libc::c_int * 32 as libc::c_int + 8 as libc::c_int) as isize),
        (*dct.offset(3 as libc::c_int as isize)).as_mut_ptr(),
    );
}
#[inline]
unsafe extern "C" fn add4x4_idct_dc(mut p_dst: *mut pixel, mut dc: dctcoef) {
    dc = ((dc as libc::c_int + 32 as libc::c_int) >> 6 as libc::c_int) as dctcoef;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        *p_dst.offset(0 as libc::c_int as isize) = x264_clip_pixel(
            *p_dst.offset(0 as libc::c_int as isize) as libc::c_int + dc as libc::c_int,
        );
        *p_dst.offset(1 as libc::c_int as isize) = x264_clip_pixel(
            *p_dst.offset(1 as libc::c_int as isize) as libc::c_int + dc as libc::c_int,
        );
        *p_dst.offset(2 as libc::c_int as isize) = x264_clip_pixel(
            *p_dst.offset(2 as libc::c_int as isize) as libc::c_int + dc as libc::c_int,
        );
        *p_dst.offset(3 as libc::c_int as isize) = x264_clip_pixel(
            *p_dst.offset(3 as libc::c_int as isize) as libc::c_int + dc as libc::c_int,
        );
        i += 1;
        i;
        p_dst = p_dst.offset(32 as libc::c_int as isize);
    }
}
unsafe extern "C" fn add8x8_idct_dc(mut p_dst: *mut pixel, mut dct: *mut dctcoef) {
    add4x4_idct_dc(
        &mut *p_dst.offset(0 as libc::c_int as isize),
        *dct.offset(0 as libc::c_int as isize),
    );
    add4x4_idct_dc(
        &mut *p_dst.offset(4 as libc::c_int as isize),
        *dct.offset(1 as libc::c_int as isize),
    );
    add4x4_idct_dc(
        &mut *p_dst.offset((4 as libc::c_int * 32 as libc::c_int + 0 as libc::c_int) as isize),
        *dct.offset(2 as libc::c_int as isize),
    );
    add4x4_idct_dc(
        &mut *p_dst.offset((4 as libc::c_int * 32 as libc::c_int + 4 as libc::c_int) as isize),
        *dct.offset(3 as libc::c_int as isize),
    );
}
unsafe extern "C" fn add16x16_idct_dc(mut p_dst: *mut pixel, mut dct: *mut dctcoef) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        add4x4_idct_dc(
            &mut *p_dst.offset(0 as libc::c_int as isize),
            *dct.offset(0 as libc::c_int as isize),
        );
        add4x4_idct_dc(
            &mut *p_dst.offset(4 as libc::c_int as isize),
            *dct.offset(1 as libc::c_int as isize),
        );
        add4x4_idct_dc(
            &mut *p_dst.offset(8 as libc::c_int as isize),
            *dct.offset(2 as libc::c_int as isize),
        );
        add4x4_idct_dc(
            &mut *p_dst.offset(12 as libc::c_int as isize),
            *dct.offset(3 as libc::c_int as isize),
        );
        i += 1;
        i;
        dct = dct.offset(4 as libc::c_int as isize);
        p_dst = p_dst.offset((4 as libc::c_int * 32 as libc::c_int) as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_dct_init(mut cpu: uint32_t, mut dctf: *mut x264_dct_function_t) {
    (*dctf).sub4x4_dct =
        Some(sub4x4_dct as unsafe extern "C" fn(*mut dctcoef, *mut pixel, *mut pixel) -> ());
    (*dctf).add4x4_idct = Some(add4x4_idct as unsafe extern "C" fn(*mut pixel, *mut dctcoef) -> ());
    (*dctf).sub8x8_dct =
        Some(sub8x8_dct as unsafe extern "C" fn(*mut [dctcoef; 16], *mut pixel, *mut pixel) -> ());
    (*dctf).sub8x8_dct_dc =
        Some(sub8x8_dct_dc as unsafe extern "C" fn(*mut dctcoef, *mut pixel, *mut pixel) -> ());
    (*dctf).add8x8_idct =
        Some(add8x8_idct as unsafe extern "C" fn(*mut pixel, *mut [dctcoef; 16]) -> ());
    (*dctf).add8x8_idct_dc =
        Some(add8x8_idct_dc as unsafe extern "C" fn(*mut pixel, *mut dctcoef) -> ());
    (*dctf).sub8x16_dct_dc =
        Some(sub8x16_dct_dc as unsafe extern "C" fn(*mut dctcoef, *mut pixel, *mut pixel) -> ());
    (*dctf).sub16x16_dct = Some(
        sub16x16_dct as unsafe extern "C" fn(*mut [dctcoef; 16], *mut pixel, *mut pixel) -> (),
    );
    (*dctf).add16x16_idct =
        Some(add16x16_idct as unsafe extern "C" fn(*mut pixel, *mut [dctcoef; 16]) -> ());
    (*dctf).add16x16_idct_dc =
        Some(add16x16_idct_dc as unsafe extern "C" fn(*mut pixel, *mut dctcoef) -> ());
    (*dctf).sub8x8_dct8 =
        Some(sub8x8_dct8 as unsafe extern "C" fn(*mut dctcoef, *mut pixel, *mut pixel) -> ());
    (*dctf).add8x8_idct8 =
        Some(add8x8_idct8 as unsafe extern "C" fn(*mut pixel, *mut dctcoef) -> ());
    (*dctf).sub16x16_dct8 = Some(
        sub16x16_dct8 as unsafe extern "C" fn(*mut [dctcoef; 64], *mut pixel, *mut pixel) -> (),
    );
    (*dctf).add16x16_idct8 =
        Some(add16x16_idct8 as unsafe extern "C" fn(*mut pixel, *mut [dctcoef; 64]) -> ());
    (*dctf).dct4x4dc = Some(dct4x4dc as unsafe extern "C" fn(*mut dctcoef) -> ());
    (*dctf).idct4x4dc = Some(idct4x4dc as unsafe extern "C" fn(*mut dctcoef) -> ());
    (*dctf).dct2x4dc =
        Some(dct2x4dc as unsafe extern "C" fn(*mut dctcoef, *mut [dctcoef; 16]) -> ());
}
unsafe extern "C" fn zigzag_scan_8x8_frame(mut level: *mut dctcoef, mut dct: *mut dctcoef) {
    *level.offset(0 as libc::c_int as isize) =
        *dct.offset((0 as libc::c_int * 8 as libc::c_int + 0 as libc::c_int) as isize);
    *level.offset(1 as libc::c_int as isize) =
        *dct.offset((1 as libc::c_int * 8 as libc::c_int + 0 as libc::c_int) as isize);
    *level.offset(2 as libc::c_int as isize) =
        *dct.offset((0 as libc::c_int * 8 as libc::c_int + 1 as libc::c_int) as isize);
    *level.offset(3 as libc::c_int as isize) =
        *dct.offset((0 as libc::c_int * 8 as libc::c_int + 2 as libc::c_int) as isize);
    *level.offset(4 as libc::c_int as isize) =
        *dct.offset((1 as libc::c_int * 8 as libc::c_int + 1 as libc::c_int) as isize);
    *level.offset(5 as libc::c_int as isize) =
        *dct.offset((2 as libc::c_int * 8 as libc::c_int + 0 as libc::c_int) as isize);
    *level.offset(6 as libc::c_int as isize) =
        *dct.offset((3 as libc::c_int * 8 as libc::c_int + 0 as libc::c_int) as isize);
    *level.offset(7 as libc::c_int as isize) =
        *dct.offset((2 as libc::c_int * 8 as libc::c_int + 1 as libc::c_int) as isize);
    *level.offset(8 as libc::c_int as isize) =
        *dct.offset((1 as libc::c_int * 8 as libc::c_int + 2 as libc::c_int) as isize);
    *level.offset(9 as libc::c_int as isize) =
        *dct.offset((0 as libc::c_int * 8 as libc::c_int + 3 as libc::c_int) as isize);
    *level.offset(10 as libc::c_int as isize) =
        *dct.offset((0 as libc::c_int * 8 as libc::c_int + 4 as libc::c_int) as isize);
    *level.offset(11 as libc::c_int as isize) =
        *dct.offset((1 as libc::c_int * 8 as libc::c_int + 3 as libc::c_int) as isize);
    *level.offset(12 as libc::c_int as isize) =
        *dct.offset((2 as libc::c_int * 8 as libc::c_int + 2 as libc::c_int) as isize);
    *level.offset(13 as libc::c_int as isize) =
        *dct.offset((3 as libc::c_int * 8 as libc::c_int + 1 as libc::c_int) as isize);
    *level.offset(14 as libc::c_int as isize) =
        *dct.offset((4 as libc::c_int * 8 as libc::c_int + 0 as libc::c_int) as isize);
    *level.offset(15 as libc::c_int as isize) =
        *dct.offset((5 as libc::c_int * 8 as libc::c_int + 0 as libc::c_int) as isize);
    *level.offset(16 as libc::c_int as isize) =
        *dct.offset((4 as libc::c_int * 8 as libc::c_int + 1 as libc::c_int) as isize);
    *level.offset(17 as libc::c_int as isize) =
        *dct.offset((3 as libc::c_int * 8 as libc::c_int + 2 as libc::c_int) as isize);
    *level.offset(18 as libc::c_int as isize) =
        *dct.offset((2 as libc::c_int * 8 as libc::c_int + 3 as libc::c_int) as isize);
    *level.offset(19 as libc::c_int as isize) =
        *dct.offset((1 as libc::c_int * 8 as libc::c_int + 4 as libc::c_int) as isize);
    *level.offset(20 as libc::c_int as isize) =
        *dct.offset((0 as libc::c_int * 8 as libc::c_int + 5 as libc::c_int) as isize);
    *level.offset(21 as libc::c_int as isize) =
        *dct.offset((0 as libc::c_int * 8 as libc::c_int + 6 as libc::c_int) as isize);
    *level.offset(22 as libc::c_int as isize) =
        *dct.offset((1 as libc::c_int * 8 as libc::c_int + 5 as libc::c_int) as isize);
    *level.offset(23 as libc::c_int as isize) =
        *dct.offset((2 as libc::c_int * 8 as libc::c_int + 4 as libc::c_int) as isize);
    *level.offset(24 as libc::c_int as isize) =
        *dct.offset((3 as libc::c_int * 8 as libc::c_int + 3 as libc::c_int) as isize);
    *level.offset(25 as libc::c_int as isize) =
        *dct.offset((4 as libc::c_int * 8 as libc::c_int + 2 as libc::c_int) as isize);
    *level.offset(26 as libc::c_int as isize) =
        *dct.offset((5 as libc::c_int * 8 as libc::c_int + 1 as libc::c_int) as isize);
    *level.offset(27 as libc::c_int as isize) =
        *dct.offset((6 as libc::c_int * 8 as libc::c_int + 0 as libc::c_int) as isize);
    *level.offset(28 as libc::c_int as isize) =
        *dct.offset((7 as libc::c_int * 8 as libc::c_int + 0 as libc::c_int) as isize);
    *level.offset(29 as libc::c_int as isize) =
        *dct.offset((6 as libc::c_int * 8 as libc::c_int + 1 as libc::c_int) as isize);
    *level.offset(30 as libc::c_int as isize) =
        *dct.offset((5 as libc::c_int * 8 as libc::c_int + 2 as libc::c_int) as isize);
    *level.offset(31 as libc::c_int as isize) =
        *dct.offset((4 as libc::c_int * 8 as libc::c_int + 3 as libc::c_int) as isize);
    *level.offset(32 as libc::c_int as isize) =
        *dct.offset((3 as libc::c_int * 8 as libc::c_int + 4 as libc::c_int) as isize);
    *level.offset(33 as libc::c_int as isize) =
        *dct.offset((2 as libc::c_int * 8 as libc::c_int + 5 as libc::c_int) as isize);
    *level.offset(34 as libc::c_int as isize) =
        *dct.offset((1 as libc::c_int * 8 as libc::c_int + 6 as libc::c_int) as isize);
    *level.offset(35 as libc::c_int as isize) =
        *dct.offset((0 as libc::c_int * 8 as libc::c_int + 7 as libc::c_int) as isize);
    *level.offset(36 as libc::c_int as isize) =
        *dct.offset((1 as libc::c_int * 8 as libc::c_int + 7 as libc::c_int) as isize);
    *level.offset(37 as libc::c_int as isize) =
        *dct.offset((2 as libc::c_int * 8 as libc::c_int + 6 as libc::c_int) as isize);
    *level.offset(38 as libc::c_int as isize) =
        *dct.offset((3 as libc::c_int * 8 as libc::c_int + 5 as libc::c_int) as isize);
    *level.offset(39 as libc::c_int as isize) =
        *dct.offset((4 as libc::c_int * 8 as libc::c_int + 4 as libc::c_int) as isize);
    *level.offset(40 as libc::c_int as isize) =
        *dct.offset((5 as libc::c_int * 8 as libc::c_int + 3 as libc::c_int) as isize);
    *level.offset(41 as libc::c_int as isize) =
        *dct.offset((6 as libc::c_int * 8 as libc::c_int + 2 as libc::c_int) as isize);
    *level.offset(42 as libc::c_int as isize) =
        *dct.offset((7 as libc::c_int * 8 as libc::c_int + 1 as libc::c_int) as isize);
    *level.offset(43 as libc::c_int as isize) =
        *dct.offset((7 as libc::c_int * 8 as libc::c_int + 2 as libc::c_int) as isize);
    *level.offset(44 as libc::c_int as isize) =
        *dct.offset((6 as libc::c_int * 8 as libc::c_int + 3 as libc::c_int) as isize);
    *level.offset(45 as libc::c_int as isize) =
        *dct.offset((5 as libc::c_int * 8 as libc::c_int + 4 as libc::c_int) as isize);
    *level.offset(46 as libc::c_int as isize) =
        *dct.offset((4 as libc::c_int * 8 as libc::c_int + 5 as libc::c_int) as isize);
    *level.offset(47 as libc::c_int as isize) =
        *dct.offset((3 as libc::c_int * 8 as libc::c_int + 6 as libc::c_int) as isize);
    *level.offset(48 as libc::c_int as isize) =
        *dct.offset((2 as libc::c_int * 8 as libc::c_int + 7 as libc::c_int) as isize);
    *level.offset(49 as libc::c_int as isize) =
        *dct.offset((3 as libc::c_int * 8 as libc::c_int + 7 as libc::c_int) as isize);
    *level.offset(50 as libc::c_int as isize) =
        *dct.offset((4 as libc::c_int * 8 as libc::c_int + 6 as libc::c_int) as isize);
    *level.offset(51 as libc::c_int as isize) =
        *dct.offset((5 as libc::c_int * 8 as libc::c_int + 5 as libc::c_int) as isize);
    *level.offset(52 as libc::c_int as isize) =
        *dct.offset((6 as libc::c_int * 8 as libc::c_int + 4 as libc::c_int) as isize);
    *level.offset(53 as libc::c_int as isize) =
        *dct.offset((7 as libc::c_int * 8 as libc::c_int + 3 as libc::c_int) as isize);
    *level.offset(54 as libc::c_int as isize) =
        *dct.offset((7 as libc::c_int * 8 as libc::c_int + 4 as libc::c_int) as isize);
    *level.offset(55 as libc::c_int as isize) =
        *dct.offset((6 as libc::c_int * 8 as libc::c_int + 5 as libc::c_int) as isize);
    *level.offset(56 as libc::c_int as isize) =
        *dct.offset((5 as libc::c_int * 8 as libc::c_int + 6 as libc::c_int) as isize);
    *level.offset(57 as libc::c_int as isize) =
        *dct.offset((4 as libc::c_int * 8 as libc::c_int + 7 as libc::c_int) as isize);
    *level.offset(58 as libc::c_int as isize) =
        *dct.offset((5 as libc::c_int * 8 as libc::c_int + 7 as libc::c_int) as isize);
    *level.offset(59 as libc::c_int as isize) =
        *dct.offset((6 as libc::c_int * 8 as libc::c_int + 6 as libc::c_int) as isize);
    *level.offset(60 as libc::c_int as isize) =
        *dct.offset((7 as libc::c_int * 8 as libc::c_int + 5 as libc::c_int) as isize);
    *level.offset(61 as libc::c_int as isize) =
        *dct.offset((7 as libc::c_int * 8 as libc::c_int + 6 as libc::c_int) as isize);
    *level.offset(62 as libc::c_int as isize) =
        *dct.offset((6 as libc::c_int * 8 as libc::c_int + 7 as libc::c_int) as isize);
    *level.offset(63 as libc::c_int as isize) =
        *dct.offset((7 as libc::c_int * 8 as libc::c_int + 7 as libc::c_int) as isize);
}
unsafe extern "C" fn zigzag_scan_8x8_field(mut level: *mut dctcoef, mut dct: *mut dctcoef) {
    *level.offset(0 as libc::c_int as isize) =
        *dct.offset((0 as libc::c_int * 8 as libc::c_int + 0 as libc::c_int) as isize);
    *level.offset(1 as libc::c_int as isize) =
        *dct.offset((0 as libc::c_int * 8 as libc::c_int + 1 as libc::c_int) as isize);
    *level.offset(2 as libc::c_int as isize) =
        *dct.offset((0 as libc::c_int * 8 as libc::c_int + 2 as libc::c_int) as isize);
    *level.offset(3 as libc::c_int as isize) =
        *dct.offset((1 as libc::c_int * 8 as libc::c_int + 0 as libc::c_int) as isize);
    *level.offset(4 as libc::c_int as isize) =
        *dct.offset((1 as libc::c_int * 8 as libc::c_int + 1 as libc::c_int) as isize);
    *level.offset(5 as libc::c_int as isize) =
        *dct.offset((0 as libc::c_int * 8 as libc::c_int + 3 as libc::c_int) as isize);
    *level.offset(6 as libc::c_int as isize) =
        *dct.offset((0 as libc::c_int * 8 as libc::c_int + 4 as libc::c_int) as isize);
    *level.offset(7 as libc::c_int as isize) =
        *dct.offset((1 as libc::c_int * 8 as libc::c_int + 2 as libc::c_int) as isize);
    *level.offset(8 as libc::c_int as isize) =
        *dct.offset((2 as libc::c_int * 8 as libc::c_int + 0 as libc::c_int) as isize);
    *level.offset(9 as libc::c_int as isize) =
        *dct.offset((1 as libc::c_int * 8 as libc::c_int + 3 as libc::c_int) as isize);
    *level.offset(10 as libc::c_int as isize) =
        *dct.offset((0 as libc::c_int * 8 as libc::c_int + 5 as libc::c_int) as isize);
    *level.offset(11 as libc::c_int as isize) =
        *dct.offset((0 as libc::c_int * 8 as libc::c_int + 6 as libc::c_int) as isize);
    *level.offset(12 as libc::c_int as isize) =
        *dct.offset((0 as libc::c_int * 8 as libc::c_int + 7 as libc::c_int) as isize);
    *level.offset(13 as libc::c_int as isize) =
        *dct.offset((1 as libc::c_int * 8 as libc::c_int + 4 as libc::c_int) as isize);
    *level.offset(14 as libc::c_int as isize) =
        *dct.offset((2 as libc::c_int * 8 as libc::c_int + 1 as libc::c_int) as isize);
    *level.offset(15 as libc::c_int as isize) =
        *dct.offset((3 as libc::c_int * 8 as libc::c_int + 0 as libc::c_int) as isize);
    *level.offset(16 as libc::c_int as isize) =
        *dct.offset((2 as libc::c_int * 8 as libc::c_int + 2 as libc::c_int) as isize);
    *level.offset(17 as libc::c_int as isize) =
        *dct.offset((1 as libc::c_int * 8 as libc::c_int + 5 as libc::c_int) as isize);
    *level.offset(18 as libc::c_int as isize) =
        *dct.offset((1 as libc::c_int * 8 as libc::c_int + 6 as libc::c_int) as isize);
    *level.offset(19 as libc::c_int as isize) =
        *dct.offset((1 as libc::c_int * 8 as libc::c_int + 7 as libc::c_int) as isize);
    *level.offset(20 as libc::c_int as isize) =
        *dct.offset((2 as libc::c_int * 8 as libc::c_int + 3 as libc::c_int) as isize);
    *level.offset(21 as libc::c_int as isize) =
        *dct.offset((3 as libc::c_int * 8 as libc::c_int + 1 as libc::c_int) as isize);
    *level.offset(22 as libc::c_int as isize) =
        *dct.offset((4 as libc::c_int * 8 as libc::c_int + 0 as libc::c_int) as isize);
    *level.offset(23 as libc::c_int as isize) =
        *dct.offset((3 as libc::c_int * 8 as libc::c_int + 2 as libc::c_int) as isize);
    *level.offset(24 as libc::c_int as isize) =
        *dct.offset((2 as libc::c_int * 8 as libc::c_int + 4 as libc::c_int) as isize);
    *level.offset(25 as libc::c_int as isize) =
        *dct.offset((2 as libc::c_int * 8 as libc::c_int + 5 as libc::c_int) as isize);
    *level.offset(26 as libc::c_int as isize) =
        *dct.offset((2 as libc::c_int * 8 as libc::c_int + 6 as libc::c_int) as isize);
    *level.offset(27 as libc::c_int as isize) =
        *dct.offset((2 as libc::c_int * 8 as libc::c_int + 7 as libc::c_int) as isize);
    *level.offset(28 as libc::c_int as isize) =
        *dct.offset((3 as libc::c_int * 8 as libc::c_int + 3 as libc::c_int) as isize);
    *level.offset(29 as libc::c_int as isize) =
        *dct.offset((4 as libc::c_int * 8 as libc::c_int + 1 as libc::c_int) as isize);
    *level.offset(30 as libc::c_int as isize) =
        *dct.offset((5 as libc::c_int * 8 as libc::c_int + 0 as libc::c_int) as isize);
    *level.offset(31 as libc::c_int as isize) =
        *dct.offset((4 as libc::c_int * 8 as libc::c_int + 2 as libc::c_int) as isize);
    *level.offset(32 as libc::c_int as isize) =
        *dct.offset((3 as libc::c_int * 8 as libc::c_int + 4 as libc::c_int) as isize);
    *level.offset(33 as libc::c_int as isize) =
        *dct.offset((3 as libc::c_int * 8 as libc::c_int + 5 as libc::c_int) as isize);
    *level.offset(34 as libc::c_int as isize) =
        *dct.offset((3 as libc::c_int * 8 as libc::c_int + 6 as libc::c_int) as isize);
    *level.offset(35 as libc::c_int as isize) =
        *dct.offset((3 as libc::c_int * 8 as libc::c_int + 7 as libc::c_int) as isize);
    *level.offset(36 as libc::c_int as isize) =
        *dct.offset((4 as libc::c_int * 8 as libc::c_int + 3 as libc::c_int) as isize);
    *level.offset(37 as libc::c_int as isize) =
        *dct.offset((5 as libc::c_int * 8 as libc::c_int + 1 as libc::c_int) as isize);
    *level.offset(38 as libc::c_int as isize) =
        *dct.offset((6 as libc::c_int * 8 as libc::c_int + 0 as libc::c_int) as isize);
    *level.offset(39 as libc::c_int as isize) =
        *dct.offset((5 as libc::c_int * 8 as libc::c_int + 2 as libc::c_int) as isize);
    *level.offset(40 as libc::c_int as isize) =
        *dct.offset((4 as libc::c_int * 8 as libc::c_int + 4 as libc::c_int) as isize);
    *level.offset(41 as libc::c_int as isize) =
        *dct.offset((4 as libc::c_int * 8 as libc::c_int + 5 as libc::c_int) as isize);
    *level.offset(42 as libc::c_int as isize) =
        *dct.offset((4 as libc::c_int * 8 as libc::c_int + 6 as libc::c_int) as isize);
    *level.offset(43 as libc::c_int as isize) =
        *dct.offset((4 as libc::c_int * 8 as libc::c_int + 7 as libc::c_int) as isize);
    *level.offset(44 as libc::c_int as isize) =
        *dct.offset((5 as libc::c_int * 8 as libc::c_int + 3 as libc::c_int) as isize);
    *level.offset(45 as libc::c_int as isize) =
        *dct.offset((6 as libc::c_int * 8 as libc::c_int + 1 as libc::c_int) as isize);
    *level.offset(46 as libc::c_int as isize) =
        *dct.offset((6 as libc::c_int * 8 as libc::c_int + 2 as libc::c_int) as isize);
    *level.offset(47 as libc::c_int as isize) =
        *dct.offset((5 as libc::c_int * 8 as libc::c_int + 4 as libc::c_int) as isize);
    *level.offset(48 as libc::c_int as isize) =
        *dct.offset((5 as libc::c_int * 8 as libc::c_int + 5 as libc::c_int) as isize);
    *level.offset(49 as libc::c_int as isize) =
        *dct.offset((5 as libc::c_int * 8 as libc::c_int + 6 as libc::c_int) as isize);
    *level.offset(50 as libc::c_int as isize) =
        *dct.offset((5 as libc::c_int * 8 as libc::c_int + 7 as libc::c_int) as isize);
    *level.offset(51 as libc::c_int as isize) =
        *dct.offset((6 as libc::c_int * 8 as libc::c_int + 3 as libc::c_int) as isize);
    *level.offset(52 as libc::c_int as isize) =
        *dct.offset((7 as libc::c_int * 8 as libc::c_int + 0 as libc::c_int) as isize);
    *level.offset(53 as libc::c_int as isize) =
        *dct.offset((7 as libc::c_int * 8 as libc::c_int + 1 as libc::c_int) as isize);
    *level.offset(54 as libc::c_int as isize) =
        *dct.offset((6 as libc::c_int * 8 as libc::c_int + 4 as libc::c_int) as isize);
    *level.offset(55 as libc::c_int as isize) =
        *dct.offset((6 as libc::c_int * 8 as libc::c_int + 5 as libc::c_int) as isize);
    *level.offset(56 as libc::c_int as isize) =
        *dct.offset((6 as libc::c_int * 8 as libc::c_int + 6 as libc::c_int) as isize);
    *level.offset(57 as libc::c_int as isize) =
        *dct.offset((6 as libc::c_int * 8 as libc::c_int + 7 as libc::c_int) as isize);
    *level.offset(58 as libc::c_int as isize) =
        *dct.offset((7 as libc::c_int * 8 as libc::c_int + 2 as libc::c_int) as isize);
    *level.offset(59 as libc::c_int as isize) =
        *dct.offset((7 as libc::c_int * 8 as libc::c_int + 3 as libc::c_int) as isize);
    *level.offset(60 as libc::c_int as isize) =
        *dct.offset((7 as libc::c_int * 8 as libc::c_int + 4 as libc::c_int) as isize);
    *level.offset(61 as libc::c_int as isize) =
        *dct.offset((7 as libc::c_int * 8 as libc::c_int + 5 as libc::c_int) as isize);
    *level.offset(62 as libc::c_int as isize) =
        *dct.offset((7 as libc::c_int * 8 as libc::c_int + 6 as libc::c_int) as isize);
    *level.offset(63 as libc::c_int as isize) =
        *dct.offset((7 as libc::c_int * 8 as libc::c_int + 7 as libc::c_int) as isize);
}
unsafe extern "C" fn zigzag_scan_4x4_frame(mut level: *mut dctcoef, mut dct: *mut dctcoef) {
    *level.offset(0 as libc::c_int as isize) =
        *dct.offset((0 as libc::c_int * 4 as libc::c_int + 0 as libc::c_int) as isize);
    *level.offset(1 as libc::c_int as isize) =
        *dct.offset((1 as libc::c_int * 4 as libc::c_int + 0 as libc::c_int) as isize);
    *level.offset(2 as libc::c_int as isize) =
        *dct.offset((0 as libc::c_int * 4 as libc::c_int + 1 as libc::c_int) as isize);
    *level.offset(3 as libc::c_int as isize) =
        *dct.offset((0 as libc::c_int * 4 as libc::c_int + 2 as libc::c_int) as isize);
    *level.offset(4 as libc::c_int as isize) =
        *dct.offset((1 as libc::c_int * 4 as libc::c_int + 1 as libc::c_int) as isize);
    *level.offset(5 as libc::c_int as isize) =
        *dct.offset((2 as libc::c_int * 4 as libc::c_int + 0 as libc::c_int) as isize);
    *level.offset(6 as libc::c_int as isize) =
        *dct.offset((3 as libc::c_int * 4 as libc::c_int + 0 as libc::c_int) as isize);
    *level.offset(7 as libc::c_int as isize) =
        *dct.offset((2 as libc::c_int * 4 as libc::c_int + 1 as libc::c_int) as isize);
    *level.offset(8 as libc::c_int as isize) =
        *dct.offset((1 as libc::c_int * 4 as libc::c_int + 2 as libc::c_int) as isize);
    *level.offset(9 as libc::c_int as isize) =
        *dct.offset((0 as libc::c_int * 4 as libc::c_int + 3 as libc::c_int) as isize);
    *level.offset(10 as libc::c_int as isize) =
        *dct.offset((1 as libc::c_int * 4 as libc::c_int + 3 as libc::c_int) as isize);
    *level.offset(11 as libc::c_int as isize) =
        *dct.offset((2 as libc::c_int * 4 as libc::c_int + 2 as libc::c_int) as isize);
    *level.offset(12 as libc::c_int as isize) =
        *dct.offset((3 as libc::c_int * 4 as libc::c_int + 1 as libc::c_int) as isize);
    *level.offset(13 as libc::c_int as isize) =
        *dct.offset((3 as libc::c_int * 4 as libc::c_int + 2 as libc::c_int) as isize);
    *level.offset(14 as libc::c_int as isize) =
        *dct.offset((2 as libc::c_int * 4 as libc::c_int + 3 as libc::c_int) as isize);
    *level.offset(15 as libc::c_int as isize) =
        *dct.offset((3 as libc::c_int * 4 as libc::c_int + 3 as libc::c_int) as isize);
}
unsafe extern "C" fn zigzag_scan_4x4_field(mut level: *mut dctcoef, mut dct: *mut dctcoef) {
    memcpy(
        level as *mut libc::c_void,
        dct as *const libc::c_void,
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<dctcoef>() as libc::c_ulong),
    );
    *level.offset(2 as libc::c_int as isize) =
        *dct.offset((1 as libc::c_int * 4 as libc::c_int + 0 as libc::c_int) as isize);
    *level.offset(3 as libc::c_int as isize) =
        *dct.offset((0 as libc::c_int * 4 as libc::c_int + 2 as libc::c_int) as isize);
    *level.offset(4 as libc::c_int as isize) =
        *dct.offset((0 as libc::c_int * 4 as libc::c_int + 3 as libc::c_int) as isize);
    *level.offset(5 as libc::c_int as isize) =
        *dct.offset((1 as libc::c_int * 4 as libc::c_int + 1 as libc::c_int) as isize);
    memcpy(
        level.offset(6 as libc::c_int as isize) as *mut libc::c_void,
        dct.offset(6 as libc::c_int as isize) as *const libc::c_void,
        (10 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<dctcoef>() as libc::c_ulong),
    );
}
unsafe extern "C" fn zigzag_sub_4x4_frame(
    mut level: *mut dctcoef,
    mut p_src: *const pixel,
    mut p_dst: *mut pixel,
) -> libc::c_int {
    let mut nz: libc::c_int = 0 as libc::c_int;
    let mut oe: libc::c_int = 0 as libc::c_int + 0 as libc::c_int * 16 as libc::c_int;
    let mut od: libc::c_int = 0 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int;
    *level.offset(0 as libc::c_int as isize) = (*p_src.offset(oe as isize) as libc::c_int
        - *p_dst.offset(od as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut oe_0: libc::c_int = 1 as libc::c_int + 0 as libc::c_int * 16 as libc::c_int;
    let mut od_0: libc::c_int = 1 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int;
    *level.offset(1 as libc::c_int as isize) = (*p_src.offset(oe_0 as isize) as libc::c_int
        - *p_dst.offset(od_0 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(1 as libc::c_int as isize) as libc::c_int;
    let mut oe_1: libc::c_int = 0 as libc::c_int + 1 as libc::c_int * 16 as libc::c_int;
    let mut od_1: libc::c_int = 0 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int;
    *level.offset(2 as libc::c_int as isize) = (*p_src.offset(oe_1 as isize) as libc::c_int
        - *p_dst.offset(od_1 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(2 as libc::c_int as isize) as libc::c_int;
    let mut oe_2: libc::c_int = 0 as libc::c_int + 2 as libc::c_int * 16 as libc::c_int;
    let mut od_2: libc::c_int = 0 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int;
    *level.offset(3 as libc::c_int as isize) = (*p_src.offset(oe_2 as isize) as libc::c_int
        - *p_dst.offset(od_2 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(3 as libc::c_int as isize) as libc::c_int;
    let mut oe_3: libc::c_int = 1 as libc::c_int + 1 as libc::c_int * 16 as libc::c_int;
    let mut od_3: libc::c_int = 1 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int;
    *level.offset(4 as libc::c_int as isize) = (*p_src.offset(oe_3 as isize) as libc::c_int
        - *p_dst.offset(od_3 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(4 as libc::c_int as isize) as libc::c_int;
    let mut oe_4: libc::c_int = 2 as libc::c_int + 0 as libc::c_int * 16 as libc::c_int;
    let mut od_4: libc::c_int = 2 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int;
    *level.offset(5 as libc::c_int as isize) = (*p_src.offset(oe_4 as isize) as libc::c_int
        - *p_dst.offset(od_4 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(5 as libc::c_int as isize) as libc::c_int;
    let mut oe_5: libc::c_int = 3 as libc::c_int + 0 as libc::c_int * 16 as libc::c_int;
    let mut od_5: libc::c_int = 3 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int;
    *level.offset(6 as libc::c_int as isize) = (*p_src.offset(oe_5 as isize) as libc::c_int
        - *p_dst.offset(od_5 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(6 as libc::c_int as isize) as libc::c_int;
    let mut oe_6: libc::c_int = 2 as libc::c_int + 1 as libc::c_int * 16 as libc::c_int;
    let mut od_6: libc::c_int = 2 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int;
    *level.offset(7 as libc::c_int as isize) = (*p_src.offset(oe_6 as isize) as libc::c_int
        - *p_dst.offset(od_6 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(7 as libc::c_int as isize) as libc::c_int;
    let mut oe_7: libc::c_int = 1 as libc::c_int + 2 as libc::c_int * 16 as libc::c_int;
    let mut od_7: libc::c_int = 1 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int;
    *level.offset(8 as libc::c_int as isize) = (*p_src.offset(oe_7 as isize) as libc::c_int
        - *p_dst.offset(od_7 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(8 as libc::c_int as isize) as libc::c_int;
    let mut oe_8: libc::c_int = 0 as libc::c_int + 3 as libc::c_int * 16 as libc::c_int;
    let mut od_8: libc::c_int = 0 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int;
    *level.offset(9 as libc::c_int as isize) = (*p_src.offset(oe_8 as isize) as libc::c_int
        - *p_dst.offset(od_8 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(9 as libc::c_int as isize) as libc::c_int;
    let mut oe_9: libc::c_int = 1 as libc::c_int + 3 as libc::c_int * 16 as libc::c_int;
    let mut od_9: libc::c_int = 1 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int;
    *level.offset(10 as libc::c_int as isize) = (*p_src.offset(oe_9 as isize) as libc::c_int
        - *p_dst.offset(od_9 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(10 as libc::c_int as isize) as libc::c_int;
    let mut oe_10: libc::c_int = 2 as libc::c_int + 2 as libc::c_int * 16 as libc::c_int;
    let mut od_10: libc::c_int = 2 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int;
    *level.offset(11 as libc::c_int as isize) = (*p_src.offset(oe_10 as isize) as libc::c_int
        - *p_dst.offset(od_10 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(11 as libc::c_int as isize) as libc::c_int;
    let mut oe_11: libc::c_int = 3 as libc::c_int + 1 as libc::c_int * 16 as libc::c_int;
    let mut od_11: libc::c_int = 3 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int;
    *level.offset(12 as libc::c_int as isize) = (*p_src.offset(oe_11 as isize) as libc::c_int
        - *p_dst.offset(od_11 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(12 as libc::c_int as isize) as libc::c_int;
    let mut oe_12: libc::c_int = 3 as libc::c_int + 2 as libc::c_int * 16 as libc::c_int;
    let mut od_12: libc::c_int = 3 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int;
    *level.offset(13 as libc::c_int as isize) = (*p_src.offset(oe_12 as isize) as libc::c_int
        - *p_dst.offset(od_12 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(13 as libc::c_int as isize) as libc::c_int;
    let mut oe_13: libc::c_int = 2 as libc::c_int + 3 as libc::c_int * 16 as libc::c_int;
    let mut od_13: libc::c_int = 2 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int;
    *level.offset(14 as libc::c_int as isize) = (*p_src.offset(oe_13 as isize) as libc::c_int
        - *p_dst.offset(od_13 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(14 as libc::c_int as isize) as libc::c_int;
    let mut oe_14: libc::c_int = 3 as libc::c_int + 3 as libc::c_int * 16 as libc::c_int;
    let mut od_14: libc::c_int = 3 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int;
    *level.offset(15 as libc::c_int as isize) = (*p_src.offset(oe_14 as isize) as libc::c_int
        - *p_dst.offset(od_14 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(15 as libc::c_int as isize) as libc::c_int;
    (*(p_dst.offset((0 as libc::c_int * 32 as libc::c_int) as isize) as *mut x264_union32_t)).i =
        (*(p_src.offset((0 as libc::c_int * 16 as libc::c_int) as isize) as *mut x264_union32_t)).i;
    (*(p_dst.offset((1 as libc::c_int * 32 as libc::c_int) as isize) as *mut x264_union32_t)).i =
        (*(p_src.offset((1 as libc::c_int * 16 as libc::c_int) as isize) as *mut x264_union32_t)).i;
    (*(p_dst.offset((2 as libc::c_int * 32 as libc::c_int) as isize) as *mut x264_union32_t)).i =
        (*(p_src.offset((2 as libc::c_int * 16 as libc::c_int) as isize) as *mut x264_union32_t)).i;
    (*(p_dst.offset((3 as libc::c_int * 32 as libc::c_int) as isize) as *mut x264_union32_t)).i =
        (*(p_src.offset((3 as libc::c_int * 16 as libc::c_int) as isize) as *mut x264_union32_t)).i;
    (nz != 0) as libc::c_int
}
unsafe extern "C" fn zigzag_sub_4x4_field(
    mut level: *mut dctcoef,
    mut p_src: *const pixel,
    mut p_dst: *mut pixel,
) -> libc::c_int {
    let mut nz: libc::c_int = 0 as libc::c_int;
    let mut oe: libc::c_int = 0 as libc::c_int + 0 as libc::c_int * 16 as libc::c_int;
    let mut od: libc::c_int = 0 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int;
    *level.offset(0 as libc::c_int as isize) = (*p_src.offset(oe as isize) as libc::c_int
        - *p_dst.offset(od as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut oe_0: libc::c_int = 0 as libc::c_int + 1 as libc::c_int * 16 as libc::c_int;
    let mut od_0: libc::c_int = 0 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int;
    *level.offset(1 as libc::c_int as isize) = (*p_src.offset(oe_0 as isize) as libc::c_int
        - *p_dst.offset(od_0 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(1 as libc::c_int as isize) as libc::c_int;
    let mut oe_1: libc::c_int = 1 as libc::c_int + 0 as libc::c_int * 16 as libc::c_int;
    let mut od_1: libc::c_int = 1 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int;
    *level.offset(2 as libc::c_int as isize) = (*p_src.offset(oe_1 as isize) as libc::c_int
        - *p_dst.offset(od_1 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(2 as libc::c_int as isize) as libc::c_int;
    let mut oe_2: libc::c_int = 0 as libc::c_int + 2 as libc::c_int * 16 as libc::c_int;
    let mut od_2: libc::c_int = 0 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int;
    *level.offset(3 as libc::c_int as isize) = (*p_src.offset(oe_2 as isize) as libc::c_int
        - *p_dst.offset(od_2 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(3 as libc::c_int as isize) as libc::c_int;
    let mut oe_3: libc::c_int = 0 as libc::c_int + 3 as libc::c_int * 16 as libc::c_int;
    let mut od_3: libc::c_int = 0 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int;
    *level.offset(4 as libc::c_int as isize) = (*p_src.offset(oe_3 as isize) as libc::c_int
        - *p_dst.offset(od_3 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(4 as libc::c_int as isize) as libc::c_int;
    let mut oe_4: libc::c_int = 1 as libc::c_int + 1 as libc::c_int * 16 as libc::c_int;
    let mut od_4: libc::c_int = 1 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int;
    *level.offset(5 as libc::c_int as isize) = (*p_src.offset(oe_4 as isize) as libc::c_int
        - *p_dst.offset(od_4 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(5 as libc::c_int as isize) as libc::c_int;
    let mut oe_5: libc::c_int = 1 as libc::c_int + 2 as libc::c_int * 16 as libc::c_int;
    let mut od_5: libc::c_int = 1 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int;
    *level.offset(6 as libc::c_int as isize) = (*p_src.offset(oe_5 as isize) as libc::c_int
        - *p_dst.offset(od_5 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(6 as libc::c_int as isize) as libc::c_int;
    let mut oe_6: libc::c_int = 1 as libc::c_int + 3 as libc::c_int * 16 as libc::c_int;
    let mut od_6: libc::c_int = 1 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int;
    *level.offset(7 as libc::c_int as isize) = (*p_src.offset(oe_6 as isize) as libc::c_int
        - *p_dst.offset(od_6 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(7 as libc::c_int as isize) as libc::c_int;
    let mut oe_7: libc::c_int = 2 as libc::c_int + 0 as libc::c_int * 16 as libc::c_int;
    let mut od_7: libc::c_int = 2 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int;
    *level.offset(8 as libc::c_int as isize) = (*p_src.offset(oe_7 as isize) as libc::c_int
        - *p_dst.offset(od_7 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(8 as libc::c_int as isize) as libc::c_int;
    let mut oe_8: libc::c_int = 2 as libc::c_int + 1 as libc::c_int * 16 as libc::c_int;
    let mut od_8: libc::c_int = 2 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int;
    *level.offset(9 as libc::c_int as isize) = (*p_src.offset(oe_8 as isize) as libc::c_int
        - *p_dst.offset(od_8 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(9 as libc::c_int as isize) as libc::c_int;
    let mut oe_9: libc::c_int = 2 as libc::c_int + 2 as libc::c_int * 16 as libc::c_int;
    let mut od_9: libc::c_int = 2 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int;
    *level.offset(10 as libc::c_int as isize) = (*p_src.offset(oe_9 as isize) as libc::c_int
        - *p_dst.offset(od_9 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(10 as libc::c_int as isize) as libc::c_int;
    let mut oe_10: libc::c_int = 2 as libc::c_int + 3 as libc::c_int * 16 as libc::c_int;
    let mut od_10: libc::c_int = 2 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int;
    *level.offset(11 as libc::c_int as isize) = (*p_src.offset(oe_10 as isize) as libc::c_int
        - *p_dst.offset(od_10 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(11 as libc::c_int as isize) as libc::c_int;
    let mut oe_11: libc::c_int = 3 as libc::c_int + 0 as libc::c_int * 16 as libc::c_int;
    let mut od_11: libc::c_int = 3 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int;
    *level.offset(12 as libc::c_int as isize) = (*p_src.offset(oe_11 as isize) as libc::c_int
        - *p_dst.offset(od_11 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(12 as libc::c_int as isize) as libc::c_int;
    let mut oe_12: libc::c_int = 3 as libc::c_int + 1 as libc::c_int * 16 as libc::c_int;
    let mut od_12: libc::c_int = 3 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int;
    *level.offset(13 as libc::c_int as isize) = (*p_src.offset(oe_12 as isize) as libc::c_int
        - *p_dst.offset(od_12 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(13 as libc::c_int as isize) as libc::c_int;
    let mut oe_13: libc::c_int = 3 as libc::c_int + 2 as libc::c_int * 16 as libc::c_int;
    let mut od_13: libc::c_int = 3 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int;
    *level.offset(14 as libc::c_int as isize) = (*p_src.offset(oe_13 as isize) as libc::c_int
        - *p_dst.offset(od_13 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(14 as libc::c_int as isize) as libc::c_int;
    let mut oe_14: libc::c_int = 3 as libc::c_int + 3 as libc::c_int * 16 as libc::c_int;
    let mut od_14: libc::c_int = 3 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int;
    *level.offset(15 as libc::c_int as isize) = (*p_src.offset(oe_14 as isize) as libc::c_int
        - *p_dst.offset(od_14 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(15 as libc::c_int as isize) as libc::c_int;
    (*(p_dst.offset((0 as libc::c_int * 32 as libc::c_int) as isize) as *mut x264_union32_t)).i =
        (*(p_src.offset((0 as libc::c_int * 16 as libc::c_int) as isize) as *mut x264_union32_t)).i;
    (*(p_dst.offset((1 as libc::c_int * 32 as libc::c_int) as isize) as *mut x264_union32_t)).i =
        (*(p_src.offset((1 as libc::c_int * 16 as libc::c_int) as isize) as *mut x264_union32_t)).i;
    (*(p_dst.offset((2 as libc::c_int * 32 as libc::c_int) as isize) as *mut x264_union32_t)).i =
        (*(p_src.offset((2 as libc::c_int * 16 as libc::c_int) as isize) as *mut x264_union32_t)).i;
    (*(p_dst.offset((3 as libc::c_int * 32 as libc::c_int) as isize) as *mut x264_union32_t)).i =
        (*(p_src.offset((3 as libc::c_int * 16 as libc::c_int) as isize) as *mut x264_union32_t)).i;
    (nz != 0) as libc::c_int
}
unsafe extern "C" fn zigzag_sub_4x4ac_frame(
    mut level: *mut dctcoef,
    mut p_src: *const pixel,
    mut p_dst: *mut pixel,
    mut dc: *mut dctcoef,
) -> libc::c_int {
    let mut nz: libc::c_int = 0 as libc::c_int;
    let mut oe: libc::c_int = 0 as libc::c_int + 0 as libc::c_int * 16 as libc::c_int;
    let mut od: libc::c_int = 0 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int;
    *dc = (*p_src.offset(oe as isize) as libc::c_int - *p_dst.offset(od as isize) as libc::c_int)
        as dctcoef;
    *level.offset(0 as libc::c_int as isize) = 0 as libc::c_int as dctcoef;
    let mut oe_0: libc::c_int = 1 as libc::c_int + 0 as libc::c_int * 16 as libc::c_int;
    let mut od_0: libc::c_int = 1 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int;
    *level.offset(1 as libc::c_int as isize) = (*p_src.offset(oe_0 as isize) as libc::c_int
        - *p_dst.offset(od_0 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(1 as libc::c_int as isize) as libc::c_int;
    let mut oe_1: libc::c_int = 0 as libc::c_int + 1 as libc::c_int * 16 as libc::c_int;
    let mut od_1: libc::c_int = 0 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int;
    *level.offset(2 as libc::c_int as isize) = (*p_src.offset(oe_1 as isize) as libc::c_int
        - *p_dst.offset(od_1 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(2 as libc::c_int as isize) as libc::c_int;
    let mut oe_2: libc::c_int = 0 as libc::c_int + 2 as libc::c_int * 16 as libc::c_int;
    let mut od_2: libc::c_int = 0 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int;
    *level.offset(3 as libc::c_int as isize) = (*p_src.offset(oe_2 as isize) as libc::c_int
        - *p_dst.offset(od_2 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(3 as libc::c_int as isize) as libc::c_int;
    let mut oe_3: libc::c_int = 1 as libc::c_int + 1 as libc::c_int * 16 as libc::c_int;
    let mut od_3: libc::c_int = 1 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int;
    *level.offset(4 as libc::c_int as isize) = (*p_src.offset(oe_3 as isize) as libc::c_int
        - *p_dst.offset(od_3 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(4 as libc::c_int as isize) as libc::c_int;
    let mut oe_4: libc::c_int = 2 as libc::c_int + 0 as libc::c_int * 16 as libc::c_int;
    let mut od_4: libc::c_int = 2 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int;
    *level.offset(5 as libc::c_int as isize) = (*p_src.offset(oe_4 as isize) as libc::c_int
        - *p_dst.offset(od_4 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(5 as libc::c_int as isize) as libc::c_int;
    let mut oe_5: libc::c_int = 3 as libc::c_int + 0 as libc::c_int * 16 as libc::c_int;
    let mut od_5: libc::c_int = 3 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int;
    *level.offset(6 as libc::c_int as isize) = (*p_src.offset(oe_5 as isize) as libc::c_int
        - *p_dst.offset(od_5 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(6 as libc::c_int as isize) as libc::c_int;
    let mut oe_6: libc::c_int = 2 as libc::c_int + 1 as libc::c_int * 16 as libc::c_int;
    let mut od_6: libc::c_int = 2 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int;
    *level.offset(7 as libc::c_int as isize) = (*p_src.offset(oe_6 as isize) as libc::c_int
        - *p_dst.offset(od_6 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(7 as libc::c_int as isize) as libc::c_int;
    let mut oe_7: libc::c_int = 1 as libc::c_int + 2 as libc::c_int * 16 as libc::c_int;
    let mut od_7: libc::c_int = 1 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int;
    *level.offset(8 as libc::c_int as isize) = (*p_src.offset(oe_7 as isize) as libc::c_int
        - *p_dst.offset(od_7 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(8 as libc::c_int as isize) as libc::c_int;
    let mut oe_8: libc::c_int = 0 as libc::c_int + 3 as libc::c_int * 16 as libc::c_int;
    let mut od_8: libc::c_int = 0 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int;
    *level.offset(9 as libc::c_int as isize) = (*p_src.offset(oe_8 as isize) as libc::c_int
        - *p_dst.offset(od_8 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(9 as libc::c_int as isize) as libc::c_int;
    let mut oe_9: libc::c_int = 1 as libc::c_int + 3 as libc::c_int * 16 as libc::c_int;
    let mut od_9: libc::c_int = 1 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int;
    *level.offset(10 as libc::c_int as isize) = (*p_src.offset(oe_9 as isize) as libc::c_int
        - *p_dst.offset(od_9 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(10 as libc::c_int as isize) as libc::c_int;
    let mut oe_10: libc::c_int = 2 as libc::c_int + 2 as libc::c_int * 16 as libc::c_int;
    let mut od_10: libc::c_int = 2 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int;
    *level.offset(11 as libc::c_int as isize) = (*p_src.offset(oe_10 as isize) as libc::c_int
        - *p_dst.offset(od_10 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(11 as libc::c_int as isize) as libc::c_int;
    let mut oe_11: libc::c_int = 3 as libc::c_int + 1 as libc::c_int * 16 as libc::c_int;
    let mut od_11: libc::c_int = 3 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int;
    *level.offset(12 as libc::c_int as isize) = (*p_src.offset(oe_11 as isize) as libc::c_int
        - *p_dst.offset(od_11 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(12 as libc::c_int as isize) as libc::c_int;
    let mut oe_12: libc::c_int = 3 as libc::c_int + 2 as libc::c_int * 16 as libc::c_int;
    let mut od_12: libc::c_int = 3 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int;
    *level.offset(13 as libc::c_int as isize) = (*p_src.offset(oe_12 as isize) as libc::c_int
        - *p_dst.offset(od_12 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(13 as libc::c_int as isize) as libc::c_int;
    let mut oe_13: libc::c_int = 2 as libc::c_int + 3 as libc::c_int * 16 as libc::c_int;
    let mut od_13: libc::c_int = 2 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int;
    *level.offset(14 as libc::c_int as isize) = (*p_src.offset(oe_13 as isize) as libc::c_int
        - *p_dst.offset(od_13 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(14 as libc::c_int as isize) as libc::c_int;
    let mut oe_14: libc::c_int = 3 as libc::c_int + 3 as libc::c_int * 16 as libc::c_int;
    let mut od_14: libc::c_int = 3 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int;
    *level.offset(15 as libc::c_int as isize) = (*p_src.offset(oe_14 as isize) as libc::c_int
        - *p_dst.offset(od_14 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(15 as libc::c_int as isize) as libc::c_int;
    (*(p_dst.offset((0 as libc::c_int * 32 as libc::c_int) as isize) as *mut x264_union32_t)).i =
        (*(p_src.offset((0 as libc::c_int * 16 as libc::c_int) as isize) as *mut x264_union32_t)).i;
    (*(p_dst.offset((1 as libc::c_int * 32 as libc::c_int) as isize) as *mut x264_union32_t)).i =
        (*(p_src.offset((1 as libc::c_int * 16 as libc::c_int) as isize) as *mut x264_union32_t)).i;
    (*(p_dst.offset((2 as libc::c_int * 32 as libc::c_int) as isize) as *mut x264_union32_t)).i =
        (*(p_src.offset((2 as libc::c_int * 16 as libc::c_int) as isize) as *mut x264_union32_t)).i;
    (*(p_dst.offset((3 as libc::c_int * 32 as libc::c_int) as isize) as *mut x264_union32_t)).i =
        (*(p_src.offset((3 as libc::c_int * 16 as libc::c_int) as isize) as *mut x264_union32_t)).i;
    (nz != 0) as libc::c_int
}
unsafe extern "C" fn zigzag_sub_4x4ac_field(
    mut level: *mut dctcoef,
    mut p_src: *const pixel,
    mut p_dst: *mut pixel,
    mut dc: *mut dctcoef,
) -> libc::c_int {
    let mut nz: libc::c_int = 0 as libc::c_int;
    let mut oe: libc::c_int = 0 as libc::c_int + 0 as libc::c_int * 16 as libc::c_int;
    let mut od: libc::c_int = 0 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int;
    *dc = (*p_src.offset(oe as isize) as libc::c_int - *p_dst.offset(od as isize) as libc::c_int)
        as dctcoef;
    *level.offset(0 as libc::c_int as isize) = 0 as libc::c_int as dctcoef;
    let mut oe_0: libc::c_int = 0 as libc::c_int + 1 as libc::c_int * 16 as libc::c_int;
    let mut od_0: libc::c_int = 0 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int;
    *level.offset(1 as libc::c_int as isize) = (*p_src.offset(oe_0 as isize) as libc::c_int
        - *p_dst.offset(od_0 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(1 as libc::c_int as isize) as libc::c_int;
    let mut oe_1: libc::c_int = 1 as libc::c_int + 0 as libc::c_int * 16 as libc::c_int;
    let mut od_1: libc::c_int = 1 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int;
    *level.offset(2 as libc::c_int as isize) = (*p_src.offset(oe_1 as isize) as libc::c_int
        - *p_dst.offset(od_1 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(2 as libc::c_int as isize) as libc::c_int;
    let mut oe_2: libc::c_int = 0 as libc::c_int + 2 as libc::c_int * 16 as libc::c_int;
    let mut od_2: libc::c_int = 0 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int;
    *level.offset(3 as libc::c_int as isize) = (*p_src.offset(oe_2 as isize) as libc::c_int
        - *p_dst.offset(od_2 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(3 as libc::c_int as isize) as libc::c_int;
    let mut oe_3: libc::c_int = 0 as libc::c_int + 3 as libc::c_int * 16 as libc::c_int;
    let mut od_3: libc::c_int = 0 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int;
    *level.offset(4 as libc::c_int as isize) = (*p_src.offset(oe_3 as isize) as libc::c_int
        - *p_dst.offset(od_3 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(4 as libc::c_int as isize) as libc::c_int;
    let mut oe_4: libc::c_int = 1 as libc::c_int + 1 as libc::c_int * 16 as libc::c_int;
    let mut od_4: libc::c_int = 1 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int;
    *level.offset(5 as libc::c_int as isize) = (*p_src.offset(oe_4 as isize) as libc::c_int
        - *p_dst.offset(od_4 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(5 as libc::c_int as isize) as libc::c_int;
    let mut oe_5: libc::c_int = 1 as libc::c_int + 2 as libc::c_int * 16 as libc::c_int;
    let mut od_5: libc::c_int = 1 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int;
    *level.offset(6 as libc::c_int as isize) = (*p_src.offset(oe_5 as isize) as libc::c_int
        - *p_dst.offset(od_5 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(6 as libc::c_int as isize) as libc::c_int;
    let mut oe_6: libc::c_int = 1 as libc::c_int + 3 as libc::c_int * 16 as libc::c_int;
    let mut od_6: libc::c_int = 1 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int;
    *level.offset(7 as libc::c_int as isize) = (*p_src.offset(oe_6 as isize) as libc::c_int
        - *p_dst.offset(od_6 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(7 as libc::c_int as isize) as libc::c_int;
    let mut oe_7: libc::c_int = 2 as libc::c_int + 0 as libc::c_int * 16 as libc::c_int;
    let mut od_7: libc::c_int = 2 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int;
    *level.offset(8 as libc::c_int as isize) = (*p_src.offset(oe_7 as isize) as libc::c_int
        - *p_dst.offset(od_7 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(8 as libc::c_int as isize) as libc::c_int;
    let mut oe_8: libc::c_int = 2 as libc::c_int + 1 as libc::c_int * 16 as libc::c_int;
    let mut od_8: libc::c_int = 2 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int;
    *level.offset(9 as libc::c_int as isize) = (*p_src.offset(oe_8 as isize) as libc::c_int
        - *p_dst.offset(od_8 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(9 as libc::c_int as isize) as libc::c_int;
    let mut oe_9: libc::c_int = 2 as libc::c_int + 2 as libc::c_int * 16 as libc::c_int;
    let mut od_9: libc::c_int = 2 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int;
    *level.offset(10 as libc::c_int as isize) = (*p_src.offset(oe_9 as isize) as libc::c_int
        - *p_dst.offset(od_9 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(10 as libc::c_int as isize) as libc::c_int;
    let mut oe_10: libc::c_int = 2 as libc::c_int + 3 as libc::c_int * 16 as libc::c_int;
    let mut od_10: libc::c_int = 2 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int;
    *level.offset(11 as libc::c_int as isize) = (*p_src.offset(oe_10 as isize) as libc::c_int
        - *p_dst.offset(od_10 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(11 as libc::c_int as isize) as libc::c_int;
    let mut oe_11: libc::c_int = 3 as libc::c_int + 0 as libc::c_int * 16 as libc::c_int;
    let mut od_11: libc::c_int = 3 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int;
    *level.offset(12 as libc::c_int as isize) = (*p_src.offset(oe_11 as isize) as libc::c_int
        - *p_dst.offset(od_11 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(12 as libc::c_int as isize) as libc::c_int;
    let mut oe_12: libc::c_int = 3 as libc::c_int + 1 as libc::c_int * 16 as libc::c_int;
    let mut od_12: libc::c_int = 3 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int;
    *level.offset(13 as libc::c_int as isize) = (*p_src.offset(oe_12 as isize) as libc::c_int
        - *p_dst.offset(od_12 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(13 as libc::c_int as isize) as libc::c_int;
    let mut oe_13: libc::c_int = 3 as libc::c_int + 2 as libc::c_int * 16 as libc::c_int;
    let mut od_13: libc::c_int = 3 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int;
    *level.offset(14 as libc::c_int as isize) = (*p_src.offset(oe_13 as isize) as libc::c_int
        - *p_dst.offset(od_13 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(14 as libc::c_int as isize) as libc::c_int;
    let mut oe_14: libc::c_int = 3 as libc::c_int + 3 as libc::c_int * 16 as libc::c_int;
    let mut od_14: libc::c_int = 3 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int;
    *level.offset(15 as libc::c_int as isize) = (*p_src.offset(oe_14 as isize) as libc::c_int
        - *p_dst.offset(od_14 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(15 as libc::c_int as isize) as libc::c_int;
    (*(p_dst.offset((0 as libc::c_int * 32 as libc::c_int) as isize) as *mut x264_union32_t)).i =
        (*(p_src.offset((0 as libc::c_int * 16 as libc::c_int) as isize) as *mut x264_union32_t)).i;
    (*(p_dst.offset((1 as libc::c_int * 32 as libc::c_int) as isize) as *mut x264_union32_t)).i =
        (*(p_src.offset((1 as libc::c_int * 16 as libc::c_int) as isize) as *mut x264_union32_t)).i;
    (*(p_dst.offset((2 as libc::c_int * 32 as libc::c_int) as isize) as *mut x264_union32_t)).i =
        (*(p_src.offset((2 as libc::c_int * 16 as libc::c_int) as isize) as *mut x264_union32_t)).i;
    (*(p_dst.offset((3 as libc::c_int * 32 as libc::c_int) as isize) as *mut x264_union32_t)).i =
        (*(p_src.offset((3 as libc::c_int * 16 as libc::c_int) as isize) as *mut x264_union32_t)).i;
    (nz != 0) as libc::c_int
}
unsafe extern "C" fn zigzag_sub_8x8_frame(
    mut level: *mut dctcoef,
    mut p_src: *const pixel,
    mut p_dst: *mut pixel,
) -> libc::c_int {
    let mut nz: libc::c_int = 0 as libc::c_int;
    let mut oe: libc::c_int = 0 as libc::c_int + 0 as libc::c_int * 16 as libc::c_int;
    let mut od: libc::c_int = 0 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int;
    *level.offset(0 as libc::c_int as isize) = (*p_src.offset(oe as isize) as libc::c_int
        - *p_dst.offset(od as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut oe_0: libc::c_int = 1 as libc::c_int + 0 as libc::c_int * 16 as libc::c_int;
    let mut od_0: libc::c_int = 1 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int;
    *level.offset(1 as libc::c_int as isize) = (*p_src.offset(oe_0 as isize) as libc::c_int
        - *p_dst.offset(od_0 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(1 as libc::c_int as isize) as libc::c_int;
    let mut oe_1: libc::c_int = 0 as libc::c_int + 1 as libc::c_int * 16 as libc::c_int;
    let mut od_1: libc::c_int = 0 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int;
    *level.offset(2 as libc::c_int as isize) = (*p_src.offset(oe_1 as isize) as libc::c_int
        - *p_dst.offset(od_1 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(2 as libc::c_int as isize) as libc::c_int;
    let mut oe_2: libc::c_int = 0 as libc::c_int + 2 as libc::c_int * 16 as libc::c_int;
    let mut od_2: libc::c_int = 0 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int;
    *level.offset(3 as libc::c_int as isize) = (*p_src.offset(oe_2 as isize) as libc::c_int
        - *p_dst.offset(od_2 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(3 as libc::c_int as isize) as libc::c_int;
    let mut oe_3: libc::c_int = 1 as libc::c_int + 1 as libc::c_int * 16 as libc::c_int;
    let mut od_3: libc::c_int = 1 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int;
    *level.offset(4 as libc::c_int as isize) = (*p_src.offset(oe_3 as isize) as libc::c_int
        - *p_dst.offset(od_3 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(4 as libc::c_int as isize) as libc::c_int;
    let mut oe_4: libc::c_int = 2 as libc::c_int + 0 as libc::c_int * 16 as libc::c_int;
    let mut od_4: libc::c_int = 2 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int;
    *level.offset(5 as libc::c_int as isize) = (*p_src.offset(oe_4 as isize) as libc::c_int
        - *p_dst.offset(od_4 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(5 as libc::c_int as isize) as libc::c_int;
    let mut oe_5: libc::c_int = 3 as libc::c_int + 0 as libc::c_int * 16 as libc::c_int;
    let mut od_5: libc::c_int = 3 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int;
    *level.offset(6 as libc::c_int as isize) = (*p_src.offset(oe_5 as isize) as libc::c_int
        - *p_dst.offset(od_5 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(6 as libc::c_int as isize) as libc::c_int;
    let mut oe_6: libc::c_int = 2 as libc::c_int + 1 as libc::c_int * 16 as libc::c_int;
    let mut od_6: libc::c_int = 2 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int;
    *level.offset(7 as libc::c_int as isize) = (*p_src.offset(oe_6 as isize) as libc::c_int
        - *p_dst.offset(od_6 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(7 as libc::c_int as isize) as libc::c_int;
    let mut oe_7: libc::c_int = 1 as libc::c_int + 2 as libc::c_int * 16 as libc::c_int;
    let mut od_7: libc::c_int = 1 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int;
    *level.offset(8 as libc::c_int as isize) = (*p_src.offset(oe_7 as isize) as libc::c_int
        - *p_dst.offset(od_7 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(8 as libc::c_int as isize) as libc::c_int;
    let mut oe_8: libc::c_int = 0 as libc::c_int + 3 as libc::c_int * 16 as libc::c_int;
    let mut od_8: libc::c_int = 0 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int;
    *level.offset(9 as libc::c_int as isize) = (*p_src.offset(oe_8 as isize) as libc::c_int
        - *p_dst.offset(od_8 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(9 as libc::c_int as isize) as libc::c_int;
    let mut oe_9: libc::c_int = 0 as libc::c_int + 4 as libc::c_int * 16 as libc::c_int;
    let mut od_9: libc::c_int = 0 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int;
    *level.offset(10 as libc::c_int as isize) = (*p_src.offset(oe_9 as isize) as libc::c_int
        - *p_dst.offset(od_9 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(10 as libc::c_int as isize) as libc::c_int;
    let mut oe_10: libc::c_int = 1 as libc::c_int + 3 as libc::c_int * 16 as libc::c_int;
    let mut od_10: libc::c_int = 1 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int;
    *level.offset(11 as libc::c_int as isize) = (*p_src.offset(oe_10 as isize) as libc::c_int
        - *p_dst.offset(od_10 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(11 as libc::c_int as isize) as libc::c_int;
    let mut oe_11: libc::c_int = 2 as libc::c_int + 2 as libc::c_int * 16 as libc::c_int;
    let mut od_11: libc::c_int = 2 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int;
    *level.offset(12 as libc::c_int as isize) = (*p_src.offset(oe_11 as isize) as libc::c_int
        - *p_dst.offset(od_11 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(12 as libc::c_int as isize) as libc::c_int;
    let mut oe_12: libc::c_int = 3 as libc::c_int + 1 as libc::c_int * 16 as libc::c_int;
    let mut od_12: libc::c_int = 3 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int;
    *level.offset(13 as libc::c_int as isize) = (*p_src.offset(oe_12 as isize) as libc::c_int
        - *p_dst.offset(od_12 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(13 as libc::c_int as isize) as libc::c_int;
    let mut oe_13: libc::c_int = 4 as libc::c_int + 0 as libc::c_int * 16 as libc::c_int;
    let mut od_13: libc::c_int = 4 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int;
    *level.offset(14 as libc::c_int as isize) = (*p_src.offset(oe_13 as isize) as libc::c_int
        - *p_dst.offset(od_13 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(14 as libc::c_int as isize) as libc::c_int;
    let mut oe_14: libc::c_int = 5 as libc::c_int + 0 as libc::c_int * 16 as libc::c_int;
    let mut od_14: libc::c_int = 5 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int;
    *level.offset(15 as libc::c_int as isize) = (*p_src.offset(oe_14 as isize) as libc::c_int
        - *p_dst.offset(od_14 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(15 as libc::c_int as isize) as libc::c_int;
    let mut oe_15: libc::c_int = 4 as libc::c_int + 1 as libc::c_int * 16 as libc::c_int;
    let mut od_15: libc::c_int = 4 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int;
    *level.offset(16 as libc::c_int as isize) = (*p_src.offset(oe_15 as isize) as libc::c_int
        - *p_dst.offset(od_15 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(16 as libc::c_int as isize) as libc::c_int;
    let mut oe_16: libc::c_int = 3 as libc::c_int + 2 as libc::c_int * 16 as libc::c_int;
    let mut od_16: libc::c_int = 3 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int;
    *level.offset(17 as libc::c_int as isize) = (*p_src.offset(oe_16 as isize) as libc::c_int
        - *p_dst.offset(od_16 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(17 as libc::c_int as isize) as libc::c_int;
    let mut oe_17: libc::c_int = 2 as libc::c_int + 3 as libc::c_int * 16 as libc::c_int;
    let mut od_17: libc::c_int = 2 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int;
    *level.offset(18 as libc::c_int as isize) = (*p_src.offset(oe_17 as isize) as libc::c_int
        - *p_dst.offset(od_17 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(18 as libc::c_int as isize) as libc::c_int;
    let mut oe_18: libc::c_int = 1 as libc::c_int + 4 as libc::c_int * 16 as libc::c_int;
    let mut od_18: libc::c_int = 1 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int;
    *level.offset(19 as libc::c_int as isize) = (*p_src.offset(oe_18 as isize) as libc::c_int
        - *p_dst.offset(od_18 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(19 as libc::c_int as isize) as libc::c_int;
    let mut oe_19: libc::c_int = 0 as libc::c_int + 5 as libc::c_int * 16 as libc::c_int;
    let mut od_19: libc::c_int = 0 as libc::c_int + 5 as libc::c_int * 32 as libc::c_int;
    *level.offset(20 as libc::c_int as isize) = (*p_src.offset(oe_19 as isize) as libc::c_int
        - *p_dst.offset(od_19 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(20 as libc::c_int as isize) as libc::c_int;
    let mut oe_20: libc::c_int = 0 as libc::c_int + 6 as libc::c_int * 16 as libc::c_int;
    let mut od_20: libc::c_int = 0 as libc::c_int + 6 as libc::c_int * 32 as libc::c_int;
    *level.offset(21 as libc::c_int as isize) = (*p_src.offset(oe_20 as isize) as libc::c_int
        - *p_dst.offset(od_20 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(21 as libc::c_int as isize) as libc::c_int;
    let mut oe_21: libc::c_int = 1 as libc::c_int + 5 as libc::c_int * 16 as libc::c_int;
    let mut od_21: libc::c_int = 1 as libc::c_int + 5 as libc::c_int * 32 as libc::c_int;
    *level.offset(22 as libc::c_int as isize) = (*p_src.offset(oe_21 as isize) as libc::c_int
        - *p_dst.offset(od_21 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(22 as libc::c_int as isize) as libc::c_int;
    let mut oe_22: libc::c_int = 2 as libc::c_int + 4 as libc::c_int * 16 as libc::c_int;
    let mut od_22: libc::c_int = 2 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int;
    *level.offset(23 as libc::c_int as isize) = (*p_src.offset(oe_22 as isize) as libc::c_int
        - *p_dst.offset(od_22 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(23 as libc::c_int as isize) as libc::c_int;
    let mut oe_23: libc::c_int = 3 as libc::c_int + 3 as libc::c_int * 16 as libc::c_int;
    let mut od_23: libc::c_int = 3 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int;
    *level.offset(24 as libc::c_int as isize) = (*p_src.offset(oe_23 as isize) as libc::c_int
        - *p_dst.offset(od_23 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(24 as libc::c_int as isize) as libc::c_int;
    let mut oe_24: libc::c_int = 4 as libc::c_int + 2 as libc::c_int * 16 as libc::c_int;
    let mut od_24: libc::c_int = 4 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int;
    *level.offset(25 as libc::c_int as isize) = (*p_src.offset(oe_24 as isize) as libc::c_int
        - *p_dst.offset(od_24 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(25 as libc::c_int as isize) as libc::c_int;
    let mut oe_25: libc::c_int = 5 as libc::c_int + 1 as libc::c_int * 16 as libc::c_int;
    let mut od_25: libc::c_int = 5 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int;
    *level.offset(26 as libc::c_int as isize) = (*p_src.offset(oe_25 as isize) as libc::c_int
        - *p_dst.offset(od_25 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(26 as libc::c_int as isize) as libc::c_int;
    let mut oe_26: libc::c_int = 6 as libc::c_int + 0 as libc::c_int * 16 as libc::c_int;
    let mut od_26: libc::c_int = 6 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int;
    *level.offset(27 as libc::c_int as isize) = (*p_src.offset(oe_26 as isize) as libc::c_int
        - *p_dst.offset(od_26 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(27 as libc::c_int as isize) as libc::c_int;
    let mut oe_27: libc::c_int = 7 as libc::c_int + 0 as libc::c_int * 16 as libc::c_int;
    let mut od_27: libc::c_int = 7 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int;
    *level.offset(28 as libc::c_int as isize) = (*p_src.offset(oe_27 as isize) as libc::c_int
        - *p_dst.offset(od_27 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(28 as libc::c_int as isize) as libc::c_int;
    let mut oe_28: libc::c_int = 6 as libc::c_int + 1 as libc::c_int * 16 as libc::c_int;
    let mut od_28: libc::c_int = 6 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int;
    *level.offset(29 as libc::c_int as isize) = (*p_src.offset(oe_28 as isize) as libc::c_int
        - *p_dst.offset(od_28 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(29 as libc::c_int as isize) as libc::c_int;
    let mut oe_29: libc::c_int = 5 as libc::c_int + 2 as libc::c_int * 16 as libc::c_int;
    let mut od_29: libc::c_int = 5 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int;
    *level.offset(30 as libc::c_int as isize) = (*p_src.offset(oe_29 as isize) as libc::c_int
        - *p_dst.offset(od_29 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(30 as libc::c_int as isize) as libc::c_int;
    let mut oe_30: libc::c_int = 4 as libc::c_int + 3 as libc::c_int * 16 as libc::c_int;
    let mut od_30: libc::c_int = 4 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int;
    *level.offset(31 as libc::c_int as isize) = (*p_src.offset(oe_30 as isize) as libc::c_int
        - *p_dst.offset(od_30 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(31 as libc::c_int as isize) as libc::c_int;
    let mut oe_31: libc::c_int = 3 as libc::c_int + 4 as libc::c_int * 16 as libc::c_int;
    let mut od_31: libc::c_int = 3 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int;
    *level.offset(32 as libc::c_int as isize) = (*p_src.offset(oe_31 as isize) as libc::c_int
        - *p_dst.offset(od_31 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(32 as libc::c_int as isize) as libc::c_int;
    let mut oe_32: libc::c_int = 2 as libc::c_int + 5 as libc::c_int * 16 as libc::c_int;
    let mut od_32: libc::c_int = 2 as libc::c_int + 5 as libc::c_int * 32 as libc::c_int;
    *level.offset(33 as libc::c_int as isize) = (*p_src.offset(oe_32 as isize) as libc::c_int
        - *p_dst.offset(od_32 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(33 as libc::c_int as isize) as libc::c_int;
    let mut oe_33: libc::c_int = 1 as libc::c_int + 6 as libc::c_int * 16 as libc::c_int;
    let mut od_33: libc::c_int = 1 as libc::c_int + 6 as libc::c_int * 32 as libc::c_int;
    *level.offset(34 as libc::c_int as isize) = (*p_src.offset(oe_33 as isize) as libc::c_int
        - *p_dst.offset(od_33 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(34 as libc::c_int as isize) as libc::c_int;
    let mut oe_34: libc::c_int = 0 as libc::c_int + 7 as libc::c_int * 16 as libc::c_int;
    let mut od_34: libc::c_int = 0 as libc::c_int + 7 as libc::c_int * 32 as libc::c_int;
    *level.offset(35 as libc::c_int as isize) = (*p_src.offset(oe_34 as isize) as libc::c_int
        - *p_dst.offset(od_34 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(35 as libc::c_int as isize) as libc::c_int;
    let mut oe_35: libc::c_int = 1 as libc::c_int + 7 as libc::c_int * 16 as libc::c_int;
    let mut od_35: libc::c_int = 1 as libc::c_int + 7 as libc::c_int * 32 as libc::c_int;
    *level.offset(36 as libc::c_int as isize) = (*p_src.offset(oe_35 as isize) as libc::c_int
        - *p_dst.offset(od_35 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(36 as libc::c_int as isize) as libc::c_int;
    let mut oe_36: libc::c_int = 2 as libc::c_int + 6 as libc::c_int * 16 as libc::c_int;
    let mut od_36: libc::c_int = 2 as libc::c_int + 6 as libc::c_int * 32 as libc::c_int;
    *level.offset(37 as libc::c_int as isize) = (*p_src.offset(oe_36 as isize) as libc::c_int
        - *p_dst.offset(od_36 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(37 as libc::c_int as isize) as libc::c_int;
    let mut oe_37: libc::c_int = 3 as libc::c_int + 5 as libc::c_int * 16 as libc::c_int;
    let mut od_37: libc::c_int = 3 as libc::c_int + 5 as libc::c_int * 32 as libc::c_int;
    *level.offset(38 as libc::c_int as isize) = (*p_src.offset(oe_37 as isize) as libc::c_int
        - *p_dst.offset(od_37 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(38 as libc::c_int as isize) as libc::c_int;
    let mut oe_38: libc::c_int = 4 as libc::c_int + 4 as libc::c_int * 16 as libc::c_int;
    let mut od_38: libc::c_int = 4 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int;
    *level.offset(39 as libc::c_int as isize) = (*p_src.offset(oe_38 as isize) as libc::c_int
        - *p_dst.offset(od_38 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(39 as libc::c_int as isize) as libc::c_int;
    let mut oe_39: libc::c_int = 5 as libc::c_int + 3 as libc::c_int * 16 as libc::c_int;
    let mut od_39: libc::c_int = 5 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int;
    *level.offset(40 as libc::c_int as isize) = (*p_src.offset(oe_39 as isize) as libc::c_int
        - *p_dst.offset(od_39 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(40 as libc::c_int as isize) as libc::c_int;
    let mut oe_40: libc::c_int = 6 as libc::c_int + 2 as libc::c_int * 16 as libc::c_int;
    let mut od_40: libc::c_int = 6 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int;
    *level.offset(41 as libc::c_int as isize) = (*p_src.offset(oe_40 as isize) as libc::c_int
        - *p_dst.offset(od_40 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(41 as libc::c_int as isize) as libc::c_int;
    let mut oe_41: libc::c_int = 7 as libc::c_int + 1 as libc::c_int * 16 as libc::c_int;
    let mut od_41: libc::c_int = 7 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int;
    *level.offset(42 as libc::c_int as isize) = (*p_src.offset(oe_41 as isize) as libc::c_int
        - *p_dst.offset(od_41 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(42 as libc::c_int as isize) as libc::c_int;
    let mut oe_42: libc::c_int = 7 as libc::c_int + 2 as libc::c_int * 16 as libc::c_int;
    let mut od_42: libc::c_int = 7 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int;
    *level.offset(43 as libc::c_int as isize) = (*p_src.offset(oe_42 as isize) as libc::c_int
        - *p_dst.offset(od_42 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(43 as libc::c_int as isize) as libc::c_int;
    let mut oe_43: libc::c_int = 6 as libc::c_int + 3 as libc::c_int * 16 as libc::c_int;
    let mut od_43: libc::c_int = 6 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int;
    *level.offset(44 as libc::c_int as isize) = (*p_src.offset(oe_43 as isize) as libc::c_int
        - *p_dst.offset(od_43 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(44 as libc::c_int as isize) as libc::c_int;
    let mut oe_44: libc::c_int = 5 as libc::c_int + 4 as libc::c_int * 16 as libc::c_int;
    let mut od_44: libc::c_int = 5 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int;
    *level.offset(45 as libc::c_int as isize) = (*p_src.offset(oe_44 as isize) as libc::c_int
        - *p_dst.offset(od_44 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(45 as libc::c_int as isize) as libc::c_int;
    let mut oe_45: libc::c_int = 4 as libc::c_int + 5 as libc::c_int * 16 as libc::c_int;
    let mut od_45: libc::c_int = 4 as libc::c_int + 5 as libc::c_int * 32 as libc::c_int;
    *level.offset(46 as libc::c_int as isize) = (*p_src.offset(oe_45 as isize) as libc::c_int
        - *p_dst.offset(od_45 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(46 as libc::c_int as isize) as libc::c_int;
    let mut oe_46: libc::c_int = 3 as libc::c_int + 6 as libc::c_int * 16 as libc::c_int;
    let mut od_46: libc::c_int = 3 as libc::c_int + 6 as libc::c_int * 32 as libc::c_int;
    *level.offset(47 as libc::c_int as isize) = (*p_src.offset(oe_46 as isize) as libc::c_int
        - *p_dst.offset(od_46 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(47 as libc::c_int as isize) as libc::c_int;
    let mut oe_47: libc::c_int = 2 as libc::c_int + 7 as libc::c_int * 16 as libc::c_int;
    let mut od_47: libc::c_int = 2 as libc::c_int + 7 as libc::c_int * 32 as libc::c_int;
    *level.offset(48 as libc::c_int as isize) = (*p_src.offset(oe_47 as isize) as libc::c_int
        - *p_dst.offset(od_47 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(48 as libc::c_int as isize) as libc::c_int;
    let mut oe_48: libc::c_int = 3 as libc::c_int + 7 as libc::c_int * 16 as libc::c_int;
    let mut od_48: libc::c_int = 3 as libc::c_int + 7 as libc::c_int * 32 as libc::c_int;
    *level.offset(49 as libc::c_int as isize) = (*p_src.offset(oe_48 as isize) as libc::c_int
        - *p_dst.offset(od_48 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(49 as libc::c_int as isize) as libc::c_int;
    let mut oe_49: libc::c_int = 4 as libc::c_int + 6 as libc::c_int * 16 as libc::c_int;
    let mut od_49: libc::c_int = 4 as libc::c_int + 6 as libc::c_int * 32 as libc::c_int;
    *level.offset(50 as libc::c_int as isize) = (*p_src.offset(oe_49 as isize) as libc::c_int
        - *p_dst.offset(od_49 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(50 as libc::c_int as isize) as libc::c_int;
    let mut oe_50: libc::c_int = 5 as libc::c_int + 5 as libc::c_int * 16 as libc::c_int;
    let mut od_50: libc::c_int = 5 as libc::c_int + 5 as libc::c_int * 32 as libc::c_int;
    *level.offset(51 as libc::c_int as isize) = (*p_src.offset(oe_50 as isize) as libc::c_int
        - *p_dst.offset(od_50 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(51 as libc::c_int as isize) as libc::c_int;
    let mut oe_51: libc::c_int = 6 as libc::c_int + 4 as libc::c_int * 16 as libc::c_int;
    let mut od_51: libc::c_int = 6 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int;
    *level.offset(52 as libc::c_int as isize) = (*p_src.offset(oe_51 as isize) as libc::c_int
        - *p_dst.offset(od_51 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(52 as libc::c_int as isize) as libc::c_int;
    let mut oe_52: libc::c_int = 7 as libc::c_int + 3 as libc::c_int * 16 as libc::c_int;
    let mut od_52: libc::c_int = 7 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int;
    *level.offset(53 as libc::c_int as isize) = (*p_src.offset(oe_52 as isize) as libc::c_int
        - *p_dst.offset(od_52 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(53 as libc::c_int as isize) as libc::c_int;
    let mut oe_53: libc::c_int = 7 as libc::c_int + 4 as libc::c_int * 16 as libc::c_int;
    let mut od_53: libc::c_int = 7 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int;
    *level.offset(54 as libc::c_int as isize) = (*p_src.offset(oe_53 as isize) as libc::c_int
        - *p_dst.offset(od_53 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(54 as libc::c_int as isize) as libc::c_int;
    let mut oe_54: libc::c_int = 6 as libc::c_int + 5 as libc::c_int * 16 as libc::c_int;
    let mut od_54: libc::c_int = 6 as libc::c_int + 5 as libc::c_int * 32 as libc::c_int;
    *level.offset(55 as libc::c_int as isize) = (*p_src.offset(oe_54 as isize) as libc::c_int
        - *p_dst.offset(od_54 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(55 as libc::c_int as isize) as libc::c_int;
    let mut oe_55: libc::c_int = 5 as libc::c_int + 6 as libc::c_int * 16 as libc::c_int;
    let mut od_55: libc::c_int = 5 as libc::c_int + 6 as libc::c_int * 32 as libc::c_int;
    *level.offset(56 as libc::c_int as isize) = (*p_src.offset(oe_55 as isize) as libc::c_int
        - *p_dst.offset(od_55 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(56 as libc::c_int as isize) as libc::c_int;
    let mut oe_56: libc::c_int = 4 as libc::c_int + 7 as libc::c_int * 16 as libc::c_int;
    let mut od_56: libc::c_int = 4 as libc::c_int + 7 as libc::c_int * 32 as libc::c_int;
    *level.offset(57 as libc::c_int as isize) = (*p_src.offset(oe_56 as isize) as libc::c_int
        - *p_dst.offset(od_56 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(57 as libc::c_int as isize) as libc::c_int;
    let mut oe_57: libc::c_int = 5 as libc::c_int + 7 as libc::c_int * 16 as libc::c_int;
    let mut od_57: libc::c_int = 5 as libc::c_int + 7 as libc::c_int * 32 as libc::c_int;
    *level.offset(58 as libc::c_int as isize) = (*p_src.offset(oe_57 as isize) as libc::c_int
        - *p_dst.offset(od_57 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(58 as libc::c_int as isize) as libc::c_int;
    let mut oe_58: libc::c_int = 6 as libc::c_int + 6 as libc::c_int * 16 as libc::c_int;
    let mut od_58: libc::c_int = 6 as libc::c_int + 6 as libc::c_int * 32 as libc::c_int;
    *level.offset(59 as libc::c_int as isize) = (*p_src.offset(oe_58 as isize) as libc::c_int
        - *p_dst.offset(od_58 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(59 as libc::c_int as isize) as libc::c_int;
    let mut oe_59: libc::c_int = 7 as libc::c_int + 5 as libc::c_int * 16 as libc::c_int;
    let mut od_59: libc::c_int = 7 as libc::c_int + 5 as libc::c_int * 32 as libc::c_int;
    *level.offset(60 as libc::c_int as isize) = (*p_src.offset(oe_59 as isize) as libc::c_int
        - *p_dst.offset(od_59 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(60 as libc::c_int as isize) as libc::c_int;
    let mut oe_60: libc::c_int = 7 as libc::c_int + 6 as libc::c_int * 16 as libc::c_int;
    let mut od_60: libc::c_int = 7 as libc::c_int + 6 as libc::c_int * 32 as libc::c_int;
    *level.offset(61 as libc::c_int as isize) = (*p_src.offset(oe_60 as isize) as libc::c_int
        - *p_dst.offset(od_60 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(61 as libc::c_int as isize) as libc::c_int;
    let mut oe_61: libc::c_int = 6 as libc::c_int + 7 as libc::c_int * 16 as libc::c_int;
    let mut od_61: libc::c_int = 6 as libc::c_int + 7 as libc::c_int * 32 as libc::c_int;
    *level.offset(62 as libc::c_int as isize) = (*p_src.offset(oe_61 as isize) as libc::c_int
        - *p_dst.offset(od_61 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(62 as libc::c_int as isize) as libc::c_int;
    let mut oe_62: libc::c_int = 7 as libc::c_int + 7 as libc::c_int * 16 as libc::c_int;
    let mut od_62: libc::c_int = 7 as libc::c_int + 7 as libc::c_int * 32 as libc::c_int;
    *level.offset(63 as libc::c_int as isize) = (*p_src.offset(oe_62 as isize) as libc::c_int
        - *p_dst.offset(od_62 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(63 as libc::c_int as isize) as libc::c_int;
    (*(p_dst.offset((0 as libc::c_int * 32 as libc::c_int) as isize) as *mut x264_union32_t)).i =
        (*(p_src.offset((0 as libc::c_int * 16 as libc::c_int) as isize) as *mut x264_union32_t)).i;
    (*(p_dst
        .offset((0 as libc::c_int * 32 as libc::c_int) as isize)
        .offset(4 as libc::c_int as isize) as *mut x264_union32_t))
        .i = (*(p_src
        .offset((0 as libc::c_int * 16 as libc::c_int) as isize)
        .offset(4 as libc::c_int as isize) as *mut x264_union32_t))
        .i;
    (*(p_dst.offset((1 as libc::c_int * 32 as libc::c_int) as isize) as *mut x264_union32_t)).i =
        (*(p_src.offset((1 as libc::c_int * 16 as libc::c_int) as isize) as *mut x264_union32_t)).i;
    (*(p_dst
        .offset((1 as libc::c_int * 32 as libc::c_int) as isize)
        .offset(4 as libc::c_int as isize) as *mut x264_union32_t))
        .i = (*(p_src
        .offset((1 as libc::c_int * 16 as libc::c_int) as isize)
        .offset(4 as libc::c_int as isize) as *mut x264_union32_t))
        .i;
    (*(p_dst.offset((2 as libc::c_int * 32 as libc::c_int) as isize) as *mut x264_union32_t)).i =
        (*(p_src.offset((2 as libc::c_int * 16 as libc::c_int) as isize) as *mut x264_union32_t)).i;
    (*(p_dst
        .offset((2 as libc::c_int * 32 as libc::c_int) as isize)
        .offset(4 as libc::c_int as isize) as *mut x264_union32_t))
        .i = (*(p_src
        .offset((2 as libc::c_int * 16 as libc::c_int) as isize)
        .offset(4 as libc::c_int as isize) as *mut x264_union32_t))
        .i;
    (*(p_dst.offset((3 as libc::c_int * 32 as libc::c_int) as isize) as *mut x264_union32_t)).i =
        (*(p_src.offset((3 as libc::c_int * 16 as libc::c_int) as isize) as *mut x264_union32_t)).i;
    (*(p_dst
        .offset((3 as libc::c_int * 32 as libc::c_int) as isize)
        .offset(4 as libc::c_int as isize) as *mut x264_union32_t))
        .i = (*(p_src
        .offset((3 as libc::c_int * 16 as libc::c_int) as isize)
        .offset(4 as libc::c_int as isize) as *mut x264_union32_t))
        .i;
    (*(p_dst.offset((4 as libc::c_int * 32 as libc::c_int) as isize) as *mut x264_union32_t)).i =
        (*(p_src.offset((4 as libc::c_int * 16 as libc::c_int) as isize) as *mut x264_union32_t)).i;
    (*(p_dst
        .offset((4 as libc::c_int * 32 as libc::c_int) as isize)
        .offset(4 as libc::c_int as isize) as *mut x264_union32_t))
        .i = (*(p_src
        .offset((4 as libc::c_int * 16 as libc::c_int) as isize)
        .offset(4 as libc::c_int as isize) as *mut x264_union32_t))
        .i;
    (*(p_dst.offset((5 as libc::c_int * 32 as libc::c_int) as isize) as *mut x264_union32_t)).i =
        (*(p_src.offset((5 as libc::c_int * 16 as libc::c_int) as isize) as *mut x264_union32_t)).i;
    (*(p_dst
        .offset((5 as libc::c_int * 32 as libc::c_int) as isize)
        .offset(4 as libc::c_int as isize) as *mut x264_union32_t))
        .i = (*(p_src
        .offset((5 as libc::c_int * 16 as libc::c_int) as isize)
        .offset(4 as libc::c_int as isize) as *mut x264_union32_t))
        .i;
    (*(p_dst.offset((6 as libc::c_int * 32 as libc::c_int) as isize) as *mut x264_union32_t)).i =
        (*(p_src.offset((6 as libc::c_int * 16 as libc::c_int) as isize) as *mut x264_union32_t)).i;
    (*(p_dst
        .offset((6 as libc::c_int * 32 as libc::c_int) as isize)
        .offset(4 as libc::c_int as isize) as *mut x264_union32_t))
        .i = (*(p_src
        .offset((6 as libc::c_int * 16 as libc::c_int) as isize)
        .offset(4 as libc::c_int as isize) as *mut x264_union32_t))
        .i;
    (*(p_dst.offset((7 as libc::c_int * 32 as libc::c_int) as isize) as *mut x264_union32_t)).i =
        (*(p_src.offset((7 as libc::c_int * 16 as libc::c_int) as isize) as *mut x264_union32_t)).i;
    (*(p_dst
        .offset((7 as libc::c_int * 32 as libc::c_int) as isize)
        .offset(4 as libc::c_int as isize) as *mut x264_union32_t))
        .i = (*(p_src
        .offset((7 as libc::c_int * 16 as libc::c_int) as isize)
        .offset(4 as libc::c_int as isize) as *mut x264_union32_t))
        .i;
    (nz != 0) as libc::c_int
}
unsafe extern "C" fn zigzag_sub_8x8_field(
    mut level: *mut dctcoef,
    mut p_src: *const pixel,
    mut p_dst: *mut pixel,
) -> libc::c_int {
    let mut nz: libc::c_int = 0 as libc::c_int;
    let mut oe: libc::c_int = 0 as libc::c_int + 0 as libc::c_int * 16 as libc::c_int;
    let mut od: libc::c_int = 0 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int;
    *level.offset(0 as libc::c_int as isize) = (*p_src.offset(oe as isize) as libc::c_int
        - *p_dst.offset(od as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(0 as libc::c_int as isize) as libc::c_int;
    let mut oe_0: libc::c_int = 0 as libc::c_int + 1 as libc::c_int * 16 as libc::c_int;
    let mut od_0: libc::c_int = 0 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int;
    *level.offset(1 as libc::c_int as isize) = (*p_src.offset(oe_0 as isize) as libc::c_int
        - *p_dst.offset(od_0 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(1 as libc::c_int as isize) as libc::c_int;
    let mut oe_1: libc::c_int = 0 as libc::c_int + 2 as libc::c_int * 16 as libc::c_int;
    let mut od_1: libc::c_int = 0 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int;
    *level.offset(2 as libc::c_int as isize) = (*p_src.offset(oe_1 as isize) as libc::c_int
        - *p_dst.offset(od_1 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(2 as libc::c_int as isize) as libc::c_int;
    let mut oe_2: libc::c_int = 1 as libc::c_int + 0 as libc::c_int * 16 as libc::c_int;
    let mut od_2: libc::c_int = 1 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int;
    *level.offset(3 as libc::c_int as isize) = (*p_src.offset(oe_2 as isize) as libc::c_int
        - *p_dst.offset(od_2 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(3 as libc::c_int as isize) as libc::c_int;
    let mut oe_3: libc::c_int = 1 as libc::c_int + 1 as libc::c_int * 16 as libc::c_int;
    let mut od_3: libc::c_int = 1 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int;
    *level.offset(4 as libc::c_int as isize) = (*p_src.offset(oe_3 as isize) as libc::c_int
        - *p_dst.offset(od_3 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(4 as libc::c_int as isize) as libc::c_int;
    let mut oe_4: libc::c_int = 0 as libc::c_int + 3 as libc::c_int * 16 as libc::c_int;
    let mut od_4: libc::c_int = 0 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int;
    *level.offset(5 as libc::c_int as isize) = (*p_src.offset(oe_4 as isize) as libc::c_int
        - *p_dst.offset(od_4 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(5 as libc::c_int as isize) as libc::c_int;
    let mut oe_5: libc::c_int = 0 as libc::c_int + 4 as libc::c_int * 16 as libc::c_int;
    let mut od_5: libc::c_int = 0 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int;
    *level.offset(6 as libc::c_int as isize) = (*p_src.offset(oe_5 as isize) as libc::c_int
        - *p_dst.offset(od_5 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(6 as libc::c_int as isize) as libc::c_int;
    let mut oe_6: libc::c_int = 1 as libc::c_int + 2 as libc::c_int * 16 as libc::c_int;
    let mut od_6: libc::c_int = 1 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int;
    *level.offset(7 as libc::c_int as isize) = (*p_src.offset(oe_6 as isize) as libc::c_int
        - *p_dst.offset(od_6 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(7 as libc::c_int as isize) as libc::c_int;
    let mut oe_7: libc::c_int = 2 as libc::c_int + 0 as libc::c_int * 16 as libc::c_int;
    let mut od_7: libc::c_int = 2 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int;
    *level.offset(8 as libc::c_int as isize) = (*p_src.offset(oe_7 as isize) as libc::c_int
        - *p_dst.offset(od_7 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(8 as libc::c_int as isize) as libc::c_int;
    let mut oe_8: libc::c_int = 1 as libc::c_int + 3 as libc::c_int * 16 as libc::c_int;
    let mut od_8: libc::c_int = 1 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int;
    *level.offset(9 as libc::c_int as isize) = (*p_src.offset(oe_8 as isize) as libc::c_int
        - *p_dst.offset(od_8 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(9 as libc::c_int as isize) as libc::c_int;
    let mut oe_9: libc::c_int = 0 as libc::c_int + 5 as libc::c_int * 16 as libc::c_int;
    let mut od_9: libc::c_int = 0 as libc::c_int + 5 as libc::c_int * 32 as libc::c_int;
    *level.offset(10 as libc::c_int as isize) = (*p_src.offset(oe_9 as isize) as libc::c_int
        - *p_dst.offset(od_9 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(10 as libc::c_int as isize) as libc::c_int;
    let mut oe_10: libc::c_int = 0 as libc::c_int + 6 as libc::c_int * 16 as libc::c_int;
    let mut od_10: libc::c_int = 0 as libc::c_int + 6 as libc::c_int * 32 as libc::c_int;
    *level.offset(11 as libc::c_int as isize) = (*p_src.offset(oe_10 as isize) as libc::c_int
        - *p_dst.offset(od_10 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(11 as libc::c_int as isize) as libc::c_int;
    let mut oe_11: libc::c_int = 0 as libc::c_int + 7 as libc::c_int * 16 as libc::c_int;
    let mut od_11: libc::c_int = 0 as libc::c_int + 7 as libc::c_int * 32 as libc::c_int;
    *level.offset(12 as libc::c_int as isize) = (*p_src.offset(oe_11 as isize) as libc::c_int
        - *p_dst.offset(od_11 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(12 as libc::c_int as isize) as libc::c_int;
    let mut oe_12: libc::c_int = 1 as libc::c_int + 4 as libc::c_int * 16 as libc::c_int;
    let mut od_12: libc::c_int = 1 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int;
    *level.offset(13 as libc::c_int as isize) = (*p_src.offset(oe_12 as isize) as libc::c_int
        - *p_dst.offset(od_12 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(13 as libc::c_int as isize) as libc::c_int;
    let mut oe_13: libc::c_int = 2 as libc::c_int + 1 as libc::c_int * 16 as libc::c_int;
    let mut od_13: libc::c_int = 2 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int;
    *level.offset(14 as libc::c_int as isize) = (*p_src.offset(oe_13 as isize) as libc::c_int
        - *p_dst.offset(od_13 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(14 as libc::c_int as isize) as libc::c_int;
    let mut oe_14: libc::c_int = 3 as libc::c_int + 0 as libc::c_int * 16 as libc::c_int;
    let mut od_14: libc::c_int = 3 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int;
    *level.offset(15 as libc::c_int as isize) = (*p_src.offset(oe_14 as isize) as libc::c_int
        - *p_dst.offset(od_14 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(15 as libc::c_int as isize) as libc::c_int;
    let mut oe_15: libc::c_int = 2 as libc::c_int + 2 as libc::c_int * 16 as libc::c_int;
    let mut od_15: libc::c_int = 2 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int;
    *level.offset(16 as libc::c_int as isize) = (*p_src.offset(oe_15 as isize) as libc::c_int
        - *p_dst.offset(od_15 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(16 as libc::c_int as isize) as libc::c_int;
    let mut oe_16: libc::c_int = 1 as libc::c_int + 5 as libc::c_int * 16 as libc::c_int;
    let mut od_16: libc::c_int = 1 as libc::c_int + 5 as libc::c_int * 32 as libc::c_int;
    *level.offset(17 as libc::c_int as isize) = (*p_src.offset(oe_16 as isize) as libc::c_int
        - *p_dst.offset(od_16 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(17 as libc::c_int as isize) as libc::c_int;
    let mut oe_17: libc::c_int = 1 as libc::c_int + 6 as libc::c_int * 16 as libc::c_int;
    let mut od_17: libc::c_int = 1 as libc::c_int + 6 as libc::c_int * 32 as libc::c_int;
    *level.offset(18 as libc::c_int as isize) = (*p_src.offset(oe_17 as isize) as libc::c_int
        - *p_dst.offset(od_17 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(18 as libc::c_int as isize) as libc::c_int;
    let mut oe_18: libc::c_int = 1 as libc::c_int + 7 as libc::c_int * 16 as libc::c_int;
    let mut od_18: libc::c_int = 1 as libc::c_int + 7 as libc::c_int * 32 as libc::c_int;
    *level.offset(19 as libc::c_int as isize) = (*p_src.offset(oe_18 as isize) as libc::c_int
        - *p_dst.offset(od_18 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(19 as libc::c_int as isize) as libc::c_int;
    let mut oe_19: libc::c_int = 2 as libc::c_int + 3 as libc::c_int * 16 as libc::c_int;
    let mut od_19: libc::c_int = 2 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int;
    *level.offset(20 as libc::c_int as isize) = (*p_src.offset(oe_19 as isize) as libc::c_int
        - *p_dst.offset(od_19 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(20 as libc::c_int as isize) as libc::c_int;
    let mut oe_20: libc::c_int = 3 as libc::c_int + 1 as libc::c_int * 16 as libc::c_int;
    let mut od_20: libc::c_int = 3 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int;
    *level.offset(21 as libc::c_int as isize) = (*p_src.offset(oe_20 as isize) as libc::c_int
        - *p_dst.offset(od_20 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(21 as libc::c_int as isize) as libc::c_int;
    let mut oe_21: libc::c_int = 4 as libc::c_int + 0 as libc::c_int * 16 as libc::c_int;
    let mut od_21: libc::c_int = 4 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int;
    *level.offset(22 as libc::c_int as isize) = (*p_src.offset(oe_21 as isize) as libc::c_int
        - *p_dst.offset(od_21 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(22 as libc::c_int as isize) as libc::c_int;
    let mut oe_22: libc::c_int = 3 as libc::c_int + 2 as libc::c_int * 16 as libc::c_int;
    let mut od_22: libc::c_int = 3 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int;
    *level.offset(23 as libc::c_int as isize) = (*p_src.offset(oe_22 as isize) as libc::c_int
        - *p_dst.offset(od_22 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(23 as libc::c_int as isize) as libc::c_int;
    let mut oe_23: libc::c_int = 2 as libc::c_int + 4 as libc::c_int * 16 as libc::c_int;
    let mut od_23: libc::c_int = 2 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int;
    *level.offset(24 as libc::c_int as isize) = (*p_src.offset(oe_23 as isize) as libc::c_int
        - *p_dst.offset(od_23 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(24 as libc::c_int as isize) as libc::c_int;
    let mut oe_24: libc::c_int = 2 as libc::c_int + 5 as libc::c_int * 16 as libc::c_int;
    let mut od_24: libc::c_int = 2 as libc::c_int + 5 as libc::c_int * 32 as libc::c_int;
    *level.offset(25 as libc::c_int as isize) = (*p_src.offset(oe_24 as isize) as libc::c_int
        - *p_dst.offset(od_24 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(25 as libc::c_int as isize) as libc::c_int;
    let mut oe_25: libc::c_int = 2 as libc::c_int + 6 as libc::c_int * 16 as libc::c_int;
    let mut od_25: libc::c_int = 2 as libc::c_int + 6 as libc::c_int * 32 as libc::c_int;
    *level.offset(26 as libc::c_int as isize) = (*p_src.offset(oe_25 as isize) as libc::c_int
        - *p_dst.offset(od_25 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(26 as libc::c_int as isize) as libc::c_int;
    let mut oe_26: libc::c_int = 2 as libc::c_int + 7 as libc::c_int * 16 as libc::c_int;
    let mut od_26: libc::c_int = 2 as libc::c_int + 7 as libc::c_int * 32 as libc::c_int;
    *level.offset(27 as libc::c_int as isize) = (*p_src.offset(oe_26 as isize) as libc::c_int
        - *p_dst.offset(od_26 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(27 as libc::c_int as isize) as libc::c_int;
    let mut oe_27: libc::c_int = 3 as libc::c_int + 3 as libc::c_int * 16 as libc::c_int;
    let mut od_27: libc::c_int = 3 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int;
    *level.offset(28 as libc::c_int as isize) = (*p_src.offset(oe_27 as isize) as libc::c_int
        - *p_dst.offset(od_27 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(28 as libc::c_int as isize) as libc::c_int;
    let mut oe_28: libc::c_int = 4 as libc::c_int + 1 as libc::c_int * 16 as libc::c_int;
    let mut od_28: libc::c_int = 4 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int;
    *level.offset(29 as libc::c_int as isize) = (*p_src.offset(oe_28 as isize) as libc::c_int
        - *p_dst.offset(od_28 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(29 as libc::c_int as isize) as libc::c_int;
    let mut oe_29: libc::c_int = 5 as libc::c_int + 0 as libc::c_int * 16 as libc::c_int;
    let mut od_29: libc::c_int = 5 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int;
    *level.offset(30 as libc::c_int as isize) = (*p_src.offset(oe_29 as isize) as libc::c_int
        - *p_dst.offset(od_29 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(30 as libc::c_int as isize) as libc::c_int;
    let mut oe_30: libc::c_int = 4 as libc::c_int + 2 as libc::c_int * 16 as libc::c_int;
    let mut od_30: libc::c_int = 4 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int;
    *level.offset(31 as libc::c_int as isize) = (*p_src.offset(oe_30 as isize) as libc::c_int
        - *p_dst.offset(od_30 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(31 as libc::c_int as isize) as libc::c_int;
    let mut oe_31: libc::c_int = 3 as libc::c_int + 4 as libc::c_int * 16 as libc::c_int;
    let mut od_31: libc::c_int = 3 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int;
    *level.offset(32 as libc::c_int as isize) = (*p_src.offset(oe_31 as isize) as libc::c_int
        - *p_dst.offset(od_31 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(32 as libc::c_int as isize) as libc::c_int;
    let mut oe_32: libc::c_int = 3 as libc::c_int + 5 as libc::c_int * 16 as libc::c_int;
    let mut od_32: libc::c_int = 3 as libc::c_int + 5 as libc::c_int * 32 as libc::c_int;
    *level.offset(33 as libc::c_int as isize) = (*p_src.offset(oe_32 as isize) as libc::c_int
        - *p_dst.offset(od_32 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(33 as libc::c_int as isize) as libc::c_int;
    let mut oe_33: libc::c_int = 3 as libc::c_int + 6 as libc::c_int * 16 as libc::c_int;
    let mut od_33: libc::c_int = 3 as libc::c_int + 6 as libc::c_int * 32 as libc::c_int;
    *level.offset(34 as libc::c_int as isize) = (*p_src.offset(oe_33 as isize) as libc::c_int
        - *p_dst.offset(od_33 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(34 as libc::c_int as isize) as libc::c_int;
    let mut oe_34: libc::c_int = 3 as libc::c_int + 7 as libc::c_int * 16 as libc::c_int;
    let mut od_34: libc::c_int = 3 as libc::c_int + 7 as libc::c_int * 32 as libc::c_int;
    *level.offset(35 as libc::c_int as isize) = (*p_src.offset(oe_34 as isize) as libc::c_int
        - *p_dst.offset(od_34 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(35 as libc::c_int as isize) as libc::c_int;
    let mut oe_35: libc::c_int = 4 as libc::c_int + 3 as libc::c_int * 16 as libc::c_int;
    let mut od_35: libc::c_int = 4 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int;
    *level.offset(36 as libc::c_int as isize) = (*p_src.offset(oe_35 as isize) as libc::c_int
        - *p_dst.offset(od_35 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(36 as libc::c_int as isize) as libc::c_int;
    let mut oe_36: libc::c_int = 5 as libc::c_int + 1 as libc::c_int * 16 as libc::c_int;
    let mut od_36: libc::c_int = 5 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int;
    *level.offset(37 as libc::c_int as isize) = (*p_src.offset(oe_36 as isize) as libc::c_int
        - *p_dst.offset(od_36 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(37 as libc::c_int as isize) as libc::c_int;
    let mut oe_37: libc::c_int = 6 as libc::c_int + 0 as libc::c_int * 16 as libc::c_int;
    let mut od_37: libc::c_int = 6 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int;
    *level.offset(38 as libc::c_int as isize) = (*p_src.offset(oe_37 as isize) as libc::c_int
        - *p_dst.offset(od_37 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(38 as libc::c_int as isize) as libc::c_int;
    let mut oe_38: libc::c_int = 5 as libc::c_int + 2 as libc::c_int * 16 as libc::c_int;
    let mut od_38: libc::c_int = 5 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int;
    *level.offset(39 as libc::c_int as isize) = (*p_src.offset(oe_38 as isize) as libc::c_int
        - *p_dst.offset(od_38 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(39 as libc::c_int as isize) as libc::c_int;
    let mut oe_39: libc::c_int = 4 as libc::c_int + 4 as libc::c_int * 16 as libc::c_int;
    let mut od_39: libc::c_int = 4 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int;
    *level.offset(40 as libc::c_int as isize) = (*p_src.offset(oe_39 as isize) as libc::c_int
        - *p_dst.offset(od_39 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(40 as libc::c_int as isize) as libc::c_int;
    let mut oe_40: libc::c_int = 4 as libc::c_int + 5 as libc::c_int * 16 as libc::c_int;
    let mut od_40: libc::c_int = 4 as libc::c_int + 5 as libc::c_int * 32 as libc::c_int;
    *level.offset(41 as libc::c_int as isize) = (*p_src.offset(oe_40 as isize) as libc::c_int
        - *p_dst.offset(od_40 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(41 as libc::c_int as isize) as libc::c_int;
    let mut oe_41: libc::c_int = 4 as libc::c_int + 6 as libc::c_int * 16 as libc::c_int;
    let mut od_41: libc::c_int = 4 as libc::c_int + 6 as libc::c_int * 32 as libc::c_int;
    *level.offset(42 as libc::c_int as isize) = (*p_src.offset(oe_41 as isize) as libc::c_int
        - *p_dst.offset(od_41 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(42 as libc::c_int as isize) as libc::c_int;
    let mut oe_42: libc::c_int = 4 as libc::c_int + 7 as libc::c_int * 16 as libc::c_int;
    let mut od_42: libc::c_int = 4 as libc::c_int + 7 as libc::c_int * 32 as libc::c_int;
    *level.offset(43 as libc::c_int as isize) = (*p_src.offset(oe_42 as isize) as libc::c_int
        - *p_dst.offset(od_42 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(43 as libc::c_int as isize) as libc::c_int;
    let mut oe_43: libc::c_int = 5 as libc::c_int + 3 as libc::c_int * 16 as libc::c_int;
    let mut od_43: libc::c_int = 5 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int;
    *level.offset(44 as libc::c_int as isize) = (*p_src.offset(oe_43 as isize) as libc::c_int
        - *p_dst.offset(od_43 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(44 as libc::c_int as isize) as libc::c_int;
    let mut oe_44: libc::c_int = 6 as libc::c_int + 1 as libc::c_int * 16 as libc::c_int;
    let mut od_44: libc::c_int = 6 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int;
    *level.offset(45 as libc::c_int as isize) = (*p_src.offset(oe_44 as isize) as libc::c_int
        - *p_dst.offset(od_44 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(45 as libc::c_int as isize) as libc::c_int;
    let mut oe_45: libc::c_int = 6 as libc::c_int + 2 as libc::c_int * 16 as libc::c_int;
    let mut od_45: libc::c_int = 6 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int;
    *level.offset(46 as libc::c_int as isize) = (*p_src.offset(oe_45 as isize) as libc::c_int
        - *p_dst.offset(od_45 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(46 as libc::c_int as isize) as libc::c_int;
    let mut oe_46: libc::c_int = 5 as libc::c_int + 4 as libc::c_int * 16 as libc::c_int;
    let mut od_46: libc::c_int = 5 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int;
    *level.offset(47 as libc::c_int as isize) = (*p_src.offset(oe_46 as isize) as libc::c_int
        - *p_dst.offset(od_46 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(47 as libc::c_int as isize) as libc::c_int;
    let mut oe_47: libc::c_int = 5 as libc::c_int + 5 as libc::c_int * 16 as libc::c_int;
    let mut od_47: libc::c_int = 5 as libc::c_int + 5 as libc::c_int * 32 as libc::c_int;
    *level.offset(48 as libc::c_int as isize) = (*p_src.offset(oe_47 as isize) as libc::c_int
        - *p_dst.offset(od_47 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(48 as libc::c_int as isize) as libc::c_int;
    let mut oe_48: libc::c_int = 5 as libc::c_int + 6 as libc::c_int * 16 as libc::c_int;
    let mut od_48: libc::c_int = 5 as libc::c_int + 6 as libc::c_int * 32 as libc::c_int;
    *level.offset(49 as libc::c_int as isize) = (*p_src.offset(oe_48 as isize) as libc::c_int
        - *p_dst.offset(od_48 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(49 as libc::c_int as isize) as libc::c_int;
    let mut oe_49: libc::c_int = 5 as libc::c_int + 7 as libc::c_int * 16 as libc::c_int;
    let mut od_49: libc::c_int = 5 as libc::c_int + 7 as libc::c_int * 32 as libc::c_int;
    *level.offset(50 as libc::c_int as isize) = (*p_src.offset(oe_49 as isize) as libc::c_int
        - *p_dst.offset(od_49 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(50 as libc::c_int as isize) as libc::c_int;
    let mut oe_50: libc::c_int = 6 as libc::c_int + 3 as libc::c_int * 16 as libc::c_int;
    let mut od_50: libc::c_int = 6 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int;
    *level.offset(51 as libc::c_int as isize) = (*p_src.offset(oe_50 as isize) as libc::c_int
        - *p_dst.offset(od_50 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(51 as libc::c_int as isize) as libc::c_int;
    let mut oe_51: libc::c_int = 7 as libc::c_int + 0 as libc::c_int * 16 as libc::c_int;
    let mut od_51: libc::c_int = 7 as libc::c_int + 0 as libc::c_int * 32 as libc::c_int;
    *level.offset(52 as libc::c_int as isize) = (*p_src.offset(oe_51 as isize) as libc::c_int
        - *p_dst.offset(od_51 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(52 as libc::c_int as isize) as libc::c_int;
    let mut oe_52: libc::c_int = 7 as libc::c_int + 1 as libc::c_int * 16 as libc::c_int;
    let mut od_52: libc::c_int = 7 as libc::c_int + 1 as libc::c_int * 32 as libc::c_int;
    *level.offset(53 as libc::c_int as isize) = (*p_src.offset(oe_52 as isize) as libc::c_int
        - *p_dst.offset(od_52 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(53 as libc::c_int as isize) as libc::c_int;
    let mut oe_53: libc::c_int = 6 as libc::c_int + 4 as libc::c_int * 16 as libc::c_int;
    let mut od_53: libc::c_int = 6 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int;
    *level.offset(54 as libc::c_int as isize) = (*p_src.offset(oe_53 as isize) as libc::c_int
        - *p_dst.offset(od_53 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(54 as libc::c_int as isize) as libc::c_int;
    let mut oe_54: libc::c_int = 6 as libc::c_int + 5 as libc::c_int * 16 as libc::c_int;
    let mut od_54: libc::c_int = 6 as libc::c_int + 5 as libc::c_int * 32 as libc::c_int;
    *level.offset(55 as libc::c_int as isize) = (*p_src.offset(oe_54 as isize) as libc::c_int
        - *p_dst.offset(od_54 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(55 as libc::c_int as isize) as libc::c_int;
    let mut oe_55: libc::c_int = 6 as libc::c_int + 6 as libc::c_int * 16 as libc::c_int;
    let mut od_55: libc::c_int = 6 as libc::c_int + 6 as libc::c_int * 32 as libc::c_int;
    *level.offset(56 as libc::c_int as isize) = (*p_src.offset(oe_55 as isize) as libc::c_int
        - *p_dst.offset(od_55 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(56 as libc::c_int as isize) as libc::c_int;
    let mut oe_56: libc::c_int = 6 as libc::c_int + 7 as libc::c_int * 16 as libc::c_int;
    let mut od_56: libc::c_int = 6 as libc::c_int + 7 as libc::c_int * 32 as libc::c_int;
    *level.offset(57 as libc::c_int as isize) = (*p_src.offset(oe_56 as isize) as libc::c_int
        - *p_dst.offset(od_56 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(57 as libc::c_int as isize) as libc::c_int;
    let mut oe_57: libc::c_int = 7 as libc::c_int + 2 as libc::c_int * 16 as libc::c_int;
    let mut od_57: libc::c_int = 7 as libc::c_int + 2 as libc::c_int * 32 as libc::c_int;
    *level.offset(58 as libc::c_int as isize) = (*p_src.offset(oe_57 as isize) as libc::c_int
        - *p_dst.offset(od_57 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(58 as libc::c_int as isize) as libc::c_int;
    let mut oe_58: libc::c_int = 7 as libc::c_int + 3 as libc::c_int * 16 as libc::c_int;
    let mut od_58: libc::c_int = 7 as libc::c_int + 3 as libc::c_int * 32 as libc::c_int;
    *level.offset(59 as libc::c_int as isize) = (*p_src.offset(oe_58 as isize) as libc::c_int
        - *p_dst.offset(od_58 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(59 as libc::c_int as isize) as libc::c_int;
    let mut oe_59: libc::c_int = 7 as libc::c_int + 4 as libc::c_int * 16 as libc::c_int;
    let mut od_59: libc::c_int = 7 as libc::c_int + 4 as libc::c_int * 32 as libc::c_int;
    *level.offset(60 as libc::c_int as isize) = (*p_src.offset(oe_59 as isize) as libc::c_int
        - *p_dst.offset(od_59 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(60 as libc::c_int as isize) as libc::c_int;
    let mut oe_60: libc::c_int = 7 as libc::c_int + 5 as libc::c_int * 16 as libc::c_int;
    let mut od_60: libc::c_int = 7 as libc::c_int + 5 as libc::c_int * 32 as libc::c_int;
    *level.offset(61 as libc::c_int as isize) = (*p_src.offset(oe_60 as isize) as libc::c_int
        - *p_dst.offset(od_60 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(61 as libc::c_int as isize) as libc::c_int;
    let mut oe_61: libc::c_int = 7 as libc::c_int + 6 as libc::c_int * 16 as libc::c_int;
    let mut od_61: libc::c_int = 7 as libc::c_int + 6 as libc::c_int * 32 as libc::c_int;
    *level.offset(62 as libc::c_int as isize) = (*p_src.offset(oe_61 as isize) as libc::c_int
        - *p_dst.offset(od_61 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(62 as libc::c_int as isize) as libc::c_int;
    let mut oe_62: libc::c_int = 7 as libc::c_int + 7 as libc::c_int * 16 as libc::c_int;
    let mut od_62: libc::c_int = 7 as libc::c_int + 7 as libc::c_int * 32 as libc::c_int;
    *level.offset(63 as libc::c_int as isize) = (*p_src.offset(oe_62 as isize) as libc::c_int
        - *p_dst.offset(od_62 as isize) as libc::c_int)
        as dctcoef;
    nz |= *level.offset(63 as libc::c_int as isize) as libc::c_int;
    (*(p_dst.offset((0 as libc::c_int * 32 as libc::c_int) as isize) as *mut x264_union32_t)).i =
        (*(p_src.offset((0 as libc::c_int * 16 as libc::c_int) as isize) as *mut x264_union32_t)).i;
    (*(p_dst
        .offset((0 as libc::c_int * 32 as libc::c_int) as isize)
        .offset(4 as libc::c_int as isize) as *mut x264_union32_t))
        .i = (*(p_src
        .offset((0 as libc::c_int * 16 as libc::c_int) as isize)
        .offset(4 as libc::c_int as isize) as *mut x264_union32_t))
        .i;
    (*(p_dst.offset((1 as libc::c_int * 32 as libc::c_int) as isize) as *mut x264_union32_t)).i =
        (*(p_src.offset((1 as libc::c_int * 16 as libc::c_int) as isize) as *mut x264_union32_t)).i;
    (*(p_dst
        .offset((1 as libc::c_int * 32 as libc::c_int) as isize)
        .offset(4 as libc::c_int as isize) as *mut x264_union32_t))
        .i = (*(p_src
        .offset((1 as libc::c_int * 16 as libc::c_int) as isize)
        .offset(4 as libc::c_int as isize) as *mut x264_union32_t))
        .i;
    (*(p_dst.offset((2 as libc::c_int * 32 as libc::c_int) as isize) as *mut x264_union32_t)).i =
        (*(p_src.offset((2 as libc::c_int * 16 as libc::c_int) as isize) as *mut x264_union32_t)).i;
    (*(p_dst
        .offset((2 as libc::c_int * 32 as libc::c_int) as isize)
        .offset(4 as libc::c_int as isize) as *mut x264_union32_t))
        .i = (*(p_src
        .offset((2 as libc::c_int * 16 as libc::c_int) as isize)
        .offset(4 as libc::c_int as isize) as *mut x264_union32_t))
        .i;
    (*(p_dst.offset((3 as libc::c_int * 32 as libc::c_int) as isize) as *mut x264_union32_t)).i =
        (*(p_src.offset((3 as libc::c_int * 16 as libc::c_int) as isize) as *mut x264_union32_t)).i;
    (*(p_dst
        .offset((3 as libc::c_int * 32 as libc::c_int) as isize)
        .offset(4 as libc::c_int as isize) as *mut x264_union32_t))
        .i = (*(p_src
        .offset((3 as libc::c_int * 16 as libc::c_int) as isize)
        .offset(4 as libc::c_int as isize) as *mut x264_union32_t))
        .i;
    (*(p_dst.offset((4 as libc::c_int * 32 as libc::c_int) as isize) as *mut x264_union32_t)).i =
        (*(p_src.offset((4 as libc::c_int * 16 as libc::c_int) as isize) as *mut x264_union32_t)).i;
    (*(p_dst
        .offset((4 as libc::c_int * 32 as libc::c_int) as isize)
        .offset(4 as libc::c_int as isize) as *mut x264_union32_t))
        .i = (*(p_src
        .offset((4 as libc::c_int * 16 as libc::c_int) as isize)
        .offset(4 as libc::c_int as isize) as *mut x264_union32_t))
        .i;
    (*(p_dst.offset((5 as libc::c_int * 32 as libc::c_int) as isize) as *mut x264_union32_t)).i =
        (*(p_src.offset((5 as libc::c_int * 16 as libc::c_int) as isize) as *mut x264_union32_t)).i;
    (*(p_dst
        .offset((5 as libc::c_int * 32 as libc::c_int) as isize)
        .offset(4 as libc::c_int as isize) as *mut x264_union32_t))
        .i = (*(p_src
        .offset((5 as libc::c_int * 16 as libc::c_int) as isize)
        .offset(4 as libc::c_int as isize) as *mut x264_union32_t))
        .i;
    (*(p_dst.offset((6 as libc::c_int * 32 as libc::c_int) as isize) as *mut x264_union32_t)).i =
        (*(p_src.offset((6 as libc::c_int * 16 as libc::c_int) as isize) as *mut x264_union32_t)).i;
    (*(p_dst
        .offset((6 as libc::c_int * 32 as libc::c_int) as isize)
        .offset(4 as libc::c_int as isize) as *mut x264_union32_t))
        .i = (*(p_src
        .offset((6 as libc::c_int * 16 as libc::c_int) as isize)
        .offset(4 as libc::c_int as isize) as *mut x264_union32_t))
        .i;
    (*(p_dst.offset((7 as libc::c_int * 32 as libc::c_int) as isize) as *mut x264_union32_t)).i =
        (*(p_src.offset((7 as libc::c_int * 16 as libc::c_int) as isize) as *mut x264_union32_t)).i;
    (*(p_dst
        .offset((7 as libc::c_int * 32 as libc::c_int) as isize)
        .offset(4 as libc::c_int as isize) as *mut x264_union32_t))
        .i = (*(p_src
        .offset((7 as libc::c_int * 16 as libc::c_int) as isize)
        .offset(4 as libc::c_int as isize) as *mut x264_union32_t))
        .i;
    (nz != 0) as libc::c_int
}
unsafe extern "C" fn zigzag_interleave_8x8_cavlc(
    mut dst: *mut dctcoef,
    mut src: *mut dctcoef,
    mut nnz: *mut uint8_t,
) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        let mut nz: libc::c_int = 0 as libc::c_int;
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < 16 as libc::c_int {
            nz |= *src.offset((i + j * 4 as libc::c_int) as isize) as libc::c_int;
            *dst.offset((i * 16 as libc::c_int + j) as isize) =
                *src.offset((i + j * 4 as libc::c_int) as isize);
            j += 1;
            j;
        }
        *nnz.offset(
            ((i & 1 as libc::c_int) + (i >> 1 as libc::c_int) * 8 as libc::c_int) as isize,
        ) = (nz != 0) as libc::c_int as uint8_t;
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_zigzag_init(
    mut cpu: uint32_t,
    mut pf_progressive: *mut x264_zigzag_function_t,
    mut pf_interlaced: *mut x264_zigzag_function_t,
) {
    (*pf_interlaced).scan_8x8 =
        Some(zigzag_scan_8x8_field as unsafe extern "C" fn(*mut dctcoef, *mut dctcoef) -> ());
    (*pf_progressive).scan_8x8 =
        Some(zigzag_scan_8x8_frame as unsafe extern "C" fn(*mut dctcoef, *mut dctcoef) -> ());
    (*pf_interlaced).scan_4x4 =
        Some(zigzag_scan_4x4_field as unsafe extern "C" fn(*mut dctcoef, *mut dctcoef) -> ());
    (*pf_progressive).scan_4x4 =
        Some(zigzag_scan_4x4_frame as unsafe extern "C" fn(*mut dctcoef, *mut dctcoef) -> ());
    (*pf_interlaced).sub_8x8 = Some(
        zigzag_sub_8x8_field
            as unsafe extern "C" fn(*mut dctcoef, *const pixel, *mut pixel) -> libc::c_int,
    );
    (*pf_progressive).sub_8x8 = Some(
        zigzag_sub_8x8_frame
            as unsafe extern "C" fn(*mut dctcoef, *const pixel, *mut pixel) -> libc::c_int,
    );
    (*pf_interlaced).sub_4x4 = Some(
        zigzag_sub_4x4_field
            as unsafe extern "C" fn(*mut dctcoef, *const pixel, *mut pixel) -> libc::c_int,
    );
    (*pf_progressive).sub_4x4 = Some(
        zigzag_sub_4x4_frame
            as unsafe extern "C" fn(*mut dctcoef, *const pixel, *mut pixel) -> libc::c_int,
    );
    (*pf_interlaced).sub_4x4ac = Some(
        zigzag_sub_4x4ac_field
            as unsafe extern "C" fn(
                *mut dctcoef,
                *const pixel,
                *mut pixel,
                *mut dctcoef,
            ) -> libc::c_int,
    );
    (*pf_progressive).sub_4x4ac = Some(
        zigzag_sub_4x4ac_frame
            as unsafe extern "C" fn(
                *mut dctcoef,
                *const pixel,
                *mut pixel,
                *mut dctcoef,
            ) -> libc::c_int,
    );
    (*pf_progressive).interleave_8x8_cavlc = Some(
        zigzag_interleave_8x8_cavlc
            as unsafe extern "C" fn(*mut dctcoef, *mut dctcoef, *mut uint8_t) -> (),
    );
    (*pf_interlaced).interleave_8x8_cavlc = (*pf_progressive).interleave_8x8_cavlc;
}
