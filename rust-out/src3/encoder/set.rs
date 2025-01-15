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
    pub type _cl_mem;
    pub type _cl_kernel;
    pub type _cl_program;
    pub type _cl_command_queue;
    pub type _cl_device_id;
    pub type _cl_context;
    pub type _cl_platform_id;
    pub type _cl_event;
    pub type x264_ratecontrol_t;
    pub type x264_threadpool_t;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn log2f(_: libc::c_float) -> libc::c_float;
    static x264_levels: [x264_level_t; 0];
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    static x264_cqm_flat16: [uint8_t; 64];
    static x264_cqm_jvt: [*const uint8_t; 8];
    fn x264_malloc(_: int64_t) -> *mut libc::c_void;
    fn x264_free(_: *mut libc::c_void);
    fn x264_param2string(p: *mut x264_param_t, b_res: libc::c_int) -> *mut libc::c_char;
    fn x264_8_log(h: *mut x264_t, i_level: libc::c_int, psz_fmt: *const libc::c_char, _: ...);
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
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type intptr_t = libc::c_long;
pub type uintptr_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __atomic_wide_counter {
    pub __value64: libc::c_ulonglong,
    pub __value32: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub __wseq: __atomic_wide_counter,
    pub __g1_start: __atomic_wide_counter,
    pub __g_refs: [libc::c_uint; 2],
    pub __g_size: [libc::c_uint; 2],
    pub __g1_orig_size: libc::c_uint,
    pub __wrefs: libc::c_uint,
    pub __g_signals: [libc::c_uint; 2],
}
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [libc::c_char; 48],
    pub __align: libc::c_longlong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_t {
    pub param: x264_param_t,
    pub api: *mut libc::c_void,
    pub thread: [*mut x264_t; 129],
    pub lookahead_thread: [*mut x264_t; 16],
    pub b_thread_active: libc::c_int,
    pub i_thread_phase: libc::c_int,
    pub i_thread_idx: libc::c_int,
    pub i_threadslice_start: libc::c_int,
    pub i_threadslice_end: libc::c_int,
    pub i_threadslice_pass: libc::c_int,
    pub threadpool: *mut x264_threadpool_t,
    pub lookaheadpool: *mut x264_threadpool_t,
    pub mutex: pthread_mutex_t,
    pub cv: pthread_cond_t,
    pub out: C2RustUnnamed_19,
    pub nal_buffer: *mut uint8_t,
    pub nal_buffer_size: libc::c_int,
    pub reconfig_h: *mut x264_t,
    pub reconfig: libc::c_int,
    pub i_frame: libc::c_int,
    pub i_frame_num: libc::c_int,
    pub i_thread_frames: libc::c_int,
    pub i_nal_type: libc::c_int,
    pub i_nal_ref_idc: libc::c_int,
    pub i_disp_fields: int64_t,
    pub i_disp_fields_last_frame: libc::c_int,
    pub i_prev_duration: int64_t,
    pub i_coded_fields: int64_t,
    pub i_cpb_delay: int64_t,
    pub i_coded_fields_lookahead: int64_t,
    pub i_cpb_delay_lookahead: int64_t,
    pub i_cpb_delay_pir_offset: int64_t,
    pub i_cpb_delay_pir_offset_next: int64_t,
    pub b_queued_intra_refresh: libc::c_int,
    pub i_last_idr_pts: int64_t,
    pub i_idr_pic_id: libc::c_int,
    pub dequant4_mf: [*mut [libc::c_int; 16]; 4],
    pub dequant8_mf: [*mut [libc::c_int; 64]; 4],
    pub unquant4_mf: [*mut [libc::c_int; 16]; 4],
    pub unquant8_mf: [*mut [libc::c_int; 64]; 4],
    pub quant4_mf: [*mut [udctcoef; 16]; 4],
    pub quant8_mf: [*mut [udctcoef; 64]; 4],
    pub quant4_bias: [*mut [udctcoef; 16]; 4],
    pub quant8_bias: [*mut [udctcoef; 64]; 4],
    pub quant4_bias0: [*mut [udctcoef; 16]; 4],
    pub quant8_bias0: [*mut [udctcoef; 64]; 4],
    pub nr_offset_emergency: *mut [[udctcoef; 64]; 4],
    pub cost_mv: [*mut uint16_t; 70],
    pub cost_mv_fpel: [[*mut uint16_t; 4]; 70],
    pub cost_table: *mut C2RustUnnamed_18,
    pub chroma_qp_table: *const uint8_t,
    pub sh: x264_slice_header_t,
    pub sps: [x264_sps_t; 1],
    pub pps: [x264_pps_t; 1],
    pub b_sh_backup: libc::c_int,
    pub sh_backup: x264_slice_header_t,
    pub cabac: x264_cabac_t,
    pub frames: C2RustUnnamed_12,
    pub fenc: *mut x264_frame_t,
    pub fdec: *mut x264_frame_t,
    pub i_ref: [libc::c_int; 2],
    pub fref: [[*mut x264_frame_t; 19]; 2],
    pub fref_nearest: [*mut x264_frame_t; 2],
    pub b_ref_reorder: [libc::c_int; 2],
    pub initial_cpb_removal_delay: libc::c_int,
    pub initial_cpb_removal_delay_offset: libc::c_int,
    pub i_reordered_pts_delay: int64_t,
    pub dct: C2RustUnnamed_11,
    pub mb: C2RustUnnamed_8,
    pub rc: *mut x264_ratecontrol_t,
    pub stat: C2RustUnnamed_7,
    pub nr_offset: *mut [udctcoef; 64],
    pub nr_residual_sum: *mut [uint32_t; 64],
    pub nr_count: *mut uint32_t,
    pub nr_offset_denoise: [[udctcoef; 64]; 4],
    pub nr_residual_sum_buf: [[[uint32_t; 64]; 4]; 2],
    pub nr_count_buf: [[uint32_t; 4]; 2],
    pub luma2chroma_pixel: [uint8_t; 7],
    pub scratch_buffer: *mut libc::c_void,
    pub scratch_buffer2: *mut libc::c_void,
    pub intra_border_backup: [[*mut pixel; 3]; 5],
    pub deblock_strength: [*mut [[[uint8_t; 4]; 8]; 2]; 2],
    pub predict_16x16: [x264_predict_t; 7],
    pub predict_8x8: [x264_predict8x8_t; 12],
    pub predict_4x4: [x264_predict_t; 12],
    pub predict_chroma: [x264_predict_t; 7],
    pub predict_8x8c: [x264_predict_t; 7],
    pub predict_8x16c: [x264_predict_t; 7],
    pub predict_8x8_filter: x264_predict_8x8_filter_t,
    pub pixf: x264_pixel_function_t,
    pub mc: x264_mc_functions_t,
    pub dctf: x264_dct_function_t,
    pub zigzagf: x264_zigzag_function_t,
    pub zigzagf_interlaced: x264_zigzag_function_t,
    pub zigzagf_progressive: x264_zigzag_function_t,
    pub quantf: x264_quant_function_t,
    pub loopf: x264_deblock_function_t,
    pub bsf: x264_bitstream_function_t,
    pub lookahead: *mut x264_lookahead_t,
    pub opencl: x264_opencl_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_opencl_t {
    pub ocl: *mut x264_opencl_function_t,
    pub context: cl_context,
    pub device: cl_device_id,
    pub queue: cl_command_queue,
    pub lookahead_program: cl_program,
    pub last_buf: cl_int,
    pub page_locked_buffer: cl_mem,
    pub page_locked_ptr: *mut libc::c_char,
    pub pl_occupancy: libc::c_int,
    pub copies: [C2RustUnnamed_0; 1024],
    pub num_copies: libc::c_int,
    pub b_device_AMD_SI: libc::c_int,
    pub b_fatal_error: libc::c_int,
    pub lookahead_thread_pri: libc::c_int,
    pub opencl_thread_pri: libc::c_int,
    pub downscale_hpel_kernel: cl_kernel,
    pub downscale_kernel1: cl_kernel,
    pub downscale_kernel2: cl_kernel,
    pub luma_16x16_image: [cl_mem; 2],
    pub weightp_hpel_kernel: cl_kernel,
    pub weightp_scaled_images_kernel: cl_kernel,
    pub weighted_scaled_images: [cl_mem; 4],
    pub weighted_luma_hpel: cl_mem,
    pub memset_kernel: cl_kernel,
    pub intra_kernel: cl_kernel,
    pub rowsum_intra_kernel: cl_kernel,
    pub row_satds: [cl_mem; 2],
    pub hme_kernel: cl_kernel,
    pub subpel_refine_kernel: cl_kernel,
    pub mv_buffers: [cl_mem; 2],
    pub lowres_mv_costs: cl_mem,
    pub mvp_buffer: cl_mem,
    pub mode_select_kernel: cl_kernel,
    pub rowsum_inter_kernel: cl_kernel,
    pub lowres_costs: [cl_mem; 2],
    pub frame_stats: [cl_mem; 2],
}
pub type cl_mem = *mut _cl_mem;
pub type cl_kernel = *mut _cl_kernel;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub src: *mut libc::c_void,
    pub dest: *mut libc::c_void,
    pub bytes: libc::c_int,
}
pub type cl_int = int32_t;
pub type cl_program = *mut _cl_program;
pub type cl_command_queue = *mut _cl_command_queue;
pub type cl_device_id = *mut _cl_device_id;
pub type cl_context = *mut _cl_context;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_opencl_function_t {
    pub library: *mut libc::c_void,
    pub clBuildProgram: clBuildProgram_func,
    pub clCreateBuffer: clCreateBuffer_func,
    pub clCreateCommandQueue: clCreateCommandQueue_func,
    pub clCreateContext: clCreateContext_func,
    pub clCreateImage2D: clCreateImage2D_func,
    pub clCreateKernel: clCreateKernel_func,
    pub clCreateProgramWithBinary: clCreateProgramWithBinary_func,
    pub clCreateProgramWithSource: clCreateProgramWithSource_func,
    pub clEnqueueCopyBuffer: clEnqueueCopyBuffer_func,
    pub clEnqueueMapBuffer: clEnqueueMapBuffer_func,
    pub clEnqueueNDRangeKernel: clEnqueueNDRangeKernel_func,
    pub clEnqueueReadBuffer: clEnqueueReadBuffer_func,
    pub clEnqueueWriteBuffer: clEnqueueWriteBuffer_func,
    pub clFinish: clFinish_func,
    pub clGetCommandQueueInfo: clGetCommandQueueInfo_func,
    pub clGetDeviceIDs: clGetDeviceIDs_func,
    pub clGetDeviceInfo: clGetDeviceInfo_func,
    pub clGetKernelWorkGroupInfo: clGetKernelWorkGroupInfo_func,
    pub clGetPlatformIDs: clGetPlatformIDs_func,
    pub clGetProgramBuildInfo: clGetProgramBuildInfo_func,
    pub clGetProgramInfo: clGetProgramInfo_func,
    pub clGetSupportedImageFormats: clGetSupportedImageFormats_func,
    pub clReleaseCommandQueue: clReleaseCommandQueue_func,
    pub clReleaseContext: clReleaseContext_func,
    pub clReleaseKernel: clReleaseKernel_func,
    pub clReleaseMemObject: clReleaseMemObject_func,
    pub clReleaseProgram: clReleaseProgram_func,
    pub clSetKernelArg: clSetKernelArg_func,
}
pub type clSetKernelArg_func =
    Option<unsafe extern "C" fn(cl_kernel, cl_uint, size_t, *const libc::c_void) -> cl_int>;
pub type cl_uint = uint32_t;
pub type clReleaseProgram_func = Option<unsafe extern "C" fn(cl_program) -> cl_int>;
pub type clReleaseMemObject_func = Option<unsafe extern "C" fn(cl_mem) -> cl_int>;
pub type clReleaseKernel_func = Option<unsafe extern "C" fn(cl_kernel) -> cl_int>;
pub type clReleaseContext_func = Option<unsafe extern "C" fn(cl_context) -> cl_int>;
pub type clReleaseCommandQueue_func = Option<unsafe extern "C" fn(cl_command_queue) -> cl_int>;
pub type clGetSupportedImageFormats_func = Option<
    unsafe extern "C" fn(
        cl_context,
        cl_mem_flags,
        cl_mem_object_type,
        cl_uint,
        *mut cl_image_format,
        *mut cl_uint,
    ) -> cl_int,
>;
pub type cl_image_format = _cl_image_format;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _cl_image_format {
    pub image_channel_order: cl_channel_order,
    pub image_channel_data_type: cl_channel_type,
}
pub type cl_channel_type = cl_uint;
pub type cl_channel_order = cl_uint;
pub type cl_mem_object_type = cl_uint;
pub type cl_mem_flags = cl_bitfield;
pub type cl_bitfield = cl_ulong;
pub type cl_ulong = uint64_t;
pub type clGetProgramInfo_func = Option<
    unsafe extern "C" fn(
        cl_program,
        cl_program_info,
        size_t,
        *mut libc::c_void,
        *mut size_t,
    ) -> cl_int,
>;
pub type cl_program_info = cl_uint;
pub type clGetProgramBuildInfo_func = Option<
    unsafe extern "C" fn(
        cl_program,
        cl_device_id,
        cl_program_build_info,
        size_t,
        *mut libc::c_void,
        *mut size_t,
    ) -> cl_int,
>;
pub type cl_program_build_info = cl_uint;
pub type clGetPlatformIDs_func =
    Option<unsafe extern "C" fn(cl_uint, *mut cl_platform_id, *mut cl_uint) -> cl_int>;
pub type cl_platform_id = *mut _cl_platform_id;
pub type clGetKernelWorkGroupInfo_func = Option<
    unsafe extern "C" fn(
        cl_kernel,
        cl_device_id,
        cl_kernel_work_group_info,
        size_t,
        *mut libc::c_void,
        *mut size_t,
    ) -> cl_int,
>;
pub type cl_kernel_work_group_info = cl_uint;
pub type clGetDeviceInfo_func = Option<
    unsafe extern "C" fn(
        cl_device_id,
        cl_device_info,
        size_t,
        *mut libc::c_void,
        *mut size_t,
    ) -> cl_int,
>;
pub type cl_device_info = cl_uint;
pub type clGetDeviceIDs_func = Option<
    unsafe extern "C" fn(
        cl_platform_id,
        cl_device_type,
        cl_uint,
        *mut cl_device_id,
        *mut cl_uint,
    ) -> cl_int,
>;
pub type cl_device_type = cl_bitfield;
pub type clGetCommandQueueInfo_func = Option<
    unsafe extern "C" fn(
        cl_command_queue,
        cl_command_queue_info,
        size_t,
        *mut libc::c_void,
        *mut size_t,
    ) -> cl_int,
>;
pub type cl_command_queue_info = cl_uint;
pub type clFinish_func = Option<unsafe extern "C" fn(cl_command_queue) -> cl_int>;
pub type clEnqueueWriteBuffer_func = Option<
    unsafe extern "C" fn(
        cl_command_queue,
        cl_mem,
        cl_bool,
        size_t,
        size_t,
        *const libc::c_void,
        cl_uint,
        *const cl_event,
        *mut cl_event,
    ) -> cl_int,
>;
pub type cl_event = *mut _cl_event;
pub type cl_bool = cl_uint;
pub type clEnqueueReadBuffer_func = Option<
    unsafe extern "C" fn(
        cl_command_queue,
        cl_mem,
        cl_bool,
        size_t,
        size_t,
        *mut libc::c_void,
        cl_uint,
        *const cl_event,
        *mut cl_event,
    ) -> cl_int,
>;
pub type clEnqueueNDRangeKernel_func = Option<
    unsafe extern "C" fn(
        cl_command_queue,
        cl_kernel,
        cl_uint,
        *const size_t,
        *const size_t,
        *const size_t,
        cl_uint,
        *const cl_event,
        *mut cl_event,
    ) -> cl_int,
>;
pub type clEnqueueMapBuffer_func = Option<
    unsafe extern "C" fn(
        cl_command_queue,
        cl_mem,
        cl_bool,
        cl_map_flags,
        size_t,
        size_t,
        cl_uint,
        *const cl_event,
        *mut cl_event,
        *mut cl_int,
    ) -> *mut libc::c_void,
>;
pub type cl_map_flags = cl_bitfield;
pub type clEnqueueCopyBuffer_func = Option<
    unsafe extern "C" fn(
        cl_command_queue,
        cl_mem,
        cl_mem,
        size_t,
        size_t,
        size_t,
        cl_uint,
        *const cl_event,
        *mut cl_event,
    ) -> cl_int,
>;
pub type clCreateProgramWithSource_func = Option<
    unsafe extern "C" fn(
        cl_context,
        cl_uint,
        *mut *const libc::c_char,
        *const size_t,
        *mut cl_int,
    ) -> cl_program,
>;
pub type clCreateProgramWithBinary_func = Option<
    unsafe extern "C" fn(
        cl_context,
        cl_uint,
        *const cl_device_id,
        *const size_t,
        *mut *const libc::c_uchar,
        *mut cl_int,
        *mut cl_int,
    ) -> cl_program,
>;
pub type clCreateKernel_func =
    Option<unsafe extern "C" fn(cl_program, *const libc::c_char, *mut cl_int) -> cl_kernel>;
pub type clCreateImage2D_func = Option<
    unsafe extern "C" fn(
        cl_context,
        cl_mem_flags,
        *const cl_image_format,
        size_t,
        size_t,
        size_t,
        *mut libc::c_void,
        *mut cl_int,
    ) -> cl_mem,
>;
pub type clCreateContext_func = Option<
    unsafe extern "C" fn(
        *const cl_context_properties,
        cl_uint,
        *const cl_device_id,
        Option<
            unsafe extern "C" fn(
                *const libc::c_char,
                *const libc::c_void,
                size_t,
                *mut libc::c_void,
            ) -> (),
        >,
        *mut libc::c_void,
        *mut cl_int,
    ) -> cl_context,
>;
pub type cl_context_properties = intptr_t;
pub type clCreateCommandQueue_func = Option<
    unsafe extern "C" fn(
        cl_context,
        cl_device_id,
        cl_command_queue_properties,
        *mut cl_int,
    ) -> cl_command_queue,
>;
pub type cl_command_queue_properties = cl_bitfield;
pub type clCreateBuffer_func = Option<
    unsafe extern "C" fn(
        cl_context,
        cl_mem_flags,
        size_t,
        *mut libc::c_void,
        *mut cl_int,
    ) -> cl_mem,
>;
pub type clBuildProgram_func = Option<
    unsafe extern "C" fn(
        cl_program,
        cl_uint,
        *const cl_device_id,
        *const libc::c_char,
        Option<unsafe extern "C" fn(cl_program, *mut libc::c_void) -> ()>,
        *mut libc::c_void,
    ) -> cl_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_lookahead_t {
    pub b_exit_thread: uint8_t,
    pub b_thread_active: uint8_t,
    pub b_analyse_keyframe: uint8_t,
    pub i_last_keyframe: libc::c_int,
    pub i_slicetype_length: libc::c_int,
    pub last_nonb: *mut x264_frame_t,
    pub thread_handle: pthread_t,
    pub ifbuf: x264_sync_frame_list_t,
    pub next: x264_sync_frame_list_t,
    pub ofbuf: x264_sync_frame_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_sync_frame_list_t {
    pub list: *mut *mut x264_frame_t,
    pub i_max_size: libc::c_int,
    pub i_size: libc::c_int,
    pub mutex: pthread_mutex_t,
    pub cv_fill: pthread_cond_t,
    pub cv_empty: pthread_cond_t,
}
pub type x264_frame_t = x264_frame;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_frame {
    pub base: *mut uint8_t,
    pub i_poc: libc::c_int,
    pub i_delta_poc: [libc::c_int; 2],
    pub i_type: libc::c_int,
    pub i_forced_type: libc::c_int,
    pub i_qpplus1: libc::c_int,
    pub i_pts: int64_t,
    pub i_dts: int64_t,
    pub i_reordered_pts: int64_t,
    pub i_duration: int64_t,
    pub f_duration: libc::c_float,
    pub i_cpb_duration: int64_t,
    pub i_cpb_delay: int64_t,
    pub i_dpb_output_delay: int64_t,
    pub param: *mut x264_param_t,
    pub i_frame: libc::c_int,
    pub i_coded: libc::c_int,
    pub i_field_cnt: int64_t,
    pub i_frame_num: libc::c_int,
    pub b_kept_as_ref: libc::c_int,
    pub i_pic_struct: libc::c_int,
    pub b_keyframe: libc::c_int,
    pub b_fdec: uint8_t,
    pub b_last_minigop_bframe: uint8_t,
    pub i_bframes: uint8_t,
    pub f_qp_avg_rc: libc::c_float,
    pub f_qp_avg_aq: libc::c_float,
    pub f_crf_avg: libc::c_float,
    pub i_poc_l0ref0: libc::c_int,
    pub i_csp: libc::c_int,
    pub i_plane: libc::c_int,
    pub i_stride: [libc::c_int; 3],
    pub i_width: [libc::c_int; 3],
    pub i_lines: [libc::c_int; 3],
    pub i_stride_lowres: libc::c_int,
    pub i_width_lowres: libc::c_int,
    pub i_lines_lowres: libc::c_int,
    pub plane: [*mut pixel; 3],
    pub plane_fld: [*mut pixel; 3],
    pub filtered: [[*mut pixel; 4]; 3],
    pub filtered_fld: [[*mut pixel; 4]; 3],
    pub lowres: [*mut pixel; 4],
    pub integral: *mut uint16_t,
    pub buffer: [*mut pixel; 4],
    pub buffer_fld: [*mut pixel; 4],
    pub buffer_lowres: *mut pixel,
    pub weight: [[x264_weight_t; 3]; 16],
    pub weighted: [*mut pixel; 16],
    pub b_duplicate: libc::c_int,
    pub orig: *mut x264_frame,
    pub mb_type: *mut int8_t,
    pub mb_partition: *mut uint8_t,
    pub mv: [*mut [int16_t; 2]; 2],
    pub mv16x16: *mut [int16_t; 2],
    pub lowres_mvs: [[*mut [int16_t; 2]; 17]; 2],
    pub field: *mut uint8_t,
    pub effective_qp: *mut uint8_t,
    pub lowres_costs: [[*mut uint16_t; 18]; 18],
    pub lowres_mv_costs: [[*mut libc::c_int; 17]; 2],
    pub ref_0: [*mut int8_t; 2],
    pub i_ref: [libc::c_int; 2],
    pub ref_poc: [[libc::c_int; 16]; 2],
    pub inv_ref_poc: [int16_t; 2],
    pub i_cost_est: [[libc::c_int; 18]; 18],
    pub i_cost_est_aq: [[libc::c_int; 18]; 18],
    pub i_satd: libc::c_int,
    pub i_intra_mbs: [libc::c_int; 18],
    pub i_row_satds: [[*mut libc::c_int; 18]; 18],
    pub i_row_satd: *mut libc::c_int,
    pub i_row_bits: *mut libc::c_int,
    pub f_row_qp: *mut libc::c_float,
    pub f_row_qscale: *mut libc::c_float,
    pub f_qp_offset: *mut libc::c_float,
    pub f_qp_offset_aq: *mut libc::c_float,
    pub b_intra_calculated: libc::c_int,
    pub i_intra_cost: *mut uint16_t,
    pub i_propagate_cost: *mut uint16_t,
    pub i_inv_qscale_factor: *mut uint16_t,
    pub b_scenecut: libc::c_int,
    pub f_weighted_cost_delta: [libc::c_float; 18],
    pub i_pixel_sum: [uint32_t; 3],
    pub i_pixel_ssd: [uint64_t; 3],
    pub hrd_timing: x264_hrd_t,
    pub i_planned_type: [uint8_t; 251],
    pub i_planned_satd: [libc::c_int; 251],
    pub f_planned_cpb_duration: [libc::c_double; 251],
    pub i_coded_fields_lookahead: int64_t,
    pub i_cpb_delay_lookahead: int64_t,
    pub i_lines_completed: libc::c_int,
    pub i_lines_weighted: libc::c_int,
    pub i_reference_count: libc::c_int,
    pub mutex: pthread_mutex_t,
    pub cv: pthread_cond_t,
    pub i_slice_count: libc::c_int,
    pub f_pir_position: libc::c_float,
    pub i_pir_start_col: libc::c_int,
    pub i_pir_end_col: libc::c_int,
    pub i_frames_since_pir: libc::c_int,
    pub b_corrupt: libc::c_int,
    pub extra_sei: x264_sei_t,
    pub opaque: *mut libc::c_void,
    pub mb_info: *mut uint8_t,
    pub mb_info_free: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub opencl: x264_frame_opencl_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_frame_opencl_t {
    pub ocl: *mut x264_opencl_function_t,
    pub scaled_image2Ds: [cl_mem; 4],
    pub luma_hpel: cl_mem,
    pub inv_qscale_factor: cl_mem,
    pub intra_cost: cl_mem,
    pub lowres_mvs0: cl_mem,
    pub lowres_mvs1: cl_mem,
    pub lowres_mv_costs0: cl_mem,
    pub lowres_mv_costs1: cl_mem,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_sei_t {
    pub num_payloads: libc::c_int,
    pub payloads: *mut x264_sei_payload_t,
    pub sei_free: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
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
pub type pixel = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_weight_t {
    pub cachea: [int16_t; 8],
    pub cacheb: [int16_t; 8],
    pub i_denom: int32_t,
    pub i_scale: int32_t,
    pub i_offset: int32_t,
    pub weightfn: *mut weight_fn_t,
}
pub type weight_fn_t = Option<
    unsafe extern "C" fn(
        *mut pixel,
        intptr_t,
        *mut pixel,
        intptr_t,
        *const x264_weight_t,
        libc::c_int,
    ) -> (),
>;
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
    pub vui: C2RustUnnamed_6,
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
    pub analyse: C2RustUnnamed_5,
    pub rc: C2RustUnnamed_4,
    pub crop_rect: C2RustUnnamed_3,
    pub i_frame_packing: libc::c_int,
    pub mastering_display: C2RustUnnamed_2,
    pub content_light_level: C2RustUnnamed_1,
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
pub struct C2RustUnnamed_1 {
    pub b_cll: libc::c_int,
    pub i_max_cll: libc::c_int,
    pub i_max_fall: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
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
pub struct C2RustUnnamed_3 {
    pub i_left: libc::c_int,
    pub i_top: libc::c_int,
    pub i_right: libc::c_int,
    pub i_bottom: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
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
pub struct C2RustUnnamed_5 {
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
pub struct C2RustUnnamed_6 {
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
pub struct x264_bitstream_function_t {
    pub nal_escape:
        Option<unsafe extern "C" fn(*mut uint8_t, *mut uint8_t, *mut uint8_t) -> *mut uint8_t>,
    pub cabac_block_residual_internal:
        Option<unsafe extern "C" fn(*mut dctcoef, libc::c_int, intptr_t, *mut x264_cabac_t) -> ()>,
    pub cabac_block_residual_rd_internal:
        Option<unsafe extern "C" fn(*mut dctcoef, libc::c_int, intptr_t, *mut x264_cabac_t) -> ()>,
    pub cabac_block_residual_8x8_rd_internal:
        Option<unsafe extern "C" fn(*mut dctcoef, libc::c_int, intptr_t, *mut x264_cabac_t) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_cabac_t {
    pub i_low: libc::c_int,
    pub i_range: libc::c_int,
    pub i_queue: libc::c_int,
    pub i_bytes_outstanding: libc::c_int,
    pub p_start: *mut uint8_t,
    pub p: *mut uint8_t,
    pub p_end: *mut uint8_t,
    pub f8_bits_encoded: libc::c_int,
    pub state: [uint8_t; 1024],
    pub padding: [uint8_t; 12],
}
pub type dctcoef = int16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_deblock_function_t {
    pub deblock_luma: [x264_deblock_inter_t; 2],
    pub deblock_chroma: [x264_deblock_inter_t; 2],
    pub deblock_h_chroma_420: x264_deblock_inter_t,
    pub deblock_h_chroma_422: x264_deblock_inter_t,
    pub deblock_luma_intra: [x264_deblock_intra_t; 2],
    pub deblock_chroma_intra: [x264_deblock_intra_t; 2],
    pub deblock_h_chroma_420_intra: x264_deblock_intra_t,
    pub deblock_h_chroma_422_intra: x264_deblock_intra_t,
    pub deblock_luma_mbaff: x264_deblock_inter_t,
    pub deblock_chroma_mbaff: x264_deblock_inter_t,
    pub deblock_chroma_420_mbaff: x264_deblock_inter_t,
    pub deblock_chroma_422_mbaff: x264_deblock_inter_t,
    pub deblock_luma_intra_mbaff: x264_deblock_intra_t,
    pub deblock_chroma_intra_mbaff: x264_deblock_intra_t,
    pub deblock_chroma_420_intra_mbaff: x264_deblock_intra_t,
    pub deblock_chroma_422_intra_mbaff: x264_deblock_intra_t,
    pub deblock_strength: Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            *mut [int8_t; 40],
            *mut [[int16_t; 2]; 40],
            *mut [[uint8_t; 4]; 8],
            libc::c_int,
            libc::c_int,
        ) -> (),
    >,
}
pub type x264_deblock_intra_t =
    Option<unsafe extern "C" fn(*mut pixel, intptr_t, libc::c_int, libc::c_int) -> ()>;
pub type x264_deblock_inter_t =
    Option<unsafe extern "C" fn(*mut pixel, intptr_t, libc::c_int, libc::c_int, *mut int8_t) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_quant_function_t {
    pub quant_8x8:
        Option<unsafe extern "C" fn(*mut dctcoef, *mut udctcoef, *mut udctcoef) -> libc::c_int>,
    pub quant_4x4:
        Option<unsafe extern "C" fn(*mut dctcoef, *mut udctcoef, *mut udctcoef) -> libc::c_int>,
    pub quant_4x4x4: Option<
        unsafe extern "C" fn(*mut [dctcoef; 16], *mut udctcoef, *mut udctcoef) -> libc::c_int,
    >,
    pub quant_4x4_dc:
        Option<unsafe extern "C" fn(*mut dctcoef, libc::c_int, libc::c_int) -> libc::c_int>,
    pub quant_2x2_dc:
        Option<unsafe extern "C" fn(*mut dctcoef, libc::c_int, libc::c_int) -> libc::c_int>,
    pub dequant_8x8:
        Option<unsafe extern "C" fn(*mut dctcoef, *mut [libc::c_int; 64], libc::c_int) -> ()>,
    pub dequant_4x4:
        Option<unsafe extern "C" fn(*mut dctcoef, *mut [libc::c_int; 16], libc::c_int) -> ()>,
    pub dequant_4x4_dc:
        Option<unsafe extern "C" fn(*mut dctcoef, *mut [libc::c_int; 16], libc::c_int) -> ()>,
    pub idct_dequant_2x4_dc: Option<
        unsafe extern "C" fn(
            *mut dctcoef,
            *mut [dctcoef; 16],
            *mut [libc::c_int; 16],
            libc::c_int,
        ) -> (),
    >,
    pub idct_dequant_2x4_dconly:
        Option<unsafe extern "C" fn(*mut dctcoef, *mut [libc::c_int; 16], libc::c_int) -> ()>,
    pub optimize_chroma_2x2_dc:
        Option<unsafe extern "C" fn(*mut dctcoef, libc::c_int) -> libc::c_int>,
    pub optimize_chroma_2x4_dc:
        Option<unsafe extern "C" fn(*mut dctcoef, libc::c_int) -> libc::c_int>,
    pub denoise_dct:
        Option<unsafe extern "C" fn(*mut dctcoef, *mut uint32_t, *mut udctcoef, libc::c_int) -> ()>,
    pub decimate_score15: Option<unsafe extern "C" fn(*mut dctcoef) -> libc::c_int>,
    pub decimate_score16: Option<unsafe extern "C" fn(*mut dctcoef) -> libc::c_int>,
    pub decimate_score64: Option<unsafe extern "C" fn(*mut dctcoef) -> libc::c_int>,
    pub coeff_last: [Option<unsafe extern "C" fn(*mut dctcoef) -> libc::c_int>; 14],
    pub coeff_last4: Option<unsafe extern "C" fn(*mut dctcoef) -> libc::c_int>,
    pub coeff_last8: Option<unsafe extern "C" fn(*mut dctcoef) -> libc::c_int>,
    pub coeff_level_run:
        [Option<unsafe extern "C" fn(*mut dctcoef, *mut x264_run_level_t) -> libc::c_int>; 13],
    pub coeff_level_run4:
        Option<unsafe extern "C" fn(*mut dctcoef, *mut x264_run_level_t) -> libc::c_int>,
    pub coeff_level_run8:
        Option<unsafe extern "C" fn(*mut dctcoef, *mut x264_run_level_t) -> libc::c_int>,
    pub trellis_cabac_4x4: Option<
        unsafe extern "C" fn(
            *const libc::c_int,
            *const uint8_t,
            libc::c_int,
            libc::c_int,
            *mut dctcoef,
            *mut dctcoef,
            *mut dctcoef,
            *mut uint8_t,
            *mut uint8_t,
            uint64_t,
            uint16_t,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub trellis_cabac_8x8: Option<
        unsafe extern "C" fn(
            *const libc::c_int,
            *const uint8_t,
            libc::c_int,
            libc::c_int,
            *mut dctcoef,
            *mut dctcoef,
            *mut dctcoef,
            *mut uint8_t,
            *mut uint8_t,
            uint64_t,
            uint16_t,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub trellis_cabac_4x4_psy: Option<
        unsafe extern "C" fn(
            *const libc::c_int,
            *const uint8_t,
            libc::c_int,
            libc::c_int,
            *mut dctcoef,
            *mut dctcoef,
            *mut dctcoef,
            *mut uint8_t,
            *mut uint8_t,
            uint64_t,
            uint16_t,
            libc::c_int,
            *mut dctcoef,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub trellis_cabac_8x8_psy: Option<
        unsafe extern "C" fn(
            *const libc::c_int,
            *const uint8_t,
            libc::c_int,
            libc::c_int,
            *mut dctcoef,
            *mut dctcoef,
            *mut dctcoef,
            *mut uint8_t,
            *mut uint8_t,
            uint64_t,
            uint16_t,
            libc::c_int,
            *mut dctcoef,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub trellis_cabac_dc: Option<
        unsafe extern "C" fn(
            *const libc::c_int,
            *const uint8_t,
            libc::c_int,
            libc::c_int,
            *mut dctcoef,
            *mut dctcoef,
            *mut dctcoef,
            *mut uint8_t,
            *mut uint8_t,
            uint64_t,
            uint16_t,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub trellis_cabac_chroma_422_dc: Option<
        unsafe extern "C" fn(
            *const libc::c_int,
            *const uint8_t,
            libc::c_int,
            libc::c_int,
            *mut dctcoef,
            *mut dctcoef,
            *mut dctcoef,
            *mut uint8_t,
            *mut uint8_t,
            uint64_t,
            uint16_t,
        ) -> libc::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_run_level_t {
    pub last: int32_t,
    pub mask: int32_t,
    pub level: [dctcoef; 18],
}
pub type udctcoef = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_zigzag_function_t {
    pub scan_8x8: Option<unsafe extern "C" fn(*mut dctcoef, *mut dctcoef) -> ()>,
    pub scan_4x4: Option<unsafe extern "C" fn(*mut dctcoef, *mut dctcoef) -> ()>,
    pub sub_8x8:
        Option<unsafe extern "C" fn(*mut dctcoef, *const pixel, *mut pixel) -> libc::c_int>,
    pub sub_4x4:
        Option<unsafe extern "C" fn(*mut dctcoef, *const pixel, *mut pixel) -> libc::c_int>,
    pub sub_4x4ac: Option<
        unsafe extern "C" fn(*mut dctcoef, *const pixel, *mut pixel, *mut dctcoef) -> libc::c_int,
    >,
    pub interleave_8x8_cavlc:
        Option<unsafe extern "C" fn(*mut dctcoef, *mut dctcoef, *mut uint8_t) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_dct_function_t {
    pub sub4x4_dct: Option<unsafe extern "C" fn(*mut dctcoef, *mut pixel, *mut pixel) -> ()>,
    pub add4x4_idct: Option<unsafe extern "C" fn(*mut pixel, *mut dctcoef) -> ()>,
    pub sub8x8_dct: Option<unsafe extern "C" fn(*mut [dctcoef; 16], *mut pixel, *mut pixel) -> ()>,
    pub sub8x8_dct_dc: Option<unsafe extern "C" fn(*mut dctcoef, *mut pixel, *mut pixel) -> ()>,
    pub add8x8_idct: Option<unsafe extern "C" fn(*mut pixel, *mut [dctcoef; 16]) -> ()>,
    pub add8x8_idct_dc: Option<unsafe extern "C" fn(*mut pixel, *mut dctcoef) -> ()>,
    pub sub8x16_dct_dc: Option<unsafe extern "C" fn(*mut dctcoef, *mut pixel, *mut pixel) -> ()>,
    pub sub16x16_dct:
        Option<unsafe extern "C" fn(*mut [dctcoef; 16], *mut pixel, *mut pixel) -> ()>,
    pub add16x16_idct: Option<unsafe extern "C" fn(*mut pixel, *mut [dctcoef; 16]) -> ()>,
    pub add16x16_idct_dc: Option<unsafe extern "C" fn(*mut pixel, *mut dctcoef) -> ()>,
    pub sub8x8_dct8: Option<unsafe extern "C" fn(*mut dctcoef, *mut pixel, *mut pixel) -> ()>,
    pub add8x8_idct8: Option<unsafe extern "C" fn(*mut pixel, *mut dctcoef) -> ()>,
    pub sub16x16_dct8:
        Option<unsafe extern "C" fn(*mut [dctcoef; 64], *mut pixel, *mut pixel) -> ()>,
    pub add16x16_idct8: Option<unsafe extern "C" fn(*mut pixel, *mut [dctcoef; 64]) -> ()>,
    pub dct4x4dc: Option<unsafe extern "C" fn(*mut dctcoef) -> ()>,
    pub idct4x4dc: Option<unsafe extern "C" fn(*mut dctcoef) -> ()>,
    pub dct2x4dc: Option<unsafe extern "C" fn(*mut dctcoef, *mut [dctcoef; 16]) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_mc_functions_t {
    pub mc_luma: Option<
        unsafe extern "C" fn(
            *mut pixel,
            intptr_t,
            *mut *mut pixel,
            intptr_t,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            *const x264_weight_t,
        ) -> (),
    >,
    pub get_ref: Option<
        unsafe extern "C" fn(
            *mut pixel,
            *mut intptr_t,
            *mut *mut pixel,
            intptr_t,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            *const x264_weight_t,
        ) -> *mut pixel,
    >,
    pub mc_chroma: Option<
        unsafe extern "C" fn(
            *mut pixel,
            *mut pixel,
            intptr_t,
            *mut pixel,
            intptr_t,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
        ) -> (),
    >,
    pub avg: [Option<
        unsafe extern "C" fn(
            *mut pixel,
            intptr_t,
            *mut pixel,
            intptr_t,
            *mut pixel,
            intptr_t,
            libc::c_int,
        ) -> (),
    >; 12],
    pub copy: [Option<
        unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t, libc::c_int) -> (),
    >; 7],
    pub copy_16x16_unaligned:
        Option<unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t, libc::c_int) -> ()>,
    pub store_interleave_chroma: Option<
        unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, *mut pixel, libc::c_int) -> (),
    >,
    pub load_deinterleave_chroma_fenc:
        Option<unsafe extern "C" fn(*mut pixel, *mut pixel, intptr_t, libc::c_int) -> ()>,
    pub load_deinterleave_chroma_fdec:
        Option<unsafe extern "C" fn(*mut pixel, *mut pixel, intptr_t, libc::c_int) -> ()>,
    pub plane_copy: Option<
        unsafe extern "C" fn(
            *mut pixel,
            intptr_t,
            *mut pixel,
            intptr_t,
            libc::c_int,
            libc::c_int,
        ) -> (),
    >,
    pub plane_copy_swap: Option<
        unsafe extern "C" fn(
            *mut pixel,
            intptr_t,
            *mut pixel,
            intptr_t,
            libc::c_int,
            libc::c_int,
        ) -> (),
    >,
    pub plane_copy_interleave: Option<
        unsafe extern "C" fn(
            *mut pixel,
            intptr_t,
            *mut pixel,
            intptr_t,
            *mut pixel,
            intptr_t,
            libc::c_int,
            libc::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave: Option<
        unsafe extern "C" fn(
            *mut pixel,
            intptr_t,
            *mut pixel,
            intptr_t,
            *mut pixel,
            intptr_t,
            libc::c_int,
            libc::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_yuyv: Option<
        unsafe extern "C" fn(
            *mut pixel,
            intptr_t,
            *mut pixel,
            intptr_t,
            *mut pixel,
            intptr_t,
            libc::c_int,
            libc::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_rgb: Option<
        unsafe extern "C" fn(
            *mut pixel,
            intptr_t,
            *mut pixel,
            intptr_t,
            *mut pixel,
            intptr_t,
            *mut pixel,
            intptr_t,
            libc::c_int,
            libc::c_int,
            libc::c_int,
        ) -> (),
    >,
    pub plane_copy_deinterleave_v210: Option<
        unsafe extern "C" fn(
            *mut pixel,
            intptr_t,
            *mut pixel,
            intptr_t,
            *mut uint32_t,
            intptr_t,
            libc::c_int,
            libc::c_int,
        ) -> (),
    >,
    pub hpel_filter: Option<
        unsafe extern "C" fn(
            *mut pixel,
            *mut pixel,
            *mut pixel,
            *mut pixel,
            intptr_t,
            libc::c_int,
            libc::c_int,
            *mut int16_t,
        ) -> (),
    >,
    pub prefetch_fenc:
        Option<unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t, libc::c_int) -> ()>,
    pub prefetch_fenc_400:
        Option<unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t, libc::c_int) -> ()>,
    pub prefetch_fenc_420:
        Option<unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t, libc::c_int) -> ()>,
    pub prefetch_fenc_422:
        Option<unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t, libc::c_int) -> ()>,
    pub prefetch_ref: Option<unsafe extern "C" fn(*mut pixel, intptr_t, libc::c_int) -> ()>,
    pub memcpy_aligned: Option<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_void, size_t) -> *mut libc::c_void,
    >,
    pub memzero_aligned: Option<unsafe extern "C" fn(*mut libc::c_void, size_t) -> ()>,
    pub integral_init4h: Option<unsafe extern "C" fn(*mut uint16_t, *mut pixel, intptr_t) -> ()>,
    pub integral_init8h: Option<unsafe extern "C" fn(*mut uint16_t, *mut pixel, intptr_t) -> ()>,
    pub integral_init4v: Option<unsafe extern "C" fn(*mut uint16_t, *mut uint16_t, intptr_t) -> ()>,
    pub integral_init8v: Option<unsafe extern "C" fn(*mut uint16_t, intptr_t) -> ()>,
    pub frame_init_lowres_core: Option<
        unsafe extern "C" fn(
            *mut pixel,
            *mut pixel,
            *mut pixel,
            *mut pixel,
            *mut pixel,
            intptr_t,
            intptr_t,
            libc::c_int,
            libc::c_int,
        ) -> (),
    >,
    pub weight: *mut weight_fn_t,
    pub offsetadd: *mut weight_fn_t,
    pub offsetsub: *mut weight_fn_t,
    pub weight_cache: Option<unsafe extern "C" fn(*mut x264_t, *mut x264_weight_t) -> ()>,
    pub mbtree_propagate_cost: Option<
        unsafe extern "C" fn(
            *mut int16_t,
            *mut uint16_t,
            *mut uint16_t,
            *mut uint16_t,
            *mut uint16_t,
            *mut libc::c_float,
            libc::c_int,
        ) -> (),
    >,
    pub mbtree_propagate_list: Option<
        unsafe extern "C" fn(
            *mut x264_t,
            *mut uint16_t,
            *mut [int16_t; 2],
            *mut int16_t,
            *mut uint16_t,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
        ) -> (),
    >,
    pub mbtree_fix8_pack:
        Option<unsafe extern "C" fn(*mut uint16_t, *mut libc::c_float, libc::c_int) -> ()>,
    pub mbtree_fix8_unpack:
        Option<unsafe extern "C" fn(*mut libc::c_float, *mut uint16_t, libc::c_int) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_pixel_function_t {
    pub sad: [x264_pixel_cmp_t; 8],
    pub ssd: [x264_pixel_cmp_t; 8],
    pub satd: [x264_pixel_cmp_t; 8],
    pub ssim: [x264_pixel_cmp_t; 7],
    pub sa8d: [x264_pixel_cmp_t; 4],
    pub mbcmp: [x264_pixel_cmp_t; 8],
    pub mbcmp_unaligned: [x264_pixel_cmp_t; 8],
    pub fpelcmp: [x264_pixel_cmp_t; 8],
    pub fpelcmp_x3: [x264_pixel_cmp_x3_t; 7],
    pub fpelcmp_x4: [x264_pixel_cmp_x4_t; 7],
    pub sad_aligned: [x264_pixel_cmp_t; 8],
    pub vsad: Option<unsafe extern "C" fn(*mut pixel, intptr_t, libc::c_int) -> libc::c_int>,
    pub asd8: Option<
        unsafe extern "C" fn(
            *mut pixel,
            intptr_t,
            *mut pixel,
            intptr_t,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub sa8d_satd:
        [Option<unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> uint64_t>; 1],
    pub var: [Option<unsafe extern "C" fn(*mut pixel, intptr_t) -> uint64_t>; 4],
    pub var2:
        [Option<unsafe extern "C" fn(*mut pixel, *mut pixel, *mut libc::c_int) -> libc::c_int>; 4],
    pub hadamard_ac: [Option<unsafe extern "C" fn(*mut pixel, intptr_t) -> uint64_t>; 4],
    pub ssd_nv12_core: Option<
        unsafe extern "C" fn(
            *mut pixel,
            intptr_t,
            *mut pixel,
            intptr_t,
            libc::c_int,
            libc::c_int,
            *mut uint64_t,
            *mut uint64_t,
        ) -> (),
    >,
    pub ssim_4x4x2_core: Option<
        unsafe extern "C" fn(
            *const pixel,
            intptr_t,
            *const pixel,
            intptr_t,
            *mut [libc::c_int; 4],
        ) -> (),
    >,
    pub ssim_end4: Option<
        unsafe extern "C" fn(
            *mut [libc::c_int; 4],
            *mut [libc::c_int; 4],
            libc::c_int,
        ) -> libc::c_float,
    >,
    pub sad_x3: [x264_pixel_cmp_x3_t; 7],
    pub sad_x4: [x264_pixel_cmp_x4_t; 7],
    pub satd_x3: [x264_pixel_cmp_x3_t; 7],
    pub satd_x4: [x264_pixel_cmp_x4_t; 7],
    pub ads: [Option<
        unsafe extern "C" fn(
            *mut libc::c_int,
            *mut uint16_t,
            libc::c_int,
            *mut uint16_t,
            *mut int16_t,
            libc::c_int,
            libc::c_int,
        ) -> libc::c_int,
    >; 7],
    pub intra_mbcmp_x3_16x16:
        Option<unsafe extern "C" fn(*mut pixel, *mut pixel, *mut libc::c_int) -> ()>,
    pub intra_satd_x3_16x16:
        Option<unsafe extern "C" fn(*mut pixel, *mut pixel, *mut libc::c_int) -> ()>,
    pub intra_sad_x3_16x16:
        Option<unsafe extern "C" fn(*mut pixel, *mut pixel, *mut libc::c_int) -> ()>,
    pub intra_mbcmp_x3_4x4:
        Option<unsafe extern "C" fn(*mut pixel, *mut pixel, *mut libc::c_int) -> ()>,
    pub intra_satd_x3_4x4:
        Option<unsafe extern "C" fn(*mut pixel, *mut pixel, *mut libc::c_int) -> ()>,
    pub intra_sad_x3_4x4:
        Option<unsafe extern "C" fn(*mut pixel, *mut pixel, *mut libc::c_int) -> ()>,
    pub intra_mbcmp_x3_chroma:
        Option<unsafe extern "C" fn(*mut pixel, *mut pixel, *mut libc::c_int) -> ()>,
    pub intra_satd_x3_chroma:
        Option<unsafe extern "C" fn(*mut pixel, *mut pixel, *mut libc::c_int) -> ()>,
    pub intra_sad_x3_chroma:
        Option<unsafe extern "C" fn(*mut pixel, *mut pixel, *mut libc::c_int) -> ()>,
    pub intra_mbcmp_x3_8x16c:
        Option<unsafe extern "C" fn(*mut pixel, *mut pixel, *mut libc::c_int) -> ()>,
    pub intra_satd_x3_8x16c:
        Option<unsafe extern "C" fn(*mut pixel, *mut pixel, *mut libc::c_int) -> ()>,
    pub intra_sad_x3_8x16c:
        Option<unsafe extern "C" fn(*mut pixel, *mut pixel, *mut libc::c_int) -> ()>,
    pub intra_mbcmp_x3_8x8c:
        Option<unsafe extern "C" fn(*mut pixel, *mut pixel, *mut libc::c_int) -> ()>,
    pub intra_satd_x3_8x8c:
        Option<unsafe extern "C" fn(*mut pixel, *mut pixel, *mut libc::c_int) -> ()>,
    pub intra_sad_x3_8x8c:
        Option<unsafe extern "C" fn(*mut pixel, *mut pixel, *mut libc::c_int) -> ()>,
    pub intra_mbcmp_x3_8x8:
        Option<unsafe extern "C" fn(*mut pixel, *mut pixel, *mut libc::c_int) -> ()>,
    pub intra_sa8d_x3_8x8:
        Option<unsafe extern "C" fn(*mut pixel, *mut pixel, *mut libc::c_int) -> ()>,
    pub intra_sad_x3_8x8:
        Option<unsafe extern "C" fn(*mut pixel, *mut pixel, *mut libc::c_int) -> ()>,
    pub intra_mbcmp_x9_4x4:
        Option<unsafe extern "C" fn(*mut pixel, *mut pixel, *mut uint16_t) -> libc::c_int>,
    pub intra_satd_x9_4x4:
        Option<unsafe extern "C" fn(*mut pixel, *mut pixel, *mut uint16_t) -> libc::c_int>,
    pub intra_sad_x9_4x4:
        Option<unsafe extern "C" fn(*mut pixel, *mut pixel, *mut uint16_t) -> libc::c_int>,
    pub intra_mbcmp_x9_8x8: Option<
        unsafe extern "C" fn(
            *mut pixel,
            *mut pixel,
            *mut pixel,
            *mut uint16_t,
            *mut uint16_t,
        ) -> libc::c_int,
    >,
    pub intra_sa8d_x9_8x8: Option<
        unsafe extern "C" fn(
            *mut pixel,
            *mut pixel,
            *mut pixel,
            *mut uint16_t,
            *mut uint16_t,
        ) -> libc::c_int,
    >,
    pub intra_sad_x9_8x8: Option<
        unsafe extern "C" fn(
            *mut pixel,
            *mut pixel,
            *mut pixel,
            *mut uint16_t,
            *mut uint16_t,
        ) -> libc::c_int,
    >,
}
pub type x264_pixel_cmp_x4_t = Option<
    unsafe extern "C" fn(
        *mut pixel,
        *mut pixel,
        *mut pixel,
        *mut pixel,
        *mut pixel,
        intptr_t,
        *mut libc::c_int,
    ) -> (),
>;
pub type x264_pixel_cmp_x3_t = Option<
    unsafe extern "C" fn(
        *mut pixel,
        *mut pixel,
        *mut pixel,
        *mut pixel,
        intptr_t,
        *mut libc::c_int,
    ) -> (),
>;
pub type x264_pixel_cmp_t =
    Option<unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> libc::c_int>;
pub type x264_predict_8x8_filter_t =
    Option<unsafe extern "C" fn(*mut pixel, *mut pixel, libc::c_int, libc::c_int) -> ()>;
pub type x264_predict_t = Option<unsafe extern "C" fn(*mut pixel) -> ()>;
pub type x264_predict8x8_t = Option<unsafe extern "C" fn(*mut pixel, *mut pixel) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub i_frame_count: [libc::c_int; 3],
    pub i_frame_size: [int64_t; 3],
    pub f_frame_qp: [libc::c_double; 3],
    pub i_consecutive_bframes: [libc::c_int; 17],
    pub f_ssd_global: [libc::c_double; 3],
    pub f_psnr_average: [libc::c_double; 3],
    pub f_psnr_mean_y: [libc::c_double; 3],
    pub f_psnr_mean_u: [libc::c_double; 3],
    pub f_psnr_mean_v: [libc::c_double; 3],
    pub f_ssim_mean_y: [libc::c_double; 3],
    pub f_frame_duration: [libc::c_double; 3],
    pub i_mb_count: [[int64_t; 19]; 3],
    pub i_mb_partition: [[int64_t; 17]; 2],
    pub i_mb_count_8x8dct: [int64_t; 2],
    pub i_mb_count_ref: [[[int64_t; 32]; 2]; 2],
    pub i_mb_cbp: [int64_t; 6],
    pub i_mb_pred_mode: [[int64_t; 13]; 4],
    pub i_mb_field: [int64_t; 3],
    pub i_direct_score: [libc::c_int; 2],
    pub i_direct_frames: [libc::c_int; 2],
    pub i_wpred: [libc::c_int; 2],
    pub frame: x264_frame_stat_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_frame_stat_t {
    pub i_mv_bits: libc::c_int,
    pub i_tex_bits: libc::c_int,
    pub i_misc_bits: libc::c_int,
    pub i_mb_count: [libc::c_int; 19],
    pub i_mb_count_i: libc::c_int,
    pub i_mb_count_p: libc::c_int,
    pub i_mb_count_skip: libc::c_int,
    pub i_mb_count_8x8dct: [libc::c_int; 2],
    pub i_mb_count_ref: [[libc::c_int; 32]; 2],
    pub i_mb_partition: [libc::c_int; 17],
    pub i_mb_cbp: [libc::c_int; 6],
    pub i_mb_pred_mode: [[libc::c_int; 13]; 4],
    pub i_mb_field: [libc::c_int; 3],
    pub i_direct_score: [libc::c_int; 2],
    pub i_ssd: [int64_t; 3],
    pub f_ssim: libc::c_double,
    pub i_ssim_cnt: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub i_mb_width: libc::c_int,
    pub i_mb_height: libc::c_int,
    pub i_mb_count: libc::c_int,
    pub chroma_h_shift: libc::c_int,
    pub chroma_v_shift: libc::c_int,
    pub i_mb_stride: libc::c_int,
    pub i_b8_stride: libc::c_int,
    pub i_b4_stride: libc::c_int,
    pub left_b8: [libc::c_int; 2],
    pub left_b4: [libc::c_int; 2],
    pub i_mb_x: libc::c_int,
    pub i_mb_y: libc::c_int,
    pub i_mb_xy: libc::c_int,
    pub i_b8_xy: libc::c_int,
    pub i_b4_xy: libc::c_int,
    pub i_me_method: libc::c_int,
    pub i_subpel_refine: libc::c_int,
    pub b_chroma_me: libc::c_int,
    pub b_trellis: libc::c_int,
    pub b_noise_reduction: libc::c_int,
    pub b_dct_decimate: libc::c_int,
    pub i_psy_rd: libc::c_int,
    pub i_psy_trellis: libc::c_int,
    pub b_interlaced: libc::c_int,
    pub b_adaptive_mbaff: libc::c_int,
    pub mv_min: [libc::c_int; 2],
    pub mv_max: [libc::c_int; 2],
    pub mv_miny_row: [libc::c_int; 3],
    pub mv_maxy_row: [libc::c_int; 3],
    pub mv_min_spel: [libc::c_int; 2],
    pub mv_max_spel: [libc::c_int; 2],
    pub mv_miny_spel_row: [libc::c_int; 3],
    pub mv_maxy_spel_row: [libc::c_int; 3],
    pub mv_limit_fpel: [[int16_t; 2]; 2],
    pub mv_miny_fpel_row: [libc::c_int; 3],
    pub mv_maxy_fpel_row: [libc::c_int; 3],
    pub i_neighbour: libc::c_uint,
    pub i_neighbour8: [libc::c_uint; 4],
    pub i_neighbour4: [libc::c_uint; 16],
    pub i_neighbour_intra: libc::c_uint,
    pub i_neighbour_frame: libc::c_uint,
    pub i_mb_type_top: libc::c_int,
    pub i_mb_type_left: [libc::c_int; 2],
    pub i_mb_type_topleft: libc::c_int,
    pub i_mb_type_topright: libc::c_int,
    pub i_mb_prev_xy: libc::c_int,
    pub i_mb_left_xy: [libc::c_int; 2],
    pub i_mb_top_xy: libc::c_int,
    pub i_mb_topleft_xy: libc::c_int,
    pub i_mb_topright_xy: libc::c_int,
    pub i_mb_top_y: libc::c_int,
    pub i_mb_topleft_y: libc::c_int,
    pub i_mb_topright_y: libc::c_int,
    pub left_index_table: *const x264_left_table_t,
    pub i_mb_top_mbpair_xy: libc::c_int,
    pub topleft_partition: libc::c_int,
    pub b_allow_skip: libc::c_int,
    pub field_decoding_flag: libc::c_int,
    pub base: *mut uint8_t,
    pub type_0: *mut int8_t,
    pub partition: *mut uint8_t,
    pub qp: *mut int8_t,
    pub cbp: *mut int16_t,
    pub intra4x4_pred_mode: *mut [int8_t; 8],
    pub non_zero_count: *mut [uint8_t; 48],
    pub chroma_pred_mode: *mut int8_t,
    pub mv: [*mut [int16_t; 2]; 2],
    pub mvd: [*mut [[uint8_t; 2]; 8]; 2],
    pub ref_0: [*mut int8_t; 2],
    pub mvr: [[*mut [int16_t; 2]; 32]; 2],
    pub skipbp: *mut int8_t,
    pub mb_transform_size: *mut int8_t,
    pub slice_table: *mut int32_t,
    pub field: *mut uint8_t,
    pub p_weight_buf: [*mut pixel; 16],
    pub i_type: libc::c_int,
    pub i_partition: libc::c_int,
    pub i_sub_partition: [uint8_t; 4],
    pub b_transform_8x8: libc::c_int,
    pub i_cbp_luma: libc::c_int,
    pub i_cbp_chroma: libc::c_int,
    pub i_intra16x16_pred_mode: libc::c_int,
    pub i_chroma_pred_mode: libc::c_int,
    pub i_skip_intra: libc::c_int,
    pub b_skip_mc: libc::c_int,
    pub b_reencode_mb: libc::c_int,
    pub ip_offset: libc::c_int,
    pub b_deblock_rdo: libc::c_int,
    pub b_overflow: libc::c_int,
    pub pic: C2RustUnnamed_10,
    pub cache: C2RustUnnamed_9,
    pub i_qp: libc::c_int,
    pub i_chroma_qp: libc::c_int,
    pub i_last_qp: libc::c_int,
    pub i_last_dqp: libc::c_int,
    pub b_variable_qp: libc::c_int,
    pub b_lossless: libc::c_int,
    pub b_direct_auto_read: libc::c_int,
    pub b_direct_auto_write: libc::c_int,
    pub i_trellis_lambda2: [[libc::c_int; 2]; 2],
    pub i_psy_rd_lambda: libc::c_int,
    pub i_chroma_lambda2_offset: libc::c_int,
    pub dist_scale_factor_buf: [[[[int16_t; 4]; 32]; 2]; 2],
    pub dist_scale_factor: *mut [int16_t; 4],
    pub bipred_weight_buf: [[[[int8_t; 4]; 32]; 2]; 2],
    pub bipred_weight: *mut [int8_t; 4],
    pub map_col_to_list0: [int8_t; 18],
    pub ref_blind_dupe: libc::c_int,
    pub deblock_ref_table: [int8_t; 34],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub intra4x4_pred_mode: [int8_t; 40],
    pub non_zero_count: [uint8_t; 120],
    pub ref_0: [[int8_t; 40]; 2],
    pub mv: [[[int16_t; 2]; 40]; 2],
    pub mvd: [[[uint8_t; 2]; 40]; 2],
    pub skip: [int8_t; 40],
    pub direct_mv: [[[int16_t; 2]; 4]; 2],
    pub direct_ref: [[int8_t; 4]; 2],
    pub direct_partition: libc::c_int,
    pub pskip_mv: [int16_t; 2],
    pub i_neighbour_transform_size: libc::c_int,
    pub i_neighbour_skip: libc::c_int,
    pub i_cbp_top: libc::c_int,
    pub i_cbp_left: libc::c_int,
    pub topright_mv: [[[int16_t; 2]; 3]; 2],
    pub topright_ref: [[int8_t; 3]; 2],
    pub deblock_strength: *mut [[uint8_t; 4]; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub fenc_buf: [pixel; 768],
    pub fdec_buf: [pixel; 1728],
    pub i4x4_fdec_buf: [pixel; 256],
    pub i8x8_fdec_buf: [pixel; 256],
    pub i8x8_dct_buf: [[dctcoef; 64]; 3],
    pub i4x4_dct_buf: [[dctcoef; 16]; 15],
    pub i4x4_nnz_buf: [uint32_t; 4],
    pub i8x8_nnz_buf: [uint32_t; 4],
    pub fenc_dct8: [[dctcoef; 64]; 4],
    pub fenc_dct4: [[dctcoef; 16]; 16],
    pub fenc_satd_cache: [uint32_t; 32],
    pub fenc_hadamard_cache: [uint64_t; 9],
    pub i4x4_cbp: libc::c_int,
    pub i8x8_cbp: libc::c_int,
    pub p_fenc: [*mut pixel; 3],
    pub p_fenc_plane: [*mut pixel; 3],
    pub p_fdec: [*mut pixel; 3],
    pub i_fref: [libc::c_int; 2],
    pub p_fref: [[[*mut pixel; 12]; 32]; 2],
    pub p_fref_w: [*mut pixel; 32],
    pub p_integral: [[*mut uint16_t; 16]; 2],
    pub i_stride: [libc::c_int; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_left_table_t {
    pub intra: [uint8_t; 4],
    pub nnz: [uint8_t; 4],
    pub nnz_chroma: [uint8_t; 4],
    pub mv: [uint8_t; 4],
    pub ref_0: [uint8_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub luma16x16_dc: [[dctcoef; 16]; 3],
    pub chroma_dc: [[dctcoef; 8]; 2],
    pub luma8x8: [[dctcoef; 64]; 12],
    pub luma4x4: [[dctcoef; 16]; 48],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub current: *mut *mut x264_frame_t,
    pub unused: [*mut *mut x264_frame_t; 2],
    pub blank_unused: *mut *mut x264_frame_t,
    pub reference: [*mut x264_frame_t; 18],
    pub i_last_keyframe: libc::c_int,
    pub i_last_idr: libc::c_int,
    pub i_poc_last_open_gop: libc::c_int,
    pub i_input: libc::c_int,
    pub i_max_dpb: libc::c_int,
    pub i_max_ref0: libc::c_int,
    pub i_max_ref1: libc::c_int,
    pub i_delay: libc::c_int,
    pub i_bframe_delay: libc::c_int,
    pub i_bframe_delay_time: int64_t,
    pub i_first_pts: int64_t,
    pub i_prev_reordered_pts: [int64_t; 2],
    pub i_largest_pts: int64_t,
    pub i_second_largest_pts: int64_t,
    pub b_have_lowres: libc::c_int,
    pub b_have_sub8x8_esa: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_slice_header_t {
    pub sps: *mut x264_sps_t,
    pub pps: *mut x264_pps_t,
    pub i_type: libc::c_int,
    pub i_first_mb: libc::c_int,
    pub i_last_mb: libc::c_int,
    pub i_pps_id: libc::c_int,
    pub i_frame_num: libc::c_int,
    pub b_mbaff: libc::c_int,
    pub b_field_pic: libc::c_int,
    pub b_bottom_field: libc::c_int,
    pub i_idr_pic_id: libc::c_int,
    pub i_poc: libc::c_int,
    pub i_delta_poc_bottom: libc::c_int,
    pub i_delta_poc: [libc::c_int; 2],
    pub i_redundant_pic_cnt: libc::c_int,
    pub b_direct_spatial_mv_pred: libc::c_int,
    pub b_num_ref_idx_override: libc::c_int,
    pub i_num_ref_idx_l0_active: libc::c_int,
    pub i_num_ref_idx_l1_active: libc::c_int,
    pub b_ref_pic_list_reordering: [libc::c_int; 2],
    pub ref_pic_list_order: [[C2RustUnnamed_14; 16]; 2],
    pub b_weighted_pred: libc::c_int,
    pub weight: [[x264_weight_t; 3]; 32],
    pub i_mmco_remove_from_end: libc::c_int,
    pub i_mmco_command_count: libc::c_int,
    pub mmco: [C2RustUnnamed_13; 16],
    pub i_cabac_init_idc: libc::c_int,
    pub i_qp: libc::c_int,
    pub i_qp_delta: libc::c_int,
    pub b_sp_for_swidth: libc::c_int,
    pub i_qs_delta: libc::c_int,
    pub i_disable_deblocking_filter_idc: libc::c_int,
    pub i_alpha_c0_offset: libc::c_int,
    pub i_beta_offset: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub i_difference_of_pic_nums: libc::c_int,
    pub i_poc: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub idc: libc::c_int,
    pub arg: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_pps_t {
    pub i_id: libc::c_int,
    pub i_sps_id: libc::c_int,
    pub b_cabac: libc::c_int,
    pub b_pic_order: libc::c_int,
    pub i_num_slice_groups: libc::c_int,
    pub i_num_ref_idx_l0_default_active: libc::c_int,
    pub i_num_ref_idx_l1_default_active: libc::c_int,
    pub b_weighted_pred: libc::c_int,
    pub b_weighted_bipred: libc::c_int,
    pub i_pic_init_qp: libc::c_int,
    pub i_pic_init_qs: libc::c_int,
    pub i_chroma_qp_index_offset: libc::c_int,
    pub b_deblocking_filter_control: libc::c_int,
    pub b_constrained_intra_pred: libc::c_int,
    pub b_redundant_pic_cnt: libc::c_int,
    pub b_transform_8x8_mode: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_sps_t {
    pub i_id: libc::c_int,
    pub i_profile_idc: libc::c_int,
    pub i_level_idc: libc::c_int,
    pub b_constraint_set0: libc::c_int,
    pub b_constraint_set1: libc::c_int,
    pub b_constraint_set2: libc::c_int,
    pub b_constraint_set3: libc::c_int,
    pub i_log2_max_frame_num: libc::c_int,
    pub i_poc_type: libc::c_int,
    pub i_log2_max_poc_lsb: libc::c_int,
    pub i_num_ref_frames: libc::c_int,
    pub b_gaps_in_frame_num_value_allowed: libc::c_int,
    pub i_mb_width: libc::c_int,
    pub i_mb_height: libc::c_int,
    pub b_frame_mbs_only: libc::c_int,
    pub b_mb_adaptive_frame_field: libc::c_int,
    pub b_direct8x8_inference: libc::c_int,
    pub b_crop: libc::c_int,
    pub crop: C2RustUnnamed_17,
    pub b_vui: libc::c_int,
    pub vui: C2RustUnnamed_15,
    pub b_qpprime_y_zero_transform_bypass: libc::c_int,
    pub i_chroma_format_idc: libc::c_int,
    pub b_avcintra_hd: libc::c_int,
    pub b_avcintra_4k: libc::c_int,
    pub i_cqm_preset: libc::c_int,
    pub scaling_list: [*const uint8_t; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub b_aspect_ratio_info_present: libc::c_int,
    pub i_sar_width: libc::c_int,
    pub i_sar_height: libc::c_int,
    pub b_overscan_info_present: libc::c_int,
    pub b_overscan_info: libc::c_int,
    pub b_signal_type_present: libc::c_int,
    pub i_vidformat: libc::c_int,
    pub b_fullrange: libc::c_int,
    pub b_color_description_present: libc::c_int,
    pub i_colorprim: libc::c_int,
    pub i_transfer: libc::c_int,
    pub i_colmatrix: libc::c_int,
    pub b_chroma_loc_info_present: libc::c_int,
    pub i_chroma_loc_top: libc::c_int,
    pub i_chroma_loc_bottom: libc::c_int,
    pub b_timing_info_present: libc::c_int,
    pub i_num_units_in_tick: uint32_t,
    pub i_time_scale: uint32_t,
    pub b_fixed_frame_rate: libc::c_int,
    pub b_nal_hrd_parameters_present: libc::c_int,
    pub b_vcl_hrd_parameters_present: libc::c_int,
    pub hrd: C2RustUnnamed_16,
    pub b_pic_struct_present: libc::c_int,
    pub b_bitstream_restriction: libc::c_int,
    pub b_motion_vectors_over_pic_boundaries: libc::c_int,
    pub i_max_bytes_per_pic_denom: libc::c_int,
    pub i_max_bits_per_mb_denom: libc::c_int,
    pub i_log2_max_mv_length_horizontal: libc::c_int,
    pub i_log2_max_mv_length_vertical: libc::c_int,
    pub i_num_reorder_frames: libc::c_int,
    pub i_max_dec_frame_buffering: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    pub i_cpb_cnt: libc::c_int,
    pub i_bit_rate_scale: libc::c_int,
    pub i_cpb_size_scale: libc::c_int,
    pub i_bit_rate_value: libc::c_int,
    pub i_cpb_size_value: libc::c_int,
    pub i_bit_rate_unscaled: libc::c_int,
    pub i_cpb_size_unscaled: libc::c_int,
    pub b_cbr_hrd: libc::c_int,
    pub i_initial_cpb_removal_delay_length: libc::c_int,
    pub i_cpb_removal_delay_length: libc::c_int,
    pub i_dpb_output_delay_length: libc::c_int,
    pub i_time_offset_length: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
    pub i_left: libc::c_int,
    pub i_right: libc::c_int,
    pub i_top: libc::c_int,
    pub i_bottom: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_18 {
    pub ref_0: [[[uint16_t; 33]; 3]; 70],
    pub i4x4_mode: [[uint16_t; 17]; 70],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
    pub i_nal: libc::c_int,
    pub i_nals_allocated: libc::c_int,
    pub nal: *mut x264_nal_t,
    pub i_bitstream: libc::c_int,
    pub p_bitstream: *mut uint8_t,
    pub bs: bs_t,
}
pub type bs_t = bs_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bs_s {
    pub p_start: *mut uint8_t,
    pub p: *mut uint8_t,
    pub p_end: *mut uint8_t,
    pub cur_bits: uintptr_t,
    pub i_left: libc::c_int,
    pub i_bits_encoded: libc::c_int,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union x264_union32_t {
    pub i: uint32_t,
    pub w: [uint16_t; 2],
    pub b: [uint8_t; 4],
}
pub type profile_e = libc::c_uint;
pub const PROFILE_HIGH444_PREDICTIVE: profile_e = 244;
pub const PROFILE_HIGH422: profile_e = 122;
pub const PROFILE_HIGH10: profile_e = 110;
pub const PROFILE_HIGH: profile_e = 100;
pub const PROFILE_MAIN: profile_e = 77;
pub const PROFILE_BASELINE: profile_e = 66;
pub type chroma_format_e = libc::c_uint;
pub const CHROMA_444: chroma_format_e = 3;
pub const CHROMA_422: chroma_format_e = 2;
pub const CHROMA_420: chroma_format_e = 1;
pub const CHROMA_400: chroma_format_e = 0;
pub type sei_payload_type_e = libc::c_uint;
pub const SEI_ALTERNATIVE_TRANSFER: sei_payload_type_e = 147;
pub const SEI_CONTENT_LIGHT_LEVEL: sei_payload_type_e = 144;
pub const SEI_MASTERING_DISPLAY: sei_payload_type_e = 137;
pub const SEI_FRAME_PACKING: sei_payload_type_e = 45;
pub const SEI_DEC_REF_PIC_MARKING: sei_payload_type_e = 7;
pub const SEI_RECOVERY_POINT: sei_payload_type_e = 6;
pub const SEI_USER_DATA_UNREGISTERED: sei_payload_type_e = 5;
pub const SEI_USER_DATA_REGISTERED: sei_payload_type_e = 4;
pub const SEI_FILLER: sei_payload_type_e = 3;
pub const SEI_PAN_SCAN_RECT: sei_payload_type_e = 2;
pub const SEI_PIC_TIMING: sei_payload_type_e = 1;
pub const SEI_BUFFERING_PERIOD: sei_payload_type_e = 0;
pub type cqm4_e = libc::c_uint;
pub const CQM_4PC: cqm4_e = 3;
pub const CQM_4IC: cqm4_e = 2;
pub const CQM_4PY: cqm4_e = 1;
pub const CQM_4IY: cqm4_e = 0;
pub type cqm8_e = libc::c_uint;
pub const CQM_8PC: cqm8_e = 3;
pub const CQM_8IC: cqm8_e = 2;
pub const CQM_8PY: cqm8_e = 1;
pub const CQM_8IY: cqm8_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_20 {
    pub w: uint8_t,
    pub h: uint8_t,
    pub sar: uint8_t,
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
#[inline(always)]
unsafe extern "C" fn endian_fix(mut x: uintptr_t) -> uintptr_t {
    if ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
        == 8 as libc::c_int as libc::c_ulong
    {
        endian_fix64(x)
    } else {
        endian_fix32(x as uint32_t) as uint64_t
    }
}
#[inline]
unsafe extern "C" fn bs_init(
    mut s: *mut bs_t,
    mut p_data: *mut libc::c_void,
    mut i_data: libc::c_int,
) {
    let mut offset: libc::c_int =
        (p_data as intptr_t & 3 as libc::c_int as intptr_t) as libc::c_int;
    (*s).p_start = (p_data as *mut uint8_t).offset(-(offset as isize));
    (*s).p = (*s).p_start;
    (*s).p_end = (p_data as *mut uint8_t).offset(i_data as isize);
    (*s).i_left = (::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
        .wrapping_sub(offset as libc::c_ulong)
        .wrapping_mul(8 as libc::c_int as libc::c_ulong) as libc::c_int;
    if offset != 0 {
        (*s).cur_bits = endian_fix32((*((*s).p as *mut x264_union32_t)).i) as uintptr_t;
        (*s).cur_bits >>= (4 as libc::c_int - offset) * 8 as libc::c_int;
    } else {
        (*s).cur_bits = 0 as libc::c_int as uintptr_t;
    };
}
#[inline]
unsafe extern "C" fn bs_pos(mut s: *mut bs_t) -> libc::c_int {
    ((8 as libc::c_int as libc::c_long * ((*s).p).offset_from((*s).p_start) as libc::c_long)
        as libc::c_ulong)
        .wrapping_add(
            (::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        )
        .wrapping_sub((*s).i_left as libc::c_ulong) as libc::c_int
}
#[inline]
unsafe extern "C" fn bs_flush(mut s: *mut bs_t) {
    (*((*s).p as *mut x264_union32_t)).i =
        endian_fix32(((*s).cur_bits << ((*s).i_left & 31 as libc::c_int)) as uint32_t);
    (*s).p = ((*s).p).offset(
        (::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
            .wrapping_sub(((*s).i_left >> 3 as libc::c_int) as libc::c_ulong) as isize,
    );
    (*s).i_left = (::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
        .wrapping_mul(8 as libc::c_int as libc::c_ulong) as libc::c_int;
}
#[inline]
unsafe extern "C" fn bs_realign(mut s: *mut bs_t) {
    let mut offset: libc::c_int =
        ((*s).p as intptr_t & 3 as libc::c_int as intptr_t) as libc::c_int;
    if offset != 0 {
        (*s).p = ((*s).p).offset(-(offset as isize));
        (*s).i_left = (::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
            .wrapping_sub(offset as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong) as libc::c_int;
        (*s).cur_bits = endian_fix32((*((*s).p as *mut x264_union32_t)).i) as uintptr_t;
        (*s).cur_bits >>= (4 as libc::c_int - offset) * 8 as libc::c_int;
    }
}
#[inline]
unsafe extern "C" fn bs_write(mut s: *mut bs_t, mut i_count: libc::c_int, mut i_bits: uint32_t) {
    if ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
        == 8 as libc::c_int as libc::c_ulong
    {
        (*s).cur_bits = ((*s).cur_bits << i_count) | i_bits as uintptr_t;
        (*s).i_left -= i_count;
        if (*s).i_left <= 32 as libc::c_int {
            (*((*s).p as *mut x264_union32_t)).i =
                endian_fix((*s).cur_bits << (*s).i_left) as uint32_t;
            (*s).i_left += 32 as libc::c_int;
            (*s).p = ((*s).p).offset(4 as libc::c_int as isize);
        }
    } else if i_count < (*s).i_left {
        (*s).cur_bits = ((*s).cur_bits << i_count) | i_bits as uintptr_t;
        (*s).i_left -= i_count;
    } else {
        i_count -= (*s).i_left;
        (*s).cur_bits = ((*s).cur_bits << (*s).i_left) | (i_bits >> i_count) as uintptr_t;
        (*((*s).p as *mut x264_union32_t)).i = endian_fix((*s).cur_bits) as uint32_t;
        (*s).p = ((*s).p).offset(4 as libc::c_int as isize);
        (*s).cur_bits = i_bits as uintptr_t;
        (*s).i_left = 32 as libc::c_int - i_count;
    };
}
#[inline]
unsafe extern "C" fn bs_write32(mut s: *mut bs_t, mut i_bits: uint32_t) {
    bs_write(s, 16 as libc::c_int, i_bits >> 16 as libc::c_int);
    bs_write(s, 16 as libc::c_int, i_bits);
}
#[inline]
unsafe extern "C" fn bs_write1(mut s: *mut bs_t, mut i_bit: uint32_t) {
    (*s).cur_bits <<= 1 as libc::c_int;
    (*s).cur_bits |= i_bit as uintptr_t;
    (*s).i_left -= 1;
    (*s).i_left;
    if (*s).i_left as libc::c_ulong
        == (::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(32 as libc::c_int as libc::c_ulong)
    {
        (*((*s).p as *mut x264_union32_t)).i = endian_fix32((*s).cur_bits as uint32_t);
        (*s).p = ((*s).p).offset(4 as libc::c_int as isize);
        (*s).i_left = (::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong) as libc::c_int;
    }
}
#[inline]
unsafe extern "C" fn bs_align_10(mut s: *mut bs_t) {
    if (*s).i_left & 7 as libc::c_int != 0 {
        bs_write(
            s,
            (*s).i_left & 7 as libc::c_int,
            ((1 as libc::c_int) << (((*s).i_left & 7 as libc::c_int) - 1 as libc::c_int))
                as uint32_t,
        );
    }
    bs_flush(s);
}
static mut x264_ue_size_tab: [uint8_t; 256] = [
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    7 as libc::c_int as uint8_t,
    7 as libc::c_int as uint8_t,
    7 as libc::c_int as uint8_t,
    7 as libc::c_int as uint8_t,
    7 as libc::c_int as uint8_t,
    7 as libc::c_int as uint8_t,
    7 as libc::c_int as uint8_t,
    7 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
];
#[inline]
unsafe extern "C" fn bs_write_ue_big(mut s: *mut bs_t, mut val: libc::c_uint) {
    let mut size: libc::c_int = 0 as libc::c_int;
    val = val.wrapping_add(1);
    let mut tmp: libc::c_int = val as libc::c_int;
    if tmp >= 0x10000 as libc::c_int {
        size = 32 as libc::c_int;
        tmp >>= 16 as libc::c_int;
    }
    if tmp >= 0x100 as libc::c_int {
        size += 16 as libc::c_int;
        tmp >>= 8 as libc::c_int;
    }
    size += x264_ue_size_tab[tmp as usize] as libc::c_int;
    bs_write(s, size >> 1 as libc::c_int, 0 as libc::c_int as uint32_t);
    bs_write(s, (size >> 1 as libc::c_int) + 1 as libc::c_int, val);
}
#[inline]
unsafe extern "C" fn bs_write_se(mut s: *mut bs_t, mut val: libc::c_int) {
    let mut size: libc::c_int = 0 as libc::c_int;
    let mut tmp: libc::c_int = 1 as libc::c_int - val * 2 as libc::c_int;
    if tmp < 0 as libc::c_int {
        tmp = val * 2 as libc::c_int;
    }
    val = tmp;
    if tmp >= 0x100 as libc::c_int {
        size = 16 as libc::c_int;
        tmp >>= 8 as libc::c_int;
    }
    size += x264_ue_size_tab[tmp as usize] as libc::c_int;
    bs_write(s, size, val as uint32_t);
}
#[inline]
unsafe extern "C" fn bs_rbsp_trailing(mut s: *mut bs_t) {
    bs_write1(s, 1 as libc::c_int as uint32_t);
    bs_write(
        s,
        (*s).i_left & 7 as libc::c_int,
        0 as libc::c_int as uint32_t,
    );
}
#[inline(always)]
unsafe extern "C" fn bs_size_se(mut val: libc::c_int) -> libc::c_int {
    let mut tmp: libc::c_int = 1 as libc::c_int - val * 2 as libc::c_int;
    if tmp < 0 as libc::c_int {
        tmp = val * 2 as libc::c_int;
    }
    if tmp < 256 as libc::c_int {
        x264_ue_size_tab[tmp as usize] as libc::c_int
    } else {
        x264_ue_size_tab[(tmp >> 8 as libc::c_int) as usize] as libc::c_int + 16 as libc::c_int
    }
}
static mut x264_zigzag_scan4: [[uint8_t; 16]; 2] = [
    [
        0 as libc::c_int as uint8_t,
        4 as libc::c_int as uint8_t,
        1 as libc::c_int as uint8_t,
        2 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        8 as libc::c_int as uint8_t,
        12 as libc::c_int as uint8_t,
        9 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        3 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        10 as libc::c_int as uint8_t,
        13 as libc::c_int as uint8_t,
        14 as libc::c_int as uint8_t,
        11 as libc::c_int as uint8_t,
        15 as libc::c_int as uint8_t,
    ],
    [
        0 as libc::c_int as uint8_t,
        1 as libc::c_int as uint8_t,
        4 as libc::c_int as uint8_t,
        2 as libc::c_int as uint8_t,
        3 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        8 as libc::c_int as uint8_t,
        9 as libc::c_int as uint8_t,
        10 as libc::c_int as uint8_t,
        11 as libc::c_int as uint8_t,
        12 as libc::c_int as uint8_t,
        13 as libc::c_int as uint8_t,
        14 as libc::c_int as uint8_t,
        15 as libc::c_int as uint8_t,
    ],
];
static mut x264_zigzag_scan8: [[uint8_t; 64]; 2] = [
    [
        0 as libc::c_int as uint8_t,
        8 as libc::c_int as uint8_t,
        1 as libc::c_int as uint8_t,
        2 as libc::c_int as uint8_t,
        9 as libc::c_int as uint8_t,
        16 as libc::c_int as uint8_t,
        24 as libc::c_int as uint8_t,
        17 as libc::c_int as uint8_t,
        10 as libc::c_int as uint8_t,
        3 as libc::c_int as uint8_t,
        4 as libc::c_int as uint8_t,
        11 as libc::c_int as uint8_t,
        18 as libc::c_int as uint8_t,
        25 as libc::c_int as uint8_t,
        32 as libc::c_int as uint8_t,
        40 as libc::c_int as uint8_t,
        33 as libc::c_int as uint8_t,
        26 as libc::c_int as uint8_t,
        19 as libc::c_int as uint8_t,
        12 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        13 as libc::c_int as uint8_t,
        20 as libc::c_int as uint8_t,
        27 as libc::c_int as uint8_t,
        34 as libc::c_int as uint8_t,
        41 as libc::c_int as uint8_t,
        48 as libc::c_int as uint8_t,
        56 as libc::c_int as uint8_t,
        49 as libc::c_int as uint8_t,
        42 as libc::c_int as uint8_t,
        35 as libc::c_int as uint8_t,
        28 as libc::c_int as uint8_t,
        21 as libc::c_int as uint8_t,
        14 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        15 as libc::c_int as uint8_t,
        22 as libc::c_int as uint8_t,
        29 as libc::c_int as uint8_t,
        36 as libc::c_int as uint8_t,
        43 as libc::c_int as uint8_t,
        50 as libc::c_int as uint8_t,
        57 as libc::c_int as uint8_t,
        58 as libc::c_int as uint8_t,
        51 as libc::c_int as uint8_t,
        44 as libc::c_int as uint8_t,
        37 as libc::c_int as uint8_t,
        30 as libc::c_int as uint8_t,
        23 as libc::c_int as uint8_t,
        31 as libc::c_int as uint8_t,
        38 as libc::c_int as uint8_t,
        45 as libc::c_int as uint8_t,
        52 as libc::c_int as uint8_t,
        59 as libc::c_int as uint8_t,
        60 as libc::c_int as uint8_t,
        53 as libc::c_int as uint8_t,
        46 as libc::c_int as uint8_t,
        39 as libc::c_int as uint8_t,
        47 as libc::c_int as uint8_t,
        54 as libc::c_int as uint8_t,
        61 as libc::c_int as uint8_t,
        62 as libc::c_int as uint8_t,
        55 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
    ],
    [
        0 as libc::c_int as uint8_t,
        1 as libc::c_int as uint8_t,
        2 as libc::c_int as uint8_t,
        8 as libc::c_int as uint8_t,
        9 as libc::c_int as uint8_t,
        3 as libc::c_int as uint8_t,
        4 as libc::c_int as uint8_t,
        10 as libc::c_int as uint8_t,
        16 as libc::c_int as uint8_t,
        11 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        12 as libc::c_int as uint8_t,
        17 as libc::c_int as uint8_t,
        24 as libc::c_int as uint8_t,
        18 as libc::c_int as uint8_t,
        13 as libc::c_int as uint8_t,
        14 as libc::c_int as uint8_t,
        15 as libc::c_int as uint8_t,
        19 as libc::c_int as uint8_t,
        25 as libc::c_int as uint8_t,
        32 as libc::c_int as uint8_t,
        26 as libc::c_int as uint8_t,
        20 as libc::c_int as uint8_t,
        21 as libc::c_int as uint8_t,
        22 as libc::c_int as uint8_t,
        23 as libc::c_int as uint8_t,
        27 as libc::c_int as uint8_t,
        33 as libc::c_int as uint8_t,
        40 as libc::c_int as uint8_t,
        34 as libc::c_int as uint8_t,
        28 as libc::c_int as uint8_t,
        29 as libc::c_int as uint8_t,
        30 as libc::c_int as uint8_t,
        31 as libc::c_int as uint8_t,
        35 as libc::c_int as uint8_t,
        41 as libc::c_int as uint8_t,
        48 as libc::c_int as uint8_t,
        42 as libc::c_int as uint8_t,
        36 as libc::c_int as uint8_t,
        37 as libc::c_int as uint8_t,
        38 as libc::c_int as uint8_t,
        39 as libc::c_int as uint8_t,
        43 as libc::c_int as uint8_t,
        49 as libc::c_int as uint8_t,
        50 as libc::c_int as uint8_t,
        44 as libc::c_int as uint8_t,
        45 as libc::c_int as uint8_t,
        46 as libc::c_int as uint8_t,
        47 as libc::c_int as uint8_t,
        51 as libc::c_int as uint8_t,
        56 as libc::c_int as uint8_t,
        57 as libc::c_int as uint8_t,
        52 as libc::c_int as uint8_t,
        53 as libc::c_int as uint8_t,
        54 as libc::c_int as uint8_t,
        55 as libc::c_int as uint8_t,
        58 as libc::c_int as uint8_t,
        59 as libc::c_int as uint8_t,
        60 as libc::c_int as uint8_t,
        61 as libc::c_int as uint8_t,
        62 as libc::c_int as uint8_t,
        63 as libc::c_int as uint8_t,
    ],
];
static mut num_clock_ts: [uint8_t; 10] = [
    0 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
];
static mut avcintra_uuid: [uint8_t; 16] = [
    0xf7 as libc::c_int as uint8_t,
    0x49 as libc::c_int as uint8_t,
    0x3e as libc::c_int as uint8_t,
    0xb3 as libc::c_int as uint8_t,
    0xd4 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x47 as libc::c_int as uint8_t,
    0x96 as libc::c_int as uint8_t,
    0x86 as libc::c_int as uint8_t,
    0x86 as libc::c_int as uint8_t,
    0xc9 as libc::c_int as uint8_t,
    0x70 as libc::c_int as uint8_t,
    0x7b as libc::c_int as uint8_t,
    0x64 as libc::c_int as uint8_t,
    0x37 as libc::c_int as uint8_t,
    0x2a as libc::c_int as uint8_t,
];
unsafe extern "C" fn transpose(mut buf: *mut uint8_t, mut w: libc::c_int) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < w {
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < i {
            let mut t: uint8_t = *buf.offset((w * i + j) as isize);
            *buf.offset((w * i + j) as isize) = *buf.offset((w * j + i) as isize);
            *buf.offset((w * j + i) as isize) = t;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn scaling_list_write(
    mut s: *mut bs_t,
    mut sps: *mut x264_sps_t,
    mut idx: libc::c_int,
) {
    let len: libc::c_int = if idx < 4 as libc::c_int {
        16 as libc::c_int
    } else {
        64 as libc::c_int
    };
    let mut zigzag: *const uint8_t = if idx < 4 as libc::c_int {
        (x264_zigzag_scan4[0 as libc::c_int as usize]).as_ptr()
    } else {
        (x264_zigzag_scan8[0 as libc::c_int as usize]).as_ptr()
    };
    let mut list: *const uint8_t = (*sps).scaling_list[idx as usize];
    let mut def_list: *const uint8_t = if idx == CQM_4IC as libc::c_int {
        (*sps).scaling_list[CQM_4IY as libc::c_int as usize]
    } else if idx == CQM_4PC as libc::c_int {
        (*sps).scaling_list[CQM_4PY as libc::c_int as usize]
    } else if idx == CQM_8IC as libc::c_int + 4 as libc::c_int {
        (*sps).scaling_list[(CQM_8IY as libc::c_int + 4 as libc::c_int) as usize]
    } else if idx == CQM_8PC as libc::c_int + 4 as libc::c_int {
        (*sps).scaling_list[(CQM_8PY as libc::c_int + 4 as libc::c_int) as usize]
    } else {
        x264_cqm_jvt[idx as usize]
    };
    if memcmp(
        list as *const libc::c_void,
        def_list as *const libc::c_void,
        len as libc::c_ulong,
    ) == 0
    {
        bs_write1(s, 0 as libc::c_int as uint32_t);
    } else if memcmp(
        list as *const libc::c_void,
        x264_cqm_jvt[idx as usize] as *const libc::c_void,
        len as libc::c_ulong,
    ) == 0
    {
        bs_write1(s, 1 as libc::c_int as uint32_t);
        bs_write_se(s, -(8 as libc::c_int));
    } else {
        let mut run: libc::c_int = 0;
        bs_write1(s, 1 as libc::c_int as uint32_t);
        run = len;
        while run > 1 as libc::c_int {
            if *list.offset(*zigzag.offset((run - 1 as libc::c_int) as isize) as isize)
                as libc::c_int
                != *list.offset(*zigzag.offset((run - 2 as libc::c_int) as isize) as isize)
                    as libc::c_int
            {
                break;
            }
            run -= 1;
            run;
        }
        if run < len
            && len - run
                < bs_size_se(
                    -(*list.offset(*zigzag.offset(run as isize) as isize) as libc::c_int) as int8_t
                        as libc::c_int,
                )
        {
            run = len;
        }
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < run {
            bs_write_se(
                s,
                (*list.offset(*zigzag.offset(j as isize) as isize) as libc::c_int
                    - (if j > 0 as libc::c_int {
                        *list.offset(*zigzag.offset((j - 1 as libc::c_int) as isize) as isize)
                            as libc::c_int
                    } else {
                        8 as libc::c_int
                    })) as int8_t as libc::c_int,
            );
            j += 1;
            j;
        }
        if run < len {
            bs_write_se(
                s,
                -(*list.offset(*zigzag.offset(run as isize) as isize) as libc::c_int) as int8_t
                    as libc::c_int,
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_sei_write(
    mut s: *mut bs_t,
    mut payload: *mut uint8_t,
    mut payload_size: libc::c_int,
    mut payload_type: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    bs_realign(s);
    i = 0 as libc::c_int;
    while i <= payload_type - 255 as libc::c_int {
        bs_write(s, 8 as libc::c_int, 255 as libc::c_int as uint32_t);
        i += 255 as libc::c_int;
    }
    bs_write(s, 8 as libc::c_int, (payload_type - i) as uint32_t);
    i = 0 as libc::c_int;
    while i <= payload_size - 255 as libc::c_int {
        bs_write(s, 8 as libc::c_int, 255 as libc::c_int as uint32_t);
        i += 255 as libc::c_int;
    }
    bs_write(s, 8 as libc::c_int, (payload_size - i) as uint32_t);
    i = 0 as libc::c_int;
    while i < payload_size {
        bs_write(s, 8 as libc::c_int, *payload.offset(i as isize) as uint32_t);
        i += 1;
        i;
    }
    bs_rbsp_trailing(s);
    bs_flush(s);
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_sps_init(
    mut sps: *mut x264_sps_t,
    mut i_id: libc::c_int,
    mut param: *mut x264_param_t,
) {
    let mut csp: libc::c_int = (*param).i_csp & 0xff as libc::c_int;
    (*sps).i_id = i_id;
    (*sps).i_mb_width = ((*param).i_width + 15 as libc::c_int) / 16 as libc::c_int;
    (*sps).i_mb_height = ((*param).i_height + 15 as libc::c_int) / 16 as libc::c_int;
    (*sps).b_frame_mbs_only =
        !((*param).b_interlaced != 0 || (*param).b_fake_interlaced != 0) as libc::c_int;
    if (*sps).b_frame_mbs_only == 0 {
        (*sps).i_mb_height = ((*sps).i_mb_height + 1 as libc::c_int) & !(1 as libc::c_int);
    }
    (*sps).i_chroma_format_idc = if csp >= 0xc as libc::c_int {
        CHROMA_444 as libc::c_int
    } else if csp >= 0x6 as libc::c_int {
        CHROMA_422 as libc::c_int
    } else if csp >= 0x2 as libc::c_int {
        CHROMA_420 as libc::c_int
    } else {
        CHROMA_400 as libc::c_int
    };
    (*sps).b_qpprime_y_zero_transform_bypass = ((*param).rc.i_rc_method == 0 as libc::c_int
        && (*param).rc.i_qp_constant == 0 as libc::c_int)
        as libc::c_int;
    if (*sps).b_qpprime_y_zero_transform_bypass != 0
        || (*sps).i_chroma_format_idc == CHROMA_444 as libc::c_int
    {
        (*sps).i_profile_idc = PROFILE_HIGH444_PREDICTIVE as libc::c_int;
    } else if (*sps).i_chroma_format_idc == CHROMA_422 as libc::c_int {
        (*sps).i_profile_idc = PROFILE_HIGH422 as libc::c_int;
    } else if 8 as libc::c_int > 8 as libc::c_int {
        (*sps).i_profile_idc = PROFILE_HIGH10 as libc::c_int;
    } else if (*param).analyse.b_transform_8x8 != 0
        || (*param).i_cqm_preset != 0 as libc::c_int
        || (*sps).i_chroma_format_idc == CHROMA_400 as libc::c_int
    {
        (*sps).i_profile_idc = PROFILE_HIGH as libc::c_int;
    } else if (*param).b_cabac != 0
        || (*param).i_bframe > 0 as libc::c_int
        || (*param).b_interlaced != 0
        || (*param).b_fake_interlaced != 0
        || (*param).analyse.i_weighted_pred > 0 as libc::c_int
    {
        (*sps).i_profile_idc = PROFILE_MAIN as libc::c_int;
    } else {
        (*sps).i_profile_idc = PROFILE_BASELINE as libc::c_int;
    }
    (*sps).b_constraint_set0 =
        ((*sps).i_profile_idc == PROFILE_BASELINE as libc::c_int) as libc::c_int;
    (*sps).b_constraint_set1 = ((*sps).i_profile_idc <= PROFILE_MAIN as libc::c_int) as libc::c_int;
    (*sps).b_constraint_set2 = 0 as libc::c_int;
    (*sps).b_constraint_set3 = 0 as libc::c_int;
    (*sps).i_level_idc = (*param).i_level_idc;
    if (*param).i_level_idc == 9 as libc::c_int
        && ((*sps).i_profile_idc == PROFILE_BASELINE as libc::c_int
            || (*sps).i_profile_idc == PROFILE_MAIN as libc::c_int)
    {
        (*sps).b_constraint_set3 = 1 as libc::c_int;
        (*sps).i_level_idc = 11 as libc::c_int;
    }
    if (*param).i_keyint_max == 1 as libc::c_int
        && (*sps).i_profile_idc >= PROFILE_HIGH as libc::c_int
    {
        (*sps).b_constraint_set3 = 1 as libc::c_int;
    }
    (*sps).vui.i_num_reorder_frames = if (*param).i_bframe_pyramid != 0 {
        2 as libc::c_int
    } else if (*param).i_bframe != 0 {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    (*sps).i_num_ref_frames = if (16 as libc::c_int)
        < (if (*param).i_frame_reference
            > (if 1 as libc::c_int + (*sps).vui.i_num_reorder_frames
                > (if (if (*param).i_bframe_pyramid != 0 {
                    4 as libc::c_int
                } else {
                    1 as libc::c_int
                }) > (*param).i_dpb_size
                {
                    if (*param).i_bframe_pyramid != 0 {
                        4 as libc::c_int
                    } else {
                        1 as libc::c_int
                    }
                } else {
                    (*param).i_dpb_size
                })
            {
                1 as libc::c_int + (*sps).vui.i_num_reorder_frames
            } else if (if (*param).i_bframe_pyramid != 0 {
                4 as libc::c_int
            } else {
                1 as libc::c_int
            }) > (*param).i_dpb_size
            {
                if (*param).i_bframe_pyramid != 0 {
                    4 as libc::c_int
                } else {
                    1 as libc::c_int
                }
            } else {
                (*param).i_dpb_size
            })
        {
            (*param).i_frame_reference
        } else if 1 as libc::c_int + (*sps).vui.i_num_reorder_frames
            > (if (if (*param).i_bframe_pyramid != 0 {
                4 as libc::c_int
            } else {
                1 as libc::c_int
            }) > (*param).i_dpb_size
            {
                if (*param).i_bframe_pyramid != 0 {
                    4 as libc::c_int
                } else {
                    1 as libc::c_int
                }
            } else {
                (*param).i_dpb_size
            })
        {
            1 as libc::c_int + (*sps).vui.i_num_reorder_frames
        } else if (if (*param).i_bframe_pyramid != 0 {
            4 as libc::c_int
        } else {
            1 as libc::c_int
        }) > (*param).i_dpb_size
        {
            if (*param).i_bframe_pyramid != 0 {
                4 as libc::c_int
            } else {
                1 as libc::c_int
            }
        } else {
            (*param).i_dpb_size
        }) {
        16 as libc::c_int
    } else if (*param).i_frame_reference
        > (if 1 as libc::c_int + (*sps).vui.i_num_reorder_frames
            > (if (if (*param).i_bframe_pyramid != 0 {
                4 as libc::c_int
            } else {
                1 as libc::c_int
            }) > (*param).i_dpb_size
            {
                if (*param).i_bframe_pyramid != 0 {
                    4 as libc::c_int
                } else {
                    1 as libc::c_int
                }
            } else {
                (*param).i_dpb_size
            })
        {
            1 as libc::c_int + (*sps).vui.i_num_reorder_frames
        } else if (if (*param).i_bframe_pyramid != 0 {
            4 as libc::c_int
        } else {
            1 as libc::c_int
        }) > (*param).i_dpb_size
        {
            if (*param).i_bframe_pyramid != 0 {
                4 as libc::c_int
            } else {
                1 as libc::c_int
            }
        } else {
            (*param).i_dpb_size
        })
    {
        (*param).i_frame_reference
    } else if 1 as libc::c_int + (*sps).vui.i_num_reorder_frames
        > (if (if (*param).i_bframe_pyramid != 0 {
            4 as libc::c_int
        } else {
            1 as libc::c_int
        }) > (*param).i_dpb_size
        {
            if (*param).i_bframe_pyramid != 0 {
                4 as libc::c_int
            } else {
                1 as libc::c_int
            }
        } else {
            (*param).i_dpb_size
        })
    {
        1 as libc::c_int + (*sps).vui.i_num_reorder_frames
    } else if (if (*param).i_bframe_pyramid != 0 {
        4 as libc::c_int
    } else {
        1 as libc::c_int
    }) > (*param).i_dpb_size
    {
        if (*param).i_bframe_pyramid != 0 {
            4 as libc::c_int
        } else {
            1 as libc::c_int
        }
    } else {
        (*param).i_dpb_size
    };
    (*sps).vui.i_max_dec_frame_buffering = (*sps).i_num_ref_frames;
    (*sps).i_num_ref_frames -= ((*param).i_bframe_pyramid == 1 as libc::c_int) as libc::c_int;
    if (*param).i_keyint_max == 1 as libc::c_int {
        (*sps).i_num_ref_frames = 0 as libc::c_int;
        (*sps).vui.i_max_dec_frame_buffering = 0 as libc::c_int;
    }
    let mut max_frame_num: libc::c_int = (*sps).vui.i_max_dec_frame_buffering
        * (((*param).i_bframe_pyramid != 0) as libc::c_int + 1 as libc::c_int)
        + 1 as libc::c_int;
    if (*param).b_intra_refresh != 0 {
        let mut time_to_recovery: libc::c_int =
            (if ((*sps).i_mb_width - 1 as libc::c_int) < (*param).i_keyint_max {
                (*sps).i_mb_width - 1 as libc::c_int
            } else {
                (*param).i_keyint_max
            }) + (*param).i_bframe
                - 1 as libc::c_int;
        max_frame_num = if max_frame_num > time_to_recovery + 1 as libc::c_int {
            max_frame_num
        } else {
            time_to_recovery + 1 as libc::c_int
        };
    }
    (*sps).i_log2_max_frame_num = 4 as libc::c_int;
    while (1 as libc::c_int) << (*sps).i_log2_max_frame_num <= max_frame_num {
        (*sps).i_log2_max_frame_num += 1;
        (*sps).i_log2_max_frame_num;
    }
    (*sps).i_poc_type =
        if (*param).i_bframe != 0 || (*param).b_interlaced != 0 || (*param).i_avcintra_class != 0 {
            0 as libc::c_int
        } else {
            2 as libc::c_int
        };
    if (*sps).i_poc_type == 0 as libc::c_int {
        let mut max_delta_poc: libc::c_int = ((*param).i_bframe + 2 as libc::c_int)
            * (((*param).i_bframe_pyramid != 0) as libc::c_int + 1 as libc::c_int)
            * 2 as libc::c_int;
        (*sps).i_log2_max_poc_lsb = 4 as libc::c_int;
        while (1 as libc::c_int) << (*sps).i_log2_max_poc_lsb <= max_delta_poc * 2 as libc::c_int {
            (*sps).i_log2_max_poc_lsb += 1;
            (*sps).i_log2_max_poc_lsb;
        }
    }
    (*sps).b_vui = 1 as libc::c_int;
    (*sps).b_gaps_in_frame_num_value_allowed = 0 as libc::c_int;
    (*sps).b_mb_adaptive_frame_field = (*param).b_interlaced;
    (*sps).b_direct8x8_inference = 1 as libc::c_int;
    x264_8_sps_init_reconfigurable(sps, param);
    (*sps).vui.b_overscan_info_present = ((*param).vui.i_overscan > 0 as libc::c_int
        && (*param).vui.i_overscan <= 2 as libc::c_int)
        as libc::c_int;
    if (*sps).vui.b_overscan_info_present != 0 {
        (*sps).vui.b_overscan_info = if (*param).vui.i_overscan == 2 as libc::c_int {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        };
    }
    (*sps).vui.b_signal_type_present = 0 as libc::c_int;
    (*sps).vui.i_vidformat = if (*param).vui.i_vidformat >= 0 as libc::c_int
        && (*param).vui.i_vidformat <= 5 as libc::c_int
    {
        (*param).vui.i_vidformat
    } else {
        5 as libc::c_int
    };
    (*sps).vui.b_fullrange = if (*param).vui.b_fullrange >= 0 as libc::c_int
        && (*param).vui.b_fullrange <= 1 as libc::c_int
    {
        (*param).vui.b_fullrange
    } else if csp >= 0xe as libc::c_int {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    (*sps).vui.b_color_description_present = 0 as libc::c_int;
    (*sps).vui.i_colorprim = if (*param).vui.i_colorprim >= 0 as libc::c_int
        && (*param).vui.i_colorprim <= 12 as libc::c_int
    {
        (*param).vui.i_colorprim
    } else {
        2 as libc::c_int
    };
    (*sps).vui.i_transfer = if (*param).vui.i_transfer >= 0 as libc::c_int
        && (*param).vui.i_transfer <= 18 as libc::c_int
    {
        (*param).vui.i_transfer
    } else {
        2 as libc::c_int
    };
    (*sps).vui.i_colmatrix = if (*param).vui.i_colmatrix >= 0 as libc::c_int
        && (*param).vui.i_colmatrix <= 14 as libc::c_int
    {
        (*param).vui.i_colmatrix
    } else if csp >= 0xe as libc::c_int {
        0 as libc::c_int
    } else {
        2 as libc::c_int
    };
    if (*sps).vui.i_colorprim != 2 as libc::c_int
        || (*sps).vui.i_transfer != 2 as libc::c_int
        || (*sps).vui.i_colmatrix != 2 as libc::c_int
    {
        (*sps).vui.b_color_description_present = 1 as libc::c_int;
    }
    if (*sps).vui.i_vidformat != 5 as libc::c_int
        || (*sps).vui.b_fullrange != 0
        || (*sps).vui.b_color_description_present != 0
    {
        (*sps).vui.b_signal_type_present = 1 as libc::c_int;
    }
    (*sps).vui.b_chroma_loc_info_present = ((*param).vui.i_chroma_loc > 0 as libc::c_int
        && (*param).vui.i_chroma_loc <= 5 as libc::c_int
        && (*sps).i_chroma_format_idc == CHROMA_420 as libc::c_int)
        as libc::c_int;
    if (*sps).vui.b_chroma_loc_info_present != 0 {
        (*sps).vui.i_chroma_loc_top = (*param).vui.i_chroma_loc;
        (*sps).vui.i_chroma_loc_bottom = (*param).vui.i_chroma_loc;
    }
    (*sps).vui.b_timing_info_present = ((*param).i_timebase_num > 0 as libc::c_int as uint32_t
        && (*param).i_timebase_den > 0 as libc::c_int as uint32_t)
        as libc::c_int;
    if (*sps).vui.b_timing_info_present != 0 {
        (*sps).vui.i_num_units_in_tick = (*param).i_timebase_num;
        (*sps).vui.i_time_scale = (*param).i_timebase_den * 2 as libc::c_int as uint32_t;
        (*sps).vui.b_fixed_frame_rate = ((*param).b_vfr_input == 0) as libc::c_int;
    }
    (*sps).vui.b_vcl_hrd_parameters_present = 0 as libc::c_int;
    (*sps).vui.b_nal_hrd_parameters_present = ((*param).i_nal_hrd != 0) as libc::c_int;
    (*sps).vui.b_pic_struct_present = (*param).b_pic_struct;
    (*sps).vui.b_bitstream_restriction = !((*sps).b_constraint_set3 != 0
        && (*sps).i_profile_idc >= PROFILE_HIGH as libc::c_int)
        as libc::c_int;
    if (*sps).vui.b_bitstream_restriction != 0 {
        (*sps).vui.b_motion_vectors_over_pic_boundaries = 1 as libc::c_int;
        (*sps).vui.i_max_bytes_per_pic_denom = 0 as libc::c_int;
        (*sps).vui.i_max_bits_per_mb_denom = 0 as libc::c_int;
        (*sps).vui.i_log2_max_mv_length_vertical = log2f(
            (if 1 as libc::c_int > (*param).analyse.i_mv_range * 4 as libc::c_int - 1 as libc::c_int
            {
                1 as libc::c_int
            } else {
                (*param).analyse.i_mv_range * 4 as libc::c_int - 1 as libc::c_int
            }) as libc::c_float,
        ) as libc::c_int
            + 1 as libc::c_int;
        (*sps).vui.i_log2_max_mv_length_horizontal = (*sps).vui.i_log2_max_mv_length_vertical;
    }
    (*sps).b_avcintra_hd = ((*param).i_avcintra_class != 0
        && (*param).i_avcintra_class <= 200 as libc::c_int)
        as libc::c_int;
    (*sps).b_avcintra_4k = ((*param).i_avcintra_class > 200 as libc::c_int) as libc::c_int;
    (*sps).i_cqm_preset = (*param).i_cqm_preset;
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_sps_init_reconfigurable(
    mut sps: *mut x264_sps_t,
    mut param: *mut x264_param_t,
) {
    (*sps).crop.i_left = (*param).crop_rect.i_left;
    (*sps).crop.i_top = (*param).crop_rect.i_top;
    (*sps).crop.i_right =
        (*param).crop_rect.i_right + (*sps).i_mb_width * 16 as libc::c_int - (*param).i_width;
    (*sps).crop.i_bottom =
        (*param).crop_rect.i_bottom + (*sps).i_mb_height * 16 as libc::c_int - (*param).i_height;
    (*sps).b_crop = ((*sps).crop.i_left != 0
        || (*sps).crop.i_top != 0
        || (*sps).crop.i_right != 0
        || (*sps).crop.i_bottom != 0) as libc::c_int;
    (*sps).vui.b_aspect_ratio_info_present = 0 as libc::c_int;
    if (*param).vui.i_sar_width > 0 as libc::c_int && (*param).vui.i_sar_height > 0 as libc::c_int {
        (*sps).vui.b_aspect_ratio_info_present = 1 as libc::c_int;
        (*sps).vui.i_sar_width = (*param).vui.i_sar_width;
        (*sps).vui.i_sar_height = (*param).vui.i_sar_height;
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_sps_init_scaling_list(
    mut sps: *mut x264_sps_t,
    mut param: *mut x264_param_t,
) {
    match (*sps).i_cqm_preset {
        0 => {
            let mut i: libc::c_int = 0 as libc::c_int;
            while i < 8 as libc::c_int {
                (*sps).scaling_list[i as usize] = x264_cqm_flat16.as_ptr();
                i += 1;
                i;
            }
        }
        1 => {
            let mut i_0: libc::c_int = 0 as libc::c_int;
            while i_0 < 8 as libc::c_int {
                (*sps).scaling_list[i_0 as usize] = x264_cqm_jvt[i_0 as usize];
                i_0 += 1;
                i_0;
            }
        }
        2 => {
            transpose(((*param).cqm_4iy).as_mut_ptr(), 4 as libc::c_int);
            transpose(((*param).cqm_4py).as_mut_ptr(), 4 as libc::c_int);
            transpose(((*param).cqm_4ic).as_mut_ptr(), 4 as libc::c_int);
            transpose(((*param).cqm_4pc).as_mut_ptr(), 4 as libc::c_int);
            transpose(((*param).cqm_8iy).as_mut_ptr(), 8 as libc::c_int);
            transpose(((*param).cqm_8py).as_mut_ptr(), 8 as libc::c_int);
            transpose(((*param).cqm_8ic).as_mut_ptr(), 8 as libc::c_int);
            transpose(((*param).cqm_8pc).as_mut_ptr(), 8 as libc::c_int);
            (*sps).scaling_list[CQM_4IY as libc::c_int as usize] = ((*param).cqm_4iy).as_mut_ptr();
            (*sps).scaling_list[CQM_4PY as libc::c_int as usize] = ((*param).cqm_4py).as_mut_ptr();
            (*sps).scaling_list[CQM_4IC as libc::c_int as usize] = ((*param).cqm_4ic).as_mut_ptr();
            (*sps).scaling_list[CQM_4PC as libc::c_int as usize] = ((*param).cqm_4pc).as_mut_ptr();
            (*sps).scaling_list[(CQM_8IY as libc::c_int + 4 as libc::c_int) as usize] =
                ((*param).cqm_8iy).as_mut_ptr();
            (*sps).scaling_list[(CQM_8PY as libc::c_int + 4 as libc::c_int) as usize] =
                ((*param).cqm_8py).as_mut_ptr();
            (*sps).scaling_list[(CQM_8IC as libc::c_int + 4 as libc::c_int) as usize] =
                ((*param).cqm_8ic).as_mut_ptr();
            (*sps).scaling_list[(CQM_8PC as libc::c_int + 4 as libc::c_int) as usize] =
                ((*param).cqm_8pc).as_mut_ptr();
            let mut i_1: libc::c_int = 0 as libc::c_int;
            while i_1 < 8 as libc::c_int {
                let mut j: libc::c_int = 0 as libc::c_int;
                while j
                    < (if i_1 < 4 as libc::c_int {
                        16 as libc::c_int
                    } else {
                        64 as libc::c_int
                    })
                {
                    if *((*sps).scaling_list[i_1 as usize]).offset(j as isize) as libc::c_int
                        == 0 as libc::c_int
                    {
                        (*sps).scaling_list[i_1 as usize] = x264_cqm_jvt[i_1 as usize];
                    }
                    j += 1;
                    j;
                }
                i_1 += 1;
                i_1;
            }
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_sps_write(mut s: *mut bs_t, mut sps: *mut x264_sps_t) {
    bs_realign(s);
    bs_write(s, 8 as libc::c_int, (*sps).i_profile_idc as uint32_t);
    bs_write1(s, (*sps).b_constraint_set0 as uint32_t);
    bs_write1(s, (*sps).b_constraint_set1 as uint32_t);
    bs_write1(s, (*sps).b_constraint_set2 as uint32_t);
    bs_write1(s, (*sps).b_constraint_set3 as uint32_t);
    bs_write(s, 4 as libc::c_int, 0 as libc::c_int as uint32_t);
    bs_write(s, 8 as libc::c_int, (*sps).i_level_idc as uint32_t);
    bs_write_ue_big(s, (*sps).i_id as libc::c_uint);
    if (*sps).i_profile_idc >= PROFILE_HIGH as libc::c_int {
        bs_write_ue_big(s, (*sps).i_chroma_format_idc as libc::c_uint);
        if (*sps).i_chroma_format_idc == CHROMA_444 as libc::c_int {
            bs_write1(s, 0 as libc::c_int as uint32_t);
        }
        bs_write_ue_big(s, (8 as libc::c_int - 8 as libc::c_int) as libc::c_uint);
        bs_write_ue_big(s, (8 as libc::c_int - 8 as libc::c_int) as libc::c_uint);
        bs_write1(s, (*sps).b_qpprime_y_zero_transform_bypass as uint32_t);
        bs_write1(s, (*sps).b_avcintra_hd as uint32_t);
        if (*sps).b_avcintra_hd != 0 {
            scaling_list_write(s, sps, CQM_4IY as libc::c_int);
            scaling_list_write(s, sps, CQM_4IC as libc::c_int);
            scaling_list_write(s, sps, CQM_4IC as libc::c_int);
            bs_write1(s, 0 as libc::c_int as uint32_t);
            bs_write1(s, 0 as libc::c_int as uint32_t);
            bs_write1(s, 0 as libc::c_int as uint32_t);
            scaling_list_write(s, sps, CQM_8IY as libc::c_int + 4 as libc::c_int);
            bs_write1(s, 0 as libc::c_int as uint32_t);
            if (*sps).i_chroma_format_idc == CHROMA_444 as libc::c_int {
                scaling_list_write(s, sps, CQM_8IC as libc::c_int + 4 as libc::c_int);
                bs_write1(s, 0 as libc::c_int as uint32_t);
                scaling_list_write(s, sps, CQM_8IC as libc::c_int + 4 as libc::c_int);
                bs_write1(s, 0 as libc::c_int as uint32_t);
            }
        }
    }
    bs_write_ue_big(
        s,
        ((*sps).i_log2_max_frame_num - 4 as libc::c_int) as libc::c_uint,
    );
    bs_write_ue_big(s, (*sps).i_poc_type as libc::c_uint);
    if (*sps).i_poc_type == 0 as libc::c_int {
        bs_write_ue_big(
            s,
            ((*sps).i_log2_max_poc_lsb - 4 as libc::c_int) as libc::c_uint,
        );
    }
    bs_write_ue_big(s, (*sps).i_num_ref_frames as libc::c_uint);
    bs_write1(s, (*sps).b_gaps_in_frame_num_value_allowed as uint32_t);
    bs_write_ue_big(s, ((*sps).i_mb_width - 1 as libc::c_int) as libc::c_uint);
    bs_write_ue_big(
        s,
        (((*sps).i_mb_height >> ((*sps).b_frame_mbs_only == 0) as libc::c_int) - 1 as libc::c_int)
            as libc::c_uint,
    );
    bs_write1(s, (*sps).b_frame_mbs_only as uint32_t);
    if (*sps).b_frame_mbs_only == 0 {
        bs_write1(s, (*sps).b_mb_adaptive_frame_field as uint32_t);
    }
    bs_write1(s, (*sps).b_direct8x8_inference as uint32_t);
    bs_write1(s, (*sps).b_crop as uint32_t);
    if (*sps).b_crop != 0 {
        let mut h_shift: libc::c_int = ((*sps).i_chroma_format_idc == CHROMA_420 as libc::c_int
            || (*sps).i_chroma_format_idc == CHROMA_422 as libc::c_int)
            as libc::c_int;
        let mut v_shift: libc::c_int = ((*sps).i_chroma_format_idc == CHROMA_420 as libc::c_int)
            as libc::c_int
            + ((*sps).b_frame_mbs_only == 0) as libc::c_int;
        bs_write_ue_big(s, ((*sps).crop.i_left >> h_shift) as libc::c_uint);
        bs_write_ue_big(s, ((*sps).crop.i_right >> h_shift) as libc::c_uint);
        bs_write_ue_big(s, ((*sps).crop.i_top >> v_shift) as libc::c_uint);
        bs_write_ue_big(s, ((*sps).crop.i_bottom >> v_shift) as libc::c_uint);
    }
    bs_write1(s, (*sps).b_vui as uint32_t);
    if (*sps).b_vui != 0 {
        bs_write1(s, (*sps).vui.b_aspect_ratio_info_present as uint32_t);
        if (*sps).vui.b_aspect_ratio_info_present != 0 {
            let mut i: libc::c_int = 0;
            static mut sar: [C2RustUnnamed_20; 17] = [
                {
                    C2RustUnnamed_20 {
                        w: 1 as libc::c_int as uint8_t,
                        h: 1 as libc::c_int as uint8_t,
                        sar: 1 as libc::c_int as uint8_t,
                    }
                },
                {
                    C2RustUnnamed_20 {
                        w: 12 as libc::c_int as uint8_t,
                        h: 11 as libc::c_int as uint8_t,
                        sar: 2 as libc::c_int as uint8_t,
                    }
                },
                {
                    C2RustUnnamed_20 {
                        w: 10 as libc::c_int as uint8_t,
                        h: 11 as libc::c_int as uint8_t,
                        sar: 3 as libc::c_int as uint8_t,
                    }
                },
                {
                    C2RustUnnamed_20 {
                        w: 16 as libc::c_int as uint8_t,
                        h: 11 as libc::c_int as uint8_t,
                        sar: 4 as libc::c_int as uint8_t,
                    }
                },
                {
                    C2RustUnnamed_20 {
                        w: 40 as libc::c_int as uint8_t,
                        h: 33 as libc::c_int as uint8_t,
                        sar: 5 as libc::c_int as uint8_t,
                    }
                },
                {
                    C2RustUnnamed_20 {
                        w: 24 as libc::c_int as uint8_t,
                        h: 11 as libc::c_int as uint8_t,
                        sar: 6 as libc::c_int as uint8_t,
                    }
                },
                {
                    C2RustUnnamed_20 {
                        w: 20 as libc::c_int as uint8_t,
                        h: 11 as libc::c_int as uint8_t,
                        sar: 7 as libc::c_int as uint8_t,
                    }
                },
                {
                    C2RustUnnamed_20 {
                        w: 32 as libc::c_int as uint8_t,
                        h: 11 as libc::c_int as uint8_t,
                        sar: 8 as libc::c_int as uint8_t,
                    }
                },
                {
                    C2RustUnnamed_20 {
                        w: 80 as libc::c_int as uint8_t,
                        h: 33 as libc::c_int as uint8_t,
                        sar: 9 as libc::c_int as uint8_t,
                    }
                },
                {
                    C2RustUnnamed_20 {
                        w: 18 as libc::c_int as uint8_t,
                        h: 11 as libc::c_int as uint8_t,
                        sar: 10 as libc::c_int as uint8_t,
                    }
                },
                {
                    C2RustUnnamed_20 {
                        w: 15 as libc::c_int as uint8_t,
                        h: 11 as libc::c_int as uint8_t,
                        sar: 11 as libc::c_int as uint8_t,
                    }
                },
                {
                    C2RustUnnamed_20 {
                        w: 64 as libc::c_int as uint8_t,
                        h: 33 as libc::c_int as uint8_t,
                        sar: 12 as libc::c_int as uint8_t,
                    }
                },
                {
                    C2RustUnnamed_20 {
                        w: 160 as libc::c_int as uint8_t,
                        h: 99 as libc::c_int as uint8_t,
                        sar: 13 as libc::c_int as uint8_t,
                    }
                },
                {
                    C2RustUnnamed_20 {
                        w: 4 as libc::c_int as uint8_t,
                        h: 3 as libc::c_int as uint8_t,
                        sar: 14 as libc::c_int as uint8_t,
                    }
                },
                {
                    C2RustUnnamed_20 {
                        w: 3 as libc::c_int as uint8_t,
                        h: 2 as libc::c_int as uint8_t,
                        sar: 15 as libc::c_int as uint8_t,
                    }
                },
                {
                    C2RustUnnamed_20 {
                        w: 2 as libc::c_int as uint8_t,
                        h: 1 as libc::c_int as uint8_t,
                        sar: 16 as libc::c_int as uint8_t,
                    }
                },
                {
                    C2RustUnnamed_20 {
                        w: 0 as libc::c_int as uint8_t,
                        h: 0 as libc::c_int as uint8_t,
                        sar: 255 as libc::c_int as uint8_t,
                    }
                },
            ];
            i = 0 as libc::c_int;
            while sar[i as usize].sar as libc::c_int != 255 as libc::c_int {
                if sar[i as usize].w as libc::c_int == (*sps).vui.i_sar_width
                    && sar[i as usize].h as libc::c_int == (*sps).vui.i_sar_height
                {
                    break;
                }
                i += 1;
                i;
            }
            bs_write(s, 8 as libc::c_int, sar[i as usize].sar as uint32_t);
            if sar[i as usize].sar as libc::c_int == 255 as libc::c_int {
                bs_write(s, 16 as libc::c_int, (*sps).vui.i_sar_width as uint32_t);
                bs_write(s, 16 as libc::c_int, (*sps).vui.i_sar_height as uint32_t);
            }
        }
        bs_write1(s, (*sps).vui.b_overscan_info_present as uint32_t);
        if (*sps).vui.b_overscan_info_present != 0 {
            bs_write1(s, (*sps).vui.b_overscan_info as uint32_t);
        }
        bs_write1(s, (*sps).vui.b_signal_type_present as uint32_t);
        if (*sps).vui.b_signal_type_present != 0 {
            bs_write(s, 3 as libc::c_int, (*sps).vui.i_vidformat as uint32_t);
            bs_write1(s, (*sps).vui.b_fullrange as uint32_t);
            bs_write1(s, (*sps).vui.b_color_description_present as uint32_t);
            if (*sps).vui.b_color_description_present != 0 {
                bs_write(s, 8 as libc::c_int, (*sps).vui.i_colorprim as uint32_t);
                bs_write(s, 8 as libc::c_int, (*sps).vui.i_transfer as uint32_t);
                bs_write(s, 8 as libc::c_int, (*sps).vui.i_colmatrix as uint32_t);
            }
        }
        bs_write1(s, (*sps).vui.b_chroma_loc_info_present as uint32_t);
        if (*sps).vui.b_chroma_loc_info_present != 0 {
            bs_write_ue_big(s, (*sps).vui.i_chroma_loc_top as libc::c_uint);
            bs_write_ue_big(s, (*sps).vui.i_chroma_loc_bottom as libc::c_uint);
        }
        bs_write1(s, (*sps).vui.b_timing_info_present as uint32_t);
        if (*sps).vui.b_timing_info_present != 0 {
            bs_write32(s, (*sps).vui.i_num_units_in_tick);
            bs_write32(s, (*sps).vui.i_time_scale);
            bs_write1(s, (*sps).vui.b_fixed_frame_rate as uint32_t);
        }
        bs_write1(s, (*sps).vui.b_nal_hrd_parameters_present as uint32_t);
        if (*sps).vui.b_nal_hrd_parameters_present != 0 {
            bs_write_ue_big(
                s,
                ((*sps).vui.hrd.i_cpb_cnt - 1 as libc::c_int) as libc::c_uint,
            );
            bs_write(
                s,
                4 as libc::c_int,
                (*sps).vui.hrd.i_bit_rate_scale as uint32_t,
            );
            bs_write(
                s,
                4 as libc::c_int,
                (*sps).vui.hrd.i_cpb_size_scale as uint32_t,
            );
            bs_write_ue_big(
                s,
                ((*sps).vui.hrd.i_bit_rate_value - 1 as libc::c_int) as libc::c_uint,
            );
            bs_write_ue_big(
                s,
                ((*sps).vui.hrd.i_cpb_size_value - 1 as libc::c_int) as libc::c_uint,
            );
            bs_write1(s, (*sps).vui.hrd.b_cbr_hrd as uint32_t);
            bs_write(
                s,
                5 as libc::c_int,
                ((*sps).vui.hrd.i_initial_cpb_removal_delay_length - 1 as libc::c_int) as uint32_t,
            );
            bs_write(
                s,
                5 as libc::c_int,
                ((*sps).vui.hrd.i_cpb_removal_delay_length - 1 as libc::c_int) as uint32_t,
            );
            bs_write(
                s,
                5 as libc::c_int,
                ((*sps).vui.hrd.i_dpb_output_delay_length - 1 as libc::c_int) as uint32_t,
            );
            bs_write(
                s,
                5 as libc::c_int,
                (*sps).vui.hrd.i_time_offset_length as uint32_t,
            );
        }
        bs_write1(s, (*sps).vui.b_vcl_hrd_parameters_present as uint32_t);
        if (*sps).vui.b_nal_hrd_parameters_present != 0
            || (*sps).vui.b_vcl_hrd_parameters_present != 0
        {
            bs_write1(s, 0 as libc::c_int as uint32_t);
        }
        bs_write1(s, (*sps).vui.b_pic_struct_present as uint32_t);
        bs_write1(s, (*sps).vui.b_bitstream_restriction as uint32_t);
        if (*sps).vui.b_bitstream_restriction != 0 {
            bs_write1(
                s,
                (*sps).vui.b_motion_vectors_over_pic_boundaries as uint32_t,
            );
            bs_write_ue_big(s, (*sps).vui.i_max_bytes_per_pic_denom as libc::c_uint);
            bs_write_ue_big(s, (*sps).vui.i_max_bits_per_mb_denom as libc::c_uint);
            bs_write_ue_big(
                s,
                (*sps).vui.i_log2_max_mv_length_horizontal as libc::c_uint,
            );
            bs_write_ue_big(s, (*sps).vui.i_log2_max_mv_length_vertical as libc::c_uint);
            bs_write_ue_big(s, (*sps).vui.i_num_reorder_frames as libc::c_uint);
            bs_write_ue_big(s, (*sps).vui.i_max_dec_frame_buffering as libc::c_uint);
        }
    }
    bs_rbsp_trailing(s);
    bs_flush(s);
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_pps_init(
    mut pps: *mut x264_pps_t,
    mut i_id: libc::c_int,
    mut param: *mut x264_param_t,
    mut sps: *mut x264_sps_t,
) {
    (*pps).i_id = i_id;
    (*pps).i_sps_id = (*sps).i_id;
    (*pps).b_cabac = (*param).b_cabac;
    (*pps).b_pic_order =
        ((*param).i_avcintra_class == 0 && (*param).b_interlaced != 0) as libc::c_int;
    (*pps).i_num_slice_groups = 1 as libc::c_int;
    (*pps).i_num_ref_idx_l0_default_active = (*param).i_frame_reference;
    (*pps).i_num_ref_idx_l1_default_active = 1 as libc::c_int;
    (*pps).b_weighted_pred = ((*param).analyse.i_weighted_pred > 0 as libc::c_int) as libc::c_int;
    (*pps).b_weighted_bipred = if (*param).analyse.b_weighted_bipred != 0 {
        2 as libc::c_int
    } else {
        0 as libc::c_int
    };
    (*pps).i_pic_init_qp =
        if (*param).rc.i_rc_method == 2 as libc::c_int || (*param).b_stitchable != 0 {
            26 as libc::c_int + 6 as libc::c_int * (8 as libc::c_int - 8 as libc::c_int)
        } else if (*param).rc.i_qp_constant
            < 51 as libc::c_int + 6 as libc::c_int * (8 as libc::c_int - 8 as libc::c_int)
        {
            (*param).rc.i_qp_constant
        } else {
            51 as libc::c_int + 6 as libc::c_int * (8 as libc::c_int - 8 as libc::c_int)
        };
    (*pps).i_pic_init_qs =
        26 as libc::c_int + 6 as libc::c_int * (8 as libc::c_int - 8 as libc::c_int);
    (*pps).i_chroma_qp_index_offset = (*param).analyse.i_chroma_qp_offset;
    (*pps).b_deblocking_filter_control = 1 as libc::c_int;
    (*pps).b_constrained_intra_pred = (*param).b_constrained_intra;
    (*pps).b_redundant_pic_cnt = 0 as libc::c_int;
    (*pps).b_transform_8x8_mode = if (*param).analyse.b_transform_8x8 != 0 {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_pps_write(
    mut s: *mut bs_t,
    mut sps: *mut x264_sps_t,
    mut pps: *mut x264_pps_t,
) {
    bs_realign(s);
    bs_write_ue_big(s, (*pps).i_id as libc::c_uint);
    bs_write_ue_big(s, (*pps).i_sps_id as libc::c_uint);
    bs_write1(s, (*pps).b_cabac as uint32_t);
    bs_write1(s, (*pps).b_pic_order as uint32_t);
    bs_write_ue_big(
        s,
        ((*pps).i_num_slice_groups - 1 as libc::c_int) as libc::c_uint,
    );
    bs_write_ue_big(
        s,
        ((*pps).i_num_ref_idx_l0_default_active - 1 as libc::c_int) as libc::c_uint,
    );
    bs_write_ue_big(
        s,
        ((*pps).i_num_ref_idx_l1_default_active - 1 as libc::c_int) as libc::c_uint,
    );
    bs_write1(s, (*pps).b_weighted_pred as uint32_t);
    bs_write(s, 2 as libc::c_int, (*pps).b_weighted_bipred as uint32_t);
    bs_write_se(
        s,
        (*pps).i_pic_init_qp
            - 26 as libc::c_int
            - 6 as libc::c_int * (8 as libc::c_int - 8 as libc::c_int),
    );
    bs_write_se(
        s,
        (*pps).i_pic_init_qs
            - 26 as libc::c_int
            - 6 as libc::c_int * (8 as libc::c_int - 8 as libc::c_int),
    );
    bs_write_se(s, (*pps).i_chroma_qp_index_offset);
    bs_write1(s, (*pps).b_deblocking_filter_control as uint32_t);
    bs_write1(s, (*pps).b_constrained_intra_pred as uint32_t);
    bs_write1(s, (*pps).b_redundant_pic_cnt as uint32_t);
    let mut b_scaling_list: libc::c_int =
        ((*sps).b_avcintra_hd == 0 && (*sps).i_cqm_preset != 0 as libc::c_int) as libc::c_int;
    if (*pps).b_transform_8x8_mode != 0 || b_scaling_list != 0 {
        bs_write1(s, (*pps).b_transform_8x8_mode as uint32_t);
        bs_write1(s, b_scaling_list as uint32_t);
        if b_scaling_list != 0 {
            scaling_list_write(s, sps, CQM_4IY as libc::c_int);
            scaling_list_write(s, sps, CQM_4IC as libc::c_int);
            if (*sps).b_avcintra_4k != 0 {
                scaling_list_write(s, sps, CQM_4IC as libc::c_int);
                bs_write1(s, 0 as libc::c_int as uint32_t);
                bs_write1(s, 0 as libc::c_int as uint32_t);
                bs_write1(s, 0 as libc::c_int as uint32_t);
            } else {
                bs_write1(s, 0 as libc::c_int as uint32_t);
                scaling_list_write(s, sps, CQM_4PY as libc::c_int);
                scaling_list_write(s, sps, CQM_4PC as libc::c_int);
                bs_write1(s, 0 as libc::c_int as uint32_t);
            }
            if (*pps).b_transform_8x8_mode != 0 {
                scaling_list_write(s, sps, CQM_8IY as libc::c_int + 4 as libc::c_int);
                if (*sps).b_avcintra_4k != 0 {
                    bs_write1(s, 0 as libc::c_int as uint32_t);
                } else {
                    scaling_list_write(s, sps, CQM_8PY as libc::c_int + 4 as libc::c_int);
                }
                if (*sps).i_chroma_format_idc == CHROMA_444 as libc::c_int {
                    scaling_list_write(s, sps, CQM_8IC as libc::c_int + 4 as libc::c_int);
                    scaling_list_write(s, sps, CQM_8PC as libc::c_int + 4 as libc::c_int);
                    bs_write1(s, 0 as libc::c_int as uint32_t);
                    bs_write1(s, 0 as libc::c_int as uint32_t);
                }
            }
        }
        bs_write_se(s, (*pps).i_chroma_qp_index_offset);
    }
    bs_rbsp_trailing(s);
    bs_flush(s);
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_sei_recovery_point_write(
    mut h: *mut x264_t,
    mut s: *mut bs_t,
    mut recovery_frame_cnt: libc::c_int,
) {
    let mut q: bs_t = bs_s {
        p_start: std::ptr::null_mut::<uint8_t>(),
        p: std::ptr::null_mut::<uint8_t>(),
        p_end: std::ptr::null_mut::<uint8_t>(),
        cur_bits: 0,
        i_left: 0,
        i_bits_encoded: 0,
    };
    let mut tmp_buf: [uint8_t; 100] = [0; 100];
    (*(tmp_buf.as_mut_ptr() as *mut x264_union32_t)).i = 0 as libc::c_int as uint32_t;
    bs_init(
        &mut q,
        tmp_buf.as_mut_ptr() as *mut libc::c_void,
        100 as libc::c_int,
    );
    bs_realign(&mut q);
    bs_write_ue_big(&mut q, recovery_frame_cnt as libc::c_uint);
    bs_write1(&mut q, 1 as libc::c_int as uint32_t);
    bs_write1(&mut q, 0 as libc::c_int as uint32_t);
    bs_write(&mut q, 2 as libc::c_int, 0 as libc::c_int as uint32_t);
    bs_align_10(&mut q);
    x264_8_sei_write(
        s,
        tmp_buf.as_mut_ptr(),
        bs_pos(&mut q) / 8 as libc::c_int,
        SEI_RECOVERY_POINT as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_sei_version_write(
    mut h: *mut x264_t,
    mut s: *mut bs_t,
) -> libc::c_int {
    static mut uuid: [uint8_t; 16] = [
        0xdc as libc::c_int as uint8_t,
        0x45 as libc::c_int as uint8_t,
        0xe9 as libc::c_int as uint8_t,
        0xbd as libc::c_int as uint8_t,
        0xe6 as libc::c_int as uint8_t,
        0xd9 as libc::c_int as uint8_t,
        0x48 as libc::c_int as uint8_t,
        0xb7 as libc::c_int as uint8_t,
        0x96 as libc::c_int as uint8_t,
        0x2c as libc::c_int as uint8_t,
        0xd8 as libc::c_int as uint8_t,
        0x20 as libc::c_int as uint8_t,
        0xd9 as libc::c_int as uint8_t,
        0x23 as libc::c_int as uint8_t,
        0xee as libc::c_int as uint8_t,
        0xef as libc::c_int as uint8_t,
    ];
    let mut opts: *mut libc::c_char = x264_param2string(&mut (*h).param, 0 as libc::c_int);
    let mut payload: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    let mut length: libc::c_int = 0;
    if opts.is_null() {
        return -(1 as libc::c_int);
    }
    payload =
        x264_malloc((200 as libc::c_int as libc::c_ulong).wrapping_add(strlen(opts)) as int64_t)
            as *mut libc::c_char;
    if payload.is_null() {
        x264_free(opts as *mut libc::c_void);
        -(1 as libc::c_int)
    } else {
        memcpy(
            payload as *mut libc::c_void,
            uuid.as_ptr() as *const libc::c_void,
            16 as libc::c_int as libc::c_ulong,
        );
        sprintf(
            payload.offset(16 as libc::c_int as isize),
            b"x264 - core %d%s - H.264/MPEG-4 AVC codec - Copy%s 2003-2025 - http://www.videolan.org/x264.html - options: %s\0"
                as *const u8 as *const libc::c_char,
            164 as libc::c_int,
            b" r3204 373697b\0" as *const u8 as *const libc::c_char,
            if 1 as libc::c_int != 0 {
                b"left\0" as *const u8 as *const libc::c_char
            } else {
                b"right\0" as *const u8 as *const libc::c_char
            },
            opts,
        );
        length = (strlen(payload)).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
        x264_8_sei_write(
            s,
            payload as *mut uint8_t,
            length,
            SEI_USER_DATA_UNREGISTERED as libc::c_int,
        );
        x264_free(opts as *mut libc::c_void);
        x264_free(payload as *mut libc::c_void);
        0 as libc::c_int
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_sei_buffering_period_write(mut h: *mut x264_t, mut s: *mut bs_t) {
    let mut sps: *mut x264_sps_t = ((*h).sps).as_mut_ptr();
    let mut q: bs_t = bs_s {
        p_start: std::ptr::null_mut::<uint8_t>(),
        p: std::ptr::null_mut::<uint8_t>(),
        p_end: std::ptr::null_mut::<uint8_t>(),
        cur_bits: 0,
        i_left: 0,
        i_bits_encoded: 0,
    };
    let mut tmp_buf: [uint8_t; 100] = [0; 100];
    (*(tmp_buf.as_mut_ptr() as *mut x264_union32_t)).i = 0 as libc::c_int as uint32_t;
    bs_init(
        &mut q,
        tmp_buf.as_mut_ptr() as *mut libc::c_void,
        100 as libc::c_int,
    );
    bs_realign(&mut q);
    bs_write_ue_big(&mut q, (*sps).i_id as libc::c_uint);
    if (*sps).vui.b_nal_hrd_parameters_present != 0 {
        bs_write(
            &mut q,
            (*sps).vui.hrd.i_initial_cpb_removal_delay_length,
            (*h).initial_cpb_removal_delay as uint32_t,
        );
        bs_write(
            &mut q,
            (*sps).vui.hrd.i_initial_cpb_removal_delay_length,
            (*h).initial_cpb_removal_delay_offset as uint32_t,
        );
    }
    bs_align_10(&mut q);
    x264_8_sei_write(
        s,
        tmp_buf.as_mut_ptr(),
        bs_pos(&mut q) / 8 as libc::c_int,
        SEI_BUFFERING_PERIOD as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_sei_pic_timing_write(mut h: *mut x264_t, mut s: *mut bs_t) {
    let mut sps: *mut x264_sps_t = ((*h).sps).as_mut_ptr();
    let mut q: bs_t = bs_s {
        p_start: std::ptr::null_mut::<uint8_t>(),
        p: std::ptr::null_mut::<uint8_t>(),
        p_end: std::ptr::null_mut::<uint8_t>(),
        cur_bits: 0,
        i_left: 0,
        i_bits_encoded: 0,
    };
    let mut tmp_buf: [uint8_t; 100] = [0; 100];
    (*(tmp_buf.as_mut_ptr() as *mut x264_union32_t)).i = 0 as libc::c_int as uint32_t;
    bs_init(
        &mut q,
        tmp_buf.as_mut_ptr() as *mut libc::c_void,
        100 as libc::c_int,
    );
    bs_realign(&mut q);
    if (*sps).vui.b_nal_hrd_parameters_present != 0 || (*sps).vui.b_vcl_hrd_parameters_present != 0
    {
        bs_write(
            &mut q,
            (*sps).vui.hrd.i_cpb_removal_delay_length,
            ((*(*h).fenc).i_cpb_delay - (*h).i_cpb_delay_pir_offset) as uint32_t,
        );
        bs_write(
            &mut q,
            (*sps).vui.hrd.i_dpb_output_delay_length,
            (*(*h).fenc).i_dpb_output_delay as uint32_t,
        );
    }
    if (*sps).vui.b_pic_struct_present != 0 {
        bs_write(
            &mut q,
            4 as libc::c_int,
            ((*(*h).fenc).i_pic_struct - 1 as libc::c_int) as uint32_t,
        );
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < num_clock_ts[(*(*h).fenc).i_pic_struct as usize] as libc::c_int {
            bs_write1(&mut q, 0 as libc::c_int as uint32_t);
            i += 1;
            i;
        }
    }
    bs_align_10(&mut q);
    x264_8_sei_write(
        s,
        tmp_buf.as_mut_ptr(),
        bs_pos(&mut q) / 8 as libc::c_int,
        SEI_PIC_TIMING as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_sei_frame_packing_write(mut h: *mut x264_t, mut s: *mut bs_t) {
    let mut quincunx_sampling_flag: libc::c_int =
        ((*h).param.i_frame_packing == 0 as libc::c_int) as libc::c_int;
    let mut q: bs_t = bs_s {
        p_start: std::ptr::null_mut::<uint8_t>(),
        p: std::ptr::null_mut::<uint8_t>(),
        p_end: std::ptr::null_mut::<uint8_t>(),
        cur_bits: 0,
        i_left: 0,
        i_bits_encoded: 0,
    };
    let mut tmp_buf: [uint8_t; 100] = [0; 100];
    (*(tmp_buf.as_mut_ptr() as *mut x264_union32_t)).i = 0 as libc::c_int as uint32_t;
    bs_init(
        &mut q,
        tmp_buf.as_mut_ptr() as *mut libc::c_void,
        100 as libc::c_int,
    );
    bs_realign(&mut q);
    bs_write_ue_big(&mut q, 0 as libc::c_int as libc::c_uint);
    bs_write1(&mut q, 0 as libc::c_int as uint32_t);
    bs_write(
        &mut q,
        7 as libc::c_int,
        (*h).param.i_frame_packing as uint32_t,
    );
    bs_write1(&mut q, quincunx_sampling_flag as uint32_t);
    bs_write(
        &mut q,
        6 as libc::c_int,
        ((*h).param.i_frame_packing != 6 as libc::c_int) as libc::c_int as uint32_t,
    );
    bs_write1(&mut q, 0 as libc::c_int as uint32_t);
    bs_write1(&mut q, 0 as libc::c_int as uint32_t);
    bs_write1(&mut q, 0 as libc::c_int as uint32_t);
    bs_write1(
        &mut q,
        ((*h).param.i_frame_packing == 5 as libc::c_int
            && (*(*h).fenc).i_frame & 1 as libc::c_int == 0) as libc::c_int as uint32_t,
    );
    bs_write1(&mut q, 0 as libc::c_int as uint32_t);
    bs_write1(&mut q, 0 as libc::c_int as uint32_t);
    if quincunx_sampling_flag == 0 as libc::c_int && (*h).param.i_frame_packing != 5 as libc::c_int
    {
        bs_write(&mut q, 4 as libc::c_int, 0 as libc::c_int as uint32_t);
        bs_write(&mut q, 4 as libc::c_int, 0 as libc::c_int as uint32_t);
        bs_write(&mut q, 4 as libc::c_int, 0 as libc::c_int as uint32_t);
        bs_write(&mut q, 4 as libc::c_int, 0 as libc::c_int as uint32_t);
    }
    bs_write(&mut q, 8 as libc::c_int, 0 as libc::c_int as uint32_t);
    bs_write_ue_big(
        &mut q,
        ((*h).param.i_frame_packing != 5 as libc::c_int) as libc::c_int as libc::c_uint,
    );
    bs_write1(&mut q, 0 as libc::c_int as uint32_t);
    bs_align_10(&mut q);
    x264_8_sei_write(
        s,
        tmp_buf.as_mut_ptr(),
        bs_pos(&mut q) / 8 as libc::c_int,
        SEI_FRAME_PACKING as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_sei_mastering_display_write(mut h: *mut x264_t, mut s: *mut bs_t) {
    let mut q: bs_t = bs_s {
        p_start: std::ptr::null_mut::<uint8_t>(),
        p: std::ptr::null_mut::<uint8_t>(),
        p_end: std::ptr::null_mut::<uint8_t>(),
        cur_bits: 0,
        i_left: 0,
        i_bits_encoded: 0,
    };
    let mut tmp_buf: [uint8_t; 100] = [0; 100];
    (*(tmp_buf.as_mut_ptr() as *mut x264_union32_t)).i = 0 as libc::c_int as uint32_t;
    bs_init(
        &mut q,
        tmp_buf.as_mut_ptr() as *mut libc::c_void,
        100 as libc::c_int,
    );
    bs_realign(&mut q);
    bs_write(
        &mut q,
        16 as libc::c_int,
        (*h).param.mastering_display.i_green_x as uint32_t,
    );
    bs_write(
        &mut q,
        16 as libc::c_int,
        (*h).param.mastering_display.i_green_y as uint32_t,
    );
    bs_write(
        &mut q,
        16 as libc::c_int,
        (*h).param.mastering_display.i_blue_x as uint32_t,
    );
    bs_write(
        &mut q,
        16 as libc::c_int,
        (*h).param.mastering_display.i_blue_y as uint32_t,
    );
    bs_write(
        &mut q,
        16 as libc::c_int,
        (*h).param.mastering_display.i_red_x as uint32_t,
    );
    bs_write(
        &mut q,
        16 as libc::c_int,
        (*h).param.mastering_display.i_red_y as uint32_t,
    );
    bs_write(
        &mut q,
        16 as libc::c_int,
        (*h).param.mastering_display.i_white_x as uint32_t,
    );
    bs_write(
        &mut q,
        16 as libc::c_int,
        (*h).param.mastering_display.i_white_y as uint32_t,
    );
    bs_write32(
        &mut q,
        (*h).param.mastering_display.i_display_max as uint32_t,
    );
    bs_write32(
        &mut q,
        (*h).param.mastering_display.i_display_min as uint32_t,
    );
    bs_align_10(&mut q);
    x264_8_sei_write(
        s,
        tmp_buf.as_mut_ptr(),
        bs_pos(&mut q) / 8 as libc::c_int,
        SEI_MASTERING_DISPLAY as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_sei_content_light_level_write(
    mut h: *mut x264_t,
    mut s: *mut bs_t,
) {
    let mut q: bs_t = bs_s {
        p_start: std::ptr::null_mut::<uint8_t>(),
        p: std::ptr::null_mut::<uint8_t>(),
        p_end: std::ptr::null_mut::<uint8_t>(),
        cur_bits: 0,
        i_left: 0,
        i_bits_encoded: 0,
    };
    let mut tmp_buf: [uint8_t; 100] = [0; 100];
    (*(tmp_buf.as_mut_ptr() as *mut x264_union32_t)).i = 0 as libc::c_int as uint32_t;
    bs_init(
        &mut q,
        tmp_buf.as_mut_ptr() as *mut libc::c_void,
        100 as libc::c_int,
    );
    bs_realign(&mut q);
    bs_write(
        &mut q,
        16 as libc::c_int,
        (*h).param.content_light_level.i_max_cll as uint32_t,
    );
    bs_write(
        &mut q,
        16 as libc::c_int,
        (*h).param.content_light_level.i_max_fall as uint32_t,
    );
    bs_align_10(&mut q);
    x264_8_sei_write(
        s,
        tmp_buf.as_mut_ptr(),
        bs_pos(&mut q) / 8 as libc::c_int,
        SEI_CONTENT_LIGHT_LEVEL as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_sei_alternative_transfer_write(
    mut h: *mut x264_t,
    mut s: *mut bs_t,
) {
    let mut q: bs_t = bs_s {
        p_start: std::ptr::null_mut::<uint8_t>(),
        p: std::ptr::null_mut::<uint8_t>(),
        p_end: std::ptr::null_mut::<uint8_t>(),
        cur_bits: 0,
        i_left: 0,
        i_bits_encoded: 0,
    };
    let mut tmp_buf: [uint8_t; 100] = [0; 100];
    (*(tmp_buf.as_mut_ptr() as *mut x264_union32_t)).i = 0 as libc::c_int as uint32_t;
    bs_init(
        &mut q,
        tmp_buf.as_mut_ptr() as *mut libc::c_void,
        100 as libc::c_int,
    );
    bs_realign(&mut q);
    bs_write(
        &mut q,
        8 as libc::c_int,
        (*h).param.i_alternative_transfer as uint32_t,
    );
    bs_align_10(&mut q);
    x264_8_sei_write(
        s,
        tmp_buf.as_mut_ptr(),
        bs_pos(&mut q) / 8 as libc::c_int,
        SEI_ALTERNATIVE_TRANSFER as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_filler_write(
    mut h: *mut x264_t,
    mut s: *mut bs_t,
    mut filler: libc::c_int,
) {
    bs_realign(s);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < filler {
        bs_write(s, 8 as libc::c_int, 0xff as libc::c_int as uint32_t);
        i += 1;
        i;
    }
    bs_rbsp_trailing(s);
    bs_flush(s);
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_sei_dec_ref_pic_marking_write(
    mut h: *mut x264_t,
    mut s: *mut bs_t,
) {
    let mut sh: *mut x264_slice_header_t = &mut (*h).sh_backup;
    let mut q: bs_t = bs_s {
        p_start: std::ptr::null_mut::<uint8_t>(),
        p: std::ptr::null_mut::<uint8_t>(),
        p_end: std::ptr::null_mut::<uint8_t>(),
        cur_bits: 0,
        i_left: 0,
        i_bits_encoded: 0,
    };
    let mut tmp_buf: [uint8_t; 100] = [0; 100];
    (*(tmp_buf.as_mut_ptr() as *mut x264_union32_t)).i = 0 as libc::c_int as uint32_t;
    bs_init(
        &mut q,
        tmp_buf.as_mut_ptr() as *mut libc::c_void,
        100 as libc::c_int,
    );
    bs_realign(&mut q);
    bs_write1(&mut q, 0 as libc::c_int as uint32_t);
    bs_write_ue_big(&mut q, (*sh).i_frame_num as libc::c_uint);
    if (*((*h).sps).as_mut_ptr()).b_frame_mbs_only == 0 {
        bs_write1(&mut q, 0 as libc::c_int as uint32_t);
    }
    bs_write1(
        &mut q,
        ((*sh).i_mmco_command_count > 0 as libc::c_int) as libc::c_int as uint32_t,
    );
    if (*sh).i_mmco_command_count > 0 as libc::c_int {
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < (*sh).i_mmco_command_count {
            bs_write_ue_big(&mut q, 1 as libc::c_int as libc::c_uint);
            bs_write_ue_big(
                &mut q,
                ((*sh).mmco[i as usize].i_difference_of_pic_nums - 1 as libc::c_int)
                    as libc::c_uint,
            );
            i += 1;
            i;
        }
        bs_write_ue_big(&mut q, 0 as libc::c_int as libc::c_uint);
    }
    bs_align_10(&mut q);
    x264_8_sei_write(
        s,
        tmp_buf.as_mut_ptr(),
        bs_pos(&mut q) / 8 as libc::c_int,
        SEI_DEC_REF_PIC_MARKING as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_sei_avcintra_umid_write(
    mut h: *mut x264_t,
    mut s: *mut bs_t,
) -> libc::c_int {
    let mut data: [uint8_t; 512] = [0; 512];
    let mut msg: *const libc::c_char = b"UMID\0" as *const u8 as *const libc::c_char;
    let len: libc::c_int = 497 as libc::c_int;
    memset(
        data.as_mut_ptr() as *mut libc::c_void,
        0xff as libc::c_int,
        len as libc::c_ulong,
    );
    memcpy(
        data.as_mut_ptr() as *mut libc::c_void,
        avcintra_uuid.as_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[uint8_t; 16]>() as libc::c_ulong,
    );
    memcpy(
        data.as_mut_ptr().offset(16 as libc::c_int as isize) as *mut libc::c_void,
        msg as *const libc::c_void,
        strlen(msg),
    );
    data[20 as libc::c_int as usize] = 0x13 as libc::c_int as uint8_t;
    data[26 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
    data[25 as libc::c_int as usize] = data[26 as libc::c_int as usize];
    data[23 as libc::c_int as usize] = data[25 as libc::c_int as usize];
    data[22 as libc::c_int as usize] = data[23 as libc::c_int as usize];
    data[28 as libc::c_int as usize] = 0x14 as libc::c_int as uint8_t;
    data[34 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
    data[33 as libc::c_int as usize] = data[34 as libc::c_int as usize];
    data[31 as libc::c_int as usize] = data[33 as libc::c_int as usize];
    data[30 as libc::c_int as usize] = data[31 as libc::c_int as usize];
    data[36 as libc::c_int as usize] = 0x60 as libc::c_int as uint8_t;
    data[41 as libc::c_int as usize] = 0x22 as libc::c_int as uint8_t;
    data[60 as libc::c_int as usize] = 0x62 as libc::c_int as uint8_t;
    data[66 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
    data[65 as libc::c_int as usize] = data[66 as libc::c_int as usize];
    data[63 as libc::c_int as usize] = data[65 as libc::c_int as usize];
    data[62 as libc::c_int as usize] = data[63 as libc::c_int as usize];
    data[68 as libc::c_int as usize] = 0x63 as libc::c_int as uint8_t;
    data[74 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
    data[73 as libc::c_int as usize] = data[74 as libc::c_int as usize];
    data[71 as libc::c_int as usize] = data[73 as libc::c_int as usize];
    data[70 as libc::c_int as usize] = data[71 as libc::c_int as usize];
    x264_8_sei_write(
        &mut (*h).out.bs,
        data.as_mut_ptr(),
        len,
        SEI_USER_DATA_UNREGISTERED as libc::c_int,
    );
    0 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_sei_avcintra_vanc_write(
    mut h: *mut x264_t,
    mut s: *mut bs_t,
    mut len: libc::c_int,
) -> libc::c_int {
    let mut data: [uint8_t; 6000] = [0; 6000];
    let mut msg: *const libc::c_char = b"VANC\0" as *const u8 as *const libc::c_char;
    if len < 0 as libc::c_int
        || len as libc::c_uint as libc::c_ulong
            > ::core::mem::size_of::<[uint8_t; 6000]>() as libc::c_ulong
    {
        x264_8_log(
            h,
            0 as libc::c_int,
            b"AVC-Intra SEI is too large (%d)\n\0" as *const u8 as *const libc::c_char,
            len,
        );
        return -(1 as libc::c_int);
    }
    memset(
        data.as_mut_ptr() as *mut libc::c_void,
        0xff as libc::c_int,
        len as libc::c_ulong,
    );
    memcpy(
        data.as_mut_ptr() as *mut libc::c_void,
        avcintra_uuid.as_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[uint8_t; 16]>() as libc::c_ulong,
    );
    memcpy(
        data.as_mut_ptr().offset(16 as libc::c_int as isize) as *mut libc::c_void,
        msg as *const libc::c_void,
        strlen(msg),
    );
    x264_8_sei_write(
        &mut (*h).out.bs,
        data.as_mut_ptr(),
        len,
        SEI_USER_DATA_UNREGISTERED as libc::c_int,
    );
    0 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_validate_levels(
    mut h: *mut x264_t,
    mut verbose: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut mbs: libc::c_int =
        (*((*h).sps).as_mut_ptr()).i_mb_width * (*((*h).sps).as_mut_ptr()).i_mb_height;
    let mut dpb: libc::c_int = mbs * (*((*h).sps).as_mut_ptr()).vui.i_max_dec_frame_buffering;
    let mut cbp_factor: libc::c_int =
        if (*((*h).sps).as_mut_ptr()).i_profile_idc >= PROFILE_HIGH422 as libc::c_int {
            16 as libc::c_int
        } else if (*((*h).sps).as_mut_ptr()).i_profile_idc == PROFILE_HIGH10 as libc::c_int {
            12 as libc::c_int
        } else if (*((*h).sps).as_mut_ptr()).i_profile_idc == PROFILE_HIGH as libc::c_int {
            5 as libc::c_int
        } else {
            4 as libc::c_int
        };
    let mut l: *const x264_level_t = x264_levels.as_ptr();
    while (*l).level_idc as libc::c_int != 0 as libc::c_int
        && (*l).level_idc as libc::c_int != (*h).param.i_level_idc
    {
        l = l.offset(1);
        l;
    }
    if (*l).frame_size < mbs
        || ((*l).frame_size * 8 as libc::c_int)
            < (*((*h).sps).as_mut_ptr()).i_mb_width * (*((*h).sps).as_mut_ptr()).i_mb_width
        || ((*l).frame_size * 8 as libc::c_int)
            < (*((*h).sps).as_mut_ptr()).i_mb_height * (*((*h).sps).as_mut_ptr()).i_mb_height
    {
        if verbose != 0 {
            x264_8_log(
                h,
                1 as libc::c_int,
                b"frame MB size (%dx%d) > level limit (%d)\n\0" as *const u8 as *const libc::c_char,
                (*((*h).sps).as_mut_ptr()).i_mb_width,
                (*((*h).sps).as_mut_ptr()).i_mb_height,
                (*l).frame_size,
            );
        }
        ret = 1 as libc::c_int;
    }
    if dpb > (*l).dpb {
        if verbose != 0 {
            x264_8_log(
                h,
                1 as libc::c_int,
                b"DPB size (%d frames, %d mbs) > level limit (%d frames, %d mbs)\n\0" as *const u8
                    as *const libc::c_char,
                (*((*h).sps).as_mut_ptr()).vui.i_max_dec_frame_buffering,
                dpb,
                (*l).dpb / mbs,
                (*l).dpb,
            );
        }
        ret = 1 as libc::c_int;
    }
    if (*h).param.rc.i_vbv_max_bitrate > (*l).bitrate * cbp_factor / 4 as libc::c_int {
        if verbose != 0 {
            x264_8_log(
                h,
                1 as libc::c_int,
                b"VBV bitrate (%ld) > level limit (%d)\n\0" as *const u8 as *const libc::c_char,
                (*h).param.rc.i_vbv_max_bitrate as int64_t,
                (*l).bitrate * cbp_factor / 4 as libc::c_int,
            );
        }
        ret = 1 as libc::c_int;
    }
    if (*h).param.rc.i_vbv_buffer_size > (*l).cpb * cbp_factor / 4 as libc::c_int {
        if verbose != 0 {
            x264_8_log(
                h,
                1 as libc::c_int,
                b"VBV buffer (%ld) > level limit (%d)\n\0" as *const u8 as *const libc::c_char,
                (*h).param.rc.i_vbv_buffer_size as int64_t,
                (*l).cpb * cbp_factor / 4 as libc::c_int,
            );
        }
        ret = 1 as libc::c_int;
    }
    if (*h).param.analyse.i_mv_range > (*l).mv_range as libc::c_int {
        if verbose != 0 {
            x264_8_log(
                h,
                1 as libc::c_int,
                b"MV range (%ld) > level limit (%d)\n\0" as *const u8 as *const libc::c_char,
                (*h).param.analyse.i_mv_range as int64_t,
                (*l).mv_range as libc::c_int,
            );
        }
        ret = 1 as libc::c_int;
    }
    if (*h).param.b_interlaced > ((*l).frame_only == 0) as libc::c_int {
        if verbose != 0 {
            x264_8_log(
                h,
                1 as libc::c_int,
                b"interlaced (%ld) > level limit (%d)\n\0" as *const u8 as *const libc::c_char,
                (*h).param.b_interlaced as int64_t,
                ((*l).frame_only == 0) as libc::c_int,
            );
        }
        ret = 1 as libc::c_int;
    }
    if (*h).param.b_fake_interlaced > ((*l).frame_only == 0) as libc::c_int {
        if verbose != 0 {
            x264_8_log(
                h,
                1 as libc::c_int,
                b"fake interlaced (%ld) > level limit (%d)\n\0" as *const u8 as *const libc::c_char,
                (*h).param.b_fake_interlaced as int64_t,
                ((*l).frame_only == 0) as libc::c_int,
            );
        }
        ret = 1 as libc::c_int;
    }
    if (*h).param.i_fps_den > 0 as libc::c_int as uint32_t
        && mbs as int64_t * (*h).param.i_fps_num as int64_t / (*h).param.i_fps_den as int64_t
            > (*l).mbps as int64_t
    {
        if verbose != 0 {
            x264_8_log(
                h,
                1 as libc::c_int,
                b"MB rate (%ld) > level limit (%d)\n\0" as *const u8 as *const libc::c_char,
                mbs as int64_t * (*h).param.i_fps_num as int64_t / (*h).param.i_fps_den as int64_t,
                (*l).mbps,
            );
        }
        ret = 1 as libc::c_int;
    }
    ret
}
