#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(c_variadic, extern_types, core_intrinsics)]

mod common;
mod encoder;
mod filters;
mod input;
mod output;
mod autocomplete;

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type x264_t;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn fscanf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fseeko(
        __stream: *mut FILE,
        __off: __off64_t,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn ftello(__stream: *mut FILE) -> __off64_t;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
    fn exit(_: libc::c_int) -> !;
    fn fmod(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    static x264_levels: [x264_level_t; 0];
    fn x264_param_default(_: *mut x264_param_t);
    fn x264_param_parse(
        _: *mut x264_param_t,
        name: *const libc::c_char,
        value: *const libc::c_char,
    ) -> libc::c_int;
    fn x264_param_cleanup(param: *mut x264_param_t);
    fn x264_param_default_preset(
        _: *mut x264_param_t,
        preset: *const libc::c_char,
        tune: *const libc::c_char,
    ) -> libc::c_int;
    fn x264_param_apply_fastfirstpass(_: *mut x264_param_t);
    fn x264_param_apply_profile(
        _: *mut x264_param_t,
        profile: *const libc::c_char,
    ) -> libc::c_int;
    static x264_chroma_format: libc::c_int;
    fn x264_picture_init(pic: *mut x264_picture_t);
    fn x264_encoder_open_164(_: *mut x264_param_t) -> *mut x264_t;
    fn x264_encoder_parameters(_: *mut x264_t, _: *mut x264_param_t);
    fn x264_encoder_headers(
        _: *mut x264_t,
        pp_nal: *mut *mut x264_nal_t,
        pi_nal: *mut libc::c_int,
    ) -> libc::c_int;
    fn x264_encoder_encode(
        _: *mut x264_t,
        pp_nal: *mut *mut x264_nal_t,
        pi_nal: *mut libc::c_int,
        pic_in: *mut x264_picture_t,
        pic_out: *mut x264_picture_t,
    ) -> libc::c_int;
    fn x264_encoder_close(_: *mut x264_t);
    fn x264_encoder_delayed_frames(_: *mut x264_t) -> libc::c_int;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn x264_mdate() -> int64_t;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn x264_cpu_num_processors() -> libc::c_int;
    fn x264_reduce_fraction(n: *mut uint32_t, d: *mut uint32_t);
    fn x264_cli_autocomplete(
        prev: *const libc::c_char,
        cur: *const libc::c_char,
    ) -> libc::c_int;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    static raw_input: cli_input_t;
    static y4m_input: cli_input_t;
    static thread_8_input: cli_input_t;
    static timecode_input: cli_input_t;
    static x264_cli_csps: [x264_cli_csp_t; 0];
    static raw_output: cli_output_t;
    static mkv_output: cli_output_t;
    static flv_output: cli_output_t;
    fn x264_register_vid_filters();
    fn x264_vid_filter_help(longhelp: libc::c_int);
    fn x264_init_vid_filter(
        name: *const libc::c_char,
        handle: *mut hnd_t,
        filter_0: *mut cli_vid_filter_t,
        info: *mut video_info_t,
        param: *mut x264_param_t,
        opt_string: *mut libc::c_char,
    ) -> libc::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type __gnuc_va_list = __builtin_va_list;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type va_list = __gnuc_va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
pub struct x264_level_t {
    pub level_idc: uint8_t,
    pub mbps: int32_t,
    pub frame_size: int32_t,
    pub dpb: int32_t,
    pub bitrate: int32_t,
    pub cpb: int32_t,
    pub mv_range: uint16_t,
    pub mvs_per_2mb: uint8_t,
    pub slice_rate: uint8_t,
    pub mincr: uint8_t,
    pub bipred8x8: uint8_t,
    pub direct8x8: uint8_t,
    pub frame_only: uint8_t,
}
pub type pic_struct_e = libc::c_uint;
pub const PIC_STRUCT_TRIPLE: pic_struct_e = 9;
pub const PIC_STRUCT_DOUBLE: pic_struct_e = 8;
pub const PIC_STRUCT_BOTTOM_TOP_BOTTOM: pic_struct_e = 7;
pub const PIC_STRUCT_TOP_BOTTOM_TOP: pic_struct_e = 6;
pub const PIC_STRUCT_BOTTOM_TOP: pic_struct_e = 5;
pub const PIC_STRUCT_TOP_BOTTOM: pic_struct_e = 4;
pub const PIC_STRUCT_PROGRESSIVE: pic_struct_e = 1;
pub const PIC_STRUCT_AUTO: pic_struct_e = 0;
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
pub type C2RustUnnamed_5 = libc::c_int;
pub const RANGE_PC: C2RustUnnamed_5 = 1;
pub const RANGE_TV: C2RustUnnamed_5 = 0;
pub const RANGE_AUTO: C2RustUnnamed_5 = -1;
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
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
pub struct x264_cli_csp_t {
    pub name: *const libc::c_char,
    pub planes: libc::c_int,
    pub width: [libc::c_float; 4],
    pub height: [libc::c_float; 4],
    pub mod_width: libc::c_int,
    pub mod_height: libc::c_int,
}
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
pub struct cli_opt_t {
    pub b_progress: libc::c_int,
    pub i_seek: libc::c_int,
    pub hin: hnd_t,
    pub hout: hnd_t,
    pub qpfile: *mut FILE,
    pub tcfile_out: *mut FILE,
    pub timebase_convert_multiplier: libc::c_double,
    pub i_pulldown: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cli_pulldown_t {
    pub mod_0: libc::c_int,
    pub pattern: [uint8_t; 24],
    pub fps_factor: libc::c_float,
}
pub const OPT_INPUT_RANGE: C2RustUnnamed_6 = 286;
pub const OPT_OUTPUT_CSP: C2RustUnnamed_6 = 285;
pub const OPT_DTS_COMPRESSION: C2RustUnnamed_6 = 284;
pub const OPT_OUTPUT_DEPTH: C2RustUnnamed_6 = 283;
pub const OPT_INPUT_DEPTH: C2RustUnnamed_6 = 282;
pub const OPT_INPUT_CSP: C2RustUnnamed_6 = 281;
pub const OPT_INPUT_RES: C2RustUnnamed_6 = 280;
pub const OPT_INPUT_FMT: C2RustUnnamed_6 = 279;
pub const OPT_VIDEO_FILTER: C2RustUnnamed_6 = 278;
pub const OPT_PULLDOWN: C2RustUnnamed_6 = 276;
pub const OPT_TIMEBASE: C2RustUnnamed_6 = 275;
pub const OPT_TCFILE_OUT: C2RustUnnamed_6 = 274;
pub const OPT_TCFILE_IN: C2RustUnnamed_6 = 273;
pub const OPT_RANGE: C2RustUnnamed_6 = 287;
pub const OPT_NOPROGRESS: C2RustUnnamed_6 = 261;
pub const OPT_LOG_LEVEL: C2RustUnnamed_6 = 277;
pub const OPT_QUIET: C2RustUnnamed_6 = 260;
pub const OPT_THREAD_INPUT: C2RustUnnamed_6 = 259;
pub const OPT_QPFILE: C2RustUnnamed_6 = 258;
pub const OPT_INDEX: C2RustUnnamed_6 = 271;
pub const OPT_DEMUXER: C2RustUnnamed_6 = 270;
pub const OPT_MUXER: C2RustUnnamed_6 = 269;
pub const OPT_SEEK: C2RustUnnamed_6 = 257;
pub const OPT_FRAMES: C2RustUnnamed_6 = 256;
pub const OPT_FPS: C2RustUnnamed_6 = 268;
pub const OPT_INTERLACED: C2RustUnnamed_6 = 272;
pub const OPT_SLOWFIRSTPASS: C2RustUnnamed_6 = 266;
pub const OPT_TUNE: C2RustUnnamed_6 = 265;
pub const OPT_PRESET: C2RustUnnamed_6 = 264;
pub const OPT_PROFILE: C2RustUnnamed_6 = 263;
pub const OPT_FULLHELP: C2RustUnnamed_6 = 267;
pub const OPT_LONGHELP: C2RustUnnamed_6 = 262;
pub type C2RustUnnamed_6 = libc::c_uint;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
static mut x264_direct_pred_names: [*const libc::c_char; 5] = [
    b"none\0" as *const u8 as *const libc::c_char,
    b"spatial\0" as *const u8 as *const libc::c_char,
    b"temporal\0" as *const u8 as *const libc::c_char,
    b"auto\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut x264_motion_est_names: [*const libc::c_char; 6] = [
    b"dia\0" as *const u8 as *const libc::c_char,
    b"hex\0" as *const u8 as *const libc::c_char,
    b"umh\0" as *const u8 as *const libc::c_char,
    b"esa\0" as *const u8 as *const libc::c_char,
    b"tesa\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut x264_b_pyramid_names: [*const libc::c_char; 4] = [
    b"none\0" as *const u8 as *const libc::c_char,
    b"strict\0" as *const u8 as *const libc::c_char,
    b"normal\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut x264_overscan_names: [*const libc::c_char; 4] = [
    b"undef\0" as *const u8 as *const libc::c_char,
    b"show\0" as *const u8 as *const libc::c_char,
    b"crop\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut x264_vidformat_names: [*const libc::c_char; 7] = [
    b"component\0" as *const u8 as *const libc::c_char,
    b"pal\0" as *const u8 as *const libc::c_char,
    b"ntsc\0" as *const u8 as *const libc::c_char,
    b"secam\0" as *const u8 as *const libc::c_char,
    b"mac\0" as *const u8 as *const libc::c_char,
    b"undef\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut x264_colorprim_names: [*const libc::c_char; 14] = [
    b"\0" as *const u8 as *const libc::c_char,
    b"bt709\0" as *const u8 as *const libc::c_char,
    b"undef\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"bt470m\0" as *const u8 as *const libc::c_char,
    b"bt470bg\0" as *const u8 as *const libc::c_char,
    b"smpte170m\0" as *const u8 as *const libc::c_char,
    b"smpte240m\0" as *const u8 as *const libc::c_char,
    b"film\0" as *const u8 as *const libc::c_char,
    b"bt2020\0" as *const u8 as *const libc::c_char,
    b"smpte428\0" as *const u8 as *const libc::c_char,
    b"smpte431\0" as *const u8 as *const libc::c_char,
    b"smpte432\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut x264_transfer_names: [*const libc::c_char; 20] = [
    b"\0" as *const u8 as *const libc::c_char,
    b"bt709\0" as *const u8 as *const libc::c_char,
    b"undef\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"bt470m\0" as *const u8 as *const libc::c_char,
    b"bt470bg\0" as *const u8 as *const libc::c_char,
    b"smpte170m\0" as *const u8 as *const libc::c_char,
    b"smpte240m\0" as *const u8 as *const libc::c_char,
    b"linear\0" as *const u8 as *const libc::c_char,
    b"log100\0" as *const u8 as *const libc::c_char,
    b"log316\0" as *const u8 as *const libc::c_char,
    b"iec61966-2-4\0" as *const u8 as *const libc::c_char,
    b"bt1361e\0" as *const u8 as *const libc::c_char,
    b"iec61966-2-1\0" as *const u8 as *const libc::c_char,
    b"bt2020-10\0" as *const u8 as *const libc::c_char,
    b"bt2020-12\0" as *const u8 as *const libc::c_char,
    b"smpte2084\0" as *const u8 as *const libc::c_char,
    b"smpte428\0" as *const u8 as *const libc::c_char,
    b"arib-std-b67\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut x264_colmatrix_names: [*const libc::c_char; 16] = [
    b"GBR\0" as *const u8 as *const libc::c_char,
    b"bt709\0" as *const u8 as *const libc::c_char,
    b"undef\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"fcc\0" as *const u8 as *const libc::c_char,
    b"bt470bg\0" as *const u8 as *const libc::c_char,
    b"smpte170m\0" as *const u8 as *const libc::c_char,
    b"smpte240m\0" as *const u8 as *const libc::c_char,
    b"YCgCo\0" as *const u8 as *const libc::c_char,
    b"bt2020nc\0" as *const u8 as *const libc::c_char,
    b"bt2020c\0" as *const u8 as *const libc::c_char,
    b"smpte2085\0" as *const u8 as *const libc::c_char,
    b"chroma-derived-nc\0" as *const u8 as *const libc::c_char,
    b"chroma-derived-c\0" as *const u8 as *const libc::c_char,
    b"ICtCp\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut x264_avcintra_flavor_names: [*const libc::c_char; 3] = [
    b"panasonic\0" as *const u8 as *const libc::c_char,
    b"sony\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
#[inline]
unsafe extern "C" fn x264_is_regular_file_path(
    mut filename: *const libc::c_char,
) -> libc::c_int {
    let mut file_stat: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    if stat(filename, &mut file_stat) != 0 {
        return 1 as libc::c_int;
    }
    return (file_stat.st_mode & 0o170000 as libc::c_int as __mode_t
        == 0o100000 as libc::c_int as __mode_t) as libc::c_int;
}
#[inline]
unsafe extern "C" fn x264_is_regular_file(mut filehandle: *mut FILE) -> libc::c_int {
    let mut file_stat: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    if fstat(fileno(filehandle), &mut file_stat) != 0 {
        return 1 as libc::c_int;
    }
    return (file_stat.st_mode & 0o170000 as libc::c_int as __mode_t
        == 0o100000 as libc::c_int as __mode_t) as libc::c_int;
}
#[inline]
unsafe extern "C" fn get_filename_extension(
    mut filename: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut ext: *mut libc::c_char = filename.offset(strlen(filename) as isize);
    while *ext as libc::c_int != '.' as i32 && ext > filename {
        ext = ext.offset(-1);
        ext;
    }
    ext = ext.offset((*ext as libc::c_int == '.' as i32) as libc::c_int as isize);
    return ext;
}
static mut b_ctrl_c: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn sigint_handler(mut a: libc::c_int) {
    ::core::ptr::write_volatile(&mut b_ctrl_c as *mut libc::c_int, 1 as libc::c_int);
}
#[no_mangle]
pub static mut cli_input: cli_input_t = cli_input_t {
    open_file: None,
    picture_alloc: None,
    read_frame: None,
    release_frame: None,
    picture_clean: None,
    close_file: None,
};
static mut cli_output: cli_output_t = cli_output_t {
    open_file: None,
    set_param: None,
    write_headers: None,
    write_frame: None,
    close_file: None,
};
static mut filter: cli_vid_filter_t = cli_vid_filter_t {
    name: 0 as *const libc::c_char,
    help: None,
    init: None,
    get_frame: None,
    release_frame: None,
    free: None,
    next: 0 as *const cli_vid_filter_t as *mut cli_vid_filter_t,
};
#[no_mangle]
pub static mut x264_avcintra_class_names: [*const libc::c_char; 6] = [
    b"50\0" as *const u8 as *const libc::c_char,
    b"100\0" as *const u8 as *const libc::c_char,
    b"200\0" as *const u8 as *const libc::c_char,
    b"300\0" as *const u8 as *const libc::c_char,
    b"480\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
#[no_mangle]
pub static mut x264_cqm_names: [*const libc::c_char; 3] = [
    b"flat\0" as *const u8 as *const libc::c_char,
    b"jvt\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
#[no_mangle]
pub static mut x264_log_level_names: [*const libc::c_char; 6] = [
    b"none\0" as *const u8 as *const libc::c_char,
    b"error\0" as *const u8 as *const libc::c_char,
    b"warning\0" as *const u8 as *const libc::c_char,
    b"info\0" as *const u8 as *const libc::c_char,
    b"debug\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
#[no_mangle]
pub static mut x264_partition_names: [*const libc::c_char; 8] = [
    b"p8x8\0" as *const u8 as *const libc::c_char,
    b"p4x4\0" as *const u8 as *const libc::c_char,
    b"b8x8\0" as *const u8 as *const libc::c_char,
    b"i8x8\0" as *const u8 as *const libc::c_char,
    b"i4x4\0" as *const u8 as *const libc::c_char,
    b"none\0" as *const u8 as *const libc::c_char,
    b"all\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
#[no_mangle]
pub static mut x264_pulldown_names: [*const libc::c_char; 8] = [
    b"none\0" as *const u8 as *const libc::c_char,
    b"22\0" as *const u8 as *const libc::c_char,
    b"32\0" as *const u8 as *const libc::c_char,
    b"64\0" as *const u8 as *const libc::c_char,
    b"double\0" as *const u8 as *const libc::c_char,
    b"triple\0" as *const u8 as *const libc::c_char,
    b"euro\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
#[no_mangle]
pub static mut x264_range_names: [*const libc::c_char; 4] = [
    b"auto\0" as *const u8 as *const libc::c_char,
    b"tv\0" as *const u8 as *const libc::c_char,
    b"pc\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
#[no_mangle]
pub static mut x264_output_csp_names: [*const libc::c_char; 6] = [
    b"i400\0" as *const u8 as *const libc::c_char,
    b"i420\0" as *const u8 as *const libc::c_char,
    b"i422\0" as *const u8 as *const libc::c_char,
    b"i444\0" as *const u8 as *const libc::c_char,
    b"rgb\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
#[no_mangle]
pub static mut x264_valid_profile_names: [*const libc::c_char; 6] = [
    b"baseline\0" as *const u8 as *const libc::c_char,
    b"main\0" as *const u8 as *const libc::c_char,
    b"high\0" as *const u8 as *const libc::c_char,
    b"high422\0" as *const u8 as *const libc::c_char,
    b"high444\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
#[no_mangle]
pub static mut x264_demuxer_names: [*const libc::c_char; 4] = [
    b"auto\0" as *const u8 as *const libc::c_char,
    b"raw\0" as *const u8 as *const libc::c_char,
    b"y4m\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
#[no_mangle]
pub static mut x264_muxer_names: [*const libc::c_char; 5] = [
    b"auto\0" as *const u8 as *const libc::c_char,
    b"raw\0" as *const u8 as *const libc::c_char,
    b"mkv\0" as *const u8 as *const libc::c_char,
    b"flv\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut chroma_format_names: [*const libc::c_char; 13] = [
    b"all\0" as *const u8 as *const libc::c_char,
    b"i400\0" as *const u8 as *const libc::c_char,
    b"i420\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    b"i422\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    b"i444\0" as *const u8 as *const libc::c_char,
];
static mut pulldown_values: [cli_pulldown_t; 7] = [
    cli_pulldown_t {
        mod_0: 0,
        pattern: [0; 24],
        fps_factor: 0.,
    },
    {
        let mut init = cli_pulldown_t {
            mod_0: 1 as libc::c_int,
            pattern: [
                PIC_STRUCT_TOP_BOTTOM as libc::c_int as uint8_t,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            fps_factor: 1.0f64 as libc::c_float,
        };
        init
    },
    {
        let mut init = cli_pulldown_t {
            mod_0: 4 as libc::c_int,
            pattern: [
                PIC_STRUCT_TOP_BOTTOM_TOP as libc::c_int as uint8_t,
                PIC_STRUCT_BOTTOM_TOP as libc::c_int as uint8_t,
                PIC_STRUCT_BOTTOM_TOP_BOTTOM as libc::c_int as uint8_t,
                PIC_STRUCT_TOP_BOTTOM as libc::c_int as uint8_t,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            fps_factor: 1.25f64 as libc::c_float,
        };
        init
    },
    {
        let mut init = cli_pulldown_t {
            mod_0: 2 as libc::c_int,
            pattern: [
                PIC_STRUCT_DOUBLE as libc::c_int as uint8_t,
                PIC_STRUCT_TRIPLE as libc::c_int as uint8_t,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            fps_factor: 1.0f64 as libc::c_float,
        };
        init
    },
    {
        let mut init = cli_pulldown_t {
            mod_0: 1 as libc::c_int,
            pattern: [
                PIC_STRUCT_DOUBLE as libc::c_int as uint8_t,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            fps_factor: 2.0f64 as libc::c_float,
        };
        init
    },
    {
        let mut init = cli_pulldown_t {
            mod_0: 1 as libc::c_int,
            pattern: [
                PIC_STRUCT_TRIPLE as libc::c_int as uint8_t,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            fps_factor: 3.0f64 as libc::c_float,
        };
        init
    },
    {
        let mut init = cli_pulldown_t {
            mod_0: 24 as libc::c_int,
            pattern: [
                PIC_STRUCT_TOP_BOTTOM_TOP as libc::c_int as uint8_t,
                PIC_STRUCT_BOTTOM_TOP as libc::c_int as uint8_t,
                PIC_STRUCT_BOTTOM_TOP as libc::c_int as uint8_t,
                PIC_STRUCT_BOTTOM_TOP as libc::c_int as uint8_t,
                PIC_STRUCT_BOTTOM_TOP as libc::c_int as uint8_t,
                PIC_STRUCT_BOTTOM_TOP as libc::c_int as uint8_t,
                PIC_STRUCT_BOTTOM_TOP as libc::c_int as uint8_t,
                PIC_STRUCT_BOTTOM_TOP as libc::c_int as uint8_t,
                PIC_STRUCT_BOTTOM_TOP as libc::c_int as uint8_t,
                PIC_STRUCT_BOTTOM_TOP as libc::c_int as uint8_t,
                PIC_STRUCT_BOTTOM_TOP as libc::c_int as uint8_t,
                PIC_STRUCT_BOTTOM_TOP as libc::c_int as uint8_t,
                PIC_STRUCT_BOTTOM_TOP_BOTTOM as libc::c_int as uint8_t,
                PIC_STRUCT_TOP_BOTTOM as libc::c_int as uint8_t,
                PIC_STRUCT_TOP_BOTTOM as libc::c_int as uint8_t,
                PIC_STRUCT_TOP_BOTTOM as libc::c_int as uint8_t,
                PIC_STRUCT_TOP_BOTTOM as libc::c_int as uint8_t,
                PIC_STRUCT_TOP_BOTTOM as libc::c_int as uint8_t,
                PIC_STRUCT_TOP_BOTTOM as libc::c_int as uint8_t,
                PIC_STRUCT_TOP_BOTTOM as libc::c_int as uint8_t,
                PIC_STRUCT_TOP_BOTTOM as libc::c_int as uint8_t,
                PIC_STRUCT_TOP_BOTTOM as libc::c_int as uint8_t,
                PIC_STRUCT_TOP_BOTTOM as libc::c_int as uint8_t,
                PIC_STRUCT_TOP_BOTTOM as libc::c_int as uint8_t,
            ],
            fps_factor: (25.0f64 / 24.0f64) as libc::c_float,
        };
        init
    },
];
static mut pulldown_frame_duration: [libc::c_float; 10] = [
    0.0f64 as libc::c_float,
    1 as libc::c_int as libc::c_float,
    0.5f64 as libc::c_float,
    0.5f64 as libc::c_float,
    1 as libc::c_int as libc::c_float,
    1 as libc::c_int as libc::c_float,
    1.5f64 as libc::c_float,
    1.5f64 as libc::c_float,
    2 as libc::c_int as libc::c_float,
    3 as libc::c_int as libc::c_float,
];
static mut cli_log_level: libc::c_int = 2 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn x264_cli_log(
    mut name: *const libc::c_char,
    mut i_level: libc::c_int,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    if i_level > cli_log_level {
        return;
    }
    let mut s_level: *mut libc::c_char = 0 as *mut libc::c_char;
    match i_level {
        0 => {
            s_level = b"error\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        1 => {
            s_level = b"warning\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        2 => {
            s_level = b"info\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        3 => {
            s_level = b"debug\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        _ => {
            s_level = b"unknown\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
    }
    fprintf(stderr, b"%s [%s]: \0" as *const u8 as *const libc::c_char, name, s_level);
    let mut arg: ::core::ffi::VaListImpl;
    arg = args.clone();
    vfprintf(stderr, fmt, arg.as_va_list());
}
#[no_mangle]
pub unsafe extern "C" fn x264_cli_printf(
    mut i_level: libc::c_int,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    if i_level > cli_log_level {
        return;
    }
    let mut arg: ::core::ffi::VaListImpl;
    arg = args.clone();
    vfprintf(stderr, fmt, arg.as_va_list());
}
unsafe extern "C" fn print_version_info() {
    printf(b"x264 0.164.3204 373697b\n\0" as *const u8 as *const libc::c_char);
    printf(b"built on Jan 14 2025, \0" as *const u8 as *const libc::c_char);
    printf(b"clang: 18.1.3 (1ubuntu1)\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"x264 configuration: --chroma-format=%s\n\0" as *const u8
            as *const libc::c_char,
        chroma_format_names[0 as libc::c_int as usize],
    );
    printf(
        b"libx264 configuration: --chroma-format=%s\n\0" as *const u8
            as *const libc::c_char,
        chroma_format_names[x264_chroma_format as usize],
    );
    printf(b"x264 license: \0" as *const u8 as *const libc::c_char);
    printf(b"GPL version 2 or later\n\0" as *const u8 as *const libc::c_char);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    if argc == 4 as libc::c_int
        && strcmp(
            *argv.offset(1 as libc::c_int as isize),
            b"--autocomplete\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        return x264_cli_autocomplete(
            *argv.offset(2 as libc::c_int as isize),
            *argv.offset(3 as libc::c_int as isize),
        );
    }
    let mut param: x264_param_t = x264_param_t {
        cpu: 0,
        i_threads: 0,
        i_lookahead_threads: 0,
        b_sliced_threads: 0,
        b_deterministic: 0,
        b_cpu_independent: 0,
        i_sync_lookahead: 0,
        i_width: 0,
        i_height: 0,
        i_csp: 0,
        i_bitdepth: 0,
        i_level_idc: 0,
        i_frame_total: 0,
        i_nal_hrd: 0,
        vui: C2RustUnnamed_4 {
            i_sar_height: 0,
            i_sar_width: 0,
            i_overscan: 0,
            i_vidformat: 0,
            b_fullrange: 0,
            i_colorprim: 0,
            i_transfer: 0,
            i_colmatrix: 0,
            i_chroma_loc: 0,
        },
        i_frame_reference: 0,
        i_dpb_size: 0,
        i_keyint_max: 0,
        i_keyint_min: 0,
        i_scenecut_threshold: 0,
        b_intra_refresh: 0,
        i_bframe: 0,
        i_bframe_adaptive: 0,
        i_bframe_bias: 0,
        i_bframe_pyramid: 0,
        b_open_gop: 0,
        b_bluray_compat: 0,
        i_avcintra_class: 0,
        i_avcintra_flavor: 0,
        b_deblocking_filter: 0,
        i_deblocking_filter_alphac0: 0,
        i_deblocking_filter_beta: 0,
        b_cabac: 0,
        i_cabac_init_idc: 0,
        b_interlaced: 0,
        b_constrained_intra: 0,
        i_cqm_preset: 0,
        psz_cqm_file: 0 as *mut libc::c_char,
        cqm_4iy: [0; 16],
        cqm_4py: [0; 16],
        cqm_4ic: [0; 16],
        cqm_4pc: [0; 16],
        cqm_8iy: [0; 64],
        cqm_8py: [0; 64],
        cqm_8ic: [0; 64],
        cqm_8pc: [0; 64],
        pf_log: None,
        p_log_private: 0 as *mut libc::c_void,
        i_log_level: 0,
        b_full_recon: 0,
        psz_dump_yuv: 0 as *mut libc::c_char,
        analyse: C2RustUnnamed_3 {
            intra: 0,
            inter: 0,
            b_transform_8x8: 0,
            i_weighted_pred: 0,
            b_weighted_bipred: 0,
            i_direct_mv_pred: 0,
            i_chroma_qp_offset: 0,
            i_me_method: 0,
            i_me_range: 0,
            i_mv_range: 0,
            i_mv_range_thread: 0,
            i_subpel_refine: 0,
            b_chroma_me: 0,
            b_mixed_references: 0,
            i_trellis: 0,
            b_fast_pskip: 0,
            b_dct_decimate: 0,
            i_noise_reduction: 0,
            f_psy_rd: 0.,
            f_psy_trellis: 0.,
            b_psy: 0,
            b_mb_info: 0,
            b_mb_info_update: 0,
            i_luma_deadzone: [0; 2],
            b_psnr: 0,
            b_ssim: 0,
        },
        rc: C2RustUnnamed_2 {
            i_rc_method: 0,
            i_qp_constant: 0,
            i_qp_min: 0,
            i_qp_max: 0,
            i_qp_step: 0,
            i_bitrate: 0,
            f_rf_constant: 0.,
            f_rf_constant_max: 0.,
            f_rate_tolerance: 0.,
            i_vbv_max_bitrate: 0,
            i_vbv_buffer_size: 0,
            f_vbv_buffer_init: 0.,
            f_ip_factor: 0.,
            f_pb_factor: 0.,
            b_filler: 0,
            i_aq_mode: 0,
            f_aq_strength: 0.,
            b_mb_tree: 0,
            i_lookahead: 0,
            b_stat_write: 0,
            psz_stat_out: 0 as *mut libc::c_char,
            b_stat_read: 0,
            psz_stat_in: 0 as *mut libc::c_char,
            f_qcompress: 0.,
            f_qblur: 0.,
            f_complexity_blur: 0.,
            zones: 0 as *mut x264_zone_t,
            i_zones: 0,
            psz_zones: 0 as *mut libc::c_char,
        },
        crop_rect: C2RustUnnamed_1 {
            i_left: 0,
            i_top: 0,
            i_right: 0,
            i_bottom: 0,
        },
        i_frame_packing: 0,
        mastering_display: C2RustUnnamed_0 {
            b_mastering_display: 0,
            i_green_x: 0,
            i_green_y: 0,
            i_blue_x: 0,
            i_blue_y: 0,
            i_red_x: 0,
            i_red_y: 0,
            i_white_x: 0,
            i_white_y: 0,
            i_display_max: 0,
            i_display_min: 0,
        },
        content_light_level: C2RustUnnamed {
            b_cll: 0,
            i_max_cll: 0,
            i_max_fall: 0,
        },
        i_alternative_transfer: 0,
        b_aud: 0,
        b_repeat_headers: 0,
        b_annexb: 0,
        i_sps_id: 0,
        b_vfr_input: 0,
        b_pulldown: 0,
        i_fps_num: 0,
        i_fps_den: 0,
        i_timebase_num: 0,
        i_timebase_den: 0,
        b_tff: 0,
        b_pic_struct: 0,
        b_fake_interlaced: 0,
        b_stitchable: 0,
        b_opencl: 0,
        i_opencl_device: 0,
        opencl_device_id: 0 as *mut libc::c_void,
        psz_clbin_file: 0 as *mut libc::c_char,
        i_slice_max_size: 0,
        i_slice_max_mbs: 0,
        i_slice_min_mbs: 0,
        i_slice_count: 0,
        i_slice_count_max: 0,
        param_free: None,
        nalu_process: None,
        opaque: 0 as *mut libc::c_void,
    };
    let mut opt: cli_opt_t = {
        let mut init = cli_opt_t {
            b_progress: 0 as libc::c_int,
            i_seek: 0,
            hin: 0 as *mut libc::c_void,
            hout: 0 as *mut libc::c_void,
            qpfile: 0 as *mut FILE,
            tcfile_out: 0 as *mut FILE,
            timebase_convert_multiplier: 0.,
            i_pulldown: 0,
        };
        init
    };
    let mut ret: libc::c_int = 0 as libc::c_int;
    x264_param_default(&mut param);
    if parse(argc, argv, &mut param, &mut opt) < 0 as libc::c_int {
        ret = -(1 as libc::c_int);
    }
    signal(
        2 as libc::c_int,
        Some(sigint_handler as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    if ret == 0 {
        ret = encode(&mut param, &mut opt);
    }
    if (filter.free).is_some() {
        (filter.free).expect("non-null function pointer")(opt.hin);
    } else if !(opt.hin).is_null() {
        (cli_input.close_file).expect("non-null function pointer")(opt.hin);
    }
    if !(opt.hout).is_null() {
        (cli_output.close_file)
            .expect(
                "non-null function pointer",
            )(opt.hout, 0 as libc::c_int as int64_t, 0 as libc::c_int as int64_t);
    }
    if !(opt.tcfile_out).is_null() {
        fclose(opt.tcfile_out);
    }
    if !(opt.qpfile).is_null() {
        fclose(opt.qpfile);
    }
    x264_param_cleanup(&mut param);
    return ret;
}
unsafe extern "C" fn strtable_lookup(
    mut table: *const *const libc::c_char,
    mut idx: libc::c_int,
) -> *const libc::c_char {
    let mut i: libc::c_int = 0 as libc::c_int;
    while !(*table.offset(i as isize)).is_null() {
        i += 1;
        i;
    }
    return if idx >= 0 as libc::c_int && idx < i
        && **table.offset(idx as isize) as libc::c_int != 0
    {
        *table.offset(idx as isize)
    } else {
        b"???\0" as *const u8 as *const libc::c_char
    };
}
unsafe extern "C" fn stringify_names(
    mut buf: *mut libc::c_char,
    mut names: *const *const libc::c_char,
) -> *mut libc::c_char {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut p: *mut libc::c_char = buf;
    *p.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
    while !(*names.offset(i as isize)).is_null() {
        if **names.offset(i as isize) != 0 {
            if p != buf {
                p = p
                    .offset(
                        sprintf(p, b", \0" as *const u8 as *const libc::c_char) as isize,
                    );
            }
            p = p
                .offset(
                    sprintf(
                        p,
                        b"%s\0" as *const u8 as *const libc::c_char,
                        *names.offset(i as isize),
                    ) as isize,
                );
        }
        i += 1;
        i;
    }
    return buf;
}
unsafe extern "C" fn print_csp_name_internal(
    mut name: *const libc::c_char,
    mut line_len: *mut size_t,
    mut last: libc::c_int,
) {
    if !name.is_null() {
        let mut name_len: size_t = strlen(name);
        if (*line_len).wrapping_add(name_len)
            > (80 as libc::c_int - 2 as libc::c_int) as size_t
        {
            printf(
                b"\n                                \0" as *const u8
                    as *const libc::c_char,
            );
            *line_len = 32 as libc::c_int as size_t;
        }
        printf(b"%s\0" as *const u8 as *const libc::c_char, name);
        *line_len = (*line_len).wrapping_add(name_len);
        if last == 0 {
            printf(b", \0" as *const u8 as *const libc::c_char);
            *line_len = (*line_len).wrapping_add(2 as libc::c_int as size_t);
        }
    }
}
unsafe extern "C" fn print_csp_names(mut longhelp: libc::c_int) {
    if longhelp < 2 as libc::c_int {
        return;
    }
    printf(
        b"                              - valid csps for `raw' demuxer:\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(b"                                \0" as *const u8 as *const libc::c_char);
    let mut line_len: size_t = 32 as libc::c_int as size_t;
    let mut i: libc::c_int = 0 as libc::c_int + 1 as libc::c_int;
    while i < 0x11 as libc::c_int {
        print_csp_name_internal(
            (*x264_cli_csps.as_ptr().offset(i as isize)).name,
            &mut line_len,
            (i == 0x11 as libc::c_int - 1 as libc::c_int) as libc::c_int,
        );
        i += 1;
        i;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn help(mut defaults: *mut x264_param_t, mut longhelp: libc::c_int) {
    let mut buf: [libc::c_char; 200] = [0; 200];
    printf(
        b"x264 core:%d%s\nSyntax: x264 [options] -o outfile infile\n\nInfile can be raw (in which case resolution is required),\n  or YUV4MPEG (*.y4m),\n  or Avisynth if compiled with support (%s).\n  or libav* formats if compiled with lavf support (%s) or ffms support (%s).\nOutfile type is selected by filename:\n .264 -> Raw bytestream\n .mkv -> Matroska\n .flv -> Flash Video\n .mp4 -> MP4 if compiled with GPAC or L-SMASH support (%s)\nOutput bit depth: %s\n\nOptions:\n\n  -h, --help                  List basic options\n      --longhelp              List more options\n      --fullhelp              List all options\n\n\0"
            as *const u8 as *const libc::c_char,
        164 as libc::c_int,
        b" r3204 373697b\0" as *const u8 as *const libc::c_char,
        b"no\0" as *const u8 as *const libc::c_char,
        b"no\0" as *const u8 as *const libc::c_char,
        b"no\0" as *const u8 as *const libc::c_char,
        b"no\0" as *const u8 as *const libc::c_char,
        b"8\0" as *const u8 as *const libc::c_char,
    );
    printf(b"Example usage:\n\0" as *const u8 as *const libc::c_char);
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"      Constant quality mode:\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"            x264 --crf 24 -o <output> <input>\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"      Two-pass with a bitrate of 1000kbps:\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"            x264 --pass 1 --bitrate 1000 -o <output> <input>\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"            x264 --pass 2 --bitrate 1000 -o <output> <input>\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"      Lossless:\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"            x264 --qp 0 -o <output> <input>\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"      Maximum PSNR at the cost of speed and visual quality:\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"            x264 --preset placebo --tune psnr -o <output> <input>\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"      Constant bitrate at 1000kbps with a 2 second-buffer:\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"            x264 --vbv-bufsize 2000 --bitrate 1000 -o <output> <input>\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"Presets:\n\0" as *const u8 as *const libc::c_char);
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"      --profile <string>      Force the limits of an H.264 profile\n                                  Overrides all settings.\n\0"
            as *const u8 as *const libc::c_char,
    );
    if longhelp == 2 as libc::c_int {
        printf(
            b"                                  - baseline:\n                                    --no-8x8dct --bframes 0 --no-cabac\n                                    --cqm flat --weightp 0\n                                    No interlaced.\n                                    No lossless.\n                                  - main:\n                                    --no-8x8dct --cqm flat\n                                    No lossless.\n                                  - high:\n                                    No lossless.\n                                  - high422:\n                                    No lossless.\n                                    Support for bit depth 8-10.\n                                    Support for 4:2:0/4:2:2 chroma subsampling.\n                                  - high444:\n                                    Support for bit depth 8-10.\n                                    Support for 4:2:0/4:2:2/4:4:4 chroma subsampling.\n\0"
                as *const u8 as *const libc::c_char,
        );
    } else {
        printf(
            b"                                  - %s\n\0" as *const u8
                as *const libc::c_char,
            stringify_names(buf.as_mut_ptr(), x264_valid_profile_names.as_ptr()),
        );
    }
    printf(
        b"      --preset <string>       Use a preset to select encoding settings [medium]\n                                  Overridden by user settings.\n\0"
            as *const u8 as *const libc::c_char,
    );
    if longhelp == 2 as libc::c_int {
        printf(
            b"                                  - ultrafast:\n                                    --no-8x8dct --aq-mode 0 --b-adapt 0\n                                    --bframes 0 --no-cabac --no-deblock\n                                    --no-mbtree --me dia --no-mixed-refs\n                                    --partitions none --rc-lookahead 0 --ref 1\n                                    --scenecut 0 --subme 0 --trellis 0\n                                    --no-weightb --weightp 0\n                                  - superfast:\n                                    --no-mbtree --me dia --no-mixed-refs\n                                    --partitions i8x8,i4x4 --rc-lookahead 0\n                                    --ref 1 --subme 1 --trellis 0 --weightp 1\n                                  - veryfast:\n                                    --no-mixed-refs --rc-lookahead 10\n                                    --ref 1 --subme 2 --trellis 0 --weightp 1\n                                  - faster:\n                                    --no-mixed-refs --rc-lookahead 20\n                                    --ref 2 --subme 4 --weightp 1\n                                  - fast:\n                                    --rc-lookahead 30 --ref 2 --subme 6\n                                    --weightp 1\n                                  - medium:\n                                    Default settings apply.\n                                  - slow:\n                                    --direct auto --rc-lookahead 50 --ref 5\n                                    --subme 8 --trellis 2\n                                  - slower:\n                                    --b-adapt 2 --direct auto --me umh\n                                    --partitions all --rc-lookahead 60\n                                    --ref 8 --subme 9 --trellis 2\n                                  - veryslow:\n                                    --b-adapt 2 --bframes 8 --direct auto\n                                    --me umh --merange 24 --partitions all\n                                    --ref 16 --subme 10 --trellis 2\n                                    --rc-lookahead 60\n                                  - placebo:\n                                    --bframes 16 --b-adapt 2 --direct auto\n                                    --slow-firstpass --no-fast-pskip\n                                    --me tesa --merange 24 --partitions all\n                                    --rc-lookahead 60 --ref 16 --subme 11\n                                    --trellis 2\n\0"
                as *const u8 as *const libc::c_char,
        );
    } else {
        printf(
            b"                                  - ultrafast,superfast,veryfast,faster,fast\n                                  - medium,slow,slower,veryslow,placebo\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    printf(
        b"      --tune <string>         Tune the settings for a particular type of source\n                              or situation\n                                  Overridden by user settings.\n                                  Multiple tunings are separated by commas.\n                                  Only one psy tuning can be used at a time.\n\0"
            as *const u8 as *const libc::c_char,
    );
    if longhelp == 2 as libc::c_int {
        printf(
            b"                                  - film (psy tuning):\n                                    --deblock -1:-1 --psy-rd <unset>:0.15\n                                  - animation (psy tuning):\n                                    --bframes {+2} --deblock 1:1\n                                    --psy-rd 0.4:<unset> --aq-strength 0.6\n                                    --ref {Double if >1 else 1}\n                                  - grain (psy tuning):\n                                    --aq-strength 0.5 --no-dct-decimate\n                                    --deadzone-inter 6 --deadzone-intra 6\n                                    --deblock -2:-2 --ipratio 1.1\n                                    --pbratio 1.1 --psy-rd <unset>:0.25\n                                    --qcomp 0.8\n                                  - stillimage (psy tuning):\n                                    --aq-strength 1.2 --deblock -3:-3\n                                    --psy-rd 2.0:0.7\n                                  - psnr (psy tuning):\n                                    --aq-mode 0 --no-psy\n                                  - ssim (psy tuning):\n                                    --aq-mode 2 --no-psy\n                                  - fastdecode:\n                                    --no-cabac --no-deblock --no-weightb\n                                    --weightp 0\n                                  - zerolatency:\n                                    --bframes 0 --force-cfr --no-mbtree\n                                    --sync-lookahead 0 --sliced-threads\n                                    --rc-lookahead 0\n\0"
                as *const u8 as *const libc::c_char,
        );
    } else {
        printf(
            b"                                  - psy tunings: film,animation,grain,\n                                                 stillimage,psnr,ssim\n                                  - other tunings: fastdecode,zerolatency\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --slow-firstpass        Don't force these faster settings with --pass 1:\n                                  --no-8x8dct --me dia --partitions none\n                                  --ref 1 --subme {2 if >2 else unchanged}\n                                  --trellis 0 --fast-pskip\n\0"
                as *const u8 as *const libc::c_char,
        );
    } else if longhelp >= 1 as libc::c_int {
        printf(
            b"      --slow-firstpass        Don't force faster settings with --pass 1\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"Frame-type options:\n\0" as *const u8 as *const libc::c_char);
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"  -I, --keyint <integer or \"infinite\"> Maximum GOP size [%d]\n\0"
            as *const u8 as *const libc::c_char,
        (*defaults).i_keyint_max,
    );
    if longhelp == 2 as libc::c_int {
        printf(
            b"  -i, --min-keyint <integer>  Minimum GOP size [auto]\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --no-scenecut           Disable adaptive I-frame decision\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --scenecut <integer>    How aggressively to insert extra I-frames [%d]\n\0"
                as *const u8 as *const libc::c_char,
            (*defaults).i_scenecut_threshold,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --intra-refresh         Use Periodic Intra Refresh instead of IDR frames\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if longhelp >= 1 as libc::c_int {
        printf(
            b"  -b, --bframes <integer>     Number of B-frames between I and P [%d]\n\0"
                as *const u8 as *const libc::c_char,
            (*defaults).i_bframe,
        );
    }
    if longhelp >= 1 as libc::c_int {
        printf(
            b"      --b-adapt <integer>     Adaptive B-frame decision method [%d]\n                                  Higher values may lower threading efficiency.\n                                  - 0: Disabled\n                                  - 1: Fast\n                                  - 2: Optimal (slow with high --bframes)\n\0"
                as *const u8 as *const libc::c_char,
            (*defaults).i_bframe_adaptive,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --b-bias <integer>      Influences how often B-frames are used [%d]\n\0"
                as *const u8 as *const libc::c_char,
            (*defaults).i_bframe_bias,
        );
    }
    if longhelp >= 1 as libc::c_int {
        printf(
            b"      --b-pyramid <string>    Keep some B-frames as references [%s]\n                                  - none: Disabled\n                                  - strict: Strictly hierarchical pyramid\n                                  - normal: Non-strict (not Blu-ray compatible)\n\0"
                as *const u8 as *const libc::c_char,
            strtable_lookup(x264_b_pyramid_names.as_ptr(), (*defaults).i_bframe_pyramid),
        );
    }
    if longhelp >= 1 as libc::c_int {
        printf(
            b"      --open-gop              Use recovery points to close GOPs\n                              Only available with b-frames\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if longhelp >= 1 as libc::c_int {
        printf(
            b"      --no-cabac              Disable CABAC\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if longhelp >= 1 as libc::c_int {
        printf(
            b"  -r, --ref <integer>         Number of reference frames [%d]\n\0"
                as *const u8 as *const libc::c_char,
            (*defaults).i_frame_reference,
        );
    }
    if longhelp >= 1 as libc::c_int {
        printf(
            b"      --no-deblock            Disable loop filter\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if longhelp >= 1 as libc::c_int {
        printf(
            b"  -f, --deblock <alpha:beta>  Loop filter parameters [%d:%d]\n\0"
                as *const u8 as *const libc::c_char,
            (*defaults).i_deblocking_filter_alphac0,
            (*defaults).i_deblocking_filter_beta,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --slices <integer>      Number of slices per frame; forces rectangular\n                              slices and is overridden by other slicing options\n\0"
                as *const u8 as *const libc::c_char,
        );
    } else if longhelp >= 1 as libc::c_int {
        printf(
            b"      --slices <integer>      Number of slices per frame\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --slices-max <integer>  Absolute maximum slices per frame; overrides\n                              slice-max-size/slice-max-mbs when necessary\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --slice-max-size <integer> Limit the size of each slice in bytes\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --slice-max-mbs <integer> Limit the size of each slice in macroblocks (max)\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --slice-min-mbs <integer> Limit the size of each slice in macroblocks (min)\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    printf(
        b"      --tff                   Enable interlaced mode (top field first)\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"      --bff                   Enable interlaced mode (bottom field first)\n\0"
            as *const u8 as *const libc::c_char,
    );
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --constrained-intra     Enable constrained intra prediction.\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    printf(
        b"      --pulldown <string>     Use soft pulldown to change frame rate\n                                  - %s (requires cfr input)\n\0"
            as *const u8 as *const libc::c_char,
        stringify_names(buf.as_mut_ptr(), x264_pulldown_names.as_ptr()),
    );
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --fake-interlaced       Flag stream as interlaced but encode progressive.\n                              Makes it possible to encode 25p and 30p Blu-Ray\n                              streams. Ignored in interlaced mode.\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --frame-packing <integer> For stereoscopic videos define frame arrangement\n                                  - 0: checkerboard - pixels are alternatively from L and R\n                                  - 1: column alternation - L and R are interlaced by column\n                                  - 2: row alternation - L and R are interlaced by row\n                                  - 3: side by side - L is on the left, R on the right\n                                  - 4: top bottom - L is on top, R on bottom\n                                  - 5: frame alternation - one view per frame\n                                  - 6: mono - 2D frame without any frame packing\n                                  - 7: tile format - L is on top-left, R split across\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"Ratecontrol:\n\0" as *const u8 as *const libc::c_char);
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    if longhelp >= 1 as libc::c_int {
        printf(
            b"  -q, --qp <integer>          Force constant QP (0-%d, 0=lossless)\n\0"
                as *const u8 as *const libc::c_char,
            51 as libc::c_int + 6 as libc::c_int * 2 as libc::c_int + 18 as libc::c_int,
        );
    }
    printf(
        b"  -B, --bitrate <integer>     Set bitrate (kbit/s)\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"      --crf <float>           Quality-based VBR (%d-51) [%.1f]\n\0"
            as *const u8 as *const libc::c_char,
        51 as libc::c_int - (51 as libc::c_int + 6 as libc::c_int * 2 as libc::c_int),
        (*defaults).rc.f_rf_constant as libc::c_double,
    );
    if longhelp >= 1 as libc::c_int {
        printf(
            b"      --rc-lookahead <integer> Number of frames for frametype lookahead [%d]\n\0"
                as *const u8 as *const libc::c_char,
            (*defaults).rc.i_lookahead,
        );
    }
    printf(
        b"      --vbv-maxrate <integer> Max local bitrate (kbit/s) [%d]\n\0" as *const u8
            as *const libc::c_char,
        (*defaults).rc.i_vbv_max_bitrate,
    );
    printf(
        b"      --vbv-bufsize <integer> Set size of the VBV buffer (kbit) [%d]\n\0"
            as *const u8 as *const libc::c_char,
        (*defaults).rc.i_vbv_buffer_size,
    );
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --vbv-init <float>      Initial VBV buffer occupancy [%.1f]\n\0"
                as *const u8 as *const libc::c_char,
            (*defaults).rc.f_vbv_buffer_init as libc::c_double,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --crf-max <float>       With CRF+VBV, limit RF to this value\n                                  May cause VBV underflows!\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --qpmin <integer>       Set min QP [%d]\n\0" as *const u8
                as *const libc::c_char,
            (*defaults).rc.i_qp_min,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --qpmax <integer>       Set max QP [%d]\n\0" as *const u8
                as *const libc::c_char,
            if (*defaults).rc.i_qp_max
                < 51 as libc::c_int + 6 as libc::c_int * 2 as libc::c_int
                    + 18 as libc::c_int
            {
                (*defaults).rc.i_qp_max
            } else {
                51 as libc::c_int + 6 as libc::c_int * 2 as libc::c_int
                    + 18 as libc::c_int
            },
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --qpstep <integer>      Set max QP step [%d]\n\0" as *const u8
                as *const libc::c_char,
            (*defaults).rc.i_qp_step,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --ratetol <float>       Tolerance of ABR ratecontrol and VBV [%.1f]\n\0"
                as *const u8 as *const libc::c_char,
            (*defaults).rc.f_rate_tolerance as libc::c_double,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --ipratio <float>       QP factor between I and P [%.2f]\n\0"
                as *const u8 as *const libc::c_char,
            (*defaults).rc.f_ip_factor as libc::c_double,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --pbratio <float>       QP factor between P and B [%.2f]\n\0"
                as *const u8 as *const libc::c_char,
            (*defaults).rc.f_pb_factor as libc::c_double,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --chroma-qp-offset <integer>  QP difference between chroma and luma [%d]\n\0"
                as *const u8 as *const libc::c_char,
            (*defaults).analyse.i_chroma_qp_offset,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --aq-mode <integer>     AQ method [%d]\n                                  - 0: Disabled\n                                  - 1: Variance AQ (complexity mask)\n                                  - 2: Auto-variance AQ\n                                  - 3: Auto-variance AQ with bias to dark scenes\n\0"
                as *const u8 as *const libc::c_char,
            (*defaults).rc.i_aq_mode,
        );
    }
    if longhelp >= 1 as libc::c_int {
        printf(
            b"      --aq-strength <float>   Reduces blocking and blurring in flat and\n                              textured areas. [%.1f]\n\0"
                as *const u8 as *const libc::c_char,
            (*defaults).rc.f_aq_strength as libc::c_double,
        );
    }
    if longhelp >= 1 as libc::c_int {
        printf(b"\n\0" as *const u8 as *const libc::c_char);
    }
    printf(
        b"  -p, --pass <integer>        Enable multipass ratecontrol\n                                  - 1: First pass, creates stats file\n                                  - 2: Last pass, does not overwrite stats file\n\0"
            as *const u8 as *const libc::c_char,
    );
    if longhelp == 2 as libc::c_int {
        printf(
            b"                                  - 3: Nth pass, overwrites stats file\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if longhelp >= 1 as libc::c_int {
        printf(
            b"      --stats <string>        Filename for 2 pass stats [\"%s\"]\n\0"
                as *const u8 as *const libc::c_char,
            (*defaults).rc.psz_stat_out,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --no-mbtree             Disable mb-tree ratecontrol.\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --qcomp <float>         QP curve compression [%.2f]\n\0" as *const u8
                as *const libc::c_char,
            (*defaults).rc.f_qcompress as libc::c_double,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --cplxblur <float>      Reduce fluctuations in QP (before curve compression) [%.1f]\n\0"
                as *const u8 as *const libc::c_char,
            (*defaults).rc.f_complexity_blur as libc::c_double,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --qblur <float>         Reduce fluctuations in QP (after curve compression) [%.1f]\n\0"
                as *const u8 as *const libc::c_char,
            (*defaults).rc.f_qblur as libc::c_double,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --zones <zone0>/<zone1>/...  Tweak the bitrate of regions of the video\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"                              Each zone is of the form\n                                  <start frame>,<end frame>,<option>\n                                  where <option> is either\n                                      q=<integer> (force QP)\n                                  or  b=<float> (bitrate multiplier)\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --qpfile <string>       Force frametypes and QPs for some or all frames\n                              Format of each line: framenumber frametype QP\n                              QP is optional (none lets x264 choose). Frametypes: I,i,K,P,B,b.\n                                  K=<I or i> depending on open-gop setting\n                              QPs are restricted by qpmin/qpmax.\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if longhelp >= 1 as libc::c_int {
        printf(b"\n\0" as *const u8 as *const libc::c_char);
    }
    if longhelp >= 1 as libc::c_int {
        printf(b"Analysis:\n\0" as *const u8 as *const libc::c_char);
    }
    if longhelp >= 1 as libc::c_int {
        printf(b"\n\0" as *const u8 as *const libc::c_char);
    }
    if longhelp >= 1 as libc::c_int {
        printf(
            b"  -A, --partitions <string>   Partitions to consider [\"p8x8,b8x8,i8x8,i4x4\"]\n                                  - %s\n                                  (p4x4 requires p8x8. i8x8 requires --8x8dct.)\n\0"
                as *const u8 as *const libc::c_char,
            stringify_names(buf.as_mut_ptr(), x264_partition_names.as_ptr()),
        );
    }
    if longhelp >= 1 as libc::c_int {
        printf(
            b"      --direct <string>       Direct MV prediction mode [\"%s\"]\n                                  - none, spatial, temporal, auto\n\0"
                as *const u8 as *const libc::c_char,
            strtable_lookup(
                x264_direct_pred_names.as_ptr(),
                (*defaults).analyse.i_direct_mv_pred,
            ),
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --no-weightb            Disable weighted prediction for B-frames\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if longhelp >= 1 as libc::c_int {
        printf(
            b"      --weightp <integer>     Weighted prediction for P-frames [%d]\n                                  - 0: Disabled\n                                  - 1: Weighted refs\n                                  - 2: Weighted refs + Duplicates\n\0"
                as *const u8 as *const libc::c_char,
            (*defaults).analyse.i_weighted_pred,
        );
    }
    if longhelp >= 1 as libc::c_int {
        printf(
            b"      --me <string>           Integer pixel motion estimation method [\"%s\"]\n\0"
                as *const u8 as *const libc::c_char,
            strtable_lookup(
                x264_motion_est_names.as_ptr(),
                (*defaults).analyse.i_me_method,
            ),
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"                                  - dia: diamond search, radius 1 (fast)\n                                  - hex: hexagonal search, radius 2\n                                  - umh: uneven multi-hexagon search\n                                  - esa: exhaustive search\n                                  - tesa: hadamard exhaustive search (slow)\n\0"
                as *const u8 as *const libc::c_char,
        );
    } else if longhelp >= 1 as libc::c_int {
        printf(
            b"                                  - dia, hex, umh\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --merange <integer>     Maximum motion vector search range [%d]\n\0"
                as *const u8 as *const libc::c_char,
            (*defaults).analyse.i_me_range,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --mvrange <integer>     Maximum motion vector length [-1 (auto)]\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --mvrange-thread <int>  Minimum buffer between threads [-1 (auto)]\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if longhelp >= 1 as libc::c_int {
        printf(
            b"  -m, --subme <integer>       Subpixel motion estimation and mode decision [%d]\n\0"
                as *const u8 as *const libc::c_char,
            (*defaults).analyse.i_subpel_refine,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"                                  - 0: fullpel only (not recommended)\n                                  - 1: SAD mode decision, one qpel iteration\n                                  - 2: SATD mode decision\n                                  - 3-5: Progressively more qpel\n                                  - 6: RD mode decision for I/P-frames\n                                  - 7: RD mode decision for all frames\n                                  - 8: RD refinement for I/P-frames\n                                  - 9: RD refinement for all frames\n                                  - 10: QP-RD - requires trellis=2, aq-mode>0\n                                  - 11: Full RD: disable all early terminations\n\0"
                as *const u8 as *const libc::c_char,
        );
    } else if longhelp >= 1 as libc::c_int {
        printf(
            b"                                  decision quality: 1=fast, 11=best\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if longhelp >= 1 as libc::c_int {
        printf(
            b"      --psy-rd <float:float>  Strength of psychovisual optimization [\"%.1f:%.1f\"]\n                                  #1: RD (requires subme>=6)\n                                  #2: Trellis (requires trellis, experimental)\n\0"
                as *const u8 as *const libc::c_char,
            (*defaults).analyse.f_psy_rd as libc::c_double,
            (*defaults).analyse.f_psy_trellis as libc::c_double,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --no-psy                Disable all visual optimizations that worsen\n                              both PSNR and SSIM.\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --no-mixed-refs         Don't decide references on a per partition basis\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --no-chroma-me          Ignore chroma in motion estimation\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if longhelp >= 1 as libc::c_int {
        printf(
            b"      --no-8x8dct             Disable adaptive spatial transform size\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if longhelp >= 1 as libc::c_int {
        printf(
            b"  -t, --trellis <integer>     Trellis RD quantization. [%d]\n                                  - 0: disabled\n                                  - 1: enabled only on the final encode of a MB\n                                  - 2: enabled on all mode decisions\n\0"
                as *const u8 as *const libc::c_char,
            (*defaults).analyse.i_trellis,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --no-fast-pskip         Disables early SKIP detection on P-frames\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --no-dct-decimate       Disables coefficient thresholding on P-frames\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if longhelp >= 1 as libc::c_int {
        printf(
            b"      --nr <integer>          Noise reduction [%d]\n\0" as *const u8
                as *const libc::c_char,
            (*defaults).analyse.i_noise_reduction,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(b"\n\0" as *const u8 as *const libc::c_char);
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --deadzone-inter <int>  Set the size of the inter luma quantization deadzone [%d]\n\0"
                as *const u8 as *const libc::c_char,
            (*defaults).analyse.i_luma_deadzone[0 as libc::c_int as usize],
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --deadzone-intra <int>  Set the size of the intra luma quantization deadzone [%d]\n\0"
                as *const u8 as *const libc::c_char,
            (*defaults).analyse.i_luma_deadzone[1 as libc::c_int as usize],
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"                                  Deadzones should be in the range 0 - 32.\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --cqm <string>          Preset quant matrices [\"%s\"]\n                                  - %s\n\0"
                as *const u8 as *const libc::c_char,
            x264_cqm_names[0 as libc::c_int as usize],
            stringify_names(buf.as_mut_ptr(), x264_cqm_names.as_ptr()),
        );
    }
    if longhelp >= 1 as libc::c_int {
        printf(
            b"      --cqmfile <string>      Read custom quant matrices from a JM-compatible file\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"                                  Overrides any other --cqm* options.\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --cqm4 <list>           Set all 4x4 quant matrices\n                                  Takes a comma-separated list of 16 integers.\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --cqm8 <list>           Set all 8x8 quant matrices\n                                  Takes a comma-separated list of 64 integers.\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --cqm4i, --cqm4p, --cqm8i, --cqm8p <list>\n                              Set both luma and chroma quant matrices\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --cqm4iy, --cqm4ic, --cqm4py, --cqm4pc <list>\n                              Set individual quant matrices\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(b"\n\0" as *const u8 as *const libc::c_char);
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"Video Usability Info (Annex E):\n\0" as *const u8 as *const libc::c_char,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"The VUI settings are not used by the encoder but are merely suggestions to\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"the playback equipment. See doc/vui.txt for details. Use at your own risk.\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(b"\n\0" as *const u8 as *const libc::c_char);
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --overscan <string>     Specify crop overscan setting [\"%s\"]\n                                  - undef, show, crop\n\0"
                as *const u8 as *const libc::c_char,
            strtable_lookup(x264_overscan_names.as_ptr(), (*defaults).vui.i_overscan),
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --videoformat <string>  Specify video format [\"%s\"]\n                                  - component, pal, ntsc, secam, mac, undef\n\0"
                as *const u8 as *const libc::c_char,
            strtable_lookup(x264_vidformat_names.as_ptr(), (*defaults).vui.i_vidformat),
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --range <string>        Specify color range [\"%s\"]\n                                  - %s\n\0"
                as *const u8 as *const libc::c_char,
            x264_range_names[0 as libc::c_int as usize],
            stringify_names(buf.as_mut_ptr(), x264_range_names.as_ptr()),
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --colorprim <string>    Specify color primaries [\"%s\"]\n                                  - undef, bt709, bt470m, bt470bg, smpte170m,\n                                    smpte240m, film, bt2020, smpte428,\n                                    smpte431, smpte432\n\0"
                as *const u8 as *const libc::c_char,
            strtable_lookup(x264_colorprim_names.as_ptr(), (*defaults).vui.i_colorprim),
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --transfer <string>     Specify transfer characteristics [\"%s\"]\n                                  - undef, bt709, bt470m, bt470bg, smpte170m,\n                                    smpte240m, linear, log100, log316,\n                                    iec61966-2-4, bt1361e, iec61966-2-1,\n                                    bt2020-10, bt2020-12, smpte2084, smpte428,\n                                    arib-std-b67\n\0"
                as *const u8 as *const libc::c_char,
            strtable_lookup(x264_transfer_names.as_ptr(), (*defaults).vui.i_transfer),
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --colormatrix <string>  Specify color matrix setting [\"%s\"]\n                                  - undef, bt709, fcc, bt470bg, smpte170m,\n                                    smpte240m, GBR, YCgCo, bt2020nc, bt2020c,\n                                    smpte2085, chroma-derived-nc,\n                                    chroma-derived-c, ICtCp\n\0"
                as *const u8 as *const libc::c_char,
            strtable_lookup(x264_colmatrix_names.as_ptr(), (*defaults).vui.i_colmatrix),
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --chromaloc <integer>   Specify chroma sample location (0 to 5) [%d]\n\0"
                as *const u8 as *const libc::c_char,
            (*defaults).vui.i_chroma_loc,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --mastering-display <string> Specify 'G(x,y)B(x,y)R(x,y)WP(x,y)L(max,min)'\n                              for primaries, white point, and display brightness\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --cll <string>          Specify 'max_content,max_frame_average' content\n                              light levels\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --alternative-transfer <string> Specify an alternative transfer\n                              characteristics [\"%s\"]\n                                  - same values as --transfer\n\0"
                as *const u8 as *const libc::c_char,
            strtable_lookup(
                x264_transfer_names.as_ptr(),
                (*defaults).i_alternative_transfer,
            ),
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --nal-hrd <string>      Signal HRD information (requires vbv-bufsize)\n                                  - none, vbr, cbr (cbr not allowed in .mp4)\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --filler                Force hard-CBR and generate filler (implied by\n                              --nal-hrd cbr)\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --pic-struct            Force pic_struct in Picture Timing SEI\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --crop-rect <string>    Add 'left,top,right,bottom' to the bitstream-level\n                              cropping rectangle\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"Input/Output:\n\0" as *const u8 as *const libc::c_char);
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"  -o, --output <string>       Specify output file\n\0" as *const u8
            as *const libc::c_char,
    );
    if longhelp >= 1 as libc::c_int {
        printf(
            b"      --muxer <string>        Specify output container format [\"%s\"]\n                                  - %s\n\0"
                as *const u8 as *const libc::c_char,
            x264_muxer_names[0 as libc::c_int as usize],
            stringify_names(buf.as_mut_ptr(), x264_muxer_names.as_ptr()),
        );
    }
    if longhelp >= 1 as libc::c_int {
        printf(
            b"      --demuxer <string>      Specify input container format [\"%s\"]\n                                  - %s\n\0"
                as *const u8 as *const libc::c_char,
            x264_demuxer_names[0 as libc::c_int as usize],
            stringify_names(buf.as_mut_ptr(), x264_demuxer_names.as_ptr()),
        );
    }
    if longhelp >= 1 as libc::c_int {
        printf(
            b"      --input-fmt <string>    Specify input file format (requires lavf support)\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if longhelp >= 1 as libc::c_int {
        printf(
            b"      --input-csp <string>    Specify input colorspace format for raw input\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    print_csp_names(longhelp);
    if longhelp >= 1 as libc::c_int {
        printf(
            b"      --output-csp <string>   Specify output colorspace [\"%s\"]\n                                  - %s\n\0"
                as *const u8 as *const libc::c_char,
            b"i420\0" as *const u8 as *const libc::c_char,
            stringify_names(buf.as_mut_ptr(), x264_output_csp_names.as_ptr()),
        );
    }
    if longhelp >= 1 as libc::c_int {
        printf(
            b"      --input-depth <integer> Specify input bit depth for raw input\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if longhelp >= 1 as libc::c_int {
        printf(
            b"      --output-depth <integer> Specify output bit depth\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if longhelp >= 1 as libc::c_int {
        printf(
            b"      --input-range <string>  Specify input color range [\"%s\"]\n                                  - %s\n\0"
                as *const u8 as *const libc::c_char,
            x264_range_names[0 as libc::c_int as usize],
            stringify_names(buf.as_mut_ptr(), x264_range_names.as_ptr()),
        );
    }
    if longhelp >= 1 as libc::c_int {
        printf(
            b"      --input-res <intxint>   Specify input resolution (width x height)\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if longhelp >= 1 as libc::c_int {
        printf(
            b"      --index <string>        Filename for input index file\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    printf(
        b"      --sar width:height      Specify Sample Aspect Ratio\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"      --fps <float|rational>  Specify framerate\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"      --seek <integer>        First frame to encode\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"      --frames <integer>      Maximum number of frames to encode\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"      --level <string>        Specify level (as defined by Annex A)\n\0"
            as *const u8 as *const libc::c_char,
    );
    if longhelp >= 1 as libc::c_int {
        printf(
            b"      --bluray-compat         Enable compatibility hacks for Blu-ray support\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if longhelp >= 1 as libc::c_int {
        printf(
            b"      --avcintra-class <integer> Use compatibility hacks for AVC-Intra class\n                                  - %s\n\0"
                as *const u8 as *const libc::c_char,
            stringify_names(buf.as_mut_ptr(), x264_avcintra_class_names.as_ptr()),
        );
    }
    if longhelp >= 1 as libc::c_int {
        printf(
            b"      --avcintra-flavor <string> AVC-Intra flavor [\"%s\"]\n                                  - %s\n\0"
                as *const u8 as *const libc::c_char,
            x264_avcintra_flavor_names[0 as libc::c_int as usize],
            stringify_names(buf.as_mut_ptr(), x264_avcintra_flavor_names.as_ptr()),
        );
    }
    if longhelp >= 1 as libc::c_int {
        printf(
            b"      --stitchable            Don't optimize headers based on video content\n                              Ensures ability to recombine a segmented encode\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if longhelp >= 1 as libc::c_int {
        printf(b"\n\0" as *const u8 as *const libc::c_char);
    }
    if longhelp >= 1 as libc::c_int {
        printf(
            b"  -v, --verbose               Print stats for each frame\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if longhelp >= 1 as libc::c_int {
        printf(
            b"      --no-progress           Don't show the progress indicator while encoding\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    printf(
        b"      --quiet                 Quiet Mode\n\0" as *const u8
            as *const libc::c_char,
    );
    if longhelp >= 1 as libc::c_int {
        printf(
            b"      --log-level <string>    Specify the maximum level of logging [\"%s\"]\n                                  - %s\n\0"
                as *const u8 as *const libc::c_char,
            strtable_lookup(
                x264_log_level_names.as_ptr(),
                cli_log_level - -(1 as libc::c_int),
            ),
            stringify_names(buf.as_mut_ptr(), x264_log_level_names.as_ptr()),
        );
    }
    if longhelp >= 1 as libc::c_int {
        printf(
            b"      --psnr                  Enable PSNR computation\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if longhelp >= 1 as libc::c_int {
        printf(
            b"      --ssim                  Enable SSIM computation\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if longhelp >= 1 as libc::c_int {
        printf(
            b"      --threads <integer>     Force a specific number of threads\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --lookahead-threads <integer> Force a specific number of lookahead threads\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --sliced-threads        Low-latency but lower-efficiency threading\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --thread-input          Run Avisynth in its own thread\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --sync-lookahead <integer> Number of buffer frames for threaded lookahead\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --non-deterministic     Slightly improve quality of SMP, at the cost of repeatability\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --cpu-independent       Ensure exact reproducibility across different cpus,\n                                  as opposed to letting them select different algorithms\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --asm <integer>         Override CPU detection\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --no-asm                Disable all CPU optimizations\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --opencl                Enable use of OpenCL\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --opencl-clbin <string> Specify path of compiled OpenCL kernel cache\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --opencl-device <integer> Specify OpenCL device ordinal\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --dump-yuv <string>     Save reconstructed frames\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --sps-id <integer>      Set SPS and PPS id numbers [%d]\n\0"
                as *const u8 as *const libc::c_char,
            (*defaults).i_sps_id,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --aud                   Use access unit delimiters\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --force-cfr             Force constant framerate timestamp generation\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --tcfile-in <string>    Force timestamp generation with timecode file\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --tcfile-out <string>   Output timecode v2 file from input timestamps\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --timebase <int/int>    Specify timebase numerator and denominator\n                 <integer>    Specify timebase numerator for input timecode file\n                              or specify timebase denominator for other input\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if longhelp == 2 as libc::c_int {
        printf(
            b"      --dts-compress          Eliminate initial delay with container DTS hack\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"Filtering:\n\0" as *const u8 as *const libc::c_char);
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"      --vf, --video-filter <filter0>/<filter1>/... Apply video filtering to the input file\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"      Filter options may be specified in <filter>:<option>=<value> format.\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"      Available filters:\n\0" as *const u8 as *const libc::c_char);
    x264_register_vid_filters();
    x264_vid_filter_help(longhelp);
    printf(b"\n\0" as *const u8 as *const libc::c_char);
}
static mut short_options: [libc::c_char; 30] = unsafe {
    *::core::mem::transmute::<
        &[u8; 30],
        &mut [libc::c_char; 30],
    >(b"8A:B:b:f:hI:i:m:o:p:q:r:t:Vvw\0")
};
static mut long_options: [option; 169] = [
    {
        let mut init = option {
            name: b"help\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'h' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"longhelp\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPT_LONGHELP as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"fullhelp\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPT_FULLHELP as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"version\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'V' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"profile\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPT_PROFILE as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"preset\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPT_PRESET as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"tune\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPT_TUNE as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"slow-firstpass\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPT_SLOWFIRSTPASS as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"bitrate\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'B' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"bframes\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'b' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"b-adapt\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"no-b-adapt\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"b-bias\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"b-pyramid\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"open-gop\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"bluray-compat\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"avcintra-class\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"avcintra-flavor\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"min-keyint\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'i' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"keyint\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'I' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"intra-refresh\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"scenecut\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"no-scenecut\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"nf\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"no-deblock\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"filter\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"deblock\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'f' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"interlaced\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPT_INTERLACED as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"tff\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPT_INTERLACED as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"bff\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPT_INTERLACED as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"no-interlaced\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPT_INTERLACED as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"constrained-intra\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"cabac\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"no-cabac\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"qp\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'q' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"qpmin\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"qpmax\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"qpstep\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"crf\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"rc-lookahead\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"ref\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'r' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"asm\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"no-asm\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"opencl\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"opencl-clbin\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"opencl-device\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"sar\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"fps\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPT_FPS as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"frames\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPT_FRAMES as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"seek\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPT_SEEK as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"output\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'o' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"muxer\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPT_MUXER as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"demuxer\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPT_DEMUXER as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"stdout\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPT_MUXER as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"stdin\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPT_DEMUXER as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"index\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPT_INDEX as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"analyse\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"partitions\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'A' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"direct\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"weightb\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'w' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"no-weightb\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"weightp\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"me\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"merange\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"mvrange\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"mvrange-thread\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"subme\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'm' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"psy-rd\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"no-psy\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"psy\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"mixed-refs\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"no-mixed-refs\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"no-chroma-me\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"8x8dct\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: '8' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"no-8x8dct\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"trellis\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 't' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"fast-pskip\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"no-fast-pskip\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"no-dct-decimate\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"aq-strength\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"aq-mode\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"deadzone-inter\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"deadzone-intra\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"level\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"ratetol\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"vbv-maxrate\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"vbv-bufsize\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"vbv-init\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"crf-max\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"ipratio\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"pbratio\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"chroma-qp-offset\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"pass\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'p' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"stats\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"qcomp\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"mbtree\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"no-mbtree\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"qblur\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"cplxblur\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"zones\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"qpfile\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPT_QPFILE as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"threads\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"lookahead-threads\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"sliced-threads\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"no-sliced-threads\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"slice-max-size\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"slice-max-mbs\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"slice-min-mbs\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"slices\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"slices-max\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"thread-input\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPT_THREAD_INPUT as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"sync-lookahead\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"non-deterministic\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"cpu-independent\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"psnr\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"ssim\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"quiet\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPT_QUIET as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"verbose\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'v' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"log-level\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPT_LOG_LEVEL as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"no-progress\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPT_NOPROGRESS as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"dump-yuv\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"sps-id\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"aud\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"nr\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"cqm\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"cqmfile\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"cqm4\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"cqm4i\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"cqm4iy\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"cqm4ic\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"cqm4p\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"cqm4py\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"cqm4pc\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"cqm8\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"cqm8i\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"cqm8p\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"overscan\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"videoformat\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"range\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPT_RANGE as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"colorprim\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"transfer\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"colormatrix\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"chromaloc\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"force-cfr\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"tcfile-in\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPT_TCFILE_IN as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"tcfile-out\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPT_TCFILE_OUT as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"timebase\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPT_TIMEBASE as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"pic-struct\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"crop-rect\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"nal-hrd\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"pulldown\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPT_PULLDOWN as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"fake-interlaced\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"frame-packing\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"mastering-display\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"cll\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"alternative-transfer\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"vf\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPT_VIDEO_FILTER as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"video-filter\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPT_VIDEO_FILTER as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"input-fmt\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPT_INPUT_FMT as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"input-res\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPT_INPUT_RES as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"input-csp\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPT_INPUT_CSP as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"input-depth\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPT_INPUT_DEPTH as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"output-depth\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPT_OUTPUT_DEPTH as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"dts-compress\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPT_DTS_COMPRESSION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"output-csp\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPT_OUTPUT_CSP as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"input-range\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: OPT_INPUT_RANGE as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"stitchable\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"filler\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: 0 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
];
unsafe extern "C" fn select_output(
    mut muxer: *const libc::c_char,
    mut filename: *mut libc::c_char,
    mut param: *mut x264_param_t,
) -> libc::c_int {
    let mut ext: *const libc::c_char = get_filename_extension(filename);
    if strcmp(filename, b"-\0" as *const u8 as *const libc::c_char) == 0
        || strcasecmp(muxer, b"auto\0" as *const u8 as *const libc::c_char) != 0
    {
        ext = muxer;
    }
    if strcasecmp(ext, b"mp4\0" as *const u8 as *const libc::c_char) == 0 {
        x264_cli_log(
            b"x264\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            b"not compiled with MP4 output support\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    } else if strcasecmp(ext, b"mkv\0" as *const u8 as *const libc::c_char) == 0 {
        cli_output = mkv_output;
        (*param).b_annexb = 0 as libc::c_int;
        (*param).b_repeat_headers = 0 as libc::c_int;
    } else if strcasecmp(ext, b"flv\0" as *const u8 as *const libc::c_char) == 0 {
        cli_output = flv_output;
        (*param).b_annexb = 0 as libc::c_int;
        (*param).b_repeat_headers = 0 as libc::c_int;
    } else {
        cli_output = raw_output;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn select_input(
    mut demuxer: *const libc::c_char,
    mut used_demuxer: *mut libc::c_char,
    mut filename: *mut libc::c_char,
    mut p_handle: *mut hnd_t,
    mut info: *mut video_info_t,
    mut opt: *mut cli_input_opt_t,
) -> libc::c_int {
    let mut b_auto: libc::c_int = (strcasecmp(
        demuxer,
        b"auto\0" as *const u8 as *const libc::c_char,
    ) == 0) as libc::c_int;
    let mut ext: *const libc::c_char = if b_auto != 0 {
        get_filename_extension(filename) as *const libc::c_char
    } else {
        b"\0" as *const u8 as *const libc::c_char
    };
    let mut b_regular: libc::c_int = strcmp(
        filename,
        b"-\0" as *const u8 as *const libc::c_char,
    );
    if b_regular == 0 && b_auto != 0 {
        ext = b"raw\0" as *const u8 as *const libc::c_char;
    }
    b_regular = (b_regular != 0 && x264_is_regular_file_path(filename) != 0)
        as libc::c_int;
    if b_regular != 0 {
        let mut f: *mut FILE = fopen(
            filename,
            b"r\0" as *const u8 as *const libc::c_char,
        );
        if !f.is_null() {
            b_regular = x264_is_regular_file(f);
            fclose(f);
        }
    }
    let mut module: *const libc::c_char = if b_auto != 0 { ext } else { demuxer };
    if strcasecmp(module, b"avs\0" as *const u8 as *const libc::c_char) == 0
        || strcasecmp(ext, b"d2v\0" as *const u8 as *const libc::c_char) == 0
        || strcasecmp(ext, b"dga\0" as *const u8 as *const libc::c_char) == 0
    {
        x264_cli_log(
            b"x264\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            b"not compiled with AVS input support\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    } else if strcasecmp(module, b"y4m\0" as *const u8 as *const libc::c_char) == 0 {
        cli_input = y4m_input;
    } else if strcasecmp(module, b"raw\0" as *const u8 as *const libc::c_char) == 0
        || strcasecmp(ext, b"yuv\0" as *const u8 as *const libc::c_char) == 0
    {
        cli_input = raw_input;
    } else {
        if b_auto != 0
            && (raw_input.open_file)
                .expect("non-null function pointer")(filename, p_handle, info, opt) == 0
        {
            module = b"raw\0" as *const u8 as *const libc::c_char;
            b_auto = 0 as libc::c_int;
            cli_input = raw_input;
        }
        if (*p_handle).is_null() {
            x264_cli_log(
                b"x264\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
                b"could not open input file `%s' via any method!\n\0" as *const u8
                    as *const libc::c_char,
                filename,
            );
            return -(1 as libc::c_int);
        }
    }
    strcpy(used_demuxer, module);
    return 0 as libc::c_int;
}
unsafe extern "C" fn init_vid_filters(
    mut sequence: *mut libc::c_char,
    mut handle: *mut hnd_t,
    mut info: *mut video_info_t,
    mut param: *mut x264_param_t,
    mut output_csp: libc::c_int,
) -> libc::c_int {
    x264_register_vid_filters();
    if x264_init_vid_filter(
        b"source\0" as *const u8 as *const libc::c_char,
        handle,
        &mut filter,
        info,
        param,
        0 as *mut libc::c_char,
    ) != 0
    {
        return -(1 as libc::c_int);
    }
    if x264_init_vid_filter(
        b"resize\0" as *const u8 as *const libc::c_char,
        handle,
        &mut filter,
        info,
        param,
        b"normcsp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) != 0
    {
        return -(1 as libc::c_int);
    }
    if x264_init_vid_filter(
        b"fix_vfr_pts\0" as *const u8 as *const libc::c_char,
        handle,
        &mut filter,
        info,
        param,
        0 as *mut libc::c_char,
    ) != 0
    {
        return -(1 as libc::c_int);
    }
    let mut p: *mut libc::c_char = sequence;
    while !p.is_null() && *p as libc::c_int != 0 {
        let mut tok_len: libc::c_int = strcspn(
            p,
            b"/\0" as *const u8 as *const libc::c_char,
        ) as libc::c_int;
        let mut p_len: libc::c_int = strlen(p) as libc::c_int;
        *p.offset(tok_len as isize) = 0 as libc::c_int as libc::c_char;
        let mut name_len: libc::c_int = strcspn(
            p,
            b":\0" as *const u8 as *const libc::c_char,
        ) as libc::c_int;
        *p.offset(name_len as isize) = 0 as libc::c_int as libc::c_char;
        name_len += (name_len != tok_len) as libc::c_int;
        if x264_init_vid_filter(
            p,
            handle,
            &mut filter,
            info,
            param,
            p.offset(name_len as isize),
        ) != 0
        {
            return -(1 as libc::c_int);
        }
        p = p
            .offset(
                (if (tok_len + 1 as libc::c_int) < p_len {
                    tok_len + 1 as libc::c_int
                } else {
                    p_len
                }) as isize,
            );
    }
    if (*param).i_width == 0 && (*param).i_height == 0 {
        (*param).i_height = (*info).height;
        (*param).i_width = (*info).width;
    }
    (*param).i_csp = (*info).csp;
    let mut csp: libc::c_int = (*info).csp & 0xff as libc::c_int;
    if output_csp == 0x1 as libc::c_int && csp != 0x1 as libc::c_int {
        (*param).i_csp = 0x1 as libc::c_int;
    } else if output_csp == 0x2 as libc::c_int
        && (csp < 0x2 as libc::c_int || csp >= 0x6 as libc::c_int)
    {
        (*param).i_csp = 0x2 as libc::c_int;
    } else if output_csp == 0x6 as libc::c_int
        && (csp < 0x6 as libc::c_int || csp >= 0xc as libc::c_int)
    {
        (*param).i_csp = 0x6 as libc::c_int;
    } else if output_csp == 0xc as libc::c_int
        && (csp < 0xc as libc::c_int || csp >= 0xe as libc::c_int)
    {
        (*param).i_csp = 0xc as libc::c_int;
    } else if output_csp == 0x10 as libc::c_int
        && (csp < 0xe as libc::c_int || csp > 0x10 as libc::c_int)
    {
        (*param).i_csp = 0x10 as libc::c_int;
    }
    (*param).i_csp |= (*info).csp & 0x2000 as libc::c_int;
    if (*param).vui.b_fullrange == RANGE_AUTO as libc::c_int {
        (*param).vui.b_fullrange = (*info).fullrange;
    }
    if x264_init_vid_filter(
        b"resize\0" as *const u8 as *const libc::c_char,
        handle,
        &mut filter,
        info,
        param,
        0 as *mut libc::c_char,
    ) != 0
    {
        return -(1 as libc::c_int);
    }
    let mut args: [libc::c_char; 20] = [0; 20];
    let mut name: [libc::c_char; 20] = [0; 20];
    sprintf(
        args.as_mut_ptr(),
        b"bit_depth=%d\0" as *const u8 as *const libc::c_char,
        (*param).i_bitdepth,
    );
    sprintf(
        name.as_mut_ptr(),
        b"depth_%d\0" as *const u8 as *const libc::c_char,
        (*param).i_bitdepth,
    );
    if x264_init_vid_filter(
        name.as_mut_ptr(),
        handle,
        &mut filter,
        info,
        param,
        args.as_mut_ptr(),
    ) != 0
    {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn parse_enum_name(
    mut arg: *const libc::c_char,
    mut names: *const *const libc::c_char,
    mut dst: *mut *const libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    while !(*names.offset(i as isize)).is_null() {
        if **names.offset(i as isize) as libc::c_int != 0
            && strcasecmp(arg, *names.offset(i as isize)) == 0
        {
            *dst = *names.offset(i as isize);
            return 0 as libc::c_int;
        }
        i += 1;
        i;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn parse_enum_value(
    mut arg: *const libc::c_char,
    mut names: *const *const libc::c_char,
    mut dst: *mut libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    while !(*names.offset(i as isize)).is_null() {
        if **names.offset(i as isize) as libc::c_int != 0
            && strcasecmp(arg, *names.offset(i as isize)) == 0
        {
            *dst = i;
            return 0 as libc::c_int;
        }
        i += 1;
        i;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn parse(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut param: *mut x264_param_t,
    mut opt: *mut cli_opt_t,
) -> libc::c_int {
    static mut output_csp_fix: [uint8_t; 5] = [
        0x1 as libc::c_int as uint8_t,
        0x2 as libc::c_int as uint8_t,
        0x6 as libc::c_int as uint8_t,
        0xc as libc::c_int as uint8_t,
        0x10 as libc::c_int as uint8_t,
    ];
    let mut current_block: u64;
    let mut input_filename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut demuxer: *const libc::c_char = x264_demuxer_names[0 as libc::c_int as usize];
    let mut output_filename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut muxer: *const libc::c_char = x264_muxer_names[0 as libc::c_int as usize];
    let mut tcfile_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut defaults: x264_param_t = x264_param_t {
        cpu: 0,
        i_threads: 0,
        i_lookahead_threads: 0,
        b_sliced_threads: 0,
        b_deterministic: 0,
        b_cpu_independent: 0,
        i_sync_lookahead: 0,
        i_width: 0,
        i_height: 0,
        i_csp: 0,
        i_bitdepth: 0,
        i_level_idc: 0,
        i_frame_total: 0,
        i_nal_hrd: 0,
        vui: C2RustUnnamed_4 {
            i_sar_height: 0,
            i_sar_width: 0,
            i_overscan: 0,
            i_vidformat: 0,
            b_fullrange: 0,
            i_colorprim: 0,
            i_transfer: 0,
            i_colmatrix: 0,
            i_chroma_loc: 0,
        },
        i_frame_reference: 0,
        i_dpb_size: 0,
        i_keyint_max: 0,
        i_keyint_min: 0,
        i_scenecut_threshold: 0,
        b_intra_refresh: 0,
        i_bframe: 0,
        i_bframe_adaptive: 0,
        i_bframe_bias: 0,
        i_bframe_pyramid: 0,
        b_open_gop: 0,
        b_bluray_compat: 0,
        i_avcintra_class: 0,
        i_avcintra_flavor: 0,
        b_deblocking_filter: 0,
        i_deblocking_filter_alphac0: 0,
        i_deblocking_filter_beta: 0,
        b_cabac: 0,
        i_cabac_init_idc: 0,
        b_interlaced: 0,
        b_constrained_intra: 0,
        i_cqm_preset: 0,
        psz_cqm_file: 0 as *mut libc::c_char,
        cqm_4iy: [0; 16],
        cqm_4py: [0; 16],
        cqm_4ic: [0; 16],
        cqm_4pc: [0; 16],
        cqm_8iy: [0; 64],
        cqm_8py: [0; 64],
        cqm_8ic: [0; 64],
        cqm_8pc: [0; 64],
        pf_log: None,
        p_log_private: 0 as *mut libc::c_void,
        i_log_level: 0,
        b_full_recon: 0,
        psz_dump_yuv: 0 as *mut libc::c_char,
        analyse: C2RustUnnamed_3 {
            intra: 0,
            inter: 0,
            b_transform_8x8: 0,
            i_weighted_pred: 0,
            b_weighted_bipred: 0,
            i_direct_mv_pred: 0,
            i_chroma_qp_offset: 0,
            i_me_method: 0,
            i_me_range: 0,
            i_mv_range: 0,
            i_mv_range_thread: 0,
            i_subpel_refine: 0,
            b_chroma_me: 0,
            b_mixed_references: 0,
            i_trellis: 0,
            b_fast_pskip: 0,
            b_dct_decimate: 0,
            i_noise_reduction: 0,
            f_psy_rd: 0.,
            f_psy_trellis: 0.,
            b_psy: 0,
            b_mb_info: 0,
            b_mb_info_update: 0,
            i_luma_deadzone: [0; 2],
            b_psnr: 0,
            b_ssim: 0,
        },
        rc: C2RustUnnamed_2 {
            i_rc_method: 0,
            i_qp_constant: 0,
            i_qp_min: 0,
            i_qp_max: 0,
            i_qp_step: 0,
            i_bitrate: 0,
            f_rf_constant: 0.,
            f_rf_constant_max: 0.,
            f_rate_tolerance: 0.,
            i_vbv_max_bitrate: 0,
            i_vbv_buffer_size: 0,
            f_vbv_buffer_init: 0.,
            f_ip_factor: 0.,
            f_pb_factor: 0.,
            b_filler: 0,
            i_aq_mode: 0,
            f_aq_strength: 0.,
            b_mb_tree: 0,
            i_lookahead: 0,
            b_stat_write: 0,
            psz_stat_out: 0 as *mut libc::c_char,
            b_stat_read: 0,
            psz_stat_in: 0 as *mut libc::c_char,
            f_qcompress: 0.,
            f_qblur: 0.,
            f_complexity_blur: 0.,
            zones: 0 as *mut x264_zone_t,
            i_zones: 0,
            psz_zones: 0 as *mut libc::c_char,
        },
        crop_rect: C2RustUnnamed_1 {
            i_left: 0,
            i_top: 0,
            i_right: 0,
            i_bottom: 0,
        },
        i_frame_packing: 0,
        mastering_display: C2RustUnnamed_0 {
            b_mastering_display: 0,
            i_green_x: 0,
            i_green_y: 0,
            i_blue_x: 0,
            i_blue_y: 0,
            i_red_x: 0,
            i_red_y: 0,
            i_white_x: 0,
            i_white_y: 0,
            i_display_max: 0,
            i_display_min: 0,
        },
        content_light_level: C2RustUnnamed {
            b_cll: 0,
            i_max_cll: 0,
            i_max_fall: 0,
        },
        i_alternative_transfer: 0,
        b_aud: 0,
        b_repeat_headers: 0,
        b_annexb: 0,
        i_sps_id: 0,
        b_vfr_input: 0,
        b_pulldown: 0,
        i_fps_num: 0,
        i_fps_den: 0,
        i_timebase_num: 0,
        i_timebase_den: 0,
        b_tff: 0,
        b_pic_struct: 0,
        b_fake_interlaced: 0,
        b_stitchable: 0,
        b_opencl: 0,
        i_opencl_device: 0,
        opencl_device_id: 0 as *mut libc::c_void,
        psz_clbin_file: 0 as *mut libc::c_char,
        i_slice_max_size: 0,
        i_slice_max_mbs: 0,
        i_slice_min_mbs: 0,
        i_slice_count: 0,
        i_slice_count_max: 0,
        param_free: None,
        nalu_process: None,
        opaque: 0 as *mut libc::c_void,
    };
    let mut profile: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut vid_filters: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut b_thread_input: libc::c_int = 0 as libc::c_int;
    let mut b_turbo: libc::c_int = 1 as libc::c_int;
    let mut b_user_ref: libc::c_int = 0 as libc::c_int;
    let mut b_user_fps: libc::c_int = 0 as libc::c_int;
    let mut b_user_interlaced: libc::c_int = 0 as libc::c_int;
    let mut input_opt: cli_input_opt_t = cli_input_opt_t {
        index_file: 0 as *mut libc::c_char,
        format: 0 as *mut libc::c_char,
        resolution: 0 as *mut libc::c_char,
        colorspace: 0 as *mut libc::c_char,
        bit_depth: 0,
        timebase: 0 as *mut libc::c_char,
        seek: 0,
        progress: 0,
        output_csp: 0,
        output_range: 0,
        input_range: 0,
    };
    let mut output_opt: cli_output_opt_t = cli_output_opt_t {
        use_dts_compress: 0,
    };
    let mut preset: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tune: *mut libc::c_char = 0 as *mut libc::c_char;
    optind = 0 as libc::c_int;
    loop {
        let mut c: libc::c_int = getopt_long(
            argc,
            argv,
            short_options.as_mut_ptr(),
            long_options.as_mut_ptr(),
            0 as *mut libc::c_int,
        );
        if c == -(1 as libc::c_int) {
            break;
        }
        if c == OPT_PRESET as libc::c_int {
            preset = optarg;
        }
        if c == OPT_TUNE as libc::c_int {
            tune = optarg;
        } else if c == '?' as i32 {
            return -(1 as libc::c_int)
        }
    }
    if !preset.is_null()
        && strcasecmp(preset, b"placebo\0" as *const u8 as *const libc::c_char) == 0
    {
        b_turbo = 0 as libc::c_int;
    }
    if (!preset.is_null() || !tune.is_null())
        && x264_param_default_preset(param, preset, tune) < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    x264_param_default(&mut defaults);
    cli_log_level = defaults.i_log_level;
    memset(
        &mut input_opt as *mut cli_input_opt_t as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<cli_input_opt_t>() as libc::c_ulong,
    );
    memset(
        &mut output_opt as *mut cli_output_opt_t as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<cli_output_opt_t>() as libc::c_ulong,
    );
    input_opt.bit_depth = 8 as libc::c_int;
    (*param).vui.b_fullrange = RANGE_AUTO as libc::c_int;
    input_opt.output_range = (*param).vui.b_fullrange;
    input_opt.input_range = input_opt.output_range;
    let mut output_csp: libc::c_int = defaults.i_csp;
    (*opt).b_progress = 1 as libc::c_int;
    optind = 0 as libc::c_int;
    loop {
        let mut b_error: libc::c_int = 0 as libc::c_int;
        let mut long_options_index: libc::c_int = -(1 as libc::c_int);
        let mut c_0: libc::c_int = getopt_long(
            argc,
            argv,
            short_options.as_mut_ptr(),
            long_options.as_mut_ptr(),
            &mut long_options_index,
        );
        if c_0 == -(1 as libc::c_int) {
            break;
        }
        match c_0 {
            104 => {
                help(&mut defaults, 0 as libc::c_int);
                exit(0 as libc::c_int);
            }
            262 => {
                help(&mut defaults, 1 as libc::c_int);
                exit(0 as libc::c_int);
            }
            267 => {
                help(&mut defaults, 2 as libc::c_int);
                exit(0 as libc::c_int);
            }
            86 => {
                print_version_info();
                exit(0 as libc::c_int);
            }
            256 => {
                (*param)
                    .i_frame_total = if atoi(optarg) > 0 as libc::c_int {
                    atoi(optarg)
                } else {
                    0 as libc::c_int
                };
                current_block = 11702799181856929651;
            }
            257 => {
                (*opt)
                    .i_seek = if atoi(optarg) > 0 as libc::c_int {
                    atoi(optarg)
                } else {
                    0 as libc::c_int
                };
                current_block = 11702799181856929651;
            }
            111 => {
                output_filename = optarg;
                current_block = 11702799181856929651;
            }
            269 => {
                if parse_enum_name(optarg, x264_muxer_names.as_ptr(), &mut muxer) != 0 {
                    x264_cli_log(
                        b"x264\0" as *const u8 as *const libc::c_char,
                        0 as libc::c_int,
                        b"Unknown muxer `%s'\n\0" as *const u8 as *const libc::c_char,
                        optarg,
                    );
                    return -(1 as libc::c_int);
                }
                current_block = 11702799181856929651;
            }
            270 => {
                if parse_enum_name(optarg, x264_demuxer_names.as_ptr(), &mut demuxer)
                    != 0
                {
                    x264_cli_log(
                        b"x264\0" as *const u8 as *const libc::c_char,
                        0 as libc::c_int,
                        b"Unknown demuxer `%s'\n\0" as *const u8 as *const libc::c_char,
                        optarg,
                    );
                    return -(1 as libc::c_int);
                }
                current_block = 11702799181856929651;
            }
            271 => {
                input_opt.index_file = optarg;
                current_block = 11702799181856929651;
            }
            258 => {
                (*opt)
                    .qpfile = fopen(optarg, b"rb\0" as *const u8 as *const libc::c_char);
                if ((*opt).qpfile).is_null() {
                    x264_cli_log(
                        b"x264\0" as *const u8 as *const libc::c_char,
                        0 as libc::c_int,
                        b"can't open qpfile `%s'\n\0" as *const u8
                            as *const libc::c_char,
                        optarg,
                    );
                    return -(1 as libc::c_int);
                }
                if x264_is_regular_file((*opt).qpfile) == 0 {
                    x264_cli_log(
                        b"x264\0" as *const u8 as *const libc::c_char,
                        0 as libc::c_int,
                        b"qpfile incompatible with non-regular file `%s'\n\0"
                            as *const u8 as *const libc::c_char,
                        optarg,
                    );
                    fclose((*opt).qpfile);
                    return -(1 as libc::c_int);
                }
                current_block = 11702799181856929651;
            }
            259 => {
                b_thread_input = 1 as libc::c_int;
                current_block = 11702799181856929651;
            }
            260 => {
                (*param).i_log_level = -(1 as libc::c_int);
                cli_log_level = (*param).i_log_level;
                current_block = 11702799181856929651;
            }
            118 => {
                (*param).i_log_level = 3 as libc::c_int;
                cli_log_level = (*param).i_log_level;
                current_block = 11702799181856929651;
            }
            277 => {
                if parse_enum_value(
                    optarg,
                    x264_log_level_names.as_ptr(),
                    &mut cli_log_level,
                ) == 0
                {
                    cli_log_level += -(1 as libc::c_int);
                } else {
                    cli_log_level = atoi(optarg);
                }
                (*param).i_log_level = cli_log_level;
                current_block = 11702799181856929651;
            }
            261 => {
                (*opt).b_progress = 0 as libc::c_int;
                current_block = 11702799181856929651;
            }
            265 | 264 => {
                current_block = 11702799181856929651;
            }
            263 => {
                profile = optarg;
                current_block = 11702799181856929651;
            }
            266 => {
                b_turbo = 0 as libc::c_int;
                current_block = 11702799181856929651;
            }
            114 => {
                b_user_ref = 1 as libc::c_int;
                current_block = 13467107903497930144;
            }
            268 => {
                b_user_fps = 1 as libc::c_int;
                (*param).b_vfr_input = 0 as libc::c_int;
                current_block = 13467107903497930144;
            }
            272 => {
                b_user_interlaced = 1 as libc::c_int;
                current_block = 13467107903497930144;
            }
            273 => {
                tcfile_name = optarg;
                current_block = 11702799181856929651;
            }
            274 => {
                (*opt)
                    .tcfile_out = fopen(
                    optarg,
                    b"wb\0" as *const u8 as *const libc::c_char,
                );
                if ((*opt).tcfile_out).is_null() {
                    x264_cli_log(
                        b"x264\0" as *const u8 as *const libc::c_char,
                        0 as libc::c_int,
                        b"can't open `%s'\n\0" as *const u8 as *const libc::c_char,
                        optarg,
                    );
                    return -(1 as libc::c_int);
                }
                current_block = 11702799181856929651;
            }
            275 => {
                input_opt.timebase = optarg;
                current_block = 11702799181856929651;
            }
            276 => {
                if parse_enum_value(
                    optarg,
                    x264_pulldown_names.as_ptr(),
                    &mut (*opt).i_pulldown,
                ) != 0
                {
                    x264_cli_log(
                        b"x264\0" as *const u8 as *const libc::c_char,
                        0 as libc::c_int,
                        b"Unknown pulldown `%s'\n\0" as *const u8 as *const libc::c_char,
                        optarg,
                    );
                    return -(1 as libc::c_int);
                }
                current_block = 11702799181856929651;
            }
            278 => {
                vid_filters = optarg;
                current_block = 11702799181856929651;
            }
            279 => {
                input_opt.format = optarg;
                current_block = 11702799181856929651;
            }
            280 => {
                input_opt.resolution = optarg;
                current_block = 11702799181856929651;
            }
            281 => {
                input_opt.colorspace = optarg;
                current_block = 11702799181856929651;
            }
            282 => {
                input_opt.bit_depth = atoi(optarg);
                current_block = 11702799181856929651;
            }
            283 => {
                (*param).i_bitdepth = atoi(optarg);
                current_block = 11702799181856929651;
            }
            284 => {
                output_opt.use_dts_compress = 1 as libc::c_int;
                current_block = 11702799181856929651;
            }
            285 => {
                if parse_enum_value(
                    optarg,
                    x264_output_csp_names.as_ptr(),
                    &mut output_csp,
                ) != 0
                {
                    x264_cli_log(
                        b"x264\0" as *const u8 as *const libc::c_char,
                        0 as libc::c_int,
                        b"Unknown output csp `%s'\n\0" as *const u8
                            as *const libc::c_char,
                        optarg,
                    );
                    return -(1 as libc::c_int);
                }
                output_csp = output_csp_fix[output_csp as usize] as libc::c_int;
                (*param).i_csp = output_csp;
                current_block = 11702799181856929651;
            }
            286 => {
                if parse_enum_value(
                    optarg,
                    x264_range_names.as_ptr(),
                    &mut input_opt.input_range,
                ) != 0
                {
                    x264_cli_log(
                        b"x264\0" as *const u8 as *const libc::c_char,
                        0 as libc::c_int,
                        b"Unknown input range `%s'\n\0" as *const u8
                            as *const libc::c_char,
                        optarg,
                    );
                    return -(1 as libc::c_int);
                }
                input_opt.input_range += RANGE_AUTO as libc::c_int;
                current_block = 11702799181856929651;
            }
            287 => {
                if parse_enum_value(
                    optarg,
                    x264_range_names.as_ptr(),
                    &mut (*param).vui.b_fullrange,
                ) != 0
                {
                    x264_cli_log(
                        b"x264\0" as *const u8 as *const libc::c_char,
                        0 as libc::c_int,
                        b"Unknown range `%s'\n\0" as *const u8 as *const libc::c_char,
                        optarg,
                    );
                    return -(1 as libc::c_int);
                }
                (*param).vui.b_fullrange += RANGE_AUTO as libc::c_int;
                input_opt.output_range = (*param).vui.b_fullrange;
                current_block = 11702799181856929651;
            }
            _ => {
                current_block = 13467107903497930144;
            }
        }
        match current_block {
            13467107903497930144 => {
                if long_options_index < 0 as libc::c_int {
                    let mut i: libc::c_int = 0 as libc::c_int;
                    while !(long_options[i as usize].name).is_null() {
                        if long_options[i as usize].val == c_0 {
                            long_options_index = i;
                            break;
                        } else {
                            i += 1;
                            i;
                        }
                    }
                    if long_options_index < 0 as libc::c_int {
                        return -(1 as libc::c_int);
                    }
                }
                b_error
                    |= x264_param_parse(
                        param,
                        long_options[long_options_index as usize].name,
                        optarg,
                    );
            }
            _ => {}
        }
        if b_error != 0 {
            let mut name: *const libc::c_char = if long_options_index > 0 as libc::c_int
            {
                long_options[long_options_index as usize].name
            } else {
                *argv.offset((optind - 2 as libc::c_int) as isize) as *const libc::c_char
            };
            x264_cli_log(
                b"x264\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
                b"invalid argument: %s = %s\n\0" as *const u8 as *const libc::c_char,
                name,
                optarg,
            );
            return -(1 as libc::c_int);
        }
    }
    if b_turbo != 0 {
        x264_param_apply_fastfirstpass(param);
    }
    if x264_param_apply_profile(param, profile) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if optind > argc - 1 as libc::c_int || output_filename.is_null() {
        x264_cli_log(
            b"x264\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            b"No %s file. Run x264 --help for a list of options.\n\0" as *const u8
                as *const libc::c_char,
            if optind > argc - 1 as libc::c_int {
                b"input\0" as *const u8 as *const libc::c_char
            } else {
                b"output\0" as *const u8 as *const libc::c_char
            },
        );
        return -(1 as libc::c_int);
    }
    if select_output(muxer, output_filename, param) != 0 {
        return -(1 as libc::c_int);
    }
    if (cli_output.open_file)
        .expect(
            "non-null function pointer",
        )(output_filename, &mut (*opt).hout, &mut output_opt) != 0
    {
        x264_cli_log(
            b"x264\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            b"could not open output file `%s'\n\0" as *const u8 as *const libc::c_char,
            output_filename,
        );
        return -(1 as libc::c_int);
    }
    let fresh0 = optind;
    optind = optind + 1;
    input_filename = *argv.offset(fresh0 as isize);
    let mut info: video_info_t = {
        let mut init = video_info_t {
            csp: 0 as libc::c_int,
            fps_num: 0,
            fps_den: 0,
            fullrange: 0,
            width: 0,
            height: 0,
            interlaced: 0,
            num_frames: 0,
            sar_width: 0,
            sar_height: 0,
            tff: 0,
            thread_safe: 0,
            timebase_num: 0,
            timebase_den: 0,
            vfr: 0,
        };
        init
    };
    let mut demuxername: [libc::c_char; 5] = [0; 5];
    info.csp = (*param).i_csp;
    info.fps_num = (*param).i_fps_num;
    info.fps_den = (*param).i_fps_den;
    info.fullrange = (input_opt.input_range == RANGE_PC as libc::c_int) as libc::c_int;
    info.interlaced = (*param).b_interlaced;
    if (*param).vui.i_sar_width > 0 as libc::c_int
        && (*param).vui.i_sar_height > 0 as libc::c_int
    {
        info.sar_width = (*param).vui.i_sar_width as uint32_t;
        info.sar_height = (*param).vui.i_sar_height as uint32_t;
    }
    info.tff = (*param).b_tff;
    info.vfr = (*param).b_vfr_input;
    input_opt.seek = (*opt).i_seek;
    input_opt.progress = (*opt).b_progress;
    input_opt.output_csp = output_csp;
    if select_input(
        demuxer,
        demuxername.as_mut_ptr(),
        input_filename,
        &mut (*opt).hin,
        &mut info,
        &mut input_opt,
    ) != 0
    {
        return -(1 as libc::c_int);
    }
    if ((*opt).hin).is_null()
        && (cli_input.open_file)
            .expect(
                "non-null function pointer",
            )(input_filename, &mut (*opt).hin, &mut info, &mut input_opt) != 0
    {
        x264_cli_log(
            b"x264\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            b"could not open input file `%s'\n\0" as *const u8 as *const libc::c_char,
            input_filename,
        );
        return -(1 as libc::c_int);
    }
    x264_reduce_fraction(&mut info.sar_width, &mut info.sar_height);
    x264_reduce_fraction(&mut info.fps_num, &mut info.fps_den);
    x264_cli_log(
        demuxername.as_mut_ptr(),
        2 as libc::c_int,
        b"%dx%d%c %u:%u @ %u/%u fps (%cfr)\n\0" as *const u8 as *const libc::c_char,
        info.width,
        info.height,
        if info.interlaced != 0 { 'i' as i32 } else { 'p' as i32 },
        info.sar_width,
        info.sar_height,
        info.fps_num,
        info.fps_den,
        if info.vfr != 0 { 'v' as i32 } else { 'c' as i32 },
    );
    if info.width <= 0 as libc::c_int || info.height <= 0 as libc::c_int
        || info.width > 16384 as libc::c_int || info.height > 16384 as libc::c_int
    {
        x264_cli_log(
            b"x264\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            b"invalid width x height (%dx%d)\n\0" as *const u8 as *const libc::c_char,
            info.width,
            info.height,
        );
        return -(1 as libc::c_int);
    }
    if !tcfile_name.is_null() {
        if b_user_fps != 0 {
            x264_cli_log(
                b"x264\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
                b"--fps + --tcfile-in is incompatible.\n\0" as *const u8
                    as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        if (timecode_input.open_file)
            .expect(
                "non-null function pointer",
            )(tcfile_name, &mut (*opt).hin, &mut info, &mut input_opt) != 0
        {
            x264_cli_log(
                b"x264\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
                b"timecode input failed\n\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        cli_input = timecode_input;
    } else if info.vfr == 0 && !(input_opt.timebase).is_null() {
        x264_cli_log(
            b"x264\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            b"--timebase is incompatible with cfr input\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    let mut thread_input: *const cli_input_t = 0 as *const cli_input_t;
    if (*param).i_bitdepth == 8 as libc::c_int {
        thread_input = &thread_8_input;
    } else {
        thread_input = 0 as *const cli_input_t;
    }
    if !thread_input.is_null() && info.thread_safe != 0
        && (b_thread_input != 0 || (*param).i_threads > 1 as libc::c_int
            || (*param).i_threads == 0 as libc::c_int
                && x264_cpu_num_processors() > 1 as libc::c_int)
    {
        if ((*thread_input).open_file)
            .expect(
                "non-null function pointer",
            )(
            0 as *mut libc::c_char,
            &mut (*opt).hin,
            &mut info,
            0 as *mut cli_input_opt_t,
        ) != 0
        {
            fprintf(
                stderr,
                b"x264 [error]: threaded input failed\n\0" as *const u8
                    as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        cli_input = *thread_input;
    }
    if (*param).vui.i_sar_width > 0 as libc::c_int
        && (*param).vui.i_sar_height > 0 as libc::c_int
    {
        info.sar_width = (*param).vui.i_sar_width as uint32_t;
        info.sar_height = (*param).vui.i_sar_height as uint32_t;
    }
    if b_user_fps != 0 {
        info.fps_num = (*param).i_fps_num;
        info.fps_den = (*param).i_fps_den;
    }
    if info.vfr == 0 {
        info.timebase_num = info.fps_den;
        info.timebase_den = info.fps_num;
    }
    if tcfile_name.is_null() && !(input_opt.timebase).is_null() {
        let mut i_user_timebase_num: uint64_t = 0;
        let mut i_user_timebase_den: uint64_t = 0;
        let mut ret: libc::c_int = sscanf(
            input_opt.timebase,
            b"%lu/%lu\0" as *const u8 as *const libc::c_char,
            &mut i_user_timebase_num as *mut uint64_t,
            &mut i_user_timebase_den as *mut uint64_t,
        );
        if ret == 0 {
            x264_cli_log(
                b"x264\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
                b"invalid argument: timebase = %s\n\0" as *const u8
                    as *const libc::c_char,
                input_opt.timebase,
            );
            return -(1 as libc::c_int);
        }
        if ret == 1 as libc::c_int {
            i_user_timebase_num = info.timebase_num as uint64_t;
            i_user_timebase_den = strtoul(
                input_opt.timebase,
                0 as *mut *mut libc::c_char,
                10 as libc::c_int,
            );
        }
        if i_user_timebase_num > 4294967295 as libc::c_uint as uint64_t
            || i_user_timebase_den > 4294967295 as libc::c_uint as uint64_t
        {
            x264_cli_log(
                b"x264\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
                b"timebase you specified exceeds H.264 maximum\n\0" as *const u8
                    as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        (*opt)
            .timebase_convert_multiplier = i_user_timebase_den as libc::c_double
            / info.timebase_den as libc::c_double
            * (info.timebase_num as libc::c_double
                / i_user_timebase_num as libc::c_double);
        info.timebase_num = i_user_timebase_num as uint32_t;
        info.timebase_den = i_user_timebase_den as uint32_t;
        info.vfr = 1 as libc::c_int;
    }
    if b_user_interlaced != 0 {
        info.interlaced = (*param).b_interlaced;
        info.tff = (*param).b_tff;
    }
    if input_opt.input_range != RANGE_AUTO as libc::c_int {
        info.fullrange = input_opt.input_range;
    }
    if init_vid_filters(vid_filters, &mut (*opt).hin, &mut info, param, output_csp) != 0
    {
        return -(1 as libc::c_int);
    }
    (*param).b_vfr_input = info.vfr;
    (*param).i_fps_num = info.fps_num;
    (*param).i_fps_den = info.fps_den;
    (*param).i_timebase_num = info.timebase_num;
    (*param).i_timebase_den = info.timebase_den;
    (*param).vui.i_sar_width = info.sar_width as libc::c_int;
    (*param).vui.i_sar_height = info.sar_height as libc::c_int;
    info
        .num_frames = if info.num_frames - (*opt).i_seek > 0 as libc::c_int {
        info.num_frames - (*opt).i_seek
    } else {
        0 as libc::c_int
    };
    if (info.num_frames == 0 || (*param).i_frame_total < info.num_frames)
        && (*param).i_frame_total > 0 as libc::c_int
    {
        info.num_frames = (*param).i_frame_total;
    }
    (*param).i_frame_total = info.num_frames;
    if b_user_interlaced == 0 && info.interlaced != 0 {
        x264_cli_log(
            b"x264\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
            b"input appears to be interlaced, enabling %cff interlaced mode.\n                If you want otherwise, use --no-interlaced or --%cff\n\0"
                as *const u8 as *const libc::c_char,
            if info.tff != 0 { 't' as i32 } else { 'b' as i32 },
            if info.tff != 0 { 'b' as i32 } else { 't' as i32 },
        );
        (*param).b_interlaced = 1 as libc::c_int;
        (*param).b_tff = (info.tff != 0) as libc::c_int;
    }
    let mut csp: libc::c_int = (*param).i_csp & 0xff as libc::c_int;
    if csp >= 0xe as libc::c_int && csp <= 0x10 as libc::c_int {
        if input_opt.output_range == RANGE_AUTO as libc::c_int {
            (*param).vui.b_fullrange = RANGE_PC as libc::c_int;
        }
        if (*param).vui.b_fullrange == 0 {
            x264_cli_log(
                b"x264\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
                b"RGB must be PC range\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    }
    if b_user_ref == 0 {
        let mut mbs: libc::c_int = ((*param).i_width + 15 as libc::c_int
            >> 4 as libc::c_int)
            * ((*param).i_height + 15 as libc::c_int >> 4 as libc::c_int);
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while (*x264_levels.as_ptr().offset(i_0 as isize)).level_idc as libc::c_int
            != 0 as libc::c_int
        {
            if (*param).i_level_idc
                == (*x264_levels.as_ptr().offset(i_0 as isize)).level_idc as libc::c_int
            {
                while mbs * (*param).i_frame_reference
                    > (*x264_levels.as_ptr().offset(i_0 as isize)).dpb
                    && (*param).i_frame_reference > 1 as libc::c_int
                {
                    (*param).i_frame_reference -= 1;
                    (*param).i_frame_reference;
                }
                break;
            } else {
                i_0 += 1;
                i_0;
            }
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn parse_qpfile(
    mut opt: *mut cli_opt_t,
    mut pic: *mut x264_picture_t,
    mut i_frame: libc::c_int,
) {
    let mut num: libc::c_int = -(1 as libc::c_int);
    let mut type_0: libc::c_char = 0;
    let mut buf: [libc::c_char; 100] = [0; 100];
    while num < i_frame {
        let mut file_pos: int64_t = ftello((*opt).qpfile);
        let mut qp: libc::c_int = -(1 as libc::c_int);
        let mut ret: libc::c_int = fscanf(
            (*opt).qpfile,
            b" %99[^\n]\n\0" as *const u8 as *const libc::c_char,
            buf.as_mut_ptr(),
        );
        if ret == 1 as libc::c_int {
            ret = sscanf(
                buf.as_mut_ptr(),
                b"%d %c %d\0" as *const u8 as *const libc::c_char,
                &mut num as *mut libc::c_int,
                &mut type_0 as *mut libc::c_char,
                &mut qp as *mut libc::c_int,
            );
            if ret == -(1 as libc::c_int) {
                ret = 0 as libc::c_int;
            }
        }
        (*pic).i_type = 0 as libc::c_int;
        (*pic).i_qpplus1 = 0 as libc::c_int;
        if num > i_frame || ret == -(1 as libc::c_int) {
            if ret == -(1 as libc::c_int) || file_pos < 0 as libc::c_int as int64_t
                || fseeko((*opt).qpfile, file_pos, 0 as libc::c_int) != 0
            {
                if ret != -(1 as libc::c_int) {
                    x264_cli_log(
                        b"x264\0" as *const u8 as *const libc::c_char,
                        0 as libc::c_int,
                        b"qpfile seeking failed\n\0" as *const u8 as *const libc::c_char,
                    );
                }
                fclose((*opt).qpfile);
                (*opt).qpfile = 0 as *mut FILE;
            }
            break;
        } else {
            if num < i_frame && ret >= 2 as libc::c_int {
                continue;
            }
            if type_0 as libc::c_int == 'I' as i32 {
                (*pic).i_type = 0x1 as libc::c_int;
            } else if type_0 as libc::c_int == 'i' as i32 {
                (*pic).i_type = 0x2 as libc::c_int;
            } else if type_0 as libc::c_int == 'K' as i32 {
                (*pic).i_type = 0x6 as libc::c_int;
            } else if type_0 as libc::c_int == 'P' as i32 {
                (*pic).i_type = 0x3 as libc::c_int;
            } else if type_0 as libc::c_int == 'B' as i32 {
                (*pic).i_type = 0x4 as libc::c_int;
            } else if type_0 as libc::c_int == 'b' as i32 {
                (*pic).i_type = 0x5 as libc::c_int;
            } else {
                ret = 0 as libc::c_int;
            }
            if ret < 2 as libc::c_int || qp < -(1 as libc::c_int)
                || qp
                    > 51 as libc::c_int + 6 as libc::c_int * 2 as libc::c_int
                        + 18 as libc::c_int
            {
                x264_cli_log(
                    b"x264\0" as *const u8 as *const libc::c_char,
                    0 as libc::c_int,
                    b"can't parse qpfile for frame %d\n\0" as *const u8
                        as *const libc::c_char,
                    i_frame,
                );
                fclose((*opt).qpfile);
                (*opt).qpfile = 0 as *mut FILE;
                break;
            } else if ret == 3 as libc::c_int && qp >= 0 as libc::c_int {
                (*pic).i_qpplus1 = qp + 1 as libc::c_int;
            }
        }
    }
}
unsafe extern "C" fn encode_frame(
    mut h: *mut x264_t,
    mut hout: hnd_t,
    mut pic: *mut x264_picture_t,
    mut last_dts: *mut int64_t,
) -> libc::c_int {
    let mut pic_out: x264_picture_t = x264_picture_t {
        i_type: 0,
        i_qpplus1: 0,
        i_pic_struct: 0,
        b_keyframe: 0,
        i_pts: 0,
        i_dts: 0,
        param: 0 as *mut x264_param_t,
        img: x264_image_t {
            i_csp: 0,
            i_plane: 0,
            i_stride: [0; 4],
            plane: [0 as *mut uint8_t; 4],
        },
        prop: x264_image_properties_t {
            quant_offsets: 0 as *mut libc::c_float,
            quant_offsets_free: None,
            mb_info: 0 as *mut uint8_t,
            mb_info_free: None,
            f_ssim: 0.,
            f_psnr_avg: 0.,
            f_psnr: [0.; 3],
            f_crf_avg: 0.,
        },
        hrd_timing: x264_hrd_t {
            cpb_initial_arrival_time: 0.,
            cpb_final_arrival_time: 0.,
            cpb_removal_time: 0.,
            dpb_output_time: 0.,
        },
        extra_sei: x264_sei_t {
            num_payloads: 0,
            payloads: 0 as *mut x264_sei_payload_t,
            sei_free: None,
        },
        opaque: 0 as *mut libc::c_void,
    };
    let mut nal: *mut x264_nal_t = 0 as *mut x264_nal_t;
    let mut i_nal: libc::c_int = 0;
    let mut i_frame_size: libc::c_int = 0 as libc::c_int;
    i_frame_size = x264_encoder_encode(h, &mut nal, &mut i_nal, pic, &mut pic_out);
    if i_frame_size < 0 as libc::c_int {
        x264_cli_log(
            b"x264\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            b"x264_encoder_encode failed\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if i_frame_size != 0 {
        i_frame_size = (cli_output.write_frame)
            .expect(
                "non-null function pointer",
            )(
            hout,
            (*nal.offset(0 as libc::c_int as isize)).p_payload,
            i_frame_size,
            &mut pic_out,
        );
        *last_dts = pic_out.i_dts;
    }
    return i_frame_size;
}
unsafe extern "C" fn print_status(
    mut i_start: int64_t,
    mut i_previous: int64_t,
    mut i_frame: libc::c_int,
    mut i_frame_total: libc::c_int,
    mut i_file: int64_t,
    mut param: *mut x264_param_t,
    mut last_ts: int64_t,
) -> int64_t {
    let mut buf: [libc::c_char; 200] = [0; 200];
    let mut i_time: int64_t = x264_mdate();
    if i_previous != 0 && i_time - i_previous < 250000 as libc::c_int as int64_t {
        return i_previous;
    }
    let mut i_elapsed: int64_t = i_time - i_start;
    let mut fps: libc::c_double = if i_elapsed > 0 as libc::c_int as int64_t {
        i_frame as libc::c_double * 1000000.0f64 / i_elapsed as libc::c_double
    } else {
        0 as libc::c_int as libc::c_double
    };
    let mut bitrate: libc::c_double = 0.;
    if last_ts != 0 {
        bitrate = i_file as libc::c_double * 8 as libc::c_int as libc::c_double
            / (last_ts as libc::c_double * 1000 as libc::c_int as libc::c_double
                * (*param).i_timebase_num as libc::c_double
                / (*param).i_timebase_den as libc::c_double);
    } else {
        bitrate = i_file as libc::c_double * 8 as libc::c_int as libc::c_double
            / (1000 as libc::c_int as libc::c_double
                * (*param).i_fps_den as libc::c_double
                / (*param).i_fps_num as libc::c_double);
    }
    if i_frame_total != 0 {
        let mut eta: libc::c_int = (i_elapsed * (i_frame_total - i_frame) as int64_t
            / (i_frame as int64_t * 1000000 as libc::c_int as int64_t)) as libc::c_int;
        sprintf(
            buf.as_mut_ptr(),
            b"x264 [%.1f%%] %d/%d frames, %.2f fps, %.2f kb/s, eta %d:%02d:%02d\0"
                as *const u8 as *const libc::c_char,
            100.0f64 * i_frame as libc::c_double / i_frame_total as libc::c_double,
            i_frame,
            i_frame_total,
            fps,
            bitrate,
            eta / 3600 as libc::c_int,
            eta / 60 as libc::c_int % 60 as libc::c_int,
            eta % 60 as libc::c_int,
        );
    } else {
        sprintf(
            buf.as_mut_ptr(),
            b"x264 %d frames: %.2f fps, %.2f kb/s\0" as *const u8 as *const libc::c_char,
            i_frame,
            fps,
            bitrate,
        );
    }
    fprintf(
        stderr,
        b"%s  \r\0" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr().offset(5 as libc::c_int as isize),
    );
    fflush(stderr);
    return i_time;
}
unsafe extern "C" fn convert_cli_to_lib_pic(
    mut lib: *mut x264_picture_t,
    mut cli: *mut cli_pic_t,
) {
    memcpy(
        ((*lib).img.i_stride).as_mut_ptr() as *mut libc::c_void,
        ((*cli).img.stride).as_mut_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[libc::c_int; 4]>() as libc::c_ulong,
    );
    memcpy(
        ((*lib).img.plane).as_mut_ptr() as *mut libc::c_void,
        ((*cli).img.plane).as_mut_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[*mut uint8_t; 4]>() as libc::c_ulong,
    );
    (*lib).img.i_plane = (*cli).img.planes;
    (*lib).img.i_csp = (*cli).img.csp;
    (*lib).i_pts = (*cli).pts;
}
unsafe extern "C" fn encode(
    mut param: *mut x264_param_t,
    mut opt: *mut cli_opt_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut h: *mut x264_t = 0 as *mut x264_t;
    let mut pic: x264_picture_t = x264_picture_t {
        i_type: 0,
        i_qpplus1: 0,
        i_pic_struct: 0,
        b_keyframe: 0,
        i_pts: 0,
        i_dts: 0,
        param: 0 as *mut x264_param_t,
        img: x264_image_t {
            i_csp: 0,
            i_plane: 0,
            i_stride: [0; 4],
            plane: [0 as *mut uint8_t; 4],
        },
        prop: x264_image_properties_t {
            quant_offsets: 0 as *mut libc::c_float,
            quant_offsets_free: None,
            mb_info: 0 as *mut uint8_t,
            mb_info_free: None,
            f_ssim: 0.,
            f_psnr_avg: 0.,
            f_psnr: [0.; 3],
            f_crf_avg: 0.,
        },
        hrd_timing: x264_hrd_t {
            cpb_initial_arrival_time: 0.,
            cpb_final_arrival_time: 0.,
            cpb_removal_time: 0.,
            dpb_output_time: 0.,
        },
        extra_sei: x264_sei_t {
            num_payloads: 0,
            payloads: 0 as *mut x264_sei_payload_t,
            sei_free: None,
        },
        opaque: 0 as *mut libc::c_void,
    };
    let mut cli_pic: cli_pic_t = cli_pic_t {
        img: cli_image_t {
            csp: 0,
            width: 0,
            height: 0,
            planes: 0,
            plane: [0 as *mut uint8_t; 4],
            stride: [0; 4],
        },
        pts: 0,
        duration: 0,
        opaque: 0 as *mut libc::c_void,
    };
    let mut pulldown: *const cli_pulldown_t = 0 as *const cli_pulldown_t;
    let mut i_frame: libc::c_int = 0 as libc::c_int;
    let mut i_frame_output: libc::c_int = 0 as libc::c_int;
    let mut i_end: int64_t = 0;
    let mut i_previous: int64_t = 0 as libc::c_int as int64_t;
    let mut i_start: int64_t = 0 as libc::c_int as int64_t;
    let mut i_file: int64_t = 0 as libc::c_int as int64_t;
    let mut i_frame_size: libc::c_int = 0;
    let mut last_dts: int64_t = 0 as libc::c_int as int64_t;
    let mut prev_dts: int64_t = 0 as libc::c_int as int64_t;
    let mut first_dts: int64_t = 0 as libc::c_int as int64_t;
    let mut pts_warning_cnt: libc::c_int = 0 as libc::c_int;
    let mut largest_pts: int64_t = -(1 as libc::c_int) as int64_t;
    let mut second_largest_pts: int64_t = -(1 as libc::c_int) as int64_t;
    let mut ticks_per_frame: int64_t = 0;
    let mut duration: libc::c_double = 0.;
    let mut pulldown_pts: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut retval: libc::c_int = 0 as libc::c_int;
    (*opt).b_progress &= ((*param).i_log_level < 3 as libc::c_int) as libc::c_int;
    if (*opt).i_pulldown != 0 && (*param).b_vfr_input == 0 {
        (*param).b_pulldown = 1 as libc::c_int;
        (*param).b_pic_struct = 1 as libc::c_int;
        pulldown = &*pulldown_values.as_ptr().offset((*opt).i_pulldown as isize)
            as *const cli_pulldown_t;
        (*param).i_timebase_num = (*param).i_fps_den;
        if fmod(
            ((*param).i_fps_num as libc::c_float * (*pulldown).fps_factor)
                as libc::c_double,
            1 as libc::c_int as libc::c_double,
        ) != 0.
        {
            x264_cli_log(
                b"x264\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
                b"unsupported framerate for chosen pulldown\n\0" as *const u8
                    as *const libc::c_char,
            );
            retval = -(1 as libc::c_int);
            current_block = 13915633153210047135;
        } else {
            (*param)
                .i_timebase_den = ((*param).i_fps_num as libc::c_float
                * (*pulldown).fps_factor) as uint32_t;
            current_block = 7149356873433890176;
        }
    } else {
        current_block = 7149356873433890176;
    }
    match current_block {
        7149356873433890176 => {
            h = x264_encoder_open_164(param);
            if h.is_null() {
                x264_cli_log(
                    b"x264\0" as *const u8 as *const libc::c_char,
                    0 as libc::c_int,
                    b"x264_encoder_open failed\n\0" as *const u8 as *const libc::c_char,
                );
                retval = -(1 as libc::c_int);
            } else {
                x264_encoder_parameters(h, param);
                if (cli_output.set_param)
                    .expect("non-null function pointer")((*opt).hout, param) != 0
                {
                    x264_cli_log(
                        b"x264\0" as *const u8 as *const libc::c_char,
                        0 as libc::c_int,
                        b"can't set outfile param\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    retval = -(1 as libc::c_int);
                } else {
                    i_start = x264_mdate();
                    ticks_per_frame = (*param).i_timebase_den as int64_t
                        * (*param).i_fps_den as int64_t
                        / (*param).i_timebase_num as int64_t
                        / (*param).i_fps_num as int64_t;
                    if ticks_per_frame < 1 as libc::c_int as int64_t
                        && (*param).b_vfr_input == 0
                    {
                        x264_cli_log(
                            b"x264\0" as *const u8 as *const libc::c_char,
                            0 as libc::c_int,
                            b"ticks_per_frame invalid: %ld\n\0" as *const u8
                                as *const libc::c_char,
                            ticks_per_frame,
                        );
                        retval = -(1 as libc::c_int);
                    } else {
                        ticks_per_frame = if ticks_per_frame
                            > 1 as libc::c_int as int64_t
                        {
                            ticks_per_frame
                        } else {
                            1 as libc::c_int as int64_t
                        };
                        if (*param).b_repeat_headers == 0 {
                            let mut headers: *mut x264_nal_t = 0 as *mut x264_nal_t;
                            let mut i_nal: libc::c_int = 0;
                            if x264_encoder_headers(h, &mut headers, &mut i_nal)
                                < 0 as libc::c_int
                            {
                                x264_cli_log(
                                    b"x264\0" as *const u8 as *const libc::c_char,
                                    0 as libc::c_int,
                                    b"x264_encoder_headers failed\n\0" as *const u8
                                        as *const libc::c_char,
                                );
                                retval = -(1 as libc::c_int);
                                current_block = 13915633153210047135;
                            } else {
                                i_file = (cli_output.write_headers)
                                    .expect("non-null function pointer")((*opt).hout, headers)
                                    as int64_t;
                                if i_file < 0 as libc::c_int as int64_t {
                                    x264_cli_log(
                                        b"x264\0" as *const u8 as *const libc::c_char,
                                        0 as libc::c_int,
                                        b"error writing headers to output file\n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                    retval = -(1 as libc::c_int);
                                    current_block = 13915633153210047135;
                                } else {
                                    current_block = 10758786907990354186;
                                }
                            }
                        } else {
                            current_block = 10758786907990354186;
                        }
                        match current_block {
                            13915633153210047135 => {}
                            _ => {
                                if !((*opt).tcfile_out).is_null() {
                                    fprintf(
                                        (*opt).tcfile_out,
                                        b"# timecode format v2\n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                }
                                while b_ctrl_c == 0
                                    && (i_frame < (*param).i_frame_total
                                        || (*param).i_frame_total == 0)
                                {
                                    if (filter.get_frame)
                                        .expect(
                                            "non-null function pointer",
                                        )((*opt).hin, &mut cli_pic, i_frame + (*opt).i_seek) != 0
                                    {
                                        break;
                                    }
                                    x264_picture_init(&mut pic);
                                    convert_cli_to_lib_pic(&mut pic, &mut cli_pic);
                                    if (*param).b_vfr_input == 0 {
                                        pic.i_pts = i_frame as int64_t;
                                    }
                                    if (*opt).i_pulldown != 0 && (*param).b_vfr_input == 0 {
                                        pic
                                            .i_pic_struct = (*pulldown)
                                            .pattern[(i_frame % (*pulldown).mod_0) as usize]
                                            as libc::c_int;
                                        pic.i_pts = (pulldown_pts + 0.5f64) as int64_t;
                                        pulldown_pts
                                            += pulldown_frame_duration[pic.i_pic_struct as usize]
                                                as libc::c_double;
                                    } else if (*opt).timebase_convert_multiplier != 0. {
                                        pic
                                            .i_pts = (pic.i_pts as libc::c_double
                                            * (*opt).timebase_convert_multiplier + 0.5f64) as int64_t;
                                    }
                                    if pic.i_pts <= largest_pts {
                                        if cli_log_level >= 3 as libc::c_int
                                            || pts_warning_cnt < 3 as libc::c_int
                                        {
                                            x264_cli_log(
                                                b"x264\0" as *const u8 as *const libc::c_char,
                                                1 as libc::c_int,
                                                b"non-strictly-monotonic pts at frame %d (%ld <= %ld)\n\0"
                                                    as *const u8 as *const libc::c_char,
                                                i_frame,
                                                pic.i_pts,
                                                largest_pts,
                                            );
                                        } else if pts_warning_cnt == 3 as libc::c_int {
                                            x264_cli_log(
                                                b"x264\0" as *const u8 as *const libc::c_char,
                                                1 as libc::c_int,
                                                b"too many nonmonotonic pts warnings, suppressing further ones\n\0"
                                                    as *const u8 as *const libc::c_char,
                                            );
                                        }
                                        pts_warning_cnt += 1;
                                        pts_warning_cnt;
                                        pic.i_pts = largest_pts + ticks_per_frame;
                                    }
                                    second_largest_pts = largest_pts;
                                    largest_pts = pic.i_pts;
                                    if !((*opt).tcfile_out).is_null() {
                                        fprintf(
                                            (*opt).tcfile_out,
                                            b"%.6f\n\0" as *const u8 as *const libc::c_char,
                                            pic.i_pts as libc::c_double
                                                * ((*param).i_timebase_num as libc::c_double
                                                    / (*param).i_timebase_den as libc::c_double) * 1e3f64,
                                        );
                                    }
                                    if !((*opt).qpfile).is_null() {
                                        parse_qpfile(opt, &mut pic, i_frame + (*opt).i_seek);
                                    }
                                    prev_dts = last_dts;
                                    i_frame_size = encode_frame(
                                        h,
                                        (*opt).hout,
                                        &mut pic,
                                        &mut last_dts,
                                    );
                                    if i_frame_size < 0 as libc::c_int {
                                        ::core::ptr::write_volatile(
                                            &mut b_ctrl_c as *mut libc::c_int,
                                            1 as libc::c_int,
                                        );
                                        retval = -(1 as libc::c_int);
                                    } else if i_frame_size != 0 {
                                        i_file += i_frame_size as int64_t;
                                        i_frame_output += 1;
                                        i_frame_output;
                                        if i_frame_output == 1 as libc::c_int {
                                            prev_dts = last_dts;
                                            first_dts = prev_dts;
                                        }
                                    }
                                    if (filter.release_frame)
                                        .expect(
                                            "non-null function pointer",
                                        )((*opt).hin, &mut cli_pic, i_frame + (*opt).i_seek) != 0
                                    {
                                        break;
                                    }
                                    if (*opt).b_progress != 0 && i_frame_output != 0 {
                                        i_previous = print_status(
                                            i_start,
                                            i_previous,
                                            i_frame_output,
                                            (*param).i_frame_total,
                                            i_file,
                                            param,
                                            2 as libc::c_int as int64_t * last_dts - prev_dts
                                                - first_dts,
                                        );
                                    }
                                    i_frame += 1;
                                    i_frame;
                                }
                                while b_ctrl_c == 0 && x264_encoder_delayed_frames(h) != 0 {
                                    prev_dts = last_dts;
                                    i_frame_size = encode_frame(
                                        h,
                                        (*opt).hout,
                                        0 as *mut x264_picture_t,
                                        &mut last_dts,
                                    );
                                    if i_frame_size < 0 as libc::c_int {
                                        ::core::ptr::write_volatile(
                                            &mut b_ctrl_c as *mut libc::c_int,
                                            1 as libc::c_int,
                                        );
                                        retval = -(1 as libc::c_int);
                                    } else if i_frame_size != 0 {
                                        i_file += i_frame_size as int64_t;
                                        i_frame_output += 1;
                                        i_frame_output;
                                        if i_frame_output == 1 as libc::c_int {
                                            prev_dts = last_dts;
                                            first_dts = prev_dts;
                                        }
                                    }
                                    if (*opt).b_progress != 0 && i_frame_output != 0 {
                                        i_previous = print_status(
                                            i_start,
                                            i_previous,
                                            i_frame_output,
                                            (*param).i_frame_total,
                                            i_file,
                                            param,
                                            2 as libc::c_int as int64_t * last_dts - prev_dts
                                                - first_dts,
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    if pts_warning_cnt >= 3 as libc::c_int && cli_log_level < 3 as libc::c_int {
        x264_cli_log(
            b"x264\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
            b"%d suppressed nonmonotonic pts warnings\n\0" as *const u8
                as *const libc::c_char,
            pts_warning_cnt - 3 as libc::c_int,
        );
    }
    if i_frame_output == 1 as libc::c_int {
        duration = (*param).i_fps_den as libc::c_double
            / (*param).i_fps_num as libc::c_double;
    } else if b_ctrl_c != 0 {
        duration = (2 as libc::c_int as int64_t * last_dts - prev_dts - first_dts)
            as libc::c_double * (*param).i_timebase_num as libc::c_double
            / (*param).i_timebase_den as libc::c_double;
    } else {
        duration = (2 as libc::c_int as int64_t * largest_pts - second_largest_pts)
            as libc::c_double * (*param).i_timebase_num as libc::c_double
            / (*param).i_timebase_den as libc::c_double;
    }
    i_end = x264_mdate();
    if (*opt).b_progress != 0 {
        fprintf(
            stderr,
            b"                                                                               \r\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if !h.is_null() {
        x264_encoder_close(h);
    }
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    if b_ctrl_c != 0 {
        fprintf(
            stderr,
            b"aborted at input frame %d, output frame %d\n\0" as *const u8
                as *const libc::c_char,
            (*opt).i_seek + i_frame,
            i_frame_output,
        );
    }
    (cli_output.close_file)
        .expect(
            "non-null function pointer",
        )((*opt).hout, largest_pts, second_largest_pts);
    (*opt).hout = 0 as *mut libc::c_void;
    if i_frame_output > 0 as libc::c_int {
        let mut fps: libc::c_double = i_frame_output as libc::c_double
            * 1000000 as libc::c_int as libc::c_double
            / (i_end - i_start) as libc::c_double;
        fprintf(
            stderr,
            b"encoded %d frames, %.2f fps, %.2f kb/s\n\0" as *const u8
                as *const libc::c_char,
            i_frame_output,
            fps,
            i_file as libc::c_double * 8 as libc::c_int as libc::c_double
                / (1000 as libc::c_int as libc::c_double * duration),
        );
    }
    return retval;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
