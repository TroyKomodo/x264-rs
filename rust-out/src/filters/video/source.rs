#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type x264_t;
    static mut cli_input: cli_input_t;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
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
pub type hnd_t = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cli_input_opt_t {
    pub index_file: *mut libc::c_char,
    pub format: *mut libc::c_char,
    pub resolution: *mut libc::c_char,
    pub colorspace: *mut libc::c_char,
    pub bit_depth: libc::c_int,
    pub timebase: *mut libc::c_char,
    pub seek: libc::c_int,
    pub progress: libc::c_int,
    pub output_csp: libc::c_int,
    pub output_range: libc::c_int,
    pub input_range: libc::c_int,
}
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
pub struct cli_input_t {
    pub open_file: Option::<
        unsafe extern "C" fn(
            *mut libc::c_char,
            *mut hnd_t,
            *mut video_info_t,
            *mut cli_input_opt_t,
        ) -> libc::c_int,
    >,
    pub picture_alloc: Option::<
        unsafe extern "C" fn(
            *mut cli_pic_t,
            hnd_t,
            libc::c_int,
            libc::c_int,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub read_frame: Option::<
        unsafe extern "C" fn(*mut cli_pic_t, hnd_t, libc::c_int) -> libc::c_int,
    >,
    pub release_frame: Option::<
        unsafe extern "C" fn(*mut cli_pic_t, hnd_t) -> libc::c_int,
    >,
    pub picture_clean: Option::<unsafe extern "C" fn(*mut cli_pic_t, hnd_t) -> ()>,
    pub close_file: Option::<unsafe extern "C" fn(hnd_t) -> libc::c_int>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cli_vid_filter_t {
    pub name: *const libc::c_char,
    pub help: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    pub init: Option::<
        unsafe extern "C" fn(
            *mut hnd_t,
            *mut cli_vid_filter_t,
            *mut video_info_t,
            *mut x264_param_t,
            *mut libc::c_char,
        ) -> libc::c_int,
    >,
    pub get_frame: Option::<
        unsafe extern "C" fn(hnd_t, *mut cli_pic_t, libc::c_int) -> libc::c_int,
    >,
    pub release_frame: Option::<
        unsafe extern "C" fn(hnd_t, *mut cli_pic_t, libc::c_int) -> libc::c_int,
    >,
    pub free: Option::<unsafe extern "C" fn(hnd_t) -> ()>,
    pub next: *mut cli_vid_filter_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct source_hnd_t {
    pub pic: cli_pic_t,
    pub hin: hnd_t,
    pub cur_frame: libc::c_int,
}
unsafe extern "C" fn init(
    mut handle: *mut hnd_t,
    mut filter: *mut cli_vid_filter_t,
    mut info: *mut video_info_t,
    mut param: *mut x264_param_t,
    mut opt_string: *mut libc::c_char,
) -> libc::c_int {
    let mut h: *mut source_hnd_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<source_hnd_t>() as libc::c_ulong,
    ) as *mut source_hnd_t;
    if h.is_null() {
        return -(1 as libc::c_int);
    }
    (*h).cur_frame = -(1 as libc::c_int);
    if (cli_input.picture_alloc)
        .expect(
            "non-null function pointer",
        )(&mut (*h).pic, *handle, (*info).csp, (*info).width, (*info).height) != 0
    {
        return -(1 as libc::c_int);
    }
    (*h).hin = *handle;
    *handle = h as hnd_t;
    *filter = source_filter;
    0 as libc::c_int
}
unsafe extern "C" fn get_frame(
    mut handle: hnd_t,
    mut output: *mut cli_pic_t,
    mut frame: libc::c_int,
) -> libc::c_int {
    let mut h: *mut source_hnd_t = handle as *mut source_hnd_t;
    if frame <= (*h).cur_frame
        || (cli_input.read_frame)
            .expect("non-null function pointer")(&mut (*h).pic, (*h).hin, frame) != 0
    {
        return -(1 as libc::c_int);
    }
    (*h).cur_frame = frame;
    *output = (*h).pic;
    0 as libc::c_int
}
unsafe extern "C" fn release_frame(
    mut handle: hnd_t,
    mut pic: *mut cli_pic_t,
    mut frame: libc::c_int,
) -> libc::c_int {
    let mut h: *mut source_hnd_t = handle as *mut source_hnd_t;
    if (cli_input.release_frame).is_some()
        && (cli_input.release_frame)
            .expect("non-null function pointer")(&mut (*h).pic, (*h).hin) != 0
    {
        return -(1 as libc::c_int);
    }
    0 as libc::c_int
}
unsafe extern "C" fn free_filter(mut handle: hnd_t) {
    let mut h: *mut source_hnd_t = handle as *mut source_hnd_t;
    (cli_input.picture_clean)
        .expect("non-null function pointer")(&mut (*h).pic, (*h).hin);
    (cli_input.close_file).expect("non-null function pointer")((*h).hin);
    free(h as *mut libc::c_void);
}
#[no_mangle]
pub static mut source_filter: cli_vid_filter_t = unsafe {
    {
        
        cli_vid_filter_t {
            name: b"source\0" as *const u8 as *const libc::c_char,
            help: None,
            init: Some(
                init
                    as unsafe extern "C" fn(
                        *mut hnd_t,
                        *mut cli_vid_filter_t,
                        *mut video_info_t,
                        *mut x264_param_t,
                        *mut libc::c_char,
                    ) -> libc::c_int,
            ),
            get_frame: Some(
                get_frame
                    as unsafe extern "C" fn(
                        hnd_t,
                        *mut cli_pic_t,
                        libc::c_int,
                    ) -> libc::c_int,
            ),
            release_frame: Some(
                release_frame
                    as unsafe extern "C" fn(
                        hnd_t,
                        *mut cli_pic_t,
                        libc::c_int,
                    ) -> libc::c_int,
            ),
            free: Some(free_filter as unsafe extern "C" fn(hnd_t) -> ()),
            next: 0 as *const cli_vid_filter_t as *mut cli_vid_filter_t,
        }
    }
};
