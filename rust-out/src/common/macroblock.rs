#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types, label_break_value, stdsimd)]
#[cfg(target_arch = "x86")]
pub use core::arch::x86::{__m128, _mm_setr_ps};
#[cfg(target_arch = "x86_64")]
pub use core::arch::x86_64::{__m128, _mm_setr_ps};
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
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    static mut x264_zero: [uint8_t; 1024];
    fn x264_malloc(_: int64_t) -> *mut libc::c_void;
    fn x264_free(_: *mut libc::c_void);
    fn x264_8_mb_predict_mv_pskip(h: *mut x264_t, mv: *mut int16_t);
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
pub union x264_union16_t {
    pub i: uint16_t,
    pub b: [uint8_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union x264_union32_t {
    pub i: uint32_t,
    pub w: [uint16_t; 2],
    pub b: [uint8_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union x264_union64_t {
    pub i: uint64_t,
    pub d: [uint32_t; 2],
    pub w: [uint16_t; 4],
    pub b: [uint8_t; 8],
}
pub type chroma_format_e = libc::c_uint;
pub const CHROMA_444: chroma_format_e = 3;
pub const CHROMA_422: chroma_format_e = 2;
pub const CHROMA_420: chroma_format_e = 1;
pub const CHROMA_400: chroma_format_e = 0;
pub type slice_type_e = libc::c_uint;
pub const SLICE_TYPE_I: slice_type_e = 2;
pub const SLICE_TYPE_B: slice_type_e = 1;
pub const SLICE_TYPE_P: slice_type_e = 0;
pub type intra_chroma_pred_e = libc::c_uint;
pub const I_PRED_CHROMA_DC_128: intra_chroma_pred_e = 6;
pub const I_PRED_CHROMA_DC_TOP: intra_chroma_pred_e = 5;
pub const I_PRED_CHROMA_DC_LEFT: intra_chroma_pred_e = 4;
pub const I_PRED_CHROMA_P: intra_chroma_pred_e = 3;
pub const I_PRED_CHROMA_V: intra_chroma_pred_e = 2;
pub const I_PRED_CHROMA_H: intra_chroma_pred_e = 1;
pub const I_PRED_CHROMA_DC: intra_chroma_pred_e = 0;
pub type intra4x4_pred_e = libc::c_uint;
pub const I_PRED_4x4_DC_128: intra4x4_pred_e = 11;
pub const I_PRED_4x4_DC_TOP: intra4x4_pred_e = 10;
pub const I_PRED_4x4_DC_LEFT: intra4x4_pred_e = 9;
pub const I_PRED_4x4_HU: intra4x4_pred_e = 8;
pub const I_PRED_4x4_VL: intra4x4_pred_e = 7;
pub const I_PRED_4x4_HD: intra4x4_pred_e = 6;
pub const I_PRED_4x4_VR: intra4x4_pred_e = 5;
pub const I_PRED_4x4_DDR: intra4x4_pred_e = 4;
pub const I_PRED_4x4_DDL: intra4x4_pred_e = 3;
pub const I_PRED_4x4_DC: intra4x4_pred_e = 2;
pub const I_PRED_4x4_H: intra4x4_pred_e = 1;
pub const I_PRED_4x4_V: intra4x4_pred_e = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mvsad_t {
    pub sad: libc::c_int,
    pub mv: [int16_t; 2],
}
pub type macroblock_position_e = libc::c_uint;
pub const ALL_NEIGHBORS: macroblock_position_e = 15;
pub const MB_PRIVATE: macroblock_position_e = 16;
pub const MB_TOPLEFT: macroblock_position_e = 8;
pub const MB_TOPRIGHT: macroblock_position_e = 4;
pub const MB_TOP: macroblock_position_e = 2;
pub const MB_LEFT: macroblock_position_e = 1;
pub type mb_class_e = libc::c_uint;
pub const X264_MBTYPE_MAX: mb_class_e = 19;
pub const B_SKIP: mb_class_e = 18;
pub const B_8x8: mb_class_e = 17;
pub const B_BI_BI: mb_class_e = 16;
pub const B_BI_L1: mb_class_e = 15;
pub const B_BI_L0: mb_class_e = 14;
pub const B_L1_BI: mb_class_e = 13;
pub const B_L1_L1: mb_class_e = 12;
pub const B_L1_L0: mb_class_e = 11;
pub const B_L0_BI: mb_class_e = 10;
pub const B_L0_L1: mb_class_e = 9;
pub const B_L0_L0: mb_class_e = 8;
pub const B_DIRECT: mb_class_e = 7;
pub const P_SKIP: mb_class_e = 6;
pub const P_8x8: mb_class_e = 5;
pub const P_L0: mb_class_e = 4;
pub const I_PCM: mb_class_e = 3;
pub const I_16x16: mb_class_e = 2;
pub const I_8x8: mb_class_e = 1;
pub const I_4x4: mb_class_e = 0;
pub type mb_partition_e = libc::c_uint;
pub const X264_PARTTYPE_MAX: mb_partition_e = 17;
pub const D_16x16: mb_partition_e = 16;
pub const D_8x16: mb_partition_e = 15;
pub const D_16x8: mb_partition_e = 14;
pub const D_8x8: mb_partition_e = 13;
pub const D_DIRECT_8x8: mb_partition_e = 12;
pub const D_BI_8x8: mb_partition_e = 11;
pub const D_BI_4x8: mb_partition_e = 10;
pub const D_BI_8x4: mb_partition_e = 9;
pub const D_BI_4x4: mb_partition_e = 8;
pub const D_L1_8x8: mb_partition_e = 7;
pub const D_L1_4x8: mb_partition_e = 6;
pub const D_L1_8x4: mb_partition_e = 5;
pub const D_L1_4x4: mb_partition_e = 4;
pub const D_L0_8x8: mb_partition_e = 3;
pub const D_L0_4x8: mb_partition_e = 2;
pub const D_L0_8x4: mb_partition_e = 1;
pub const D_L0_4x4: mb_partition_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union x264_union128_sse_t {
    pub i: __m128,
    pub q: [uint64_t; 2],
    pub d: [uint32_t; 4],
    pub w: [uint16_t; 8],
    pub b: [uint8_t; 16],
}
static mut x264_scan8: [uint8_t; 51] = [
    (4 as libc::c_int + 1 as libc::c_int * 8 as libc::c_int) as uint8_t,
    (5 as libc::c_int + 1 as libc::c_int * 8 as libc::c_int) as uint8_t,
    (4 as libc::c_int + 2 as libc::c_int * 8 as libc::c_int) as uint8_t,
    (5 as libc::c_int + 2 as libc::c_int * 8 as libc::c_int) as uint8_t,
    (6 as libc::c_int + 1 as libc::c_int * 8 as libc::c_int) as uint8_t,
    (7 as libc::c_int + 1 as libc::c_int * 8 as libc::c_int) as uint8_t,
    (6 as libc::c_int + 2 as libc::c_int * 8 as libc::c_int) as uint8_t,
    (7 as libc::c_int + 2 as libc::c_int * 8 as libc::c_int) as uint8_t,
    (4 as libc::c_int + 3 as libc::c_int * 8 as libc::c_int) as uint8_t,
    (5 as libc::c_int + 3 as libc::c_int * 8 as libc::c_int) as uint8_t,
    (4 as libc::c_int + 4 as libc::c_int * 8 as libc::c_int) as uint8_t,
    (5 as libc::c_int + 4 as libc::c_int * 8 as libc::c_int) as uint8_t,
    (6 as libc::c_int + 3 as libc::c_int * 8 as libc::c_int) as uint8_t,
    (7 as libc::c_int + 3 as libc::c_int * 8 as libc::c_int) as uint8_t,
    (6 as libc::c_int + 4 as libc::c_int * 8 as libc::c_int) as uint8_t,
    (7 as libc::c_int + 4 as libc::c_int * 8 as libc::c_int) as uint8_t,
    (4 as libc::c_int + 6 as libc::c_int * 8 as libc::c_int) as uint8_t,
    (5 as libc::c_int + 6 as libc::c_int * 8 as libc::c_int) as uint8_t,
    (4 as libc::c_int + 7 as libc::c_int * 8 as libc::c_int) as uint8_t,
    (5 as libc::c_int + 7 as libc::c_int * 8 as libc::c_int) as uint8_t,
    (6 as libc::c_int + 6 as libc::c_int * 8 as libc::c_int) as uint8_t,
    (7 as libc::c_int + 6 as libc::c_int * 8 as libc::c_int) as uint8_t,
    (6 as libc::c_int + 7 as libc::c_int * 8 as libc::c_int) as uint8_t,
    (7 as libc::c_int + 7 as libc::c_int * 8 as libc::c_int) as uint8_t,
    (4 as libc::c_int + 8 as libc::c_int * 8 as libc::c_int) as uint8_t,
    (5 as libc::c_int + 8 as libc::c_int * 8 as libc::c_int) as uint8_t,
    (4 as libc::c_int + 9 as libc::c_int * 8 as libc::c_int) as uint8_t,
    (5 as libc::c_int + 9 as libc::c_int * 8 as libc::c_int) as uint8_t,
    (6 as libc::c_int + 8 as libc::c_int * 8 as libc::c_int) as uint8_t,
    (7 as libc::c_int + 8 as libc::c_int * 8 as libc::c_int) as uint8_t,
    (6 as libc::c_int + 9 as libc::c_int * 8 as libc::c_int) as uint8_t,
    (7 as libc::c_int + 9 as libc::c_int * 8 as libc::c_int) as uint8_t,
    (4 as libc::c_int + 11 as libc::c_int * 8 as libc::c_int) as uint8_t,
    (5 as libc::c_int + 11 as libc::c_int * 8 as libc::c_int) as uint8_t,
    (4 as libc::c_int + 12 as libc::c_int * 8 as libc::c_int) as uint8_t,
    (5 as libc::c_int + 12 as libc::c_int * 8 as libc::c_int) as uint8_t,
    (6 as libc::c_int + 11 as libc::c_int * 8 as libc::c_int) as uint8_t,
    (7 as libc::c_int + 11 as libc::c_int * 8 as libc::c_int) as uint8_t,
    (6 as libc::c_int + 12 as libc::c_int * 8 as libc::c_int) as uint8_t,
    (7 as libc::c_int + 12 as libc::c_int * 8 as libc::c_int) as uint8_t,
    (4 as libc::c_int + 13 as libc::c_int * 8 as libc::c_int) as uint8_t,
    (5 as libc::c_int + 13 as libc::c_int * 8 as libc::c_int) as uint8_t,
    (4 as libc::c_int + 14 as libc::c_int * 8 as libc::c_int) as uint8_t,
    (5 as libc::c_int + 14 as libc::c_int * 8 as libc::c_int) as uint8_t,
    (6 as libc::c_int + 13 as libc::c_int * 8 as libc::c_int) as uint8_t,
    (7 as libc::c_int + 13 as libc::c_int * 8 as libc::c_int) as uint8_t,
    (6 as libc::c_int + 14 as libc::c_int * 8 as libc::c_int) as uint8_t,
    (7 as libc::c_int + 14 as libc::c_int * 8 as libc::c_int) as uint8_t,
    (0 as libc::c_int + 0 as libc::c_int * 8 as libc::c_int) as uint8_t,
    (0 as libc::c_int + 5 as libc::c_int * 8 as libc::c_int) as uint8_t,
    (0 as libc::c_int + 10 as libc::c_int * 8 as libc::c_int) as uint8_t,
];
#[inline(always)]
unsafe extern "C" fn x264_clip3(
    mut v: libc::c_int,
    mut i_min: libc::c_int,
    mut i_max: libc::c_int,
) -> libc::c_int {
    if v < i_min { i_min } else if v > i_max { i_max } else { v }
}
static mut x264_mb_chroma_pred_mode_fix: [uint8_t; 7] = [
    I_PRED_CHROMA_DC as libc::c_int as uint8_t,
    I_PRED_CHROMA_H as libc::c_int as uint8_t,
    I_PRED_CHROMA_V as libc::c_int as uint8_t,
    I_PRED_CHROMA_P as libc::c_int as uint8_t,
    I_PRED_CHROMA_DC as libc::c_int as uint8_t,
    I_PRED_CHROMA_DC as libc::c_int as uint8_t,
    I_PRED_CHROMA_DC as libc::c_int as uint8_t,
];
static mut x264_mb_type_fix: [uint8_t; 19] = [
    I_4x4 as libc::c_int as uint8_t,
    I_4x4 as libc::c_int as uint8_t,
    I_16x16 as libc::c_int as uint8_t,
    I_PCM as libc::c_int as uint8_t,
    P_L0 as libc::c_int as uint8_t,
    P_8x8 as libc::c_int as uint8_t,
    P_SKIP as libc::c_int as uint8_t,
    B_DIRECT as libc::c_int as uint8_t,
    B_L0_L0 as libc::c_int as uint8_t,
    B_L0_L1 as libc::c_int as uint8_t,
    B_L0_BI as libc::c_int as uint8_t,
    B_L1_L0 as libc::c_int as uint8_t,
    B_L1_L1 as libc::c_int as uint8_t,
    B_L1_BI as libc::c_int as uint8_t,
    B_BI_L0 as libc::c_int as uint8_t,
    B_BI_L1 as libc::c_int as uint8_t,
    B_BI_BI as libc::c_int as uint8_t,
    B_8x8 as libc::c_int as uint8_t,
    B_SKIP as libc::c_int as uint8_t,
];
#[inline(always)]
unsafe extern "C" fn x264_macroblock_cache_skip(
    mut h: *mut x264_t,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut b_skip: libc::c_int,
) {
    x264_macroblock_cache_rect(
        &mut *((*h).mb.cache.skip)
            .as_mut_ptr()
            .offset(
                (4 as libc::c_int + 1 as libc::c_int * 8 as libc::c_int + x
                    + 8 as libc::c_int * y) as isize,
            ) as *mut int8_t as *mut libc::c_void,
        width,
        height,
        1 as libc::c_int,
        b_skip as uint32_t,
    );
}
#[inline(always)]
unsafe extern "C" fn x264_macroblock_cache_rect(
    mut dst: *mut libc::c_void,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut s: libc::c_int,
    mut v: uint32_t,
) {
    let mut d: *mut uint8_t = dst as *mut uint8_t;
    let mut v2: uint16_t = (if s >= 2 as libc::c_int {
        v
    } else {
        v * 0x101 as libc::c_int as uint32_t
    }) as uint16_t;
    let mut v4: uint32_t = if s >= 4 as libc::c_int {
        v
    } else if s >= 2 as libc::c_int {
        v * 0x10001 as libc::c_int as uint32_t
    } else {
        v * 0x1010101 as libc::c_int as uint32_t
    };
    let mut v8: uint64_t = (v4 as uint64_t)
        .wrapping_add((v4 as uint64_t) << 32 as libc::c_int);
    s *= 8 as libc::c_int;
    if w == 2 as libc::c_int {
        (*(d.offset((s * 0 as libc::c_int) as isize) as *mut x264_union16_t)).i = v2;
        if h == 1 as libc::c_int {
            return;
        }
        (*(d.offset((s * 1 as libc::c_int) as isize) as *mut x264_union16_t)).i = v2;
        if h == 2 as libc::c_int {
            return;
        }
        (*(d.offset((s * 2 as libc::c_int) as isize) as *mut x264_union16_t)).i = v2;
        (*(d.offset((s * 3 as libc::c_int) as isize) as *mut x264_union16_t)).i = v2;
    } else if w == 4 as libc::c_int {
        (*(d.offset((s * 0 as libc::c_int) as isize) as *mut x264_union32_t)).i = v4;
        if h == 1 as libc::c_int {
            return;
        }
        (*(d.offset((s * 1 as libc::c_int) as isize) as *mut x264_union32_t)).i = v4;
        if h == 2 as libc::c_int {
            return;
        }
        (*(d.offset((s * 2 as libc::c_int) as isize) as *mut x264_union32_t)).i = v4;
        (*(d.offset((s * 3 as libc::c_int) as isize) as *mut x264_union32_t)).i = v4;
    } else if w == 8 as libc::c_int {
        if ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
            == 8 as libc::c_int as libc::c_ulong
        {
            (*(d.offset((s * 0 as libc::c_int) as isize) as *mut x264_union64_t)).i = v8;
            if h == 1 as libc::c_int {
                return;
            }
            (*(d.offset((s * 1 as libc::c_int) as isize) as *mut x264_union64_t)).i = v8;
            if h == 2 as libc::c_int {
                return;
            }
            (*(d.offset((s * 2 as libc::c_int) as isize) as *mut x264_union64_t)).i = v8;
            (*(d.offset((s * 3 as libc::c_int) as isize) as *mut x264_union64_t)).i = v8;
        } else {
            (*(d
                .offset((s * 0 as libc::c_int) as isize)
                .offset(0 as libc::c_int as isize) as *mut x264_union32_t))
                .i = v4;
            (*(d
                .offset((s * 0 as libc::c_int) as isize)
                .offset(4 as libc::c_int as isize) as *mut x264_union32_t))
                .i = v4;
            if h == 1 as libc::c_int {
                return;
            }
            (*(d
                .offset((s * 1 as libc::c_int) as isize)
                .offset(0 as libc::c_int as isize) as *mut x264_union32_t))
                .i = v4;
            (*(d
                .offset((s * 1 as libc::c_int) as isize)
                .offset(4 as libc::c_int as isize) as *mut x264_union32_t))
                .i = v4;
            if h == 2 as libc::c_int {
                return;
            }
            (*(d
                .offset((s * 2 as libc::c_int) as isize)
                .offset(0 as libc::c_int as isize) as *mut x264_union32_t))
                .i = v4;
            (*(d
                .offset((s * 2 as libc::c_int) as isize)
                .offset(4 as libc::c_int as isize) as *mut x264_union32_t))
                .i = v4;
            (*(d
                .offset((s * 3 as libc::c_int) as isize)
                .offset(0 as libc::c_int as isize) as *mut x264_union32_t))
                .i = v4;
            (*(d
                .offset((s * 3 as libc::c_int) as isize)
                .offset(4 as libc::c_int as isize) as *mut x264_union32_t))
                .i = v4;
        }
    } else if w == 16 as libc::c_int {
        if h != 1 as libc::c_int {} else {
            __assert_fail(
                b"h != 1\0" as *const u8 as *const libc::c_char,
                b"common/rectangle.h\0" as *const u8 as *const libc::c_char,
                82 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 65],
                    &[libc::c_char; 65],
                >(b"void x264_macroblock_cache_rect(void *, int, int, int, uint32_t)\0"))
                    .as_ptr(),
            );
        }
        'c_30189: {
            if h != 1 as libc::c_int {} else {
                __assert_fail(
                    b"h != 1\0" as *const u8 as *const libc::c_char,
                    b"common/rectangle.h\0" as *const u8 as *const libc::c_char,
                    82 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 65],
                        &[libc::c_char; 65],
                    >(
                        b"void x264_macroblock_cache_rect(void *, int, int, int, uint32_t)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        if ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
            == 8 as libc::c_int as libc::c_ulong
        {
            loop {
                (*(d
                    .offset((s * 0 as libc::c_int) as isize)
                    .offset(0 as libc::c_int as isize) as *mut x264_union64_t))
                    .i = v8;
                (*(d
                    .offset((s * 0 as libc::c_int) as isize)
                    .offset(8 as libc::c_int as isize) as *mut x264_union64_t))
                    .i = v8;
                (*(d
                    .offset((s * 1 as libc::c_int) as isize)
                    .offset(0 as libc::c_int as isize) as *mut x264_union64_t))
                    .i = v8;
                (*(d
                    .offset((s * 1 as libc::c_int) as isize)
                    .offset(8 as libc::c_int as isize) as *mut x264_union64_t))
                    .i = v8;
                h -= 2 as libc::c_int;
                d = d.offset((s * 2 as libc::c_int) as isize);
                if h == 0 {
                    break;
                }
            }
        } else {
            loop {
                (*(d.offset(0 as libc::c_int as isize) as *mut x264_union32_t)).i = v4;
                (*(d.offset(4 as libc::c_int as isize) as *mut x264_union32_t)).i = v4;
                (*(d.offset(8 as libc::c_int as isize) as *mut x264_union32_t)).i = v4;
                (*(d.offset(12 as libc::c_int as isize) as *mut x264_union32_t)).i = v4;
                d = d.offset(s as isize);
                h -= 1;
                if h == 0 {
                    break;
                }
            }
        }
    } else {
        __assert_fail(
            b"0\0" as *const u8 as *const libc::c_char,
            b"common/rectangle.h\0" as *const u8 as *const libc::c_char,
            118 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 65],
                &[libc::c_char; 65],
            >(b"void x264_macroblock_cache_rect(void *, int, int, int, uint32_t)\0"))
                .as_ptr(),
        );
        'c_29956: {
            __assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"common/rectangle.h\0" as *const u8 as *const libc::c_char,
                118 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 65],
                    &[libc::c_char; 65],
                >(b"void x264_macroblock_cache_rect(void *, int, int, int, uint32_t)\0"))
                    .as_ptr(),
            );
        };
    };
}
#[inline(always)]
unsafe extern "C" fn pack16to32(mut a: uint32_t, mut b: uint32_t) -> uint32_t {
    a.wrapping_add(b << 16 as libc::c_int)
}
#[inline(always)]
unsafe extern "C" fn pack8to32(
    mut a: uint32_t,
    mut b: uint32_t,
    mut c: uint32_t,
    mut d: uint32_t,
) -> uint32_t {
    a
        .wrapping_add(b << 8 as libc::c_int)
        .wrapping_add(c << 16 as libc::c_int)
        .wrapping_add(d << 24 as libc::c_int)
}
static mut x264_size2pixel: [[uint8_t; 5]; 5] = [
    [0 as libc::c_int as uint8_t, 0, 0, 0, 0],
    [
        0 as libc::c_int as uint8_t,
        PIXEL_4x4 as libc::c_int as uint8_t,
        PIXEL_8x4 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
    ],
    [
        0 as libc::c_int as uint8_t,
        PIXEL_4x8 as libc::c_int as uint8_t,
        PIXEL_8x8 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        PIXEL_16x8 as libc::c_int as uint8_t,
    ],
    [0 as libc::c_int as uint8_t, 0, 0, 0, 0],
    [
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        PIXEL_8x16 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        PIXEL_16x16 as libc::c_int as uint8_t,
    ],
];
#[inline(never)]
unsafe extern "C" fn mb_mc_0xywh(
    mut h: *mut x264_t,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
) {
    let mut i8: libc::c_int = x264_scan8[0 as libc::c_int as usize] as libc::c_int + x
        + 8 as libc::c_int * y;
    let mut i_ref: libc::c_int = (*h)
        .mb
        .cache
        .ref_0[0 as libc::c_int as usize][i8 as usize] as libc::c_int;
    let mut mvx: libc::c_int = x264_clip3(
        (*h)
            .mb
            .cache
            .mv[0 as libc::c_int as usize][i8 as usize][0 as libc::c_int as usize]
            as libc::c_int,
        (*h).mb.mv_min[0 as libc::c_int as usize],
        (*h).mb.mv_max[0 as libc::c_int as usize],
    ) + 4 as libc::c_int * 4 as libc::c_int * x;
    let mut mvy: libc::c_int = x264_clip3(
        (*h)
            .mb
            .cache
            .mv[0 as libc::c_int as usize][i8 as usize][1 as libc::c_int as usize]
            as libc::c_int,
        (*h).mb.mv_min[1 as libc::c_int as usize],
        (*h).mb.mv_max[1 as libc::c_int as usize],
    ) + 4 as libc::c_int * 4 as libc::c_int * y;
    ((*h).mc.mc_luma)
        .expect(
            "non-null function pointer",
        )(
        &mut *(*((*h).mb.pic.p_fdec).as_mut_ptr().offset(0 as libc::c_int as isize))
            .offset(
                (4 as libc::c_int * y * 32 as libc::c_int + 4 as libc::c_int * x)
                    as isize,
            ),
        32 as libc::c_int as intptr_t,
        &mut *(*(*((*h).mb.pic.p_fref).as_mut_ptr().offset(0 as libc::c_int as isize))
            .as_mut_ptr()
            .offset(i_ref as isize))
            .as_mut_ptr()
            .offset((0 as libc::c_int * 4 as libc::c_int) as isize),
        (*h).mb.pic.i_stride[0 as libc::c_int as usize] as intptr_t,
        mvx,
        mvy,
        4 as libc::c_int * width,
        4 as libc::c_int * height,
        if 0 as libc::c_int != 0 {
            x264_zero.as_mut_ptr() as *const x264_weight_t
        } else {
            &mut *(*((*h).sh.weight).as_mut_ptr().offset(i_ref as isize))
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as *mut x264_weight_t
                as *const x264_weight_t
        },
    );
    if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as libc::c_int {
        ((*h).mc.mc_luma)
            .expect(
                "non-null function pointer",
            )(
            &mut *(*((*h).mb.pic.p_fdec).as_mut_ptr().offset(1 as libc::c_int as isize))
                .offset(
                    (4 as libc::c_int * y * 32 as libc::c_int + 4 as libc::c_int * x)
                        as isize,
                ),
            32 as libc::c_int as intptr_t,
            &mut *(*(*((*h).mb.pic.p_fref)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize))
                .as_mut_ptr()
                .offset(i_ref as isize))
                .as_mut_ptr()
                .offset((1 as libc::c_int * 4 as libc::c_int) as isize),
            (*h).mb.pic.i_stride[1 as libc::c_int as usize] as intptr_t,
            mvx,
            mvy,
            4 as libc::c_int * width,
            4 as libc::c_int * height,
            if 0 as libc::c_int != 0 {
                x264_zero.as_mut_ptr() as *const x264_weight_t
            } else {
                &mut *(*((*h).sh.weight).as_mut_ptr().offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize) as *mut x264_weight_t
                    as *const x264_weight_t
            },
        );
        ((*h).mc.mc_luma)
            .expect(
                "non-null function pointer",
            )(
            &mut *(*((*h).mb.pic.p_fdec).as_mut_ptr().offset(2 as libc::c_int as isize))
                .offset(
                    (4 as libc::c_int * y * 32 as libc::c_int + 4 as libc::c_int * x)
                        as isize,
                ),
            32 as libc::c_int as intptr_t,
            &mut *(*(*((*h).mb.pic.p_fref)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize))
                .as_mut_ptr()
                .offset(i_ref as isize))
                .as_mut_ptr()
                .offset((2 as libc::c_int * 4 as libc::c_int) as isize),
            (*h).mb.pic.i_stride[2 as libc::c_int as usize] as intptr_t,
            mvx,
            mvy,
            4 as libc::c_int * width,
            4 as libc::c_int * height,
            if 0 as libc::c_int != 0 {
                x264_zero.as_mut_ptr() as *const x264_weight_t
            } else {
                &mut *(*((*h).sh.weight).as_mut_ptr().offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as isize) as *mut x264_weight_t
                    as *const x264_weight_t
            },
        );
    } else if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc != 0 {
        let mut v_shift: libc::c_int = (*h).mb.chroma_v_shift;
        if v_shift & (*h).mb.b_interlaced & i_ref != 0 {
            mvy
                += ((*h).mb.i_mb_y & 1 as libc::c_int) * 4 as libc::c_int
                    - 2 as libc::c_int;
        }
        let mut offset: libc::c_int = ((4 as libc::c_int * 32 as libc::c_int) >> v_shift)
            * y + 2 as libc::c_int * x;
        height = (4 as libc::c_int * height) >> v_shift;
        ((*h).mc.mc_chroma)
            .expect(
                "non-null function pointer",
            )(
            &mut *(*((*h).mb.pic.p_fdec).as_mut_ptr().offset(1 as libc::c_int as isize))
                .offset(offset as isize),
            &mut *(*((*h).mb.pic.p_fdec).as_mut_ptr().offset(2 as libc::c_int as isize))
                .offset(offset as isize),
            32 as libc::c_int as intptr_t,
            (*h)
                .mb
                .pic
                .p_fref[0 as libc::c_int
                as usize][i_ref as usize][4 as libc::c_int as usize],
            (*h).mb.pic.i_stride[1 as libc::c_int as usize] as intptr_t,
            mvx,
            (2 as libc::c_int * mvy) >> v_shift,
            2 as libc::c_int * width,
            height,
        );
        if !((*h).sh.weight[i_ref as usize][1 as libc::c_int as usize].weightfn)
            .is_null()
        {
            (*((*h).sh.weight[i_ref as usize][1 as libc::c_int as usize].weightfn)
                .offset((width >> 1 as libc::c_int) as isize))
                .expect(
                    "non-null function pointer",
                )(
                &mut *(*((*h).mb.pic.p_fdec)
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize))
                    .offset(offset as isize),
                32 as libc::c_int as intptr_t,
                &mut *(*((*h).mb.pic.p_fdec)
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize))
                    .offset(offset as isize),
                32 as libc::c_int as intptr_t,
                &mut *(*((*h).sh.weight).as_mut_ptr().offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize),
                height,
            );
        }
        if !((*h).sh.weight[i_ref as usize][2 as libc::c_int as usize].weightfn)
            .is_null()
        {
            (*((*h).sh.weight[i_ref as usize][2 as libc::c_int as usize].weightfn)
                .offset((width >> 1 as libc::c_int) as isize))
                .expect(
                    "non-null function pointer",
                )(
                &mut *(*((*h).mb.pic.p_fdec)
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as isize))
                    .offset(offset as isize),
                32 as libc::c_int as intptr_t,
                &mut *(*((*h).mb.pic.p_fdec)
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as isize))
                    .offset(offset as isize),
                32 as libc::c_int as intptr_t,
                &mut *(*((*h).sh.weight).as_mut_ptr().offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as isize),
                height,
            );
        }
    }
}
#[inline(never)]
unsafe extern "C" fn mb_mc_1xywh(
    mut h: *mut x264_t,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
) {
    let mut i8: libc::c_int = x264_scan8[0 as libc::c_int as usize] as libc::c_int + x
        + 8 as libc::c_int * y;
    let mut i_ref: libc::c_int = (*h)
        .mb
        .cache
        .ref_0[1 as libc::c_int as usize][i8 as usize] as libc::c_int;
    let mut mvx: libc::c_int = x264_clip3(
        (*h)
            .mb
            .cache
            .mv[1 as libc::c_int as usize][i8 as usize][0 as libc::c_int as usize]
            as libc::c_int,
        (*h).mb.mv_min[0 as libc::c_int as usize],
        (*h).mb.mv_max[0 as libc::c_int as usize],
    ) + 4 as libc::c_int * 4 as libc::c_int * x;
    let mut mvy: libc::c_int = x264_clip3(
        (*h)
            .mb
            .cache
            .mv[1 as libc::c_int as usize][i8 as usize][1 as libc::c_int as usize]
            as libc::c_int,
        (*h).mb.mv_min[1 as libc::c_int as usize],
        (*h).mb.mv_max[1 as libc::c_int as usize],
    ) + 4 as libc::c_int * 4 as libc::c_int * y;
    ((*h).mc.mc_luma)
        .expect(
            "non-null function pointer",
        )(
        &mut *(*((*h).mb.pic.p_fdec).as_mut_ptr().offset(0 as libc::c_int as isize))
            .offset(
                (4 as libc::c_int * y * 32 as libc::c_int + 4 as libc::c_int * x)
                    as isize,
            ),
        32 as libc::c_int as intptr_t,
        &mut *(*(*((*h).mb.pic.p_fref).as_mut_ptr().offset(1 as libc::c_int as isize))
            .as_mut_ptr()
            .offset(i_ref as isize))
            .as_mut_ptr()
            .offset((0 as libc::c_int * 4 as libc::c_int) as isize),
        (*h).mb.pic.i_stride[0 as libc::c_int as usize] as intptr_t,
        mvx,
        mvy,
        4 as libc::c_int * width,
        4 as libc::c_int * height,
        if 1 as libc::c_int != 0 {
            x264_zero.as_mut_ptr() as *const x264_weight_t
        } else {
            &mut *(*((*h).sh.weight).as_mut_ptr().offset(i_ref as isize))
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as *mut x264_weight_t
                as *const x264_weight_t
        },
    );
    if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as libc::c_int {
        ((*h).mc.mc_luma)
            .expect(
                "non-null function pointer",
            )(
            &mut *(*((*h).mb.pic.p_fdec).as_mut_ptr().offset(1 as libc::c_int as isize))
                .offset(
                    (4 as libc::c_int * y * 32 as libc::c_int + 4 as libc::c_int * x)
                        as isize,
                ),
            32 as libc::c_int as intptr_t,
            &mut *(*(*((*h).mb.pic.p_fref)
                .as_mut_ptr()
                .offset(1 as libc::c_int as isize))
                .as_mut_ptr()
                .offset(i_ref as isize))
                .as_mut_ptr()
                .offset((1 as libc::c_int * 4 as libc::c_int) as isize),
            (*h).mb.pic.i_stride[1 as libc::c_int as usize] as intptr_t,
            mvx,
            mvy,
            4 as libc::c_int * width,
            4 as libc::c_int * height,
            if 1 as libc::c_int != 0 {
                x264_zero.as_mut_ptr() as *const x264_weight_t
            } else {
                &mut *(*((*h).sh.weight).as_mut_ptr().offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize) as *mut x264_weight_t
                    as *const x264_weight_t
            },
        );
        ((*h).mc.mc_luma)
            .expect(
                "non-null function pointer",
            )(
            &mut *(*((*h).mb.pic.p_fdec).as_mut_ptr().offset(2 as libc::c_int as isize))
                .offset(
                    (4 as libc::c_int * y * 32 as libc::c_int + 4 as libc::c_int * x)
                        as isize,
                ),
            32 as libc::c_int as intptr_t,
            &mut *(*(*((*h).mb.pic.p_fref)
                .as_mut_ptr()
                .offset(1 as libc::c_int as isize))
                .as_mut_ptr()
                .offset(i_ref as isize))
                .as_mut_ptr()
                .offset((2 as libc::c_int * 4 as libc::c_int) as isize),
            (*h).mb.pic.i_stride[2 as libc::c_int as usize] as intptr_t,
            mvx,
            mvy,
            4 as libc::c_int * width,
            4 as libc::c_int * height,
            if 1 as libc::c_int != 0 {
                x264_zero.as_mut_ptr() as *const x264_weight_t
            } else {
                &mut *(*((*h).sh.weight).as_mut_ptr().offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as isize) as *mut x264_weight_t
                    as *const x264_weight_t
            },
        );
    } else if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc != 0 {
        let mut v_shift: libc::c_int = (*h).mb.chroma_v_shift;
        if v_shift & (*h).mb.b_interlaced & i_ref != 0 {
            mvy
                += ((*h).mb.i_mb_y & 1 as libc::c_int) * 4 as libc::c_int
                    - 2 as libc::c_int;
        }
        let mut offset: libc::c_int = ((4 as libc::c_int * 32 as libc::c_int) >> v_shift)
            * y + 2 as libc::c_int * x;
        ((*h).mc.mc_chroma)
            .expect(
                "non-null function pointer",
            )(
            &mut *(*((*h).mb.pic.p_fdec).as_mut_ptr().offset(1 as libc::c_int as isize))
                .offset(offset as isize),
            &mut *(*((*h).mb.pic.p_fdec).as_mut_ptr().offset(2 as libc::c_int as isize))
                .offset(offset as isize),
            32 as libc::c_int as intptr_t,
            (*h)
                .mb
                .pic
                .p_fref[1 as libc::c_int
                as usize][i_ref as usize][4 as libc::c_int as usize],
            (*h).mb.pic.i_stride[1 as libc::c_int as usize] as intptr_t,
            mvx,
            (2 as libc::c_int * mvy) >> v_shift,
            2 as libc::c_int * width,
            (4 as libc::c_int * height) >> v_shift,
        );
    }
}
#[inline(never)]
unsafe extern "C" fn mb_mc_01xywh(
    mut h: *mut x264_t,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
) {
    let mut i8: libc::c_int = x264_scan8[0 as libc::c_int as usize] as libc::c_int + x
        + 8 as libc::c_int * y;
    let mut i_ref0: libc::c_int = (*h)
        .mb
        .cache
        .ref_0[0 as libc::c_int as usize][i8 as usize] as libc::c_int;
    let mut i_ref1: libc::c_int = (*h)
        .mb
        .cache
        .ref_0[1 as libc::c_int as usize][i8 as usize] as libc::c_int;
    let mut weight: libc::c_int = (*((*h).mb.bipred_weight)
        .offset(i_ref0 as isize))[i_ref1 as usize] as libc::c_int;
    let mut mvx0: libc::c_int = x264_clip3(
        (*h)
            .mb
            .cache
            .mv[0 as libc::c_int as usize][i8 as usize][0 as libc::c_int as usize]
            as libc::c_int,
        (*h).mb.mv_min[0 as libc::c_int as usize],
        (*h).mb.mv_max[0 as libc::c_int as usize],
    ) + 4 as libc::c_int * 4 as libc::c_int * x;
    let mut mvx1: libc::c_int = x264_clip3(
        (*h)
            .mb
            .cache
            .mv[1 as libc::c_int as usize][i8 as usize][0 as libc::c_int as usize]
            as libc::c_int,
        (*h).mb.mv_min[0 as libc::c_int as usize],
        (*h).mb.mv_max[0 as libc::c_int as usize],
    ) + 4 as libc::c_int * 4 as libc::c_int * x;
    let mut mvy0: libc::c_int = x264_clip3(
        (*h)
            .mb
            .cache
            .mv[0 as libc::c_int as usize][i8 as usize][1 as libc::c_int as usize]
            as libc::c_int,
        (*h).mb.mv_min[1 as libc::c_int as usize],
        (*h).mb.mv_max[1 as libc::c_int as usize],
    ) + 4 as libc::c_int * 4 as libc::c_int * y;
    let mut mvy1: libc::c_int = x264_clip3(
        (*h)
            .mb
            .cache
            .mv[1 as libc::c_int as usize][i8 as usize][1 as libc::c_int as usize]
            as libc::c_int,
        (*h).mb.mv_min[1 as libc::c_int as usize],
        (*h).mb.mv_max[1 as libc::c_int as usize],
    ) + 4 as libc::c_int * 4 as libc::c_int * y;
    let mut i_mode: libc::c_int = x264_size2pixel[height as usize][width as usize]
        as libc::c_int;
    let mut i_stride0: intptr_t = 16 as libc::c_int as intptr_t;
    let mut i_stride1: intptr_t = 16 as libc::c_int as intptr_t;
    let mut tmp0: [pixel; 256] = [0; 256];
    let mut tmp1: [pixel; 256] = [0; 256];
    let mut src0: *mut pixel = std::ptr::null_mut::<pixel>();
    let mut src1: *mut pixel = std::ptr::null_mut::<pixel>();
    src0 = ((*h).mc.get_ref)
        .expect(
            "non-null function pointer",
        )(
        tmp0.as_mut_ptr(),
        &mut i_stride0,
        &mut *(*(*((*h).mb.pic.p_fref).as_mut_ptr().offset(0 as libc::c_int as isize))
            .as_mut_ptr()
            .offset(i_ref0 as isize))
            .as_mut_ptr()
            .offset((0 as libc::c_int * 4 as libc::c_int) as isize),
        (*h).mb.pic.i_stride[0 as libc::c_int as usize] as intptr_t,
        mvx0,
        mvy0,
        4 as libc::c_int * width,
        4 as libc::c_int * height,
        x264_zero.as_mut_ptr() as *const x264_weight_t,
    );
    src1 = ((*h).mc.get_ref)
        .expect(
            "non-null function pointer",
        )(
        tmp1.as_mut_ptr(),
        &mut i_stride1,
        &mut *(*(*((*h).mb.pic.p_fref).as_mut_ptr().offset(1 as libc::c_int as isize))
            .as_mut_ptr()
            .offset(i_ref1 as isize))
            .as_mut_ptr()
            .offset((0 as libc::c_int * 4 as libc::c_int) as isize),
        (*h).mb.pic.i_stride[0 as libc::c_int as usize] as intptr_t,
        mvx1,
        mvy1,
        4 as libc::c_int * width,
        4 as libc::c_int * height,
        x264_zero.as_mut_ptr() as *const x264_weight_t,
    );
    ((*h).mc.avg[i_mode as usize])
        .expect(
            "non-null function pointer",
        )(
        &mut *(*((*h).mb.pic.p_fdec).as_mut_ptr().offset(0 as libc::c_int as isize))
            .offset(
                (4 as libc::c_int * y * 32 as libc::c_int + 4 as libc::c_int * x)
                    as isize,
            ),
        32 as libc::c_int as intptr_t,
        src0,
        i_stride0,
        src1,
        i_stride1,
        weight,
    );
    if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as libc::c_int {
        src0 = ((*h).mc.get_ref)
            .expect(
                "non-null function pointer",
            )(
            tmp0.as_mut_ptr(),
            &mut i_stride0,
            &mut *(*(*((*h).mb.pic.p_fref)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize))
                .as_mut_ptr()
                .offset(i_ref0 as isize))
                .as_mut_ptr()
                .offset((1 as libc::c_int * 4 as libc::c_int) as isize),
            (*h).mb.pic.i_stride[1 as libc::c_int as usize] as intptr_t,
            mvx0,
            mvy0,
            4 as libc::c_int * width,
            4 as libc::c_int * height,
            x264_zero.as_mut_ptr() as *const x264_weight_t,
        );
        src1 = ((*h).mc.get_ref)
            .expect(
                "non-null function pointer",
            )(
            tmp1.as_mut_ptr(),
            &mut i_stride1,
            &mut *(*(*((*h).mb.pic.p_fref)
                .as_mut_ptr()
                .offset(1 as libc::c_int as isize))
                .as_mut_ptr()
                .offset(i_ref1 as isize))
                .as_mut_ptr()
                .offset((1 as libc::c_int * 4 as libc::c_int) as isize),
            (*h).mb.pic.i_stride[1 as libc::c_int as usize] as intptr_t,
            mvx1,
            mvy1,
            4 as libc::c_int * width,
            4 as libc::c_int * height,
            x264_zero.as_mut_ptr() as *const x264_weight_t,
        );
        ((*h).mc.avg[i_mode as usize])
            .expect(
                "non-null function pointer",
            )(
            &mut *(*((*h).mb.pic.p_fdec).as_mut_ptr().offset(1 as libc::c_int as isize))
                .offset(
                    (4 as libc::c_int * y * 32 as libc::c_int + 4 as libc::c_int * x)
                        as isize,
                ),
            32 as libc::c_int as intptr_t,
            src0,
            i_stride0,
            src1,
            i_stride1,
            weight,
        );
        src0 = ((*h).mc.get_ref)
            .expect(
                "non-null function pointer",
            )(
            tmp0.as_mut_ptr(),
            &mut i_stride0,
            &mut *(*(*((*h).mb.pic.p_fref)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize))
                .as_mut_ptr()
                .offset(i_ref0 as isize))
                .as_mut_ptr()
                .offset((2 as libc::c_int * 4 as libc::c_int) as isize),
            (*h).mb.pic.i_stride[2 as libc::c_int as usize] as intptr_t,
            mvx0,
            mvy0,
            4 as libc::c_int * width,
            4 as libc::c_int * height,
            x264_zero.as_mut_ptr() as *const x264_weight_t,
        );
        src1 = ((*h).mc.get_ref)
            .expect(
                "non-null function pointer",
            )(
            tmp1.as_mut_ptr(),
            &mut i_stride1,
            &mut *(*(*((*h).mb.pic.p_fref)
                .as_mut_ptr()
                .offset(1 as libc::c_int as isize))
                .as_mut_ptr()
                .offset(i_ref1 as isize))
                .as_mut_ptr()
                .offset((2 as libc::c_int * 4 as libc::c_int) as isize),
            (*h).mb.pic.i_stride[2 as libc::c_int as usize] as intptr_t,
            mvx1,
            mvy1,
            4 as libc::c_int * width,
            4 as libc::c_int * height,
            x264_zero.as_mut_ptr() as *const x264_weight_t,
        );
        ((*h).mc.avg[i_mode as usize])
            .expect(
                "non-null function pointer",
            )(
            &mut *(*((*h).mb.pic.p_fdec).as_mut_ptr().offset(2 as libc::c_int as isize))
                .offset(
                    (4 as libc::c_int * y * 32 as libc::c_int + 4 as libc::c_int * x)
                        as isize,
                ),
            32 as libc::c_int as intptr_t,
            src0,
            i_stride0,
            src1,
            i_stride1,
            weight,
        );
    } else if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc != 0 {
        let mut v_shift: libc::c_int = (*h).mb.chroma_v_shift;
        if v_shift & (*h).mb.b_interlaced & i_ref0 != 0 {
            mvy0
                += ((*h).mb.i_mb_y & 1 as libc::c_int) * 4 as libc::c_int
                    - 2 as libc::c_int;
        }
        if v_shift & (*h).mb.b_interlaced & i_ref1 != 0 {
            mvy1
                += ((*h).mb.i_mb_y & 1 as libc::c_int) * 4 as libc::c_int
                    - 2 as libc::c_int;
        }
        ((*h).mc.mc_chroma)
            .expect(
                "non-null function pointer",
            )(
            tmp0.as_mut_ptr(),
            tmp0.as_mut_ptr().offset(8 as libc::c_int as isize),
            16 as libc::c_int as intptr_t,
            (*h)
                .mb
                .pic
                .p_fref[0 as libc::c_int
                as usize][i_ref0 as usize][4 as libc::c_int as usize],
            (*h).mb.pic.i_stride[1 as libc::c_int as usize] as intptr_t,
            mvx0,
            (2 as libc::c_int * mvy0) >> v_shift,
            2 as libc::c_int * width,
            (4 as libc::c_int * height) >> v_shift,
        );
        ((*h).mc.mc_chroma)
            .expect(
                "non-null function pointer",
            )(
            tmp1.as_mut_ptr(),
            tmp1.as_mut_ptr().offset(8 as libc::c_int as isize),
            16 as libc::c_int as intptr_t,
            (*h)
                .mb
                .pic
                .p_fref[1 as libc::c_int
                as usize][i_ref1 as usize][4 as libc::c_int as usize],
            (*h).mb.pic.i_stride[1 as libc::c_int as usize] as intptr_t,
            mvx1,
            (2 as libc::c_int * mvy1) >> v_shift,
            2 as libc::c_int * width,
            (4 as libc::c_int * height) >> v_shift,
        );
        let mut chromapix: libc::c_int = (*h).luma2chroma_pixel[i_mode as usize]
            as libc::c_int;
        let mut offset: libc::c_int = ((4 as libc::c_int * 32 as libc::c_int) >> v_shift)
            * y + 2 as libc::c_int * x;
        ((*h).mc.avg[chromapix as usize])
            .expect(
                "non-null function pointer",
            )(
            &mut *(*((*h).mb.pic.p_fdec).as_mut_ptr().offset(1 as libc::c_int as isize))
                .offset(offset as isize),
            32 as libc::c_int as intptr_t,
            tmp0.as_mut_ptr(),
            16 as libc::c_int as intptr_t,
            tmp1.as_mut_ptr(),
            16 as libc::c_int as intptr_t,
            weight,
        );
        ((*h).mc.avg[chromapix as usize])
            .expect(
                "non-null function pointer",
            )(
            &mut *(*((*h).mb.pic.p_fdec).as_mut_ptr().offset(2 as libc::c_int as isize))
                .offset(offset as isize),
            32 as libc::c_int as intptr_t,
            tmp0.as_mut_ptr().offset(8 as libc::c_int as isize),
            16 as libc::c_int as intptr_t,
            tmp1.as_mut_ptr().offset(8 as libc::c_int as isize),
            16 as libc::c_int as intptr_t,
            weight,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_mb_mc_8x8(mut h: *mut x264_t, mut i8: libc::c_int) {
    let mut x: libc::c_int = 2 as libc::c_int * (i8 & 1 as libc::c_int);
    let mut y: libc::c_int = 2 as libc::c_int * (i8 >> 1 as libc::c_int);
    if (*h).sh.i_type == SLICE_TYPE_P as libc::c_int {
        match (*h).mb.i_sub_partition[i8 as usize] as libc::c_int {
            3 => {
                mb_mc_0xywh(h, x, y, 2 as libc::c_int, 2 as libc::c_int);
            }
            1 => {
                mb_mc_0xywh(
                    h,
                    x,
                    y + 0 as libc::c_int,
                    2 as libc::c_int,
                    1 as libc::c_int,
                );
                mb_mc_0xywh(
                    h,
                    x,
                    y + 1 as libc::c_int,
                    2 as libc::c_int,
                    1 as libc::c_int,
                );
            }
            2 => {
                mb_mc_0xywh(
                    h,
                    x + 0 as libc::c_int,
                    y,
                    1 as libc::c_int,
                    2 as libc::c_int,
                );
                mb_mc_0xywh(
                    h,
                    x + 1 as libc::c_int,
                    y,
                    1 as libc::c_int,
                    2 as libc::c_int,
                );
            }
            0 => {
                mb_mc_0xywh(
                    h,
                    x + 0 as libc::c_int,
                    y + 0 as libc::c_int,
                    1 as libc::c_int,
                    1 as libc::c_int,
                );
                mb_mc_0xywh(
                    h,
                    x + 1 as libc::c_int,
                    y + 0 as libc::c_int,
                    1 as libc::c_int,
                    1 as libc::c_int,
                );
                mb_mc_0xywh(
                    h,
                    x + 0 as libc::c_int,
                    y + 1 as libc::c_int,
                    1 as libc::c_int,
                    1 as libc::c_int,
                );
                mb_mc_0xywh(
                    h,
                    x + 1 as libc::c_int,
                    y + 1 as libc::c_int,
                    1 as libc::c_int,
                    1 as libc::c_int,
                );
            }
            _ => {}
        }
    } else {
        let mut scan8: libc::c_int = x264_scan8[0 as libc::c_int as usize] as libc::c_int
            + x + 8 as libc::c_int * y;
        if (*h).mb.cache.ref_0[0 as libc::c_int as usize][scan8 as usize] as libc::c_int
            >= 0 as libc::c_int
        {
            if (*h).mb.cache.ref_0[1 as libc::c_int as usize][scan8 as usize]
                as libc::c_int >= 0 as libc::c_int
            {
                mb_mc_01xywh(h, x, y, 2 as libc::c_int, 2 as libc::c_int);
            } else {
                mb_mc_0xywh(h, x, y, 2 as libc::c_int, 2 as libc::c_int);
            }
        } else {
            mb_mc_1xywh(h, x, y, 2 as libc::c_int, 2 as libc::c_int);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_mb_mc(mut h: *mut x264_t) {
    if (*h).mb.i_partition == D_8x8 as libc::c_int {
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            x264_8_mb_mc_8x8(h, i);
            i += 1;
            i;
        }
    } else {
        let mut ref0a: libc::c_int = (*h)
            .mb
            .cache
            .ref_0[0 as libc::c_int
            as usize][x264_scan8[0 as libc::c_int as usize] as usize] as libc::c_int;
        let mut ref0b: libc::c_int = (*h)
            .mb
            .cache
            .ref_0[0 as libc::c_int
            as usize][x264_scan8[12 as libc::c_int as usize] as usize] as libc::c_int;
        let mut ref1a: libc::c_int = (*h)
            .mb
            .cache
            .ref_0[1 as libc::c_int
            as usize][x264_scan8[0 as libc::c_int as usize] as usize] as libc::c_int;
        let mut ref1b: libc::c_int = (*h)
            .mb
            .cache
            .ref_0[1 as libc::c_int
            as usize][x264_scan8[12 as libc::c_int as usize] as usize] as libc::c_int;
        if (*h).mb.i_partition == D_16x16 as libc::c_int {
            if ref0a >= 0 as libc::c_int {
                if ref1a >= 0 as libc::c_int {
                    mb_mc_01xywh(
                        h,
                        0 as libc::c_int,
                        0 as libc::c_int,
                        4 as libc::c_int,
                        4 as libc::c_int,
                    );
                } else {
                    mb_mc_0xywh(
                        h,
                        0 as libc::c_int,
                        0 as libc::c_int,
                        4 as libc::c_int,
                        4 as libc::c_int,
                    );
                }
            } else {
                mb_mc_1xywh(
                    h,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    4 as libc::c_int,
                    4 as libc::c_int,
                );
            }
        } else if (*h).mb.i_partition == D_16x8 as libc::c_int {
            if ref0a >= 0 as libc::c_int {
                if ref1a >= 0 as libc::c_int {
                    mb_mc_01xywh(
                        h,
                        0 as libc::c_int,
                        0 as libc::c_int,
                        4 as libc::c_int,
                        2 as libc::c_int,
                    );
                } else {
                    mb_mc_0xywh(
                        h,
                        0 as libc::c_int,
                        0 as libc::c_int,
                        4 as libc::c_int,
                        2 as libc::c_int,
                    );
                }
            } else {
                mb_mc_1xywh(
                    h,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    4 as libc::c_int,
                    2 as libc::c_int,
                );
            }
            if ref0b >= 0 as libc::c_int {
                if ref1b >= 0 as libc::c_int {
                    mb_mc_01xywh(
                        h,
                        0 as libc::c_int,
                        2 as libc::c_int,
                        4 as libc::c_int,
                        2 as libc::c_int,
                    );
                } else {
                    mb_mc_0xywh(
                        h,
                        0 as libc::c_int,
                        2 as libc::c_int,
                        4 as libc::c_int,
                        2 as libc::c_int,
                    );
                }
            } else {
                mb_mc_1xywh(
                    h,
                    0 as libc::c_int,
                    2 as libc::c_int,
                    4 as libc::c_int,
                    2 as libc::c_int,
                );
            }
        } else if (*h).mb.i_partition == D_8x16 as libc::c_int {
            if ref0a >= 0 as libc::c_int {
                if ref1a >= 0 as libc::c_int {
                    mb_mc_01xywh(
                        h,
                        0 as libc::c_int,
                        0 as libc::c_int,
                        2 as libc::c_int,
                        4 as libc::c_int,
                    );
                } else {
                    mb_mc_0xywh(
                        h,
                        0 as libc::c_int,
                        0 as libc::c_int,
                        2 as libc::c_int,
                        4 as libc::c_int,
                    );
                }
            } else {
                mb_mc_1xywh(
                    h,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    2 as libc::c_int,
                    4 as libc::c_int,
                );
            }
            if ref0b >= 0 as libc::c_int {
                if ref1b >= 0 as libc::c_int {
                    mb_mc_01xywh(
                        h,
                        2 as libc::c_int,
                        0 as libc::c_int,
                        2 as libc::c_int,
                        4 as libc::c_int,
                    );
                } else {
                    mb_mc_0xywh(
                        h,
                        2 as libc::c_int,
                        0 as libc::c_int,
                        2 as libc::c_int,
                        4 as libc::c_int,
                    );
                }
            } else {
                mb_mc_1xywh(
                    h,
                    2 as libc::c_int,
                    0 as libc::c_int,
                    2 as libc::c_int,
                    4 as libc::c_int,
                );
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_macroblock_cache_allocate(
    mut h: *mut x264_t,
) -> libc::c_int {
    let mut i_mb_count: libc::c_int = (*h).mb.i_mb_count;
    (*h).mb.i_mb_stride = (*h).mb.i_mb_width;
    (*h).mb.i_b8_stride = (*h).mb.i_mb_width * 2 as libc::c_int;
    (*h).mb.i_b4_stride = (*h).mb.i_mb_width * 4 as libc::c_int;
    (*h).mb.b_interlaced = (*h).param.b_interlaced;
    let mut prealloc_idx: libc::c_int = 0 as libc::c_int;
    let mut prealloc_size: int64_t = 0 as libc::c_int as int64_t;
    let mut preallocs: [*mut *mut uint8_t; 1024] = [std::ptr::null_mut::<*mut uint8_t>(); 1024];
    (*h).mb.qp = prealloc_size as *mut libc::c_void as *mut int8_t;
    let fresh0 = prealloc_idx;
    prealloc_idx += 1;
    preallocs[fresh0
        as usize] = &mut (*h).mb.qp as *mut *mut int8_t as *mut *mut uint8_t;
    prealloc_size
        += ((i_mb_count as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<int8_t>() as libc::c_ulong) as int64_t
            + (64 as libc::c_int - 1 as libc::c_int) as int64_t) & !(64 as libc::c_int - 1 as libc::c_int) as int64_t;
    (*h).mb.cbp = prealloc_size as *mut libc::c_void as *mut int16_t;
    let fresh1 = prealloc_idx;
    prealloc_idx += 1;
    preallocs[fresh1
        as usize] = &mut (*h).mb.cbp as *mut *mut int16_t as *mut *mut uint8_t;
    prealloc_size
        += ((i_mb_count as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong) as int64_t
            + (64 as libc::c_int - 1 as libc::c_int) as int64_t) & !(64 as libc::c_int - 1 as libc::c_int) as int64_t;
    (*h).mb.mb_transform_size = prealloc_size as *mut libc::c_void as *mut int8_t;
    let fresh2 = prealloc_idx;
    prealloc_idx += 1;
    preallocs[fresh2
        as usize] = &mut (*h).mb.mb_transform_size as *mut *mut int8_t
        as *mut *mut uint8_t;
    prealloc_size
        += ((i_mb_count as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<int8_t>() as libc::c_ulong) as int64_t
            + (64 as libc::c_int - 1 as libc::c_int) as int64_t) & !(64 as libc::c_int - 1 as libc::c_int) as int64_t;
    (*h).mb.slice_table = prealloc_size as *mut libc::c_void as *mut int32_t;
    let fresh3 = prealloc_idx;
    prealloc_idx += 1;
    preallocs[fresh3
        as usize] = &mut (*h).mb.slice_table as *mut *mut int32_t as *mut *mut uint8_t;
    prealloc_size
        += ((i_mb_count as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<int32_t>() as libc::c_ulong) as int64_t
            + (64 as libc::c_int - 1 as libc::c_int) as int64_t) & !(64 as libc::c_int - 1 as libc::c_int) as int64_t;
    (*h).mb.intra4x4_pred_mode = prealloc_size as *mut libc::c_void as *mut [int8_t; 8];
    let fresh4 = prealloc_idx;
    prealloc_idx += 1;
    preallocs[fresh4
        as usize] = &mut (*h).mb.intra4x4_pred_mode as *mut *mut [int8_t; 8]
        as *mut *mut uint8_t;
    prealloc_size
        += (((i_mb_count * 8 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<int8_t>() as libc::c_ulong) as int64_t
            + (64 as libc::c_int - 1 as libc::c_int) as int64_t) & !(64 as libc::c_int - 1 as libc::c_int) as int64_t;
    (*h).mb.non_zero_count = prealloc_size as *mut libc::c_void as *mut [uint8_t; 48];
    let fresh5 = prealloc_idx;
    prealloc_idx += 1;
    preallocs[fresh5
        as usize] = &mut (*h).mb.non_zero_count as *mut *mut [uint8_t; 48]
        as *mut *mut uint8_t;
    prealloc_size
        += (((i_mb_count * 48 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<uint8_t>() as libc::c_ulong) as int64_t
            + (64 as libc::c_int - 1 as libc::c_int) as int64_t) & !(64 as libc::c_int - 1 as libc::c_int) as int64_t;
    if (*h).param.b_cabac != 0 {
        (*h).mb.skipbp = prealloc_size as *mut libc::c_void as *mut int8_t;
        let fresh6 = prealloc_idx;
        prealloc_idx += 1;
        preallocs[fresh6
            as usize] = &mut (*h).mb.skipbp as *mut *mut int8_t as *mut *mut uint8_t;
        prealloc_size
            += ((i_mb_count as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<int8_t>() as libc::c_ulong)
                as int64_t + (64 as libc::c_int - 1 as libc::c_int) as int64_t) & !(64 as libc::c_int - 1 as libc::c_int) as int64_t;
        (*h).mb.chroma_pred_mode = prealloc_size as *mut libc::c_void as *mut int8_t;
        let fresh7 = prealloc_idx;
        prealloc_idx += 1;
        preallocs[fresh7
            as usize] = &mut (*h).mb.chroma_pred_mode as *mut *mut int8_t
            as *mut *mut uint8_t;
        prealloc_size
            += ((i_mb_count as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<int8_t>() as libc::c_ulong)
                as int64_t + (64 as libc::c_int - 1 as libc::c_int) as int64_t) & !(64 as libc::c_int - 1 as libc::c_int) as int64_t;
        (*h)
            .mb
            .mvd[0 as libc::c_int
            as usize] = prealloc_size as *mut libc::c_void as *mut [[uint8_t; 2]; 8];
        let fresh8 = prealloc_idx;
        prealloc_idx += 1;
        preallocs[fresh8
            as usize] = &mut *((*h).mb.mvd)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) as *mut *mut [[uint8_t; 2]; 8]
            as *mut *mut uint8_t;
        prealloc_size
            += ((i_mb_count as libc::c_ulong)
                .wrapping_mul(
                    ::core::mem::size_of::<[[uint8_t; 2]; 8]>() as libc::c_ulong,
                ) as int64_t + (64 as libc::c_int - 1 as libc::c_int) as int64_t) & !(64 as libc::c_int - 1 as libc::c_int) as int64_t;
        if (*h).param.i_bframe != 0 {
            (*h)
                .mb
                .mvd[1 as libc::c_int
                as usize] = prealloc_size as *mut libc::c_void as *mut [[uint8_t; 2]; 8];
            let fresh9 = prealloc_idx;
            prealloc_idx += 1;
            preallocs[fresh9
                as usize] = &mut *((*h).mb.mvd)
                .as_mut_ptr()
                .offset(1 as libc::c_int as isize) as *mut *mut [[uint8_t; 2]; 8]
                as *mut *mut uint8_t;
            prealloc_size
                += ((i_mb_count as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<[[uint8_t; 2]; 8]>() as libc::c_ulong,
                    ) as int64_t + (64 as libc::c_int - 1 as libc::c_int) as int64_t) & !(64 as libc::c_int - 1 as libc::c_int) as int64_t;
        }
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        let mut i_refs: libc::c_int = (if (16 as libc::c_int)
            < (if i != 0 {
                1 as libc::c_int + ((*h).param.i_bframe_pyramid != 0) as libc::c_int
            } else {
                (*h).param.i_frame_reference
            })
        {
            16 as libc::c_int
        } else if i != 0 {
            1 as libc::c_int + ((*h).param.i_bframe_pyramid != 0) as libc::c_int
        } else {
            (*h).param.i_frame_reference
        }) << (*h).param.b_interlaced;
        if (*h).param.analyse.i_weighted_pred == 2 as libc::c_int {
            i_refs = if (16 as libc::c_int)
                < i_refs + 1 as libc::c_int
                    + (8 as libc::c_int == 8 as libc::c_int) as libc::c_int
            {
                16 as libc::c_int
            } else {
                i_refs + 1 as libc::c_int
                    + (8 as libc::c_int == 8 as libc::c_int) as libc::c_int
            };
        }
        let mut j: libc::c_int = (i == 0) as libc::c_int;
        while j < i_refs {
            (*h)
                .mb
                .mvr[i
                as usize][j
                as usize] = prealloc_size as *mut libc::c_void as *mut [int16_t; 2];
            let fresh10 = prealloc_idx;
            prealloc_idx += 1;
            preallocs[fresh10
                as usize] = &mut *(*((*h).mb.mvr).as_mut_ptr().offset(i as isize))
                .as_mut_ptr()
                .offset(j as isize) as *mut *mut [int16_t; 2] as *mut *mut uint8_t;
            prealloc_size
                += (((2 as libc::c_int * (i_mb_count + 1 as libc::c_int))
                    as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong)
                    as int64_t + (64 as libc::c_int - 1 as libc::c_int) as int64_t) & !(64 as libc::c_int - 1 as libc::c_int) as int64_t;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    if (*h).param.analyse.i_weighted_pred != 0 {
        let mut i_padv: libc::c_int = (32 as libc::c_int) << (*h).param.b_interlaced;
        let mut luma_plane_size: libc::c_int = 0 as libc::c_int;
        let mut numweightbuf: libc::c_int = 0;
        if (*h).param.analyse.i_weighted_pred == -(1 as libc::c_int) {
            if (*h).param.i_sync_lookahead == 0
                || h == (*h).thread[(*h).param.i_threads as usize]
            {
                luma_plane_size = (*(*h).fdec).i_stride_lowres
                    * ((*h).mb.i_mb_height * 8 as libc::c_int
                        + 2 as libc::c_int * i_padv);
                numweightbuf = 1 as libc::c_int;
            } else {
                numweightbuf = 0 as libc::c_int;
            }
        } else {
            luma_plane_size = (*(*h).fdec).i_stride[0 as libc::c_int as usize]
                * ((*h).mb.i_mb_height
                    * ((16 as libc::c_int)
                        << ((*((*h).sps).as_mut_ptr()).i_chroma_format_idc
                            == CHROMA_422 as libc::c_int) as libc::c_int)
                    + 2 as libc::c_int * i_padv);
            if (*h).param.analyse.i_weighted_pred == 2 as libc::c_int {
                numweightbuf = 1 as libc::c_int
                    + (8 as libc::c_int == 8 as libc::c_int) as libc::c_int;
            } else {
                numweightbuf = 1 as libc::c_int;
            }
        }
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < numweightbuf {
            (*h)
                .mb
                .p_weight_buf[i_0
                as usize] = prealloc_size as *mut libc::c_void as *mut pixel;
            let fresh11 = prealloc_idx;
            prealloc_idx += 1;
            preallocs[fresh11
                as usize] = &mut *((*h).mb.p_weight_buf)
                .as_mut_ptr()
                .offset(i_0 as isize) as *mut *mut pixel as *mut *mut uint8_t;
            prealloc_size
                += ((luma_plane_size
                    * ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int)
                    as int64_t + (64 as libc::c_int - 1 as libc::c_int) as int64_t) & !(64 as libc::c_int - 1 as libc::c_int) as int64_t;
            i_0 += 1;
            i_0;
        }
    }
    (*h).mb.base = x264_malloc(prealloc_size) as *mut uint8_t;
    if ((*h).mb.base).is_null() {
        -(1 as libc::c_int)
    } else {
        loop {
            let fresh12 = prealloc_idx;
            prealloc_idx -= 1;
            if fresh12 == 0 {
                break;
            }
            *preallocs[prealloc_idx
                as usize] = (*preallocs[prealloc_idx as usize] as intptr_t
                + (*h).mb.base as intptr_t) as *mut uint8_t;
        }
        memset(
            (*h).mb.slice_table as *mut libc::c_void,
            -(1 as libc::c_int),
            (i_mb_count as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<int32_t>() as libc::c_ulong),
        );
        let mut i_1: libc::c_int = 0 as libc::c_int;
        while i_1 < 2 as libc::c_int {
            let mut i_refs_0: libc::c_int = (if (16 as libc::c_int)
                < (if i_1 != 0 {
                    1 as libc::c_int + ((*h).param.i_bframe_pyramid != 0) as libc::c_int
                } else {
                    (*h).param.i_frame_reference
                })
            {
                16 as libc::c_int
            } else if i_1 != 0 {
                1 as libc::c_int + ((*h).param.i_bframe_pyramid != 0) as libc::c_int
            } else {
                (*h).param.i_frame_reference
            }) << (*h).param.b_interlaced;
            if (*h).param.analyse.i_weighted_pred == 2 as libc::c_int {
                i_refs_0 = if (16 as libc::c_int)
                    < i_refs_0 + 1 as libc::c_int
                        + (8 as libc::c_int == 8 as libc::c_int) as libc::c_int
                {
                    16 as libc::c_int
                } else {
                    i_refs_0 + 1 as libc::c_int
                        + (8 as libc::c_int == 8 as libc::c_int) as libc::c_int
                };
            }
            let mut j_0: libc::c_int = (i_1 == 0) as libc::c_int;
            while j_0 < i_refs_0 {
                (*((*((*h).mb.mvr[i_1 as usize][j_0 as usize])
                    .offset(0 as libc::c_int as isize))
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i = 0 as libc::c_int as uint32_t;
                (*h)
                    .mb
                    .mvr[i_1
                    as usize][j_0
                    as usize] = ((*h).mb.mvr[i_1 as usize][j_0 as usize]).offset(1);
                (*h).mb.mvr[i_1 as usize][j_0 as usize];
                j_0 += 1;
                j_0;
            }
            i_1 += 1;
            i_1;
        }
        0 as libc::c_int
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_macroblock_cache_free(mut h: *mut x264_t) {
    x264_free((*h).mb.base as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_macroblock_thread_allocate(
    mut h: *mut x264_t,
    mut b_lookahead: libc::c_int,
) -> libc::c_int {
    let mut scratch_size: libc::c_int = 0;
    let mut buf_mbtree: libc::c_int = 0;
    let mut buf_lookahead_threads: libc::c_int = 0;
    let mut buf_mbtree2: libc::c_int = 0;
    let mut current_block: u64;
    if b_lookahead == 0 {
        let mut i: libc::c_int = 0 as libc::c_int;
        's_5: loop {
            if i >= (if (*h).param.b_interlaced != 0 {
                    5 as libc::c_int
                } else {
                    2 as libc::c_int
                })
            {
                current_block = 8515828400728868193;
                break;
            }
            let mut j: libc::c_int = 0 as libc::c_int;
            while j
                < (if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc
                    == CHROMA_444 as libc::c_int
                {
                    3 as libc::c_int
                } else {
                    2 as libc::c_int
                })
            {
                (*h)
                    .intra_border_backup[i
                    as usize][j
                    as usize] = x264_malloc(
                    (((*((*h).sps).as_mut_ptr()).i_mb_width * 16 as libc::c_int
                        + 32 as libc::c_int)
                        * ::core::mem::size_of::<pixel>() as libc::c_ulong
                            as libc::c_int) as int64_t,
                ) as *mut pixel;
                if ((*h).intra_border_backup[i as usize][j as usize]).is_null() {
                    current_block = 16210657447058269181;
                    break 's_5;
                }
                (*h)
                    .intra_border_backup[i
                    as usize][j
                    as usize] = ((*h).intra_border_backup[i as usize][j as usize])
                    .offset(16 as libc::c_int as isize);
                j += 1;
                j;
            }
            i += 1;
            i;
        }
        match current_block {
            16210657447058269181 => {}
            _ => {
                let mut i_0: libc::c_int = 0 as libc::c_int;
                loop {
                    if i_0 > (*h).param.b_interlaced {
                        current_block = 5783071609795492627;
                        break;
                    }
                    if (*h).param.b_sliced_threads != 0 {
                        if h == (*h).thread[0 as libc::c_int as usize] && i_0 == 0 {
                            (*h)
                                .deblock_strength[0 as libc::c_int
                                as usize] = x264_malloc(
                                (::core::mem::size_of::<[[[uint8_t; 4]; 8]; 2]>()
                                    as libc::c_ulong)
                                    .wrapping_mul((*h).mb.i_mb_count as libc::c_ulong)
                                    as int64_t,
                            ) as *mut [[[uint8_t; 4]; 8]; 2];
                            if ((*h).deblock_strength[0 as libc::c_int as usize])
                                .is_null()
                            {
                                current_block = 16210657447058269181;
                                break;
                            }
                        } else {
                            (*h)
                                .deblock_strength[i_0
                                as usize] = (*(*h).thread[0 as libc::c_int as usize])
                                .deblock_strength[0 as libc::c_int as usize];
                        }
                    } else {
                        (*h)
                            .deblock_strength[i_0
                            as usize] = x264_malloc(
                            (::core::mem::size_of::<[[[uint8_t; 4]; 8]; 2]>()
                                as libc::c_ulong)
                                .wrapping_mul((*h).mb.i_mb_width as libc::c_ulong)
                                as int64_t,
                        ) as *mut [[[uint8_t; 4]; 8]; 2];
                        if ((*h).deblock_strength[i_0 as usize]).is_null() {
                            current_block = 16210657447058269181;
                            break;
                        }
                    }
                    (*h)
                        .deblock_strength[1 as libc::c_int
                        as usize] = (*h).deblock_strength[i_0 as usize];
                    i_0 += 1;
                    i_0;
                }
            }
        }
    } else {
        current_block = 5783071609795492627;
    }
    if current_block == 5783071609795492627 {
        scratch_size = 0 as libc::c_int;
        if b_lookahead == 0 {
            let mut buf_hpel: libc::c_int = (((*(*(*h)
                .thread[0 as libc::c_int as usize])
                .fdec)
                .i_width[0 as libc::c_int as usize] + 48 as libc::c_int
                + 32 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong)
                as libc::c_int;
            let mut buf_ssim: libc::c_int = (((*h).param.analyse.b_ssim
                * 8 as libc::c_int
                * ((*h).param.i_width / 4 as libc::c_int + 3 as libc::c_int))
                as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                as libc::c_int;
            let mut me_range: libc::c_int = if (*h).param.analyse.i_me_range
                < (*h).param.analyse.i_mv_range
            {
                (*h).param.analyse.i_me_range
            } else {
                (*h).param.analyse.i_mv_range
            };
            let mut buf_tesa: libc::c_int = (((*h).param.analyse.i_me_method
                >= 3 as libc::c_int) as libc::c_int as libc::c_ulong)
                .wrapping_mul(
                    ((me_range * 2 as libc::c_int + 24 as libc::c_int)
                        as libc::c_ulong)
                        .wrapping_mul(
                            ::core::mem::size_of::<int16_t>() as libc::c_ulong,
                        )
                        .wrapping_add(
                            (((me_range + 4 as libc::c_int)
                                * (me_range + 1 as libc::c_int) * 4 as libc::c_int)
                                as libc::c_ulong)
                                .wrapping_mul(
                                    ::core::mem::size_of::<mvsad_t>() as libc::c_ulong,
                                ),
                        ),
                ) as libc::c_int;
            scratch_size = if buf_hpel
                > (if buf_ssim > buf_tesa { buf_ssim } else { buf_tesa })
            {
                buf_hpel
            } else if buf_ssim > buf_tesa {
                buf_ssim
            } else {
                buf_tesa
            };
        }
        buf_mbtree = ((*h).param.rc.b_mb_tree as libc::c_ulong)
            .wrapping_mul(
                ((*h).mb.i_mb_width as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<int16_t>() as libc::c_ulong)
                    .wrapping_add(
                        (64 as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
                    ) & !(64 as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
            ) as libc::c_int;
        scratch_size = if scratch_size > buf_mbtree {
            scratch_size
        } else {
            buf_mbtree
        };
        if scratch_size != 0 {
            (*h).scratch_buffer = x264_malloc(scratch_size as int64_t);
            if ((*h).scratch_buffer).is_null() {
                current_block = 16210657447058269181;
            } else {
                current_block = 2891135413264362348;
            }
        } else {
            (*h).scratch_buffer = std::ptr::null_mut::<libc::c_void>();
            current_block = 2891135413264362348;
        }
        match current_block {
            16210657447058269181 => {}
            _ => {
                buf_lookahead_threads = (((*h).mb.i_mb_height
                    + (4 as libc::c_int + 32 as libc::c_int)
                        * (*h).param.i_lookahead_threads) as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                    )
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong) as libc::c_int;
                buf_mbtree2 = buf_mbtree * 12 as libc::c_int;
                scratch_size = if buf_lookahead_threads > buf_mbtree2 {
                    buf_lookahead_threads
                } else {
                    buf_mbtree2
                };
                (*h).scratch_buffer2 = x264_malloc(scratch_size as int64_t);
                if !((*h).scratch_buffer2).is_null() {
                    return 0 as libc::c_int;
                }
            }
        }
    }
    -(1 as libc::c_int)
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_macroblock_thread_free(
    mut h: *mut x264_t,
    mut b_lookahead: libc::c_int,
) {
    if b_lookahead == 0 {
        let mut i: libc::c_int = 0 as libc::c_int;
        while i <= (*h).param.b_interlaced {
            if (*h).param.b_sliced_threads == 0
                || h == (*h).thread[0 as libc::c_int as usize] && i == 0
            {
                x264_free((*h).deblock_strength[i as usize] as *mut libc::c_void);
            }
            i += 1;
            i;
        }
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0
            < (if (*h).param.b_interlaced != 0 {
                5 as libc::c_int
            } else {
                2 as libc::c_int
            })
        {
            let mut j: libc::c_int = 0 as libc::c_int;
            while j
                < (if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc
                    == CHROMA_444 as libc::c_int
                {
                    3 as libc::c_int
                } else {
                    2 as libc::c_int
                })
            {
                x264_free(
                    ((*h).intra_border_backup[i_0 as usize][j as usize])
                        .offset(-(16 as libc::c_int as isize)) as *mut libc::c_void,
                );
                j += 1;
                j;
            }
            i_0 += 1;
            i_0;
        }
    }
    x264_free((*h).scratch_buffer);
    x264_free((*h).scratch_buffer2);
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_macroblock_slice_init(mut h: *mut x264_t) {
    (*h).mb.mv[0 as libc::c_int as usize] = (*(*h).fdec).mv[0 as libc::c_int as usize];
    (*h).mb.mv[1 as libc::c_int as usize] = (*(*h).fdec).mv[1 as libc::c_int as usize];
    (*h)
        .mb
        .mvr[0 as libc::c_int
        as usize][0 as libc::c_int as usize] = (*(*h).fdec).mv16x16;
    (*h)
        .mb
        .ref_0[0 as libc::c_int
        as usize] = (*(*h).fdec).ref_0[0 as libc::c_int as usize];
    (*h)
        .mb
        .ref_0[1 as libc::c_int
        as usize] = (*(*h).fdec).ref_0[1 as libc::c_int as usize];
    (*h).mb.type_0 = (*(*h).fdec).mb_type;
    (*h).mb.partition = (*(*h).fdec).mb_partition;
    (*h).mb.field = (*(*h).fdec).field;
    (*(*h).fdec)
        .i_ref[0 as libc::c_int as usize] = (*h).i_ref[0 as libc::c_int as usize];
    (*(*h).fdec)
        .i_ref[1 as libc::c_int as usize] = (*h).i_ref[1 as libc::c_int as usize];
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*h).i_ref[0 as libc::c_int as usize] {
        (*(*h).fdec)
            .ref_poc[0 as libc::c_int
            as usize][i
            as usize] = (*(*h).fref[0 as libc::c_int as usize][i as usize]).i_poc;
        i += 1;
        i;
    }
    if (*h).sh.i_type == SLICE_TYPE_B as libc::c_int {
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < (*h).i_ref[1 as libc::c_int as usize] {
            (*(*h).fdec)
                .ref_poc[1 as libc::c_int
                as usize][i_0
                as usize] = (*(*h).fref[1 as libc::c_int as usize][i_0 as usize]).i_poc;
            i_0 += 1;
            i_0;
        }
        (*h)
            .mb
            .map_col_to_list0[(-(1 as libc::c_int) + 2 as libc::c_int)
            as usize] = -(1 as libc::c_int) as int8_t;
        (*h)
            .mb
            .map_col_to_list0[(-(2 as libc::c_int) + 2 as libc::c_int)
            as usize] = -(2 as libc::c_int) as int8_t;
        let mut i_1: libc::c_int = 0 as libc::c_int;
        while i_1
            < (*(*h).fref[1 as libc::c_int as usize][0 as libc::c_int as usize])
                .i_ref[0 as libc::c_int as usize]
        {
            let mut poc: libc::c_int = (*(*h)
                .fref[1 as libc::c_int as usize][0 as libc::c_int as usize])
                .ref_poc[0 as libc::c_int as usize][i_1 as usize];
            (*h)
                .mb
                .map_col_to_list0[(i_1 + 2 as libc::c_int)
                as usize] = -(2 as libc::c_int) as int8_t;
            let mut j: libc::c_int = 0 as libc::c_int;
            while j < (*h).i_ref[0 as libc::c_int as usize] {
                if (*(*h).fref[0 as libc::c_int as usize][j as usize]).i_poc == poc {
                    (*h)
                        .mb
                        .map_col_to_list0[(i_1 + 2 as libc::c_int)
                        as usize] = j as int8_t;
                    break;
                } else {
                    j += 1;
                    j;
                }
            }
            i_1 += 1;
            i_1;
        }
    } else if (*h).sh.i_type == SLICE_TYPE_P as libc::c_int && (*h).sh.i_disable_deblocking_filter_idc != 1 as libc::c_int && (*h).param.analyse.i_weighted_pred == 2 as libc::c_int {
        (*h)
            .mb
            .deblock_ref_table[(-(2 as libc::c_int) + 2 as libc::c_int)
            as usize] = -(2 as libc::c_int) as int8_t;
        (*h)
            .mb
            .deblock_ref_table[(-(1 as libc::c_int) + 2 as libc::c_int)
            as usize] = -(1 as libc::c_int) as int8_t;
        let mut i_2: libc::c_int = 0 as libc::c_int;
        while i_2 < (*h).i_ref[0 as libc::c_int as usize] << (*h).sh.b_mbaff {
            if (*h).mb.b_interlaced == 0 {
                (*h)
                    .mb
                    .deblock_ref_table[(i_2 + 2 as libc::c_int)
                    as usize] = ((*(*h)
                    .fref[0 as libc::c_int as usize][i_2 as usize])
                    .i_frame_num & 63 as libc::c_int) as int8_t;
            } else {
                (*h)
                    .mb
                    .deblock_ref_table[(i_2 + 2 as libc::c_int)
                    as usize] = ((((*(*h)
                    .fref[0 as libc::c_int
                    as usize][(i_2 >> 1 as libc::c_int) as usize])
                    .i_frame_num & 63 as libc::c_int) << 1 as libc::c_int)
                    + (i_2 & 1 as libc::c_int)) as int8_t;
            }
            i_2 += 1;
            i_2;
        }
    }
    memset(
        ((*h).mb.cache.ref_0).as_mut_ptr() as *mut libc::c_void,
        -(2 as libc::c_int),
        ::core::mem::size_of::<[[int8_t; 40]; 2]>() as libc::c_ulong,
    );
    if (*h).i_ref[0 as libc::c_int as usize] > 0 as libc::c_int {
        let mut field: libc::c_int = 0 as libc::c_int;
        while field <= (*h).sh.b_mbaff {
            let mut curpoc: libc::c_int = (*(*h).fdec).i_poc
                + (*(*h).fdec).i_delta_poc[field as usize];
            let mut refpoc: libc::c_int = (*(*h)
                .fref[0 as libc::c_int as usize][0 as libc::c_int as usize])
                .i_poc
                + (*(*h).fref[0 as libc::c_int as usize][0 as libc::c_int as usize])
                    .i_delta_poc[field as usize];
            let mut delta: libc::c_int = curpoc - refpoc;
            (*(*h).fdec)
                .inv_ref_poc[field
                as usize] = ((256 as libc::c_int + delta / 2 as libc::c_int) / delta)
                as int16_t;
            field += 1;
            field;
        }
    }
    (*h)
        .mb
        .i_neighbour4[14 as libc::c_int
        as usize] = (MB_LEFT as libc::c_int | MB_TOP as libc::c_int
        | MB_TOPLEFT as libc::c_int | MB_TOPRIGHT as libc::c_int) as libc::c_uint;
    (*h)
        .mb
        .i_neighbour4[12 as libc::c_int
        as usize] = (*h).mb.i_neighbour4[14 as libc::c_int as usize];
    (*h)
        .mb
        .i_neighbour4[9 as libc::c_int
        as usize] = (*h).mb.i_neighbour4[12 as libc::c_int as usize];
    (*h)
        .mb
        .i_neighbour4[6 as libc::c_int
        as usize] = (*h).mb.i_neighbour4[9 as libc::c_int as usize];
    (*h)
        .mb
        .i_neighbour8[3 as libc::c_int
        as usize] = (MB_LEFT as libc::c_int | MB_TOP as libc::c_int
        | MB_TOPLEFT as libc::c_int) as libc::c_uint;
    (*h)
        .mb
        .i_neighbour4[15 as libc::c_int
        as usize] = (*h).mb.i_neighbour8[3 as libc::c_int as usize];
    (*h)
        .mb
        .i_neighbour4[13 as libc::c_int
        as usize] = (*h).mb.i_neighbour4[15 as libc::c_int as usize];
    (*h)
        .mb
        .i_neighbour4[11 as libc::c_int
        as usize] = (*h).mb.i_neighbour4[13 as libc::c_int as usize];
    (*h)
        .mb
        .i_neighbour4[7 as libc::c_int
        as usize] = (*h).mb.i_neighbour4[11 as libc::c_int as usize];
    (*h)
        .mb
        .i_neighbour4[3 as libc::c_int
        as usize] = (*h).mb.i_neighbour4[7 as libc::c_int as usize];
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_macroblock_thread_init(mut h: *mut x264_t) {
    (*h).mb.i_me_method = (*h).param.analyse.i_me_method;
    (*h).mb.i_subpel_refine = (*h).param.analyse.i_subpel_refine;
    if (*h).sh.i_type == SLICE_TYPE_B as libc::c_int
        && ((*h).mb.i_subpel_refine == 6 as libc::c_int
            || (*h).mb.i_subpel_refine == 8 as libc::c_int)
    {
        (*h).mb.i_subpel_refine -= 1;
        (*h).mb.i_subpel_refine;
    }
    (*h)
        .mb
        .b_chroma_me = ((*h).param.analyse.b_chroma_me != 0
        && ((*h).sh.i_type == SLICE_TYPE_P as libc::c_int
            && (*h).mb.i_subpel_refine >= 5 as libc::c_int
            || (*h).sh.i_type == SLICE_TYPE_B as libc::c_int
                && (*h).mb.i_subpel_refine >= 9 as libc::c_int)) as libc::c_int;
    (*h)
        .mb
        .b_dct_decimate = ((*h).sh.i_type == SLICE_TYPE_B as libc::c_int
        || (*h).param.analyse.b_dct_decimate != 0
            && (*h).sh.i_type != SLICE_TYPE_I as libc::c_int) as libc::c_int;
    (*h).mb.i_mb_prev_xy = -(1 as libc::c_int);
    (*h).mb.pic.p_fenc[0 as libc::c_int as usize] = ((*h).mb.pic.fenc_buf).as_mut_ptr();
    (*h)
        .mb
        .pic
        .p_fdec[0 as libc::c_int
        as usize] = ((*h).mb.pic.fdec_buf)
        .as_mut_ptr()
        .offset((2 as libc::c_int * 32 as libc::c_int) as isize);
    if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc != 0 {
        (*h)
            .mb
            .pic
            .p_fenc[1 as libc::c_int
            as usize] = ((*h).mb.pic.fenc_buf)
            .as_mut_ptr()
            .offset((16 as libc::c_int * 16 as libc::c_int) as isize);
        (*h)
            .mb
            .pic
            .p_fdec[1 as libc::c_int
            as usize] = ((*h).mb.pic.fdec_buf)
            .as_mut_ptr()
            .offset((20 as libc::c_int * 32 as libc::c_int) as isize);
        if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as libc::c_int {
            (*h)
                .mb
                .pic
                .p_fenc[2 as libc::c_int
                as usize] = ((*h).mb.pic.fenc_buf)
                .as_mut_ptr()
                .offset((32 as libc::c_int * 16 as libc::c_int) as isize);
            (*h)
                .mb
                .pic
                .p_fdec[2 as libc::c_int
                as usize] = ((*h).mb.pic.fdec_buf)
                .as_mut_ptr()
                .offset((38 as libc::c_int * 32 as libc::c_int) as isize);
        } else {
            (*h)
                .mb
                .pic
                .p_fenc[2 as libc::c_int
                as usize] = ((*h).mb.pic.fenc_buf)
                .as_mut_ptr()
                .offset((16 as libc::c_int * 16 as libc::c_int) as isize)
                .offset(8 as libc::c_int as isize);
            (*h)
                .mb
                .pic
                .p_fdec[2 as libc::c_int
                as usize] = ((*h).mb.pic.fdec_buf)
                .as_mut_ptr()
                .offset((20 as libc::c_int * 32 as libc::c_int) as isize)
                .offset(16 as libc::c_int as isize);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_prefetch_fenc(
    mut h: *mut x264_t,
    mut fenc: *mut x264_frame_t,
    mut i_mb_x: libc::c_int,
    mut i_mb_y: libc::c_int,
) {
    let mut stride_y: libc::c_int = (*fenc).i_stride[0 as libc::c_int as usize];
    let mut stride_uv: libc::c_int = (*fenc).i_stride[1 as libc::c_int as usize];
    let mut off_y: libc::c_int = 16 as libc::c_int * i_mb_x
        + 16 as libc::c_int * i_mb_y * stride_y;
    let mut off_uv: libc::c_int = 16 as libc::c_int * i_mb_x
        + ((16 as libc::c_int * i_mb_y * stride_uv) >> (*h).mb.chroma_v_shift);
    ((*h).mc.prefetch_fenc)
        .expect(
            "non-null function pointer",
        )(
        ((*fenc).plane[0 as libc::c_int as usize]).offset(off_y as isize),
        stride_y as intptr_t,
        if !((*fenc).plane[1 as libc::c_int as usize]).is_null() {
            ((*fenc).plane[1 as libc::c_int as usize]).offset(off_uv as isize)
        } else {
            std::ptr::null_mut::<pixel>()
        },
        stride_uv as intptr_t,
        i_mb_x,
    );
}
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn x264_8_copy_column8(mut dst: *mut pixel, mut src: *mut pixel) {
    let mut i: libc::c_int = -(4 as libc::c_int);
    while i < 4 as libc::c_int {
        *dst
            .offset(
                (i * 32 as libc::c_int) as isize,
            ) = *src.offset((i * 32 as libc::c_int) as isize);
        i += 1;
        i;
    }
}
#[inline(always)]
unsafe extern "C" fn macroblock_load_pic_pointers(
    mut h: *mut x264_t,
    mut mb_x: libc::c_int,
    mut mb_y: libc::c_int,
    mut i: libc::c_int,
    mut b_chroma: libc::c_int,
    mut b_mbaff: libc::c_int,
) {
    let mut mb_interlaced: libc::c_int = (b_mbaff != 0 && (*h).mb.b_interlaced != 0)
        as libc::c_int;
    let mut height: libc::c_int = if b_chroma != 0 {
        16 as libc::c_int >> (*h).mb.chroma_v_shift
    } else {
        16 as libc::c_int
    };
    let mut i_stride: libc::c_int = (*(*h).fdec).i_stride[i as usize];
    let mut i_stride2: libc::c_int = i_stride << mb_interlaced;
    let mut i_pix_offset: libc::c_int = if mb_interlaced != 0 {
        16 as libc::c_int * mb_x + height * (mb_y & !(1 as libc::c_int)) * i_stride
            + (mb_y & 1 as libc::c_int) * i_stride
    } else {
        16 as libc::c_int * mb_x + height * mb_y * i_stride
    };
    let mut plane_fdec: *mut pixel = &mut *(*((*(*h).fdec).plane)
        .as_mut_ptr()
        .offset(i as isize))
        .offset(i_pix_offset as isize) as *mut pixel;
    let mut fdec_idx: libc::c_int = if b_mbaff != 0 {
        if mb_interlaced != 0 {
            3 as libc::c_int + (mb_y & 1 as libc::c_int)
        } else if mb_y & 1 as libc::c_int != 0 {
            2 as libc::c_int
        } else {
            4 as libc::c_int
        }
    } else {
        (mb_y & 1 as libc::c_int == 0) as libc::c_int
    };
    let mut intra_fdec: *mut pixel = &mut *(*(*((*h).intra_border_backup)
        .as_mut_ptr()
        .offset(fdec_idx as isize))
        .as_mut_ptr()
        .offset(i as isize))
        .offset((mb_x * 16 as libc::c_int) as isize) as *mut pixel;
    let mut ref_pix_offset: [libc::c_int; 2] = [i_pix_offset, i_pix_offset];
    if mb_interlaced != 0 {
        ref_pix_offset[1 as libc::c_int as usize]
            += (1 as libc::c_int - 2 as libc::c_int * (mb_y & 1 as libc::c_int))
                * i_stride;
    }
    (*h).mb.pic.i_stride[i as usize] = i_stride2;
    (*h)
        .mb
        .pic
        .p_fenc_plane[i
        as usize] = &mut *(*((*(*h).fenc).plane).as_mut_ptr().offset(i as isize))
        .offset(i_pix_offset as isize) as *mut pixel;
    if b_chroma != 0 {
        ((*h).mc.load_deinterleave_chroma_fenc)
            .expect(
                "non-null function pointer",
            )(
            (*h).mb.pic.p_fenc[1 as libc::c_int as usize],
            (*h).mb.pic.p_fenc_plane[1 as libc::c_int as usize],
            i_stride2 as intptr_t,
            height,
        );
        memcpy(
            ((*h).mb.pic.p_fdec[1 as libc::c_int as usize])
                .offset(-(32 as libc::c_int as isize)) as *mut libc::c_void,
            intra_fdec as *const libc::c_void,
            (8 as libc::c_int
                * ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int)
                as libc::c_ulong,
        );
        memcpy(
            ((*h).mb.pic.p_fdec[2 as libc::c_int as usize])
                .offset(-(32 as libc::c_int as isize)) as *mut libc::c_void,
            intra_fdec.offset(8 as libc::c_int as isize) as *const libc::c_void,
            (8 as libc::c_int
                * ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int)
                as libc::c_ulong,
        );
        *((*h).mb.pic.p_fdec[1 as libc::c_int as usize])
            .offset(
                (-(32 as libc::c_int) - 1 as libc::c_int) as isize,
            ) = *intra_fdec.offset((-(1 as libc::c_int) - 8 as libc::c_int) as isize);
        *((*h).mb.pic.p_fdec[2 as libc::c_int as usize])
            .offset(
                (-(32 as libc::c_int) - 1 as libc::c_int) as isize,
            ) = *intra_fdec.offset(-(1 as libc::c_int) as isize);
    } else {
        ((*h).mc.copy[PIXEL_16x16 as libc::c_int as usize])
            .expect(
                "non-null function pointer",
            )(
            (*h).mb.pic.p_fenc[i as usize],
            16 as libc::c_int as intptr_t,
            (*h).mb.pic.p_fenc_plane[i as usize],
            i_stride2 as intptr_t,
            16 as libc::c_int,
        );
        memcpy(
            ((*h).mb.pic.p_fdec[i as usize]).offset(-(32 as libc::c_int as isize))
                as *mut libc::c_void,
            intra_fdec as *const libc::c_void,
            (24 as libc::c_int
                * ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int)
                as libc::c_ulong,
        );
        *((*h).mb.pic.p_fdec[i as usize])
            .offset(
                (-(32 as libc::c_int) - 1 as libc::c_int) as isize,
            ) = *intra_fdec.offset(-(1 as libc::c_int) as isize);
    }
    if b_mbaff != 0 || (*h).mb.b_reencode_mb != 0 {
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < height {
            if b_chroma != 0 {
                *((*h).mb.pic.p_fdec[1 as libc::c_int as usize])
                    .offset(
                        (-(1 as libc::c_int) + j * 32 as libc::c_int) as isize,
                    ) = *plane_fdec
                    .offset((-(2 as libc::c_int) + j * i_stride2) as isize);
                *((*h).mb.pic.p_fdec[2 as libc::c_int as usize])
                    .offset(
                        (-(1 as libc::c_int) + j * 32 as libc::c_int) as isize,
                    ) = *plane_fdec
                    .offset((-(1 as libc::c_int) + j * i_stride2) as isize);
            } else {
                *((*h).mb.pic.p_fdec[i as usize])
                    .offset(
                        (-(1 as libc::c_int) + j * 32 as libc::c_int) as isize,
                    ) = *plane_fdec
                    .offset((-(1 as libc::c_int) + j * i_stride2) as isize);
            }
            j += 1;
            j;
        }
    }
    let mut plane_src: *mut pixel = std::ptr::null_mut::<pixel>();
    let mut filtered_src: *mut *mut pixel = std::ptr::null_mut::<*mut pixel>();
    let mut j_0: libc::c_int = 0 as libc::c_int;
    while j_0 < (*h).mb.pic.i_fref[0 as libc::c_int as usize] {
        if mb_interlaced != 0 {
            plane_src = (*(*h)
                .fref[0 as libc::c_int as usize][(j_0 >> 1 as libc::c_int) as usize])
                .plane_fld[i as usize];
            filtered_src = ((*(*h)
                .fref[0 as libc::c_int as usize][(j_0 >> 1 as libc::c_int) as usize])
                .filtered_fld[i as usize])
                .as_mut_ptr();
        } else {
            plane_src = (*(*h).fref[0 as libc::c_int as usize][j_0 as usize])
                .plane[i as usize];
            filtered_src = ((*(*h).fref[0 as libc::c_int as usize][j_0 as usize])
                .filtered[i as usize])
                .as_mut_ptr();
        }
        (*h)
            .mb
            .pic
            .p_fref[0 as libc::c_int
            as usize][j_0
            as usize][(i * 4 as libc::c_int)
            as usize] = plane_src
            .offset(ref_pix_offset[(j_0 & 1 as libc::c_int) as usize] as isize);
        if b_chroma == 0 {
            if (*h).param.analyse.i_subpel_refine != 0 {
                let mut k: libc::c_int = 1 as libc::c_int;
                while k < 4 as libc::c_int {
                    (*h)
                        .mb
                        .pic
                        .p_fref[0 as libc::c_int
                        as usize][j_0
                        as usize][(i * 4 as libc::c_int + k)
                        as usize] = (*filtered_src.offset(k as isize))
                        .offset(
                            ref_pix_offset[(j_0 & 1 as libc::c_int) as usize] as isize,
                        );
                    k += 1;
                    k;
                }
            }
            if i == 0 {
                if !((*h).sh.weight[j_0 as usize][0 as libc::c_int as usize].weightfn)
                    .is_null()
                {
                    (*h)
                        .mb
                        .pic
                        .p_fref_w[j_0
                        as usize] = &mut *(*((*(*h).fenc).weighted)
                        .as_mut_ptr()
                        .offset((j_0 >> mb_interlaced) as isize))
                        .offset(
                            *ref_pix_offset
                                .as_mut_ptr()
                                .offset((j_0 & 1 as libc::c_int) as isize) as isize,
                        ) as *mut pixel;
                } else {
                    (*h)
                        .mb
                        .pic
                        .p_fref_w[j_0
                        as usize] = (*h)
                        .mb
                        .pic
                        .p_fref[0 as libc::c_int
                        as usize][j_0 as usize][0 as libc::c_int as usize];
                }
            }
        }
        j_0 += 1;
        j_0;
    }
    if (*h).sh.i_type == SLICE_TYPE_B as libc::c_int {
        let mut j_1: libc::c_int = 0 as libc::c_int;
        while j_1 < (*h).mb.pic.i_fref[1 as libc::c_int as usize] {
            if mb_interlaced != 0 {
                plane_src = (*(*h)
                    .fref[1 as libc::c_int as usize][(j_1 >> 1 as libc::c_int) as usize])
                    .plane_fld[i as usize];
                filtered_src = ((*(*h)
                    .fref[1 as libc::c_int as usize][(j_1 >> 1 as libc::c_int) as usize])
                    .filtered_fld[i as usize])
                    .as_mut_ptr();
            } else {
                plane_src = (*(*h).fref[1 as libc::c_int as usize][j_1 as usize])
                    .plane[i as usize];
                filtered_src = ((*(*h).fref[1 as libc::c_int as usize][j_1 as usize])
                    .filtered[i as usize])
                    .as_mut_ptr();
            }
            (*h)
                .mb
                .pic
                .p_fref[1 as libc::c_int
                as usize][j_1
                as usize][(i * 4 as libc::c_int)
                as usize] = plane_src
                .offset(ref_pix_offset[(j_1 & 1 as libc::c_int) as usize] as isize);
            if b_chroma == 0 && (*h).param.analyse.i_subpel_refine != 0 {
                let mut k_0: libc::c_int = 1 as libc::c_int;
                while k_0 < 4 as libc::c_int {
                    (*h)
                        .mb
                        .pic
                        .p_fref[1 as libc::c_int
                        as usize][j_1
                        as usize][(i * 4 as libc::c_int + k_0)
                        as usize] = (*filtered_src.offset(k_0 as isize))
                        .offset(
                            ref_pix_offset[(j_1 & 1 as libc::c_int) as usize] as isize,
                        );
                    k_0 += 1;
                    k_0;
                }
            }
            j_1 += 1;
            j_1;
        }
    }
}
static mut left_indices: [x264_left_table_t; 4] = [
    {
        
        x264_left_table_t {
            intra: [
                4 as libc::c_int as uint8_t,
                4 as libc::c_int as uint8_t,
                5 as libc::c_int as uint8_t,
                5 as libc::c_int as uint8_t,
            ],
            nnz: [
                3 as libc::c_int as uint8_t,
                3 as libc::c_int as uint8_t,
                7 as libc::c_int as uint8_t,
                7 as libc::c_int as uint8_t,
            ],
            nnz_chroma: [
                (16 as libc::c_int + 1 as libc::c_int) as uint8_t,
                (16 as libc::c_int + 1 as libc::c_int) as uint8_t,
                (32 as libc::c_int + 1 as libc::c_int) as uint8_t,
                (32 as libc::c_int + 1 as libc::c_int) as uint8_t,
            ],
            mv: [
                0 as libc::c_int as uint8_t,
                0 as libc::c_int as uint8_t,
                1 as libc::c_int as uint8_t,
                1 as libc::c_int as uint8_t,
            ],
            ref_0: [
                0 as libc::c_int as uint8_t,
                0 as libc::c_int as uint8_t,
                0 as libc::c_int as uint8_t,
                0 as libc::c_int as uint8_t,
            ],
        }
    },
    {
        
        x264_left_table_t {
            intra: [
                6 as libc::c_int as uint8_t,
                6 as libc::c_int as uint8_t,
                3 as libc::c_int as uint8_t,
                3 as libc::c_int as uint8_t,
            ],
            nnz: [
                11 as libc::c_int as uint8_t,
                11 as libc::c_int as uint8_t,
                15 as libc::c_int as uint8_t,
                15 as libc::c_int as uint8_t,
            ],
            nnz_chroma: [
                (16 as libc::c_int + 5 as libc::c_int) as uint8_t,
                (16 as libc::c_int + 5 as libc::c_int) as uint8_t,
                (32 as libc::c_int + 5 as libc::c_int) as uint8_t,
                (32 as libc::c_int + 5 as libc::c_int) as uint8_t,
            ],
            mv: [
                2 as libc::c_int as uint8_t,
                2 as libc::c_int as uint8_t,
                3 as libc::c_int as uint8_t,
                3 as libc::c_int as uint8_t,
            ],
            ref_0: [
                1 as libc::c_int as uint8_t,
                1 as libc::c_int as uint8_t,
                1 as libc::c_int as uint8_t,
                1 as libc::c_int as uint8_t,
            ],
        }
    },
    {
        
        x264_left_table_t {
            intra: [
                4 as libc::c_int as uint8_t,
                6 as libc::c_int as uint8_t,
                4 as libc::c_int as uint8_t,
                6 as libc::c_int as uint8_t,
            ],
            nnz: [
                3 as libc::c_int as uint8_t,
                11 as libc::c_int as uint8_t,
                3 as libc::c_int as uint8_t,
                11 as libc::c_int as uint8_t,
            ],
            nnz_chroma: [
                (16 as libc::c_int + 1 as libc::c_int) as uint8_t,
                (16 as libc::c_int + 1 as libc::c_int) as uint8_t,
                (32 as libc::c_int + 1 as libc::c_int) as uint8_t,
                (32 as libc::c_int + 1 as libc::c_int) as uint8_t,
            ],
            mv: [
                0 as libc::c_int as uint8_t,
                2 as libc::c_int as uint8_t,
                0 as libc::c_int as uint8_t,
                2 as libc::c_int as uint8_t,
            ],
            ref_0: [
                0 as libc::c_int as uint8_t,
                1 as libc::c_int as uint8_t,
                0 as libc::c_int as uint8_t,
                1 as libc::c_int as uint8_t,
            ],
        }
    },
    {
        
        x264_left_table_t {
            intra: [
                4 as libc::c_int as uint8_t,
                5 as libc::c_int as uint8_t,
                6 as libc::c_int as uint8_t,
                3 as libc::c_int as uint8_t,
            ],
            nnz: [
                3 as libc::c_int as uint8_t,
                7 as libc::c_int as uint8_t,
                11 as libc::c_int as uint8_t,
                15 as libc::c_int as uint8_t,
            ],
            nnz_chroma: [
                (16 as libc::c_int + 1 as libc::c_int) as uint8_t,
                (16 as libc::c_int + 5 as libc::c_int) as uint8_t,
                (32 as libc::c_int + 1 as libc::c_int) as uint8_t,
                (32 as libc::c_int + 5 as libc::c_int) as uint8_t,
            ],
            mv: [
                0 as libc::c_int as uint8_t,
                1 as libc::c_int as uint8_t,
                2 as libc::c_int as uint8_t,
                3 as libc::c_int as uint8_t,
            ],
            ref_0: [
                0 as libc::c_int as uint8_t,
                0 as libc::c_int as uint8_t,
                1 as libc::c_int as uint8_t,
                1 as libc::c_int as uint8_t,
            ],
        }
    },
];
#[inline(always)]
unsafe extern "C" fn macroblock_cache_load_neighbours(
    mut h: *mut x264_t,
    mut mb_x: libc::c_int,
    mut mb_y: libc::c_int,
    mut b_interlaced: libc::c_int,
) {
    let mb_interlaced: libc::c_int = (b_interlaced != 0 && (*h).mb.b_interlaced != 0)
        as libc::c_int;
    let mut top_y: libc::c_int = mb_y - ((1 as libc::c_int) << mb_interlaced);
    let mut top: libc::c_int = top_y * (*h).mb.i_mb_stride + mb_x;
    (*h).mb.i_mb_x = mb_x;
    (*h).mb.i_mb_y = mb_y;
    (*h).mb.i_mb_xy = mb_y * (*h).mb.i_mb_stride + mb_x;
    (*h).mb.i_b8_xy = 2 as libc::c_int * (mb_y * (*h).mb.i_b8_stride + mb_x);
    (*h).mb.i_b4_xy = 4 as libc::c_int * (mb_y * (*h).mb.i_b4_stride + mb_x);
    (*h).mb.left_b8[1 as libc::c_int as usize] = -(1 as libc::c_int);
    (*h)
        .mb
        .left_b8[0 as libc::c_int as usize] = (*h).mb.left_b8[1 as libc::c_int as usize];
    (*h).mb.left_b4[1 as libc::c_int as usize] = -(1 as libc::c_int);
    (*h)
        .mb
        .left_b4[0 as libc::c_int as usize] = (*h).mb.left_b4[1 as libc::c_int as usize];
    (*h).mb.i_neighbour = 0 as libc::c_int as libc::c_uint;
    (*h).mb.i_neighbour_intra = 0 as libc::c_int as libc::c_uint;
    (*h).mb.i_neighbour_frame = 0 as libc::c_int as libc::c_uint;
    (*h).mb.i_mb_top_xy = -(1 as libc::c_int);
    (*h).mb.i_mb_top_y = -(1 as libc::c_int);
    (*h).mb.i_mb_left_xy[1 as libc::c_int as usize] = -(1 as libc::c_int);
    (*h)
        .mb
        .i_mb_left_xy[0 as libc::c_int
        as usize] = (*h).mb.i_mb_left_xy[1 as libc::c_int as usize];
    (*h).mb.i_mb_topleft_xy = -(1 as libc::c_int);
    (*h).mb.i_mb_topright_xy = -(1 as libc::c_int);
    (*h).mb.i_mb_type_top = -(1 as libc::c_int);
    (*h).mb.i_mb_type_left[1 as libc::c_int as usize] = -(1 as libc::c_int);
    (*h)
        .mb
        .i_mb_type_left[0 as libc::c_int
        as usize] = (*h).mb.i_mb_type_left[1 as libc::c_int as usize];
    (*h).mb.i_mb_type_topleft = -(1 as libc::c_int);
    (*h).mb.i_mb_type_topright = -(1 as libc::c_int);
    (*h)
        .mb
        .left_index_table = &*left_indices.as_ptr().offset(3 as libc::c_int as isize)
        as *const x264_left_table_t;
    (*h).mb.topleft_partition = 0 as libc::c_int;
    let mut topleft_y: libc::c_int = top_y;
    let mut topright_y: libc::c_int = top_y;
    let mut left: [libc::c_int; 2] = [0; 2];
    left[1 as libc::c_int as usize] = (*h).mb.i_mb_xy - 1 as libc::c_int;
    left[0 as libc::c_int as usize] = left[1 as libc::c_int as usize];
    (*h).mb.left_b8[1 as libc::c_int as usize] = (*h).mb.i_b8_xy - 2 as libc::c_int;
    (*h)
        .mb
        .left_b8[0 as libc::c_int as usize] = (*h).mb.left_b8[1 as libc::c_int as usize];
    (*h).mb.left_b4[1 as libc::c_int as usize] = (*h).mb.i_b4_xy - 4 as libc::c_int;
    (*h)
        .mb
        .left_b4[0 as libc::c_int as usize] = (*h).mb.left_b4[1 as libc::c_int as usize];
    if b_interlaced != 0 {
        (*h)
            .mb
            .i_mb_top_mbpair_xy = (*h).mb.i_mb_xy
            - 2 as libc::c_int * (*h).mb.i_mb_stride;
        (*h).mb.i_mb_topleft_y = -(1 as libc::c_int);
        (*h).mb.i_mb_topright_y = -(1 as libc::c_int);
        if mb_y & 1 as libc::c_int != 0 {
            if mb_x != 0
                && mb_interlaced
                    != *((*h).mb.field)
                        .offset(((*h).mb.i_mb_xy - 1 as libc::c_int) as isize)
                        as libc::c_int
            {
                left[1 as libc::c_int
                    as usize] = (*h).mb.i_mb_xy - 1 as libc::c_int - (*h).mb.i_mb_stride;
                left[0 as libc::c_int as usize] = left[1 as libc::c_int as usize];
                (*h)
                    .mb
                    .left_b8[1 as libc::c_int
                    as usize] = (*h).mb.i_b8_xy - 2 as libc::c_int
                    - 2 as libc::c_int * (*h).mb.i_b8_stride;
                (*h)
                    .mb
                    .left_b8[0 as libc::c_int
                    as usize] = (*h).mb.left_b8[1 as libc::c_int as usize];
                (*h)
                    .mb
                    .left_b4[1 as libc::c_int
                    as usize] = (*h).mb.i_b4_xy - 4 as libc::c_int
                    - 4 as libc::c_int * (*h).mb.i_b4_stride;
                (*h)
                    .mb
                    .left_b4[0 as libc::c_int
                    as usize] = (*h).mb.left_b4[1 as libc::c_int as usize];
                if mb_interlaced != 0 {
                    (*h)
                        .mb
                        .left_index_table = &*left_indices
                        .as_ptr()
                        .offset(2 as libc::c_int as isize) as *const x264_left_table_t;
                    left[1 as libc::c_int as usize] += (*h).mb.i_mb_stride;
                    (*h).mb.left_b8[1 as libc::c_int as usize]
                        += 2 as libc::c_int * (*h).mb.i_b8_stride;
                    (*h).mb.left_b4[1 as libc::c_int as usize]
                        += 4 as libc::c_int * (*h).mb.i_b4_stride;
                } else {
                    (*h)
                        .mb
                        .left_index_table = &*left_indices
                        .as_ptr()
                        .offset(1 as libc::c_int as isize) as *const x264_left_table_t;
                    topleft_y += 1;
                    topleft_y;
                    (*h).mb.topleft_partition = 1 as libc::c_int;
                }
            }
            if mb_interlaced == 0 {
                topright_y = -(1 as libc::c_int);
            }
        } else {
            if mb_interlaced != 0 && top >= 0 as libc::c_int {
                if *((*h).mb.field).offset(top as isize) == 0 {
                    top += (*h).mb.i_mb_stride;
                    top_y += 1;
                    top_y;
                }
                if mb_x != 0 {
                    topleft_y
                        += (*((*h).mb.field)
                            .offset(
                                ((*h).mb.i_mb_stride * topleft_y + mb_x - 1 as libc::c_int)
                                    as isize,
                            ) == 0) as libc::c_int;
                }
                if mb_x < (*h).mb.i_mb_width - 1 as libc::c_int {
                    topright_y
                        += (*((*h).mb.field)
                            .offset(
                                ((*h).mb.i_mb_stride * topright_y + mb_x + 1 as libc::c_int)
                                    as isize,
                            ) == 0) as libc::c_int;
                }
            }
            if mb_x != 0
                && mb_interlaced
                    != *((*h).mb.field)
                        .offset(((*h).mb.i_mb_xy - 1 as libc::c_int) as isize)
                        as libc::c_int
            {
                if mb_interlaced != 0 {
                    (*h)
                        .mb
                        .left_index_table = &*left_indices
                        .as_ptr()
                        .offset(2 as libc::c_int as isize) as *const x264_left_table_t;
                    left[1 as libc::c_int as usize] += (*h).mb.i_mb_stride;
                    (*h).mb.left_b8[1 as libc::c_int as usize]
                        += 2 as libc::c_int * (*h).mb.i_b8_stride;
                    (*h).mb.left_b4[1 as libc::c_int as usize]
                        += 4 as libc::c_int * (*h).mb.i_b4_stride;
                } else {
                    (*h)
                        .mb
                        .left_index_table = &*left_indices
                        .as_ptr()
                        .offset(0 as libc::c_int as isize) as *const x264_left_table_t;
                }
            }
        }
    }
    if mb_x > 0 as libc::c_int {
        (*h).mb.i_neighbour_frame |= MB_LEFT as libc::c_int as libc::c_uint;
        (*h)
            .mb
            .i_mb_left_xy[0 as libc::c_int as usize] = left[0 as libc::c_int as usize];
        (*h)
            .mb
            .i_mb_left_xy[1 as libc::c_int as usize] = left[1 as libc::c_int as usize];
        (*h)
            .mb
            .i_mb_type_left[0 as libc::c_int
            as usize] = *((*h).mb.type_0)
            .offset((*h).mb.i_mb_left_xy[0 as libc::c_int as usize] as isize)
            as libc::c_int;
        (*h)
            .mb
            .i_mb_type_left[1 as libc::c_int
            as usize] = *((*h).mb.type_0)
            .offset((*h).mb.i_mb_left_xy[1 as libc::c_int as usize] as isize)
            as libc::c_int;
        if *((*h).mb.slice_table).offset(left[0 as libc::c_int as usize] as isize)
            == (*h).sh.i_first_mb
        {
            (*h).mb.i_neighbour |= MB_LEFT as libc::c_int as libc::c_uint;
            if (*h).param.b_constrained_intra == 0
                || ((*h).mb.i_mb_type_left[0 as libc::c_int as usize]
                    == I_4x4 as libc::c_int
                    || (*h).mb.i_mb_type_left[0 as libc::c_int as usize]
                        == I_8x8 as libc::c_int
                    || (*h).mb.i_mb_type_left[0 as libc::c_int as usize]
                        == I_16x16 as libc::c_int
                    || (*h).mb.i_mb_type_left[0 as libc::c_int as usize]
                        == I_PCM as libc::c_int)
            {
                (*h).mb.i_neighbour_intra |= MB_LEFT as libc::c_int as libc::c_uint;
            }
        }
    }
    if (*h).i_threadslice_start >> mb_interlaced != mb_y >> mb_interlaced {
        if top >= 0 as libc::c_int {
            (*h).mb.i_neighbour_frame |= MB_TOP as libc::c_int as libc::c_uint;
            (*h).mb.i_mb_top_xy = top;
            (*h).mb.i_mb_top_y = top_y;
            (*h)
                .mb
                .i_mb_type_top = *((*h).mb.type_0).offset((*h).mb.i_mb_top_xy as isize)
                as libc::c_int;
            if *((*h).mb.slice_table).offset(top as isize) == (*h).sh.i_first_mb {
                (*h).mb.i_neighbour |= MB_TOP as libc::c_int as libc::c_uint;
                if (*h).param.b_constrained_intra == 0
                    || ((*h).mb.i_mb_type_top == I_4x4 as libc::c_int
                        || (*h).mb.i_mb_type_top == I_8x8 as libc::c_int
                        || (*h).mb.i_mb_type_top == I_16x16 as libc::c_int
                        || (*h).mb.i_mb_type_top == I_PCM as libc::c_int)
                {
                    (*h).mb.i_neighbour_intra |= MB_TOP as libc::c_int as libc::c_uint;
                }
                ((*h).mb.cbp).offset(top as isize);
                (*((*h).mb.non_zero_count).offset(top as isize))
                    .as_mut_ptr()
                    .offset(12 as libc::c_int as isize);
                ((*h).mb.mb_transform_size).offset(top as isize);
                if (*h).param.b_cabac != 0 {
                    ((*h).mb.skipbp).offset(top as isize);
                }
            }
        }
        if mb_x > 0 as libc::c_int && topleft_y >= 0 as libc::c_int {
            (*h).mb.i_neighbour_frame |= MB_TOPLEFT as libc::c_int as libc::c_uint;
            (*h)
                .mb
                .i_mb_topleft_xy = (*h).mb.i_mb_stride * topleft_y + mb_x
                - 1 as libc::c_int;
            (*h).mb.i_mb_topleft_y = topleft_y;
            (*h)
                .mb
                .i_mb_type_topleft = *((*h).mb.type_0)
                .offset((*h).mb.i_mb_topleft_xy as isize) as libc::c_int;
            if *((*h).mb.slice_table).offset((*h).mb.i_mb_topleft_xy as isize)
                == (*h).sh.i_first_mb
            {
                (*h).mb.i_neighbour |= MB_TOPLEFT as libc::c_int as libc::c_uint;
                if (*h).param.b_constrained_intra == 0
                    || ((*h).mb.i_mb_type_topleft == I_4x4 as libc::c_int
                        || (*h).mb.i_mb_type_topleft == I_8x8 as libc::c_int
                        || (*h).mb.i_mb_type_topleft == I_16x16 as libc::c_int
                        || (*h).mb.i_mb_type_topleft == I_PCM as libc::c_int)
                {
                    (*h).mb.i_neighbour_intra
                        |= MB_TOPLEFT as libc::c_int as libc::c_uint;
                }
            }
        }
        if mb_x < (*h).mb.i_mb_width - 1 as libc::c_int && topright_y >= 0 as libc::c_int
        {
            (*h).mb.i_neighbour_frame |= MB_TOPRIGHT as libc::c_int as libc::c_uint;
            (*h)
                .mb
                .i_mb_topright_xy = (*h).mb.i_mb_stride * topright_y + mb_x
                + 1 as libc::c_int;
            (*h).mb.i_mb_topright_y = topright_y;
            (*h)
                .mb
                .i_mb_type_topright = *((*h).mb.type_0)
                .offset((*h).mb.i_mb_topright_xy as isize) as libc::c_int;
            if *((*h).mb.slice_table).offset((*h).mb.i_mb_topright_xy as isize)
                == (*h).sh.i_first_mb
            {
                (*h).mb.i_neighbour |= MB_TOPRIGHT as libc::c_int as libc::c_uint;
                if (*h).param.b_constrained_intra == 0
                    || ((*h).mb.i_mb_type_topright == I_4x4 as libc::c_int
                        || (*h).mb.i_mb_type_topright == I_8x8 as libc::c_int
                        || (*h).mb.i_mb_type_topright == I_16x16 as libc::c_int
                        || (*h).mb.i_mb_type_topright == I_PCM as libc::c_int)
                {
                    (*h).mb.i_neighbour_intra
                        |= MB_TOPRIGHT as libc::c_int as libc::c_uint;
                }
            }
        }
    }
}
#[inline(always)]
unsafe extern "C" fn macroblock_cache_load(
    mut h: *mut x264_t,
    mut mb_x: libc::c_int,
    mut mb_y: libc::c_int,
    mut b_mbaff: libc::c_int,
) {
    macroblock_cache_load_neighbours(h, mb_x, mb_y, b_mbaff);
    let mut left: *mut libc::c_int = ((*h).mb.i_mb_left_xy).as_mut_ptr();
    let mut top: libc::c_int = (*h).mb.i_mb_top_xy;
    let mut top_y: libc::c_int = (*h).mb.i_mb_top_y;
    let mut s8x8: libc::c_int = (*h).mb.i_b8_stride;
    let mut s4x4: libc::c_int = (*h).mb.i_b4_stride;
    let mut top_8x8: libc::c_int = (2 as libc::c_int * top_y + 1 as libc::c_int) * s8x8
        + 2 as libc::c_int * mb_x;
    let mut top_4x4: libc::c_int = (4 as libc::c_int * top_y + 3 as libc::c_int) * s4x4
        + 4 as libc::c_int * mb_x;
    let mut lists: libc::c_int = ((1 as libc::c_int) << (*h).sh.i_type) & 3 as libc::c_int;
    let mut i4x4: *mut [int8_t; 8] = (*h).mb.intra4x4_pred_mode;
    let mut nnz: *mut [uint8_t; 48] = (*h).mb.non_zero_count;
    let mut cbp: *mut int16_t = (*h).mb.cbp;
    let mut left_index_table: *const x264_left_table_t = (*h).mb.left_index_table;
    (*h)
        .mb
        .cache
        .deblock_strength = (*((*h).deblock_strength[(mb_y & 1 as libc::c_int) as usize])
        .offset(
            (if (*h).param.b_sliced_threads != 0 { (*h).mb.i_mb_xy } else { mb_x })
                as isize,
        ))
        .as_mut_ptr();
    if (*h).mb.i_neighbour & MB_TOP as libc::c_int as libc::c_uint != 0 {
        (*h).mb.cache.i_cbp_top = *cbp.offset(top as isize) as libc::c_int;
        (*(&mut *((*h).mb.cache.intra4x4_pred_mode)
            .as_mut_ptr()
            .offset(
                (*x264_scan8.as_ptr().offset(0 as libc::c_int as isize) as libc::c_int
                    - 8 as libc::c_int) as isize,
            ) as *mut int8_t as *mut x264_union32_t))
            .i = (*(&mut *(*i4x4.offset(top as isize))
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) as *mut int8_t as *mut x264_union32_t))
            .i;
        (*(&mut *((*h).mb.cache.non_zero_count)
            .as_mut_ptr()
            .offset(
                (*x264_scan8.as_ptr().offset(0 as libc::c_int as isize) as libc::c_int
                    - 8 as libc::c_int) as isize,
            ) as *mut uint8_t as *mut x264_union32_t))
            .i = (*(&mut *(*nnz.offset(top as isize))
            .as_mut_ptr()
            .offset(12 as libc::c_int as isize) as *mut uint8_t as *mut x264_union32_t))
            .i;
        (*(&mut *((*h).mb.cache.non_zero_count)
            .as_mut_ptr()
            .offset(
                (*x264_scan8.as_ptr().offset(16 as libc::c_int as isize) as libc::c_int
                    - 8 as libc::c_int) as isize,
            ) as *mut uint8_t as *mut x264_union32_t))
            .i = (*(&mut *(*nnz.offset(top as isize))
            .as_mut_ptr()
            .offset(
                (16 as libc::c_int - 4 as libc::c_int
                    + (16 as libc::c_int >> (*h).mb.chroma_v_shift)) as isize,
            ) as *mut uint8_t as *mut x264_union32_t))
            .i;
        (*(&mut *((*h).mb.cache.non_zero_count)
            .as_mut_ptr()
            .offset(
                (*x264_scan8.as_ptr().offset(32 as libc::c_int as isize) as libc::c_int
                    - 8 as libc::c_int) as isize,
            ) as *mut uint8_t as *mut x264_union32_t))
            .i = (*(&mut *(*nnz.offset(top as isize))
            .as_mut_ptr()
            .offset(
                (32 as libc::c_int - 4 as libc::c_int
                    + (16 as libc::c_int >> (*h).mb.chroma_v_shift)) as isize,
            ) as *mut uint8_t as *mut x264_union32_t))
            .i;
        let mut l: libc::c_int = 0 as libc::c_int;
        while l < lists {
            (*((*h).mb.mv).as_mut_ptr().offset(l as isize))
                .offset((top_4x4 - 1 as libc::c_int) as isize);
            (*((*h).mb.mv).as_mut_ptr().offset(l as isize))
                .offset((top_4x4 + 4 as libc::c_int) as isize);
            (*((*h).mb.ref_0).as_mut_ptr().offset(l as isize))
                .offset((top_8x8 - 1 as libc::c_int) as isize);
            if (*h).param.b_cabac != 0 {
                (*((*h).mb.mvd).as_mut_ptr().offset(l as isize))
                    .offset(top as isize);
            }
            l += 1;
            l;
        }
    } else {
        (*h).mb.cache.i_cbp_top = -(1 as libc::c_int);
        (*(&mut *((*h).mb.cache.intra4x4_pred_mode)
            .as_mut_ptr()
            .offset(
                (*x264_scan8.as_ptr().offset(0 as libc::c_int as isize) as libc::c_int
                    - 8 as libc::c_int) as isize,
            ) as *mut int8_t as *mut x264_union32_t))
            .i = 0xffffffff as libc::c_uint;
        (*(&mut *((*h).mb.cache.non_zero_count)
            .as_mut_ptr()
            .offset(
                (*x264_scan8.as_ptr().offset(0 as libc::c_int as isize) as libc::c_int
                    - 8 as libc::c_int) as isize,
            ) as *mut uint8_t as *mut x264_union32_t))
            .i = 0x80808080 as libc::c_uint;
        (*(&mut *((*h).mb.cache.non_zero_count)
            .as_mut_ptr()
            .offset(
                (*x264_scan8.as_ptr().offset(16 as libc::c_int as isize) as libc::c_int
                    - 8 as libc::c_int) as isize,
            ) as *mut uint8_t as *mut x264_union32_t))
            .i = 0x80808080 as libc::c_uint;
        (*(&mut *((*h).mb.cache.non_zero_count)
            .as_mut_ptr()
            .offset(
                (*x264_scan8.as_ptr().offset(32 as libc::c_int as isize) as libc::c_int
                    - 8 as libc::c_int) as isize,
            ) as *mut uint8_t as *mut x264_union32_t))
            .i = 0x80808080 as libc::c_uint;
    }
    if (*h).mb.i_neighbour & MB_LEFT as libc::c_int as libc::c_uint != 0 {
        let mut ltop: libc::c_int = *left.offset(0 as libc::c_int as isize);
        let mut lbot: libc::c_int = if b_mbaff != 0 {
            *left.offset(1 as libc::c_int as isize)
        } else {
            ltop
        };
        if b_mbaff != 0 {
            let top_luma: int16_t = ((*cbp.offset(ltop as isize) as libc::c_int
                >> ((*left_index_table).mv[0 as libc::c_int as usize] as libc::c_int
                    & !(1 as libc::c_int))) & 2 as libc::c_int) as int16_t;
            let bot_luma: int16_t = ((*cbp.offset(lbot as isize) as libc::c_int
                >> ((*left_index_table).mv[2 as libc::c_int as usize] as libc::c_int
                    & !(1 as libc::c_int))) & 2 as libc::c_int) as int16_t;
            (*h)
                .mb
                .cache
                .i_cbp_left = *cbp.offset(ltop as isize) as libc::c_int
                & 0xfff0 as libc::c_int | ((bot_luma as libc::c_int) << 2 as libc::c_int)
                | top_luma as libc::c_int;
        } else {
            (*h).mb.cache.i_cbp_left = *cbp.offset(ltop as isize) as libc::c_int;
        }
        (*h)
            .mb
            .cache
            .intra4x4_pred_mode[(x264_scan8[0 as libc::c_int as usize] as libc::c_int
            - 1 as libc::c_int)
            as usize] = (*i4x4
            .offset(
                ltop as isize,
            ))[(*left_index_table).intra[0 as libc::c_int as usize] as usize];
        (*h)
            .mb
            .cache
            .intra4x4_pred_mode[(x264_scan8[2 as libc::c_int as usize] as libc::c_int
            - 1 as libc::c_int)
            as usize] = (*i4x4
            .offset(
                ltop as isize,
            ))[(*left_index_table).intra[1 as libc::c_int as usize] as usize];
        (*h)
            .mb
            .cache
            .intra4x4_pred_mode[(x264_scan8[8 as libc::c_int as usize] as libc::c_int
            - 1 as libc::c_int)
            as usize] = (*i4x4
            .offset(
                lbot as isize,
            ))[(*left_index_table).intra[2 as libc::c_int as usize] as usize];
        (*h)
            .mb
            .cache
            .intra4x4_pred_mode[(x264_scan8[10 as libc::c_int as usize] as libc::c_int
            - 1 as libc::c_int)
            as usize] = (*i4x4
            .offset(
                lbot as isize,
            ))[(*left_index_table).intra[3 as libc::c_int as usize] as usize];
        (*h)
            .mb
            .cache
            .non_zero_count[(x264_scan8[0 as libc::c_int as usize] as libc::c_int
            - 1 as libc::c_int)
            as usize] = (*nnz
            .offset(
                ltop as isize,
            ))[(*left_index_table).nnz[0 as libc::c_int as usize] as usize];
        (*h)
            .mb
            .cache
            .non_zero_count[(x264_scan8[2 as libc::c_int as usize] as libc::c_int
            - 1 as libc::c_int)
            as usize] = (*nnz
            .offset(
                ltop as isize,
            ))[(*left_index_table).nnz[1 as libc::c_int as usize] as usize];
        (*h)
            .mb
            .cache
            .non_zero_count[(x264_scan8[8 as libc::c_int as usize] as libc::c_int
            - 1 as libc::c_int)
            as usize] = (*nnz
            .offset(
                lbot as isize,
            ))[(*left_index_table).nnz[2 as libc::c_int as usize] as usize];
        (*h)
            .mb
            .cache
            .non_zero_count[(x264_scan8[10 as libc::c_int as usize] as libc::c_int
            - 1 as libc::c_int)
            as usize] = (*nnz
            .offset(
                lbot as isize,
            ))[(*left_index_table).nnz[3 as libc::c_int as usize] as usize];
        if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc >= CHROMA_422 as libc::c_int {
            let mut offset: libc::c_int = (4 as libc::c_int >> (*h).mb.chroma_h_shift)
                - 4 as libc::c_int;
            (*h)
                .mb
                .cache
                .non_zero_count[(x264_scan8[(16 as libc::c_int + 0 as libc::c_int)
                as usize] as libc::c_int - 1 as libc::c_int)
                as usize] = (*nnz
                .offset(
                    ltop as isize,
                ))[((*left_index_table).nnz[0 as libc::c_int as usize] as libc::c_int
                + 16 as libc::c_int + offset) as usize];
            (*h)
                .mb
                .cache
                .non_zero_count[(x264_scan8[(16 as libc::c_int + 2 as libc::c_int)
                as usize] as libc::c_int - 1 as libc::c_int)
                as usize] = (*nnz
                .offset(
                    ltop as isize,
                ))[((*left_index_table).nnz[1 as libc::c_int as usize] as libc::c_int
                + 16 as libc::c_int + offset) as usize];
            (*h)
                .mb
                .cache
                .non_zero_count[(x264_scan8[(16 as libc::c_int + 8 as libc::c_int)
                as usize] as libc::c_int - 1 as libc::c_int)
                as usize] = (*nnz
                .offset(
                    lbot as isize,
                ))[((*left_index_table).nnz[2 as libc::c_int as usize] as libc::c_int
                + 16 as libc::c_int + offset) as usize];
            (*h)
                .mb
                .cache
                .non_zero_count[(x264_scan8[(16 as libc::c_int + 10 as libc::c_int)
                as usize] as libc::c_int - 1 as libc::c_int)
                as usize] = (*nnz
                .offset(
                    lbot as isize,
                ))[((*left_index_table).nnz[3 as libc::c_int as usize] as libc::c_int
                + 16 as libc::c_int + offset) as usize];
            (*h)
                .mb
                .cache
                .non_zero_count[(x264_scan8[(32 as libc::c_int + 0 as libc::c_int)
                as usize] as libc::c_int - 1 as libc::c_int)
                as usize] = (*nnz
                .offset(
                    ltop as isize,
                ))[((*left_index_table).nnz[0 as libc::c_int as usize] as libc::c_int
                + 32 as libc::c_int + offset) as usize];
            (*h)
                .mb
                .cache
                .non_zero_count[(x264_scan8[(32 as libc::c_int + 2 as libc::c_int)
                as usize] as libc::c_int - 1 as libc::c_int)
                as usize] = (*nnz
                .offset(
                    ltop as isize,
                ))[((*left_index_table).nnz[1 as libc::c_int as usize] as libc::c_int
                + 32 as libc::c_int + offset) as usize];
            (*h)
                .mb
                .cache
                .non_zero_count[(x264_scan8[(32 as libc::c_int + 8 as libc::c_int)
                as usize] as libc::c_int - 1 as libc::c_int)
                as usize] = (*nnz
                .offset(
                    lbot as isize,
                ))[((*left_index_table).nnz[2 as libc::c_int as usize] as libc::c_int
                + 32 as libc::c_int + offset) as usize];
            (*h)
                .mb
                .cache
                .non_zero_count[(x264_scan8[(32 as libc::c_int + 10 as libc::c_int)
                as usize] as libc::c_int - 1 as libc::c_int)
                as usize] = (*nnz
                .offset(
                    lbot as isize,
                ))[((*left_index_table).nnz[3 as libc::c_int as usize] as libc::c_int
                + 32 as libc::c_int + offset) as usize];
        } else {
            (*h)
                .mb
                .cache
                .non_zero_count[(x264_scan8[(16 as libc::c_int + 0 as libc::c_int)
                as usize] as libc::c_int - 1 as libc::c_int)
                as usize] = (*nnz
                .offset(
                    ltop as isize,
                ))[(*left_index_table).nnz_chroma[0 as libc::c_int as usize] as usize];
            (*h)
                .mb
                .cache
                .non_zero_count[(x264_scan8[(16 as libc::c_int + 2 as libc::c_int)
                as usize] as libc::c_int - 1 as libc::c_int)
                as usize] = (*nnz
                .offset(
                    lbot as isize,
                ))[(*left_index_table).nnz_chroma[1 as libc::c_int as usize] as usize];
            (*h)
                .mb
                .cache
                .non_zero_count[(x264_scan8[(32 as libc::c_int + 0 as libc::c_int)
                as usize] as libc::c_int - 1 as libc::c_int)
                as usize] = (*nnz
                .offset(
                    ltop as isize,
                ))[(*left_index_table).nnz_chroma[2 as libc::c_int as usize] as usize];
            (*h)
                .mb
                .cache
                .non_zero_count[(x264_scan8[(32 as libc::c_int + 2 as libc::c_int)
                as usize] as libc::c_int - 1 as libc::c_int)
                as usize] = (*nnz
                .offset(
                    lbot as isize,
                ))[(*left_index_table).nnz_chroma[3 as libc::c_int as usize] as usize];
        }
    } else {
        (*h).mb.cache.i_cbp_left = -(1 as libc::c_int);
        (*h)
            .mb
            .cache
            .intra4x4_pred_mode[(x264_scan8[10 as libc::c_int as usize] as libc::c_int
            - 1 as libc::c_int) as usize] = -(1 as libc::c_int) as int8_t;
        (*h)
            .mb
            .cache
            .intra4x4_pred_mode[(x264_scan8[8 as libc::c_int as usize] as libc::c_int
            - 1 as libc::c_int)
            as usize] = (*h)
            .mb
            .cache
            .intra4x4_pred_mode[(x264_scan8[10 as libc::c_int as usize] as libc::c_int
            - 1 as libc::c_int) as usize];
        (*h)
            .mb
            .cache
            .intra4x4_pred_mode[(x264_scan8[2 as libc::c_int as usize] as libc::c_int
            - 1 as libc::c_int)
            as usize] = (*h)
            .mb
            .cache
            .intra4x4_pred_mode[(x264_scan8[8 as libc::c_int as usize] as libc::c_int
            - 1 as libc::c_int) as usize];
        (*h)
            .mb
            .cache
            .intra4x4_pred_mode[(x264_scan8[0 as libc::c_int as usize] as libc::c_int
            - 1 as libc::c_int)
            as usize] = (*h)
            .mb
            .cache
            .intra4x4_pred_mode[(x264_scan8[2 as libc::c_int as usize] as libc::c_int
            - 1 as libc::c_int) as usize];
        (*h)
            .mb
            .cache
            .non_zero_count[(x264_scan8[(32 as libc::c_int + 2 as libc::c_int) as usize]
            as libc::c_int - 1 as libc::c_int)
            as usize] = 0x80 as libc::c_int as uint8_t;
        (*h)
            .mb
            .cache
            .non_zero_count[(x264_scan8[(32 as libc::c_int + 0 as libc::c_int) as usize]
            as libc::c_int - 1 as libc::c_int)
            as usize] = (*h)
            .mb
            .cache
            .non_zero_count[(x264_scan8[(32 as libc::c_int + 2 as libc::c_int) as usize]
            as libc::c_int - 1 as libc::c_int) as usize];
        (*h)
            .mb
            .cache
            .non_zero_count[(x264_scan8[(16 as libc::c_int + 2 as libc::c_int) as usize]
            as libc::c_int - 1 as libc::c_int)
            as usize] = (*h)
            .mb
            .cache
            .non_zero_count[(x264_scan8[(32 as libc::c_int + 0 as libc::c_int) as usize]
            as libc::c_int - 1 as libc::c_int) as usize];
        (*h)
            .mb
            .cache
            .non_zero_count[(x264_scan8[(16 as libc::c_int + 0 as libc::c_int) as usize]
            as libc::c_int - 1 as libc::c_int)
            as usize] = (*h)
            .mb
            .cache
            .non_zero_count[(x264_scan8[(16 as libc::c_int + 2 as libc::c_int) as usize]
            as libc::c_int - 1 as libc::c_int) as usize];
        (*h)
            .mb
            .cache
            .non_zero_count[(x264_scan8[10 as libc::c_int as usize] as libc::c_int
            - 1 as libc::c_int)
            as usize] = (*h)
            .mb
            .cache
            .non_zero_count[(x264_scan8[(16 as libc::c_int + 0 as libc::c_int) as usize]
            as libc::c_int - 1 as libc::c_int) as usize];
        (*h)
            .mb
            .cache
            .non_zero_count[(x264_scan8[8 as libc::c_int as usize] as libc::c_int
            - 1 as libc::c_int)
            as usize] = (*h)
            .mb
            .cache
            .non_zero_count[(x264_scan8[10 as libc::c_int as usize] as libc::c_int
            - 1 as libc::c_int) as usize];
        (*h)
            .mb
            .cache
            .non_zero_count[(x264_scan8[2 as libc::c_int as usize] as libc::c_int
            - 1 as libc::c_int)
            as usize] = (*h)
            .mb
            .cache
            .non_zero_count[(x264_scan8[8 as libc::c_int as usize] as libc::c_int
            - 1 as libc::c_int) as usize];
        (*h)
            .mb
            .cache
            .non_zero_count[(x264_scan8[0 as libc::c_int as usize] as libc::c_int
            - 1 as libc::c_int)
            as usize] = (*h)
            .mb
            .cache
            .non_zero_count[(x264_scan8[2 as libc::c_int as usize] as libc::c_int
            - 1 as libc::c_int) as usize];
        if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc >= CHROMA_422 as libc::c_int {
            (*h)
                .mb
                .cache
                .non_zero_count[(x264_scan8[(32 as libc::c_int + 10 as libc::c_int)
                as usize] as libc::c_int - 1 as libc::c_int)
                as usize] = 0x80 as libc::c_int as uint8_t;
            (*h)
                .mb
                .cache
                .non_zero_count[(x264_scan8[(32 as libc::c_int + 8 as libc::c_int)
                as usize] as libc::c_int - 1 as libc::c_int)
                as usize] = (*h)
                .mb
                .cache
                .non_zero_count[(x264_scan8[(32 as libc::c_int + 10 as libc::c_int)
                as usize] as libc::c_int - 1 as libc::c_int) as usize];
            (*h)
                .mb
                .cache
                .non_zero_count[(x264_scan8[(16 as libc::c_int + 10 as libc::c_int)
                as usize] as libc::c_int - 1 as libc::c_int)
                as usize] = (*h)
                .mb
                .cache
                .non_zero_count[(x264_scan8[(32 as libc::c_int + 8 as libc::c_int)
                as usize] as libc::c_int - 1 as libc::c_int) as usize];
            (*h)
                .mb
                .cache
                .non_zero_count[(x264_scan8[(16 as libc::c_int + 8 as libc::c_int)
                as usize] as libc::c_int - 1 as libc::c_int)
                as usize] = (*h)
                .mb
                .cache
                .non_zero_count[(x264_scan8[(16 as libc::c_int + 10 as libc::c_int)
                as usize] as libc::c_int - 1 as libc::c_int) as usize];
        }
    }
    if (*((*h).pps).as_mut_ptr()).b_transform_8x8_mode != 0 {
        (*h)
            .mb
            .cache
            .i_neighbour_transform_size = ((*h).mb.i_neighbour
            & MB_LEFT as libc::c_int as libc::c_uint != 0
            && *((*h).mb.mb_transform_size)
                .offset(*left.offset(0 as libc::c_int as isize) as isize) as libc::c_int
                != 0) as libc::c_int
            + ((*h).mb.i_neighbour & MB_TOP as libc::c_int as libc::c_uint != 0
                && *((*h).mb.mb_transform_size).offset(top as isize) as libc::c_int != 0)
                as libc::c_int;
    }
    if b_mbaff != 0 {
        (*h)
            .mb
            .pic
            .i_fref[0 as libc::c_int
            as usize] = (*h).i_ref[0 as libc::c_int as usize] << (*h).mb.b_interlaced;
        (*h)
            .mb
            .pic
            .i_fref[1 as libc::c_int
            as usize] = (*h).i_ref[1 as libc::c_int as usize] << (*h).mb.b_interlaced;
    }
    if b_mbaff == 0 {
        x264_8_copy_column8(
            ((*h).mb.pic.p_fdec[0 as libc::c_int as usize])
                .offset(-(1 as libc::c_int as isize))
                .offset((4 as libc::c_int * 32 as libc::c_int) as isize),
            ((*h).mb.pic.p_fdec[0 as libc::c_int as usize])
                .offset(15 as libc::c_int as isize)
                .offset((4 as libc::c_int * 32 as libc::c_int) as isize),
        );
        x264_8_copy_column8(
            ((*h).mb.pic.p_fdec[0 as libc::c_int as usize])
                .offset(-(1 as libc::c_int as isize))
                .offset((12 as libc::c_int * 32 as libc::c_int) as isize),
            ((*h).mb.pic.p_fdec[0 as libc::c_int as usize])
                .offset(15 as libc::c_int as isize)
                .offset((12 as libc::c_int * 32 as libc::c_int) as isize),
        );
        macroblock_load_pic_pointers(
            h,
            mb_x,
            mb_y,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
        );
        if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as libc::c_int {
            x264_8_copy_column8(
                ((*h).mb.pic.p_fdec[1 as libc::c_int as usize])
                    .offset(-(1 as libc::c_int as isize))
                    .offset((4 as libc::c_int * 32 as libc::c_int) as isize),
                ((*h).mb.pic.p_fdec[1 as libc::c_int as usize])
                    .offset(15 as libc::c_int as isize)
                    .offset((4 as libc::c_int * 32 as libc::c_int) as isize),
            );
            x264_8_copy_column8(
                ((*h).mb.pic.p_fdec[1 as libc::c_int as usize])
                    .offset(-(1 as libc::c_int as isize))
                    .offset((12 as libc::c_int * 32 as libc::c_int) as isize),
                ((*h).mb.pic.p_fdec[1 as libc::c_int as usize])
                    .offset(15 as libc::c_int as isize)
                    .offset((12 as libc::c_int * 32 as libc::c_int) as isize),
            );
            x264_8_copy_column8(
                ((*h).mb.pic.p_fdec[2 as libc::c_int as usize])
                    .offset(-(1 as libc::c_int as isize))
                    .offset((4 as libc::c_int * 32 as libc::c_int) as isize),
                ((*h).mb.pic.p_fdec[2 as libc::c_int as usize])
                    .offset(15 as libc::c_int as isize)
                    .offset((4 as libc::c_int * 32 as libc::c_int) as isize),
            );
            x264_8_copy_column8(
                ((*h).mb.pic.p_fdec[2 as libc::c_int as usize])
                    .offset(-(1 as libc::c_int as isize))
                    .offset((12 as libc::c_int * 32 as libc::c_int) as isize),
                ((*h).mb.pic.p_fdec[2 as libc::c_int as usize])
                    .offset(15 as libc::c_int as isize)
                    .offset((12 as libc::c_int * 32 as libc::c_int) as isize),
            );
            macroblock_load_pic_pointers(
                h,
                mb_x,
                mb_y,
                1 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
            );
            macroblock_load_pic_pointers(
                h,
                mb_x,
                mb_y,
                2 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
            );
        } else if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc != 0 {
            x264_8_copy_column8(
                ((*h).mb.pic.p_fdec[1 as libc::c_int as usize])
                    .offset(-(1 as libc::c_int as isize))
                    .offset((4 as libc::c_int * 32 as libc::c_int) as isize),
                ((*h).mb.pic.p_fdec[1 as libc::c_int as usize])
                    .offset(7 as libc::c_int as isize)
                    .offset((4 as libc::c_int * 32 as libc::c_int) as isize),
            );
            x264_8_copy_column8(
                ((*h).mb.pic.p_fdec[2 as libc::c_int as usize])
                    .offset(-(1 as libc::c_int as isize))
                    .offset((4 as libc::c_int * 32 as libc::c_int) as isize),
                ((*h).mb.pic.p_fdec[2 as libc::c_int as usize])
                    .offset(7 as libc::c_int as isize)
                    .offset((4 as libc::c_int * 32 as libc::c_int) as isize),
            );
            if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc
                == CHROMA_422 as libc::c_int
            {
                x264_8_copy_column8(
                    ((*h).mb.pic.p_fdec[1 as libc::c_int as usize])
                        .offset(-(1 as libc::c_int as isize))
                        .offset((12 as libc::c_int * 32 as libc::c_int) as isize),
                    ((*h).mb.pic.p_fdec[1 as libc::c_int as usize])
                        .offset(7 as libc::c_int as isize)
                        .offset((12 as libc::c_int * 32 as libc::c_int) as isize),
                );
                x264_8_copy_column8(
                    ((*h).mb.pic.p_fdec[2 as libc::c_int as usize])
                        .offset(-(1 as libc::c_int as isize))
                        .offset((12 as libc::c_int * 32 as libc::c_int) as isize),
                    ((*h).mb.pic.p_fdec[2 as libc::c_int as usize])
                        .offset(7 as libc::c_int as isize)
                        .offset((12 as libc::c_int * 32 as libc::c_int) as isize),
                );
            }
            macroblock_load_pic_pointers(
                h,
                mb_x,
                mb_y,
                1 as libc::c_int,
                1 as libc::c_int,
                0 as libc::c_int,
            );
        }
    } else {
        macroblock_load_pic_pointers(
            h,
            mb_x,
            mb_y,
            0 as libc::c_int,
            0 as libc::c_int,
            1 as libc::c_int,
        );
        if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as libc::c_int {
            macroblock_load_pic_pointers(
                h,
                mb_x,
                mb_y,
                1 as libc::c_int,
                0 as libc::c_int,
                1 as libc::c_int,
            );
            macroblock_load_pic_pointers(
                h,
                mb_x,
                mb_y,
                2 as libc::c_int,
                0 as libc::c_int,
                1 as libc::c_int,
            );
        } else if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc != 0 {
            macroblock_load_pic_pointers(
                h,
                mb_x,
                mb_y,
                1 as libc::c_int,
                1 as libc::c_int,
                1 as libc::c_int,
            );
        }
    }
    if !((*(*h).fdec).integral).is_null() {
        let mut offset_0: libc::c_int = 16 as libc::c_int
            * (mb_x + mb_y * (*(*h).fdec).i_stride[0 as libc::c_int as usize]);
        let mut list: libc::c_int = 0 as libc::c_int;
        while list < 2 as libc::c_int {
            let mut i: libc::c_int = 0 as libc::c_int;
            while i < (*h).mb.pic.i_fref[list as usize] {
                (*h)
                    .mb
                    .pic
                    .p_integral[list
                    as usize][i
                    as usize] = &mut *((**(*((*h).fref)
                    .as_mut_ptr()
                    .offset(list as isize))
                    .as_mut_ptr()
                    .offset(i as isize))
                    .integral)
                    .offset(offset_0 as isize) as *mut uint16_t;
                i += 1;
                i;
            }
            list += 1;
            list;
        }
    }
    x264_8_prefetch_fenc(h, (*h).fenc, mb_x, mb_y);
    let mut l_0: libc::c_int = 0 as libc::c_int;
    while l_0 < lists {
        let mut mv: *mut [int16_t; 2] = (*h).mb.mv[l_0 as usize];
        let mut ref_0: *mut int8_t = (*h).mb.ref_0[l_0 as usize];
        let mut i8: libc::c_int = x264_scan8[0 as libc::c_int as usize] as libc::c_int
            - 1 as libc::c_int - 1 as libc::c_int * 8 as libc::c_int;
        if (*h).mb.i_neighbour & MB_TOPLEFT as libc::c_int as libc::c_uint != 0 {
            let mut ir: libc::c_int = if b_mbaff != 0 {
                2 as libc::c_int
                    * (s8x8 * (*h).mb.i_mb_topleft_y + mb_x - 1 as libc::c_int)
                    + 1 as libc::c_int + s8x8
            } else {
                top_8x8 - 1 as libc::c_int
            };
            let mut iv: libc::c_int = if b_mbaff != 0 {
                4 as libc::c_int
                    * (s4x4 * (*h).mb.i_mb_topleft_y + mb_x - 1 as libc::c_int)
                    + 3 as libc::c_int + 3 as libc::c_int * s4x4
            } else {
                top_4x4 - 1 as libc::c_int
            };
            if b_mbaff != 0 && (*h).mb.topleft_partition != 0 {
                iv -= 2 as libc::c_int * s4x4;
                ir -= s8x8;
            }
            (*h).mb.cache.ref_0[l_0 as usize][i8 as usize] = *ref_0.offset(ir as isize);
            (*(((*h).mb.cache.mv[l_0 as usize][i8 as usize]).as_mut_ptr()
                as *mut x264_union32_t))
                .i = (*((*mv.offset(iv as isize)).as_mut_ptr() as *mut x264_union32_t))
                .i;
        } else {
            (*h)
                .mb
                .cache
                .ref_0[l_0 as usize][i8 as usize] = -(2 as libc::c_int) as int8_t;
            (*(((*h).mb.cache.mv[l_0 as usize][i8 as usize]).as_mut_ptr()
                as *mut x264_union32_t))
                .i = 0 as libc::c_int as uint32_t;
        }
        i8 = x264_scan8[0 as libc::c_int as usize] as libc::c_int - 8 as libc::c_int;
        if (*h).mb.i_neighbour & MB_TOP as libc::c_int as libc::c_uint != 0 {
            (*h)
                .mb
                .cache
                .ref_0[l_0
                as usize][(i8 + 1 as libc::c_int)
                as usize] = *ref_0.offset((top_8x8 + 0 as libc::c_int) as isize);
            (*h)
                .mb
                .cache
                .ref_0[l_0
                as usize][(i8 + 0 as libc::c_int)
                as usize] = (*h)
                .mb
                .cache
                .ref_0[l_0 as usize][(i8 + 1 as libc::c_int) as usize];
            (*h)
                .mb
                .cache
                .ref_0[l_0
                as usize][(i8 + 3 as libc::c_int)
                as usize] = *ref_0.offset((top_8x8 + 1 as libc::c_int) as isize);
            (*h)
                .mb
                .cache
                .ref_0[l_0
                as usize][(i8 + 2 as libc::c_int)
                as usize] = (*h)
                .mb
                .cache
                .ref_0[l_0 as usize][(i8 + 3 as libc::c_int) as usize];
            (*(((*h).mb.cache.mv[l_0 as usize][i8 as usize]).as_mut_ptr()
                as *mut x264_union128_sse_t))
                .i = (*((*mv.offset(top_4x4 as isize)).as_mut_ptr()
                as *mut x264_union128_sse_t))
                .i;
        } else {
            (*(((*h).mb.cache.mv[l_0 as usize][i8 as usize]).as_mut_ptr()
                as *mut x264_union128_sse_t))
                .i = _mm_setr_ps(
                0 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
            (*(&mut *(*((*h).mb.cache.ref_0).as_mut_ptr().offset(l_0 as isize))
                .as_mut_ptr()
                .offset(i8 as isize) as *mut int8_t as *mut x264_union32_t))
                .i = (-(2 as libc::c_int) as uint8_t as libc::c_uint)
                .wrapping_mul(0x1010101 as libc::c_uint);
        }
        i8 = x264_scan8[0 as libc::c_int as usize] as libc::c_int + 4 as libc::c_int
            - 1 as libc::c_int * 8 as libc::c_int;
        if (*h).mb.i_neighbour & MB_TOPRIGHT as libc::c_int as libc::c_uint != 0 {
            let mut ir_0: libc::c_int = if b_mbaff != 0 {
                2 as libc::c_int
                    * (s8x8 * (*h).mb.i_mb_topright_y + (mb_x + 1 as libc::c_int)) + s8x8
            } else {
                top_8x8 + 2 as libc::c_int
            };
            let mut iv_0: libc::c_int = if b_mbaff != 0 {
                4 as libc::c_int
                    * (s4x4 * (*h).mb.i_mb_topright_y + (mb_x + 1 as libc::c_int))
                    + 3 as libc::c_int * s4x4
            } else {
                top_4x4 + 4 as libc::c_int
            };
            (*h)
                .mb
                .cache
                .ref_0[l_0 as usize][i8 as usize] = *ref_0.offset(ir_0 as isize);
            (*(((*h).mb.cache.mv[l_0 as usize][i8 as usize]).as_mut_ptr()
                as *mut x264_union32_t))
                .i = (*((*mv.offset(iv_0 as isize)).as_mut_ptr() as *mut x264_union32_t))
                .i;
        } else {
            (*h)
                .mb
                .cache
                .ref_0[l_0 as usize][i8 as usize] = -(2 as libc::c_int) as int8_t;
        }
        i8 = x264_scan8[0 as libc::c_int as usize] as libc::c_int - 1 as libc::c_int;
        if (*h).mb.i_neighbour & MB_LEFT as libc::c_int as libc::c_uint != 0 {
            if b_mbaff != 0 {
                (*h)
                    .mb
                    .cache
                    .ref_0[l_0
                    as usize][(i8 + 0 as libc::c_int * 8 as libc::c_int)
                    as usize] = *ref_0
                    .offset(
                        ((*h).mb.left_b8[0 as libc::c_int as usize] + 1 as libc::c_int
                            + s8x8
                                * (*left_index_table).ref_0[0 as libc::c_int as usize]
                                    as libc::c_int) as isize,
                    );
                (*h)
                    .mb
                    .cache
                    .ref_0[l_0
                    as usize][(i8 + 1 as libc::c_int * 8 as libc::c_int)
                    as usize] = *ref_0
                    .offset(
                        ((*h).mb.left_b8[0 as libc::c_int as usize] + 1 as libc::c_int
                            + s8x8
                                * (*left_index_table).ref_0[1 as libc::c_int as usize]
                                    as libc::c_int) as isize,
                    );
                (*h)
                    .mb
                    .cache
                    .ref_0[l_0
                    as usize][(i8 + 2 as libc::c_int * 8 as libc::c_int)
                    as usize] = *ref_0
                    .offset(
                        ((*h).mb.left_b8[1 as libc::c_int as usize] + 1 as libc::c_int
                            + s8x8
                                * (*left_index_table).ref_0[2 as libc::c_int as usize]
                                    as libc::c_int) as isize,
                    );
                (*h)
                    .mb
                    .cache
                    .ref_0[l_0
                    as usize][(i8 + 3 as libc::c_int * 8 as libc::c_int)
                    as usize] = *ref_0
                    .offset(
                        ((*h).mb.left_b8[1 as libc::c_int as usize] + 1 as libc::c_int
                            + s8x8
                                * (*left_index_table).ref_0[3 as libc::c_int as usize]
                                    as libc::c_int) as isize,
                    );
                (*(((*h)
                    .mb
                    .cache
                    .mv[l_0
                    as usize][(i8 + 0 as libc::c_int * 8 as libc::c_int) as usize])
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i = (*((*mv
                    .offset(
                        ((*h).mb.left_b4[0 as libc::c_int as usize] + 3 as libc::c_int
                            + s4x4
                                * (*left_index_table).mv[0 as libc::c_int as usize]
                                    as libc::c_int) as isize,
                    ))
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i;
                (*(((*h)
                    .mb
                    .cache
                    .mv[l_0
                    as usize][(i8 + 1 as libc::c_int * 8 as libc::c_int) as usize])
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i = (*((*mv
                    .offset(
                        ((*h).mb.left_b4[0 as libc::c_int as usize] + 3 as libc::c_int
                            + s4x4
                                * (*left_index_table).mv[1 as libc::c_int as usize]
                                    as libc::c_int) as isize,
                    ))
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i;
                (*(((*h)
                    .mb
                    .cache
                    .mv[l_0
                    as usize][(i8 + 2 as libc::c_int * 8 as libc::c_int) as usize])
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i = (*((*mv
                    .offset(
                        ((*h).mb.left_b4[1 as libc::c_int as usize] + 3 as libc::c_int
                            + s4x4
                                * (*left_index_table).mv[2 as libc::c_int as usize]
                                    as libc::c_int) as isize,
                    ))
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i;
                (*(((*h)
                    .mb
                    .cache
                    .mv[l_0
                    as usize][(i8 + 3 as libc::c_int * 8 as libc::c_int) as usize])
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i = (*((*mv
                    .offset(
                        ((*h).mb.left_b4[1 as libc::c_int as usize] + 3 as libc::c_int
                            + s4x4
                                * (*left_index_table).mv[3 as libc::c_int as usize]
                                    as libc::c_int) as isize,
                    ))
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i;
            } else {
                let ir_1: libc::c_int = (*h).mb.i_b8_xy - 1 as libc::c_int;
                let iv_1: libc::c_int = (*h).mb.i_b4_xy - 1 as libc::c_int;
                (*h)
                    .mb
                    .cache
                    .ref_0[l_0
                    as usize][(i8 + 1 as libc::c_int * 8 as libc::c_int)
                    as usize] = *ref_0.offset((ir_1 + 0 as libc::c_int * s8x8) as isize);
                (*h)
                    .mb
                    .cache
                    .ref_0[l_0
                    as usize][(i8 + 0 as libc::c_int * 8 as libc::c_int)
                    as usize] = (*h)
                    .mb
                    .cache
                    .ref_0[l_0
                    as usize][(i8 + 1 as libc::c_int * 8 as libc::c_int) as usize];
                (*h)
                    .mb
                    .cache
                    .ref_0[l_0
                    as usize][(i8 + 3 as libc::c_int * 8 as libc::c_int)
                    as usize] = *ref_0.offset((ir_1 + 1 as libc::c_int * s8x8) as isize);
                (*h)
                    .mb
                    .cache
                    .ref_0[l_0
                    as usize][(i8 + 2 as libc::c_int * 8 as libc::c_int)
                    as usize] = (*h)
                    .mb
                    .cache
                    .ref_0[l_0
                    as usize][(i8 + 3 as libc::c_int * 8 as libc::c_int) as usize];
                (*(((*h)
                    .mb
                    .cache
                    .mv[l_0
                    as usize][(i8 + 0 as libc::c_int * 8 as libc::c_int) as usize])
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i = (*((*mv.offset((iv_1 + 0 as libc::c_int * s4x4) as isize))
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i;
                (*(((*h)
                    .mb
                    .cache
                    .mv[l_0
                    as usize][(i8 + 1 as libc::c_int * 8 as libc::c_int) as usize])
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i = (*((*mv.offset((iv_1 + 1 as libc::c_int * s4x4) as isize))
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i;
                (*(((*h)
                    .mb
                    .cache
                    .mv[l_0
                    as usize][(i8 + 2 as libc::c_int * 8 as libc::c_int) as usize])
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i = (*((*mv.offset((iv_1 + 2 as libc::c_int * s4x4) as isize))
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i;
                (*(((*h)
                    .mb
                    .cache
                    .mv[l_0
                    as usize][(i8 + 3 as libc::c_int * 8 as libc::c_int) as usize])
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i = (*((*mv.offset((iv_1 + 3 as libc::c_int * s4x4) as isize))
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i;
            }
        } else {
            let mut i_0: libc::c_int = 0 as libc::c_int;
            while i_0 < 4 as libc::c_int {
                (*h)
                    .mb
                    .cache
                    .ref_0[l_0
                    as usize][(i8 + i_0 * 8 as libc::c_int)
                    as usize] = -(2 as libc::c_int) as int8_t;
                (*(((*h)
                    .mb
                    .cache
                    .mv[l_0 as usize][(i8 + i_0 * 8 as libc::c_int) as usize])
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i = 0 as libc::c_int as uint32_t;
                i_0 += 1;
                i_0;
            }
        }
        if b_mbaff != 0
            && (*h).mb.i_neighbour & MB_LEFT as libc::c_int as libc::c_uint != 0
        {
            if (*h).mb.b_interlaced != 0
                && *((*h).mb.field).offset(((*h).mb.i_mb_xy - 1 as libc::c_int) as isize)
                    == 0
            {
                (*h)
                    .mb
                    .cache
                    .topright_ref[l_0
                    as usize][0 as libc::c_int
                    as usize] = *ref_0
                    .offset(
                        ((*h).mb.left_b8[0 as libc::c_int as usize] + 1 as libc::c_int
                            + s8x8 * 0 as libc::c_int) as isize,
                    );
                (*h)
                    .mb
                    .cache
                    .topright_ref[l_0
                    as usize][1 as libc::c_int
                    as usize] = *ref_0
                    .offset(
                        ((*h).mb.left_b8[0 as libc::c_int as usize] + 1 as libc::c_int
                            + s8x8 * 1 as libc::c_int) as isize,
                    );
                (*h)
                    .mb
                    .cache
                    .topright_ref[l_0
                    as usize][2 as libc::c_int
                    as usize] = *ref_0
                    .offset(
                        ((*h).mb.left_b8[1 as libc::c_int as usize] + 1 as libc::c_int
                            + s8x8 * 0 as libc::c_int) as isize,
                    );
                (*(((*h).mb.cache.topright_mv[l_0 as usize][0 as libc::c_int as usize])
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i = (*((*mv
                    .offset(
                        ((*h).mb.left_b4[0 as libc::c_int as usize] + 3 as libc::c_int
                            + s4x4
                                * ((*left_index_table).mv[0 as libc::c_int as usize]
                                    as libc::c_int + 1 as libc::c_int)) as isize,
                    ))
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i;
                (*(((*h).mb.cache.topright_mv[l_0 as usize][1 as libc::c_int as usize])
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i = (*((*mv
                    .offset(
                        ((*h).mb.left_b4[0 as libc::c_int as usize] + 3 as libc::c_int
                            + s4x4
                                * ((*left_index_table).mv[1 as libc::c_int as usize]
                                    as libc::c_int + 1 as libc::c_int)) as isize,
                    ))
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i;
                (*(((*h).mb.cache.topright_mv[l_0 as usize][2 as libc::c_int as usize])
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i = (*((*mv
                    .offset(
                        ((*h).mb.left_b4[1 as libc::c_int as usize] + 3 as libc::c_int
                            + s4x4
                                * ((*left_index_table).mv[2 as libc::c_int as usize]
                                    as libc::c_int + 1 as libc::c_int)) as isize,
                    ))
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i;
            } else if (*h).mb.b_interlaced == 0
                && *((*h).mb.field).offset(((*h).mb.i_mb_xy - 1 as libc::c_int) as isize)
                    as libc::c_int != 0
            {
                (*h)
                    .mb
                    .cache
                    .topright_ref[l_0
                    as usize][0 as libc::c_int
                    as usize] = *ref_0
                    .offset(
                        ((*h).mb.left_b8[0 as libc::c_int as usize] + 1 as libc::c_int
                            + s8x8 * 2 as libc::c_int
                            + s8x8
                                * (*left_index_table).ref_0[0 as libc::c_int as usize]
                                    as libc::c_int) as isize,
                    );
                (*h)
                    .mb
                    .cache
                    .topright_ref[l_0
                    as usize][1 as libc::c_int
                    as usize] = *ref_0
                    .offset(
                        ((*h).mb.left_b8[0 as libc::c_int as usize] + 1 as libc::c_int
                            + s8x8 * 2 as libc::c_int
                            + s8x8
                                * (*left_index_table).ref_0[1 as libc::c_int as usize]
                                    as libc::c_int) as isize,
                    );
                (*h)
                    .mb
                    .cache
                    .topright_ref[l_0
                    as usize][2 as libc::c_int
                    as usize] = *ref_0
                    .offset(
                        ((*h).mb.left_b8[0 as libc::c_int as usize] + 1 as libc::c_int
                            + s8x8 * 2 as libc::c_int
                            + s8x8
                                * (*left_index_table).ref_0[2 as libc::c_int as usize]
                                    as libc::c_int) as isize,
                    );
                (*(((*h).mb.cache.topright_mv[l_0 as usize][0 as libc::c_int as usize])
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i = (*((*mv
                    .offset(
                        ((*h).mb.left_b4[0 as libc::c_int as usize] + 3 as libc::c_int
                            + s4x4 * 4 as libc::c_int
                            + s4x4
                                * (*left_index_table).mv[0 as libc::c_int as usize]
                                    as libc::c_int) as isize,
                    ))
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i;
                (*(((*h).mb.cache.topright_mv[l_0 as usize][1 as libc::c_int as usize])
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i = (*((*mv
                    .offset(
                        ((*h).mb.left_b4[0 as libc::c_int as usize] + 3 as libc::c_int
                            + s4x4 * 4 as libc::c_int
                            + s4x4
                                * (*left_index_table).mv[1 as libc::c_int as usize]
                                    as libc::c_int) as isize,
                    ))
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i;
                (*(((*h).mb.cache.topright_mv[l_0 as usize][2 as libc::c_int as usize])
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i = (*((*mv
                    .offset(
                        ((*h).mb.left_b4[0 as libc::c_int as usize] + 3 as libc::c_int
                            + s4x4 * 4 as libc::c_int
                            + s4x4
                                * (*left_index_table).mv[2 as libc::c_int as usize]
                                    as libc::c_int) as isize,
                    ))
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i;
            }
        }
        if (*h).param.b_cabac != 0 {
            let mut mvd: *mut [[uint8_t; 2]; 8] = (*h).mb.mvd[l_0 as usize];
            if (*h).mb.i_neighbour & MB_TOP as libc::c_int as libc::c_uint != 0 {
                (*(((*h)
                    .mb
                    .cache
                    .mvd[l_0
                    as usize][(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                    - 8 as libc::c_int) as usize])
                    .as_mut_ptr() as *mut x264_union64_t))
                    .i = (*(((*mvd.offset(top as isize))[0 as libc::c_int as usize])
                    .as_mut_ptr() as *mut x264_union64_t))
                    .i;
            } else {
                (*(((*h)
                    .mb
                    .cache
                    .mvd[l_0
                    as usize][(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                    - 8 as libc::c_int) as usize])
                    .as_mut_ptr() as *mut x264_union64_t))
                    .i = 0 as libc::c_int as uint64_t;
            }
            if (*h).mb.i_neighbour & MB_LEFT as libc::c_int as libc::c_uint != 0
                && (b_mbaff == 0
                    || (*h)
                        .mb
                        .cache
                        .ref_0[l_0
                        as usize][(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                        - 1 as libc::c_int) as usize] as libc::c_int >= 0 as libc::c_int)
            {
                (*(((*h)
                    .mb
                    .cache
                    .mvd[l_0
                    as usize][(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                    - 1 as libc::c_int) as usize])
                    .as_mut_ptr() as *mut x264_union16_t))
                    .i = (*(((*mvd
                    .offset(
                        *left.offset(0 as libc::c_int as isize) as isize,
                    ))[(*left_index_table).intra[0 as libc::c_int as usize] as usize])
                    .as_mut_ptr() as *mut x264_union16_t))
                    .i;
                (*(((*h)
                    .mb
                    .cache
                    .mvd[l_0
                    as usize][(x264_scan8[2 as libc::c_int as usize] as libc::c_int
                    - 1 as libc::c_int) as usize])
                    .as_mut_ptr() as *mut x264_union16_t))
                    .i = (*(((*mvd
                    .offset(
                        *left.offset(0 as libc::c_int as isize) as isize,
                    ))[(*left_index_table).intra[1 as libc::c_int as usize] as usize])
                    .as_mut_ptr() as *mut x264_union16_t))
                    .i;
            } else {
                (*(((*h)
                    .mb
                    .cache
                    .mvd[l_0
                    as usize][(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                    - 1 as libc::c_int + 0 as libc::c_int * 8 as libc::c_int) as usize])
                    .as_mut_ptr() as *mut x264_union16_t))
                    .i = 0 as libc::c_int as uint16_t;
                (*(((*h)
                    .mb
                    .cache
                    .mvd[l_0
                    as usize][(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                    - 1 as libc::c_int + 1 as libc::c_int * 8 as libc::c_int) as usize])
                    .as_mut_ptr() as *mut x264_union16_t))
                    .i = 0 as libc::c_int as uint16_t;
            }
            if (*h).mb.i_neighbour & MB_LEFT as libc::c_int as libc::c_uint != 0
                && (b_mbaff == 0
                    || (*h)
                        .mb
                        .cache
                        .ref_0[l_0
                        as usize][(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                        - 1 as libc::c_int + 2 as libc::c_int * 8 as libc::c_int)
                        as usize] as libc::c_int >= 0 as libc::c_int)
            {
                (*(((*h)
                    .mb
                    .cache
                    .mvd[l_0
                    as usize][(x264_scan8[8 as libc::c_int as usize] as libc::c_int
                    - 1 as libc::c_int) as usize])
                    .as_mut_ptr() as *mut x264_union16_t))
                    .i = (*(((*mvd
                    .offset(
                        *left.offset(1 as libc::c_int as isize) as isize,
                    ))[(*left_index_table).intra[2 as libc::c_int as usize] as usize])
                    .as_mut_ptr() as *mut x264_union16_t))
                    .i;
                (*(((*h)
                    .mb
                    .cache
                    .mvd[l_0
                    as usize][(x264_scan8[10 as libc::c_int as usize] as libc::c_int
                    - 1 as libc::c_int) as usize])
                    .as_mut_ptr() as *mut x264_union16_t))
                    .i = (*(((*mvd
                    .offset(
                        *left.offset(1 as libc::c_int as isize) as isize,
                    ))[(*left_index_table).intra[3 as libc::c_int as usize] as usize])
                    .as_mut_ptr() as *mut x264_union16_t))
                    .i;
            } else {
                (*(((*h)
                    .mb
                    .cache
                    .mvd[l_0
                    as usize][(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                    - 1 as libc::c_int + 2 as libc::c_int * 8 as libc::c_int) as usize])
                    .as_mut_ptr() as *mut x264_union16_t))
                    .i = 0 as libc::c_int as uint16_t;
                (*(((*h)
                    .mb
                    .cache
                    .mvd[l_0
                    as usize][(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                    - 1 as libc::c_int + 3 as libc::c_int * 8 as libc::c_int) as usize])
                    .as_mut_ptr() as *mut x264_union16_t))
                    .i = 0 as libc::c_int as uint16_t;
            }
        }
        if b_mbaff != 0 {
            if (*h).mb.b_interlaced != 0 {
                if (*h).mb.i_mb_topleft_xy >= 0 as libc::c_int && *((*h).mb.field).offset((*h).mb.i_mb_topleft_xy as isize) == 0 && (*h)
                        .mb
                        .cache
                        .ref_0[l_0
                        as usize][(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                        - 1 as libc::c_int - 1 as libc::c_int * 8 as libc::c_int)
                        as usize] as libc::c_int >= 0 as libc::c_int {
                    (*h)
                        .mb
                        .cache
                        .ref_0[l_0
                        as usize][(x264_scan8[0 as libc::c_int as usize]
                        as libc::c_int - 1 as libc::c_int
                        - 1 as libc::c_int * 8 as libc::c_int)
                        as usize] = (((*h)
                        .mb
                        .cache
                        .ref_0[l_0
                        as usize][(x264_scan8[0 as libc::c_int as usize]
                        as libc::c_int - 1 as libc::c_int
                        - 1 as libc::c_int * 8 as libc::c_int) as usize]
                        as libc::c_int) << 1 as libc::c_int) as int8_t;
                    (*h)
                        .mb
                        .cache
                        .mv[l_0
                        as usize][(x264_scan8[0 as libc::c_int as usize]
                        as libc::c_int - 1 as libc::c_int
                        - 1 as libc::c_int * 8 as libc::c_int)
                        as usize][1 as libc::c_int
                        as usize] = ((*h)
                        .mb
                        .cache
                        .mv[l_0
                        as usize][(x264_scan8[0 as libc::c_int as usize]
                        as libc::c_int - 1 as libc::c_int
                        - 1 as libc::c_int * 8 as libc::c_int)
                        as usize][1 as libc::c_int as usize] as libc::c_int
                        / 2 as libc::c_int) as int16_t;
                    (*h)
                        .mb
                        .cache
                        .mvd[l_0
                        as usize][(x264_scan8[0 as libc::c_int as usize]
                        as libc::c_int - 1 as libc::c_int
                        - 1 as libc::c_int * 8 as libc::c_int)
                        as usize][1 as libc::c_int
                        as usize] = ((*h)
                        .mb
                        .cache
                        .mvd[l_0
                        as usize][(x264_scan8[0 as libc::c_int as usize]
                        as libc::c_int - 1 as libc::c_int
                        - 1 as libc::c_int * 8 as libc::c_int)
                        as usize][1 as libc::c_int as usize] as libc::c_int
                        >> 1 as libc::c_int) as uint8_t;
                }
                if top >= 0 as libc::c_int && *((*h).mb.field).offset(top as isize) == 0
                {
                    if (*h)
                        .mb
                        .cache
                        .ref_0[l_0
                        as usize][(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                        + 0 as libc::c_int - 1 as libc::c_int * 8 as libc::c_int)
                        as usize] as libc::c_int >= 0 as libc::c_int
                    {
                        (*h)
                            .mb
                            .cache
                            .ref_0[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int + 0 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize] = (((*h)
                            .mb
                            .cache
                            .ref_0[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int + 0 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int) as usize]
                            as libc::c_int) << 1 as libc::c_int) as int8_t;
                        (*h)
                            .mb
                            .cache
                            .mv[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int + 0 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int
                            as usize] = ((*h)
                            .mb
                            .cache
                            .mv[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int + 0 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] as libc::c_int
                            / 2 as libc::c_int) as int16_t;
                        (*h)
                            .mb
                            .cache
                            .mvd[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int + 0 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int
                            as usize] = ((*h)
                            .mb
                            .cache
                            .mvd[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int + 0 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] as libc::c_int
                            >> 1 as libc::c_int) as uint8_t;
                    }
                    if (*h)
                        .mb
                        .cache
                        .ref_0[l_0
                        as usize][(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                        + 1 as libc::c_int - 1 as libc::c_int * 8 as libc::c_int)
                        as usize] as libc::c_int >= 0 as libc::c_int
                    {
                        (*h)
                            .mb
                            .cache
                            .ref_0[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int + 1 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize] = (((*h)
                            .mb
                            .cache
                            .ref_0[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int + 1 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int) as usize]
                            as libc::c_int) << 1 as libc::c_int) as int8_t;
                        (*h)
                            .mb
                            .cache
                            .mv[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int + 1 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int
                            as usize] = ((*h)
                            .mb
                            .cache
                            .mv[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int + 1 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] as libc::c_int
                            / 2 as libc::c_int) as int16_t;
                        (*h)
                            .mb
                            .cache
                            .mvd[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int + 1 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int
                            as usize] = ((*h)
                            .mb
                            .cache
                            .mvd[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int + 1 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] as libc::c_int
                            >> 1 as libc::c_int) as uint8_t;
                    }
                    if (*h)
                        .mb
                        .cache
                        .ref_0[l_0
                        as usize][(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                        + 2 as libc::c_int - 1 as libc::c_int * 8 as libc::c_int)
                        as usize] as libc::c_int >= 0 as libc::c_int
                    {
                        (*h)
                            .mb
                            .cache
                            .ref_0[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int + 2 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize] = (((*h)
                            .mb
                            .cache
                            .ref_0[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int + 2 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int) as usize]
                            as libc::c_int) << 1 as libc::c_int) as int8_t;
                        (*h)
                            .mb
                            .cache
                            .mv[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int + 2 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int
                            as usize] = ((*h)
                            .mb
                            .cache
                            .mv[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int + 2 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] as libc::c_int
                            / 2 as libc::c_int) as int16_t;
                        (*h)
                            .mb
                            .cache
                            .mvd[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int + 2 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int
                            as usize] = ((*h)
                            .mb
                            .cache
                            .mvd[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int + 2 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] as libc::c_int
                            >> 1 as libc::c_int) as uint8_t;
                    }
                    if (*h)
                        .mb
                        .cache
                        .ref_0[l_0
                        as usize][(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                        + 3 as libc::c_int - 1 as libc::c_int * 8 as libc::c_int)
                        as usize] as libc::c_int >= 0 as libc::c_int
                    {
                        (*h)
                            .mb
                            .cache
                            .ref_0[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int + 3 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize] = (((*h)
                            .mb
                            .cache
                            .ref_0[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int + 3 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int) as usize]
                            as libc::c_int) << 1 as libc::c_int) as int8_t;
                        (*h)
                            .mb
                            .cache
                            .mv[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int + 3 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int
                            as usize] = ((*h)
                            .mb
                            .cache
                            .mv[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int + 3 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] as libc::c_int
                            / 2 as libc::c_int) as int16_t;
                        (*h)
                            .mb
                            .cache
                            .mvd[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int + 3 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int
                            as usize] = ((*h)
                            .mb
                            .cache
                            .mvd[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int + 3 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] as libc::c_int
                            >> 1 as libc::c_int) as uint8_t;
                    }
                }
                if (*h).mb.i_mb_topright_xy >= 0 as libc::c_int && *((*h).mb.field).offset((*h).mb.i_mb_topright_xy as isize) == 0 && (*h)
                        .mb
                        .cache
                        .ref_0[l_0
                        as usize][(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                        + 4 as libc::c_int - 1 as libc::c_int * 8 as libc::c_int)
                        as usize] as libc::c_int >= 0 as libc::c_int {
                    (*h)
                        .mb
                        .cache
                        .ref_0[l_0
                        as usize][(x264_scan8[0 as libc::c_int as usize]
                        as libc::c_int + 4 as libc::c_int
                        - 1 as libc::c_int * 8 as libc::c_int)
                        as usize] = (((*h)
                        .mb
                        .cache
                        .ref_0[l_0
                        as usize][(x264_scan8[0 as libc::c_int as usize]
                        as libc::c_int + 4 as libc::c_int
                        - 1 as libc::c_int * 8 as libc::c_int) as usize]
                        as libc::c_int) << 1 as libc::c_int) as int8_t;
                    (*h)
                        .mb
                        .cache
                        .mv[l_0
                        as usize][(x264_scan8[0 as libc::c_int as usize]
                        as libc::c_int + 4 as libc::c_int
                        - 1 as libc::c_int * 8 as libc::c_int)
                        as usize][1 as libc::c_int
                        as usize] = ((*h)
                        .mb
                        .cache
                        .mv[l_0
                        as usize][(x264_scan8[0 as libc::c_int as usize]
                        as libc::c_int + 4 as libc::c_int
                        - 1 as libc::c_int * 8 as libc::c_int)
                        as usize][1 as libc::c_int as usize] as libc::c_int
                        / 2 as libc::c_int) as int16_t;
                    (*h)
                        .mb
                        .cache
                        .mvd[l_0
                        as usize][(x264_scan8[0 as libc::c_int as usize]
                        as libc::c_int + 4 as libc::c_int
                        - 1 as libc::c_int * 8 as libc::c_int)
                        as usize][1 as libc::c_int
                        as usize] = ((*h)
                        .mb
                        .cache
                        .mvd[l_0
                        as usize][(x264_scan8[0 as libc::c_int as usize]
                        as libc::c_int + 4 as libc::c_int
                        - 1 as libc::c_int * 8 as libc::c_int)
                        as usize][1 as libc::c_int as usize] as libc::c_int
                        >> 1 as libc::c_int) as uint8_t;
                }
                if *left.offset(0 as libc::c_int as isize) >= 0 as libc::c_int
                    && *((*h).mb.field)
                        .offset(*left.offset(0 as libc::c_int as isize) as isize) == 0
                {
                    if (*h)
                        .mb
                        .cache
                        .ref_0[l_0
                        as usize][(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                        - 1 as libc::c_int + 0 as libc::c_int * 8 as libc::c_int)
                        as usize] as libc::c_int >= 0 as libc::c_int
                    {
                        (*h)
                            .mb
                            .cache
                            .ref_0[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int - 1 as libc::c_int
                            + 0 as libc::c_int * 8 as libc::c_int)
                            as usize] = (((*h)
                            .mb
                            .cache
                            .ref_0[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int - 1 as libc::c_int
                            + 0 as libc::c_int * 8 as libc::c_int) as usize]
                            as libc::c_int) << 1 as libc::c_int) as int8_t;
                        (*h)
                            .mb
                            .cache
                            .mv[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int - 1 as libc::c_int
                            + 0 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int
                            as usize] = ((*h)
                            .mb
                            .cache
                            .mv[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int - 1 as libc::c_int
                            + 0 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] as libc::c_int
                            / 2 as libc::c_int) as int16_t;
                        (*h)
                            .mb
                            .cache
                            .mvd[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int - 1 as libc::c_int
                            + 0 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int
                            as usize] = ((*h)
                            .mb
                            .cache
                            .mvd[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int - 1 as libc::c_int
                            + 0 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] as libc::c_int
                            >> 1 as libc::c_int) as uint8_t;
                    }
                    if (*h)
                        .mb
                        .cache
                        .ref_0[l_0
                        as usize][(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                        - 1 as libc::c_int + 1 as libc::c_int * 8 as libc::c_int)
                        as usize] as libc::c_int >= 0 as libc::c_int
                    {
                        (*h)
                            .mb
                            .cache
                            .ref_0[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int - 1 as libc::c_int
                            + 1 as libc::c_int * 8 as libc::c_int)
                            as usize] = (((*h)
                            .mb
                            .cache
                            .ref_0[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int - 1 as libc::c_int
                            + 1 as libc::c_int * 8 as libc::c_int) as usize]
                            as libc::c_int) << 1 as libc::c_int) as int8_t;
                        (*h)
                            .mb
                            .cache
                            .mv[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int - 1 as libc::c_int
                            + 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int
                            as usize] = ((*h)
                            .mb
                            .cache
                            .mv[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int - 1 as libc::c_int
                            + 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] as libc::c_int
                            / 2 as libc::c_int) as int16_t;
                        (*h)
                            .mb
                            .cache
                            .mvd[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int - 1 as libc::c_int
                            + 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int
                            as usize] = ((*h)
                            .mb
                            .cache
                            .mvd[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int - 1 as libc::c_int
                            + 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] as libc::c_int
                            >> 1 as libc::c_int) as uint8_t;
                    }
                    if (*h)
                        .mb
                        .cache
                        .ref_0[l_0
                        as usize][(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                        - 1 as libc::c_int + 2 as libc::c_int * 8 as libc::c_int)
                        as usize] as libc::c_int >= 0 as libc::c_int
                    {
                        (*h)
                            .mb
                            .cache
                            .ref_0[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int - 1 as libc::c_int
                            + 2 as libc::c_int * 8 as libc::c_int)
                            as usize] = (((*h)
                            .mb
                            .cache
                            .ref_0[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int - 1 as libc::c_int
                            + 2 as libc::c_int * 8 as libc::c_int) as usize]
                            as libc::c_int) << 1 as libc::c_int) as int8_t;
                        (*h)
                            .mb
                            .cache
                            .mv[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int - 1 as libc::c_int
                            + 2 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int
                            as usize] = ((*h)
                            .mb
                            .cache
                            .mv[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int - 1 as libc::c_int
                            + 2 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] as libc::c_int
                            / 2 as libc::c_int) as int16_t;
                        (*h)
                            .mb
                            .cache
                            .mvd[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int - 1 as libc::c_int
                            + 2 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int
                            as usize] = ((*h)
                            .mb
                            .cache
                            .mvd[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int - 1 as libc::c_int
                            + 2 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] as libc::c_int
                            >> 1 as libc::c_int) as uint8_t;
                    }
                    if (*h)
                        .mb
                        .cache
                        .ref_0[l_0
                        as usize][(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                        - 1 as libc::c_int + 3 as libc::c_int * 8 as libc::c_int)
                        as usize] as libc::c_int >= 0 as libc::c_int
                    {
                        (*h)
                            .mb
                            .cache
                            .ref_0[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int - 1 as libc::c_int
                            + 3 as libc::c_int * 8 as libc::c_int)
                            as usize] = (((*h)
                            .mb
                            .cache
                            .ref_0[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int - 1 as libc::c_int
                            + 3 as libc::c_int * 8 as libc::c_int) as usize]
                            as libc::c_int) << 1 as libc::c_int) as int8_t;
                        (*h)
                            .mb
                            .cache
                            .mv[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int - 1 as libc::c_int
                            + 3 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int
                            as usize] = ((*h)
                            .mb
                            .cache
                            .mv[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int - 1 as libc::c_int
                            + 3 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] as libc::c_int
                            / 2 as libc::c_int) as int16_t;
                        (*h)
                            .mb
                            .cache
                            .mvd[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int - 1 as libc::c_int
                            + 3 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int
                            as usize] = ((*h)
                            .mb
                            .cache
                            .mvd[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int - 1 as libc::c_int
                            + 3 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] as libc::c_int
                            >> 1 as libc::c_int) as uint8_t;
                    }
                    if (*h)
                        .mb
                        .cache
                        .topright_ref[l_0 as usize][0 as libc::c_int as usize]
                        as libc::c_int >= 0 as libc::c_int
                    {
                        (*h)
                            .mb
                            .cache
                            .topright_ref[l_0
                            as usize][0 as libc::c_int
                            as usize] = (((*h)
                            .mb
                            .cache
                            .topright_ref[l_0 as usize][0 as libc::c_int as usize]
                            as libc::c_int) << 1 as libc::c_int) as int8_t;
                        (*h)
                            .mb
                            .cache
                            .topright_mv[l_0
                            as usize][0 as libc::c_int
                            as usize][1 as libc::c_int
                            as usize] = ((*h)
                            .mb
                            .cache
                            .topright_mv[l_0
                            as usize][0 as libc::c_int
                            as usize][1 as libc::c_int as usize] as libc::c_int
                            / 2 as libc::c_int) as int16_t;
                        (*h)
                            .mb
                            .cache
                            .mvd[l_0
                            as usize][0 as libc::c_int
                            as usize][1 as libc::c_int
                            as usize] = ((*h)
                            .mb
                            .cache
                            .mvd[l_0
                            as usize][0 as libc::c_int
                            as usize][1 as libc::c_int as usize] as libc::c_int
                            >> 1 as libc::c_int) as uint8_t;
                    }
                    if (*h)
                        .mb
                        .cache
                        .topright_ref[l_0 as usize][1 as libc::c_int as usize]
                        as libc::c_int >= 0 as libc::c_int
                    {
                        (*h)
                            .mb
                            .cache
                            .topright_ref[l_0
                            as usize][1 as libc::c_int
                            as usize] = (((*h)
                            .mb
                            .cache
                            .topright_ref[l_0 as usize][1 as libc::c_int as usize]
                            as libc::c_int) << 1 as libc::c_int) as int8_t;
                        (*h)
                            .mb
                            .cache
                            .topright_mv[l_0
                            as usize][1 as libc::c_int
                            as usize][1 as libc::c_int
                            as usize] = ((*h)
                            .mb
                            .cache
                            .topright_mv[l_0
                            as usize][1 as libc::c_int
                            as usize][1 as libc::c_int as usize] as libc::c_int
                            / 2 as libc::c_int) as int16_t;
                        (*h)
                            .mb
                            .cache
                            .mvd[l_0
                            as usize][1 as libc::c_int
                            as usize][1 as libc::c_int
                            as usize] = ((*h)
                            .mb
                            .cache
                            .mvd[l_0
                            as usize][1 as libc::c_int
                            as usize][1 as libc::c_int as usize] as libc::c_int
                            >> 1 as libc::c_int) as uint8_t;
                    }
                    if (*h)
                        .mb
                        .cache
                        .topright_ref[l_0 as usize][2 as libc::c_int as usize]
                        as libc::c_int >= 0 as libc::c_int
                    {
                        (*h)
                            .mb
                            .cache
                            .topright_ref[l_0
                            as usize][2 as libc::c_int
                            as usize] = (((*h)
                            .mb
                            .cache
                            .topright_ref[l_0 as usize][2 as libc::c_int as usize]
                            as libc::c_int) << 1 as libc::c_int) as int8_t;
                        (*h)
                            .mb
                            .cache
                            .topright_mv[l_0
                            as usize][2 as libc::c_int
                            as usize][1 as libc::c_int
                            as usize] = ((*h)
                            .mb
                            .cache
                            .topright_mv[l_0
                            as usize][2 as libc::c_int
                            as usize][1 as libc::c_int as usize] as libc::c_int
                            / 2 as libc::c_int) as int16_t;
                        (*h)
                            .mb
                            .cache
                            .mvd[l_0
                            as usize][2 as libc::c_int
                            as usize][1 as libc::c_int
                            as usize] = ((*h)
                            .mb
                            .cache
                            .mvd[l_0
                            as usize][2 as libc::c_int
                            as usize][1 as libc::c_int as usize] as libc::c_int
                            >> 1 as libc::c_int) as uint8_t;
                    }
                }
            } else {
                if (*h).mb.i_mb_topleft_xy >= 0 as libc::c_int && *((*h).mb.field).offset((*h).mb.i_mb_topleft_xy as isize)
                        as libc::c_int != 0 && (*h)
                        .mb
                        .cache
                        .ref_0[l_0
                        as usize][(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                        - 1 as libc::c_int - 1 as libc::c_int * 8 as libc::c_int)
                        as usize] as libc::c_int >= 0 as libc::c_int {
                    (*h)
                        .mb
                        .cache
                        .ref_0[l_0
                        as usize][(x264_scan8[0 as libc::c_int as usize]
                        as libc::c_int - 1 as libc::c_int
                        - 1 as libc::c_int * 8 as libc::c_int)
                        as usize] = ((*h)
                        .mb
                        .cache
                        .ref_0[l_0
                        as usize][(x264_scan8[0 as libc::c_int as usize]
                        as libc::c_int - 1 as libc::c_int
                        - 1 as libc::c_int * 8 as libc::c_int) as usize]
                        as libc::c_int >> 1 as libc::c_int) as int8_t;
                    (*h)
                        .mb
                        .cache
                        .mv[l_0
                        as usize][(x264_scan8[0 as libc::c_int as usize]
                        as libc::c_int - 1 as libc::c_int
                        - 1 as libc::c_int * 8 as libc::c_int)
                        as usize][1 as libc::c_int
                        as usize] = ((*h)
                        .mb
                        .cache
                        .mv[l_0
                        as usize][(x264_scan8[0 as libc::c_int as usize]
                        as libc::c_int - 1 as libc::c_int
                        - 1 as libc::c_int * 8 as libc::c_int)
                        as usize][1 as libc::c_int as usize] as libc::c_int
                        * 2 as libc::c_int) as int16_t;
                    (*h)
                        .mb
                        .cache
                        .mvd[l_0
                        as usize][(x264_scan8[0 as libc::c_int as usize]
                        as libc::c_int - 1 as libc::c_int
                        - 1 as libc::c_int * 8 as libc::c_int)
                        as usize][1 as libc::c_int
                        as usize] = (((*h)
                        .mb
                        .cache
                        .mvd[l_0
                        as usize][(x264_scan8[0 as libc::c_int as usize]
                        as libc::c_int - 1 as libc::c_int
                        - 1 as libc::c_int * 8 as libc::c_int)
                        as usize][1 as libc::c_int as usize] as libc::c_int)
                        << 1 as libc::c_int) as uint8_t;
                }
                if top >= 0 as libc::c_int
                    && *((*h).mb.field).offset(top as isize) as libc::c_int != 0
                {
                    if (*h)
                        .mb
                        .cache
                        .ref_0[l_0
                        as usize][(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                        + 0 as libc::c_int - 1 as libc::c_int * 8 as libc::c_int)
                        as usize] as libc::c_int >= 0 as libc::c_int
                    {
                        (*h)
                            .mb
                            .cache
                            .ref_0[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int + 0 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize] = ((*h)
                            .mb
                            .cache
                            .ref_0[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int + 0 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int) as usize]
                            as libc::c_int >> 1 as libc::c_int) as int8_t;
                        (*h)
                            .mb
                            .cache
                            .mv[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int + 0 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int
                            as usize] = ((*h)
                            .mb
                            .cache
                            .mv[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int + 0 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] as libc::c_int
                            * 2 as libc::c_int) as int16_t;
                        (*h)
                            .mb
                            .cache
                            .mvd[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int + 0 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int
                            as usize] = (((*h)
                            .mb
                            .cache
                            .mvd[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int + 0 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] as libc::c_int)
                            << 1 as libc::c_int) as uint8_t;
                    }
                    if (*h)
                        .mb
                        .cache
                        .ref_0[l_0
                        as usize][(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                        + 1 as libc::c_int - 1 as libc::c_int * 8 as libc::c_int)
                        as usize] as libc::c_int >= 0 as libc::c_int
                    {
                        (*h)
                            .mb
                            .cache
                            .ref_0[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int + 1 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize] = ((*h)
                            .mb
                            .cache
                            .ref_0[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int + 1 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int) as usize]
                            as libc::c_int >> 1 as libc::c_int) as int8_t;
                        (*h)
                            .mb
                            .cache
                            .mv[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int + 1 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int
                            as usize] = ((*h)
                            .mb
                            .cache
                            .mv[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int + 1 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] as libc::c_int
                            * 2 as libc::c_int) as int16_t;
                        (*h)
                            .mb
                            .cache
                            .mvd[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int + 1 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int
                            as usize] = (((*h)
                            .mb
                            .cache
                            .mvd[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int + 1 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] as libc::c_int)
                            << 1 as libc::c_int) as uint8_t;
                    }
                    if (*h)
                        .mb
                        .cache
                        .ref_0[l_0
                        as usize][(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                        + 2 as libc::c_int - 1 as libc::c_int * 8 as libc::c_int)
                        as usize] as libc::c_int >= 0 as libc::c_int
                    {
                        (*h)
                            .mb
                            .cache
                            .ref_0[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int + 2 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize] = ((*h)
                            .mb
                            .cache
                            .ref_0[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int + 2 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int) as usize]
                            as libc::c_int >> 1 as libc::c_int) as int8_t;
                        (*h)
                            .mb
                            .cache
                            .mv[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int + 2 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int
                            as usize] = ((*h)
                            .mb
                            .cache
                            .mv[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int + 2 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] as libc::c_int
                            * 2 as libc::c_int) as int16_t;
                        (*h)
                            .mb
                            .cache
                            .mvd[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int + 2 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int
                            as usize] = (((*h)
                            .mb
                            .cache
                            .mvd[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int + 2 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] as libc::c_int)
                            << 1 as libc::c_int) as uint8_t;
                    }
                    if (*h)
                        .mb
                        .cache
                        .ref_0[l_0
                        as usize][(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                        + 3 as libc::c_int - 1 as libc::c_int * 8 as libc::c_int)
                        as usize] as libc::c_int >= 0 as libc::c_int
                    {
                        (*h)
                            .mb
                            .cache
                            .ref_0[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int + 3 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize] = ((*h)
                            .mb
                            .cache
                            .ref_0[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int + 3 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int) as usize]
                            as libc::c_int >> 1 as libc::c_int) as int8_t;
                        (*h)
                            .mb
                            .cache
                            .mv[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int + 3 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int
                            as usize] = ((*h)
                            .mb
                            .cache
                            .mv[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int + 3 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] as libc::c_int
                            * 2 as libc::c_int) as int16_t;
                        (*h)
                            .mb
                            .cache
                            .mvd[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int + 3 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int
                            as usize] = (((*h)
                            .mb
                            .cache
                            .mvd[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int + 3 as libc::c_int
                            - 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] as libc::c_int)
                            << 1 as libc::c_int) as uint8_t;
                    }
                }
                if (*h).mb.i_mb_topright_xy >= 0 as libc::c_int && *((*h).mb.field).offset((*h).mb.i_mb_topright_xy as isize)
                        as libc::c_int != 0 && (*h)
                        .mb
                        .cache
                        .ref_0[l_0
                        as usize][(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                        + 4 as libc::c_int - 1 as libc::c_int * 8 as libc::c_int)
                        as usize] as libc::c_int >= 0 as libc::c_int {
                    (*h)
                        .mb
                        .cache
                        .ref_0[l_0
                        as usize][(x264_scan8[0 as libc::c_int as usize]
                        as libc::c_int + 4 as libc::c_int
                        - 1 as libc::c_int * 8 as libc::c_int)
                        as usize] = ((*h)
                        .mb
                        .cache
                        .ref_0[l_0
                        as usize][(x264_scan8[0 as libc::c_int as usize]
                        as libc::c_int + 4 as libc::c_int
                        - 1 as libc::c_int * 8 as libc::c_int) as usize]
                        as libc::c_int >> 1 as libc::c_int) as int8_t;
                    (*h)
                        .mb
                        .cache
                        .mv[l_0
                        as usize][(x264_scan8[0 as libc::c_int as usize]
                        as libc::c_int + 4 as libc::c_int
                        - 1 as libc::c_int * 8 as libc::c_int)
                        as usize][1 as libc::c_int
                        as usize] = ((*h)
                        .mb
                        .cache
                        .mv[l_0
                        as usize][(x264_scan8[0 as libc::c_int as usize]
                        as libc::c_int + 4 as libc::c_int
                        - 1 as libc::c_int * 8 as libc::c_int)
                        as usize][1 as libc::c_int as usize] as libc::c_int
                        * 2 as libc::c_int) as int16_t;
                    (*h)
                        .mb
                        .cache
                        .mvd[l_0
                        as usize][(x264_scan8[0 as libc::c_int as usize]
                        as libc::c_int + 4 as libc::c_int
                        - 1 as libc::c_int * 8 as libc::c_int)
                        as usize][1 as libc::c_int
                        as usize] = (((*h)
                        .mb
                        .cache
                        .mvd[l_0
                        as usize][(x264_scan8[0 as libc::c_int as usize]
                        as libc::c_int + 4 as libc::c_int
                        - 1 as libc::c_int * 8 as libc::c_int)
                        as usize][1 as libc::c_int as usize] as libc::c_int)
                        << 1 as libc::c_int) as uint8_t;
                }
                if *left.offset(0 as libc::c_int as isize) >= 0 as libc::c_int
                    && *((*h).mb.field)
                        .offset(*left.offset(0 as libc::c_int as isize) as isize)
                        as libc::c_int != 0
                {
                    if (*h)
                        .mb
                        .cache
                        .ref_0[l_0
                        as usize][(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                        - 1 as libc::c_int + 0 as libc::c_int * 8 as libc::c_int)
                        as usize] as libc::c_int >= 0 as libc::c_int
                    {
                        (*h)
                            .mb
                            .cache
                            .ref_0[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int - 1 as libc::c_int
                            + 0 as libc::c_int * 8 as libc::c_int)
                            as usize] = ((*h)
                            .mb
                            .cache
                            .ref_0[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int - 1 as libc::c_int
                            + 0 as libc::c_int * 8 as libc::c_int) as usize]
                            as libc::c_int >> 1 as libc::c_int) as int8_t;
                        (*h)
                            .mb
                            .cache
                            .mv[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int - 1 as libc::c_int
                            + 0 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int
                            as usize] = ((*h)
                            .mb
                            .cache
                            .mv[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int - 1 as libc::c_int
                            + 0 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] as libc::c_int
                            * 2 as libc::c_int) as int16_t;
                        (*h)
                            .mb
                            .cache
                            .mvd[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int - 1 as libc::c_int
                            + 0 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int
                            as usize] = (((*h)
                            .mb
                            .cache
                            .mvd[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int - 1 as libc::c_int
                            + 0 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] as libc::c_int)
                            << 1 as libc::c_int) as uint8_t;
                    }
                    if (*h)
                        .mb
                        .cache
                        .ref_0[l_0
                        as usize][(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                        - 1 as libc::c_int + 1 as libc::c_int * 8 as libc::c_int)
                        as usize] as libc::c_int >= 0 as libc::c_int
                    {
                        (*h)
                            .mb
                            .cache
                            .ref_0[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int - 1 as libc::c_int
                            + 1 as libc::c_int * 8 as libc::c_int)
                            as usize] = ((*h)
                            .mb
                            .cache
                            .ref_0[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int - 1 as libc::c_int
                            + 1 as libc::c_int * 8 as libc::c_int) as usize]
                            as libc::c_int >> 1 as libc::c_int) as int8_t;
                        (*h)
                            .mb
                            .cache
                            .mv[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int - 1 as libc::c_int
                            + 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int
                            as usize] = ((*h)
                            .mb
                            .cache
                            .mv[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int - 1 as libc::c_int
                            + 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] as libc::c_int
                            * 2 as libc::c_int) as int16_t;
                        (*h)
                            .mb
                            .cache
                            .mvd[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int - 1 as libc::c_int
                            + 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int
                            as usize] = (((*h)
                            .mb
                            .cache
                            .mvd[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int - 1 as libc::c_int
                            + 1 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] as libc::c_int)
                            << 1 as libc::c_int) as uint8_t;
                    }
                    if (*h)
                        .mb
                        .cache
                        .ref_0[l_0
                        as usize][(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                        - 1 as libc::c_int + 2 as libc::c_int * 8 as libc::c_int)
                        as usize] as libc::c_int >= 0 as libc::c_int
                    {
                        (*h)
                            .mb
                            .cache
                            .ref_0[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int - 1 as libc::c_int
                            + 2 as libc::c_int * 8 as libc::c_int)
                            as usize] = ((*h)
                            .mb
                            .cache
                            .ref_0[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int - 1 as libc::c_int
                            + 2 as libc::c_int * 8 as libc::c_int) as usize]
                            as libc::c_int >> 1 as libc::c_int) as int8_t;
                        (*h)
                            .mb
                            .cache
                            .mv[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int - 1 as libc::c_int
                            + 2 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int
                            as usize] = ((*h)
                            .mb
                            .cache
                            .mv[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int - 1 as libc::c_int
                            + 2 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] as libc::c_int
                            * 2 as libc::c_int) as int16_t;
                        (*h)
                            .mb
                            .cache
                            .mvd[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int - 1 as libc::c_int
                            + 2 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int
                            as usize] = (((*h)
                            .mb
                            .cache
                            .mvd[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int - 1 as libc::c_int
                            + 2 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] as libc::c_int)
                            << 1 as libc::c_int) as uint8_t;
                    }
                    if (*h)
                        .mb
                        .cache
                        .ref_0[l_0
                        as usize][(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                        - 1 as libc::c_int + 3 as libc::c_int * 8 as libc::c_int)
                        as usize] as libc::c_int >= 0 as libc::c_int
                    {
                        (*h)
                            .mb
                            .cache
                            .ref_0[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int - 1 as libc::c_int
                            + 3 as libc::c_int * 8 as libc::c_int)
                            as usize] = ((*h)
                            .mb
                            .cache
                            .ref_0[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int - 1 as libc::c_int
                            + 3 as libc::c_int * 8 as libc::c_int) as usize]
                            as libc::c_int >> 1 as libc::c_int) as int8_t;
                        (*h)
                            .mb
                            .cache
                            .mv[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int - 1 as libc::c_int
                            + 3 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int
                            as usize] = ((*h)
                            .mb
                            .cache
                            .mv[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int - 1 as libc::c_int
                            + 3 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] as libc::c_int
                            * 2 as libc::c_int) as int16_t;
                        (*h)
                            .mb
                            .cache
                            .mvd[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int - 1 as libc::c_int
                            + 3 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int
                            as usize] = (((*h)
                            .mb
                            .cache
                            .mvd[l_0
                            as usize][(x264_scan8[0 as libc::c_int as usize]
                            as libc::c_int - 1 as libc::c_int
                            + 3 as libc::c_int * 8 as libc::c_int)
                            as usize][1 as libc::c_int as usize] as libc::c_int)
                            << 1 as libc::c_int) as uint8_t;
                    }
                    if (*h)
                        .mb
                        .cache
                        .topright_ref[l_0 as usize][0 as libc::c_int as usize]
                        as libc::c_int >= 0 as libc::c_int
                    {
                        (*h)
                            .mb
                            .cache
                            .topright_ref[l_0
                            as usize][0 as libc::c_int
                            as usize] = ((*h)
                            .mb
                            .cache
                            .topright_ref[l_0 as usize][0 as libc::c_int as usize]
                            as libc::c_int >> 1 as libc::c_int) as int8_t;
                        (*h)
                            .mb
                            .cache
                            .topright_mv[l_0
                            as usize][0 as libc::c_int
                            as usize][1 as libc::c_int
                            as usize] = ((*h)
                            .mb
                            .cache
                            .topright_mv[l_0
                            as usize][0 as libc::c_int
                            as usize][1 as libc::c_int as usize] as libc::c_int
                            * 2 as libc::c_int) as int16_t;
                        (*h)
                            .mb
                            .cache
                            .mvd[l_0
                            as usize][0 as libc::c_int
                            as usize][1 as libc::c_int
                            as usize] = (((*h)
                            .mb
                            .cache
                            .mvd[l_0
                            as usize][0 as libc::c_int
                            as usize][1 as libc::c_int as usize] as libc::c_int)
                            << 1 as libc::c_int) as uint8_t;
                    }
                    if (*h)
                        .mb
                        .cache
                        .topright_ref[l_0 as usize][1 as libc::c_int as usize]
                        as libc::c_int >= 0 as libc::c_int
                    {
                        (*h)
                            .mb
                            .cache
                            .topright_ref[l_0
                            as usize][1 as libc::c_int
                            as usize] = ((*h)
                            .mb
                            .cache
                            .topright_ref[l_0 as usize][1 as libc::c_int as usize]
                            as libc::c_int >> 1 as libc::c_int) as int8_t;
                        (*h)
                            .mb
                            .cache
                            .topright_mv[l_0
                            as usize][1 as libc::c_int
                            as usize][1 as libc::c_int
                            as usize] = ((*h)
                            .mb
                            .cache
                            .topright_mv[l_0
                            as usize][1 as libc::c_int
                            as usize][1 as libc::c_int as usize] as libc::c_int
                            * 2 as libc::c_int) as int16_t;
                        (*h)
                            .mb
                            .cache
                            .mvd[l_0
                            as usize][1 as libc::c_int
                            as usize][1 as libc::c_int
                            as usize] = (((*h)
                            .mb
                            .cache
                            .mvd[l_0
                            as usize][1 as libc::c_int
                            as usize][1 as libc::c_int as usize] as libc::c_int)
                            << 1 as libc::c_int) as uint8_t;
                    }
                    if (*h)
                        .mb
                        .cache
                        .topright_ref[l_0 as usize][2 as libc::c_int as usize]
                        as libc::c_int >= 0 as libc::c_int
                    {
                        (*h)
                            .mb
                            .cache
                            .topright_ref[l_0
                            as usize][2 as libc::c_int
                            as usize] = ((*h)
                            .mb
                            .cache
                            .topright_ref[l_0 as usize][2 as libc::c_int as usize]
                            as libc::c_int >> 1 as libc::c_int) as int8_t;
                        (*h)
                            .mb
                            .cache
                            .topright_mv[l_0
                            as usize][2 as libc::c_int
                            as usize][1 as libc::c_int
                            as usize] = ((*h)
                            .mb
                            .cache
                            .topright_mv[l_0
                            as usize][2 as libc::c_int
                            as usize][1 as libc::c_int as usize] as libc::c_int
                            * 2 as libc::c_int) as int16_t;
                        (*h)
                            .mb
                            .cache
                            .mvd[l_0
                            as usize][2 as libc::c_int
                            as usize][1 as libc::c_int
                            as usize] = (((*h)
                            .mb
                            .cache
                            .mvd[l_0
                            as usize][2 as libc::c_int
                            as usize][1 as libc::c_int as usize] as libc::c_int)
                            << 1 as libc::c_int) as uint8_t;
                    }
                }
            }
        }
        l_0 += 1;
        l_0;
    }
    if b_mbaff != 0 && mb_x == 0 as libc::c_int && mb_y & 1 as libc::c_int == 0 {
        if (*h).mb.i_mb_top_xy >= (*h).sh.i_first_mb {
            (*h)
                .mb
                .field_decoding_flag = *((*h).mb.field)
                .offset((*h).mb.i_mb_top_xy as isize) as libc::c_int;
        } else {
            (*h).mb.field_decoding_flag = 0 as libc::c_int;
        }
    }
    (*h).mb.b_allow_skip = 1 as libc::c_int;
    if b_mbaff != 0 && (*h).mb.b_interlaced != (*h).mb.field_decoding_flag
            && mb_y & 1 as libc::c_int != 0 && (*((*h).mb.type_0)
                .offset(((*h).mb.i_mb_xy - (*h).mb.i_mb_stride) as isize) as libc::c_int
                == P_SKIP as libc::c_int
                || *((*h).mb.type_0)
                    .offset(((*h).mb.i_mb_xy - (*h).mb.i_mb_stride) as isize)
                    as libc::c_int == B_SKIP as libc::c_int) {
        (*h).mb.b_allow_skip = 0 as libc::c_int;
    }
    if (*h).param.b_cabac != 0 {
        if b_mbaff != 0 {
            let mut left_xy: libc::c_int = 0;
            let mut top_xy: libc::c_int = 0;
            let mut mb_xy: libc::c_int = mb_x
                + (mb_y & !(1 as libc::c_int)) * (*h).mb.i_mb_stride;
            left_xy = mb_xy - 1 as libc::c_int;
            if mb_y & 1 as libc::c_int != 0 && mb_x > 0 as libc::c_int
                && (*h).mb.field_decoding_flag
                    == *((*h).mb.field).offset(left_xy as isize) as libc::c_int
            {
                left_xy += (*h).mb.i_mb_stride;
            }
            if (*h).mb.field_decoding_flag != 0 {
                top_xy = mb_xy - (*h).mb.i_mb_stride;
                if mb_y & 1 as libc::c_int == 0 && top_xy >= 0 as libc::c_int
                    && *((*h).mb.slice_table).offset(top_xy as isize)
                        == (*h).sh.i_first_mb
                    && *((*h).mb.field).offset(top_xy as isize) as libc::c_int != 0
                {
                    top_xy -= (*h).mb.i_mb_stride;
                }
            } else {
                top_xy = mb_x + (mb_y - 1 as libc::c_int) * (*h).mb.i_mb_stride;
            }
            (*h)
                .mb
                .cache
                .i_neighbour_skip = (mb_x > 0 as libc::c_int
                && *((*h).mb.slice_table).offset(left_xy as isize) == (*h).sh.i_first_mb
                && !(*((*h).mb.type_0).offset(left_xy as isize) as libc::c_int
                    == P_SKIP as libc::c_int
                    || *((*h).mb.type_0).offset(left_xy as isize) as libc::c_int
                        == B_SKIP as libc::c_int)) as libc::c_int
                + (top_xy >= 0 as libc::c_int
                    && *((*h).mb.slice_table).offset(top_xy as isize)
                        == (*h).sh.i_first_mb
                    && !(*((*h).mb.type_0).offset(top_xy as isize) as libc::c_int
                        == P_SKIP as libc::c_int
                        || *((*h).mb.type_0).offset(top_xy as isize) as libc::c_int
                            == B_SKIP as libc::c_int)) as libc::c_int;
        } else {
            (*h)
                .mb
                .cache
                .i_neighbour_skip = ((*h).mb.i_neighbour
                & MB_LEFT as libc::c_int as libc::c_uint != 0
                && !((*h).mb.i_mb_type_left[0 as libc::c_int as usize]
                    == P_SKIP as libc::c_int
                    || (*h).mb.i_mb_type_left[0 as libc::c_int as usize]
                        == B_SKIP as libc::c_int)) as libc::c_int
                + ((*h).mb.i_neighbour & MB_TOP as libc::c_int as libc::c_uint != 0
                    && !((*h).mb.i_mb_type_top == P_SKIP as libc::c_int
                        || (*h).mb.i_mb_type_top == B_SKIP as libc::c_int))
                    as libc::c_int;
        }
    }
    if (*h).sh.i_type == SLICE_TYPE_B as libc::c_int {
        (*h)
            .mb
            .bipred_weight = ((*h)
            .mb
            .bipred_weight_buf[(*h).mb.b_interlaced
            as usize][((*h).mb.b_interlaced & (mb_y & 1 as libc::c_int)) as usize])
            .as_mut_ptr();
        (*h)
            .mb
            .dist_scale_factor = ((*h)
            .mb
            .dist_scale_factor_buf[(*h).mb.b_interlaced
            as usize][((*h).mb.b_interlaced & (mb_y & 1 as libc::c_int)) as usize])
            .as_mut_ptr();
        if (*h).param.b_cabac != 0 {
            let mut skipbp: uint8_t = 0;
            x264_macroblock_cache_skip(
                h,
                0 as libc::c_int,
                0 as libc::c_int,
                4 as libc::c_int,
                4 as libc::c_int,
                0 as libc::c_int,
            );
            if b_mbaff != 0 {
                skipbp = (if (*h).mb.i_neighbour & MB_LEFT as libc::c_int as libc::c_uint
                    != 0
                {
                    *((*h).mb.skipbp)
                        .offset(*left.offset(0 as libc::c_int as isize) as isize)
                        as libc::c_int
                } else {
                    0 as libc::c_int
                }) as uint8_t;
                (*h)
                    .mb
                    .cache
                    .skip[(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                    - 1 as libc::c_int)
                    as usize] = ((skipbp as libc::c_int >> (1 as libc::c_int
                        + ((*left_index_table).mv[0 as libc::c_int as usize]
                            as libc::c_int & !(1 as libc::c_int)))) & 1 as libc::c_int)
                    as int8_t;
                skipbp = (if (*h).mb.i_neighbour & MB_LEFT as libc::c_int as libc::c_uint
                    != 0
                {
                    *((*h).mb.skipbp)
                        .offset(*left.offset(1 as libc::c_int as isize) as isize)
                        as libc::c_int
                } else {
                    0 as libc::c_int
                }) as uint8_t;
                (*h)
                    .mb
                    .cache
                    .skip[(x264_scan8[8 as libc::c_int as usize] as libc::c_int
                    - 1 as libc::c_int)
                    as usize] = ((skipbp as libc::c_int >> (1 as libc::c_int
                        + ((*left_index_table).mv[2 as libc::c_int as usize]
                            as libc::c_int & !(1 as libc::c_int)))) & 1 as libc::c_int)
                    as int8_t;
            } else {
                skipbp = (if (*h).mb.i_neighbour & MB_LEFT as libc::c_int as libc::c_uint
                    != 0
                {
                    *((*h).mb.skipbp)
                        .offset(*left.offset(0 as libc::c_int as isize) as isize)
                        as libc::c_int
                } else {
                    0 as libc::c_int
                }) as uint8_t;
                (*h)
                    .mb
                    .cache
                    .skip[(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                    - 1 as libc::c_int)
                    as usize] = (skipbp as libc::c_int & 0x2 as libc::c_int) as int8_t;
                (*h)
                    .mb
                    .cache
                    .skip[(x264_scan8[8 as libc::c_int as usize] as libc::c_int
                    - 1 as libc::c_int)
                    as usize] = (skipbp as libc::c_int & 0x8 as libc::c_int) as int8_t;
            }
            skipbp = (if (*h).mb.i_neighbour & MB_TOP as libc::c_int as libc::c_uint != 0
            {
                *((*h).mb.skipbp).offset(top as isize) as libc::c_int
            } else {
                0 as libc::c_int
            }) as uint8_t;
            (*h)
                .mb
                .cache
                .skip[(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                - 8 as libc::c_int)
                as usize] = (skipbp as libc::c_int & 0x4 as libc::c_int) as int8_t;
            (*h)
                .mb
                .cache
                .skip[(x264_scan8[4 as libc::c_int as usize] as libc::c_int
                - 8 as libc::c_int)
                as usize] = (skipbp as libc::c_int & 0x8 as libc::c_int) as int8_t;
        }
    }
    if (*h).sh.i_type == SLICE_TYPE_P as libc::c_int {
        x264_8_mb_predict_mv_pskip(h, ((*h).mb.cache.pskip_mv).as_mut_ptr());
    }
    (*h)
        .mb
        .i_neighbour8[0 as libc::c_int
        as usize] = (*h).mb.i_neighbour_intra
        & (MB_TOP as libc::c_int | MB_LEFT as libc::c_int | MB_TOPLEFT as libc::c_int)
            as libc::c_uint
        | (if (*h).mb.i_neighbour_intra & MB_TOP as libc::c_int as libc::c_uint != 0 {
            MB_TOPRIGHT as libc::c_int
        } else {
            0 as libc::c_int
        }) as libc::c_uint;
    (*h)
        .mb
        .i_neighbour4[0 as libc::c_int
        as usize] = (*h).mb.i_neighbour8[0 as libc::c_int as usize];
    (*h)
        .mb
        .i_neighbour4[1 as libc::c_int
        as usize] = (MB_LEFT as libc::c_int
        | (if (*h).mb.i_neighbour_intra & MB_TOP as libc::c_int as libc::c_uint != 0 {
            MB_TOP as libc::c_int | MB_TOPLEFT as libc::c_int
                | MB_TOPRIGHT as libc::c_int
        } else {
            0 as libc::c_int
        })) as libc::c_uint;
    (*h)
        .mb
        .i_neighbour4[4 as libc::c_int
        as usize] = (*h).mb.i_neighbour4[1 as libc::c_int as usize];
    (*h)
        .mb
        .i_neighbour8[2 as libc::c_int
        as usize] = (MB_TOP as libc::c_int | MB_TOPRIGHT as libc::c_int
        | (if (*h).mb.i_neighbour_intra & MB_LEFT as libc::c_int as libc::c_uint != 0 {
            MB_LEFT as libc::c_int | MB_TOPLEFT as libc::c_int
        } else {
            0 as libc::c_int
        })) as libc::c_uint;
    (*h)
        .mb
        .i_neighbour4[10 as libc::c_int
        as usize] = (*h).mb.i_neighbour8[2 as libc::c_int as usize];
    (*h)
        .mb
        .i_neighbour4[8 as libc::c_int
        as usize] = (*h).mb.i_neighbour4[10 as libc::c_int as usize];
    (*h)
        .mb
        .i_neighbour4[2 as libc::c_int
        as usize] = (*h).mb.i_neighbour4[8 as libc::c_int as usize];
    (*h)
        .mb
        .i_neighbour8[1 as libc::c_int
        as usize] = MB_LEFT as libc::c_int as libc::c_uint
        | (*h).mb.i_neighbour_intra & MB_TOPRIGHT as libc::c_int as libc::c_uint
        | (if (*h).mb.i_neighbour_intra & MB_TOP as libc::c_int as libc::c_uint != 0 {
            MB_TOP as libc::c_int | MB_TOPLEFT as libc::c_int
        } else {
            0 as libc::c_int
        }) as libc::c_uint;
    (*h)
        .mb
        .i_neighbour4[5 as libc::c_int
        as usize] = (*h).mb.i_neighbour8[1 as libc::c_int as usize];
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_macroblock_cache_load_progressive(
    mut h: *mut x264_t,
    mut mb_x: libc::c_int,
    mut mb_y: libc::c_int,
) {
    macroblock_cache_load(h, mb_x, mb_y, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_macroblock_cache_load_interlaced(
    mut h: *mut x264_t,
    mut mb_x: libc::c_int,
    mut mb_y: libc::c_int,
) {
    macroblock_cache_load(h, mb_x, mb_y, 1 as libc::c_int);
}
unsafe extern "C" fn macroblock_deblock_strength_mbaff(
    mut h: *mut x264_t,
    mut bs: *mut [[uint8_t; 4]; 8],
) {
    if (*h).mb.i_neighbour & MB_LEFT as libc::c_int as libc::c_uint != 0
        && *((*h).mb.field)
            .offset((*h).mb.i_mb_left_xy[0 as libc::c_int as usize] as isize)
            as libc::c_int != (*h).mb.b_interlaced
    {
        static mut offset: [[[uint8_t; 8]; 2]; 2] = [
            [
                [
                    0 as libc::c_int as uint8_t,
                    0 as libc::c_int as uint8_t,
                    0 as libc::c_int as uint8_t,
                    0 as libc::c_int as uint8_t,
                    1 as libc::c_int as uint8_t,
                    1 as libc::c_int as uint8_t,
                    1 as libc::c_int as uint8_t,
                    1 as libc::c_int as uint8_t,
                ],
                [
                    2 as libc::c_int as uint8_t,
                    2 as libc::c_int as uint8_t,
                    2 as libc::c_int as uint8_t,
                    2 as libc::c_int as uint8_t,
                    3 as libc::c_int as uint8_t,
                    3 as libc::c_int as uint8_t,
                    3 as libc::c_int as uint8_t,
                    3 as libc::c_int as uint8_t,
                ],
            ],
            [
                [
                    0 as libc::c_int as uint8_t,
                    1 as libc::c_int as uint8_t,
                    2 as libc::c_int as uint8_t,
                    3 as libc::c_int as uint8_t,
                    0 as libc::c_int as uint8_t,
                    1 as libc::c_int as uint8_t,
                    2 as libc::c_int as uint8_t,
                    3 as libc::c_int as uint8_t,
                ],
                [
                    0 as libc::c_int as uint8_t,
                    1 as libc::c_int as uint8_t,
                    2 as libc::c_int as uint8_t,
                    3 as libc::c_int as uint8_t,
                    0 as libc::c_int as uint8_t,
                    1 as libc::c_int as uint8_t,
                    2 as libc::c_int as uint8_t,
                    3 as libc::c_int as uint8_t,
                ],
            ],
        ];
        let mut tmpbs: [uint8_t; 8] = [0; 8];
        let mut off: *const uint8_t = (offset[(*h).mb.b_interlaced
            as usize][((*h).mb.i_mb_y & 1 as libc::c_int) as usize])
            .as_ptr();
        let mut nnz: *mut [uint8_t; 48] = (*h).mb.non_zero_count;
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < 8 as libc::c_int {
            let mut left: libc::c_int = (*h)
                .mb
                .i_mb_left_xy[(if (*h).mb.b_interlaced != 0 {
                i >> 2 as libc::c_int
            } else {
                i & 1 as libc::c_int
            }) as usize];
            let mut nnz_this: libc::c_int = (*h)
                .mb
                .cache
                .non_zero_count[(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                + 8 as libc::c_int * (i >> 1 as libc::c_int)) as usize] as libc::c_int;
            let mut nnz_left: libc::c_int = (*nnz
                .offset(
                    left as isize,
                ))[(3 as libc::c_int
                + 4 as libc::c_int * *off.offset(i as isize) as libc::c_int) as usize]
                as libc::c_int;
            if (*h).param.b_cabac == 0
                && (*((*h).pps).as_mut_ptr()).b_transform_8x8_mode != 0
            {
                let mut j: libc::c_int = *off.offset(i as isize) as libc::c_int
                    & !(1 as libc::c_int);
                if *((*h).mb.mb_transform_size).offset(left as isize) != 0 {
                    nnz_left = ((*(&mut *(*nnz.offset(left as isize))
                        .as_mut_ptr()
                        .offset((2 as libc::c_int + 4 as libc::c_int * j) as isize)
                        as *mut uint8_t as *mut x264_union16_t))
                        .i as libc::c_int
                        | (*(&mut *(*nnz.offset(left as isize))
                            .as_mut_ptr()
                            .offset(
                                (2 as libc::c_int
                                    + 4 as libc::c_int * (1 as libc::c_int + j)) as isize,
                            ) as *mut uint8_t as *mut x264_union16_t))
                            .i as libc::c_int != 0) as libc::c_int;
                }
            }
            tmpbs[i
                as usize] = (if nnz_left != 0 || nnz_this != 0 {
                2 as libc::c_int
            } else {
                1 as libc::c_int
            }) as uint8_t;
            i += 1;
            i;
        }
        if (*h).mb.b_interlaced != 0 {
            (*(((*bs.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize])
                .as_mut_ptr() as *mut x264_union32_t))
                .i = (*(&mut *tmpbs.as_mut_ptr().offset(0 as libc::c_int as isize)
                as *mut uint8_t as *mut x264_union32_t))
                .i;
            (*(((*bs.offset(0 as libc::c_int as isize))[4 as libc::c_int as usize])
                .as_mut_ptr() as *mut x264_union32_t))
                .i = (*(&mut *tmpbs.as_mut_ptr().offset(4 as libc::c_int as isize)
                as *mut uint8_t as *mut x264_union32_t))
                .i;
        } else {
            let mut i_0: libc::c_int = 0 as libc::c_int;
            while i_0 < 4 as libc::c_int {
                (*bs
                    .offset(
                        0 as libc::c_int as isize,
                    ))[0 as libc::c_int
                    as usize][i_0 as usize] = tmpbs[(2 as libc::c_int * i_0) as usize];
                i_0 += 1;
                i_0;
            }
            let mut i_1: libc::c_int = 0 as libc::c_int;
            while i_1 < 4 as libc::c_int {
                (*bs
                    .offset(
                        0 as libc::c_int as isize,
                    ))[4 as libc::c_int
                    as usize][i_1
                    as usize] = tmpbs[(1 as libc::c_int + 2 as libc::c_int * i_1)
                    as usize];
                i_1 += 1;
                i_1;
            }
        }
    }
    if (*h).mb.i_neighbour & MB_TOP as libc::c_int as libc::c_uint != 0
        && (*h).mb.b_interlaced
            != *((*h).mb.field).offset((*h).mb.i_mb_top_xy as isize) as libc::c_int
    {
        if (*h).mb.i_mb_y & 1 as libc::c_int == 0 && (*h).mb.b_interlaced == 0 {
            let mut mbn_xy: libc::c_int = (*h).mb.i_mb_xy
                - 2 as libc::c_int * (*h).mb.i_mb_stride;
            let mut nnz_cur: *mut uint8_t = &mut *((*h).mb.cache.non_zero_count)
                .as_mut_ptr()
                .offset(*x264_scan8.as_ptr().offset(0 as libc::c_int as isize) as isize)
                as *mut uint8_t;
            let mut j_0: libc::c_int = 0 as libc::c_int;
            while j_0 < 2 as libc::c_int {
                let mut nnz_0: *mut [uint8_t; 48] = (*h).mb.non_zero_count;
                let mut nnz_top: [uint8_t; 4] = [0; 4];
                (*(nnz_top.as_mut_ptr() as *mut x264_union32_t))
                    .i = (*(&mut *(*nnz_0.offset(mbn_xy as isize))
                    .as_mut_ptr()
                    .offset((3 as libc::c_int * 4 as libc::c_int) as isize)
                    as *mut uint8_t as *mut x264_union32_t))
                    .i;
                if (*h).param.b_cabac == 0
                    && (*((*h).pps).as_mut_ptr()).b_transform_8x8_mode != 0
                    && *((*h).mb.mb_transform_size).offset(mbn_xy as isize)
                        as libc::c_int != 0
                {
                    nnz_top[1 as libc::c_int
                        as usize] = ((*(&mut *(*nnz_0.offset(mbn_xy as isize))
                        .as_mut_ptr()
                        .offset(8 as libc::c_int as isize) as *mut uint8_t
                        as *mut x264_union16_t))
                        .i as libc::c_int != 0
                        || (*(&mut *(*nnz_0.offset(mbn_xy as isize))
                            .as_mut_ptr()
                            .offset(12 as libc::c_int as isize) as *mut uint8_t
                            as *mut x264_union16_t))
                            .i as libc::c_int != 0) as libc::c_int as uint8_t;
                    nnz_top[0 as libc::c_int
                        as usize] = nnz_top[1 as libc::c_int as usize];
                    nnz_top[3 as libc::c_int
                        as usize] = ((*(&mut *(*nnz_0.offset(mbn_xy as isize))
                        .as_mut_ptr()
                        .offset(10 as libc::c_int as isize) as *mut uint8_t
                        as *mut x264_union16_t))
                        .i as libc::c_int != 0
                        || (*(&mut *(*nnz_0.offset(mbn_xy as isize))
                            .as_mut_ptr()
                            .offset(14 as libc::c_int as isize) as *mut uint8_t
                            as *mut x264_union16_t))
                            .i as libc::c_int != 0) as libc::c_int as uint8_t;
                    nnz_top[2 as libc::c_int
                        as usize] = nnz_top[3 as libc::c_int as usize];
                }
                let mut i_2: libc::c_int = 0 as libc::c_int;
                while i_2 < 4 as libc::c_int {
                    (*bs
                        .offset(
                            1 as libc::c_int as isize,
                        ))[(4 as libc::c_int * j_0)
                        as usize][i_2
                        as usize] = (if *nnz_cur.offset(i_2 as isize) as libc::c_int != 0
                        || nnz_top[i_2 as usize] as libc::c_int != 0
                    {
                        2 as libc::c_int
                    } else {
                        1 as libc::c_int
                    }) as uint8_t;
                    i_2 += 1;
                    i_2;
                }
                j_0 += 1;
                j_0;
                mbn_xy += (*h).mb.i_mb_stride;
            }
        } else {
            let mut i_3: libc::c_int = 0 as libc::c_int;
            while i_3 < 4 as libc::c_int {
                (*bs
                    .offset(
                        1 as libc::c_int as isize,
                    ))[0 as libc::c_int
                    as usize][i_3
                    as usize] = (if (*bs
                    .offset(
                        1 as libc::c_int as isize,
                    ))[0 as libc::c_int as usize][i_3 as usize] as libc::c_int
                    > 1 as libc::c_int
                {
                    (*bs
                        .offset(
                            1 as libc::c_int as isize,
                        ))[0 as libc::c_int as usize][i_3 as usize] as libc::c_int
                } else {
                    1 as libc::c_int
                }) as uint8_t;
                i_3 += 1;
                i_3;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_macroblock_deblock_strength(mut h: *mut x264_t) {
    let mut bs: *mut [[uint8_t; 4]; 8] = (*h).mb.cache.deblock_strength;
    if (*h).mb.i_type == I_4x4 as libc::c_int || (*h).mb.i_type == I_8x8 as libc::c_int
        || (*h).mb.i_type == I_16x16 as libc::c_int
        || (*h).mb.i_type == I_PCM as libc::c_int
    {
        (*(((*bs.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize])
            .as_mut_ptr() as *mut x264_union32_t))
            .i = 0x3030303 as libc::c_int as uint32_t;
        (*(((*bs.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize])
            .as_mut_ptr() as *mut x264_union64_t))
            .i = 0x303030303030303 as libc::c_ulonglong as uint64_t;
        (*(((*bs.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize])
            .as_mut_ptr() as *mut x264_union32_t))
            .i = 0x3030303 as libc::c_int as uint32_t;
        (*(((*bs.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize])
            .as_mut_ptr() as *mut x264_union64_t))
            .i = 0x303030303030303 as libc::c_ulonglong as uint64_t;
        return;
    }
    if (*h).mb.b_transform_8x8 != 0
        && ((*((*h).sps).as_mut_ptr()).i_chroma_format_idc != CHROMA_444 as libc::c_int)
    {
        let mut cbp_mask: libc::c_int = 0xf as libc::c_int >> (*h).mb.chroma_v_shift;
        if (*h).mb.i_cbp_luma & cbp_mask == cbp_mask {
            (*(((*bs.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize])
                .as_mut_ptr() as *mut x264_union32_t))
                .i = 0x2020202 as libc::c_int as uint32_t;
            (*(((*bs.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize])
                .as_mut_ptr() as *mut x264_union32_t))
                .i = 0x2020202 as libc::c_int as uint32_t;
            (*(((*bs.offset(0 as libc::c_int as isize))[4 as libc::c_int as usize])
                .as_mut_ptr() as *mut x264_union32_t))
                .i = 0x2020202 as libc::c_int as uint32_t;
            (*(((*bs.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize])
                .as_mut_ptr() as *mut x264_union64_t))
                .i = 0x202020202020202 as libc::c_ulonglong as uint64_t;
            (*(((*bs.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize])
                .as_mut_ptr() as *mut x264_union64_t))
                .i = 0x202020202020202 as libc::c_ulonglong as uint64_t;
            (*(((*bs.offset(1 as libc::c_int as isize))[4 as libc::c_int as usize])
                .as_mut_ptr() as *mut x264_union32_t))
                .i = 0x2020202 as libc::c_int as uint32_t;
            return;
        }
    }
    let mut neighbour_changed: libc::c_int = 0 as libc::c_int;
    if (*h).sh.i_disable_deblocking_filter_idc != 2 as libc::c_int {
        neighbour_changed = ((*h).mb.i_neighbour_frame & !(*h).mb.i_neighbour)
            as libc::c_int;
        (*h).mb.i_neighbour = (*h).mb.i_neighbour_frame;
    }
    if (*h).sh.b_mbaff != 0
        && (*h).mb.i_neighbour & MB_LEFT as libc::c_int as libc::c_uint != 0
        && *((*h).mb.field).offset(((*h).mb.i_mb_xy - 1 as libc::c_int) as isize)
            as libc::c_int != (*h).mb.b_interlaced
    {
        (*h)
            .mb
            .i_mb_left_xy[0 as libc::c_int
            as usize] = (*h).mb.i_mb_xy - 1 as libc::c_int;
        (*h)
            .mb
            .i_mb_left_xy[1 as libc::c_int
            as usize] = (*h).mb.i_mb_left_xy[0 as libc::c_int as usize];
        if (*h).mb.i_mb_y & 1 as libc::c_int != 0 {
            (*h).mb.i_mb_left_xy[0 as libc::c_int as usize] -= (*h).mb.i_mb_stride;
        } else {
            (*h).mb.i_mb_left_xy[1 as libc::c_int as usize] += (*h).mb.i_mb_stride;
        }
    }
    if neighbour_changed != 0 {
        let mut top_y: libc::c_int = (*h).mb.i_mb_top_y;
        let mut top_8x8: libc::c_int = (2 as libc::c_int * top_y + 1 as libc::c_int)
            * (*h).mb.i_b8_stride + 2 as libc::c_int * (*h).mb.i_mb_x;
        let mut top_4x4: libc::c_int = (4 as libc::c_int * top_y + 3 as libc::c_int)
            * (*h).mb.i_b4_stride + 4 as libc::c_int * (*h).mb.i_mb_x;
        let mut s8x8: libc::c_int = (*h).mb.i_b8_stride;
        let mut s4x4: libc::c_int = (*h).mb.i_b4_stride;
        let mut nnz: *mut [uint8_t; 48] = (*h).mb.non_zero_count;
        let mut left_index_table: *const x264_left_table_t = if (*h).sh.b_mbaff != 0 {
            (*h).mb.left_index_table
        } else {
            &*left_indices.as_ptr().offset(3 as libc::c_int as isize)
                as *const x264_left_table_t
        };
        if neighbour_changed & MB_TOP as libc::c_int != 0 {
            (*(&mut *((*h).mb.cache.non_zero_count)
                .as_mut_ptr()
                .offset(
                    (*x264_scan8.as_ptr().offset(0 as libc::c_int as isize)
                        as libc::c_int - 8 as libc::c_int) as isize,
                ) as *mut uint8_t as *mut x264_union32_t))
                .i = (*(&mut *(*nnz.offset((*h).mb.i_mb_top_xy as isize))
                .as_mut_ptr()
                .offset(12 as libc::c_int as isize) as *mut uint8_t
                as *mut x264_union32_t))
                .i;
        }
        if neighbour_changed & MB_LEFT as libc::c_int != 0 {
            let mut left: *mut libc::c_int = ((*h).mb.i_mb_left_xy).as_mut_ptr();
            (*h)
                .mb
                .cache
                .non_zero_count[(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                - 1 as libc::c_int)
                as usize] = (*nnz
                .offset(
                    *left.offset(0 as libc::c_int as isize) as isize,
                ))[(*left_index_table).nnz[0 as libc::c_int as usize] as usize];
            (*h)
                .mb
                .cache
                .non_zero_count[(x264_scan8[2 as libc::c_int as usize] as libc::c_int
                - 1 as libc::c_int)
                as usize] = (*nnz
                .offset(
                    *left.offset(0 as libc::c_int as isize) as isize,
                ))[(*left_index_table).nnz[1 as libc::c_int as usize] as usize];
            (*h)
                .mb
                .cache
                .non_zero_count[(x264_scan8[8 as libc::c_int as usize] as libc::c_int
                - 1 as libc::c_int)
                as usize] = (*nnz
                .offset(
                    *left.offset(1 as libc::c_int as isize) as isize,
                ))[(*left_index_table).nnz[2 as libc::c_int as usize] as usize];
            (*h)
                .mb
                .cache
                .non_zero_count[(x264_scan8[10 as libc::c_int as usize] as libc::c_int
                - 1 as libc::c_int)
                as usize] = (*nnz
                .offset(
                    *left.offset(1 as libc::c_int as isize) as isize,
                ))[(*left_index_table).nnz[3 as libc::c_int as usize] as usize];
        }
        let mut l: libc::c_int = 0 as libc::c_int;
        while l <= ((*h).sh.i_type == SLICE_TYPE_B as libc::c_int) as libc::c_int {
            let mut mv: *mut [int16_t; 2] = (*h).mb.mv[l as usize];
            let mut ref_0: *mut int8_t = (*h).mb.ref_0[l as usize];
            let mut i8: libc::c_int = x264_scan8[0 as libc::c_int as usize]
                as libc::c_int - 8 as libc::c_int;
            if neighbour_changed & MB_TOP as libc::c_int != 0 {
                (*h)
                    .mb
                    .cache
                    .ref_0[l
                    as usize][(i8 + 1 as libc::c_int)
                    as usize] = *ref_0.offset((top_8x8 + 0 as libc::c_int) as isize);
                (*h)
                    .mb
                    .cache
                    .ref_0[l
                    as usize][(i8 + 0 as libc::c_int)
                    as usize] = (*h)
                    .mb
                    .cache
                    .ref_0[l as usize][(i8 + 1 as libc::c_int) as usize];
                (*h)
                    .mb
                    .cache
                    .ref_0[l
                    as usize][(i8 + 3 as libc::c_int)
                    as usize] = *ref_0.offset((top_8x8 + 1 as libc::c_int) as isize);
                (*h)
                    .mb
                    .cache
                    .ref_0[l
                    as usize][(i8 + 2 as libc::c_int)
                    as usize] = (*h)
                    .mb
                    .cache
                    .ref_0[l as usize][(i8 + 3 as libc::c_int) as usize];
                (*(((*h).mb.cache.mv[l as usize][i8 as usize]).as_mut_ptr()
                    as *mut x264_union128_sse_t))
                    .i = (*((*mv.offset(top_4x4 as isize)).as_mut_ptr()
                    as *mut x264_union128_sse_t))
                    .i;
            }
            i8 = x264_scan8[0 as libc::c_int as usize] as libc::c_int - 1 as libc::c_int;
            if neighbour_changed & MB_LEFT as libc::c_int != 0 {
                (*h)
                    .mb
                    .cache
                    .ref_0[l
                    as usize][(i8 + 1 as libc::c_int * 8 as libc::c_int)
                    as usize] = *ref_0
                    .offset(
                        ((*h).mb.left_b8[0 as libc::c_int as usize] + 1 as libc::c_int
                            + s8x8
                                * (*left_index_table).ref_0[0 as libc::c_int as usize]
                                    as libc::c_int) as isize,
                    );
                (*h)
                    .mb
                    .cache
                    .ref_0[l
                    as usize][(i8 + 0 as libc::c_int * 8 as libc::c_int)
                    as usize] = (*h)
                    .mb
                    .cache
                    .ref_0[l
                    as usize][(i8 + 1 as libc::c_int * 8 as libc::c_int) as usize];
                (*h)
                    .mb
                    .cache
                    .ref_0[l
                    as usize][(i8 + 3 as libc::c_int * 8 as libc::c_int)
                    as usize] = *ref_0
                    .offset(
                        ((*h).mb.left_b8[1 as libc::c_int as usize] + 1 as libc::c_int
                            + s8x8
                                * (*left_index_table).ref_0[2 as libc::c_int as usize]
                                    as libc::c_int) as isize,
                    );
                (*h)
                    .mb
                    .cache
                    .ref_0[l
                    as usize][(i8 + 2 as libc::c_int * 8 as libc::c_int)
                    as usize] = (*h)
                    .mb
                    .cache
                    .ref_0[l
                    as usize][(i8 + 3 as libc::c_int * 8 as libc::c_int) as usize];
                (*(((*h)
                    .mb
                    .cache
                    .mv[l as usize][(i8 + 0 as libc::c_int * 8 as libc::c_int) as usize])
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i = (*((*mv
                    .offset(
                        ((*h).mb.left_b4[0 as libc::c_int as usize] + 3 as libc::c_int
                            + s4x4
                                * (*left_index_table).mv[0 as libc::c_int as usize]
                                    as libc::c_int) as isize,
                    ))
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i;
                (*(((*h)
                    .mb
                    .cache
                    .mv[l as usize][(i8 + 1 as libc::c_int * 8 as libc::c_int) as usize])
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i = (*((*mv
                    .offset(
                        ((*h).mb.left_b4[0 as libc::c_int as usize] + 3 as libc::c_int
                            + s4x4
                                * (*left_index_table).mv[1 as libc::c_int as usize]
                                    as libc::c_int) as isize,
                    ))
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i;
                (*(((*h)
                    .mb
                    .cache
                    .mv[l as usize][(i8 + 2 as libc::c_int * 8 as libc::c_int) as usize])
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i = (*((*mv
                    .offset(
                        ((*h).mb.left_b4[1 as libc::c_int as usize] + 3 as libc::c_int
                            + s4x4
                                * (*left_index_table).mv[2 as libc::c_int as usize]
                                    as libc::c_int) as isize,
                    ))
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i;
                (*(((*h)
                    .mb
                    .cache
                    .mv[l as usize][(i8 + 3 as libc::c_int * 8 as libc::c_int) as usize])
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i = (*((*mv
                    .offset(
                        ((*h).mb.left_b4[1 as libc::c_int as usize] + 3 as libc::c_int
                            + s4x4
                                * (*left_index_table).mv[3 as libc::c_int as usize]
                                    as libc::c_int) as isize,
                    ))
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i;
            }
            l += 1;
            l;
        }
    }
    if (*h).param.analyse.i_weighted_pred == 2 as libc::c_int
        && (*h).sh.i_type == SLICE_TYPE_P as libc::c_int
    {
        let mut i8_0: libc::c_int = x264_scan8[0 as libc::c_int as usize] as libc::c_int
            - 8 as libc::c_int;
        (*h)
            .mb
            .cache
            .ref_0[0 as libc::c_int
            as usize][(i8_0 + 1 as libc::c_int)
            as usize] = (*h)
            .mb
            .deblock_ref_table[((*h)
            .mb
            .cache
            .ref_0[0 as libc::c_int as usize][(i8_0 + 0 as libc::c_int) as usize]
            as libc::c_int + 2 as libc::c_int) as usize];
        (*h)
            .mb
            .cache
            .ref_0[0 as libc::c_int
            as usize][(i8_0 + 0 as libc::c_int)
            as usize] = (*h)
            .mb
            .cache
            .ref_0[0 as libc::c_int as usize][(i8_0 + 1 as libc::c_int) as usize];
        (*h)
            .mb
            .cache
            .ref_0[0 as libc::c_int
            as usize][(i8_0 + 3 as libc::c_int)
            as usize] = (*h)
            .mb
            .deblock_ref_table[((*h)
            .mb
            .cache
            .ref_0[0 as libc::c_int as usize][(i8_0 + 2 as libc::c_int) as usize]
            as libc::c_int + 2 as libc::c_int) as usize];
        (*h)
            .mb
            .cache
            .ref_0[0 as libc::c_int
            as usize][(i8_0 + 2 as libc::c_int)
            as usize] = (*h)
            .mb
            .cache
            .ref_0[0 as libc::c_int as usize][(i8_0 + 3 as libc::c_int) as usize];
        i8_0 = x264_scan8[0 as libc::c_int as usize] as libc::c_int - 1 as libc::c_int;
        (*h)
            .mb
            .cache
            .ref_0[0 as libc::c_int
            as usize][(i8_0 + 1 as libc::c_int * 8 as libc::c_int)
            as usize] = (*h)
            .mb
            .deblock_ref_table[((*h)
            .mb
            .cache
            .ref_0[0 as libc::c_int
            as usize][(i8_0 + 0 as libc::c_int * 8 as libc::c_int) as usize]
            as libc::c_int + 2 as libc::c_int) as usize];
        (*h)
            .mb
            .cache
            .ref_0[0 as libc::c_int
            as usize][(i8_0 + 0 as libc::c_int * 8 as libc::c_int)
            as usize] = (*h)
            .mb
            .cache
            .ref_0[0 as libc::c_int
            as usize][(i8_0 + 1 as libc::c_int * 8 as libc::c_int) as usize];
        (*h)
            .mb
            .cache
            .ref_0[0 as libc::c_int
            as usize][(i8_0 + 3 as libc::c_int * 8 as libc::c_int)
            as usize] = (*h)
            .mb
            .deblock_ref_table[((*h)
            .mb
            .cache
            .ref_0[0 as libc::c_int
            as usize][(i8_0 + 2 as libc::c_int * 8 as libc::c_int) as usize]
            as libc::c_int + 2 as libc::c_int) as usize];
        (*h)
            .mb
            .cache
            .ref_0[0 as libc::c_int
            as usize][(i8_0 + 2 as libc::c_int * 8 as libc::c_int)
            as usize] = (*h)
            .mb
            .cache
            .ref_0[0 as libc::c_int
            as usize][(i8_0 + 3 as libc::c_int * 8 as libc::c_int) as usize];
        let mut ref0: libc::c_int = (*h)
            .mb
            .deblock_ref_table[((*h)
            .mb
            .cache
            .ref_0[0 as libc::c_int
            as usize][x264_scan8[0 as libc::c_int as usize] as usize] as libc::c_int
            + 2 as libc::c_int) as usize] as libc::c_int;
        let mut ref1: libc::c_int = (*h)
            .mb
            .deblock_ref_table[((*h)
            .mb
            .cache
            .ref_0[0 as libc::c_int
            as usize][x264_scan8[4 as libc::c_int as usize] as usize] as libc::c_int
            + 2 as libc::c_int) as usize] as libc::c_int;
        let mut ref2: libc::c_int = (*h)
            .mb
            .deblock_ref_table[((*h)
            .mb
            .cache
            .ref_0[0 as libc::c_int
            as usize][x264_scan8[8 as libc::c_int as usize] as usize] as libc::c_int
            + 2 as libc::c_int) as usize] as libc::c_int;
        let mut ref3: libc::c_int = (*h)
            .mb
            .deblock_ref_table[((*h)
            .mb
            .cache
            .ref_0[0 as libc::c_int
            as usize][x264_scan8[12 as libc::c_int as usize] as usize] as libc::c_int
            + 2 as libc::c_int) as usize] as libc::c_int;
        let mut reftop: uint32_t = pack16to32(
            ref0 as uint8_t as uint32_t,
            ref1 as uint8_t as uint32_t,
        ) * 0x101 as libc::c_int as uint32_t;
        let mut refbot: uint32_t = pack16to32(
            ref2 as uint8_t as uint32_t,
            ref3 as uint8_t as uint32_t,
        ) * 0x101 as libc::c_int as uint32_t;
        (*(&mut *(*((*h).mb.cache.ref_0).as_mut_ptr().offset(0 as libc::c_int as isize))
            .as_mut_ptr()
            .offset(
                (*x264_scan8.as_ptr().offset(0 as libc::c_int as isize) as libc::c_int
                    + 8 as libc::c_int * 0 as libc::c_int) as isize,
            ) as *mut int8_t as *mut x264_union32_t))
            .i = reftop;
        (*(&mut *(*((*h).mb.cache.ref_0).as_mut_ptr().offset(0 as libc::c_int as isize))
            .as_mut_ptr()
            .offset(
                (*x264_scan8.as_ptr().offset(0 as libc::c_int as isize) as libc::c_int
                    + 8 as libc::c_int * 1 as libc::c_int) as isize,
            ) as *mut int8_t as *mut x264_union32_t))
            .i = reftop;
        (*(&mut *(*((*h).mb.cache.ref_0).as_mut_ptr().offset(0 as libc::c_int as isize))
            .as_mut_ptr()
            .offset(
                (*x264_scan8.as_ptr().offset(0 as libc::c_int as isize) as libc::c_int
                    + 8 as libc::c_int * 2 as libc::c_int) as isize,
            ) as *mut int8_t as *mut x264_union32_t))
            .i = refbot;
        (*(&mut *(*((*h).mb.cache.ref_0).as_mut_ptr().offset(0 as libc::c_int as isize))
            .as_mut_ptr()
            .offset(
                (*x264_scan8.as_ptr().offset(0 as libc::c_int as isize) as libc::c_int
                    + 8 as libc::c_int * 3 as libc::c_int) as isize,
            ) as *mut int8_t as *mut x264_union32_t))
            .i = refbot;
    }
    if (*h).param.b_cabac == 0 && (*((*h).pps).as_mut_ptr()).b_transform_8x8_mode != 0 {
        let mut nnz_0: *mut [uint8_t; 48] = (*h).mb.non_zero_count;
        let mut top: libc::c_int = (*h).mb.i_mb_top_xy;
        let mut left_0: *mut libc::c_int = ((*h).mb.i_mb_left_xy).as_mut_ptr();
        if (*h).mb.i_neighbour & MB_TOP as libc::c_int as libc::c_uint != 0
            && *((*h).mb.mb_transform_size).offset(top as isize) as libc::c_int != 0
        {
            let mut i8_1: libc::c_int = x264_scan8[0 as libc::c_int as usize]
                as libc::c_int - 8 as libc::c_int;
            let mut nnz_top0: libc::c_int = (*(&mut *(*nnz_0.offset(top as isize))
                .as_mut_ptr()
                .offset(8 as libc::c_int as isize) as *mut uint8_t
                as *mut x264_union16_t))
                .i as libc::c_int
                | (*(&mut *(*nnz_0.offset(top as isize))
                    .as_mut_ptr()
                    .offset(12 as libc::c_int as isize) as *mut uint8_t
                    as *mut x264_union16_t))
                    .i as libc::c_int;
            let mut nnz_top1: libc::c_int = (*(&mut *(*nnz_0.offset(top as isize))
                .as_mut_ptr()
                .offset(10 as libc::c_int as isize) as *mut uint8_t
                as *mut x264_union16_t))
                .i as libc::c_int
                | (*(&mut *(*nnz_0.offset(top as isize))
                    .as_mut_ptr()
                    .offset(14 as libc::c_int as isize) as *mut uint8_t
                    as *mut x264_union16_t))
                    .i as libc::c_int;
            (*(&mut *((*h).mb.cache.non_zero_count)
                .as_mut_ptr()
                .offset((i8_1 + 0 as libc::c_int) as isize) as *mut uint8_t
                as *mut x264_union16_t))
                .i = (if nnz_top0 != 0 {
                0x101 as libc::c_int
            } else {
                0 as libc::c_int
            }) as uint16_t;
            (*(&mut *((*h).mb.cache.non_zero_count)
                .as_mut_ptr()
                .offset((i8_1 + 2 as libc::c_int) as isize) as *mut uint8_t
                as *mut x264_union16_t))
                .i = (if nnz_top1 != 0 {
                0x101 as libc::c_int
            } else {
                0 as libc::c_int
            }) as uint16_t;
        }
        if (*h).mb.i_neighbour & MB_LEFT as libc::c_int as libc::c_uint != 0 {
            let mut i8_2: libc::c_int = x264_scan8[0 as libc::c_int as usize]
                as libc::c_int - 1 as libc::c_int;
            if *((*h).mb.mb_transform_size)
                .offset(*left_0.offset(0 as libc::c_int as isize) as isize) != 0
            {
                let mut nnz_left0: libc::c_int = (*(&mut *(*nnz_0
                    .offset(*left_0.offset(0 as libc::c_int as isize) as isize))
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as isize) as *mut uint8_t
                    as *mut x264_union16_t))
                    .i as libc::c_int
                    | (*(&mut *(*nnz_0
                        .offset(*left_0.offset(0 as libc::c_int as isize) as isize))
                        .as_mut_ptr()
                        .offset(6 as libc::c_int as isize) as *mut uint8_t
                        as *mut x264_union16_t))
                        .i as libc::c_int;
                (*h)
                    .mb
                    .cache
                    .non_zero_count[(i8_2 + 8 as libc::c_int * 0 as libc::c_int)
                    as usize] = (nnz_left0 != 0) as libc::c_int as uint8_t;
                (*h)
                    .mb
                    .cache
                    .non_zero_count[(i8_2 + 8 as libc::c_int * 1 as libc::c_int)
                    as usize] = (nnz_left0 != 0) as libc::c_int as uint8_t;
            }
            if *((*h).mb.mb_transform_size)
                .offset(*left_0.offset(1 as libc::c_int as isize) as isize) != 0
            {
                let mut nnz_left1: libc::c_int = (*(&mut *(*nnz_0
                    .offset(*left_0.offset(1 as libc::c_int as isize) as isize))
                    .as_mut_ptr()
                    .offset(10 as libc::c_int as isize) as *mut uint8_t
                    as *mut x264_union16_t))
                    .i as libc::c_int
                    | (*(&mut *(*nnz_0
                        .offset(*left_0.offset(1 as libc::c_int as isize) as isize))
                        .as_mut_ptr()
                        .offset(14 as libc::c_int as isize) as *mut uint8_t
                        as *mut x264_union16_t))
                        .i as libc::c_int;
                (*h)
                    .mb
                    .cache
                    .non_zero_count[(i8_2 + 8 as libc::c_int * 2 as libc::c_int)
                    as usize] = (nnz_left1 != 0) as libc::c_int as uint8_t;
                (*h)
                    .mb
                    .cache
                    .non_zero_count[(i8_2 + 8 as libc::c_int * 3 as libc::c_int)
                    as usize] = (nnz_left1 != 0) as libc::c_int as uint8_t;
            }
        }
        if (*h).mb.b_transform_8x8 != 0 {
            let mut nnz0: libc::c_int = (*(&mut *((*h).mb.cache.non_zero_count)
                .as_mut_ptr()
                .offset(*x264_scan8.as_ptr().offset(0 as libc::c_int as isize) as isize)
                as *mut uint8_t as *mut x264_union16_t))
                .i as libc::c_int
                | (*(&mut *((*h).mb.cache.non_zero_count)
                    .as_mut_ptr()
                    .offset(
                        *x264_scan8.as_ptr().offset(2 as libc::c_int as isize) as isize,
                    ) as *mut uint8_t as *mut x264_union16_t))
                    .i as libc::c_int;
            let mut nnz1: libc::c_int = (*(&mut *((*h).mb.cache.non_zero_count)
                .as_mut_ptr()
                .offset(*x264_scan8.as_ptr().offset(4 as libc::c_int as isize) as isize)
                as *mut uint8_t as *mut x264_union16_t))
                .i as libc::c_int
                | (*(&mut *((*h).mb.cache.non_zero_count)
                    .as_mut_ptr()
                    .offset(
                        *x264_scan8.as_ptr().offset(6 as libc::c_int as isize) as isize,
                    ) as *mut uint8_t as *mut x264_union16_t))
                    .i as libc::c_int;
            let mut nnz2: libc::c_int = (*(&mut *((*h).mb.cache.non_zero_count)
                .as_mut_ptr()
                .offset(*x264_scan8.as_ptr().offset(8 as libc::c_int as isize) as isize)
                as *mut uint8_t as *mut x264_union16_t))
                .i as libc::c_int
                | (*(&mut *((*h).mb.cache.non_zero_count)
                    .as_mut_ptr()
                    .offset(
                        *x264_scan8.as_ptr().offset(10 as libc::c_int as isize) as isize,
                    ) as *mut uint8_t as *mut x264_union16_t))
                    .i as libc::c_int;
            let mut nnz3: libc::c_int = (*(&mut *((*h).mb.cache.non_zero_count)
                .as_mut_ptr()
                .offset(*x264_scan8.as_ptr().offset(12 as libc::c_int as isize) as isize)
                as *mut uint8_t as *mut x264_union16_t))
                .i as libc::c_int
                | (*(&mut *((*h).mb.cache.non_zero_count)
                    .as_mut_ptr()
                    .offset(
                        *x264_scan8.as_ptr().offset(14 as libc::c_int as isize) as isize,
                    ) as *mut uint8_t as *mut x264_union16_t))
                    .i as libc::c_int;
            let mut nnztop: uint32_t = pack16to32(
                (nnz0 != 0) as libc::c_int as uint32_t,
                (nnz1 != 0) as libc::c_int as uint32_t,
            ) * 0x101 as libc::c_int as uint32_t;
            let mut nnzbot: uint32_t = pack16to32(
                (nnz2 != 0) as libc::c_int as uint32_t,
                (nnz3 != 0) as libc::c_int as uint32_t,
            ) * 0x101 as libc::c_int as uint32_t;
            (*(&mut *((*h).mb.cache.non_zero_count)
                .as_mut_ptr()
                .offset(
                    (*x264_scan8.as_ptr().offset(0 as libc::c_int as isize)
                        as libc::c_int + 8 as libc::c_int * 0 as libc::c_int) as isize,
                ) as *mut uint8_t as *mut x264_union32_t))
                .i = nnztop;
            (*(&mut *((*h).mb.cache.non_zero_count)
                .as_mut_ptr()
                .offset(
                    (*x264_scan8.as_ptr().offset(0 as libc::c_int as isize)
                        as libc::c_int + 8 as libc::c_int * 1 as libc::c_int) as isize,
                ) as *mut uint8_t as *mut x264_union32_t))
                .i = nnztop;
            (*(&mut *((*h).mb.cache.non_zero_count)
                .as_mut_ptr()
                .offset(
                    (*x264_scan8.as_ptr().offset(0 as libc::c_int as isize)
                        as libc::c_int + 8 as libc::c_int * 2 as libc::c_int) as isize,
                ) as *mut uint8_t as *mut x264_union32_t))
                .i = nnzbot;
            (*(&mut *((*h).mb.cache.non_zero_count)
                .as_mut_ptr()
                .offset(
                    (*x264_scan8.as_ptr().offset(0 as libc::c_int as isize)
                        as libc::c_int + 8 as libc::c_int * 3 as libc::c_int) as isize,
                ) as *mut uint8_t as *mut x264_union32_t))
                .i = nnzbot;
        }
    }
    ((*h).loopf.deblock_strength)
        .expect(
            "non-null function pointer",
        )(
        ((*h).mb.cache.non_zero_count).as_mut_ptr(),
        ((*h).mb.cache.ref_0).as_mut_ptr(),
        ((*h).mb.cache.mv).as_mut_ptr(),
        bs,
        4 as libc::c_int >> (*h).mb.b_interlaced,
        ((*h).sh.i_type == SLICE_TYPE_B as libc::c_int) as libc::c_int,
    );
    if (*h).sh.b_mbaff != 0 {
        macroblock_deblock_strength_mbaff(h, bs);
    }
}
#[inline(always)]
unsafe extern "C" fn macroblock_store_pic(
    mut h: *mut x264_t,
    mut mb_x: libc::c_int,
    mut mb_y: libc::c_int,
    mut i: libc::c_int,
    mut b_chroma: libc::c_int,
    mut b_mbaff: libc::c_int,
) {
    let mut height: libc::c_int = if b_chroma != 0 {
        16 as libc::c_int >> (*h).mb.chroma_v_shift
    } else {
        16 as libc::c_int
    };
    let mut i_stride: libc::c_int = (*(*h).fdec).i_stride[i as usize];
    let mut i_stride2: libc::c_int = i_stride
        << (b_mbaff != 0 && (*h).mb.b_interlaced != 0) as libc::c_int;
    let mut i_pix_offset: libc::c_int = if b_mbaff != 0 && (*h).mb.b_interlaced != 0 {
        16 as libc::c_int * mb_x + height * (mb_y & !(1 as libc::c_int)) * i_stride
            + (mb_y & 1 as libc::c_int) * i_stride
    } else {
        16 as libc::c_int * mb_x + height * mb_y * i_stride
    };
    if b_chroma != 0 {
        ((*h).mc.store_interleave_chroma)
            .expect(
                "non-null function pointer",
            )(
            &mut *(*((*(*h).fdec).plane).as_mut_ptr().offset(1 as libc::c_int as isize))
                .offset(i_pix_offset as isize),
            i_stride2 as intptr_t,
            (*h).mb.pic.p_fdec[1 as libc::c_int as usize],
            (*h).mb.pic.p_fdec[2 as libc::c_int as usize],
            height,
        );
    } else {
        ((*h).mc.copy[PIXEL_16x16 as libc::c_int as usize])
            .expect(
                "non-null function pointer",
            )(
            &mut *(*((*(*h).fdec).plane).as_mut_ptr().offset(i as isize))
                .offset(i_pix_offset as isize),
            i_stride2 as intptr_t,
            (*h).mb.pic.p_fdec[i as usize],
            32 as libc::c_int as intptr_t,
            16 as libc::c_int,
        );
    };
}
#[inline(always)]
unsafe extern "C" fn macroblock_backup_intra(
    mut h: *mut x264_t,
    mut mb_x: libc::c_int,
    mut mb_y: libc::c_int,
    mut b_mbaff: libc::c_int,
) {
    let mut backup_dst: libc::c_int = if b_mbaff == 0 {
        mb_y & 1 as libc::c_int
    } else if mb_y & 1 as libc::c_int != 0 {
        1 as libc::c_int
    } else if (*h).mb.b_interlaced != 0 {
        0 as libc::c_int
    } else {
        2 as libc::c_int
    };
    memcpy(
        &mut *(*(*((*h).intra_border_backup).as_mut_ptr().offset(backup_dst as isize))
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize))
            .offset((mb_x * 16 as libc::c_int) as isize) as *mut pixel
            as *mut libc::c_void,
        ((*h).mb.pic.p_fdec[0 as libc::c_int as usize])
            .offset((32 as libc::c_int * 15 as libc::c_int) as isize)
            as *const libc::c_void,
        (16 as libc::c_int
            * ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int)
            as libc::c_ulong,
    );
    if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as libc::c_int {
        memcpy(
            &mut *(*(*((*h).intra_border_backup)
                .as_mut_ptr()
                .offset(backup_dst as isize))
                .as_mut_ptr()
                .offset(1 as libc::c_int as isize))
                .offset((mb_x * 16 as libc::c_int) as isize) as *mut pixel
                as *mut libc::c_void,
            ((*h).mb.pic.p_fdec[1 as libc::c_int as usize])
                .offset((32 as libc::c_int * 15 as libc::c_int) as isize)
                as *const libc::c_void,
            (16 as libc::c_int
                * ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int)
                as libc::c_ulong,
        );
        memcpy(
            &mut *(*(*((*h).intra_border_backup)
                .as_mut_ptr()
                .offset(backup_dst as isize))
                .as_mut_ptr()
                .offset(2 as libc::c_int as isize))
                .offset((mb_x * 16 as libc::c_int) as isize) as *mut pixel
                as *mut libc::c_void,
            ((*h).mb.pic.p_fdec[2 as libc::c_int as usize])
                .offset((32 as libc::c_int * 15 as libc::c_int) as isize)
                as *const libc::c_void,
            (16 as libc::c_int
                * ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int)
                as libc::c_ulong,
        );
    } else if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc != 0 {
        let mut backup_src: libc::c_int = (15 as libc::c_int >> (*h).mb.chroma_v_shift)
            * 32 as libc::c_int;
        memcpy(
            &mut *(*(*((*h).intra_border_backup)
                .as_mut_ptr()
                .offset(backup_dst as isize))
                .as_mut_ptr()
                .offset(1 as libc::c_int as isize))
                .offset((mb_x * 16 as libc::c_int) as isize) as *mut pixel
                as *mut libc::c_void,
            ((*h).mb.pic.p_fdec[1 as libc::c_int as usize]).offset(backup_src as isize)
                as *const libc::c_void,
            (8 as libc::c_int
                * ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int)
                as libc::c_ulong,
        );
        memcpy(
            &mut *(*(*((*h).intra_border_backup)
                .as_mut_ptr()
                .offset(backup_dst as isize))
                .as_mut_ptr()
                .offset(1 as libc::c_int as isize))
                .offset((mb_x * 16 as libc::c_int + 8 as libc::c_int) as isize)
                as *mut pixel as *mut libc::c_void,
            ((*h).mb.pic.p_fdec[2 as libc::c_int as usize]).offset(backup_src as isize)
                as *const libc::c_void,
            (8 as libc::c_int
                * ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int)
                as libc::c_ulong,
        );
    }
    if b_mbaff != 0 && mb_y & 1 as libc::c_int != 0 {
        let mut backup_src_0: libc::c_int = (if (*h).mb.b_interlaced != 0 {
            7 as libc::c_int
        } else {
            14 as libc::c_int
        }) * 32 as libc::c_int;
        backup_dst = if (*h).mb.b_interlaced != 0 {
            2 as libc::c_int
        } else {
            0 as libc::c_int
        };
        memcpy(
            &mut *(*(*((*h).intra_border_backup)
                .as_mut_ptr()
                .offset(backup_dst as isize))
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize))
                .offset((mb_x * 16 as libc::c_int) as isize) as *mut pixel
                as *mut libc::c_void,
            ((*h).mb.pic.p_fdec[0 as libc::c_int as usize])
                .offset(backup_src_0 as isize) as *const libc::c_void,
            (16 as libc::c_int
                * ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int)
                as libc::c_ulong,
        );
        if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc
            == CHROMA_444 as libc::c_int
        {
            memcpy(
                &mut *(*(*((*h).intra_border_backup)
                    .as_mut_ptr()
                    .offset(backup_dst as isize))
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize))
                    .offset((mb_x * 16 as libc::c_int) as isize) as *mut pixel
                    as *mut libc::c_void,
                ((*h).mb.pic.p_fdec[1 as libc::c_int as usize])
                    .offset(backup_src_0 as isize) as *const libc::c_void,
                (16 as libc::c_int
                    * ::core::mem::size_of::<pixel>() as libc::c_ulong
                        as libc::c_int) as libc::c_ulong,
            );
            memcpy(
                &mut *(*(*((*h).intra_border_backup)
                    .as_mut_ptr()
                    .offset(backup_dst as isize))
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as isize))
                    .offset((mb_x * 16 as libc::c_int) as isize) as *mut pixel
                    as *mut libc::c_void,
                ((*h).mb.pic.p_fdec[2 as libc::c_int as usize])
                    .offset(backup_src_0 as isize) as *const libc::c_void,
                (16 as libc::c_int
                    * ::core::mem::size_of::<pixel>() as libc::c_ulong
                        as libc::c_int) as libc::c_ulong,
            );
        } else if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc != 0 {
            if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc
                == CHROMA_420 as libc::c_int
            {
                backup_src_0 = (if (*h).mb.b_interlaced != 0 {
                    3 as libc::c_int
                } else {
                    6 as libc::c_int
                }) * 32 as libc::c_int;
            }
            memcpy(
                &mut *(*(*((*h).intra_border_backup)
                    .as_mut_ptr()
                    .offset(backup_dst as isize))
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize))
                    .offset((mb_x * 16 as libc::c_int) as isize) as *mut pixel
                    as *mut libc::c_void,
                ((*h).mb.pic.p_fdec[1 as libc::c_int as usize])
                    .offset(backup_src_0 as isize) as *const libc::c_void,
                (8 as libc::c_int
                    * ::core::mem::size_of::<pixel>() as libc::c_ulong
                        as libc::c_int) as libc::c_ulong,
            );
            memcpy(
                &mut *(*(*((*h).intra_border_backup)
                    .as_mut_ptr()
                    .offset(backup_dst as isize))
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize))
                    .offset((mb_x * 16 as libc::c_int + 8 as libc::c_int) as isize)
                    as *mut pixel as *mut libc::c_void,
                ((*h).mb.pic.p_fdec[2 as libc::c_int as usize])
                    .offset(backup_src_0 as isize) as *const libc::c_void,
                (8 as libc::c_int
                    * ::core::mem::size_of::<pixel>() as libc::c_ulong
                        as libc::c_int) as libc::c_ulong,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_macroblock_cache_save(mut h: *mut x264_t) {
    let i_mb_xy: libc::c_int = (*h).mb.i_mb_xy;
    let i_mb_type: libc::c_int = x264_mb_type_fix[(*h).mb.i_type as usize]
        as libc::c_int;
    let s8x8: libc::c_int = (*h).mb.i_b8_stride;
    let s4x4: libc::c_int = (*h).mb.i_b4_stride;
    let i_mb_4x4: libc::c_int = (*h).mb.i_b4_xy;
    let i_mb_8x8: libc::c_int = (*h).mb.i_b8_xy;
    let mut i4x4: *mut int8_t = (*((*h).mb.intra4x4_pred_mode).offset(i_mb_xy as isize))
        .as_mut_ptr();
    let mut nnz: *mut uint8_t = (*((*h).mb.non_zero_count).offset(i_mb_xy as isize))
        .as_mut_ptr();
    if (*h).sh.b_mbaff != 0 {
        macroblock_backup_intra(h, (*h).mb.i_mb_x, (*h).mb.i_mb_y, 1 as libc::c_int);
        macroblock_store_pic(
            h,
            (*h).mb.i_mb_x,
            (*h).mb.i_mb_y,
            0 as libc::c_int,
            0 as libc::c_int,
            1 as libc::c_int,
        );
        if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as libc::c_int {
            macroblock_store_pic(
                h,
                (*h).mb.i_mb_x,
                (*h).mb.i_mb_y,
                1 as libc::c_int,
                0 as libc::c_int,
                1 as libc::c_int,
            );
            macroblock_store_pic(
                h,
                (*h).mb.i_mb_x,
                (*h).mb.i_mb_y,
                2 as libc::c_int,
                0 as libc::c_int,
                1 as libc::c_int,
            );
        } else if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc != 0 {
            macroblock_store_pic(
                h,
                (*h).mb.i_mb_x,
                (*h).mb.i_mb_y,
                1 as libc::c_int,
                1 as libc::c_int,
                1 as libc::c_int,
            );
        }
    } else {
        macroblock_backup_intra(h, (*h).mb.i_mb_x, (*h).mb.i_mb_y, 0 as libc::c_int);
        macroblock_store_pic(
            h,
            (*h).mb.i_mb_x,
            (*h).mb.i_mb_y,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
        );
        if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as libc::c_int {
            macroblock_store_pic(
                h,
                (*h).mb.i_mb_x,
                (*h).mb.i_mb_y,
                1 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
            );
            macroblock_store_pic(
                h,
                (*h).mb.i_mb_x,
                (*h).mb.i_mb_y,
                2 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
            );
        } else if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc != 0 {
            macroblock_store_pic(
                h,
                (*h).mb.i_mb_x,
                (*h).mb.i_mb_y,
                1 as libc::c_int,
                1 as libc::c_int,
                0 as libc::c_int,
            );
        }
    }
    x264_8_prefetch_fenc(h, (*h).fdec, (*h).mb.i_mb_x, (*h).mb.i_mb_y);
    *((*h).mb.type_0).offset(i_mb_xy as isize) = i_mb_type as int8_t;
    *((*h).mb.slice_table).offset(i_mb_xy as isize) = (*h).sh.i_first_mb;
    *((*h).mb.partition)
        .offset(
            i_mb_xy as isize,
        ) = (if i_mb_type == I_4x4 as libc::c_int || i_mb_type == I_8x8 as libc::c_int
        || i_mb_type == I_16x16 as libc::c_int || i_mb_type == I_PCM as libc::c_int
    {
        D_16x16 as libc::c_int
    } else {
        (*h).mb.i_partition
    }) as uint8_t;
    (*h).mb.i_mb_prev_xy = i_mb_xy;
    if i_mb_type == I_4x4 as libc::c_int {
        (*(&mut *i4x4.offset(0 as libc::c_int as isize) as *mut int8_t
            as *mut x264_union32_t))
            .i = (*(&mut *((*h).mb.cache.intra4x4_pred_mode)
            .as_mut_ptr()
            .offset(*x264_scan8.as_ptr().offset(10 as libc::c_int as isize) as isize)
            as *mut int8_t as *mut x264_union32_t))
            .i;
        (*(&mut *i4x4.offset(4 as libc::c_int as isize) as *mut int8_t
            as *mut x264_union32_t))
            .i = pack8to32(
            (*h)
                .mb
                .cache
                .intra4x4_pred_mode[x264_scan8[5 as libc::c_int as usize] as usize]
                as uint32_t,
            (*h)
                .mb
                .cache
                .intra4x4_pred_mode[x264_scan8[7 as libc::c_int as usize] as usize]
                as uint32_t,
            (*h)
                .mb
                .cache
                .intra4x4_pred_mode[x264_scan8[13 as libc::c_int as usize] as usize]
                as uint32_t,
            0 as libc::c_int as uint32_t,
        );
    } else if (*h).param.b_constrained_intra == 0
        || (i_mb_type == I_4x4 as libc::c_int || i_mb_type == I_8x8 as libc::c_int
            || i_mb_type == I_16x16 as libc::c_int || i_mb_type == I_PCM as libc::c_int)
    {
        (*(i4x4 as *mut x264_union64_t))
            .i = (I_PRED_4x4_DC as libc::c_int as libc::c_ulonglong)
            .wrapping_mul(0x101010101010101 as libc::c_ulonglong) as uint64_t;
    } else {
        (*(i4x4 as *mut x264_union64_t))
            .i = (-(1 as libc::c_int) as uint8_t as libc::c_ulonglong)
            .wrapping_mul(0x101010101010101 as libc::c_ulonglong) as uint64_t;
    }
    if i_mb_type == I_PCM as libc::c_int {
        *((*h).mb.qp).offset(i_mb_xy as isize) = 0 as libc::c_int as int8_t;
        (*h).mb.i_last_dqp = 0 as libc::c_int;
        (*h)
            .mb
            .i_cbp_chroma = if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc
            == CHROMA_444 as libc::c_int
        {
            0 as libc::c_int
        } else {
            2 as libc::c_int
        };
        (*h).mb.i_cbp_luma = 0xf as libc::c_int;
        *((*h).mb.cbp)
            .offset(
                i_mb_xy as isize,
            ) = (((*h).mb.i_cbp_chroma << 4 as libc::c_int) | (*h).mb.i_cbp_luma
            | 0x1700 as libc::c_int) as int16_t;
        (*h).mb.b_transform_8x8 = 0 as libc::c_int;
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < 48 as libc::c_int {
            (*h)
                .mb
                .cache
                .non_zero_count[x264_scan8[i as usize]
                as usize] = (if (*h).param.b_cabac != 0 {
                1 as libc::c_int
            } else {
                16 as libc::c_int
            }) as uint8_t;
            i += 1;
            i;
        }
    } else {
        if (*h).mb.i_type != I_16x16 as libc::c_int
            && (*h).mb.i_cbp_luma == 0 as libc::c_int
            && (*h).mb.i_cbp_chroma == 0 as libc::c_int
        {
            (*h).mb.i_qp = (*h).mb.i_last_qp;
        }
        *((*h).mb.qp).offset(i_mb_xy as isize) = (*h).mb.i_qp as int8_t;
        (*h).mb.i_last_dqp = (*h).mb.i_qp - (*h).mb.i_last_qp;
        (*h).mb.i_last_qp = (*h).mb.i_qp;
    }
    (*(&mut *nnz
        .offset((0 as libc::c_int + 0 as libc::c_int * 4 as libc::c_int) as isize)
        as *mut uint8_t as *mut x264_union32_t))
        .i = (*(&mut *((*h).mb.cache.non_zero_count)
        .as_mut_ptr()
        .offset(*x264_scan8.as_ptr().offset(0 as libc::c_int as isize) as isize)
        as *mut uint8_t as *mut x264_union32_t))
        .i;
    (*(&mut *nnz
        .offset((0 as libc::c_int + 1 as libc::c_int * 4 as libc::c_int) as isize)
        as *mut uint8_t as *mut x264_union32_t))
        .i = (*(&mut *((*h).mb.cache.non_zero_count)
        .as_mut_ptr()
        .offset(*x264_scan8.as_ptr().offset(2 as libc::c_int as isize) as isize)
        as *mut uint8_t as *mut x264_union32_t))
        .i;
    (*(&mut *nnz
        .offset((0 as libc::c_int + 2 as libc::c_int * 4 as libc::c_int) as isize)
        as *mut uint8_t as *mut x264_union32_t))
        .i = (*(&mut *((*h).mb.cache.non_zero_count)
        .as_mut_ptr()
        .offset(*x264_scan8.as_ptr().offset(8 as libc::c_int as isize) as isize)
        as *mut uint8_t as *mut x264_union32_t))
        .i;
    (*(&mut *nnz
        .offset((0 as libc::c_int + 3 as libc::c_int * 4 as libc::c_int) as isize)
        as *mut uint8_t as *mut x264_union32_t))
        .i = (*(&mut *((*h).mb.cache.non_zero_count)
        .as_mut_ptr()
        .offset(*x264_scan8.as_ptr().offset(10 as libc::c_int as isize) as isize)
        as *mut uint8_t as *mut x264_union32_t))
        .i;
    (*(&mut *nnz
        .offset((16 as libc::c_int + 0 as libc::c_int * 4 as libc::c_int) as isize)
        as *mut uint8_t as *mut x264_union32_t))
        .i = (*(&mut *((*h).mb.cache.non_zero_count)
        .as_mut_ptr()
        .offset(
            *x264_scan8.as_ptr().offset((16 as libc::c_int + 0 as libc::c_int) as isize)
                as isize,
        ) as *mut uint8_t as *mut x264_union32_t))
        .i;
    (*(&mut *nnz
        .offset((16 as libc::c_int + 1 as libc::c_int * 4 as libc::c_int) as isize)
        as *mut uint8_t as *mut x264_union32_t))
        .i = (*(&mut *((*h).mb.cache.non_zero_count)
        .as_mut_ptr()
        .offset(
            *x264_scan8.as_ptr().offset((16 as libc::c_int + 2 as libc::c_int) as isize)
                as isize,
        ) as *mut uint8_t as *mut x264_union32_t))
        .i;
    (*(&mut *nnz
        .offset((32 as libc::c_int + 0 as libc::c_int * 4 as libc::c_int) as isize)
        as *mut uint8_t as *mut x264_union32_t))
        .i = (*(&mut *((*h).mb.cache.non_zero_count)
        .as_mut_ptr()
        .offset(
            *x264_scan8.as_ptr().offset((32 as libc::c_int + 0 as libc::c_int) as isize)
                as isize,
        ) as *mut uint8_t as *mut x264_union32_t))
        .i;
    (*(&mut *nnz
        .offset((32 as libc::c_int + 1 as libc::c_int * 4 as libc::c_int) as isize)
        as *mut uint8_t as *mut x264_union32_t))
        .i = (*(&mut *((*h).mb.cache.non_zero_count)
        .as_mut_ptr()
        .offset(
            *x264_scan8.as_ptr().offset((32 as libc::c_int + 2 as libc::c_int) as isize)
                as isize,
        ) as *mut uint8_t as *mut x264_union32_t))
        .i;
    if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc >= CHROMA_422 as libc::c_int {
        (*(&mut *nnz
            .offset((16 as libc::c_int + 2 as libc::c_int * 4 as libc::c_int) as isize)
            as *mut uint8_t as *mut x264_union32_t))
            .i = (*(&mut *((*h).mb.cache.non_zero_count)
            .as_mut_ptr()
            .offset(
                *x264_scan8
                    .as_ptr()
                    .offset((16 as libc::c_int + 8 as libc::c_int) as isize) as isize,
            ) as *mut uint8_t as *mut x264_union32_t))
            .i;
        (*(&mut *nnz
            .offset((16 as libc::c_int + 3 as libc::c_int * 4 as libc::c_int) as isize)
            as *mut uint8_t as *mut x264_union32_t))
            .i = (*(&mut *((*h).mb.cache.non_zero_count)
            .as_mut_ptr()
            .offset(
                *x264_scan8
                    .as_ptr()
                    .offset((16 as libc::c_int + 10 as libc::c_int) as isize) as isize,
            ) as *mut uint8_t as *mut x264_union32_t))
            .i;
        (*(&mut *nnz
            .offset((32 as libc::c_int + 2 as libc::c_int * 4 as libc::c_int) as isize)
            as *mut uint8_t as *mut x264_union32_t))
            .i = (*(&mut *((*h).mb.cache.non_zero_count)
            .as_mut_ptr()
            .offset(
                *x264_scan8
                    .as_ptr()
                    .offset((32 as libc::c_int + 8 as libc::c_int) as isize) as isize,
            ) as *mut uint8_t as *mut x264_union32_t))
            .i;
        (*(&mut *nnz
            .offset((32 as libc::c_int + 3 as libc::c_int * 4 as libc::c_int) as isize)
            as *mut uint8_t as *mut x264_union32_t))
            .i = (*(&mut *((*h).mb.cache.non_zero_count)
            .as_mut_ptr()
            .offset(
                *x264_scan8
                    .as_ptr()
                    .offset((32 as libc::c_int + 10 as libc::c_int) as isize) as isize,
            ) as *mut uint8_t as *mut x264_union32_t))
            .i;
    }
    if (*h).mb.i_cbp_luma == 0 as libc::c_int && (*h).mb.i_type != I_8x8 as libc::c_int {
        (*h).mb.b_transform_8x8 = 0 as libc::c_int;
    }
    *((*h).mb.mb_transform_size)
        .offset(i_mb_xy as isize) = (*h).mb.b_transform_8x8 as int8_t;
    if (*h).sh.i_type != SLICE_TYPE_I as libc::c_int {
        let mut mv0: *mut [int16_t; 2] = &mut *(*((*h).mb.mv)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize))
            .offset(i_mb_4x4 as isize) as *mut [int16_t; 2];
        let mut ref0: *mut int8_t = &mut *(*((*h).mb.ref_0)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize))
            .offset(i_mb_8x8 as isize) as *mut int8_t;
        if !(i_mb_type == I_4x4 as libc::c_int || i_mb_type == I_8x8 as libc::c_int
            || i_mb_type == I_16x16 as libc::c_int || i_mb_type == I_PCM as libc::c_int)
        {
            *ref0
                .offset(
                    (0 as libc::c_int + 0 as libc::c_int * s8x8) as isize,
                ) = (*h)
                .mb
                .cache
                .ref_0[0 as libc::c_int
                as usize][x264_scan8[0 as libc::c_int as usize] as usize];
            *ref0
                .offset(
                    (1 as libc::c_int + 0 as libc::c_int * s8x8) as isize,
                ) = (*h)
                .mb
                .cache
                .ref_0[0 as libc::c_int
                as usize][x264_scan8[4 as libc::c_int as usize] as usize];
            *ref0
                .offset(
                    (0 as libc::c_int + 1 as libc::c_int * s8x8) as isize,
                ) = (*h)
                .mb
                .cache
                .ref_0[0 as libc::c_int
                as usize][x264_scan8[8 as libc::c_int as usize] as usize];
            *ref0
                .offset(
                    (1 as libc::c_int + 1 as libc::c_int * s8x8) as isize,
                ) = (*h)
                .mb
                .cache
                .ref_0[0 as libc::c_int
                as usize][x264_scan8[12 as libc::c_int as usize] as usize];
            (*(&mut *mv0.offset((0 as libc::c_int * s4x4) as isize) as *mut [int16_t; 2]
                as *mut x264_union128_sse_t))
                .i = (*(((*h)
                .mb
                .cache
                .mv[0 as libc::c_int
                as usize][(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                + 8 as libc::c_int * 0 as libc::c_int) as usize])
                .as_mut_ptr() as *mut x264_union128_sse_t))
                .i;
            (*(&mut *mv0.offset((1 as libc::c_int * s4x4) as isize) as *mut [int16_t; 2]
                as *mut x264_union128_sse_t))
                .i = (*(((*h)
                .mb
                .cache
                .mv[0 as libc::c_int
                as usize][(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                + 8 as libc::c_int * 1 as libc::c_int) as usize])
                .as_mut_ptr() as *mut x264_union128_sse_t))
                .i;
            (*(&mut *mv0.offset((2 as libc::c_int * s4x4) as isize) as *mut [int16_t; 2]
                as *mut x264_union128_sse_t))
                .i = (*(((*h)
                .mb
                .cache
                .mv[0 as libc::c_int
                as usize][(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                + 8 as libc::c_int * 2 as libc::c_int) as usize])
                .as_mut_ptr() as *mut x264_union128_sse_t))
                .i;
            (*(&mut *mv0.offset((3 as libc::c_int * s4x4) as isize) as *mut [int16_t; 2]
                as *mut x264_union128_sse_t))
                .i = (*(((*h)
                .mb
                .cache
                .mv[0 as libc::c_int
                as usize][(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                + 8 as libc::c_int * 3 as libc::c_int) as usize])
                .as_mut_ptr() as *mut x264_union128_sse_t))
                .i;
            if (*h).sh.i_type == SLICE_TYPE_B as libc::c_int {
                let mut mv1: *mut [int16_t; 2] = &mut *(*((*h).mb.mv)
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize))
                    .offset(i_mb_4x4 as isize) as *mut [int16_t; 2];
                let mut ref1: *mut int8_t = &mut *(*((*h).mb.ref_0)
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize))
                    .offset(i_mb_8x8 as isize) as *mut int8_t;
                *ref1
                    .offset(
                        (0 as libc::c_int + 0 as libc::c_int * s8x8) as isize,
                    ) = (*h)
                    .mb
                    .cache
                    .ref_0[1 as libc::c_int
                    as usize][x264_scan8[0 as libc::c_int as usize] as usize];
                *ref1
                    .offset(
                        (1 as libc::c_int + 0 as libc::c_int * s8x8) as isize,
                    ) = (*h)
                    .mb
                    .cache
                    .ref_0[1 as libc::c_int
                    as usize][x264_scan8[4 as libc::c_int as usize] as usize];
                *ref1
                    .offset(
                        (0 as libc::c_int + 1 as libc::c_int * s8x8) as isize,
                    ) = (*h)
                    .mb
                    .cache
                    .ref_0[1 as libc::c_int
                    as usize][x264_scan8[8 as libc::c_int as usize] as usize];
                *ref1
                    .offset(
                        (1 as libc::c_int + 1 as libc::c_int * s8x8) as isize,
                    ) = (*h)
                    .mb
                    .cache
                    .ref_0[1 as libc::c_int
                    as usize][x264_scan8[12 as libc::c_int as usize] as usize];
                (*(&mut *mv1.offset((0 as libc::c_int * s4x4) as isize)
                    as *mut [int16_t; 2] as *mut x264_union128_sse_t))
                    .i = (*(((*h)
                    .mb
                    .cache
                    .mv[1 as libc::c_int
                    as usize][(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                    + 8 as libc::c_int * 0 as libc::c_int) as usize])
                    .as_mut_ptr() as *mut x264_union128_sse_t))
                    .i;
                (*(&mut *mv1.offset((1 as libc::c_int * s4x4) as isize)
                    as *mut [int16_t; 2] as *mut x264_union128_sse_t))
                    .i = (*(((*h)
                    .mb
                    .cache
                    .mv[1 as libc::c_int
                    as usize][(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                    + 8 as libc::c_int * 1 as libc::c_int) as usize])
                    .as_mut_ptr() as *mut x264_union128_sse_t))
                    .i;
                (*(&mut *mv1.offset((2 as libc::c_int * s4x4) as isize)
                    as *mut [int16_t; 2] as *mut x264_union128_sse_t))
                    .i = (*(((*h)
                    .mb
                    .cache
                    .mv[1 as libc::c_int
                    as usize][(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                    + 8 as libc::c_int * 2 as libc::c_int) as usize])
                    .as_mut_ptr() as *mut x264_union128_sse_t))
                    .i;
                (*(&mut *mv1.offset((3 as libc::c_int * s4x4) as isize)
                    as *mut [int16_t; 2] as *mut x264_union128_sse_t))
                    .i = (*(((*h)
                    .mb
                    .cache
                    .mv[1 as libc::c_int
                    as usize][(x264_scan8[0 as libc::c_int as usize] as libc::c_int
                    + 8 as libc::c_int * 3 as libc::c_int) as usize])
                    .as_mut_ptr() as *mut x264_union128_sse_t))
                    .i;
            }
        } else {
            (*(&mut *ref0.offset((0 as libc::c_int * s8x8) as isize) as *mut int8_t
                as *mut x264_union16_t))
                .i = (-(1 as libc::c_int) as uint8_t as libc::c_int
                * 0x101 as libc::c_int) as uint16_t;
            (*(&mut *ref0.offset((1 as libc::c_int * s8x8) as isize) as *mut int8_t
                as *mut x264_union16_t))
                .i = (-(1 as libc::c_int) as uint8_t as libc::c_int
                * 0x101 as libc::c_int) as uint16_t;
            (*(&mut *mv0.offset((0 as libc::c_int * s4x4) as isize) as *mut [int16_t; 2]
                as *mut x264_union128_sse_t))
                .i = _mm_setr_ps(
                0 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
            (*(&mut *mv0.offset((1 as libc::c_int * s4x4) as isize) as *mut [int16_t; 2]
                as *mut x264_union128_sse_t))
                .i = _mm_setr_ps(
                0 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
            (*(&mut *mv0.offset((2 as libc::c_int * s4x4) as isize) as *mut [int16_t; 2]
                as *mut x264_union128_sse_t))
                .i = _mm_setr_ps(
                0 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
            (*(&mut *mv0.offset((3 as libc::c_int * s4x4) as isize) as *mut [int16_t; 2]
                as *mut x264_union128_sse_t))
                .i = _mm_setr_ps(
                0 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
            if (*h).sh.i_type == SLICE_TYPE_B as libc::c_int {
                let mut mv1_0: *mut [int16_t; 2] = &mut *(*((*h).mb.mv)
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize))
                    .offset(i_mb_4x4 as isize) as *mut [int16_t; 2];
                let mut ref1_0: *mut int8_t = &mut *(*((*h).mb.ref_0)
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize))
                    .offset(i_mb_8x8 as isize) as *mut int8_t;
                (*(&mut *ref1_0.offset((0 as libc::c_int * s8x8) as isize) as *mut int8_t
                    as *mut x264_union16_t))
                    .i = (-(1 as libc::c_int) as uint8_t as libc::c_int
                    * 0x101 as libc::c_int) as uint16_t;
                (*(&mut *ref1_0.offset((1 as libc::c_int * s8x8) as isize) as *mut int8_t
                    as *mut x264_union16_t))
                    .i = (-(1 as libc::c_int) as uint8_t as libc::c_int
                    * 0x101 as libc::c_int) as uint16_t;
                (*(&mut *mv1_0.offset((0 as libc::c_int * s4x4) as isize)
                    as *mut [int16_t; 2] as *mut x264_union128_sse_t))
                    .i = _mm_setr_ps(
                    0 as libc::c_int as libc::c_float,
                    0 as libc::c_int as libc::c_float,
                    0 as libc::c_int as libc::c_float,
                    0 as libc::c_int as libc::c_float,
                );
                (*(&mut *mv1_0.offset((1 as libc::c_int * s4x4) as isize)
                    as *mut [int16_t; 2] as *mut x264_union128_sse_t))
                    .i = _mm_setr_ps(
                    0 as libc::c_int as libc::c_float,
                    0 as libc::c_int as libc::c_float,
                    0 as libc::c_int as libc::c_float,
                    0 as libc::c_int as libc::c_float,
                );
                (*(&mut *mv1_0.offset((2 as libc::c_int * s4x4) as isize)
                    as *mut [int16_t; 2] as *mut x264_union128_sse_t))
                    .i = _mm_setr_ps(
                    0 as libc::c_int as libc::c_float,
                    0 as libc::c_int as libc::c_float,
                    0 as libc::c_int as libc::c_float,
                    0 as libc::c_int as libc::c_float,
                );
                (*(&mut *mv1_0.offset((3 as libc::c_int * s4x4) as isize)
                    as *mut [int16_t; 2] as *mut x264_union128_sse_t))
                    .i = _mm_setr_ps(
                    0 as libc::c_int as libc::c_float,
                    0 as libc::c_int as libc::c_float,
                    0 as libc::c_int as libc::c_float,
                    0 as libc::c_int as libc::c_float,
                );
            }
        }
    }
    if (*h).param.b_cabac != 0 {
        let mut mvd0: *mut [uint8_t; 2] = (*((*h).mb.mvd[0 as libc::c_int as usize])
            .offset(i_mb_xy as isize))
            .as_mut_ptr();
        if (i_mb_type == I_4x4 as libc::c_int || i_mb_type == I_8x8 as libc::c_int
            || i_mb_type == I_16x16 as libc::c_int || i_mb_type == I_PCM as libc::c_int)
            && i_mb_type != I_PCM as libc::c_int
        {
            *((*h).mb.chroma_pred_mode)
                .offset(
                    i_mb_xy as isize,
                ) = x264_mb_chroma_pred_mode_fix[(*h).mb.i_chroma_pred_mode as usize]
                as int8_t;
        } else {
            *((*h).mb.chroma_pred_mode)
                .offset(i_mb_xy as isize) = I_PRED_CHROMA_DC as libc::c_int as int8_t;
        }
        if (0x3ff30 as libc::c_int >> i_mb_type) & 1 as libc::c_int != 0 {
            (*((*mvd0.offset(0 as libc::c_int as isize)).as_mut_ptr()
                as *mut x264_union64_t))
                .i = (*(((*h)
                .mb
                .cache
                .mvd[0 as libc::c_int
                as usize][x264_scan8[10 as libc::c_int as usize] as usize])
                .as_mut_ptr() as *mut x264_union64_t))
                .i;
            (*((*mvd0.offset(4 as libc::c_int as isize)).as_mut_ptr()
                as *mut x264_union16_t))
                .i = (*(((*h)
                .mb
                .cache
                .mvd[0 as libc::c_int
                as usize][x264_scan8[5 as libc::c_int as usize] as usize])
                .as_mut_ptr() as *mut x264_union16_t))
                .i;
            (*((*mvd0.offset(5 as libc::c_int as isize)).as_mut_ptr()
                as *mut x264_union16_t))
                .i = (*(((*h)
                .mb
                .cache
                .mvd[0 as libc::c_int
                as usize][x264_scan8[7 as libc::c_int as usize] as usize])
                .as_mut_ptr() as *mut x264_union16_t))
                .i;
            (*((*mvd0.offset(6 as libc::c_int as isize)).as_mut_ptr()
                as *mut x264_union16_t))
                .i = (*(((*h)
                .mb
                .cache
                .mvd[0 as libc::c_int
                as usize][x264_scan8[13 as libc::c_int as usize] as usize])
                .as_mut_ptr() as *mut x264_union16_t))
                .i;
            if (*h).sh.i_type == SLICE_TYPE_B as libc::c_int {
                let mut mvd1: *mut [uint8_t; 2] = (*((*h)
                    .mb
                    .mvd[1 as libc::c_int as usize])
                    .offset(i_mb_xy as isize))
                    .as_mut_ptr();
                (*((*mvd1.offset(0 as libc::c_int as isize)).as_mut_ptr()
                    as *mut x264_union64_t))
                    .i = (*(((*h)
                    .mb
                    .cache
                    .mvd[1 as libc::c_int
                    as usize][x264_scan8[10 as libc::c_int as usize] as usize])
                    .as_mut_ptr() as *mut x264_union64_t))
                    .i;
                (*((*mvd1.offset(4 as libc::c_int as isize)).as_mut_ptr()
                    as *mut x264_union16_t))
                    .i = (*(((*h)
                    .mb
                    .cache
                    .mvd[1 as libc::c_int
                    as usize][x264_scan8[5 as libc::c_int as usize] as usize])
                    .as_mut_ptr() as *mut x264_union16_t))
                    .i;
                (*((*mvd1.offset(5 as libc::c_int as isize)).as_mut_ptr()
                    as *mut x264_union16_t))
                    .i = (*(((*h)
                    .mb
                    .cache
                    .mvd[1 as libc::c_int
                    as usize][x264_scan8[7 as libc::c_int as usize] as usize])
                    .as_mut_ptr() as *mut x264_union16_t))
                    .i;
                (*((*mvd1.offset(6 as libc::c_int as isize)).as_mut_ptr()
                    as *mut x264_union16_t))
                    .i = (*(((*h)
                    .mb
                    .cache
                    .mvd[1 as libc::c_int
                    as usize][x264_scan8[13 as libc::c_int as usize] as usize])
                    .as_mut_ptr() as *mut x264_union16_t))
                    .i;
            }
        } else {
            (*((*mvd0.offset(0 as libc::c_int as isize)).as_mut_ptr()
                as *mut x264_union128_sse_t))
                .i = _mm_setr_ps(
                0 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
            if (*h).sh.i_type == SLICE_TYPE_B as libc::c_int {
                let mut mvd1_0: *mut [uint8_t; 2] = (*((*h)
                    .mb
                    .mvd[1 as libc::c_int as usize])
                    .offset(i_mb_xy as isize))
                    .as_mut_ptr();
                (*((*mvd1_0.offset(0 as libc::c_int as isize)).as_mut_ptr()
                    as *mut x264_union128_sse_t))
                    .i = _mm_setr_ps(
                    0 as libc::c_int as libc::c_float,
                    0 as libc::c_int as libc::c_float,
                    0 as libc::c_int as libc::c_float,
                    0 as libc::c_int as libc::c_float,
                );
            }
        }
        if (*h).sh.i_type == SLICE_TYPE_B as libc::c_int {
            if i_mb_type == B_SKIP as libc::c_int || i_mb_type == B_DIRECT as libc::c_int
            {
                *((*h).mb.skipbp)
                    .offset(i_mb_xy as isize) = 0xf as libc::c_int as int8_t;
            } else if i_mb_type == B_8x8 as libc::c_int {
                let mut skipbp: libc::c_int = (((*h)
                    .mb
                    .i_sub_partition[0 as libc::c_int as usize] as libc::c_int
                    == D_DIRECT_8x8 as libc::c_int) as libc::c_int) << 0 as libc::c_int;
                skipbp
                    |= (((*h).mb.i_sub_partition[1 as libc::c_int as usize]
                        as libc::c_int == D_DIRECT_8x8 as libc::c_int) as libc::c_int)
                        << 1 as libc::c_int;
                skipbp
                    |= (((*h).mb.i_sub_partition[2 as libc::c_int as usize]
                        as libc::c_int == D_DIRECT_8x8 as libc::c_int) as libc::c_int)
                        << 2 as libc::c_int;
                skipbp
                    |= (((*h).mb.i_sub_partition[3 as libc::c_int as usize]
                        as libc::c_int == D_DIRECT_8x8 as libc::c_int) as libc::c_int)
                        << 3 as libc::c_int;
                *((*h).mb.skipbp).offset(i_mb_xy as isize) = skipbp as int8_t;
            } else {
                *((*h).mb.skipbp).offset(i_mb_xy as isize) = 0 as libc::c_int as int8_t;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_macroblock_bipred_init(mut h: *mut x264_t) {
    let mut mbfield: libc::c_int = 0 as libc::c_int;
    while mbfield <= (*h).sh.b_mbaff {
        let mut field: libc::c_int = 0 as libc::c_int;
        while field <= (*h).sh.b_mbaff {
            let mut i_ref0: libc::c_int = 0 as libc::c_int;
            while i_ref0 < (*h).i_ref[0 as libc::c_int as usize] << mbfield {
                let mut l0: *mut x264_frame_t = (*h)
                    .fref[0 as libc::c_int as usize][(i_ref0 >> mbfield) as usize];
                let mut poc0: libc::c_int = (*l0).i_poc
                    + mbfield
                        * (*l0)
                            .i_delta_poc[(field ^ i_ref0 & 1 as libc::c_int) as usize];
                let mut i_ref1: libc::c_int = 0 as libc::c_int;
                while i_ref1 < (*h).i_ref[1 as libc::c_int as usize] << mbfield {
                    let mut l1: *mut x264_frame_t = (*h)
                        .fref[1 as libc::c_int as usize][(i_ref1 >> mbfield) as usize];
                    let mut cur_poc: libc::c_int = (*(*h).fdec).i_poc
                        + mbfield * (*(*h).fdec).i_delta_poc[field as usize];
                    let mut poc1: libc::c_int = (*l1).i_poc
                        + mbfield
                            * (*l1)
                                .i_delta_poc[(field ^ i_ref1 & 1 as libc::c_int) as usize];
                    let mut td: libc::c_int = x264_clip3(
                        poc1 - poc0,
                        -(128 as libc::c_int),
                        127 as libc::c_int,
                    );
                    if td == 0 as libc::c_int {
                        (*h)
                            .mb
                            .dist_scale_factor_buf[mbfield
                            as usize][field
                            as usize][i_ref0
                            as usize][i_ref1 as usize] = 256 as libc::c_int as int16_t;
                        (*h)
                            .mb
                            .bipred_weight_buf[mbfield
                            as usize][field
                            as usize][i_ref0
                            as usize][i_ref1 as usize] = 32 as libc::c_int as int8_t;
                    } else {
                        let mut tb: libc::c_int = x264_clip3(
                            cur_poc - poc0,
                            -(128 as libc::c_int),
                            127 as libc::c_int,
                        );
                        let mut tx: libc::c_int = (16384 as libc::c_int
                            + (abs(td) >> 1 as libc::c_int)) / td;
                        let mut dist_scale_factor: libc::c_int = x264_clip3(
                            (tb * tx + 32 as libc::c_int) >> 6 as libc::c_int,
                            -(1024 as libc::c_int),
                            1023 as libc::c_int,
                        );
                        (*h)
                            .mb
                            .dist_scale_factor_buf[mbfield
                            as usize][field
                            as usize][i_ref0
                            as usize][i_ref1 as usize] = dist_scale_factor as int16_t;
                        dist_scale_factor >>= 2 as libc::c_int;
                        if (*h).param.analyse.b_weighted_bipred != 0
                            && dist_scale_factor >= -(64 as libc::c_int)
                            && dist_scale_factor <= 128 as libc::c_int
                        {
                            (*h)
                                .mb
                                .bipred_weight_buf[mbfield
                                as usize][field
                                as usize][i_ref0
                                as usize][i_ref1
                                as usize] = (64 as libc::c_int - dist_scale_factor)
                                as int8_t;
                            if dist_scale_factor >= -(63 as libc::c_int)
                                && dist_scale_factor <= 127 as libc::c_int
                            {} else {
                                __assert_fail(
                                    b"dist_scale_factor >= -63 && dist_scale_factor <= 127\0"
                                        as *const u8 as *const libc::c_char,
                                    b"common/macroblock.c\0" as *const u8
                                        as *const libc::c_char,
                                    1918 as libc::c_int as libc::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 45],
                                        &[libc::c_char; 45],
                                    >(b"void x264_8_macroblock_bipred_init(x264_t *)\0"))
                                        .as_ptr(),
                                );
                            }
                            'c_49035: {
                                if dist_scale_factor >= -(63 as libc::c_int)
                                    && dist_scale_factor <= 127 as libc::c_int
                                {} else {
                                    __assert_fail(
                                        b"dist_scale_factor >= -63 && dist_scale_factor <= 127\0"
                                            as *const u8 as *const libc::c_char,
                                        b"common/macroblock.c\0" as *const u8
                                            as *const libc::c_char,
                                        1918 as libc::c_int as libc::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 45],
                                            &[libc::c_char; 45],
                                        >(b"void x264_8_macroblock_bipred_init(x264_t *)\0"))
                                            .as_ptr(),
                                    );
                                }
                            };
                        } else {
                            (*h)
                                .mb
                                .bipred_weight_buf[mbfield
                                as usize][field
                                as usize][i_ref0
                                as usize][i_ref1 as usize] = 32 as libc::c_int as int8_t;
                        }
                    }
                    i_ref1 += 1;
                    i_ref1;
                }
                i_ref0 += 1;
                i_ref0;
            }
            field += 1;
            field;
        }
        mbfield += 1;
        mbfield;
    }
}
