#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type x264_t;
    pub type mk_writer;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn mk_create_writer(filename: *const libc::c_char) -> *mut mk_writer;
    fn mk_write_header(
        w: *mut mk_writer,
        writing_app: *const libc::c_char,
        codec_id: *const libc::c_char,
        codec_private: *const libc::c_void,
        codec_private_size: libc::c_uint,
        default_frame_duration: int64_t,
        timescale: int64_t,
        width: libc::c_uint,
        height: libc::c_uint,
        d_width: libc::c_uint,
        d_height: libc::c_uint,
        display_size_units: libc::c_int,
        stereo_mode: libc::c_int,
    ) -> libc::c_int;
    fn mk_start_frame(w: *mut mk_writer) -> libc::c_int;
    fn mk_add_frame_data(
        w: *mut mk_writer,
        data: *const libc::c_void,
        size: libc::c_uint,
    ) -> libc::c_int;
    fn mk_set_frame_flags(
        w: *mut mk_writer,
        timestamp: int64_t,
        keyframe: libc::c_int,
        skippable: libc::c_int,
    ) -> libc::c_int;
    fn mk_close(w: *mut mk_writer, last_delta: int64_t) -> libc::c_int;
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
pub struct x264_hrd_t {
    pub cpb_initial_arrival_time: libc::c_double,
    pub cpb_final_arrival_time: libc::c_double,
    pub cpb_removal_time: libc::c_double,
    pub dpb_output_time: libc::c_double,
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
pub struct x264_sei_t {
    pub num_payloads: libc::c_int,
    pub payloads: *mut x264_sei_payload_t,
    pub sei_free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_image_t {
    pub i_csp: libc::c_int,
    pub i_plane: libc::c_int,
    pub i_stride: [libc::c_int; 4],
    pub plane: [*mut uint8_t; 4],
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
pub type hnd_t = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cli_output_opt_t {
    pub use_dts_compress: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cli_output_t {
    pub open_file: Option::<
        unsafe extern "C" fn(
            *mut libc::c_char,
            *mut hnd_t,
            *mut cli_output_opt_t,
        ) -> libc::c_int,
    >,
    pub set_param: Option::<
        unsafe extern "C" fn(hnd_t, *mut x264_param_t) -> libc::c_int,
    >,
    pub write_headers: Option::<
        unsafe extern "C" fn(hnd_t, *mut x264_nal_t) -> libc::c_int,
    >,
    pub write_frame: Option::<
        unsafe extern "C" fn(
            hnd_t,
            *mut uint8_t,
            libc::c_int,
            *mut x264_picture_t,
        ) -> libc::c_int,
    >,
    pub close_file: Option::<
        unsafe extern "C" fn(hnd_t, int64_t, int64_t) -> libc::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mkv_hnd_t {
    pub w: *mut mk_writer,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub d_width: libc::c_int,
    pub d_height: libc::c_int,
    pub display_size_units: libc::c_int,
    pub stereo_mode: libc::c_int,
    pub frame_duration: int64_t,
    pub b_writing_frame: libc::c_char,
    pub i_timebase_num: uint32_t,
    pub i_timebase_den: uint32_t,
}
unsafe extern "C" fn open_file(
    mut psz_filename: *mut libc::c_char,
    mut p_handle: *mut hnd_t,
    mut opt: *mut cli_output_opt_t,
) -> libc::c_int {
    *p_handle = 0 as *mut libc::c_void;
    let mut p_mkv: *mut mkv_hnd_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<mkv_hnd_t>() as libc::c_ulong,
    ) as *mut mkv_hnd_t;
    if p_mkv.is_null() {
        return -(1 as libc::c_int);
    }
    (*p_mkv).w = mk_create_writer(psz_filename);
    if ((*p_mkv).w).is_null() {
        free(p_mkv as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    *p_handle = p_mkv as hnd_t;
    return 0 as libc::c_int;
}
static mut stereo_modes: [uint8_t; 7] = [
    5 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    7 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
];
static mut stereo_w_div: [uint8_t; 7] = [
    1 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
];
static mut stereo_h_div: [uint8_t; 7] = [
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
];
unsafe extern "C" fn set_param(
    mut handle: hnd_t,
    mut p_param: *mut x264_param_t,
) -> libc::c_int {
    let mut p_mkv: *mut mkv_hnd_t = handle as *mut mkv_hnd_t;
    let mut dw: int64_t = 0;
    let mut dh: int64_t = 0;
    if (*p_param).i_fps_num > 0 as libc::c_int as uint32_t && (*p_param).b_vfr_input == 0
    {
        (*p_mkv)
            .frame_duration = (*p_param).i_fps_den as int64_t
            * 1000000000 as libc::c_int as int64_t / (*p_param).i_fps_num as int64_t;
    } else {
        (*p_mkv).frame_duration = 0 as libc::c_int as int64_t;
    }
    (*p_mkv).width = (*p_param).i_width;
    dw = (*p_mkv).width as int64_t;
    (*p_mkv).height = (*p_param).i_height;
    dh = (*p_mkv).height as int64_t;
    (*p_mkv).display_size_units = 0 as libc::c_int;
    (*p_mkv).stereo_mode = -(1 as libc::c_int);
    if (*p_param).i_frame_packing >= 0 as libc::c_int
        && (*p_param).i_frame_packing < 7 as libc::c_int
    {
        (*p_mkv)
            .stereo_mode = stereo_modes[(*p_param).i_frame_packing as usize]
            as libc::c_int;
        dw /= stereo_w_div[(*p_param).i_frame_packing as usize] as int64_t;
        dh /= stereo_h_div[(*p_param).i_frame_packing as usize] as int64_t;
    }
    if (*p_param).vui.i_sar_width != 0 && (*p_param).vui.i_sar_height != 0
        && (*p_param).vui.i_sar_width != (*p_param).vui.i_sar_height
    {
        if (*p_param).vui.i_sar_width > (*p_param).vui.i_sar_height {
            dw = dw * (*p_param).vui.i_sar_width as int64_t
                / (*p_param).vui.i_sar_height as int64_t;
        } else {
            dh = dh * (*p_param).vui.i_sar_height as int64_t
                / (*p_param).vui.i_sar_width as int64_t;
        }
    }
    (*p_mkv).d_width = dw as libc::c_int;
    (*p_mkv).d_height = dh as libc::c_int;
    (*p_mkv).i_timebase_num = (*p_param).i_timebase_num;
    (*p_mkv).i_timebase_den = (*p_param).i_timebase_den;
    return 0 as libc::c_int;
}
unsafe extern "C" fn write_headers(
    mut handle: hnd_t,
    mut p_nal: *mut x264_nal_t,
) -> libc::c_int {
    let mut p_mkv: *mut mkv_hnd_t = handle as *mut mkv_hnd_t;
    let mut sps_size: libc::c_int = (*p_nal.offset(0 as libc::c_int as isize)).i_payload
        - 4 as libc::c_int;
    let mut pps_size: libc::c_int = (*p_nal.offset(1 as libc::c_int as isize)).i_payload
        - 4 as libc::c_int;
    let mut sei_size: libc::c_int = (*p_nal.offset(2 as libc::c_int as isize)).i_payload;
    let mut sps: *mut uint8_t = ((*p_nal.offset(0 as libc::c_int as isize)).p_payload)
        .offset(4 as libc::c_int as isize);
    let mut pps: *mut uint8_t = ((*p_nal.offset(1 as libc::c_int as isize)).p_payload)
        .offset(4 as libc::c_int as isize);
    let mut sei: *mut uint8_t = (*p_nal.offset(2 as libc::c_int as isize)).p_payload;
    let mut ret: libc::c_int = 0;
    let mut avcC: *mut uint8_t = 0 as *mut uint8_t;
    let mut avcC_len: libc::c_int = 0;
    if (*p_mkv).width == 0 || (*p_mkv).height == 0 || (*p_mkv).d_width == 0
        || (*p_mkv).d_height == 0
    {
        return -(1 as libc::c_int);
    }
    avcC_len = 5 as libc::c_int + 1 as libc::c_int + 2 as libc::c_int + sps_size
        + 1 as libc::c_int + 2 as libc::c_int + pps_size;
    avcC = malloc(avcC_len as libc::c_ulong) as *mut uint8_t;
    if avcC.is_null() {
        return -(1 as libc::c_int);
    }
    *avcC.offset(0 as libc::c_int as isize) = 1 as libc::c_int as uint8_t;
    *avcC.offset(1 as libc::c_int as isize) = *sps.offset(1 as libc::c_int as isize);
    *avcC.offset(2 as libc::c_int as isize) = *sps.offset(2 as libc::c_int as isize);
    *avcC.offset(3 as libc::c_int as isize) = *sps.offset(3 as libc::c_int as isize);
    *avcC.offset(4 as libc::c_int as isize) = 0xff as libc::c_int as uint8_t;
    *avcC.offset(5 as libc::c_int as isize) = 0xe1 as libc::c_int as uint8_t;
    *avcC.offset(6 as libc::c_int as isize) = (sps_size >> 8 as libc::c_int) as uint8_t;
    *avcC.offset(7 as libc::c_int as isize) = sps_size as uint8_t;
    memcpy(
        avcC.offset(8 as libc::c_int as isize) as *mut libc::c_void,
        sps as *const libc::c_void,
        sps_size as libc::c_ulong,
    );
    *avcC.offset((8 as libc::c_int + sps_size) as isize) = 1 as libc::c_int as uint8_t;
    *avcC
        .offset(
            (9 as libc::c_int + sps_size) as isize,
        ) = (pps_size >> 8 as libc::c_int) as uint8_t;
    *avcC.offset((10 as libc::c_int + sps_size) as isize) = pps_size as uint8_t;
    memcpy(
        avcC.offset(11 as libc::c_int as isize).offset(sps_size as isize)
            as *mut libc::c_void,
        pps as *const libc::c_void,
        pps_size as libc::c_ulong,
    );
    ret = mk_write_header(
        (*p_mkv).w,
        b"x264 r3204 373697b\0" as *const u8 as *const libc::c_char,
        b"V_MPEG4/ISO/AVC\0" as *const u8 as *const libc::c_char,
        avcC as *const libc::c_void,
        avcC_len as libc::c_uint,
        (*p_mkv).frame_duration,
        50000 as libc::c_int as int64_t,
        (*p_mkv).width as libc::c_uint,
        (*p_mkv).height as libc::c_uint,
        (*p_mkv).d_width as libc::c_uint,
        (*p_mkv).d_height as libc::c_uint,
        (*p_mkv).display_size_units,
        (*p_mkv).stereo_mode,
    );
    free(avcC as *mut libc::c_void);
    if ret < 0 as libc::c_int {
        return ret;
    }
    if (*p_mkv).b_writing_frame == 0 {
        if mk_start_frame((*p_mkv).w) < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        (*p_mkv).b_writing_frame = 1 as libc::c_int as libc::c_char;
    }
    if mk_add_frame_data(
        (*p_mkv).w,
        sei as *const libc::c_void,
        sei_size as libc::c_uint,
    ) < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    return sei_size + sps_size + pps_size;
}
unsafe extern "C" fn write_frame(
    mut handle: hnd_t,
    mut p_nalu: *mut uint8_t,
    mut i_size: libc::c_int,
    mut p_picture: *mut x264_picture_t,
) -> libc::c_int {
    let mut p_mkv: *mut mkv_hnd_t = handle as *mut mkv_hnd_t;
    if (*p_mkv).b_writing_frame == 0 {
        if mk_start_frame((*p_mkv).w) < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        (*p_mkv).b_writing_frame = 1 as libc::c_int as libc::c_char;
    }
    if mk_add_frame_data(
        (*p_mkv).w,
        p_nalu as *const libc::c_void,
        i_size as libc::c_uint,
    ) < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    let mut i_stamp: int64_t = ((*p_picture).i_pts as libc::c_double * 1e9f64
        * (*p_mkv).i_timebase_num as libc::c_double
        / (*p_mkv).i_timebase_den as libc::c_double + 0.5f64) as int64_t;
    (*p_mkv).b_writing_frame = 0 as libc::c_int as libc::c_char;
    if mk_set_frame_flags(
        (*p_mkv).w,
        i_stamp,
        (*p_picture).b_keyframe,
        ((*p_picture).i_type == 0x5 as libc::c_int) as libc::c_int,
    ) < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    return i_size;
}
unsafe extern "C" fn close_file(
    mut handle: hnd_t,
    mut largest_pts: int64_t,
    mut second_largest_pts: int64_t,
) -> libc::c_int {
    let mut p_mkv: *mut mkv_hnd_t = handle as *mut mkv_hnd_t;
    let mut ret: libc::c_int = 0;
    let mut i_last_delta: int64_t = 0;
    i_last_delta = if (*p_mkv).i_timebase_den != 0 {
        (((largest_pts - second_largest_pts) * (*p_mkv).i_timebase_num as int64_t
            / (*p_mkv).i_timebase_den as int64_t) as libc::c_double + 0.5f64) as int64_t
    } else {
        0 as libc::c_int as int64_t
    };
    ret = mk_close((*p_mkv).w, i_last_delta);
    free(p_mkv as *mut libc::c_void);
    return ret;
}
#[no_mangle]
pub static mut mkv_output: cli_output_t = unsafe {
    {
        let mut init = cli_output_t {
            open_file: Some(
                open_file
                    as unsafe extern "C" fn(
                        *mut libc::c_char,
                        *mut hnd_t,
                        *mut cli_output_opt_t,
                    ) -> libc::c_int,
            ),
            set_param: Some(
                set_param
                    as unsafe extern "C" fn(hnd_t, *mut x264_param_t) -> libc::c_int,
            ),
            write_headers: Some(
                write_headers
                    as unsafe extern "C" fn(hnd_t, *mut x264_nal_t) -> libc::c_int,
            ),
            write_frame: Some(
                write_frame
                    as unsafe extern "C" fn(
                        hnd_t,
                        *mut uint8_t,
                        libc::c_int,
                        *mut x264_picture_t,
                    ) -> libc::c_int,
            ),
            close_file: Some(
                close_file
                    as unsafe extern "C" fn(hnd_t, int64_t, int64_t) -> libc::c_int,
            ),
        };
        init
    }
};
