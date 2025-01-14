#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type x264_t;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fseeko(
        __stream: *mut FILE,
        __off: __off64_t,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn ftello(__stream: *mut FILE) -> __off64_t;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn x264_cli_log(
        name: *const libc::c_char,
        i_level: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    fn flv_create_writer(filename: *const libc::c_char) -> *mut flv_buffer;
    fn flv_append_data(
        c: *mut flv_buffer,
        data: *mut uint8_t,
        size: libc::c_uint,
    ) -> libc::c_int;
    fn flv_flush_data(c: *mut flv_buffer) -> libc::c_int;
    fn flv_rewrite_amf_be24(
        c: *mut flv_buffer,
        length: libc::c_uint,
        start: libc::c_uint,
    );
    fn flv_dbl2int(value: libc::c_double) -> uint64_t;
    fn flv_put_byte(c: *mut flv_buffer, b: uint8_t);
    fn flv_put_be32(c: *mut flv_buffer, val: uint32_t);
    fn flv_put_be16(c: *mut flv_buffer, val: uint16_t);
    fn flv_put_be24(c: *mut flv_buffer, val: uint32_t);
    fn flv_put_tag(c: *mut flv_buffer, tag: *const libc::c_char);
    fn flv_put_amf_string(c: *mut flv_buffer, str: *const libc::c_char);
    fn flv_put_amf_double(c: *mut flv_buffer, d: libc::c_double);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
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
pub struct flv_hnd_t {
    pub c: *mut flv_buffer,
    pub sei: *mut uint8_t,
    pub sei_len: libc::c_int,
    pub i_fps_num: int64_t,
    pub i_fps_den: int64_t,
    pub i_framenum: int64_t,
    pub i_framerate_pos: uint64_t,
    pub i_duration_pos: uint64_t,
    pub i_filesize_pos: uint64_t,
    pub i_bitrate_pos: uint64_t,
    pub b_write_length: uint8_t,
    pub i_prev_dts: int64_t,
    pub i_prev_cts: int64_t,
    pub i_delay_time: int64_t,
    pub i_init_delta: int64_t,
    pub i_delay_frames: libc::c_int,
    pub d_timebase: libc::c_double,
    pub b_vfr_input: libc::c_int,
    pub b_dts_compress: libc::c_int,
    pub start: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct flv_buffer {
    pub data: *mut uint8_t,
    pub d_cur: libc::c_uint,
    pub d_max: libc::c_uint,
    pub fp: *mut FILE,
    pub d_total: uint64_t,
}
pub const FLV_CODECID_H264: C2RustUnnamed_6 = 7;
pub const FLV_FRAME_INTER: C2RustUnnamed_7 = 32;
pub const FLV_FRAME_KEY: C2RustUnnamed_7 = 16;
pub const FLV_TAG_TYPE_VIDEO: C2RustUnnamed_5 = 9;
pub const AMF_DATA_TYPE_MIXEDARRAY: C2RustUnnamed_8 = 8;
pub const AMF_DATA_TYPE_STRING: C2RustUnnamed_8 = 2;
pub const FLV_TAG_TYPE_META: C2RustUnnamed_5 = 18;
pub type C2RustUnnamed_5 = libc::c_uint;
pub const FLV_TAG_TYPE_AUDIO: C2RustUnnamed_5 = 8;
pub type C2RustUnnamed_6 = libc::c_uint;
pub type C2RustUnnamed_7 = libc::c_uint;
pub type C2RustUnnamed_8 = libc::c_uint;
pub const AMF_DATA_TYPE_UNSUPPORTED: C2RustUnnamed_8 = 13;
pub const AMF_DATA_TYPE_LONG_STRING: C2RustUnnamed_8 = 12;
pub const AMF_DATA_TYPE_DATE: C2RustUnnamed_8 = 11;
pub const AMF_DATA_TYPE_ARRAY: C2RustUnnamed_8 = 10;
pub const AMF_DATA_TYPE_OBJECT_END: C2RustUnnamed_8 = 9;
pub const AMF_DATA_TYPE_REFERENCE: C2RustUnnamed_8 = 7;
pub const AMF_DATA_TYPE_UNDEFINED: C2RustUnnamed_8 = 6;
pub const AMF_DATA_TYPE_NULL: C2RustUnnamed_8 = 5;
pub const AMF_DATA_TYPE_OBJECT: C2RustUnnamed_8 = 3;
pub const AMF_DATA_TYPE_BOOL: C2RustUnnamed_8 = 1;
pub const AMF_DATA_TYPE_NUMBER: C2RustUnnamed_8 = 0;
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
    (file_stat.st_mode & 0o170000 as libc::c_int as __mode_t
        == 0o100000 as libc::c_int as __mode_t) as libc::c_int
}
#[inline(always)]
unsafe extern "C" fn endian_fix32(mut x: uint32_t) -> uint32_t {
    (x << 24 as libc::c_int)
        .wrapping_add((x << 8 as libc::c_int) & 0xff0000 as libc::c_int as uint32_t)
        .wrapping_add((x >> 8 as libc::c_int) & 0xff00 as libc::c_int as uint32_t)
        .wrapping_add(x >> 24 as libc::c_int)
}
#[inline(always)]
unsafe extern "C" fn endian_fix64(mut x: uint64_t) -> uint64_t {
    (endian_fix32((x >> 32 as libc::c_int) as uint32_t) as uint64_t)
        .wrapping_add((endian_fix32(x as uint32_t) as uint64_t) << 32 as libc::c_int)
}
unsafe extern "C" fn write_header(mut c: *mut flv_buffer) -> libc::c_int {
    flv_put_tag(c, b"FLV\0" as *const u8 as *const libc::c_char);
    flv_put_byte(c, 1 as libc::c_int as uint8_t);
    flv_put_byte(c, 1 as libc::c_int as uint8_t);
    flv_put_be32(c, 9 as libc::c_int as uint32_t);
    flv_put_be32(c, 0 as libc::c_int as uint32_t);
    flv_flush_data(c)
}
unsafe extern "C" fn open_file(
    mut psz_filename: *mut libc::c_char,
    mut p_handle: *mut hnd_t,
    mut opt: *mut cli_output_opt_t,
) -> libc::c_int {
    let mut p_flv: *mut flv_hnd_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<flv_hnd_t>() as libc::c_ulong,
    ) as *mut flv_hnd_t;
    if !p_flv.is_null() {
        let mut c: *mut flv_buffer = flv_create_writer(psz_filename);
        if !c.is_null() {
            if write_header(c) == 0 {
                (*p_flv).c = c;
                (*p_flv).b_dts_compress = (*opt).use_dts_compress;
                *p_handle = p_flv as hnd_t;
                return 0 as libc::c_int;
            }
            fclose((*c).fp);
            free((*c).data as *mut libc::c_void);
            free(c as *mut libc::c_void);
        }
        free(p_flv as *mut libc::c_void);
    }
    *p_handle = std::ptr::null_mut::<libc::c_void>();
    -(1 as libc::c_int)
}
unsafe extern "C" fn set_param(
    mut handle: hnd_t,
    mut p_param: *mut x264_param_t,
) -> libc::c_int {
    let mut p_flv: *mut flv_hnd_t = handle as *mut flv_hnd_t;
    let mut c: *mut flv_buffer = (*p_flv).c;
    flv_put_byte(c, FLV_TAG_TYPE_META as libc::c_int as uint8_t);
    let mut start: libc::c_int = (*c).d_cur as libc::c_int;
    flv_put_be24(c, 0 as libc::c_int as uint32_t);
    flv_put_be24(c, 0 as libc::c_int as uint32_t);
    flv_put_be32(c, 0 as libc::c_int as uint32_t);
    flv_put_byte(c, AMF_DATA_TYPE_STRING as libc::c_int as uint8_t);
    flv_put_amf_string(c, b"onMetaData\0" as *const u8 as *const libc::c_char);
    flv_put_byte(c, AMF_DATA_TYPE_MIXEDARRAY as libc::c_int as uint8_t);
    flv_put_be32(c, 7 as libc::c_int as uint32_t);
    flv_put_amf_string(c, b"width\0" as *const u8 as *const libc::c_char);
    flv_put_amf_double(c, (*p_param).i_width as libc::c_double);
    flv_put_amf_string(c, b"height\0" as *const u8 as *const libc::c_char);
    flv_put_amf_double(c, (*p_param).i_height as libc::c_double);
    flv_put_amf_string(c, b"framerate\0" as *const u8 as *const libc::c_char);
    if (*p_param).b_vfr_input == 0 {
        flv_put_amf_double(
            c,
            (*p_param).i_fps_num as libc::c_double
                / (*p_param).i_fps_den as libc::c_double,
        );
    } else {
        (*p_flv)
            .i_framerate_pos = ((*c).d_cur as uint64_t)
            .wrapping_add((*c).d_total)
            .wrapping_add(1 as libc::c_int as uint64_t);
        flv_put_amf_double(c, 0 as libc::c_int as libc::c_double);
    }
    flv_put_amf_string(c, b"videocodecid\0" as *const u8 as *const libc::c_char);
    flv_put_amf_double(c, FLV_CODECID_H264 as libc::c_int as libc::c_double);
    flv_put_amf_string(c, b"duration\0" as *const u8 as *const libc::c_char);
    (*p_flv)
        .i_duration_pos = ((*c).d_cur as uint64_t)
        .wrapping_add((*c).d_total)
        .wrapping_add(1 as libc::c_int as uint64_t);
    flv_put_amf_double(c, 0 as libc::c_int as libc::c_double);
    flv_put_amf_string(c, b"filesize\0" as *const u8 as *const libc::c_char);
    (*p_flv)
        .i_filesize_pos = ((*c).d_cur as uint64_t)
        .wrapping_add((*c).d_total)
        .wrapping_add(1 as libc::c_int as uint64_t);
    flv_put_amf_double(c, 0 as libc::c_int as libc::c_double);
    flv_put_amf_string(c, b"videodatarate\0" as *const u8 as *const libc::c_char);
    (*p_flv)
        .i_bitrate_pos = ((*c).d_cur as uint64_t)
        .wrapping_add((*c).d_total)
        .wrapping_add(1 as libc::c_int as uint64_t);
    flv_put_amf_double(c, 0 as libc::c_int as libc::c_double);
    flv_put_amf_string(c, b"\0" as *const u8 as *const libc::c_char);
    flv_put_byte(c, 0x9 as libc::c_int as uint8_t);
    let mut length: libc::c_uint = ((*c).d_cur).wrapping_sub(start as libc::c_uint);
    flv_rewrite_amf_be24(
        c,
        length.wrapping_sub(10 as libc::c_int as libc::c_uint),
        start as libc::c_uint,
    );
    flv_put_be32(c, length.wrapping_add(1 as libc::c_int as libc::c_uint));
    (*p_flv).i_fps_num = (*p_param).i_fps_num as int64_t;
    (*p_flv).i_fps_den = (*p_param).i_fps_den as int64_t;
    (*p_flv)
        .d_timebase = (*p_param).i_timebase_num as libc::c_double
        / (*p_param).i_timebase_den as libc::c_double;
    (*p_flv).b_vfr_input = (*p_param).b_vfr_input;
    (*p_flv)
        .i_delay_frames = if (*p_param).i_bframe != 0 {
        if (*p_param).i_bframe_pyramid != 0 {
            2 as libc::c_int
        } else {
            1 as libc::c_int
        }
    } else {
        0 as libc::c_int
    };
    0 as libc::c_int
}
unsafe extern "C" fn write_headers(
    mut handle: hnd_t,
    mut p_nal: *mut x264_nal_t,
) -> libc::c_int {
    let mut p_flv: *mut flv_hnd_t = handle as *mut flv_hnd_t;
    let mut c: *mut flv_buffer = (*p_flv).c;
    let mut sps_size: libc::c_int = (*p_nal.offset(0 as libc::c_int as isize)).i_payload;
    let mut pps_size: libc::c_int = (*p_nal.offset(1 as libc::c_int as isize)).i_payload;
    let mut sei_size: libc::c_int = (*p_nal.offset(2 as libc::c_int as isize)).i_payload;
    (*p_flv).sei = malloc(sei_size as libc::c_ulong) as *mut uint8_t;
    if ((*p_flv).sei).is_null() {
        return -(1 as libc::c_int);
    }
    (*p_flv).sei_len = sei_size;
    memcpy(
        (*p_flv).sei as *mut libc::c_void,
        (*p_nal.offset(2 as libc::c_int as isize)).p_payload as *const libc::c_void,
        sei_size as libc::c_ulong,
    );
    let mut sps: *mut uint8_t = ((*p_nal.offset(0 as libc::c_int as isize)).p_payload)
        .offset(4 as libc::c_int as isize);
    flv_put_byte(c, FLV_TAG_TYPE_VIDEO as libc::c_int as uint8_t);
    flv_put_be24(c, 0 as libc::c_int as uint32_t);
    flv_put_be24(c, 0 as libc::c_int as uint32_t);
    flv_put_byte(c, 0 as libc::c_int as uint8_t);
    flv_put_be24(c, 0 as libc::c_int as uint32_t);
    (*p_flv).start = (*c).d_cur;
    flv_put_byte(
        c,
        (FLV_FRAME_KEY as libc::c_int | FLV_CODECID_H264 as libc::c_int) as uint8_t,
    );
    flv_put_byte(c, 0 as libc::c_int as uint8_t);
    flv_put_be24(c, 0 as libc::c_int as uint32_t);
    flv_put_byte(c, 1 as libc::c_int as uint8_t);
    flv_put_byte(c, *sps.offset(1 as libc::c_int as isize));
    flv_put_byte(c, *sps.offset(2 as libc::c_int as isize));
    flv_put_byte(c, *sps.offset(3 as libc::c_int as isize));
    flv_put_byte(c, 0xff as libc::c_int as uint8_t);
    flv_put_byte(c, 0xe1 as libc::c_int as uint8_t);
    flv_put_be16(c, (sps_size - 4 as libc::c_int) as uint16_t);
    flv_append_data(c, sps, (sps_size - 4 as libc::c_int) as libc::c_uint);
    flv_put_byte(c, 1 as libc::c_int as uint8_t);
    flv_put_be16(c, (pps_size - 4 as libc::c_int) as uint16_t);
    flv_append_data(
        c,
        ((*p_nal.offset(1 as libc::c_int as isize)).p_payload)
            .offset(4 as libc::c_int as isize),
        (pps_size - 4 as libc::c_int) as libc::c_uint,
    );
    let mut length: libc::c_uint = ((*c).d_cur).wrapping_sub((*p_flv).start);
    flv_rewrite_amf_be24(
        c,
        length,
        ((*p_flv).start).wrapping_sub(10 as libc::c_int as libc::c_uint),
    );
    flv_put_be32(c, length.wrapping_add(11 as libc::c_int as libc::c_uint));
    if flv_flush_data(c) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    sei_size + sps_size + pps_size
}
unsafe extern "C" fn write_frame(
    mut handle: hnd_t,
    mut p_nalu: *mut uint8_t,
    mut i_size: libc::c_int,
    mut p_picture: *mut x264_picture_t,
) -> libc::c_int {
    let mut p_flv: *mut flv_hnd_t = handle as *mut flv_hnd_t;
    let mut c: *mut flv_buffer = (*p_flv).c;
    if (*p_flv).i_framenum == 0 {
        (*p_flv).i_delay_time = (*p_picture).i_dts * -(1 as libc::c_int) as int64_t;
        if (*p_flv).b_dts_compress == 0 && (*p_flv).i_delay_time != 0 {
            x264_cli_log(
                b"flv\0" as *const u8 as *const libc::c_char,
                2 as libc::c_int,
                b"initial delay %ld ms\n\0" as *const u8 as *const libc::c_char,
                (((*p_picture).i_pts + (*p_flv).i_delay_time) as libc::c_double
                    * (*p_flv).d_timebase * 1000 as libc::c_int as libc::c_double
                    + 0.5f64) as int64_t,
            );
        }
    }
    let mut dts: int64_t = 0;
    let mut cts: int64_t = 0;
    let mut offset: int64_t = 0;
    if (*p_flv).b_dts_compress != 0 {
        if (*p_flv).i_framenum == 1 as libc::c_int as int64_t {
            (*p_flv)
                .i_init_delta = (((*p_picture).i_dts + (*p_flv).i_delay_time)
                as libc::c_double * (*p_flv).d_timebase
                * 1000 as libc::c_int as libc::c_double + 0.5f64) as int64_t;
        }
        dts = if (*p_flv).i_framenum > (*p_flv).i_delay_frames as int64_t {
            ((*p_picture).i_dts as libc::c_double * (*p_flv).d_timebase
                * 1000 as libc::c_int as libc::c_double + 0.5f64) as int64_t
        } else {
            (*p_flv).i_framenum * (*p_flv).i_init_delta
                / ((*p_flv).i_delay_frames + 1 as libc::c_int) as int64_t
        };
        cts = ((*p_picture).i_pts as libc::c_double * (*p_flv).d_timebase
            * 1000 as libc::c_int as libc::c_double + 0.5f64) as int64_t;
    } else {
        dts = (((*p_picture).i_dts + (*p_flv).i_delay_time) as libc::c_double
            * (*p_flv).d_timebase * 1000 as libc::c_int as libc::c_double + 0.5f64)
            as int64_t;
        cts = (((*p_picture).i_pts + (*p_flv).i_delay_time) as libc::c_double
            * (*p_flv).d_timebase * 1000 as libc::c_int as libc::c_double + 0.5f64)
            as int64_t;
    }
    offset = cts - dts;
    if (*p_flv).i_framenum != 0 {
        if (*p_flv).i_prev_dts == dts {
            x264_cli_log(
                b"flv\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int,
                b"duplicate DTS %ld generated by rounding\n               decoding framerate cannot exceed 1000fps\n\0"
                    as *const u8 as *const libc::c_char,
                dts,
            );
        }
        if (*p_flv).i_prev_cts == cts {
            x264_cli_log(
                b"flv\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int,
                b"duplicate CTS %ld generated by rounding\n               composition framerate cannot exceed 1000fps\n\0"
                    as *const u8 as *const libc::c_char,
                cts,
            );
        }
    }
    (*p_flv).i_prev_dts = dts;
    (*p_flv).i_prev_cts = cts;
    flv_put_byte(c, FLV_TAG_TYPE_VIDEO as libc::c_int as uint8_t);
    flv_put_be24(c, 0 as libc::c_int as uint32_t);
    flv_put_be24(c, dts as uint32_t);
    flv_put_byte(c, (dts >> 24 as libc::c_int) as uint8_t);
    flv_put_be24(c, 0 as libc::c_int as uint32_t);
    (*p_flv).start = (*c).d_cur;
    flv_put_byte(
        c,
        ((if (*p_picture).b_keyframe != 0 {
            FLV_FRAME_KEY as libc::c_int
        } else {
            FLV_FRAME_INTER as libc::c_int
        }) | FLV_CODECID_H264 as libc::c_int) as uint8_t,
    );
    flv_put_byte(c, 1 as libc::c_int as uint8_t);
    flv_put_be24(c, offset as uint32_t);
    if !((*p_flv).sei).is_null() {
        flv_append_data(c, (*p_flv).sei, (*p_flv).sei_len as libc::c_uint);
        free((*p_flv).sei as *mut libc::c_void);
        (*p_flv).sei = std::ptr::null_mut::<uint8_t>();
    }
    flv_append_data(c, p_nalu, i_size as libc::c_uint);
    let mut length: libc::c_uint = ((*c).d_cur).wrapping_sub((*p_flv).start);
    flv_rewrite_amf_be24(
        c,
        length,
        ((*p_flv).start).wrapping_sub(10 as libc::c_int as libc::c_uint),
    );
    flv_put_be32(c, (11 as libc::c_int as libc::c_uint).wrapping_add(length));
    if flv_flush_data(c) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    (*p_flv).i_framenum += 1;
    (*p_flv).i_framenum;
    i_size
}
unsafe extern "C" fn rewrite_amf_double(
    mut fp: *mut FILE,
    mut position: uint64_t,
    mut value: libc::c_double,
) -> libc::c_int {
    let mut x: uint64_t = endian_fix64(flv_dbl2int(value));
    if fseeko(fp, position as __off64_t, 0 as libc::c_int) == 0
        && fwrite(
            &mut x as *mut uint64_t as *const libc::c_void,
            8 as libc::c_int as libc::c_ulong,
            1 as libc::c_int as libc::c_ulong,
            fp,
        ) == 1 as libc::c_int as libc::c_ulong
    {
        0 as libc::c_int
    } else {
        -(1 as libc::c_int)
    }
}
unsafe extern "C" fn close_file(
    mut handle: hnd_t,
    mut largest_pts: int64_t,
    mut second_largest_pts: int64_t,
) -> libc::c_int {
    let mut total_duration: libc::c_double = 0.;
    let mut current_block: u64;
    let mut ret: libc::c_int = -(1 as libc::c_int);
    let mut p_flv: *mut flv_hnd_t = handle as *mut flv_hnd_t;
    let mut c: *mut flv_buffer = (*p_flv).c;
    if flv_flush_data(c) >= 0 as libc::c_int {
        total_duration = 0.;
        if (*p_flv).i_framenum == 1 as libc::c_int as int64_t {
            total_duration = if (*p_flv).i_fps_num != 0 {
                (*p_flv).i_fps_den as libc::c_double
                    / (*p_flv).i_fps_num as libc::c_double
            } else {
                0 as libc::c_int as libc::c_double
            };
        } else {
            total_duration = (2 as libc::c_int as int64_t * largest_pts
                - second_largest_pts) as libc::c_double * (*p_flv).d_timebase;
        }
        if x264_is_regular_file((*c).fp) != 0
            && total_duration > 0 as libc::c_int as libc::c_double
        {
            let mut framerate: libc::c_double = 0.;
            let mut filesize: int64_t = ftello((*c).fp);
            if (*p_flv).i_framerate_pos != 0 {
                framerate = (*p_flv).i_framenum as libc::c_double / total_duration;
                if rewrite_amf_double((*c).fp, (*p_flv).i_framerate_pos, framerate)
                    < 0 as libc::c_int
                {
                    current_block = 4818846981140956894;
                } else {
                    current_block = 13586036798005543211;
                }
            } else {
                current_block = 13586036798005543211;
            }
            match current_block {
                4818846981140956894 => {}
                _ => {
                    if rewrite_amf_double(
                        (*c).fp,
                        (*p_flv).i_duration_pos,
                        total_duration,
                    ) < 0 as libc::c_int
                    {
                        current_block = 4818846981140956894;
                    } else if rewrite_amf_double(
                        (*c).fp,
                        (*p_flv).i_filesize_pos,
                        filesize as libc::c_double,
                    ) < 0 as libc::c_int
                    {
                        current_block = 4818846981140956894;
                    } else if rewrite_amf_double(
                        (*c).fp,
                        (*p_flv).i_bitrate_pos,
                        filesize as libc::c_double * 8.0f64
                            / (total_duration * 1000 as libc::c_int as libc::c_double),
                    ) < 0 as libc::c_int
                    {
                        current_block = 4818846981140956894;
                    } else {
                        current_block = 224731115979188411;
                    }
                }
            }
        } else {
            current_block = 224731115979188411;
        }
        match current_block {
            4818846981140956894 => {}
            _ => {
                ret = 0 as libc::c_int;
            }
        }
    }
    fclose((*c).fp);
    free((*c).data as *mut libc::c_void);
    free(c as *mut libc::c_void);
    free(p_flv as *mut libc::c_void);
    ret
}
#[no_mangle]
pub static mut flv_output: cli_output_t = unsafe {
    {
        
        cli_output_t {
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
        }
    }
};
