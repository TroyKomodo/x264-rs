#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
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
    fn abs(_: libc::c_int) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
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
pub type clSetKernelArg_func = Option::<
    unsafe extern "C" fn(cl_kernel, cl_uint, size_t, *const libc::c_void) -> cl_int,
>;
pub type cl_uint = uint32_t;
pub type clReleaseProgram_func = Option::<unsafe extern "C" fn(cl_program) -> cl_int>;
pub type clReleaseMemObject_func = Option::<unsafe extern "C" fn(cl_mem) -> cl_int>;
pub type clReleaseKernel_func = Option::<unsafe extern "C" fn(cl_kernel) -> cl_int>;
pub type clReleaseContext_func = Option::<unsafe extern "C" fn(cl_context) -> cl_int>;
pub type clReleaseCommandQueue_func = Option::<
    unsafe extern "C" fn(cl_command_queue) -> cl_int,
>;
pub type clGetSupportedImageFormats_func = Option::<
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
pub type clGetProgramInfo_func = Option::<
    unsafe extern "C" fn(
        cl_program,
        cl_program_info,
        size_t,
        *mut libc::c_void,
        *mut size_t,
    ) -> cl_int,
>;
pub type cl_program_info = cl_uint;
pub type clGetProgramBuildInfo_func = Option::<
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
pub type clGetPlatformIDs_func = Option::<
    unsafe extern "C" fn(cl_uint, *mut cl_platform_id, *mut cl_uint) -> cl_int,
>;
pub type cl_platform_id = *mut _cl_platform_id;
pub type clGetKernelWorkGroupInfo_func = Option::<
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
pub type clGetDeviceInfo_func = Option::<
    unsafe extern "C" fn(
        cl_device_id,
        cl_device_info,
        size_t,
        *mut libc::c_void,
        *mut size_t,
    ) -> cl_int,
>;
pub type cl_device_info = cl_uint;
pub type clGetDeviceIDs_func = Option::<
    unsafe extern "C" fn(
        cl_platform_id,
        cl_device_type,
        cl_uint,
        *mut cl_device_id,
        *mut cl_uint,
    ) -> cl_int,
>;
pub type cl_device_type = cl_bitfield;
pub type clGetCommandQueueInfo_func = Option::<
    unsafe extern "C" fn(
        cl_command_queue,
        cl_command_queue_info,
        size_t,
        *mut libc::c_void,
        *mut size_t,
    ) -> cl_int,
>;
pub type cl_command_queue_info = cl_uint;
pub type clFinish_func = Option::<unsafe extern "C" fn(cl_command_queue) -> cl_int>;
pub type clEnqueueWriteBuffer_func = Option::<
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
pub type clEnqueueReadBuffer_func = Option::<
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
pub type clEnqueueNDRangeKernel_func = Option::<
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
pub type clEnqueueMapBuffer_func = Option::<
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
pub type clEnqueueCopyBuffer_func = Option::<
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
pub type clCreateProgramWithSource_func = Option::<
    unsafe extern "C" fn(
        cl_context,
        cl_uint,
        *mut *const libc::c_char,
        *const size_t,
        *mut cl_int,
    ) -> cl_program,
>;
pub type clCreateProgramWithBinary_func = Option::<
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
pub type clCreateKernel_func = Option::<
    unsafe extern "C" fn(cl_program, *const libc::c_char, *mut cl_int) -> cl_kernel,
>;
pub type clCreateImage2D_func = Option::<
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
pub type clCreateContext_func = Option::<
    unsafe extern "C" fn(
        *const cl_context_properties,
        cl_uint,
        *const cl_device_id,
        Option::<
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
pub type clCreateCommandQueue_func = Option::<
    unsafe extern "C" fn(
        cl_context,
        cl_device_id,
        cl_command_queue_properties,
        *mut cl_int,
    ) -> cl_command_queue,
>;
pub type cl_command_queue_properties = cl_bitfield;
pub type clCreateBuffer_func = Option::<
    unsafe extern "C" fn(
        cl_context,
        cl_mem_flags,
        size_t,
        *mut libc::c_void,
        *mut cl_int,
    ) -> cl_mem,
>;
pub type clBuildProgram_func = Option::<
    unsafe extern "C" fn(
        cl_program,
        cl_uint,
        *const cl_device_id,
        *const libc::c_char,
        Option::<unsafe extern "C" fn(cl_program, *mut libc::c_void) -> ()>,
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
    pub mb_info_free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
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
pub type weight_fn_t = Option::<
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
    pub param_free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub nalu_process: Option::<
        unsafe extern "C" fn(*mut x264_t, *mut x264_nal_t, *mut libc::c_void) -> (),
    >,
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
    pub nal_escape: Option::<
        unsafe extern "C" fn(*mut uint8_t, *mut uint8_t, *mut uint8_t) -> *mut uint8_t,
    >,
    pub cabac_block_residual_internal: Option::<
        unsafe extern "C" fn(
            *mut dctcoef,
            libc::c_int,
            intptr_t,
            *mut x264_cabac_t,
        ) -> (),
    >,
    pub cabac_block_residual_rd_internal: Option::<
        unsafe extern "C" fn(
            *mut dctcoef,
            libc::c_int,
            intptr_t,
            *mut x264_cabac_t,
        ) -> (),
    >,
    pub cabac_block_residual_8x8_rd_internal: Option::<
        unsafe extern "C" fn(
            *mut dctcoef,
            libc::c_int,
            intptr_t,
            *mut x264_cabac_t,
        ) -> (),
    >,
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
    pub deblock_strength: Option::<
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
pub type x264_deblock_intra_t = Option::<
    unsafe extern "C" fn(*mut pixel, intptr_t, libc::c_int, libc::c_int) -> (),
>;
pub type x264_deblock_inter_t = Option::<
    unsafe extern "C" fn(
        *mut pixel,
        intptr_t,
        libc::c_int,
        libc::c_int,
        *mut int8_t,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_quant_function_t {
    pub quant_8x8: Option::<
        unsafe extern "C" fn(*mut dctcoef, *mut udctcoef, *mut udctcoef) -> libc::c_int,
    >,
    pub quant_4x4: Option::<
        unsafe extern "C" fn(*mut dctcoef, *mut udctcoef, *mut udctcoef) -> libc::c_int,
    >,
    pub quant_4x4x4: Option::<
        unsafe extern "C" fn(
            *mut [dctcoef; 16],
            *mut udctcoef,
            *mut udctcoef,
        ) -> libc::c_int,
    >,
    pub quant_4x4_dc: Option::<
        unsafe extern "C" fn(*mut dctcoef, libc::c_int, libc::c_int) -> libc::c_int,
    >,
    pub quant_2x2_dc: Option::<
        unsafe extern "C" fn(*mut dctcoef, libc::c_int, libc::c_int) -> libc::c_int,
    >,
    pub dequant_8x8: Option::<
        unsafe extern "C" fn(*mut dctcoef, *mut [libc::c_int; 64], libc::c_int) -> (),
    >,
    pub dequant_4x4: Option::<
        unsafe extern "C" fn(*mut dctcoef, *mut [libc::c_int; 16], libc::c_int) -> (),
    >,
    pub dequant_4x4_dc: Option::<
        unsafe extern "C" fn(*mut dctcoef, *mut [libc::c_int; 16], libc::c_int) -> (),
    >,
    pub idct_dequant_2x4_dc: Option::<
        unsafe extern "C" fn(
            *mut dctcoef,
            *mut [dctcoef; 16],
            *mut [libc::c_int; 16],
            libc::c_int,
        ) -> (),
    >,
    pub idct_dequant_2x4_dconly: Option::<
        unsafe extern "C" fn(*mut dctcoef, *mut [libc::c_int; 16], libc::c_int) -> (),
    >,
    pub optimize_chroma_2x2_dc: Option::<
        unsafe extern "C" fn(*mut dctcoef, libc::c_int) -> libc::c_int,
    >,
    pub optimize_chroma_2x4_dc: Option::<
        unsafe extern "C" fn(*mut dctcoef, libc::c_int) -> libc::c_int,
    >,
    pub denoise_dct: Option::<
        unsafe extern "C" fn(
            *mut dctcoef,
            *mut uint32_t,
            *mut udctcoef,
            libc::c_int,
        ) -> (),
    >,
    pub decimate_score15: Option::<unsafe extern "C" fn(*mut dctcoef) -> libc::c_int>,
    pub decimate_score16: Option::<unsafe extern "C" fn(*mut dctcoef) -> libc::c_int>,
    pub decimate_score64: Option::<unsafe extern "C" fn(*mut dctcoef) -> libc::c_int>,
    pub coeff_last: [Option::<unsafe extern "C" fn(*mut dctcoef) -> libc::c_int>; 14],
    pub coeff_last4: Option::<unsafe extern "C" fn(*mut dctcoef) -> libc::c_int>,
    pub coeff_last8: Option::<unsafe extern "C" fn(*mut dctcoef) -> libc::c_int>,
    pub coeff_level_run: [Option::<
        unsafe extern "C" fn(*mut dctcoef, *mut x264_run_level_t) -> libc::c_int,
    >; 13],
    pub coeff_level_run4: Option::<
        unsafe extern "C" fn(*mut dctcoef, *mut x264_run_level_t) -> libc::c_int,
    >,
    pub coeff_level_run8: Option::<
        unsafe extern "C" fn(*mut dctcoef, *mut x264_run_level_t) -> libc::c_int,
    >,
    pub trellis_cabac_4x4: Option::<
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
    pub trellis_cabac_8x8: Option::<
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
    pub trellis_cabac_4x4_psy: Option::<
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
    pub trellis_cabac_8x8_psy: Option::<
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
    pub trellis_cabac_dc: Option::<
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
    pub trellis_cabac_chroma_422_dc: Option::<
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
    pub scan_8x8: Option::<unsafe extern "C" fn(*mut dctcoef, *mut dctcoef) -> ()>,
    pub scan_4x4: Option::<unsafe extern "C" fn(*mut dctcoef, *mut dctcoef) -> ()>,
    pub sub_8x8: Option::<
        unsafe extern "C" fn(*mut dctcoef, *const pixel, *mut pixel) -> libc::c_int,
    >,
    pub sub_4x4: Option::<
        unsafe extern "C" fn(*mut dctcoef, *const pixel, *mut pixel) -> libc::c_int,
    >,
    pub sub_4x4ac: Option::<
        unsafe extern "C" fn(
            *mut dctcoef,
            *const pixel,
            *mut pixel,
            *mut dctcoef,
        ) -> libc::c_int,
    >,
    pub interleave_8x8_cavlc: Option::<
        unsafe extern "C" fn(*mut dctcoef, *mut dctcoef, *mut uint8_t) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_dct_function_t {
    pub sub4x4_dct: Option::<
        unsafe extern "C" fn(*mut dctcoef, *mut pixel, *mut pixel) -> (),
    >,
    pub add4x4_idct: Option::<unsafe extern "C" fn(*mut pixel, *mut dctcoef) -> ()>,
    pub sub8x8_dct: Option::<
        unsafe extern "C" fn(*mut [dctcoef; 16], *mut pixel, *mut pixel) -> (),
    >,
    pub sub8x8_dct_dc: Option::<
        unsafe extern "C" fn(*mut dctcoef, *mut pixel, *mut pixel) -> (),
    >,
    pub add8x8_idct: Option::<
        unsafe extern "C" fn(*mut pixel, *mut [dctcoef; 16]) -> (),
    >,
    pub add8x8_idct_dc: Option::<unsafe extern "C" fn(*mut pixel, *mut dctcoef) -> ()>,
    pub sub8x16_dct_dc: Option::<
        unsafe extern "C" fn(*mut dctcoef, *mut pixel, *mut pixel) -> (),
    >,
    pub sub16x16_dct: Option::<
        unsafe extern "C" fn(*mut [dctcoef; 16], *mut pixel, *mut pixel) -> (),
    >,
    pub add16x16_idct: Option::<
        unsafe extern "C" fn(*mut pixel, *mut [dctcoef; 16]) -> (),
    >,
    pub add16x16_idct_dc: Option::<unsafe extern "C" fn(*mut pixel, *mut dctcoef) -> ()>,
    pub sub8x8_dct8: Option::<
        unsafe extern "C" fn(*mut dctcoef, *mut pixel, *mut pixel) -> (),
    >,
    pub add8x8_idct8: Option::<unsafe extern "C" fn(*mut pixel, *mut dctcoef) -> ()>,
    pub sub16x16_dct8: Option::<
        unsafe extern "C" fn(*mut [dctcoef; 64], *mut pixel, *mut pixel) -> (),
    >,
    pub add16x16_idct8: Option::<
        unsafe extern "C" fn(*mut pixel, *mut [dctcoef; 64]) -> (),
    >,
    pub dct4x4dc: Option::<unsafe extern "C" fn(*mut dctcoef) -> ()>,
    pub idct4x4dc: Option::<unsafe extern "C" fn(*mut dctcoef) -> ()>,
    pub dct2x4dc: Option::<unsafe extern "C" fn(*mut dctcoef, *mut [dctcoef; 16]) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_mc_functions_t {
    pub mc_luma: Option::<
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
    pub get_ref: Option::<
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
    pub mc_chroma: Option::<
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
    pub avg: [Option::<
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
    pub copy: [Option::<
        unsafe extern "C" fn(
            *mut pixel,
            intptr_t,
            *mut pixel,
            intptr_t,
            libc::c_int,
        ) -> (),
    >; 7],
    pub copy_16x16_unaligned: Option::<
        unsafe extern "C" fn(
            *mut pixel,
            intptr_t,
            *mut pixel,
            intptr_t,
            libc::c_int,
        ) -> (),
    >,
    pub store_interleave_chroma: Option::<
        unsafe extern "C" fn(
            *mut pixel,
            intptr_t,
            *mut pixel,
            *mut pixel,
            libc::c_int,
        ) -> (),
    >,
    pub load_deinterleave_chroma_fenc: Option::<
        unsafe extern "C" fn(*mut pixel, *mut pixel, intptr_t, libc::c_int) -> (),
    >,
    pub load_deinterleave_chroma_fdec: Option::<
        unsafe extern "C" fn(*mut pixel, *mut pixel, intptr_t, libc::c_int) -> (),
    >,
    pub plane_copy: Option::<
        unsafe extern "C" fn(
            *mut pixel,
            intptr_t,
            *mut pixel,
            intptr_t,
            libc::c_int,
            libc::c_int,
        ) -> (),
    >,
    pub plane_copy_swap: Option::<
        unsafe extern "C" fn(
            *mut pixel,
            intptr_t,
            *mut pixel,
            intptr_t,
            libc::c_int,
            libc::c_int,
        ) -> (),
    >,
    pub plane_copy_interleave: Option::<
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
    pub plane_copy_deinterleave: Option::<
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
    pub plane_copy_deinterleave_yuyv: Option::<
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
    pub plane_copy_deinterleave_rgb: Option::<
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
    pub plane_copy_deinterleave_v210: Option::<
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
    pub hpel_filter: Option::<
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
    pub prefetch_fenc: Option::<
        unsafe extern "C" fn(
            *mut pixel,
            intptr_t,
            *mut pixel,
            intptr_t,
            libc::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_400: Option::<
        unsafe extern "C" fn(
            *mut pixel,
            intptr_t,
            *mut pixel,
            intptr_t,
            libc::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_420: Option::<
        unsafe extern "C" fn(
            *mut pixel,
            intptr_t,
            *mut pixel,
            intptr_t,
            libc::c_int,
        ) -> (),
    >,
    pub prefetch_fenc_422: Option::<
        unsafe extern "C" fn(
            *mut pixel,
            intptr_t,
            *mut pixel,
            intptr_t,
            libc::c_int,
        ) -> (),
    >,
    pub prefetch_ref: Option::<
        unsafe extern "C" fn(*mut pixel, intptr_t, libc::c_int) -> (),
    >,
    pub memcpy_aligned: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_void,
            size_t,
        ) -> *mut libc::c_void,
    >,
    pub memzero_aligned: Option::<unsafe extern "C" fn(*mut libc::c_void, size_t) -> ()>,
    pub integral_init4h: Option::<
        unsafe extern "C" fn(*mut uint16_t, *mut pixel, intptr_t) -> (),
    >,
    pub integral_init8h: Option::<
        unsafe extern "C" fn(*mut uint16_t, *mut pixel, intptr_t) -> (),
    >,
    pub integral_init4v: Option::<
        unsafe extern "C" fn(*mut uint16_t, *mut uint16_t, intptr_t) -> (),
    >,
    pub integral_init8v: Option::<unsafe extern "C" fn(*mut uint16_t, intptr_t) -> ()>,
    pub frame_init_lowres_core: Option::<
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
    pub weight_cache: Option::<
        unsafe extern "C" fn(*mut x264_t, *mut x264_weight_t) -> (),
    >,
    pub mbtree_propagate_cost: Option::<
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
    pub mbtree_propagate_list: Option::<
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
    pub mbtree_fix8_pack: Option::<
        unsafe extern "C" fn(*mut uint16_t, *mut libc::c_float, libc::c_int) -> (),
    >,
    pub mbtree_fix8_unpack: Option::<
        unsafe extern "C" fn(*mut libc::c_float, *mut uint16_t, libc::c_int) -> (),
    >,
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
    pub vsad: Option::<
        unsafe extern "C" fn(*mut pixel, intptr_t, libc::c_int) -> libc::c_int,
    >,
    pub asd8: Option::<
        unsafe extern "C" fn(
            *mut pixel,
            intptr_t,
            *mut pixel,
            intptr_t,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub sa8d_satd: [Option::<
        unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> uint64_t,
    >; 1],
    pub var: [Option::<unsafe extern "C" fn(*mut pixel, intptr_t) -> uint64_t>; 4],
    pub var2: [Option::<
        unsafe extern "C" fn(*mut pixel, *mut pixel, *mut libc::c_int) -> libc::c_int,
    >; 4],
    pub hadamard_ac: [Option::<
        unsafe extern "C" fn(*mut pixel, intptr_t) -> uint64_t,
    >; 4],
    pub ssd_nv12_core: Option::<
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
    pub ssim_4x4x2_core: Option::<
        unsafe extern "C" fn(
            *const pixel,
            intptr_t,
            *const pixel,
            intptr_t,
            *mut [libc::c_int; 4],
        ) -> (),
    >,
    pub ssim_end4: Option::<
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
    pub ads: [Option::<
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
    pub intra_mbcmp_x3_16x16: Option::<
        unsafe extern "C" fn(*mut pixel, *mut pixel, *mut libc::c_int) -> (),
    >,
    pub intra_satd_x3_16x16: Option::<
        unsafe extern "C" fn(*mut pixel, *mut pixel, *mut libc::c_int) -> (),
    >,
    pub intra_sad_x3_16x16: Option::<
        unsafe extern "C" fn(*mut pixel, *mut pixel, *mut libc::c_int) -> (),
    >,
    pub intra_mbcmp_x3_4x4: Option::<
        unsafe extern "C" fn(*mut pixel, *mut pixel, *mut libc::c_int) -> (),
    >,
    pub intra_satd_x3_4x4: Option::<
        unsafe extern "C" fn(*mut pixel, *mut pixel, *mut libc::c_int) -> (),
    >,
    pub intra_sad_x3_4x4: Option::<
        unsafe extern "C" fn(*mut pixel, *mut pixel, *mut libc::c_int) -> (),
    >,
    pub intra_mbcmp_x3_chroma: Option::<
        unsafe extern "C" fn(*mut pixel, *mut pixel, *mut libc::c_int) -> (),
    >,
    pub intra_satd_x3_chroma: Option::<
        unsafe extern "C" fn(*mut pixel, *mut pixel, *mut libc::c_int) -> (),
    >,
    pub intra_sad_x3_chroma: Option::<
        unsafe extern "C" fn(*mut pixel, *mut pixel, *mut libc::c_int) -> (),
    >,
    pub intra_mbcmp_x3_8x16c: Option::<
        unsafe extern "C" fn(*mut pixel, *mut pixel, *mut libc::c_int) -> (),
    >,
    pub intra_satd_x3_8x16c: Option::<
        unsafe extern "C" fn(*mut pixel, *mut pixel, *mut libc::c_int) -> (),
    >,
    pub intra_sad_x3_8x16c: Option::<
        unsafe extern "C" fn(*mut pixel, *mut pixel, *mut libc::c_int) -> (),
    >,
    pub intra_mbcmp_x3_8x8c: Option::<
        unsafe extern "C" fn(*mut pixel, *mut pixel, *mut libc::c_int) -> (),
    >,
    pub intra_satd_x3_8x8c: Option::<
        unsafe extern "C" fn(*mut pixel, *mut pixel, *mut libc::c_int) -> (),
    >,
    pub intra_sad_x3_8x8c: Option::<
        unsafe extern "C" fn(*mut pixel, *mut pixel, *mut libc::c_int) -> (),
    >,
    pub intra_mbcmp_x3_8x8: Option::<
        unsafe extern "C" fn(*mut pixel, *mut pixel, *mut libc::c_int) -> (),
    >,
    pub intra_sa8d_x3_8x8: Option::<
        unsafe extern "C" fn(*mut pixel, *mut pixel, *mut libc::c_int) -> (),
    >,
    pub intra_sad_x3_8x8: Option::<
        unsafe extern "C" fn(*mut pixel, *mut pixel, *mut libc::c_int) -> (),
    >,
    pub intra_mbcmp_x9_4x4: Option::<
        unsafe extern "C" fn(*mut pixel, *mut pixel, *mut uint16_t) -> libc::c_int,
    >,
    pub intra_satd_x9_4x4: Option::<
        unsafe extern "C" fn(*mut pixel, *mut pixel, *mut uint16_t) -> libc::c_int,
    >,
    pub intra_sad_x9_4x4: Option::<
        unsafe extern "C" fn(*mut pixel, *mut pixel, *mut uint16_t) -> libc::c_int,
    >,
    pub intra_mbcmp_x9_8x8: Option::<
        unsafe extern "C" fn(
            *mut pixel,
            *mut pixel,
            *mut pixel,
            *mut uint16_t,
            *mut uint16_t,
        ) -> libc::c_int,
    >,
    pub intra_sa8d_x9_8x8: Option::<
        unsafe extern "C" fn(
            *mut pixel,
            *mut pixel,
            *mut pixel,
            *mut uint16_t,
            *mut uint16_t,
        ) -> libc::c_int,
    >,
    pub intra_sad_x9_8x8: Option::<
        unsafe extern "C" fn(
            *mut pixel,
            *mut pixel,
            *mut pixel,
            *mut uint16_t,
            *mut uint16_t,
        ) -> libc::c_int,
    >,
}
pub type x264_pixel_cmp_x4_t = Option::<
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
pub type x264_pixel_cmp_x3_t = Option::<
    unsafe extern "C" fn(
        *mut pixel,
        *mut pixel,
        *mut pixel,
        *mut pixel,
        intptr_t,
        *mut libc::c_int,
    ) -> (),
>;
pub type x264_pixel_cmp_t = Option::<
    unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> libc::c_int,
>;
pub type x264_predict_8x8_filter_t = Option::<
    unsafe extern "C" fn(*mut pixel, *mut pixel, libc::c_int, libc::c_int) -> (),
>;
pub type x264_predict_t = Option::<unsafe extern "C" fn(*mut pixel) -> ()>;
pub type x264_predict8x8_t = Option::<
    unsafe extern "C" fn(*mut pixel, *mut pixel) -> (),
>;
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
pub type C2RustUnnamed_20 = libc::c_uint;
pub const PIXEL_2x2: C2RustUnnamed_20 = 11;
pub const PIXEL_2x4: C2RustUnnamed_20 = 10;
pub const PIXEL_2x8: C2RustUnnamed_20 = 9;
pub const PIXEL_4x2: C2RustUnnamed_20 = 8;
pub const PIXEL_4x16: C2RustUnnamed_20 = 7;
pub const PIXEL_4x4: C2RustUnnamed_20 = 6;
pub const PIXEL_4x8: C2RustUnnamed_20 = 5;
pub const PIXEL_8x4: C2RustUnnamed_20 = 4;
pub const PIXEL_8x8: C2RustUnnamed_20 = 3;
pub const PIXEL_8x16: C2RustUnnamed_20 = 2;
pub const PIXEL_16x8: C2RustUnnamed_20 = 1;
pub const PIXEL_16x16: C2RustUnnamed_20 = 0;
pub type sum2_t = uint32_t;
pub type sum_t = uint16_t;
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
            i_sum
                += abs(
                    *pix1.offset(x as isize) as libc::c_int
                        - *pix2.offset(x as isize) as libc::c_int,
                );
            x += 1;
            x;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
        y;
    }
    return i_sum;
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
            i_sum
                += abs(
                    *pix1.offset(x as isize) as libc::c_int
                        - *pix2.offset(x as isize) as libc::c_int,
                );
            x += 1;
            x;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
        y;
    }
    return i_sum;
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
            i_sum
                += abs(
                    *pix1.offset(x as isize) as libc::c_int
                        - *pix2.offset(x as isize) as libc::c_int,
                );
            x += 1;
            x;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
        y;
    }
    return i_sum;
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
            i_sum
                += abs(
                    *pix1.offset(x as isize) as libc::c_int
                        - *pix2.offset(x as isize) as libc::c_int,
                );
            x += 1;
            x;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
        y;
    }
    return i_sum;
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
            i_sum
                += abs(
                    *pix1.offset(x as isize) as libc::c_int
                        - *pix2.offset(x as isize) as libc::c_int,
                );
            x += 1;
            x;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
        y;
    }
    return i_sum;
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
            i_sum
                += abs(
                    *pix1.offset(x as isize) as libc::c_int
                        - *pix2.offset(x as isize) as libc::c_int,
                );
            x += 1;
            x;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
        y;
    }
    return i_sum;
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
            i_sum
                += abs(
                    *pix1.offset(x as isize) as libc::c_int
                        - *pix2.offset(x as isize) as libc::c_int,
                );
            x += 1;
            x;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
        y;
    }
    return i_sum;
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
            i_sum
                += abs(
                    *pix1.offset(x as isize) as libc::c_int
                        - *pix2.offset(x as isize) as libc::c_int,
                );
            x += 1;
            x;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
        y;
    }
    return i_sum;
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
            let mut d: libc::c_int = *pix1.offset(x as isize) as libc::c_int
                - *pix2.offset(x as isize) as libc::c_int;
            i_sum += d * d;
            x += 1;
            x;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
        y;
    }
    return i_sum;
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
            let mut d: libc::c_int = *pix1.offset(x as isize) as libc::c_int
                - *pix2.offset(x as isize) as libc::c_int;
            i_sum += d * d;
            x += 1;
            x;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
        y;
    }
    return i_sum;
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
            let mut d: libc::c_int = *pix1.offset(x as isize) as libc::c_int
                - *pix2.offset(x as isize) as libc::c_int;
            i_sum += d * d;
            x += 1;
            x;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
        y;
    }
    return i_sum;
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
            let mut d: libc::c_int = *pix1.offset(x as isize) as libc::c_int
                - *pix2.offset(x as isize) as libc::c_int;
            i_sum += d * d;
            x += 1;
            x;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
        y;
    }
    return i_sum;
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
            let mut d: libc::c_int = *pix1.offset(x as isize) as libc::c_int
                - *pix2.offset(x as isize) as libc::c_int;
            i_sum += d * d;
            x += 1;
            x;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
        y;
    }
    return i_sum;
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
            let mut d: libc::c_int = *pix1.offset(x as isize) as libc::c_int
                - *pix2.offset(x as isize) as libc::c_int;
            i_sum += d * d;
            x += 1;
            x;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
        y;
    }
    return i_sum;
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
            let mut d: libc::c_int = *pix1.offset(x as isize) as libc::c_int
                - *pix2.offset(x as isize) as libc::c_int;
            i_sum += d * d;
            x += 1;
            x;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
        y;
    }
    return i_sum;
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
            let mut d: libc::c_int = *pix1.offset(x as isize) as libc::c_int
                - *pix2.offset(x as isize) as libc::c_int;
            i_sum += d * d;
            x += 1;
            x;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
        y;
    }
    return i_sum;
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
        & 15 as libc::c_int as intptr_t == 0) as libc::c_int;
    y = 0 as libc::c_int;
    while y < i_height - 15 as libc::c_int {
        let mut x: libc::c_int = 0 as libc::c_int;
        if align != 0 {
            while x < i_width - 15 as libc::c_int {
                i_ssd = i_ssd
                    .wrapping_add(
                        ((*pf).ssd[PIXEL_16x16 as libc::c_int as usize])
                            .expect(
                                "non-null function pointer",
                            )(
                            pix1
                                .offset((y as intptr_t * i_pix1) as isize)
                                .offset(x as isize),
                            i_pix1,
                            pix2
                                .offset((y as intptr_t * i_pix2) as isize)
                                .offset(x as isize),
                            i_pix2,
                        ) as uint64_t,
                    );
                x += 16 as libc::c_int;
            }
        }
        while x < i_width - 7 as libc::c_int {
            i_ssd = i_ssd
                .wrapping_add(
                    ((*pf).ssd[PIXEL_8x16 as libc::c_int as usize])
                        .expect(
                            "non-null function pointer",
                        )(
                        pix1
                            .offset((y as intptr_t * i_pix1) as isize)
                            .offset(x as isize),
                        i_pix1,
                        pix2
                            .offset((y as intptr_t * i_pix2) as isize)
                            .offset(x as isize),
                        i_pix2,
                    ) as uint64_t,
                );
            x += 8 as libc::c_int;
        }
        y += 16 as libc::c_int;
    }
    if y < i_height - 7 as libc::c_int {
        let mut x_0: libc::c_int = 0 as libc::c_int;
        while x_0 < i_width - 7 as libc::c_int {
            i_ssd = i_ssd
                .wrapping_add(
                    ((*pf).ssd[PIXEL_8x8 as libc::c_int as usize])
                        .expect(
                            "non-null function pointer",
                        )(
                        pix1
                            .offset((y as intptr_t * i_pix1) as isize)
                            .offset(x_0 as isize),
                        i_pix1,
                        pix2
                            .offset((y as intptr_t * i_pix2) as isize)
                            .offset(x_0 as isize),
                        i_pix2,
                    ) as uint64_t,
                );
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
    return i_ssd;
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
                - *pixuv2.offset((2 as libc::c_int * x + 1 as libc::c_int) as isize)
                    as libc::c_int;
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
    ((*pf).ssd_nv12_core)
        .expect(
            "non-null function pointer",
        )(
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
unsafe extern "C" fn pixel_var_16x16(
    mut pix: *mut pixel,
    mut i_stride: intptr_t,
) -> uint64_t {
    let mut sum: uint32_t = 0 as libc::c_int as uint32_t;
    let mut sqr: uint32_t = 0 as libc::c_int as uint32_t;
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < 16 as libc::c_int {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < 16 as libc::c_int {
            sum = sum.wrapping_add(*pix.offset(x as isize) as uint32_t);
            sqr = sqr
                .wrapping_add(
                    (*pix.offset(x as isize) as libc::c_int
                        * *pix.offset(x as isize) as libc::c_int) as uint32_t,
                );
            x += 1;
            x;
        }
        pix = pix.offset(i_stride as isize);
        y += 1;
        y;
    }
    return (sum as uint64_t).wrapping_add((sqr as uint64_t) << 32 as libc::c_int);
}
unsafe extern "C" fn pixel_var_8x16(
    mut pix: *mut pixel,
    mut i_stride: intptr_t,
) -> uint64_t {
    let mut sum: uint32_t = 0 as libc::c_int as uint32_t;
    let mut sqr: uint32_t = 0 as libc::c_int as uint32_t;
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < 16 as libc::c_int {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < 8 as libc::c_int {
            sum = sum.wrapping_add(*pix.offset(x as isize) as uint32_t);
            sqr = sqr
                .wrapping_add(
                    (*pix.offset(x as isize) as libc::c_int
                        * *pix.offset(x as isize) as libc::c_int) as uint32_t,
                );
            x += 1;
            x;
        }
        pix = pix.offset(i_stride as isize);
        y += 1;
        y;
    }
    return (sum as uint64_t).wrapping_add((sqr as uint64_t) << 32 as libc::c_int);
}
unsafe extern "C" fn pixel_var_8x8(
    mut pix: *mut pixel,
    mut i_stride: intptr_t,
) -> uint64_t {
    let mut sum: uint32_t = 0 as libc::c_int as uint32_t;
    let mut sqr: uint32_t = 0 as libc::c_int as uint32_t;
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < 8 as libc::c_int {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < 8 as libc::c_int {
            sum = sum.wrapping_add(*pix.offset(x as isize) as uint32_t);
            sqr = sqr
                .wrapping_add(
                    (*pix.offset(x as isize) as libc::c_int
                        * *pix.offset(x as isize) as libc::c_int) as uint32_t,
                );
            x += 1;
            x;
        }
        pix = pix.offset(i_stride as isize);
        y += 1;
        y;
    }
    return (sum as uint64_t).wrapping_add((sqr as uint64_t) << 32 as libc::c_int);
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
            let mut diff_u: libc::c_int = *fenc.offset(x as isize) as libc::c_int
                - *fdec.offset(x as isize) as libc::c_int;
            let mut diff_v: libc::c_int = *fenc
                .offset((x + 16 as libc::c_int / 2 as libc::c_int) as isize)
                as libc::c_int
                - *fdec.offset((x + 32 as libc::c_int / 2 as libc::c_int) as isize)
                    as libc::c_int;
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
    return (sqr_u as int64_t - (sum_u as int64_t * sum_u as int64_t >> 7 as libc::c_int)
        + sqr_v as int64_t - (sum_v as int64_t * sum_v as int64_t >> 7 as libc::c_int))
        as libc::c_int;
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
            let mut diff_u: libc::c_int = *fenc.offset(x as isize) as libc::c_int
                - *fdec.offset(x as isize) as libc::c_int;
            let mut diff_v: libc::c_int = *fenc
                .offset((x + 16 as libc::c_int / 2 as libc::c_int) as isize)
                as libc::c_int
                - *fdec.offset((x + 32 as libc::c_int / 2 as libc::c_int) as isize)
                    as libc::c_int;
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
    return (sqr_u as int64_t - (sum_u as int64_t * sum_u as int64_t >> 6 as libc::c_int)
        + sqr_v as int64_t - (sum_v as int64_t * sum_v as int64_t >> 6 as libc::c_int))
        as libc::c_int;
}
#[inline(always)]
unsafe extern "C" fn abs2(mut a: sum2_t) -> sum2_t {
    let mut s: sum2_t = (a
        >> (8 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<sum_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        & ((1 as libc::c_int as sum2_t)
            << (8 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<sum_t>() as libc::c_ulong))
            .wrapping_add(1 as libc::c_int as sum2_t))
        * -(1 as libc::c_int) as sum_t as sum2_t;
    return a.wrapping_add(s) ^ s;
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
        b0 = a0
            .wrapping_add(a1)
            .wrapping_add(
                a0.wrapping_sub(a1)
                    << (8 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<sum_t>() as libc::c_ulong),
            );
        a2 = (*pix1.offset(2 as libc::c_int as isize) as libc::c_int
            - *pix2.offset(2 as libc::c_int as isize) as libc::c_int) as sum2_t;
        a3 = (*pix1.offset(3 as libc::c_int as isize) as libc::c_int
            - *pix2.offset(3 as libc::c_int as isize) as libc::c_int) as sum2_t;
        b1 = a2
            .wrapping_add(a3)
            .wrapping_add(
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
        sum = sum
            .wrapping_add(
                (a0 as sum_t as sum2_t)
                    .wrapping_add(
                        a0
                            >> (8 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(
                                    ::core::mem::size_of::<sum_t>() as libc::c_ulong,
                                ),
                    ),
            );
        i_0 += 1;
        i_0;
    }
    return (sum >> 1 as libc::c_int) as libc::c_int;
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
                    - *pix2.offset(4 as libc::c_int as isize) as libc::c_int) as sum2_t)
                    << (8 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<sum_t>() as libc::c_ulong),
            );
        a1 = ((*pix1.offset(1 as libc::c_int as isize) as libc::c_int
            - *pix2.offset(1 as libc::c_int as isize) as libc::c_int) as sum2_t)
            .wrapping_add(
                ((*pix1.offset(5 as libc::c_int as isize) as libc::c_int
                    - *pix2.offset(5 as libc::c_int as isize) as libc::c_int) as sum2_t)
                    << (8 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<sum_t>() as libc::c_ulong),
            );
        a2 = ((*pix1.offset(2 as libc::c_int as isize) as libc::c_int
            - *pix2.offset(2 as libc::c_int as isize) as libc::c_int) as sum2_t)
            .wrapping_add(
                ((*pix1.offset(6 as libc::c_int as isize) as libc::c_int
                    - *pix2.offset(6 as libc::c_int as isize) as libc::c_int) as sum2_t)
                    << (8 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<sum_t>() as libc::c_ulong),
            );
        a3 = ((*pix1.offset(3 as libc::c_int as isize) as libc::c_int
            - *pix2.offset(3 as libc::c_int as isize) as libc::c_int) as sum2_t)
            .wrapping_add(
                ((*pix1.offset(7 as libc::c_int as isize) as libc::c_int
                    - *pix2.offset(7 as libc::c_int as isize) as libc::c_int) as sum2_t)
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
        sum = sum
            .wrapping_add(
                (abs2(a0))
                    .wrapping_add(abs2(a1))
                    .wrapping_add(abs2(a2))
                    .wrapping_add(abs2(a3)),
            );
        i_0 += 1;
        i_0;
    }
    return ((sum as sum_t as sum2_t)
        .wrapping_add(
            sum
                >> (8 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<sum_t>() as libc::c_ulong),
        ) >> 1 as libc::c_int) as libc::c_int;
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
        sum
            += x264_pixel_satd_8x4(
                pix1.offset(8 as libc::c_int as isize),
                i_pix1,
                pix2.offset(8 as libc::c_int as isize),
                i_pix2,
            )
                + x264_pixel_satd_8x4(
                    pix1
                        .offset(8 as libc::c_int as isize)
                        .offset((4 as libc::c_int as intptr_t * i_pix1) as isize),
                    i_pix1,
                    pix2
                        .offset(8 as libc::c_int as isize)
                        .offset((4 as libc::c_int as intptr_t * i_pix2) as isize),
                    i_pix2,
                );
    }
    if 16 as libc::c_int == 16 as libc::c_int {
        sum
            += x264_pixel_satd_8x4(
                pix1.offset((8 as libc::c_int as intptr_t * i_pix1) as isize),
                i_pix1,
                pix2.offset((8 as libc::c_int as intptr_t * i_pix2) as isize),
                i_pix2,
            )
                + x264_pixel_satd_8x4(
                    pix1.offset((12 as libc::c_int as intptr_t * i_pix1) as isize),
                    i_pix1,
                    pix2.offset((12 as libc::c_int as intptr_t * i_pix2) as isize),
                    i_pix2,
                );
    }
    if 16 as libc::c_int == 16 as libc::c_int && 16 as libc::c_int == 16 as libc::c_int {
        sum
            += x264_pixel_satd_8x4(
                pix1
                    .offset(8 as libc::c_int as isize)
                    .offset((8 as libc::c_int as intptr_t * i_pix1) as isize),
                i_pix1,
                pix2
                    .offset(8 as libc::c_int as isize)
                    .offset((8 as libc::c_int as intptr_t * i_pix2) as isize),
                i_pix2,
            )
                + x264_pixel_satd_8x4(
                    pix1
                        .offset(8 as libc::c_int as isize)
                        .offset((12 as libc::c_int as intptr_t * i_pix1) as isize),
                    i_pix1,
                    pix2
                        .offset(8 as libc::c_int as isize)
                        .offset((12 as libc::c_int as intptr_t * i_pix2) as isize),
                    i_pix2,
                );
    }
    return sum;
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
        sum
            += x264_pixel_satd_8x4(
                pix1.offset(8 as libc::c_int as isize),
                i_pix1,
                pix2.offset(8 as libc::c_int as isize),
                i_pix2,
            )
                + x264_pixel_satd_8x4(
                    pix1
                        .offset(8 as libc::c_int as isize)
                        .offset((4 as libc::c_int as intptr_t * i_pix1) as isize),
                    i_pix1,
                    pix2
                        .offset(8 as libc::c_int as isize)
                        .offset((4 as libc::c_int as intptr_t * i_pix2) as isize),
                    i_pix2,
                );
    }
    if 8 as libc::c_int == 16 as libc::c_int {
        sum
            += x264_pixel_satd_8x4(
                pix1.offset((8 as libc::c_int as intptr_t * i_pix1) as isize),
                i_pix1,
                pix2.offset((8 as libc::c_int as intptr_t * i_pix2) as isize),
                i_pix2,
            )
                + x264_pixel_satd_8x4(
                    pix1.offset((12 as libc::c_int as intptr_t * i_pix1) as isize),
                    i_pix1,
                    pix2.offset((12 as libc::c_int as intptr_t * i_pix2) as isize),
                    i_pix2,
                );
    }
    if 16 as libc::c_int == 16 as libc::c_int && 8 as libc::c_int == 16 as libc::c_int {
        sum
            += x264_pixel_satd_8x4(
                pix1
                    .offset(8 as libc::c_int as isize)
                    .offset((8 as libc::c_int as intptr_t * i_pix1) as isize),
                i_pix1,
                pix2
                    .offset(8 as libc::c_int as isize)
                    .offset((8 as libc::c_int as intptr_t * i_pix2) as isize),
                i_pix2,
            )
                + x264_pixel_satd_8x4(
                    pix1
                        .offset(8 as libc::c_int as isize)
                        .offset((12 as libc::c_int as intptr_t * i_pix1) as isize),
                    i_pix1,
                    pix2
                        .offset(8 as libc::c_int as isize)
                        .offset((12 as libc::c_int as intptr_t * i_pix2) as isize),
                    i_pix2,
                );
    }
    return sum;
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
        sum
            += x264_pixel_satd_8x4(
                pix1.offset(8 as libc::c_int as isize),
                i_pix1,
                pix2.offset(8 as libc::c_int as isize),
                i_pix2,
            )
                + x264_pixel_satd_8x4(
                    pix1
                        .offset(8 as libc::c_int as isize)
                        .offset((4 as libc::c_int as intptr_t * i_pix1) as isize),
                    i_pix1,
                    pix2
                        .offset(8 as libc::c_int as isize)
                        .offset((4 as libc::c_int as intptr_t * i_pix2) as isize),
                    i_pix2,
                );
    }
    if 16 as libc::c_int == 16 as libc::c_int {
        sum
            += x264_pixel_satd_8x4(
                pix1.offset((8 as libc::c_int as intptr_t * i_pix1) as isize),
                i_pix1,
                pix2.offset((8 as libc::c_int as intptr_t * i_pix2) as isize),
                i_pix2,
            )
                + x264_pixel_satd_8x4(
                    pix1.offset((12 as libc::c_int as intptr_t * i_pix1) as isize),
                    i_pix1,
                    pix2.offset((12 as libc::c_int as intptr_t * i_pix2) as isize),
                    i_pix2,
                );
    }
    if 8 as libc::c_int == 16 as libc::c_int && 16 as libc::c_int == 16 as libc::c_int {
        sum
            += x264_pixel_satd_8x4(
                pix1
                    .offset(8 as libc::c_int as isize)
                    .offset((8 as libc::c_int as intptr_t * i_pix1) as isize),
                i_pix1,
                pix2
                    .offset(8 as libc::c_int as isize)
                    .offset((8 as libc::c_int as intptr_t * i_pix2) as isize),
                i_pix2,
            )
                + x264_pixel_satd_8x4(
                    pix1
                        .offset(8 as libc::c_int as isize)
                        .offset((12 as libc::c_int as intptr_t * i_pix1) as isize),
                    i_pix1,
                    pix2
                        .offset(8 as libc::c_int as isize)
                        .offset((12 as libc::c_int as intptr_t * i_pix2) as isize),
                    i_pix2,
                );
    }
    return sum;
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
        sum
            += x264_pixel_satd_8x4(
                pix1.offset(8 as libc::c_int as isize),
                i_pix1,
                pix2.offset(8 as libc::c_int as isize),
                i_pix2,
            )
                + x264_pixel_satd_8x4(
                    pix1
                        .offset(8 as libc::c_int as isize)
                        .offset((4 as libc::c_int as intptr_t * i_pix1) as isize),
                    i_pix1,
                    pix2
                        .offset(8 as libc::c_int as isize)
                        .offset((4 as libc::c_int as intptr_t * i_pix2) as isize),
                    i_pix2,
                );
    }
    if 8 as libc::c_int == 16 as libc::c_int {
        sum
            += x264_pixel_satd_8x4(
                pix1.offset((8 as libc::c_int as intptr_t * i_pix1) as isize),
                i_pix1,
                pix2.offset((8 as libc::c_int as intptr_t * i_pix2) as isize),
                i_pix2,
            )
                + x264_pixel_satd_8x4(
                    pix1.offset((12 as libc::c_int as intptr_t * i_pix1) as isize),
                    i_pix1,
                    pix2.offset((12 as libc::c_int as intptr_t * i_pix2) as isize),
                    i_pix2,
                );
    }
    if 8 as libc::c_int == 16 as libc::c_int && 8 as libc::c_int == 16 as libc::c_int {
        sum
            += x264_pixel_satd_8x4(
                pix1
                    .offset(8 as libc::c_int as isize)
                    .offset((8 as libc::c_int as intptr_t * i_pix1) as isize),
                i_pix1,
                pix2
                    .offset(8 as libc::c_int as isize)
                    .offset((8 as libc::c_int as intptr_t * i_pix2) as isize),
                i_pix2,
            )
                + x264_pixel_satd_8x4(
                    pix1
                        .offset(8 as libc::c_int as isize)
                        .offset((12 as libc::c_int as intptr_t * i_pix1) as isize),
                    i_pix1,
                    pix2
                        .offset(8 as libc::c_int as isize)
                        .offset((12 as libc::c_int as intptr_t * i_pix2) as isize),
                    i_pix2,
                );
    }
    return sum;
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
        sum
            += x264_pixel_satd_4x4(
                pix1.offset(8 as libc::c_int as isize),
                i_pix1,
                pix2.offset(8 as libc::c_int as isize),
                i_pix2,
            )
                + x264_pixel_satd_4x4(
                    pix1
                        .offset(8 as libc::c_int as isize)
                        .offset((4 as libc::c_int as intptr_t * i_pix1) as isize),
                    i_pix1,
                    pix2
                        .offset(8 as libc::c_int as isize)
                        .offset((4 as libc::c_int as intptr_t * i_pix2) as isize),
                    i_pix2,
                );
    }
    if 16 as libc::c_int == 16 as libc::c_int {
        sum
            += x264_pixel_satd_4x4(
                pix1.offset((8 as libc::c_int as intptr_t * i_pix1) as isize),
                i_pix1,
                pix2.offset((8 as libc::c_int as intptr_t * i_pix2) as isize),
                i_pix2,
            )
                + x264_pixel_satd_4x4(
                    pix1.offset((12 as libc::c_int as intptr_t * i_pix1) as isize),
                    i_pix1,
                    pix2.offset((12 as libc::c_int as intptr_t * i_pix2) as isize),
                    i_pix2,
                );
    }
    if 4 as libc::c_int == 16 as libc::c_int && 16 as libc::c_int == 16 as libc::c_int {
        sum
            += x264_pixel_satd_4x4(
                pix1
                    .offset(8 as libc::c_int as isize)
                    .offset((8 as libc::c_int as intptr_t * i_pix1) as isize),
                i_pix1,
                pix2
                    .offset(8 as libc::c_int as isize)
                    .offset((8 as libc::c_int as intptr_t * i_pix2) as isize),
                i_pix2,
            )
                + x264_pixel_satd_4x4(
                    pix1
                        .offset(8 as libc::c_int as isize)
                        .offset((12 as libc::c_int as intptr_t * i_pix1) as isize),
                    i_pix1,
                    pix2
                        .offset(8 as libc::c_int as isize)
                        .offset((12 as libc::c_int as intptr_t * i_pix2) as isize),
                    i_pix2,
                );
    }
    return sum;
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
        sum
            += x264_pixel_satd_4x4(
                pix1.offset(8 as libc::c_int as isize),
                i_pix1,
                pix2.offset(8 as libc::c_int as isize),
                i_pix2,
            )
                + x264_pixel_satd_4x4(
                    pix1
                        .offset(8 as libc::c_int as isize)
                        .offset((4 as libc::c_int as intptr_t * i_pix1) as isize),
                    i_pix1,
                    pix2
                        .offset(8 as libc::c_int as isize)
                        .offset((4 as libc::c_int as intptr_t * i_pix2) as isize),
                    i_pix2,
                );
    }
    if 8 as libc::c_int == 16 as libc::c_int {
        sum
            += x264_pixel_satd_4x4(
                pix1.offset((8 as libc::c_int as intptr_t * i_pix1) as isize),
                i_pix1,
                pix2.offset((8 as libc::c_int as intptr_t * i_pix2) as isize),
                i_pix2,
            )
                + x264_pixel_satd_4x4(
                    pix1.offset((12 as libc::c_int as intptr_t * i_pix1) as isize),
                    i_pix1,
                    pix2.offset((12 as libc::c_int as intptr_t * i_pix2) as isize),
                    i_pix2,
                );
    }
    if 4 as libc::c_int == 16 as libc::c_int && 8 as libc::c_int == 16 as libc::c_int {
        sum
            += x264_pixel_satd_4x4(
                pix1
                    .offset(8 as libc::c_int as isize)
                    .offset((8 as libc::c_int as intptr_t * i_pix1) as isize),
                i_pix1,
                pix2
                    .offset(8 as libc::c_int as isize)
                    .offset((8 as libc::c_int as intptr_t * i_pix2) as isize),
                i_pix2,
            )
                + x264_pixel_satd_4x4(
                    pix1
                        .offset(8 as libc::c_int as isize)
                        .offset((12 as libc::c_int as intptr_t * i_pix1) as isize),
                    i_pix1,
                    pix2
                        .offset(8 as libc::c_int as isize)
                        .offset((12 as libc::c_int as intptr_t * i_pix2) as isize),
                    i_pix2,
                );
    }
    return sum;
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
        b0 = a0
            .wrapping_add(a1)
            .wrapping_add(
                a0.wrapping_sub(a1)
                    << (8 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<sum_t>() as libc::c_ulong),
            );
        a2 = (*pix1.offset(2 as libc::c_int as isize) as libc::c_int
            - *pix2.offset(2 as libc::c_int as isize) as libc::c_int) as sum2_t;
        a3 = (*pix1.offset(3 as libc::c_int as isize) as libc::c_int
            - *pix2.offset(3 as libc::c_int as isize) as libc::c_int) as sum2_t;
        b1 = a2
            .wrapping_add(a3)
            .wrapping_add(
                a2.wrapping_sub(a3)
                    << (8 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<sum_t>() as libc::c_ulong),
            );
        a4 = (*pix1.offset(4 as libc::c_int as isize) as libc::c_int
            - *pix2.offset(4 as libc::c_int as isize) as libc::c_int) as sum2_t;
        a5 = (*pix1.offset(5 as libc::c_int as isize) as libc::c_int
            - *pix2.offset(5 as libc::c_int as isize) as libc::c_int) as sum2_t;
        b2 = a4
            .wrapping_add(a5)
            .wrapping_add(
                a4.wrapping_sub(a5)
                    << (8 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<sum_t>() as libc::c_ulong),
            );
        a6 = (*pix1.offset(6 as libc::c_int as isize) as libc::c_int
            - *pix2.offset(6 as libc::c_int as isize) as libc::c_int) as sum2_t;
        a7 = (*pix1.offset(7 as libc::c_int as isize) as libc::c_int
            - *pix2.offset(7 as libc::c_int as isize) as libc::c_int) as sum2_t;
        b3 = a6
            .wrapping_add(a7)
            .wrapping_add(
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
        b0 = b0
            .wrapping_add(
                (abs2(a1.wrapping_add(a5))).wrapping_add(abs2(a1.wrapping_sub(a5))),
            );
        b0 = b0
            .wrapping_add(
                (abs2(a2.wrapping_add(a6))).wrapping_add(abs2(a2.wrapping_sub(a6))),
            );
        b0 = b0
            .wrapping_add(
                (abs2(a3.wrapping_add(a7))).wrapping_add(abs2(a3.wrapping_sub(a7))),
            );
        sum = sum
            .wrapping_add(
                (b0 as sum_t as sum2_t)
                    .wrapping_add(
                        b0
                            >> (8 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(
                                    ::core::mem::size_of::<sum_t>() as libc::c_ulong,
                                ),
                    ),
            );
        i_0 += 1;
        i_0;
    }
    return sum as libc::c_int;
}
unsafe extern "C" fn x264_pixel_sa8d_8x8(
    mut pix1: *mut pixel,
    mut i_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_pix2: intptr_t,
) -> libc::c_int {
    let mut sum: libc::c_int = sa8d_8x8(pix1, i_pix1, pix2, i_pix2);
    return sum + 2 as libc::c_int >> 2 as libc::c_int;
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
            pix1
                .offset(8 as libc::c_int as isize)
                .offset((8 as libc::c_int as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2
                .offset(8 as libc::c_int as isize)
                .offset((8 as libc::c_int as intptr_t * i_pix2) as isize),
            i_pix2,
        );
    return sum + 2 as libc::c_int >> 2 as libc::c_int;
}
#[inline(never)]
unsafe extern "C" fn pixel_hadamard_ac(
    mut pix: *mut pixel,
    mut stride: intptr_t,
) -> uint64_t {
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
                    - *pix.offset(1 as libc::c_int as isize) as libc::c_int) as sum2_t)
                    << (8 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<sum_t>() as libc::c_ulong),
            );
        a1 = ((*pix.offset(2 as libc::c_int as isize) as libc::c_int
            + *pix.offset(3 as libc::c_int as isize) as libc::c_int) as sum2_t)
            .wrapping_add(
                ((*pix.offset(2 as libc::c_int as isize) as libc::c_int
                    - *pix.offset(3 as libc::c_int as isize) as libc::c_int) as sum2_t)
                    << (8 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<sum_t>() as libc::c_ulong),
            );
        *t.offset(0 as libc::c_int as isize) = a0.wrapping_add(a1);
        *t.offset(4 as libc::c_int as isize) = a0.wrapping_sub(a1);
        a2 = ((*pix.offset(4 as libc::c_int as isize) as libc::c_int
            + *pix.offset(5 as libc::c_int as isize) as libc::c_int) as sum2_t)
            .wrapping_add(
                ((*pix.offset(4 as libc::c_int as isize) as libc::c_int
                    - *pix.offset(5 as libc::c_int as isize) as libc::c_int) as sum2_t)
                    << (8 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<sum_t>() as libc::c_ulong),
            );
        a3 = ((*pix.offset(6 as libc::c_int as isize) as libc::c_int
            + *pix.offset(7 as libc::c_int as isize) as libc::c_int) as sum2_t)
            .wrapping_add(
                ((*pix.offset(6 as libc::c_int as isize) as libc::c_int
                    - *pix.offset(7 as libc::c_int as isize) as libc::c_int) as sum2_t)
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
        sum4 = sum4
            .wrapping_add(
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
        let mut t0_0: sum2_t = (tmp[i_1 as usize])
            .wrapping_add(tmp[(8 as libc::c_int + i_1) as usize]);
        let mut t1_0: sum2_t = (tmp[i_1 as usize])
            .wrapping_sub(tmp[(8 as libc::c_int + i_1) as usize]);
        let mut t2_0: sum2_t = (tmp[(16 as libc::c_int + i_1) as usize])
            .wrapping_add(tmp[(24 as libc::c_int + i_1) as usize]);
        let mut t3_0: sum2_t = (tmp[(16 as libc::c_int + i_1) as usize])
            .wrapping_sub(tmp[(24 as libc::c_int + i_1) as usize]);
        a0 = t0_0.wrapping_add(t2_0);
        a2 = t0_0.wrapping_sub(t2_0);
        a1 = t1_0.wrapping_add(t3_0);
        a3 = t1_0.wrapping_sub(t3_0);
        sum8 = sum8
            .wrapping_add(
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
            sum4
                >> (8 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<sum_t>() as libc::c_ulong),
        )
        .wrapping_sub(dc);
    sum8 = (sum8 as sum_t as sum2_t)
        .wrapping_add(
            sum8
                >> (8 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<sum_t>() as libc::c_ulong),
        )
        .wrapping_sub(dc);
    return ((sum8 as uint64_t) << 32 as libc::c_int).wrapping_add(sum4 as uint64_t);
}
unsafe extern "C" fn x264_pixel_hadamard_ac_16x16(
    mut pix: *mut pixel,
    mut stride: intptr_t,
) -> uint64_t {
    let mut sum: uint64_t = pixel_hadamard_ac(pix, stride);
    if 16 as libc::c_int == 16 as libc::c_int {
        sum = sum
            .wrapping_add(
                pixel_hadamard_ac(pix.offset(8 as libc::c_int as isize), stride),
            );
    }
    if 16 as libc::c_int == 16 as libc::c_int {
        sum = sum
            .wrapping_add(
                pixel_hadamard_ac(
                    pix.offset((8 as libc::c_int as intptr_t * stride) as isize),
                    stride,
                ),
            );
    }
    if 16 as libc::c_int == 16 as libc::c_int && 16 as libc::c_int == 16 as libc::c_int {
        sum = sum
            .wrapping_add(
                pixel_hadamard_ac(
                    pix
                        .offset((8 as libc::c_int as intptr_t * stride) as isize)
                        .offset(8 as libc::c_int as isize),
                    stride,
                ),
            );
    }
    return ((sum >> 34 as libc::c_int) << 32 as libc::c_int)
        .wrapping_add((sum as uint32_t >> 1 as libc::c_int) as uint64_t);
}
unsafe extern "C" fn x264_pixel_hadamard_ac_16x8(
    mut pix: *mut pixel,
    mut stride: intptr_t,
) -> uint64_t {
    let mut sum: uint64_t = pixel_hadamard_ac(pix, stride);
    if 16 as libc::c_int == 16 as libc::c_int {
        sum = sum
            .wrapping_add(
                pixel_hadamard_ac(pix.offset(8 as libc::c_int as isize), stride),
            );
    }
    if 8 as libc::c_int == 16 as libc::c_int {
        sum = sum
            .wrapping_add(
                pixel_hadamard_ac(
                    pix.offset((8 as libc::c_int as intptr_t * stride) as isize),
                    stride,
                ),
            );
    }
    if 16 as libc::c_int == 16 as libc::c_int && 8 as libc::c_int == 16 as libc::c_int {
        sum = sum
            .wrapping_add(
                pixel_hadamard_ac(
                    pix
                        .offset((8 as libc::c_int as intptr_t * stride) as isize)
                        .offset(8 as libc::c_int as isize),
                    stride,
                ),
            );
    }
    return ((sum >> 34 as libc::c_int) << 32 as libc::c_int)
        .wrapping_add((sum as uint32_t >> 1 as libc::c_int) as uint64_t);
}
unsafe extern "C" fn x264_pixel_hadamard_ac_8x16(
    mut pix: *mut pixel,
    mut stride: intptr_t,
) -> uint64_t {
    let mut sum: uint64_t = pixel_hadamard_ac(pix, stride);
    if 8 as libc::c_int == 16 as libc::c_int {
        sum = sum
            .wrapping_add(
                pixel_hadamard_ac(pix.offset(8 as libc::c_int as isize), stride),
            );
    }
    if 16 as libc::c_int == 16 as libc::c_int {
        sum = sum
            .wrapping_add(
                pixel_hadamard_ac(
                    pix.offset((8 as libc::c_int as intptr_t * stride) as isize),
                    stride,
                ),
            );
    }
    if 8 as libc::c_int == 16 as libc::c_int && 16 as libc::c_int == 16 as libc::c_int {
        sum = sum
            .wrapping_add(
                pixel_hadamard_ac(
                    pix
                        .offset((8 as libc::c_int as intptr_t * stride) as isize)
                        .offset(8 as libc::c_int as isize),
                    stride,
                ),
            );
    }
    return ((sum >> 34 as libc::c_int) << 32 as libc::c_int)
        .wrapping_add((sum as uint32_t >> 1 as libc::c_int) as uint64_t);
}
unsafe extern "C" fn x264_pixel_hadamard_ac_8x8(
    mut pix: *mut pixel,
    mut stride: intptr_t,
) -> uint64_t {
    let mut sum: uint64_t = pixel_hadamard_ac(pix, stride);
    if 8 as libc::c_int == 16 as libc::c_int {
        sum = sum
            .wrapping_add(
                pixel_hadamard_ac(pix.offset(8 as libc::c_int as isize), stride),
            );
    }
    if 8 as libc::c_int == 16 as libc::c_int {
        sum = sum
            .wrapping_add(
                pixel_hadamard_ac(
                    pix.offset((8 as libc::c_int as intptr_t * stride) as isize),
                    stride,
                ),
            );
    }
    if 8 as libc::c_int == 16 as libc::c_int && 8 as libc::c_int == 16 as libc::c_int {
        sum = sum
            .wrapping_add(
                pixel_hadamard_ac(
                    pix
                        .offset((8 as libc::c_int as intptr_t * stride) as isize)
                        .offset(8 as libc::c_int as isize),
                    stride,
                ),
            );
    }
    return ((sum >> 34 as libc::c_int) << 32 as libc::c_int)
        .wrapping_add((sum as uint32_t >> 1 as libc::c_int) as uint64_t);
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
    *scores
        .offset(
            0 as libc::c_int as isize,
        ) = x264_pixel_sad_16x16(fenc, 16 as libc::c_int as intptr_t, pix0, i_stride);
    *scores
        .offset(
            1 as libc::c_int as isize,
        ) = x264_pixel_sad_16x16(fenc, 16 as libc::c_int as intptr_t, pix1, i_stride);
    *scores
        .offset(
            2 as libc::c_int as isize,
        ) = x264_pixel_sad_16x16(fenc, 16 as libc::c_int as intptr_t, pix2, i_stride);
    *scores
        .offset(
            3 as libc::c_int as isize,
        ) = x264_pixel_sad_16x16(fenc, 16 as libc::c_int as intptr_t, pix3, i_stride);
}
unsafe extern "C" fn x264_pixel_sad_x3_16x16(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut libc::c_int,
) {
    *scores
        .offset(
            0 as libc::c_int as isize,
        ) = x264_pixel_sad_16x16(fenc, 16 as libc::c_int as intptr_t, pix0, i_stride);
    *scores
        .offset(
            1 as libc::c_int as isize,
        ) = x264_pixel_sad_16x16(fenc, 16 as libc::c_int as intptr_t, pix1, i_stride);
    *scores
        .offset(
            2 as libc::c_int as isize,
        ) = x264_pixel_sad_16x16(fenc, 16 as libc::c_int as intptr_t, pix2, i_stride);
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
    *scores
        .offset(
            0 as libc::c_int as isize,
        ) = x264_pixel_sad_16x8(fenc, 16 as libc::c_int as intptr_t, pix0, i_stride);
    *scores
        .offset(
            1 as libc::c_int as isize,
        ) = x264_pixel_sad_16x8(fenc, 16 as libc::c_int as intptr_t, pix1, i_stride);
    *scores
        .offset(
            2 as libc::c_int as isize,
        ) = x264_pixel_sad_16x8(fenc, 16 as libc::c_int as intptr_t, pix2, i_stride);
    *scores
        .offset(
            3 as libc::c_int as isize,
        ) = x264_pixel_sad_16x8(fenc, 16 as libc::c_int as intptr_t, pix3, i_stride);
}
unsafe extern "C" fn x264_pixel_sad_x3_16x8(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut libc::c_int,
) {
    *scores
        .offset(
            0 as libc::c_int as isize,
        ) = x264_pixel_sad_16x8(fenc, 16 as libc::c_int as intptr_t, pix0, i_stride);
    *scores
        .offset(
            1 as libc::c_int as isize,
        ) = x264_pixel_sad_16x8(fenc, 16 as libc::c_int as intptr_t, pix1, i_stride);
    *scores
        .offset(
            2 as libc::c_int as isize,
        ) = x264_pixel_sad_16x8(fenc, 16 as libc::c_int as intptr_t, pix2, i_stride);
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
    *scores
        .offset(
            0 as libc::c_int as isize,
        ) = x264_pixel_sad_8x16(fenc, 16 as libc::c_int as intptr_t, pix0, i_stride);
    *scores
        .offset(
            1 as libc::c_int as isize,
        ) = x264_pixel_sad_8x16(fenc, 16 as libc::c_int as intptr_t, pix1, i_stride);
    *scores
        .offset(
            2 as libc::c_int as isize,
        ) = x264_pixel_sad_8x16(fenc, 16 as libc::c_int as intptr_t, pix2, i_stride);
    *scores
        .offset(
            3 as libc::c_int as isize,
        ) = x264_pixel_sad_8x16(fenc, 16 as libc::c_int as intptr_t, pix3, i_stride);
}
unsafe extern "C" fn x264_pixel_sad_x3_8x16(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut libc::c_int,
) {
    *scores
        .offset(
            0 as libc::c_int as isize,
        ) = x264_pixel_sad_8x16(fenc, 16 as libc::c_int as intptr_t, pix0, i_stride);
    *scores
        .offset(
            1 as libc::c_int as isize,
        ) = x264_pixel_sad_8x16(fenc, 16 as libc::c_int as intptr_t, pix1, i_stride);
    *scores
        .offset(
            2 as libc::c_int as isize,
        ) = x264_pixel_sad_8x16(fenc, 16 as libc::c_int as intptr_t, pix2, i_stride);
}
unsafe extern "C" fn x264_pixel_sad_x3_8x8(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut libc::c_int,
) {
    *scores
        .offset(
            0 as libc::c_int as isize,
        ) = x264_pixel_sad_8x8(fenc, 16 as libc::c_int as intptr_t, pix0, i_stride);
    *scores
        .offset(
            1 as libc::c_int as isize,
        ) = x264_pixel_sad_8x8(fenc, 16 as libc::c_int as intptr_t, pix1, i_stride);
    *scores
        .offset(
            2 as libc::c_int as isize,
        ) = x264_pixel_sad_8x8(fenc, 16 as libc::c_int as intptr_t, pix2, i_stride);
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
    *scores
        .offset(
            0 as libc::c_int as isize,
        ) = x264_pixel_sad_8x8(fenc, 16 as libc::c_int as intptr_t, pix0, i_stride);
    *scores
        .offset(
            1 as libc::c_int as isize,
        ) = x264_pixel_sad_8x8(fenc, 16 as libc::c_int as intptr_t, pix1, i_stride);
    *scores
        .offset(
            2 as libc::c_int as isize,
        ) = x264_pixel_sad_8x8(fenc, 16 as libc::c_int as intptr_t, pix2, i_stride);
    *scores
        .offset(
            3 as libc::c_int as isize,
        ) = x264_pixel_sad_8x8(fenc, 16 as libc::c_int as intptr_t, pix3, i_stride);
}
unsafe extern "C" fn x264_pixel_sad_x3_8x4(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut libc::c_int,
) {
    *scores
        .offset(
            0 as libc::c_int as isize,
        ) = x264_pixel_sad_8x4(fenc, 16 as libc::c_int as intptr_t, pix0, i_stride);
    *scores
        .offset(
            1 as libc::c_int as isize,
        ) = x264_pixel_sad_8x4(fenc, 16 as libc::c_int as intptr_t, pix1, i_stride);
    *scores
        .offset(
            2 as libc::c_int as isize,
        ) = x264_pixel_sad_8x4(fenc, 16 as libc::c_int as intptr_t, pix2, i_stride);
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
    *scores
        .offset(
            0 as libc::c_int as isize,
        ) = x264_pixel_sad_8x4(fenc, 16 as libc::c_int as intptr_t, pix0, i_stride);
    *scores
        .offset(
            1 as libc::c_int as isize,
        ) = x264_pixel_sad_8x4(fenc, 16 as libc::c_int as intptr_t, pix1, i_stride);
    *scores
        .offset(
            2 as libc::c_int as isize,
        ) = x264_pixel_sad_8x4(fenc, 16 as libc::c_int as intptr_t, pix2, i_stride);
    *scores
        .offset(
            3 as libc::c_int as isize,
        ) = x264_pixel_sad_8x4(fenc, 16 as libc::c_int as intptr_t, pix3, i_stride);
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
    *scores
        .offset(
            0 as libc::c_int as isize,
        ) = x264_pixel_sad_4x8(fenc, 16 as libc::c_int as intptr_t, pix0, i_stride);
    *scores
        .offset(
            1 as libc::c_int as isize,
        ) = x264_pixel_sad_4x8(fenc, 16 as libc::c_int as intptr_t, pix1, i_stride);
    *scores
        .offset(
            2 as libc::c_int as isize,
        ) = x264_pixel_sad_4x8(fenc, 16 as libc::c_int as intptr_t, pix2, i_stride);
    *scores
        .offset(
            3 as libc::c_int as isize,
        ) = x264_pixel_sad_4x8(fenc, 16 as libc::c_int as intptr_t, pix3, i_stride);
}
unsafe extern "C" fn x264_pixel_sad_x3_4x8(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut libc::c_int,
) {
    *scores
        .offset(
            0 as libc::c_int as isize,
        ) = x264_pixel_sad_4x8(fenc, 16 as libc::c_int as intptr_t, pix0, i_stride);
    *scores
        .offset(
            1 as libc::c_int as isize,
        ) = x264_pixel_sad_4x8(fenc, 16 as libc::c_int as intptr_t, pix1, i_stride);
    *scores
        .offset(
            2 as libc::c_int as isize,
        ) = x264_pixel_sad_4x8(fenc, 16 as libc::c_int as intptr_t, pix2, i_stride);
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
    *scores
        .offset(
            0 as libc::c_int as isize,
        ) = x264_pixel_sad_4x4(fenc, 16 as libc::c_int as intptr_t, pix0, i_stride);
    *scores
        .offset(
            1 as libc::c_int as isize,
        ) = x264_pixel_sad_4x4(fenc, 16 as libc::c_int as intptr_t, pix1, i_stride);
    *scores
        .offset(
            2 as libc::c_int as isize,
        ) = x264_pixel_sad_4x4(fenc, 16 as libc::c_int as intptr_t, pix2, i_stride);
    *scores
        .offset(
            3 as libc::c_int as isize,
        ) = x264_pixel_sad_4x4(fenc, 16 as libc::c_int as intptr_t, pix3, i_stride);
}
unsafe extern "C" fn x264_pixel_sad_x3_4x4(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut libc::c_int,
) {
    *scores
        .offset(
            0 as libc::c_int as isize,
        ) = x264_pixel_sad_4x4(fenc, 16 as libc::c_int as intptr_t, pix0, i_stride);
    *scores
        .offset(
            1 as libc::c_int as isize,
        ) = x264_pixel_sad_4x4(fenc, 16 as libc::c_int as intptr_t, pix1, i_stride);
    *scores
        .offset(
            2 as libc::c_int as isize,
        ) = x264_pixel_sad_4x4(fenc, 16 as libc::c_int as intptr_t, pix2, i_stride);
}
unsafe extern "C" fn x264_pixel_satd_x3_8x16(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut libc::c_int,
) {
    *scores
        .offset(
            0 as libc::c_int as isize,
        ) = x264_pixel_satd_8x16(fenc, 16 as libc::c_int as intptr_t, pix0, i_stride);
    *scores
        .offset(
            1 as libc::c_int as isize,
        ) = x264_pixel_satd_8x16(fenc, 16 as libc::c_int as intptr_t, pix1, i_stride);
    *scores
        .offset(
            2 as libc::c_int as isize,
        ) = x264_pixel_satd_8x16(fenc, 16 as libc::c_int as intptr_t, pix2, i_stride);
}
unsafe extern "C" fn x264_pixel_satd_x3_16x16(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut libc::c_int,
) {
    *scores
        .offset(
            0 as libc::c_int as isize,
        ) = x264_pixel_satd_16x16(fenc, 16 as libc::c_int as intptr_t, pix0, i_stride);
    *scores
        .offset(
            1 as libc::c_int as isize,
        ) = x264_pixel_satd_16x16(fenc, 16 as libc::c_int as intptr_t, pix1, i_stride);
    *scores
        .offset(
            2 as libc::c_int as isize,
        ) = x264_pixel_satd_16x16(fenc, 16 as libc::c_int as intptr_t, pix2, i_stride);
}
unsafe extern "C" fn x264_pixel_satd_x3_16x8(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut libc::c_int,
) {
    *scores
        .offset(
            0 as libc::c_int as isize,
        ) = x264_pixel_satd_16x8(fenc, 16 as libc::c_int as intptr_t, pix0, i_stride);
    *scores
        .offset(
            1 as libc::c_int as isize,
        ) = x264_pixel_satd_16x8(fenc, 16 as libc::c_int as intptr_t, pix1, i_stride);
    *scores
        .offset(
            2 as libc::c_int as isize,
        ) = x264_pixel_satd_16x8(fenc, 16 as libc::c_int as intptr_t, pix2, i_stride);
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
    *scores
        .offset(
            0 as libc::c_int as isize,
        ) = x264_pixel_satd_4x4(fenc, 16 as libc::c_int as intptr_t, pix0, i_stride);
    *scores
        .offset(
            1 as libc::c_int as isize,
        ) = x264_pixel_satd_4x4(fenc, 16 as libc::c_int as intptr_t, pix1, i_stride);
    *scores
        .offset(
            2 as libc::c_int as isize,
        ) = x264_pixel_satd_4x4(fenc, 16 as libc::c_int as intptr_t, pix2, i_stride);
    *scores
        .offset(
            3 as libc::c_int as isize,
        ) = x264_pixel_satd_4x4(fenc, 16 as libc::c_int as intptr_t, pix3, i_stride);
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
    *scores
        .offset(
            0 as libc::c_int as isize,
        ) = x264_pixel_satd_4x8(fenc, 16 as libc::c_int as intptr_t, pix0, i_stride);
    *scores
        .offset(
            1 as libc::c_int as isize,
        ) = x264_pixel_satd_4x8(fenc, 16 as libc::c_int as intptr_t, pix1, i_stride);
    *scores
        .offset(
            2 as libc::c_int as isize,
        ) = x264_pixel_satd_4x8(fenc, 16 as libc::c_int as intptr_t, pix2, i_stride);
    *scores
        .offset(
            3 as libc::c_int as isize,
        ) = x264_pixel_satd_4x8(fenc, 16 as libc::c_int as intptr_t, pix3, i_stride);
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
    *scores
        .offset(
            0 as libc::c_int as isize,
        ) = x264_pixel_satd_8x4(fenc, 16 as libc::c_int as intptr_t, pix0, i_stride);
    *scores
        .offset(
            1 as libc::c_int as isize,
        ) = x264_pixel_satd_8x4(fenc, 16 as libc::c_int as intptr_t, pix1, i_stride);
    *scores
        .offset(
            2 as libc::c_int as isize,
        ) = x264_pixel_satd_8x4(fenc, 16 as libc::c_int as intptr_t, pix2, i_stride);
    *scores
        .offset(
            3 as libc::c_int as isize,
        ) = x264_pixel_satd_8x4(fenc, 16 as libc::c_int as intptr_t, pix3, i_stride);
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
    *scores
        .offset(
            0 as libc::c_int as isize,
        ) = x264_pixel_satd_8x8(fenc, 16 as libc::c_int as intptr_t, pix0, i_stride);
    *scores
        .offset(
            1 as libc::c_int as isize,
        ) = x264_pixel_satd_8x8(fenc, 16 as libc::c_int as intptr_t, pix1, i_stride);
    *scores
        .offset(
            2 as libc::c_int as isize,
        ) = x264_pixel_satd_8x8(fenc, 16 as libc::c_int as intptr_t, pix2, i_stride);
    *scores
        .offset(
            3 as libc::c_int as isize,
        ) = x264_pixel_satd_8x8(fenc, 16 as libc::c_int as intptr_t, pix3, i_stride);
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
    *scores
        .offset(
            0 as libc::c_int as isize,
        ) = x264_pixel_satd_8x16(fenc, 16 as libc::c_int as intptr_t, pix0, i_stride);
    *scores
        .offset(
            1 as libc::c_int as isize,
        ) = x264_pixel_satd_8x16(fenc, 16 as libc::c_int as intptr_t, pix1, i_stride);
    *scores
        .offset(
            2 as libc::c_int as isize,
        ) = x264_pixel_satd_8x16(fenc, 16 as libc::c_int as intptr_t, pix2, i_stride);
    *scores
        .offset(
            3 as libc::c_int as isize,
        ) = x264_pixel_satd_8x16(fenc, 16 as libc::c_int as intptr_t, pix3, i_stride);
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
    *scores
        .offset(
            0 as libc::c_int as isize,
        ) = x264_pixel_satd_16x8(fenc, 16 as libc::c_int as intptr_t, pix0, i_stride);
    *scores
        .offset(
            1 as libc::c_int as isize,
        ) = x264_pixel_satd_16x8(fenc, 16 as libc::c_int as intptr_t, pix1, i_stride);
    *scores
        .offset(
            2 as libc::c_int as isize,
        ) = x264_pixel_satd_16x8(fenc, 16 as libc::c_int as intptr_t, pix2, i_stride);
    *scores
        .offset(
            3 as libc::c_int as isize,
        ) = x264_pixel_satd_16x8(fenc, 16 as libc::c_int as intptr_t, pix3, i_stride);
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
    *scores
        .offset(
            0 as libc::c_int as isize,
        ) = x264_pixel_satd_16x16(fenc, 16 as libc::c_int as intptr_t, pix0, i_stride);
    *scores
        .offset(
            1 as libc::c_int as isize,
        ) = x264_pixel_satd_16x16(fenc, 16 as libc::c_int as intptr_t, pix1, i_stride);
    *scores
        .offset(
            2 as libc::c_int as isize,
        ) = x264_pixel_satd_16x16(fenc, 16 as libc::c_int as intptr_t, pix2, i_stride);
    *scores
        .offset(
            3 as libc::c_int as isize,
        ) = x264_pixel_satd_16x16(fenc, 16 as libc::c_int as intptr_t, pix3, i_stride);
}
unsafe extern "C" fn x264_pixel_satd_x3_4x4(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut libc::c_int,
) {
    *scores
        .offset(
            0 as libc::c_int as isize,
        ) = x264_pixel_satd_4x4(fenc, 16 as libc::c_int as intptr_t, pix0, i_stride);
    *scores
        .offset(
            1 as libc::c_int as isize,
        ) = x264_pixel_satd_4x4(fenc, 16 as libc::c_int as intptr_t, pix1, i_stride);
    *scores
        .offset(
            2 as libc::c_int as isize,
        ) = x264_pixel_satd_4x4(fenc, 16 as libc::c_int as intptr_t, pix2, i_stride);
}
unsafe extern "C" fn x264_pixel_satd_x3_4x8(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut libc::c_int,
) {
    *scores
        .offset(
            0 as libc::c_int as isize,
        ) = x264_pixel_satd_4x8(fenc, 16 as libc::c_int as intptr_t, pix0, i_stride);
    *scores
        .offset(
            1 as libc::c_int as isize,
        ) = x264_pixel_satd_4x8(fenc, 16 as libc::c_int as intptr_t, pix1, i_stride);
    *scores
        .offset(
            2 as libc::c_int as isize,
        ) = x264_pixel_satd_4x8(fenc, 16 as libc::c_int as intptr_t, pix2, i_stride);
}
unsafe extern "C" fn x264_pixel_satd_x3_8x4(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut libc::c_int,
) {
    *scores
        .offset(
            0 as libc::c_int as isize,
        ) = x264_pixel_satd_8x4(fenc, 16 as libc::c_int as intptr_t, pix0, i_stride);
    *scores
        .offset(
            1 as libc::c_int as isize,
        ) = x264_pixel_satd_8x4(fenc, 16 as libc::c_int as intptr_t, pix1, i_stride);
    *scores
        .offset(
            2 as libc::c_int as isize,
        ) = x264_pixel_satd_8x4(fenc, 16 as libc::c_int as intptr_t, pix2, i_stride);
}
unsafe extern "C" fn x264_pixel_satd_x3_8x8(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut libc::c_int,
) {
    *scores
        .offset(
            0 as libc::c_int as isize,
        ) = x264_pixel_satd_8x8(fenc, 16 as libc::c_int as intptr_t, pix0, i_stride);
    *scores
        .offset(
            1 as libc::c_int as isize,
        ) = x264_pixel_satd_8x8(fenc, 16 as libc::c_int as intptr_t, pix1, i_stride);
    *scores
        .offset(
            2 as libc::c_int as isize,
        ) = x264_pixel_satd_8x8(fenc, 16 as libc::c_int as intptr_t, pix2, i_stride);
}
unsafe extern "C" fn intra_sad_x3_8x8(
    mut fenc: *mut pixel,
    mut edge: *mut pixel,
    mut res: *mut libc::c_int,
) {
    let mut pix: [pixel; 256] = [0; 256];
    x264_8_predict_8x8_v_c(pix.as_mut_ptr(), edge);
    *res
        .offset(
            0 as libc::c_int as isize,
        ) = x264_pixel_sad_8x8(
        pix.as_mut_ptr(),
        32 as libc::c_int as intptr_t,
        fenc,
        16 as libc::c_int as intptr_t,
    );
    x264_8_predict_8x8_h_c(pix.as_mut_ptr(), edge);
    *res
        .offset(
            1 as libc::c_int as isize,
        ) = x264_pixel_sad_8x8(
        pix.as_mut_ptr(),
        32 as libc::c_int as intptr_t,
        fenc,
        16 as libc::c_int as intptr_t,
    );
    x264_8_predict_8x8_dc_c(pix.as_mut_ptr(), edge);
    *res
        .offset(
            2 as libc::c_int as isize,
        ) = x264_pixel_sad_8x8(
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
    *res
        .offset(
            0 as libc::c_int as isize,
        ) = x264_pixel_sa8d_8x8(
        pix.as_mut_ptr(),
        32 as libc::c_int as intptr_t,
        fenc,
        16 as libc::c_int as intptr_t,
    );
    x264_8_predict_8x8_h_c(pix.as_mut_ptr(), edge);
    *res
        .offset(
            1 as libc::c_int as isize,
        ) = x264_pixel_sa8d_8x8(
        pix.as_mut_ptr(),
        32 as libc::c_int as intptr_t,
        fenc,
        16 as libc::c_int as intptr_t,
    );
    x264_8_predict_8x8_dc_c(pix.as_mut_ptr(), edge);
    *res
        .offset(
            2 as libc::c_int as isize,
        ) = x264_pixel_sa8d_8x8(
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
    *res
        .offset(
            0 as libc::c_int as isize,
        ) = x264_pixel_sad_4x4(
        fdec,
        32 as libc::c_int as intptr_t,
        fenc,
        16 as libc::c_int as intptr_t,
    );
    x264_8_predict_4x4_h_c(fdec);
    *res
        .offset(
            1 as libc::c_int as isize,
        ) = x264_pixel_sad_4x4(
        fdec,
        32 as libc::c_int as intptr_t,
        fenc,
        16 as libc::c_int as intptr_t,
    );
    x264_8_predict_4x4_dc_c(fdec);
    *res
        .offset(
            2 as libc::c_int as isize,
        ) = x264_pixel_sad_4x4(
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
    *res
        .offset(
            0 as libc::c_int as isize,
        ) = x264_pixel_satd_4x4(
        fdec,
        32 as libc::c_int as intptr_t,
        fenc,
        16 as libc::c_int as intptr_t,
    );
    x264_8_predict_4x4_h_c(fdec);
    *res
        .offset(
            1 as libc::c_int as isize,
        ) = x264_pixel_satd_4x4(
        fdec,
        32 as libc::c_int as intptr_t,
        fenc,
        16 as libc::c_int as intptr_t,
    );
    x264_8_predict_4x4_dc_c(fdec);
    *res
        .offset(
            2 as libc::c_int as isize,
        ) = x264_pixel_satd_4x4(
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
    *res
        .offset(
            0 as libc::c_int as isize,
        ) = x264_pixel_sad_8x8(
        fdec,
        32 as libc::c_int as intptr_t,
        fenc,
        16 as libc::c_int as intptr_t,
    );
    x264_8_predict_8x8c_h_c(fdec);
    *res
        .offset(
            1 as libc::c_int as isize,
        ) = x264_pixel_sad_8x8(
        fdec,
        32 as libc::c_int as intptr_t,
        fenc,
        16 as libc::c_int as intptr_t,
    );
    x264_8_predict_8x8c_v_c(fdec);
    *res
        .offset(
            2 as libc::c_int as isize,
        ) = x264_pixel_sad_8x8(
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
    *res
        .offset(
            0 as libc::c_int as isize,
        ) = x264_pixel_satd_8x8(
        fdec,
        32 as libc::c_int as intptr_t,
        fenc,
        16 as libc::c_int as intptr_t,
    );
    x264_8_predict_8x8c_h_c(fdec);
    *res
        .offset(
            1 as libc::c_int as isize,
        ) = x264_pixel_satd_8x8(
        fdec,
        32 as libc::c_int as intptr_t,
        fenc,
        16 as libc::c_int as intptr_t,
    );
    x264_8_predict_8x8c_v_c(fdec);
    *res
        .offset(
            2 as libc::c_int as isize,
        ) = x264_pixel_satd_8x8(
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
    *res
        .offset(
            0 as libc::c_int as isize,
        ) = x264_pixel_sad_8x16(
        fdec,
        32 as libc::c_int as intptr_t,
        fenc,
        16 as libc::c_int as intptr_t,
    );
    x264_8_predict_8x16c_h_c(fdec);
    *res
        .offset(
            1 as libc::c_int as isize,
        ) = x264_pixel_sad_8x16(
        fdec,
        32 as libc::c_int as intptr_t,
        fenc,
        16 as libc::c_int as intptr_t,
    );
    x264_8_predict_8x16c_v_c(fdec);
    *res
        .offset(
            2 as libc::c_int as isize,
        ) = x264_pixel_sad_8x16(
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
    *res
        .offset(
            0 as libc::c_int as isize,
        ) = x264_pixel_satd_8x16(
        fdec,
        32 as libc::c_int as intptr_t,
        fenc,
        16 as libc::c_int as intptr_t,
    );
    x264_8_predict_8x16c_h_c(fdec);
    *res
        .offset(
            1 as libc::c_int as isize,
        ) = x264_pixel_satd_8x16(
        fdec,
        32 as libc::c_int as intptr_t,
        fenc,
        16 as libc::c_int as intptr_t,
    );
    x264_8_predict_8x16c_v_c(fdec);
    *res
        .offset(
            2 as libc::c_int as isize,
        ) = x264_pixel_satd_8x16(
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
    *res
        .offset(
            0 as libc::c_int as isize,
        ) = x264_pixel_sad_16x16(
        fdec,
        32 as libc::c_int as intptr_t,
        fenc,
        16 as libc::c_int as intptr_t,
    );
    x264_8_predict_16x16_h_c(fdec);
    *res
        .offset(
            1 as libc::c_int as isize,
        ) = x264_pixel_sad_16x16(
        fdec,
        32 as libc::c_int as intptr_t,
        fenc,
        16 as libc::c_int as intptr_t,
    );
    x264_8_predict_16x16_dc_c(fdec);
    *res
        .offset(
            2 as libc::c_int as isize,
        ) = x264_pixel_sad_16x16(
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
    *res
        .offset(
            0 as libc::c_int as isize,
        ) = x264_pixel_satd_16x16(
        fdec,
        32 as libc::c_int as intptr_t,
        fenc,
        16 as libc::c_int as intptr_t,
    );
    x264_8_predict_16x16_h_c(fdec);
    *res
        .offset(
            1 as libc::c_int as isize,
        ) = x264_pixel_satd_16x16(
        fdec,
        32 as libc::c_int as intptr_t,
        fenc,
        16 as libc::c_int as intptr_t,
    );
    x264_8_predict_16x16_dc_c(fdec);
    *res
        .offset(
            2 as libc::c_int as isize,
        ) = x264_pixel_satd_16x16(
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
                let mut a: libc::c_int = *pix1
                    .offset((x as intptr_t + y as intptr_t * stride1) as isize)
                    as libc::c_int;
                let mut b: libc::c_int = *pix2
                    .offset((x as intptr_t + y as intptr_t * stride2) as isize)
                    as libc::c_int;
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
    static mut ssim_c1: libc::c_int = (0.01f64 * 0.01f64
        * (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int) as libc::c_double
        * (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int) as libc::c_double
        * 64 as libc::c_int as libc::c_double + 0.5f64) as libc::c_int;
    static mut ssim_c2: libc::c_int = (0.03f64 * 0.03f64
        * (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int) as libc::c_double
        * (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int) as libc::c_double
        * 64 as libc::c_int as libc::c_double * 63 as libc::c_int as libc::c_double
        + 0.5f64) as libc::c_int;
    let mut fs1: libc::c_int = s1;
    let mut fs2: libc::c_int = s2;
    let mut fss: libc::c_int = ss;
    let mut fs12: libc::c_int = s12;
    let mut vars: libc::c_int = fss * 64 as libc::c_int - fs1 * fs1 - fs2 * fs2;
    let mut covar: libc::c_int = fs12 * 64 as libc::c_int - fs1 * fs2;
    return (2 as libc::c_int * fs1 * fs2 + ssim_c1) as libc::c_float
        * (2 as libc::c_int * covar + ssim_c2) as libc::c_float
        / ((fs1 * fs1 + fs2 * fs2 + ssim_c1) as libc::c_float
            * (vars + ssim_c2) as libc::c_float);
}
unsafe extern "C" fn ssim_end4(
    mut sum0: *mut [libc::c_int; 4],
    mut sum1: *mut [libc::c_int; 4],
    mut width: libc::c_int,
) -> libc::c_float {
    let mut ssim: libc::c_float = 0.0f64 as libc::c_float;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < width {
        ssim
            += ssim_end1(
                (*sum0.offset(i as isize))[0 as libc::c_int as usize]
                    + (*sum0
                        .offset(
                            (i + 1 as libc::c_int) as isize,
                        ))[0 as libc::c_int as usize]
                    + (*sum1.offset(i as isize))[0 as libc::c_int as usize]
                    + (*sum1
                        .offset(
                            (i + 1 as libc::c_int) as isize,
                        ))[0 as libc::c_int as usize],
                (*sum0.offset(i as isize))[1 as libc::c_int as usize]
                    + (*sum0
                        .offset(
                            (i + 1 as libc::c_int) as isize,
                        ))[1 as libc::c_int as usize]
                    + (*sum1.offset(i as isize))[1 as libc::c_int as usize]
                    + (*sum1
                        .offset(
                            (i + 1 as libc::c_int) as isize,
                        ))[1 as libc::c_int as usize],
                (*sum0.offset(i as isize))[2 as libc::c_int as usize]
                    + (*sum0
                        .offset(
                            (i + 1 as libc::c_int) as isize,
                        ))[2 as libc::c_int as usize]
                    + (*sum1.offset(i as isize))[2 as libc::c_int as usize]
                    + (*sum1
                        .offset(
                            (i + 1 as libc::c_int) as isize,
                        ))[2 as libc::c_int as usize],
                (*sum0.offset(i as isize))[3 as libc::c_int as usize]
                    + (*sum0
                        .offset(
                            (i + 1 as libc::c_int) as isize,
                        ))[3 as libc::c_int as usize]
                    + (*sum1.offset(i as isize))[3 as libc::c_int as usize]
                    + (*sum1
                        .offset(
                            (i + 1 as libc::c_int) as isize,
                        ))[3 as libc::c_int as usize],
            );
        i += 1;
        i;
    }
    return ssim;
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
                ((*pf).ssim_4x4x2_core)
                    .expect(
                        "non-null function pointer",
                    )(
                    &mut *pix1
                        .offset(
                            (4 as libc::c_int as intptr_t
                                * (x as intptr_t + z as intptr_t * stride1)) as isize,
                        ),
                    stride1,
                    &mut *pix2
                        .offset(
                            (4 as libc::c_int as intptr_t
                                * (x as intptr_t + z as intptr_t * stride2)) as isize,
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
            ssim
                += ((*pf).ssim_end4)
                    .expect(
                        "non-null function pointer",
                    )(
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
    return ssim;
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
            score
                += abs(
                    *src.offset(j as isize) as libc::c_int
                        - *src.offset((j as intptr_t + stride) as isize) as libc::c_int,
                );
            j += 1;
            j;
        }
        i += 1;
        i;
        src = src.offset(stride as isize);
    }
    return score;
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
    let mut mbpair_height: libc::c_int = if ((*h).param.i_height
        - mb_y * 16 as libc::c_int) < 32 as libc::c_int
    {
        (*h).param.i_height - mb_y * 16 as libc::c_int
    } else {
        32 as libc::c_int
    };
    score_frame = ((*h).pixf.vsad)
        .expect("non-null function pointer")(fenc, stride as intptr_t, mbpair_height);
    score_field = ((*h).pixf.vsad)
        .expect(
            "non-null function pointer",
        )(
        fenc,
        (stride * 2 as libc::c_int) as intptr_t,
        mbpair_height >> 1 as libc::c_int,
    );
    score_field
        += ((*h).pixf.vsad)
            .expect(
                "non-null function pointer",
            )(
            fenc.offset(stride as isize),
            (stride * 2 as libc::c_int) as intptr_t,
            mbpair_height >> 1 as libc::c_int,
        );
    if mb_x > 0 as libc::c_int {
        score_field
            += 512 as libc::c_int
                - *((*h).mb.field).offset((mb_xy - 1 as libc::c_int) as isize)
                    as libc::c_int * 1024 as libc::c_int;
    }
    if mb_y > 0 as libc::c_int {
        score_field
            += 512 as libc::c_int
                - *((*h).mb.field).offset((mb_xy - mb_stride) as isize) as libc::c_int
                    * 1024 as libc::c_int;
    }
    return (score_field < score_frame) as libc::c_int;
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
            sum
                += *pix1.offset(x as isize) as libc::c_int
                    - *pix2.offset(x as isize) as libc::c_int;
            x += 1;
            x;
        }
        y += 1;
        y;
        pix1 = pix1.offset(stride1 as isize);
        pix2 = pix2.offset(stride2 as isize);
    }
    return abs(sum);
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
        let mut ads: libc::c_int = abs(
            *enc_dc.offset(0 as libc::c_int as isize)
                - *sums.offset(0 as libc::c_int as isize) as libc::c_int,
        )
            + abs(
                *enc_dc.offset(1 as libc::c_int as isize)
                    - *sums.offset(8 as libc::c_int as isize) as libc::c_int,
            )
            + abs(
                *enc_dc.offset(2 as libc::c_int as isize)
                    - *sums.offset(delta as isize) as libc::c_int,
            )
            + abs(
                *enc_dc.offset(3 as libc::c_int as isize)
                    - *sums.offset((delta + 8 as libc::c_int) as isize) as libc::c_int,
            ) + *cost_mvx.offset(i as isize) as libc::c_int;
        if ads < thresh {
            let fresh0 = nmv;
            nmv = nmv + 1;
            *mvs.offset(fresh0 as isize) = i as int16_t;
        }
        i += 1;
        i;
        sums = sums.offset(1);
        sums;
    }
    return nmv;
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
        let mut ads: libc::c_int = abs(
            *enc_dc.offset(0 as libc::c_int as isize)
                - *sums.offset(0 as libc::c_int as isize) as libc::c_int,
        )
            + abs(
                *enc_dc.offset(1 as libc::c_int as isize)
                    - *sums.offset(delta as isize) as libc::c_int,
            ) + *cost_mvx.offset(i as isize) as libc::c_int;
        if ads < thresh {
            let fresh1 = nmv;
            nmv = nmv + 1;
            *mvs.offset(fresh1 as isize) = i as int16_t;
        }
        i += 1;
        i;
        sums = sums.offset(1);
        sums;
    }
    return nmv;
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
        let mut ads: libc::c_int = abs(
            *enc_dc.offset(0 as libc::c_int as isize)
                - *sums.offset(0 as libc::c_int as isize) as libc::c_int,
        ) + *cost_mvx.offset(i as isize) as libc::c_int;
        if ads < thresh {
            let fresh2 = nmv;
            nmv = nmv + 1;
            *mvs.offset(fresh2 as isize) = i as int16_t;
        }
        i += 1;
        i;
        sums = sums.offset(1);
        sums;
    }
    return nmv;
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
    (*pixf)
        .sad[PIXEL_16x16 as libc::c_int
        as usize] = Some(
        x264_pixel_sad_16x16
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> libc::c_int,
    );
    (*pixf)
        .sad[PIXEL_16x8 as libc::c_int
        as usize] = Some(
        x264_pixel_sad_16x8
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> libc::c_int,
    );
    (*pixf)
        .sad[PIXEL_8x16 as libc::c_int
        as usize] = Some(
        x264_pixel_sad_8x16
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> libc::c_int,
    );
    (*pixf)
        .sad[PIXEL_8x8 as libc::c_int
        as usize] = Some(
        x264_pixel_sad_8x8
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> libc::c_int,
    );
    (*pixf)
        .sad[PIXEL_8x4 as libc::c_int
        as usize] = Some(
        x264_pixel_sad_8x4
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> libc::c_int,
    );
    (*pixf)
        .sad[PIXEL_4x8 as libc::c_int
        as usize] = Some(
        x264_pixel_sad_4x8
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> libc::c_int,
    );
    (*pixf)
        .sad[PIXEL_4x4 as libc::c_int
        as usize] = Some(
        x264_pixel_sad_4x4
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> libc::c_int,
    );
    (*pixf)
        .sad[PIXEL_4x16 as libc::c_int
        as usize] = Some(
        x264_pixel_sad_4x16
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> libc::c_int,
    );
    (*pixf)
        .sad_aligned[PIXEL_16x16 as libc::c_int
        as usize] = Some(
        x264_pixel_sad_16x16
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> libc::c_int,
    );
    (*pixf)
        .sad_aligned[PIXEL_16x8 as libc::c_int
        as usize] = Some(
        x264_pixel_sad_16x8
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> libc::c_int,
    );
    (*pixf)
        .sad_aligned[PIXEL_8x16 as libc::c_int
        as usize] = Some(
        x264_pixel_sad_8x16
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> libc::c_int,
    );
    (*pixf)
        .sad_aligned[PIXEL_8x8 as libc::c_int
        as usize] = Some(
        x264_pixel_sad_8x8
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> libc::c_int,
    );
    (*pixf)
        .sad_aligned[PIXEL_8x4 as libc::c_int
        as usize] = Some(
        x264_pixel_sad_8x4
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> libc::c_int,
    );
    (*pixf)
        .sad_aligned[PIXEL_4x8 as libc::c_int
        as usize] = Some(
        x264_pixel_sad_4x8
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> libc::c_int,
    );
    (*pixf)
        .sad_aligned[PIXEL_4x4 as libc::c_int
        as usize] = Some(
        x264_pixel_sad_4x4
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> libc::c_int,
    );
    (*pixf)
        .sad_aligned[PIXEL_4x16 as libc::c_int
        as usize] = Some(
        x264_pixel_sad_4x16
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> libc::c_int,
    );
    (*pixf)
        .sad_x3[PIXEL_16x16 as libc::c_int
        as usize] = Some(
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
    (*pixf)
        .sad_x3[PIXEL_16x8 as libc::c_int
        as usize] = Some(
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
    (*pixf)
        .sad_x3[PIXEL_8x16 as libc::c_int
        as usize] = Some(
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
    (*pixf)
        .sad_x3[PIXEL_8x8 as libc::c_int
        as usize] = Some(
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
    (*pixf)
        .sad_x3[PIXEL_8x4 as libc::c_int
        as usize] = Some(
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
    (*pixf)
        .sad_x3[PIXEL_4x8 as libc::c_int
        as usize] = Some(
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
    (*pixf)
        .sad_x3[PIXEL_4x4 as libc::c_int
        as usize] = Some(
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
    (*pixf)
        .sad_x4[PIXEL_16x16 as libc::c_int
        as usize] = Some(
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
    (*pixf)
        .sad_x4[PIXEL_16x8 as libc::c_int
        as usize] = Some(
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
    (*pixf)
        .sad_x4[PIXEL_8x16 as libc::c_int
        as usize] = Some(
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
    (*pixf)
        .sad_x4[PIXEL_8x8 as libc::c_int
        as usize] = Some(
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
    (*pixf)
        .sad_x4[PIXEL_8x4 as libc::c_int
        as usize] = Some(
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
    (*pixf)
        .sad_x4[PIXEL_4x8 as libc::c_int
        as usize] = Some(
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
    (*pixf)
        .sad_x4[PIXEL_4x4 as libc::c_int
        as usize] = Some(
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
    (*pixf)
        .ssd[PIXEL_16x16 as libc::c_int
        as usize] = Some(
        x264_pixel_ssd_16x16
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> libc::c_int,
    );
    (*pixf)
        .ssd[PIXEL_16x8 as libc::c_int
        as usize] = Some(
        x264_pixel_ssd_16x8
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> libc::c_int,
    );
    (*pixf)
        .ssd[PIXEL_8x16 as libc::c_int
        as usize] = Some(
        x264_pixel_ssd_8x16
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> libc::c_int,
    );
    (*pixf)
        .ssd[PIXEL_8x8 as libc::c_int
        as usize] = Some(
        x264_pixel_ssd_8x8
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> libc::c_int,
    );
    (*pixf)
        .ssd[PIXEL_8x4 as libc::c_int
        as usize] = Some(
        x264_pixel_ssd_8x4
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> libc::c_int,
    );
    (*pixf)
        .ssd[PIXEL_4x8 as libc::c_int
        as usize] = Some(
        x264_pixel_ssd_4x8
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> libc::c_int,
    );
    (*pixf)
        .ssd[PIXEL_4x4 as libc::c_int
        as usize] = Some(
        x264_pixel_ssd_4x4
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> libc::c_int,
    );
    (*pixf)
        .ssd[PIXEL_4x16 as libc::c_int
        as usize] = Some(
        x264_pixel_ssd_4x16
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> libc::c_int,
    );
    (*pixf)
        .satd[PIXEL_16x16 as libc::c_int
        as usize] = Some(
        x264_pixel_satd_16x16
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> libc::c_int,
    );
    (*pixf)
        .satd[PIXEL_16x8 as libc::c_int
        as usize] = Some(
        x264_pixel_satd_16x8
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> libc::c_int,
    );
    (*pixf)
        .satd[PIXEL_8x16 as libc::c_int
        as usize] = Some(
        x264_pixel_satd_8x16
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> libc::c_int,
    );
    (*pixf)
        .satd[PIXEL_8x8 as libc::c_int
        as usize] = Some(
        x264_pixel_satd_8x8
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> libc::c_int,
    );
    (*pixf)
        .satd[PIXEL_8x4 as libc::c_int
        as usize] = Some(
        x264_pixel_satd_8x4
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> libc::c_int,
    );
    (*pixf)
        .satd[PIXEL_4x8 as libc::c_int
        as usize] = Some(
        x264_pixel_satd_4x8
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> libc::c_int,
    );
    (*pixf)
        .satd[PIXEL_4x4 as libc::c_int
        as usize] = Some(
        x264_pixel_satd_4x4
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> libc::c_int,
    );
    (*pixf)
        .satd[PIXEL_4x16 as libc::c_int
        as usize] = Some(
        x264_pixel_satd_4x16
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> libc::c_int,
    );
    (*pixf)
        .satd_x3[PIXEL_16x16 as libc::c_int
        as usize] = Some(
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
    (*pixf)
        .satd_x3[PIXEL_16x8 as libc::c_int
        as usize] = Some(
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
    (*pixf)
        .satd_x3[PIXEL_8x16 as libc::c_int
        as usize] = Some(
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
    (*pixf)
        .satd_x3[PIXEL_8x8 as libc::c_int
        as usize] = Some(
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
    (*pixf)
        .satd_x3[PIXEL_8x4 as libc::c_int
        as usize] = Some(
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
    (*pixf)
        .satd_x3[PIXEL_4x8 as libc::c_int
        as usize] = Some(
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
    (*pixf)
        .satd_x3[PIXEL_4x4 as libc::c_int
        as usize] = Some(
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
    (*pixf)
        .satd_x4[PIXEL_16x16 as libc::c_int
        as usize] = Some(
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
    (*pixf)
        .satd_x4[PIXEL_16x8 as libc::c_int
        as usize] = Some(
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
    (*pixf)
        .satd_x4[PIXEL_8x16 as libc::c_int
        as usize] = Some(
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
    (*pixf)
        .satd_x4[PIXEL_8x8 as libc::c_int
        as usize] = Some(
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
    (*pixf)
        .satd_x4[PIXEL_8x4 as libc::c_int
        as usize] = Some(
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
    (*pixf)
        .satd_x4[PIXEL_4x8 as libc::c_int
        as usize] = Some(
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
    (*pixf)
        .satd_x4[PIXEL_4x4 as libc::c_int
        as usize] = Some(
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
    (*pixf)
        .hadamard_ac[PIXEL_16x16 as libc::c_int
        as usize] = Some(
        x264_pixel_hadamard_ac_16x16
            as unsafe extern "C" fn(*mut pixel, intptr_t) -> uint64_t,
    );
    (*pixf)
        .hadamard_ac[PIXEL_16x8 as libc::c_int
        as usize] = Some(
        x264_pixel_hadamard_ac_16x8
            as unsafe extern "C" fn(*mut pixel, intptr_t) -> uint64_t,
    );
    (*pixf)
        .hadamard_ac[PIXEL_8x16 as libc::c_int
        as usize] = Some(
        x264_pixel_hadamard_ac_8x16
            as unsafe extern "C" fn(*mut pixel, intptr_t) -> uint64_t,
    );
    (*pixf)
        .hadamard_ac[PIXEL_8x8 as libc::c_int
        as usize] = Some(
        x264_pixel_hadamard_ac_8x8
            as unsafe extern "C" fn(*mut pixel, intptr_t) -> uint64_t,
    );
    (*pixf)
        .ads[PIXEL_16x16 as libc::c_int
        as usize] = Some(
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
    (*pixf)
        .ads[PIXEL_16x8 as libc::c_int
        as usize] = Some(
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
    (*pixf)
        .ads[PIXEL_8x8 as libc::c_int
        as usize] = Some(
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
    (*pixf)
        .sa8d[PIXEL_16x16 as libc::c_int
        as usize] = Some(
        x264_pixel_sa8d_16x16
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> libc::c_int,
    );
    (*pixf)
        .sa8d[PIXEL_8x8 as libc::c_int
        as usize] = Some(
        x264_pixel_sa8d_8x8
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> libc::c_int,
    );
    (*pixf)
        .var[PIXEL_16x16 as libc::c_int
        as usize] = Some(
        pixel_var_16x16 as unsafe extern "C" fn(*mut pixel, intptr_t) -> uint64_t,
    );
    (*pixf)
        .var[PIXEL_8x16 as libc::c_int
        as usize] = Some(
        pixel_var_8x16 as unsafe extern "C" fn(*mut pixel, intptr_t) -> uint64_t,
    );
    (*pixf)
        .var[PIXEL_8x8 as libc::c_int
        as usize] = Some(
        pixel_var_8x8 as unsafe extern "C" fn(*mut pixel, intptr_t) -> uint64_t,
    );
    (*pixf)
        .var2[PIXEL_8x16 as libc::c_int
        as usize] = Some(
        pixel_var2_8x16
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut libc::c_int,
            ) -> libc::c_int,
    );
    (*pixf)
        .var2[PIXEL_8x8 as libc::c_int
        as usize] = Some(
        pixel_var2_8x8
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut libc::c_int,
            ) -> libc::c_int,
    );
    (*pixf)
        .ssd_nv12_core = Some(
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
    (*pixf)
        .ssim_4x4x2_core = Some(
        ssim_4x4x2_core
            as unsafe extern "C" fn(
                *const pixel,
                intptr_t,
                *const pixel,
                intptr_t,
                *mut [libc::c_int; 4],
            ) -> (),
    );
    (*pixf)
        .ssim_end4 = Some(
        ssim_end4
            as unsafe extern "C" fn(
                *mut [libc::c_int; 4],
                *mut [libc::c_int; 4],
                libc::c_int,
            ) -> libc::c_float,
    );
    (*pixf)
        .vsad = Some(
        pixel_vsad
            as unsafe extern "C" fn(*mut pixel, intptr_t, libc::c_int) -> libc::c_int,
    );
    (*pixf)
        .asd8 = Some(
        pixel_asd8
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                libc::c_int,
            ) -> libc::c_int,
    );
    (*pixf)
        .intra_sad_x3_4x4 = Some(
        intra_sad_x3_4x4
            as unsafe extern "C" fn(*mut pixel, *mut pixel, *mut libc::c_int) -> (),
    );
    (*pixf)
        .intra_satd_x3_4x4 = Some(
        intra_satd_x3_4x4
            as unsafe extern "C" fn(*mut pixel, *mut pixel, *mut libc::c_int) -> (),
    );
    (*pixf)
        .intra_sad_x3_8x8 = Some(
        intra_sad_x3_8x8
            as unsafe extern "C" fn(*mut pixel, *mut pixel, *mut libc::c_int) -> (),
    );
    (*pixf)
        .intra_sa8d_x3_8x8 = Some(
        intra_sa8d_x3_8x8
            as unsafe extern "C" fn(*mut pixel, *mut pixel, *mut libc::c_int) -> (),
    );
    (*pixf)
        .intra_sad_x3_8x8c = Some(
        intra_sad_x3_8x8c
            as unsafe extern "C" fn(*mut pixel, *mut pixel, *mut libc::c_int) -> (),
    );
    (*pixf)
        .intra_satd_x3_8x8c = Some(
        intra_satd_x3_8x8c
            as unsafe extern "C" fn(*mut pixel, *mut pixel, *mut libc::c_int) -> (),
    );
    (*pixf)
        .intra_sad_x3_8x16c = Some(
        intra_sad_x3_8x16c
            as unsafe extern "C" fn(*mut pixel, *mut pixel, *mut libc::c_int) -> (),
    );
    (*pixf)
        .intra_satd_x3_8x16c = Some(
        intra_satd_x3_8x16c
            as unsafe extern "C" fn(*mut pixel, *mut pixel, *mut libc::c_int) -> (),
    );
    (*pixf)
        .intra_sad_x3_16x16 = Some(
        intra_sad_x3_16x16
            as unsafe extern "C" fn(*mut pixel, *mut pixel, *mut libc::c_int) -> (),
    );
    (*pixf)
        .intra_satd_x3_16x16 = Some(
        intra_satd_x3_16x16
            as unsafe extern "C" fn(*mut pixel, *mut pixel, *mut libc::c_int) -> (),
    );
    (*pixf)
        .ads[PIXEL_4x8 as libc::c_int
        as usize] = (*pixf).ads[PIXEL_16x8 as libc::c_int as usize];
    (*pixf)
        .ads[PIXEL_8x4 as libc::c_int
        as usize] = (*pixf).ads[PIXEL_4x8 as libc::c_int as usize];
    (*pixf)
        .ads[PIXEL_8x16 as libc::c_int
        as usize] = (*pixf).ads[PIXEL_8x4 as libc::c_int as usize];
    (*pixf)
        .ads[PIXEL_4x4 as libc::c_int
        as usize] = (*pixf).ads[PIXEL_8x8 as libc::c_int as usize];
}
