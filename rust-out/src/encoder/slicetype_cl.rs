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
    fn x264_8_log(h: *mut x264_t, i_level: libc::c_int, psz_fmt: *const libc::c_char, _: ...);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn x264_8_weights_analyse(
        h: *mut x264_t,
        fenc: *mut x264_frame_t,
        ref_0: *mut x264_frame_t,
        b_lookahead: libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_opencl_flush(mut h: *mut x264_t) {
    let mut ocl: *mut x264_opencl_function_t = (*h).opencl.ocl;
    ((*ocl).clFinish).expect("non-null function pointer")((*h).opencl.queue);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*h).opencl.num_copies {
        memcpy(
            (*h).opencl.copies[i as usize].dest,
            (*h).opencl.copies[i as usize].src,
            (*h).opencl.copies[i as usize].bytes as libc::c_ulong,
        );
        i += 1;
        i;
    }
    (*h).opencl.num_copies = 0 as libc::c_int;
    (*h).opencl.pl_occupancy = 0 as libc::c_int;
}
unsafe extern "C" fn opencl_alloc_locked(
    mut h: *mut x264_t,
    mut bytes: libc::c_int,
) -> *mut libc::c_void {
    if (*h).opencl.pl_occupancy + bytes
        >= 32 as libc::c_int * 1024 as libc::c_int * 1024 as libc::c_int
    {
        x264_8_opencl_flush(h);
    }
    if bytes < 32 as libc::c_int * 1024 as libc::c_int * 1024 as libc::c_int {
    } else {
        __assert_fail(
            b"bytes < PAGE_LOCKED_BUF_SIZE\0" as *const u8 as *const libc::c_char,
            b"encoder/slicetype-cl.c\0" as *const u8 as *const libc::c_char,
            76 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 41], &[libc::c_char; 41]>(
                b"void *opencl_alloc_locked(x264_t *, int)\0",
            ))
            .as_ptr(),
        );
    }
    'c_29389: {
        if bytes < 32 as libc::c_int * 1024 as libc::c_int * 1024 as libc::c_int {
        } else {
            __assert_fail(
                b"bytes < PAGE_LOCKED_BUF_SIZE\0" as *const u8 as *const libc::c_char,
                b"encoder/slicetype-cl.c\0" as *const u8 as *const libc::c_char,
                76 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 41], &[libc::c_char; 41]>(
                    b"void *opencl_alloc_locked(x264_t *, int)\0",
                ))
                .as_ptr(),
            );
        }
    };
    let mut ptr: *mut libc::c_char =
        ((*h).opencl.page_locked_ptr).offset((*h).opencl.pl_occupancy as isize);
    (*h).opencl.pl_occupancy += bytes;
    ptr as *mut libc::c_void
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_opencl_lowres_init(
    mut h: *mut x264_t,
    mut fenc: *mut x264_frame_t,
    mut lambda: libc::c_int,
) -> libc::c_int {
    if (*fenc).b_intra_calculated != 0 {
        return 0 as libc::c_int;
    }
    (*fenc).b_intra_calculated = 1 as libc::c_int;
    let mut ocl: *mut x264_opencl_function_t = (*h).opencl.ocl;
    let mut luma_length: libc::c_int =
        (*fenc).i_stride[0 as libc::c_int as usize] * (*fenc).i_lines[0 as libc::c_int as usize];
    let mut mb_count: libc::c_int = (*h).mb.i_mb_count;
    let mut status: cl_int = 0;
    if ((*h).opencl.lowres_mv_costs).is_null() {
        let mut width: libc::c_int = (*h).mb.i_mb_width
            * 8 as libc::c_int
            * ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int;
        let mut height: libc::c_int = (*h).mb.i_mb_height
            * 8 as libc::c_int
            * ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int;
        let mut pixel_format: cl_image_format = _cl_image_format {
            image_channel_order: 0,
            image_channel_data_type: 0,
        };
        pixel_format.image_channel_order = 0x10b0 as libc::c_int as cl_channel_order;
        pixel_format.image_channel_data_type = 0x10dc as libc::c_int as cl_channel_type;
        (*h).opencl.weighted_luma_hpel = ((*ocl).clCreateImage2D)
            .expect("non-null function pointer")(
            (*h).opencl.context,
            ((1 as libc::c_int) << 0 as libc::c_int) as cl_mem_flags,
            &mut pixel_format,
            width as size_t,
            height as size_t,
            0 as libc::c_int as size_t,
            std::ptr::null_mut::<libc::c_void>(),
            &mut status,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clCreateImage2D error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            pixel_format.image_channel_order = 0x10b5 as libc::c_int as cl_channel_order;
            pixel_format.image_channel_data_type = 0x10da as libc::c_int as cl_channel_type;
            (*h).opencl.weighted_scaled_images[i as usize] = ((*ocl).clCreateImage2D)
                .expect("non-null function pointer")(
                (*h).opencl.context,
                ((1 as libc::c_int) << 0 as libc::c_int) as cl_mem_flags,
                &mut pixel_format,
                width as size_t,
                height as size_t,
                0 as libc::c_int as size_t,
                std::ptr::null_mut::<libc::c_void>(),
                &mut status,
            );
            if status != 0 as libc::c_int {
                (*h).param.b_opencl = 0 as libc::c_int;
                x264_8_log(
                    h,
                    0 as libc::c_int,
                    b"clCreateImage2D error '%d'\n\0" as *const u8 as *const libc::c_char,
                    status,
                );
                return -(1 as libc::c_int);
            }
            width >>= 1 as libc::c_int;
            height >>= 1 as libc::c_int;
            i += 1;
            i;
        }
        (*h).opencl.lowres_mv_costs = ((*ocl).clCreateBuffer).expect("non-null function pointer")(
            (*h).opencl.context,
            ((1 as libc::c_int) << 0 as libc::c_int) as cl_mem_flags,
            (mb_count as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong),
            std::ptr::null_mut::<libc::c_void>(),
            &mut status,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clCreateBuffer error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        (*h).opencl.lowres_costs[0 as libc::c_int as usize] = ((*ocl).clCreateBuffer)
            .expect("non-null function pointer")(
            (*h).opencl.context,
            ((1 as libc::c_int) << 0 as libc::c_int) as cl_mem_flags,
            (mb_count as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong),
            std::ptr::null_mut::<libc::c_void>(),
            &mut status,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clCreateBuffer error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        (*h).opencl.lowres_costs[1 as libc::c_int as usize] = ((*ocl).clCreateBuffer)
            .expect("non-null function pointer")(
            (*h).opencl.context,
            ((1 as libc::c_int) << 0 as libc::c_int) as cl_mem_flags,
            (mb_count as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong),
            std::ptr::null_mut::<libc::c_void>(),
            &mut status,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clCreateBuffer error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        (*h).opencl.mv_buffers[0 as libc::c_int as usize] = ((*ocl).clCreateBuffer)
            .expect("non-null function pointer")(
            (*h).opencl.context,
            ((1 as libc::c_int) << 0 as libc::c_int) as cl_mem_flags,
            (mb_count as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong),
            std::ptr::null_mut::<libc::c_void>(),
            &mut status,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clCreateBuffer error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        (*h).opencl.mv_buffers[1 as libc::c_int as usize] = ((*ocl).clCreateBuffer)
            .expect("non-null function pointer")(
            (*h).opencl.context,
            ((1 as libc::c_int) << 0 as libc::c_int) as cl_mem_flags,
            (mb_count as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong),
            std::ptr::null_mut::<libc::c_void>(),
            &mut status,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clCreateBuffer error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        (*h).opencl.mvp_buffer = ((*ocl).clCreateBuffer).expect("non-null function pointer")(
            (*h).opencl.context,
            ((1 as libc::c_int) << 0 as libc::c_int) as cl_mem_flags,
            (mb_count as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong),
            std::ptr::null_mut::<libc::c_void>(),
            &mut status,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clCreateBuffer error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        (*h).opencl.frame_stats[0 as libc::c_int as usize] = ((*ocl).clCreateBuffer)
            .expect("non-null function pointer")(
            (*h).opencl.context,
            ((1 as libc::c_int) << 1 as libc::c_int) as cl_mem_flags,
            (4 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
            std::ptr::null_mut::<libc::c_void>(),
            &mut status,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clCreateBuffer error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        (*h).opencl.frame_stats[1 as libc::c_int as usize] = ((*ocl).clCreateBuffer)
            .expect("non-null function pointer")(
            (*h).opencl.context,
            ((1 as libc::c_int) << 1 as libc::c_int) as cl_mem_flags,
            (4 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
            std::ptr::null_mut::<libc::c_void>(),
            &mut status,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clCreateBuffer error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        (*h).opencl.row_satds[0 as libc::c_int as usize] = ((*ocl).clCreateBuffer)
            .expect("non-null function pointer")(
            (*h).opencl.context,
            ((1 as libc::c_int) << 1 as libc::c_int) as cl_mem_flags,
            ((*h).mb.i_mb_height as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
            std::ptr::null_mut::<libc::c_void>(),
            &mut status,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clCreateBuffer error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        (*h).opencl.row_satds[1 as libc::c_int as usize] = ((*ocl).clCreateBuffer)
            .expect("non-null function pointer")(
            (*h).opencl.context,
            ((1 as libc::c_int) << 1 as libc::c_int) as cl_mem_flags,
            ((*h).mb.i_mb_height as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
            std::ptr::null_mut::<libc::c_void>(),
            &mut status,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clCreateBuffer error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        (*h).opencl.luma_16x16_image[0 as libc::c_int as usize] = ((*ocl).clCreateBuffer)
            .expect("non-null function pointer")(
            (*h).opencl.context,
            ((1 as libc::c_int) << 2 as libc::c_int) as cl_mem_flags,
            luma_length as size_t,
            std::ptr::null_mut::<libc::c_void>(),
            &mut status,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clCreateBuffer error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        (*h).opencl.luma_16x16_image[1 as libc::c_int as usize] = ((*ocl).clCreateBuffer)
            .expect("non-null function pointer")(
            (*h).opencl.context,
            ((1 as libc::c_int) << 2 as libc::c_int) as cl_mem_flags,
            luma_length as size_t,
            std::ptr::null_mut::<libc::c_void>(),
            &mut status,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clCreateBuffer error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
    }
    if ((*fenc).opencl.intra_cost).is_null() {
        let mut width_0: libc::c_int = (*h).mb.i_mb_width
            * 8 as libc::c_int
            * ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int;
        let mut height_0: libc::c_int = (*h).mb.i_mb_height
            * 8 as libc::c_int
            * ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int;
        let mut pixel_format_0: cl_image_format = _cl_image_format {
            image_channel_order: 0,
            image_channel_data_type: 0,
        };
        pixel_format_0.image_channel_order = 0x10b0 as libc::c_int as cl_channel_order;
        pixel_format_0.image_channel_data_type = 0x10dc as libc::c_int as cl_channel_type;
        (*fenc).opencl.luma_hpel = ((*ocl).clCreateImage2D).expect("non-null function pointer")(
            (*h).opencl.context,
            ((1 as libc::c_int) << 0 as libc::c_int) as cl_mem_flags,
            &mut pixel_format_0,
            width_0 as size_t,
            height_0 as size_t,
            0 as libc::c_int as size_t,
            std::ptr::null_mut::<libc::c_void>(),
            &mut status,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clCreateImage2D error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < 4 as libc::c_int {
            pixel_format_0.image_channel_order = 0x10b5 as libc::c_int as cl_channel_order;
            pixel_format_0.image_channel_data_type = 0x10da as libc::c_int as cl_channel_type;
            (*fenc).opencl.scaled_image2Ds[i_0 as usize] = ((*ocl).clCreateImage2D)
                .expect("non-null function pointer")(
                (*h).opencl.context,
                ((1 as libc::c_int) << 0 as libc::c_int) as cl_mem_flags,
                &mut pixel_format_0,
                width_0 as size_t,
                height_0 as size_t,
                0 as libc::c_int as size_t,
                std::ptr::null_mut::<libc::c_void>(),
                &mut status,
            );
            if status != 0 as libc::c_int {
                (*h).param.b_opencl = 0 as libc::c_int;
                x264_8_log(
                    h,
                    0 as libc::c_int,
                    b"clCreateImage2D error '%d'\n\0" as *const u8 as *const libc::c_char,
                    status,
                );
                return -(1 as libc::c_int);
            }
            width_0 >>= 1 as libc::c_int;
            height_0 >>= 1 as libc::c_int;
            i_0 += 1;
            i_0;
        }
        (*fenc).opencl.inv_qscale_factor = ((*ocl).clCreateBuffer)
            .expect("non-null function pointer")(
            (*h).opencl.context,
            ((1 as libc::c_int) << 2 as libc::c_int) as cl_mem_flags,
            (mb_count as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong),
            std::ptr::null_mut::<libc::c_void>(),
            &mut status,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clCreateBuffer error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        (*fenc).opencl.intra_cost = ((*ocl).clCreateBuffer).expect("non-null function pointer")(
            (*h).opencl.context,
            ((1 as libc::c_int) << 1 as libc::c_int) as cl_mem_flags,
            (mb_count as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong),
            std::ptr::null_mut::<libc::c_void>(),
            &mut status,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clCreateBuffer error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        (*fenc).opencl.lowres_mvs0 = ((*ocl).clCreateBuffer).expect("non-null function pointer")(
            (*h).opencl.context,
            ((1 as libc::c_int) << 0 as libc::c_int) as cl_mem_flags,
            ((mb_count * 2 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong)
                .wrapping_mul(((*h).param.i_bframe + 1 as libc::c_int) as libc::c_ulong),
            std::ptr::null_mut::<libc::c_void>(),
            &mut status,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clCreateBuffer error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        (*fenc).opencl.lowres_mvs1 = ((*ocl).clCreateBuffer).expect("non-null function pointer")(
            (*h).opencl.context,
            ((1 as libc::c_int) << 0 as libc::c_int) as cl_mem_flags,
            ((mb_count * 2 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong)
                .wrapping_mul(((*h).param.i_bframe + 1 as libc::c_int) as libc::c_ulong),
            std::ptr::null_mut::<libc::c_void>(),
            &mut status,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clCreateBuffer error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        (*fenc).opencl.lowres_mv_costs0 = ((*ocl).clCreateBuffer)
            .expect("non-null function pointer")(
            (*h).opencl.context,
            ((1 as libc::c_int) << 0 as libc::c_int) as cl_mem_flags,
            (mb_count as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong)
                .wrapping_mul(((*h).param.i_bframe + 1 as libc::c_int) as libc::c_ulong),
            std::ptr::null_mut::<libc::c_void>(),
            &mut status,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clCreateBuffer error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        (*fenc).opencl.lowres_mv_costs1 = ((*ocl).clCreateBuffer)
            .expect("non-null function pointer")(
            (*h).opencl.context,
            ((1 as libc::c_int) << 0 as libc::c_int) as cl_mem_flags,
            (mb_count as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong)
                .wrapping_mul(((*h).param.i_bframe + 1 as libc::c_int) as libc::c_ulong),
            std::ptr::null_mut::<libc::c_void>(),
            &mut status,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clCreateBuffer error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
    }
    let mut locked: *mut libc::c_char = opencl_alloc_locked(h, luma_length) as *mut libc::c_char;
    memcpy(
        locked as *mut libc::c_void,
        (*fenc).plane[0 as libc::c_int as usize] as *const libc::c_void,
        luma_length as libc::c_ulong,
    );
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    status = ((*ocl).clEnqueueWriteBuffer).expect("non-null function pointer")(
        (*h).opencl.queue,
        (*h).opencl.luma_16x16_image[(*h).opencl.last_buf as usize],
        0 as libc::c_int as cl_bool,
        0 as libc::c_int as size_t,
        luma_length as size_t,
        locked as *const libc::c_void,
        0 as libc::c_int as cl_uint,
        std::ptr::null::<cl_event>(),
        std::ptr::null_mut::<cl_event>(),
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clEnqueueWriteBuffer error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    let mut gdim: [size_t; 2] = [0; 2];
    if (*h).param.rc.i_aq_mode != 0 && !((*fenc).i_inv_qscale_factor).is_null() {
        let mut size: libc::c_int = ((*h).mb.i_mb_count as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong)
            as libc::c_int;
        locked = opencl_alloc_locked(h, size) as *mut libc::c_char;
        memcpy(
            locked as *mut libc::c_void,
            (*fenc).i_inv_qscale_factor as *const libc::c_void,
            size as libc::c_ulong,
        );
        if (*h).opencl.b_fatal_error != 0 {
            return -(1 as libc::c_int);
        }
        status = ((*ocl).clEnqueueWriteBuffer).expect("non-null function pointer")(
            (*h).opencl.queue,
            (*fenc).opencl.inv_qscale_factor,
            0 as libc::c_int as cl_bool,
            0 as libc::c_int as size_t,
            size as size_t,
            locked as *const libc::c_void,
            0 as libc::c_int as cl_uint,
            std::ptr::null::<cl_event>(),
            std::ptr::null_mut::<cl_event>(),
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            (*h).opencl.b_fatal_error = 1 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clEnqueueWriteBuffer error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
    } else {
        let mut arg: cl_uint = 0 as libc::c_int as cl_uint;
        let mut value: int16_t = 256 as libc::c_int as int16_t;
        if (*h).opencl.b_fatal_error != 0 {
            return -(1 as libc::c_int);
        }
        let fresh0 = arg;
        arg = arg.wrapping_add(1);
        status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
            (*h).opencl.memset_kernel,
            fresh0,
            ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
            &mut (*fenc).opencl.inv_qscale_factor as *mut cl_mem as *const libc::c_void,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            (*h).opencl.b_fatal_error = 1 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        if (*h).opencl.b_fatal_error != 0 {
            return -(1 as libc::c_int);
        }
        let fresh1 = arg;
        arg = arg.wrapping_add(1);
        status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
            (*h).opencl.memset_kernel,
            fresh1,
            ::core::mem::size_of::<int16_t>() as libc::c_ulong,
            &mut value as *mut int16_t as *const libc::c_void,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            (*h).opencl.b_fatal_error = 1 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        gdim[0 as libc::c_int as usize] = (*h).mb.i_mb_count as size_t;
        if (*h).opencl.b_fatal_error != 0 {
            return -(1 as libc::c_int);
        }
        status = ((*ocl).clEnqueueNDRangeKernel).expect("non-null function pointer")(
            (*h).opencl.queue,
            (*h).opencl.memset_kernel,
            1 as libc::c_int as cl_uint,
            std::ptr::null::<size_t>(),
            gdim.as_mut_ptr(),
            std::ptr::null::<size_t>(),
            0 as libc::c_int as cl_uint,
            std::ptr::null::<cl_event>(),
            std::ptr::null_mut::<cl_event>(),
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            (*h).opencl.b_fatal_error = 1 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clEnqueueNDRangeKernel error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
    }
    let mut stride: libc::c_int = (*fenc).i_stride[0 as libc::c_int as usize];
    let mut arg_0: cl_uint = 0 as libc::c_int as cl_uint;
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh2 = arg_0;
    arg_0 = arg_0.wrapping_add(1);
    status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
        (*h).opencl.downscale_hpel_kernel,
        fresh2,
        ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
        &mut *((*h).opencl.luma_16x16_image)
            .as_mut_ptr()
            .offset((*h).opencl.last_buf as isize) as *mut cl_mem as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh3 = arg_0;
    arg_0 = arg_0.wrapping_add(1);
    status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
        (*h).opencl.downscale_hpel_kernel,
        fresh3,
        ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
        &mut *((*fenc).opencl.scaled_image2Ds)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) as *mut cl_mem as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh4 = arg_0;
    arg_0 = arg_0.wrapping_add(1);
    status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
        (*h).opencl.downscale_hpel_kernel,
        fresh4,
        ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
        &mut (*fenc).opencl.luma_hpel as *mut cl_mem as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh5 = arg_0;
    arg_0 = arg_0.wrapping_add(1);
    status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
        (*h).opencl.downscale_hpel_kernel,
        fresh5,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        &mut stride as *mut libc::c_int as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    gdim[0 as libc::c_int as usize] = (8 as libc::c_int * (*h).mb.i_mb_width) as size_t;
    gdim[1 as libc::c_int as usize] = (8 as libc::c_int * (*h).mb.i_mb_height) as size_t;
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    status = ((*ocl).clEnqueueNDRangeKernel).expect("non-null function pointer")(
        (*h).opencl.queue,
        (*h).opencl.downscale_hpel_kernel,
        2 as libc::c_int as cl_uint,
        std::ptr::null::<size_t>(),
        gdim.as_mut_ptr(),
        std::ptr::null::<size_t>(),
        0 as libc::c_int as cl_uint,
        std::ptr::null::<cl_event>(),
        std::ptr::null_mut::<cl_event>(),
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clEnqueueNDRangeKernel error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < 4 as libc::c_int - 1 as libc::c_int {
        let mut kern: cl_kernel = if i_1 & 1 as libc::c_int != 0 {
            (*h).opencl.downscale_kernel1
        } else {
            (*h).opencl.downscale_kernel2
        };
        arg_0 = 0 as libc::c_int as cl_uint;
        if (*h).opencl.b_fatal_error != 0 {
            return -(1 as libc::c_int);
        }
        let fresh6 = arg_0;
        arg_0 = arg_0.wrapping_add(1);
        status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
            kern,
            fresh6,
            ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
            &mut *((*fenc).opencl.scaled_image2Ds)
                .as_mut_ptr()
                .offset(i_1 as isize) as *mut cl_mem as *const libc::c_void,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            (*h).opencl.b_fatal_error = 1 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        if (*h).opencl.b_fatal_error != 0 {
            return -(1 as libc::c_int);
        }
        let fresh7 = arg_0;
        arg_0 = arg_0.wrapping_add(1);
        status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
            kern,
            fresh7,
            ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
            &mut *((*fenc).opencl.scaled_image2Ds)
                .as_mut_ptr()
                .offset((i_1 + 1 as libc::c_int) as isize) as *mut cl_mem
                as *const libc::c_void,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            (*h).opencl.b_fatal_error = 1 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        gdim[0 as libc::c_int as usize] >>= 1 as libc::c_int;
        gdim[1 as libc::c_int as usize] >>= 1 as libc::c_int;
        if gdim[0 as libc::c_int as usize] < 16 as libc::c_int as size_t
            || gdim[1 as libc::c_int as usize] < 16 as libc::c_int as size_t
        {
            break;
        }
        if (*h).opencl.b_fatal_error != 0 {
            return -(1 as libc::c_int);
        }
        status = ((*ocl).clEnqueueNDRangeKernel).expect("non-null function pointer")(
            (*h).opencl.queue,
            kern,
            2 as libc::c_int as cl_uint,
            std::ptr::null::<size_t>(),
            gdim.as_mut_ptr(),
            std::ptr::null::<size_t>(),
            0 as libc::c_int as cl_uint,
            std::ptr::null::<cl_event>(),
            std::ptr::null_mut::<cl_event>(),
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            (*h).opencl.b_fatal_error = 1 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clEnqueueNDRangeKernel error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        i_1 += 1;
        i_1;
    }
    let mut ldim: [size_t; 2] = [0; 2];
    gdim[0 as libc::c_int as usize] = ((((*h).mb.i_mb_width + 31 as libc::c_int)
        >> 5 as libc::c_int)
        << 5 as libc::c_int) as size_t;
    gdim[1 as libc::c_int as usize] = (8 as libc::c_int * (*h).mb.i_mb_height) as size_t;
    ldim[0 as libc::c_int as usize] = 32 as libc::c_int as size_t;
    ldim[1 as libc::c_int as usize] = 8 as libc::c_int as size_t;
    arg_0 = 0 as libc::c_int as cl_uint;
    let mut slow: libc::c_int =
        ((*h).param.analyse.i_subpel_refine > 7 as libc::c_int) as libc::c_int;
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh8 = arg_0;
    arg_0 = arg_0.wrapping_add(1);
    status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
        (*h).opencl.intra_kernel,
        fresh8,
        ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
        &mut *((*fenc).opencl.scaled_image2Ds)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) as *mut cl_mem as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh9 = arg_0;
    arg_0 = arg_0.wrapping_add(1);
    status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
        (*h).opencl.intra_kernel,
        fresh9,
        ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
        &mut (*fenc).opencl.intra_cost as *mut cl_mem as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh10 = arg_0;
    arg_0 = arg_0.wrapping_add(1);
    status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
        (*h).opencl.intra_kernel,
        fresh10,
        ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
        &mut *((*h).opencl.frame_stats)
            .as_mut_ptr()
            .offset((*h).opencl.last_buf as isize) as *mut cl_mem as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh11 = arg_0;
    arg_0 = arg_0.wrapping_add(1);
    status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
        (*h).opencl.intra_kernel,
        fresh11,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        &mut lambda as *mut libc::c_int as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh12 = arg_0;
    arg_0 = arg_0.wrapping_add(1);
    status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
        (*h).opencl.intra_kernel,
        fresh12,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        &mut (*h).mb.i_mb_width as *mut libc::c_int as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh13 = arg_0;
    arg_0 = arg_0.wrapping_add(1);
    status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
        (*h).opencl.intra_kernel,
        fresh13,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        &mut slow as *mut libc::c_int as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    status = ((*ocl).clEnqueueNDRangeKernel).expect("non-null function pointer")(
        (*h).opencl.queue,
        (*h).opencl.intra_kernel,
        2 as libc::c_int as cl_uint,
        std::ptr::null::<size_t>(),
        gdim.as_mut_ptr(),
        ldim.as_mut_ptr(),
        0 as libc::c_int as cl_uint,
        std::ptr::null::<cl_event>(),
        std::ptr::null_mut::<cl_event>(),
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clEnqueueNDRangeKernel error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    gdim[0 as libc::c_int as usize] = 256 as libc::c_int as size_t;
    gdim[1 as libc::c_int as usize] = (*h).mb.i_mb_height as size_t;
    ldim[0 as libc::c_int as usize] = 256 as libc::c_int as size_t;
    ldim[1 as libc::c_int as usize] = 1 as libc::c_int as size_t;
    arg_0 = 0 as libc::c_int as cl_uint;
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh14 = arg_0;
    arg_0 = arg_0.wrapping_add(1);
    status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
        (*h).opencl.rowsum_intra_kernel,
        fresh14,
        ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
        &mut (*fenc).opencl.intra_cost as *mut cl_mem as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh15 = arg_0;
    arg_0 = arg_0.wrapping_add(1);
    status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
        (*h).opencl.rowsum_intra_kernel,
        fresh15,
        ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
        &mut (*fenc).opencl.inv_qscale_factor as *mut cl_mem as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh16 = arg_0;
    arg_0 = arg_0.wrapping_add(1);
    status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
        (*h).opencl.rowsum_intra_kernel,
        fresh16,
        ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
        &mut *((*h).opencl.row_satds)
            .as_mut_ptr()
            .offset((*h).opencl.last_buf as isize) as *mut cl_mem as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh17 = arg_0;
    arg_0 = arg_0.wrapping_add(1);
    status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
        (*h).opencl.rowsum_intra_kernel,
        fresh17,
        ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
        &mut *((*h).opencl.frame_stats)
            .as_mut_ptr()
            .offset((*h).opencl.last_buf as isize) as *mut cl_mem as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh18 = arg_0;
    arg_0 = arg_0.wrapping_add(1);
    status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
        (*h).opencl.rowsum_intra_kernel,
        fresh18,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        &mut (*h).mb.i_mb_width as *mut libc::c_int as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    status = ((*ocl).clEnqueueNDRangeKernel).expect("non-null function pointer")(
        (*h).opencl.queue,
        (*h).opencl.rowsum_intra_kernel,
        2 as libc::c_int as cl_uint,
        std::ptr::null::<size_t>(),
        gdim.as_mut_ptr(),
        ldim.as_mut_ptr(),
        0 as libc::c_int as cl_uint,
        std::ptr::null::<cl_event>(),
        std::ptr::null_mut::<cl_event>(),
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clEnqueueNDRangeKernel error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.num_copies >= 1024 as libc::c_int - 4 as libc::c_int {
        x264_8_opencl_flush(h);
    }
    let mut size_0: libc::c_int = ((*h).mb.i_mb_count as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong)
        as libc::c_int;
    locked = opencl_alloc_locked(h, size_0) as *mut libc::c_char;
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    status = ((*ocl).clEnqueueReadBuffer).expect("non-null function pointer")(
        (*h).opencl.queue,
        (*fenc).opencl.intra_cost,
        0 as libc::c_int as cl_bool,
        0 as libc::c_int as size_t,
        size_0 as size_t,
        locked as *mut libc::c_void,
        0 as libc::c_int as cl_uint,
        std::ptr::null::<cl_event>(),
        std::ptr::null_mut::<cl_event>(),
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clEnqueueReadBuffer error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    (*h).opencl.copies[(*h).opencl.num_copies as usize].dest = (*fenc).lowres_costs
        [0 as libc::c_int as usize][0 as libc::c_int as usize]
        as *mut libc::c_void;
    (*h).opencl.copies[(*h).opencl.num_copies as usize].src = locked as *mut libc::c_void;
    (*h).opencl.copies[(*h).opencl.num_copies as usize].bytes = size_0;
    (*h).opencl.num_copies += 1;
    (*h).opencl.num_copies;
    size_0 = ((*h).mb.i_mb_height as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
        as libc::c_int;
    locked = opencl_alloc_locked(h, size_0) as *mut libc::c_char;
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    status = ((*ocl).clEnqueueReadBuffer).expect("non-null function pointer")(
        (*h).opencl.queue,
        (*h).opencl.row_satds[(*h).opencl.last_buf as usize],
        0 as libc::c_int as cl_bool,
        0 as libc::c_int as size_t,
        size_0 as size_t,
        locked as *mut libc::c_void,
        0 as libc::c_int as cl_uint,
        std::ptr::null::<cl_event>(),
        std::ptr::null_mut::<cl_event>(),
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clEnqueueReadBuffer error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    (*h).opencl.copies[(*h).opencl.num_copies as usize].dest = (*fenc).i_row_satds
        [0 as libc::c_int as usize][0 as libc::c_int as usize]
        as *mut libc::c_void;
    (*h).opencl.copies[(*h).opencl.num_copies as usize].src = locked as *mut libc::c_void;
    (*h).opencl.copies[(*h).opencl.num_copies as usize].bytes = size_0;
    (*h).opencl.num_copies += 1;
    (*h).opencl.num_copies;
    size_0 = (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
        .wrapping_mul(4 as libc::c_int as libc::c_ulong) as libc::c_int;
    locked = opencl_alloc_locked(h, size_0) as *mut libc::c_char;
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    status = ((*ocl).clEnqueueReadBuffer).expect("non-null function pointer")(
        (*h).opencl.queue,
        (*h).opencl.frame_stats[(*h).opencl.last_buf as usize],
        0 as libc::c_int as cl_bool,
        0 as libc::c_int as size_t,
        size_0 as size_t,
        locked as *mut libc::c_void,
        0 as libc::c_int as cl_uint,
        std::ptr::null::<cl_event>(),
        std::ptr::null_mut::<cl_event>(),
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clEnqueueReadBuffer error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    (*h).opencl.copies[(*h).opencl.num_copies as usize].dest =
        &mut *(*((*fenc).i_cost_est)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize))
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) as *mut libc::c_int as *mut libc::c_void;
    (*h).opencl.copies[(*h).opencl.num_copies as usize].src = locked as *mut libc::c_void;
    (*h).opencl.copies[(*h).opencl.num_copies as usize].bytes =
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int;
    (*h).opencl.num_copies += 1;
    (*h).opencl.num_copies;
    (*h).opencl.copies[(*h).opencl.num_copies as usize].dest =
        &mut *(*((*fenc).i_cost_est_aq)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize))
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) as *mut libc::c_int as *mut libc::c_void;
    (*h).opencl.copies[(*h).opencl.num_copies as usize].src = locked
        .offset(::core::mem::size_of::<libc::c_int>() as libc::c_ulong as isize)
        as *mut libc::c_void;
    (*h).opencl.copies[(*h).opencl.num_copies as usize].bytes =
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int;
    (*h).opencl.num_copies += 1;
    (*h).opencl.num_copies;
    (*h).opencl.last_buf = ((*h).opencl.last_buf == 0) as libc::c_int;
    0 as libc::c_int
}
unsafe extern "C" fn optimal_launch_dims(
    mut h: *mut x264_t,
    mut gdims: *mut size_t,
    mut ldims: *mut size_t,
    kernel: cl_kernel,
    device: cl_device_id,
) {
    let mut ocl: *mut x264_opencl_function_t = (*h).opencl.ocl;
    let mut max_work_group: size_t = 256 as libc::c_int as size_t;
    let mut preferred_multiple: size_t = 64 as libc::c_int as size_t;
    let mut num_cus: cl_uint = 6 as libc::c_int as cl_uint;
    ((*ocl).clGetKernelWorkGroupInfo).expect("non-null function pointer")(
        kernel,
        device,
        0x11b0 as libc::c_int as cl_kernel_work_group_info,
        ::core::mem::size_of::<size_t>() as libc::c_ulong,
        &mut max_work_group as *mut size_t as *mut libc::c_void,
        std::ptr::null_mut::<size_t>(),
    );
    ((*ocl).clGetKernelWorkGroupInfo).expect("non-null function pointer")(
        kernel,
        device,
        0x11b3 as libc::c_int as cl_kernel_work_group_info,
        ::core::mem::size_of::<size_t>() as libc::c_ulong,
        &mut preferred_multiple as *mut size_t as *mut libc::c_void,
        std::ptr::null_mut::<size_t>(),
    );
    ((*ocl).clGetDeviceInfo).expect("non-null function pointer")(
        device,
        0x1002 as libc::c_int as cl_device_info,
        ::core::mem::size_of::<cl_uint>() as libc::c_ulong,
        &mut num_cus as *mut cl_uint as *mut libc::c_void,
        std::ptr::null_mut::<size_t>(),
    );
    *ldims.offset(0 as libc::c_int as isize) = preferred_multiple;
    *ldims.offset(1 as libc::c_int as isize) = 8 as libc::c_int as size_t;
    while *gdims.offset(1 as libc::c_int as isize)
        & (*ldims.offset(1 as libc::c_int as isize)).wrapping_sub(1 as libc::c_int as size_t)
        != 0
    {
        *ldims.offset(0 as libc::c_int as isize) <<= 1 as libc::c_int;
        *ldims.offset(1 as libc::c_int as isize) >>= 1 as libc::c_int;
    }
    while *ldims.offset(0 as libc::c_int as isize) * *ldims.offset(1 as libc::c_int as isize)
        > max_work_group
    {
        if *ldims.offset(0 as libc::c_int as isize) <= preferred_multiple
            && *ldims.offset(1 as libc::c_int as isize) > 1 as libc::c_int as size_t
        {
            *ldims.offset(1 as libc::c_int as isize) >>= 1 as libc::c_int;
        } else {
            *ldims.offset(0 as libc::c_int as isize) >>= 1 as libc::c_int;
        }
    }
    if *ldims.offset(0 as libc::c_int as isize) > *gdims.offset(0 as libc::c_int as isize) {
        while (*gdims.offset(0 as libc::c_int as isize)).wrapping_add(preferred_multiple)
            < *ldims.offset(0 as libc::c_int as isize)
        {
            let fresh19 = &mut (*ldims.offset(0 as libc::c_int as isize));
            *fresh19 = (*fresh19).wrapping_sub(preferred_multiple);
        }
        *gdims.offset(0 as libc::c_int as isize) = *ldims.offset(0 as libc::c_int as isize);
    } else {
        *gdims.offset(0 as libc::c_int as isize) = (*gdims.offset(0 as libc::c_int as isize))
            .wrapping_add(*ldims.offset(0 as libc::c_int as isize))
            .wrapping_sub(1 as libc::c_int as size_t)
            / *ldims.offset(0 as libc::c_int as isize);
        let fresh20 = &mut (*gdims.offset(0 as libc::c_int as isize));
        *fresh20 *= *ldims.offset(0 as libc::c_int as isize);
    }
    while *gdims.offset(0 as libc::c_int as isize) / *ldims.offset(0 as libc::c_int as isize)
        * (*gdims.offset(1 as libc::c_int as isize) / *ldims.offset(1 as libc::c_int as isize))
        * 2 as libc::c_int as size_t
        <= num_cus as size_t
    {
        if *ldims.offset(0 as libc::c_int as isize) > preferred_multiple {
            *ldims.offset(0 as libc::c_int as isize) >>= 1 as libc::c_int;
        } else {
            if *ldims.offset(1 as libc::c_int as isize) <= 1 as libc::c_int as size_t {
                break;
            }
            *ldims.offset(1 as libc::c_int as isize) >>= 1 as libc::c_int;
        }
    }
    if num_cus == 6 as libc::c_int as cl_uint
        && *ldims.offset(0 as libc::c_int as isize) == 64 as libc::c_int as size_t
        && *ldims.offset(1 as libc::c_int as isize) == 4 as libc::c_int as size_t
    {
        *ldims.offset(0 as libc::c_int as isize) = 32 as libc::c_int as size_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_opencl_motionsearch(
    mut h: *mut x264_t,
    mut frames: *mut *mut x264_frame_t,
    mut b: libc::c_int,
    mut ref_0: libc::c_int,
    mut b_islist1: libc::c_int,
    mut lambda: libc::c_int,
    mut w: *const x264_weight_t,
) -> libc::c_int {
    let mut ocl: *mut x264_opencl_function_t = (*h).opencl.ocl;
    let mut fenc: *mut x264_frame_t = *frames.offset(b as isize);
    let mut fref: *mut x264_frame_t = *frames.offset(ref_0 as isize);
    let mut ref_scaled_images: [cl_mem; 4] = [std::ptr::null_mut::<_cl_mem>(); 4];
    let mut ref_luma_hpel: cl_mem = std::ptr::null_mut::<_cl_mem>();
    let mut status: cl_int = 0;
    if !w.is_null() && !((*w).weightfn).is_null() {
        let mut gdims: [size_t; 2] = [0; 2];
        gdims[0 as libc::c_int as usize] = (8 as libc::c_int * (*h).mb.i_mb_width) as size_t;
        gdims[1 as libc::c_int as usize] = (8 as libc::c_int * (*h).mb.i_mb_height) as size_t;
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            let mut arg: cl_uint = 0 as libc::c_int as cl_uint;
            if (*h).opencl.b_fatal_error != 0 {
                return -(1 as libc::c_int);
            }
            let fresh21 = arg;
            arg = arg.wrapping_add(1);
            status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
                (*h).opencl.weightp_scaled_images_kernel,
                fresh21,
                ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
                &mut *((*fref).opencl.scaled_image2Ds)
                    .as_mut_ptr()
                    .offset(i as isize) as *mut cl_mem as *const libc::c_void,
            );
            if status != 0 as libc::c_int {
                (*h).param.b_opencl = 0 as libc::c_int;
                (*h).opencl.b_fatal_error = 1 as libc::c_int;
                x264_8_log(
                    h,
                    0 as libc::c_int,
                    b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                    status,
                );
                return -(1 as libc::c_int);
            }
            if (*h).opencl.b_fatal_error != 0 {
                return -(1 as libc::c_int);
            }
            let fresh22 = arg;
            arg = arg.wrapping_add(1);
            status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
                (*h).opencl.weightp_scaled_images_kernel,
                fresh22,
                ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
                &mut *((*h).opencl.weighted_scaled_images)
                    .as_mut_ptr()
                    .offset(i as isize) as *mut cl_mem as *const libc::c_void,
            );
            if status != 0 as libc::c_int {
                (*h).param.b_opencl = 0 as libc::c_int;
                (*h).opencl.b_fatal_error = 1 as libc::c_int;
                x264_8_log(
                    h,
                    0 as libc::c_int,
                    b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                    status,
                );
                return -(1 as libc::c_int);
            }
            if (*h).opencl.b_fatal_error != 0 {
                return -(1 as libc::c_int);
            }
            let fresh23 = arg;
            arg = arg.wrapping_add(1);
            status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
                (*h).opencl.weightp_scaled_images_kernel,
                fresh23,
                ::core::mem::size_of::<int32_t>() as libc::c_ulong,
                &(*w).i_offset as *const int32_t as *const libc::c_void,
            );
            if status != 0 as libc::c_int {
                (*h).param.b_opencl = 0 as libc::c_int;
                (*h).opencl.b_fatal_error = 1 as libc::c_int;
                x264_8_log(
                    h,
                    0 as libc::c_int,
                    b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                    status,
                );
                return -(1 as libc::c_int);
            }
            if (*h).opencl.b_fatal_error != 0 {
                return -(1 as libc::c_int);
            }
            let fresh24 = arg;
            arg = arg.wrapping_add(1);
            status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
                (*h).opencl.weightp_scaled_images_kernel,
                fresh24,
                ::core::mem::size_of::<int32_t>() as libc::c_ulong,
                &(*w).i_scale as *const int32_t as *const libc::c_void,
            );
            if status != 0 as libc::c_int {
                (*h).param.b_opencl = 0 as libc::c_int;
                (*h).opencl.b_fatal_error = 1 as libc::c_int;
                x264_8_log(
                    h,
                    0 as libc::c_int,
                    b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                    status,
                );
                return -(1 as libc::c_int);
            }
            if (*h).opencl.b_fatal_error != 0 {
                return -(1 as libc::c_int);
            }
            let fresh25 = arg;
            arg = arg.wrapping_add(1);
            status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
                (*h).opencl.weightp_scaled_images_kernel,
                fresh25,
                ::core::mem::size_of::<int32_t>() as libc::c_ulong,
                &(*w).i_denom as *const int32_t as *const libc::c_void,
            );
            if status != 0 as libc::c_int {
                (*h).param.b_opencl = 0 as libc::c_int;
                (*h).opencl.b_fatal_error = 1 as libc::c_int;
                x264_8_log(
                    h,
                    0 as libc::c_int,
                    b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                    status,
                );
                return -(1 as libc::c_int);
            }
            if (*h).opencl.b_fatal_error != 0 {
                return -(1 as libc::c_int);
            }
            status = ((*ocl).clEnqueueNDRangeKernel).expect("non-null function pointer")(
                (*h).opencl.queue,
                (*h).opencl.weightp_scaled_images_kernel,
                2 as libc::c_int as cl_uint,
                std::ptr::null::<size_t>(),
                gdims.as_mut_ptr(),
                std::ptr::null::<size_t>(),
                0 as libc::c_int as cl_uint,
                std::ptr::null::<cl_event>(),
                std::ptr::null_mut::<cl_event>(),
            );
            if status != 0 as libc::c_int {
                (*h).param.b_opencl = 0 as libc::c_int;
                (*h).opencl.b_fatal_error = 1 as libc::c_int;
                x264_8_log(
                    h,
                    0 as libc::c_int,
                    b"clEnqueueNDRangeKernel error '%d'\n\0" as *const u8 as *const libc::c_char,
                    status,
                );
                return -(1 as libc::c_int);
            }
            gdims[0 as libc::c_int as usize] >>= 1 as libc::c_int;
            gdims[1 as libc::c_int as usize] >>= 1 as libc::c_int;
            if gdims[0 as libc::c_int as usize] < 16 as libc::c_int as size_t
                || gdims[1 as libc::c_int as usize] < 16 as libc::c_int as size_t
            {
                break;
            }
            i += 1;
            i;
        }
        let mut arg_0: cl_uint = 0 as libc::c_int as cl_uint;
        gdims[0 as libc::c_int as usize] = (8 as libc::c_int * (*h).mb.i_mb_width) as size_t;
        gdims[1 as libc::c_int as usize] = (8 as libc::c_int * (*h).mb.i_mb_height) as size_t;
        if (*h).opencl.b_fatal_error != 0 {
            return -(1 as libc::c_int);
        }
        let fresh26 = arg_0;
        arg_0 = arg_0.wrapping_add(1);
        status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
            (*h).opencl.weightp_hpel_kernel,
            fresh26,
            ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
            &mut (*fref).opencl.luma_hpel as *mut cl_mem as *const libc::c_void,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            (*h).opencl.b_fatal_error = 1 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        if (*h).opencl.b_fatal_error != 0 {
            return -(1 as libc::c_int);
        }
        let fresh27 = arg_0;
        arg_0 = arg_0.wrapping_add(1);
        status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
            (*h).opencl.weightp_hpel_kernel,
            fresh27,
            ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
            &mut (*h).opencl.weighted_luma_hpel as *mut cl_mem as *const libc::c_void,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            (*h).opencl.b_fatal_error = 1 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        if (*h).opencl.b_fatal_error != 0 {
            return -(1 as libc::c_int);
        }
        let fresh28 = arg_0;
        arg_0 = arg_0.wrapping_add(1);
        status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
            (*h).opencl.weightp_hpel_kernel,
            fresh28,
            ::core::mem::size_of::<int32_t>() as libc::c_ulong,
            &(*w).i_offset as *const int32_t as *const libc::c_void,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            (*h).opencl.b_fatal_error = 1 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        if (*h).opencl.b_fatal_error != 0 {
            return -(1 as libc::c_int);
        }
        let fresh29 = arg_0;
        arg_0 = arg_0.wrapping_add(1);
        status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
            (*h).opencl.weightp_hpel_kernel,
            fresh29,
            ::core::mem::size_of::<int32_t>() as libc::c_ulong,
            &(*w).i_scale as *const int32_t as *const libc::c_void,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            (*h).opencl.b_fatal_error = 1 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        if (*h).opencl.b_fatal_error != 0 {
            return -(1 as libc::c_int);
        }
        let fresh30 = arg_0;
        arg_0 = arg_0.wrapping_add(1);
        status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
            (*h).opencl.weightp_hpel_kernel,
            fresh30,
            ::core::mem::size_of::<int32_t>() as libc::c_ulong,
            &(*w).i_denom as *const int32_t as *const libc::c_void,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            (*h).opencl.b_fatal_error = 1 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        if (*h).opencl.b_fatal_error != 0 {
            return -(1 as libc::c_int);
        }
        status = ((*ocl).clEnqueueNDRangeKernel).expect("non-null function pointer")(
            (*h).opencl.queue,
            (*h).opencl.weightp_hpel_kernel,
            2 as libc::c_int as cl_uint,
            std::ptr::null::<size_t>(),
            gdims.as_mut_ptr(),
            std::ptr::null::<size_t>(),
            0 as libc::c_int as cl_uint,
            std::ptr::null::<cl_event>(),
            std::ptr::null_mut::<cl_event>(),
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            (*h).opencl.b_fatal_error = 1 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clEnqueueNDRangeKernel error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < 4 as libc::c_int {
            ref_scaled_images[i_0 as usize] = (*h).opencl.weighted_scaled_images[i_0 as usize];
            i_0 += 1;
            i_0;
        }
        ref_luma_hpel = (*h).opencl.weighted_luma_hpel;
    } else {
        let mut i_1: libc::c_int = 0 as libc::c_int;
        while i_1 < 4 as libc::c_int {
            ref_scaled_images[i_1 as usize] = (*fref).opencl.scaled_image2Ds[i_1 as usize];
            i_1 += 1;
            i_1;
        }
        ref_luma_hpel = (*fref).opencl.luma_hpel;
    }
    let num_iterations: [libc::c_int; 4] = [
        1 as libc::c_int,
        1 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
    ];
    let mut b_first_iteration: libc::c_int = 1 as libc::c_int;
    let mut b_reverse_references: libc::c_int = 1 as libc::c_int;
    let mut A: libc::c_int = 1 as libc::c_int;
    let mut mb_per_group: libc::c_int = 0 as libc::c_int;
    let mut cost_local_size: libc::c_int = 0 as libc::c_int;
    let mut mvc_local_size: libc::c_int = 0 as libc::c_int;
    let mut mb_width: libc::c_int = 0;
    let mut gdims_0: [size_t; 2] = [0; 2];
    let mut ldims: [size_t; 2] = [0; 2];
    let mut scale: libc::c_int = 4 as libc::c_int - 1 as libc::c_int;
    while scale >= 0 as libc::c_int {
        mb_width = (*h).mb.i_mb_width >> scale;
        gdims_0[0 as libc::c_int as usize] = mb_width as size_t;
        gdims_0[1 as libc::c_int as usize] = ((*h).mb.i_mb_height >> scale) as size_t;
        if !(gdims_0[0 as libc::c_int as usize] < 2 as libc::c_int as size_t
            || gdims_0[1 as libc::c_int as usize] < 2 as libc::c_int as size_t)
        {
            gdims_0[0 as libc::c_int as usize] <<= 2 as libc::c_int;
            optimal_launch_dims(
                h,
                gdims_0.as_mut_ptr(),
                ldims.as_mut_ptr(),
                (*h).opencl.hme_kernel,
                (*h).opencl.device,
            );
            mb_per_group = ((ldims[0 as libc::c_int as usize] >> 2 as libc::c_int)
                * ldims[1 as libc::c_int as usize]) as libc::c_int;
            cost_local_size = ((4 as libc::c_int * mb_per_group) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong)
                as libc::c_int;
            mvc_local_size = ((4 as libc::c_int * mb_per_group) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                as libc::c_int;
            let mut scaled_me_range: libc::c_int = (*h).param.analyse.i_me_range >> scale;
            let mut b_shift_index: libc::c_int = 1 as libc::c_int;
            let mut arg_1: cl_uint = 0 as libc::c_int as cl_uint;
            if (*h).opencl.b_fatal_error != 0 {
                return -(1 as libc::c_int);
            }
            let fresh31 = arg_1;
            arg_1 = arg_1.wrapping_add(1);
            status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
                (*h).opencl.hme_kernel,
                fresh31,
                ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
                &mut *((*fenc).opencl.scaled_image2Ds)
                    .as_mut_ptr()
                    .offset(scale as isize) as *mut cl_mem as *const libc::c_void,
            );
            if status != 0 as libc::c_int {
                (*h).param.b_opencl = 0 as libc::c_int;
                (*h).opencl.b_fatal_error = 1 as libc::c_int;
                x264_8_log(
                    h,
                    0 as libc::c_int,
                    b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                    status,
                );
                return -(1 as libc::c_int);
            }
            if (*h).opencl.b_fatal_error != 0 {
                return -(1 as libc::c_int);
            }
            let fresh32 = arg_1;
            arg_1 = arg_1.wrapping_add(1);
            status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
                (*h).opencl.hme_kernel,
                fresh32,
                ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
                &mut *ref_scaled_images.as_mut_ptr().offset(scale as isize) as *mut cl_mem
                    as *const libc::c_void,
            );
            if status != 0 as libc::c_int {
                (*h).param.b_opencl = 0 as libc::c_int;
                (*h).opencl.b_fatal_error = 1 as libc::c_int;
                x264_8_log(
                    h,
                    0 as libc::c_int,
                    b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                    status,
                );
                return -(1 as libc::c_int);
            }
            if (*h).opencl.b_fatal_error != 0 {
                return -(1 as libc::c_int);
            }
            let fresh33 = arg_1;
            arg_1 = arg_1.wrapping_add(1);
            status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
                (*h).opencl.hme_kernel,
                fresh33,
                ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
                &mut *((*h).opencl.mv_buffers).as_mut_ptr().offset(A as isize) as *mut cl_mem
                    as *const libc::c_void,
            );
            if status != 0 as libc::c_int {
                (*h).param.b_opencl = 0 as libc::c_int;
                (*h).opencl.b_fatal_error = 1 as libc::c_int;
                x264_8_log(
                    h,
                    0 as libc::c_int,
                    b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                    status,
                );
                return -(1 as libc::c_int);
            }
            if (*h).opencl.b_fatal_error != 0 {
                return -(1 as libc::c_int);
            }
            let fresh34 = arg_1;
            arg_1 = arg_1.wrapping_add(1);
            status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
                (*h).opencl.hme_kernel,
                fresh34,
                ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
                &mut *((*h).opencl.mv_buffers)
                    .as_mut_ptr()
                    .offset((A == 0) as libc::c_int as isize) as *mut cl_mem
                    as *const libc::c_void,
            );
            if status != 0 as libc::c_int {
                (*h).param.b_opencl = 0 as libc::c_int;
                (*h).opencl.b_fatal_error = 1 as libc::c_int;
                x264_8_log(
                    h,
                    0 as libc::c_int,
                    b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                    status,
                );
                return -(1 as libc::c_int);
            }
            if (*h).opencl.b_fatal_error != 0 {
                return -(1 as libc::c_int);
            }
            let fresh35 = arg_1;
            arg_1 = arg_1.wrapping_add(1);
            status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
                (*h).opencl.hme_kernel,
                fresh35,
                ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
                &mut (*h).opencl.lowres_mv_costs as *mut cl_mem as *const libc::c_void,
            );
            if status != 0 as libc::c_int {
                (*h).param.b_opencl = 0 as libc::c_int;
                (*h).opencl.b_fatal_error = 1 as libc::c_int;
                x264_8_log(
                    h,
                    0 as libc::c_int,
                    b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                    status,
                );
                return -(1 as libc::c_int);
            }
            if (*h).opencl.b_fatal_error != 0 {
                return -(1 as libc::c_int);
            }
            let fresh36 = arg_1;
            arg_1 = arg_1.wrapping_add(1);
            status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
                (*h).opencl.hme_kernel,
                fresh36,
                ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
                &mut (*h).opencl.mvp_buffer as *mut cl_mem as *mut libc::c_void,
            );
            if status != 0 as libc::c_int {
                (*h).param.b_opencl = 0 as libc::c_int;
                (*h).opencl.b_fatal_error = 1 as libc::c_int;
                x264_8_log(
                    h,
                    0 as libc::c_int,
                    b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                    status,
                );
                return -(1 as libc::c_int);
            }
            if (*h).opencl.b_fatal_error != 0 {
                return -(1 as libc::c_int);
            }
            let fresh37 = arg_1;
            arg_1 = arg_1.wrapping_add(1);
            status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
                (*h).opencl.hme_kernel,
                fresh37,
                cost_local_size as size_t,
                std::ptr::null::<libc::c_void>(),
            );
            if status != 0 as libc::c_int {
                (*h).param.b_opencl = 0 as libc::c_int;
                (*h).opencl.b_fatal_error = 1 as libc::c_int;
                x264_8_log(
                    h,
                    0 as libc::c_int,
                    b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                    status,
                );
                return -(1 as libc::c_int);
            }
            if (*h).opencl.b_fatal_error != 0 {
                return -(1 as libc::c_int);
            }
            let fresh38 = arg_1;
            arg_1 = arg_1.wrapping_add(1);
            status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
                (*h).opencl.hme_kernel,
                fresh38,
                mvc_local_size as size_t,
                std::ptr::null::<libc::c_void>(),
            );
            if status != 0 as libc::c_int {
                (*h).param.b_opencl = 0 as libc::c_int;
                (*h).opencl.b_fatal_error = 1 as libc::c_int;
                x264_8_log(
                    h,
                    0 as libc::c_int,
                    b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                    status,
                );
                return -(1 as libc::c_int);
            }
            if (*h).opencl.b_fatal_error != 0 {
                return -(1 as libc::c_int);
            }
            let fresh39 = arg_1;
            arg_1 = arg_1.wrapping_add(1);
            status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
                (*h).opencl.hme_kernel,
                fresh39,
                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                &mut mb_width as *mut libc::c_int as *const libc::c_void,
            );
            if status != 0 as libc::c_int {
                (*h).param.b_opencl = 0 as libc::c_int;
                (*h).opencl.b_fatal_error = 1 as libc::c_int;
                x264_8_log(
                    h,
                    0 as libc::c_int,
                    b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                    status,
                );
                return -(1 as libc::c_int);
            }
            if (*h).opencl.b_fatal_error != 0 {
                return -(1 as libc::c_int);
            }
            let fresh40 = arg_1;
            arg_1 = arg_1.wrapping_add(1);
            status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
                (*h).opencl.hme_kernel,
                fresh40,
                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                &mut lambda as *mut libc::c_int as *const libc::c_void,
            );
            if status != 0 as libc::c_int {
                (*h).param.b_opencl = 0 as libc::c_int;
                (*h).opencl.b_fatal_error = 1 as libc::c_int;
                x264_8_log(
                    h,
                    0 as libc::c_int,
                    b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                    status,
                );
                return -(1 as libc::c_int);
            }
            if (*h).opencl.b_fatal_error != 0 {
                return -(1 as libc::c_int);
            }
            let fresh41 = arg_1;
            arg_1 = arg_1.wrapping_add(1);
            status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
                (*h).opencl.hme_kernel,
                fresh41,
                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                &mut scaled_me_range as *mut libc::c_int as *const libc::c_void,
            );
            if status != 0 as libc::c_int {
                (*h).param.b_opencl = 0 as libc::c_int;
                (*h).opencl.b_fatal_error = 1 as libc::c_int;
                x264_8_log(
                    h,
                    0 as libc::c_int,
                    b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                    status,
                );
                return -(1 as libc::c_int);
            }
            if (*h).opencl.b_fatal_error != 0 {
                return -(1 as libc::c_int);
            }
            let fresh42 = arg_1;
            arg_1 = arg_1.wrapping_add(1);
            status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
                (*h).opencl.hme_kernel,
                fresh42,
                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                &mut scale as *mut libc::c_int as *const libc::c_void,
            );
            if status != 0 as libc::c_int {
                (*h).param.b_opencl = 0 as libc::c_int;
                (*h).opencl.b_fatal_error = 1 as libc::c_int;
                x264_8_log(
                    h,
                    0 as libc::c_int,
                    b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                    status,
                );
                return -(1 as libc::c_int);
            }
            if (*h).opencl.b_fatal_error != 0 {
                return -(1 as libc::c_int);
            }
            let fresh43 = arg_1;
            arg_1 = arg_1.wrapping_add(1);
            status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
                (*h).opencl.hme_kernel,
                fresh43,
                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                &mut b_shift_index as *mut libc::c_int as *const libc::c_void,
            );
            if status != 0 as libc::c_int {
                (*h).param.b_opencl = 0 as libc::c_int;
                (*h).opencl.b_fatal_error = 1 as libc::c_int;
                x264_8_log(
                    h,
                    0 as libc::c_int,
                    b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                    status,
                );
                return -(1 as libc::c_int);
            }
            if (*h).opencl.b_fatal_error != 0 {
                return -(1 as libc::c_int);
            }
            let fresh44 = arg_1;
            arg_1 = arg_1.wrapping_add(1);
            status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
                (*h).opencl.hme_kernel,
                fresh44,
                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                &mut b_first_iteration as *mut libc::c_int as *const libc::c_void,
            );
            if status != 0 as libc::c_int {
                (*h).param.b_opencl = 0 as libc::c_int;
                (*h).opencl.b_fatal_error = 1 as libc::c_int;
                x264_8_log(
                    h,
                    0 as libc::c_int,
                    b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                    status,
                );
                return -(1 as libc::c_int);
            }
            if (*h).opencl.b_fatal_error != 0 {
                return -(1 as libc::c_int);
            }
            let fresh45 = arg_1;
            arg_1 = arg_1.wrapping_add(1);
            status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
                (*h).opencl.hme_kernel,
                fresh45,
                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                &mut b_reverse_references as *mut libc::c_int as *const libc::c_void,
            );
            if status != 0 as libc::c_int {
                (*h).param.b_opencl = 0 as libc::c_int;
                (*h).opencl.b_fatal_error = 1 as libc::c_int;
                x264_8_log(
                    h,
                    0 as libc::c_int,
                    b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                    status,
                );
                return -(1 as libc::c_int);
            }
            let mut iter: libc::c_int = 0 as libc::c_int;
            while iter < num_iterations[scale as usize] {
                if (*h).opencl.b_fatal_error != 0 {
                    return -(1 as libc::c_int);
                }
                status = ((*ocl).clEnqueueNDRangeKernel).expect("non-null function pointer")(
                    (*h).opencl.queue,
                    (*h).opencl.hme_kernel,
                    2 as libc::c_int as cl_uint,
                    std::ptr::null::<size_t>(),
                    gdims_0.as_mut_ptr(),
                    ldims.as_mut_ptr(),
                    0 as libc::c_int as cl_uint,
                    std::ptr::null::<cl_event>(),
                    std::ptr::null_mut::<cl_event>(),
                );
                if status != 0 as libc::c_int {
                    (*h).param.b_opencl = 0 as libc::c_int;
                    (*h).opencl.b_fatal_error = 1 as libc::c_int;
                    x264_8_log(
                        h,
                        0 as libc::c_int,
                        b"clEnqueueNDRangeKernel error '%d'\n\0" as *const u8
                            as *const libc::c_char,
                        status,
                    );
                    return -(1 as libc::c_int);
                }
                b_shift_index = 0 as libc::c_int;
                b_first_iteration = 0 as libc::c_int;
                if scale > 2 as libc::c_int {
                    b_reverse_references ^= 1 as libc::c_int;
                } else {
                    b_reverse_references = 0 as libc::c_int;
                }
                A = (A == 0) as libc::c_int;
                if (*h).opencl.b_fatal_error != 0 {
                    return -(1 as libc::c_int);
                }
                status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
                    (*h).opencl.hme_kernel,
                    2 as libc::c_int as cl_uint,
                    ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
                    &mut *((*h).opencl.mv_buffers).as_mut_ptr().offset(A as isize) as *mut cl_mem
                        as *const libc::c_void,
                );
                if status != 0 as libc::c_int {
                    (*h).param.b_opencl = 0 as libc::c_int;
                    (*h).opencl.b_fatal_error = 1 as libc::c_int;
                    x264_8_log(
                        h,
                        0 as libc::c_int,
                        b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                        status,
                    );
                    return -(1 as libc::c_int);
                }
                if (*h).opencl.b_fatal_error != 0 {
                    return -(1 as libc::c_int);
                }
                status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
                    (*h).opencl.hme_kernel,
                    3 as libc::c_int as cl_uint,
                    ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
                    &mut *((*h).opencl.mv_buffers)
                        .as_mut_ptr()
                        .offset((A == 0) as libc::c_int as isize) as *mut cl_mem
                        as *const libc::c_void,
                );
                if status != 0 as libc::c_int {
                    (*h).param.b_opencl = 0 as libc::c_int;
                    (*h).opencl.b_fatal_error = 1 as libc::c_int;
                    x264_8_log(
                        h,
                        0 as libc::c_int,
                        b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                        status,
                    );
                    return -(1 as libc::c_int);
                }
                if (*h).opencl.b_fatal_error != 0 {
                    return -(1 as libc::c_int);
                }
                status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
                    (*h).opencl.hme_kernel,
                    arg_1.wrapping_sub(3 as libc::c_int as cl_uint),
                    ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                    &mut b_shift_index as *mut libc::c_int as *const libc::c_void,
                );
                if status != 0 as libc::c_int {
                    (*h).param.b_opencl = 0 as libc::c_int;
                    (*h).opencl.b_fatal_error = 1 as libc::c_int;
                    x264_8_log(
                        h,
                        0 as libc::c_int,
                        b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                        status,
                    );
                    return -(1 as libc::c_int);
                }
                if (*h).opencl.b_fatal_error != 0 {
                    return -(1 as libc::c_int);
                }
                status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
                    (*h).opencl.hme_kernel,
                    arg_1.wrapping_sub(2 as libc::c_int as cl_uint),
                    ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                    &mut b_first_iteration as *mut libc::c_int as *const libc::c_void,
                );
                if status != 0 as libc::c_int {
                    (*h).param.b_opencl = 0 as libc::c_int;
                    (*h).opencl.b_fatal_error = 1 as libc::c_int;
                    x264_8_log(
                        h,
                        0 as libc::c_int,
                        b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                        status,
                    );
                    return -(1 as libc::c_int);
                }
                if (*h).opencl.b_fatal_error != 0 {
                    return -(1 as libc::c_int);
                }
                status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
                    (*h).opencl.hme_kernel,
                    arg_1.wrapping_sub(1 as libc::c_int as cl_uint),
                    ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                    &mut b_reverse_references as *mut libc::c_int as *const libc::c_void,
                );
                if status != 0 as libc::c_int {
                    (*h).param.b_opencl = 0 as libc::c_int;
                    (*h).opencl.b_fatal_error = 1 as libc::c_int;
                    x264_8_log(
                        h,
                        0 as libc::c_int,
                        b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                        status,
                    );
                    return -(1 as libc::c_int);
                }
                iter += 1;
                iter;
            }
        }
        scale -= 1;
        scale;
    }
    let mut satd_local_size: libc::c_int = (mb_per_group as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<uint32_t>() as libc::c_ulong)
        .wrapping_mul(16 as libc::c_int as libc::c_ulong)
        as libc::c_int;
    let mut arg_2: cl_uint = 0 as libc::c_int as cl_uint;
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh46 = arg_2;
    arg_2 = arg_2.wrapping_add(1);
    status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
        (*h).opencl.subpel_refine_kernel,
        fresh46,
        ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
        &mut *((*fenc).opencl.scaled_image2Ds)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) as *mut cl_mem as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh47 = arg_2;
    arg_2 = arg_2.wrapping_add(1);
    status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
        (*h).opencl.subpel_refine_kernel,
        fresh47,
        ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
        &mut ref_luma_hpel as *mut cl_mem as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh48 = arg_2;
    arg_2 = arg_2.wrapping_add(1);
    status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
        (*h).opencl.subpel_refine_kernel,
        fresh48,
        ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
        &mut *((*h).opencl.mv_buffers).as_mut_ptr().offset(A as isize) as *mut cl_mem
            as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh49 = arg_2;
    arg_2 = arg_2.wrapping_add(1);
    status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
        (*h).opencl.subpel_refine_kernel,
        fresh49,
        ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
        &mut (*h).opencl.lowres_mv_costs as *mut cl_mem as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh50 = arg_2;
    arg_2 = arg_2.wrapping_add(1);
    status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
        (*h).opencl.subpel_refine_kernel,
        fresh50,
        cost_local_size as size_t,
        std::ptr::null::<libc::c_void>(),
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh51 = arg_2;
    arg_2 = arg_2.wrapping_add(1);
    status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
        (*h).opencl.subpel_refine_kernel,
        fresh51,
        satd_local_size as size_t,
        std::ptr::null::<libc::c_void>(),
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh52 = arg_2;
    arg_2 = arg_2.wrapping_add(1);
    status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
        (*h).opencl.subpel_refine_kernel,
        fresh52,
        mvc_local_size as size_t,
        std::ptr::null::<libc::c_void>(),
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if b_islist1 != 0 {
        if (*h).opencl.b_fatal_error != 0 {
            return -(1 as libc::c_int);
        }
        let fresh53 = arg_2;
        arg_2 = arg_2.wrapping_add(1);
        status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
            (*h).opencl.subpel_refine_kernel,
            fresh53,
            ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
            &mut (*fenc).opencl.lowres_mvs1 as *mut cl_mem as *const libc::c_void,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            (*h).opencl.b_fatal_error = 1 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        if (*h).opencl.b_fatal_error != 0 {
            return -(1 as libc::c_int);
        }
        let fresh54 = arg_2;
        arg_2 = arg_2.wrapping_add(1);
        status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
            (*h).opencl.subpel_refine_kernel,
            fresh54,
            ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
            &mut (*fenc).opencl.lowres_mv_costs1 as *mut cl_mem as *const libc::c_void,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            (*h).opencl.b_fatal_error = 1 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
    } else {
        if (*h).opencl.b_fatal_error != 0 {
            return -(1 as libc::c_int);
        }
        let fresh55 = arg_2;
        arg_2 = arg_2.wrapping_add(1);
        status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
            (*h).opencl.subpel_refine_kernel,
            fresh55,
            ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
            &mut (*fenc).opencl.lowres_mvs0 as *mut cl_mem as *const libc::c_void,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            (*h).opencl.b_fatal_error = 1 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        if (*h).opencl.b_fatal_error != 0 {
            return -(1 as libc::c_int);
        }
        let fresh56 = arg_2;
        arg_2 = arg_2.wrapping_add(1);
        status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
            (*h).opencl.subpel_refine_kernel,
            fresh56,
            ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
            &mut (*fenc).opencl.lowres_mv_costs0 as *mut cl_mem as *const libc::c_void,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            (*h).opencl.b_fatal_error = 1 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh57 = arg_2;
    arg_2 = arg_2.wrapping_add(1);
    status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
        (*h).opencl.subpel_refine_kernel,
        fresh57,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        &mut mb_width as *mut libc::c_int as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh58 = arg_2;
    arg_2 = arg_2.wrapping_add(1);
    status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
        (*h).opencl.subpel_refine_kernel,
        fresh58,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        &mut lambda as *mut libc::c_int as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh59 = arg_2;
    arg_2 = arg_2.wrapping_add(1);
    status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
        (*h).opencl.subpel_refine_kernel,
        fresh59,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        &mut b as *mut libc::c_int as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh60 = arg_2;
    arg_2 = arg_2.wrapping_add(1);
    status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
        (*h).opencl.subpel_refine_kernel,
        fresh60,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        &mut ref_0 as *mut libc::c_int as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh61 = arg_2;
    arg_2 = arg_2.wrapping_add(1);
    status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
        (*h).opencl.subpel_refine_kernel,
        fresh61,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        &mut b_islist1 as *mut libc::c_int as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_device_AMD_SI != 0 {
        if (*h).opencl.b_fatal_error != 0 {
            return -(1 as libc::c_int);
        }
        status = ((*ocl).clEnqueueCopyBuffer).expect("non-null function pointer")(
            (*h).opencl.queue,
            (*h).opencl.mv_buffers[A as usize],
            (*h).opencl.mv_buffers[(A == 0) as libc::c_int as usize],
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            20 as libc::c_int as size_t,
            0 as libc::c_int as cl_uint,
            std::ptr::null::<cl_event>(),
            std::ptr::null_mut::<cl_event>(),
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            (*h).opencl.b_fatal_error = 1 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clEnqueueCopyBuffer error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    status = ((*ocl).clEnqueueNDRangeKernel).expect("non-null function pointer")(
        (*h).opencl.queue,
        (*h).opencl.subpel_refine_kernel,
        2 as libc::c_int as cl_uint,
        std::ptr::null::<size_t>(),
        gdims_0.as_mut_ptr(),
        ldims.as_mut_ptr(),
        0 as libc::c_int as cl_uint,
        std::ptr::null::<cl_event>(),
        std::ptr::null_mut::<cl_event>(),
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clEnqueueNDRangeKernel error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    let mut mvlen: libc::c_int = (2 as libc::c_int as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong)
        .wrapping_mul((*h).mb.i_mb_count as libc::c_ulong)
        as libc::c_int;
    if (*h).opencl.num_copies >= 1024 as libc::c_int - 1 as libc::c_int {
        x264_8_opencl_flush(h);
    }
    let mut locked: *mut libc::c_char = opencl_alloc_locked(h, mvlen) as *mut libc::c_char;
    (*h).opencl.copies[(*h).opencl.num_copies as usize].src = locked as *mut libc::c_void;
    (*h).opencl.copies[(*h).opencl.num_copies as usize].bytes = mvlen;
    if b_islist1 != 0 {
        let mut mvs_offset: libc::c_int = mvlen * (ref_0 - b - 1 as libc::c_int);
        if (*h).opencl.b_fatal_error != 0 {
            return -(1 as libc::c_int);
        }
        status = ((*ocl).clEnqueueReadBuffer).expect("non-null function pointer")(
            (*h).opencl.queue,
            (*fenc).opencl.lowres_mvs1,
            0 as libc::c_int as cl_bool,
            mvs_offset as size_t,
            mvlen as size_t,
            locked as *mut libc::c_void,
            0 as libc::c_int as cl_uint,
            std::ptr::null::<cl_event>(),
            std::ptr::null_mut::<cl_event>(),
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            (*h).opencl.b_fatal_error = 1 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clEnqueueReadBuffer error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        (*h).opencl.copies[(*h).opencl.num_copies as usize].dest = (*fenc).lowres_mvs
            [1 as libc::c_int as usize][(ref_0 - b - 1 as libc::c_int) as usize]
            as *mut libc::c_void;
    } else {
        let mut mvs_offset_0: libc::c_int = mvlen * (b - ref_0 - 1 as libc::c_int);
        if (*h).opencl.b_fatal_error != 0 {
            return -(1 as libc::c_int);
        }
        status = ((*ocl).clEnqueueReadBuffer).expect("non-null function pointer")(
            (*h).opencl.queue,
            (*fenc).opencl.lowres_mvs0,
            0 as libc::c_int as cl_bool,
            mvs_offset_0 as size_t,
            mvlen as size_t,
            locked as *mut libc::c_void,
            0 as libc::c_int as cl_uint,
            std::ptr::null::<cl_event>(),
            std::ptr::null_mut::<cl_event>(),
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            (*h).opencl.b_fatal_error = 1 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clEnqueueReadBuffer error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        (*h).opencl.copies[(*h).opencl.num_copies as usize].dest = (*fenc).lowres_mvs
            [0 as libc::c_int as usize][(b - ref_0 - 1 as libc::c_int) as usize]
            as *mut libc::c_void;
    }
    (*h).opencl.num_copies += 1;
    (*h).opencl.num_copies;
    0 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_opencl_finalize_cost(
    mut h: *mut x264_t,
    mut lambda: libc::c_int,
    mut frames: *mut *mut x264_frame_t,
    mut p0: libc::c_int,
    mut p1: libc::c_int,
    mut b: libc::c_int,
    mut dist_scale_factor: libc::c_int,
) -> libc::c_int {
    let mut ocl: *mut x264_opencl_function_t = (*h).opencl.ocl;
    let mut status: cl_int = 0;
    let mut fenc: *mut x264_frame_t = *frames.offset(b as isize);
    let mut fref0: *mut x264_frame_t = *frames.offset(p0 as isize);
    let mut fref1: *mut x264_frame_t = *frames.offset(p1 as isize);
    let mut bipred_weight: libc::c_int = if (*h).param.analyse.b_weighted_bipred != 0 {
        64 as libc::c_int - (dist_scale_factor >> 2 as libc::c_int)
    } else {
        32 as libc::c_int
    };
    let mut gdims: [size_t; 2] = [(*h).mb.i_mb_width as size_t, (*h).mb.i_mb_height as size_t];
    let mut ldim_bidir: [size_t; 2] = [0; 2];
    let mut ldims: *mut size_t = std::ptr::null_mut::<size_t>();
    let mut cost_local_size: libc::c_int = 4 as libc::c_int;
    let mut satd_local_size: libc::c_int = 4 as libc::c_int;
    if b < p1 {
        ldims = ldim_bidir.as_mut_ptr();
        gdims[0 as libc::c_int as usize] <<= 2 as libc::c_int;
        optimal_launch_dims(
            h,
            gdims.as_mut_ptr(),
            ldims,
            (*h).opencl.mode_select_kernel,
            (*h).opencl.device,
        );
        let mut mb_per_group: libc::c_int =
            ((*ldims.offset(0 as libc::c_int as isize) >> 2 as libc::c_int)
                * *ldims.offset(1 as libc::c_int as isize)) as libc::c_int;
        cost_local_size = ((4 as libc::c_int * mb_per_group) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong)
            as libc::c_int;
        satd_local_size = ((16 as libc::c_int * mb_per_group) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<uint32_t>() as libc::c_ulong)
            as libc::c_int;
    }
    let mut arg: cl_uint = 0 as libc::c_int as cl_uint;
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh62 = arg;
    arg = arg.wrapping_add(1);
    status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
        (*h).opencl.mode_select_kernel,
        fresh62,
        ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
        &mut *((*fenc).opencl.scaled_image2Ds)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) as *mut cl_mem as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh63 = arg;
    arg = arg.wrapping_add(1);
    status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
        (*h).opencl.mode_select_kernel,
        fresh63,
        ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
        &mut (*fref0).opencl.luma_hpel as *mut cl_mem as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh64 = arg;
    arg = arg.wrapping_add(1);
    status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
        (*h).opencl.mode_select_kernel,
        fresh64,
        ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
        &mut (*fref1).opencl.luma_hpel as *mut cl_mem as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh65 = arg;
    arg = arg.wrapping_add(1);
    status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
        (*h).opencl.mode_select_kernel,
        fresh65,
        ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
        &mut (*fenc).opencl.lowres_mvs0 as *mut cl_mem as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh66 = arg;
    arg = arg.wrapping_add(1);
    status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
        (*h).opencl.mode_select_kernel,
        fresh66,
        ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
        &mut (*fenc).opencl.lowres_mvs1 as *mut cl_mem as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh67 = arg;
    arg = arg.wrapping_add(1);
    status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
        (*h).opencl.mode_select_kernel,
        fresh67,
        ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
        &mut (*fref1).opencl.lowres_mvs0 as *mut cl_mem as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh68 = arg;
    arg = arg.wrapping_add(1);
    status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
        (*h).opencl.mode_select_kernel,
        fresh68,
        ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
        &mut (*fenc).opencl.lowres_mv_costs0 as *mut cl_mem as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh69 = arg;
    arg = arg.wrapping_add(1);
    status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
        (*h).opencl.mode_select_kernel,
        fresh69,
        ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
        &mut (*fenc).opencl.lowres_mv_costs1 as *mut cl_mem as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh70 = arg;
    arg = arg.wrapping_add(1);
    status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
        (*h).opencl.mode_select_kernel,
        fresh70,
        ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
        &mut (*fenc).opencl.intra_cost as *mut cl_mem as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh71 = arg;
    arg = arg.wrapping_add(1);
    status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
        (*h).opencl.mode_select_kernel,
        fresh71,
        ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
        &mut *((*h).opencl.lowres_costs)
            .as_mut_ptr()
            .offset((*h).opencl.last_buf as isize) as *mut cl_mem as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh72 = arg;
    arg = arg.wrapping_add(1);
    status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
        (*h).opencl.mode_select_kernel,
        fresh72,
        ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
        &mut *((*h).opencl.frame_stats)
            .as_mut_ptr()
            .offset((*h).opencl.last_buf as isize) as *mut cl_mem as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh73 = arg;
    arg = arg.wrapping_add(1);
    status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
        (*h).opencl.mode_select_kernel,
        fresh73,
        cost_local_size as size_t,
        std::ptr::null::<libc::c_void>(),
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh74 = arg;
    arg = arg.wrapping_add(1);
    status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
        (*h).opencl.mode_select_kernel,
        fresh74,
        satd_local_size as size_t,
        std::ptr::null::<libc::c_void>(),
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh75 = arg;
    arg = arg.wrapping_add(1);
    status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
        (*h).opencl.mode_select_kernel,
        fresh75,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        &mut (*h).mb.i_mb_width as *mut libc::c_int as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh76 = arg;
    arg = arg.wrapping_add(1);
    status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
        (*h).opencl.mode_select_kernel,
        fresh76,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        &mut bipred_weight as *mut libc::c_int as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh77 = arg;
    arg = arg.wrapping_add(1);
    status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
        (*h).opencl.mode_select_kernel,
        fresh77,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        &mut dist_scale_factor as *mut libc::c_int as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh78 = arg;
    arg = arg.wrapping_add(1);
    status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
        (*h).opencl.mode_select_kernel,
        fresh78,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        &mut b as *mut libc::c_int as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh79 = arg;
    arg = arg.wrapping_add(1);
    status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
        (*h).opencl.mode_select_kernel,
        fresh79,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        &mut p0 as *mut libc::c_int as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh80 = arg;
    arg = arg.wrapping_add(1);
    status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
        (*h).opencl.mode_select_kernel,
        fresh80,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        &mut p1 as *mut libc::c_int as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh81 = arg;
    arg = arg.wrapping_add(1);
    status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
        (*h).opencl.mode_select_kernel,
        fresh81,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        &mut lambda as *mut libc::c_int as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    status = ((*ocl).clEnqueueNDRangeKernel).expect("non-null function pointer")(
        (*h).opencl.queue,
        (*h).opencl.mode_select_kernel,
        2 as libc::c_int as cl_uint,
        std::ptr::null::<size_t>(),
        gdims.as_mut_ptr(),
        ldims,
        0 as libc::c_int as cl_uint,
        std::ptr::null::<cl_event>(),
        std::ptr::null_mut::<cl_event>(),
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clEnqueueNDRangeKernel error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    let mut gdim: [size_t; 2] = [256 as libc::c_int as size_t, (*h).mb.i_mb_height as size_t];
    let mut ldim: [size_t; 2] = [256 as libc::c_int as size_t, 1 as libc::c_int as size_t];
    arg = 0 as libc::c_int as cl_uint;
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh82 = arg;
    arg = arg.wrapping_add(1);
    status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
        (*h).opencl.rowsum_inter_kernel,
        fresh82,
        ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
        &mut *((*h).opencl.lowres_costs)
            .as_mut_ptr()
            .offset((*h).opencl.last_buf as isize) as *mut cl_mem as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh83 = arg;
    arg = arg.wrapping_add(1);
    status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
        (*h).opencl.rowsum_inter_kernel,
        fresh83,
        ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
        &mut (*fenc).opencl.inv_qscale_factor as *mut cl_mem as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh84 = arg;
    arg = arg.wrapping_add(1);
    status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
        (*h).opencl.rowsum_inter_kernel,
        fresh84,
        ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
        &mut *((*h).opencl.row_satds)
            .as_mut_ptr()
            .offset((*h).opencl.last_buf as isize) as *mut cl_mem as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh85 = arg;
    arg = arg.wrapping_add(1);
    status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
        (*h).opencl.rowsum_inter_kernel,
        fresh85,
        ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
        &mut *((*h).opencl.frame_stats)
            .as_mut_ptr()
            .offset((*h).opencl.last_buf as isize) as *mut cl_mem as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh86 = arg;
    arg = arg.wrapping_add(1);
    status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
        (*h).opencl.rowsum_inter_kernel,
        fresh86,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        &mut (*h).mb.i_mb_width as *mut libc::c_int as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh87 = arg;
    arg = arg.wrapping_add(1);
    status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
        (*h).opencl.rowsum_inter_kernel,
        fresh87,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        &mut (*h).param.i_bframe_bias as *mut libc::c_int as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh88 = arg;
    arg = arg.wrapping_add(1);
    status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
        (*h).opencl.rowsum_inter_kernel,
        fresh88,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        &mut b as *mut libc::c_int as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh89 = arg;
    arg = arg.wrapping_add(1);
    status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
        (*h).opencl.rowsum_inter_kernel,
        fresh89,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        &mut p0 as *mut libc::c_int as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh90 = arg;
    arg = arg.wrapping_add(1);
    status = ((*ocl).clSetKernelArg).expect("non-null function pointer")(
        (*h).opencl.rowsum_inter_kernel,
        fresh90,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        &mut p1 as *mut libc::c_int as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    status = ((*ocl).clEnqueueNDRangeKernel).expect("non-null function pointer")(
        (*h).opencl.queue,
        (*h).opencl.rowsum_inter_kernel,
        2 as libc::c_int as cl_uint,
        std::ptr::null::<size_t>(),
        gdim.as_mut_ptr(),
        ldim.as_mut_ptr(),
        0 as libc::c_int as cl_uint,
        std::ptr::null::<cl_event>(),
        std::ptr::null_mut::<cl_event>(),
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clEnqueueNDRangeKernel error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.num_copies >= 1024 as libc::c_int - 4 as libc::c_int {
        x264_8_opencl_flush(h);
    }
    let mut size: libc::c_int = ((*h).mb.i_mb_count as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong)
        as libc::c_int;
    let mut locked: *mut libc::c_char = opencl_alloc_locked(h, size) as *mut libc::c_char;
    (*h).opencl.copies[(*h).opencl.num_copies as usize].src = locked as *mut libc::c_void;
    (*h).opencl.copies[(*h).opencl.num_copies as usize].dest =
        (*fenc).lowres_costs[(b - p0) as usize][(p1 - b) as usize] as *mut libc::c_void;
    (*h).opencl.copies[(*h).opencl.num_copies as usize].bytes = size;
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    status = ((*ocl).clEnqueueReadBuffer).expect("non-null function pointer")(
        (*h).opencl.queue,
        (*h).opencl.lowres_costs[(*h).opencl.last_buf as usize],
        0 as libc::c_int as cl_bool,
        0 as libc::c_int as size_t,
        size as size_t,
        locked as *mut libc::c_void,
        0 as libc::c_int as cl_uint,
        std::ptr::null::<cl_event>(),
        std::ptr::null_mut::<cl_event>(),
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clEnqueueReadBuffer error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    (*h).opencl.num_copies += 1;
    (*h).opencl.num_copies;
    size = ((*h).mb.i_mb_height as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
        as libc::c_int;
    locked = opencl_alloc_locked(h, size) as *mut libc::c_char;
    (*h).opencl.copies[(*h).opencl.num_copies as usize].src = locked as *mut libc::c_void;
    (*h).opencl.copies[(*h).opencl.num_copies as usize].dest =
        (*fenc).i_row_satds[(b - p0) as usize][(p1 - b) as usize] as *mut libc::c_void;
    (*h).opencl.copies[(*h).opencl.num_copies as usize].bytes = size;
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    status = ((*ocl).clEnqueueReadBuffer).expect("non-null function pointer")(
        (*h).opencl.queue,
        (*h).opencl.row_satds[(*h).opencl.last_buf as usize],
        0 as libc::c_int as cl_bool,
        0 as libc::c_int as size_t,
        size as size_t,
        locked as *mut libc::c_void,
        0 as libc::c_int as cl_uint,
        std::ptr::null::<cl_event>(),
        std::ptr::null_mut::<cl_event>(),
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clEnqueueReadBuffer error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    (*h).opencl.num_copies += 1;
    (*h).opencl.num_copies;
    size = (4 as libc::c_int as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
        as libc::c_int;
    locked = opencl_alloc_locked(h, size) as *mut libc::c_char;
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    status = ((*ocl).clEnqueueReadBuffer).expect("non-null function pointer")(
        (*h).opencl.queue,
        (*h).opencl.frame_stats[(*h).opencl.last_buf as usize],
        0 as libc::c_int as cl_bool,
        0 as libc::c_int as size_t,
        size as size_t,
        locked as *mut libc::c_void,
        0 as libc::c_int as cl_uint,
        std::ptr::null::<cl_event>(),
        std::ptr::null_mut::<cl_event>(),
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clEnqueueReadBuffer error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    (*h).opencl.last_buf = ((*h).opencl.last_buf == 0) as libc::c_int;
    (*h).opencl.copies[(*h).opencl.num_copies as usize].src = locked as *mut libc::c_void;
    (*h).opencl.copies[(*h).opencl.num_copies as usize].dest =
        &mut *(*((*fenc).i_cost_est).as_mut_ptr().offset((b - p0) as isize))
            .as_mut_ptr()
            .offset((p1 - b) as isize) as *mut libc::c_int as *mut libc::c_void;
    (*h).opencl.copies[(*h).opencl.num_copies as usize].bytes =
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int;
    (*h).opencl.num_copies += 1;
    (*h).opencl.num_copies;
    (*h).opencl.copies[(*h).opencl.num_copies as usize].src = locked
        .offset(::core::mem::size_of::<libc::c_int>() as libc::c_ulong as isize)
        as *mut libc::c_void;
    (*h).opencl.copies[(*h).opencl.num_copies as usize].dest =
        &mut *(*((*fenc).i_cost_est_aq)
            .as_mut_ptr()
            .offset((b - p0) as isize))
        .as_mut_ptr()
        .offset((p1 - b) as isize) as *mut libc::c_int as *mut libc::c_void;
    (*h).opencl.copies[(*h).opencl.num_copies as usize].bytes =
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int;
    (*h).opencl.num_copies += 1;
    (*h).opencl.num_copies;
    if b == p1 {
        (*h).opencl.copies[(*h).opencl.num_copies as usize].src = locked.offset(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                as isize,
        ) as *mut libc::c_void;
        (*h).opencl.copies[(*h).opencl.num_copies as usize].dest =
            &mut *((*fenc).i_intra_mbs).as_mut_ptr().offset((b - p0) as isize) as *mut libc::c_int
                as *mut libc::c_void;
        (*h).opencl.copies[(*h).opencl.num_copies as usize].bytes =
            ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int;
        (*h).opencl.num_copies += 1;
        (*h).opencl.num_copies;
    }
    0 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_opencl_slicetype_prep(
    mut h: *mut x264_t,
    mut frames: *mut *mut x264_frame_t,
    mut num_frames: libc::c_int,
    mut lambda: libc::c_int,
) {
    if (*h).param.b_opencl != 0 {
        let mut i: libc::c_int = 0 as libc::c_int;
        while i <= num_frames {
            x264_8_opencl_lowres_init(h, *frames.offset(i as isize), lambda);
            i += 1;
            i;
        }
        x264_8_opencl_flush(h);
        if (*h).param.i_bframe_adaptive == 2 as libc::c_int && (*h).param.i_bframe != 0 {
            let mut b: libc::c_int = 0 as libc::c_int;
            while b <= num_frames {
                let mut j: libc::c_int = 1 as libc::c_int;
                while j < (*h).param.i_bframe {
                    let mut p0: libc::c_int = b - j;
                    if p0 >= 0 as libc::c_int
                        && (*((**frames.offset(b as isize)).lowres_mvs[0 as libc::c_int as usize]
                            [(b - p0 - 1 as libc::c_int) as usize])
                            .offset(0 as libc::c_int as isize))
                            [0 as libc::c_int as usize] as libc::c_int
                            == 0x7fff as libc::c_int
                    {
                        let mut w: *const x264_weight_t =
                            x264_zero.as_mut_ptr() as *const x264_weight_t;
                        if (*h).param.analyse.i_weighted_pred != 0 {
                            x264_8_weights_analyse(
                                h,
                                *frames.offset(b as isize),
                                *frames.offset(p0 as isize),
                                1 as libc::c_int,
                            );
                            w = ((**frames.offset(b as isize)).weight[0 as libc::c_int as usize])
                                .as_mut_ptr();
                        }
                        (*((**frames.offset(b as isize)).lowres_mvs[0 as libc::c_int as usize]
                            [(b - p0 - 1 as libc::c_int) as usize])
                            .offset(0 as libc::c_int as isize))
                            [0 as libc::c_int as usize] = 0 as libc::c_int as int16_t;
                        x264_8_opencl_motionsearch(h, frames, b, p0, 0 as libc::c_int, lambda, w);
                    }
                    let mut p1: libc::c_int = b + j;
                    if p1 <= num_frames
                        && (*((**frames.offset(b as isize)).lowres_mvs[1 as libc::c_int as usize]
                            [(p1 - b - 1 as libc::c_int) as usize])
                            .offset(0 as libc::c_int as isize))
                            [0 as libc::c_int as usize] as libc::c_int
                            == 0x7fff as libc::c_int
                    {
                        (*((**frames.offset(b as isize)).lowres_mvs[1 as libc::c_int as usize]
                            [(p1 - b - 1 as libc::c_int) as usize])
                            .offset(0 as libc::c_int as isize))
                            [0 as libc::c_int as usize] = 0 as libc::c_int as int16_t;
                        x264_8_opencl_motionsearch(
                            h,
                            frames,
                            b,
                            p1,
                            1 as libc::c_int,
                            lambda,
                            std::ptr::null::<x264_weight_t>(),
                        );
                    }
                    j += 1;
                    j;
                }
                b += 1;
                b;
            }
            x264_8_opencl_flush(h);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_opencl_slicetype_end(mut h: *mut x264_t) {}
#[no_mangle]
pub unsafe extern "C" fn x264_8_opencl_precalculate_frame_cost(
    mut h: *mut x264_t,
    mut frames: *mut *mut x264_frame_t,
    mut lambda: libc::c_int,
    mut p0: libc::c_int,
    mut p1: libc::c_int,
    mut b: libc::c_int,
) -> libc::c_int {
    if (**frames.offset(b as isize)).i_cost_est[(b - p0) as usize][(p1 - b) as usize]
        >= 0 as libc::c_int
        || b == p0 && b == p1
    {
        0 as libc::c_int
    } else {
        let mut do_search: [libc::c_int; 2] = [0; 2];
        let mut dist_scale_factor: libc::c_int = 128 as libc::c_int;
        let mut w: *const x264_weight_t = x264_zero.as_mut_ptr() as *const x264_weight_t;
        (**frames.offset(b as isize)).i_cost_est[(b - p0) as usize][(p1 - b) as usize] =
            0 as libc::c_int;
        do_search[0 as libc::c_int as usize] = (b != p0
            && (*((**frames.offset(b as isize)).lowres_mvs[0 as libc::c_int as usize]
                [(b - p0 - 1 as libc::c_int) as usize])
                .offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
                as libc::c_int
                == 0x7fff as libc::c_int)
            as libc::c_int;
        do_search[1 as libc::c_int as usize] = (b != p1
            && (*((**frames.offset(b as isize)).lowres_mvs[1 as libc::c_int as usize]
                [(p1 - b - 1 as libc::c_int) as usize])
                .offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
                as libc::c_int
                == 0x7fff as libc::c_int)
            as libc::c_int;
        if do_search[0 as libc::c_int as usize] != 0 {
            if (*h).param.analyse.i_weighted_pred != 0 && b == p1 {
                x264_8_weights_analyse(
                    h,
                    *frames.offset(b as isize),
                    *frames.offset(p0 as isize),
                    1 as libc::c_int,
                );
                w = ((**frames.offset(b as isize)).weight[0 as libc::c_int as usize]).as_mut_ptr();
            }
            (*((**frames.offset(b as isize)).lowres_mvs[0 as libc::c_int as usize]
                [(b - p0 - 1 as libc::c_int) as usize])
                .offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] =
                0 as libc::c_int as int16_t;
        }
        if do_search[1 as libc::c_int as usize] != 0 {
            (*((**frames.offset(b as isize)).lowres_mvs[1 as libc::c_int as usize]
                [(p1 - b - 1 as libc::c_int) as usize])
                .offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] =
                0 as libc::c_int as int16_t;
        }
        if b == p1 {
            (**frames.offset(b as isize)).i_intra_mbs[(b - p0) as usize] = 0 as libc::c_int;
        }
        if p1 != p0 {
            dist_scale_factor =
                (((b - p0) << 8 as libc::c_int) + ((p1 - p0) >> 1 as libc::c_int)) / (p1 - p0);
        }
        (**frames.offset(b as isize)).i_cost_est[(b - p0) as usize][(p1 - b) as usize] =
            0 as libc::c_int;
        (**frames.offset(b as isize)).i_cost_est_aq[(b - p0) as usize][(p1 - b) as usize] =
            0 as libc::c_int;
        x264_8_opencl_lowres_init(h, *frames.offset(b as isize), lambda);
        if do_search[0 as libc::c_int as usize] != 0 {
            x264_8_opencl_lowres_init(h, *frames.offset(p0 as isize), lambda);
            x264_8_opencl_motionsearch(h, frames, b, p0, 0 as libc::c_int, lambda, w);
        }
        if do_search[1 as libc::c_int as usize] != 0 {
            x264_8_opencl_lowres_init(h, *frames.offset(p1 as isize), lambda);
            x264_8_opencl_motionsearch(
                h,
                frames,
                b,
                p1,
                1 as libc::c_int,
                lambda,
                std::ptr::null::<x264_weight_t>(),
            );
        }
        x264_8_opencl_finalize_cost(h, lambda, frames, p0, p1, b, dist_scale_factor);
        1 as libc::c_int
    }
}
