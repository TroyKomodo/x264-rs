use crate::types::*;
extern "C" {
    fn abs(_: libc::c_int) -> libc::c_int;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn x264_8_predict_8x8_dc_c(src: *mut pixel, edge: *mut pixel);
    fn x264_8_predict_8x8_h_c(src: *mut pixel, edge: *mut pixel);
    fn x264_8_predict_8x8_v_c(src: *mut pixel, edge: *mut pixel);
    fn x264_8_predict_4x4_dc_c(src: *mut pixel);
    fn x264_8_predict_4x4_h_c(src: *mut pixel);
    fn x264_8_predict_4x4_v_c(src: *mut pixel);
    fn x264_8_predict_16x16_dc_c(src: *mut pixel);
    fn x264_8_predict_16x16_h_c(src: *mut pixel);
    fn x264_8_predict_16x16_v_c(src: *mut pixel);
    fn x264_8_predict_8x8c_dc_c(src: *mut pixel);
    fn x264_8_predict_8x8c_h_c(src: *mut pixel);
    fn x264_8_predict_8x8c_v_c(src: *mut pixel);
    fn x264_8_predict_8x16c_dc_c(src: *mut pixel);
    fn x264_8_predict_8x16c_h_c(src: *mut pixel);
    fn x264_8_predict_8x16c_v_c(src: *mut pixel);
}
unsafe extern "C" fn x264_pixel_sad_16x16(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
) -> libc::c_int {
    let mut i_sum: libc::c_int = 0 as libc::c_int;
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < 16 as libc::c_int {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < 16 as libc::c_int {
            i_sum +=
                abs(*pix1.offset(x as isize) as libc::c_int
                    - *pix2.offset(x as isize) as libc::c_int);
            x += 1;
            x;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
        y;
    }
    i_sum
}
unsafe extern "C" fn x264_pixel_sad_16x8(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
) -> libc::c_int {
    let mut i_sum: libc::c_int = 0 as libc::c_int;
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < 8 as libc::c_int {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < 16 as libc::c_int {
            i_sum +=
                abs(*pix1.offset(x as isize) as libc::c_int
                    - *pix2.offset(x as isize) as libc::c_int);
            x += 1;
            x;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
        y;
    }
    i_sum
}
unsafe extern "C" fn x264_pixel_sad_8x16(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
) -> libc::c_int {
    let mut i_sum: libc::c_int = 0 as libc::c_int;
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < 16 as libc::c_int {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < 8 as libc::c_int {
            i_sum +=
                abs(*pix1.offset(x as isize) as libc::c_int
                    - *pix2.offset(x as isize) as libc::c_int);
            x += 1;
            x;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
        y;
    }
    i_sum
}
unsafe extern "C" fn x264_pixel_sad_8x8(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
) -> libc::c_int {
    let mut i_sum: libc::c_int = 0 as libc::c_int;
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < 8 as libc::c_int {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < 8 as libc::c_int {
            i_sum +=
                abs(*pix1.offset(x as isize) as libc::c_int
                    - *pix2.offset(x as isize) as libc::c_int);
            x += 1;
            x;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
        y;
    }
    i_sum
}
unsafe extern "C" fn x264_pixel_sad_8x4(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
) -> libc::c_int {
    let mut i_sum: libc::c_int = 0 as libc::c_int;
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < 4 as libc::c_int {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < 8 as libc::c_int {
            i_sum +=
                abs(*pix1.offset(x as isize) as libc::c_int
                    - *pix2.offset(x as isize) as libc::c_int);
            x += 1;
            x;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
        y;
    }
    i_sum
}
unsafe extern "C" fn x264_pixel_sad_4x16(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
) -> libc::c_int {
    let mut i_sum: libc::c_int = 0 as libc::c_int;
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < 16 as libc::c_int {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < 4 as libc::c_int {
            i_sum +=
                abs(*pix1.offset(x as isize) as libc::c_int
                    - *pix2.offset(x as isize) as libc::c_int);
            x += 1;
            x;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
        y;
    }
    i_sum
}
unsafe extern "C" fn x264_pixel_sad_4x8(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
) -> libc::c_int {
    let mut i_sum: libc::c_int = 0 as libc::c_int;
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < 8 as libc::c_int {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < 4 as libc::c_int {
            i_sum +=
                abs(*pix1.offset(x as isize) as libc::c_int
                    - *pix2.offset(x as isize) as libc::c_int);
            x += 1;
            x;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
        y;
    }
    i_sum
}
unsafe extern "C" fn x264_pixel_sad_4x4(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
) -> libc::c_int {
    let mut i_sum: libc::c_int = 0 as libc::c_int;
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < 4 as libc::c_int {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < 4 as libc::c_int {
            i_sum +=
                abs(*pix1.offset(x as isize) as libc::c_int
                    - *pix2.offset(x as isize) as libc::c_int);
            x += 1;
            x;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
        y;
    }
    i_sum
}
unsafe extern "C" fn x264_pixel_ssd_16x16(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
) -> libc::c_int {
    let mut i_sum: libc::c_int = 0 as libc::c_int;
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < 16 as libc::c_int {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < 16 as libc::c_int {
            let mut d: libc::c_int =
                *pix1.offset(x as isize) as libc::c_int - *pix2.offset(x as isize) as libc::c_int;
            i_sum += d * d;
            x += 1;
            x;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
        y;
    }
    i_sum
}
unsafe extern "C" fn x264_pixel_ssd_16x8(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
) -> libc::c_int {
    let mut i_sum: libc::c_int = 0 as libc::c_int;
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < 8 as libc::c_int {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < 16 as libc::c_int {
            let mut d: libc::c_int =
                *pix1.offset(x as isize) as libc::c_int - *pix2.offset(x as isize) as libc::c_int;
            i_sum += d * d;
            x += 1;
            x;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
        y;
    }
    i_sum
}
unsafe extern "C" fn x264_pixel_ssd_8x16(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
) -> libc::c_int {
    let mut i_sum: libc::c_int = 0 as libc::c_int;
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < 16 as libc::c_int {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < 8 as libc::c_int {
            let mut d: libc::c_int =
                *pix1.offset(x as isize) as libc::c_int - *pix2.offset(x as isize) as libc::c_int;
            i_sum += d * d;
            x += 1;
            x;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
        y;
    }
    i_sum
}
unsafe extern "C" fn x264_pixel_ssd_8x8(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
) -> libc::c_int {
    let mut i_sum: libc::c_int = 0 as libc::c_int;
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < 8 as libc::c_int {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < 8 as libc::c_int {
            let mut d: libc::c_int =
                *pix1.offset(x as isize) as libc::c_int - *pix2.offset(x as isize) as libc::c_int;
            i_sum += d * d;
            x += 1;
            x;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
        y;
    }
    i_sum
}
unsafe extern "C" fn x264_pixel_ssd_8x4(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
) -> libc::c_int {
    let mut i_sum: libc::c_int = 0 as libc::c_int;
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < 4 as libc::c_int {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < 8 as libc::c_int {
            let mut d: libc::c_int =
                *pix1.offset(x as isize) as libc::c_int - *pix2.offset(x as isize) as libc::c_int;
            i_sum += d * d;
            x += 1;
            x;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
        y;
    }
    i_sum
}
unsafe extern "C" fn x264_pixel_ssd_4x16(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
) -> libc::c_int {
    let mut i_sum: libc::c_int = 0 as libc::c_int;
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < 16 as libc::c_int {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < 4 as libc::c_int {
            let mut d: libc::c_int =
                *pix1.offset(x as isize) as libc::c_int - *pix2.offset(x as isize) as libc::c_int;
            i_sum += d * d;
            x += 1;
            x;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
        y;
    }
    i_sum
}
unsafe extern "C" fn x264_pixel_ssd_4x8(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
) -> libc::c_int {
    let mut i_sum: libc::c_int = 0 as libc::c_int;
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < 8 as libc::c_int {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < 4 as libc::c_int {
            let mut d: libc::c_int =
                *pix1.offset(x as isize) as libc::c_int - *pix2.offset(x as isize) as libc::c_int;
            i_sum += d * d;
            x += 1;
            x;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
        y;
    }
    i_sum
}
unsafe extern "C" fn x264_pixel_ssd_4x4(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
) -> libc::c_int {
    let mut i_sum: libc::c_int = 0 as libc::c_int;
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < 4 as libc::c_int {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < 4 as libc::c_int {
            let mut d: libc::c_int =
                *pix1.offset(x as isize) as libc::c_int - *pix2.offset(x as isize) as libc::c_int;
            i_sum += d * d;
            x += 1;
            x;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
        y;
    }
    i_sum
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_pixel_ssd_wxh(
    mut pf: *mut x264_pixel_function_t,
    mut pix1: *mut pixel,
    mut i_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_pix2: intptr_t,
    mut i_width: libc::c_int,
    mut i_height: libc::c_int,
) -> uint64_t {
    let mut i_ssd: uint64_t = 0 as libc::c_int as uint64_t;
    let mut y: libc::c_int = 0;
    let mut align: libc::c_int = ((pix1 as intptr_t | pix2 as intptr_t | i_pix1 | i_pix2)
        & 15 as libc::c_int as intptr_t
        == 0) as libc::c_int;
    y = 0 as libc::c_int;
    while y < i_height - 15 as libc::c_int {
        let mut x: libc::c_int = 0 as libc::c_int;
        if align != 0 {
            while x < i_width - 15 as libc::c_int {
                i_ssd = i_ssd.wrapping_add(((*pf).ssd[PIXEL_16x16 as libc::c_int as usize])
                    .expect("non-null function pointer")(
                    pix1.offset((y as intptr_t * i_pix1) as isize)
                        .offset(x as isize),
                    i_pix1,
                    pix2.offset((y as intptr_t * i_pix2) as isize)
                        .offset(x as isize),
                    i_pix2,
                ) as uint64_t);
                x += 16 as libc::c_int;
            }
        }
        while x < i_width - 7 as libc::c_int {
            i_ssd = i_ssd.wrapping_add(((*pf).ssd[PIXEL_8x16 as libc::c_int as usize])
                .expect("non-null function pointer")(
                pix1.offset((y as intptr_t * i_pix1) as isize)
                    .offset(x as isize),
                i_pix1,
                pix2.offset((y as intptr_t * i_pix2) as isize)
                    .offset(x as isize),
                i_pix2,
            ) as uint64_t);
            x += 8 as libc::c_int;
        }
        y += 16 as libc::c_int;
    }
    if y < i_height - 7 as libc::c_int {
        let mut x_0: libc::c_int = 0 as libc::c_int;
        while x_0 < i_width - 7 as libc::c_int {
            i_ssd = i_ssd.wrapping_add(((*pf).ssd[PIXEL_8x8 as libc::c_int as usize])
                .expect("non-null function pointer")(
                pix1.offset((y as intptr_t * i_pix1) as isize)
                    .offset(x_0 as isize),
                i_pix1,
                pix2.offset((y as intptr_t * i_pix2) as isize)
                    .offset(x_0 as isize),
                i_pix2,
            ) as uint64_t);
            x_0 += 8 as libc::c_int;
        }
    }
    if i_width & 7 as libc::c_int != 0 {
        y = 0 as libc::c_int;
        while y < i_height & !(7 as libc::c_int) {
            let mut x_1: libc::c_int = i_width & !(7 as libc::c_int);
            while x_1 < i_width {
                let mut d: libc::c_int = *pix1
                    .offset((y as intptr_t * i_pix1 + x_1 as intptr_t) as isize)
                    as libc::c_int
                    - *pix2.offset((y as intptr_t * i_pix2 + x_1 as intptr_t) as isize)
                        as libc::c_int;
                i_ssd = i_ssd.wrapping_add((d * d) as uint64_t);
                x_1 += 1;
                x_1;
            }
            y += 1;
            y;
        }
    }
    if i_height & 7 as libc::c_int != 0 {
        y = i_height & !(7 as libc::c_int);
        while y < i_height {
            let mut x_2: libc::c_int = 0 as libc::c_int;
            while x_2 < i_width {
                let mut d_0: libc::c_int = *pix1
                    .offset((y as intptr_t * i_pix1 + x_2 as intptr_t) as isize)
                    as libc::c_int
                    - *pix2.offset((y as intptr_t * i_pix2 + x_2 as intptr_t) as isize)
                        as libc::c_int;
                i_ssd = i_ssd.wrapping_add((d_0 * d_0) as uint64_t);
                x_2 += 1;
                x_2;
            }
            y += 1;
            y;
        }
    }
    i_ssd
}
unsafe extern "C" fn pixel_ssd_nv12_core(
    mut pixuv1: *mut pixel,
    mut stride1: intptr_t,
    mut pixuv2: *mut pixel,
    mut stride2: intptr_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut ssd_u: *mut uint64_t,
    mut ssd_v: *mut uint64_t,
) {
    *ssd_u = 0 as libc::c_int as uint64_t;
    *ssd_v = 0 as libc::c_int as uint64_t;
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < height {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < width {
            let mut du: libc::c_int = *pixuv1.offset((2 as libc::c_int * x) as isize)
                as libc::c_int
                - *pixuv2.offset((2 as libc::c_int * x) as isize) as libc::c_int;
            let mut dv: libc::c_int = *pixuv1
                .offset((2 as libc::c_int * x + 1 as libc::c_int) as isize)
                as libc::c_int
                - *pixuv2.offset((2 as libc::c_int * x + 1 as libc::c_int) as isize) as libc::c_int;
            *ssd_u = (*ssd_u).wrapping_add((du * du) as uint64_t);
            *ssd_v = (*ssd_v).wrapping_add((dv * dv) as uint64_t);
            x += 1;
            x;
        }
        y += 1;
        y;
        pixuv1 = pixuv1.offset(stride1 as isize);
        pixuv2 = pixuv2.offset(stride2 as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_pixel_ssd_nv12(
    mut pf: *mut x264_pixel_function_t,
    mut pix1: *mut pixel,
    mut i_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_pix2: intptr_t,
    mut i_width: libc::c_int,
    mut i_height: libc::c_int,
    mut ssd_u: *mut uint64_t,
    mut ssd_v: *mut uint64_t,
) {
    ((*pf).ssd_nv12_core).expect("non-null function pointer")(
        pix1,
        i_pix1,
        pix2,
        i_pix2,
        i_width & !(7 as libc::c_int),
        i_height,
        ssd_u,
        ssd_v,
    );
    if i_width & 7 as libc::c_int != 0 {
        let mut tmp: [uint64_t; 2] = [0; 2];
        pixel_ssd_nv12_core(
            pix1.offset((i_width & !(7 as libc::c_int)) as isize),
            i_pix1,
            pix2.offset((i_width & !(7 as libc::c_int)) as isize),
            i_pix2,
            i_width & 7 as libc::c_int,
            i_height,
            &mut *tmp.as_mut_ptr().offset(0 as libc::c_int as isize),
            &mut *tmp.as_mut_ptr().offset(1 as libc::c_int as isize),
        );
        *ssd_u = (*ssd_u).wrapping_add(tmp[0 as libc::c_int as usize]);
        *ssd_v = (*ssd_v).wrapping_add(tmp[1 as libc::c_int as usize]);
    }
}
unsafe extern "C" fn pixel_var_16x16(mut pix: *mut pixel, mut i_stride: intptr_t) -> uint64_t {
    let mut sum: uint32_t = 0 as libc::c_int as uint32_t;
    let mut sqr: uint32_t = 0 as libc::c_int as uint32_t;
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < 16 as libc::c_int {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < 16 as libc::c_int {
            sum = sum.wrapping_add(*pix.offset(x as isize) as uint32_t);
            sqr = sqr.wrapping_add(
                (*pix.offset(x as isize) as libc::c_int * *pix.offset(x as isize) as libc::c_int)
                    as uint32_t,
            );
            x += 1;
            x;
        }
        pix = pix.offset(i_stride as isize);
        y += 1;
        y;
    }
    (sum as uint64_t).wrapping_add((sqr as uint64_t) << 32 as libc::c_int)
}
unsafe extern "C" fn pixel_var_8x16(mut pix: *mut pixel, mut i_stride: intptr_t) -> uint64_t {
    let mut sum: uint32_t = 0 as libc::c_int as uint32_t;
    let mut sqr: uint32_t = 0 as libc::c_int as uint32_t;
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < 16 as libc::c_int {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < 8 as libc::c_int {
            sum = sum.wrapping_add(*pix.offset(x as isize) as uint32_t);
            sqr = sqr.wrapping_add(
                (*pix.offset(x as isize) as libc::c_int * *pix.offset(x as isize) as libc::c_int)
                    as uint32_t,
            );
            x += 1;
            x;
        }
        pix = pix.offset(i_stride as isize);
        y += 1;
        y;
    }
    (sum as uint64_t).wrapping_add((sqr as uint64_t) << 32 as libc::c_int)
}
unsafe extern "C" fn pixel_var_8x8(mut pix: *mut pixel, mut i_stride: intptr_t) -> uint64_t {
    let mut sum: uint32_t = 0 as libc::c_int as uint32_t;
    let mut sqr: uint32_t = 0 as libc::c_int as uint32_t;
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < 8 as libc::c_int {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < 8 as libc::c_int {
            sum = sum.wrapping_add(*pix.offset(x as isize) as uint32_t);
            sqr = sqr.wrapping_add(
                (*pix.offset(x as isize) as libc::c_int * *pix.offset(x as isize) as libc::c_int)
                    as uint32_t,
            );
            x += 1;
            x;
        }
        pix = pix.offset(i_stride as isize);
        y += 1;
        y;
    }
    (sum as uint64_t).wrapping_add((sqr as uint64_t) << 32 as libc::c_int)
}
unsafe extern "C" fn pixel_var2_8x16(
    mut fenc: *mut pixel,
    mut fdec: *mut pixel,
    mut ssd: *mut libc::c_int,
) -> libc::c_int {
    let mut sum_u: libc::c_int = 0 as libc::c_int;
    let mut sum_v: libc::c_int = 0 as libc::c_int;
    let mut sqr_u: libc::c_int = 0 as libc::c_int;
    let mut sqr_v: libc::c_int = 0 as libc::c_int;
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < 16 as libc::c_int {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < 8 as libc::c_int {
            let mut diff_u: libc::c_int =
                *fenc.offset(x as isize) as libc::c_int - *fdec.offset(x as isize) as libc::c_int;
            let mut diff_v: libc::c_int = *fenc
                .offset((x + 16 as libc::c_int / 2 as libc::c_int) as isize)
                as libc::c_int
                - *fdec.offset((x + 32 as libc::c_int / 2 as libc::c_int) as isize) as libc::c_int;
            sum_u += diff_u;
            sum_v += diff_v;
            sqr_u += diff_u * diff_u;
            sqr_v += diff_v * diff_v;
            x += 1;
            x;
        }
        fenc = fenc.offset(16 as libc::c_int as isize);
        fdec = fdec.offset(32 as libc::c_int as isize);
        y += 1;
        y;
    }
    *ssd.offset(0 as libc::c_int as isize) = sqr_u;
    *ssd.offset(1 as libc::c_int as isize) = sqr_v;
    (sqr_u as int64_t - ((sum_u as int64_t * sum_u as int64_t) >> 7 as libc::c_int)
        + sqr_v as int64_t
        - ((sum_v as int64_t * sum_v as int64_t) >> 7 as libc::c_int)) as libc::c_int
}
unsafe extern "C" fn pixel_var2_8x8(
    mut fenc: *mut pixel,
    mut fdec: *mut pixel,
    mut ssd: *mut libc::c_int,
) -> libc::c_int {
    let mut sum_u: libc::c_int = 0 as libc::c_int;
    let mut sum_v: libc::c_int = 0 as libc::c_int;
    let mut sqr_u: libc::c_int = 0 as libc::c_int;
    let mut sqr_v: libc::c_int = 0 as libc::c_int;
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < 8 as libc::c_int {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < 8 as libc::c_int {
            let mut diff_u: libc::c_int =
                *fenc.offset(x as isize) as libc::c_int - *fdec.offset(x as isize) as libc::c_int;
            let mut diff_v: libc::c_int = *fenc
                .offset((x + 16 as libc::c_int / 2 as libc::c_int) as isize)
                as libc::c_int
                - *fdec.offset((x + 32 as libc::c_int / 2 as libc::c_int) as isize) as libc::c_int;
            sum_u += diff_u;
            sum_v += diff_v;
            sqr_u += diff_u * diff_u;
            sqr_v += diff_v * diff_v;
            x += 1;
            x;
        }
        fenc = fenc.offset(16 as libc::c_int as isize);
        fdec = fdec.offset(32 as libc::c_int as isize);
        y += 1;
        y;
    }
    *ssd.offset(0 as libc::c_int as isize) = sqr_u;
    *ssd.offset(1 as libc::c_int as isize) = sqr_v;
    (sqr_u as int64_t - ((sum_u as int64_t * sum_u as int64_t) >> 6 as libc::c_int)
        + sqr_v as int64_t
        - ((sum_v as int64_t * sum_v as int64_t) >> 6 as libc::c_int)) as libc::c_int
}
#[inline(always)]
unsafe extern "C" fn abs2(mut a: sum2_t) -> sum2_t {
    let mut s: sum2_t = ((a
        >> (8 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<sum_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong))
        & ((1 as libc::c_int as sum2_t)
            << (8 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<sum_t>() as libc::c_ulong))
        .wrapping_add(1 as libc::c_int as sum2_t))
        * -(1 as libc::c_int) as sum_t as sum2_t;
    a.wrapping_add(s) ^ s
}
#[inline(never)]
unsafe extern "C" fn x264_pixel_satd_4x4(
    mut pix1: *mut pixel,
    mut i_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_pix2: intptr_t,
) -> libc::c_int {
    let mut tmp: [[sum2_t; 2]; 4] = [[0; 2]; 4];
    let mut a0: sum2_t = 0;
    let mut a1: sum2_t = 0;
    let mut a2: sum2_t = 0;
    let mut a3: sum2_t = 0;
    let mut b0: sum2_t = 0;
    let mut b1: sum2_t = 0;
    let mut sum: sum2_t = 0 as libc::c_int as sum2_t;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        a0 = (*pix1.offset(0 as libc::c_int as isize) as libc::c_int
            - *pix2.offset(0 as libc::c_int as isize) as libc::c_int) as sum2_t;
        a1 = (*pix1.offset(1 as libc::c_int as isize) as libc::c_int
            - *pix2.offset(1 as libc::c_int as isize) as libc::c_int) as sum2_t;
        b0 = a0.wrapping_add(a1).wrapping_add(
            a0.wrapping_sub(a1)
                << (8 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<sum_t>() as libc::c_ulong),
        );
        a2 = (*pix1.offset(2 as libc::c_int as isize) as libc::c_int
            - *pix2.offset(2 as libc::c_int as isize) as libc::c_int) as sum2_t;
        a3 = (*pix1.offset(3 as libc::c_int as isize) as libc::c_int
            - *pix2.offset(3 as libc::c_int as isize) as libc::c_int) as sum2_t;
        b1 = a2.wrapping_add(a3).wrapping_add(
            a2.wrapping_sub(a3)
                << (8 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<sum_t>() as libc::c_ulong),
        );
        tmp[i as usize][0 as libc::c_int as usize] = b0.wrapping_add(b1);
        tmp[i as usize][1 as libc::c_int as usize] = b0.wrapping_sub(b1);
        i += 1;
        i;
        pix1 = pix1.offset(i_pix1 as isize);
        pix2 = pix2.offset(i_pix2 as isize);
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < 2 as libc::c_int {
        let mut t0: sum2_t = (tmp[0 as libc::c_int as usize][i_0 as usize])
            .wrapping_add(tmp[1 as libc::c_int as usize][i_0 as usize]);
        let mut t1: sum2_t = (tmp[0 as libc::c_int as usize][i_0 as usize])
            .wrapping_sub(tmp[1 as libc::c_int as usize][i_0 as usize]);
        let mut t2: sum2_t = (tmp[2 as libc::c_int as usize][i_0 as usize])
            .wrapping_add(tmp[3 as libc::c_int as usize][i_0 as usize]);
        let mut t3: sum2_t = (tmp[2 as libc::c_int as usize][i_0 as usize])
            .wrapping_sub(tmp[3 as libc::c_int as usize][i_0 as usize]);
        a0 = t0.wrapping_add(t2);
        a2 = t0.wrapping_sub(t2);
        a1 = t1.wrapping_add(t3);
        a3 = t1.wrapping_sub(t3);
        a0 = (abs2(a0))
            .wrapping_add(abs2(a1))
            .wrapping_add(abs2(a2))
            .wrapping_add(abs2(a3));
        sum = sum.wrapping_add(
            (a0 as sum_t as sum2_t).wrapping_add(
                a0 >> (8 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<sum_t>() as libc::c_ulong),
            ),
        );
        i_0 += 1;
        i_0;
    }
    (sum >> 1 as libc::c_int) as libc::c_int
}
#[inline(never)]
unsafe extern "C" fn x264_pixel_satd_8x4(
    mut pix1: *mut pixel,
    mut i_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_pix2: intptr_t,
) -> libc::c_int {
    let mut tmp: [[sum2_t; 4]; 4] = [[0; 4]; 4];
    let mut a0: sum2_t = 0;
    let mut a1: sum2_t = 0;
    let mut a2: sum2_t = 0;
    let mut a3: sum2_t = 0;
    let mut sum: sum2_t = 0 as libc::c_int as sum2_t;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        a0 = ((*pix1.offset(0 as libc::c_int as isize) as libc::c_int
            - *pix2.offset(0 as libc::c_int as isize) as libc::c_int) as sum2_t)
            .wrapping_add(
                ((*pix1.offset(4 as libc::c_int as isize) as libc::c_int
                    - *pix2.offset(4 as libc::c_int as isize) as libc::c_int)
                    as sum2_t)
                    << (8 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<sum_t>() as libc::c_ulong),
            );
        a1 = ((*pix1.offset(1 as libc::c_int as isize) as libc::c_int
            - *pix2.offset(1 as libc::c_int as isize) as libc::c_int) as sum2_t)
            .wrapping_add(
                ((*pix1.offset(5 as libc::c_int as isize) as libc::c_int
                    - *pix2.offset(5 as libc::c_int as isize) as libc::c_int)
                    as sum2_t)
                    << (8 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<sum_t>() as libc::c_ulong),
            );
        a2 = ((*pix1.offset(2 as libc::c_int as isize) as libc::c_int
            - *pix2.offset(2 as libc::c_int as isize) as libc::c_int) as sum2_t)
            .wrapping_add(
                ((*pix1.offset(6 as libc::c_int as isize) as libc::c_int
                    - *pix2.offset(6 as libc::c_int as isize) as libc::c_int)
                    as sum2_t)
                    << (8 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<sum_t>() as libc::c_ulong),
            );
        a3 = ((*pix1.offset(3 as libc::c_int as isize) as libc::c_int
            - *pix2.offset(3 as libc::c_int as isize) as libc::c_int) as sum2_t)
            .wrapping_add(
                ((*pix1.offset(7 as libc::c_int as isize) as libc::c_int
                    - *pix2.offset(7 as libc::c_int as isize) as libc::c_int)
                    as sum2_t)
                    << (8 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<sum_t>() as libc::c_ulong),
            );
        let mut t0: sum2_t = a0.wrapping_add(a1);
        let mut t1: sum2_t = a0.wrapping_sub(a1);
        let mut t2: sum2_t = a2.wrapping_add(a3);
        let mut t3: sum2_t = a2.wrapping_sub(a3);
        tmp[i as usize][0 as libc::c_int as usize] = t0.wrapping_add(t2);
        tmp[i as usize][2 as libc::c_int as usize] = t0.wrapping_sub(t2);
        tmp[i as usize][1 as libc::c_int as usize] = t1.wrapping_add(t3);
        tmp[i as usize][3 as libc::c_int as usize] = t1.wrapping_sub(t3);
        i += 1;
        i;
        pix1 = pix1.offset(i_pix1 as isize);
        pix2 = pix2.offset(i_pix2 as isize);
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < 4 as libc::c_int {
        let mut t0_0: sum2_t = (tmp[0 as libc::c_int as usize][i_0 as usize])
            .wrapping_add(tmp[1 as libc::c_int as usize][i_0 as usize]);
        let mut t1_0: sum2_t = (tmp[0 as libc::c_int as usize][i_0 as usize])
            .wrapping_sub(tmp[1 as libc::c_int as usize][i_0 as usize]);
        let mut t2_0: sum2_t = (tmp[2 as libc::c_int as usize][i_0 as usize])
            .wrapping_add(tmp[3 as libc::c_int as usize][i_0 as usize]);
        let mut t3_0: sum2_t = (tmp[2 as libc::c_int as usize][i_0 as usize])
            .wrapping_sub(tmp[3 as libc::c_int as usize][i_0 as usize]);
        a0 = t0_0.wrapping_add(t2_0);
        a2 = t0_0.wrapping_sub(t2_0);
        a1 = t1_0.wrapping_add(t3_0);
        a3 = t1_0.wrapping_sub(t3_0);
        sum = sum.wrapping_add(
            (abs2(a0))
                .wrapping_add(abs2(a1))
                .wrapping_add(abs2(a2))
                .wrapping_add(abs2(a3)),
        );
        i_0 += 1;
        i_0;
    }
    ((sum as sum_t as sum2_t).wrapping_add(
        sum >> (8 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<sum_t>() as libc::c_ulong),
    ) >> 1 as libc::c_int) as libc::c_int
}
unsafe extern "C" fn x264_pixel_satd_16x16(
    mut pix1: *mut pixel,
    mut i_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_pix2: intptr_t,
) -> libc::c_int {
    let mut sum: libc::c_int = x264_pixel_satd_8x4(pix1, i_pix1, pix2, i_pix2)
        + x264_pixel_satd_8x4(
            pix1.offset((4 as libc::c_int as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset((4 as libc::c_int as intptr_t * i_pix2) as isize),
            i_pix2,
        );
    if 16 as libc::c_int == 16 as libc::c_int {
        sum += x264_pixel_satd_8x4(
            pix1.offset(8 as libc::c_int as isize),
            i_pix1,
            pix2.offset(8 as libc::c_int as isize),
            i_pix2,
        ) + x264_pixel_satd_8x4(
            pix1.offset(8 as libc::c_int as isize)
                .offset((4 as libc::c_int as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset(8 as libc::c_int as isize)
                .offset((4 as libc::c_int as intptr_t * i_pix2) as isize),
            i_pix2,
        );
    }
    if 16 as libc::c_int == 16 as libc::c_int {
        sum += x264_pixel_satd_8x4(
            pix1.offset((8 as libc::c_int as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset((8 as libc::c_int as intptr_t * i_pix2) as isize),
            i_pix2,
        ) + x264_pixel_satd_8x4(
            pix1.offset((12 as libc::c_int as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset((12 as libc::c_int as intptr_t * i_pix2) as isize),
            i_pix2,
        );
    }
    if 16 as libc::c_int == 16 as libc::c_int && 16 as libc::c_int == 16 as libc::c_int {
        sum += x264_pixel_satd_8x4(
            pix1.offset(8 as libc::c_int as isize)
                .offset((8 as libc::c_int as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset(8 as libc::c_int as isize)
                .offset((8 as libc::c_int as intptr_t * i_pix2) as isize),
            i_pix2,
        ) + x264_pixel_satd_8x4(
            pix1.offset(8 as libc::c_int as isize)
                .offset((12 as libc::c_int as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset(8 as libc::c_int as isize)
                .offset((12 as libc::c_int as intptr_t * i_pix2) as isize),
            i_pix2,
        );
    }
    sum
}
unsafe extern "C" fn x264_pixel_satd_16x8(
    mut pix1: *mut pixel,
    mut i_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_pix2: intptr_t,
) -> libc::c_int {
    let mut sum: libc::c_int = x264_pixel_satd_8x4(pix1, i_pix1, pix2, i_pix2)
        + x264_pixel_satd_8x4(
            pix1.offset((4 as libc::c_int as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset((4 as libc::c_int as intptr_t * i_pix2) as isize),
            i_pix2,
        );
    if 16 as libc::c_int == 16 as libc::c_int {
        sum += x264_pixel_satd_8x4(
            pix1.offset(8 as libc::c_int as isize),
            i_pix1,
            pix2.offset(8 as libc::c_int as isize),
            i_pix2,
        ) + x264_pixel_satd_8x4(
            pix1.offset(8 as libc::c_int as isize)
                .offset((4 as libc::c_int as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset(8 as libc::c_int as isize)
                .offset((4 as libc::c_int as intptr_t * i_pix2) as isize),
            i_pix2,
        );
    }
    if 8 as libc::c_int == 16 as libc::c_int {
        sum += x264_pixel_satd_8x4(
            pix1.offset((8 as libc::c_int as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset((8 as libc::c_int as intptr_t * i_pix2) as isize),
            i_pix2,
        ) + x264_pixel_satd_8x4(
            pix1.offset((12 as libc::c_int as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset((12 as libc::c_int as intptr_t * i_pix2) as isize),
            i_pix2,
        );
    }
    if 16 as libc::c_int == 16 as libc::c_int && 8 as libc::c_int == 16 as libc::c_int {
        sum += x264_pixel_satd_8x4(
            pix1.offset(8 as libc::c_int as isize)
                .offset((8 as libc::c_int as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset(8 as libc::c_int as isize)
                .offset((8 as libc::c_int as intptr_t * i_pix2) as isize),
            i_pix2,
        ) + x264_pixel_satd_8x4(
            pix1.offset(8 as libc::c_int as isize)
                .offset((12 as libc::c_int as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset(8 as libc::c_int as isize)
                .offset((12 as libc::c_int as intptr_t * i_pix2) as isize),
            i_pix2,
        );
    }
    sum
}
unsafe extern "C" fn x264_pixel_satd_8x16(
    mut pix1: *mut pixel,
    mut i_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_pix2: intptr_t,
) -> libc::c_int {
    let mut sum: libc::c_int = x264_pixel_satd_8x4(pix1, i_pix1, pix2, i_pix2)
        + x264_pixel_satd_8x4(
            pix1.offset((4 as libc::c_int as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset((4 as libc::c_int as intptr_t * i_pix2) as isize),
            i_pix2,
        );
    if 8 as libc::c_int == 16 as libc::c_int {
        sum += x264_pixel_satd_8x4(
            pix1.offset(8 as libc::c_int as isize),
            i_pix1,
            pix2.offset(8 as libc::c_int as isize),
            i_pix2,
        ) + x264_pixel_satd_8x4(
            pix1.offset(8 as libc::c_int as isize)
                .offset((4 as libc::c_int as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset(8 as libc::c_int as isize)
                .offset((4 as libc::c_int as intptr_t * i_pix2) as isize),
            i_pix2,
        );
    }
    if 16 as libc::c_int == 16 as libc::c_int {
        sum += x264_pixel_satd_8x4(
            pix1.offset((8 as libc::c_int as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset((8 as libc::c_int as intptr_t * i_pix2) as isize),
            i_pix2,
        ) + x264_pixel_satd_8x4(
            pix1.offset((12 as libc::c_int as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset((12 as libc::c_int as intptr_t * i_pix2) as isize),
            i_pix2,
        );
    }
    if 8 as libc::c_int == 16 as libc::c_int && 16 as libc::c_int == 16 as libc::c_int {
        sum += x264_pixel_satd_8x4(
            pix1.offset(8 as libc::c_int as isize)
                .offset((8 as libc::c_int as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset(8 as libc::c_int as isize)
                .offset((8 as libc::c_int as intptr_t * i_pix2) as isize),
            i_pix2,
        ) + x264_pixel_satd_8x4(
            pix1.offset(8 as libc::c_int as isize)
                .offset((12 as libc::c_int as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset(8 as libc::c_int as isize)
                .offset((12 as libc::c_int as intptr_t * i_pix2) as isize),
            i_pix2,
        );
    }
    sum
}
unsafe extern "C" fn x264_pixel_satd_8x8(
    mut pix1: *mut pixel,
    mut i_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_pix2: intptr_t,
) -> libc::c_int {
    let mut sum: libc::c_int = x264_pixel_satd_8x4(pix1, i_pix1, pix2, i_pix2)
        + x264_pixel_satd_8x4(
            pix1.offset((4 as libc::c_int as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset((4 as libc::c_int as intptr_t * i_pix2) as isize),
            i_pix2,
        );
    if 8 as libc::c_int == 16 as libc::c_int {
        sum += x264_pixel_satd_8x4(
            pix1.offset(8 as libc::c_int as isize),
            i_pix1,
            pix2.offset(8 as libc::c_int as isize),
            i_pix2,
        ) + x264_pixel_satd_8x4(
            pix1.offset(8 as libc::c_int as isize)
                .offset((4 as libc::c_int as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset(8 as libc::c_int as isize)
                .offset((4 as libc::c_int as intptr_t * i_pix2) as isize),
            i_pix2,
        );
    }
    if 8 as libc::c_int == 16 as libc::c_int {
        sum += x264_pixel_satd_8x4(
            pix1.offset((8 as libc::c_int as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset((8 as libc::c_int as intptr_t * i_pix2) as isize),
            i_pix2,
        ) + x264_pixel_satd_8x4(
            pix1.offset((12 as libc::c_int as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset((12 as libc::c_int as intptr_t * i_pix2) as isize),
            i_pix2,
        );
    }
    if 8 as libc::c_int == 16 as libc::c_int && 8 as libc::c_int == 16 as libc::c_int {
        sum += x264_pixel_satd_8x4(
            pix1.offset(8 as libc::c_int as isize)
                .offset((8 as libc::c_int as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset(8 as libc::c_int as isize)
                .offset((8 as libc::c_int as intptr_t * i_pix2) as isize),
            i_pix2,
        ) + x264_pixel_satd_8x4(
            pix1.offset(8 as libc::c_int as isize)
                .offset((12 as libc::c_int as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset(8 as libc::c_int as isize)
                .offset((12 as libc::c_int as intptr_t * i_pix2) as isize),
            i_pix2,
        );
    }
    sum
}
unsafe extern "C" fn x264_pixel_satd_4x16(
    mut pix1: *mut pixel,
    mut i_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_pix2: intptr_t,
) -> libc::c_int {
    let mut sum: libc::c_int = x264_pixel_satd_4x4(pix1, i_pix1, pix2, i_pix2)
        + x264_pixel_satd_4x4(
            pix1.offset((4 as libc::c_int as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset((4 as libc::c_int as intptr_t * i_pix2) as isize),
            i_pix2,
        );
    if 4 as libc::c_int == 16 as libc::c_int {
        sum += x264_pixel_satd_4x4(
            pix1.offset(8 as libc::c_int as isize),
            i_pix1,
            pix2.offset(8 as libc::c_int as isize),
            i_pix2,
        ) + x264_pixel_satd_4x4(
            pix1.offset(8 as libc::c_int as isize)
                .offset((4 as libc::c_int as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset(8 as libc::c_int as isize)
                .offset((4 as libc::c_int as intptr_t * i_pix2) as isize),
            i_pix2,
        );
    }
    if 16 as libc::c_int == 16 as libc::c_int {
        sum += x264_pixel_satd_4x4(
            pix1.offset((8 as libc::c_int as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset((8 as libc::c_int as intptr_t * i_pix2) as isize),
            i_pix2,
        ) + x264_pixel_satd_4x4(
            pix1.offset((12 as libc::c_int as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset((12 as libc::c_int as intptr_t * i_pix2) as isize),
            i_pix2,
        );
    }
    if 4 as libc::c_int == 16 as libc::c_int && 16 as libc::c_int == 16 as libc::c_int {
        sum += x264_pixel_satd_4x4(
            pix1.offset(8 as libc::c_int as isize)
                .offset((8 as libc::c_int as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset(8 as libc::c_int as isize)
                .offset((8 as libc::c_int as intptr_t * i_pix2) as isize),
            i_pix2,
        ) + x264_pixel_satd_4x4(
            pix1.offset(8 as libc::c_int as isize)
                .offset((12 as libc::c_int as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset(8 as libc::c_int as isize)
                .offset((12 as libc::c_int as intptr_t * i_pix2) as isize),
            i_pix2,
        );
    }
    sum
}
unsafe extern "C" fn x264_pixel_satd_4x8(
    mut pix1: *mut pixel,
    mut i_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_pix2: intptr_t,
) -> libc::c_int {
    let mut sum: libc::c_int = x264_pixel_satd_4x4(pix1, i_pix1, pix2, i_pix2)
        + x264_pixel_satd_4x4(
            pix1.offset((4 as libc::c_int as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset((4 as libc::c_int as intptr_t * i_pix2) as isize),
            i_pix2,
        );
    if 4 as libc::c_int == 16 as libc::c_int {
        sum += x264_pixel_satd_4x4(
            pix1.offset(8 as libc::c_int as isize),
            i_pix1,
            pix2.offset(8 as libc::c_int as isize),
            i_pix2,
        ) + x264_pixel_satd_4x4(
            pix1.offset(8 as libc::c_int as isize)
                .offset((4 as libc::c_int as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset(8 as libc::c_int as isize)
                .offset((4 as libc::c_int as intptr_t * i_pix2) as isize),
            i_pix2,
        );
    }
    if 8 as libc::c_int == 16 as libc::c_int {
        sum += x264_pixel_satd_4x4(
            pix1.offset((8 as libc::c_int as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset((8 as libc::c_int as intptr_t * i_pix2) as isize),
            i_pix2,
        ) + x264_pixel_satd_4x4(
            pix1.offset((12 as libc::c_int as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset((12 as libc::c_int as intptr_t * i_pix2) as isize),
            i_pix2,
        );
    }
    if 4 as libc::c_int == 16 as libc::c_int && 8 as libc::c_int == 16 as libc::c_int {
        sum += x264_pixel_satd_4x4(
            pix1.offset(8 as libc::c_int as isize)
                .offset((8 as libc::c_int as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset(8 as libc::c_int as isize)
                .offset((8 as libc::c_int as intptr_t * i_pix2) as isize),
            i_pix2,
        ) + x264_pixel_satd_4x4(
            pix1.offset(8 as libc::c_int as isize)
                .offset((12 as libc::c_int as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset(8 as libc::c_int as isize)
                .offset((12 as libc::c_int as intptr_t * i_pix2) as isize),
            i_pix2,
        );
    }
    sum
}
#[inline(never)]
unsafe extern "C" fn sa8d_8x8(
    mut pix1: *mut pixel,
    mut i_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_pix2: intptr_t,
) -> libc::c_int {
    let mut tmp: [[sum2_t; 4]; 8] = [[0; 4]; 8];
    let mut a0: sum2_t = 0;
    let mut a1: sum2_t = 0;
    let mut a2: sum2_t = 0;
    let mut a3: sum2_t = 0;
    let mut a4: sum2_t = 0;
    let mut a5: sum2_t = 0;
    let mut a6: sum2_t = 0;
    let mut a7: sum2_t = 0;
    let mut b0: sum2_t = 0;
    let mut b1: sum2_t = 0;
    let mut b2: sum2_t = 0;
    let mut b3: sum2_t = 0;
    let mut sum: sum2_t = 0 as libc::c_int as sum2_t;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        a0 = (*pix1.offset(0 as libc::c_int as isize) as libc::c_int
            - *pix2.offset(0 as libc::c_int as isize) as libc::c_int) as sum2_t;
        a1 = (*pix1.offset(1 as libc::c_int as isize) as libc::c_int
            - *pix2.offset(1 as libc::c_int as isize) as libc::c_int) as sum2_t;
        b0 = a0.wrapping_add(a1).wrapping_add(
            a0.wrapping_sub(a1)
                << (8 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<sum_t>() as libc::c_ulong),
        );
        a2 = (*pix1.offset(2 as libc::c_int as isize) as libc::c_int
            - *pix2.offset(2 as libc::c_int as isize) as libc::c_int) as sum2_t;
        a3 = (*pix1.offset(3 as libc::c_int as isize) as libc::c_int
            - *pix2.offset(3 as libc::c_int as isize) as libc::c_int) as sum2_t;
        b1 = a2.wrapping_add(a3).wrapping_add(
            a2.wrapping_sub(a3)
                << (8 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<sum_t>() as libc::c_ulong),
        );
        a4 = (*pix1.offset(4 as libc::c_int as isize) as libc::c_int
            - *pix2.offset(4 as libc::c_int as isize) as libc::c_int) as sum2_t;
        a5 = (*pix1.offset(5 as libc::c_int as isize) as libc::c_int
            - *pix2.offset(5 as libc::c_int as isize) as libc::c_int) as sum2_t;
        b2 = a4.wrapping_add(a5).wrapping_add(
            a4.wrapping_sub(a5)
                << (8 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<sum_t>() as libc::c_ulong),
        );
        a6 = (*pix1.offset(6 as libc::c_int as isize) as libc::c_int
            - *pix2.offset(6 as libc::c_int as isize) as libc::c_int) as sum2_t;
        a7 = (*pix1.offset(7 as libc::c_int as isize) as libc::c_int
            - *pix2.offset(7 as libc::c_int as isize) as libc::c_int) as sum2_t;
        b3 = a6.wrapping_add(a7).wrapping_add(
            a6.wrapping_sub(a7)
                << (8 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<sum_t>() as libc::c_ulong),
        );
        let mut t0: sum2_t = b0.wrapping_add(b1);
        let mut t1: sum2_t = b0.wrapping_sub(b1);
        let mut t2: sum2_t = b2.wrapping_add(b3);
        let mut t3: sum2_t = b2.wrapping_sub(b3);
        tmp[i as usize][0 as libc::c_int as usize] = t0.wrapping_add(t2);
        tmp[i as usize][2 as libc::c_int as usize] = t0.wrapping_sub(t2);
        tmp[i as usize][1 as libc::c_int as usize] = t1.wrapping_add(t3);
        tmp[i as usize][3 as libc::c_int as usize] = t1.wrapping_sub(t3);
        i += 1;
        i;
        pix1 = pix1.offset(i_pix1 as isize);
        pix2 = pix2.offset(i_pix2 as isize);
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < 4 as libc::c_int {
        let mut t0_0: sum2_t = (tmp[0 as libc::c_int as usize][i_0 as usize])
            .wrapping_add(tmp[1 as libc::c_int as usize][i_0 as usize]);
        let mut t1_0: sum2_t = (tmp[0 as libc::c_int as usize][i_0 as usize])
            .wrapping_sub(tmp[1 as libc::c_int as usize][i_0 as usize]);
        let mut t2_0: sum2_t = (tmp[2 as libc::c_int as usize][i_0 as usize])
            .wrapping_add(tmp[3 as libc::c_int as usize][i_0 as usize]);
        let mut t3_0: sum2_t = (tmp[2 as libc::c_int as usize][i_0 as usize])
            .wrapping_sub(tmp[3 as libc::c_int as usize][i_0 as usize]);
        a0 = t0_0.wrapping_add(t2_0);
        a2 = t0_0.wrapping_sub(t2_0);
        a1 = t1_0.wrapping_add(t3_0);
        a3 = t1_0.wrapping_sub(t3_0);
        let mut t0_1: sum2_t = (tmp[4 as libc::c_int as usize][i_0 as usize])
            .wrapping_add(tmp[5 as libc::c_int as usize][i_0 as usize]);
        let mut t1_1: sum2_t = (tmp[4 as libc::c_int as usize][i_0 as usize])
            .wrapping_sub(tmp[5 as libc::c_int as usize][i_0 as usize]);
        let mut t2_1: sum2_t = (tmp[6 as libc::c_int as usize][i_0 as usize])
            .wrapping_add(tmp[7 as libc::c_int as usize][i_0 as usize]);
        let mut t3_1: sum2_t = (tmp[6 as libc::c_int as usize][i_0 as usize])
            .wrapping_sub(tmp[7 as libc::c_int as usize][i_0 as usize]);
        a4 = t0_1.wrapping_add(t2_1);
        a6 = t0_1.wrapping_sub(t2_1);
        a5 = t1_1.wrapping_add(t3_1);
        a7 = t1_1.wrapping_sub(t3_1);
        b0 = (abs2(a0.wrapping_add(a4))).wrapping_add(abs2(a0.wrapping_sub(a4)));
        b0 = b0.wrapping_add((abs2(a1.wrapping_add(a5))).wrapping_add(abs2(a1.wrapping_sub(a5))));
        b0 = b0.wrapping_add((abs2(a2.wrapping_add(a6))).wrapping_add(abs2(a2.wrapping_sub(a6))));
        b0 = b0.wrapping_add((abs2(a3.wrapping_add(a7))).wrapping_add(abs2(a3.wrapping_sub(a7))));
        sum = sum.wrapping_add(
            (b0 as sum_t as sum2_t).wrapping_add(
                b0 >> (8 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<sum_t>() as libc::c_ulong),
            ),
        );
        i_0 += 1;
        i_0;
    }
    sum as libc::c_int
}
unsafe extern "C" fn x264_pixel_sa8d_8x8(
    mut pix1: *mut pixel,
    mut i_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_pix2: intptr_t,
) -> libc::c_int {
    let mut sum: libc::c_int = sa8d_8x8(pix1, i_pix1, pix2, i_pix2);
    (sum + 2 as libc::c_int) >> 2 as libc::c_int
}
unsafe extern "C" fn x264_pixel_sa8d_16x16(
    mut pix1: *mut pixel,
    mut i_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_pix2: intptr_t,
) -> libc::c_int {
    let mut sum: libc::c_int = sa8d_8x8(pix1, i_pix1, pix2, i_pix2)
        + sa8d_8x8(
            pix1.offset(8 as libc::c_int as isize),
            i_pix1,
            pix2.offset(8 as libc::c_int as isize),
            i_pix2,
        )
        + sa8d_8x8(
            pix1.offset((8 as libc::c_int as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset((8 as libc::c_int as intptr_t * i_pix2) as isize),
            i_pix2,
        )
        + sa8d_8x8(
            pix1.offset(8 as libc::c_int as isize)
                .offset((8 as libc::c_int as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset(8 as libc::c_int as isize)
                .offset((8 as libc::c_int as intptr_t * i_pix2) as isize),
            i_pix2,
        );
    (sum + 2 as libc::c_int) >> 2 as libc::c_int
}
#[inline(never)]
unsafe extern "C" fn pixel_hadamard_ac(mut pix: *mut pixel, mut stride: intptr_t) -> uint64_t {
    let mut tmp: [sum2_t; 32] = [0; 32];
    let mut a0: sum2_t = 0;
    let mut a1: sum2_t = 0;
    let mut a2: sum2_t = 0;
    let mut a3: sum2_t = 0;
    let mut dc: sum2_t = 0;
    let mut sum4: sum2_t = 0 as libc::c_int as sum2_t;
    let mut sum8: sum2_t = 0 as libc::c_int as sum2_t;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        let mut t: *mut sum2_t = tmp
            .as_mut_ptr()
            .offset((i & 3 as libc::c_int) as isize)
            .offset(((i & 4 as libc::c_int) * 4 as libc::c_int) as isize);
        a0 = ((*pix.offset(0 as libc::c_int as isize) as libc::c_int
            + *pix.offset(1 as libc::c_int as isize) as libc::c_int) as sum2_t)
            .wrapping_add(
                ((*pix.offset(0 as libc::c_int as isize) as libc::c_int
                    - *pix.offset(1 as libc::c_int as isize) as libc::c_int)
                    as sum2_t)
                    << (8 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<sum_t>() as libc::c_ulong),
            );
        a1 = ((*pix.offset(2 as libc::c_int as isize) as libc::c_int
            + *pix.offset(3 as libc::c_int as isize) as libc::c_int) as sum2_t)
            .wrapping_add(
                ((*pix.offset(2 as libc::c_int as isize) as libc::c_int
                    - *pix.offset(3 as libc::c_int as isize) as libc::c_int)
                    as sum2_t)
                    << (8 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<sum_t>() as libc::c_ulong),
            );
        *t.offset(0 as libc::c_int as isize) = a0.wrapping_add(a1);
        *t.offset(4 as libc::c_int as isize) = a0.wrapping_sub(a1);
        a2 = ((*pix.offset(4 as libc::c_int as isize) as libc::c_int
            + *pix.offset(5 as libc::c_int as isize) as libc::c_int) as sum2_t)
            .wrapping_add(
                ((*pix.offset(4 as libc::c_int as isize) as libc::c_int
                    - *pix.offset(5 as libc::c_int as isize) as libc::c_int)
                    as sum2_t)
                    << (8 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<sum_t>() as libc::c_ulong),
            );
        a3 = ((*pix.offset(6 as libc::c_int as isize) as libc::c_int
            + *pix.offset(7 as libc::c_int as isize) as libc::c_int) as sum2_t)
            .wrapping_add(
                ((*pix.offset(6 as libc::c_int as isize) as libc::c_int
                    - *pix.offset(7 as libc::c_int as isize) as libc::c_int)
                    as sum2_t)
                    << (8 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<sum_t>() as libc::c_ulong),
            );
        *t.offset(8 as libc::c_int as isize) = a2.wrapping_add(a3);
        *t.offset(12 as libc::c_int as isize) = a2.wrapping_sub(a3);
        i += 1;
        i;
        pix = pix.offset(stride as isize);
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < 8 as libc::c_int {
        let mut t0: sum2_t = (tmp[(i_0 * 4 as libc::c_int + 0 as libc::c_int) as usize])
            .wrapping_add(tmp[(i_0 * 4 as libc::c_int + 1 as libc::c_int) as usize]);
        let mut t1: sum2_t = (tmp[(i_0 * 4 as libc::c_int + 0 as libc::c_int) as usize])
            .wrapping_sub(tmp[(i_0 * 4 as libc::c_int + 1 as libc::c_int) as usize]);
        let mut t2: sum2_t = (tmp[(i_0 * 4 as libc::c_int + 2 as libc::c_int) as usize])
            .wrapping_add(tmp[(i_0 * 4 as libc::c_int + 3 as libc::c_int) as usize]);
        let mut t3: sum2_t = (tmp[(i_0 * 4 as libc::c_int + 2 as libc::c_int) as usize])
            .wrapping_sub(tmp[(i_0 * 4 as libc::c_int + 3 as libc::c_int) as usize]);
        a0 = t0.wrapping_add(t2);
        a2 = t0.wrapping_sub(t2);
        a1 = t1.wrapping_add(t3);
        a3 = t1.wrapping_sub(t3);
        tmp[(i_0 * 4 as libc::c_int + 0 as libc::c_int) as usize] = a0;
        tmp[(i_0 * 4 as libc::c_int + 1 as libc::c_int) as usize] = a1;
        tmp[(i_0 * 4 as libc::c_int + 2 as libc::c_int) as usize] = a2;
        tmp[(i_0 * 4 as libc::c_int + 3 as libc::c_int) as usize] = a3;
        sum4 = sum4.wrapping_add(
            (abs2(a0))
                .wrapping_add(abs2(a1))
                .wrapping_add(abs2(a2))
                .wrapping_add(abs2(a3)),
        );
        i_0 += 1;
        i_0;
    }
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < 8 as libc::c_int {
        let mut t0_0: sum2_t =
            (tmp[i_1 as usize]).wrapping_add(tmp[(8 as libc::c_int + i_1) as usize]);
        let mut t1_0: sum2_t =
            (tmp[i_1 as usize]).wrapping_sub(tmp[(8 as libc::c_int + i_1) as usize]);
        let mut t2_0: sum2_t = (tmp[(16 as libc::c_int + i_1) as usize])
            .wrapping_add(tmp[(24 as libc::c_int + i_1) as usize]);
        let mut t3_0: sum2_t = (tmp[(16 as libc::c_int + i_1) as usize])
            .wrapping_sub(tmp[(24 as libc::c_int + i_1) as usize]);
        a0 = t0_0.wrapping_add(t2_0);
        a2 = t0_0.wrapping_sub(t2_0);
        a1 = t1_0.wrapping_add(t3_0);
        a3 = t1_0.wrapping_sub(t3_0);
        sum8 = sum8.wrapping_add(
            (abs2(a0))
                .wrapping_add(abs2(a1))
                .wrapping_add(abs2(a2))
                .wrapping_add(abs2(a3)),
        );
        i_1 += 1;
        i_1;
    }
    dc = (tmp[0 as libc::c_int as usize])
        .wrapping_add(tmp[8 as libc::c_int as usize])
        .wrapping_add(tmp[16 as libc::c_int as usize])
        .wrapping_add(tmp[24 as libc::c_int as usize]) as sum_t as sum2_t;
    sum4 = (sum4 as sum_t as sum2_t)
        .wrapping_add(
            sum4 >> (8 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<sum_t>() as libc::c_ulong),
        )
        .wrapping_sub(dc);
    sum8 = (sum8 as sum_t as sum2_t)
        .wrapping_add(
            sum8 >> (8 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<sum_t>() as libc::c_ulong),
        )
        .wrapping_sub(dc);
    ((sum8 as uint64_t) << 32 as libc::c_int).wrapping_add(sum4 as uint64_t)
}
unsafe extern "C" fn x264_pixel_hadamard_ac_16x16(
    mut pix: *mut pixel,
    mut stride: intptr_t,
) -> uint64_t {
    let mut sum: uint64_t = pixel_hadamard_ac(pix, stride);
    if 16 as libc::c_int == 16 as libc::c_int {
        sum = sum.wrapping_add(pixel_hadamard_ac(
            pix.offset(8 as libc::c_int as isize),
            stride,
        ));
    }
    if 16 as libc::c_int == 16 as libc::c_int {
        sum = sum.wrapping_add(pixel_hadamard_ac(
            pix.offset((8 as libc::c_int as intptr_t * stride) as isize),
            stride,
        ));
    }
    if 16 as libc::c_int == 16 as libc::c_int && 16 as libc::c_int == 16 as libc::c_int {
        sum = sum.wrapping_add(pixel_hadamard_ac(
            pix.offset((8 as libc::c_int as intptr_t * stride) as isize)
                .offset(8 as libc::c_int as isize),
            stride,
        ));
    }
    ((sum >> 34 as libc::c_int) << 32 as libc::c_int)
        .wrapping_add((sum as uint32_t >> 1 as libc::c_int) as uint64_t)
}
unsafe extern "C" fn x264_pixel_hadamard_ac_16x8(
    mut pix: *mut pixel,
    mut stride: intptr_t,
) -> uint64_t {
    let mut sum: uint64_t = pixel_hadamard_ac(pix, stride);
    if 16 as libc::c_int == 16 as libc::c_int {
        sum = sum.wrapping_add(pixel_hadamard_ac(
            pix.offset(8 as libc::c_int as isize),
            stride,
        ));
    }
    if 8 as libc::c_int == 16 as libc::c_int {
        sum = sum.wrapping_add(pixel_hadamard_ac(
            pix.offset((8 as libc::c_int as intptr_t * stride) as isize),
            stride,
        ));
    }
    if 16 as libc::c_int == 16 as libc::c_int && 8 as libc::c_int == 16 as libc::c_int {
        sum = sum.wrapping_add(pixel_hadamard_ac(
            pix.offset((8 as libc::c_int as intptr_t * stride) as isize)
                .offset(8 as libc::c_int as isize),
            stride,
        ));
    }
    ((sum >> 34 as libc::c_int) << 32 as libc::c_int)
        .wrapping_add((sum as uint32_t >> 1 as libc::c_int) as uint64_t)
}
unsafe extern "C" fn x264_pixel_hadamard_ac_8x16(
    mut pix: *mut pixel,
    mut stride: intptr_t,
) -> uint64_t {
    let mut sum: uint64_t = pixel_hadamard_ac(pix, stride);
    if 8 as libc::c_int == 16 as libc::c_int {
        sum = sum.wrapping_add(pixel_hadamard_ac(
            pix.offset(8 as libc::c_int as isize),
            stride,
        ));
    }
    if 16 as libc::c_int == 16 as libc::c_int {
        sum = sum.wrapping_add(pixel_hadamard_ac(
            pix.offset((8 as libc::c_int as intptr_t * stride) as isize),
            stride,
        ));
    }
    if 8 as libc::c_int == 16 as libc::c_int && 16 as libc::c_int == 16 as libc::c_int {
        sum = sum.wrapping_add(pixel_hadamard_ac(
            pix.offset((8 as libc::c_int as intptr_t * stride) as isize)
                .offset(8 as libc::c_int as isize),
            stride,
        ));
    }
    ((sum >> 34 as libc::c_int) << 32 as libc::c_int)
        .wrapping_add((sum as uint32_t >> 1 as libc::c_int) as uint64_t)
}
unsafe extern "C" fn x264_pixel_hadamard_ac_8x8(
    mut pix: *mut pixel,
    mut stride: intptr_t,
) -> uint64_t {
    let mut sum: uint64_t = pixel_hadamard_ac(pix, stride);
    if 8 as libc::c_int == 16 as libc::c_int {
        sum = sum.wrapping_add(pixel_hadamard_ac(
            pix.offset(8 as libc::c_int as isize),
            stride,
        ));
    }
    if 8 as libc::c_int == 16 as libc::c_int {
        sum = sum.wrapping_add(pixel_hadamard_ac(
            pix.offset((8 as libc::c_int as intptr_t * stride) as isize),
            stride,
        ));
    }
    if 8 as libc::c_int == 16 as libc::c_int && 8 as libc::c_int == 16 as libc::c_int {
        sum = sum.wrapping_add(pixel_hadamard_ac(
            pix.offset((8 as libc::c_int as intptr_t * stride) as isize)
                .offset(8 as libc::c_int as isize),
            stride,
        ));
    }
    ((sum >> 34 as libc::c_int) << 32 as libc::c_int)
        .wrapping_add((sum as uint32_t >> 1 as libc::c_int) as uint64_t)
}
unsafe extern "C" fn x264_pixel_sad_x4_16x16(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut pix3: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut libc::c_int,
) {
    *scores.offset(0 as libc::c_int as isize) =
        x264_pixel_sad_16x16(fenc, 16 as libc::c_int as intptr_t, pix0, i_stride);
    *scores.offset(1 as libc::c_int as isize) =
        x264_pixel_sad_16x16(fenc, 16 as libc::c_int as intptr_t, pix1, i_stride);
    *scores.offset(2 as libc::c_int as isize) =
        x264_pixel_sad_16x16(fenc, 16 as libc::c_int as intptr_t, pix2, i_stride);
    *scores.offset(3 as libc::c_int as isize) =
        x264_pixel_sad_16x16(fenc, 16 as libc::c_int as intptr_t, pix3, i_stride);
}
unsafe extern "C" fn x264_pixel_sad_x3_16x16(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut libc::c_int,
) {
    *scores.offset(0 as libc::c_int as isize) =
        x264_pixel_sad_16x16(fenc, 16 as libc::c_int as intptr_t, pix0, i_stride);
    *scores.offset(1 as libc::c_int as isize) =
        x264_pixel_sad_16x16(fenc, 16 as libc::c_int as intptr_t, pix1, i_stride);
    *scores.offset(2 as libc::c_int as isize) =
        x264_pixel_sad_16x16(fenc, 16 as libc::c_int as intptr_t, pix2, i_stride);
}
unsafe extern "C" fn x264_pixel_sad_x4_16x8(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut pix3: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut libc::c_int,
) {
    *scores.offset(0 as libc::c_int as isize) =
        x264_pixel_sad_16x8(fenc, 16 as libc::c_int as intptr_t, pix0, i_stride);
    *scores.offset(1 as libc::c_int as isize) =
        x264_pixel_sad_16x8(fenc, 16 as libc::c_int as intptr_t, pix1, i_stride);
    *scores.offset(2 as libc::c_int as isize) =
        x264_pixel_sad_16x8(fenc, 16 as libc::c_int as intptr_t, pix2, i_stride);
    *scores.offset(3 as libc::c_int as isize) =
        x264_pixel_sad_16x8(fenc, 16 as libc::c_int as intptr_t, pix3, i_stride);
}
unsafe extern "C" fn x264_pixel_sad_x3_16x8(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut libc::c_int,
) {
    *scores.offset(0 as libc::c_int as isize) =
        x264_pixel_sad_16x8(fenc, 16 as libc::c_int as intptr_t, pix0, i_stride);
    *scores.offset(1 as libc::c_int as isize) =
        x264_pixel_sad_16x8(fenc, 16 as libc::c_int as intptr_t, pix1, i_stride);
    *scores.offset(2 as libc::c_int as isize) =
        x264_pixel_sad_16x8(fenc, 16 as libc::c_int as intptr_t, pix2, i_stride);
}
unsafe extern "C" fn x264_pixel_sad_x4_8x16(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut pix3: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut libc::c_int,
) {
    *scores.offset(0 as libc::c_int as isize) =
        x264_pixel_sad_8x16(fenc, 16 as libc::c_int as intptr_t, pix0, i_stride);
    *scores.offset(1 as libc::c_int as isize) =
        x264_pixel_sad_8x16(fenc, 16 as libc::c_int as intptr_t, pix1, i_stride);
    *scores.offset(2 as libc::c_int as isize) =
        x264_pixel_sad_8x16(fenc, 16 as libc::c_int as intptr_t, pix2, i_stride);
    *scores.offset(3 as libc::c_int as isize) =
        x264_pixel_sad_8x16(fenc, 16 as libc::c_int as intptr_t, pix3, i_stride);
}
unsafe extern "C" fn x264_pixel_sad_x3_8x16(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut libc::c_int,
) {
    *scores.offset(0 as libc::c_int as isize) =
        x264_pixel_sad_8x16(fenc, 16 as libc::c_int as intptr_t, pix0, i_stride);
    *scores.offset(1 as libc::c_int as isize) =
        x264_pixel_sad_8x16(fenc, 16 as libc::c_int as intptr_t, pix1, i_stride);
    *scores.offset(2 as libc::c_int as isize) =
        x264_pixel_sad_8x16(fenc, 16 as libc::c_int as intptr_t, pix2, i_stride);
}
unsafe extern "C" fn x264_pixel_sad_x3_8x8(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut libc::c_int,
) {
    *scores.offset(0 as libc::c_int as isize) =
        x264_pixel_sad_8x8(fenc, 16 as libc::c_int as intptr_t, pix0, i_stride);
    *scores.offset(1 as libc::c_int as isize) =
        x264_pixel_sad_8x8(fenc, 16 as libc::c_int as intptr_t, pix1, i_stride);
    *scores.offset(2 as libc::c_int as isize) =
        x264_pixel_sad_8x8(fenc, 16 as libc::c_int as intptr_t, pix2, i_stride);
}
unsafe extern "C" fn x264_pixel_sad_x4_8x8(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut pix3: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut libc::c_int,
) {
    *scores.offset(0 as libc::c_int as isize) =
        x264_pixel_sad_8x8(fenc, 16 as libc::c_int as intptr_t, pix0, i_stride);
    *scores.offset(1 as libc::c_int as isize) =
        x264_pixel_sad_8x8(fenc, 16 as libc::c_int as intptr_t, pix1, i_stride);
    *scores.offset(2 as libc::c_int as isize) =
        x264_pixel_sad_8x8(fenc, 16 as libc::c_int as intptr_t, pix2, i_stride);
    *scores.offset(3 as libc::c_int as isize) =
        x264_pixel_sad_8x8(fenc, 16 as libc::c_int as intptr_t, pix3, i_stride);
}
unsafe extern "C" fn x264_pixel_sad_x3_8x4(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut libc::c_int,
) {
    *scores.offset(0 as libc::c_int as isize) =
        x264_pixel_sad_8x4(fenc, 16 as libc::c_int as intptr_t, pix0, i_stride);
    *scores.offset(1 as libc::c_int as isize) =
        x264_pixel_sad_8x4(fenc, 16 as libc::c_int as intptr_t, pix1, i_stride);
    *scores.offset(2 as libc::c_int as isize) =
        x264_pixel_sad_8x4(fenc, 16 as libc::c_int as intptr_t, pix2, i_stride);
}
unsafe extern "C" fn x264_pixel_sad_x4_8x4(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut pix3: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut libc::c_int,
) {
    *scores.offset(0 as libc::c_int as isize) =
        x264_pixel_sad_8x4(fenc, 16 as libc::c_int as intptr_t, pix0, i_stride);
    *scores.offset(1 as libc::c_int as isize) =
        x264_pixel_sad_8x4(fenc, 16 as libc::c_int as intptr_t, pix1, i_stride);
    *scores.offset(2 as libc::c_int as isize) =
        x264_pixel_sad_8x4(fenc, 16 as libc::c_int as intptr_t, pix2, i_stride);
    *scores.offset(3 as libc::c_int as isize) =
        x264_pixel_sad_8x4(fenc, 16 as libc::c_int as intptr_t, pix3, i_stride);
}
unsafe extern "C" fn x264_pixel_sad_x4_4x8(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut pix3: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut libc::c_int,
) {
    *scores.offset(0 as libc::c_int as isize) =
        x264_pixel_sad_4x8(fenc, 16 as libc::c_int as intptr_t, pix0, i_stride);
    *scores.offset(1 as libc::c_int as isize) =
        x264_pixel_sad_4x8(fenc, 16 as libc::c_int as intptr_t, pix1, i_stride);
    *scores.offset(2 as libc::c_int as isize) =
        x264_pixel_sad_4x8(fenc, 16 as libc::c_int as intptr_t, pix2, i_stride);
    *scores.offset(3 as libc::c_int as isize) =
        x264_pixel_sad_4x8(fenc, 16 as libc::c_int as intptr_t, pix3, i_stride);
}
unsafe extern "C" fn x264_pixel_sad_x3_4x8(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut libc::c_int,
) {
    *scores.offset(0 as libc::c_int as isize) =
        x264_pixel_sad_4x8(fenc, 16 as libc::c_int as intptr_t, pix0, i_stride);
    *scores.offset(1 as libc::c_int as isize) =
        x264_pixel_sad_4x8(fenc, 16 as libc::c_int as intptr_t, pix1, i_stride);
    *scores.offset(2 as libc::c_int as isize) =
        x264_pixel_sad_4x8(fenc, 16 as libc::c_int as intptr_t, pix2, i_stride);
}
unsafe extern "C" fn x264_pixel_sad_x4_4x4(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut pix3: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut libc::c_int,
) {
    *scores.offset(0 as libc::c_int as isize) =
        x264_pixel_sad_4x4(fenc, 16 as libc::c_int as intptr_t, pix0, i_stride);
    *scores.offset(1 as libc::c_int as isize) =
        x264_pixel_sad_4x4(fenc, 16 as libc::c_int as intptr_t, pix1, i_stride);
    *scores.offset(2 as libc::c_int as isize) =
        x264_pixel_sad_4x4(fenc, 16 as libc::c_int as intptr_t, pix2, i_stride);
    *scores.offset(3 as libc::c_int as isize) =
        x264_pixel_sad_4x4(fenc, 16 as libc::c_int as intptr_t, pix3, i_stride);
}
unsafe extern "C" fn x264_pixel_sad_x3_4x4(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut libc::c_int,
) {
    *scores.offset(0 as libc::c_int as isize) =
        x264_pixel_sad_4x4(fenc, 16 as libc::c_int as intptr_t, pix0, i_stride);
    *scores.offset(1 as libc::c_int as isize) =
        x264_pixel_sad_4x4(fenc, 16 as libc::c_int as intptr_t, pix1, i_stride);
    *scores.offset(2 as libc::c_int as isize) =
        x264_pixel_sad_4x4(fenc, 16 as libc::c_int as intptr_t, pix2, i_stride);
}
unsafe extern "C" fn x264_pixel_satd_x3_8x16(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut libc::c_int,
) {
    *scores.offset(0 as libc::c_int as isize) =
        x264_pixel_satd_8x16(fenc, 16 as libc::c_int as intptr_t, pix0, i_stride);
    *scores.offset(1 as libc::c_int as isize) =
        x264_pixel_satd_8x16(fenc, 16 as libc::c_int as intptr_t, pix1, i_stride);
    *scores.offset(2 as libc::c_int as isize) =
        x264_pixel_satd_8x16(fenc, 16 as libc::c_int as intptr_t, pix2, i_stride);
}
unsafe extern "C" fn x264_pixel_satd_x3_16x16(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut libc::c_int,
) {
    *scores.offset(0 as libc::c_int as isize) =
        x264_pixel_satd_16x16(fenc, 16 as libc::c_int as intptr_t, pix0, i_stride);
    *scores.offset(1 as libc::c_int as isize) =
        x264_pixel_satd_16x16(fenc, 16 as libc::c_int as intptr_t, pix1, i_stride);
    *scores.offset(2 as libc::c_int as isize) =
        x264_pixel_satd_16x16(fenc, 16 as libc::c_int as intptr_t, pix2, i_stride);
}
unsafe extern "C" fn x264_pixel_satd_x3_16x8(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut libc::c_int,
) {
    *scores.offset(0 as libc::c_int as isize) =
        x264_pixel_satd_16x8(fenc, 16 as libc::c_int as intptr_t, pix0, i_stride);
    *scores.offset(1 as libc::c_int as isize) =
        x264_pixel_satd_16x8(fenc, 16 as libc::c_int as intptr_t, pix1, i_stride);
    *scores.offset(2 as libc::c_int as isize) =
        x264_pixel_satd_16x8(fenc, 16 as libc::c_int as intptr_t, pix2, i_stride);
}
unsafe extern "C" fn x264_pixel_satd_x4_4x4(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut pix3: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut libc::c_int,
) {
    *scores.offset(0 as libc::c_int as isize) =
        x264_pixel_satd_4x4(fenc, 16 as libc::c_int as intptr_t, pix0, i_stride);
    *scores.offset(1 as libc::c_int as isize) =
        x264_pixel_satd_4x4(fenc, 16 as libc::c_int as intptr_t, pix1, i_stride);
    *scores.offset(2 as libc::c_int as isize) =
        x264_pixel_satd_4x4(fenc, 16 as libc::c_int as intptr_t, pix2, i_stride);
    *scores.offset(3 as libc::c_int as isize) =
        x264_pixel_satd_4x4(fenc, 16 as libc::c_int as intptr_t, pix3, i_stride);
}
unsafe extern "C" fn x264_pixel_satd_x4_4x8(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut pix3: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut libc::c_int,
) {
    *scores.offset(0 as libc::c_int as isize) =
        x264_pixel_satd_4x8(fenc, 16 as libc::c_int as intptr_t, pix0, i_stride);
    *scores.offset(1 as libc::c_int as isize) =
        x264_pixel_satd_4x8(fenc, 16 as libc::c_int as intptr_t, pix1, i_stride);
    *scores.offset(2 as libc::c_int as isize) =
        x264_pixel_satd_4x8(fenc, 16 as libc::c_int as intptr_t, pix2, i_stride);
    *scores.offset(3 as libc::c_int as isize) =
        x264_pixel_satd_4x8(fenc, 16 as libc::c_int as intptr_t, pix3, i_stride);
}
unsafe extern "C" fn x264_pixel_satd_x4_8x4(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut pix3: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut libc::c_int,
) {
    *scores.offset(0 as libc::c_int as isize) =
        x264_pixel_satd_8x4(fenc, 16 as libc::c_int as intptr_t, pix0, i_stride);
    *scores.offset(1 as libc::c_int as isize) =
        x264_pixel_satd_8x4(fenc, 16 as libc::c_int as intptr_t, pix1, i_stride);
    *scores.offset(2 as libc::c_int as isize) =
        x264_pixel_satd_8x4(fenc, 16 as libc::c_int as intptr_t, pix2, i_stride);
    *scores.offset(3 as libc::c_int as isize) =
        x264_pixel_satd_8x4(fenc, 16 as libc::c_int as intptr_t, pix3, i_stride);
}
unsafe extern "C" fn x264_pixel_satd_x4_8x8(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut pix3: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut libc::c_int,
) {
    *scores.offset(0 as libc::c_int as isize) =
        x264_pixel_satd_8x8(fenc, 16 as libc::c_int as intptr_t, pix0, i_stride);
    *scores.offset(1 as libc::c_int as isize) =
        x264_pixel_satd_8x8(fenc, 16 as libc::c_int as intptr_t, pix1, i_stride);
    *scores.offset(2 as libc::c_int as isize) =
        x264_pixel_satd_8x8(fenc, 16 as libc::c_int as intptr_t, pix2, i_stride);
    *scores.offset(3 as libc::c_int as isize) =
        x264_pixel_satd_8x8(fenc, 16 as libc::c_int as intptr_t, pix3, i_stride);
}
unsafe extern "C" fn x264_pixel_satd_x4_8x16(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut pix3: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut libc::c_int,
) {
    *scores.offset(0 as libc::c_int as isize) =
        x264_pixel_satd_8x16(fenc, 16 as libc::c_int as intptr_t, pix0, i_stride);
    *scores.offset(1 as libc::c_int as isize) =
        x264_pixel_satd_8x16(fenc, 16 as libc::c_int as intptr_t, pix1, i_stride);
    *scores.offset(2 as libc::c_int as isize) =
        x264_pixel_satd_8x16(fenc, 16 as libc::c_int as intptr_t, pix2, i_stride);
    *scores.offset(3 as libc::c_int as isize) =
        x264_pixel_satd_8x16(fenc, 16 as libc::c_int as intptr_t, pix3, i_stride);
}
unsafe extern "C" fn x264_pixel_satd_x4_16x8(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut pix3: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut libc::c_int,
) {
    *scores.offset(0 as libc::c_int as isize) =
        x264_pixel_satd_16x8(fenc, 16 as libc::c_int as intptr_t, pix0, i_stride);
    *scores.offset(1 as libc::c_int as isize) =
        x264_pixel_satd_16x8(fenc, 16 as libc::c_int as intptr_t, pix1, i_stride);
    *scores.offset(2 as libc::c_int as isize) =
        x264_pixel_satd_16x8(fenc, 16 as libc::c_int as intptr_t, pix2, i_stride);
    *scores.offset(3 as libc::c_int as isize) =
        x264_pixel_satd_16x8(fenc, 16 as libc::c_int as intptr_t, pix3, i_stride);
}
unsafe extern "C" fn x264_pixel_satd_x4_16x16(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut pix3: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut libc::c_int,
) {
    *scores.offset(0 as libc::c_int as isize) =
        x264_pixel_satd_16x16(fenc, 16 as libc::c_int as intptr_t, pix0, i_stride);
    *scores.offset(1 as libc::c_int as isize) =
        x264_pixel_satd_16x16(fenc, 16 as libc::c_int as intptr_t, pix1, i_stride);
    *scores.offset(2 as libc::c_int as isize) =
        x264_pixel_satd_16x16(fenc, 16 as libc::c_int as intptr_t, pix2, i_stride);
    *scores.offset(3 as libc::c_int as isize) =
        x264_pixel_satd_16x16(fenc, 16 as libc::c_int as intptr_t, pix3, i_stride);
}
unsafe extern "C" fn x264_pixel_satd_x3_4x4(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut libc::c_int,
) {
    *scores.offset(0 as libc::c_int as isize) =
        x264_pixel_satd_4x4(fenc, 16 as libc::c_int as intptr_t, pix0, i_stride);
    *scores.offset(1 as libc::c_int as isize) =
        x264_pixel_satd_4x4(fenc, 16 as libc::c_int as intptr_t, pix1, i_stride);
    *scores.offset(2 as libc::c_int as isize) =
        x264_pixel_satd_4x4(fenc, 16 as libc::c_int as intptr_t, pix2, i_stride);
}
unsafe extern "C" fn x264_pixel_satd_x3_4x8(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut libc::c_int,
) {
    *scores.offset(0 as libc::c_int as isize) =
        x264_pixel_satd_4x8(fenc, 16 as libc::c_int as intptr_t, pix0, i_stride);
    *scores.offset(1 as libc::c_int as isize) =
        x264_pixel_satd_4x8(fenc, 16 as libc::c_int as intptr_t, pix1, i_stride);
    *scores.offset(2 as libc::c_int as isize) =
        x264_pixel_satd_4x8(fenc, 16 as libc::c_int as intptr_t, pix2, i_stride);
}
unsafe extern "C" fn x264_pixel_satd_x3_8x4(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut libc::c_int,
) {
    *scores.offset(0 as libc::c_int as isize) =
        x264_pixel_satd_8x4(fenc, 16 as libc::c_int as intptr_t, pix0, i_stride);
    *scores.offset(1 as libc::c_int as isize) =
        x264_pixel_satd_8x4(fenc, 16 as libc::c_int as intptr_t, pix1, i_stride);
    *scores.offset(2 as libc::c_int as isize) =
        x264_pixel_satd_8x4(fenc, 16 as libc::c_int as intptr_t, pix2, i_stride);
}
unsafe extern "C" fn x264_pixel_satd_x3_8x8(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut libc::c_int,
) {
    *scores.offset(0 as libc::c_int as isize) =
        x264_pixel_satd_8x8(fenc, 16 as libc::c_int as intptr_t, pix0, i_stride);
    *scores.offset(1 as libc::c_int as isize) =
        x264_pixel_satd_8x8(fenc, 16 as libc::c_int as intptr_t, pix1, i_stride);
    *scores.offset(2 as libc::c_int as isize) =
        x264_pixel_satd_8x8(fenc, 16 as libc::c_int as intptr_t, pix2, i_stride);
}
unsafe extern "C" fn intra_sad_x3_8x8(
    mut fenc: *mut pixel,
    mut edge: *mut pixel,
    mut res: *mut libc::c_int,
) {
    let mut pix: [pixel; 256] = [0; 256];
    x264_8_predict_8x8_v_c(pix.as_mut_ptr(), edge);
    *res.offset(0 as libc::c_int as isize) = x264_pixel_sad_8x8(
        pix.as_mut_ptr(),
        32 as libc::c_int as intptr_t,
        fenc,
        16 as libc::c_int as intptr_t,
    );
    x264_8_predict_8x8_h_c(pix.as_mut_ptr(), edge);
    *res.offset(1 as libc::c_int as isize) = x264_pixel_sad_8x8(
        pix.as_mut_ptr(),
        32 as libc::c_int as intptr_t,
        fenc,
        16 as libc::c_int as intptr_t,
    );
    x264_8_predict_8x8_dc_c(pix.as_mut_ptr(), edge);
    *res.offset(2 as libc::c_int as isize) = x264_pixel_sad_8x8(
        pix.as_mut_ptr(),
        32 as libc::c_int as intptr_t,
        fenc,
        16 as libc::c_int as intptr_t,
    );
}
unsafe extern "C" fn intra_sa8d_x3_8x8(
    mut fenc: *mut pixel,
    mut edge: *mut pixel,
    mut res: *mut libc::c_int,
) {
    let mut pix: [pixel; 256] = [0; 256];
    x264_8_predict_8x8_v_c(pix.as_mut_ptr(), edge);
    *res.offset(0 as libc::c_int as isize) = x264_pixel_sa8d_8x8(
        pix.as_mut_ptr(),
        32 as libc::c_int as intptr_t,
        fenc,
        16 as libc::c_int as intptr_t,
    );
    x264_8_predict_8x8_h_c(pix.as_mut_ptr(), edge);
    *res.offset(1 as libc::c_int as isize) = x264_pixel_sa8d_8x8(
        pix.as_mut_ptr(),
        32 as libc::c_int as intptr_t,
        fenc,
        16 as libc::c_int as intptr_t,
    );
    x264_8_predict_8x8_dc_c(pix.as_mut_ptr(), edge);
    *res.offset(2 as libc::c_int as isize) = x264_pixel_sa8d_8x8(
        pix.as_mut_ptr(),
        32 as libc::c_int as intptr_t,
        fenc,
        16 as libc::c_int as intptr_t,
    );
}
unsafe extern "C" fn intra_sad_x3_4x4(
    mut fenc: *mut pixel,
    mut fdec: *mut pixel,
    mut res: *mut libc::c_int,
) {
    x264_8_predict_4x4_v_c(fdec);
    *res.offset(0 as libc::c_int as isize) = x264_pixel_sad_4x4(
        fdec,
        32 as libc::c_int as intptr_t,
        fenc,
        16 as libc::c_int as intptr_t,
    );
    x264_8_predict_4x4_h_c(fdec);
    *res.offset(1 as libc::c_int as isize) = x264_pixel_sad_4x4(
        fdec,
        32 as libc::c_int as intptr_t,
        fenc,
        16 as libc::c_int as intptr_t,
    );
    x264_8_predict_4x4_dc_c(fdec);
    *res.offset(2 as libc::c_int as isize) = x264_pixel_sad_4x4(
        fdec,
        32 as libc::c_int as intptr_t,
        fenc,
        16 as libc::c_int as intptr_t,
    );
}
unsafe extern "C" fn intra_satd_x3_4x4(
    mut fenc: *mut pixel,
    mut fdec: *mut pixel,
    mut res: *mut libc::c_int,
) {
    x264_8_predict_4x4_v_c(fdec);
    *res.offset(0 as libc::c_int as isize) = x264_pixel_satd_4x4(
        fdec,
        32 as libc::c_int as intptr_t,
        fenc,
        16 as libc::c_int as intptr_t,
    );
    x264_8_predict_4x4_h_c(fdec);
    *res.offset(1 as libc::c_int as isize) = x264_pixel_satd_4x4(
        fdec,
        32 as libc::c_int as intptr_t,
        fenc,
        16 as libc::c_int as intptr_t,
    );
    x264_8_predict_4x4_dc_c(fdec);
    *res.offset(2 as libc::c_int as isize) = x264_pixel_satd_4x4(
        fdec,
        32 as libc::c_int as intptr_t,
        fenc,
        16 as libc::c_int as intptr_t,
    );
}
unsafe extern "C" fn intra_sad_x3_8x8c(
    mut fenc: *mut pixel,
    mut fdec: *mut pixel,
    mut res: *mut libc::c_int,
) {
    x264_8_predict_8x8c_dc_c(fdec);
    *res.offset(0 as libc::c_int as isize) = x264_pixel_sad_8x8(
        fdec,
        32 as libc::c_int as intptr_t,
        fenc,
        16 as libc::c_int as intptr_t,
    );
    x264_8_predict_8x8c_h_c(fdec);
    *res.offset(1 as libc::c_int as isize) = x264_pixel_sad_8x8(
        fdec,
        32 as libc::c_int as intptr_t,
        fenc,
        16 as libc::c_int as intptr_t,
    );
    x264_8_predict_8x8c_v_c(fdec);
    *res.offset(2 as libc::c_int as isize) = x264_pixel_sad_8x8(
        fdec,
        32 as libc::c_int as intptr_t,
        fenc,
        16 as libc::c_int as intptr_t,
    );
}
unsafe extern "C" fn intra_satd_x3_8x8c(
    mut fenc: *mut pixel,
    mut fdec: *mut pixel,
    mut res: *mut libc::c_int,
) {
    x264_8_predict_8x8c_dc_c(fdec);
    *res.offset(0 as libc::c_int as isize) = x264_pixel_satd_8x8(
        fdec,
        32 as libc::c_int as intptr_t,
        fenc,
        16 as libc::c_int as intptr_t,
    );
    x264_8_predict_8x8c_h_c(fdec);
    *res.offset(1 as libc::c_int as isize) = x264_pixel_satd_8x8(
        fdec,
        32 as libc::c_int as intptr_t,
        fenc,
        16 as libc::c_int as intptr_t,
    );
    x264_8_predict_8x8c_v_c(fdec);
    *res.offset(2 as libc::c_int as isize) = x264_pixel_satd_8x8(
        fdec,
        32 as libc::c_int as intptr_t,
        fenc,
        16 as libc::c_int as intptr_t,
    );
}
unsafe extern "C" fn intra_sad_x3_8x16c(
    mut fenc: *mut pixel,
    mut fdec: *mut pixel,
    mut res: *mut libc::c_int,
) {
    x264_8_predict_8x16c_dc_c(fdec);
    *res.offset(0 as libc::c_int as isize) = x264_pixel_sad_8x16(
        fdec,
        32 as libc::c_int as intptr_t,
        fenc,
        16 as libc::c_int as intptr_t,
    );
    x264_8_predict_8x16c_h_c(fdec);
    *res.offset(1 as libc::c_int as isize) = x264_pixel_sad_8x16(
        fdec,
        32 as libc::c_int as intptr_t,
        fenc,
        16 as libc::c_int as intptr_t,
    );
    x264_8_predict_8x16c_v_c(fdec);
    *res.offset(2 as libc::c_int as isize) = x264_pixel_sad_8x16(
        fdec,
        32 as libc::c_int as intptr_t,
        fenc,
        16 as libc::c_int as intptr_t,
    );
}
unsafe extern "C" fn intra_satd_x3_8x16c(
    mut fenc: *mut pixel,
    mut fdec: *mut pixel,
    mut res: *mut libc::c_int,
) {
    x264_8_predict_8x16c_dc_c(fdec);
    *res.offset(0 as libc::c_int as isize) = x264_pixel_satd_8x16(
        fdec,
        32 as libc::c_int as intptr_t,
        fenc,
        16 as libc::c_int as intptr_t,
    );
    x264_8_predict_8x16c_h_c(fdec);
    *res.offset(1 as libc::c_int as isize) = x264_pixel_satd_8x16(
        fdec,
        32 as libc::c_int as intptr_t,
        fenc,
        16 as libc::c_int as intptr_t,
    );
    x264_8_predict_8x16c_v_c(fdec);
    *res.offset(2 as libc::c_int as isize) = x264_pixel_satd_8x16(
        fdec,
        32 as libc::c_int as intptr_t,
        fenc,
        16 as libc::c_int as intptr_t,
    );
}
unsafe extern "C" fn intra_sad_x3_16x16(
    mut fenc: *mut pixel,
    mut fdec: *mut pixel,
    mut res: *mut libc::c_int,
) {
    x264_8_predict_16x16_v_c(fdec);
    *res.offset(0 as libc::c_int as isize) = x264_pixel_sad_16x16(
        fdec,
        32 as libc::c_int as intptr_t,
        fenc,
        16 as libc::c_int as intptr_t,
    );
    x264_8_predict_16x16_h_c(fdec);
    *res.offset(1 as libc::c_int as isize) = x264_pixel_sad_16x16(
        fdec,
        32 as libc::c_int as intptr_t,
        fenc,
        16 as libc::c_int as intptr_t,
    );
    x264_8_predict_16x16_dc_c(fdec);
    *res.offset(2 as libc::c_int as isize) = x264_pixel_sad_16x16(
        fdec,
        32 as libc::c_int as intptr_t,
        fenc,
        16 as libc::c_int as intptr_t,
    );
}
unsafe extern "C" fn intra_satd_x3_16x16(
    mut fenc: *mut pixel,
    mut fdec: *mut pixel,
    mut res: *mut libc::c_int,
) {
    x264_8_predict_16x16_v_c(fdec);
    *res.offset(0 as libc::c_int as isize) = x264_pixel_satd_16x16(
        fdec,
        32 as libc::c_int as intptr_t,
        fenc,
        16 as libc::c_int as intptr_t,
    );
    x264_8_predict_16x16_h_c(fdec);
    *res.offset(1 as libc::c_int as isize) = x264_pixel_satd_16x16(
        fdec,
        32 as libc::c_int as intptr_t,
        fenc,
        16 as libc::c_int as intptr_t,
    );
    x264_8_predict_16x16_dc_c(fdec);
    *res.offset(2 as libc::c_int as isize) = x264_pixel_satd_16x16(
        fdec,
        32 as libc::c_int as intptr_t,
        fenc,
        16 as libc::c_int as intptr_t,
    );
}
unsafe extern "C" fn ssim_4x4x2_core(
    mut pix1: *const pixel,
    mut stride1: intptr_t,
    mut pix2: *const pixel,
    mut stride2: intptr_t,
    mut sums: *mut [libc::c_int; 4],
) {
    let mut z: libc::c_int = 0 as libc::c_int;
    while z < 2 as libc::c_int {
        let mut s1: uint32_t = 0 as libc::c_int as uint32_t;
        let mut s2: uint32_t = 0 as libc::c_int as uint32_t;
        let mut ss: uint32_t = 0 as libc::c_int as uint32_t;
        let mut s12: uint32_t = 0 as libc::c_int as uint32_t;
        let mut y: libc::c_int = 0 as libc::c_int;
        while y < 4 as libc::c_int {
            let mut x: libc::c_int = 0 as libc::c_int;
            while x < 4 as libc::c_int {
                let mut a: libc::c_int =
                    *pix1.offset((x as intptr_t + y as intptr_t * stride1) as isize) as libc::c_int;
                let mut b: libc::c_int =
                    *pix2.offset((x as intptr_t + y as intptr_t * stride2) as isize) as libc::c_int;
                s1 = s1.wrapping_add(a as uint32_t);
                s2 = s2.wrapping_add(b as uint32_t);
                ss = ss.wrapping_add((a * a) as uint32_t);
                ss = ss.wrapping_add((b * b) as uint32_t);
                s12 = s12.wrapping_add((a * b) as uint32_t);
                x += 1;
                x;
            }
            y += 1;
            y;
        }
        (*sums.offset(z as isize))[0 as libc::c_int as usize] = s1 as libc::c_int;
        (*sums.offset(z as isize))[1 as libc::c_int as usize] = s2 as libc::c_int;
        (*sums.offset(z as isize))[2 as libc::c_int as usize] = ss as libc::c_int;
        (*sums.offset(z as isize))[3 as libc::c_int as usize] = s12 as libc::c_int;
        pix1 = pix1.offset(4 as libc::c_int as isize);
        pix2 = pix2.offset(4 as libc::c_int as isize);
        z += 1;
        z;
    }
}
unsafe extern "C" fn ssim_end1(
    mut s1: libc::c_int,
    mut s2: libc::c_int,
    mut ss: libc::c_int,
    mut s12: libc::c_int,
) -> libc::c_float {
    static mut ssim_c1: libc::c_int = (0.01f64
        * 0.01f64
        * (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int) as libc::c_double
        * (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int) as libc::c_double
        * 64 as libc::c_int as libc::c_double
        + 0.5f64) as libc::c_int;
    static mut ssim_c2: libc::c_int = (0.03f64
        * 0.03f64
        * (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int) as libc::c_double
        * (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int) as libc::c_double
        * 64 as libc::c_int as libc::c_double
        * 63 as libc::c_int as libc::c_double
        + 0.5f64) as libc::c_int;
    let mut fs1: libc::c_int = s1;
    let mut fs2: libc::c_int = s2;
    let mut fss: libc::c_int = ss;
    let mut fs12: libc::c_int = s12;
    let mut vars: libc::c_int = fss * 64 as libc::c_int - fs1 * fs1 - fs2 * fs2;
    let mut covar: libc::c_int = fs12 * 64 as libc::c_int - fs1 * fs2;
    (2 as libc::c_int * fs1 * fs2 + ssim_c1) as libc::c_float
        * (2 as libc::c_int * covar + ssim_c2) as libc::c_float
        / ((fs1 * fs1 + fs2 * fs2 + ssim_c1) as libc::c_float * (vars + ssim_c2) as libc::c_float)
}
unsafe extern "C" fn ssim_end4(
    mut sum0: *mut [libc::c_int; 4],
    mut sum1: *mut [libc::c_int; 4],
    mut width: libc::c_int,
) -> libc::c_float {
    let mut ssim: libc::c_float = 0.0f64 as libc::c_float;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < width {
        ssim += ssim_end1(
            (*sum0.offset(i as isize))[0 as libc::c_int as usize]
                + (*sum0.offset((i + 1 as libc::c_int) as isize))[0 as libc::c_int as usize]
                + (*sum1.offset(i as isize))[0 as libc::c_int as usize]
                + (*sum1.offset((i + 1 as libc::c_int) as isize))[0 as libc::c_int as usize],
            (*sum0.offset(i as isize))[1 as libc::c_int as usize]
                + (*sum0.offset((i + 1 as libc::c_int) as isize))[1 as libc::c_int as usize]
                + (*sum1.offset(i as isize))[1 as libc::c_int as usize]
                + (*sum1.offset((i + 1 as libc::c_int) as isize))[1 as libc::c_int as usize],
            (*sum0.offset(i as isize))[2 as libc::c_int as usize]
                + (*sum0.offset((i + 1 as libc::c_int) as isize))[2 as libc::c_int as usize]
                + (*sum1.offset(i as isize))[2 as libc::c_int as usize]
                + (*sum1.offset((i + 1 as libc::c_int) as isize))[2 as libc::c_int as usize],
            (*sum0.offset(i as isize))[3 as libc::c_int as usize]
                + (*sum0.offset((i + 1 as libc::c_int) as isize))[3 as libc::c_int as usize]
                + (*sum1.offset(i as isize))[3 as libc::c_int as usize]
                + (*sum1.offset((i + 1 as libc::c_int) as isize))[3 as libc::c_int as usize],
        );
        i += 1;
        i;
    }
    ssim
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_pixel_ssim_wxh(
    mut pf: *mut x264_pixel_function_t,
    mut pix1: *mut pixel,
    mut stride1: intptr_t,
    mut pix2: *mut pixel,
    mut stride2: intptr_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut buf: *mut libc::c_void,
    mut cnt: *mut libc::c_int,
) -> libc::c_float {
    let mut z: libc::c_int = 0 as libc::c_int;
    let mut ssim: libc::c_float = 0.0f64 as libc::c_float;
    let mut sum0: *mut [libc::c_int; 4] = buf as *mut [libc::c_int; 4];
    let mut sum1: *mut [libc::c_int; 4] = sum0
        .offset((width >> 2 as libc::c_int) as isize)
        .offset(3 as libc::c_int as isize);
    width >>= 2 as libc::c_int;
    height >>= 2 as libc::c_int;
    let mut y: libc::c_int = 1 as libc::c_int;
    while y < height {
        while z <= y {
            let mut t: *mut libc::c_void = sum0 as *mut libc::c_void;
            sum0 = sum1;
            sum1 = t as *mut [libc::c_int; 4];
            let mut x: libc::c_int = 0 as libc::c_int;
            while x < width {
                ((*pf).ssim_4x4x2_core).expect("non-null function pointer")(
                    &mut *pix1.offset(
                        (4 as libc::c_int as intptr_t * (x as intptr_t + z as intptr_t * stride1))
                            as isize,
                    ),
                    stride1,
                    &mut *pix2.offset(
                        (4 as libc::c_int as intptr_t * (x as intptr_t + z as intptr_t * stride2))
                            as isize,
                    ),
                    stride2,
                    &mut *sum0.offset(x as isize),
                );
                x += 2 as libc::c_int;
            }
            z += 1;
            z;
        }
        let mut x_0: libc::c_int = 0 as libc::c_int;
        while x_0 < width - 1 as libc::c_int {
            ssim += ((*pf).ssim_end4).expect("non-null function pointer")(
                sum0.offset(x_0 as isize),
                sum1.offset(x_0 as isize),
                if (4 as libc::c_int) < width - x_0 - 1 as libc::c_int {
                    4 as libc::c_int
                } else {
                    width - x_0 - 1 as libc::c_int
                },
            );
            x_0 += 4 as libc::c_int;
        }
        y += 1;
        y;
    }
    *cnt = (height - 1 as libc::c_int) * (width - 1 as libc::c_int);
    ssim
}
unsafe extern "C" fn pixel_vsad(
    mut src: *mut pixel,
    mut stride: intptr_t,
    mut height: libc::c_int,
) -> libc::c_int {
    let mut score: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 1 as libc::c_int;
    while i < height {
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < 16 as libc::c_int {
            score += abs(*src.offset(j as isize) as libc::c_int
                - *src.offset((j as intptr_t + stride) as isize) as libc::c_int);
            j += 1;
            j;
        }
        i += 1;
        i;
        src = src.offset(stride as isize);
    }
    score
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_field_vsad(
    mut h: *mut x264_t,
    mut mb_x: libc::c_int,
    mut mb_y: libc::c_int,
) -> libc::c_int {
    let mut score_field: libc::c_int = 0;
    let mut score_frame: libc::c_int = 0;
    let mut stride: libc::c_int = (*(*h).fenc).i_stride[0 as libc::c_int as usize];
    let mut mb_stride: libc::c_int = (*h).mb.i_mb_stride;
    let mut fenc: *mut pixel = ((*(*h).fenc).plane[0 as libc::c_int as usize])
        .offset((16 as libc::c_int * (mb_x + mb_y * stride)) as isize);
    let mut mb_xy: libc::c_int = mb_x + mb_y * mb_stride;
    let mut mbpair_height: libc::c_int =
        if ((*h).param.i_height - mb_y * 16 as libc::c_int) < 32 as libc::c_int {
            (*h).param.i_height - mb_y * 16 as libc::c_int
        } else {
            32 as libc::c_int
        };
    score_frame = ((*h).pixf.vsad).expect("non-null function pointer")(
        fenc,
        stride as intptr_t,
        mbpair_height,
    );
    score_field = ((*h).pixf.vsad).expect("non-null function pointer")(
        fenc,
        (stride * 2 as libc::c_int) as intptr_t,
        mbpair_height >> 1 as libc::c_int,
    );
    score_field += ((*h).pixf.vsad).expect("non-null function pointer")(
        fenc.offset(stride as isize),
        (stride * 2 as libc::c_int) as intptr_t,
        mbpair_height >> 1 as libc::c_int,
    );
    if mb_x > 0 as libc::c_int {
        score_field += 512 as libc::c_int
            - *((*h).mb.field).offset((mb_xy - 1 as libc::c_int) as isize) as libc::c_int
                * 1024 as libc::c_int;
    }
    if mb_y > 0 as libc::c_int {
        score_field += 512 as libc::c_int
            - *((*h).mb.field).offset((mb_xy - mb_stride) as isize) as libc::c_int
                * 1024 as libc::c_int;
    }
    (score_field < score_frame) as libc::c_int
}
unsafe extern "C" fn pixel_asd8(
    mut pix1: *mut pixel,
    mut stride1: intptr_t,
    mut pix2: *mut pixel,
    mut stride2: intptr_t,
    mut height: libc::c_int,
) -> libc::c_int {
    let mut sum: libc::c_int = 0 as libc::c_int;
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < height {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < 8 as libc::c_int {
            sum +=
                *pix1.offset(x as isize) as libc::c_int - *pix2.offset(x as isize) as libc::c_int;
            x += 1;
            x;
        }
        y += 1;
        y;
        pix1 = pix1.offset(stride1 as isize);
        pix2 = pix2.offset(stride2 as isize);
    }
    abs(sum)
}
unsafe extern "C" fn x264_pixel_ads4(
    mut enc_dc: *mut libc::c_int,
    mut sums: *mut uint16_t,
    mut delta: libc::c_int,
    mut cost_mvx: *mut uint16_t,
    mut mvs: *mut int16_t,
    mut width: libc::c_int,
    mut thresh: libc::c_int,
) -> libc::c_int {
    let mut nmv: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < width {
        let mut ads: libc::c_int = abs(*enc_dc.offset(0 as libc::c_int as isize)
            - *sums.offset(0 as libc::c_int as isize) as libc::c_int)
            + abs(*enc_dc.offset(1 as libc::c_int as isize)
                - *sums.offset(8 as libc::c_int as isize) as libc::c_int)
            + abs(*enc_dc.offset(2 as libc::c_int as isize)
                - *sums.offset(delta as isize) as libc::c_int)
            + abs(*enc_dc.offset(3 as libc::c_int as isize)
                - *sums.offset((delta + 8 as libc::c_int) as isize) as libc::c_int)
            + *cost_mvx.offset(i as isize) as libc::c_int;
        if ads < thresh {
            let fresh0 = nmv;
            nmv += 1;
            *mvs.offset(fresh0 as isize) = i as int16_t;
        }
        i += 1;
        i;
        sums = sums.offset(1);
        sums;
    }
    nmv
}
unsafe extern "C" fn x264_pixel_ads2(
    mut enc_dc: *mut libc::c_int,
    mut sums: *mut uint16_t,
    mut delta: libc::c_int,
    mut cost_mvx: *mut uint16_t,
    mut mvs: *mut int16_t,
    mut width: libc::c_int,
    mut thresh: libc::c_int,
) -> libc::c_int {
    let mut nmv: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < width {
        let mut ads: libc::c_int = abs(*enc_dc.offset(0 as libc::c_int as isize)
            - *sums.offset(0 as libc::c_int as isize) as libc::c_int)
            + abs(*enc_dc.offset(1 as libc::c_int as isize)
                - *sums.offset(delta as isize) as libc::c_int)
            + *cost_mvx.offset(i as isize) as libc::c_int;
        if ads < thresh {
            let fresh1 = nmv;
            nmv += 1;
            *mvs.offset(fresh1 as isize) = i as int16_t;
        }
        i += 1;
        i;
        sums = sums.offset(1);
        sums;
    }
    nmv
}
unsafe extern "C" fn x264_pixel_ads1(
    mut enc_dc: *mut libc::c_int,
    mut sums: *mut uint16_t,
    mut delta: libc::c_int,
    mut cost_mvx: *mut uint16_t,
    mut mvs: *mut int16_t,
    mut width: libc::c_int,
    mut thresh: libc::c_int,
) -> libc::c_int {
    let mut nmv: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < width {
        let mut ads: libc::c_int = abs(*enc_dc.offset(0 as libc::c_int as isize)
            - *sums.offset(0 as libc::c_int as isize) as libc::c_int)
            + *cost_mvx.offset(i as isize) as libc::c_int;
        if ads < thresh {
            let fresh2 = nmv;
            nmv += 1;
            *mvs.offset(fresh2 as isize) = i as int16_t;
        }
        i += 1;
        i;
        sums = sums.offset(1);
        sums;
    }
    nmv
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_pixel_init(
    mut cpu: uint32_t,
    mut pixf: *mut x264_pixel_function_t,
) {
    memset(
        pixf as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<x264_pixel_function_t>() as libc::c_ulong,
    );
    (*pixf).sad[PIXEL_16x16 as libc::c_int as usize] = Some(
        x264_pixel_sad_16x16
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> libc::c_int,
    );
    (*pixf).sad[PIXEL_16x8 as libc::c_int as usize] = Some(
        x264_pixel_sad_16x8
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> libc::c_int,
    );
    (*pixf).sad[PIXEL_8x16 as libc::c_int as usize] = Some(
        x264_pixel_sad_8x16
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> libc::c_int,
    );
    (*pixf).sad[PIXEL_8x8 as libc::c_int as usize] = Some(
        x264_pixel_sad_8x8
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> libc::c_int,
    );
    (*pixf).sad[PIXEL_8x4 as libc::c_int as usize] = Some(
        x264_pixel_sad_8x4
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> libc::c_int,
    );
    (*pixf).sad[PIXEL_4x8 as libc::c_int as usize] = Some(
        x264_pixel_sad_4x8
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> libc::c_int,
    );
    (*pixf).sad[PIXEL_4x4 as libc::c_int as usize] = Some(
        x264_pixel_sad_4x4
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> libc::c_int,
    );
    (*pixf).sad[PIXEL_4x16 as libc::c_int as usize] = Some(
        x264_pixel_sad_4x16
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> libc::c_int,
    );
    (*pixf).sad_aligned[PIXEL_16x16 as libc::c_int as usize] = Some(
        x264_pixel_sad_16x16
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> libc::c_int,
    );
    (*pixf).sad_aligned[PIXEL_16x8 as libc::c_int as usize] = Some(
        x264_pixel_sad_16x8
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> libc::c_int,
    );
    (*pixf).sad_aligned[PIXEL_8x16 as libc::c_int as usize] = Some(
        x264_pixel_sad_8x16
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> libc::c_int,
    );
    (*pixf).sad_aligned[PIXEL_8x8 as libc::c_int as usize] = Some(
        x264_pixel_sad_8x8
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> libc::c_int,
    );
    (*pixf).sad_aligned[PIXEL_8x4 as libc::c_int as usize] = Some(
        x264_pixel_sad_8x4
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> libc::c_int,
    );
    (*pixf).sad_aligned[PIXEL_4x8 as libc::c_int as usize] = Some(
        x264_pixel_sad_4x8
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> libc::c_int,
    );
    (*pixf).sad_aligned[PIXEL_4x4 as libc::c_int as usize] = Some(
        x264_pixel_sad_4x4
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> libc::c_int,
    );
    (*pixf).sad_aligned[PIXEL_4x16 as libc::c_int as usize] = Some(
        x264_pixel_sad_4x16
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> libc::c_int,
    );
    (*pixf).sad_x3[PIXEL_16x16 as libc::c_int as usize] = Some(
        x264_pixel_sad_x3_16x16
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut libc::c_int,
            ) -> (),
    );
    (*pixf).sad_x3[PIXEL_16x8 as libc::c_int as usize] = Some(
        x264_pixel_sad_x3_16x8
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut libc::c_int,
            ) -> (),
    );
    (*pixf).sad_x3[PIXEL_8x16 as libc::c_int as usize] = Some(
        x264_pixel_sad_x3_8x16
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut libc::c_int,
            ) -> (),
    );
    (*pixf).sad_x3[PIXEL_8x8 as libc::c_int as usize] = Some(
        x264_pixel_sad_x3_8x8
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut libc::c_int,
            ) -> (),
    );
    (*pixf).sad_x3[PIXEL_8x4 as libc::c_int as usize] = Some(
        x264_pixel_sad_x3_8x4
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut libc::c_int,
            ) -> (),
    );
    (*pixf).sad_x3[PIXEL_4x8 as libc::c_int as usize] = Some(
        x264_pixel_sad_x3_4x8
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut libc::c_int,
            ) -> (),
    );
    (*pixf).sad_x3[PIXEL_4x4 as libc::c_int as usize] = Some(
        x264_pixel_sad_x3_4x4
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut libc::c_int,
            ) -> (),
    );
    (*pixf).sad_x4[PIXEL_16x16 as libc::c_int as usize] = Some(
        x264_pixel_sad_x4_16x16
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut libc::c_int,
            ) -> (),
    );
    (*pixf).sad_x4[PIXEL_16x8 as libc::c_int as usize] = Some(
        x264_pixel_sad_x4_16x8
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut libc::c_int,
            ) -> (),
    );
    (*pixf).sad_x4[PIXEL_8x16 as libc::c_int as usize] = Some(
        x264_pixel_sad_x4_8x16
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut libc::c_int,
            ) -> (),
    );
    (*pixf).sad_x4[PIXEL_8x8 as libc::c_int as usize] = Some(
        x264_pixel_sad_x4_8x8
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut libc::c_int,
            ) -> (),
    );
    (*pixf).sad_x4[PIXEL_8x4 as libc::c_int as usize] = Some(
        x264_pixel_sad_x4_8x4
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut libc::c_int,
            ) -> (),
    );
    (*pixf).sad_x4[PIXEL_4x8 as libc::c_int as usize] = Some(
        x264_pixel_sad_x4_4x8
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut libc::c_int,
            ) -> (),
    );
    (*pixf).sad_x4[PIXEL_4x4 as libc::c_int as usize] = Some(
        x264_pixel_sad_x4_4x4
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut libc::c_int,
            ) -> (),
    );
    (*pixf).ssd[PIXEL_16x16 as libc::c_int as usize] = Some(
        x264_pixel_ssd_16x16
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> libc::c_int,
    );
    (*pixf).ssd[PIXEL_16x8 as libc::c_int as usize] = Some(
        x264_pixel_ssd_16x8
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> libc::c_int,
    );
    (*pixf).ssd[PIXEL_8x16 as libc::c_int as usize] = Some(
        x264_pixel_ssd_8x16
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> libc::c_int,
    );
    (*pixf).ssd[PIXEL_8x8 as libc::c_int as usize] = Some(
        x264_pixel_ssd_8x8
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> libc::c_int,
    );
    (*pixf).ssd[PIXEL_8x4 as libc::c_int as usize] = Some(
        x264_pixel_ssd_8x4
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> libc::c_int,
    );
    (*pixf).ssd[PIXEL_4x8 as libc::c_int as usize] = Some(
        x264_pixel_ssd_4x8
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> libc::c_int,
    );
    (*pixf).ssd[PIXEL_4x4 as libc::c_int as usize] = Some(
        x264_pixel_ssd_4x4
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> libc::c_int,
    );
    (*pixf).ssd[PIXEL_4x16 as libc::c_int as usize] = Some(
        x264_pixel_ssd_4x16
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> libc::c_int,
    );
    (*pixf).satd[PIXEL_16x16 as libc::c_int as usize] = Some(
        x264_pixel_satd_16x16
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> libc::c_int,
    );
    (*pixf).satd[PIXEL_16x8 as libc::c_int as usize] = Some(
        x264_pixel_satd_16x8
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> libc::c_int,
    );
    (*pixf).satd[PIXEL_8x16 as libc::c_int as usize] = Some(
        x264_pixel_satd_8x16
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> libc::c_int,
    );
    (*pixf).satd[PIXEL_8x8 as libc::c_int as usize] = Some(
        x264_pixel_satd_8x8
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> libc::c_int,
    );
    (*pixf).satd[PIXEL_8x4 as libc::c_int as usize] = Some(
        x264_pixel_satd_8x4
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> libc::c_int,
    );
    (*pixf).satd[PIXEL_4x8 as libc::c_int as usize] = Some(
        x264_pixel_satd_4x8
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> libc::c_int,
    );
    (*pixf).satd[PIXEL_4x4 as libc::c_int as usize] = Some(
        x264_pixel_satd_4x4
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> libc::c_int,
    );
    (*pixf).satd[PIXEL_4x16 as libc::c_int as usize] = Some(
        x264_pixel_satd_4x16
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> libc::c_int,
    );
    (*pixf).satd_x3[PIXEL_16x16 as libc::c_int as usize] = Some(
        x264_pixel_satd_x3_16x16
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut libc::c_int,
            ) -> (),
    );
    (*pixf).satd_x3[PIXEL_16x8 as libc::c_int as usize] = Some(
        x264_pixel_satd_x3_16x8
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut libc::c_int,
            ) -> (),
    );
    (*pixf).satd_x3[PIXEL_8x16 as libc::c_int as usize] = Some(
        x264_pixel_satd_x3_8x16
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut libc::c_int,
            ) -> (),
    );
    (*pixf).satd_x3[PIXEL_8x8 as libc::c_int as usize] = Some(
        x264_pixel_satd_x3_8x8
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut libc::c_int,
            ) -> (),
    );
    (*pixf).satd_x3[PIXEL_8x4 as libc::c_int as usize] = Some(
        x264_pixel_satd_x3_8x4
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut libc::c_int,
            ) -> (),
    );
    (*pixf).satd_x3[PIXEL_4x8 as libc::c_int as usize] = Some(
        x264_pixel_satd_x3_4x8
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut libc::c_int,
            ) -> (),
    );
    (*pixf).satd_x3[PIXEL_4x4 as libc::c_int as usize] = Some(
        x264_pixel_satd_x3_4x4
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut libc::c_int,
            ) -> (),
    );
    (*pixf).satd_x4[PIXEL_16x16 as libc::c_int as usize] = Some(
        x264_pixel_satd_x4_16x16
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut libc::c_int,
            ) -> (),
    );
    (*pixf).satd_x4[PIXEL_16x8 as libc::c_int as usize] = Some(
        x264_pixel_satd_x4_16x8
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut libc::c_int,
            ) -> (),
    );
    (*pixf).satd_x4[PIXEL_8x16 as libc::c_int as usize] = Some(
        x264_pixel_satd_x4_8x16
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut libc::c_int,
            ) -> (),
    );
    (*pixf).satd_x4[PIXEL_8x8 as libc::c_int as usize] = Some(
        x264_pixel_satd_x4_8x8
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut libc::c_int,
            ) -> (),
    );
    (*pixf).satd_x4[PIXEL_8x4 as libc::c_int as usize] = Some(
        x264_pixel_satd_x4_8x4
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut libc::c_int,
            ) -> (),
    );
    (*pixf).satd_x4[PIXEL_4x8 as libc::c_int as usize] = Some(
        x264_pixel_satd_x4_4x8
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut libc::c_int,
            ) -> (),
    );
    (*pixf).satd_x4[PIXEL_4x4 as libc::c_int as usize] = Some(
        x264_pixel_satd_x4_4x4
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut libc::c_int,
            ) -> (),
    );
    (*pixf).hadamard_ac[PIXEL_16x16 as libc::c_int as usize] = Some(
        x264_pixel_hadamard_ac_16x16 as unsafe extern "C" fn(*mut pixel, intptr_t) -> uint64_t,
    );
    (*pixf).hadamard_ac[PIXEL_16x8 as libc::c_int as usize] =
        Some(x264_pixel_hadamard_ac_16x8 as unsafe extern "C" fn(*mut pixel, intptr_t) -> uint64_t);
    (*pixf).hadamard_ac[PIXEL_8x16 as libc::c_int as usize] =
        Some(x264_pixel_hadamard_ac_8x16 as unsafe extern "C" fn(*mut pixel, intptr_t) -> uint64_t);
    (*pixf).hadamard_ac[PIXEL_8x8 as libc::c_int as usize] =
        Some(x264_pixel_hadamard_ac_8x8 as unsafe extern "C" fn(*mut pixel, intptr_t) -> uint64_t);
    (*pixf).ads[PIXEL_16x16 as libc::c_int as usize] = Some(
        x264_pixel_ads4
            as unsafe extern "C" fn(
                *mut libc::c_int,
                *mut uint16_t,
                libc::c_int,
                *mut uint16_t,
                *mut int16_t,
                libc::c_int,
                libc::c_int,
            ) -> libc::c_int,
    );
    (*pixf).ads[PIXEL_16x8 as libc::c_int as usize] = Some(
        x264_pixel_ads2
            as unsafe extern "C" fn(
                *mut libc::c_int,
                *mut uint16_t,
                libc::c_int,
                *mut uint16_t,
                *mut int16_t,
                libc::c_int,
                libc::c_int,
            ) -> libc::c_int,
    );
    (*pixf).ads[PIXEL_8x8 as libc::c_int as usize] = Some(
        x264_pixel_ads1
            as unsafe extern "C" fn(
                *mut libc::c_int,
                *mut uint16_t,
                libc::c_int,
                *mut uint16_t,
                *mut int16_t,
                libc::c_int,
                libc::c_int,
            ) -> libc::c_int,
    );
    (*pixf).sa8d[PIXEL_16x16 as libc::c_int as usize] = Some(
        x264_pixel_sa8d_16x16
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> libc::c_int,
    );
    (*pixf).sa8d[PIXEL_8x8 as libc::c_int as usize] = Some(
        x264_pixel_sa8d_8x8
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> libc::c_int,
    );
    (*pixf).var[PIXEL_16x16 as libc::c_int as usize] =
        Some(pixel_var_16x16 as unsafe extern "C" fn(*mut pixel, intptr_t) -> uint64_t);
    (*pixf).var[PIXEL_8x16 as libc::c_int as usize] =
        Some(pixel_var_8x16 as unsafe extern "C" fn(*mut pixel, intptr_t) -> uint64_t);
    (*pixf).var[PIXEL_8x8 as libc::c_int as usize] =
        Some(pixel_var_8x8 as unsafe extern "C" fn(*mut pixel, intptr_t) -> uint64_t);
    (*pixf).var2[PIXEL_8x16 as libc::c_int as usize] = Some(
        pixel_var2_8x16
            as unsafe extern "C" fn(*mut pixel, *mut pixel, *mut libc::c_int) -> libc::c_int,
    );
    (*pixf).var2[PIXEL_8x8 as libc::c_int as usize] = Some(
        pixel_var2_8x8
            as unsafe extern "C" fn(*mut pixel, *mut pixel, *mut libc::c_int) -> libc::c_int,
    );
    (*pixf).ssd_nv12_core = Some(
        pixel_ssd_nv12_core
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                libc::c_int,
                libc::c_int,
                *mut uint64_t,
                *mut uint64_t,
            ) -> (),
    );
    (*pixf).ssim_4x4x2_core = Some(
        ssim_4x4x2_core
            as unsafe extern "C" fn(
                *const pixel,
                intptr_t,
                *const pixel,
                intptr_t,
                *mut [libc::c_int; 4],
            ) -> (),
    );
    (*pixf).ssim_end4 = Some(
        ssim_end4
            as unsafe extern "C" fn(
                *mut [libc::c_int; 4],
                *mut [libc::c_int; 4],
                libc::c_int,
            ) -> libc::c_float,
    );
    (*pixf).vsad =
        Some(pixel_vsad as unsafe extern "C" fn(*mut pixel, intptr_t, libc::c_int) -> libc::c_int);
    (*pixf).asd8 = Some(
        pixel_asd8
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                libc::c_int,
            ) -> libc::c_int,
    );
    (*pixf).intra_sad_x3_4x4 = Some(
        intra_sad_x3_4x4 as unsafe extern "C" fn(*mut pixel, *mut pixel, *mut libc::c_int) -> (),
    );
    (*pixf).intra_satd_x3_4x4 = Some(
        intra_satd_x3_4x4 as unsafe extern "C" fn(*mut pixel, *mut pixel, *mut libc::c_int) -> (),
    );
    (*pixf).intra_sad_x3_8x8 = Some(
        intra_sad_x3_8x8 as unsafe extern "C" fn(*mut pixel, *mut pixel, *mut libc::c_int) -> (),
    );
    (*pixf).intra_sa8d_x3_8x8 = Some(
        intra_sa8d_x3_8x8 as unsafe extern "C" fn(*mut pixel, *mut pixel, *mut libc::c_int) -> (),
    );
    (*pixf).intra_sad_x3_8x8c = Some(
        intra_sad_x3_8x8c as unsafe extern "C" fn(*mut pixel, *mut pixel, *mut libc::c_int) -> (),
    );
    (*pixf).intra_satd_x3_8x8c = Some(
        intra_satd_x3_8x8c as unsafe extern "C" fn(*mut pixel, *mut pixel, *mut libc::c_int) -> (),
    );
    (*pixf).intra_sad_x3_8x16c = Some(
        intra_sad_x3_8x16c as unsafe extern "C" fn(*mut pixel, *mut pixel, *mut libc::c_int) -> (),
    );
    (*pixf).intra_satd_x3_8x16c = Some(
        intra_satd_x3_8x16c as unsafe extern "C" fn(*mut pixel, *mut pixel, *mut libc::c_int) -> (),
    );
    (*pixf).intra_sad_x3_16x16 = Some(
        intra_sad_x3_16x16 as unsafe extern "C" fn(*mut pixel, *mut pixel, *mut libc::c_int) -> (),
    );
    (*pixf).intra_satd_x3_16x16 = Some(
        intra_satd_x3_16x16 as unsafe extern "C" fn(*mut pixel, *mut pixel, *mut libc::c_int) -> (),
    );
    (*pixf).ads[PIXEL_4x8 as libc::c_int as usize] =
        (*pixf).ads[PIXEL_16x8 as libc::c_int as usize];
    (*pixf).ads[PIXEL_8x4 as libc::c_int as usize] = (*pixf).ads[PIXEL_4x8 as libc::c_int as usize];
    (*pixf).ads[PIXEL_8x16 as libc::c_int as usize] =
        (*pixf).ads[PIXEL_8x4 as libc::c_int as usize];
    (*pixf).ads[PIXEL_4x4 as libc::c_int as usize] = (*pixf).ads[PIXEL_8x8 as libc::c_int as usize];
}
