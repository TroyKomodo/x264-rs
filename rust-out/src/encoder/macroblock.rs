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
    static x264_lambda2_tab: [libc::c_int; 82];
    static x264_dct4_weight2_tab: [uint32_t; 16];
    static x264_dct8_weight2_tab: [uint32_t; 64];
    fn x264_8_copy_column8(dst: *mut pixel, src: *mut pixel);
    fn x264_8_mb_mc(h: *mut x264_t);
    fn x264_8_mb_mc_8x8(h: *mut x264_t, i8: libc::c_int);
    fn x264_8_quant_luma_dc_trellis(
        h: *mut x264_t,
        dct: *mut dctcoef,
        i_quant_cat: libc::c_int,
        i_qp: libc::c_int,
        ctx_block_cat: libc::c_int,
        b_intra: libc::c_int,
        idx: libc::c_int,
    ) -> libc::c_int;
    fn x264_8_quant_chroma_dc_trellis(
        h: *mut x264_t,
        dct: *mut dctcoef,
        i_qp: libc::c_int,
        b_intra: libc::c_int,
        idx: libc::c_int,
    ) -> libc::c_int;
    fn x264_8_quant_4x4_trellis(
        h: *mut x264_t,
        dct: *mut dctcoef,
        i_quant_cat: libc::c_int,
        i_qp: libc::c_int,
        ctx_block_cat: libc::c_int,
        b_intra: libc::c_int,
        b_chroma: libc::c_int,
        idx: libc::c_int,
    ) -> libc::c_int;
    fn x264_8_quant_8x8_trellis(
        h: *mut x264_t,
        dct: *mut dctcoef,
        i_quant_cat: libc::c_int,
        i_qp: libc::c_int,
        ctx_block_cat: libc::c_int,
        b_intra: libc::c_int,
        b_chroma: libc::c_int,
        idx: libc::c_int,
    ) -> libc::c_int;
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
pub type intra_chroma_pred_e = libc::c_uint;
pub const I_PRED_CHROMA_DC_128: intra_chroma_pred_e = 6;
pub const I_PRED_CHROMA_DC_TOP: intra_chroma_pred_e = 5;
pub const I_PRED_CHROMA_DC_LEFT: intra_chroma_pred_e = 4;
pub const I_PRED_CHROMA_P: intra_chroma_pred_e = 3;
pub const I_PRED_CHROMA_V: intra_chroma_pred_e = 2;
pub const I_PRED_CHROMA_H: intra_chroma_pred_e = 1;
pub const I_PRED_CHROMA_DC: intra_chroma_pred_e = 0;
pub type intra16x16_pred_e = libc::c_uint;
pub const I_PRED_16x16_DC_128: intra16x16_pred_e = 6;
pub const I_PRED_16x16_DC_TOP: intra16x16_pred_e = 5;
pub const I_PRED_16x16_DC_LEFT: intra16x16_pred_e = 4;
pub const I_PRED_16x16_P: intra16x16_pred_e = 3;
pub const I_PRED_16x16_DC: intra16x16_pred_e = 2;
pub const I_PRED_16x16_H: intra16x16_pred_e = 1;
pub const I_PRED_16x16_V: intra16x16_pred_e = 0;
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
pub type intra8x8_pred_e = libc::c_uint;
pub const I_PRED_8x8_DC_128: intra8x8_pred_e = 11;
pub const I_PRED_8x8_DC_TOP: intra8x8_pred_e = 10;
pub const I_PRED_8x8_DC_LEFT: intra8x8_pred_e = 9;
pub const I_PRED_8x8_HU: intra8x8_pred_e = 8;
pub const I_PRED_8x8_VL: intra8x8_pred_e = 7;
pub const I_PRED_8x8_HD: intra8x8_pred_e = 6;
pub const I_PRED_8x8_VR: intra8x8_pred_e = 5;
pub const I_PRED_8x8_DDR: intra8x8_pred_e = 4;
pub const I_PRED_8x8_DDL: intra8x8_pred_e = 3;
pub const I_PRED_8x8_DC: intra8x8_pred_e = 2;
pub const I_PRED_8x8_H: intra8x8_pred_e = 1;
pub const I_PRED_8x8_V: intra8x8_pred_e = 0;
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
pub type cabac_ctx_block_cat_e = libc::c_uint;
pub const DCT_CHROMAV_8x8: cabac_ctx_block_cat_e = 13;
pub const DCT_CHROMAV_4x4: cabac_ctx_block_cat_e = 12;
pub const DCT_CHROMAV_AC: cabac_ctx_block_cat_e = 11;
pub const DCT_CHROMAV_DC: cabac_ctx_block_cat_e = 10;
pub const DCT_CHROMAU_8x8: cabac_ctx_block_cat_e = 9;
pub const DCT_CHROMAU_4x4: cabac_ctx_block_cat_e = 8;
pub const DCT_CHROMAU_AC: cabac_ctx_block_cat_e = 7;
pub const DCT_CHROMAU_DC: cabac_ctx_block_cat_e = 6;
pub const DCT_LUMA_8x8: cabac_ctx_block_cat_e = 5;
pub const DCT_CHROMA_AC: cabac_ctx_block_cat_e = 4;
pub const DCT_CHROMA_DC: cabac_ctx_block_cat_e = 3;
pub const DCT_LUMA_4x4: cabac_ctx_block_cat_e = 2;
pub const DCT_LUMA_AC: cabac_ctx_block_cat_e = 1;
pub const DCT_LUMA_DC: cabac_ctx_block_cat_e = 0;
#[inline(always)]
unsafe extern "C" fn x264_ctz_4bit(mut x: uint32_t) -> libc::c_int {
    static mut lut: [uint8_t; 16] = [
        4 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        1 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        2 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        1 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        3 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        1 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        2 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        1 as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
    ];
    lut[x as usize] as libc::c_int
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
static mut x264_pred_i4x4_neighbors: [uint8_t; 12] = [
    MB_TOP as libc::c_int as uint8_t,
    MB_LEFT as libc::c_int as uint8_t,
    (MB_LEFT as libc::c_int | MB_TOP as libc::c_int) as uint8_t,
    (MB_TOP as libc::c_int | MB_TOPRIGHT as libc::c_int) as uint8_t,
    (MB_LEFT as libc::c_int | MB_TOPLEFT as libc::c_int | MB_TOP as libc::c_int)
        as uint8_t,
    (MB_LEFT as libc::c_int | MB_TOPLEFT as libc::c_int | MB_TOP as libc::c_int)
        as uint8_t,
    (MB_LEFT as libc::c_int | MB_TOPLEFT as libc::c_int | MB_TOP as libc::c_int)
        as uint8_t,
    (MB_TOP as libc::c_int | MB_TOPRIGHT as libc::c_int) as uint8_t,
    MB_LEFT as libc::c_int as uint8_t,
    MB_LEFT as libc::c_int as uint8_t,
    MB_TOP as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
];
static mut block_idx_x: [uint8_t; 16] = [
    0 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
];
static mut block_idx_y: [uint8_t; 16] = [
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
];
static mut block_idx_xy_1d: [uint8_t; 16] = [
    0 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    7 as libc::c_int as uint8_t,
    8 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    12 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    14 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
];
static mut block_idx_yx_1d: [uint8_t; 16] = [
    0 as libc::c_int as uint8_t,
    4 as libc::c_int as uint8_t,
    1 as libc::c_int as uint8_t,
    5 as libc::c_int as uint8_t,
    8 as libc::c_int as uint8_t,
    12 as libc::c_int as uint8_t,
    9 as libc::c_int as uint8_t,
    13 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    3 as libc::c_int as uint8_t,
    7 as libc::c_int as uint8_t,
    10 as libc::c_int as uint8_t,
    14 as libc::c_int as uint8_t,
    11 as libc::c_int as uint8_t,
    15 as libc::c_int as uint8_t,
];
static mut block_idx_xy_fenc: [uint8_t; 16] = [
    (0 as libc::c_int * 4 as libc::c_int
        + 0 as libc::c_int * 4 as libc::c_int * 16 as libc::c_int) as uint8_t,
    (1 as libc::c_int * 4 as libc::c_int
        + 0 as libc::c_int * 4 as libc::c_int * 16 as libc::c_int) as uint8_t,
    (0 as libc::c_int * 4 as libc::c_int
        + 1 as libc::c_int * 4 as libc::c_int * 16 as libc::c_int) as uint8_t,
    (1 as libc::c_int * 4 as libc::c_int
        + 1 as libc::c_int * 4 as libc::c_int * 16 as libc::c_int) as uint8_t,
    (2 as libc::c_int * 4 as libc::c_int
        + 0 as libc::c_int * 4 as libc::c_int * 16 as libc::c_int) as uint8_t,
    (3 as libc::c_int * 4 as libc::c_int
        + 0 as libc::c_int * 4 as libc::c_int * 16 as libc::c_int) as uint8_t,
    (2 as libc::c_int * 4 as libc::c_int
        + 1 as libc::c_int * 4 as libc::c_int * 16 as libc::c_int) as uint8_t,
    (3 as libc::c_int * 4 as libc::c_int
        + 1 as libc::c_int * 4 as libc::c_int * 16 as libc::c_int) as uint8_t,
    (0 as libc::c_int * 4 as libc::c_int
        + 2 as libc::c_int * 4 as libc::c_int * 16 as libc::c_int) as uint8_t,
    (1 as libc::c_int * 4 as libc::c_int
        + 2 as libc::c_int * 4 as libc::c_int * 16 as libc::c_int) as uint8_t,
    (0 as libc::c_int * 4 as libc::c_int
        + 3 as libc::c_int * 4 as libc::c_int * 16 as libc::c_int) as uint8_t,
    (1 as libc::c_int * 4 as libc::c_int
        + 3 as libc::c_int * 4 as libc::c_int * 16 as libc::c_int) as uint8_t,
    (2 as libc::c_int * 4 as libc::c_int
        + 2 as libc::c_int * 4 as libc::c_int * 16 as libc::c_int) as uint8_t,
    (3 as libc::c_int * 4 as libc::c_int
        + 2 as libc::c_int * 4 as libc::c_int * 16 as libc::c_int) as uint8_t,
    (2 as libc::c_int * 4 as libc::c_int
        + 3 as libc::c_int * 4 as libc::c_int * 16 as libc::c_int) as uint8_t,
    (3 as libc::c_int * 4 as libc::c_int
        + 3 as libc::c_int * 4 as libc::c_int * 16 as libc::c_int) as uint8_t,
];
static mut block_idx_xy_fdec: [uint16_t; 16] = [
    (0 as libc::c_int * 4 as libc::c_int
        + 0 as libc::c_int * 4 as libc::c_int * 32 as libc::c_int) as uint16_t,
    (1 as libc::c_int * 4 as libc::c_int
        + 0 as libc::c_int * 4 as libc::c_int * 32 as libc::c_int) as uint16_t,
    (0 as libc::c_int * 4 as libc::c_int
        + 1 as libc::c_int * 4 as libc::c_int * 32 as libc::c_int) as uint16_t,
    (1 as libc::c_int * 4 as libc::c_int
        + 1 as libc::c_int * 4 as libc::c_int * 32 as libc::c_int) as uint16_t,
    (2 as libc::c_int * 4 as libc::c_int
        + 0 as libc::c_int * 4 as libc::c_int * 32 as libc::c_int) as uint16_t,
    (3 as libc::c_int * 4 as libc::c_int
        + 0 as libc::c_int * 4 as libc::c_int * 32 as libc::c_int) as uint16_t,
    (2 as libc::c_int * 4 as libc::c_int
        + 1 as libc::c_int * 4 as libc::c_int * 32 as libc::c_int) as uint16_t,
    (3 as libc::c_int * 4 as libc::c_int
        + 1 as libc::c_int * 4 as libc::c_int * 32 as libc::c_int) as uint16_t,
    (0 as libc::c_int * 4 as libc::c_int
        + 2 as libc::c_int * 4 as libc::c_int * 32 as libc::c_int) as uint16_t,
    (1 as libc::c_int * 4 as libc::c_int
        + 2 as libc::c_int * 4 as libc::c_int * 32 as libc::c_int) as uint16_t,
    (0 as libc::c_int * 4 as libc::c_int
        + 3 as libc::c_int * 4 as libc::c_int * 32 as libc::c_int) as uint16_t,
    (1 as libc::c_int * 4 as libc::c_int
        + 3 as libc::c_int * 4 as libc::c_int * 32 as libc::c_int) as uint16_t,
    (2 as libc::c_int * 4 as libc::c_int
        + 2 as libc::c_int * 4 as libc::c_int * 32 as libc::c_int) as uint16_t,
    (3 as libc::c_int * 4 as libc::c_int
        + 2 as libc::c_int * 4 as libc::c_int * 32 as libc::c_int) as uint16_t,
    (2 as libc::c_int * 4 as libc::c_int
        + 3 as libc::c_int * 4 as libc::c_int * 32 as libc::c_int) as uint16_t,
    (3 as libc::c_int * 4 as libc::c_int
        + 3 as libc::c_int * 4 as libc::c_int * 32 as libc::c_int) as uint16_t,
];
static mut ctx_cat_plane: [[uint8_t; 3]; 6] = [
    [
        DCT_LUMA_DC as libc::c_int as uint8_t,
        DCT_CHROMAU_DC as libc::c_int as uint8_t,
        DCT_CHROMAV_DC as libc::c_int as uint8_t,
    ],
    [
        DCT_LUMA_AC as libc::c_int as uint8_t,
        DCT_CHROMAU_AC as libc::c_int as uint8_t,
        DCT_CHROMAV_AC as libc::c_int as uint8_t,
    ],
    [
        DCT_LUMA_4x4 as libc::c_int as uint8_t,
        DCT_CHROMAU_4x4 as libc::c_int as uint8_t,
        DCT_CHROMAV_4x4 as libc::c_int as uint8_t,
    ],
    [0 as libc::c_int as uint8_t, 0, 0],
    [0 as libc::c_int as uint8_t, 0, 0],
    [
        DCT_LUMA_8x8 as libc::c_int as uint8_t,
        DCT_CHROMAU_8x8 as libc::c_int as uint8_t,
        DCT_CHROMAV_8x8 as libc::c_int as uint8_t,
    ],
];
#[inline(always)]
unsafe extern "C" fn x264_quant_4x4(
    mut h: *mut x264_t,
    mut dct: *mut dctcoef,
    mut i_qp: libc::c_int,
    mut ctx_block_cat: libc::c_int,
    mut b_intra: libc::c_int,
    mut p: libc::c_int,
    mut idx: libc::c_int,
) -> libc::c_int {
    let mut i_quant_cat: libc::c_int = if b_intra != 0 {
        if p != 0 { CQM_4IC as libc::c_int } else { CQM_4IY as libc::c_int }
    } else if p != 0 {
        CQM_4PC as libc::c_int
    } else {
        CQM_4PY as libc::c_int
    };
    if (*h).mb.b_noise_reduction != 0 {
        ((*h).quantf.denoise_dct)
            .expect(
                "non-null function pointer",
            )(
            dct,
            (*((*h).nr_residual_sum)
                .offset(
                    (0 as libc::c_int + (p != 0) as libc::c_int * 2 as libc::c_int)
                        as isize,
                ))
                .as_mut_ptr(),
            (*((*h).nr_offset)
                .offset(
                    (0 as libc::c_int + (p != 0) as libc::c_int * 2 as libc::c_int)
                        as isize,
                ))
                .as_mut_ptr(),
            16 as libc::c_int,
        );
    }
    if (*h).mb.b_trellis != 0 {
        x264_8_quant_4x4_trellis(
            h,
            dct,
            i_quant_cat,
            i_qp,
            ctx_block_cat,
            b_intra,
            (p != 0) as libc::c_int,
            idx + p * 16 as libc::c_int,
        )
    } else {
        ((*h).quantf.quant_4x4)
            .expect(
                "non-null function pointer",
            )(
            dct,
            (*((*h).quant4_mf[i_quant_cat as usize]).offset(i_qp as isize)).as_mut_ptr(),
            (*((*h).quant4_bias[i_quant_cat as usize]).offset(i_qp as isize))
                .as_mut_ptr(),
        )
    }
}
#[inline(always)]
unsafe extern "C" fn x264_quant_8x8(
    mut h: *mut x264_t,
    mut dct: *mut dctcoef,
    mut i_qp: libc::c_int,
    mut ctx_block_cat: libc::c_int,
    mut b_intra: libc::c_int,
    mut p: libc::c_int,
    mut idx: libc::c_int,
) -> libc::c_int {
    let mut i_quant_cat: libc::c_int = if b_intra != 0 {
        if p != 0 { CQM_8IC as libc::c_int } else { CQM_8IY as libc::c_int }
    } else if p != 0 {
        CQM_8PC as libc::c_int
    } else {
        CQM_8PY as libc::c_int
    };
    if (*h).mb.b_noise_reduction != 0 {
        ((*h).quantf.denoise_dct)
            .expect(
                "non-null function pointer",
            )(
            dct,
            (*((*h).nr_residual_sum)
                .offset(
                    (1 as libc::c_int + (p != 0) as libc::c_int * 2 as libc::c_int)
                        as isize,
                ))
                .as_mut_ptr(),
            (*((*h).nr_offset)
                .offset(
                    (1 as libc::c_int + (p != 0) as libc::c_int * 2 as libc::c_int)
                        as isize,
                ))
                .as_mut_ptr(),
            64 as libc::c_int,
        );
    }
    if (*h).mb.b_trellis != 0 {
        x264_8_quant_8x8_trellis(
            h,
            dct,
            i_quant_cat,
            i_qp,
            ctx_block_cat,
            b_intra,
            (p != 0) as libc::c_int,
            idx + p * 4 as libc::c_int,
        )
    } else {
        ((*h).quantf.quant_8x8)
            .expect(
                "non-null function pointer",
            )(
            dct,
            (*((*h).quant8_mf[i_quant_cat as usize]).offset(i_qp as isize)).as_mut_ptr(),
            (*((*h).quant8_bias[i_quant_cat as usize]).offset(i_qp as isize))
                .as_mut_ptr(),
        )
    }
}
#[inline(always)]
unsafe extern "C" fn x264_mb_encode_i4x4(
    mut h: *mut x264_t,
    mut p: libc::c_int,
    mut idx: libc::c_int,
    mut i_qp: libc::c_int,
    mut i_mode: libc::c_int,
    mut b_predict: libc::c_int,
) {
    let mut nz: libc::c_int = 0;
    let mut p_src: *mut pixel = &mut *(*((*h).mb.pic.p_fenc)
        .as_mut_ptr()
        .offset(p as isize))
        .offset(*block_idx_xy_fenc.as_ptr().offset(idx as isize) as isize) as *mut pixel;
    let mut p_dst: *mut pixel = &mut *(*((*h).mb.pic.p_fdec)
        .as_mut_ptr()
        .offset(p as isize))
        .offset(*block_idx_xy_fdec.as_ptr().offset(idx as isize) as isize) as *mut pixel;
    let mut dct4x4: [dctcoef; 16] = [0; 16];
    if b_predict != 0 {
        if (*h).mb.b_lossless != 0 {
            x264_8_predict_lossless_4x4(h, p_dst, p, idx, i_mode);
        } else {
            ((*h).predict_4x4[i_mode as usize])
                .expect("non-null function pointer")(p_dst);
        }
    }
    if (*h).mb.b_lossless != 0 {
        nz = ((*h).zigzagf.sub_4x4)
            .expect(
                "non-null function pointer",
            )(
            ((*h).dct.luma4x4[(p * 16 as libc::c_int + idx) as usize]).as_mut_ptr(),
            p_src,
            p_dst,
        );
        (*h)
            .mb
            .cache
            .non_zero_count[x264_scan8[(p * 16 as libc::c_int + idx) as usize]
            as usize] = nz as uint8_t;
        (*h).mb.i_cbp_luma |= nz << (idx >> 2 as libc::c_int);
        return;
    }
    ((*h).dctf.sub4x4_dct)
        .expect("non-null function pointer")(dct4x4.as_mut_ptr(), p_src, p_dst);
    nz = x264_quant_4x4(
        h,
        dct4x4.as_mut_ptr(),
        i_qp,
        ctx_cat_plane[DCT_LUMA_4x4 as libc::c_int as usize][p as usize] as libc::c_int,
        1 as libc::c_int,
        p,
        idx,
    );
    (*h)
        .mb
        .cache
        .non_zero_count[x264_scan8[(p * 16 as libc::c_int + idx) as usize]
        as usize] = nz as uint8_t;
    if nz != 0 {
        (*h).mb.i_cbp_luma |= (1 as libc::c_int) << (idx >> 2 as libc::c_int);
        ((*h).zigzagf.scan_4x4)
            .expect(
                "non-null function pointer",
            )(
            ((*h).dct.luma4x4[(p * 16 as libc::c_int + idx) as usize]).as_mut_ptr(),
            dct4x4.as_mut_ptr(),
        );
        ((*h).quantf.dequant_4x4)
            .expect(
                "non-null function pointer",
            )(
            dct4x4.as_mut_ptr(),
            (*h)
                .dequant4_mf[(if p != 0 {
                CQM_4IC as libc::c_int
            } else {
                CQM_4IY as libc::c_int
            }) as usize],
            i_qp,
        );
        ((*h).dctf.add4x4_idct)
            .expect("non-null function pointer")(p_dst, dct4x4.as_mut_ptr());
    }
}
#[inline(always)]
unsafe extern "C" fn x264_mb_encode_i8x8(
    mut h: *mut x264_t,
    mut p: libc::c_int,
    mut idx: libc::c_int,
    mut i_qp: libc::c_int,
    mut i_mode: libc::c_int,
    mut edge: *mut pixel,
    mut b_predict: libc::c_int,
) {
    let mut x: libc::c_int = idx & 1 as libc::c_int;
    let mut y: libc::c_int = idx >> 1 as libc::c_int;
    let mut nz: libc::c_int = 0;
    let mut p_src: *mut pixel = &mut *(*((*h).mb.pic.p_fenc)
        .as_mut_ptr()
        .offset(p as isize))
        .offset(
            (8 as libc::c_int * x + 8 as libc::c_int * y * 16 as libc::c_int) as isize,
        ) as *mut pixel;
    let mut p_dst: *mut pixel = &mut *(*((*h).mb.pic.p_fdec)
        .as_mut_ptr()
        .offset(p as isize))
        .offset(
            (8 as libc::c_int * x + 8 as libc::c_int * y * 32 as libc::c_int) as isize,
        ) as *mut pixel;
    let mut dct8x8: [dctcoef; 64] = [0; 64];
    let mut edge_buf: [pixel; 36] = [0; 36];
    if b_predict != 0 {
        if edge.is_null() {
            ((*h).predict_8x8_filter)
                .expect(
                    "non-null function pointer",
                )(
                p_dst,
                edge_buf.as_mut_ptr(),
                (*h).mb.i_neighbour8[idx as usize] as libc::c_int,
                x264_pred_i4x4_neighbors[i_mode as usize] as libc::c_int,
            );
            edge = edge_buf.as_mut_ptr();
        }
        if (*h).mb.b_lossless != 0 {
            x264_8_predict_lossless_8x8(h, p_dst, p, idx, i_mode, edge);
        } else {
            ((*h).predict_8x8[i_mode as usize])
                .expect("non-null function pointer")(p_dst, edge);
        }
    }
    if (*h).mb.b_lossless != 0 {
        nz = ((*h).zigzagf.sub_8x8)
            .expect(
                "non-null function pointer",
            )(
            ((*h).dct.luma8x8[(p * 4 as libc::c_int + idx) as usize]).as_mut_ptr(),
            p_src,
            p_dst,
        );
        (*(&mut *((*h).mb.cache.non_zero_count)
            .as_mut_ptr()
            .offset(
                (*x264_scan8
                    .as_ptr()
                    .offset((p * 16 as libc::c_int + idx * 4 as libc::c_int) as isize)
                    as libc::c_int + 0 as libc::c_int) as isize,
            ) as *mut uint8_t as *mut x264_union16_t))
            .i = (nz * 0x101 as libc::c_int) as uint16_t;
        (*(&mut *((*h).mb.cache.non_zero_count)
            .as_mut_ptr()
            .offset(
                (*x264_scan8
                    .as_ptr()
                    .offset((p * 16 as libc::c_int + idx * 4 as libc::c_int) as isize)
                    as libc::c_int + 8 as libc::c_int) as isize,
            ) as *mut uint8_t as *mut x264_union16_t))
            .i = (nz * 0x101 as libc::c_int) as uint16_t;
        (*h).mb.i_cbp_luma |= nz << idx;
        return;
    }
    ((*h).dctf.sub8x8_dct8)
        .expect("non-null function pointer")(dct8x8.as_mut_ptr(), p_src, p_dst);
    nz = x264_quant_8x8(
        h,
        dct8x8.as_mut_ptr(),
        i_qp,
        ctx_cat_plane[DCT_LUMA_8x8 as libc::c_int as usize][p as usize] as libc::c_int,
        1 as libc::c_int,
        p,
        idx,
    );
    if nz != 0 {
        (*h).mb.i_cbp_luma |= (1 as libc::c_int) << idx;
        ((*h).zigzagf.scan_8x8)
            .expect(
                "non-null function pointer",
            )(
            ((*h).dct.luma8x8[(p * 4 as libc::c_int + idx) as usize]).as_mut_ptr(),
            dct8x8.as_mut_ptr(),
        );
        ((*h).quantf.dequant_8x8)
            .expect(
                "non-null function pointer",
            )(
            dct8x8.as_mut_ptr(),
            (*h)
                .dequant8_mf[(if p != 0 {
                CQM_8IC as libc::c_int
            } else {
                CQM_8IY as libc::c_int
            }) as usize],
            i_qp,
        );
        ((*h).dctf.add8x8_idct8)
            .expect("non-null function pointer")(p_dst, dct8x8.as_mut_ptr());
        (*(&mut *((*h).mb.cache.non_zero_count)
            .as_mut_ptr()
            .offset(
                (*x264_scan8
                    .as_ptr()
                    .offset((p * 16 as libc::c_int + idx * 4 as libc::c_int) as isize)
                    as libc::c_int + 0 as libc::c_int) as isize,
            ) as *mut uint8_t as *mut x264_union16_t))
            .i = (1 as libc::c_int * 0x101 as libc::c_int) as uint16_t;
        (*(&mut *((*h).mb.cache.non_zero_count)
            .as_mut_ptr()
            .offset(
                (*x264_scan8
                    .as_ptr()
                    .offset((p * 16 as libc::c_int + idx * 4 as libc::c_int) as isize)
                    as libc::c_int + 8 as libc::c_int) as isize,
            ) as *mut uint8_t as *mut x264_union16_t))
            .i = (1 as libc::c_int * 0x101 as libc::c_int) as uint16_t;
    } else {
        (*(&mut *((*h).mb.cache.non_zero_count)
            .as_mut_ptr()
            .offset(
                (*x264_scan8
                    .as_ptr()
                    .offset((p * 16 as libc::c_int + idx * 4 as libc::c_int) as isize)
                    as libc::c_int + 0 as libc::c_int) as isize,
            ) as *mut uint8_t as *mut x264_union16_t))
            .i = (0 as libc::c_int * 0x101 as libc::c_int) as uint16_t;
        (*(&mut *((*h).mb.cache.non_zero_count)
            .as_mut_ptr()
            .offset(
                (*x264_scan8
                    .as_ptr()
                    .offset((p * 16 as libc::c_int + idx * 4 as libc::c_int) as isize)
                    as libc::c_int + 8 as libc::c_int) as isize,
            ) as *mut uint8_t as *mut x264_union16_t))
            .i = (0 as libc::c_int * 0x101 as libc::c_int) as uint16_t;
    };
}
#[inline]
unsafe extern "C" fn zigzag_scan_2x2_dc(mut level: *mut dctcoef, mut dct: *mut dctcoef) {
    *level
        .offset(
            0 as libc::c_int as isize,
        ) = *dct
        .offset((0 as libc::c_int * 2 as libc::c_int + 0 as libc::c_int) as isize);
    *level
        .offset(
            1 as libc::c_int as isize,
        ) = *dct
        .offset((1 as libc::c_int * 2 as libc::c_int + 0 as libc::c_int) as isize);
    *level
        .offset(
            2 as libc::c_int as isize,
        ) = *dct
        .offset((0 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as isize);
    *level
        .offset(
            3 as libc::c_int as isize,
        ) = *dct
        .offset((1 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as isize);
}
#[inline]
unsafe extern "C" fn zigzag_scan_2x4_dc(mut level: *mut dctcoef, mut dct: *mut dctcoef) {
    *level.offset(0 as libc::c_int as isize) = *dct.offset(0 as libc::c_int as isize);
    *level.offset(1 as libc::c_int as isize) = *dct.offset(2 as libc::c_int as isize);
    *level.offset(2 as libc::c_int as isize) = *dct.offset(1 as libc::c_int as isize);
    *level.offset(3 as libc::c_int as isize) = *dct.offset(4 as libc::c_int as isize);
    *level.offset(4 as libc::c_int as isize) = *dct.offset(6 as libc::c_int as isize);
    *level.offset(5 as libc::c_int as isize) = *dct.offset(3 as libc::c_int as isize);
    *level.offset(6 as libc::c_int as isize) = *dct.offset(5 as libc::c_int as isize);
    *level.offset(7 as libc::c_int as isize) = *dct.offset(7 as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn idct_dequant_2x2_dc(
    mut dct: *mut dctcoef,
    mut dct4x4: *mut [dctcoef; 16],
    mut dequant_mf: *mut [libc::c_int; 16],
    mut i_qp: libc::c_int,
) {
    let mut d0: libc::c_int = *dct.offset(0 as libc::c_int as isize) as libc::c_int
        + *dct.offset(1 as libc::c_int as isize) as libc::c_int;
    let mut d1: libc::c_int = *dct.offset(2 as libc::c_int as isize) as libc::c_int
        + *dct.offset(3 as libc::c_int as isize) as libc::c_int;
    let mut d2: libc::c_int = *dct.offset(0 as libc::c_int as isize) as libc::c_int
        - *dct.offset(1 as libc::c_int as isize) as libc::c_int;
    let mut d3: libc::c_int = *dct.offset(2 as libc::c_int as isize) as libc::c_int
        - *dct.offset(3 as libc::c_int as isize) as libc::c_int;
    let mut dmf: libc::c_int = (*dequant_mf
        .offset((i_qp % 6 as libc::c_int) as isize))[0 as libc::c_int as usize] << (i_qp / 6 as libc::c_int);
    (*dct4x4
        .offset(
            0 as libc::c_int as isize,
        ))[0 as libc::c_int as usize] = (((d0 + d1) * dmf) >> 5 as libc::c_int) as dctcoef;
    (*dct4x4
        .offset(
            1 as libc::c_int as isize,
        ))[0 as libc::c_int as usize] = (((d0 - d1) * dmf) >> 5 as libc::c_int) as dctcoef;
    (*dct4x4
        .offset(
            2 as libc::c_int as isize,
        ))[0 as libc::c_int as usize] = (((d2 + d3) * dmf) >> 5 as libc::c_int) as dctcoef;
    (*dct4x4
        .offset(
            3 as libc::c_int as isize,
        ))[0 as libc::c_int as usize] = (((d2 - d3) * dmf) >> 5 as libc::c_int) as dctcoef;
}
#[inline]
unsafe extern "C" fn idct_dequant_2x2_dconly(
    mut dct: *mut dctcoef,
    mut dequant_mf: *mut [libc::c_int; 16],
    mut i_qp: libc::c_int,
) {
    let mut d0: libc::c_int = *dct.offset(0 as libc::c_int as isize) as libc::c_int
        + *dct.offset(1 as libc::c_int as isize) as libc::c_int;
    let mut d1: libc::c_int = *dct.offset(2 as libc::c_int as isize) as libc::c_int
        + *dct.offset(3 as libc::c_int as isize) as libc::c_int;
    let mut d2: libc::c_int = *dct.offset(0 as libc::c_int as isize) as libc::c_int
        - *dct.offset(1 as libc::c_int as isize) as libc::c_int;
    let mut d3: libc::c_int = *dct.offset(2 as libc::c_int as isize) as libc::c_int
        - *dct.offset(3 as libc::c_int as isize) as libc::c_int;
    let mut dmf: libc::c_int = (*dequant_mf
        .offset((i_qp % 6 as libc::c_int) as isize))[0 as libc::c_int as usize] << (i_qp / 6 as libc::c_int);
    *dct
        .offset(
            0 as libc::c_int as isize,
        ) = (((d0 + d1) * dmf) >> 5 as libc::c_int) as dctcoef;
    *dct
        .offset(
            1 as libc::c_int as isize,
        ) = (((d0 - d1) * dmf) >> 5 as libc::c_int) as dctcoef;
    *dct
        .offset(
            2 as libc::c_int as isize,
        ) = (((d2 + d3) * dmf) >> 5 as libc::c_int) as dctcoef;
    *dct
        .offset(
            3 as libc::c_int as isize,
        ) = (((d2 - d3) * dmf) >> 5 as libc::c_int) as dctcoef;
}
#[inline]
unsafe extern "C" fn dct2x2dc(mut d: *mut dctcoef, mut dct4x4: *mut [dctcoef; 16]) {
    let mut d0: libc::c_int = (*dct4x4
        .offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] as libc::c_int
        + (*dct4x4.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize]
            as libc::c_int;
    let mut d1: libc::c_int = (*dct4x4
        .offset(2 as libc::c_int as isize))[0 as libc::c_int as usize] as libc::c_int
        + (*dct4x4.offset(3 as libc::c_int as isize))[0 as libc::c_int as usize]
            as libc::c_int;
    let mut d2: libc::c_int = (*dct4x4
        .offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] as libc::c_int
        - (*dct4x4.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize]
            as libc::c_int;
    let mut d3: libc::c_int = (*dct4x4
        .offset(2 as libc::c_int as isize))[0 as libc::c_int as usize] as libc::c_int
        - (*dct4x4.offset(3 as libc::c_int as isize))[0 as libc::c_int as usize]
            as libc::c_int;
    *d.offset(0 as libc::c_int as isize) = (d0 + d1) as dctcoef;
    *d.offset(2 as libc::c_int as isize) = (d2 + d3) as dctcoef;
    *d.offset(1 as libc::c_int as isize) = (d0 - d1) as dctcoef;
    *d.offset(3 as libc::c_int as isize) = (d2 - d3) as dctcoef;
    (*dct4x4
        .offset(
            0 as libc::c_int as isize,
        ))[0 as libc::c_int as usize] = 0 as libc::c_int as dctcoef;
    (*dct4x4
        .offset(
            1 as libc::c_int as isize,
        ))[0 as libc::c_int as usize] = 0 as libc::c_int as dctcoef;
    (*dct4x4
        .offset(
            2 as libc::c_int as isize,
        ))[0 as libc::c_int as usize] = 0 as libc::c_int as dctcoef;
    (*dct4x4
        .offset(
            3 as libc::c_int as isize,
        ))[0 as libc::c_int as usize] = 0 as libc::c_int as dctcoef;
}
#[inline(always)]
unsafe extern "C" fn array_non_zero(
    mut v: *mut dctcoef,
    mut i_count: libc::c_int,
) -> libc::c_int {
    if ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
        == 8 as libc::c_int as libc::c_ulong
    {
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < i_count {
            if (*(&mut *v.offset(i as isize) as *mut dctcoef as *mut x264_union64_t)).i
                != 0
            {
                return 1 as libc::c_int;
            }
            i = (i as libc::c_ulong)
                .wrapping_add(
                    (8 as libc::c_int as libc::c_ulong)
                        .wrapping_div(::core::mem::size_of::<dctcoef>() as libc::c_ulong),
                ) as libc::c_int as libc::c_int;
        }
    } else {
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < i_count {
            if (*(&mut *v.offset(i_0 as isize) as *mut dctcoef as *mut x264_union32_t)).i
                != 0
            {
                return 1 as libc::c_int;
            }
            i_0 = (i_0 as libc::c_ulong)
                .wrapping_add(
                    (4 as libc::c_int as libc::c_ulong)
                        .wrapping_div(::core::mem::size_of::<dctcoef>() as libc::c_ulong),
                ) as libc::c_int as libc::c_int;
        }
    }
    0 as libc::c_int
}
unsafe extern "C" fn mb_encode_i16x16(
    mut h: *mut x264_t,
    mut p: libc::c_int,
    mut i_qp: libc::c_int,
) {
    let mut p_src: *mut pixel = (*h).mb.pic.p_fenc[p as usize];
    let mut p_dst: *mut pixel = (*h).mb.pic.p_fdec[p as usize];
    let mut dct4x4: [[dctcoef; 16]; 16] = [[0; 16]; 16];
    let mut dct_dc4x4: [dctcoef; 16] = [0; 16];
    let mut nz: libc::c_int = 0;
    let mut block_cbp: libc::c_int = 0 as libc::c_int;
    let mut decimate_score: libc::c_int = if (*h).mb.b_dct_decimate != 0 {
        0 as libc::c_int
    } else {
        9 as libc::c_int
    };
    let mut i_quant_cat: libc::c_int = if p != 0 {
        CQM_4IC as libc::c_int
    } else {
        CQM_4IY as libc::c_int
    };
    let mut i_mode: libc::c_int = (*h).mb.i_intra16x16_pred_mode;
    if (*h).mb.b_lossless != 0 {
        x264_8_predict_lossless_16x16(h, p, i_mode);
    } else {
        ((*h).predict_16x16[i_mode as usize])
            .expect("non-null function pointer")((*h).mb.pic.p_fdec[p as usize]);
    }
    if (*h).mb.b_lossless != 0 {
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < 16 as libc::c_int {
            let mut oe: libc::c_int = block_idx_xy_fenc[i as usize] as libc::c_int;
            let mut od: libc::c_int = block_idx_xy_fdec[i as usize] as libc::c_int;
            nz = ((*h).zigzagf.sub_4x4ac)
                .expect(
                    "non-null function pointer",
                )(
                ((*h).dct.luma4x4[(16 as libc::c_int * p + i) as usize]).as_mut_ptr(),
                p_src.offset(oe as isize),
                p_dst.offset(od as isize),
                &mut *dct_dc4x4
                    .as_mut_ptr()
                    .offset(*block_idx_yx_1d.as_ptr().offset(i as isize) as isize),
            );
            (*h)
                .mb
                .cache
                .non_zero_count[x264_scan8[(16 as libc::c_int * p + i) as usize]
                as usize] = nz as uint8_t;
            block_cbp |= nz;
            i += 1;
            i;
        }
        (*h).mb.i_cbp_luma |= block_cbp * 0xf as libc::c_int;
        (*h)
            .mb
            .cache
            .non_zero_count[x264_scan8[(48 as libc::c_int + p) as usize]
            as usize] = array_non_zero(dct_dc4x4.as_mut_ptr(), 16 as libc::c_int)
            as uint8_t;
        ((*h).zigzagf.scan_4x4)
            .expect(
                "non-null function pointer",
            )(((*h).dct.luma16x16_dc[p as usize]).as_mut_ptr(), dct_dc4x4.as_mut_ptr());
        return;
    }
    (*(&mut *((*h).mb.cache.non_zero_count)
        .as_mut_ptr()
        .offset(
            (*x264_scan8.as_ptr().offset((16 as libc::c_int * p) as isize) as libc::c_int
                + 0 as libc::c_int * 8 as libc::c_int) as isize,
        ) as *mut uint8_t as *mut x264_union32_t))
        .i = 0 as libc::c_int as uint32_t;
    (*(&mut *((*h).mb.cache.non_zero_count)
        .as_mut_ptr()
        .offset(
            (*x264_scan8.as_ptr().offset((16 as libc::c_int * p) as isize) as libc::c_int
                + 1 as libc::c_int * 8 as libc::c_int) as isize,
        ) as *mut uint8_t as *mut x264_union32_t))
        .i = 0 as libc::c_int as uint32_t;
    (*(&mut *((*h).mb.cache.non_zero_count)
        .as_mut_ptr()
        .offset(
            (*x264_scan8.as_ptr().offset((16 as libc::c_int * p) as isize) as libc::c_int
                + 2 as libc::c_int * 8 as libc::c_int) as isize,
        ) as *mut uint8_t as *mut x264_union32_t))
        .i = 0 as libc::c_int as uint32_t;
    (*(&mut *((*h).mb.cache.non_zero_count)
        .as_mut_ptr()
        .offset(
            (*x264_scan8.as_ptr().offset((16 as libc::c_int * p) as isize) as libc::c_int
                + 3 as libc::c_int * 8 as libc::c_int) as isize,
        ) as *mut uint8_t as *mut x264_union32_t))
        .i = 0 as libc::c_int as uint32_t;
    ((*h).dctf.sub16x16_dct)
        .expect("non-null function pointer")(dct4x4.as_mut_ptr(), p_src, p_dst);
    if (*h).mb.b_noise_reduction != 0 {
        let mut idx: libc::c_int = 0 as libc::c_int;
        while idx < 16 as libc::c_int {
            ((*h).quantf.denoise_dct)
                .expect(
                    "non-null function pointer",
                )(
                (dct4x4[idx as usize]).as_mut_ptr(),
                (*((*h).nr_residual_sum).offset(0 as libc::c_int as isize)).as_mut_ptr(),
                (*((*h).nr_offset).offset(0 as libc::c_int as isize)).as_mut_ptr(),
                16 as libc::c_int,
            );
            idx += 1;
            idx;
        }
    }
    let mut idx_0: libc::c_int = 0 as libc::c_int;
    while idx_0 < 16 as libc::c_int {
        dct_dc4x4[block_idx_xy_1d[idx_0 as usize]
            as usize] = dct4x4[idx_0 as usize][0 as libc::c_int as usize];
        dct4x4[idx_0 as usize][0 as libc::c_int as usize] = 0 as libc::c_int as dctcoef;
        idx_0 += 1;
        idx_0;
    }
    if (*h).mb.b_trellis != 0 {
        let mut idx_1: libc::c_int = 0 as libc::c_int;
        while idx_1 < 16 as libc::c_int {
            if x264_8_quant_4x4_trellis(
                h,
                (dct4x4[idx_1 as usize]).as_mut_ptr(),
                i_quant_cat,
                i_qp,
                ctx_cat_plane[DCT_LUMA_AC as libc::c_int as usize][p as usize]
                    as libc::c_int,
                1 as libc::c_int,
                (p != 0) as libc::c_int,
                idx_1,
            ) != 0
            {
                block_cbp = 0xf as libc::c_int;
                ((*h).zigzagf.scan_4x4)
                    .expect(
                        "non-null function pointer",
                    )(
                    ((*h).dct.luma4x4[(16 as libc::c_int * p + idx_1) as usize])
                        .as_mut_ptr(),
                    (dct4x4[idx_1 as usize]).as_mut_ptr(),
                );
                ((*h).quantf.dequant_4x4)
                    .expect(
                        "non-null function pointer",
                    )(
                    (dct4x4[idx_1 as usize]).as_mut_ptr(),
                    (*h).dequant4_mf[i_quant_cat as usize],
                    i_qp,
                );
                if decimate_score < 6 as libc::c_int {
                    decimate_score
                        += ((*h).quantf.decimate_score15)
                            .expect(
                                "non-null function pointer",
                            )(
                            ((*h).dct.luma4x4[(16 as libc::c_int * p + idx_1) as usize])
                                .as_mut_ptr(),
                        );
                }
                (*h)
                    .mb
                    .cache
                    .non_zero_count[x264_scan8[(16 as libc::c_int * p + idx_1) as usize]
                    as usize] = 1 as libc::c_int as uint8_t;
            }
            idx_1 += 1;
            idx_1;
        }
    } else {
        let mut i8x8: libc::c_int = 0 as libc::c_int;
        while i8x8 < 4 as libc::c_int {
            nz = ((*h).quantf.quant_4x4x4)
                .expect(
                    "non-null function pointer",
                )(
                &mut *dct4x4.as_mut_ptr().offset((i8x8 * 4 as libc::c_int) as isize),
                (*((*h).quant4_mf[i_quant_cat as usize]).offset(i_qp as isize))
                    .as_mut_ptr(),
                (*((*h).quant4_bias[i_quant_cat as usize]).offset(i_qp as isize))
                    .as_mut_ptr(),
            );
            if nz != 0 {
                block_cbp = 0xf as libc::c_int;
                let mut idx_2: libc::c_int = i8x8 * 4 as libc::c_int;
                let mut msk: libc::c_int = nz;
                let mut skip: libc::c_int = 0;
                while msk != 0
                    && {
                        skip = x264_ctz_4bit(msk as uint32_t);
                        idx_2 += skip;
                        msk >>= skip + 1 as libc::c_int;
                        1 as libc::c_int != 0
                    }
                {
                    ((*h).zigzagf.scan_4x4)
                        .expect(
                            "non-null function pointer",
                        )(
                        ((*h).dct.luma4x4[(16 as libc::c_int * p + idx_2) as usize])
                            .as_mut_ptr(),
                        (dct4x4[idx_2 as usize]).as_mut_ptr(),
                    );
                    ((*h).quantf.dequant_4x4)
                        .expect(
                            "non-null function pointer",
                        )(
                        (dct4x4[idx_2 as usize]).as_mut_ptr(),
                        (*h).dequant4_mf[i_quant_cat as usize],
                        i_qp,
                    );
                    if decimate_score < 6 as libc::c_int {
                        decimate_score
                            += ((*h).quantf.decimate_score15)
                                .expect(
                                    "non-null function pointer",
                                )(
                                ((*h).dct.luma4x4[(16 as libc::c_int * p + idx_2) as usize])
                                    .as_mut_ptr(),
                            );
                    }
                    (*h)
                        .mb
                        .cache
                        .non_zero_count[x264_scan8[(16 as libc::c_int * p + idx_2)
                        as usize] as usize] = 1 as libc::c_int as uint8_t;
                    idx_2 += 1;
                    idx_2;
                }
            }
            i8x8 += 1;
            i8x8;
        }
    }
    if decimate_score < 6 as libc::c_int {
        (*(&mut *((*h).mb.cache.non_zero_count)
            .as_mut_ptr()
            .offset(
                (*x264_scan8.as_ptr().offset((16 as libc::c_int * p) as isize)
                    as libc::c_int + 0 as libc::c_int * 8 as libc::c_int) as isize,
            ) as *mut uint8_t as *mut x264_union32_t))
            .i = 0 as libc::c_int as uint32_t;
        (*(&mut *((*h).mb.cache.non_zero_count)
            .as_mut_ptr()
            .offset(
                (*x264_scan8.as_ptr().offset((16 as libc::c_int * p) as isize)
                    as libc::c_int + 1 as libc::c_int * 8 as libc::c_int) as isize,
            ) as *mut uint8_t as *mut x264_union32_t))
            .i = 0 as libc::c_int as uint32_t;
        (*(&mut *((*h).mb.cache.non_zero_count)
            .as_mut_ptr()
            .offset(
                (*x264_scan8.as_ptr().offset((16 as libc::c_int * p) as isize)
                    as libc::c_int + 2 as libc::c_int * 8 as libc::c_int) as isize,
            ) as *mut uint8_t as *mut x264_union32_t))
            .i = 0 as libc::c_int as uint32_t;
        (*(&mut *((*h).mb.cache.non_zero_count)
            .as_mut_ptr()
            .offset(
                (*x264_scan8.as_ptr().offset((16 as libc::c_int * p) as isize)
                    as libc::c_int + 3 as libc::c_int * 8 as libc::c_int) as isize,
            ) as *mut uint8_t as *mut x264_union32_t))
            .i = 0 as libc::c_int as uint32_t;
        block_cbp = 0 as libc::c_int;
    } else {
        (*h).mb.i_cbp_luma |= block_cbp;
    }
    ((*h).dctf.dct4x4dc).expect("non-null function pointer")(dct_dc4x4.as_mut_ptr());
    if (*h).mb.b_trellis != 0 {
        nz = x264_8_quant_luma_dc_trellis(
            h,
            dct_dc4x4.as_mut_ptr(),
            i_quant_cat,
            i_qp,
            ctx_cat_plane[DCT_LUMA_DC as libc::c_int as usize][p as usize]
                as libc::c_int,
            1 as libc::c_int,
            48 as libc::c_int + p,
        );
    } else {
        nz = ((*h).quantf.quant_4x4_dc)
            .expect(
                "non-null function pointer",
            )(
            dct_dc4x4.as_mut_ptr(),
            (*((*h).quant4_mf[i_quant_cat as usize])
                .offset(i_qp as isize))[0 as libc::c_int as usize] as libc::c_int
                >> 1 as libc::c_int,
            ((*((*h).quant4_bias[i_quant_cat as usize])
                .offset(i_qp as isize))[0 as libc::c_int as usize] as libc::c_int)
                << 1 as libc::c_int,
        );
    }
    (*h)
        .mb
        .cache
        .non_zero_count[x264_scan8[(48 as libc::c_int + p) as usize]
        as usize] = nz as uint8_t;
    if nz != 0 {
        ((*h).zigzagf.scan_4x4)
            .expect(
                "non-null function pointer",
            )(((*h).dct.luma16x16_dc[p as usize]).as_mut_ptr(), dct_dc4x4.as_mut_ptr());
        ((*h).dctf.idct4x4dc)
            .expect("non-null function pointer")(dct_dc4x4.as_mut_ptr());
        ((*h).quantf.dequant_4x4_dc)
            .expect(
                "non-null function pointer",
            )(dct_dc4x4.as_mut_ptr(), (*h).dequant4_mf[i_quant_cat as usize], i_qp);
        if block_cbp != 0 {
            let mut i_0: libc::c_int = 0 as libc::c_int;
            while i_0 < 16 as libc::c_int {
                dct4x4[i_0
                    as usize][0 as libc::c_int
                    as usize] = dct_dc4x4[block_idx_xy_1d[i_0 as usize] as usize];
                i_0 += 1;
                i_0;
            }
        }
    }
    if block_cbp != 0 {
        ((*h).dctf.add16x16_idct)
            .expect("non-null function pointer")(p_dst, dct4x4.as_mut_ptr());
    } else if nz != 0 {
        ((*h).dctf.add16x16_idct_dc)
            .expect("non-null function pointer")(p_dst, dct_dc4x4.as_mut_ptr());
    }
}
#[inline(always)]
unsafe extern "C" fn mb_optimize_chroma_dc(
    mut h: *mut x264_t,
    mut dct_dc: *mut dctcoef,
    mut dequant_mf: *mut [libc::c_int; 16],
    mut i_qp: libc::c_int,
    mut chroma422: libc::c_int,
) -> libc::c_int {
    let mut dmf: libc::c_int = (*dequant_mf
        .offset((i_qp % 6 as libc::c_int) as isize))[0 as libc::c_int as usize] << (i_qp / 6 as libc::c_int);
    if dmf > 32 as libc::c_int * 64 as libc::c_int {
        return 1 as libc::c_int;
    }
    if chroma422 != 0 {
        ((*h).quantf.optimize_chroma_2x4_dc)
            .expect("non-null function pointer")(dct_dc, dmf)
    } else {
        ((*h).quantf.optimize_chroma_2x2_dc)
            .expect("non-null function pointer")(dct_dc, dmf)
    }
}
#[inline(always)]
unsafe extern "C" fn mb_encode_chroma_internal(
    mut h: *mut x264_t,
    mut b_inter: libc::c_int,
    mut i_qp: libc::c_int,
    mut chroma422: libc::c_int,
) {
    let mut nz: libc::c_int = 0;
    let mut nz_dc: libc::c_int = 0;
    let mut b_decimate: libc::c_int = (b_inter != 0 && (*h).mb.b_dct_decimate != 0)
        as libc::c_int;
    let mut dequant_mf: *mut [libc::c_int; 16] = (*h)
        .dequant4_mf[(CQM_4IC as libc::c_int + b_inter) as usize];
    let mut dct_dc: [dctcoef; 8] = [0; 8];
    (*h).mb.i_cbp_chroma = 0 as libc::c_int;
    let fresh0 = &mut (*((*h).nr_count).offset(2 as libc::c_int as isize));
    *fresh0 = (*fresh0)
        .wrapping_add(((*h).mb.b_noise_reduction * 4 as libc::c_int) as uint32_t);
    (*(&mut *((*h).mb.cache.non_zero_count)
        .as_mut_ptr()
        .offset(*x264_scan8.as_ptr().offset(16 as libc::c_int as isize) as isize)
        as *mut uint8_t as *mut x264_union16_t))
        .i = 0 as libc::c_int as uint16_t;
    (*(&mut *((*h).mb.cache.non_zero_count)
        .as_mut_ptr()
        .offset(*x264_scan8.as_ptr().offset(18 as libc::c_int as isize) as isize)
        as *mut uint8_t as *mut x264_union16_t))
        .i = 0 as libc::c_int as uint16_t;
    (*(&mut *((*h).mb.cache.non_zero_count)
        .as_mut_ptr()
        .offset(*x264_scan8.as_ptr().offset(32 as libc::c_int as isize) as isize)
        as *mut uint8_t as *mut x264_union16_t))
        .i = 0 as libc::c_int as uint16_t;
    (*(&mut *((*h).mb.cache.non_zero_count)
        .as_mut_ptr()
        .offset(*x264_scan8.as_ptr().offset(34 as libc::c_int as isize) as isize)
        as *mut uint8_t as *mut x264_union16_t))
        .i = 0 as libc::c_int as uint16_t;
    if chroma422 != 0 {
        (*(&mut *((*h).mb.cache.non_zero_count)
            .as_mut_ptr()
            .offset(*x264_scan8.as_ptr().offset(24 as libc::c_int as isize) as isize)
            as *mut uint8_t as *mut x264_union16_t))
            .i = 0 as libc::c_int as uint16_t;
        (*(&mut *((*h).mb.cache.non_zero_count)
            .as_mut_ptr()
            .offset(*x264_scan8.as_ptr().offset(26 as libc::c_int as isize) as isize)
            as *mut uint8_t as *mut x264_union16_t))
            .i = 0 as libc::c_int as uint16_t;
        (*(&mut *((*h).mb.cache.non_zero_count)
            .as_mut_ptr()
            .offset(*x264_scan8.as_ptr().offset(40 as libc::c_int as isize) as isize)
            as *mut uint8_t as *mut x264_union16_t))
            .i = 0 as libc::c_int as uint16_t;
        (*(&mut *((*h).mb.cache.non_zero_count)
            .as_mut_ptr()
            .offset(*x264_scan8.as_ptr().offset(42 as libc::c_int as isize) as isize)
            as *mut uint8_t as *mut x264_union16_t))
            .i = 0 as libc::c_int as uint16_t;
    }
    if b_decimate != 0
        && i_qp
            >= (if (*h).mb.b_trellis != 0 {
                12 as libc::c_int
            } else {
                18 as libc::c_int
            }) && (*h).mb.b_noise_reduction == 0
    {
        let mut thresh: libc::c_int = if chroma422 != 0 {
            (x264_lambda2_tab[i_qp as usize] + 16 as libc::c_int) >> 5 as libc::c_int
        } else {
            (x264_lambda2_tab[i_qp as usize] + 32 as libc::c_int) >> 6 as libc::c_int
        };
        let mut ssd: [libc::c_int; 2] = [0; 2];
        let mut chromapix: libc::c_int = if chroma422 != 0 {
            PIXEL_8x16 as libc::c_int
        } else {
            PIXEL_8x8 as libc::c_int
        };
        if ((*h).pixf.var2[chromapix as usize])
            .expect(
                "non-null function pointer",
            )(
            (*h).mb.pic.p_fenc[1 as libc::c_int as usize],
            (*h).mb.pic.p_fdec[1 as libc::c_int as usize],
            ssd.as_mut_ptr(),
        ) < thresh * 4 as libc::c_int
        {
            (*h)
                .mb
                .cache
                .non_zero_count[x264_scan8[(49 as libc::c_int + 0 as libc::c_int)
                as usize] as usize] = 0 as libc::c_int as uint8_t;
            (*h)
                .mb
                .cache
                .non_zero_count[x264_scan8[(49 as libc::c_int + 1 as libc::c_int)
                as usize] as usize] = 0 as libc::c_int as uint8_t;
            let mut ch: libc::c_int = 0 as libc::c_int;
            while ch < 2 as libc::c_int {
                if ssd[ch as usize] > thresh {
                    let mut p_src: *mut pixel = (*h)
                        .mb
                        .pic
                        .p_fenc[(1 as libc::c_int + ch) as usize];
                    let mut p_dst: *mut pixel = (*h)
                        .mb
                        .pic
                        .p_fdec[(1 as libc::c_int + ch) as usize];
                    if chroma422 != 0 {
                        ((*h).dctf.sub8x16_dct_dc)
                            .expect(
                                "non-null function pointer",
                            )(dct_dc.as_mut_ptr(), p_src, p_dst);
                    } else {
                        ((*h).dctf.sub8x8_dct_dc)
                            .expect(
                                "non-null function pointer",
                            )(dct_dc.as_mut_ptr(), p_src, p_dst);
                    }
                    if (*h).mb.b_trellis != 0 {
                        nz_dc = x264_8_quant_chroma_dc_trellis(
                            h,
                            dct_dc.as_mut_ptr(),
                            i_qp + 3 as libc::c_int * chroma422,
                            (b_inter == 0) as libc::c_int,
                            49 as libc::c_int + ch,
                        );
                    } else {
                        nz_dc = 0 as libc::c_int;
                        let mut i: libc::c_int = 0 as libc::c_int;
                        while i <= chroma422 {
                            nz_dc
                                |= ((*h).quantf.quant_2x2_dc)
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    &mut *dct_dc
                                        .as_mut_ptr()
                                        .offset((4 as libc::c_int * i) as isize),
                                    (*((*h)
                                        .quant4_mf[(CQM_4IC as libc::c_int + b_inter) as usize])
                                        .offset(
                                            (i_qp + 3 as libc::c_int * chroma422) as isize,
                                        ))[0 as libc::c_int as usize] as libc::c_int
                                        >> 1 as libc::c_int,
                                    ((*((*h)
                                        .quant4_bias[(CQM_4IC as libc::c_int + b_inter) as usize])
                                        .offset(
                                            (i_qp + 3 as libc::c_int * chroma422) as isize,
                                        ))[0 as libc::c_int as usize] as libc::c_int)
                                        << 1 as libc::c_int,
                                );
                            i += 1;
                            i;
                        }
                    }
                    if nz_dc != 0 && mb_optimize_chroma_dc(
                            h,
                            dct_dc.as_mut_ptr(),
                            dequant_mf,
                            i_qp + 3 as libc::c_int * chroma422,
                            chroma422,
                        ) != 0 {
                        (*h)
                            .mb
                            .cache
                            .non_zero_count[x264_scan8[(49 as libc::c_int + ch)
                            as usize] as usize] = 1 as libc::c_int as uint8_t;
                        if chroma422 != 0 {
                            zigzag_scan_2x4_dc(
                                ((*h).dct.chroma_dc[ch as usize]).as_mut_ptr(),
                                dct_dc.as_mut_ptr(),
                            );
                            ((*h).quantf.idct_dequant_2x4_dconly)
                                .expect(
                                    "non-null function pointer",
                                )(dct_dc.as_mut_ptr(), dequant_mf, i_qp + 3 as libc::c_int);
                        } else {
                            zigzag_scan_2x2_dc(
                                ((*h).dct.chroma_dc[ch as usize]).as_mut_ptr(),
                                dct_dc.as_mut_ptr(),
                            );
                            idct_dequant_2x2_dconly(
                                dct_dc.as_mut_ptr(),
                                dequant_mf,
                                i_qp,
                            );
                        }
                        let mut i_0: libc::c_int = 0 as libc::c_int;
                        while i_0 <= chroma422 {
                            ((*h).dctf.add8x8_idct_dc)
                                .expect(
                                    "non-null function pointer",
                                )(
                                p_dst
                                    .offset(
                                        (8 as libc::c_int * i_0 * 32 as libc::c_int) as isize,
                                    ),
                                &mut *dct_dc
                                    .as_mut_ptr()
                                    .offset((4 as libc::c_int * i_0) as isize),
                            );
                            i_0 += 1;
                            i_0;
                        }
                        (*h).mb.i_cbp_chroma = 1 as libc::c_int;
                    }
                }
                ch += 1;
                ch;
            }
            return;
        }
    }
    let mut ch_0: libc::c_int = 0 as libc::c_int;
    while ch_0 < 2 as libc::c_int {
        let mut p_src_0: *mut pixel = (*h)
            .mb
            .pic
            .p_fenc[(1 as libc::c_int + ch_0) as usize];
        let mut p_dst_0: *mut pixel = (*h)
            .mb
            .pic
            .p_fdec[(1 as libc::c_int + ch_0) as usize];
        let mut i_decimate_score: libc::c_int = if b_decimate != 0 {
            0 as libc::c_int
        } else {
            7 as libc::c_int
        };
        let mut nz_ac: libc::c_int = 0 as libc::c_int;
        let mut dct4x4: [[dctcoef; 16]; 8] = [[0; 16]; 8];
        if (*h).mb.b_lossless != 0 {
            static mut chroma422_scan: [uint8_t; 8] = [
                0 as libc::c_int as uint8_t,
                2 as libc::c_int as uint8_t,
                1 as libc::c_int as uint8_t,
                5 as libc::c_int as uint8_t,
                3 as libc::c_int as uint8_t,
                6 as libc::c_int as uint8_t,
                4 as libc::c_int as uint8_t,
                7 as libc::c_int as uint8_t,
            ];
            let mut i_1: libc::c_int = 0 as libc::c_int;
            while i_1
                < (if chroma422 != 0 { 8 as libc::c_int } else { 4 as libc::c_int })
            {
                let mut oe: libc::c_int = 4 as libc::c_int * (i_1 & 1 as libc::c_int)
                    + 4 as libc::c_int * (i_1 >> 1 as libc::c_int) * 16 as libc::c_int;
                let mut od: libc::c_int = 4 as libc::c_int * (i_1 & 1 as libc::c_int)
                    + 4 as libc::c_int * (i_1 >> 1 as libc::c_int) * 32 as libc::c_int;
                nz = ((*h).zigzagf.sub_4x4ac)
                    .expect(
                        "non-null function pointer",
                    )(
                    ((*h)
                        .dct
                        .luma4x4[(16 as libc::c_int + i_1
                        + (if chroma422 != 0 {
                            i_1 & 4 as libc::c_int
                        } else {
                            0 as libc::c_int
                        }) + ch_0 * 16 as libc::c_int) as usize])
                        .as_mut_ptr(),
                    p_src_0.offset(oe as isize),
                    p_dst_0.offset(od as isize),
                    &mut *(*((*h).dct.chroma_dc).as_mut_ptr().offset(ch_0 as isize))
                        .as_mut_ptr()
                        .offset(
                            (if chroma422 != 0 {
                                *chroma422_scan.as_ptr().offset(i_1 as isize) as libc::c_int
                            } else {
                                i_1
                            }) as isize,
                        ),
                );
                (*h)
                    .mb
                    .cache
                    .non_zero_count[x264_scan8[(16 as libc::c_int + i_1
                    + (if chroma422 != 0 {
                        i_1 & 4 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) + ch_0 * 16 as libc::c_int) as usize] as usize] = nz as uint8_t;
                (*h).mb.i_cbp_chroma |= nz;
                i_1 += 1;
                i_1;
            }
            (*h)
                .mb
                .cache
                .non_zero_count[x264_scan8[(49 as libc::c_int + ch_0) as usize]
                as usize] = array_non_zero(
                ((*h).dct.chroma_dc[ch_0 as usize]).as_mut_ptr(),
                if chroma422 != 0 { 8 as libc::c_int } else { 4 as libc::c_int },
            ) as uint8_t;
        } else {
            let mut i_2: libc::c_int = 0 as libc::c_int;
            while i_2 <= chroma422 {
                ((*h).dctf.sub8x8_dct)
                    .expect(
                        "non-null function pointer",
                    )(
                    &mut *dct4x4.as_mut_ptr().offset((4 as libc::c_int * i_2) as isize),
                    p_src_0
                        .offset((8 as libc::c_int * i_2 * 16 as libc::c_int) as isize),
                    p_dst_0.offset((8 as libc::c_int * i_2 * 32 as libc::c_int) as isize),
                );
                i_2 += 1;
                i_2;
            }
            if (*h).mb.b_noise_reduction != 0 {
                let mut i_3: libc::c_int = 0 as libc::c_int;
                while i_3
                    < (if chroma422 != 0 { 8 as libc::c_int } else { 4 as libc::c_int })
                {
                    ((*h).quantf.denoise_dct)
                        .expect(
                            "non-null function pointer",
                        )(
                        (dct4x4[i_3 as usize]).as_mut_ptr(),
                        (*((*h).nr_residual_sum).offset(2 as libc::c_int as isize))
                            .as_mut_ptr(),
                        (*((*h).nr_offset).offset(2 as libc::c_int as isize))
                            .as_mut_ptr(),
                        16 as libc::c_int,
                    );
                    i_3 += 1;
                    i_3;
                }
            }
            if chroma422 != 0 {
                ((*h).dctf.dct2x4dc)
                    .expect(
                        "non-null function pointer",
                    )(dct_dc.as_mut_ptr(), dct4x4.as_mut_ptr());
            } else {
                dct2x2dc(dct_dc.as_mut_ptr(), dct4x4.as_mut_ptr());
            }
            let mut i8x8: libc::c_int = 0 as libc::c_int;
            while i8x8
                < (if chroma422 != 0 { 2 as libc::c_int } else { 1 as libc::c_int })
            {
                if (*h).mb.b_trellis != 0 {
                    let mut i4x4: libc::c_int = 0 as libc::c_int;
                    while i4x4 < 4 as libc::c_int {
                        if x264_8_quant_4x4_trellis(
                            h,
                            (dct4x4[(i8x8 * 4 as libc::c_int + i4x4) as usize])
                                .as_mut_ptr(),
                            CQM_4IC as libc::c_int + b_inter,
                            i_qp,
                            DCT_CHROMA_AC as libc::c_int,
                            (b_inter == 0) as libc::c_int,
                            1 as libc::c_int,
                            0 as libc::c_int,
                        ) != 0
                        {
                            let mut idx: libc::c_int = 16 as libc::c_int
                                + ch_0 * 16 as libc::c_int + i8x8 * 8 as libc::c_int + i4x4;
                            ((*h).zigzagf.scan_4x4)
                                .expect(
                                    "non-null function pointer",
                                )(
                                ((*h).dct.luma4x4[idx as usize]).as_mut_ptr(),
                                (dct4x4[(i8x8 * 4 as libc::c_int + i4x4) as usize])
                                    .as_mut_ptr(),
                            );
                            ((*h).quantf.dequant_4x4)
                                .expect(
                                    "non-null function pointer",
                                )(
                                (dct4x4[(i8x8 * 4 as libc::c_int + i4x4) as usize])
                                    .as_mut_ptr(),
                                dequant_mf,
                                i_qp,
                            );
                            if i_decimate_score < 7 as libc::c_int {
                                i_decimate_score
                                    += ((*h).quantf.decimate_score15)
                                        .expect(
                                            "non-null function pointer",
                                        )(((*h).dct.luma4x4[idx as usize]).as_mut_ptr());
                            }
                            (*h)
                                .mb
                                .cache
                                .non_zero_count[x264_scan8[idx as usize]
                                as usize] = 1 as libc::c_int as uint8_t;
                            nz_ac = 1 as libc::c_int;
                        }
                        i4x4 += 1;
                        i4x4;
                    }
                } else {
                    nz = ((*h).quantf.quant_4x4x4)
                        .expect(
                            "non-null function pointer",
                        )(
                        &mut *dct4x4
                            .as_mut_ptr()
                            .offset((i8x8 * 4 as libc::c_int) as isize),
                        (*((*h).quant4_mf[(CQM_4IC as libc::c_int + b_inter) as usize])
                            .offset(i_qp as isize))
                            .as_mut_ptr(),
                        (*((*h).quant4_bias[(CQM_4IC as libc::c_int + b_inter) as usize])
                            .offset(i_qp as isize))
                            .as_mut_ptr(),
                    );
                    nz_ac |= nz;
                    let mut i4x4_0: libc::c_int = 0 as libc::c_int;
                    let mut msk: libc::c_int = nz;
                    let mut skip: libc::c_int = 0;
                    while msk != 0
                        && {
                            skip = x264_ctz_4bit(msk as uint32_t);
                            i4x4_0 += skip;
                            msk >>= skip + 1 as libc::c_int;
                            1 as libc::c_int != 0
                        }
                    {
                        let mut idx_0: libc::c_int = 16 as libc::c_int
                            + ch_0 * 16 as libc::c_int + i8x8 * 8 as libc::c_int
                            + i4x4_0;
                        ((*h).zigzagf.scan_4x4)
                            .expect(
                                "non-null function pointer",
                            )(
                            ((*h).dct.luma4x4[idx_0 as usize]).as_mut_ptr(),
                            (dct4x4[(i8x8 * 4 as libc::c_int + i4x4_0) as usize])
                                .as_mut_ptr(),
                        );
                        ((*h).quantf.dequant_4x4)
                            .expect(
                                "non-null function pointer",
                            )(
                            (dct4x4[(i8x8 * 4 as libc::c_int + i4x4_0) as usize])
                                .as_mut_ptr(),
                            dequant_mf,
                            i_qp,
                        );
                        if i_decimate_score < 7 as libc::c_int {
                            i_decimate_score
                                += ((*h).quantf.decimate_score15)
                                    .expect(
                                        "non-null function pointer",
                                    )(((*h).dct.luma4x4[idx_0 as usize]).as_mut_ptr());
                        }
                        (*h)
                            .mb
                            .cache
                            .non_zero_count[x264_scan8[idx_0 as usize]
                            as usize] = 1 as libc::c_int as uint8_t;
                        i4x4_0 += 1;
                        i4x4_0;
                    }
                }
                i8x8 += 1;
                i8x8;
            }
            if (*h).mb.b_trellis != 0 {
                nz_dc = x264_8_quant_chroma_dc_trellis(
                    h,
                    dct_dc.as_mut_ptr(),
                    i_qp + 3 as libc::c_int * chroma422,
                    (b_inter == 0) as libc::c_int,
                    49 as libc::c_int + ch_0,
                );
            } else {
                nz_dc = 0 as libc::c_int;
                let mut i_4: libc::c_int = 0 as libc::c_int;
                while i_4 <= chroma422 {
                    nz_dc
                        |= ((*h).quantf.quant_2x2_dc)
                            .expect(
                                "non-null function pointer",
                            )(
                            &mut *dct_dc
                                .as_mut_ptr()
                                .offset((4 as libc::c_int * i_4) as isize),
                            (*((*h)
                                .quant4_mf[(CQM_4IC as libc::c_int + b_inter) as usize])
                                .offset(
                                    (i_qp + 3 as libc::c_int * chroma422) as isize,
                                ))[0 as libc::c_int as usize] as libc::c_int
                                >> 1 as libc::c_int,
                            ((*((*h)
                                .quant4_bias[(CQM_4IC as libc::c_int + b_inter) as usize])
                                .offset(
                                    (i_qp + 3 as libc::c_int * chroma422) as isize,
                                ))[0 as libc::c_int as usize] as libc::c_int)
                                << 1 as libc::c_int,
                        );
                    i_4 += 1;
                    i_4;
                }
            }
            (*h)
                .mb
                .cache
                .non_zero_count[x264_scan8[(49 as libc::c_int + ch_0) as usize]
                as usize] = nz_dc as uint8_t;
            if i_decimate_score < 7 as libc::c_int || nz_ac == 0 {
                (*(&mut *((*h).mb.cache.non_zero_count)
                    .as_mut_ptr()
                    .offset(
                        *x264_scan8
                            .as_ptr()
                            .offset(
                                (16 as libc::c_int + 16 as libc::c_int * ch_0) as isize,
                            ) as isize,
                    ) as *mut uint8_t as *mut x264_union16_t))
                    .i = 0 as libc::c_int as uint16_t;
                (*(&mut *((*h).mb.cache.non_zero_count)
                    .as_mut_ptr()
                    .offset(
                        *x264_scan8
                            .as_ptr()
                            .offset(
                                (18 as libc::c_int + 16 as libc::c_int * ch_0) as isize,
                            ) as isize,
                    ) as *mut uint8_t as *mut x264_union16_t))
                    .i = 0 as libc::c_int as uint16_t;
                if chroma422 != 0 {
                    (*(&mut *((*h).mb.cache.non_zero_count)
                        .as_mut_ptr()
                        .offset(
                            *x264_scan8
                                .as_ptr()
                                .offset(
                                    (24 as libc::c_int + 16 as libc::c_int * ch_0) as isize,
                                ) as isize,
                        ) as *mut uint8_t as *mut x264_union16_t))
                        .i = 0 as libc::c_int as uint16_t;
                    (*(&mut *((*h).mb.cache.non_zero_count)
                        .as_mut_ptr()
                        .offset(
                            *x264_scan8
                                .as_ptr()
                                .offset(
                                    (26 as libc::c_int + 16 as libc::c_int * ch_0) as isize,
                                ) as isize,
                        ) as *mut uint8_t as *mut x264_union16_t))
                        .i = 0 as libc::c_int as uint16_t;
                }
                if nz_dc != 0 {
                    if mb_optimize_chroma_dc(
                        h,
                        dct_dc.as_mut_ptr(),
                        dequant_mf,
                        i_qp + 3 as libc::c_int * chroma422,
                        chroma422,
                    ) == 0
                    {
                        (*h)
                            .mb
                            .cache
                            .non_zero_count[x264_scan8[(49 as libc::c_int + ch_0)
                            as usize] as usize] = 0 as libc::c_int as uint8_t;
                    } else {
                        if chroma422 != 0 {
                            zigzag_scan_2x4_dc(
                                ((*h).dct.chroma_dc[ch_0 as usize]).as_mut_ptr(),
                                dct_dc.as_mut_ptr(),
                            );
                            ((*h).quantf.idct_dequant_2x4_dconly)
                                .expect(
                                    "non-null function pointer",
                                )(dct_dc.as_mut_ptr(), dequant_mf, i_qp + 3 as libc::c_int);
                        } else {
                            zigzag_scan_2x2_dc(
                                ((*h).dct.chroma_dc[ch_0 as usize]).as_mut_ptr(),
                                dct_dc.as_mut_ptr(),
                            );
                            idct_dequant_2x2_dconly(
                                dct_dc.as_mut_ptr(),
                                dequant_mf,
                                i_qp,
                            );
                        }
                        let mut i_5: libc::c_int = 0 as libc::c_int;
                        while i_5 <= chroma422 {
                            ((*h).dctf.add8x8_idct_dc)
                                .expect(
                                    "non-null function pointer",
                                )(
                                p_dst_0
                                    .offset(
                                        (8 as libc::c_int * i_5 * 32 as libc::c_int) as isize,
                                    ),
                                &mut *dct_dc
                                    .as_mut_ptr()
                                    .offset((4 as libc::c_int * i_5) as isize),
                            );
                            i_5 += 1;
                            i_5;
                        }
                    }
                }
            } else {
                (*h).mb.i_cbp_chroma = 1 as libc::c_int;
                if nz_dc != 0 {
                    if chroma422 != 0 {
                        zigzag_scan_2x4_dc(
                            ((*h).dct.chroma_dc[ch_0 as usize]).as_mut_ptr(),
                            dct_dc.as_mut_ptr(),
                        );
                        ((*h).quantf.idct_dequant_2x4_dc)
                            .expect(
                                "non-null function pointer",
                            )(
                            dct_dc.as_mut_ptr(),
                            dct4x4.as_mut_ptr(),
                            dequant_mf,
                            i_qp + 3 as libc::c_int,
                        );
                    } else {
                        zigzag_scan_2x2_dc(
                            ((*h).dct.chroma_dc[ch_0 as usize]).as_mut_ptr(),
                            dct_dc.as_mut_ptr(),
                        );
                        idct_dequant_2x2_dc(
                            dct_dc.as_mut_ptr(),
                            dct4x4.as_mut_ptr(),
                            dequant_mf,
                            i_qp,
                        );
                    }
                }
                let mut i_6: libc::c_int = 0 as libc::c_int;
                while i_6 <= chroma422 {
                    ((*h).dctf.add8x8_idct)
                        .expect(
                            "non-null function pointer",
                        )(
                        p_dst_0
                            .offset(
                                (8 as libc::c_int * i_6 * 32 as libc::c_int) as isize,
                            ),
                        &mut *dct4x4
                            .as_mut_ptr()
                            .offset((4 as libc::c_int * i_6) as isize),
                    );
                    i_6 += 1;
                    i_6;
                }
            }
        }
        ch_0 += 1;
        ch_0;
    }
    (*h).mb.i_cbp_chroma
        += (*h)
            .mb
            .cache
            .non_zero_count[x264_scan8[(49 as libc::c_int + 0 as libc::c_int) as usize]
            as usize] as libc::c_int
            | (*h)
                .mb
                .cache
                .non_zero_count[x264_scan8[(49 as libc::c_int + 1 as libc::c_int)
                as usize] as usize] as libc::c_int | (*h).mb.i_cbp_chroma;
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_mb_encode_chroma(
    mut h: *mut x264_t,
    mut b_inter: libc::c_int,
    mut i_qp: libc::c_int,
) {
    if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_420 as libc::c_int {
        mb_encode_chroma_internal(h, b_inter, i_qp, 0 as libc::c_int);
    } else {
        mb_encode_chroma_internal(h, b_inter, i_qp, 1 as libc::c_int);
    };
}
unsafe extern "C" fn macroblock_encode_skip(mut h: *mut x264_t) {
    (*(&mut *((*h).mb.cache.non_zero_count)
        .as_mut_ptr()
        .offset(*x264_scan8.as_ptr().offset(0 as libc::c_int as isize) as isize)
        as *mut uint8_t as *mut x264_union32_t))
        .i = 0 as libc::c_int as uint32_t;
    (*(&mut *((*h).mb.cache.non_zero_count)
        .as_mut_ptr()
        .offset(*x264_scan8.as_ptr().offset(2 as libc::c_int as isize) as isize)
        as *mut uint8_t as *mut x264_union32_t))
        .i = 0 as libc::c_int as uint32_t;
    (*(&mut *((*h).mb.cache.non_zero_count)
        .as_mut_ptr()
        .offset(*x264_scan8.as_ptr().offset(8 as libc::c_int as isize) as isize)
        as *mut uint8_t as *mut x264_union32_t))
        .i = 0 as libc::c_int as uint32_t;
    (*(&mut *((*h).mb.cache.non_zero_count)
        .as_mut_ptr()
        .offset(*x264_scan8.as_ptr().offset(10 as libc::c_int as isize) as isize)
        as *mut uint8_t as *mut x264_union32_t))
        .i = 0 as libc::c_int as uint32_t;
    (*(&mut *((*h).mb.cache.non_zero_count)
        .as_mut_ptr()
        .offset(
            *x264_scan8.as_ptr().offset((16 as libc::c_int + 0 as libc::c_int) as isize)
                as isize,
        ) as *mut uint8_t as *mut x264_union32_t))
        .i = 0 as libc::c_int as uint32_t;
    (*(&mut *((*h).mb.cache.non_zero_count)
        .as_mut_ptr()
        .offset(
            *x264_scan8.as_ptr().offset((16 as libc::c_int + 2 as libc::c_int) as isize)
                as isize,
        ) as *mut uint8_t as *mut x264_union32_t))
        .i = 0 as libc::c_int as uint32_t;
    (*(&mut *((*h).mb.cache.non_zero_count)
        .as_mut_ptr()
        .offset(
            *x264_scan8.as_ptr().offset((32 as libc::c_int + 0 as libc::c_int) as isize)
                as isize,
        ) as *mut uint8_t as *mut x264_union32_t))
        .i = 0 as libc::c_int as uint32_t;
    (*(&mut *((*h).mb.cache.non_zero_count)
        .as_mut_ptr()
        .offset(
            *x264_scan8.as_ptr().offset((32 as libc::c_int + 2 as libc::c_int) as isize)
                as isize,
        ) as *mut uint8_t as *mut x264_union32_t))
        .i = 0 as libc::c_int as uint32_t;
    if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc >= CHROMA_422 as libc::c_int {
        (*(&mut *((*h).mb.cache.non_zero_count)
            .as_mut_ptr()
            .offset(
                *x264_scan8
                    .as_ptr()
                    .offset((16 as libc::c_int + 8 as libc::c_int) as isize) as isize,
            ) as *mut uint8_t as *mut x264_union32_t))
            .i = 0 as libc::c_int as uint32_t;
        (*(&mut *((*h).mb.cache.non_zero_count)
            .as_mut_ptr()
            .offset(
                *x264_scan8
                    .as_ptr()
                    .offset((16 as libc::c_int + 10 as libc::c_int) as isize) as isize,
            ) as *mut uint8_t as *mut x264_union32_t))
            .i = 0 as libc::c_int as uint32_t;
        (*(&mut *((*h).mb.cache.non_zero_count)
            .as_mut_ptr()
            .offset(
                *x264_scan8
                    .as_ptr()
                    .offset((32 as libc::c_int + 8 as libc::c_int) as isize) as isize,
            ) as *mut uint8_t as *mut x264_union32_t))
            .i = 0 as libc::c_int as uint32_t;
        (*(&mut *((*h).mb.cache.non_zero_count)
            .as_mut_ptr()
            .offset(
                *x264_scan8
                    .as_ptr()
                    .offset((32 as libc::c_int + 10 as libc::c_int) as isize) as isize,
            ) as *mut uint8_t as *mut x264_union32_t))
            .i = 0 as libc::c_int as uint32_t;
    }
    (*h).mb.i_cbp_luma = 0 as libc::c_int;
    (*h).mb.i_cbp_chroma = 0 as libc::c_int;
    *((*h).mb.cbp).offset((*h).mb.i_mb_xy as isize) = 0 as libc::c_int as int16_t;
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_predict_lossless_chroma(
    mut h: *mut x264_t,
    mut i_mode: libc::c_int,
) {
    let mut height: libc::c_int = 16 as libc::c_int >> (*h).mb.chroma_v_shift;
    if i_mode == I_PRED_CHROMA_V as libc::c_int {
        ((*h).mc.copy[PIXEL_8x8 as libc::c_int as usize])
            .expect(
                "non-null function pointer",
            )(
            (*h).mb.pic.p_fdec[1 as libc::c_int as usize],
            32 as libc::c_int as intptr_t,
            ((*h).mb.pic.p_fenc[1 as libc::c_int as usize])
                .offset(-(16 as libc::c_int as isize)),
            16 as libc::c_int as intptr_t,
            height,
        );
        ((*h).mc.copy[PIXEL_8x8 as libc::c_int as usize])
            .expect(
                "non-null function pointer",
            )(
            (*h).mb.pic.p_fdec[2 as libc::c_int as usize],
            32 as libc::c_int as intptr_t,
            ((*h).mb.pic.p_fenc[2 as libc::c_int as usize])
                .offset(-(16 as libc::c_int as isize)),
            16 as libc::c_int as intptr_t,
            height,
        );
        memcpy(
            (*h).mb.pic.p_fdec[1 as libc::c_int as usize] as *mut libc::c_void,
            ((*h).mb.pic.p_fdec[1 as libc::c_int as usize])
                .offset(-(32 as libc::c_int as isize)) as *const libc::c_void,
            (8 as libc::c_int
                * ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int)
                as libc::c_ulong,
        );
        memcpy(
            (*h).mb.pic.p_fdec[2 as libc::c_int as usize] as *mut libc::c_void,
            ((*h).mb.pic.p_fdec[2 as libc::c_int as usize])
                .offset(-(32 as libc::c_int as isize)) as *const libc::c_void,
            (8 as libc::c_int
                * ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int)
                as libc::c_ulong,
        );
    } else if i_mode == I_PRED_CHROMA_H as libc::c_int {
        ((*h).mc.copy[PIXEL_8x8 as libc::c_int as usize])
            .expect(
                "non-null function pointer",
            )(
            (*h).mb.pic.p_fdec[1 as libc::c_int as usize],
            32 as libc::c_int as intptr_t,
            ((*h).mb.pic.p_fenc[1 as libc::c_int as usize])
                .offset(-(1 as libc::c_int as isize)),
            16 as libc::c_int as intptr_t,
            height,
        );
        ((*h).mc.copy[PIXEL_8x8 as libc::c_int as usize])
            .expect(
                "non-null function pointer",
            )(
            (*h).mb.pic.p_fdec[2 as libc::c_int as usize],
            32 as libc::c_int as intptr_t,
            ((*h).mb.pic.p_fenc[2 as libc::c_int as usize])
                .offset(-(1 as libc::c_int as isize)),
            16 as libc::c_int as intptr_t,
            height,
        );
        x264_8_copy_column8(
            ((*h).mb.pic.p_fdec[1 as libc::c_int as usize])
                .offset((4 as libc::c_int * 32 as libc::c_int) as isize),
            ((*h).mb.pic.p_fdec[1 as libc::c_int as usize])
                .offset((4 as libc::c_int * 32 as libc::c_int) as isize)
                .offset(-(1 as libc::c_int as isize)),
        );
        x264_8_copy_column8(
            ((*h).mb.pic.p_fdec[2 as libc::c_int as usize])
                .offset((4 as libc::c_int * 32 as libc::c_int) as isize),
            ((*h).mb.pic.p_fdec[2 as libc::c_int as usize])
                .offset((4 as libc::c_int * 32 as libc::c_int) as isize)
                .offset(-(1 as libc::c_int as isize)),
        );
        if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_422 as libc::c_int {
            x264_8_copy_column8(
                ((*h).mb.pic.p_fdec[1 as libc::c_int as usize])
                    .offset((12 as libc::c_int * 32 as libc::c_int) as isize),
                ((*h).mb.pic.p_fdec[1 as libc::c_int as usize])
                    .offset((12 as libc::c_int * 32 as libc::c_int) as isize)
                    .offset(-(1 as libc::c_int as isize)),
            );
            x264_8_copy_column8(
                ((*h).mb.pic.p_fdec[2 as libc::c_int as usize])
                    .offset((12 as libc::c_int * 32 as libc::c_int) as isize),
                ((*h).mb.pic.p_fdec[2 as libc::c_int as usize])
                    .offset((12 as libc::c_int * 32 as libc::c_int) as isize)
                    .offset(-(1 as libc::c_int as isize)),
            );
        }
    } else {
        ((*h).predict_chroma[i_mode as usize])
            .expect(
                "non-null function pointer",
            )((*h).mb.pic.p_fdec[1 as libc::c_int as usize]);
        ((*h).predict_chroma[i_mode as usize])
            .expect(
                "non-null function pointer",
            )((*h).mb.pic.p_fdec[2 as libc::c_int as usize]);
    };
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_predict_lossless_4x4(
    mut h: *mut x264_t,
    mut p_dst: *mut pixel,
    mut p: libc::c_int,
    mut idx: libc::c_int,
    mut i_mode: libc::c_int,
) {
    let mut stride: libc::c_int = (*(*h).fenc).i_stride[p as usize]
        << (*h).mb.b_interlaced;
    let mut p_src: *mut pixel = ((*h).mb.pic.p_fenc_plane[p as usize])
        .offset((block_idx_x[idx as usize] as libc::c_int * 4 as libc::c_int) as isize)
        .offset(
            (block_idx_y[idx as usize] as libc::c_int * 4 as libc::c_int * stride)
                as isize,
        );
    if i_mode == I_PRED_4x4_V as libc::c_int {
        ((*h).mc.copy[PIXEL_4x4 as libc::c_int as usize])
            .expect(
                "non-null function pointer",
            )(
            p_dst,
            32 as libc::c_int as intptr_t,
            p_src.offset(-(stride as isize)),
            stride as intptr_t,
            4 as libc::c_int,
        );
        memcpy(
            p_dst as *mut libc::c_void,
            p_dst.offset(-(32 as libc::c_int as isize)) as *const libc::c_void,
            (4 as libc::c_int
                * ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int)
                as libc::c_ulong,
        );
    } else if i_mode == I_PRED_4x4_H as libc::c_int {
        ((*h).mc.copy[PIXEL_4x4 as libc::c_int as usize])
            .expect(
                "non-null function pointer",
            )(
            p_dst,
            32 as libc::c_int as intptr_t,
            p_src.offset(-(1 as libc::c_int as isize)),
            stride as intptr_t,
            4 as libc::c_int,
        );
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            *p_dst
                .offset(
                    (i * 32 as libc::c_int) as isize,
                ) = *p_dst.offset((i * 32 as libc::c_int - 1 as libc::c_int) as isize);
            i += 1;
            i;
        }
    } else {
        ((*h).predict_4x4[i_mode as usize]).expect("non-null function pointer")(p_dst);
    };
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_predict_lossless_8x8(
    mut h: *mut x264_t,
    mut p_dst: *mut pixel,
    mut p: libc::c_int,
    mut idx: libc::c_int,
    mut i_mode: libc::c_int,
    mut edge: *mut pixel,
) {
    let mut stride: libc::c_int = (*(*h).fenc).i_stride[p as usize]
        << (*h).mb.b_interlaced;
    let mut p_src: *mut pixel = ((*h).mb.pic.p_fenc_plane[p as usize])
        .offset(((idx & 1 as libc::c_int) * 8 as libc::c_int) as isize)
        .offset(((idx >> 1 as libc::c_int) * 8 as libc::c_int * stride) as isize);
    if i_mode == I_PRED_8x8_V as libc::c_int {
        ((*h).mc.copy[PIXEL_8x8 as libc::c_int as usize])
            .expect(
                "non-null function pointer",
            )(
            p_dst,
            32 as libc::c_int as intptr_t,
            p_src.offset(-(stride as isize)),
            stride as intptr_t,
            8 as libc::c_int,
        );
        memcpy(
            p_dst as *mut libc::c_void,
            &mut *edge.offset(16 as libc::c_int as isize) as *mut pixel
                as *const libc::c_void,
            (8 as libc::c_int
                * ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int)
                as libc::c_ulong,
        );
    } else if i_mode == I_PRED_8x8_H as libc::c_int {
        ((*h).mc.copy[PIXEL_8x8 as libc::c_int as usize])
            .expect(
                "non-null function pointer",
            )(
            p_dst,
            32 as libc::c_int as intptr_t,
            p_src.offset(-(1 as libc::c_int as isize)),
            stride as intptr_t,
            8 as libc::c_int,
        );
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < 8 as libc::c_int {
            *p_dst
                .offset(
                    (i * 32 as libc::c_int) as isize,
                ) = *edge.offset((14 as libc::c_int - i) as isize);
            i += 1;
            i;
        }
    } else {
        ((*h).predict_8x8[i_mode as usize])
            .expect("non-null function pointer")(p_dst, edge);
    };
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_predict_lossless_16x16(
    mut h: *mut x264_t,
    mut p: libc::c_int,
    mut i_mode: libc::c_int,
) {
    let mut stride: libc::c_int = (*(*h).fenc).i_stride[p as usize]
        << (*h).mb.b_interlaced;
    let mut p_dst: *mut pixel = (*h).mb.pic.p_fdec[p as usize];
    if i_mode == I_PRED_16x16_V as libc::c_int {
        ((*h).mc.copy[PIXEL_16x16 as libc::c_int as usize])
            .expect(
                "non-null function pointer",
            )(
            p_dst,
            32 as libc::c_int as intptr_t,
            ((*h).mb.pic.p_fenc_plane[p as usize]).offset(-(stride as isize)),
            stride as intptr_t,
            16 as libc::c_int,
        );
        memcpy(
            p_dst as *mut libc::c_void,
            p_dst.offset(-(32 as libc::c_int as isize)) as *const libc::c_void,
            (16 as libc::c_int
                * ::core::mem::size_of::<pixel>() as libc::c_ulong as libc::c_int)
                as libc::c_ulong,
        );
    } else if i_mode == I_PRED_16x16_H as libc::c_int {
        ((*h).mc.copy_16x16_unaligned)
            .expect(
                "non-null function pointer",
            )(
            p_dst,
            32 as libc::c_int as intptr_t,
            ((*h).mb.pic.p_fenc_plane[p as usize]).offset(-(1 as libc::c_int as isize)),
            stride as intptr_t,
            16 as libc::c_int,
        );
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < 16 as libc::c_int {
            *p_dst
                .offset(
                    (i * 32 as libc::c_int) as isize,
                ) = *p_dst.offset((i * 32 as libc::c_int - 1 as libc::c_int) as isize);
            i += 1;
            i;
        }
    } else {
        ((*h).predict_16x16[i_mode as usize]).expect("non-null function pointer")(p_dst);
    };
}
#[inline(always)]
unsafe extern "C" fn macroblock_encode_internal(
    mut h: *mut x264_t,
    mut plane_count: libc::c_int,
    mut chroma: libc::c_int,
) {
    let mut i_qp: libc::c_int = (*h).mb.i_qp;
    let mut b_decimate: libc::c_int = (*h).mb.b_dct_decimate;
    let mut b_force_no_skip: libc::c_int = 0 as libc::c_int;
    let mut nz: libc::c_int = 0;
    (*h).mb.i_cbp_luma = 0 as libc::c_int;
    let mut p: libc::c_int = 0 as libc::c_int;
    while p < plane_count {
        (*h)
            .mb
            .cache
            .non_zero_count[x264_scan8[(48 as libc::c_int + p) as usize]
            as usize] = 0 as libc::c_int as uint8_t;
        p += 1;
        p;
    }
    if (*h).mb.i_type == I_PCM as libc::c_int {
        let mut p_0: libc::c_int = 0 as libc::c_int;
        while p_0 < plane_count {
            ((*h).mc.copy[PIXEL_16x16 as libc::c_int as usize])
                .expect(
                    "non-null function pointer",
                )(
                (*h).mb.pic.p_fdec[p_0 as usize],
                32 as libc::c_int as intptr_t,
                (*h).mb.pic.p_fenc[p_0 as usize],
                16 as libc::c_int as intptr_t,
                16 as libc::c_int,
            );
            p_0 += 1;
            p_0;
        }
        if chroma != 0 {
            let mut height: libc::c_int = 16 as libc::c_int >> (*h).mb.chroma_v_shift;
            ((*h).mc.copy[PIXEL_8x8 as libc::c_int as usize])
                .expect(
                    "non-null function pointer",
                )(
                (*h).mb.pic.p_fdec[1 as libc::c_int as usize],
                32 as libc::c_int as intptr_t,
                (*h).mb.pic.p_fenc[1 as libc::c_int as usize],
                16 as libc::c_int as intptr_t,
                height,
            );
            ((*h).mc.copy[PIXEL_8x8 as libc::c_int as usize])
                .expect(
                    "non-null function pointer",
                )(
                (*h).mb.pic.p_fdec[2 as libc::c_int as usize],
                32 as libc::c_int as intptr_t,
                (*h).mb.pic.p_fenc[2 as libc::c_int as usize],
                16 as libc::c_int as intptr_t,
                height,
            );
        }
        return;
    }
    if (*h).mb.b_allow_skip == 0 {
        b_force_no_skip = 1 as libc::c_int;
        if (*h).mb.i_type == P_SKIP as libc::c_int
            || (*h).mb.i_type == B_SKIP as libc::c_int
        {
            if (*h).mb.i_type == P_SKIP as libc::c_int {
                (*h).mb.i_type = P_L0 as libc::c_int;
            } else if (*h).mb.i_type == B_SKIP as libc::c_int {
                (*h).mb.i_type = B_DIRECT as libc::c_int;
            }
        }
    }
    if (*h).mb.i_type == P_SKIP as libc::c_int {
        if (*h).mb.b_skip_mc == 0 {
            let mut mvx: libc::c_int = x264_clip3(
                (*h)
                    .mb
                    .cache
                    .mv[0 as libc::c_int
                    as usize][x264_scan8[0 as libc::c_int as usize]
                    as usize][0 as libc::c_int as usize] as libc::c_int,
                (*h).mb.mv_min[0 as libc::c_int as usize],
                (*h).mb.mv_max[0 as libc::c_int as usize],
            );
            let mut mvy: libc::c_int = x264_clip3(
                (*h)
                    .mb
                    .cache
                    .mv[0 as libc::c_int
                    as usize][x264_scan8[0 as libc::c_int as usize]
                    as usize][1 as libc::c_int as usize] as libc::c_int,
                (*h).mb.mv_min[1 as libc::c_int as usize],
                (*h).mb.mv_max[1 as libc::c_int as usize],
            );
            let mut p_1: libc::c_int = 0 as libc::c_int;
            while p_1 < plane_count {
                ((*h).mc.mc_luma)
                    .expect(
                        "non-null function pointer",
                    )(
                    (*h).mb.pic.p_fdec[p_1 as usize],
                    32 as libc::c_int as intptr_t,
                    &mut *(*(*((*h).mb.pic.p_fref)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset((p_1 * 4 as libc::c_int) as isize),
                    (*h).mb.pic.i_stride[p_1 as usize] as intptr_t,
                    mvx,
                    mvy,
                    16 as libc::c_int,
                    16 as libc::c_int,
                    &mut *(*((*h).sh.weight)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(p_1 as isize),
                );
                p_1 += 1;
                p_1;
            }
            if chroma != 0 {
                let mut v_shift: libc::c_int = (*h).mb.chroma_v_shift;
                let mut height_0: libc::c_int = 16 as libc::c_int >> v_shift;
                if mvx | mvy != 0 {
                    ((*h).mc.mc_chroma)
                        .expect(
                            "non-null function pointer",
                        )(
                        (*h).mb.pic.p_fdec[1 as libc::c_int as usize],
                        (*h).mb.pic.p_fdec[2 as libc::c_int as usize],
                        32 as libc::c_int as intptr_t,
                        (*h)
                            .mb
                            .pic
                            .p_fref[0 as libc::c_int
                            as usize][0 as libc::c_int
                            as usize][4 as libc::c_int as usize],
                        (*h).mb.pic.i_stride[1 as libc::c_int as usize] as intptr_t,
                        mvx,
                        (2 as libc::c_int * mvy) >> v_shift,
                        8 as libc::c_int,
                        height_0,
                    );
                } else {
                    ((*h).mc.load_deinterleave_chroma_fdec)
                        .expect(
                            "non-null function pointer",
                        )(
                        (*h).mb.pic.p_fdec[1 as libc::c_int as usize],
                        (*h)
                            .mb
                            .pic
                            .p_fref[0 as libc::c_int
                            as usize][0 as libc::c_int
                            as usize][4 as libc::c_int as usize],
                        (*h).mb.pic.i_stride[1 as libc::c_int as usize] as intptr_t,
                        height_0,
                    );
                }
                if !((*h)
                    .sh
                    .weight[0 as libc::c_int as usize][1 as libc::c_int as usize]
                    .weightfn)
                    .is_null()
                {
                    (*((*h)
                        .sh
                        .weight[0 as libc::c_int as usize][1 as libc::c_int as usize]
                        .weightfn)
                        .offset((8 as libc::c_int >> 2 as libc::c_int) as isize))
                        .expect(
                            "non-null function pointer",
                        )(
                        (*h).mb.pic.p_fdec[1 as libc::c_int as usize],
                        32 as libc::c_int as intptr_t,
                        (*h).mb.pic.p_fdec[1 as libc::c_int as usize],
                        32 as libc::c_int as intptr_t,
                        &mut *(*((*h).sh.weight)
                            .as_mut_ptr()
                            .offset(0 as libc::c_int as isize))
                            .as_mut_ptr()
                            .offset(1 as libc::c_int as isize),
                        height_0,
                    );
                }
                if !((*h)
                    .sh
                    .weight[0 as libc::c_int as usize][2 as libc::c_int as usize]
                    .weightfn)
                    .is_null()
                {
                    (*((*h)
                        .sh
                        .weight[0 as libc::c_int as usize][2 as libc::c_int as usize]
                        .weightfn)
                        .offset((8 as libc::c_int >> 2 as libc::c_int) as isize))
                        .expect(
                            "non-null function pointer",
                        )(
                        (*h).mb.pic.p_fdec[2 as libc::c_int as usize],
                        32 as libc::c_int as intptr_t,
                        (*h).mb.pic.p_fdec[2 as libc::c_int as usize],
                        32 as libc::c_int as intptr_t,
                        &mut *(*((*h).sh.weight)
                            .as_mut_ptr()
                            .offset(0 as libc::c_int as isize))
                            .as_mut_ptr()
                            .offset(2 as libc::c_int as isize),
                        height_0,
                    );
                }
            }
        }
        macroblock_encode_skip(h);
        return;
    }
    if (*h).mb.i_type == B_SKIP as libc::c_int {
        if (*h).mb.b_skip_mc == 0 {
            x264_8_mb_mc(h);
        }
        macroblock_encode_skip(h);
        return;
    }
    if (*h).mb.i_type == I_16x16 as libc::c_int {
        (*h).mb.b_transform_8x8 = 0 as libc::c_int;
        let mut p_2: libc::c_int = 0 as libc::c_int;
        while p_2 < plane_count {
            mb_encode_i16x16(h, p_2, i_qp);
            p_2 += 1;
            p_2;
            i_qp = (*h).mb.i_chroma_qp;
        }
    } else if (*h).mb.i_type == I_8x8 as libc::c_int {
        (*h).mb.b_transform_8x8 = 1 as libc::c_int;
        if (*h).mb.i_skip_intra != 0 {
            ((*h).mc.copy[PIXEL_16x16 as libc::c_int as usize])
                .expect(
                    "non-null function pointer",
                )(
                (*h).mb.pic.p_fdec[0 as libc::c_int as usize],
                32 as libc::c_int as intptr_t,
                ((*h).mb.pic.i8x8_fdec_buf).as_mut_ptr(),
                16 as libc::c_int as intptr_t,
                16 as libc::c_int,
            );
            (*(&mut *((*h).mb.cache.non_zero_count)
                .as_mut_ptr()
                .offset(*x264_scan8.as_ptr().offset(0 as libc::c_int as isize) as isize)
                as *mut uint8_t as *mut x264_union32_t))
                .i = (*h).mb.pic.i8x8_nnz_buf[0 as libc::c_int as usize];
            (*(&mut *((*h).mb.cache.non_zero_count)
                .as_mut_ptr()
                .offset(*x264_scan8.as_ptr().offset(2 as libc::c_int as isize) as isize)
                as *mut uint8_t as *mut x264_union32_t))
                .i = (*h).mb.pic.i8x8_nnz_buf[1 as libc::c_int as usize];
            (*(&mut *((*h).mb.cache.non_zero_count)
                .as_mut_ptr()
                .offset(*x264_scan8.as_ptr().offset(8 as libc::c_int as isize) as isize)
                as *mut uint8_t as *mut x264_union32_t))
                .i = (*h).mb.pic.i8x8_nnz_buf[2 as libc::c_int as usize];
            (*(&mut *((*h).mb.cache.non_zero_count)
                .as_mut_ptr()
                .offset(*x264_scan8.as_ptr().offset(10 as libc::c_int as isize) as isize)
                as *mut uint8_t as *mut x264_union32_t))
                .i = (*h).mb.pic.i8x8_nnz_buf[3 as libc::c_int as usize];
            (*h).mb.i_cbp_luma = (*h).mb.pic.i8x8_cbp;
            if (*h).mb.i_skip_intra == 2 as libc::c_int {
                ((*h).mc.memcpy_aligned)
                    .expect(
                        "non-null function pointer",
                    )(
                    ((*h).dct.luma8x8).as_mut_ptr() as *mut libc::c_void,
                    ((*h).mb.pic.i8x8_dct_buf).as_mut_ptr() as *const libc::c_void,
                    ::core::mem::size_of::<[[dctcoef; 64]; 3]>() as libc::c_ulong,
                );
            }
        }
        let mut p_3: libc::c_int = 0 as libc::c_int;
        while p_3 < plane_count {
            let mut i: libc::c_int = if p_3 == 0 as libc::c_int
                && (*h).mb.i_skip_intra != 0
            {
                3 as libc::c_int
            } else {
                0 as libc::c_int
            };
            while i < 4 as libc::c_int {
                let mut i_mode: libc::c_int = (*h)
                    .mb
                    .cache
                    .intra4x4_pred_mode[x264_scan8[(4 as libc::c_int * i) as usize]
                    as usize] as libc::c_int;
                x264_mb_encode_i8x8(
                    h,
                    p_3,
                    i,
                    i_qp,
                    i_mode,
                    std::ptr::null_mut::<pixel>(),
                    1 as libc::c_int,
                );
                i += 1;
                i;
            }
            p_3 += 1;
            p_3;
            i_qp = (*h).mb.i_chroma_qp;
        }
    } else if (*h).mb.i_type == I_4x4 as libc::c_int {
        (*h).mb.b_transform_8x8 = 0 as libc::c_int;
        if (*h).mb.i_skip_intra != 0 {
            ((*h).mc.copy[PIXEL_16x16 as libc::c_int as usize])
                .expect(
                    "non-null function pointer",
                )(
                (*h).mb.pic.p_fdec[0 as libc::c_int as usize],
                32 as libc::c_int as intptr_t,
                ((*h).mb.pic.i4x4_fdec_buf).as_mut_ptr(),
                16 as libc::c_int as intptr_t,
                16 as libc::c_int,
            );
            (*(&mut *((*h).mb.cache.non_zero_count)
                .as_mut_ptr()
                .offset(*x264_scan8.as_ptr().offset(0 as libc::c_int as isize) as isize)
                as *mut uint8_t as *mut x264_union32_t))
                .i = (*h).mb.pic.i4x4_nnz_buf[0 as libc::c_int as usize];
            (*(&mut *((*h).mb.cache.non_zero_count)
                .as_mut_ptr()
                .offset(*x264_scan8.as_ptr().offset(2 as libc::c_int as isize) as isize)
                as *mut uint8_t as *mut x264_union32_t))
                .i = (*h).mb.pic.i4x4_nnz_buf[1 as libc::c_int as usize];
            (*(&mut *((*h).mb.cache.non_zero_count)
                .as_mut_ptr()
                .offset(*x264_scan8.as_ptr().offset(8 as libc::c_int as isize) as isize)
                as *mut uint8_t as *mut x264_union32_t))
                .i = (*h).mb.pic.i4x4_nnz_buf[2 as libc::c_int as usize];
            (*(&mut *((*h).mb.cache.non_zero_count)
                .as_mut_ptr()
                .offset(*x264_scan8.as_ptr().offset(10 as libc::c_int as isize) as isize)
                as *mut uint8_t as *mut x264_union32_t))
                .i = (*h).mb.pic.i4x4_nnz_buf[3 as libc::c_int as usize];
            (*h).mb.i_cbp_luma = (*h).mb.pic.i4x4_cbp;
            if (*h).mb.i_skip_intra == 2 as libc::c_int {
                ((*h).mc.memcpy_aligned)
                    .expect(
                        "non-null function pointer",
                    )(
                    ((*h).dct.luma4x4).as_mut_ptr() as *mut libc::c_void,
                    ((*h).mb.pic.i4x4_dct_buf).as_mut_ptr() as *const libc::c_void,
                    ::core::mem::size_of::<[[dctcoef; 16]; 15]>() as libc::c_ulong,
                );
            }
        }
        let mut p_4: libc::c_int = 0 as libc::c_int;
        while p_4 < plane_count {
            let mut i_0: libc::c_int = if p_4 == 0 as libc::c_int
                && (*h).mb.i_skip_intra != 0
            {
                15 as libc::c_int
            } else {
                0 as libc::c_int
            };
            while i_0 < 16 as libc::c_int {
                let mut p_dst: *mut pixel = &mut *(*((*h).mb.pic.p_fdec)
                    .as_mut_ptr()
                    .offset(p_4 as isize))
                    .offset(*block_idx_xy_fdec.as_ptr().offset(i_0 as isize) as isize)
                    as *mut pixel;
                let mut i_mode_0: libc::c_int = (*h)
                    .mb
                    .cache
                    .intra4x4_pred_mode[x264_scan8[i_0 as usize] as usize]
                    as libc::c_int;
                if (*h).mb.i_neighbour4[i_0 as usize]
                    & (MB_TOPRIGHT as libc::c_int | MB_TOP as libc::c_int)
                        as libc::c_uint == MB_TOP as libc::c_int as libc::c_uint
                {
                    (*(&mut *p_dst
                        .offset((4 as libc::c_int - 32 as libc::c_int) as isize)
                        as *mut pixel as *mut x264_union32_t))
                        .i = (*p_dst
                        .offset((3 as libc::c_int - 32 as libc::c_int) as isize)
                        as libc::c_uint)
                        .wrapping_mul(0x1010101 as libc::c_uint);
                }
                x264_mb_encode_i4x4(h, p_4, i_0, i_qp, i_mode_0, 1 as libc::c_int);
                i_0 += 1;
                i_0;
            }
            p_4 += 1;
            p_4;
            i_qp = (*h).mb.i_chroma_qp;
        }
    } else {
        let mut i_decimate_mb: libc::c_int = 0 as libc::c_int;
        if (*h).mb.b_skip_mc == 0 {
            x264_8_mb_mc(h);
        }
        if (*h).mb.b_lossless != 0 {
            if (*h).mb.b_transform_8x8 != 0 {
                let mut p_5: libc::c_int = 0 as libc::c_int;
                while p_5 < plane_count {
                    let mut i8x8: libc::c_int = 0 as libc::c_int;
                    while i8x8 < 4 as libc::c_int {
                        let mut x: libc::c_int = i8x8 & 1 as libc::c_int;
                        let mut y: libc::c_int = i8x8 >> 1 as libc::c_int;
                        nz = ((*h).zigzagf.sub_8x8)
                            .expect(
                                "non-null function pointer",
                            )(
                            ((*h).dct.luma8x8[(p_5 * 4 as libc::c_int + i8x8) as usize])
                                .as_mut_ptr(),
                            ((*h).mb.pic.p_fenc[p_5 as usize])
                                .offset((8 as libc::c_int * x) as isize)
                                .offset(
                                    (8 as libc::c_int * y * 16 as libc::c_int) as isize,
                                ),
                            ((*h).mb.pic.p_fdec[p_5 as usize])
                                .offset((8 as libc::c_int * x) as isize)
                                .offset((8 as libc::c_int * y * 32 as libc::c_int) as isize),
                        );
                        (*(&mut *((*h).mb.cache.non_zero_count)
                            .as_mut_ptr()
                            .offset(
                                (*x264_scan8
                                    .as_ptr()
                                    .offset(
                                        (p_5 * 16 as libc::c_int + i8x8 * 4 as libc::c_int) as isize,
                                    ) as libc::c_int + 0 as libc::c_int) as isize,
                            ) as *mut uint8_t as *mut x264_union16_t))
                            .i = (nz * 0x101 as libc::c_int) as uint16_t;
                        (*(&mut *((*h).mb.cache.non_zero_count)
                            .as_mut_ptr()
                            .offset(
                                (*x264_scan8
                                    .as_ptr()
                                    .offset(
                                        (p_5 * 16 as libc::c_int + i8x8 * 4 as libc::c_int) as isize,
                                    ) as libc::c_int + 8 as libc::c_int) as isize,
                            ) as *mut uint8_t as *mut x264_union16_t))
                            .i = (nz * 0x101 as libc::c_int) as uint16_t;
                        (*h).mb.i_cbp_luma |= nz << i8x8;
                        i8x8 += 1;
                        i8x8;
                    }
                    p_5 += 1;
                    p_5;
                }
            } else {
                let mut p_6: libc::c_int = 0 as libc::c_int;
                while p_6 < plane_count {
                    let mut i4x4: libc::c_int = 0 as libc::c_int;
                    while i4x4 < 16 as libc::c_int {
                        nz = ((*h).zigzagf.sub_4x4)
                            .expect(
                                "non-null function pointer",
                            )(
                            ((*h).dct.luma4x4[(p_6 * 16 as libc::c_int + i4x4) as usize])
                                .as_mut_ptr(),
                            ((*h).mb.pic.p_fenc[p_6 as usize])
                                .offset(
                                    block_idx_xy_fenc[i4x4 as usize] as libc::c_int as isize,
                                ),
                            ((*h).mb.pic.p_fdec[p_6 as usize])
                                .offset(
                                    block_idx_xy_fdec[i4x4 as usize] as libc::c_int as isize,
                                ),
                        );
                        (*h)
                            .mb
                            .cache
                            .non_zero_count[x264_scan8[(p_6 * 16 as libc::c_int + i4x4)
                            as usize] as usize] = nz as uint8_t;
                        (*h).mb.i_cbp_luma |= nz << (i4x4 >> 2 as libc::c_int);
                        i4x4 += 1;
                        i4x4;
                    }
                    p_6 += 1;
                    p_6;
                }
            }
        } else if (*h).mb.b_transform_8x8 != 0 {
            let mut dct8x8: [[dctcoef; 64]; 4] = [[0; 64]; 4];
            b_decimate
                &= ((*h).mb.b_trellis == 0 || (*h).param.b_cabac == 0) as libc::c_int;
            let mut p_7: libc::c_int = 0 as libc::c_int;
            while p_7 < plane_count {
                let mut quant_cat: libc::c_int = if p_7 != 0 {
                    CQM_8PC as libc::c_int
                } else {
                    CQM_8PY as libc::c_int
                };
                (*(&mut *((*h).mb.cache.non_zero_count)
                    .as_mut_ptr()
                    .offset(
                        (*x264_scan8.as_ptr().offset((16 as libc::c_int * p_7) as isize)
                            as libc::c_int + 0 as libc::c_int * 8 as libc::c_int)
                            as isize,
                    ) as *mut uint8_t as *mut x264_union32_t))
                    .i = 0 as libc::c_int as uint32_t;
                (*(&mut *((*h).mb.cache.non_zero_count)
                    .as_mut_ptr()
                    .offset(
                        (*x264_scan8.as_ptr().offset((16 as libc::c_int * p_7) as isize)
                            as libc::c_int + 1 as libc::c_int * 8 as libc::c_int)
                            as isize,
                    ) as *mut uint8_t as *mut x264_union32_t))
                    .i = 0 as libc::c_int as uint32_t;
                (*(&mut *((*h).mb.cache.non_zero_count)
                    .as_mut_ptr()
                    .offset(
                        (*x264_scan8.as_ptr().offset((16 as libc::c_int * p_7) as isize)
                            as libc::c_int + 2 as libc::c_int * 8 as libc::c_int)
                            as isize,
                    ) as *mut uint8_t as *mut x264_union32_t))
                    .i = 0 as libc::c_int as uint32_t;
                (*(&mut *((*h).mb.cache.non_zero_count)
                    .as_mut_ptr()
                    .offset(
                        (*x264_scan8.as_ptr().offset((16 as libc::c_int * p_7) as isize)
                            as libc::c_int + 3 as libc::c_int * 8 as libc::c_int)
                            as isize,
                    ) as *mut uint8_t as *mut x264_union32_t))
                    .i = 0 as libc::c_int as uint32_t;
                ((*h).dctf.sub16x16_dct8)
                    .expect(
                        "non-null function pointer",
                    )(
                    dct8x8.as_mut_ptr(),
                    (*h).mb.pic.p_fenc[p_7 as usize],
                    (*h).mb.pic.p_fdec[p_7 as usize],
                );
                let fresh1 = &mut (*((*h).nr_count)
                    .offset(
                        (1 as libc::c_int + (p_7 != 0) as libc::c_int * 2 as libc::c_int)
                            as isize,
                    ));
                *fresh1 = (*fresh1)
                    .wrapping_add(
                        ((*h).mb.b_noise_reduction * 4 as libc::c_int) as uint32_t,
                    );
                let mut plane_cbp: libc::c_int = 0 as libc::c_int;
                let mut idx: libc::c_int = 0 as libc::c_int;
                while idx < 4 as libc::c_int {
                    nz = x264_quant_8x8(
                        h,
                        (dct8x8[idx as usize]).as_mut_ptr(),
                        i_qp,
                        ctx_cat_plane[DCT_LUMA_8x8 as libc::c_int as usize][p_7 as usize]
                            as libc::c_int,
                        0 as libc::c_int,
                        p_7,
                        idx,
                    );
                    if nz != 0 {
                        ((*h).zigzagf.scan_8x8)
                            .expect(
                                "non-null function pointer",
                            )(
                            ((*h).dct.luma8x8[(p_7 * 4 as libc::c_int + idx) as usize])
                                .as_mut_ptr(),
                            (dct8x8[idx as usize]).as_mut_ptr(),
                        );
                        if b_decimate != 0 {
                            let mut i_decimate_8x8: libc::c_int = ((*h)
                                .quantf
                                .decimate_score64)
                                .expect(
                                    "non-null function pointer",
                                )(
                                ((*h).dct.luma8x8[(p_7 * 4 as libc::c_int + idx) as usize])
                                    .as_mut_ptr(),
                            );
                            i_decimate_mb += i_decimate_8x8;
                            if i_decimate_8x8 >= 4 as libc::c_int {
                                plane_cbp |= (1 as libc::c_int) << idx;
                            }
                        } else {
                            plane_cbp |= (1 as libc::c_int) << idx;
                        }
                    }
                    idx += 1;
                    idx;
                }
                if i_decimate_mb >= 6 as libc::c_int || b_decimate == 0 {
                    (*h).mb.i_cbp_luma |= plane_cbp;
                    let mut idx_0: libc::c_int = 0 as libc::c_int;
                    let mut msk: libc::c_int = plane_cbp;
                    let mut skip: libc::c_int = 0;
                    while msk != 0
                        && {
                            skip = x264_ctz_4bit(msk as uint32_t);
                            idx_0 += skip;
                            msk >>= skip + 1 as libc::c_int;
                            1 as libc::c_int != 0
                        }
                    {
                        ((*h).quantf.dequant_8x8)
                            .expect(
                                "non-null function pointer",
                            )(
                            (dct8x8[idx_0 as usize]).as_mut_ptr(),
                            (*h).dequant8_mf[quant_cat as usize],
                            i_qp,
                        );
                        ((*h).dctf.add8x8_idct8)
                            .expect(
                                "non-null function pointer",
                            )(
                            &mut *(*((*h).mb.pic.p_fdec)
                                .as_mut_ptr()
                                .offset(p_7 as isize))
                                .offset(
                                    (8 as libc::c_int * (idx_0 & 1 as libc::c_int)
                                        + 8 as libc::c_int * (idx_0 >> 1 as libc::c_int)
                                            * 32 as libc::c_int) as isize,
                                ),
                            (dct8x8[idx_0 as usize]).as_mut_ptr(),
                        );
                        (*(&mut *((*h).mb.cache.non_zero_count)
                            .as_mut_ptr()
                            .offset(
                                (*x264_scan8
                                    .as_ptr()
                                    .offset(
                                        (p_7 * 16 as libc::c_int + idx_0 * 4 as libc::c_int)
                                            as isize,
                                    ) as libc::c_int + 0 as libc::c_int) as isize,
                            ) as *mut uint8_t as *mut x264_union16_t))
                            .i = (1 as libc::c_int * 0x101 as libc::c_int) as uint16_t;
                        (*(&mut *((*h).mb.cache.non_zero_count)
                            .as_mut_ptr()
                            .offset(
                                (*x264_scan8
                                    .as_ptr()
                                    .offset(
                                        (p_7 * 16 as libc::c_int + idx_0 * 4 as libc::c_int)
                                            as isize,
                                    ) as libc::c_int + 8 as libc::c_int) as isize,
                            ) as *mut uint8_t as *mut x264_union16_t))
                            .i = (1 as libc::c_int * 0x101 as libc::c_int) as uint16_t;
                        idx_0 += 1;
                        idx_0;
                    }
                }
                p_7 += 1;
                p_7;
                i_qp = (*h).mb.i_chroma_qp;
            }
        } else {
            let mut dct4x4: [[dctcoef; 16]; 16] = [[0; 16]; 16];
            let mut p_8: libc::c_int = 0 as libc::c_int;
            while p_8 < plane_count {
                let mut quant_cat_0: libc::c_int = if p_8 != 0 {
                    CQM_4PC as libc::c_int
                } else {
                    CQM_4PY as libc::c_int
                };
                (*(&mut *((*h).mb.cache.non_zero_count)
                    .as_mut_ptr()
                    .offset(
                        (*x264_scan8.as_ptr().offset((16 as libc::c_int * p_8) as isize)
                            as libc::c_int + 0 as libc::c_int * 8 as libc::c_int)
                            as isize,
                    ) as *mut uint8_t as *mut x264_union32_t))
                    .i = 0 as libc::c_int as uint32_t;
                (*(&mut *((*h).mb.cache.non_zero_count)
                    .as_mut_ptr()
                    .offset(
                        (*x264_scan8.as_ptr().offset((16 as libc::c_int * p_8) as isize)
                            as libc::c_int + 1 as libc::c_int * 8 as libc::c_int)
                            as isize,
                    ) as *mut uint8_t as *mut x264_union32_t))
                    .i = 0 as libc::c_int as uint32_t;
                (*(&mut *((*h).mb.cache.non_zero_count)
                    .as_mut_ptr()
                    .offset(
                        (*x264_scan8.as_ptr().offset((16 as libc::c_int * p_8) as isize)
                            as libc::c_int + 2 as libc::c_int * 8 as libc::c_int)
                            as isize,
                    ) as *mut uint8_t as *mut x264_union32_t))
                    .i = 0 as libc::c_int as uint32_t;
                (*(&mut *((*h).mb.cache.non_zero_count)
                    .as_mut_ptr()
                    .offset(
                        (*x264_scan8.as_ptr().offset((16 as libc::c_int * p_8) as isize)
                            as libc::c_int + 3 as libc::c_int * 8 as libc::c_int)
                            as isize,
                    ) as *mut uint8_t as *mut x264_union32_t))
                    .i = 0 as libc::c_int as uint32_t;
                ((*h).dctf.sub16x16_dct)
                    .expect(
                        "non-null function pointer",
                    )(
                    dct4x4.as_mut_ptr(),
                    (*h).mb.pic.p_fenc[p_8 as usize],
                    (*h).mb.pic.p_fdec[p_8 as usize],
                );
                if (*h).mb.b_noise_reduction != 0 {
                    let fresh2 = &mut (*((*h).nr_count)
                        .offset(
                            (0 as libc::c_int
                                + (p_8 != 0) as libc::c_int * 2 as libc::c_int) as isize,
                        ));
                    *fresh2 = (*fresh2).wrapping_add(16 as libc::c_int as uint32_t);
                    let mut idx_1: libc::c_int = 0 as libc::c_int;
                    while idx_1 < 16 as libc::c_int {
                        ((*h).quantf.denoise_dct)
                            .expect(
                                "non-null function pointer",
                            )(
                            (dct4x4[idx_1 as usize]).as_mut_ptr(),
                            (*((*h).nr_residual_sum)
                                .offset(
                                    (0 as libc::c_int
                                        + (p_8 != 0) as libc::c_int * 2 as libc::c_int) as isize,
                                ))
                                .as_mut_ptr(),
                            (*((*h).nr_offset)
                                .offset(
                                    (0 as libc::c_int
                                        + (p_8 != 0) as libc::c_int * 2 as libc::c_int) as isize,
                                ))
                                .as_mut_ptr(),
                            16 as libc::c_int,
                        );
                        idx_1 += 1;
                        idx_1;
                    }
                }
                let mut plane_cbp_0: libc::c_int = 0 as libc::c_int;
                let mut i8x8_0: libc::c_int = 0 as libc::c_int;
                while i8x8_0 < 4 as libc::c_int {
                    let mut i_decimate_8x8_0: libc::c_int = if b_decimate != 0 {
                        0 as libc::c_int
                    } else {
                        6 as libc::c_int
                    };
                    let mut nnz8x8: libc::c_int = 0 as libc::c_int;
                    if (*h).mb.b_trellis != 0 {
                        let mut i4x4_0: libc::c_int = 0 as libc::c_int;
                        while i4x4_0 < 4 as libc::c_int {
                            let mut idx_2: libc::c_int = i8x8_0 * 4 as libc::c_int
                                + i4x4_0;
                            if x264_8_quant_4x4_trellis(
                                h,
                                (dct4x4[idx_2 as usize]).as_mut_ptr(),
                                quant_cat_0,
                                i_qp,
                                ctx_cat_plane[DCT_LUMA_4x4 as libc::c_int
                                    as usize][p_8 as usize] as libc::c_int,
                                0 as libc::c_int,
                                (p_8 != 0) as libc::c_int,
                                p_8 * 16 as libc::c_int + idx_2,
                            ) != 0
                            {
                                ((*h).zigzagf.scan_4x4)
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    ((*h)
                                        .dct
                                        .luma4x4[(p_8 * 16 as libc::c_int + idx_2) as usize])
                                        .as_mut_ptr(),
                                    (dct4x4[idx_2 as usize]).as_mut_ptr(),
                                );
                                ((*h).quantf.dequant_4x4)
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    (dct4x4[idx_2 as usize]).as_mut_ptr(),
                                    (*h).dequant4_mf[quant_cat_0 as usize],
                                    i_qp,
                                );
                                if i_decimate_8x8_0 < 6 as libc::c_int {
                                    i_decimate_8x8_0
                                        += ((*h).quantf.decimate_score16)
                                            .expect(
                                                "non-null function pointer",
                                            )(
                                            ((*h)
                                                .dct
                                                .luma4x4[(p_8 * 16 as libc::c_int + idx_2) as usize])
                                                .as_mut_ptr(),
                                        );
                                }
                                (*h)
                                    .mb
                                    .cache
                                    .non_zero_count[x264_scan8[(p_8 * 16 as libc::c_int + idx_2)
                                    as usize] as usize] = 1 as libc::c_int as uint8_t;
                                nnz8x8 = 1 as libc::c_int;
                            }
                            i4x4_0 += 1;
                            i4x4_0;
                        }
                    } else {
                        nz = ((*h).quantf.quant_4x4x4)
                            .expect(
                                "non-null function pointer",
                            )(
                            &mut *dct4x4
                                .as_mut_ptr()
                                .offset((i8x8_0 * 4 as libc::c_int) as isize),
                            (*((*h).quant4_mf[quant_cat_0 as usize])
                                .offset(i_qp as isize))
                                .as_mut_ptr(),
                            (*((*h).quant4_bias[quant_cat_0 as usize])
                                .offset(i_qp as isize))
                                .as_mut_ptr(),
                        );
                        nnz8x8 = nz;
                        if nz != 0 {
                            let mut idx_3: libc::c_int = i8x8_0 * 4 as libc::c_int;
                            let mut msk_0: libc::c_int = nz;
                            let mut skip_0: libc::c_int = 0;
                            while msk_0 != 0
                                && {
                                    skip_0 = x264_ctz_4bit(msk_0 as uint32_t);
                                    idx_3 += skip_0;
                                    msk_0 >>= skip_0 + 1 as libc::c_int;
                                    1 as libc::c_int != 0
                                }
                            {
                                ((*h).zigzagf.scan_4x4)
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    ((*h)
                                        .dct
                                        .luma4x4[(p_8 * 16 as libc::c_int + idx_3) as usize])
                                        .as_mut_ptr(),
                                    (dct4x4[idx_3 as usize]).as_mut_ptr(),
                                );
                                ((*h).quantf.dequant_4x4)
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    (dct4x4[idx_3 as usize]).as_mut_ptr(),
                                    (*h).dequant4_mf[quant_cat_0 as usize],
                                    i_qp,
                                );
                                if i_decimate_8x8_0 < 6 as libc::c_int {
                                    i_decimate_8x8_0
                                        += ((*h).quantf.decimate_score16)
                                            .expect(
                                                "non-null function pointer",
                                            )(
                                            ((*h)
                                                .dct
                                                .luma4x4[(p_8 * 16 as libc::c_int + idx_3) as usize])
                                                .as_mut_ptr(),
                                        );
                                }
                                (*h)
                                    .mb
                                    .cache
                                    .non_zero_count[x264_scan8[(p_8 * 16 as libc::c_int + idx_3)
                                    as usize] as usize] = 1 as libc::c_int as uint8_t;
                                idx_3 += 1;
                                idx_3;
                            }
                        }
                    }
                    if nnz8x8 != 0 {
                        i_decimate_mb += i_decimate_8x8_0;
                        if i_decimate_8x8_0 < 4 as libc::c_int {
                            (*(&mut *((*h).mb.cache.non_zero_count)
                                .as_mut_ptr()
                                .offset(
                                    (*x264_scan8
                                        .as_ptr()
                                        .offset(
                                            (p_8 * 16 as libc::c_int + i8x8_0 * 4 as libc::c_int)
                                                as isize,
                                        ) as libc::c_int + 0 as libc::c_int) as isize,
                                ) as *mut uint8_t as *mut x264_union16_t))
                                .i = (0 as libc::c_int * 0x101 as libc::c_int) as uint16_t;
                            (*(&mut *((*h).mb.cache.non_zero_count)
                                .as_mut_ptr()
                                .offset(
                                    (*x264_scan8
                                        .as_ptr()
                                        .offset(
                                            (p_8 * 16 as libc::c_int + i8x8_0 * 4 as libc::c_int)
                                                as isize,
                                        ) as libc::c_int + 8 as libc::c_int) as isize,
                                ) as *mut uint8_t as *mut x264_union16_t))
                                .i = (0 as libc::c_int * 0x101 as libc::c_int) as uint16_t;
                        } else {
                            plane_cbp_0 |= (1 as libc::c_int) << i8x8_0;
                        }
                    }
                    i8x8_0 += 1;
                    i8x8_0;
                }
                if i_decimate_mb < 6 as libc::c_int {
                    plane_cbp_0 = 0 as libc::c_int;
                    (*(&mut *((*h).mb.cache.non_zero_count)
                        .as_mut_ptr()
                        .offset(
                            (*x264_scan8
                                .as_ptr()
                                .offset((16 as libc::c_int * p_8) as isize) as libc::c_int
                                + 0 as libc::c_int * 8 as libc::c_int) as isize,
                        ) as *mut uint8_t as *mut x264_union32_t))
                        .i = 0 as libc::c_int as uint32_t;
                    (*(&mut *((*h).mb.cache.non_zero_count)
                        .as_mut_ptr()
                        .offset(
                            (*x264_scan8
                                .as_ptr()
                                .offset((16 as libc::c_int * p_8) as isize) as libc::c_int
                                + 1 as libc::c_int * 8 as libc::c_int) as isize,
                        ) as *mut uint8_t as *mut x264_union32_t))
                        .i = 0 as libc::c_int as uint32_t;
                    (*(&mut *((*h).mb.cache.non_zero_count)
                        .as_mut_ptr()
                        .offset(
                            (*x264_scan8
                                .as_ptr()
                                .offset((16 as libc::c_int * p_8) as isize) as libc::c_int
                                + 2 as libc::c_int * 8 as libc::c_int) as isize,
                        ) as *mut uint8_t as *mut x264_union32_t))
                        .i = 0 as libc::c_int as uint32_t;
                    (*(&mut *((*h).mb.cache.non_zero_count)
                        .as_mut_ptr()
                        .offset(
                            (*x264_scan8
                                .as_ptr()
                                .offset((16 as libc::c_int * p_8) as isize) as libc::c_int
                                + 3 as libc::c_int * 8 as libc::c_int) as isize,
                        ) as *mut uint8_t as *mut x264_union32_t))
                        .i = 0 as libc::c_int as uint32_t;
                } else {
                    (*h).mb.i_cbp_luma |= plane_cbp_0;
                    let mut i8x8_1: libc::c_int = 0 as libc::c_int;
                    let mut msk_1: libc::c_int = plane_cbp_0;
                    let mut skip_1: libc::c_int = 0;
                    while msk_1 != 0
                        && {
                            skip_1 = x264_ctz_4bit(msk_1 as uint32_t);
                            i8x8_1 += skip_1;
                            msk_1 >>= skip_1 + 1 as libc::c_int;
                            1 as libc::c_int != 0
                        }
                    {
                        ((*h).dctf.add8x8_idct)
                            .expect(
                                "non-null function pointer",
                            )(
                            &mut *(*((*h).mb.pic.p_fdec)
                                .as_mut_ptr()
                                .offset(p_8 as isize))
                                .offset(
                                    ((i8x8_1 & 1 as libc::c_int) * 8 as libc::c_int
                                        + (i8x8_1 >> 1 as libc::c_int) * 8 as libc::c_int
                                            * 32 as libc::c_int) as isize,
                                ),
                            &mut *dct4x4
                                .as_mut_ptr()
                                .offset((i8x8_1 * 4 as libc::c_int) as isize),
                        );
                        i8x8_1 += 1;
                        i8x8_1;
                    }
                }
                p_8 += 1;
                p_8;
                i_qp = (*h).mb.i_chroma_qp;
            }
        }
    }
    if chroma != 0 {
        if (*h).mb.i_type == I_4x4 as libc::c_int
            || (*h).mb.i_type == I_8x8 as libc::c_int
            || (*h).mb.i_type == I_16x16 as libc::c_int
            || (*h).mb.i_type == I_PCM as libc::c_int
        {
            let mut i_mode_1: libc::c_int = (*h).mb.i_chroma_pred_mode;
            if (*h).mb.b_lossless != 0 {
                x264_8_predict_lossless_chroma(h, i_mode_1);
            } else {
                ((*h).predict_chroma[i_mode_1 as usize])
                    .expect(
                        "non-null function pointer",
                    )((*h).mb.pic.p_fdec[1 as libc::c_int as usize]);
                ((*h).predict_chroma[i_mode_1 as usize])
                    .expect(
                        "non-null function pointer",
                    )((*h).mb.pic.p_fdec[2 as libc::c_int as usize]);
            }
        }
        x264_8_mb_encode_chroma(
            h,
            !((*h).mb.i_type == I_4x4 as libc::c_int
                || (*h).mb.i_type == I_8x8 as libc::c_int
                || (*h).mb.i_type == I_16x16 as libc::c_int
                || (*h).mb.i_type == I_PCM as libc::c_int) as libc::c_int,
            (*h).mb.i_chroma_qp,
        );
    } else {
        (*h).mb.i_cbp_chroma = 0 as libc::c_int;
    }
    let mut cbp: libc::c_int = ((*h).mb.i_cbp_chroma << 4 as libc::c_int) | (*h).mb.i_cbp_luma;
    if (*h).param.b_cabac != 0 {
        cbp
            |= (((*h)
                .mb
                .cache
                .non_zero_count[x264_scan8[48 as libc::c_int as usize] as usize]
                as libc::c_int) << 8 as libc::c_int) | (((*h)
                    .mb
                    .cache
                    .non_zero_count[x264_scan8[(49 as libc::c_int + 0 as libc::c_int)
                    as usize] as usize] as libc::c_int) << 9 as libc::c_int) | (((*h)
                    .mb
                    .cache
                    .non_zero_count[x264_scan8[(49 as libc::c_int + 1 as libc::c_int)
                    as usize] as usize] as libc::c_int) << 10 as libc::c_int);
    }
    *((*h).mb.cbp).offset((*h).mb.i_mb_xy as isize) = cbp as int16_t;
    if b_force_no_skip == 0 {
        if (*h).mb.i_type == P_L0 as libc::c_int
            && (*h).mb.i_partition == D_16x16 as libc::c_int
            && (*h).mb.i_cbp_luma | (*h).mb.i_cbp_chroma == 0
            && (*(((*h)
                .mb
                .cache
                .mv[0 as libc::c_int
                as usize][x264_scan8[0 as libc::c_int as usize] as usize])
                .as_mut_ptr() as *mut x264_union32_t))
                .i == (*(((*h).mb.cache.pskip_mv).as_mut_ptr() as *mut x264_union32_t)).i
            && (*h)
                .mb
                .cache
                .ref_0[0 as libc::c_int
                as usize][x264_scan8[0 as libc::c_int as usize] as usize] as libc::c_int
                == 0 as libc::c_int
        {
            (*h).mb.i_type = P_SKIP as libc::c_int;
        }
        if (*h).mb.i_type == B_DIRECT as libc::c_int
            && (*h).mb.i_cbp_luma | (*h).mb.i_cbp_chroma == 0
        {
            (*h).mb.i_type = B_SKIP as libc::c_int;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_macroblock_encode(mut h: *mut x264_t) {
    if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as libc::c_int {
        macroblock_encode_internal(h, 3 as libc::c_int, 0 as libc::c_int);
    } else if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc != 0 {
        macroblock_encode_internal(h, 1 as libc::c_int, 1 as libc::c_int);
    } else {
        macroblock_encode_internal(h, 1 as libc::c_int, 0 as libc::c_int);
    };
}
#[inline(always)]
unsafe extern "C" fn macroblock_probe_skip_internal(
    mut h: *mut x264_t,
    mut b_bidir: libc::c_int,
    mut plane_count: libc::c_int,
    mut chroma: libc::c_int,
) -> libc::c_int {
    let mut dct4x4: [[dctcoef; 16]; 8] = [[0; 16]; 8];
    let mut dctscan: [dctcoef; 16] = [0; 16];
    let mut mvp: [int16_t; 2] = [0; 2];
    let mut i_qp: libc::c_int = (*h).mb.i_qp;
    let mut p: libc::c_int = 0 as libc::c_int;
    while p < plane_count {
        let mut quant_cat: libc::c_int = if p != 0 {
            CQM_4PC as libc::c_int
        } else {
            CQM_4PY as libc::c_int
        };
        if b_bidir == 0 {
            mvp[0 as libc::c_int
                as usize] = x264_clip3(
                (*h).mb.cache.pskip_mv[0 as libc::c_int as usize] as libc::c_int,
                (*h).mb.mv_min[0 as libc::c_int as usize],
                (*h).mb.mv_max[0 as libc::c_int as usize],
            ) as int16_t;
            mvp[1 as libc::c_int
                as usize] = x264_clip3(
                (*h).mb.cache.pskip_mv[1 as libc::c_int as usize] as libc::c_int,
                (*h).mb.mv_min[1 as libc::c_int as usize],
                (*h).mb.mv_max[1 as libc::c_int as usize],
            ) as int16_t;
            ((*h).mc.mc_luma)
                .expect(
                    "non-null function pointer",
                )(
                (*h).mb.pic.p_fdec[p as usize],
                32 as libc::c_int as intptr_t,
                &mut *(*(*((*h).mb.pic.p_fref)
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize))
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize))
                    .as_mut_ptr()
                    .offset((p * 4 as libc::c_int) as isize),
                (*h).mb.pic.i_stride[p as usize] as intptr_t,
                mvp[0 as libc::c_int as usize] as libc::c_int,
                mvp[1 as libc::c_int as usize] as libc::c_int,
                16 as libc::c_int,
                16 as libc::c_int,
                &mut *(*((*h).sh.weight).as_mut_ptr().offset(0 as libc::c_int as isize))
                    .as_mut_ptr()
                    .offset(p as isize),
            );
        }
        let mut i8x8: libc::c_int = 0 as libc::c_int;
        let mut i_decimate_mb: libc::c_int = 0 as libc::c_int;
        while i8x8 < 4 as libc::c_int {
            let mut fenc_offset: libc::c_int = (i8x8 & 1 as libc::c_int)
                * 8 as libc::c_int
                + (i8x8 >> 1 as libc::c_int) * 16 as libc::c_int * 8 as libc::c_int;
            let mut fdec_offset: libc::c_int = (i8x8 & 1 as libc::c_int)
                * 8 as libc::c_int
                + (i8x8 >> 1 as libc::c_int) * 32 as libc::c_int * 8 as libc::c_int;
            ((*h).dctf.sub8x8_dct)
                .expect(
                    "non-null function pointer",
                )(
                dct4x4.as_mut_ptr(),
                ((*h).mb.pic.p_fenc[p as usize]).offset(fenc_offset as isize),
                ((*h).mb.pic.p_fdec[p as usize]).offset(fdec_offset as isize),
            );
            if (*h).mb.b_noise_reduction != 0 {
                let mut i4x4: libc::c_int = 0 as libc::c_int;
                while i4x4 < 4 as libc::c_int {
                    ((*h).quantf.denoise_dct)
                        .expect(
                            "non-null function pointer",
                        )(
                        (dct4x4[i4x4 as usize]).as_mut_ptr(),
                        (*((*h).nr_residual_sum)
                            .offset(
                                (0 as libc::c_int
                                    + (p != 0) as libc::c_int * 2 as libc::c_int) as isize,
                            ))
                            .as_mut_ptr(),
                        (*((*h).nr_offset)
                            .offset(
                                (0 as libc::c_int
                                    + (p != 0) as libc::c_int * 2 as libc::c_int) as isize,
                            ))
                            .as_mut_ptr(),
                        16 as libc::c_int,
                    );
                    i4x4 += 1;
                    i4x4;
                }
            }
            let mut nz: libc::c_int = ((*h).quantf.quant_4x4x4)
                .expect(
                    "non-null function pointer",
                )(
                dct4x4.as_mut_ptr(),
                (*((*h).quant4_mf[quant_cat as usize]).offset(i_qp as isize))
                    .as_mut_ptr(),
                (*((*h).quant4_bias[quant_cat as usize]).offset(i_qp as isize))
                    .as_mut_ptr(),
            );
            let mut idx: libc::c_int = 0 as libc::c_int;
            let mut msk: libc::c_int = nz;
            let mut skip: libc::c_int = 0;
            while msk != 0
                && {
                    skip = x264_ctz_4bit(msk as uint32_t);
                    idx += skip;
                    msk >>= skip + 1 as libc::c_int;
                    1 as libc::c_int != 0
                }
            {
                ((*h).zigzagf.scan_4x4)
                    .expect(
                        "non-null function pointer",
                    )(dctscan.as_mut_ptr(), (dct4x4[idx as usize]).as_mut_ptr());
                i_decimate_mb
                    += ((*h).quantf.decimate_score16)
                        .expect("non-null function pointer")(dctscan.as_mut_ptr());
                if i_decimate_mb >= 6 as libc::c_int {
                    return 0 as libc::c_int;
                }
                idx += 1;
                idx;
            }
            i8x8 += 1;
            i8x8;
        }
        p += 1;
        p;
        i_qp = (*h).mb.i_chroma_qp;
    }
    if chroma == CHROMA_420 as libc::c_int || chroma == CHROMA_422 as libc::c_int {
        i_qp = (*h).mb.i_chroma_qp;
        let mut chroma422: libc::c_int = (chroma == CHROMA_422 as libc::c_int)
            as libc::c_int;
        let mut thresh: libc::c_int = if chroma422 != 0 {
            (x264_lambda2_tab[i_qp as usize] + 16 as libc::c_int) >> 5 as libc::c_int
        } else {
            (x264_lambda2_tab[i_qp as usize] + 32 as libc::c_int) >> 6 as libc::c_int
        };
        let mut ssd: libc::c_int = 0;
        let mut dct_dc: [dctcoef; 8] = [0; 8];
        if b_bidir == 0 {
            if (*(mvp.as_mut_ptr() as *mut x264_union32_t)).i != 0 {
                ((*h).mc.mc_chroma)
                    .expect(
                        "non-null function pointer",
                    )(
                    (*h).mb.pic.p_fdec[1 as libc::c_int as usize],
                    (*h).mb.pic.p_fdec[2 as libc::c_int as usize],
                    32 as libc::c_int as intptr_t,
                    (*h)
                        .mb
                        .pic
                        .p_fref[0 as libc::c_int
                        as usize][0 as libc::c_int as usize][4 as libc::c_int as usize],
                    (*h).mb.pic.i_stride[1 as libc::c_int as usize] as intptr_t,
                    mvp[0 as libc::c_int as usize] as libc::c_int,
                    mvp[1 as libc::c_int as usize] as libc::c_int
                        * ((1 as libc::c_int) << chroma422),
                    8 as libc::c_int,
                    if chroma422 != 0 { 16 as libc::c_int } else { 8 as libc::c_int },
                );
            } else {
                ((*h).mc.load_deinterleave_chroma_fdec)
                    .expect(
                        "non-null function pointer",
                    )(
                    (*h).mb.pic.p_fdec[1 as libc::c_int as usize],
                    (*h)
                        .mb
                        .pic
                        .p_fref[0 as libc::c_int
                        as usize][0 as libc::c_int as usize][4 as libc::c_int as usize],
                    (*h).mb.pic.i_stride[1 as libc::c_int as usize] as intptr_t,
                    if chroma422 != 0 { 16 as libc::c_int } else { 8 as libc::c_int },
                );
            }
        }
        let mut ch: libc::c_int = 0 as libc::c_int;
        while ch < 2 as libc::c_int {
            let mut p_src: *mut pixel = (*h)
                .mb
                .pic
                .p_fenc[(1 as libc::c_int + ch) as usize];
            let mut p_dst: *mut pixel = (*h)
                .mb
                .pic
                .p_fdec[(1 as libc::c_int + ch) as usize];
            if b_bidir == 0
                && !((*h)
                    .sh
                    .weight[0 as libc::c_int as usize][(1 as libc::c_int + ch) as usize]
                    .weightfn)
                    .is_null()
            {
                (*((*h)
                    .sh
                    .weight[0 as libc::c_int as usize][(1 as libc::c_int + ch) as usize]
                    .weightfn)
                    .offset((8 as libc::c_int >> 2 as libc::c_int) as isize))
                    .expect(
                        "non-null function pointer",
                    )(
                    (*h).mb.pic.p_fdec[(1 as libc::c_int + ch) as usize],
                    32 as libc::c_int as intptr_t,
                    (*h).mb.pic.p_fdec[(1 as libc::c_int + ch) as usize],
                    32 as libc::c_int as intptr_t,
                    &mut *(*((*h).sh.weight)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset((1 as libc::c_int + ch) as isize),
                    if chroma422 != 0 { 16 as libc::c_int } else { 8 as libc::c_int },
                );
            }
            ssd = ((*h)
                .pixf
                .ssd[(if chroma422 != 0 {
                PIXEL_8x16 as libc::c_int
            } else {
                PIXEL_8x8 as libc::c_int
            }) as usize])
                .expect(
                    "non-null function pointer",
                )(
                p_dst,
                32 as libc::c_int as intptr_t,
                p_src,
                16 as libc::c_int as intptr_t,
            );
            if ssd >= thresh {
                if (*h).mb.b_noise_reduction != 0 {
                    let mut i: libc::c_int = 0 as libc::c_int;
                    while i <= chroma422 {
                        ((*h).dctf.sub8x8_dct)
                            .expect(
                                "non-null function pointer",
                            )(
                            &mut *dct4x4
                                .as_mut_ptr()
                                .offset((4 as libc::c_int * i) as isize),
                            p_src
                                .offset(
                                    (8 as libc::c_int * i * 16 as libc::c_int) as isize,
                                ),
                            p_dst
                                .offset((8 as libc::c_int * i * 32 as libc::c_int) as isize),
                        );
                        i += 1;
                        i;
                    }
                    let mut i4x4_0: libc::c_int = 0 as libc::c_int;
                    while i4x4_0
                        < (if chroma422 != 0 {
                            8 as libc::c_int
                        } else {
                            4 as libc::c_int
                        })
                    {
                        ((*h).quantf.denoise_dct)
                            .expect(
                                "non-null function pointer",
                            )(
                            (dct4x4[i4x4_0 as usize]).as_mut_ptr(),
                            (*((*h).nr_residual_sum).offset(2 as libc::c_int as isize))
                                .as_mut_ptr(),
                            (*((*h).nr_offset).offset(2 as libc::c_int as isize))
                                .as_mut_ptr(),
                            16 as libc::c_int,
                        );
                        dct_dc[i4x4_0
                            as usize] = dct4x4[i4x4_0
                            as usize][0 as libc::c_int as usize];
                        dct4x4[i4x4_0
                            as usize][0 as libc::c_int
                            as usize] = 0 as libc::c_int as dctcoef;
                        i4x4_0 += 1;
                        i4x4_0;
                    }
                } else if chroma422 != 0 {
                    ((*h).dctf.sub8x16_dct_dc)
                        .expect(
                            "non-null function pointer",
                        )(dct_dc.as_mut_ptr(), p_src, p_dst);
                } else {
                    ((*h).dctf.sub8x8_dct_dc)
                        .expect(
                            "non-null function pointer",
                        )(dct_dc.as_mut_ptr(), p_src, p_dst);
                }
                let mut i_0: libc::c_int = 0 as libc::c_int;
                while i_0 <= chroma422 {
                    if ((*h).quantf.quant_2x2_dc)
                        .expect(
                            "non-null function pointer",
                        )(
                        &mut *dct_dc
                            .as_mut_ptr()
                            .offset((4 as libc::c_int * i_0) as isize),
                        (*((*h).quant4_mf[CQM_4PC as libc::c_int as usize])
                            .offset(
                                (i_qp + 3 as libc::c_int * chroma422) as isize,
                            ))[0 as libc::c_int as usize] as libc::c_int
                            >> 1 as libc::c_int,
                        ((*((*h).quant4_bias[CQM_4PC as libc::c_int as usize])
                            .offset(
                                (i_qp + 3 as libc::c_int * chroma422) as isize,
                            ))[0 as libc::c_int as usize] as libc::c_int)
                            << 1 as libc::c_int,
                    ) != 0
                    {
                        return 0 as libc::c_int;
                    }
                    i_0 += 1;
                    i_0;
                }
                if ssd >= thresh * 4 as libc::c_int {
                    if (*h).mb.b_noise_reduction == 0 {
                        let mut i_1: libc::c_int = 0 as libc::c_int;
                        while i_1 <= chroma422 {
                            ((*h).dctf.sub8x8_dct)
                                .expect(
                                    "non-null function pointer",
                                )(
                                &mut *dct4x4
                                    .as_mut_ptr()
                                    .offset((4 as libc::c_int * i_1) as isize),
                                p_src
                                    .offset(
                                        (8 as libc::c_int * i_1 * 16 as libc::c_int) as isize,
                                    ),
                                p_dst
                                    .offset(
                                        (8 as libc::c_int * i_1 * 32 as libc::c_int) as isize,
                                    ),
                            );
                            dct4x4[(i_1 * 4 as libc::c_int + 0 as libc::c_int)
                                as usize][0 as libc::c_int
                                as usize] = 0 as libc::c_int as dctcoef;
                            dct4x4[(i_1 * 4 as libc::c_int + 1 as libc::c_int)
                                as usize][0 as libc::c_int
                                as usize] = 0 as libc::c_int as dctcoef;
                            dct4x4[(i_1 * 4 as libc::c_int + 2 as libc::c_int)
                                as usize][0 as libc::c_int
                                as usize] = 0 as libc::c_int as dctcoef;
                            dct4x4[(i_1 * 4 as libc::c_int + 3 as libc::c_int)
                                as usize][0 as libc::c_int
                                as usize] = 0 as libc::c_int as dctcoef;
                            i_1 += 1;
                            i_1;
                        }
                    }
                    let mut i8x8_0: libc::c_int = 0 as libc::c_int;
                    let mut i_decimate_mb_0: libc::c_int = 0 as libc::c_int;
                    while i8x8_0
                        < (if chroma422 != 0 {
                            2 as libc::c_int
                        } else {
                            1 as libc::c_int
                        })
                    {
                        let mut nz_0: libc::c_int = ((*h).quantf.quant_4x4x4)
                            .expect(
                                "non-null function pointer",
                            )(
                            &mut *dct4x4
                                .as_mut_ptr()
                                .offset((i8x8_0 * 4 as libc::c_int) as isize),
                            (*((*h).quant4_mf[CQM_4PC as libc::c_int as usize])
                                .offset(i_qp as isize))
                                .as_mut_ptr(),
                            (*((*h).quant4_bias[CQM_4PC as libc::c_int as usize])
                                .offset(i_qp as isize))
                                .as_mut_ptr(),
                        );
                        let mut idx_0: libc::c_int = i8x8_0 * 4 as libc::c_int;
                        let mut msk_0: libc::c_int = nz_0;
                        let mut skip_0: libc::c_int = 0;
                        while msk_0 != 0
                            && {
                                skip_0 = x264_ctz_4bit(msk_0 as uint32_t);
                                idx_0 += skip_0;
                                msk_0 >>= skip_0 + 1 as libc::c_int;
                                1 as libc::c_int != 0
                            }
                        {
                            ((*h).zigzagf.scan_4x4)
                                .expect(
                                    "non-null function pointer",
                                )(
                                dctscan.as_mut_ptr(),
                                (dct4x4[idx_0 as usize]).as_mut_ptr(),
                            );
                            i_decimate_mb_0
                                += ((*h).quantf.decimate_score15)
                                    .expect("non-null function pointer")(dctscan.as_mut_ptr());
                            if i_decimate_mb_0 >= 7 as libc::c_int {
                                return 0 as libc::c_int;
                            }
                            idx_0 += 1;
                            idx_0;
                        }
                        i8x8_0 += 1;
                        i8x8_0;
                    }
                }
            }
            ch += 1;
            ch;
        }
    }
    (*h).mb.b_skip_mc = 1 as libc::c_int;
    1 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_macroblock_probe_skip(
    mut h: *mut x264_t,
    mut b_bidir: libc::c_int,
) -> libc::c_int {
    if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_420 as libc::c_int {
        macroblock_probe_skip_internal(
            h,
            b_bidir,
            1 as libc::c_int,
            CHROMA_420 as libc::c_int,
        )
    } else if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_422 as libc::c_int
    {
        return macroblock_probe_skip_internal(
            h,
            b_bidir,
            1 as libc::c_int,
            CHROMA_422 as libc::c_int,
        )
    } else if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as libc::c_int
    {
        return macroblock_probe_skip_internal(
            h,
            b_bidir,
            3 as libc::c_int,
            CHROMA_444 as libc::c_int,
        )
    } else {
        return macroblock_probe_skip_internal(
            h,
            b_bidir,
            1 as libc::c_int,
            CHROMA_400 as libc::c_int,
        )
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_noise_reduction_update(mut h: *mut x264_t) {
    (*h).nr_offset = ((*h).nr_offset_denoise).as_mut_ptr();
    (*h)
        .nr_residual_sum = ((*h).nr_residual_sum_buf[0 as libc::c_int as usize])
        .as_mut_ptr();
    (*h).nr_count = ((*h).nr_count_buf[0 as libc::c_int as usize]).as_mut_ptr();
    let mut cat: libc::c_int = 0 as libc::c_int;
    while cat
        < 3 as libc::c_int
            + ((*((*h).sps).as_mut_ptr()).i_chroma_format_idc
                == CHROMA_444 as libc::c_int) as libc::c_int
    {
        let mut dct8x8: libc::c_int = cat & 1 as libc::c_int;
        let mut size: libc::c_int = if dct8x8 != 0 {
            64 as libc::c_int
        } else {
            16 as libc::c_int
        };
        let mut weight: *const uint32_t = if dct8x8 != 0 {
            x264_dct8_weight2_tab.as_ptr()
        } else {
            x264_dct4_weight2_tab.as_ptr()
        };
        if *((*h).nr_count).offset(cat as isize)
            > (if dct8x8 != 0 {
                (1 as libc::c_int) << 16 as libc::c_int
            } else {
                (1 as libc::c_int) << 18 as libc::c_int
            }) as uint32_t
        {
            let mut i: libc::c_int = 0 as libc::c_int;
            while i < size {
                (*((*h).nr_residual_sum).offset(cat as isize))[i as usize]
                    >>= 1 as libc::c_int;
                i += 1;
                i;
            }
            *((*h).nr_count).offset(cat as isize) >>= 1 as libc::c_int;
        }
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < size {
            (*((*h).nr_offset)
                .offset(
                    cat as isize,
                ))[i_0
                as usize] = (((*h).param.analyse.i_noise_reduction as uint64_t
                * *((*h).nr_count).offset(cat as isize) as uint64_t)
                .wrapping_add(
                    ((*((*h).nr_residual_sum).offset(cat as isize))[i_0 as usize]
                        / 2 as libc::c_int as uint32_t) as uint64_t,
                )
                / ((*((*h).nr_residual_sum).offset(cat as isize))[i_0 as usize]
                    as uint64_t * *weight.offset(i_0 as isize) as uint64_t
                    / 256 as libc::c_int as uint64_t)
                    .wrapping_add(1 as libc::c_int as uint64_t)) as udctcoef;
            i_0 += 1;
            i_0;
        }
        (*((*h).nr_offset)
            .offset(
                cat as isize,
            ))[0 as libc::c_int as usize] = 0 as libc::c_int as udctcoef;
        cat += 1;
        cat;
    }
}
#[inline(always)]
unsafe extern "C" fn macroblock_encode_p8x8_internal(
    mut h: *mut x264_t,
    mut i8: libc::c_int,
    mut plane_count: libc::c_int,
    mut chroma: libc::c_int,
) {
    let mut b_decimate: libc::c_int = (*h).mb.b_dct_decimate;
    let mut i_qp: libc::c_int = (*h).mb.i_qp;
    let mut x: libc::c_int = i8 & 1 as libc::c_int;
    let mut y: libc::c_int = i8 >> 1 as libc::c_int;
    let mut nz: libc::c_int = 0;
    let mut chroma422: libc::c_int = (chroma == CHROMA_422 as libc::c_int)
        as libc::c_int;
    (*h).mb.i_cbp_chroma = 0 as libc::c_int;
    (*h).mb.i_cbp_luma &= !((1 as libc::c_int) << i8);
    if (*h).mb.b_skip_mc == 0 {
        x264_8_mb_mc_8x8(h, i8);
    }
    if (*h).mb.b_lossless != 0 {
        let mut p: libc::c_int = 0 as libc::c_int;
        while p < plane_count {
            let mut p_fenc: *mut pixel = ((*h).mb.pic.p_fenc[p as usize])
                .offset((8 as libc::c_int * x) as isize)
                .offset((8 as libc::c_int * y * 16 as libc::c_int) as isize);
            let mut p_fdec: *mut pixel = ((*h).mb.pic.p_fdec[p as usize])
                .offset((8 as libc::c_int * x) as isize)
                .offset((8 as libc::c_int * y * 32 as libc::c_int) as isize);
            let mut nnz8x8: libc::c_int = 0 as libc::c_int;
            if (*h).mb.b_transform_8x8 != 0 {
                nnz8x8 = ((*h).zigzagf.sub_8x8)
                    .expect(
                        "non-null function pointer",
                    )(
                    ((*h).dct.luma8x8[(4 as libc::c_int * p + i8) as usize])
                        .as_mut_ptr(),
                    p_fenc,
                    p_fdec,
                );
                (*(&mut *((*h).mb.cache.non_zero_count)
                    .as_mut_ptr()
                    .offset(
                        (*x264_scan8
                            .as_ptr()
                            .offset(
                                (p * 16 as libc::c_int + i8 * 4 as libc::c_int) as isize,
                            ) as libc::c_int + 0 as libc::c_int) as isize,
                    ) as *mut uint8_t as *mut x264_union16_t))
                    .i = (nnz8x8 * 0x101 as libc::c_int) as uint16_t;
                (*(&mut *((*h).mb.cache.non_zero_count)
                    .as_mut_ptr()
                    .offset(
                        (*x264_scan8
                            .as_ptr()
                            .offset(
                                (p * 16 as libc::c_int + i8 * 4 as libc::c_int) as isize,
                            ) as libc::c_int + 8 as libc::c_int) as isize,
                    ) as *mut uint8_t as *mut x264_union16_t))
                    .i = (nnz8x8 * 0x101 as libc::c_int) as uint16_t;
            } else {
                let mut i4: libc::c_int = i8 * 4 as libc::c_int;
                while i4 < i8 * 4 as libc::c_int + 4 as libc::c_int {
                    nz = ((*h).zigzagf.sub_4x4)
                        .expect(
                            "non-null function pointer",
                        )(
                        ((*h).dct.luma4x4[(16 as libc::c_int * p + i4) as usize])
                            .as_mut_ptr(),
                        ((*h).mb.pic.p_fenc[p as usize])
                            .offset(
                                block_idx_xy_fenc[i4 as usize] as libc::c_int as isize,
                            ),
                        ((*h).mb.pic.p_fdec[p as usize])
                            .offset(
                                block_idx_xy_fdec[i4 as usize] as libc::c_int as isize,
                            ),
                    );
                    (*h)
                        .mb
                        .cache
                        .non_zero_count[x264_scan8[(16 as libc::c_int * p + i4) as usize]
                        as usize] = nz as uint8_t;
                    nnz8x8 |= nz;
                    i4 += 1;
                    i4;
                }
            }
            (*h).mb.i_cbp_luma |= nnz8x8 << i8;
            p += 1;
            p;
        }
        if chroma == CHROMA_420 as libc::c_int || chroma == CHROMA_422 as libc::c_int {
            let mut ch: libc::c_int = 0 as libc::c_int;
            while ch < 2 as libc::c_int {
                let mut dc: dctcoef = 0;
                let mut p_fenc_0: *mut pixel = ((*h)
                    .mb
                    .pic
                    .p_fenc[(1 as libc::c_int + ch) as usize])
                    .offset((4 as libc::c_int * x) as isize)
                    .offset(
                        ((if chroma422 != 0 {
                            8 as libc::c_int
                        } else {
                            4 as libc::c_int
                        }) * y * 16 as libc::c_int) as isize,
                    );
                let mut p_fdec_0: *mut pixel = ((*h)
                    .mb
                    .pic
                    .p_fdec[(1 as libc::c_int + ch) as usize])
                    .offset((4 as libc::c_int * x) as isize)
                    .offset(
                        ((if chroma422 != 0 {
                            8 as libc::c_int
                        } else {
                            4 as libc::c_int
                        }) * y * 32 as libc::c_int) as isize,
                    );
                let mut i4x4: libc::c_int = 0 as libc::c_int;
                while i4x4 <= chroma422 {
                    let mut offset: libc::c_int = if chroma422 != 0 {
                        8 as libc::c_int * y + 2 as libc::c_int * i4x4 + x
                    } else {
                        i8
                    };
                    nz = ((*h).zigzagf.sub_4x4ac)
                        .expect(
                            "non-null function pointer",
                        )(
                        ((*h)
                            .dct
                            .luma4x4[(16 as libc::c_int + offset
                            + ch * 16 as libc::c_int) as usize])
                            .as_mut_ptr(),
                        p_fenc_0
                            .offset(
                                (4 as libc::c_int * i4x4 * 16 as libc::c_int) as isize,
                            ),
                        p_fdec_0
                            .offset(
                                (4 as libc::c_int * i4x4 * 32 as libc::c_int) as isize,
                            ),
                        &mut dc,
                    );
                    (*h)
                        .mb
                        .cache
                        .non_zero_count[x264_scan8[(16 as libc::c_int + offset
                        + ch * 16 as libc::c_int) as usize] as usize] = nz as uint8_t;
                    i4x4 += 1;
                    i4x4;
                }
                ch += 1;
                ch;
            }
            (*h).mb.i_cbp_chroma = 0x2 as libc::c_int;
        }
    } else {
        if (*h).mb.b_transform_8x8 != 0 {
            let mut p_0: libc::c_int = 0 as libc::c_int;
            while p_0 < plane_count {
                let mut quant_cat: libc::c_int = if p_0 != 0 {
                    CQM_8PC as libc::c_int
                } else {
                    CQM_8PY as libc::c_int
                };
                let mut p_fenc_1: *mut pixel = ((*h).mb.pic.p_fenc[p_0 as usize])
                    .offset((8 as libc::c_int * x) as isize)
                    .offset((8 as libc::c_int * y * 16 as libc::c_int) as isize);
                let mut p_fdec_1: *mut pixel = ((*h).mb.pic.p_fdec[p_0 as usize])
                    .offset((8 as libc::c_int * x) as isize)
                    .offset((8 as libc::c_int * y * 32 as libc::c_int) as isize);
                let mut dct8x8: [dctcoef; 64] = [0; 64];
                ((*h).dctf.sub8x8_dct8)
                    .expect(
                        "non-null function pointer",
                    )(dct8x8.as_mut_ptr(), p_fenc_1, p_fdec_1);
                let mut nnz8x8_0: libc::c_int = x264_quant_8x8(
                    h,
                    dct8x8.as_mut_ptr(),
                    i_qp,
                    ctx_cat_plane[DCT_LUMA_8x8 as libc::c_int as usize][p_0 as usize]
                        as libc::c_int,
                    0 as libc::c_int,
                    p_0,
                    i8,
                );
                if nnz8x8_0 != 0 {
                    ((*h).zigzagf.scan_8x8)
                        .expect(
                            "non-null function pointer",
                        )(
                        ((*h).dct.luma8x8[(4 as libc::c_int * p_0 + i8) as usize])
                            .as_mut_ptr(),
                        dct8x8.as_mut_ptr(),
                    );
                    if b_decimate != 0 && (*h).mb.b_trellis == 0 {
                        nnz8x8_0 = (4 as libc::c_int
                            <= ((*h).quantf.decimate_score64)
                                .expect(
                                    "non-null function pointer",
                                )(
                                ((*h).dct.luma8x8[(4 as libc::c_int * p_0 + i8) as usize])
                                    .as_mut_ptr(),
                            )) as libc::c_int;
                    }
                    if nnz8x8_0 != 0 {
                        ((*h).quantf.dequant_8x8)
                            .expect(
                                "non-null function pointer",
                            )(
                            dct8x8.as_mut_ptr(),
                            (*h).dequant8_mf[quant_cat as usize],
                            i_qp,
                        );
                        ((*h).dctf.add8x8_idct8)
                            .expect(
                                "non-null function pointer",
                            )(p_fdec_1, dct8x8.as_mut_ptr());
                        (*(&mut *((*h).mb.cache.non_zero_count)
                            .as_mut_ptr()
                            .offset(
                                (*x264_scan8
                                    .as_ptr()
                                    .offset(
                                        (p_0 * 16 as libc::c_int + i8 * 4 as libc::c_int) as isize,
                                    ) as libc::c_int + 0 as libc::c_int) as isize,
                            ) as *mut uint8_t as *mut x264_union16_t))
                            .i = (1 as libc::c_int * 0x101 as libc::c_int) as uint16_t;
                        (*(&mut *((*h).mb.cache.non_zero_count)
                            .as_mut_ptr()
                            .offset(
                                (*x264_scan8
                                    .as_ptr()
                                    .offset(
                                        (p_0 * 16 as libc::c_int + i8 * 4 as libc::c_int) as isize,
                                    ) as libc::c_int + 8 as libc::c_int) as isize,
                            ) as *mut uint8_t as *mut x264_union16_t))
                            .i = (1 as libc::c_int * 0x101 as libc::c_int) as uint16_t;
                        (*h).mb.i_cbp_luma |= (1 as libc::c_int) << i8;
                    } else {
                        (*(&mut *((*h).mb.cache.non_zero_count)
                            .as_mut_ptr()
                            .offset(
                                (*x264_scan8
                                    .as_ptr()
                                    .offset(
                                        (p_0 * 16 as libc::c_int + i8 * 4 as libc::c_int) as isize,
                                    ) as libc::c_int + 0 as libc::c_int) as isize,
                            ) as *mut uint8_t as *mut x264_union16_t))
                            .i = (0 as libc::c_int * 0x101 as libc::c_int) as uint16_t;
                        (*(&mut *((*h).mb.cache.non_zero_count)
                            .as_mut_ptr()
                            .offset(
                                (*x264_scan8
                                    .as_ptr()
                                    .offset(
                                        (p_0 * 16 as libc::c_int + i8 * 4 as libc::c_int) as isize,
                                    ) as libc::c_int + 8 as libc::c_int) as isize,
                            ) as *mut uint8_t as *mut x264_union16_t))
                            .i = (0 as libc::c_int * 0x101 as libc::c_int) as uint16_t;
                    }
                } else {
                    (*(&mut *((*h).mb.cache.non_zero_count)
                        .as_mut_ptr()
                        .offset(
                            (*x264_scan8
                                .as_ptr()
                                .offset(
                                    (p_0 * 16 as libc::c_int + i8 * 4 as libc::c_int) as isize,
                                ) as libc::c_int + 0 as libc::c_int) as isize,
                        ) as *mut uint8_t as *mut x264_union16_t))
                        .i = (0 as libc::c_int * 0x101 as libc::c_int) as uint16_t;
                    (*(&mut *((*h).mb.cache.non_zero_count)
                        .as_mut_ptr()
                        .offset(
                            (*x264_scan8
                                .as_ptr()
                                .offset(
                                    (p_0 * 16 as libc::c_int + i8 * 4 as libc::c_int) as isize,
                                ) as libc::c_int + 8 as libc::c_int) as isize,
                        ) as *mut uint8_t as *mut x264_union16_t))
                        .i = (0 as libc::c_int * 0x101 as libc::c_int) as uint16_t;
                }
                p_0 += 1;
                p_0;
                i_qp = (*h).mb.i_chroma_qp;
            }
        } else {
            let mut p_1: libc::c_int = 0 as libc::c_int;
            while p_1 < plane_count {
                let mut quant_cat_0: libc::c_int = if p_1 != 0 {
                    CQM_4PC as libc::c_int
                } else {
                    CQM_4PY as libc::c_int
                };
                let mut p_fenc_2: *mut pixel = ((*h).mb.pic.p_fenc[p_1 as usize])
                    .offset((8 as libc::c_int * x) as isize)
                    .offset((8 as libc::c_int * y * 16 as libc::c_int) as isize);
                let mut p_fdec_2: *mut pixel = ((*h).mb.pic.p_fdec[p_1 as usize])
                    .offset((8 as libc::c_int * x) as isize)
                    .offset((8 as libc::c_int * y * 32 as libc::c_int) as isize);
                let mut i_decimate_8x8: libc::c_int = if b_decimate != 0 {
                    0 as libc::c_int
                } else {
                    4 as libc::c_int
                };
                let mut dct4x4: [[dctcoef; 16]; 4] = [[0; 16]; 4];
                let mut nnz8x8_1: libc::c_int = 0 as libc::c_int;
                ((*h).dctf.sub8x8_dct)
                    .expect(
                        "non-null function pointer",
                    )(dct4x4.as_mut_ptr(), p_fenc_2, p_fdec_2);
                (*(&mut *((*h).mb.cache.non_zero_count)
                    .as_mut_ptr()
                    .offset(
                        (*x264_scan8
                            .as_ptr()
                            .offset(
                                (p_1 * 16 as libc::c_int + i8 * 4 as libc::c_int) as isize,
                            ) as libc::c_int + 0 as libc::c_int) as isize,
                    ) as *mut uint8_t as *mut x264_union16_t))
                    .i = (0 as libc::c_int * 0x101 as libc::c_int) as uint16_t;
                (*(&mut *((*h).mb.cache.non_zero_count)
                    .as_mut_ptr()
                    .offset(
                        (*x264_scan8
                            .as_ptr()
                            .offset(
                                (p_1 * 16 as libc::c_int + i8 * 4 as libc::c_int) as isize,
                            ) as libc::c_int + 8 as libc::c_int) as isize,
                    ) as *mut uint8_t as *mut x264_union16_t))
                    .i = (0 as libc::c_int * 0x101 as libc::c_int) as uint16_t;
                if (*h).mb.b_noise_reduction != 0 {
                    let mut idx: libc::c_int = 0 as libc::c_int;
                    while idx < 4 as libc::c_int {
                        ((*h).quantf.denoise_dct)
                            .expect(
                                "non-null function pointer",
                            )(
                            (dct4x4[idx as usize]).as_mut_ptr(),
                            (*((*h).nr_residual_sum)
                                .offset(
                                    (0 as libc::c_int
                                        + (p_1 != 0) as libc::c_int * 2 as libc::c_int) as isize,
                                ))
                                .as_mut_ptr(),
                            (*((*h).nr_offset)
                                .offset(
                                    (0 as libc::c_int
                                        + (p_1 != 0) as libc::c_int * 2 as libc::c_int) as isize,
                                ))
                                .as_mut_ptr(),
                            16 as libc::c_int,
                        );
                        idx += 1;
                        idx;
                    }
                }
                if (*h).mb.b_trellis != 0 {
                    let mut i4x4_0: libc::c_int = 0 as libc::c_int;
                    while i4x4_0 < 4 as libc::c_int {
                        if x264_8_quant_4x4_trellis(
                            h,
                            (dct4x4[i4x4_0 as usize]).as_mut_ptr(),
                            quant_cat_0,
                            i_qp,
                            ctx_cat_plane[DCT_LUMA_4x4 as libc::c_int
                                as usize][p_1 as usize] as libc::c_int,
                            0 as libc::c_int,
                            (p_1 != 0) as libc::c_int,
                            i8 * 4 as libc::c_int + i4x4_0 + p_1 * 16 as libc::c_int,
                        ) != 0
                        {
                            ((*h).zigzagf.scan_4x4)
                                .expect(
                                    "non-null function pointer",
                                )(
                                ((*h)
                                    .dct
                                    .luma4x4[(p_1 * 16 as libc::c_int + i8 * 4 as libc::c_int
                                    + i4x4_0) as usize])
                                    .as_mut_ptr(),
                                (dct4x4[i4x4_0 as usize]).as_mut_ptr(),
                            );
                            ((*h).quantf.dequant_4x4)
                                .expect(
                                    "non-null function pointer",
                                )(
                                (dct4x4[i4x4_0 as usize]).as_mut_ptr(),
                                (*h).dequant4_mf[quant_cat_0 as usize],
                                i_qp,
                            );
                            if i_decimate_8x8 < 4 as libc::c_int {
                                i_decimate_8x8
                                    += ((*h).quantf.decimate_score16)
                                        .expect(
                                            "non-null function pointer",
                                        )(
                                        ((*h)
                                            .dct
                                            .luma4x4[(p_1 * 16 as libc::c_int + i8 * 4 as libc::c_int
                                            + i4x4_0) as usize])
                                            .as_mut_ptr(),
                                    );
                            }
                            (*h)
                                .mb
                                .cache
                                .non_zero_count[x264_scan8[(p_1 * 16 as libc::c_int
                                + i8 * 4 as libc::c_int + i4x4_0) as usize]
                                as usize] = 1 as libc::c_int as uint8_t;
                            nnz8x8_1 = 1 as libc::c_int;
                        }
                        i4x4_0 += 1;
                        i4x4_0;
                    }
                } else {
                    nz = ((*h).quantf.quant_4x4x4)
                        .expect(
                            "non-null function pointer",
                        )(
                        dct4x4.as_mut_ptr(),
                        (*((*h).quant4_mf[quant_cat_0 as usize]).offset(i_qp as isize))
                            .as_mut_ptr(),
                        (*((*h).quant4_bias[quant_cat_0 as usize]).offset(i_qp as isize))
                            .as_mut_ptr(),
                    );
                    nnz8x8_1 = nz;
                    if nz != 0 {
                        let mut i4x4_1: libc::c_int = 0 as libc::c_int;
                        let mut msk: libc::c_int = nz;
                        let mut skip: libc::c_int = 0;
                        while msk != 0
                            && {
                                skip = x264_ctz_4bit(msk as uint32_t);
                                i4x4_1 += skip;
                                msk >>= skip + 1 as libc::c_int;
                                1 as libc::c_int != 0
                            }
                        {
                            ((*h).zigzagf.scan_4x4)
                                .expect(
                                    "non-null function pointer",
                                )(
                                ((*h)
                                    .dct
                                    .luma4x4[(p_1 * 16 as libc::c_int + i8 * 4 as libc::c_int
                                    + i4x4_1) as usize])
                                    .as_mut_ptr(),
                                (dct4x4[i4x4_1 as usize]).as_mut_ptr(),
                            );
                            ((*h).quantf.dequant_4x4)
                                .expect(
                                    "non-null function pointer",
                                )(
                                (dct4x4[i4x4_1 as usize]).as_mut_ptr(),
                                (*h).dequant4_mf[quant_cat_0 as usize],
                                i_qp,
                            );
                            if i_decimate_8x8 < 4 as libc::c_int {
                                i_decimate_8x8
                                    += ((*h).quantf.decimate_score16)
                                        .expect(
                                            "non-null function pointer",
                                        )(
                                        ((*h)
                                            .dct
                                            .luma4x4[(p_1 * 16 as libc::c_int + i8 * 4 as libc::c_int
                                            + i4x4_1) as usize])
                                            .as_mut_ptr(),
                                    );
                            }
                            (*h)
                                .mb
                                .cache
                                .non_zero_count[x264_scan8[(p_1 * 16 as libc::c_int
                                + i8 * 4 as libc::c_int + i4x4_1) as usize]
                                as usize] = 1 as libc::c_int as uint8_t;
                            i4x4_1 += 1;
                            i4x4_1;
                        }
                    }
                }
                if nnz8x8_1 != 0 {
                    if i_decimate_8x8 < 4 as libc::c_int {
                        (*(&mut *((*h).mb.cache.non_zero_count)
                            .as_mut_ptr()
                            .offset(
                                (*x264_scan8
                                    .as_ptr()
                                    .offset(
                                        (p_1 * 16 as libc::c_int + i8 * 4 as libc::c_int) as isize,
                                    ) as libc::c_int + 0 as libc::c_int) as isize,
                            ) as *mut uint8_t as *mut x264_union16_t))
                            .i = (0 as libc::c_int * 0x101 as libc::c_int) as uint16_t;
                        (*(&mut *((*h).mb.cache.non_zero_count)
                            .as_mut_ptr()
                            .offset(
                                (*x264_scan8
                                    .as_ptr()
                                    .offset(
                                        (p_1 * 16 as libc::c_int + i8 * 4 as libc::c_int) as isize,
                                    ) as libc::c_int + 8 as libc::c_int) as isize,
                            ) as *mut uint8_t as *mut x264_union16_t))
                            .i = (0 as libc::c_int * 0x101 as libc::c_int) as uint16_t;
                    } else {
                        ((*h).dctf.add8x8_idct)
                            .expect(
                                "non-null function pointer",
                            )(p_fdec_2, dct4x4.as_mut_ptr());
                        (*h).mb.i_cbp_luma |= (1 as libc::c_int) << i8;
                    }
                }
                p_1 += 1;
                p_1;
                i_qp = (*h).mb.i_chroma_qp;
            }
        }
        if chroma == CHROMA_420 as libc::c_int || chroma == CHROMA_422 as libc::c_int {
            i_qp = (*h).mb.i_chroma_qp;
            let mut ch_0: libc::c_int = 0 as libc::c_int;
            while ch_0 < 2 as libc::c_int {
                let mut dct4x4_0: [[dctcoef; 16]; 2] = [[0; 16]; 2];
                let mut p_fenc_3: *mut pixel = ((*h)
                    .mb
                    .pic
                    .p_fenc[(1 as libc::c_int + ch_0) as usize])
                    .offset((4 as libc::c_int * x) as isize)
                    .offset(
                        ((if chroma422 != 0 {
                            8 as libc::c_int
                        } else {
                            4 as libc::c_int
                        }) * y * 16 as libc::c_int) as isize,
                    );
                let mut p_fdec_3: *mut pixel = ((*h)
                    .mb
                    .pic
                    .p_fdec[(1 as libc::c_int + ch_0) as usize])
                    .offset((4 as libc::c_int * x) as isize)
                    .offset(
                        ((if chroma422 != 0 {
                            8 as libc::c_int
                        } else {
                            4 as libc::c_int
                        }) * y * 32 as libc::c_int) as isize,
                    );
                let mut i4x4_2: libc::c_int = 0 as libc::c_int;
                while i4x4_2 <= chroma422 {
                    ((*h).dctf.sub4x4_dct)
                        .expect(
                            "non-null function pointer",
                        )(
                        (dct4x4_0[i4x4_2 as usize]).as_mut_ptr(),
                        p_fenc_3
                            .offset(
                                (4 as libc::c_int * i4x4_2 * 16 as libc::c_int) as isize,
                            ),
                        p_fdec_3
                            .offset(
                                (4 as libc::c_int * i4x4_2 * 32 as libc::c_int) as isize,
                            ),
                    );
                    if (*h).mb.b_noise_reduction != 0 {
                        ((*h).quantf.denoise_dct)
                            .expect(
                                "non-null function pointer",
                            )(
                            (dct4x4_0[i4x4_2 as usize]).as_mut_ptr(),
                            (*((*h).nr_residual_sum).offset(2 as libc::c_int as isize))
                                .as_mut_ptr(),
                            (*((*h).nr_offset).offset(2 as libc::c_int as isize))
                                .as_mut_ptr(),
                            16 as libc::c_int,
                        );
                    }
                    dct4x4_0[i4x4_2
                        as usize][0 as libc::c_int
                        as usize] = 0 as libc::c_int as dctcoef;
                    if (*h).mb.b_trellis != 0 {
                        nz = x264_8_quant_4x4_trellis(
                            h,
                            (dct4x4_0[i4x4_2 as usize]).as_mut_ptr(),
                            CQM_4PC as libc::c_int,
                            i_qp,
                            DCT_CHROMA_AC as libc::c_int,
                            0 as libc::c_int,
                            1 as libc::c_int,
                            0 as libc::c_int,
                        );
                    } else {
                        nz = ((*h).quantf.quant_4x4)
                            .expect(
                                "non-null function pointer",
                            )(
                            (dct4x4_0[i4x4_2 as usize]).as_mut_ptr(),
                            (*((*h).quant4_mf[CQM_4PC as libc::c_int as usize])
                                .offset(i_qp as isize))
                                .as_mut_ptr(),
                            (*((*h).quant4_bias[CQM_4PC as libc::c_int as usize])
                                .offset(i_qp as isize))
                                .as_mut_ptr(),
                        );
                    }
                    let mut offset_0: libc::c_int = if chroma422 != 0 {
                        ((5 as libc::c_int * i8) & 0x9 as libc::c_int)
                            + 2 as libc::c_int * i4x4_2
                    } else {
                        i8
                    };
                    (*h)
                        .mb
                        .cache
                        .non_zero_count[x264_scan8[(16 as libc::c_int + offset_0
                        + ch_0 * 16 as libc::c_int) as usize] as usize] = nz as uint8_t;
                    if nz != 0 {
                        ((*h).zigzagf.scan_4x4)
                            .expect(
                                "non-null function pointer",
                            )(
                            ((*h)
                                .dct
                                .luma4x4[(16 as libc::c_int + offset_0
                                + ch_0 * 16 as libc::c_int) as usize])
                                .as_mut_ptr(),
                            (dct4x4_0[i4x4_2 as usize]).as_mut_ptr(),
                        );
                        ((*h).quantf.dequant_4x4)
                            .expect(
                                "non-null function pointer",
                            )(
                            (dct4x4_0[i4x4_2 as usize]).as_mut_ptr(),
                            (*h).dequant4_mf[CQM_4PC as libc::c_int as usize],
                            i_qp,
                        );
                        ((*h).dctf.add4x4_idct)
                            .expect(
                                "non-null function pointer",
                            )(
                            p_fdec_3
                                .offset(
                                    (4 as libc::c_int * i4x4_2 * 32 as libc::c_int) as isize,
                                ),
                            (dct4x4_0[i4x4_2 as usize]).as_mut_ptr(),
                        );
                    }
                    i4x4_2 += 1;
                    i4x4_2;
                }
                ch_0 += 1;
                ch_0;
            }
            (*h).mb.i_cbp_chroma = 0x2 as libc::c_int;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_macroblock_encode_p8x8(
    mut h: *mut x264_t,
    mut i8: libc::c_int,
) {
    if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_420 as libc::c_int {
        macroblock_encode_p8x8_internal(
            h,
            i8,
            1 as libc::c_int,
            CHROMA_420 as libc::c_int,
        );
    } else if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_422 as libc::c_int
    {
        macroblock_encode_p8x8_internal(
            h,
            i8,
            1 as libc::c_int,
            CHROMA_422 as libc::c_int,
        );
    } else if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as libc::c_int
    {
        macroblock_encode_p8x8_internal(
            h,
            i8,
            3 as libc::c_int,
            CHROMA_444 as libc::c_int,
        );
    } else {
        macroblock_encode_p8x8_internal(
            h,
            i8,
            1 as libc::c_int,
            CHROMA_400 as libc::c_int,
        );
    };
}
#[inline(always)]
unsafe extern "C" fn macroblock_encode_p4x4_internal(
    mut h: *mut x264_t,
    mut i4: libc::c_int,
    mut plane_count: libc::c_int,
) {
    let mut i_qp: libc::c_int = (*h).mb.i_qp;
    let mut p: libc::c_int = 0 as libc::c_int;
    while p < plane_count {
        let mut quant_cat: libc::c_int = if p != 0 {
            CQM_4PC as libc::c_int
        } else {
            CQM_4PY as libc::c_int
        };
        let mut p_fenc: *mut pixel = &mut *(*((*h).mb.pic.p_fenc)
            .as_mut_ptr()
            .offset(p as isize))
            .offset(*block_idx_xy_fenc.as_ptr().offset(i4 as isize) as isize)
            as *mut pixel;
        let mut p_fdec: *mut pixel = &mut *(*((*h).mb.pic.p_fdec)
            .as_mut_ptr()
            .offset(p as isize))
            .offset(*block_idx_xy_fdec.as_ptr().offset(i4 as isize) as isize)
            as *mut pixel;
        let mut nz: libc::c_int = 0;
        if (*h).mb.b_lossless != 0 {
            nz = ((*h).zigzagf.sub_4x4)
                .expect(
                    "non-null function pointer",
                )(
                ((*h).dct.luma4x4[(p * 16 as libc::c_int + i4) as usize]).as_mut_ptr(),
                p_fenc,
                p_fdec,
            );
            (*h)
                .mb
                .cache
                .non_zero_count[x264_scan8[(p * 16 as libc::c_int + i4) as usize]
                as usize] = nz as uint8_t;
        } else {
            let mut dct4x4: [dctcoef; 16] = [0; 16];
            ((*h).dctf.sub4x4_dct)
                .expect(
                    "non-null function pointer",
                )(dct4x4.as_mut_ptr(), p_fenc, p_fdec);
            nz = x264_quant_4x4(
                h,
                dct4x4.as_mut_ptr(),
                i_qp,
                ctx_cat_plane[DCT_LUMA_4x4 as libc::c_int as usize][p as usize]
                    as libc::c_int,
                0 as libc::c_int,
                p,
                i4,
            );
            (*h)
                .mb
                .cache
                .non_zero_count[x264_scan8[(p * 16 as libc::c_int + i4) as usize]
                as usize] = nz as uint8_t;
            if nz != 0 {
                ((*h).zigzagf.scan_4x4)
                    .expect(
                        "non-null function pointer",
                    )(
                    ((*h).dct.luma4x4[(p * 16 as libc::c_int + i4) as usize])
                        .as_mut_ptr(),
                    dct4x4.as_mut_ptr(),
                );
                ((*h).quantf.dequant_4x4)
                    .expect(
                        "non-null function pointer",
                    )(dct4x4.as_mut_ptr(), (*h).dequant4_mf[quant_cat as usize], i_qp);
                ((*h).dctf.add4x4_idct)
                    .expect("non-null function pointer")(p_fdec, dct4x4.as_mut_ptr());
            }
        }
        p += 1;
        p;
        i_qp = (*h).mb.i_chroma_qp;
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_macroblock_encode_p4x4(
    mut h: *mut x264_t,
    mut i8: libc::c_int,
) {
    if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as libc::c_int {
        macroblock_encode_p4x4_internal(h, i8, 3 as libc::c_int);
    } else {
        macroblock_encode_p4x4_internal(h, i8, 1 as libc::c_int);
    };
}
