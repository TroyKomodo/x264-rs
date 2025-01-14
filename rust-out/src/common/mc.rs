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
    static x264_hpel_ref0: [uint8_t; 16];
    static x264_hpel_ref1: [uint8_t; 16];
    fn x264_8_frame_expand_border_lowres(frame: *mut x264_frame_t);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union x264_union32_t {
    pub i: uint32_t,
    pub w: [uint16_t; 2],
    pub b: [uint8_t; 4],
}
pub type chroma_format_e = libc::c_uint;
pub const CHROMA_444: chroma_format_e = 3;
pub const CHROMA_422: chroma_format_e = 2;
pub const CHROMA_420: chroma_format_e = 1;
pub const CHROMA_400: chroma_format_e = 0;
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
#[inline(always)]
unsafe extern "C" fn x264_clip_pixel(mut x: libc::c_int) -> pixel {
    (if x & !(((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int) != 0 {
        (-x >> 31 as libc::c_int) & (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
    } else {
        x
    }) as pixel
}
#[inline(always)]
unsafe extern "C" fn endian_fix16(mut x: uint16_t) -> uint16_t {
    (((x as libc::c_int) << 8 as libc::c_int) | (x as libc::c_int >> 8 as libc::c_int)) as uint16_t
}
#[inline]
unsafe extern "C" fn pixel_avg(
    mut dst: *mut pixel,
    mut i_dst_stride: intptr_t,
    mut src1: *mut pixel,
    mut i_src1_stride: intptr_t,
    mut src2: *mut pixel,
    mut i_src2_stride: intptr_t,
    mut i_width: libc::c_int,
    mut i_height: libc::c_int,
) {
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < i_height {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < i_width {
            *dst
                .offset(
                    x as isize,
                ) = ((*src1.offset(x as isize) as libc::c_int
                + *src2.offset(x as isize) as libc::c_int + 1 as libc::c_int) >> 1 as libc::c_int) as pixel;
            x += 1;
            x;
        }
        dst = dst.offset(i_dst_stride as isize);
        src1 = src1.offset(i_src1_stride as isize);
        src2 = src2.offset(i_src2_stride as isize);
        y += 1;
        y;
    }
}
#[inline]
unsafe extern "C" fn pixel_avg_wxh(
    mut dst: *mut pixel,
    mut i_dst: intptr_t,
    mut src1: *mut pixel,
    mut i_src1: intptr_t,
    mut src2: *mut pixel,
    mut i_src2: intptr_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
) {
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < height {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < width {
            *dst
                .offset(
                    x as isize,
                ) = ((*src1.offset(x as isize) as libc::c_int
                + *src2.offset(x as isize) as libc::c_int + 1 as libc::c_int) >> 1 as libc::c_int) as pixel;
            x += 1;
            x;
        }
        src1 = src1.offset(i_src1 as isize);
        src2 = src2.offset(i_src2 as isize);
        dst = dst.offset(i_dst as isize);
        y += 1;
        y;
    }
}
#[inline]
unsafe extern "C" fn pixel_avg_weight_wxh(
    mut dst: *mut pixel,
    mut i_dst: intptr_t,
    mut src1: *mut pixel,
    mut i_src1: intptr_t,
    mut src2: *mut pixel,
    mut i_src2: intptr_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut i_weight1: libc::c_int,
) {
    let mut i_weight2: libc::c_int = 64 as libc::c_int - i_weight1;
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < height {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < width {
            *dst
                .offset(
                    x as isize,
                ) = x264_clip_pixel(
                (*src1.offset(x as isize) as libc::c_int * i_weight1
                    + *src2.offset(x as isize) as libc::c_int * i_weight2
                    + ((1 as libc::c_int) << 5 as libc::c_int)) >> 6 as libc::c_int,
            );
            x += 1;
            x;
        }
        y += 1;
        y;
        dst = dst.offset(i_dst as isize);
        src1 = src1.offset(i_src1 as isize);
        src2 = src2.offset(i_src2 as isize);
    }
}
unsafe extern "C" fn pixel_avg_16x16(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
    mut pix3: *mut pixel,
    mut i_stride_pix3: intptr_t,
    mut weight: libc::c_int,
) {
    if weight == 32 as libc::c_int {
        pixel_avg_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            16 as libc::c_int,
            16 as libc::c_int,
        );
    } else {
        pixel_avg_weight_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            16 as libc::c_int,
            16 as libc::c_int,
            weight,
        );
    };
}
unsafe extern "C" fn pixel_avg_16x8(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
    mut pix3: *mut pixel,
    mut i_stride_pix3: intptr_t,
    mut weight: libc::c_int,
) {
    if weight == 32 as libc::c_int {
        pixel_avg_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            16 as libc::c_int,
            8 as libc::c_int,
        );
    } else {
        pixel_avg_weight_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            16 as libc::c_int,
            8 as libc::c_int,
            weight,
        );
    };
}
unsafe extern "C" fn pixel_avg_8x16(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
    mut pix3: *mut pixel,
    mut i_stride_pix3: intptr_t,
    mut weight: libc::c_int,
) {
    if weight == 32 as libc::c_int {
        pixel_avg_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            8 as libc::c_int,
            16 as libc::c_int,
        );
    } else {
        pixel_avg_weight_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            8 as libc::c_int,
            16 as libc::c_int,
            weight,
        );
    };
}
unsafe extern "C" fn pixel_avg_8x8(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
    mut pix3: *mut pixel,
    mut i_stride_pix3: intptr_t,
    mut weight: libc::c_int,
) {
    if weight == 32 as libc::c_int {
        pixel_avg_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            8 as libc::c_int,
            8 as libc::c_int,
        );
    } else {
        pixel_avg_weight_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            8 as libc::c_int,
            8 as libc::c_int,
            weight,
        );
    };
}
unsafe extern "C" fn pixel_avg_8x4(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
    mut pix3: *mut pixel,
    mut i_stride_pix3: intptr_t,
    mut weight: libc::c_int,
) {
    if weight == 32 as libc::c_int {
        pixel_avg_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            8 as libc::c_int,
            4 as libc::c_int,
        );
    } else {
        pixel_avg_weight_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            8 as libc::c_int,
            4 as libc::c_int,
            weight,
        );
    };
}
unsafe extern "C" fn pixel_avg_4x16(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
    mut pix3: *mut pixel,
    mut i_stride_pix3: intptr_t,
    mut weight: libc::c_int,
) {
    if weight == 32 as libc::c_int {
        pixel_avg_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            4 as libc::c_int,
            16 as libc::c_int,
        );
    } else {
        pixel_avg_weight_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            4 as libc::c_int,
            16 as libc::c_int,
            weight,
        );
    };
}
unsafe extern "C" fn pixel_avg_4x8(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
    mut pix3: *mut pixel,
    mut i_stride_pix3: intptr_t,
    mut weight: libc::c_int,
) {
    if weight == 32 as libc::c_int {
        pixel_avg_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            4 as libc::c_int,
            8 as libc::c_int,
        );
    } else {
        pixel_avg_weight_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            4 as libc::c_int,
            8 as libc::c_int,
            weight,
        );
    };
}
unsafe extern "C" fn pixel_avg_4x4(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
    mut pix3: *mut pixel,
    mut i_stride_pix3: intptr_t,
    mut weight: libc::c_int,
) {
    if weight == 32 as libc::c_int {
        pixel_avg_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            4 as libc::c_int,
            4 as libc::c_int,
        );
    } else {
        pixel_avg_weight_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            4 as libc::c_int,
            4 as libc::c_int,
            weight,
        );
    };
}
unsafe extern "C" fn pixel_avg_4x2(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
    mut pix3: *mut pixel,
    mut i_stride_pix3: intptr_t,
    mut weight: libc::c_int,
) {
    if weight == 32 as libc::c_int {
        pixel_avg_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            4 as libc::c_int,
            2 as libc::c_int,
        );
    } else {
        pixel_avg_weight_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            4 as libc::c_int,
            2 as libc::c_int,
            weight,
        );
    };
}
unsafe extern "C" fn pixel_avg_2x8(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
    mut pix3: *mut pixel,
    mut i_stride_pix3: intptr_t,
    mut weight: libc::c_int,
) {
    if weight == 32 as libc::c_int {
        pixel_avg_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            2 as libc::c_int,
            8 as libc::c_int,
        );
    } else {
        pixel_avg_weight_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            2 as libc::c_int,
            8 as libc::c_int,
            weight,
        );
    };
}
unsafe extern "C" fn pixel_avg_2x4(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
    mut pix3: *mut pixel,
    mut i_stride_pix3: intptr_t,
    mut weight: libc::c_int,
) {
    if weight == 32 as libc::c_int {
        pixel_avg_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            2 as libc::c_int,
            4 as libc::c_int,
        );
    } else {
        pixel_avg_weight_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            2 as libc::c_int,
            4 as libc::c_int,
            weight,
        );
    };
}
unsafe extern "C" fn pixel_avg_2x2(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
    mut pix3: *mut pixel,
    mut i_stride_pix3: intptr_t,
    mut weight: libc::c_int,
) {
    if weight == 32 as libc::c_int {
        pixel_avg_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            2 as libc::c_int,
            2 as libc::c_int,
        );
    } else {
        pixel_avg_weight_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            2 as libc::c_int,
            2 as libc::c_int,
            weight,
        );
    };
}
unsafe extern "C" fn weight_cache(mut h: *mut x264_t, mut w: *mut x264_weight_t) {
    (*w).weightfn = (*h).mc.weight;
}
unsafe extern "C" fn mc_weight(
    mut dst: *mut pixel,
    mut i_dst_stride: intptr_t,
    mut src: *mut pixel,
    mut i_src_stride: intptr_t,
    mut weight: *const x264_weight_t,
    mut i_width: libc::c_int,
    mut i_height: libc::c_int,
) {
    let mut offset: libc::c_int = (*weight).i_offset
        * ((1 as libc::c_int) << (8 as libc::c_int - 8 as libc::c_int));
    let mut scale: libc::c_int = (*weight).i_scale;
    let mut denom: libc::c_int = (*weight).i_denom;
    if denom >= 1 as libc::c_int {
        let mut y: libc::c_int = 0 as libc::c_int;
        while y < i_height {
            let mut x: libc::c_int = 0 as libc::c_int;
            while x < i_width {
                *dst
                    .offset(
                        x as isize,
                    ) = x264_clip_pixel(
                    ((*src.offset(x as isize) as libc::c_int * scale
                        + ((1 as libc::c_int) << (denom - 1 as libc::c_int))) >> denom)
                        + offset,
                );
                x += 1;
                x;
            }
            y += 1;
            y;
            dst = dst.offset(i_dst_stride as isize);
            src = src.offset(i_src_stride as isize);
        }
    } else {
        let mut y_0: libc::c_int = 0 as libc::c_int;
        while y_0 < i_height {
            let mut x_0: libc::c_int = 0 as libc::c_int;
            while x_0 < i_width {
                *dst
                    .offset(
                        x_0 as isize,
                    ) = x264_clip_pixel(
                    *src.offset(x_0 as isize) as libc::c_int * scale + offset,
                );
                x_0 += 1;
                x_0;
            }
            y_0 += 1;
            y_0;
            dst = dst.offset(i_dst_stride as isize);
            src = src.offset(i_src_stride as isize);
        }
    };
}
unsafe extern "C" fn mc_weight_w20(
    mut dst: *mut pixel,
    mut i_dst_stride: intptr_t,
    mut src: *mut pixel,
    mut i_src_stride: intptr_t,
    mut weight: *const x264_weight_t,
    mut height: libc::c_int,
) {
    mc_weight(dst, i_dst_stride, src, i_src_stride, weight, 20 as libc::c_int, height);
}
unsafe extern "C" fn mc_weight_w16(
    mut dst: *mut pixel,
    mut i_dst_stride: intptr_t,
    mut src: *mut pixel,
    mut i_src_stride: intptr_t,
    mut weight: *const x264_weight_t,
    mut height: libc::c_int,
) {
    mc_weight(dst, i_dst_stride, src, i_src_stride, weight, 16 as libc::c_int, height);
}
unsafe extern "C" fn mc_weight_w12(
    mut dst: *mut pixel,
    mut i_dst_stride: intptr_t,
    mut src: *mut pixel,
    mut i_src_stride: intptr_t,
    mut weight: *const x264_weight_t,
    mut height: libc::c_int,
) {
    mc_weight(dst, i_dst_stride, src, i_src_stride, weight, 12 as libc::c_int, height);
}
unsafe extern "C" fn mc_weight_w8(
    mut dst: *mut pixel,
    mut i_dst_stride: intptr_t,
    mut src: *mut pixel,
    mut i_src_stride: intptr_t,
    mut weight: *const x264_weight_t,
    mut height: libc::c_int,
) {
    mc_weight(dst, i_dst_stride, src, i_src_stride, weight, 8 as libc::c_int, height);
}
unsafe extern "C" fn mc_weight_w4(
    mut dst: *mut pixel,
    mut i_dst_stride: intptr_t,
    mut src: *mut pixel,
    mut i_src_stride: intptr_t,
    mut weight: *const x264_weight_t,
    mut height: libc::c_int,
) {
    mc_weight(dst, i_dst_stride, src, i_src_stride, weight, 4 as libc::c_int, height);
}
unsafe extern "C" fn mc_weight_w2(
    mut dst: *mut pixel,
    mut i_dst_stride: intptr_t,
    mut src: *mut pixel,
    mut i_src_stride: intptr_t,
    mut weight: *const x264_weight_t,
    mut height: libc::c_int,
) {
    mc_weight(dst, i_dst_stride, src, i_src_stride, weight, 2 as libc::c_int, height);
}
static mut mc_weight_wtab: [weight_fn_t; 6] = unsafe {
    [
        Some(
            mc_weight_w2
                as unsafe extern "C" fn(
                    *mut pixel,
                    intptr_t,
                    *mut pixel,
                    intptr_t,
                    *const x264_weight_t,
                    libc::c_int,
                ) -> (),
        ),
        Some(
            mc_weight_w4
                as unsafe extern "C" fn(
                    *mut pixel,
                    intptr_t,
                    *mut pixel,
                    intptr_t,
                    *const x264_weight_t,
                    libc::c_int,
                ) -> (),
        ),
        Some(
            mc_weight_w8
                as unsafe extern "C" fn(
                    *mut pixel,
                    intptr_t,
                    *mut pixel,
                    intptr_t,
                    *const x264_weight_t,
                    libc::c_int,
                ) -> (),
        ),
        Some(
            mc_weight_w12
                as unsafe extern "C" fn(
                    *mut pixel,
                    intptr_t,
                    *mut pixel,
                    intptr_t,
                    *const x264_weight_t,
                    libc::c_int,
                ) -> (),
        ),
        Some(
            mc_weight_w16
                as unsafe extern "C" fn(
                    *mut pixel,
                    intptr_t,
                    *mut pixel,
                    intptr_t,
                    *const x264_weight_t,
                    libc::c_int,
                ) -> (),
        ),
        Some(
            mc_weight_w20
                as unsafe extern "C" fn(
                    *mut pixel,
                    intptr_t,
                    *mut pixel,
                    intptr_t,
                    *const x264_weight_t,
                    libc::c_int,
                ) -> (),
        ),
    ]
};
unsafe extern "C" fn mc_copy(
    mut src: *mut pixel,
    mut i_src_stride: intptr_t,
    mut dst: *mut pixel,
    mut i_dst_stride: intptr_t,
    mut i_width: libc::c_int,
    mut i_height: libc::c_int,
) {
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < i_height {
        memcpy(
            dst as *mut libc::c_void,
            src as *const libc::c_void,
            (i_width * ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int)
                as libc::c_ulong,
        );
        src = src.offset(i_src_stride as isize);
        dst = dst.offset(i_dst_stride as isize);
        y += 1;
        y;
    }
}
unsafe extern "C" fn hpel_filter(
    mut dsth: *mut pixel,
    mut dstv: *mut pixel,
    mut dstc: *mut pixel,
    mut src: *mut pixel,
    mut stride: intptr_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut buf: *mut int16_t,
) {
    let pad: libc::c_int = if 8 as libc::c_int > 9 as libc::c_int {
        -(10 as libc::c_int)
            * (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
    } else {
        0 as libc::c_int
    };
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < height {
        let mut x: libc::c_int = -(2 as libc::c_int);
        while x < width + 3 as libc::c_int {
            let mut v: libc::c_int = *src
                .offset((x as intptr_t - 2 as libc::c_int as intptr_t * stride) as isize)
                as libc::c_int
                + *src
                    .offset(
                        (x as intptr_t + 3 as libc::c_int as intptr_t * stride) as isize,
                    ) as libc::c_int
                - 5 as libc::c_int
                    * (*src.offset((x as intptr_t - stride) as isize) as libc::c_int
                        + *src
                            .offset(
                                (x as intptr_t + 2 as libc::c_int as intptr_t * stride)
                                    as isize,
                            ) as libc::c_int)
                + 20 as libc::c_int
                    * (*src.offset(x as isize) as libc::c_int
                        + *src.offset((x as intptr_t + stride) as isize) as libc::c_int);
            *dstv
                .offset(
                    x as isize,
                ) = x264_clip_pixel((v + 16 as libc::c_int) >> 5 as libc::c_int);
            *buf.offset((x + 2 as libc::c_int) as isize) = (v + pad) as int16_t;
            x += 1;
            x;
        }
        let mut x_0: libc::c_int = 0 as libc::c_int;
        while x_0 < width {
            *dstc
                .offset(
                    x_0 as isize,
                ) = x264_clip_pixel(
                (*buf
                    .offset(2 as libc::c_int as isize)
                    .offset((x_0 - 2 as libc::c_int * 1 as libc::c_int) as isize)
                    as libc::c_int
                    + *buf
                        .offset(2 as libc::c_int as isize)
                        .offset((x_0 + 3 as libc::c_int * 1 as libc::c_int) as isize)
                        as libc::c_int
                    - 5 as libc::c_int
                        * (*buf
                            .offset(2 as libc::c_int as isize)
                            .offset((x_0 - 1 as libc::c_int) as isize) as libc::c_int
                            + *buf
                                .offset(2 as libc::c_int as isize)
                                .offset(
                                    (x_0 + 2 as libc::c_int * 1 as libc::c_int) as isize,
                                ) as libc::c_int)
                    + 20 as libc::c_int
                        * (*buf.offset(2 as libc::c_int as isize).offset(x_0 as isize)
                            as libc::c_int
                            + *buf
                                .offset(2 as libc::c_int as isize)
                                .offset((x_0 + 1 as libc::c_int) as isize) as libc::c_int)
                    - 32 as libc::c_int * pad + 512 as libc::c_int) >> 10 as libc::c_int,
            );
            x_0 += 1;
            x_0;
        }
        let mut x_1: libc::c_int = 0 as libc::c_int;
        while x_1 < width {
            *dsth
                .offset(
                    x_1 as isize,
                ) = x264_clip_pixel(
                (*src.offset((x_1 - 2 as libc::c_int * 1 as libc::c_int) as isize)
                    as libc::c_int
                    + *src.offset((x_1 + 3 as libc::c_int * 1 as libc::c_int) as isize)
                        as libc::c_int
                    - 5 as libc::c_int
                        * (*src.offset((x_1 - 1 as libc::c_int) as isize) as libc::c_int
                            + *src
                                .offset(
                                    (x_1 + 2 as libc::c_int * 1 as libc::c_int) as isize,
                                ) as libc::c_int)
                    + 20 as libc::c_int
                        * (*src.offset(x_1 as isize) as libc::c_int
                            + *src.offset((x_1 + 1 as libc::c_int) as isize)
                                as libc::c_int) + 16 as libc::c_int) >> 5 as libc::c_int,
            );
            x_1 += 1;
            x_1;
        }
        dsth = dsth.offset(stride as isize);
        dstv = dstv.offset(stride as isize);
        dstc = dstc.offset(stride as isize);
        src = src.offset(stride as isize);
        y += 1;
        y;
    }
}
unsafe extern "C" fn mc_luma(
    mut dst: *mut pixel,
    mut i_dst_stride: intptr_t,
    mut src: *mut *mut pixel,
    mut i_src_stride: intptr_t,
    mut mvx: libc::c_int,
    mut mvy: libc::c_int,
    mut i_width: libc::c_int,
    mut i_height: libc::c_int,
    mut weight: *const x264_weight_t,
) {
    let mut qpel_idx: libc::c_int = ((mvy & 3 as libc::c_int) << 2 as libc::c_int)
        + (mvx & 3 as libc::c_int);
    let mut offset: libc::c_int = ((mvy >> 2 as libc::c_int) as intptr_t * i_src_stride
        + (mvx >> 2 as libc::c_int) as intptr_t) as libc::c_int;
    let mut src1: *mut pixel = (*src.offset(x264_hpel_ref0[qpel_idx as usize] as isize))
        .offset(offset as isize)
        .offset(
            ((mvy & 3 as libc::c_int == 3 as libc::c_int) as libc::c_int as intptr_t
                * i_src_stride) as isize,
        );
    if qpel_idx & 5 as libc::c_int != 0 {
        let mut src2: *mut pixel = (*src
            .offset(x264_hpel_ref1[qpel_idx as usize] as isize))
            .offset(offset as isize)
            .offset(
                (mvx & 3 as libc::c_int == 3 as libc::c_int) as libc::c_int as isize,
            );
        pixel_avg(
            dst,
            i_dst_stride,
            src1,
            i_src_stride,
            src2,
            i_src_stride,
            i_width,
            i_height,
        );
        if !((*weight).weightfn).is_null() {
            mc_weight(dst, i_dst_stride, dst, i_dst_stride, weight, i_width, i_height);
        }
    } else if !((*weight).weightfn).is_null() {
        mc_weight(dst, i_dst_stride, src1, i_src_stride, weight, i_width, i_height);
    } else {
        mc_copy(src1, i_src_stride, dst, i_dst_stride, i_width, i_height);
    };
}
unsafe extern "C" fn get_ref(
    mut dst: *mut pixel,
    mut i_dst_stride: *mut intptr_t,
    mut src: *mut *mut pixel,
    mut i_src_stride: intptr_t,
    mut mvx: libc::c_int,
    mut mvy: libc::c_int,
    mut i_width: libc::c_int,
    mut i_height: libc::c_int,
    mut weight: *const x264_weight_t,
) -> *mut pixel {
    let mut qpel_idx: libc::c_int = ((mvy & 3 as libc::c_int) << 2 as libc::c_int)
        + (mvx & 3 as libc::c_int);
    let mut offset: libc::c_int = ((mvy >> 2 as libc::c_int) as intptr_t * i_src_stride
        + (mvx >> 2 as libc::c_int) as intptr_t) as libc::c_int;
    let mut src1: *mut pixel = (*src.offset(x264_hpel_ref0[qpel_idx as usize] as isize))
        .offset(offset as isize)
        .offset(
            ((mvy & 3 as libc::c_int == 3 as libc::c_int) as libc::c_int as intptr_t
                * i_src_stride) as isize,
        );
    if qpel_idx & 5 as libc::c_int != 0 {
        let mut src2: *mut pixel = (*src
            .offset(x264_hpel_ref1[qpel_idx as usize] as isize))
            .offset(offset as isize)
            .offset(
                (mvx & 3 as libc::c_int == 3 as libc::c_int) as libc::c_int as isize,
            );
        pixel_avg(
            dst,
            *i_dst_stride,
            src1,
            i_src_stride,
            src2,
            i_src_stride,
            i_width,
            i_height,
        );
        if !((*weight).weightfn).is_null() {
            mc_weight(dst, *i_dst_stride, dst, *i_dst_stride, weight, i_width, i_height);
        }
        dst
    } else if !((*weight).weightfn).is_null() {
        mc_weight(dst, *i_dst_stride, src1, i_src_stride, weight, i_width, i_height);
        return dst;
    } else {
        *i_dst_stride = i_src_stride;
        return src1;
    }
}
unsafe extern "C" fn mc_chroma(
    mut dstu: *mut pixel,
    mut dstv: *mut pixel,
    mut i_dst_stride: intptr_t,
    mut src: *mut pixel,
    mut i_src_stride: intptr_t,
    mut mvx: libc::c_int,
    mut mvy: libc::c_int,
    mut i_width: libc::c_int,
    mut i_height: libc::c_int,
) {
    let mut srcp: *mut pixel = std::ptr::null_mut::<pixel>();
    let mut d8x: libc::c_int = mvx & 0x7 as libc::c_int;
    let mut d8y: libc::c_int = mvy & 0x7 as libc::c_int;
    let mut cA: libc::c_int = (8 as libc::c_int - d8x) * (8 as libc::c_int - d8y);
    let mut cB: libc::c_int = d8x * (8 as libc::c_int - d8y);
    let mut cC: libc::c_int = (8 as libc::c_int - d8x) * d8y;
    let mut cD: libc::c_int = d8x * d8y;
    src = src
        .offset(
            ((mvy >> 3 as libc::c_int) as intptr_t * i_src_stride
                + ((mvx >> 3 as libc::c_int) * 2 as libc::c_int) as intptr_t) as isize,
        );
    srcp = &mut *src.offset(i_src_stride as isize) as *mut pixel;
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < i_height {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < i_width {
            *dstu
                .offset(
                    x as isize,
                ) = ((cA * *src.offset((2 as libc::c_int * x) as isize) as libc::c_int
                + cB
                    * *src.offset((2 as libc::c_int * x + 2 as libc::c_int) as isize)
                        as libc::c_int
                + cC * *srcp.offset((2 as libc::c_int * x) as isize) as libc::c_int
                + cD
                    * *srcp.offset((2 as libc::c_int * x + 2 as libc::c_int) as isize)
                        as libc::c_int + 32 as libc::c_int) >> 6 as libc::c_int) as pixel;
            *dstv
                .offset(
                    x as isize,
                ) = ((cA
                * *src.offset((2 as libc::c_int * x + 1 as libc::c_int) as isize)
                    as libc::c_int
                + cB
                    * *src.offset((2 as libc::c_int * x + 3 as libc::c_int) as isize)
                        as libc::c_int
                + cC
                    * *srcp.offset((2 as libc::c_int * x + 1 as libc::c_int) as isize)
                        as libc::c_int
                + cD
                    * *srcp.offset((2 as libc::c_int * x + 3 as libc::c_int) as isize)
                        as libc::c_int + 32 as libc::c_int) >> 6 as libc::c_int) as pixel;
            x += 1;
            x;
        }
        dstu = dstu.offset(i_dst_stride as isize);
        dstv = dstv.offset(i_dst_stride as isize);
        src = srcp;
        srcp = srcp.offset(i_src_stride as isize);
        y += 1;
        y;
    }
}
unsafe extern "C" fn mc_copy_w16(
    mut dst: *mut pixel,
    mut i_dst: intptr_t,
    mut src: *mut pixel,
    mut i_src: intptr_t,
    mut i_height: libc::c_int,
) {
    mc_copy(src, i_src, dst, i_dst, 16 as libc::c_int, i_height);
}
unsafe extern "C" fn mc_copy_w8(
    mut dst: *mut pixel,
    mut i_dst: intptr_t,
    mut src: *mut pixel,
    mut i_src: intptr_t,
    mut i_height: libc::c_int,
) {
    mc_copy(src, i_src, dst, i_dst, 8 as libc::c_int, i_height);
}
unsafe extern "C" fn mc_copy_w4(
    mut dst: *mut pixel,
    mut i_dst: intptr_t,
    mut src: *mut pixel,
    mut i_src: intptr_t,
    mut i_height: libc::c_int,
) {
    mc_copy(src, i_src, dst, i_dst, 4 as libc::c_int, i_height);
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_plane_copy_c(
    mut dst: *mut pixel,
    mut i_dst: intptr_t,
    mut src: *mut pixel,
    mut i_src: intptr_t,
    mut w: libc::c_int,
    mut h: libc::c_int,
) {
    loop {
        let fresh0 = h;
        h -= 1;
        if fresh0 == 0 {
            break;
        }
        memcpy(
            dst as *mut libc::c_void,
            src as *const libc::c_void,
            (w * ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int)
                as libc::c_ulong,
        );
        dst = dst.offset(i_dst as isize);
        src = src.offset(i_src as isize);
    };
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_plane_copy_swap_c(
    mut dst: *mut pixel,
    mut i_dst: intptr_t,
    mut src: *mut pixel,
    mut i_src: intptr_t,
    mut w: libc::c_int,
    mut h: libc::c_int,
) {
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < h {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < 2 as libc::c_int * w {
            *dst.offset(x as isize) = *src.offset((x + 1 as libc::c_int) as isize);
            *dst.offset((x + 1 as libc::c_int) as isize) = *src.offset(x as isize);
            x += 2 as libc::c_int;
        }
        y += 1;
        y;
        dst = dst.offset(i_dst as isize);
        src = src.offset(i_src as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_plane_copy_interleave_c(
    mut dst: *mut pixel,
    mut i_dst: intptr_t,
    mut srcu: *mut pixel,
    mut i_srcu: intptr_t,
    mut srcv: *mut pixel,
    mut i_srcv: intptr_t,
    mut w: libc::c_int,
    mut h: libc::c_int,
) {
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < h {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < w {
            *dst.offset((2 as libc::c_int * x) as isize) = *srcu.offset(x as isize);
            *dst
                .offset(
                    (2 as libc::c_int * x + 1 as libc::c_int) as isize,
                ) = *srcv.offset(x as isize);
            x += 1;
            x;
        }
        y += 1;
        y;
        dst = dst.offset(i_dst as isize);
        srcu = srcu.offset(i_srcu as isize);
        srcv = srcv.offset(i_srcv as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_plane_copy_deinterleave_c(
    mut dsta: *mut pixel,
    mut i_dsta: intptr_t,
    mut dstb: *mut pixel,
    mut i_dstb: intptr_t,
    mut src: *mut pixel,
    mut i_src: intptr_t,
    mut w: libc::c_int,
    mut h: libc::c_int,
) {
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < h {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < w {
            *dsta.offset(x as isize) = *src.offset((2 as libc::c_int * x) as isize);
            *dstb
                .offset(
                    x as isize,
                ) = *src.offset((2 as libc::c_int * x + 1 as libc::c_int) as isize);
            x += 1;
            x;
        }
        y += 1;
        y;
        dsta = dsta.offset(i_dsta as isize);
        dstb = dstb.offset(i_dstb as isize);
        src = src.offset(i_src as isize);
    }
}
unsafe extern "C" fn plane_copy_deinterleave_rgb_c(
    mut dsta: *mut pixel,
    mut i_dsta: intptr_t,
    mut dstb: *mut pixel,
    mut i_dstb: intptr_t,
    mut dstc: *mut pixel,
    mut i_dstc: intptr_t,
    mut src: *mut pixel,
    mut i_src: intptr_t,
    mut pw: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
) {
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < h {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < w {
            *dsta.offset(x as isize) = *src.offset((x * pw) as isize);
            *dstb.offset(x as isize) = *src.offset((x * pw + 1 as libc::c_int) as isize);
            *dstc.offset(x as isize) = *src.offset((x * pw + 2 as libc::c_int) as isize);
            x += 1;
            x;
        }
        y += 1;
        y;
        dsta = dsta.offset(i_dsta as isize);
        dstb = dstb.offset(i_dstb as isize);
        dstc = dstc.offset(i_dstc as isize);
        src = src.offset(i_src as isize);
    }
}
unsafe extern "C" fn plane_copy_deinterleave_v210_c(
    mut dsty: *mut pixel,
    mut i_dsty: intptr_t,
    mut dstc: *mut pixel,
    mut i_dstc: intptr_t,
    mut src: *mut uint32_t,
    mut i_src: intptr_t,
    mut w: libc::c_int,
    mut h: libc::c_int,
) {
    let mut l: libc::c_int = 0 as libc::c_int;
    while l < h {
        let mut dsty0: *mut pixel = dsty;
        let mut dstc0: *mut pixel = dstc;
        let mut src0: *mut uint32_t = src;
        let mut n: libc::c_int = 0 as libc::c_int;
        while n < w {
            let fresh1 = src0;
            src0 = src0.offset(1);
            let mut s: uint32_t = *fresh1;
            let fresh2 = dstc0;
            dstc0 = dstc0.offset(1);
            *fresh2 = (s & 0x3ff as libc::c_int as uint32_t) as pixel;
            let fresh3 = dsty0;
            dsty0 = dsty0.offset(1);
            *fresh3 = ((s >> 10 as libc::c_int) & 0x3ff as libc::c_int as uint32_t)
                as pixel;
            let fresh4 = dstc0;
            dstc0 = dstc0.offset(1);
            *fresh4 = ((s >> 20 as libc::c_int) & 0x3ff as libc::c_int as uint32_t)
                as pixel;
            let fresh5 = src0;
            src0 = src0.offset(1);
            s = *fresh5;
            let fresh6 = dsty0;
            dsty0 = dsty0.offset(1);
            *fresh6 = (s & 0x3ff as libc::c_int as uint32_t) as pixel;
            let fresh7 = dstc0;
            dstc0 = dstc0.offset(1);
            *fresh7 = ((s >> 10 as libc::c_int) & 0x3ff as libc::c_int as uint32_t)
                as pixel;
            let fresh8 = dsty0;
            dsty0 = dsty0.offset(1);
            *fresh8 = ((s >> 20 as libc::c_int) & 0x3ff as libc::c_int as uint32_t)
                as pixel;
            n += 3 as libc::c_int;
        }
        dsty = dsty.offset(i_dsty as isize);
        dstc = dstc.offset(i_dstc as isize);
        src = src.offset(i_src as isize);
        l += 1;
        l;
    }
}
unsafe extern "C" fn store_interleave_chroma(
    mut dst: *mut pixel,
    mut i_dst: intptr_t,
    mut srcu: *mut pixel,
    mut srcv: *mut pixel,
    mut height: libc::c_int,
) {
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < height {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < 8 as libc::c_int {
            *dst.offset((2 as libc::c_int * x) as isize) = *srcu.offset(x as isize);
            *dst
                .offset(
                    (2 as libc::c_int * x + 1 as libc::c_int) as isize,
                ) = *srcv.offset(x as isize);
            x += 1;
            x;
        }
        y += 1;
        y;
        dst = dst.offset(i_dst as isize);
        srcu = srcu.offset(32 as libc::c_int as isize);
        srcv = srcv.offset(32 as libc::c_int as isize);
    }
}
unsafe extern "C" fn load_deinterleave_chroma_fenc(
    mut dst: *mut pixel,
    mut src: *mut pixel,
    mut i_src: intptr_t,
    mut height: libc::c_int,
) {
    x264_8_plane_copy_deinterleave_c(
        dst,
        16 as libc::c_int as intptr_t,
        dst.offset((16 as libc::c_int / 2 as libc::c_int) as isize),
        16 as libc::c_int as intptr_t,
        src,
        i_src,
        8 as libc::c_int,
        height,
    );
}
unsafe extern "C" fn load_deinterleave_chroma_fdec(
    mut dst: *mut pixel,
    mut src: *mut pixel,
    mut i_src: intptr_t,
    mut height: libc::c_int,
) {
    x264_8_plane_copy_deinterleave_c(
        dst,
        32 as libc::c_int as intptr_t,
        dst.offset((32 as libc::c_int / 2 as libc::c_int) as isize),
        32 as libc::c_int as intptr_t,
        src,
        i_src,
        8 as libc::c_int,
        height,
    );
}
unsafe extern "C" fn prefetch_fenc_null(
    mut pix_y: *mut pixel,
    mut stride_y: intptr_t,
    mut pix_uv: *mut pixel,
    mut stride_uv: intptr_t,
    mut mb_x: libc::c_int,
) {}
unsafe extern "C" fn prefetch_ref_null(
    mut pix: *mut pixel,
    mut stride: intptr_t,
    mut parity: libc::c_int,
) {}
unsafe extern "C" fn memzero_aligned(mut dst: *mut libc::c_void, mut n: size_t) {
    memset(dst, 0 as libc::c_int, n);
}
unsafe extern "C" fn integral_init4h(
    mut sum: *mut uint16_t,
    mut pix: *mut pixel,
    mut stride: intptr_t,
) {
    let mut v: libc::c_int = *pix.offset(0 as libc::c_int as isize) as libc::c_int
        + *pix.offset(1 as libc::c_int as isize) as libc::c_int
        + *pix.offset(2 as libc::c_int as isize) as libc::c_int
        + *pix.offset(3 as libc::c_int as isize) as libc::c_int;
    let mut x: libc::c_int = 0 as libc::c_int;
    while (x as intptr_t) < stride - 4 as libc::c_int as intptr_t {
        *sum
            .offset(
                x as isize,
            ) = (v + *sum.offset((x as intptr_t - stride) as isize) as libc::c_int)
            as uint16_t;
        v
            += *pix.offset((x + 4 as libc::c_int) as isize) as libc::c_int
                - *pix.offset(x as isize) as libc::c_int;
        x += 1;
        x;
    }
}
unsafe extern "C" fn integral_init8h(
    mut sum: *mut uint16_t,
    mut pix: *mut pixel,
    mut stride: intptr_t,
) {
    let mut v: libc::c_int = *pix.offset(0 as libc::c_int as isize) as libc::c_int
        + *pix.offset(1 as libc::c_int as isize) as libc::c_int
        + *pix.offset(2 as libc::c_int as isize) as libc::c_int
        + *pix.offset(3 as libc::c_int as isize) as libc::c_int
        + *pix.offset(4 as libc::c_int as isize) as libc::c_int
        + *pix.offset(5 as libc::c_int as isize) as libc::c_int
        + *pix.offset(6 as libc::c_int as isize) as libc::c_int
        + *pix.offset(7 as libc::c_int as isize) as libc::c_int;
    let mut x: libc::c_int = 0 as libc::c_int;
    while (x as intptr_t) < stride - 8 as libc::c_int as intptr_t {
        *sum
            .offset(
                x as isize,
            ) = (v + *sum.offset((x as intptr_t - stride) as isize) as libc::c_int)
            as uint16_t;
        v
            += *pix.offset((x + 8 as libc::c_int) as isize) as libc::c_int
                - *pix.offset(x as isize) as libc::c_int;
        x += 1;
        x;
    }
}
unsafe extern "C" fn integral_init4v(
    mut sum8: *mut uint16_t,
    mut sum4: *mut uint16_t,
    mut stride: intptr_t,
) {
    let mut x: libc::c_int = 0 as libc::c_int;
    while (x as intptr_t) < stride - 8 as libc::c_int as intptr_t {
        *sum4
            .offset(
                x as isize,
            ) = (*sum8
            .offset((x as intptr_t + 4 as libc::c_int as intptr_t * stride) as isize)
            as libc::c_int - *sum8.offset(x as isize) as libc::c_int) as uint16_t;
        x += 1;
        x;
    }
    let mut x_0: libc::c_int = 0 as libc::c_int;
    while (x_0 as intptr_t) < stride - 8 as libc::c_int as intptr_t {
        *sum8
            .offset(
                x_0 as isize,
            ) = (*sum8
            .offset((x_0 as intptr_t + 8 as libc::c_int as intptr_t * stride) as isize)
            as libc::c_int
            + *sum8
                .offset(
                    (x_0 as intptr_t + 8 as libc::c_int as intptr_t * stride
                        + 4 as libc::c_int as intptr_t) as isize,
                ) as libc::c_int - *sum8.offset(x_0 as isize) as libc::c_int
            - *sum8.offset((x_0 + 4 as libc::c_int) as isize) as libc::c_int)
            as uint16_t;
        x_0 += 1;
        x_0;
    }
}
unsafe extern "C" fn integral_init8v(mut sum8: *mut uint16_t, mut stride: intptr_t) {
    let mut x: libc::c_int = 0 as libc::c_int;
    while (x as intptr_t) < stride - 8 as libc::c_int as intptr_t {
        *sum8
            .offset(
                x as isize,
            ) = (*sum8
            .offset((x as intptr_t + 8 as libc::c_int as intptr_t * stride) as isize)
            as libc::c_int - *sum8.offset(x as isize) as libc::c_int) as uint16_t;
        x += 1;
        x;
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_frame_init_lowres(
    mut h: *mut x264_t,
    mut frame: *mut x264_frame_t,
) {
    let mut src: *mut pixel = (*frame).plane[0 as libc::c_int as usize];
    let mut i_stride: libc::c_int = (*frame).i_stride[0 as libc::c_int as usize];
    let mut i_height: libc::c_int = (*frame).i_lines[0 as libc::c_int as usize];
    let mut i_width: libc::c_int = (*frame).i_width[0 as libc::c_int as usize];
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < i_height {
        *src
            .offset(
                (i_width + y * i_stride) as isize,
            ) = *src.offset((i_width - 1 as libc::c_int + y * i_stride) as isize);
        y += 1;
        y;
    }
    memcpy(
        src.offset((i_stride * i_height) as isize) as *mut libc::c_void,
        src.offset((i_stride * (i_height - 1 as libc::c_int)) as isize)
            as *const libc::c_void,
        ((i_width + 1 as libc::c_int)
            * ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int)
            as libc::c_ulong,
    );
    ((*h).mc.frame_init_lowres_core)
        .expect(
            "non-null function pointer",
        )(
        src,
        (*frame).lowres[0 as libc::c_int as usize],
        (*frame).lowres[1 as libc::c_int as usize],
        (*frame).lowres[2 as libc::c_int as usize],
        (*frame).lowres[3 as libc::c_int as usize],
        i_stride as intptr_t,
        (*frame).i_stride_lowres as intptr_t,
        (*frame).i_width_lowres,
        (*frame).i_lines_lowres,
    );
    x264_8_frame_expand_border_lowres(frame);
    memset(
        ((*frame).i_cost_est).as_mut_ptr() as *mut libc::c_void,
        -(1 as libc::c_int),
        ::core::mem::size_of::<[[libc::c_int; 18]; 18]>() as libc::c_ulong,
    );
    let mut y_0: libc::c_int = 0 as libc::c_int;
    while y_0 < (*h).param.i_bframe + 2 as libc::c_int {
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < (*h).param.i_bframe + 2 as libc::c_int {
            *((*frame).i_row_satds[y_0 as usize][x as usize])
                .offset(0 as libc::c_int as isize) = -(1 as libc::c_int);
            x += 1;
            x;
        }
        y_0 += 1;
        y_0;
    }
    let mut y_1: libc::c_int = 0 as libc::c_int;
    while y_1 <= ((*h).param.i_bframe != 0) as libc::c_int {
        let mut x_0: libc::c_int = 0 as libc::c_int;
        while x_0 <= (*h).param.i_bframe {
            (*((*frame).lowres_mvs[y_1 as usize][x_0 as usize])
                .offset(
                    0 as libc::c_int as isize,
                ))[0 as libc::c_int as usize] = 0x7fff as libc::c_int as int16_t;
            x_0 += 1;
            x_0;
        }
        y_1 += 1;
        y_1;
    }
}
unsafe extern "C" fn frame_init_lowres_core(
    mut src0: *mut pixel,
    mut dst0: *mut pixel,
    mut dsth: *mut pixel,
    mut dstv: *mut pixel,
    mut dstc: *mut pixel,
    mut src_stride: intptr_t,
    mut dst_stride: intptr_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
) {
    let mut y: libc::c_int = 0 as libc::c_int;
    while y < height {
        let mut src1: *mut pixel = src0.offset(src_stride as isize);
        let mut src2: *mut pixel = src1.offset(src_stride as isize);
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < width {
            *dst0
                .offset(
                    x as isize,
                ) = ((((*src0.offset((2 as libc::c_int * x) as isize) as libc::c_int
                + *src1.offset((2 as libc::c_int * x) as isize) as libc::c_int
                + 1 as libc::c_int) >> 1 as libc::c_int)
                + ((*src0.offset((2 as libc::c_int * x + 1 as libc::c_int) as isize)
                    as libc::c_int
                    + *src1.offset((2 as libc::c_int * x + 1 as libc::c_int) as isize)
                        as libc::c_int + 1 as libc::c_int) >> 1 as libc::c_int)
                + 1 as libc::c_int) >> 1 as libc::c_int) as pixel;
            *dsth
                .offset(
                    x as isize,
                ) = ((((*src0.offset((2 as libc::c_int * x + 1 as libc::c_int) as isize)
                as libc::c_int
                + *src1.offset((2 as libc::c_int * x + 1 as libc::c_int) as isize)
                    as libc::c_int + 1 as libc::c_int) >> 1 as libc::c_int)
                + ((*src0.offset((2 as libc::c_int * x + 2 as libc::c_int) as isize)
                    as libc::c_int
                    + *src1.offset((2 as libc::c_int * x + 2 as libc::c_int) as isize)
                        as libc::c_int + 1 as libc::c_int) >> 1 as libc::c_int)
                + 1 as libc::c_int) >> 1 as libc::c_int) as pixel;
            *dstv
                .offset(
                    x as isize,
                ) = ((((*src1.offset((2 as libc::c_int * x) as isize) as libc::c_int
                + *src2.offset((2 as libc::c_int * x) as isize) as libc::c_int
                + 1 as libc::c_int) >> 1 as libc::c_int)
                + ((*src1.offset((2 as libc::c_int * x + 1 as libc::c_int) as isize)
                    as libc::c_int
                    + *src2.offset((2 as libc::c_int * x + 1 as libc::c_int) as isize)
                        as libc::c_int + 1 as libc::c_int) >> 1 as libc::c_int)
                + 1 as libc::c_int) >> 1 as libc::c_int) as pixel;
            *dstc
                .offset(
                    x as isize,
                ) = ((((*src1.offset((2 as libc::c_int * x + 1 as libc::c_int) as isize)
                as libc::c_int
                + *src2.offset((2 as libc::c_int * x + 1 as libc::c_int) as isize)
                    as libc::c_int + 1 as libc::c_int) >> 1 as libc::c_int)
                + ((*src1.offset((2 as libc::c_int * x + 2 as libc::c_int) as isize)
                    as libc::c_int
                    + *src2.offset((2 as libc::c_int * x + 2 as libc::c_int) as isize)
                        as libc::c_int + 1 as libc::c_int) >> 1 as libc::c_int)
                + 1 as libc::c_int) >> 1 as libc::c_int) as pixel;
            x += 1;
            x;
        }
        src0 = src0.offset((src_stride * 2 as libc::c_int as intptr_t) as isize);
        dst0 = dst0.offset(dst_stride as isize);
        dsth = dsth.offset(dst_stride as isize);
        dstv = dstv.offset(dst_stride as isize);
        dstc = dstc.offset(dst_stride as isize);
        y += 1;
        y;
    }
}
unsafe extern "C" fn mbtree_propagate_cost(
    mut dst: *mut int16_t,
    mut propagate_in: *mut uint16_t,
    mut intra_costs: *mut uint16_t,
    mut inter_costs: *mut uint16_t,
    mut inv_qscales: *mut uint16_t,
    mut fps_factor: *mut libc::c_float,
    mut len: libc::c_int,
) {
    let mut fps: libc::c_float = *fps_factor;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < len {
        let mut intra_cost: libc::c_int = *intra_costs.offset(i as isize) as libc::c_int;
        let mut inter_cost: libc::c_int = if (*intra_costs.offset(i as isize)
            as libc::c_int)
            < *inter_costs.offset(i as isize) as libc::c_int & (((1 as libc::c_int) << 14 as libc::c_int) - 1 as libc::c_int)
        {
            *intra_costs.offset(i as isize) as libc::c_int
        } else {
            *inter_costs.offset(i as isize) as libc::c_int & (((1 as libc::c_int) << 14 as libc::c_int) - 1 as libc::c_int)
        };
        let mut propagate_intra: libc::c_float = (intra_cost
            * *inv_qscales.offset(i as isize) as libc::c_int) as libc::c_float;
        let mut propagate_amount: libc::c_float = *propagate_in.offset(i as isize)
            as libc::c_int as libc::c_float + propagate_intra * fps;
        let mut propagate_num: libc::c_float = (intra_cost - inter_cost)
            as libc::c_float;
        let mut propagate_denom: libc::c_float = intra_cost as libc::c_float;
        *dst
            .offset(
                i as isize,
            ) = (if ((propagate_amount * propagate_num / propagate_denom + 0.5f32)
            as libc::c_int) < 32767 as libc::c_int
        {
            (propagate_amount * propagate_num / propagate_denom + 0.5f32) as libc::c_int
        } else {
            32767 as libc::c_int
        }) as int16_t;
        i += 1;
        i;
    }
}
unsafe extern "C" fn mbtree_propagate_list(
    mut h: *mut x264_t,
    mut ref_costs: *mut uint16_t,
    mut mvs: *mut [int16_t; 2],
    mut propagate_amount: *mut int16_t,
    mut lowres_costs: *mut uint16_t,
    mut bipred_weight: libc::c_int,
    mut mb_y: libc::c_int,
    mut len: libc::c_int,
    mut list: libc::c_int,
) {
    let mut stride: libc::c_uint = (*h).mb.i_mb_stride as libc::c_uint;
    let mut width: libc::c_uint = (*h).mb.i_mb_width as libc::c_uint;
    let mut height: libc::c_uint = (*h).mb.i_mb_height as libc::c_uint;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < len {
        let mut lists_used: libc::c_int = *lowres_costs.offset(i as isize) as libc::c_int
            >> 14 as libc::c_int;
        if lists_used & ((1 as libc::c_int) << list) != 0 {
            let mut listamount: libc::c_int = *propagate_amount.offset(i as isize)
                as libc::c_int;
            if lists_used == 3 as libc::c_int {
                listamount = (listamount * bipred_weight + 32 as libc::c_int) >> 6 as libc::c_int;
            }
            if (*((*mvs.offset(i as isize)).as_mut_ptr() as *mut x264_union32_t)).i == 0
            {
                *ref_costs
                    .offset(
                        (mb_y as libc::c_uint)
                            .wrapping_mul(stride)
                            .wrapping_add(i as libc::c_uint) as isize,
                    ) = (if *ref_costs
                    .offset(
                        (mb_y as libc::c_uint)
                            .wrapping_mul(stride)
                            .wrapping_add(i as libc::c_uint) as isize,
                    ) as libc::c_int + listamount
                    < ((1 as libc::c_int) << 15 as libc::c_int) - 1 as libc::c_int
                {
                    *ref_costs
                        .offset(
                            (mb_y as libc::c_uint)
                                .wrapping_mul(stride)
                                .wrapping_add(i as libc::c_uint) as isize,
                        ) as libc::c_int + listamount
                } else {
                    ((1 as libc::c_int) << 15 as libc::c_int) - 1 as libc::c_int
                }) as uint16_t;
            } else {
                let mut x: libc::c_int = (*mvs
                    .offset(i as isize))[0 as libc::c_int as usize] as libc::c_int;
                let mut y: libc::c_int = (*mvs
                    .offset(i as isize))[1 as libc::c_int as usize] as libc::c_int;
                let mut mbx: libc::c_uint = ((x >> 5 as libc::c_int) + i)
                    as libc::c_uint;
                let mut mby: libc::c_uint = ((y >> 5 as libc::c_int) + mb_y)
                    as libc::c_uint;
                let mut idx0: libc::c_uint = mbx.wrapping_add(mby.wrapping_mul(stride));
                let mut idx2: libc::c_uint = idx0.wrapping_add(stride);
                x &= 31 as libc::c_int;
                y &= 31 as libc::c_int;
                let mut idx0weight: libc::c_int = (32 as libc::c_int - y)
                    * (32 as libc::c_int - x);
                let mut idx1weight: libc::c_int = (32 as libc::c_int - y) * x;
                let mut idx2weight: libc::c_int = y * (32 as libc::c_int - x);
                let mut idx3weight: libc::c_int = y * x;
                idx0weight = (idx0weight * listamount + 512 as libc::c_int) >> 10 as libc::c_int;
                idx1weight = (idx1weight * listamount + 512 as libc::c_int) >> 10 as libc::c_int;
                idx2weight = (idx2weight * listamount + 512 as libc::c_int) >> 10 as libc::c_int;
                idx3weight = (idx3weight * listamount + 512 as libc::c_int) >> 10 as libc::c_int;
                if mbx < width.wrapping_sub(1 as libc::c_int as libc::c_uint)
                    && mby < height.wrapping_sub(1 as libc::c_int as libc::c_uint)
                {
                    *ref_costs
                        .offset(
                            idx0.wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
                        ) = (if *ref_costs
                        .offset(
                            idx0.wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
                        ) as libc::c_int + idx0weight
                        < ((1 as libc::c_int) << 15 as libc::c_int) - 1 as libc::c_int
                    {
                        *ref_costs
                            .offset(
                                idx0.wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
                            ) as libc::c_int + idx0weight
                    } else {
                        ((1 as libc::c_int) << 15 as libc::c_int) - 1 as libc::c_int
                    }) as uint16_t;
                    *ref_costs
                        .offset(
                            idx0.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                        ) = (if *ref_costs
                        .offset(
                            idx0.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                        ) as libc::c_int + idx1weight
                        < ((1 as libc::c_int) << 15 as libc::c_int) - 1 as libc::c_int
                    {
                        *ref_costs
                            .offset(
                                idx0.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                            ) as libc::c_int + idx1weight
                    } else {
                        ((1 as libc::c_int) << 15 as libc::c_int) - 1 as libc::c_int
                    }) as uint16_t;
                    *ref_costs
                        .offset(
                            idx2.wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
                        ) = (if *ref_costs
                        .offset(
                            idx2.wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
                        ) as libc::c_int + idx2weight
                        < ((1 as libc::c_int) << 15 as libc::c_int) - 1 as libc::c_int
                    {
                        *ref_costs
                            .offset(
                                idx2.wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
                            ) as libc::c_int + idx2weight
                    } else {
                        ((1 as libc::c_int) << 15 as libc::c_int) - 1 as libc::c_int
                    }) as uint16_t;
                    *ref_costs
                        .offset(
                            idx2.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                        ) = (if *ref_costs
                        .offset(
                            idx2.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                        ) as libc::c_int + idx3weight
                        < ((1 as libc::c_int) << 15 as libc::c_int) - 1 as libc::c_int
                    {
                        *ref_costs
                            .offset(
                                idx2.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                            ) as libc::c_int + idx3weight
                    } else {
                        ((1 as libc::c_int) << 15 as libc::c_int) - 1 as libc::c_int
                    }) as uint16_t;
                } else {
                    if mby < height {
                        if mbx < width {
                            *ref_costs
                                .offset(
                                    idx0.wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
                                ) = (if *ref_costs
                                .offset(
                                    idx0.wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
                                ) as libc::c_int + idx0weight
                                < ((1 as libc::c_int) << 15 as libc::c_int)
                                    - 1 as libc::c_int
                            {
                                *ref_costs
                                    .offset(
                                        idx0.wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
                                    ) as libc::c_int + idx0weight
                            } else {
                                ((1 as libc::c_int) << 15 as libc::c_int) - 1 as libc::c_int
                            }) as uint16_t;
                        }
                        if mbx.wrapping_add(1 as libc::c_int as libc::c_uint) < width {
                            *ref_costs
                                .offset(
                                    idx0.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                                ) = (if *ref_costs
                                .offset(
                                    idx0.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                                ) as libc::c_int + idx1weight
                                < ((1 as libc::c_int) << 15 as libc::c_int)
                                    - 1 as libc::c_int
                            {
                                *ref_costs
                                    .offset(
                                        idx0.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                                    ) as libc::c_int + idx1weight
                            } else {
                                ((1 as libc::c_int) << 15 as libc::c_int) - 1 as libc::c_int
                            }) as uint16_t;
                        }
                    }
                    if mby.wrapping_add(1 as libc::c_int as libc::c_uint) < height {
                        if mbx < width {
                            *ref_costs
                                .offset(
                                    idx2.wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
                                ) = (if *ref_costs
                                .offset(
                                    idx2.wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
                                ) as libc::c_int + idx2weight
                                < ((1 as libc::c_int) << 15 as libc::c_int)
                                    - 1 as libc::c_int
                            {
                                *ref_costs
                                    .offset(
                                        idx2.wrapping_add(0 as libc::c_int as libc::c_uint) as isize,
                                    ) as libc::c_int + idx2weight
                            } else {
                                ((1 as libc::c_int) << 15 as libc::c_int) - 1 as libc::c_int
                            }) as uint16_t;
                        }
                        if mbx.wrapping_add(1 as libc::c_int as libc::c_uint) < width {
                            *ref_costs
                                .offset(
                                    idx2.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                                ) = (if *ref_costs
                                .offset(
                                    idx2.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                                ) as libc::c_int + idx3weight
                                < ((1 as libc::c_int) << 15 as libc::c_int)
                                    - 1 as libc::c_int
                            {
                                *ref_costs
                                    .offset(
                                        idx2.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                                    ) as libc::c_int + idx3weight
                            } else {
                                ((1 as libc::c_int) << 15 as libc::c_int) - 1 as libc::c_int
                            }) as uint16_t;
                        }
                    }
                }
            }
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn mbtree_fix8_pack(
    mut dst: *mut uint16_t,
    mut src: *mut libc::c_float,
    mut count: libc::c_int,
) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < count {
        *dst
            .offset(
                i as isize,
            ) = endian_fix16(
            (*src.offset(i as isize) * 256.0f32) as int16_t as uint16_t,
        );
        i += 1;
        i;
    }
}
unsafe extern "C" fn mbtree_fix8_unpack(
    mut dst: *mut libc::c_float,
    mut src: *mut uint16_t,
    mut count: libc::c_int,
) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < count {
        *dst
            .offset(
                i as isize,
            ) = endian_fix16(*src.offset(i as isize)) as int16_t as libc::c_int
            as libc::c_float * (1.0f32 / 256.0f32);
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_mc_init(
    mut cpu: uint32_t,
    mut pf: *mut x264_mc_functions_t,
    mut cpu_independent: libc::c_int,
) {
    (*pf)
        .mc_luma = Some(
        mc_luma
            as unsafe extern "C" fn(
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
    );
    (*pf)
        .get_ref = Some(
        get_ref
            as unsafe extern "C" fn(
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
    );
    (*pf)
        .mc_chroma = Some(
        mc_chroma
            as unsafe extern "C" fn(
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
    );
    (*pf)
        .avg[PIXEL_16x16 as libc::c_int
        as usize] = Some(
        pixel_avg_16x16
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                libc::c_int,
            ) -> (),
    );
    (*pf)
        .avg[PIXEL_16x8 as libc::c_int
        as usize] = Some(
        pixel_avg_16x8
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                libc::c_int,
            ) -> (),
    );
    (*pf)
        .avg[PIXEL_8x16 as libc::c_int
        as usize] = Some(
        pixel_avg_8x16
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                libc::c_int,
            ) -> (),
    );
    (*pf)
        .avg[PIXEL_8x8 as libc::c_int
        as usize] = Some(
        pixel_avg_8x8
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                libc::c_int,
            ) -> (),
    );
    (*pf)
        .avg[PIXEL_8x4 as libc::c_int
        as usize] = Some(
        pixel_avg_8x4
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                libc::c_int,
            ) -> (),
    );
    (*pf)
        .avg[PIXEL_4x16 as libc::c_int
        as usize] = Some(
        pixel_avg_4x16
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                libc::c_int,
            ) -> (),
    );
    (*pf)
        .avg[PIXEL_4x8 as libc::c_int
        as usize] = Some(
        pixel_avg_4x8
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                libc::c_int,
            ) -> (),
    );
    (*pf)
        .avg[PIXEL_4x4 as libc::c_int
        as usize] = Some(
        pixel_avg_4x4
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                libc::c_int,
            ) -> (),
    );
    (*pf)
        .avg[PIXEL_4x2 as libc::c_int
        as usize] = Some(
        pixel_avg_4x2
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                libc::c_int,
            ) -> (),
    );
    (*pf)
        .avg[PIXEL_2x8 as libc::c_int
        as usize] = Some(
        pixel_avg_2x8
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                libc::c_int,
            ) -> (),
    );
    (*pf)
        .avg[PIXEL_2x4 as libc::c_int
        as usize] = Some(
        pixel_avg_2x4
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                libc::c_int,
            ) -> (),
    );
    (*pf)
        .avg[PIXEL_2x2 as libc::c_int
        as usize] = Some(
        pixel_avg_2x2
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                libc::c_int,
            ) -> (),
    );
    (*pf).weight = mc_weight_wtab.as_mut_ptr();
    (*pf).offsetadd = mc_weight_wtab.as_mut_ptr();
    (*pf).offsetsub = mc_weight_wtab.as_mut_ptr();
    (*pf)
        .weight_cache = Some(
        weight_cache as unsafe extern "C" fn(*mut x264_t, *mut x264_weight_t) -> (),
    );
    (*pf)
        .copy_16x16_unaligned = Some(
        mc_copy_w16
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                libc::c_int,
            ) -> (),
    );
    (*pf)
        .copy[PIXEL_16x16 as libc::c_int
        as usize] = Some(
        mc_copy_w16
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                libc::c_int,
            ) -> (),
    );
    (*pf)
        .copy[PIXEL_8x8 as libc::c_int
        as usize] = Some(
        mc_copy_w8
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                libc::c_int,
            ) -> (),
    );
    (*pf)
        .copy[PIXEL_4x4 as libc::c_int
        as usize] = Some(
        mc_copy_w4
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                libc::c_int,
            ) -> (),
    );
    (*pf)
        .store_interleave_chroma = Some(
        store_interleave_chroma
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                *mut pixel,
                libc::c_int,
            ) -> (),
    );
    (*pf)
        .load_deinterleave_chroma_fenc = Some(
        load_deinterleave_chroma_fenc
            as unsafe extern "C" fn(*mut pixel, *mut pixel, intptr_t, libc::c_int) -> (),
    );
    (*pf)
        .load_deinterleave_chroma_fdec = Some(
        load_deinterleave_chroma_fdec
            as unsafe extern "C" fn(*mut pixel, *mut pixel, intptr_t, libc::c_int) -> (),
    );
    (*pf)
        .plane_copy = Some(
        x264_8_plane_copy_c
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*pf)
        .plane_copy_swap = Some(
        x264_8_plane_copy_swap_c
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*pf)
        .plane_copy_interleave = Some(
        x264_8_plane_copy_interleave_c
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*pf)
        .plane_copy_deinterleave = Some(
        x264_8_plane_copy_deinterleave_c
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*pf)
        .plane_copy_deinterleave_yuyv = Some(
        x264_8_plane_copy_deinterleave_c
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*pf)
        .plane_copy_deinterleave_rgb = Some(
        plane_copy_deinterleave_rgb_c
            as unsafe extern "C" fn(
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
    );
    (*pf)
        .plane_copy_deinterleave_v210 = Some(
        plane_copy_deinterleave_v210_c
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut uint32_t,
                intptr_t,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*pf)
        .hpel_filter = Some(
        hpel_filter
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                libc::c_int,
                libc::c_int,
                *mut int16_t,
            ) -> (),
    );
    (*pf)
        .prefetch_fenc_400 = Some(
        prefetch_fenc_null
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                libc::c_int,
            ) -> (),
    );
    (*pf)
        .prefetch_fenc_420 = Some(
        prefetch_fenc_null
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                libc::c_int,
            ) -> (),
    );
    (*pf)
        .prefetch_fenc_422 = Some(
        prefetch_fenc_null
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                libc::c_int,
            ) -> (),
    );
    (*pf)
        .prefetch_ref = Some(
        prefetch_ref_null
            as unsafe extern "C" fn(*mut pixel, intptr_t, libc::c_int) -> (),
    );
    (*pf)
        .memcpy_aligned = Some(
        memcpy
            as unsafe extern "C" fn(
                *mut libc::c_void,
                *const libc::c_void,
                libc::c_ulong,
            ) -> *mut libc::c_void,
    );
    (*pf)
        .memzero_aligned = Some(
        memzero_aligned as unsafe extern "C" fn(*mut libc::c_void, size_t) -> (),
    );
    (*pf)
        .frame_init_lowres_core = Some(
        frame_init_lowres_core
            as unsafe extern "C" fn(
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
    );
    (*pf)
        .integral_init4h = Some(
        integral_init4h
            as unsafe extern "C" fn(*mut uint16_t, *mut pixel, intptr_t) -> (),
    );
    (*pf)
        .integral_init8h = Some(
        integral_init8h
            as unsafe extern "C" fn(*mut uint16_t, *mut pixel, intptr_t) -> (),
    );
    (*pf)
        .integral_init4v = Some(
        integral_init4v
            as unsafe extern "C" fn(*mut uint16_t, *mut uint16_t, intptr_t) -> (),
    );
    (*pf)
        .integral_init8v = Some(
        integral_init8v as unsafe extern "C" fn(*mut uint16_t, intptr_t) -> (),
    );
    (*pf)
        .mbtree_propagate_cost = Some(
        mbtree_propagate_cost
            as unsafe extern "C" fn(
                *mut int16_t,
                *mut uint16_t,
                *mut uint16_t,
                *mut uint16_t,
                *mut uint16_t,
                *mut libc::c_float,
                libc::c_int,
            ) -> (),
    );
    (*pf)
        .mbtree_propagate_list = Some(
        mbtree_propagate_list
            as unsafe extern "C" fn(
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
    );
    (*pf)
        .mbtree_fix8_pack = Some(
        mbtree_fix8_pack
            as unsafe extern "C" fn(*mut uint16_t, *mut libc::c_float, libc::c_int) -> (),
    );
    (*pf)
        .mbtree_fix8_unpack = Some(
        mbtree_fix8_unpack
            as unsafe extern "C" fn(*mut libc::c_float, *mut uint16_t, libc::c_int) -> (),
    );
    if cpu_independent != 0 {
        (*pf)
            .mbtree_propagate_cost = Some(
            mbtree_propagate_cost
                as unsafe extern "C" fn(
                    *mut int16_t,
                    *mut uint16_t,
                    *mut uint16_t,
                    *mut uint16_t,
                    *mut uint16_t,
                    *mut libc::c_float,
                    libc::c_int,
                ) -> (),
        );
        (*pf)
            .mbtree_propagate_list = Some(
            mbtree_propagate_list
                as unsafe extern "C" fn(
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
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_frame_filter(
    mut h: *mut x264_t,
    mut frame: *mut x264_frame_t,
    mut mb_y: libc::c_int,
    mut b_end: libc::c_int,
) {
    let b_interlaced: libc::c_int = (*h).param.b_interlaced;
    let mut start: libc::c_int = mb_y * 16 as libc::c_int - 8 as libc::c_int;
    let mut height: libc::c_int = (if b_end != 0 {
        (*frame).i_lines[0 as libc::c_int as usize]
            + 16 as libc::c_int * (*h).param.b_interlaced
    } else {
        (mb_y + b_interlaced) * 16 as libc::c_int
    }) + 8 as libc::c_int;
    if mb_y & b_interlaced != 0 {
        return;
    }
    let mut p: libc::c_int = 0 as libc::c_int;
    while p
        < (if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as libc::c_int
        {
            3 as libc::c_int
        } else {
            1 as libc::c_int
        })
    {
        let mut stride: libc::c_int = (*frame).i_stride[p as usize];
        let width: libc::c_int = (*frame).i_width[p as usize];
        let mut offs: libc::c_int = start * stride - 8 as libc::c_int;
        if b_interlaced == 0 || (*h).mb.b_adaptive_mbaff != 0 {
            ((*h).mc.hpel_filter)
                .expect(
                    "non-null function pointer",
                )(
                ((*frame).filtered[p as usize][1 as libc::c_int as usize])
                    .offset(offs as isize),
                ((*frame).filtered[p as usize][2 as libc::c_int as usize])
                    .offset(offs as isize),
                ((*frame).filtered[p as usize][3 as libc::c_int as usize])
                    .offset(offs as isize),
                ((*frame).plane[p as usize]).offset(offs as isize),
                stride as intptr_t,
                width + 16 as libc::c_int,
                height - start,
                (*h).scratch_buffer as *mut int16_t,
            );
        }
        if b_interlaced != 0 {
            stride = (*frame).i_stride[p as usize] << 1 as libc::c_int;
            start = ((mb_y * 16 as libc::c_int) >> 1 as libc::c_int) - 8 as libc::c_int;
            let mut height_fld: libc::c_int = ((if b_end != 0 {
                (*frame).i_lines[p as usize]
            } else {
                mb_y * 16 as libc::c_int
            }) >> 1 as libc::c_int) + 8 as libc::c_int;
            offs = start * stride - 8 as libc::c_int;
            let mut i: libc::c_int = 0 as libc::c_int;
            while i < 2 as libc::c_int {
                ((*h).mc.hpel_filter)
                    .expect(
                        "non-null function pointer",
                    )(
                    ((*frame).filtered_fld[p as usize][1 as libc::c_int as usize])
                        .offset(offs as isize),
                    ((*frame).filtered_fld[p as usize][2 as libc::c_int as usize])
                        .offset(offs as isize),
                    ((*frame).filtered_fld[p as usize][3 as libc::c_int as usize])
                        .offset(offs as isize),
                    ((*frame).plane_fld[p as usize]).offset(offs as isize),
                    stride as intptr_t,
                    width + 16 as libc::c_int,
                    height_fld - start,
                    (*h).scratch_buffer as *mut int16_t,
                );
                i += 1;
                i;
                offs += (*frame).i_stride[p as usize];
            }
        }
        p += 1;
        p;
    }
    if !((*frame).integral).is_null() {
        let mut stride_0: libc::c_int = (*frame).i_stride[0 as libc::c_int as usize];
        if start < 0 as libc::c_int {
            memset(
                ((*frame).integral)
                    .offset(-((32 as libc::c_int * stride_0) as isize))
                    .offset(
                        -((if 32 as libc::c_int
                            > 64 as libc::c_int
                                / ::core::mem::size_of::<pixel>() as libc::c_ulong
                                    as libc::c_int
                        {
                            32 as libc::c_int
                        } else {
                            64 as libc::c_int
                                / ::core::mem::size_of::<pixel>() as libc::c_ulong
                                    as libc::c_int
                        }) as isize),
                    ) as *mut libc::c_void,
                0 as libc::c_int,
                (stride_0 as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<uint16_t>() as libc::c_ulong),
            );
            start = -(32 as libc::c_int);
        }
        if b_end != 0 {
            height += 32 as libc::c_int - 9 as libc::c_int;
        }
        let mut y: libc::c_int = start;
        while y < height {
            let mut pix: *mut pixel = ((*frame).plane[0 as libc::c_int as usize])
                .offset((y * stride_0) as isize)
                .offset(
                    -((if 32 as libc::c_int
                        > 64 as libc::c_int
                            / ::core::mem::size_of::<pixel>() as libc::c_ulong
                                as libc::c_int
                    {
                        32 as libc::c_int
                    } else {
                        64 as libc::c_int
                            / ::core::mem::size_of::<pixel>() as libc::c_ulong
                                as libc::c_int
                    }) as isize),
                );
            let mut sum8: *mut uint16_t = ((*frame).integral)
                .offset(((y + 1 as libc::c_int) * stride_0) as isize)
                .offset(
                    -((if 32 as libc::c_int
                        > 64 as libc::c_int
                            / ::core::mem::size_of::<pixel>() as libc::c_ulong
                                as libc::c_int
                    {
                        32 as libc::c_int
                    } else {
                        64 as libc::c_int
                            / ::core::mem::size_of::<pixel>() as libc::c_ulong
                                as libc::c_int
                    }) as isize),
                );
            let mut sum4: *mut uint16_t = std::ptr::null_mut::<uint16_t>();
            if (*h).frames.b_have_sub8x8_esa != 0 {
                ((*h).mc.integral_init4h)
                    .expect(
                        "non-null function pointer",
                    )(sum8, pix, stride_0 as intptr_t);
                sum8 = sum8.offset(-((8 as libc::c_int * stride_0) as isize));
                sum4 = sum8
                    .offset(
                        (stride_0
                            * ((*frame).i_lines[0 as libc::c_int as usize]
                                + 32 as libc::c_int * 2 as libc::c_int)) as isize,
                    );
                if y >= 8 as libc::c_int - 32 as libc::c_int {
                    ((*h).mc.integral_init4v)
                        .expect(
                            "non-null function pointer",
                        )(sum8, sum4, stride_0 as intptr_t);
                }
            } else {
                ((*h).mc.integral_init8h)
                    .expect(
                        "non-null function pointer",
                    )(sum8, pix, stride_0 as intptr_t);
                if y >= 8 as libc::c_int - 32 as libc::c_int {
                    ((*h).mc.integral_init8v)
                        .expect(
                            "non-null function pointer",
                        )(
                        sum8.offset(-((8 as libc::c_int * stride_0) as isize)),
                        stride_0 as intptr_t,
                    );
                }
            }
            y += 1;
            y;
        }
    }
}
