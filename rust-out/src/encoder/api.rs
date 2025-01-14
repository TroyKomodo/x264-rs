#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type x264_t;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn x264_log_internal(i_level: libc::c_int, psz_fmt: *const libc::c_char, _: ...);
    fn x264_8_encoder_open(_: *mut x264_param_t, _: *mut libc::c_void) -> *mut x264_t;
    fn x264_8_nal_encode(h: *mut x264_t, dst: *mut uint8_t, nal: *mut x264_nal_t);
    fn x264_8_encoder_reconfig(_: *mut x264_t, _: *mut x264_param_t) -> libc::c_int;
    fn x264_8_encoder_parameters(_: *mut x264_t, _: *mut x264_param_t);
    fn x264_8_encoder_headers(
        _: *mut x264_t,
        pp_nal: *mut *mut x264_nal_t,
        pi_nal: *mut libc::c_int,
    ) -> libc::c_int;
    fn x264_8_encoder_encode(
        _: *mut x264_t,
        pp_nal: *mut *mut x264_nal_t,
        pi_nal: *mut libc::c_int,
        pic_in: *mut x264_picture_t,
        pic_out: *mut x264_picture_t,
    ) -> libc::c_int;
    fn x264_8_encoder_close(_: *mut x264_t);
    fn x264_8_encoder_delayed_frames(_: *mut x264_t) -> libc::c_int;
    fn x264_8_encoder_maximum_delayed_frames(_: *mut x264_t) -> libc::c_int;
    fn x264_8_encoder_intra_refresh(_: *mut x264_t);
    fn x264_8_encoder_invalidate_reference(_: *mut x264_t, pts: int64_t) -> libc::c_int;
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
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
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
    pub pf_log: Option::<
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
    pub param_free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub nalu_process: Option::<
        unsafe extern "C" fn(*mut x264_t, *mut x264_nal_t, *mut libc::c_void) -> (),
    >,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_api_t {
    pub x264: *mut x264_t,
    pub nal_encode: Option::<
        unsafe extern "C" fn(*mut x264_t, *mut uint8_t, *mut x264_nal_t) -> (),
    >,
    pub encoder_reconfig: Option::<
        unsafe extern "C" fn(*mut x264_t, *mut x264_param_t) -> libc::c_int,
    >,
    pub encoder_parameters: Option::<
        unsafe extern "C" fn(*mut x264_t, *mut x264_param_t) -> (),
    >,
    pub encoder_headers: Option::<
        unsafe extern "C" fn(
            *mut x264_t,
            *mut *mut x264_nal_t,
            *mut libc::c_int,
        ) -> libc::c_int,
    >,
    pub encoder_encode: Option::<
        unsafe extern "C" fn(
            *mut x264_t,
            *mut *mut x264_nal_t,
            *mut libc::c_int,
            *mut x264_picture_t,
            *mut x264_picture_t,
        ) -> libc::c_int,
    >,
    pub encoder_close: Option::<unsafe extern "C" fn(*mut x264_t) -> ()>,
    pub encoder_delayed_frames: Option::<
        unsafe extern "C" fn(*mut x264_t) -> libc::c_int,
    >,
    pub encoder_maximum_delayed_frames: Option::<
        unsafe extern "C" fn(*mut x264_t) -> libc::c_int,
    >,
    pub encoder_intra_refresh: Option::<unsafe extern "C" fn(*mut x264_t) -> ()>,
    pub encoder_invalidate_reference: Option::<
        unsafe extern "C" fn(*mut x264_t, int64_t) -> libc::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_picture_t {
    pub i_type: libc::c_int,
    pub i_qpplus1: libc::c_int,
    pub i_pic_struct: libc::c_int,
    pub b_keyframe: libc::c_int,
    pub i_pts: int64_t,
    pub i_dts: int64_t,
    pub param: *mut x264_param_t,
    pub img: x264_image_t,
    pub prop: x264_image_properties_t,
    pub hrd_timing: x264_hrd_t,
    pub extra_sei: x264_sei_t,
    pub opaque: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_sei_t {
    pub num_payloads: libc::c_int,
    pub payloads: *mut x264_sei_payload_t,
    pub sei_free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_sei_payload_t {
    pub payload_size: libc::c_int,
    pub payload_type: libc::c_int,
    pub payload: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_hrd_t {
    pub cpb_initial_arrival_time: libc::c_double,
    pub cpb_final_arrival_time: libc::c_double,
    pub cpb_removal_time: libc::c_double,
    pub dpb_output_time: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_image_properties_t {
    pub quant_offsets: *mut libc::c_float,
    pub quant_offsets_free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub mb_info: *mut uint8_t,
    pub mb_info_free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub f_ssim: libc::c_double,
    pub f_psnr_avg: libc::c_double,
    pub f_psnr: [libc::c_double; 3],
    pub f_crf_avg: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_image_t {
    pub i_csp: libc::c_int,
    pub i_plane: libc::c_int,
    pub i_stride: [libc::c_int; 4],
    pub plane: [*mut uint8_t; 4],
}
#[no_mangle]
pub static mut x264_chroma_format: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn x264_encoder_open_164(
    mut param: *mut x264_param_t,
) -> *mut x264_t {
    let mut api: *mut x264_api_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<x264_api_t>() as libc::c_ulong,
    ) as *mut x264_api_t;
    if api.is_null() {
        return 0 as *mut x264_t;
    }
    if (*param).i_bitdepth == 8 as libc::c_int {
        (*api)
            .nal_encode = Some(
            x264_8_nal_encode
                as unsafe extern "C" fn(*mut x264_t, *mut uint8_t, *mut x264_nal_t) -> (),
        );
        (*api)
            .encoder_reconfig = Some(
            x264_8_encoder_reconfig
                as unsafe extern "C" fn(*mut x264_t, *mut x264_param_t) -> libc::c_int,
        );
        (*api)
            .encoder_parameters = Some(
            x264_8_encoder_parameters
                as unsafe extern "C" fn(*mut x264_t, *mut x264_param_t) -> (),
        );
        (*api)
            .encoder_headers = Some(
            x264_8_encoder_headers
                as unsafe extern "C" fn(
                    *mut x264_t,
                    *mut *mut x264_nal_t,
                    *mut libc::c_int,
                ) -> libc::c_int,
        );
        (*api)
            .encoder_encode = Some(
            x264_8_encoder_encode
                as unsafe extern "C" fn(
                    *mut x264_t,
                    *mut *mut x264_nal_t,
                    *mut libc::c_int,
                    *mut x264_picture_t,
                    *mut x264_picture_t,
                ) -> libc::c_int,
        );
        (*api)
            .encoder_close = Some(
            x264_8_encoder_close as unsafe extern "C" fn(*mut x264_t) -> (),
        );
        (*api)
            .encoder_delayed_frames = Some(
            x264_8_encoder_delayed_frames
                as unsafe extern "C" fn(*mut x264_t) -> libc::c_int,
        );
        (*api)
            .encoder_maximum_delayed_frames = Some(
            x264_8_encoder_maximum_delayed_frames
                as unsafe extern "C" fn(*mut x264_t) -> libc::c_int,
        );
        (*api)
            .encoder_intra_refresh = Some(
            x264_8_encoder_intra_refresh as unsafe extern "C" fn(*mut x264_t) -> (),
        );
        (*api)
            .encoder_invalidate_reference = Some(
            x264_8_encoder_invalidate_reference
                as unsafe extern "C" fn(*mut x264_t, int64_t) -> libc::c_int,
        );
        (*api).x264 = x264_8_encoder_open(param, api as *mut libc::c_void);
    } else {
        x264_log_internal(
            0 as libc::c_int,
            b"not compiled with %d bit depth support\n\0" as *const u8
                as *const libc::c_char,
            (*param).i_bitdepth,
        );
    }
    if ((*api).x264).is_null() {
        free(api as *mut libc::c_void);
        return 0 as *mut x264_t;
    }
    return api as *mut x264_t;
}
#[no_mangle]
pub unsafe extern "C" fn x264_encoder_close(mut h: *mut x264_t) {
    let mut api: *mut x264_api_t = h as *mut x264_api_t;
    ((*api).encoder_close).expect("non-null function pointer")((*api).x264);
    free(api as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn x264_nal_encode(
    mut h: *mut x264_t,
    mut dst: *mut uint8_t,
    mut nal: *mut x264_nal_t,
) {
    let mut api: *mut x264_api_t = h as *mut x264_api_t;
    ((*api).nal_encode).expect("non-null function pointer")((*api).x264, dst, nal);
}
#[no_mangle]
pub unsafe extern "C" fn x264_encoder_reconfig(
    mut h: *mut x264_t,
    mut param: *mut x264_param_t,
) -> libc::c_int {
    let mut api: *mut x264_api_t = h as *mut x264_api_t;
    return ((*api).encoder_reconfig)
        .expect("non-null function pointer")((*api).x264, param);
}
#[no_mangle]
pub unsafe extern "C" fn x264_encoder_parameters(
    mut h: *mut x264_t,
    mut param: *mut x264_param_t,
) {
    let mut api: *mut x264_api_t = h as *mut x264_api_t;
    ((*api).encoder_parameters).expect("non-null function pointer")((*api).x264, param);
}
#[no_mangle]
pub unsafe extern "C" fn x264_encoder_headers(
    mut h: *mut x264_t,
    mut pp_nal: *mut *mut x264_nal_t,
    mut pi_nal: *mut libc::c_int,
) -> libc::c_int {
    let mut api: *mut x264_api_t = h as *mut x264_api_t;
    return ((*api).encoder_headers)
        .expect("non-null function pointer")((*api).x264, pp_nal, pi_nal);
}
#[no_mangle]
pub unsafe extern "C" fn x264_encoder_encode(
    mut h: *mut x264_t,
    mut pp_nal: *mut *mut x264_nal_t,
    mut pi_nal: *mut libc::c_int,
    mut pic_in: *mut x264_picture_t,
    mut pic_out: *mut x264_picture_t,
) -> libc::c_int {
    let mut api: *mut x264_api_t = h as *mut x264_api_t;
    return ((*api).encoder_encode)
        .expect(
            "non-null function pointer",
        )((*api).x264, pp_nal, pi_nal, pic_in, pic_out);
}
#[no_mangle]
pub unsafe extern "C" fn x264_encoder_delayed_frames(mut h: *mut x264_t) -> libc::c_int {
    let mut api: *mut x264_api_t = h as *mut x264_api_t;
    return ((*api).encoder_delayed_frames)
        .expect("non-null function pointer")((*api).x264);
}
#[no_mangle]
pub unsafe extern "C" fn x264_encoder_maximum_delayed_frames(
    mut h: *mut x264_t,
) -> libc::c_int {
    let mut api: *mut x264_api_t = h as *mut x264_api_t;
    return ((*api).encoder_maximum_delayed_frames)
        .expect("non-null function pointer")((*api).x264);
}
#[no_mangle]
pub unsafe extern "C" fn x264_encoder_intra_refresh(mut h: *mut x264_t) {
    let mut api: *mut x264_api_t = h as *mut x264_api_t;
    ((*api).encoder_intra_refresh).expect("non-null function pointer")((*api).x264);
}
#[no_mangle]
pub unsafe extern "C" fn x264_encoder_invalidate_reference(
    mut h: *mut x264_t,
    mut pts: int64_t,
) -> libc::c_int {
    let mut api: *mut x264_api_t = h as *mut x264_api_t;
    return ((*api).encoder_invalidate_reference)
        .expect("non-null function pointer")((*api).x264, pts);
}
