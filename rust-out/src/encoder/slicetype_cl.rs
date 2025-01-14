#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types, label_break_value)]
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
    fn x264_8_log(
        h: *mut x264_t,
        i_level: libc::c_int,
        psz_fmt: *const libc::c_char,
        _: ...
    );
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    static mut x264_zero: [uint8_t; 1024];
    fn x264_8_weights_analyse(
        h: *mut x264_t,
        fenc: *mut x264_frame_t,
        ref_0: *mut x264_frame_t,
        b_lookahead: libc::c_int,
    );
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
#[no_mangle]
pub unsafe extern "C" fn x264_8_opencl_flush(mut h: *mut x264_t) {
    let mut ocl: *mut x264_opencl_function_t = (*h).opencl.ocl;
    ((*ocl).clFinish).expect("non-null function pointer")((*h).opencl.queue);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*h).opencl.num_copies {
        memcpy(
            (*h).opencl.copies[i as usize].dest,
            (*h).opencl.copies[i as usize].src,
            (*h).opencl.copies[i as usize].bytes as libc::c_ulong,
        );
        i += 1;
        i;
    }
    (*h).opencl.num_copies = 0 as libc::c_int;
    (*h).opencl.pl_occupancy = 0 as libc::c_int;
}
unsafe extern "C" fn opencl_alloc_locked(
    mut h: *mut x264_t,
    mut bytes: libc::c_int,
) -> *mut libc::c_void {
    if (*h).opencl.pl_occupancy + bytes
        >= 32 as libc::c_int * 1024 as libc::c_int * 1024 as libc::c_int
    {
        x264_8_opencl_flush(h);
    }
    if bytes < 32 as libc::c_int * 1024 as libc::c_int * 1024 as libc::c_int {} else {
        __assert_fail(
            b"bytes < PAGE_LOCKED_BUF_SIZE\0" as *const u8 as *const libc::c_char,
            b"encoder/slicetype-cl.c\0" as *const u8 as *const libc::c_char,
            76 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 41],
                &[libc::c_char; 41],
            >(b"void *opencl_alloc_locked(x264_t *, int)\0"))
                .as_ptr(),
        );
    }
    'c_29389: {
        if bytes < 32 as libc::c_int * 1024 as libc::c_int * 1024 as libc::c_int
        {} else {
            __assert_fail(
                b"bytes < PAGE_LOCKED_BUF_SIZE\0" as *const u8 as *const libc::c_char,
                b"encoder/slicetype-cl.c\0" as *const u8 as *const libc::c_char,
                76 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 41],
                    &[libc::c_char; 41],
                >(b"void *opencl_alloc_locked(x264_t *, int)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut ptr: *mut libc::c_char = ((*h).opencl.page_locked_ptr)
        .offset((*h).opencl.pl_occupancy as isize);
    (*h).opencl.pl_occupancy += bytes;
    ptr as *mut libc::c_void
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_opencl_lowres_init(
    mut h: *mut x264_t,
    mut fenc: *mut x264_frame_t,
    mut lambda: libc::c_int,
) -> libc::c_int {
    if (*fenc).b_intra_calculated != 0 {
        return 0 as libc::c_int;
    }
    (*fenc).b_intra_calculated = 1 as libc::c_int;
    let mut ocl: *mut x264_opencl_function_t = (*h).opencl.ocl;
    let mut luma_length: libc::c_int = (*fenc).i_stride[0 as libc::c_int as usize]
        * (*fenc).i_lines[0 as libc::c_int as usize];
    let mut mb_count: libc::c_int = (*h).mb.i_mb_count;
    let mut status: cl_int = 0;
    if ((*h).opencl.lowres_mv_costs).is_null() {
        let mut width: libc::c_int = (*h).mb.i_mb_width * 8 as libc::c_int
            * ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int;
        let mut height: libc::c_int = (*h).mb.i_mb_height * 8 as libc::c_int
            * ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int;
        let mut pixel_format: cl_image_format = _cl_image_format {
            image_channel_order: 0,
            image_channel_data_type: 0,
        };
        pixel_format.image_channel_order = 0x10b0 as libc::c_int as cl_channel_order;
        pixel_format.image_channel_data_type = 0x10dc as libc::c_int as cl_channel_type;
        (*h)
            .opencl
            .weighted_luma_hpel = ((*ocl).clCreateImage2D)
            .expect(
                "non-null function pointer",
            )(
            (*h).opencl.context,
            ((1 as libc::c_int) << 0 as libc::c_int) as cl_mem_flags,
            &mut pixel_format,
            width as size_t,
            height as size_t,
            0 as libc::c_int as size_t,
            std::ptr::null_mut::<libc::c_void>(),
            &mut status,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clCreateImage2D error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            pixel_format.image_channel_order = 0x10b5 as libc::c_int as cl_channel_order;
            pixel_format
                .image_channel_data_type = 0x10da as libc::c_int as cl_channel_type;
            (*h)
                .opencl
                .weighted_scaled_images[i
                as usize] = ((*ocl).clCreateImage2D)
                .expect(
                    "non-null function pointer",
                )(
                (*h).opencl.context,
                ((1 as libc::c_int) << 0 as libc::c_int) as cl_mem_flags,
                &mut pixel_format,
                width as size_t,
                height as size_t,
                0 as libc::c_int as size_t,
                std::ptr::null_mut::<libc::c_void>(),
                &mut status,
            );
            if status != 0 as libc::c_int {
                (*h).param.b_opencl = 0 as libc::c_int;
                x264_8_log(
                    h,
                    0 as libc::c_int,
                    b"clCreateImage2D error '%d'\n\0" as *const u8
                        as *const libc::c_char,
                    status,
                );
                return -(1 as libc::c_int);
            }
            width >>= 1 as libc::c_int;
            height >>= 1 as libc::c_int;
            i += 1;
            i;
        }
        (*h)
            .opencl
            .lowres_mv_costs = ((*ocl).clCreateBuffer)
            .expect(
                "non-null function pointer",
            )(
            (*h).opencl.context,
            ((1 as libc::c_int) << 0 as libc::c_int) as cl_mem_flags,
            (mb_count as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong),
            std::ptr::null_mut::<libc::c_void>(),
            &mut status,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clCreateBuffer error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        (*h)
            .opencl
            .lowres_costs[0 as libc::c_int
            as usize] = ((*ocl).clCreateBuffer)
            .expect(
                "non-null function pointer",
            )(
            (*h).opencl.context,
            ((1 as libc::c_int) << 0 as libc::c_int) as cl_mem_flags,
            (mb_count as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong),
            std::ptr::null_mut::<libc::c_void>(),
            &mut status,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clCreateBuffer error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        (*h)
            .opencl
            .lowres_costs[1 as libc::c_int
            as usize] = ((*ocl).clCreateBuffer)
            .expect(
                "non-null function pointer",
            )(
            (*h).opencl.context,
            ((1 as libc::c_int) << 0 as libc::c_int) as cl_mem_flags,
            (mb_count as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong),
            std::ptr::null_mut::<libc::c_void>(),
            &mut status,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clCreateBuffer error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        (*h)
            .opencl
            .mv_buffers[0 as libc::c_int
            as usize] = ((*ocl).clCreateBuffer)
            .expect(
                "non-null function pointer",
            )(
            (*h).opencl.context,
            ((1 as libc::c_int) << 0 as libc::c_int) as cl_mem_flags,
            (mb_count as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong),
            std::ptr::null_mut::<libc::c_void>(),
            &mut status,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clCreateBuffer error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        (*h)
            .opencl
            .mv_buffers[1 as libc::c_int
            as usize] = ((*ocl).clCreateBuffer)
            .expect(
                "non-null function pointer",
            )(
            (*h).opencl.context,
            ((1 as libc::c_int) << 0 as libc::c_int) as cl_mem_flags,
            (mb_count as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong),
            std::ptr::null_mut::<libc::c_void>(),
            &mut status,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clCreateBuffer error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        (*h)
            .opencl
            .mvp_buffer = ((*ocl).clCreateBuffer)
            .expect(
                "non-null function pointer",
            )(
            (*h).opencl.context,
            ((1 as libc::c_int) << 0 as libc::c_int) as cl_mem_flags,
            (mb_count as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong),
            std::ptr::null_mut::<libc::c_void>(),
            &mut status,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clCreateBuffer error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        (*h)
            .opencl
            .frame_stats[0 as libc::c_int
            as usize] = ((*ocl).clCreateBuffer)
            .expect(
                "non-null function pointer",
            )(
            (*h).opencl.context,
            ((1 as libc::c_int) << 1 as libc::c_int) as cl_mem_flags,
            (4 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
            std::ptr::null_mut::<libc::c_void>(),
            &mut status,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clCreateBuffer error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        (*h)
            .opencl
            .frame_stats[1 as libc::c_int
            as usize] = ((*ocl).clCreateBuffer)
            .expect(
                "non-null function pointer",
            )(
            (*h).opencl.context,
            ((1 as libc::c_int) << 1 as libc::c_int) as cl_mem_flags,
            (4 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
            std::ptr::null_mut::<libc::c_void>(),
            &mut status,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clCreateBuffer error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        (*h)
            .opencl
            .row_satds[0 as libc::c_int
            as usize] = ((*ocl).clCreateBuffer)
            .expect(
                "non-null function pointer",
            )(
            (*h).opencl.context,
            ((1 as libc::c_int) << 1 as libc::c_int) as cl_mem_flags,
            ((*h).mb.i_mb_height as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
            std::ptr::null_mut::<libc::c_void>(),
            &mut status,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clCreateBuffer error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        (*h)
            .opencl
            .row_satds[1 as libc::c_int
            as usize] = ((*ocl).clCreateBuffer)
            .expect(
                "non-null function pointer",
            )(
            (*h).opencl.context,
            ((1 as libc::c_int) << 1 as libc::c_int) as cl_mem_flags,
            ((*h).mb.i_mb_height as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
            std::ptr::null_mut::<libc::c_void>(),
            &mut status,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clCreateBuffer error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        (*h)
            .opencl
            .luma_16x16_image[0 as libc::c_int
            as usize] = ((*ocl).clCreateBuffer)
            .expect(
                "non-null function pointer",
            )(
            (*h).opencl.context,
            ((1 as libc::c_int) << 2 as libc::c_int) as cl_mem_flags,
            luma_length as size_t,
            std::ptr::null_mut::<libc::c_void>(),
            &mut status,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clCreateBuffer error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        (*h)
            .opencl
            .luma_16x16_image[1 as libc::c_int
            as usize] = ((*ocl).clCreateBuffer)
            .expect(
                "non-null function pointer",
            )(
            (*h).opencl.context,
            ((1 as libc::c_int) << 2 as libc::c_int) as cl_mem_flags,
            luma_length as size_t,
            std::ptr::null_mut::<libc::c_void>(),
            &mut status,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clCreateBuffer error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
    }
    if ((*fenc).opencl.intra_cost).is_null() {
        let mut width_0: libc::c_int = (*h).mb.i_mb_width * 8 as libc::c_int
            * ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int;
        let mut height_0: libc::c_int = (*h).mb.i_mb_height * 8 as libc::c_int
            * ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int;
        let mut pixel_format_0: cl_image_format = _cl_image_format {
            image_channel_order: 0,
            image_channel_data_type: 0,
        };
        pixel_format_0.image_channel_order = 0x10b0 as libc::c_int as cl_channel_order;
        pixel_format_0
            .image_channel_data_type = 0x10dc as libc::c_int as cl_channel_type;
        (*fenc)
            .opencl
            .luma_hpel = ((*ocl).clCreateImage2D)
            .expect(
                "non-null function pointer",
            )(
            (*h).opencl.context,
            ((1 as libc::c_int) << 0 as libc::c_int) as cl_mem_flags,
            &mut pixel_format_0,
            width_0 as size_t,
            height_0 as size_t,
            0 as libc::c_int as size_t,
            std::ptr::null_mut::<libc::c_void>(),
            &mut status,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clCreateImage2D error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < 4 as libc::c_int {
            pixel_format_0
                .image_channel_order = 0x10b5 as libc::c_int as cl_channel_order;
            pixel_format_0
                .image_channel_data_type = 0x10da as libc::c_int as cl_channel_type;
            (*fenc)
                .opencl
                .scaled_image2Ds[i_0
                as usize] = ((*ocl).clCreateImage2D)
                .expect(
                    "non-null function pointer",
                )(
                (*h).opencl.context,
                ((1 as libc::c_int) << 0 as libc::c_int) as cl_mem_flags,
                &mut pixel_format_0,
                width_0 as size_t,
                height_0 as size_t,
                0 as libc::c_int as size_t,
                std::ptr::null_mut::<libc::c_void>(),
                &mut status,
            );
            if status != 0 as libc::c_int {
                (*h).param.b_opencl = 0 as libc::c_int;
                x264_8_log(
                    h,
                    0 as libc::c_int,
                    b"clCreateImage2D error '%d'\n\0" as *const u8
                        as *const libc::c_char,
                    status,
                );
                return -(1 as libc::c_int);
            }
            width_0 >>= 1 as libc::c_int;
            height_0 >>= 1 as libc::c_int;
            i_0 += 1;
            i_0;
        }
        (*fenc)
            .opencl
            .inv_qscale_factor = ((*ocl).clCreateBuffer)
            .expect(
                "non-null function pointer",
            )(
            (*h).opencl.context,
            ((1 as libc::c_int) << 2 as libc::c_int) as cl_mem_flags,
            (mb_count as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong),
            std::ptr::null_mut::<libc::c_void>(),
            &mut status,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clCreateBuffer error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        (*fenc)
            .opencl
            .intra_cost = ((*ocl).clCreateBuffer)
            .expect(
                "non-null function pointer",
            )(
            (*h).opencl.context,
            ((1 as libc::c_int) << 1 as libc::c_int) as cl_mem_flags,
            (mb_count as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong),
            std::ptr::null_mut::<libc::c_void>(),
            &mut status,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clCreateBuffer error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        (*fenc)
            .opencl
            .lowres_mvs0 = ((*ocl).clCreateBuffer)
            .expect(
                "non-null function pointer",
            )(
            (*h).opencl.context,
            ((1 as libc::c_int) << 0 as libc::c_int) as cl_mem_flags,
            ((mb_count * 2 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong)
                .wrapping_mul(((*h).param.i_bframe + 1 as libc::c_int) as libc::c_ulong),
            std::ptr::null_mut::<libc::c_void>(),
            &mut status,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clCreateBuffer error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        (*fenc)
            .opencl
            .lowres_mvs1 = ((*ocl).clCreateBuffer)
            .expect(
                "non-null function pointer",
            )(
            (*h).opencl.context,
            ((1 as libc::c_int) << 0 as libc::c_int) as cl_mem_flags,
            ((mb_count * 2 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong)
                .wrapping_mul(((*h).param.i_bframe + 1 as libc::c_int) as libc::c_ulong),
            std::ptr::null_mut::<libc::c_void>(),
            &mut status,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clCreateBuffer error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        (*fenc)
            .opencl
            .lowres_mv_costs0 = ((*ocl).clCreateBuffer)
            .expect(
                "non-null function pointer",
            )(
            (*h).opencl.context,
            ((1 as libc::c_int) << 0 as libc::c_int) as cl_mem_flags,
            (mb_count as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong)
                .wrapping_mul(((*h).param.i_bframe + 1 as libc::c_int) as libc::c_ulong),
            std::ptr::null_mut::<libc::c_void>(),
            &mut status,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clCreateBuffer error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        (*fenc)
            .opencl
            .lowres_mv_costs1 = ((*ocl).clCreateBuffer)
            .expect(
                "non-null function pointer",
            )(
            (*h).opencl.context,
            ((1 as libc::c_int) << 0 as libc::c_int) as cl_mem_flags,
            (mb_count as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong)
                .wrapping_mul(((*h).param.i_bframe + 1 as libc::c_int) as libc::c_ulong),
            std::ptr::null_mut::<libc::c_void>(),
            &mut status,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clCreateBuffer error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
    }
    let mut locked: *mut libc::c_char = opencl_alloc_locked(h, luma_length)
        as *mut libc::c_char;
    memcpy(
        locked as *mut libc::c_void,
        (*fenc).plane[0 as libc::c_int as usize] as *const libc::c_void,
        luma_length as libc::c_ulong,
    );
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    status = ((*ocl).clEnqueueWriteBuffer)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.queue,
        (*h).opencl.luma_16x16_image[(*h).opencl.last_buf as usize],
        0 as libc::c_int as cl_bool,
        0 as libc::c_int as size_t,
        luma_length as size_t,
        locked as *const libc::c_void,
        0 as libc::c_int as cl_uint,
        std::ptr::null::<cl_event>(),
        std::ptr::null_mut::<cl_event>(),
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clEnqueueWriteBuffer error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    let mut gdim: [size_t; 2] = [0; 2];
    if (*h).param.rc.i_aq_mode != 0 && !((*fenc).i_inv_qscale_factor).is_null() {
        let mut size: libc::c_int = ((*h).mb.i_mb_count as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong)
            as libc::c_int;
        locked = opencl_alloc_locked(h, size) as *mut libc::c_char;
        memcpy(
            locked as *mut libc::c_void,
            (*fenc).i_inv_qscale_factor as *const libc::c_void,
            size as libc::c_ulong,
        );
        if (*h).opencl.b_fatal_error != 0 {
            return -(1 as libc::c_int);
        }
        status = ((*ocl).clEnqueueWriteBuffer)
            .expect(
                "non-null function pointer",
            )(
            (*h).opencl.queue,
            (*fenc).opencl.inv_qscale_factor,
            0 as libc::c_int as cl_bool,
            0 as libc::c_int as size_t,
            size as size_t,
            locked as *const libc::c_void,
            0 as libc::c_int as cl_uint,
            std::ptr::null::<cl_event>(),
            std::ptr::null_mut::<cl_event>(),
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            (*h).opencl.b_fatal_error = 1 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clEnqueueWriteBuffer error '%d'\n\0" as *const u8
                    as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
    } else {
        let mut arg: cl_uint = 0 as libc::c_int as cl_uint;
        let mut value: int16_t = 256 as libc::c_int as int16_t;
        if (*h).opencl.b_fatal_error != 0 {
            return -(1 as libc::c_int);
        }
        let fresh0 = arg;
        arg = arg.wrapping_add(1);
        status = ((*ocl).clSetKernelArg)
            .expect(
                "non-null function pointer",
            )(
            (*h).opencl.memset_kernel,
            fresh0,
            ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
            &mut (*fenc).opencl.inv_qscale_factor as *mut cl_mem as *const libc::c_void,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            (*h).opencl.b_fatal_error = 1 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        if (*h).opencl.b_fatal_error != 0 {
            return -(1 as libc::c_int);
        }
        let fresh1 = arg;
        arg = arg.wrapping_add(1);
        status = ((*ocl).clSetKernelArg)
            .expect(
                "non-null function pointer",
            )(
            (*h).opencl.memset_kernel,
            fresh1,
            ::core::mem::size_of::<int16_t>() as libc::c_ulong,
            &mut value as *mut int16_t as *const libc::c_void,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            (*h).opencl.b_fatal_error = 1 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        gdim[0 as libc::c_int as usize] = (*h).mb.i_mb_count as size_t;
        if (*h).opencl.b_fatal_error != 0 {
            return -(1 as libc::c_int);
        }
        status = ((*ocl).clEnqueueNDRangeKernel)
            .expect(
                "non-null function pointer",
            )(
            (*h).opencl.queue,
            (*h).opencl.memset_kernel,
            1 as libc::c_int as cl_uint,
            std::ptr::null::<size_t>(),
            gdim.as_mut_ptr(),
            std::ptr::null::<size_t>(),
            0 as libc::c_int as cl_uint,
            std::ptr::null::<cl_event>(),
            std::ptr::null_mut::<cl_event>(),
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            (*h).opencl.b_fatal_error = 1 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clEnqueueNDRangeKernel error '%d'\n\0" as *const u8
                    as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
    }
    let mut stride: libc::c_int = (*fenc).i_stride[0 as libc::c_int as usize];
    let mut arg_0: cl_uint = 0 as libc::c_int as cl_uint;
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh2 = arg_0;
    arg_0 = arg_0.wrapping_add(1);
    status = ((*ocl).clSetKernelArg)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.downscale_hpel_kernel,
        fresh2,
        ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
        &mut *((*h).opencl.luma_16x16_image)
            .as_mut_ptr()
            .offset((*h).opencl.last_buf as isize) as *mut cl_mem as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh3 = arg_0;
    arg_0 = arg_0.wrapping_add(1);
    status = ((*ocl).clSetKernelArg)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.downscale_hpel_kernel,
        fresh3,
        ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
        &mut *((*fenc).opencl.scaled_image2Ds)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) as *mut cl_mem as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh4 = arg_0;
    arg_0 = arg_0.wrapping_add(1);
    status = ((*ocl).clSetKernelArg)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.downscale_hpel_kernel,
        fresh4,
        ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
        &mut (*fenc).opencl.luma_hpel as *mut cl_mem as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh5 = arg_0;
    arg_0 = arg_0.wrapping_add(1);
    status = ((*ocl).clSetKernelArg)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.downscale_hpel_kernel,
        fresh5,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        &mut stride as *mut libc::c_int as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    gdim[0 as libc::c_int as usize] = (8 as libc::c_int * (*h).mb.i_mb_width) as size_t;
    gdim[1 as libc::c_int as usize] = (8 as libc::c_int * (*h).mb.i_mb_height) as size_t;
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    status = ((*ocl).clEnqueueNDRangeKernel)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.queue,
        (*h).opencl.downscale_hpel_kernel,
        2 as libc::c_int as cl_uint,
        std::ptr::null::<size_t>(),
        gdim.as_mut_ptr(),
        std::ptr::null::<size_t>(),
        0 as libc::c_int as cl_uint,
        std::ptr::null::<cl_event>(),
        std::ptr::null_mut::<cl_event>(),
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clEnqueueNDRangeKernel error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < 4 as libc::c_int - 1 as libc::c_int {
        let mut kern: cl_kernel = if i_1 & 1 as libc::c_int != 0 {
            (*h).opencl.downscale_kernel1
        } else {
            (*h).opencl.downscale_kernel2
        };
        arg_0 = 0 as libc::c_int as cl_uint;
        if (*h).opencl.b_fatal_error != 0 {
            return -(1 as libc::c_int);
        }
        let fresh6 = arg_0;
        arg_0 = arg_0.wrapping_add(1);
        status = ((*ocl).clSetKernelArg)
            .expect(
                "non-null function pointer",
            )(
            kern,
            fresh6,
            ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
            &mut *((*fenc).opencl.scaled_image2Ds).as_mut_ptr().offset(i_1 as isize)
                as *mut cl_mem as *const libc::c_void,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            (*h).opencl.b_fatal_error = 1 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        if (*h).opencl.b_fatal_error != 0 {
            return -(1 as libc::c_int);
        }
        let fresh7 = arg_0;
        arg_0 = arg_0.wrapping_add(1);
        status = ((*ocl).clSetKernelArg)
            .expect(
                "non-null function pointer",
            )(
            kern,
            fresh7,
            ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
            &mut *((*fenc).opencl.scaled_image2Ds)
                .as_mut_ptr()
                .offset((i_1 + 1 as libc::c_int) as isize) as *mut cl_mem
                as *const libc::c_void,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            (*h).opencl.b_fatal_error = 1 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        gdim[0 as libc::c_int as usize] >>= 1 as libc::c_int;
        gdim[1 as libc::c_int as usize] >>= 1 as libc::c_int;
        if gdim[0 as libc::c_int as usize] < 16 as libc::c_int as size_t
            || gdim[1 as libc::c_int as usize] < 16 as libc::c_int as size_t
        {
            break;
        }
        if (*h).opencl.b_fatal_error != 0 {
            return -(1 as libc::c_int);
        }
        status = ((*ocl).clEnqueueNDRangeKernel)
            .expect(
                "non-null function pointer",
            )(
            (*h).opencl.queue,
            kern,
            2 as libc::c_int as cl_uint,
            std::ptr::null::<size_t>(),
            gdim.as_mut_ptr(),
            std::ptr::null::<size_t>(),
            0 as libc::c_int as cl_uint,
            std::ptr::null::<cl_event>(),
            std::ptr::null_mut::<cl_event>(),
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            (*h).opencl.b_fatal_error = 1 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clEnqueueNDRangeKernel error '%d'\n\0" as *const u8
                    as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        i_1 += 1;
        i_1;
    }
    let mut ldim: [size_t; 2] = [0; 2];
    gdim[0 as libc::c_int
        as usize] = ((((*h).mb.i_mb_width + 31 as libc::c_int) >> 5 as libc::c_int)
        << 5 as libc::c_int) as size_t;
    gdim[1 as libc::c_int as usize] = (8 as libc::c_int * (*h).mb.i_mb_height) as size_t;
    ldim[0 as libc::c_int as usize] = 32 as libc::c_int as size_t;
    ldim[1 as libc::c_int as usize] = 8 as libc::c_int as size_t;
    arg_0 = 0 as libc::c_int as cl_uint;
    let mut slow: libc::c_int = ((*h).param.analyse.i_subpel_refine > 7 as libc::c_int)
        as libc::c_int;
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh8 = arg_0;
    arg_0 = arg_0.wrapping_add(1);
    status = ((*ocl).clSetKernelArg)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.intra_kernel,
        fresh8,
        ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
        &mut *((*fenc).opencl.scaled_image2Ds)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) as *mut cl_mem as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh9 = arg_0;
    arg_0 = arg_0.wrapping_add(1);
    status = ((*ocl).clSetKernelArg)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.intra_kernel,
        fresh9,
        ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
        &mut (*fenc).opencl.intra_cost as *mut cl_mem as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh10 = arg_0;
    arg_0 = arg_0.wrapping_add(1);
    status = ((*ocl).clSetKernelArg)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.intra_kernel,
        fresh10,
        ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
        &mut *((*h).opencl.frame_stats)
            .as_mut_ptr()
            .offset((*h).opencl.last_buf as isize) as *mut cl_mem as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh11 = arg_0;
    arg_0 = arg_0.wrapping_add(1);
    status = ((*ocl).clSetKernelArg)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.intra_kernel,
        fresh11,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        &mut lambda as *mut libc::c_int as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh12 = arg_0;
    arg_0 = arg_0.wrapping_add(1);
    status = ((*ocl).clSetKernelArg)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.intra_kernel,
        fresh12,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        &mut (*h).mb.i_mb_width as *mut libc::c_int as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh13 = arg_0;
    arg_0 = arg_0.wrapping_add(1);
    status = ((*ocl).clSetKernelArg)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.intra_kernel,
        fresh13,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        &mut slow as *mut libc::c_int as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    status = ((*ocl).clEnqueueNDRangeKernel)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.queue,
        (*h).opencl.intra_kernel,
        2 as libc::c_int as cl_uint,
        std::ptr::null::<size_t>(),
        gdim.as_mut_ptr(),
        ldim.as_mut_ptr(),
        0 as libc::c_int as cl_uint,
        std::ptr::null::<cl_event>(),
        std::ptr::null_mut::<cl_event>(),
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clEnqueueNDRangeKernel error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    gdim[0 as libc::c_int as usize] = 256 as libc::c_int as size_t;
    gdim[1 as libc::c_int as usize] = (*h).mb.i_mb_height as size_t;
    ldim[0 as libc::c_int as usize] = 256 as libc::c_int as size_t;
    ldim[1 as libc::c_int as usize] = 1 as libc::c_int as size_t;
    arg_0 = 0 as libc::c_int as cl_uint;
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh14 = arg_0;
    arg_0 = arg_0.wrapping_add(1);
    status = ((*ocl).clSetKernelArg)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.rowsum_intra_kernel,
        fresh14,
        ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
        &mut (*fenc).opencl.intra_cost as *mut cl_mem as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh15 = arg_0;
    arg_0 = arg_0.wrapping_add(1);
    status = ((*ocl).clSetKernelArg)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.rowsum_intra_kernel,
        fresh15,
        ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
        &mut (*fenc).opencl.inv_qscale_factor as *mut cl_mem as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh16 = arg_0;
    arg_0 = arg_0.wrapping_add(1);
    status = ((*ocl).clSetKernelArg)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.rowsum_intra_kernel,
        fresh16,
        ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
        &mut *((*h).opencl.row_satds).as_mut_ptr().offset((*h).opencl.last_buf as isize)
            as *mut cl_mem as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh17 = arg_0;
    arg_0 = arg_0.wrapping_add(1);
    status = ((*ocl).clSetKernelArg)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.rowsum_intra_kernel,
        fresh17,
        ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
        &mut *((*h).opencl.frame_stats)
            .as_mut_ptr()
            .offset((*h).opencl.last_buf as isize) as *mut cl_mem as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh18 = arg_0;
    arg_0 = arg_0.wrapping_add(1);
    status = ((*ocl).clSetKernelArg)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.rowsum_intra_kernel,
        fresh18,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        &mut (*h).mb.i_mb_width as *mut libc::c_int as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    status = ((*ocl).clEnqueueNDRangeKernel)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.queue,
        (*h).opencl.rowsum_intra_kernel,
        2 as libc::c_int as cl_uint,
        std::ptr::null::<size_t>(),
        gdim.as_mut_ptr(),
        ldim.as_mut_ptr(),
        0 as libc::c_int as cl_uint,
        std::ptr::null::<cl_event>(),
        std::ptr::null_mut::<cl_event>(),
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clEnqueueNDRangeKernel error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.num_copies >= 1024 as libc::c_int - 4 as libc::c_int {
        x264_8_opencl_flush(h);
    }
    let mut size_0: libc::c_int = ((*h).mb.i_mb_count as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong) as libc::c_int;
    locked = opencl_alloc_locked(h, size_0) as *mut libc::c_char;
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    status = ((*ocl).clEnqueueReadBuffer)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.queue,
        (*fenc).opencl.intra_cost,
        0 as libc::c_int as cl_bool,
        0 as libc::c_int as size_t,
        size_0 as size_t,
        locked as *mut libc::c_void,
        0 as libc::c_int as cl_uint,
        std::ptr::null::<cl_event>(),
        std::ptr::null_mut::<cl_event>(),
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clEnqueueReadBuffer error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    (*h)
        .opencl
        .copies[(*h).opencl.num_copies as usize]
        .dest = (*fenc)
        .lowres_costs[0 as libc::c_int as usize][0 as libc::c_int as usize]
        as *mut libc::c_void;
    (*h)
        .opencl
        .copies[(*h).opencl.num_copies as usize]
        .src = locked as *mut libc::c_void;
    (*h).opencl.copies[(*h).opencl.num_copies as usize].bytes = size_0;
    (*h).opencl.num_copies += 1;
    (*h).opencl.num_copies;
    size_0 = ((*h).mb.i_mb_height as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
        as libc::c_int;
    locked = opencl_alloc_locked(h, size_0) as *mut libc::c_char;
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    status = ((*ocl).clEnqueueReadBuffer)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.queue,
        (*h).opencl.row_satds[(*h).opencl.last_buf as usize],
        0 as libc::c_int as cl_bool,
        0 as libc::c_int as size_t,
        size_0 as size_t,
        locked as *mut libc::c_void,
        0 as libc::c_int as cl_uint,
        std::ptr::null::<cl_event>(),
        std::ptr::null_mut::<cl_event>(),
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clEnqueueReadBuffer error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    (*h)
        .opencl
        .copies[(*h).opencl.num_copies as usize]
        .dest = (*fenc).i_row_satds[0 as libc::c_int as usize][0 as libc::c_int as usize]
        as *mut libc::c_void;
    (*h)
        .opencl
        .copies[(*h).opencl.num_copies as usize]
        .src = locked as *mut libc::c_void;
    (*h).opencl.copies[(*h).opencl.num_copies as usize].bytes = size_0;
    (*h).opencl.num_copies += 1;
    (*h).opencl.num_copies;
    size_0 = (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
        .wrapping_mul(4 as libc::c_int as libc::c_ulong) as libc::c_int;
    locked = opencl_alloc_locked(h, size_0) as *mut libc::c_char;
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    status = ((*ocl).clEnqueueReadBuffer)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.queue,
        (*h).opencl.frame_stats[(*h).opencl.last_buf as usize],
        0 as libc::c_int as cl_bool,
        0 as libc::c_int as size_t,
        size_0 as size_t,
        locked as *mut libc::c_void,
        0 as libc::c_int as cl_uint,
        std::ptr::null::<cl_event>(),
        std::ptr::null_mut::<cl_event>(),
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clEnqueueReadBuffer error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    (*h)
        .opencl
        .copies[(*h).opencl.num_copies as usize]
        .dest = &mut *(*((*fenc).i_cost_est)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize))
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) as *mut libc::c_int as *mut libc::c_void;
    (*h)
        .opencl
        .copies[(*h).opencl.num_copies as usize]
        .src = locked as *mut libc::c_void;
    (*h)
        .opencl
        .copies[(*h).opencl.num_copies as usize]
        .bytes = ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int;
    (*h).opencl.num_copies += 1;
    (*h).opencl.num_copies;
    (*h)
        .opencl
        .copies[(*h).opencl.num_copies as usize]
        .dest = &mut *(*((*fenc).i_cost_est_aq)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize))
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) as *mut libc::c_int as *mut libc::c_void;
    (*h)
        .opencl
        .copies[(*h).opencl.num_copies as usize]
        .src = locked
        .offset(::core::mem::size_of::<libc::c_int>() as libc::c_ulong as isize)
        as *mut libc::c_void;
    (*h)
        .opencl
        .copies[(*h).opencl.num_copies as usize]
        .bytes = ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int;
    (*h).opencl.num_copies += 1;
    (*h).opencl.num_copies;
    (*h).opencl.last_buf = ((*h).opencl.last_buf == 0) as libc::c_int;
    0 as libc::c_int
}
unsafe extern "C" fn optimal_launch_dims(
    mut h: *mut x264_t,
    mut gdims: *mut size_t,
    mut ldims: *mut size_t,
    kernel: cl_kernel,
    device: cl_device_id,
) {
    let mut ocl: *mut x264_opencl_function_t = (*h).opencl.ocl;
    let mut max_work_group: size_t = 256 as libc::c_int as size_t;
    let mut preferred_multiple: size_t = 64 as libc::c_int as size_t;
    let mut num_cus: cl_uint = 6 as libc::c_int as cl_uint;
    ((*ocl).clGetKernelWorkGroupInfo)
        .expect(
            "non-null function pointer",
        )(
        kernel,
        device,
        0x11b0 as libc::c_int as cl_kernel_work_group_info,
        ::core::mem::size_of::<size_t>() as libc::c_ulong,
        &mut max_work_group as *mut size_t as *mut libc::c_void,
        std::ptr::null_mut::<size_t>(),
    );
    ((*ocl).clGetKernelWorkGroupInfo)
        .expect(
            "non-null function pointer",
        )(
        kernel,
        device,
        0x11b3 as libc::c_int as cl_kernel_work_group_info,
        ::core::mem::size_of::<size_t>() as libc::c_ulong,
        &mut preferred_multiple as *mut size_t as *mut libc::c_void,
        std::ptr::null_mut::<size_t>(),
    );
    ((*ocl).clGetDeviceInfo)
        .expect(
            "non-null function pointer",
        )(
        device,
        0x1002 as libc::c_int as cl_device_info,
        ::core::mem::size_of::<cl_uint>() as libc::c_ulong,
        &mut num_cus as *mut cl_uint as *mut libc::c_void,
        std::ptr::null_mut::<size_t>(),
    );
    *ldims.offset(0 as libc::c_int as isize) = preferred_multiple;
    *ldims.offset(1 as libc::c_int as isize) = 8 as libc::c_int as size_t;
    while *gdims.offset(1 as libc::c_int as isize)
        & (*ldims.offset(1 as libc::c_int as isize))
            .wrapping_sub(1 as libc::c_int as size_t) != 0
    {
        *ldims.offset(0 as libc::c_int as isize) <<= 1 as libc::c_int;
        *ldims.offset(1 as libc::c_int as isize) >>= 1 as libc::c_int;
    }
    while *ldims.offset(0 as libc::c_int as isize)
        * *ldims.offset(1 as libc::c_int as isize) > max_work_group
    {
        if *ldims.offset(0 as libc::c_int as isize) <= preferred_multiple
            && *ldims.offset(1 as libc::c_int as isize) > 1 as libc::c_int as size_t
        {
            *ldims.offset(1 as libc::c_int as isize) >>= 1 as libc::c_int;
        } else {
            *ldims.offset(0 as libc::c_int as isize) >>= 1 as libc::c_int;
        }
    }
    if *ldims.offset(0 as libc::c_int as isize)
        > *gdims.offset(0 as libc::c_int as isize)
    {
        while (*gdims.offset(0 as libc::c_int as isize)).wrapping_add(preferred_multiple)
            < *ldims.offset(0 as libc::c_int as isize)
        {
            let fresh19 = &mut (*ldims.offset(0 as libc::c_int as isize));
            *fresh19 = (*fresh19).wrapping_sub(preferred_multiple);
        }
        *gdims
            .offset(
                0 as libc::c_int as isize,
            ) = *ldims.offset(0 as libc::c_int as isize);
    } else {
        *gdims
            .offset(
                0 as libc::c_int as isize,
            ) = (*gdims.offset(0 as libc::c_int as isize))
            .wrapping_add(*ldims.offset(0 as libc::c_int as isize))
            .wrapping_sub(1 as libc::c_int as size_t)
            / *ldims.offset(0 as libc::c_int as isize);
        let fresh20 = &mut (*gdims.offset(0 as libc::c_int as isize));
        *fresh20 *= *ldims.offset(0 as libc::c_int as isize);
    }
    while *gdims.offset(0 as libc::c_int as isize)
        / *ldims.offset(0 as libc::c_int as isize)
        * (*gdims.offset(1 as libc::c_int as isize)
            / *ldims.offset(1 as libc::c_int as isize)) * 2 as libc::c_int as size_t
        <= num_cus as size_t
    {
        if *ldims.offset(0 as libc::c_int as isize) > preferred_multiple {
            *ldims.offset(0 as libc::c_int as isize) >>= 1 as libc::c_int;
        } else {
            if *ldims.offset(1 as libc::c_int as isize) <= 1 as libc::c_int as size_t {
                break;
            }
            *ldims.offset(1 as libc::c_int as isize) >>= 1 as libc::c_int;
        }
    }
    if num_cus == 6 as libc::c_int as cl_uint
        && *ldims.offset(0 as libc::c_int as isize) == 64 as libc::c_int as size_t
        && *ldims.offset(1 as libc::c_int as isize) == 4 as libc::c_int as size_t
    {
        *ldims.offset(0 as libc::c_int as isize) = 32 as libc::c_int as size_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_opencl_motionsearch(
    mut h: *mut x264_t,
    mut frames: *mut *mut x264_frame_t,
    mut b: libc::c_int,
    mut ref_0: libc::c_int,
    mut b_islist1: libc::c_int,
    mut lambda: libc::c_int,
    mut w: *const x264_weight_t,
) -> libc::c_int {
    let mut ocl: *mut x264_opencl_function_t = (*h).opencl.ocl;
    let mut fenc: *mut x264_frame_t = *frames.offset(b as isize);
    let mut fref: *mut x264_frame_t = *frames.offset(ref_0 as isize);
    let mut ref_scaled_images: [cl_mem; 4] = [std::ptr::null_mut::<_cl_mem>(); 4];
    let mut ref_luma_hpel: cl_mem = std::ptr::null_mut::<_cl_mem>();
    let mut status: cl_int = 0;
    if !w.is_null() && !((*w).weightfn).is_null() {
        let mut gdims: [size_t; 2] = [0; 2];
        gdims[0 as libc::c_int
            as usize] = (8 as libc::c_int * (*h).mb.i_mb_width) as size_t;
        gdims[1 as libc::c_int
            as usize] = (8 as libc::c_int * (*h).mb.i_mb_height) as size_t;
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            let mut arg: cl_uint = 0 as libc::c_int as cl_uint;
            if (*h).opencl.b_fatal_error != 0 {
                return -(1 as libc::c_int);
            }
            let fresh21 = arg;
            arg = arg.wrapping_add(1);
            status = ((*ocl).clSetKernelArg)
                .expect(
                    "non-null function pointer",
                )(
                (*h).opencl.weightp_scaled_images_kernel,
                fresh21,
                ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
                &mut *((*fref).opencl.scaled_image2Ds).as_mut_ptr().offset(i as isize)
                    as *mut cl_mem as *const libc::c_void,
            );
            if status != 0 as libc::c_int {
                (*h).param.b_opencl = 0 as libc::c_int;
                (*h).opencl.b_fatal_error = 1 as libc::c_int;
                x264_8_log(
                    h,
                    0 as libc::c_int,
                    b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                    status,
                );
                return -(1 as libc::c_int);
            }
            if (*h).opencl.b_fatal_error != 0 {
                return -(1 as libc::c_int);
            }
            let fresh22 = arg;
            arg = arg.wrapping_add(1);
            status = ((*ocl).clSetKernelArg)
                .expect(
                    "non-null function pointer",
                )(
                (*h).opencl.weightp_scaled_images_kernel,
                fresh22,
                ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
                &mut *((*h).opencl.weighted_scaled_images)
                    .as_mut_ptr()
                    .offset(i as isize) as *mut cl_mem as *const libc::c_void,
            );
            if status != 0 as libc::c_int {
                (*h).param.b_opencl = 0 as libc::c_int;
                (*h).opencl.b_fatal_error = 1 as libc::c_int;
                x264_8_log(
                    h,
                    0 as libc::c_int,
                    b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                    status,
                );
                return -(1 as libc::c_int);
            }
            if (*h).opencl.b_fatal_error != 0 {
                return -(1 as libc::c_int);
            }
            let fresh23 = arg;
            arg = arg.wrapping_add(1);
            status = ((*ocl).clSetKernelArg)
                .expect(
                    "non-null function pointer",
                )(
                (*h).opencl.weightp_scaled_images_kernel,
                fresh23,
                ::core::mem::size_of::<int32_t>() as libc::c_ulong,
                &(*w).i_offset as *const int32_t as *const libc::c_void,
            );
            if status != 0 as libc::c_int {
                (*h).param.b_opencl = 0 as libc::c_int;
                (*h).opencl.b_fatal_error = 1 as libc::c_int;
                x264_8_log(
                    h,
                    0 as libc::c_int,
                    b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                    status,
                );
                return -(1 as libc::c_int);
            }
            if (*h).opencl.b_fatal_error != 0 {
                return -(1 as libc::c_int);
            }
            let fresh24 = arg;
            arg = arg.wrapping_add(1);
            status = ((*ocl).clSetKernelArg)
                .expect(
                    "non-null function pointer",
                )(
                (*h).opencl.weightp_scaled_images_kernel,
                fresh24,
                ::core::mem::size_of::<int32_t>() as libc::c_ulong,
                &(*w).i_scale as *const int32_t as *const libc::c_void,
            );
            if status != 0 as libc::c_int {
                (*h).param.b_opencl = 0 as libc::c_int;
                (*h).opencl.b_fatal_error = 1 as libc::c_int;
                x264_8_log(
                    h,
                    0 as libc::c_int,
                    b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                    status,
                );
                return -(1 as libc::c_int);
            }
            if (*h).opencl.b_fatal_error != 0 {
                return -(1 as libc::c_int);
            }
            let fresh25 = arg;
            arg = arg.wrapping_add(1);
            status = ((*ocl).clSetKernelArg)
                .expect(
                    "non-null function pointer",
                )(
                (*h).opencl.weightp_scaled_images_kernel,
                fresh25,
                ::core::mem::size_of::<int32_t>() as libc::c_ulong,
                &(*w).i_denom as *const int32_t as *const libc::c_void,
            );
            if status != 0 as libc::c_int {
                (*h).param.b_opencl = 0 as libc::c_int;
                (*h).opencl.b_fatal_error = 1 as libc::c_int;
                x264_8_log(
                    h,
                    0 as libc::c_int,
                    b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                    status,
                );
                return -(1 as libc::c_int);
            }
            if (*h).opencl.b_fatal_error != 0 {
                return -(1 as libc::c_int);
            }
            status = ((*ocl).clEnqueueNDRangeKernel)
                .expect(
                    "non-null function pointer",
                )(
                (*h).opencl.queue,
                (*h).opencl.weightp_scaled_images_kernel,
                2 as libc::c_int as cl_uint,
                std::ptr::null::<size_t>(),
                gdims.as_mut_ptr(),
                std::ptr::null::<size_t>(),
                0 as libc::c_int as cl_uint,
                std::ptr::null::<cl_event>(),
                std::ptr::null_mut::<cl_event>(),
            );
            if status != 0 as libc::c_int {
                (*h).param.b_opencl = 0 as libc::c_int;
                (*h).opencl.b_fatal_error = 1 as libc::c_int;
                x264_8_log(
                    h,
                    0 as libc::c_int,
                    b"clEnqueueNDRangeKernel error '%d'\n\0" as *const u8
                        as *const libc::c_char,
                    status,
                );
                return -(1 as libc::c_int);
            }
            gdims[0 as libc::c_int as usize] >>= 1 as libc::c_int;
            gdims[1 as libc::c_int as usize] >>= 1 as libc::c_int;
            if gdims[0 as libc::c_int as usize] < 16 as libc::c_int as size_t
                || gdims[1 as libc::c_int as usize] < 16 as libc::c_int as size_t
            {
                break;
            }
            i += 1;
            i;
        }
        let mut arg_0: cl_uint = 0 as libc::c_int as cl_uint;
        gdims[0 as libc::c_int
            as usize] = (8 as libc::c_int * (*h).mb.i_mb_width) as size_t;
        gdims[1 as libc::c_int
            as usize] = (8 as libc::c_int * (*h).mb.i_mb_height) as size_t;
        if (*h).opencl.b_fatal_error != 0 {
            return -(1 as libc::c_int);
        }
        let fresh26 = arg_0;
        arg_0 = arg_0.wrapping_add(1);
        status = ((*ocl).clSetKernelArg)
            .expect(
                "non-null function pointer",
            )(
            (*h).opencl.weightp_hpel_kernel,
            fresh26,
            ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
            &mut (*fref).opencl.luma_hpel as *mut cl_mem as *const libc::c_void,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            (*h).opencl.b_fatal_error = 1 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        if (*h).opencl.b_fatal_error != 0 {
            return -(1 as libc::c_int);
        }
        let fresh27 = arg_0;
        arg_0 = arg_0.wrapping_add(1);
        status = ((*ocl).clSetKernelArg)
            .expect(
                "non-null function pointer",
            )(
            (*h).opencl.weightp_hpel_kernel,
            fresh27,
            ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
            &mut (*h).opencl.weighted_luma_hpel as *mut cl_mem as *const libc::c_void,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            (*h).opencl.b_fatal_error = 1 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        if (*h).opencl.b_fatal_error != 0 {
            return -(1 as libc::c_int);
        }
        let fresh28 = arg_0;
        arg_0 = arg_0.wrapping_add(1);
        status = ((*ocl).clSetKernelArg)
            .expect(
                "non-null function pointer",
            )(
            (*h).opencl.weightp_hpel_kernel,
            fresh28,
            ::core::mem::size_of::<int32_t>() as libc::c_ulong,
            &(*w).i_offset as *const int32_t as *const libc::c_void,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            (*h).opencl.b_fatal_error = 1 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        if (*h).opencl.b_fatal_error != 0 {
            return -(1 as libc::c_int);
        }
        let fresh29 = arg_0;
        arg_0 = arg_0.wrapping_add(1);
        status = ((*ocl).clSetKernelArg)
            .expect(
                "non-null function pointer",
            )(
            (*h).opencl.weightp_hpel_kernel,
            fresh29,
            ::core::mem::size_of::<int32_t>() as libc::c_ulong,
            &(*w).i_scale as *const int32_t as *const libc::c_void,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            (*h).opencl.b_fatal_error = 1 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        if (*h).opencl.b_fatal_error != 0 {
            return -(1 as libc::c_int);
        }
        let fresh30 = arg_0;
        arg_0 = arg_0.wrapping_add(1);
        status = ((*ocl).clSetKernelArg)
            .expect(
                "non-null function pointer",
            )(
            (*h).opencl.weightp_hpel_kernel,
            fresh30,
            ::core::mem::size_of::<int32_t>() as libc::c_ulong,
            &(*w).i_denom as *const int32_t as *const libc::c_void,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            (*h).opencl.b_fatal_error = 1 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        if (*h).opencl.b_fatal_error != 0 {
            return -(1 as libc::c_int);
        }
        status = ((*ocl).clEnqueueNDRangeKernel)
            .expect(
                "non-null function pointer",
            )(
            (*h).opencl.queue,
            (*h).opencl.weightp_hpel_kernel,
            2 as libc::c_int as cl_uint,
            std::ptr::null::<size_t>(),
            gdims.as_mut_ptr(),
            std::ptr::null::<size_t>(),
            0 as libc::c_int as cl_uint,
            std::ptr::null::<cl_event>(),
            std::ptr::null_mut::<cl_event>(),
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            (*h).opencl.b_fatal_error = 1 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clEnqueueNDRangeKernel error '%d'\n\0" as *const u8
                    as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < 4 as libc::c_int {
            ref_scaled_images[i_0
                as usize] = (*h).opencl.weighted_scaled_images[i_0 as usize];
            i_0 += 1;
            i_0;
        }
        ref_luma_hpel = (*h).opencl.weighted_luma_hpel;
    } else {
        let mut i_1: libc::c_int = 0 as libc::c_int;
        while i_1 < 4 as libc::c_int {
            ref_scaled_images[i_1
                as usize] = (*fref).opencl.scaled_image2Ds[i_1 as usize];
            i_1 += 1;
            i_1;
        }
        ref_luma_hpel = (*fref).opencl.luma_hpel;
    }
    let num_iterations: [libc::c_int; 4] = [
        1 as libc::c_int,
        1 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
    ];
    let mut b_first_iteration: libc::c_int = 1 as libc::c_int;
    let mut b_reverse_references: libc::c_int = 1 as libc::c_int;
    let mut A: libc::c_int = 1 as libc::c_int;
    let mut mb_per_group: libc::c_int = 0 as libc::c_int;
    let mut cost_local_size: libc::c_int = 0 as libc::c_int;
    let mut mvc_local_size: libc::c_int = 0 as libc::c_int;
    let mut mb_width: libc::c_int = 0;
    let mut gdims_0: [size_t; 2] = [0; 2];
    let mut ldims: [size_t; 2] = [0; 2];
    let mut scale: libc::c_int = 4 as libc::c_int - 1 as libc::c_int;
    while scale >= 0 as libc::c_int {
        mb_width = (*h).mb.i_mb_width >> scale;
        gdims_0[0 as libc::c_int as usize] = mb_width as size_t;
        gdims_0[1 as libc::c_int as usize] = ((*h).mb.i_mb_height >> scale) as size_t;
        if !(gdims_0[0 as libc::c_int as usize] < 2 as libc::c_int as size_t
            || gdims_0[1 as libc::c_int as usize] < 2 as libc::c_int as size_t)
        {
            gdims_0[0 as libc::c_int as usize] <<= 2 as libc::c_int;
            optimal_launch_dims(
                h,
                gdims_0.as_mut_ptr(),
                ldims.as_mut_ptr(),
                (*h).opencl.hme_kernel,
                (*h).opencl.device,
            );
            mb_per_group = ((ldims[0 as libc::c_int as usize] >> 2 as libc::c_int)
                * ldims[1 as libc::c_int as usize]) as libc::c_int;
            cost_local_size = ((4 as libc::c_int * mb_per_group) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong)
                as libc::c_int;
            mvc_local_size = ((4 as libc::c_int * mb_per_group) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong) as libc::c_int;
            let mut scaled_me_range: libc::c_int = (*h).param.analyse.i_me_range
                >> scale;
            let mut b_shift_index: libc::c_int = 1 as libc::c_int;
            let mut arg_1: cl_uint = 0 as libc::c_int as cl_uint;
            if (*h).opencl.b_fatal_error != 0 {
                return -(1 as libc::c_int);
            }
            let fresh31 = arg_1;
            arg_1 = arg_1.wrapping_add(1);
            status = ((*ocl).clSetKernelArg)
                .expect(
                    "non-null function pointer",
                )(
                (*h).opencl.hme_kernel,
                fresh31,
                ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
                &mut *((*fenc).opencl.scaled_image2Ds)
                    .as_mut_ptr()
                    .offset(scale as isize) as *mut cl_mem as *const libc::c_void,
            );
            if status != 0 as libc::c_int {
                (*h).param.b_opencl = 0 as libc::c_int;
                (*h).opencl.b_fatal_error = 1 as libc::c_int;
                x264_8_log(
                    h,
                    0 as libc::c_int,
                    b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                    status,
                );
                return -(1 as libc::c_int);
            }
            if (*h).opencl.b_fatal_error != 0 {
                return -(1 as libc::c_int);
            }
            let fresh32 = arg_1;
            arg_1 = arg_1.wrapping_add(1);
            status = ((*ocl).clSetKernelArg)
                .expect(
                    "non-null function pointer",
                )(
                (*h).opencl.hme_kernel,
                fresh32,
                ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
                &mut *ref_scaled_images.as_mut_ptr().offset(scale as isize)
                    as *mut cl_mem as *const libc::c_void,
            );
            if status != 0 as libc::c_int {
                (*h).param.b_opencl = 0 as libc::c_int;
                (*h).opencl.b_fatal_error = 1 as libc::c_int;
                x264_8_log(
                    h,
                    0 as libc::c_int,
                    b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                    status,
                );
                return -(1 as libc::c_int);
            }
            if (*h).opencl.b_fatal_error != 0 {
                return -(1 as libc::c_int);
            }
            let fresh33 = arg_1;
            arg_1 = arg_1.wrapping_add(1);
            status = ((*ocl).clSetKernelArg)
                .expect(
                    "non-null function pointer",
                )(
                (*h).opencl.hme_kernel,
                fresh33,
                ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
                &mut *((*h).opencl.mv_buffers).as_mut_ptr().offset(A as isize)
                    as *mut cl_mem as *const libc::c_void,
            );
            if status != 0 as libc::c_int {
                (*h).param.b_opencl = 0 as libc::c_int;
                (*h).opencl.b_fatal_error = 1 as libc::c_int;
                x264_8_log(
                    h,
                    0 as libc::c_int,
                    b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                    status,
                );
                return -(1 as libc::c_int);
            }
            if (*h).opencl.b_fatal_error != 0 {
                return -(1 as libc::c_int);
            }
            let fresh34 = arg_1;
            arg_1 = arg_1.wrapping_add(1);
            status = ((*ocl).clSetKernelArg)
                .expect(
                    "non-null function pointer",
                )(
                (*h).opencl.hme_kernel,
                fresh34,
                ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
                &mut *((*h).opencl.mv_buffers)
                    .as_mut_ptr()
                    .offset((A == 0) as libc::c_int as isize) as *mut cl_mem
                    as *const libc::c_void,
            );
            if status != 0 as libc::c_int {
                (*h).param.b_opencl = 0 as libc::c_int;
                (*h).opencl.b_fatal_error = 1 as libc::c_int;
                x264_8_log(
                    h,
                    0 as libc::c_int,
                    b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                    status,
                );
                return -(1 as libc::c_int);
            }
            if (*h).opencl.b_fatal_error != 0 {
                return -(1 as libc::c_int);
            }
            let fresh35 = arg_1;
            arg_1 = arg_1.wrapping_add(1);
            status = ((*ocl).clSetKernelArg)
                .expect(
                    "non-null function pointer",
                )(
                (*h).opencl.hme_kernel,
                fresh35,
                ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
                &mut (*h).opencl.lowres_mv_costs as *mut cl_mem as *const libc::c_void,
            );
            if status != 0 as libc::c_int {
                (*h).param.b_opencl = 0 as libc::c_int;
                (*h).opencl.b_fatal_error = 1 as libc::c_int;
                x264_8_log(
                    h,
                    0 as libc::c_int,
                    b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                    status,
                );
                return -(1 as libc::c_int);
            }
            if (*h).opencl.b_fatal_error != 0 {
                return -(1 as libc::c_int);
            }
            let fresh36 = arg_1;
            arg_1 = arg_1.wrapping_add(1);
            status = ((*ocl).clSetKernelArg)
                .expect(
                    "non-null function pointer",
                )(
                (*h).opencl.hme_kernel,
                fresh36,
                ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
                &mut (*h).opencl.mvp_buffer as *mut cl_mem as *mut libc::c_void,
            );
            if status != 0 as libc::c_int {
                (*h).param.b_opencl = 0 as libc::c_int;
                (*h).opencl.b_fatal_error = 1 as libc::c_int;
                x264_8_log(
                    h,
                    0 as libc::c_int,
                    b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                    status,
                );
                return -(1 as libc::c_int);
            }
            if (*h).opencl.b_fatal_error != 0 {
                return -(1 as libc::c_int);
            }
            let fresh37 = arg_1;
            arg_1 = arg_1.wrapping_add(1);
            status = ((*ocl).clSetKernelArg)
                .expect(
                    "non-null function pointer",
                )(
                (*h).opencl.hme_kernel,
                fresh37,
                cost_local_size as size_t,
                std::ptr::null::<libc::c_void>(),
            );
            if status != 0 as libc::c_int {
                (*h).param.b_opencl = 0 as libc::c_int;
                (*h).opencl.b_fatal_error = 1 as libc::c_int;
                x264_8_log(
                    h,
                    0 as libc::c_int,
                    b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                    status,
                );
                return -(1 as libc::c_int);
            }
            if (*h).opencl.b_fatal_error != 0 {
                return -(1 as libc::c_int);
            }
            let fresh38 = arg_1;
            arg_1 = arg_1.wrapping_add(1);
            status = ((*ocl).clSetKernelArg)
                .expect(
                    "non-null function pointer",
                )(
                (*h).opencl.hme_kernel,
                fresh38,
                mvc_local_size as size_t,
                std::ptr::null::<libc::c_void>(),
            );
            if status != 0 as libc::c_int {
                (*h).param.b_opencl = 0 as libc::c_int;
                (*h).opencl.b_fatal_error = 1 as libc::c_int;
                x264_8_log(
                    h,
                    0 as libc::c_int,
                    b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                    status,
                );
                return -(1 as libc::c_int);
            }
            if (*h).opencl.b_fatal_error != 0 {
                return -(1 as libc::c_int);
            }
            let fresh39 = arg_1;
            arg_1 = arg_1.wrapping_add(1);
            status = ((*ocl).clSetKernelArg)
                .expect(
                    "non-null function pointer",
                )(
                (*h).opencl.hme_kernel,
                fresh39,
                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                &mut mb_width as *mut libc::c_int as *const libc::c_void,
            );
            if status != 0 as libc::c_int {
                (*h).param.b_opencl = 0 as libc::c_int;
                (*h).opencl.b_fatal_error = 1 as libc::c_int;
                x264_8_log(
                    h,
                    0 as libc::c_int,
                    b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                    status,
                );
                return -(1 as libc::c_int);
            }
            if (*h).opencl.b_fatal_error != 0 {
                return -(1 as libc::c_int);
            }
            let fresh40 = arg_1;
            arg_1 = arg_1.wrapping_add(1);
            status = ((*ocl).clSetKernelArg)
                .expect(
                    "non-null function pointer",
                )(
                (*h).opencl.hme_kernel,
                fresh40,
                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                &mut lambda as *mut libc::c_int as *const libc::c_void,
            );
            if status != 0 as libc::c_int {
                (*h).param.b_opencl = 0 as libc::c_int;
                (*h).opencl.b_fatal_error = 1 as libc::c_int;
                x264_8_log(
                    h,
                    0 as libc::c_int,
                    b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                    status,
                );
                return -(1 as libc::c_int);
            }
            if (*h).opencl.b_fatal_error != 0 {
                return -(1 as libc::c_int);
            }
            let fresh41 = arg_1;
            arg_1 = arg_1.wrapping_add(1);
            status = ((*ocl).clSetKernelArg)
                .expect(
                    "non-null function pointer",
                )(
                (*h).opencl.hme_kernel,
                fresh41,
                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                &mut scaled_me_range as *mut libc::c_int as *const libc::c_void,
            );
            if status != 0 as libc::c_int {
                (*h).param.b_opencl = 0 as libc::c_int;
                (*h).opencl.b_fatal_error = 1 as libc::c_int;
                x264_8_log(
                    h,
                    0 as libc::c_int,
                    b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                    status,
                );
                return -(1 as libc::c_int);
            }
            if (*h).opencl.b_fatal_error != 0 {
                return -(1 as libc::c_int);
            }
            let fresh42 = arg_1;
            arg_1 = arg_1.wrapping_add(1);
            status = ((*ocl).clSetKernelArg)
                .expect(
                    "non-null function pointer",
                )(
                (*h).opencl.hme_kernel,
                fresh42,
                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                &mut scale as *mut libc::c_int as *const libc::c_void,
            );
            if status != 0 as libc::c_int {
                (*h).param.b_opencl = 0 as libc::c_int;
                (*h).opencl.b_fatal_error = 1 as libc::c_int;
                x264_8_log(
                    h,
                    0 as libc::c_int,
                    b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                    status,
                );
                return -(1 as libc::c_int);
            }
            if (*h).opencl.b_fatal_error != 0 {
                return -(1 as libc::c_int);
            }
            let fresh43 = arg_1;
            arg_1 = arg_1.wrapping_add(1);
            status = ((*ocl).clSetKernelArg)
                .expect(
                    "non-null function pointer",
                )(
                (*h).opencl.hme_kernel,
                fresh43,
                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                &mut b_shift_index as *mut libc::c_int as *const libc::c_void,
            );
            if status != 0 as libc::c_int {
                (*h).param.b_opencl = 0 as libc::c_int;
                (*h).opencl.b_fatal_error = 1 as libc::c_int;
                x264_8_log(
                    h,
                    0 as libc::c_int,
                    b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                    status,
                );
                return -(1 as libc::c_int);
            }
            if (*h).opencl.b_fatal_error != 0 {
                return -(1 as libc::c_int);
            }
            let fresh44 = arg_1;
            arg_1 = arg_1.wrapping_add(1);
            status = ((*ocl).clSetKernelArg)
                .expect(
                    "non-null function pointer",
                )(
                (*h).opencl.hme_kernel,
                fresh44,
                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                &mut b_first_iteration as *mut libc::c_int as *const libc::c_void,
            );
            if status != 0 as libc::c_int {
                (*h).param.b_opencl = 0 as libc::c_int;
                (*h).opencl.b_fatal_error = 1 as libc::c_int;
                x264_8_log(
                    h,
                    0 as libc::c_int,
                    b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                    status,
                );
                return -(1 as libc::c_int);
            }
            if (*h).opencl.b_fatal_error != 0 {
                return -(1 as libc::c_int);
            }
            let fresh45 = arg_1;
            arg_1 = arg_1.wrapping_add(1);
            status = ((*ocl).clSetKernelArg)
                .expect(
                    "non-null function pointer",
                )(
                (*h).opencl.hme_kernel,
                fresh45,
                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                &mut b_reverse_references as *mut libc::c_int as *const libc::c_void,
            );
            if status != 0 as libc::c_int {
                (*h).param.b_opencl = 0 as libc::c_int;
                (*h).opencl.b_fatal_error = 1 as libc::c_int;
                x264_8_log(
                    h,
                    0 as libc::c_int,
                    b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                    status,
                );
                return -(1 as libc::c_int);
            }
            let mut iter: libc::c_int = 0 as libc::c_int;
            while iter < num_iterations[scale as usize] {
                if (*h).opencl.b_fatal_error != 0 {
                    return -(1 as libc::c_int);
                }
                status = ((*ocl).clEnqueueNDRangeKernel)
                    .expect(
                        "non-null function pointer",
                    )(
                    (*h).opencl.queue,
                    (*h).opencl.hme_kernel,
                    2 as libc::c_int as cl_uint,
                    std::ptr::null::<size_t>(),
                    gdims_0.as_mut_ptr(),
                    ldims.as_mut_ptr(),
                    0 as libc::c_int as cl_uint,
                    std::ptr::null::<cl_event>(),
                    std::ptr::null_mut::<cl_event>(),
                );
                if status != 0 as libc::c_int {
                    (*h).param.b_opencl = 0 as libc::c_int;
                    (*h).opencl.b_fatal_error = 1 as libc::c_int;
                    x264_8_log(
                        h,
                        0 as libc::c_int,
                        b"clEnqueueNDRangeKernel error '%d'\n\0" as *const u8
                            as *const libc::c_char,
                        status,
                    );
                    return -(1 as libc::c_int);
                }
                b_shift_index = 0 as libc::c_int;
                b_first_iteration = 0 as libc::c_int;
                if scale > 2 as libc::c_int {
                    b_reverse_references ^= 1 as libc::c_int;
                } else {
                    b_reverse_references = 0 as libc::c_int;
                }
                A = (A == 0) as libc::c_int;
                if (*h).opencl.b_fatal_error != 0 {
                    return -(1 as libc::c_int);
                }
                status = ((*ocl).clSetKernelArg)
                    .expect(
                        "non-null function pointer",
                    )(
                    (*h).opencl.hme_kernel,
                    2 as libc::c_int as cl_uint,
                    ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
                    &mut *((*h).opencl.mv_buffers).as_mut_ptr().offset(A as isize)
                        as *mut cl_mem as *const libc::c_void,
                );
                if status != 0 as libc::c_int {
                    (*h).param.b_opencl = 0 as libc::c_int;
                    (*h).opencl.b_fatal_error = 1 as libc::c_int;
                    x264_8_log(
                        h,
                        0 as libc::c_int,
                        b"clSetKernelArg error '%d'\n\0" as *const u8
                            as *const libc::c_char,
                        status,
                    );
                    return -(1 as libc::c_int);
                }
                if (*h).opencl.b_fatal_error != 0 {
                    return -(1 as libc::c_int);
                }
                status = ((*ocl).clSetKernelArg)
                    .expect(
                        "non-null function pointer",
                    )(
                    (*h).opencl.hme_kernel,
                    3 as libc::c_int as cl_uint,
                    ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
                    &mut *((*h).opencl.mv_buffers)
                        .as_mut_ptr()
                        .offset((A == 0) as libc::c_int as isize) as *mut cl_mem
                        as *const libc::c_void,
                );
                if status != 0 as libc::c_int {
                    (*h).param.b_opencl = 0 as libc::c_int;
                    (*h).opencl.b_fatal_error = 1 as libc::c_int;
                    x264_8_log(
                        h,
                        0 as libc::c_int,
                        b"clSetKernelArg error '%d'\n\0" as *const u8
                            as *const libc::c_char,
                        status,
                    );
                    return -(1 as libc::c_int);
                }
                if (*h).opencl.b_fatal_error != 0 {
                    return -(1 as libc::c_int);
                }
                status = ((*ocl).clSetKernelArg)
                    .expect(
                        "non-null function pointer",
                    )(
                    (*h).opencl.hme_kernel,
                    arg_1.wrapping_sub(3 as libc::c_int as cl_uint),
                    ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                    &mut b_shift_index as *mut libc::c_int as *const libc::c_void,
                );
                if status != 0 as libc::c_int {
                    (*h).param.b_opencl = 0 as libc::c_int;
                    (*h).opencl.b_fatal_error = 1 as libc::c_int;
                    x264_8_log(
                        h,
                        0 as libc::c_int,
                        b"clSetKernelArg error '%d'\n\0" as *const u8
                            as *const libc::c_char,
                        status,
                    );
                    return -(1 as libc::c_int);
                }
                if (*h).opencl.b_fatal_error != 0 {
                    return -(1 as libc::c_int);
                }
                status = ((*ocl).clSetKernelArg)
                    .expect(
                        "non-null function pointer",
                    )(
                    (*h).opencl.hme_kernel,
                    arg_1.wrapping_sub(2 as libc::c_int as cl_uint),
                    ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                    &mut b_first_iteration as *mut libc::c_int as *const libc::c_void,
                );
                if status != 0 as libc::c_int {
                    (*h).param.b_opencl = 0 as libc::c_int;
                    (*h).opencl.b_fatal_error = 1 as libc::c_int;
                    x264_8_log(
                        h,
                        0 as libc::c_int,
                        b"clSetKernelArg error '%d'\n\0" as *const u8
                            as *const libc::c_char,
                        status,
                    );
                    return -(1 as libc::c_int);
                }
                if (*h).opencl.b_fatal_error != 0 {
                    return -(1 as libc::c_int);
                }
                status = ((*ocl).clSetKernelArg)
                    .expect(
                        "non-null function pointer",
                    )(
                    (*h).opencl.hme_kernel,
                    arg_1.wrapping_sub(1 as libc::c_int as cl_uint),
                    ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                    &mut b_reverse_references as *mut libc::c_int as *const libc::c_void,
                );
                if status != 0 as libc::c_int {
                    (*h).param.b_opencl = 0 as libc::c_int;
                    (*h).opencl.b_fatal_error = 1 as libc::c_int;
                    x264_8_log(
                        h,
                        0 as libc::c_int,
                        b"clSetKernelArg error '%d'\n\0" as *const u8
                            as *const libc::c_char,
                        status,
                    );
                    return -(1 as libc::c_int);
                }
                iter += 1;
                iter;
            }
        }
        scale -= 1;
        scale;
    }
    let mut satd_local_size: libc::c_int = (mb_per_group as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<uint32_t>() as libc::c_ulong)
        .wrapping_mul(16 as libc::c_int as libc::c_ulong) as libc::c_int;
    let mut arg_2: cl_uint = 0 as libc::c_int as cl_uint;
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh46 = arg_2;
    arg_2 = arg_2.wrapping_add(1);
    status = ((*ocl).clSetKernelArg)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.subpel_refine_kernel,
        fresh46,
        ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
        &mut *((*fenc).opencl.scaled_image2Ds)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) as *mut cl_mem as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh47 = arg_2;
    arg_2 = arg_2.wrapping_add(1);
    status = ((*ocl).clSetKernelArg)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.subpel_refine_kernel,
        fresh47,
        ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
        &mut ref_luma_hpel as *mut cl_mem as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh48 = arg_2;
    arg_2 = arg_2.wrapping_add(1);
    status = ((*ocl).clSetKernelArg)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.subpel_refine_kernel,
        fresh48,
        ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
        &mut *((*h).opencl.mv_buffers).as_mut_ptr().offset(A as isize) as *mut cl_mem
            as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh49 = arg_2;
    arg_2 = arg_2.wrapping_add(1);
    status = ((*ocl).clSetKernelArg)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.subpel_refine_kernel,
        fresh49,
        ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
        &mut (*h).opencl.lowres_mv_costs as *mut cl_mem as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh50 = arg_2;
    arg_2 = arg_2.wrapping_add(1);
    status = ((*ocl).clSetKernelArg)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.subpel_refine_kernel,
        fresh50,
        cost_local_size as size_t,
        std::ptr::null::<libc::c_void>(),
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh51 = arg_2;
    arg_2 = arg_2.wrapping_add(1);
    status = ((*ocl).clSetKernelArg)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.subpel_refine_kernel,
        fresh51,
        satd_local_size as size_t,
        std::ptr::null::<libc::c_void>(),
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh52 = arg_2;
    arg_2 = arg_2.wrapping_add(1);
    status = ((*ocl).clSetKernelArg)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.subpel_refine_kernel,
        fresh52,
        mvc_local_size as size_t,
        std::ptr::null::<libc::c_void>(),
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if b_islist1 != 0 {
        if (*h).opencl.b_fatal_error != 0 {
            return -(1 as libc::c_int);
        }
        let fresh53 = arg_2;
        arg_2 = arg_2.wrapping_add(1);
        status = ((*ocl).clSetKernelArg)
            .expect(
                "non-null function pointer",
            )(
            (*h).opencl.subpel_refine_kernel,
            fresh53,
            ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
            &mut (*fenc).opencl.lowres_mvs1 as *mut cl_mem as *const libc::c_void,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            (*h).opencl.b_fatal_error = 1 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        if (*h).opencl.b_fatal_error != 0 {
            return -(1 as libc::c_int);
        }
        let fresh54 = arg_2;
        arg_2 = arg_2.wrapping_add(1);
        status = ((*ocl).clSetKernelArg)
            .expect(
                "non-null function pointer",
            )(
            (*h).opencl.subpel_refine_kernel,
            fresh54,
            ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
            &mut (*fenc).opencl.lowres_mv_costs1 as *mut cl_mem as *const libc::c_void,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            (*h).opencl.b_fatal_error = 1 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
    } else {
        if (*h).opencl.b_fatal_error != 0 {
            return -(1 as libc::c_int);
        }
        let fresh55 = arg_2;
        arg_2 = arg_2.wrapping_add(1);
        status = ((*ocl).clSetKernelArg)
            .expect(
                "non-null function pointer",
            )(
            (*h).opencl.subpel_refine_kernel,
            fresh55,
            ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
            &mut (*fenc).opencl.lowres_mvs0 as *mut cl_mem as *const libc::c_void,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            (*h).opencl.b_fatal_error = 1 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        if (*h).opencl.b_fatal_error != 0 {
            return -(1 as libc::c_int);
        }
        let fresh56 = arg_2;
        arg_2 = arg_2.wrapping_add(1);
        status = ((*ocl).clSetKernelArg)
            .expect(
                "non-null function pointer",
            )(
            (*h).opencl.subpel_refine_kernel,
            fresh56,
            ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
            &mut (*fenc).opencl.lowres_mv_costs0 as *mut cl_mem as *const libc::c_void,
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            (*h).opencl.b_fatal_error = 1 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh57 = arg_2;
    arg_2 = arg_2.wrapping_add(1);
    status = ((*ocl).clSetKernelArg)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.subpel_refine_kernel,
        fresh57,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        &mut mb_width as *mut libc::c_int as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh58 = arg_2;
    arg_2 = arg_2.wrapping_add(1);
    status = ((*ocl).clSetKernelArg)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.subpel_refine_kernel,
        fresh58,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        &mut lambda as *mut libc::c_int as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh59 = arg_2;
    arg_2 = arg_2.wrapping_add(1);
    status = ((*ocl).clSetKernelArg)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.subpel_refine_kernel,
        fresh59,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        &mut b as *mut libc::c_int as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh60 = arg_2;
    arg_2 = arg_2.wrapping_add(1);
    status = ((*ocl).clSetKernelArg)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.subpel_refine_kernel,
        fresh60,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        &mut ref_0 as *mut libc::c_int as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh61 = arg_2;
    arg_2 = arg_2.wrapping_add(1);
    status = ((*ocl).clSetKernelArg)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.subpel_refine_kernel,
        fresh61,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        &mut b_islist1 as *mut libc::c_int as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_device_AMD_SI != 0 {
        if (*h).opencl.b_fatal_error != 0 {
            return -(1 as libc::c_int);
        }
        status = ((*ocl).clEnqueueCopyBuffer)
            .expect(
                "non-null function pointer",
            )(
            (*h).opencl.queue,
            (*h).opencl.mv_buffers[A as usize],
            (*h).opencl.mv_buffers[(A == 0) as libc::c_int as usize],
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            20 as libc::c_int as size_t,
            0 as libc::c_int as cl_uint,
            std::ptr::null::<cl_event>(),
            std::ptr::null_mut::<cl_event>(),
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            (*h).opencl.b_fatal_error = 1 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clEnqueueCopyBuffer error '%d'\n\0" as *const u8
                    as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    status = ((*ocl).clEnqueueNDRangeKernel)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.queue,
        (*h).opencl.subpel_refine_kernel,
        2 as libc::c_int as cl_uint,
        std::ptr::null::<size_t>(),
        gdims_0.as_mut_ptr(),
        ldims.as_mut_ptr(),
        0 as libc::c_int as cl_uint,
        std::ptr::null::<cl_event>(),
        std::ptr::null_mut::<cl_event>(),
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clEnqueueNDRangeKernel error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    let mut mvlen: libc::c_int = (2 as libc::c_int as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong)
        .wrapping_mul((*h).mb.i_mb_count as libc::c_ulong) as libc::c_int;
    if (*h).opencl.num_copies >= 1024 as libc::c_int - 1 as libc::c_int {
        x264_8_opencl_flush(h);
    }
    let mut locked: *mut libc::c_char = opencl_alloc_locked(h, mvlen)
        as *mut libc::c_char;
    (*h)
        .opencl
        .copies[(*h).opencl.num_copies as usize]
        .src = locked as *mut libc::c_void;
    (*h).opencl.copies[(*h).opencl.num_copies as usize].bytes = mvlen;
    if b_islist1 != 0 {
        let mut mvs_offset: libc::c_int = mvlen * (ref_0 - b - 1 as libc::c_int);
        if (*h).opencl.b_fatal_error != 0 {
            return -(1 as libc::c_int);
        }
        status = ((*ocl).clEnqueueReadBuffer)
            .expect(
                "non-null function pointer",
            )(
            (*h).opencl.queue,
            (*fenc).opencl.lowres_mvs1,
            0 as libc::c_int as cl_bool,
            mvs_offset as size_t,
            mvlen as size_t,
            locked as *mut libc::c_void,
            0 as libc::c_int as cl_uint,
            std::ptr::null::<cl_event>(),
            std::ptr::null_mut::<cl_event>(),
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            (*h).opencl.b_fatal_error = 1 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clEnqueueReadBuffer error '%d'\n\0" as *const u8
                    as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        (*h)
            .opencl
            .copies[(*h).opencl.num_copies as usize]
            .dest = (*fenc)
            .lowres_mvs[1 as libc::c_int
            as usize][(ref_0 - b - 1 as libc::c_int) as usize] as *mut libc::c_void;
    } else {
        let mut mvs_offset_0: libc::c_int = mvlen * (b - ref_0 - 1 as libc::c_int);
        if (*h).opencl.b_fatal_error != 0 {
            return -(1 as libc::c_int);
        }
        status = ((*ocl).clEnqueueReadBuffer)
            .expect(
                "non-null function pointer",
            )(
            (*h).opencl.queue,
            (*fenc).opencl.lowres_mvs0,
            0 as libc::c_int as cl_bool,
            mvs_offset_0 as size_t,
            mvlen as size_t,
            locked as *mut libc::c_void,
            0 as libc::c_int as cl_uint,
            std::ptr::null::<cl_event>(),
            std::ptr::null_mut::<cl_event>(),
        );
        if status != 0 as libc::c_int {
            (*h).param.b_opencl = 0 as libc::c_int;
            (*h).opencl.b_fatal_error = 1 as libc::c_int;
            x264_8_log(
                h,
                0 as libc::c_int,
                b"clEnqueueReadBuffer error '%d'\n\0" as *const u8
                    as *const libc::c_char,
                status,
            );
            return -(1 as libc::c_int);
        }
        (*h)
            .opencl
            .copies[(*h).opencl.num_copies as usize]
            .dest = (*fenc)
            .lowres_mvs[0 as libc::c_int
            as usize][(b - ref_0 - 1 as libc::c_int) as usize] as *mut libc::c_void;
    }
    (*h).opencl.num_copies += 1;
    (*h).opencl.num_copies;
    0 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_opencl_finalize_cost(
    mut h: *mut x264_t,
    mut lambda: libc::c_int,
    mut frames: *mut *mut x264_frame_t,
    mut p0: libc::c_int,
    mut p1: libc::c_int,
    mut b: libc::c_int,
    mut dist_scale_factor: libc::c_int,
) -> libc::c_int {
    let mut ocl: *mut x264_opencl_function_t = (*h).opencl.ocl;
    let mut status: cl_int = 0;
    let mut fenc: *mut x264_frame_t = *frames.offset(b as isize);
    let mut fref0: *mut x264_frame_t = *frames.offset(p0 as isize);
    let mut fref1: *mut x264_frame_t = *frames.offset(p1 as isize);
    let mut bipred_weight: libc::c_int = if (*h).param.analyse.b_weighted_bipred != 0 {
        64 as libc::c_int - (dist_scale_factor >> 2 as libc::c_int)
    } else {
        32 as libc::c_int
    };
    let mut gdims: [size_t; 2] = [
        (*h).mb.i_mb_width as size_t,
        (*h).mb.i_mb_height as size_t,
    ];
    let mut ldim_bidir: [size_t; 2] = [0; 2];
    let mut ldims: *mut size_t = std::ptr::null_mut::<size_t>();
    let mut cost_local_size: libc::c_int = 4 as libc::c_int;
    let mut satd_local_size: libc::c_int = 4 as libc::c_int;
    if b < p1 {
        ldims = ldim_bidir.as_mut_ptr();
        gdims[0 as libc::c_int as usize] <<= 2 as libc::c_int;
        optimal_launch_dims(
            h,
            gdims.as_mut_ptr(),
            ldims,
            (*h).opencl.mode_select_kernel,
            (*h).opencl.device,
        );
        let mut mb_per_group: libc::c_int = ((*ldims.offset(0 as libc::c_int as isize)
            >> 2 as libc::c_int) * *ldims.offset(1 as libc::c_int as isize))
            as libc::c_int;
        cost_local_size = ((4 as libc::c_int * mb_per_group) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong)
            as libc::c_int;
        satd_local_size = ((16 as libc::c_int * mb_per_group) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<uint32_t>() as libc::c_ulong)
            as libc::c_int;
    }
    let mut arg: cl_uint = 0 as libc::c_int as cl_uint;
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh62 = arg;
    arg = arg.wrapping_add(1);
    status = ((*ocl).clSetKernelArg)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.mode_select_kernel,
        fresh62,
        ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
        &mut *((*fenc).opencl.scaled_image2Ds)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) as *mut cl_mem as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh63 = arg;
    arg = arg.wrapping_add(1);
    status = ((*ocl).clSetKernelArg)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.mode_select_kernel,
        fresh63,
        ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
        &mut (*fref0).opencl.luma_hpel as *mut cl_mem as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh64 = arg;
    arg = arg.wrapping_add(1);
    status = ((*ocl).clSetKernelArg)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.mode_select_kernel,
        fresh64,
        ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
        &mut (*fref1).opencl.luma_hpel as *mut cl_mem as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh65 = arg;
    arg = arg.wrapping_add(1);
    status = ((*ocl).clSetKernelArg)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.mode_select_kernel,
        fresh65,
        ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
        &mut (*fenc).opencl.lowres_mvs0 as *mut cl_mem as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh66 = arg;
    arg = arg.wrapping_add(1);
    status = ((*ocl).clSetKernelArg)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.mode_select_kernel,
        fresh66,
        ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
        &mut (*fenc).opencl.lowres_mvs1 as *mut cl_mem as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh67 = arg;
    arg = arg.wrapping_add(1);
    status = ((*ocl).clSetKernelArg)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.mode_select_kernel,
        fresh67,
        ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
        &mut (*fref1).opencl.lowres_mvs0 as *mut cl_mem as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh68 = arg;
    arg = arg.wrapping_add(1);
    status = ((*ocl).clSetKernelArg)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.mode_select_kernel,
        fresh68,
        ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
        &mut (*fenc).opencl.lowres_mv_costs0 as *mut cl_mem as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh69 = arg;
    arg = arg.wrapping_add(1);
    status = ((*ocl).clSetKernelArg)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.mode_select_kernel,
        fresh69,
        ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
        &mut (*fenc).opencl.lowres_mv_costs1 as *mut cl_mem as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh70 = arg;
    arg = arg.wrapping_add(1);
    status = ((*ocl).clSetKernelArg)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.mode_select_kernel,
        fresh70,
        ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
        &mut (*fenc).opencl.intra_cost as *mut cl_mem as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh71 = arg;
    arg = arg.wrapping_add(1);
    status = ((*ocl).clSetKernelArg)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.mode_select_kernel,
        fresh71,
        ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
        &mut *((*h).opencl.lowres_costs)
            .as_mut_ptr()
            .offset((*h).opencl.last_buf as isize) as *mut cl_mem as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh72 = arg;
    arg = arg.wrapping_add(1);
    status = ((*ocl).clSetKernelArg)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.mode_select_kernel,
        fresh72,
        ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
        &mut *((*h).opencl.frame_stats)
            .as_mut_ptr()
            .offset((*h).opencl.last_buf as isize) as *mut cl_mem as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh73 = arg;
    arg = arg.wrapping_add(1);
    status = ((*ocl).clSetKernelArg)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.mode_select_kernel,
        fresh73,
        cost_local_size as size_t,
        std::ptr::null::<libc::c_void>(),
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh74 = arg;
    arg = arg.wrapping_add(1);
    status = ((*ocl).clSetKernelArg)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.mode_select_kernel,
        fresh74,
        satd_local_size as size_t,
        std::ptr::null::<libc::c_void>(),
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh75 = arg;
    arg = arg.wrapping_add(1);
    status = ((*ocl).clSetKernelArg)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.mode_select_kernel,
        fresh75,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        &mut (*h).mb.i_mb_width as *mut libc::c_int as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh76 = arg;
    arg = arg.wrapping_add(1);
    status = ((*ocl).clSetKernelArg)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.mode_select_kernel,
        fresh76,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        &mut bipred_weight as *mut libc::c_int as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh77 = arg;
    arg = arg.wrapping_add(1);
    status = ((*ocl).clSetKernelArg)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.mode_select_kernel,
        fresh77,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        &mut dist_scale_factor as *mut libc::c_int as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh78 = arg;
    arg = arg.wrapping_add(1);
    status = ((*ocl).clSetKernelArg)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.mode_select_kernel,
        fresh78,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        &mut b as *mut libc::c_int as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh79 = arg;
    arg = arg.wrapping_add(1);
    status = ((*ocl).clSetKernelArg)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.mode_select_kernel,
        fresh79,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        &mut p0 as *mut libc::c_int as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh80 = arg;
    arg = arg.wrapping_add(1);
    status = ((*ocl).clSetKernelArg)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.mode_select_kernel,
        fresh80,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        &mut p1 as *mut libc::c_int as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh81 = arg;
    arg = arg.wrapping_add(1);
    status = ((*ocl).clSetKernelArg)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.mode_select_kernel,
        fresh81,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        &mut lambda as *mut libc::c_int as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    status = ((*ocl).clEnqueueNDRangeKernel)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.queue,
        (*h).opencl.mode_select_kernel,
        2 as libc::c_int as cl_uint,
        std::ptr::null::<size_t>(),
        gdims.as_mut_ptr(),
        ldims,
        0 as libc::c_int as cl_uint,
        std::ptr::null::<cl_event>(),
        std::ptr::null_mut::<cl_event>(),
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clEnqueueNDRangeKernel error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    let mut gdim: [size_t; 2] = [
        256 as libc::c_int as size_t,
        (*h).mb.i_mb_height as size_t,
    ];
    let mut ldim: [size_t; 2] = [
        256 as libc::c_int as size_t,
        1 as libc::c_int as size_t,
    ];
    arg = 0 as libc::c_int as cl_uint;
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh82 = arg;
    arg = arg.wrapping_add(1);
    status = ((*ocl).clSetKernelArg)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.rowsum_inter_kernel,
        fresh82,
        ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
        &mut *((*h).opencl.lowres_costs)
            .as_mut_ptr()
            .offset((*h).opencl.last_buf as isize) as *mut cl_mem as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh83 = arg;
    arg = arg.wrapping_add(1);
    status = ((*ocl).clSetKernelArg)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.rowsum_inter_kernel,
        fresh83,
        ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
        &mut (*fenc).opencl.inv_qscale_factor as *mut cl_mem as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh84 = arg;
    arg = arg.wrapping_add(1);
    status = ((*ocl).clSetKernelArg)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.rowsum_inter_kernel,
        fresh84,
        ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
        &mut *((*h).opencl.row_satds).as_mut_ptr().offset((*h).opencl.last_buf as isize)
            as *mut cl_mem as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh85 = arg;
    arg = arg.wrapping_add(1);
    status = ((*ocl).clSetKernelArg)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.rowsum_inter_kernel,
        fresh85,
        ::core::mem::size_of::<cl_mem>() as libc::c_ulong,
        &mut *((*h).opencl.frame_stats)
            .as_mut_ptr()
            .offset((*h).opencl.last_buf as isize) as *mut cl_mem as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh86 = arg;
    arg = arg.wrapping_add(1);
    status = ((*ocl).clSetKernelArg)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.rowsum_inter_kernel,
        fresh86,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        &mut (*h).mb.i_mb_width as *mut libc::c_int as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh87 = arg;
    arg = arg.wrapping_add(1);
    status = ((*ocl).clSetKernelArg)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.rowsum_inter_kernel,
        fresh87,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        &mut (*h).param.i_bframe_bias as *mut libc::c_int as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh88 = arg;
    arg = arg.wrapping_add(1);
    status = ((*ocl).clSetKernelArg)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.rowsum_inter_kernel,
        fresh88,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        &mut b as *mut libc::c_int as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh89 = arg;
    arg = arg.wrapping_add(1);
    status = ((*ocl).clSetKernelArg)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.rowsum_inter_kernel,
        fresh89,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        &mut p0 as *mut libc::c_int as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    let fresh90 = arg;
    arg = arg.wrapping_add(1);
    status = ((*ocl).clSetKernelArg)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.rowsum_inter_kernel,
        fresh90,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        &mut p1 as *mut libc::c_int as *const libc::c_void,
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clSetKernelArg error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    status = ((*ocl).clEnqueueNDRangeKernel)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.queue,
        (*h).opencl.rowsum_inter_kernel,
        2 as libc::c_int as cl_uint,
        std::ptr::null::<size_t>(),
        gdim.as_mut_ptr(),
        ldim.as_mut_ptr(),
        0 as libc::c_int as cl_uint,
        std::ptr::null::<cl_event>(),
        std::ptr::null_mut::<cl_event>(),
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clEnqueueNDRangeKernel error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    if (*h).opencl.num_copies >= 1024 as libc::c_int - 4 as libc::c_int {
        x264_8_opencl_flush(h);
    }
    let mut size: libc::c_int = ((*h).mb.i_mb_count as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong) as libc::c_int;
    let mut locked: *mut libc::c_char = opencl_alloc_locked(h, size)
        as *mut libc::c_char;
    (*h)
        .opencl
        .copies[(*h).opencl.num_copies as usize]
        .src = locked as *mut libc::c_void;
    (*h)
        .opencl
        .copies[(*h).opencl.num_copies as usize]
        .dest = (*fenc).lowres_costs[(b - p0) as usize][(p1 - b) as usize]
        as *mut libc::c_void;
    (*h).opencl.copies[(*h).opencl.num_copies as usize].bytes = size;
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    status = ((*ocl).clEnqueueReadBuffer)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.queue,
        (*h).opencl.lowres_costs[(*h).opencl.last_buf as usize],
        0 as libc::c_int as cl_bool,
        0 as libc::c_int as size_t,
        size as size_t,
        locked as *mut libc::c_void,
        0 as libc::c_int as cl_uint,
        std::ptr::null::<cl_event>(),
        std::ptr::null_mut::<cl_event>(),
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clEnqueueReadBuffer error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    (*h).opencl.num_copies += 1;
    (*h).opencl.num_copies;
    size = ((*h).mb.i_mb_height as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
        as libc::c_int;
    locked = opencl_alloc_locked(h, size) as *mut libc::c_char;
    (*h)
        .opencl
        .copies[(*h).opencl.num_copies as usize]
        .src = locked as *mut libc::c_void;
    (*h)
        .opencl
        .copies[(*h).opencl.num_copies as usize]
        .dest = (*fenc).i_row_satds[(b - p0) as usize][(p1 - b) as usize]
        as *mut libc::c_void;
    (*h).opencl.copies[(*h).opencl.num_copies as usize].bytes = size;
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    status = ((*ocl).clEnqueueReadBuffer)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.queue,
        (*h).opencl.row_satds[(*h).opencl.last_buf as usize],
        0 as libc::c_int as cl_bool,
        0 as libc::c_int as size_t,
        size as size_t,
        locked as *mut libc::c_void,
        0 as libc::c_int as cl_uint,
        std::ptr::null::<cl_event>(),
        std::ptr::null_mut::<cl_event>(),
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clEnqueueReadBuffer error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    (*h).opencl.num_copies += 1;
    (*h).opencl.num_copies;
    size = (4 as libc::c_int as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
        as libc::c_int;
    locked = opencl_alloc_locked(h, size) as *mut libc::c_char;
    if (*h).opencl.b_fatal_error != 0 {
        return -(1 as libc::c_int);
    }
    status = ((*ocl).clEnqueueReadBuffer)
        .expect(
            "non-null function pointer",
        )(
        (*h).opencl.queue,
        (*h).opencl.frame_stats[(*h).opencl.last_buf as usize],
        0 as libc::c_int as cl_bool,
        0 as libc::c_int as size_t,
        size as size_t,
        locked as *mut libc::c_void,
        0 as libc::c_int as cl_uint,
        std::ptr::null::<cl_event>(),
        std::ptr::null_mut::<cl_event>(),
    );
    if status != 0 as libc::c_int {
        (*h).param.b_opencl = 0 as libc::c_int;
        (*h).opencl.b_fatal_error = 1 as libc::c_int;
        x264_8_log(
            h,
            0 as libc::c_int,
            b"clEnqueueReadBuffer error '%d'\n\0" as *const u8 as *const libc::c_char,
            status,
        );
        return -(1 as libc::c_int);
    }
    (*h).opencl.last_buf = ((*h).opencl.last_buf == 0) as libc::c_int;
    (*h)
        .opencl
        .copies[(*h).opencl.num_copies as usize]
        .src = locked as *mut libc::c_void;
    (*h)
        .opencl
        .copies[(*h).opencl.num_copies as usize]
        .dest = &mut *(*((*fenc).i_cost_est).as_mut_ptr().offset((b - p0) as isize))
        .as_mut_ptr()
        .offset((p1 - b) as isize) as *mut libc::c_int as *mut libc::c_void;
    (*h)
        .opencl
        .copies[(*h).opencl.num_copies as usize]
        .bytes = ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int;
    (*h).opencl.num_copies += 1;
    (*h).opencl.num_copies;
    (*h)
        .opencl
        .copies[(*h).opencl.num_copies as usize]
        .src = locked
        .offset(::core::mem::size_of::<libc::c_int>() as libc::c_ulong as isize)
        as *mut libc::c_void;
    (*h)
        .opencl
        .copies[(*h).opencl.num_copies as usize]
        .dest = &mut *(*((*fenc).i_cost_est_aq).as_mut_ptr().offset((b - p0) as isize))
        .as_mut_ptr()
        .offset((p1 - b) as isize) as *mut libc::c_int as *mut libc::c_void;
    (*h)
        .opencl
        .copies[(*h).opencl.num_copies as usize]
        .bytes = ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int;
    (*h).opencl.num_copies += 1;
    (*h).opencl.num_copies;
    if b == p1 {
        (*h)
            .opencl
            .copies[(*h).opencl.num_copies as usize]
            .src = locked
            .offset(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                    as isize,
            ) as *mut libc::c_void;
        (*h)
            .opencl
            .copies[(*h).opencl.num_copies as usize]
            .dest = &mut *((*fenc).i_intra_mbs).as_mut_ptr().offset((b - p0) as isize)
            as *mut libc::c_int as *mut libc::c_void;
        (*h)
            .opencl
            .copies[(*h).opencl.num_copies as usize]
            .bytes = ::core::mem::size_of::<libc::c_int>() as libc::c_ulong
            as libc::c_int;
        (*h).opencl.num_copies += 1;
        (*h).opencl.num_copies;
    }
    0 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_opencl_slicetype_prep(
    mut h: *mut x264_t,
    mut frames: *mut *mut x264_frame_t,
    mut num_frames: libc::c_int,
    mut lambda: libc::c_int,
) {
    if (*h).param.b_opencl != 0 {
        let mut i: libc::c_int = 0 as libc::c_int;
        while i <= num_frames {
            x264_8_opencl_lowres_init(h, *frames.offset(i as isize), lambda);
            i += 1;
            i;
        }
        x264_8_opencl_flush(h);
        if (*h).param.i_bframe_adaptive == 2 as libc::c_int && (*h).param.i_bframe != 0 {
            let mut b: libc::c_int = 0 as libc::c_int;
            while b <= num_frames {
                let mut j: libc::c_int = 1 as libc::c_int;
                while j < (*h).param.i_bframe {
                    let mut p0: libc::c_int = b - j;
                    if p0 >= 0 as libc::c_int
                        && (*((**frames.offset(b as isize))
                            .lowres_mvs[0 as libc::c_int
                            as usize][(b - p0 - 1 as libc::c_int) as usize])
                            .offset(
                                0 as libc::c_int as isize,
                            ))[0 as libc::c_int as usize] as libc::c_int
                            == 0x7fff as libc::c_int
                    {
                        let mut w: *const x264_weight_t = x264_zero.as_mut_ptr()
                            as *const x264_weight_t;
                        if (*h).param.analyse.i_weighted_pred != 0 {
                            x264_8_weights_analyse(
                                h,
                                *frames.offset(b as isize),
                                *frames.offset(p0 as isize),
                                1 as libc::c_int,
                            );
                            w = ((**frames.offset(b as isize))
                                .weight[0 as libc::c_int as usize])
                                .as_mut_ptr();
                        }
                        (*((**frames.offset(b as isize))
                            .lowres_mvs[0 as libc::c_int
                            as usize][(b - p0 - 1 as libc::c_int) as usize])
                            .offset(
                                0 as libc::c_int as isize,
                            ))[0 as libc::c_int as usize] = 0 as libc::c_int as int16_t;
                        x264_8_opencl_motionsearch(
                            h,
                            frames,
                            b,
                            p0,
                            0 as libc::c_int,
                            lambda,
                            w,
                        );
                    }
                    let mut p1: libc::c_int = b + j;
                    if p1 <= num_frames
                        && (*((**frames.offset(b as isize))
                            .lowres_mvs[1 as libc::c_int
                            as usize][(p1 - b - 1 as libc::c_int) as usize])
                            .offset(
                                0 as libc::c_int as isize,
                            ))[0 as libc::c_int as usize] as libc::c_int
                            == 0x7fff as libc::c_int
                    {
                        (*((**frames.offset(b as isize))
                            .lowres_mvs[1 as libc::c_int
                            as usize][(p1 - b - 1 as libc::c_int) as usize])
                            .offset(
                                0 as libc::c_int as isize,
                            ))[0 as libc::c_int as usize] = 0 as libc::c_int as int16_t;
                        x264_8_opencl_motionsearch(
                            h,
                            frames,
                            b,
                            p1,
                            1 as libc::c_int,
                            lambda,
                            std::ptr::null::<x264_weight_t>(),
                        );
                    }
                    j += 1;
                    j;
                }
                b += 1;
                b;
            }
            x264_8_opencl_flush(h);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_opencl_slicetype_end(mut h: *mut x264_t) {}
#[no_mangle]
pub unsafe extern "C" fn x264_8_opencl_precalculate_frame_cost(
    mut h: *mut x264_t,
    mut frames: *mut *mut x264_frame_t,
    mut lambda: libc::c_int,
    mut p0: libc::c_int,
    mut p1: libc::c_int,
    mut b: libc::c_int,
) -> libc::c_int {
    if (**frames.offset(b as isize)).i_cost_est[(b - p0) as usize][(p1 - b) as usize]
        >= 0 as libc::c_int || b == p0 && b == p1
    {
        0 as libc::c_int
    } else {
        let mut do_search: [libc::c_int; 2] = [0; 2];
        let mut dist_scale_factor: libc::c_int = 128 as libc::c_int;
        let mut w: *const x264_weight_t = x264_zero.as_mut_ptr() as *const x264_weight_t;
        (**frames.offset(b as isize))
            .i_cost_est[(b - p0) as usize][(p1 - b) as usize] = 0 as libc::c_int;
        do_search[0 as libc::c_int
            as usize] = (b != p0
            && (*((**frames.offset(b as isize))
                .lowres_mvs[0 as libc::c_int
                as usize][(b - p0 - 1 as libc::c_int) as usize])
                .offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
                as libc::c_int == 0x7fff as libc::c_int) as libc::c_int;
        do_search[1 as libc::c_int
            as usize] = (b != p1
            && (*((**frames.offset(b as isize))
                .lowres_mvs[1 as libc::c_int
                as usize][(p1 - b - 1 as libc::c_int) as usize])
                .offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
                as libc::c_int == 0x7fff as libc::c_int) as libc::c_int;
        if do_search[0 as libc::c_int as usize] != 0 {
            if (*h).param.analyse.i_weighted_pred != 0 && b == p1 {
                x264_8_weights_analyse(
                    h,
                    *frames.offset(b as isize),
                    *frames.offset(p0 as isize),
                    1 as libc::c_int,
                );
                w = ((**frames.offset(b as isize)).weight[0 as libc::c_int as usize])
                    .as_mut_ptr();
            }
            (*((**frames.offset(b as isize))
                .lowres_mvs[0 as libc::c_int
                as usize][(b - p0 - 1 as libc::c_int) as usize])
                .offset(
                    0 as libc::c_int as isize,
                ))[0 as libc::c_int as usize] = 0 as libc::c_int as int16_t;
        }
        if do_search[1 as libc::c_int as usize] != 0 {
            (*((**frames.offset(b as isize))
                .lowres_mvs[1 as libc::c_int
                as usize][(p1 - b - 1 as libc::c_int) as usize])
                .offset(
                    0 as libc::c_int as isize,
                ))[0 as libc::c_int as usize] = 0 as libc::c_int as int16_t;
        }
        if b == p1 {
            (**frames.offset(b as isize))
                .i_intra_mbs[(b - p0) as usize] = 0 as libc::c_int;
        }
        if p1 != p0 {
            dist_scale_factor = (((b - p0) << 8 as libc::c_int)
                + ((p1 - p0) >> 1 as libc::c_int)) / (p1 - p0);
        }
        (**frames.offset(b as isize))
            .i_cost_est[(b - p0) as usize][(p1 - b) as usize] = 0 as libc::c_int;
        (**frames.offset(b as isize))
            .i_cost_est_aq[(b - p0) as usize][(p1 - b) as usize] = 0 as libc::c_int;
        x264_8_opencl_lowres_init(h, *frames.offset(b as isize), lambda);
        if do_search[0 as libc::c_int as usize] != 0 {
            x264_8_opencl_lowres_init(h, *frames.offset(p0 as isize), lambda);
            x264_8_opencl_motionsearch(h, frames, b, p0, 0 as libc::c_int, lambda, w);
        }
        if do_search[1 as libc::c_int as usize] != 0 {
            x264_8_opencl_lowres_init(h, *frames.offset(p1 as isize), lambda);
            x264_8_opencl_motionsearch(
                h,
                frames,
                b,
                p1,
                1 as libc::c_int,
                lambda,
                std::ptr::null::<x264_weight_t>(),
            );
        }
        x264_8_opencl_finalize_cost(h, lambda, frames, p0, p1, b, dist_scale_factor);
        1 as libc::c_int
    }
}
