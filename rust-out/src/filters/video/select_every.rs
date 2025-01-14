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
extern "C" {
    pub type x264_t;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn strtok_r(
        __s: *mut libc::c_char,
        __delim: *const libc::c_char,
        __save_ptr: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn x264_reduce_fraction(n: *mut uint32_t, d: *mut uint32_t);
    fn x264_cli_log(
        name: *const libc::c_char,
        i_level: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    fn x264_otoi(str: *const libc::c_char, def: libc::c_int) -> libc::c_int;
    fn x264_init_vid_filter(
        name: *const libc::c_char,
        handle: *mut hnd_t,
        filter: *mut cli_vid_filter_t,
        info: *mut video_info_t,
        param: *mut x264_param_t,
        opt_string: *mut libc::c_char,
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type intptr_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_nal_t {
    pub i_ref_idc: libc::c_int,
    pub i_type: libc::c_int,
    pub b_long_startcode: libc::c_int,
    pub i_first_mb: libc::c_int,
    pub i_last_mb: libc::c_int,
    pub i_payload: libc::c_int,
    pub p_payload: *mut uint8_t,
    pub i_padding: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_zone_t {
    pub i_start: libc::c_int,
    pub i_end: libc::c_int,
    pub b_force_qp: libc::c_int,
    pub i_qp: libc::c_int,
    pub f_bitrate_factor: libc::c_float,
    pub param: *mut x264_param_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_param_t {
    pub cpu: uint32_t,
    pub i_threads: libc::c_int,
    pub i_lookahead_threads: libc::c_int,
    pub b_sliced_threads: libc::c_int,
    pub b_deterministic: libc::c_int,
    pub b_cpu_independent: libc::c_int,
    pub i_sync_lookahead: libc::c_int,
    pub i_width: libc::c_int,
    pub i_height: libc::c_int,
    pub i_csp: libc::c_int,
    pub i_bitdepth: libc::c_int,
    pub i_level_idc: libc::c_int,
    pub i_frame_total: libc::c_int,
    pub i_nal_hrd: libc::c_int,
    pub vui: C2RustUnnamed_4,
    pub i_frame_reference: libc::c_int,
    pub i_dpb_size: libc::c_int,
    pub i_keyint_max: libc::c_int,
    pub i_keyint_min: libc::c_int,
    pub i_scenecut_threshold: libc::c_int,
    pub b_intra_refresh: libc::c_int,
    pub i_bframe: libc::c_int,
    pub i_bframe_adaptive: libc::c_int,
    pub i_bframe_bias: libc::c_int,
    pub i_bframe_pyramid: libc::c_int,
    pub b_open_gop: libc::c_int,
    pub b_bluray_compat: libc::c_int,
    pub i_avcintra_class: libc::c_int,
    pub i_avcintra_flavor: libc::c_int,
    pub b_deblocking_filter: libc::c_int,
    pub i_deblocking_filter_alphac0: libc::c_int,
    pub i_deblocking_filter_beta: libc::c_int,
    pub b_cabac: libc::c_int,
    pub i_cabac_init_idc: libc::c_int,
    pub b_interlaced: libc::c_int,
    pub b_constrained_intra: libc::c_int,
    pub i_cqm_preset: libc::c_int,
    pub psz_cqm_file: *mut libc::c_char,
    pub cqm_4iy: [uint8_t; 16],
    pub cqm_4py: [uint8_t; 16],
    pub cqm_4ic: [uint8_t; 16],
    pub cqm_4pc: [uint8_t; 16],
    pub cqm_8iy: [uint8_t; 64],
    pub cqm_8py: [uint8_t; 64],
    pub cqm_8ic: [uint8_t; 64],
    pub cqm_8pc: [uint8_t; 64],
    pub pf_log: Option<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_int,
            *const libc::c_char,
            ::core::ffi::VaList,
        ) -> (),
    >,
    pub p_log_private: *mut libc::c_void,
    pub i_log_level: libc::c_int,
    pub b_full_recon: libc::c_int,
    pub psz_dump_yuv: *mut libc::c_char,
    pub analyse: C2RustUnnamed_3,
    pub rc: C2RustUnnamed_2,
    pub crop_rect: C2RustUnnamed_1,
    pub i_frame_packing: libc::c_int,
    pub mastering_display: C2RustUnnamed_0,
    pub content_light_level: C2RustUnnamed,
    pub i_alternative_transfer: libc::c_int,
    pub b_aud: libc::c_int,
    pub b_repeat_headers: libc::c_int,
    pub b_annexb: libc::c_int,
    pub i_sps_id: libc::c_int,
    pub b_vfr_input: libc::c_int,
    pub b_pulldown: libc::c_int,
    pub i_fps_num: uint32_t,
    pub i_fps_den: uint32_t,
    pub i_timebase_num: uint32_t,
    pub i_timebase_den: uint32_t,
    pub b_tff: libc::c_int,
    pub b_pic_struct: libc::c_int,
    pub b_fake_interlaced: libc::c_int,
    pub b_stitchable: libc::c_int,
    pub b_opencl: libc::c_int,
    pub i_opencl_device: libc::c_int,
    pub opencl_device_id: *mut libc::c_void,
    pub psz_clbin_file: *mut libc::c_char,
    pub i_slice_max_size: libc::c_int,
    pub i_slice_max_mbs: libc::c_int,
    pub i_slice_min_mbs: libc::c_int,
    pub i_slice_count: libc::c_int,
    pub i_slice_count_max: libc::c_int,
    pub param_free: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub nalu_process:
        Option<unsafe extern "C" fn(*mut x264_t, *mut x264_nal_t, *mut libc::c_void) -> ()>,
    pub opaque: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub b_cll: libc::c_int,
    pub i_max_cll: libc::c_int,
    pub i_max_fall: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub b_mastering_display: libc::c_int,
    pub i_green_x: libc::c_int,
    pub i_green_y: libc::c_int,
    pub i_blue_x: libc::c_int,
    pub i_blue_y: libc::c_int,
    pub i_red_x: libc::c_int,
    pub i_red_y: libc::c_int,
    pub i_white_x: libc::c_int,
    pub i_white_y: libc::c_int,
    pub i_display_max: int64_t,
    pub i_display_min: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub i_left: libc::c_int,
    pub i_top: libc::c_int,
    pub i_right: libc::c_int,
    pub i_bottom: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub i_rc_method: libc::c_int,
    pub i_qp_constant: libc::c_int,
    pub i_qp_min: libc::c_int,
    pub i_qp_max: libc::c_int,
    pub i_qp_step: libc::c_int,
    pub i_bitrate: libc::c_int,
    pub f_rf_constant: libc::c_float,
    pub f_rf_constant_max: libc::c_float,
    pub f_rate_tolerance: libc::c_float,
    pub i_vbv_max_bitrate: libc::c_int,
    pub i_vbv_buffer_size: libc::c_int,
    pub f_vbv_buffer_init: libc::c_float,
    pub f_ip_factor: libc::c_float,
    pub f_pb_factor: libc::c_float,
    pub b_filler: libc::c_int,
    pub i_aq_mode: libc::c_int,
    pub f_aq_strength: libc::c_float,
    pub b_mb_tree: libc::c_int,
    pub i_lookahead: libc::c_int,
    pub b_stat_write: libc::c_int,
    pub psz_stat_out: *mut libc::c_char,
    pub b_stat_read: libc::c_int,
    pub psz_stat_in: *mut libc::c_char,
    pub f_qcompress: libc::c_float,
    pub f_qblur: libc::c_float,
    pub f_complexity_blur: libc::c_float,
    pub zones: *mut x264_zone_t,
    pub i_zones: libc::c_int,
    pub psz_zones: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub intra: libc::c_uint,
    pub inter: libc::c_uint,
    pub b_transform_8x8: libc::c_int,
    pub i_weighted_pred: libc::c_int,
    pub b_weighted_bipred: libc::c_int,
    pub i_direct_mv_pred: libc::c_int,
    pub i_chroma_qp_offset: libc::c_int,
    pub i_me_method: libc::c_int,
    pub i_me_range: libc::c_int,
    pub i_mv_range: libc::c_int,
    pub i_mv_range_thread: libc::c_int,
    pub i_subpel_refine: libc::c_int,
    pub b_chroma_me: libc::c_int,
    pub b_mixed_references: libc::c_int,
    pub i_trellis: libc::c_int,
    pub b_fast_pskip: libc::c_int,
    pub b_dct_decimate: libc::c_int,
    pub i_noise_reduction: libc::c_int,
    pub f_psy_rd: libc::c_float,
    pub f_psy_trellis: libc::c_float,
    pub b_psy: libc::c_int,
    pub b_mb_info: libc::c_int,
    pub b_mb_info_update: libc::c_int,
    pub i_luma_deadzone: [libc::c_int; 2],
    pub b_psnr: libc::c_int,
    pub b_ssim: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub i_sar_height: libc::c_int,
    pub i_sar_width: libc::c_int,
    pub i_overscan: libc::c_int,
    pub i_vidformat: libc::c_int,
    pub b_fullrange: libc::c_int,
    pub i_colorprim: libc::c_int,
    pub i_transfer: libc::c_int,
    pub i_colmatrix: libc::c_int,
    pub i_chroma_loc: libc::c_int,
}
pub type hnd_t = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct video_info_t {
    pub csp: libc::c_int,
    pub fps_num: uint32_t,
    pub fps_den: uint32_t,
    pub fullrange: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub interlaced: libc::c_int,
    pub num_frames: libc::c_int,
    pub sar_width: uint32_t,
    pub sar_height: uint32_t,
    pub tff: libc::c_int,
    pub thread_safe: libc::c_int,
    pub timebase_num: uint32_t,
    pub timebase_den: uint32_t,
    pub vfr: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cli_image_t {
    pub csp: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub planes: libc::c_int,
    pub plane: [*mut uint8_t; 4],
    pub stride: [libc::c_int; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cli_pic_t {
    pub img: cli_image_t,
    pub pts: int64_t,
    pub duration: int64_t,
    pub opaque: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cli_vid_filter_t {
    pub name: *const libc::c_char,
    pub help: Option<unsafe extern "C" fn(libc::c_int) -> ()>,
    pub init: Option<
        unsafe extern "C" fn(
            *mut hnd_t,
            *mut cli_vid_filter_t,
            *mut video_info_t,
            *mut x264_param_t,
            *mut libc::c_char,
        ) -> libc::c_int,
    >,
    pub get_frame: Option<unsafe extern "C" fn(hnd_t, *mut cli_pic_t, libc::c_int) -> libc::c_int>,
    pub release_frame:
        Option<unsafe extern "C" fn(hnd_t, *mut cli_pic_t, libc::c_int) -> libc::c_int>,
    pub free: Option<unsafe extern "C" fn(hnd_t) -> ()>,
    pub next: *mut cli_vid_filter_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct selvry_hnd_t {
    pub prev_hnd: hnd_t,
    pub prev_filter: cli_vid_filter_t,
    pub pattern: *mut libc::c_int,
    pub pattern_len: libc::c_int,
    pub step_size: libc::c_int,
    pub vfr: libc::c_int,
    pub pts: int64_t,
}
unsafe extern "C" fn help(mut longhelp: libc::c_int) {
    printf(b"      select_every:step,offset1[,...]\n\0" as *const u8 as *const libc::c_char);
    if longhelp == 0 {
        return;
    }
    printf(
        b"            apply a selection pattern to input frames\n            step: the number of frames in the pattern\n            offsets: the offset into the step to select a frame\n            see: http://avisynth.nl/index.php/Select#SelectEvery\n\0"
            as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn init(
    mut handle: *mut hnd_t,
    mut filter: *mut cli_vid_filter_t,
    mut info: *mut video_info_t,
    mut param: *mut x264_param_t,
    mut opt_string: *mut libc::c_char,
) -> libc::c_int {
    let mut h: *mut selvry_hnd_t =
        malloc(::core::mem::size_of::<selvry_hnd_t>() as libc::c_ulong) as *mut selvry_hnd_t;
    if h.is_null() {
        return -(1 as libc::c_int);
    }
    (*h).pattern_len = 0 as libc::c_int;
    (*h).step_size = 0 as libc::c_int;
    let mut offsets: [libc::c_int; 100] = [0; 100];
    let mut tok: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    let mut p: *mut libc::c_char = opt_string;
    let mut saveptr: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    loop {
        tok = strtok_r(p, b",\0" as *const u8 as *const libc::c_char, &mut saveptr);
        if tok.is_null() {
            break;
        }
        let mut val: libc::c_int = x264_otoi(tok, -(1 as libc::c_int));
        if !p.is_null() {
            if val <= 0 as libc::c_int {
                x264_cli_log(
                    b"select_every\0" as *const u8 as *const libc::c_char,
                    0 as libc::c_int,
                    b"invalid step `%s'\n\0" as *const u8 as *const libc::c_char,
                    tok,
                );
                return -(1 as libc::c_int);
            }
            (*h).step_size = val;
        } else {
            if val < 0 as libc::c_int || val >= (*h).step_size {
                x264_cli_log(
                    b"select_every\0" as *const u8 as *const libc::c_char,
                    0 as libc::c_int,
                    b"invalid offset `%s'\n\0" as *const u8 as *const libc::c_char,
                    tok,
                );
                return -(1 as libc::c_int);
            }
            if (*h).pattern_len >= 100 as libc::c_int {
                x264_cli_log(
                    b"select_every\0" as *const u8 as *const libc::c_char,
                    0 as libc::c_int,
                    b"max pattern size %d reached\n\0" as *const u8 as *const libc::c_char,
                    100 as libc::c_int,
                );
                return -(1 as libc::c_int);
            }
            let fresh0 = (*h).pattern_len;
            (*h).pattern_len += 1;
            offsets[fresh0 as usize] = val;
        }
        p = std::ptr::null_mut::<libc::c_char>();
    }
    if (*h).step_size == 0 {
        x264_cli_log(
            b"select_every\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            b"no step size provided\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (*h).pattern_len == 0 {
        x264_cli_log(
            b"select_every\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            b"no offsets supplied\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    (*h).pattern = malloc(
        ((*h).pattern_len as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    if ((*h).pattern).is_null() {
        return -(1 as libc::c_int);
    }
    memcpy(
        (*h).pattern as *mut libc::c_void,
        offsets.as_mut_ptr() as *const libc::c_void,
        ((*h).pattern_len as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    let mut max_rewind: intptr_t = 0 as libc::c_int as intptr_t;
    let mut min: libc::c_int = (*h).step_size;
    let mut i: libc::c_int = (*h).pattern_len - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        min = if min < offsets[i as usize] {
            min
        } else {
            offsets[i as usize]
        };
        if i != 0 {
            max_rewind = if max_rewind
                > (offsets[(i - 1 as libc::c_int) as usize] - min + 1 as libc::c_int) as intptr_t
            {
                max_rewind
            } else {
                (offsets[(i - 1 as libc::c_int) as usize] - min + 1 as libc::c_int) as intptr_t
            };
        }
        if max_rewind == (*h).step_size as intptr_t {
            break;
        }
        i -= 1;
        i;
    }
    let mut name: [libc::c_char; 20] = [0; 20];
    sprintf(
        name.as_mut_ptr(),
        b"cache_%d\0" as *const u8 as *const libc::c_char,
        (*param).i_bitdepth,
    );
    if x264_init_vid_filter(
        name.as_mut_ptr(),
        handle,
        filter,
        info,
        param,
        max_rewind as *mut libc::c_void as *mut libc::c_char,
    ) != 0
    {
        return -(1 as libc::c_int);
    }
    if (*h).step_size != (*h).pattern_len {
        (*info).num_frames = ((*info).num_frames as uint64_t * (*h).pattern_len as uint64_t
            / (*h).step_size as uint64_t) as libc::c_int;
        (*info).fps_den *= (*h).step_size as uint32_t;
        (*info).fps_num *= (*h).pattern_len as uint32_t;
        x264_reduce_fraction(&mut (*info).fps_num, &mut (*info).fps_den);
        if (*info).vfr != 0 {
            (*info).timebase_den *= (*h).pattern_len as uint32_t;
            (*info).timebase_num *= (*h).step_size as uint32_t;
            x264_reduce_fraction(&mut (*info).timebase_num, &mut (*info).timebase_den);
        }
    }
    (*h).pts = 0 as libc::c_int as int64_t;
    (*h).vfr = (*info).vfr;
    (*h).prev_filter = *filter;
    (*h).prev_hnd = *handle;
    *filter = select_every_filter;
    *handle = h as hnd_t;
    0 as libc::c_int
}
unsafe extern "C" fn get_frame(
    mut handle: hnd_t,
    mut output: *mut cli_pic_t,
    mut frame: libc::c_int,
) -> libc::c_int {
    let mut h: *mut selvry_hnd_t = handle as *mut selvry_hnd_t;
    let mut pat_frame: libc::c_int = *((*h).pattern).offset((frame % (*h).pattern_len) as isize)
        + frame / (*h).pattern_len * (*h).step_size;
    if ((*h).prev_filter.get_frame).expect("non-null function pointer")(
        (*h).prev_hnd,
        output,
        pat_frame,
    ) != 0
    {
        return -(1 as libc::c_int);
    }
    if (*h).vfr != 0 {
        (*output).pts = (*h).pts;
        (*h).pts += (*output).duration;
    }
    0 as libc::c_int
}
unsafe extern "C" fn release_frame(
    mut handle: hnd_t,
    mut pic: *mut cli_pic_t,
    mut frame: libc::c_int,
) -> libc::c_int {
    let mut h: *mut selvry_hnd_t = handle as *mut selvry_hnd_t;
    let mut pat_frame: libc::c_int = *((*h).pattern).offset((frame % (*h).pattern_len) as isize)
        + frame / (*h).pattern_len * (*h).step_size;
    ((*h).prev_filter.release_frame).expect("non-null function pointer")(
        (*h).prev_hnd,
        pic,
        pat_frame,
    )
}
unsafe extern "C" fn free_filter(mut handle: hnd_t) {
    let mut h: *mut selvry_hnd_t = handle as *mut selvry_hnd_t;
    ((*h).prev_filter.free).expect("non-null function pointer")((*h).prev_hnd);
    free((*h).pattern as *mut libc::c_void);
    free(h as *mut libc::c_void);
}
#[no_mangle]
pub static mut select_every_filter: cli_vid_filter_t = unsafe {
    {
        cli_vid_filter_t {
            name: b"select_every\0" as *const u8 as *const libc::c_char,
            help: Some(help as unsafe extern "C" fn(libc::c_int) -> ()),
            init: Some(
                init as unsafe extern "C" fn(
                    *mut hnd_t,
                    *mut cli_vid_filter_t,
                    *mut video_info_t,
                    *mut x264_param_t,
                    *mut libc::c_char,
                ) -> libc::c_int,
            ),
            get_frame: Some(
                get_frame
                    as unsafe extern "C" fn(hnd_t, *mut cli_pic_t, libc::c_int) -> libc::c_int,
            ),
            release_frame: Some(
                release_frame
                    as unsafe extern "C" fn(hnd_t, *mut cli_pic_t, libc::c_int) -> libc::c_int,
            ),
            free: Some(free_filter as unsafe extern "C" fn(hnd_t) -> ()),
            next: 0 as *const cli_vid_filter_t as *mut cli_vid_filter_t,
        }
    }
};
