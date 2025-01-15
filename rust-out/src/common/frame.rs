use crate::types::*;
extern "C" {
    fn abs(_: libc::c_int) -> libc::c_int;
    fn x264_param_cleanup(param: *mut x264_param_t);
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> libc::c_int;
    fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_wait(__cond: *mut pthread_cond_t, __mutex: *mut pthread_mutex_t)
        -> libc::c_int;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn x264_malloc(_: int64_t) -> *mut libc::c_void;
    fn x264_free(_: *mut libc::c_void);
    fn x264_8_opencl_frame_delete(frame: *mut x264_frame);
    fn x264_8_log(h: *mut x264_t, i_level: libc::c_int, psz_fmt: *const libc::c_char, _: ...);
}
#[inline(always)]
unsafe extern "C" fn x264_pthread_fetch_and_add(
    mut val: *mut libc::c_int,
    mut add: libc::c_int,
    mut mutex: *mut pthread_mutex_t,
) -> libc::c_int {
    ::core::intrinsics::atomic_xadd_seqcst(val, add)
}
unsafe extern "C" fn align_stride(
    mut x: libc::c_int,
    mut align: libc::c_int,
    mut disalign: libc::c_int,
) -> libc::c_int {
    x = (x + (align - 1 as libc::c_int)) & !(align - 1 as libc::c_int);
    if x & (disalign - 1 as libc::c_int) == 0 {
        x += align;
    }
    x
}
unsafe extern "C" fn align_plane_size(
    mut x: libc::c_int,
    mut disalign: libc::c_int,
) -> libc::c_int {
    if x & (disalign - 1 as libc::c_int) == 0 {
        x += (if 128 as libc::c_int > 64 as libc::c_int {
            128 as libc::c_int
        } else {
            64 as libc::c_int
        }) / ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int;
    }
    x
}
unsafe extern "C" fn frame_internal_csp(mut external_csp: libc::c_int) -> libc::c_int {
    let mut csp: libc::c_int = external_csp & 0xff as libc::c_int;
    if csp == 0x1 as libc::c_int {
        return 0x1 as libc::c_int;
    }
    if csp >= 0x2 as libc::c_int && csp < 0x6 as libc::c_int {
        return 0x4 as libc::c_int;
    }
    if csp >= 0x6 as libc::c_int && csp < 0xc as libc::c_int {
        return 0x8 as libc::c_int;
    }
    if csp >= 0xc as libc::c_int && csp <= 0x10 as libc::c_int {
        return 0xc as libc::c_int;
    }
    0 as libc::c_int
}
unsafe extern "C" fn frame_new(mut h: *mut x264_t, mut b_fdec: libc::c_int) -> *mut x264_frame_t {
    let mut prealloc_idx: libc::c_int = 0;
    let mut prealloc_size: int64_t = 0;
    let mut preallocs: [*mut *mut uint8_t; 1024] = [std::ptr::null_mut::<*mut uint8_t>(); 1024];
    let mut current_block: u64;
    let mut frame: *mut x264_frame_t = std::ptr::null_mut::<x264_frame_t>();
    let mut i_csp: libc::c_int = frame_internal_csp((*h).param.i_csp);
    let mut i_mb_count: libc::c_int = (*h).mb.i_mb_count;
    let mut i_stride: libc::c_int = 0;
    let mut i_width: libc::c_int = 0;
    let mut i_lines: libc::c_int = 0;
    let mut luma_plane_count: libc::c_int = 0;
    let mut i_padv: libc::c_int = (32 as libc::c_int) << (*h).param.b_interlaced;
    let mut align: libc::c_int =
        64 as libc::c_int / ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int;
    if (*h).param.cpu & ((1 as libc::c_uint) << 18 as libc::c_int) != 0
        || (*h).param.cpu & ((1 as libc::c_uint) << 16 as libc::c_int) != 0
    {
        align = 64 as libc::c_int / ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int;
    } else if (*h).param.cpu & ((1 as libc::c_uint) << 17 as libc::c_int) != 0
        || (*h).param.cpu & ((1 as libc::c_uint) << 9 as libc::c_int) != 0
    {
        align = 32 as libc::c_int / ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int;
    } else {
        align = 16 as libc::c_int / ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int;
    }
    let mut disalign: libc::c_int = ((1 as libc::c_int) << 10 as libc::c_int)
        / ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int;
    frame = x264_malloc(::core::mem::size_of::<x264_frame_t>() as libc::c_ulong as int64_t)
        as *mut x264_frame_t;
    if !frame.is_null() {
        memset(
            frame as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<x264_frame_t>() as libc::c_ulong,
        );
        prealloc_idx = 0 as libc::c_int;
        prealloc_size = 0 as libc::c_int as int64_t;
        preallocs = [std::ptr::null_mut::<*mut uint8_t>(); 1024];
        i_width = (*h).mb.i_mb_width * 16 as libc::c_int;
        i_lines = (*h).mb.i_mb_height * 16 as libc::c_int;
        i_stride = align_stride(
            i_width
                + ((if 32 as libc::c_int
                    > 64 as libc::c_int
                        / ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int
                {
                    32 as libc::c_int
                } else {
                    64 as libc::c_int
                        / ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int
                }) + 32 as libc::c_int),
            align,
            disalign,
        );
        if i_csp == 0x4 as libc::c_int || i_csp == 0x8 as libc::c_int {
            luma_plane_count = 1 as libc::c_int;
            (*frame).i_plane = 2 as libc::c_int;
            let mut i: libc::c_int = 0 as libc::c_int;
            while i < 2 as libc::c_int {
                (*frame).i_width[i as usize] = i_width >> i;
                (*frame).i_lines[i as usize] =
                    i_lines >> (i != 0 && i_csp == 0x4 as libc::c_int) as libc::c_int;
                (*frame).i_stride[i as usize] = i_stride;
                i += 1;
                i;
            }
            current_block = 7245201122033322888;
        } else if i_csp == 0xc as libc::c_int {
            luma_plane_count = 3 as libc::c_int;
            (*frame).i_plane = 3 as libc::c_int;
            let mut i_0: libc::c_int = 0 as libc::c_int;
            while i_0 < 3 as libc::c_int {
                (*frame).i_width[i_0 as usize] = i_width;
                (*frame).i_lines[i_0 as usize] = i_lines;
                (*frame).i_stride[i_0 as usize] = i_stride;
                i_0 += 1;
                i_0;
            }
            current_block = 7245201122033322888;
        } else if i_csp == 0x1 as libc::c_int {
            luma_plane_count = 1 as libc::c_int;
            (*frame).i_plane = 1 as libc::c_int;
            (*frame).i_width[0 as libc::c_int as usize] = i_width;
            (*frame).i_lines[0 as libc::c_int as usize] = i_lines;
            (*frame).i_stride[0 as libc::c_int as usize] = i_stride;
            current_block = 7245201122033322888;
        } else {
            current_block = 3641503318819273726;
        }
        match current_block {
            3641503318819273726 => {}
            _ => {
                (*frame).i_csp = i_csp;
                (*frame).i_width_lowres =
                    (*frame).i_width[0 as libc::c_int as usize] / 2 as libc::c_int;
                (*frame).i_lines_lowres =
                    (*frame).i_lines[0 as libc::c_int as usize] / 2 as libc::c_int;
                (*frame).i_stride_lowres = align_stride(
                    (*frame).i_width_lowres
                        + ((if 32 as libc::c_int
                            > 64 as libc::c_int
                                / ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int
                        {
                            32 as libc::c_int
                        } else {
                            64 as libc::c_int
                                / ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int
                        }) + 32 as libc::c_int),
                    align,
                    disalign << 1 as libc::c_int,
                );
                let mut i_1: libc::c_int = 0 as libc::c_int;
                while i_1 < (*h).param.i_bframe + 2 as libc::c_int {
                    let mut j: libc::c_int = 0 as libc::c_int;
                    while j < (*h).param.i_bframe + 2 as libc::c_int {
                        (*frame).i_row_satds[i_1 as usize][j as usize] =
                            prealloc_size as *mut libc::c_void as *mut libc::c_int;
                        let fresh0 = prealloc_idx;
                        prealloc_idx += 1;
                        preallocs[fresh0 as usize] =
                            &mut *(*((*frame).i_row_satds).as_mut_ptr().offset(i_1 as isize))
                                .as_mut_ptr()
                                .offset(j as isize)
                                as *mut *mut libc::c_int
                                as *mut *mut uint8_t;
                        prealloc_size += (((i_lines / 16 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                            as int64_t
                            + (64 as libc::c_int - 1 as libc::c_int) as int64_t)
                            & !(64 as libc::c_int - 1 as libc::c_int) as int64_t;
                        j += 1;
                        j;
                    }
                    i_1 += 1;
                    i_1;
                }
                (*frame).i_poc = -(1 as libc::c_int);
                (*frame).i_type = 0 as libc::c_int;
                (*frame).i_qpplus1 = 0 as libc::c_int;
                (*frame).i_pts = -(1 as libc::c_int) as int64_t;
                (*frame).i_frame = -(1 as libc::c_int);
                (*frame).i_frame_num = -(1 as libc::c_int);
                (*frame).i_lines_completed = -(1 as libc::c_int);
                (*frame).b_fdec = b_fdec as uint8_t;
                (*frame).i_pic_struct = PIC_STRUCT_AUTO as libc::c_int;
                (*frame).i_field_cnt = -(1 as libc::c_int) as int64_t;
                (*frame).i_cpb_delay = 0 as libc::c_int as int64_t;
                (*frame).i_dpb_output_delay = (*frame).i_cpb_delay;
                (*frame).i_cpb_duration = (*frame).i_dpb_output_delay;
                (*frame).i_duration = (*frame).i_cpb_duration;
                (*frame).i_cpb_delay_lookahead = -(1 as libc::c_int) as int64_t;
                (*frame).i_coded_fields_lookahead = (*frame).i_cpb_delay_lookahead;
                (*frame).orig = frame;
                if i_csp == 0x4 as libc::c_int || i_csp == 0x8 as libc::c_int {
                    let mut chroma_padv: libc::c_int =
                        i_padv >> (i_csp == 0x4 as libc::c_int) as libc::c_int;
                    let mut chroma_plane_size: libc::c_int = (*frame).i_stride
                        [1 as libc::c_int as usize]
                        * ((*frame).i_lines[1 as libc::c_int as usize]
                            + 2 as libc::c_int * chroma_padv);
                    (*frame).buffer[1 as libc::c_int as usize] =
                        prealloc_size as *mut libc::c_void as *mut pixel;
                    let fresh1 = prealloc_idx;
                    prealloc_idx += 1;
                    preallocs[fresh1 as usize] = &mut *((*frame).buffer)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize)
                        as *mut *mut pixel
                        as *mut *mut uint8_t;
                    prealloc_size += ((chroma_plane_size
                        * ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int)
                        as int64_t
                        + (64 as libc::c_int - 1 as libc::c_int) as int64_t)
                        & !(64 as libc::c_int - 1 as libc::c_int) as int64_t;
                    if (*h).param.b_interlaced != 0 {
                        (*frame).buffer_fld[1 as libc::c_int as usize] =
                            prealloc_size as *mut libc::c_void as *mut pixel;
                        let fresh2 = prealloc_idx;
                        prealloc_idx += 1;
                        preallocs[fresh2 as usize] = &mut *((*frame).buffer_fld)
                            .as_mut_ptr()
                            .offset(1 as libc::c_int as isize)
                            as *mut *mut pixel
                            as *mut *mut uint8_t;
                        prealloc_size += ((chroma_plane_size
                            * ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int)
                            as int64_t
                            + (64 as libc::c_int - 1 as libc::c_int) as int64_t)
                            & !(64 as libc::c_int - 1 as libc::c_int) as int64_t;
                    }
                }
                let mut p: libc::c_int = 0 as libc::c_int;
                while p < luma_plane_count {
                    let mut luma_plane_size: int64_t = align_plane_size(
                        (*frame).i_stride[p as usize]
                            * ((*frame).i_lines[p as usize] + 2 as libc::c_int * i_padv),
                        disalign,
                    ) as int64_t;
                    if (*h).param.analyse.i_subpel_refine != 0 && b_fdec != 0 {
                        luma_plane_size *= 4 as libc::c_int as int64_t;
                    }
                    (*frame).buffer[p as usize] = prealloc_size as *mut libc::c_void as *mut pixel;
                    let fresh3 = prealloc_idx;
                    prealloc_idx += 1;
                    preallocs[fresh3 as usize] =
                        &mut *((*frame).buffer).as_mut_ptr().offset(p as isize) as *mut *mut pixel
                            as *mut *mut uint8_t;
                    prealloc_size += (luma_plane_size
                        * ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int
                            as int64_t
                        + (64 as libc::c_int - 1 as libc::c_int) as int64_t)
                        & !(64 as libc::c_int - 1 as libc::c_int) as int64_t;
                    if (*h).param.b_interlaced != 0 {
                        (*frame).buffer_fld[p as usize] =
                            prealloc_size as *mut libc::c_void as *mut pixel;
                        let fresh4 = prealloc_idx;
                        prealloc_idx += 1;
                        preallocs[fresh4 as usize] =
                            &mut *((*frame).buffer_fld).as_mut_ptr().offset(p as isize)
                                as *mut *mut pixel as *mut *mut uint8_t;
                        prealloc_size += (luma_plane_size
                            * ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int
                                as int64_t
                            + (64 as libc::c_int - 1 as libc::c_int) as int64_t)
                            & !(64 as libc::c_int - 1 as libc::c_int) as int64_t;
                    }
                    p += 1;
                    p;
                }
                (*frame).b_duplicate = 0 as libc::c_int;
                if b_fdec != 0 {
                    (*frame).mb_type = prealloc_size as *mut libc::c_void as *mut int8_t;
                    let fresh5 = prealloc_idx;
                    prealloc_idx += 1;
                    preallocs[fresh5 as usize] =
                        &mut (*frame).mb_type as *mut *mut int8_t as *mut *mut uint8_t;
                    prealloc_size += ((i_mb_count as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<int8_t>() as libc::c_ulong)
                        as int64_t
                        + (64 as libc::c_int - 1 as libc::c_int) as int64_t)
                        & !(64 as libc::c_int - 1 as libc::c_int) as int64_t;
                    (*frame).mb_partition = prealloc_size as *mut libc::c_void as *mut uint8_t;
                    let fresh6 = prealloc_idx;
                    prealloc_idx += 1;
                    preallocs[fresh6 as usize] = &mut (*frame).mb_partition as *mut *mut uint8_t;
                    prealloc_size += ((i_mb_count as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<uint8_t>() as libc::c_ulong)
                        as int64_t
                        + (64 as libc::c_int - 1 as libc::c_int) as int64_t)
                        & !(64 as libc::c_int - 1 as libc::c_int) as int64_t;
                    (*frame).mv[0 as libc::c_int as usize] =
                        prealloc_size as *mut libc::c_void as *mut [int16_t; 2];
                    let fresh7 = prealloc_idx;
                    prealloc_idx += 1;
                    preallocs[fresh7 as usize] =
                        &mut *((*frame).mv).as_mut_ptr().offset(0 as libc::c_int as isize)
                            as *mut *mut [int16_t; 2] as *mut *mut uint8_t;
                    prealloc_size += (((2 as libc::c_int * 16 as libc::c_int * i_mb_count)
                        as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong)
                        as int64_t
                        + (64 as libc::c_int - 1 as libc::c_int) as int64_t)
                        & !(64 as libc::c_int - 1 as libc::c_int) as int64_t;
                    (*frame).mv16x16 = prealloc_size as *mut libc::c_void as *mut [int16_t; 2];
                    let fresh8 = prealloc_idx;
                    prealloc_idx += 1;
                    preallocs[fresh8 as usize] =
                        &mut (*frame).mv16x16 as *mut *mut [int16_t; 2] as *mut *mut uint8_t;
                    prealloc_size += (((2 as libc::c_int * (i_mb_count + 1 as libc::c_int))
                        as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong)
                        as int64_t
                        + (64 as libc::c_int - 1 as libc::c_int) as int64_t)
                        & !(64 as libc::c_int - 1 as libc::c_int) as int64_t;
                    (*frame).ref_0[0 as libc::c_int as usize] =
                        prealloc_size as *mut libc::c_void as *mut int8_t;
                    let fresh9 = prealloc_idx;
                    prealloc_idx += 1;
                    preallocs[fresh9 as usize] = &mut *((*frame).ref_0)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize)
                        as *mut *mut int8_t
                        as *mut *mut uint8_t;
                    prealloc_size += (((4 as libc::c_int * i_mb_count) as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<int8_t>() as libc::c_ulong)
                        as int64_t
                        + (64 as libc::c_int - 1 as libc::c_int) as int64_t)
                        & !(64 as libc::c_int - 1 as libc::c_int) as int64_t;
                    if (*h).param.i_bframe != 0 {
                        (*frame).mv[1 as libc::c_int as usize] =
                            prealloc_size as *mut libc::c_void as *mut [int16_t; 2];
                        let fresh10 = prealloc_idx;
                        prealloc_idx += 1;
                        preallocs[fresh10 as usize] =
                            &mut *((*frame).mv).as_mut_ptr().offset(1 as libc::c_int as isize)
                                as *mut *mut [int16_t; 2]
                                as *mut *mut uint8_t;
                        prealloc_size += (((2 as libc::c_int * 16 as libc::c_int * i_mb_count)
                            as libc::c_ulong)
                            .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong)
                            as int64_t
                            + (64 as libc::c_int - 1 as libc::c_int) as int64_t)
                            & !(64 as libc::c_int - 1 as libc::c_int) as int64_t;
                        (*frame).ref_0[1 as libc::c_int as usize] =
                            prealloc_size as *mut libc::c_void as *mut int8_t;
                        let fresh11 = prealloc_idx;
                        prealloc_idx += 1;
                        preallocs[fresh11 as usize] = &mut *((*frame).ref_0)
                            .as_mut_ptr()
                            .offset(1 as libc::c_int as isize)
                            as *mut *mut int8_t
                            as *mut *mut uint8_t;
                        prealloc_size += (((4 as libc::c_int * i_mb_count) as libc::c_ulong)
                            .wrapping_mul(::core::mem::size_of::<int8_t>() as libc::c_ulong)
                            as int64_t
                            + (64 as libc::c_int - 1 as libc::c_int) as int64_t)
                            & !(64 as libc::c_int - 1 as libc::c_int) as int64_t;
                    } else {
                        (*frame).mv[1 as libc::c_int as usize] =
                            std::ptr::null_mut::<[int16_t; 2]>();
                        (*frame).ref_0[1 as libc::c_int as usize] = std::ptr::null_mut::<int8_t>();
                    }
                    (*frame).i_row_bits = prealloc_size as *mut libc::c_void as *mut libc::c_int;
                    let fresh12 = prealloc_idx;
                    prealloc_idx += 1;
                    preallocs[fresh12 as usize] =
                        &mut (*frame).i_row_bits as *mut *mut libc::c_int as *mut *mut uint8_t;
                    prealloc_size += (((i_lines / 16 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                        as int64_t
                        + (64 as libc::c_int - 1 as libc::c_int) as int64_t)
                        & !(64 as libc::c_int - 1 as libc::c_int) as int64_t;
                    (*frame).f_row_qp = prealloc_size as *mut libc::c_void as *mut libc::c_float;
                    let fresh13 = prealloc_idx;
                    prealloc_idx += 1;
                    preallocs[fresh13 as usize] =
                        &mut (*frame).f_row_qp as *mut *mut libc::c_float as *mut *mut uint8_t;
                    prealloc_size += (((i_lines / 16 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<libc::c_float>() as libc::c_ulong)
                        as int64_t
                        + (64 as libc::c_int - 1 as libc::c_int) as int64_t)
                        & !(64 as libc::c_int - 1 as libc::c_int) as int64_t;
                    (*frame).f_row_qscale =
                        prealloc_size as *mut libc::c_void as *mut libc::c_float;
                    let fresh14 = prealloc_idx;
                    prealloc_idx += 1;
                    preallocs[fresh14 as usize] =
                        &mut (*frame).f_row_qscale as *mut *mut libc::c_float as *mut *mut uint8_t;
                    prealloc_size += (((i_lines / 16 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<libc::c_float>() as libc::c_ulong)
                        as int64_t
                        + (64 as libc::c_int - 1 as libc::c_int) as int64_t)
                        & !(64 as libc::c_int - 1 as libc::c_int) as int64_t;
                    if (*h).param.analyse.i_me_method >= 3 as libc::c_int {
                        (*frame).buffer[3 as libc::c_int as usize] =
                            prealloc_size as *mut libc::c_void as *mut pixel;
                        let fresh15 = prealloc_idx;
                        prealloc_idx += 1;
                        preallocs[fresh15 as usize] = &mut *((*frame).buffer)
                            .as_mut_ptr()
                            .offset(3 as libc::c_int as isize)
                            as *mut *mut pixel
                            as *mut *mut uint8_t;
                        prealloc_size += (((((*frame).i_stride[0 as libc::c_int as usize]
                            * ((*frame).i_lines[0 as libc::c_int as usize]
                                + 2 as libc::c_int * i_padv))
                            as libc::c_ulong)
                            .wrapping_mul(::core::mem::size_of::<uint16_t>() as libc::c_ulong)
                            << (*h).frames.b_have_sub8x8_esa)
                            as int64_t
                            + (64 as libc::c_int - 1 as libc::c_int) as int64_t)
                            & !(64 as libc::c_int - 1 as libc::c_int) as int64_t;
                    }
                    if (*h).param.b_interlaced != 0 {
                        (*frame).field = prealloc_size as *mut libc::c_void as *mut uint8_t;
                        let fresh16 = prealloc_idx;
                        prealloc_idx += 1;
                        preallocs[fresh16 as usize] = &mut (*frame).field as *mut *mut uint8_t;
                        prealloc_size += ((i_mb_count as libc::c_ulong)
                            .wrapping_mul(::core::mem::size_of::<uint8_t>() as libc::c_ulong)
                            as int64_t
                            + (64 as libc::c_int - 1 as libc::c_int) as int64_t)
                            & !(64 as libc::c_int - 1 as libc::c_int) as int64_t;
                    }
                    if (*h).param.analyse.b_mb_info != 0 {
                        (*frame).effective_qp = prealloc_size as *mut libc::c_void as *mut uint8_t;
                        let fresh17 = prealloc_idx;
                        prealloc_idx += 1;
                        preallocs[fresh17 as usize] =
                            &mut (*frame).effective_qp as *mut *mut uint8_t;
                        prealloc_size += ((i_mb_count as libc::c_ulong)
                            .wrapping_mul(::core::mem::size_of::<uint8_t>() as libc::c_ulong)
                            as int64_t
                            + (64 as libc::c_int - 1 as libc::c_int) as int64_t)
                            & !(64 as libc::c_int - 1 as libc::c_int) as int64_t;
                    }
                } else {
                    if (*h).frames.b_have_lowres != 0 {
                        let mut luma_plane_size_0: int64_t = align_plane_size(
                            (*frame).i_stride_lowres
                                * ((*frame).i_lines[0 as libc::c_int as usize] / 2 as libc::c_int
                                    + 2 as libc::c_int * 32 as libc::c_int),
                            disalign,
                        ) as int64_t;
                        (*frame).buffer_lowres = prealloc_size as *mut libc::c_void as *mut pixel;
                        let fresh18 = prealloc_idx;
                        prealloc_idx += 1;
                        preallocs[fresh18 as usize] =
                            &mut (*frame).buffer_lowres as *mut *mut pixel as *mut *mut uint8_t;
                        prealloc_size += (4 as libc::c_int as int64_t
                            * luma_plane_size_0
                            * ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int
                                as int64_t
                            + (64 as libc::c_int - 1 as libc::c_int) as int64_t)
                            & !(64 as libc::c_int - 1 as libc::c_int) as int64_t;
                        let mut j_0: libc::c_int = 0 as libc::c_int;
                        while j_0 <= ((*h).param.i_bframe != 0) as libc::c_int {
                            let mut i_2: libc::c_int = 0 as libc::c_int;
                            while i_2 <= (*h).param.i_bframe {
                                (*frame).lowres_mvs[j_0 as usize][i_2 as usize] =
                                    prealloc_size as *mut libc::c_void as *mut [int16_t; 2];
                                let fresh19 = prealloc_idx;
                                prealloc_idx += 1;
                                preallocs[fresh19 as usize] = &mut *(*((*frame).lowres_mvs)
                                    .as_mut_ptr()
                                    .offset(j_0 as isize))
                                .as_mut_ptr()
                                .offset(i_2 as isize)
                                    as *mut *mut [int16_t; 2]
                                    as *mut *mut uint8_t;
                                prealloc_size +=
                                    (((2 as libc::c_int * i_mb_count) as libc::c_ulong)
                                        .wrapping_mul(
                                            ::core::mem::size_of::<int16_t>() as libc::c_ulong
                                        ) as int64_t
                                        + (64 as libc::c_int - 1 as libc::c_int) as int64_t)
                                        & !(64 as libc::c_int - 1 as libc::c_int) as int64_t;
                                (*frame).lowres_mv_costs[j_0 as usize][i_2 as usize] =
                                    prealloc_size as *mut libc::c_void as *mut libc::c_int;
                                let fresh20 = prealloc_idx;
                                prealloc_idx += 1;
                                preallocs[fresh20 as usize] = &mut *(*((*frame).lowres_mv_costs)
                                    .as_mut_ptr()
                                    .offset(j_0 as isize))
                                .as_mut_ptr()
                                .offset(i_2 as isize)
                                    as *mut *mut libc::c_int
                                    as *mut *mut uint8_t;
                                prealloc_size += ((i_mb_count as libc::c_ulong).wrapping_mul(
                                    ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                                ) as int64_t
                                    + (64 as libc::c_int - 1 as libc::c_int) as int64_t)
                                    & !(64 as libc::c_int - 1 as libc::c_int) as int64_t;
                                i_2 += 1;
                                i_2;
                            }
                            j_0 += 1;
                            j_0;
                        }
                        (*frame).i_propagate_cost =
                            prealloc_size as *mut libc::c_void as *mut uint16_t;
                        let fresh21 = prealloc_idx;
                        prealloc_idx += 1;
                        preallocs[fresh21 as usize] = &mut (*frame).i_propagate_cost
                            as *mut *mut uint16_t
                            as *mut *mut uint8_t;
                        prealloc_size += ((i_mb_count as libc::c_ulong)
                            .wrapping_mul(::core::mem::size_of::<uint16_t>() as libc::c_ulong)
                            as int64_t
                            + (64 as libc::c_int - 1 as libc::c_int) as int64_t)
                            & !(64 as libc::c_int - 1 as libc::c_int) as int64_t;
                        let mut j_1: libc::c_int = 0 as libc::c_int;
                        while j_1 <= (*h).param.i_bframe + 1 as libc::c_int {
                            let mut i_3: libc::c_int = 0 as libc::c_int;
                            while i_3 <= (*h).param.i_bframe + 1 as libc::c_int {
                                (*frame).lowres_costs[j_1 as usize][i_3 as usize] =
                                    prealloc_size as *mut libc::c_void as *mut uint16_t;
                                let fresh22 = prealloc_idx;
                                prealloc_idx += 1;
                                preallocs[fresh22 as usize] = &mut *(*((*frame).lowres_costs)
                                    .as_mut_ptr()
                                    .offset(j_1 as isize))
                                .as_mut_ptr()
                                .offset(i_3 as isize)
                                    as *mut *mut uint16_t
                                    as *mut *mut uint8_t;
                                prealloc_size += ((i_mb_count as libc::c_ulong).wrapping_mul(
                                    ::core::mem::size_of::<uint16_t>() as libc::c_ulong,
                                ) as int64_t
                                    + (64 as libc::c_int - 1 as libc::c_int) as int64_t)
                                    & !(64 as libc::c_int - 1 as libc::c_int) as int64_t;
                                i_3 += 1;
                                i_3;
                            }
                            j_1 += 1;
                            j_1;
                        }
                    }
                    if (*h).param.rc.i_aq_mode != 0 {
                        (*frame).f_qp_offset =
                            prealloc_size as *mut libc::c_void as *mut libc::c_float;
                        let fresh23 = prealloc_idx;
                        prealloc_idx += 1;
                        preallocs[fresh23 as usize] = &mut (*frame).f_qp_offset
                            as *mut *mut libc::c_float
                            as *mut *mut uint8_t;
                        prealloc_size += ((i_mb_count as libc::c_ulong)
                            .wrapping_mul(::core::mem::size_of::<libc::c_float>() as libc::c_ulong)
                            as int64_t
                            + (64 as libc::c_int - 1 as libc::c_int) as int64_t)
                            & !(64 as libc::c_int - 1 as libc::c_int) as int64_t;
                        (*frame).f_qp_offset_aq =
                            prealloc_size as *mut libc::c_void as *mut libc::c_float;
                        let fresh24 = prealloc_idx;
                        prealloc_idx += 1;
                        preallocs[fresh24 as usize] = &mut (*frame).f_qp_offset_aq
                            as *mut *mut libc::c_float
                            as *mut *mut uint8_t;
                        prealloc_size += ((i_mb_count as libc::c_ulong)
                            .wrapping_mul(::core::mem::size_of::<libc::c_float>() as libc::c_ulong)
                            as int64_t
                            + (64 as libc::c_int - 1 as libc::c_int) as int64_t)
                            & !(64 as libc::c_int - 1 as libc::c_int) as int64_t;
                        if (*h).frames.b_have_lowres != 0 {
                            (*frame).i_inv_qscale_factor =
                                prealloc_size as *mut libc::c_void as *mut uint16_t;
                            let fresh25 = prealloc_idx;
                            prealloc_idx += 1;
                            preallocs[fresh25 as usize] = &mut (*frame).i_inv_qscale_factor
                                as *mut *mut uint16_t
                                as *mut *mut uint8_t;
                            prealloc_size += ((i_mb_count as libc::c_ulong)
                                .wrapping_mul(::core::mem::size_of::<uint16_t>() as libc::c_ulong)
                                as int64_t
                                + (64 as libc::c_int - 1 as libc::c_int) as int64_t)
                                & !(64 as libc::c_int - 1 as libc::c_int) as int64_t;
                        }
                    }
                    if (*h).frames.b_have_lowres != 0 {
                        prealloc_size += 64 as libc::c_int as int64_t;
                    }
                }
                (*frame).base = x264_malloc(prealloc_size) as *mut uint8_t;
                if !((*frame).base).is_null() {
                    loop {
                        let fresh26 = prealloc_idx;
                        prealloc_idx -= 1;
                        if fresh26 == 0 {
                            break;
                        }
                        *preallocs[prealloc_idx as usize] = (*preallocs[prealloc_idx as usize]
                            as intptr_t
                            + (*frame).base as intptr_t)
                            as *mut uint8_t;
                    }
                    if i_csp == 0x4 as libc::c_int || i_csp == 0x8 as libc::c_int {
                        let mut chroma_padv_0: libc::c_int =
                            i_padv >> (i_csp == 0x4 as libc::c_int) as libc::c_int;
                        (*frame).plane[1 as libc::c_int as usize] = ((*frame).buffer
                            [1 as libc::c_int as usize])
                            .offset(
                                ((*frame).i_stride[1 as libc::c_int as usize] * chroma_padv_0)
                                    as isize,
                            )
                            .offset(
                                (if 32 as libc::c_int
                                    > 64 as libc::c_int
                                        / ::core::mem::size_of::<pixel>() as libc::c_ulong
                                            as libc::c_int
                                {
                                    32 as libc::c_int
                                } else {
                                    64 as libc::c_int
                                        / ::core::mem::size_of::<pixel>() as libc::c_ulong
                                            as libc::c_int
                                }) as isize,
                            );
                        if (*h).param.b_interlaced != 0 {
                            (*frame).plane_fld[1 as libc::c_int as usize] = ((*frame).buffer_fld
                                [1 as libc::c_int as usize])
                                .offset(
                                    ((*frame).i_stride[1 as libc::c_int as usize] * chroma_padv_0)
                                        as isize,
                                )
                                .offset(
                                    (if 32 as libc::c_int
                                        > 64 as libc::c_int
                                            / ::core::mem::size_of::<pixel>() as libc::c_ulong
                                                as libc::c_int
                                    {
                                        32 as libc::c_int
                                    } else {
                                        64 as libc::c_int
                                            / ::core::mem::size_of::<pixel>() as libc::c_ulong
                                                as libc::c_int
                                    }) as isize,
                                );
                        }
                    }
                    let mut p_0: libc::c_int = 0 as libc::c_int;
                    while p_0 < luma_plane_count {
                        let mut luma_plane_size_1: int64_t = align_plane_size(
                            (*frame).i_stride[p_0 as usize]
                                * ((*frame).i_lines[p_0 as usize] + 2 as libc::c_int * i_padv),
                            disalign,
                        ) as int64_t;
                        if (*h).param.analyse.i_subpel_refine != 0 && b_fdec != 0 {
                            let mut i_4: libc::c_int = 0 as libc::c_int;
                            while i_4 < 4 as libc::c_int {
                                (*frame).filtered[p_0 as usize][i_4 as usize] = ((*frame).buffer
                                    [p_0 as usize])
                                    .offset((i_4 as int64_t * luma_plane_size_1) as isize)
                                    .offset(((*frame).i_stride[p_0 as usize] * i_padv) as isize)
                                    .offset(
                                        (if 32 as libc::c_int
                                            > 64 as libc::c_int
                                                / ::core::mem::size_of::<pixel>() as libc::c_ulong
                                                    as libc::c_int
                                        {
                                            32 as libc::c_int
                                        } else {
                                            64 as libc::c_int
                                                / ::core::mem::size_of::<pixel>() as libc::c_ulong
                                                    as libc::c_int
                                        }) as isize,
                                    );
                                if (*h).param.b_interlaced != 0 {
                                    (*frame).filtered_fld[p_0 as usize][i_4 as usize] = ((*frame)
                                        .buffer_fld[p_0 as usize])
                                        .offset((i_4 as int64_t * luma_plane_size_1) as isize)
                                        .offset(((*frame).i_stride[p_0 as usize] * i_padv) as isize)
                                        .offset(
                                            (if 32 as libc::c_int
                                                > 64 as libc::c_int
                                                    / ::core::mem::size_of::<pixel>()
                                                        as libc::c_ulong
                                                        as libc::c_int
                                            {
                                                32 as libc::c_int
                                            } else {
                                                64 as libc::c_int
                                                    / ::core::mem::size_of::<pixel>()
                                                        as libc::c_ulong
                                                        as libc::c_int
                                            }) as isize,
                                        );
                                }
                                i_4 += 1;
                                i_4;
                            }
                            (*frame).plane[p_0 as usize] =
                                (*frame).filtered[p_0 as usize][0 as libc::c_int as usize];
                            (*frame).plane_fld[p_0 as usize] =
                                (*frame).filtered_fld[p_0 as usize][0 as libc::c_int as usize];
                        } else {
                            (*frame).plane[p_0 as usize] = ((*frame).buffer[p_0 as usize])
                                .offset(((*frame).i_stride[p_0 as usize] * i_padv) as isize)
                                .offset(
                                    (if 32 as libc::c_int
                                        > 64 as libc::c_int
                                            / ::core::mem::size_of::<pixel>() as libc::c_ulong
                                                as libc::c_int
                                    {
                                        32 as libc::c_int
                                    } else {
                                        64 as libc::c_int
                                            / ::core::mem::size_of::<pixel>() as libc::c_ulong
                                                as libc::c_int
                                    }) as isize,
                                );
                            (*frame).filtered[p_0 as usize][0 as libc::c_int as usize] =
                                (*frame).plane[p_0 as usize];
                            if (*h).param.b_interlaced != 0 {
                                (*frame).plane_fld[p_0 as usize] = ((*frame).buffer_fld
                                    [p_0 as usize])
                                    .offset(((*frame).i_stride[p_0 as usize] * i_padv) as isize)
                                    .offset(
                                        (if 32 as libc::c_int
                                            > 64 as libc::c_int
                                                / ::core::mem::size_of::<pixel>() as libc::c_ulong
                                                    as libc::c_int
                                        {
                                            32 as libc::c_int
                                        } else {
                                            64 as libc::c_int
                                                / ::core::mem::size_of::<pixel>() as libc::c_ulong
                                                    as libc::c_int
                                        }) as isize,
                                    );
                                (*frame).filtered_fld[p_0 as usize][0 as libc::c_int as usize] =
                                    (*frame).plane_fld[p_0 as usize];
                            }
                        }
                        p_0 += 1;
                        p_0;
                    }
                    if b_fdec != 0 {
                        (*((*((*frame).mv16x16).offset(0 as libc::c_int as isize)).as_mut_ptr()
                            as *mut x264_union32_t))
                            .i = 0 as libc::c_int as uint32_t;
                        (*frame).mv16x16 = ((*frame).mv16x16).offset(1);
                        (*frame).mv16x16;
                        if (*h).param.analyse.i_me_method >= 3 as libc::c_int {
                            (*frame).integral = ((*frame).buffer[3 as libc::c_int as usize]
                                as *mut uint16_t)
                                .offset(
                                    ((*frame).i_stride[0 as libc::c_int as usize] * i_padv)
                                        as isize,
                                )
                                .offset(
                                    (if 32 as libc::c_int
                                        > 64 as libc::c_int
                                            / ::core::mem::size_of::<pixel>() as libc::c_ulong
                                                as libc::c_int
                                    {
                                        32 as libc::c_int
                                    } else {
                                        64 as libc::c_int
                                            / ::core::mem::size_of::<pixel>() as libc::c_ulong
                                                as libc::c_int
                                    }) as isize,
                                );
                        }
                    } else if (*h).frames.b_have_lowres != 0 {
                        let mut luma_plane_size_2: int64_t = align_plane_size(
                            (*frame).i_stride_lowres
                                * ((*frame).i_lines[0 as libc::c_int as usize] / 2 as libc::c_int
                                    + 2 as libc::c_int * 32 as libc::c_int),
                            disalign,
                        ) as int64_t;
                        let mut i_5: libc::c_int = 0 as libc::c_int;
                        while i_5 < 4 as libc::c_int {
                            (*frame).lowres[i_5 as usize] = ((*frame).buffer_lowres)
                                .offset(((*frame).i_stride_lowres * 32 as libc::c_int) as isize)
                                .offset(
                                    (if 32 as libc::c_int
                                        > 64 as libc::c_int
                                            / ::core::mem::size_of::<pixel>() as libc::c_ulong
                                                as libc::c_int
                                    {
                                        32 as libc::c_int
                                    } else {
                                        64 as libc::c_int
                                            / ::core::mem::size_of::<pixel>() as libc::c_ulong
                                                as libc::c_int
                                    }) as isize,
                                )
                                .offset((i_5 as int64_t * luma_plane_size_2) as isize);
                            i_5 += 1;
                            i_5;
                        }
                        let mut j_2: libc::c_int = 0 as libc::c_int;
                        while j_2 <= ((*h).param.i_bframe != 0) as libc::c_int {
                            let mut i_6: libc::c_int = 0 as libc::c_int;
                            while i_6 <= (*h).param.i_bframe {
                                memset(
                                    (*frame).lowres_mvs[j_2 as usize][i_6 as usize]
                                        as *mut libc::c_void,
                                    0 as libc::c_int,
                                    ((2 as libc::c_int * i_mb_count) as libc::c_ulong)
                                        .wrapping_mul(
                                            ::core::mem::size_of::<int16_t>() as libc::c_ulong
                                        ),
                                );
                                i_6 += 1;
                                i_6;
                            }
                            j_2 += 1;
                            j_2;
                        }
                        (*frame).i_intra_cost = (*frame).lowres_costs[0 as libc::c_int as usize]
                            [0 as libc::c_int as usize];
                        memset(
                            (*frame).i_intra_cost as *mut libc::c_void,
                            -(1 as libc::c_int),
                            (i_mb_count as libc::c_ulong).wrapping_mul(::core::mem::size_of::<
                                uint16_t,
                            >(
                            )
                                as libc::c_ulong),
                        );
                        if (*h).param.rc.i_aq_mode != 0 {
                            memset(
                                (*frame).i_inv_qscale_factor as *mut libc::c_void,
                                0 as libc::c_int,
                                (i_mb_count as libc::c_ulong).wrapping_mul(::core::mem::size_of::<
                                    uint16_t,
                                >(
                                )
                                    as libc::c_ulong),
                            );
                        }
                    }
                    if pthread_mutex_init(
                        &mut (*frame).mutex,
                        std::ptr::null::<pthread_mutexattr_t>(),
                    ) == 0
                        && pthread_cond_init(
                            &mut (*frame).cv,
                            std::ptr::null::<pthread_condattr_t>(),
                        ) == 0
                    {
                        (*frame).opencl.ocl = (*h).opencl.ocl;
                        return frame;
                    }
                }
            }
        }
    }
    x264_free(frame as *mut libc::c_void);
    std::ptr::null_mut::<x264_frame_t>()
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_frame_delete(mut frame: *mut x264_frame_t) {
    if (*frame).b_duplicate == 0 {
        x264_free((*frame).base as *mut libc::c_void);
        if !((*frame).param).is_null() && ((*(*frame).param).param_free).is_some() {
            x264_param_cleanup((*frame).param);
            ((*(*frame).param).param_free).expect("non-null function pointer")(
                (*frame).param as *mut libc::c_void,
            );
        }
        if ((*frame).mb_info_free).is_some() {
            ((*frame).mb_info_free).expect("non-null function pointer")(
                (*frame).mb_info as *mut libc::c_void,
            );
        }
        if ((*frame).extra_sei.sei_free).is_some() {
            let mut i: libc::c_int = 0 as libc::c_int;
            while i < (*frame).extra_sei.num_payloads {
                ((*frame).extra_sei.sei_free).expect("non-null function pointer")(
                    (*((*frame).extra_sei.payloads).offset(i as isize)).payload
                        as *mut libc::c_void,
                );
                i += 1;
                i;
            }
            ((*frame).extra_sei.sei_free).expect("non-null function pointer")(
                (*frame).extra_sei.payloads as *mut libc::c_void,
            );
        }
        pthread_mutex_destroy(&mut (*frame).mutex);
        pthread_cond_destroy(&mut (*frame).cv);
        x264_8_opencl_frame_delete(frame);
    }
    x264_free(frame as *mut libc::c_void);
}
unsafe extern "C" fn get_plane_ptr(
    mut h: *mut x264_t,
    mut src: *mut x264_picture_t,
    mut pix: *mut *mut uint8_t,
    mut stride: *mut libc::c_int,
    mut plane: libc::c_int,
    mut xshift: libc::c_int,
    mut yshift: libc::c_int,
) -> libc::c_int {
    let mut width: libc::c_int = (*h).param.i_width >> xshift;
    let mut height: libc::c_int = (*h).param.i_height >> yshift;
    *pix = (*src).img.plane[plane as usize];
    *stride = (*src).img.i_stride[plane as usize];
    if (*src).img.i_csp & 0x1000 as libc::c_int != 0 {
        *pix = (*pix).offset(((height - 1 as libc::c_int) * *stride) as isize);
        *stride = -*stride;
    }
    if width > abs(*stride) {
        x264_8_log(
            h,
            0 as libc::c_int,
            b"Input picture width (%d) is greater than stride (%d)\n\0" as *const u8
                as *const libc::c_char,
            width,
            *stride,
        );
        return -(1 as libc::c_int);
    }
    0 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_frame_copy_picture(
    mut h: *mut x264_t,
    mut dst: *mut x264_frame_t,
    mut src: *mut x264_picture_t,
) -> libc::c_int {
    let mut i_csp: libc::c_int = (*src).img.i_csp & 0xff as libc::c_int;
    if (*dst).i_csp != frame_internal_csp(i_csp) {
        x264_8_log(
            h,
            0 as libc::c_int,
            b"Invalid input colorspace\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (*src).img.i_csp & 0x2000 as libc::c_int != 0 {
        x264_8_log(
            h,
            0 as libc::c_int,
            b"This build of x264 requires 8-bit input. Rebuild to support high depth input.\n\0"
                as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if 8 as libc::c_int != 10 as libc::c_int && i_csp == 0xb as libc::c_int {
        x264_8_log(
            h,
            0 as libc::c_int,
            b"v210 input is only compatible with bit-depth of 10 bits\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (*src).i_type < 0 as libc::c_int || (*src).i_type > 0x6 as libc::c_int {
        x264_8_log(
            h,
            1 as libc::c_int,
            b"forced frame type (%d) at %d is unknown\n\0" as *const u8 as *const libc::c_char,
            (*src).i_type,
            (*h).frames.i_input,
        );
        (*dst).i_forced_type = 0 as libc::c_int;
    } else {
        (*dst).i_forced_type = (*src).i_type;
    }
    (*dst).i_type = (*dst).i_forced_type;
    (*dst).i_qpplus1 = (*src).i_qpplus1;
    (*dst).i_reordered_pts = (*src).i_pts;
    (*dst).i_pts = (*dst).i_reordered_pts;
    (*dst).param = (*src).param;
    (*dst).i_pic_struct = (*src).i_pic_struct;
    (*dst).extra_sei = (*src).extra_sei;
    (*dst).opaque = (*src).opaque;
    (*dst).mb_info = if (*h).param.analyse.b_mb_info != 0 {
        (*src).prop.mb_info
    } else {
        std::ptr::null_mut::<uint8_t>()
    };
    (*dst).mb_info_free = if (*h).param.analyse.b_mb_info != 0 {
        (*src).prop.mb_info_free
    } else {
        None
    };
    let mut pix: [*mut uint8_t; 3] = [std::ptr::null_mut::<uint8_t>(); 3];
    let mut stride: [libc::c_int; 3] = [0; 3];
    if i_csp == 0x9 as libc::c_int || i_csp == 0xa as libc::c_int {
        let mut p: libc::c_int = (i_csp == 0xa as libc::c_int) as libc::c_int;
        ((*h).mc.plane_copy_deinterleave_yuyv).expect("non-null function pointer")(
            (*dst).plane[p as usize],
            (*dst).i_stride[p as usize] as intptr_t,
            (*dst).plane[(p ^ 1 as libc::c_int) as usize],
            (*dst).i_stride[(p ^ 1 as libc::c_int) as usize] as intptr_t,
            (*src).img.plane[0 as libc::c_int as usize] as *mut pixel,
            ((*src).img.i_stride[0 as libc::c_int as usize]
                / ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int)
                as intptr_t,
            (*h).param.i_width,
            (*h).param.i_height,
        );
    } else if i_csp == 0xb as libc::c_int {
        stride[0 as libc::c_int as usize] = (*src).img.i_stride[0 as libc::c_int as usize];
        pix[0 as libc::c_int as usize] = (*src).img.plane[0 as libc::c_int as usize];
        ((*h).mc.plane_copy_deinterleave_v210).expect("non-null function pointer")(
            (*dst).plane[0 as libc::c_int as usize],
            (*dst).i_stride[0 as libc::c_int as usize] as intptr_t,
            (*dst).plane[1 as libc::c_int as usize],
            (*dst).i_stride[1 as libc::c_int as usize] as intptr_t,
            pix[0 as libc::c_int as usize] as *mut uint32_t,
            (stride[0 as libc::c_int as usize]
                / ::core::mem::size_of::<uint32_t>() as libc::c_ulong as libc::c_int)
                as intptr_t,
            (*h).param.i_width,
            (*h).param.i_height,
        );
    } else if i_csp >= 0xe as libc::c_int {
        stride[0 as libc::c_int as usize] = (*src).img.i_stride[0 as libc::c_int as usize];
        pix[0 as libc::c_int as usize] = (*src).img.plane[0 as libc::c_int as usize];
        if (*src).img.i_csp & 0x1000 as libc::c_int != 0 {
            pix[0 as libc::c_int as usize] = (pix[0 as libc::c_int as usize]).offset(
                (((*h).param.i_height - 1 as libc::c_int) * stride[0 as libc::c_int as usize])
                    as isize,
            );
            stride[0 as libc::c_int as usize] = -stride[0 as libc::c_int as usize];
        }
        let mut b: libc::c_int = (i_csp == 0x10 as libc::c_int) as libc::c_int;
        ((*h).mc.plane_copy_deinterleave_rgb).expect("non-null function pointer")(
            (*dst).plane[(1 as libc::c_int + b) as usize],
            (*dst).i_stride[(1 as libc::c_int + b) as usize] as intptr_t,
            (*dst).plane[0 as libc::c_int as usize],
            (*dst).i_stride[0 as libc::c_int as usize] as intptr_t,
            (*dst).plane[(2 as libc::c_int - b) as usize],
            (*dst).i_stride[(2 as libc::c_int - b) as usize] as intptr_t,
            pix[0 as libc::c_int as usize] as *mut pixel,
            (stride[0 as libc::c_int as usize]
                / ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int)
                as intptr_t,
            if i_csp == 0xf as libc::c_int {
                4 as libc::c_int
            } else {
                3 as libc::c_int
            },
            (*h).param.i_width,
            (*h).param.i_height,
        );
    } else {
        let mut v_shift: libc::c_int = (*h).mb.chroma_v_shift;
        if get_plane_ptr(
            h,
            src,
            &mut *pix.as_mut_ptr().offset(0 as libc::c_int as isize),
            &mut *stride.as_mut_ptr().offset(0 as libc::c_int as isize),
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
        ) < 0 as libc::c_int
        {
            return -(1 as libc::c_int);
        }
        ((*h).mc.plane_copy).expect("non-null function pointer")(
            (*dst).plane[0 as libc::c_int as usize],
            (*dst).i_stride[0 as libc::c_int as usize] as intptr_t,
            pix[0 as libc::c_int as usize] as *mut pixel,
            (stride[0 as libc::c_int as usize]
                / ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int)
                as intptr_t,
            (*h).param.i_width,
            (*h).param.i_height,
        );
        if i_csp == 0x4 as libc::c_int || i_csp == 0x8 as libc::c_int {
            if get_plane_ptr(
                h,
                src,
                &mut *pix.as_mut_ptr().offset(1 as libc::c_int as isize),
                &mut *stride.as_mut_ptr().offset(1 as libc::c_int as isize),
                1 as libc::c_int,
                0 as libc::c_int,
                v_shift,
            ) < 0 as libc::c_int
            {
                return -(1 as libc::c_int);
            }
            ((*h).mc.plane_copy).expect("non-null function pointer")(
                (*dst).plane[1 as libc::c_int as usize],
                (*dst).i_stride[1 as libc::c_int as usize] as intptr_t,
                pix[1 as libc::c_int as usize] as *mut pixel,
                (stride[1 as libc::c_int as usize]
                    / ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int)
                    as intptr_t,
                (*h).param.i_width,
                (*h).param.i_height >> v_shift,
            );
        } else if i_csp == 0x5 as libc::c_int {
            if get_plane_ptr(
                h,
                src,
                &mut *pix.as_mut_ptr().offset(1 as libc::c_int as isize),
                &mut *stride.as_mut_ptr().offset(1 as libc::c_int as isize),
                1 as libc::c_int,
                0 as libc::c_int,
                v_shift,
            ) < 0 as libc::c_int
            {
                return -(1 as libc::c_int);
            }
            ((*h).mc.plane_copy_swap).expect("non-null function pointer")(
                (*dst).plane[1 as libc::c_int as usize],
                (*dst).i_stride[1 as libc::c_int as usize] as intptr_t,
                pix[1 as libc::c_int as usize] as *mut pixel,
                (stride[1 as libc::c_int as usize]
                    / ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int)
                    as intptr_t,
                (*h).param.i_width >> 1 as libc::c_int,
                (*h).param.i_height >> v_shift,
            );
        } else if i_csp == 0x2 as libc::c_int
            || i_csp == 0x6 as libc::c_int
            || i_csp == 0x3 as libc::c_int
            || i_csp == 0x7 as libc::c_int
        {
            let mut uv_swap: libc::c_int =
                (i_csp == 0x3 as libc::c_int || i_csp == 0x7 as libc::c_int) as libc::c_int;
            if get_plane_ptr(
                h,
                src,
                &mut *pix.as_mut_ptr().offset(1 as libc::c_int as isize),
                &mut *stride.as_mut_ptr().offset(1 as libc::c_int as isize),
                if uv_swap != 0 {
                    2 as libc::c_int
                } else {
                    1 as libc::c_int
                },
                1 as libc::c_int,
                v_shift,
            ) < 0 as libc::c_int
            {
                return -(1 as libc::c_int);
            }
            if get_plane_ptr(
                h,
                src,
                &mut *pix.as_mut_ptr().offset(2 as libc::c_int as isize),
                &mut *stride.as_mut_ptr().offset(2 as libc::c_int as isize),
                if uv_swap != 0 {
                    1 as libc::c_int
                } else {
                    2 as libc::c_int
                },
                1 as libc::c_int,
                v_shift,
            ) < 0 as libc::c_int
            {
                return -(1 as libc::c_int);
            }
            ((*h).mc.plane_copy_interleave).expect("non-null function pointer")(
                (*dst).plane[1 as libc::c_int as usize],
                (*dst).i_stride[1 as libc::c_int as usize] as intptr_t,
                pix[1 as libc::c_int as usize] as *mut pixel,
                (stride[1 as libc::c_int as usize]
                    / ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int)
                    as intptr_t,
                pix[2 as libc::c_int as usize] as *mut pixel,
                (stride[2 as libc::c_int as usize]
                    / ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int)
                    as intptr_t,
                (*h).param.i_width >> 1 as libc::c_int,
                (*h).param.i_height >> v_shift,
            );
        } else if i_csp == 0xc as libc::c_int || i_csp == 0xd as libc::c_int {
            if get_plane_ptr(
                h,
                src,
                &mut *pix.as_mut_ptr().offset(1 as libc::c_int as isize),
                &mut *stride.as_mut_ptr().offset(1 as libc::c_int as isize),
                if i_csp == 0xc as libc::c_int {
                    1 as libc::c_int
                } else {
                    2 as libc::c_int
                },
                0 as libc::c_int,
                0 as libc::c_int,
            ) < 0 as libc::c_int
            {
                return -(1 as libc::c_int);
            }
            if get_plane_ptr(
                h,
                src,
                &mut *pix.as_mut_ptr().offset(2 as libc::c_int as isize),
                &mut *stride.as_mut_ptr().offset(2 as libc::c_int as isize),
                if i_csp == 0xc as libc::c_int {
                    2 as libc::c_int
                } else {
                    1 as libc::c_int
                },
                0 as libc::c_int,
                0 as libc::c_int,
            ) < 0 as libc::c_int
            {
                return -(1 as libc::c_int);
            }
            ((*h).mc.plane_copy).expect("non-null function pointer")(
                (*dst).plane[1 as libc::c_int as usize],
                (*dst).i_stride[1 as libc::c_int as usize] as intptr_t,
                pix[1 as libc::c_int as usize] as *mut pixel,
                (stride[1 as libc::c_int as usize]
                    / ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int)
                    as intptr_t,
                (*h).param.i_width,
                (*h).param.i_height,
            );
            ((*h).mc.plane_copy).expect("non-null function pointer")(
                (*dst).plane[2 as libc::c_int as usize],
                (*dst).i_stride[2 as libc::c_int as usize] as intptr_t,
                pix[2 as libc::c_int as usize] as *mut pixel,
                (stride[2 as libc::c_int as usize]
                    / ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int)
                    as intptr_t,
                (*h).param.i_width,
                (*h).param.i_height,
            );
        }
    }
    0 as libc::c_int
}
#[inline(always)]
unsafe extern "C" fn pixel_memset(
    mut dst: *mut pixel,
    mut src: *mut pixel,
    mut len: libc::c_int,
    mut size: libc::c_int,
) {
    let mut dstp: *mut uint8_t = dst as *mut uint8_t;
    let mut v1: uint32_t = *src as uint32_t;
    let mut v2: uint32_t = if size == 1 as libc::c_int {
        v1.wrapping_add(v1 << 8 as libc::c_int)
    } else {
        (*(src as *mut x264_union16_t)).i as uint32_t
    };
    let mut v4: uint32_t = if size <= 2 as libc::c_int {
        v2.wrapping_add(v2 << 16 as libc::c_int)
    } else {
        (*(src as *mut x264_union32_t)).i
    };
    let mut i: libc::c_int = 0 as libc::c_int;
    len *= size;
    if dstp as intptr_t as libc::c_ulong
        & (::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        != 0
    {
        if size <= 2 as libc::c_int && dstp as intptr_t & 3 as libc::c_int as intptr_t != 0 {
            if size == 1 as libc::c_int && dstp as intptr_t & 1 as libc::c_int as intptr_t != 0 {
                let fresh27 = i;
                i += 1;
                *dstp.offset(fresh27 as isize) = v1 as uint8_t;
            }
            if dstp as intptr_t & 2 as libc::c_int as intptr_t != 0 {
                (*(dstp.offset(i as isize) as *mut x264_union16_t)).i = v2 as uint16_t;
                i += 2 as libc::c_int;
            }
        }
        if ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
            == 8 as libc::c_int as libc::c_ulong
            && dstp as intptr_t & 4 as libc::c_int as intptr_t != 0
        {
            (*(dstp.offset(i as isize) as *mut x264_union32_t)).i = v4;
            i += 4 as libc::c_int;
        }
    }
    if ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
        == 8 as libc::c_int as libc::c_ulong
    {
        let mut v8: uint64_t = (v4 as uint64_t).wrapping_add((v4 as uint64_t) << 32 as libc::c_int);
        while i < len - 7 as libc::c_int {
            (*(dstp.offset(i as isize) as *mut x264_union64_t)).i = v8;
            i += 8 as libc::c_int;
        }
    }
    while i < len - 3 as libc::c_int {
        (*(dstp.offset(i as isize) as *mut x264_union32_t)).i = v4;
        i += 4 as libc::c_int;
    }
    if size <= 2 as libc::c_int {
        if i < len - 1 as libc::c_int {
            (*(dstp.offset(i as isize) as *mut x264_union16_t)).i = v2 as uint16_t;
            i += 2 as libc::c_int;
        }
        if size == 1 as libc::c_int && i != len {
            *dstp.offset(i as isize) = v1 as uint8_t;
        }
    }
}
#[inline(always)]
unsafe extern "C" fn plane_expand_border(
    mut pix: *mut pixel,
    mut i_stride: libc::c_int,
    mut i_width: libc::c_int,
    mut i_height: libc::c_int,
    mut i_padh: libc::c_int,
    mut i_padv: libc::c_int,
    mut b_pad_top: libc::c_int,
    mut b_pad_bottom: libc::c_int,
    mut b_chroma: libc::c_int,
) {
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < i_height {
        pixel_memset(
            pix.offset(-i_padh as isize).offset((y * i_stride) as isize),
            pix.offset(0 as libc::c_int as isize)
                .offset((y * i_stride) as isize),
            i_padh >> b_chroma,
            (::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int) << b_chroma,
        );
        pixel_memset(
            pix.offset(i_width as isize).offset((y * i_stride) as isize),
            pix.offset((i_width - 1 as libc::c_int - b_chroma) as isize)
                .offset((y * i_stride) as isize),
            i_padh >> b_chroma,
            (::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int) << b_chroma,
        );
        y += 1;
        y;
    }
    if b_pad_top != 0 {
        let mut y_0: libc::c_int = 0 as libc::c_int;
        while y_0 < i_padv {
            memcpy(
                pix.offset(-i_padh as isize)
                    .offset(((-y_0 - 1 as libc::c_int) * i_stride) as isize)
                    as *mut libc::c_void,
                pix.offset(-i_padh as isize)
                    .offset((0 as libc::c_int * i_stride) as isize)
                    as *const libc::c_void,
                ((i_width + 2 as libc::c_int * i_padh)
                    * ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int)
                    as libc::c_ulong,
            );
            y_0 += 1;
            y_0;
        }
    }
    if b_pad_bottom != 0 {
        let mut y_1: libc::c_int = 0 as libc::c_int;
        while y_1 < i_padv {
            memcpy(
                pix.offset(-i_padh as isize)
                    .offset(((i_height + y_1) * i_stride) as isize)
                    as *mut libc::c_void,
                pix.offset(-i_padh as isize)
                    .offset(((i_height - 1 as libc::c_int) * i_stride) as isize)
                    as *const libc::c_void,
                ((i_width + 2 as libc::c_int * i_padh)
                    * ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int)
                    as libc::c_ulong,
            );
            y_1 += 1;
            y_1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_frame_expand_border(
    mut h: *mut x264_t,
    mut frame: *mut x264_frame_t,
    mut mb_y: libc::c_int,
) {
    let mut pad_top: libc::c_int = (mb_y == 0 as libc::c_int) as libc::c_int;
    let mut pad_bot: libc::c_int =
        (mb_y == (*h).mb.i_mb_height - ((1 as libc::c_int) << (*h).sh.b_mbaff)) as libc::c_int;
    let mut b_start: libc::c_int = (mb_y == (*h).i_threadslice_start) as libc::c_int;
    let mut b_end: libc::c_int =
        (mb_y == (*h).i_threadslice_end - ((1 as libc::c_int) << (*h).sh.b_mbaff)) as libc::c_int;
    if mb_y & (*h).sh.b_mbaff != 0 {
        return;
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*frame).i_plane {
        let mut h_shift: libc::c_int = (i != 0 && (*h).mb.chroma_h_shift != 0) as libc::c_int;
        let mut v_shift: libc::c_int = (i != 0 && (*h).mb.chroma_v_shift != 0) as libc::c_int;
        let mut stride: libc::c_int = (*frame).i_stride[i as usize];
        let mut width: libc::c_int = 16 as libc::c_int * (*h).mb.i_mb_width;
        let mut height: libc::c_int = (if pad_bot != 0 {
            (16 as libc::c_int * ((*h).mb.i_mb_height - mb_y)) >> (*h).sh.b_mbaff
        } else {
            16 as libc::c_int
        }) >> v_shift;
        let mut padh: libc::c_int = 32 as libc::c_int;
        let mut padv: libc::c_int = 32 as libc::c_int >> v_shift;
        if b_end != 0 && b_start == 0 {
            height += 4 as libc::c_int >> (v_shift + (*h).sh.b_mbaff);
        }
        let mut pix: *mut pixel = std::ptr::null_mut::<pixel>();
        let mut starty: libc::c_int =
            16 as libc::c_int * mb_y - 4 as libc::c_int * (b_start == 0) as libc::c_int;
        if (*h).sh.b_mbaff != 0 {
            pix = ((*frame).plane_fld[i as usize]).offset(((starty * stride) >> v_shift) as isize);
            plane_expand_border(
                pix,
                stride * 2 as libc::c_int,
                width,
                height,
                padh,
                padv,
                pad_top,
                pad_bot,
                h_shift,
            );
            plane_expand_border(
                pix.offset(stride as isize),
                stride * 2 as libc::c_int,
                width,
                height,
                padh,
                padv,
                pad_top,
                pad_bot,
                h_shift,
            );
            height = (if pad_bot != 0 {
                16 as libc::c_int * ((*h).mb.i_mb_height - mb_y)
            } else {
                32 as libc::c_int
            }) >> v_shift;
            if b_end != 0 && b_start == 0 {
                height += 4 as libc::c_int >> v_shift;
            }
            pix = ((*frame).plane[i as usize]).offset(((starty * stride) >> v_shift) as isize);
            plane_expand_border(
                pix, stride, width, height, padh, padv, pad_top, pad_bot, h_shift,
            );
        } else {
            pix = ((*frame).plane[i as usize]).offset(((starty * stride) >> v_shift) as isize);
            plane_expand_border(
                pix, stride, width, height, padh, padv, pad_top, pad_bot, h_shift,
            );
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_frame_expand_border_filtered(
    mut h: *mut x264_t,
    mut frame: *mut x264_frame_t,
    mut mb_y: libc::c_int,
    mut b_end: libc::c_int,
) {
    let mut b_start: libc::c_int = (mb_y == 0) as libc::c_int;
    let mut width: libc::c_int = 16 as libc::c_int * (*h).mb.i_mb_width + 8 as libc::c_int;
    let mut height: libc::c_int = if b_end != 0 {
        ((16 as libc::c_int * ((*h).mb.i_mb_height - mb_y)) >> (*h).sh.b_mbaff) + 16 as libc::c_int
    } else {
        16 as libc::c_int
    };
    let mut padh: libc::c_int = 32 as libc::c_int - 4 as libc::c_int;
    let mut padv: libc::c_int = 32 as libc::c_int - 8 as libc::c_int;
    let mut p: libc::c_int = 0 as libc::c_int;
    while p
        < (if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as libc::c_int {
            3 as libc::c_int
        } else {
            1 as libc::c_int
        })
    {
        let mut i: libc::c_int = 1 as libc::c_int;
        while i < 4 as libc::c_int {
            let mut stride: libc::c_int = (*frame).i_stride[p as usize];
            let mut pix: *mut pixel = std::ptr::null_mut::<pixel>();
            if (*h).sh.b_mbaff != 0 {
                pix = ((*frame).filtered_fld[p as usize][i as usize])
                    .offset(((16 as libc::c_int * mb_y - 16 as libc::c_int) * stride) as isize)
                    .offset(-(4 as libc::c_int as isize));
                plane_expand_border(
                    pix,
                    stride * 2 as libc::c_int,
                    width,
                    height,
                    padh,
                    padv,
                    b_start,
                    b_end,
                    0 as libc::c_int,
                );
                plane_expand_border(
                    pix.offset(stride as isize),
                    stride * 2 as libc::c_int,
                    width,
                    height,
                    padh,
                    padv,
                    b_start,
                    b_end,
                    0 as libc::c_int,
                );
            }
            pix = ((*frame).filtered[p as usize][i as usize])
                .offset(((16 as libc::c_int * mb_y - 8 as libc::c_int) * stride) as isize)
                .offset(-(4 as libc::c_int as isize));
            plane_expand_border(
                pix,
                stride,
                width,
                height << (*h).sh.b_mbaff,
                padh,
                padv,
                b_start,
                b_end,
                0 as libc::c_int,
            );
            i += 1;
            i;
        }
        p += 1;
        p;
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_frame_expand_border_lowres(mut frame: *mut x264_frame_t) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        plane_expand_border(
            (*frame).lowres[i as usize],
            (*frame).i_stride_lowres,
            (*frame).i_width_lowres,
            (*frame).i_lines_lowres,
            32 as libc::c_int,
            32 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            0 as libc::c_int,
        );
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_frame_expand_border_chroma(
    mut h: *mut x264_t,
    mut frame: *mut x264_frame_t,
    mut plane: libc::c_int,
) {
    let mut v_shift: libc::c_int = (*h).mb.chroma_v_shift;
    plane_expand_border(
        (*frame).plane[plane as usize],
        (*frame).i_stride[plane as usize],
        16 as libc::c_int * (*h).mb.i_mb_width,
        (16 as libc::c_int * (*h).mb.i_mb_height) >> v_shift,
        32 as libc::c_int,
        32 as libc::c_int >> v_shift,
        1 as libc::c_int,
        1 as libc::c_int,
        (*h).mb.chroma_h_shift,
    );
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_frame_expand_border_mod16(
    mut h: *mut x264_t,
    mut frame: *mut x264_frame_t,
) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*frame).i_plane {
        let mut i_width: libc::c_int = (*h).param.i_width;
        let mut h_shift: libc::c_int = (i != 0 && (*h).mb.chroma_h_shift != 0) as libc::c_int;
        let mut v_shift: libc::c_int = (i != 0 && (*h).mb.chroma_v_shift != 0) as libc::c_int;
        let mut i_height: libc::c_int = (*h).param.i_height >> v_shift;
        let mut i_padx: libc::c_int = (*h).mb.i_mb_width * 16 as libc::c_int - (*h).param.i_width;
        let mut i_pady: libc::c_int =
            ((*h).mb.i_mb_height * 16 as libc::c_int - (*h).param.i_height) >> v_shift;
        if i_padx != 0 {
            let mut y: libc::c_int = 0 as libc::c_int;
            while y < i_height {
                pixel_memset(
                    &mut *(*((*frame).plane).as_mut_ptr().offset(i as isize)).offset(
                        (y * *((*frame).i_stride).as_mut_ptr().offset(i as isize) + i_width)
                            as isize,
                    ),
                    &mut *(*((*frame).plane).as_mut_ptr().offset(i as isize)).offset(
                        (y * *((*frame).i_stride).as_mut_ptr().offset(i as isize) + i_width
                            - 1 as libc::c_int
                            - h_shift) as isize,
                    ),
                    i_padx >> h_shift,
                    (::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int) << h_shift,
                );
                y += 1;
                y;
            }
        }
        if i_pady != 0 {
            let mut y_0: libc::c_int = i_height;
            while y_0 < i_height + i_pady {
                memcpy(
                    &mut *(*((*frame).plane).as_mut_ptr().offset(i as isize)).offset(
                        (y_0 * *((*frame).i_stride).as_mut_ptr().offset(i as isize)) as isize,
                    ) as *mut pixel as *mut libc::c_void,
                    &mut *(*((*frame).plane).as_mut_ptr().offset(i as isize)).offset(
                        ((i_height - (!y_0 & (*h).param.b_interlaced) - 1 as libc::c_int)
                            * *((*frame).i_stride).as_mut_ptr().offset(i as isize))
                            as isize,
                    ) as *mut pixel as *const libc::c_void,
                    ((i_width + i_padx)
                        * ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int)
                        as libc::c_ulong,
                );
                y_0 += 1;
                y_0;
            }
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_expand_border_mbpair(
    mut h: *mut x264_t,
    mut mb_x: libc::c_int,
    mut mb_y: libc::c_int,
) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*(*h).fenc).i_plane {
        let mut v_shift: libc::c_int = (i != 0 && (*h).mb.chroma_v_shift != 0) as libc::c_int;
        let mut stride: libc::c_int = (*(*h).fenc).i_stride[i as usize];
        let mut height: libc::c_int = (*h).param.i_height >> v_shift;
        let mut pady: libc::c_int =
            ((*h).mb.i_mb_height * 16 as libc::c_int - (*h).param.i_height) >> v_shift;
        let mut fenc: *mut pixel =
            ((*(*h).fenc).plane[i as usize]).offset((16 as libc::c_int * mb_x) as isize);
        let mut y: libc::c_int = height;
        while y < height + pady {
            memcpy(
                fenc.offset((y * stride) as isize) as *mut libc::c_void,
                fenc.offset(((height - 1 as libc::c_int) * stride) as isize) as *const libc::c_void,
                (16 as libc::c_int
                    * ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int)
                    as libc::c_ulong,
            );
            y += 1;
            y;
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_frame_cond_broadcast(
    mut frame: *mut x264_frame_t,
    mut i_lines_completed: libc::c_int,
) {
    pthread_mutex_lock(&mut (*frame).mutex);
    (*frame).i_lines_completed = i_lines_completed;
    pthread_cond_broadcast(&mut (*frame).cv);
    pthread_mutex_unlock(&mut (*frame).mutex);
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_frame_cond_wait(
    mut frame: *mut x264_frame_t,
    mut i_lines_completed: libc::c_int,
) -> libc::c_int {
    let mut completed: libc::c_int = 0;
    pthread_mutex_lock(&mut (*frame).mutex);
    loop {
        completed = (*frame).i_lines_completed;
        if !(completed < i_lines_completed && i_lines_completed >= 0 as libc::c_int) {
            break;
        }
        pthread_cond_wait(&mut (*frame).cv, &mut (*frame).mutex);
    }
    pthread_mutex_unlock(&mut (*frame).mutex);
    completed
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_threadslice_cond_broadcast(
    mut h: *mut x264_t,
    mut pass: libc::c_int,
) {
    pthread_mutex_lock(&mut (*h).mutex);
    (*h).i_threadslice_pass = pass;
    if pass > 0 as libc::c_int {
        pthread_cond_broadcast(&mut (*h).cv);
    }
    pthread_mutex_unlock(&mut (*h).mutex);
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_threadslice_cond_wait(mut h: *mut x264_t, mut pass: libc::c_int) {
    pthread_mutex_lock(&mut (*h).mutex);
    while (*h).i_threadslice_pass < pass {
        pthread_cond_wait(&mut (*h).cv, &mut (*h).mutex);
    }
    pthread_mutex_unlock(&mut (*h).mutex);
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_frame_new_slice(
    mut h: *mut x264_t,
    mut frame: *mut x264_frame_t,
) -> libc::c_int {
    if (*h).param.i_slice_count_max != 0 {
        let mut slice_count: libc::c_int = 0;
        if (*h).param.b_sliced_threads != 0 {
            slice_count = x264_pthread_fetch_and_add(
                &mut (*frame).i_slice_count,
                1 as libc::c_int,
                &mut (*frame).mutex,
            );
        } else {
            let fresh28 = (*frame).i_slice_count;
            (*frame).i_slice_count += 1;
            slice_count = fresh28;
        }
        if slice_count >= (*h).param.i_slice_count_max {
            return -(1 as libc::c_int);
        }
    }
    0 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_frame_push(
    mut list: *mut *mut x264_frame_t,
    mut frame: *mut x264_frame_t,
) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while !(*list.offset(i as isize)).is_null() {
        i += 1;
        i;
    }
    let fresh29 = &mut (*list.offset(i as isize));
    *fresh29 = frame;
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_frame_pop(mut list: *mut *mut x264_frame_t) -> *mut x264_frame_t {
    let mut frame: *mut x264_frame_t = std::ptr::null_mut::<x264_frame_t>();
    let mut i: libc::c_int = 0 as libc::c_int;
    if !(*list.offset(0 as libc::c_int as isize)).is_null() {
    } else {
        __assert_fail(
            b"list[0]\0" as *const u8 as *const libc::c_char,
            b"common/frame.c\0" as *const u8 as *const libc::c_char,
            746 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 48], &[libc::c_char; 48]>(
                b"x264_frame_t *x264_8_frame_pop(x264_frame_t **)\0",
            ))
            .as_ptr(),
        );
    }
    'c_27229: {
        if !(*list.offset(0 as libc::c_int as isize)).is_null() {
        } else {
            __assert_fail(
                b"list[0]\0" as *const u8 as *const libc::c_char,
                b"common/frame.c\0" as *const u8 as *const libc::c_char,
                746 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 48], &[libc::c_char; 48]>(
                    b"x264_frame_t *x264_8_frame_pop(x264_frame_t **)\0",
                ))
                .as_ptr(),
            );
        }
    };
    while !(*list.offset((i + 1 as libc::c_int) as isize)).is_null() {
        i += 1;
        i;
    }
    frame = *list.offset(i as isize);
    let fresh30 = &mut (*list.offset(i as isize));
    *fresh30 = std::ptr::null_mut::<x264_frame_t>();
    frame
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_frame_unshift(
    mut list: *mut *mut x264_frame_t,
    mut frame: *mut x264_frame_t,
) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while !(*list.offset(i as isize)).is_null() {
        i += 1;
        i;
    }
    loop {
        let fresh31 = i;
        i -= 1;
        if fresh31 == 0 {
            break;
        }
        let fresh32 = &mut (*list.offset((i + 1 as libc::c_int) as isize));
        *fresh32 = *list.offset(i as isize);
    }
    let fresh33 = &mut (*list.offset(0 as libc::c_int as isize));
    *fresh33 = frame;
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_frame_shift(mut list: *mut *mut x264_frame_t) -> *mut x264_frame_t {
    let mut frame: *mut x264_frame_t = *list.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while !(*list.offset(i as isize)).is_null() {
        let fresh34 = &mut (*list.offset(i as isize));
        *fresh34 = *list.offset((i + 1 as libc::c_int) as isize);
        i += 1;
        i;
    }
    if !frame.is_null() {
    } else {
        __assert_fail(
            b"frame\0" as *const u8 as *const libc::c_char,
            b"common/frame.c\0" as *const u8 as *const libc::c_char,
            768 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                b"x264_frame_t *x264_8_frame_shift(x264_frame_t **)\0",
            ))
            .as_ptr(),
        );
    }
    'c_27330: {
        if !frame.is_null() {
        } else {
            __assert_fail(
                b"frame\0" as *const u8 as *const libc::c_char,
                b"common/frame.c\0" as *const u8 as *const libc::c_char,
                768 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                    b"x264_frame_t *x264_8_frame_shift(x264_frame_t **)\0",
                ))
                .as_ptr(),
            );
        }
    };
    frame
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_frame_push_unused(
    mut h: *mut x264_t,
    mut frame: *mut x264_frame_t,
) {
    if (*frame).i_reference_count > 0 as libc::c_int {
    } else {
        __assert_fail(
            b"frame->i_reference_count > 0\0" as *const u8 as *const libc::c_char,
            b"common/frame.c\0" as *const u8 as *const libc::c_char,
            774 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 56], &[libc::c_char; 56]>(
                b"void x264_8_frame_push_unused(x264_t *, x264_frame_t *)\0",
            ))
            .as_ptr(),
        );
    }
    'c_27427: {
        if (*frame).i_reference_count > 0 as libc::c_int {
        } else {
            __assert_fail(
                b"frame->i_reference_count > 0\0" as *const u8 as *const libc::c_char,
                b"common/frame.c\0" as *const u8 as *const libc::c_char,
                774 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 56], &[libc::c_char; 56]>(
                    b"void x264_8_frame_push_unused(x264_t *, x264_frame_t *)\0",
                ))
                .as_ptr(),
            );
        }
    };
    (*frame).i_reference_count -= 1;
    (*frame).i_reference_count;
    if (*frame).i_reference_count == 0 as libc::c_int {
        x264_8_frame_push((*h).frames.unused[(*frame).b_fdec as usize], frame);
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_frame_pop_unused(
    mut h: *mut x264_t,
    mut b_fdec: libc::c_int,
) -> *mut x264_frame_t {
    let mut frame: *mut x264_frame_t = std::ptr::null_mut::<x264_frame_t>();
    if !(*((*h).frames.unused[b_fdec as usize]).offset(0 as libc::c_int as isize)).is_null() {
        frame = x264_8_frame_pop((*h).frames.unused[b_fdec as usize]);
    } else {
        frame = frame_new(h, b_fdec);
    }
    if frame.is_null() {
        return std::ptr::null_mut::<x264_frame_t>();
    }
    (*frame).b_last_minigop_bframe = 0 as libc::c_int as uint8_t;
    (*frame).i_reference_count = 1 as libc::c_int;
    (*frame).b_intra_calculated = 0 as libc::c_int;
    (*frame).b_scenecut = 1 as libc::c_int;
    (*frame).b_keyframe = 0 as libc::c_int;
    (*frame).b_corrupt = 0 as libc::c_int;
    (*frame).i_slice_count = if (*h).param.b_sliced_threads != 0 {
        (*h).param.i_threads
    } else {
        1 as libc::c_int
    };
    memset(
        ((*frame).weight).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[[x264_weight_t; 3]; 16]>() as libc::c_ulong,
    );
    memset(
        ((*frame).f_weighted_cost_delta).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[libc::c_float; 18]>() as libc::c_ulong,
    );
    frame
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_frame_push_blank_unused(
    mut h: *mut x264_t,
    mut frame: *mut x264_frame_t,
) {
    if (*frame).i_reference_count > 0 as libc::c_int {
    } else {
        __assert_fail(
            b"frame->i_reference_count > 0\0" as *const u8 as *const libc::c_char,
            b"common/frame.c\0" as *const u8 as *const libc::c_char,
            805 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 62], &[libc::c_char; 62]>(
                b"void x264_8_frame_push_blank_unused(x264_t *, x264_frame_t *)\0",
            ))
            .as_ptr(),
        );
    }
    'c_27497: {
        if (*frame).i_reference_count > 0 as libc::c_int {
        } else {
            __assert_fail(
                b"frame->i_reference_count > 0\0" as *const u8 as *const libc::c_char,
                b"common/frame.c\0" as *const u8 as *const libc::c_char,
                805 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 62], &[libc::c_char; 62]>(
                    b"void x264_8_frame_push_blank_unused(x264_t *, x264_frame_t *)\0",
                ))
                .as_ptr(),
            );
        }
    };
    (*frame).i_reference_count -= 1;
    (*frame).i_reference_count;
    if (*frame).i_reference_count == 0 as libc::c_int {
        x264_8_frame_push((*h).frames.blank_unused, frame);
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_frame_pop_blank_unused(mut h: *mut x264_t) -> *mut x264_frame_t {
    let mut frame: *mut x264_frame_t = std::ptr::null_mut::<x264_frame_t>();
    if !(*((*h).frames.blank_unused).offset(0 as libc::c_int as isize)).is_null() {
        frame = x264_8_frame_pop((*h).frames.blank_unused);
    } else {
        frame = x264_malloc(::core::mem::size_of::<x264_frame_t>() as libc::c_ulong as int64_t)
            as *mut x264_frame_t;
    }
    if frame.is_null() {
        return std::ptr::null_mut::<x264_frame_t>();
    }
    (*frame).b_duplicate = 1 as libc::c_int;
    (*frame).i_reference_count = 1 as libc::c_int;
    frame
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_weight_scale_plane(
    mut h: *mut x264_t,
    mut dst: *mut pixel,
    mut i_dst_stride: intptr_t,
    mut src: *mut pixel,
    mut i_src_stride: intptr_t,
    mut i_width: libc::c_int,
    mut i_height: libc::c_int,
    mut w: *mut x264_weight_t,
) {
    while i_height > 0 as libc::c_int {
        let mut x: libc::c_int = 0;
        x = 0 as libc::c_int;
        while x < i_width - 8 as libc::c_int {
            (*((*w).weightfn).offset((16 as libc::c_int >> 2 as libc::c_int) as isize))
                .expect("non-null function pointer")(
                dst.offset(x as isize),
                i_dst_stride,
                src.offset(x as isize),
                i_src_stride,
                w,
                if i_height < 16 as libc::c_int {
                    i_height
                } else {
                    16 as libc::c_int
                },
            );
            x += 16 as libc::c_int;
        }
        if x < i_width {
            (*((*w).weightfn).offset((8 as libc::c_int >> 2 as libc::c_int) as isize))
                .expect("non-null function pointer")(
                dst.offset(x as isize),
                i_dst_stride,
                src.offset(x as isize),
                i_src_stride,
                w,
                if i_height < 16 as libc::c_int {
                    i_height
                } else {
                    16 as libc::c_int
                },
            );
        }
        i_height -= 16 as libc::c_int;
        dst = dst.offset((16 as libc::c_int as intptr_t * i_dst_stride) as isize);
        src = src.offset((16 as libc::c_int as intptr_t * i_src_stride) as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_frame_delete_list(mut list: *mut *mut x264_frame_t) {
    let mut i: libc::c_int = 0 as libc::c_int;
    if list.is_null() {
        return;
    }
    while !(*list.offset(i as isize)).is_null() {
        let fresh35 = i;
        i += 1;
        x264_8_frame_delete(*list.offset(fresh35 as isize));
    }
    x264_free(list as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_sync_frame_list_init(
    mut slist: *mut x264_sync_frame_list_t,
    mut max_size: libc::c_int,
) -> libc::c_int {
    if max_size < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    (*slist).i_max_size = max_size;
    (*slist).i_size = 0 as libc::c_int;
    (*slist).list = x264_malloc(
        ((max_size + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut x264_frame_t>() as libc::c_ulong)
            as int64_t,
    ) as *mut *mut x264_frame_t;
    if ((*slist).list).is_null() {
        -(1 as libc::c_int)
    } else {
        memset(
            (*slist).list as *mut libc::c_void,
            0 as libc::c_int,
            ((max_size + 1 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<*mut x264_frame_t>() as libc::c_ulong),
        );
        if pthread_mutex_init(&mut (*slist).mutex, std::ptr::null::<pthread_mutexattr_t>()) != 0
            || pthread_cond_init(
                &mut (*slist).cv_fill,
                std::ptr::null::<pthread_condattr_t>(),
            ) != 0
            || pthread_cond_init(
                &mut (*slist).cv_empty,
                std::ptr::null::<pthread_condattr_t>(),
            ) != 0
        {
            return -(1 as libc::c_int);
        }
        0 as libc::c_int
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_sync_frame_list_delete(mut slist: *mut x264_sync_frame_list_t) {
    pthread_mutex_destroy(&mut (*slist).mutex);
    pthread_cond_destroy(&mut (*slist).cv_fill);
    pthread_cond_destroy(&mut (*slist).cv_empty);
    x264_8_frame_delete_list((*slist).list);
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_sync_frame_list_push(
    mut slist: *mut x264_sync_frame_list_t,
    mut frame: *mut x264_frame_t,
) {
    pthread_mutex_lock(&mut (*slist).mutex);
    while (*slist).i_size == (*slist).i_max_size {
        pthread_cond_wait(&mut (*slist).cv_empty, &mut (*slist).mutex);
    }
    let fresh36 = (*slist).i_size;
    (*slist).i_size += 1;
    let fresh37 = &mut (*((*slist).list).offset(fresh36 as isize));
    *fresh37 = frame;
    pthread_mutex_unlock(&mut (*slist).mutex);
    pthread_cond_broadcast(&mut (*slist).cv_fill);
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_sync_frame_list_pop(
    mut slist: *mut x264_sync_frame_list_t,
) -> *mut x264_frame_t {
    let mut frame: *mut x264_frame_t = std::ptr::null_mut::<x264_frame_t>();
    pthread_mutex_lock(&mut (*slist).mutex);
    while (*slist).i_size == 0 {
        pthread_cond_wait(&mut (*slist).cv_fill, &mut (*slist).mutex);
    }
    (*slist).i_size -= 1;
    frame = *((*slist).list).offset((*slist).i_size as isize);
    let fresh38 = &mut (*((*slist).list).offset((*slist).i_size as isize));
    *fresh38 = std::ptr::null_mut::<x264_frame_t>();
    pthread_cond_broadcast(&mut (*slist).cv_empty);
    pthread_mutex_unlock(&mut (*slist).mutex);
    frame
}
