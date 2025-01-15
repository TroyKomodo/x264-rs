use crate::types::*;
extern "C" {
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fseeko(__stream: *mut FILE, __off: __off64_t, __whence: libc::c_int) -> libc::c_int;
    fn ftello(__stream: *mut FILE) -> __off64_t;
    fn x264_8_log(h: *mut x264_t, i_level: libc::c_int, psz_fmt: *const libc::c_char, _: ...);
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn x264_malloc(_: int64_t) -> *mut libc::c_void;
    fn x264_free(_: *mut libc::c_void);
    fn dlopen(__file: *const libc::c_char, __mode: libc::c_int) -> *mut libc::c_void;
    fn dlclose(__handle: *mut libc::c_void) -> libc::c_int;
    fn dlsym(__handle: *mut libc::c_void, __name: *const libc::c_char) -> *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_opencl_load_library() -> *mut x264_opencl_function_t {
    let mut ocl: *mut x264_opencl_function_t = std::ptr::null_mut::<x264_opencl_function_t>();
    ocl = x264_malloc(::core::mem::size_of::<x264_opencl_function_t>() as libc::c_ulong as int64_t)
        as *mut x264_opencl_function_t;
    if !ocl.is_null() {
        memset(
            ocl as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<x264_opencl_function_t>() as libc::c_ulong,
        );
        (*ocl).library = dlopen(
            b"libOpenCL.so\0" as *const u8 as *const libc::c_char,
            0x2 as libc::c_int,
        );
        if !((*ocl).library).is_null() {
            (*ocl).clBuildProgram =
                ::core::mem::transmute::<*mut libc::c_void, clBuildProgram_func>(dlsym(
                    (*ocl).library,
                    b"clBuildProgram\0" as *const u8 as *const libc::c_char,
                ));
            if !(0 as libc::c_int == 0 && ((*ocl).clBuildProgram).is_none()) {
                (*ocl).clCreateBuffer =
                    ::core::mem::transmute::<*mut libc::c_void, clCreateBuffer_func>(dlsym(
                        (*ocl).library,
                        b"clCreateBuffer\0" as *const u8 as *const libc::c_char,
                    ));
                if !(0 as libc::c_int == 0 && ((*ocl).clCreateBuffer).is_none()) {
                    (*ocl).clCreateCommandQueue = ::core::mem::transmute::<
                        *mut libc::c_void,
                        clCreateCommandQueue_func,
                    >(dlsym(
                        (*ocl).library,
                        b"clCreateCommandQueue\0" as *const u8 as *const libc::c_char,
                    ));
                    if !(0 as libc::c_int == 0 && ((*ocl).clCreateCommandQueue).is_none()) {
                        (*ocl).clCreateContext = ::core::mem::transmute::<
                            *mut libc::c_void,
                            clCreateContext_func,
                        >(dlsym(
                            (*ocl).library,
                            b"clCreateContext\0" as *const u8 as *const libc::c_char,
                        ));
                        if !(0 as libc::c_int == 0 && ((*ocl).clCreateContext).is_none()) {
                            (*ocl).clCreateImage2D = ::core::mem::transmute::<
                                *mut libc::c_void,
                                clCreateImage2D_func,
                            >(dlsym(
                                (*ocl).library,
                                b"clCreateImage2D\0" as *const u8 as *const libc::c_char,
                            ));
                            if !(0 as libc::c_int == 0 && ((*ocl).clCreateImage2D).is_none()) {
                                (*ocl).clCreateKernel = ::core::mem::transmute::<
                                    *mut libc::c_void,
                                    clCreateKernel_func,
                                >(dlsym(
                                    (*ocl).library,
                                    b"clCreateKernel\0" as *const u8 as *const libc::c_char,
                                ));
                                if !(0 as libc::c_int == 0 && ((*ocl).clCreateKernel).is_none()) {
                                    (*ocl).clCreateProgramWithBinary =
                                        ::core::mem::transmute::<
                                            *mut libc::c_void,
                                            clCreateProgramWithBinary_func,
                                        >(dlsym(
                                            (*ocl).library,
                                            b"clCreateProgramWithBinary\0" as *const u8
                                                as *const libc::c_char,
                                        ));
                                    if !(0 as libc::c_int == 0
                                        && ((*ocl).clCreateProgramWithBinary).is_none())
                                    {
                                        (*ocl).clCreateProgramWithSource =
                                            ::core::mem::transmute::<
                                                *mut libc::c_void,
                                                clCreateProgramWithSource_func,
                                            >(dlsym(
                                                (*ocl).library,
                                                b"clCreateProgramWithSource\0" as *const u8
                                                    as *const libc::c_char,
                                            ));
                                        if !(0 as libc::c_int == 0
                                            && ((*ocl).clCreateProgramWithSource).is_none())
                                        {
                                            (*ocl).clEnqueueCopyBuffer = ::core::mem::transmute::<
                                                *mut libc::c_void,
                                                clEnqueueCopyBuffer_func,
                                            >(
                                                dlsym(
                                                    (*ocl).library,
                                                    b"clEnqueueCopyBuffer\0" as *const u8
                                                        as *const libc::c_char,
                                                ),
                                            );
                                            if !(0 as libc::c_int == 0
                                                && ((*ocl).clEnqueueCopyBuffer).is_none())
                                            {
                                                (*ocl).clEnqueueMapBuffer = ::core::mem::transmute::<
                                                    *mut libc::c_void,
                                                    clEnqueueMapBuffer_func,
                                                >(
                                                    dlsym(
                                                        (*ocl).library,
                                                        b"clEnqueueMapBuffer\0" as *const u8
                                                            as *const libc::c_char,
                                                    ),
                                                );
                                                if !(0 as libc::c_int == 0
                                                    && ((*ocl).clEnqueueMapBuffer).is_none())
                                                {
                                                    (*ocl).clEnqueueNDRangeKernel =
                                                        ::core::mem::transmute::<
                                                            *mut libc::c_void,
                                                            clEnqueueNDRangeKernel_func,
                                                        >(
                                                            dlsym(
                                                                (*ocl).library,
                                                                b"clEnqueueNDRangeKernel\0"
                                                                    as *const u8
                                                                    as *const libc::c_char,
                                                            ),
                                                        );
                                                    if !(0 as libc::c_int == 0
                                                        && ((*ocl).clEnqueueNDRangeKernel)
                                                            .is_none())
                                                    {
                                                        (*ocl).clEnqueueReadBuffer =
                                                            ::core::mem::transmute::<
                                                                *mut libc::c_void,
                                                                clEnqueueReadBuffer_func,
                                                            >(
                                                                dlsym(
                                                                    (*ocl).library,
                                                                    b"clEnqueueReadBuffer\0"
                                                                        as *const u8
                                                                        as *const libc::c_char,
                                                                ),
                                                            );
                                                        if !(0 as libc::c_int == 0
                                                            && ((*ocl).clEnqueueReadBuffer)
                                                                .is_none())
                                                        {
                                                            (*ocl).clEnqueueWriteBuffer =
                                                                ::core::mem::transmute::<
                                                                    *mut libc::c_void,
                                                                    clEnqueueWriteBuffer_func,
                                                                >(
                                                                    dlsym(
                                                                        (*ocl).library,
                                                                        b"clEnqueueWriteBuffer\0"
                                                                            as *const u8
                                                                            as *const libc::c_char,
                                                                    ),
                                                                );
                                                            if !(0 as libc::c_int == 0
                                                                && ((*ocl).clEnqueueWriteBuffer)
                                                                    .is_none())
                                                            {
                                                                (*ocl).clFinish = ::core::mem::transmute::<
                                                                    *mut libc::c_void,
                                                                    clFinish_func,
                                                                >(
                                                                    dlsym(
                                                                        (*ocl).library,
                                                                        b"clFinish\0" as *const u8 as *const libc::c_char,
                                                                    ),
                                                                );
                                                                if !(0 as libc::c_int == 0
                                                                    && ((*ocl).clFinish).is_none())
                                                                {
                                                                    (*ocl).clGetCommandQueueInfo = ::core::mem::transmute::<
                                                                        *mut libc::c_void,
                                                                        clGetCommandQueueInfo_func,
                                                                    >(
                                                                        dlsym(
                                                                            (*ocl).library,
                                                                            b"clGetCommandQueueInfo\0" as *const u8
                                                                                as *const libc::c_char,
                                                                        ),
                                                                    );
                                                                    if !(0 as libc::c_int == 0
                                                                        && ((*ocl)
                                                                            .clGetCommandQueueInfo)
                                                                            .is_none())
                                                                    {
                                                                        (*ocl).clGetDeviceIDs = ::core::mem::transmute::<
                                                                            *mut libc::c_void,
                                                                            clGetDeviceIDs_func,
                                                                        >(
                                                                            dlsym(
                                                                                (*ocl).library,
                                                                                b"clGetDeviceIDs\0" as *const u8 as *const libc::c_char,
                                                                            ),
                                                                        );
                                                                        if !(0 as libc::c_int == 0
                                                                            && ((*ocl)
                                                                                .clGetDeviceIDs)
                                                                                .is_none())
                                                                        {
                                                                            (*ocl).clGetDeviceInfo = ::core::mem::transmute::<
                                                                                *mut libc::c_void,
                                                                                clGetDeviceInfo_func,
                                                                            >(
                                                                                dlsym(
                                                                                    (*ocl).library,
                                                                                    b"clGetDeviceInfo\0" as *const u8 as *const libc::c_char,
                                                                                ),
                                                                            );
                                                                            if !(0 as libc::c_int == 0
                                                                                && ((*ocl).clGetDeviceInfo).is_none())
                                                                            {
                                                                                (*ocl).clGetKernelWorkGroupInfo = ::core::mem::transmute::<
                                                                                    *mut libc::c_void,
                                                                                    clGetKernelWorkGroupInfo_func,
                                                                                >(
                                                                                    dlsym(
                                                                                        (*ocl).library,
                                                                                        b"clGetKernelWorkGroupInfo\0" as *const u8
                                                                                            as *const libc::c_char,
                                                                                    ),
                                                                                );
                                                                                if !(0 as libc::c_int == 0
                                                                                    && ((*ocl).clGetKernelWorkGroupInfo).is_none())
                                                                                {
                                                                                    (*ocl).clGetPlatformIDs = ::core::mem::transmute::<
                                                                                        *mut libc::c_void,
                                                                                        clGetPlatformIDs_func,
                                                                                    >(
                                                                                        dlsym(
                                                                                            (*ocl).library,
                                                                                            b"clGetPlatformIDs\0" as *const u8 as *const libc::c_char,
                                                                                        ),
                                                                                    );
                                                                                    if !(0 as libc::c_int == 0
                                                                                        && ((*ocl).clGetPlatformIDs).is_none())
                                                                                    {
                                                                                        (*ocl).clGetProgramBuildInfo = ::core::mem::transmute::<
                                                                                            *mut libc::c_void,
                                                                                            clGetProgramBuildInfo_func,
                                                                                        >(
                                                                                            dlsym(
                                                                                                (*ocl).library,
                                                                                                b"clGetProgramBuildInfo\0" as *const u8
                                                                                                    as *const libc::c_char,
                                                                                            ),
                                                                                        );
                                                                                        if !(0 as libc::c_int == 0
                                                                                            && ((*ocl).clGetProgramBuildInfo).is_none())
                                                                                        {
                                                                                            (*ocl).clGetProgramInfo = ::core::mem::transmute::<
                                                                                                *mut libc::c_void,
                                                                                                clGetProgramInfo_func,
                                                                                            >(
                                                                                                dlsym(
                                                                                                    (*ocl).library,
                                                                                                    b"clGetProgramInfo\0" as *const u8 as *const libc::c_char,
                                                                                                ),
                                                                                            );
                                                                                            if !(0 as libc::c_int == 0
                                                                                                && ((*ocl).clGetProgramInfo).is_none())
                                                                                            {
                                                                                                (*ocl).clGetSupportedImageFormats = ::core::mem::transmute::<
                                                                                                    *mut libc::c_void,
                                                                                                    clGetSupportedImageFormats_func,
                                                                                                >(
                                                                                                    dlsym(
                                                                                                        (*ocl).library,
                                                                                                        b"clGetSupportedImageFormats\0" as *const u8
                                                                                                            as *const libc::c_char,
                                                                                                    ),
                                                                                                );
                                                                                                if !(0 as libc::c_int == 0
                                                                                                    && ((*ocl).clGetSupportedImageFormats).is_none())
                                                                                                {
                                                                                                    (*ocl).clReleaseCommandQueue = ::core::mem::transmute::<
                                                                                                        *mut libc::c_void,
                                                                                                        clReleaseCommandQueue_func,
                                                                                                    >(
                                                                                                        dlsym(
                                                                                                            (*ocl).library,
                                                                                                            b"clReleaseCommandQueue\0" as *const u8
                                                                                                                as *const libc::c_char,
                                                                                                        ),
                                                                                                    );
                                                                                                    if !(0 as libc::c_int == 0
                                                                                                        && ((*ocl).clReleaseCommandQueue).is_none())
                                                                                                    {
                                                                                                        (*ocl).clReleaseContext = ::core::mem::transmute::<
                                                                                                            *mut libc::c_void,
                                                                                                            clReleaseContext_func,
                                                                                                        >(
                                                                                                            dlsym(
                                                                                                                (*ocl).library,
                                                                                                                b"clReleaseContext\0" as *const u8 as *const libc::c_char,
                                                                                                            ),
                                                                                                        );
                                                                                                        if !(0 as libc::c_int == 0
                                                                                                            && ((*ocl).clReleaseContext).is_none())
                                                                                                        {
                                                                                                            (*ocl).clReleaseKernel = ::core::mem::transmute::<
                                                                                                                *mut libc::c_void,
                                                                                                                clReleaseKernel_func,
                                                                                                            >(
                                                                                                                dlsym(
                                                                                                                    (*ocl).library,
                                                                                                                    b"clReleaseKernel\0" as *const u8 as *const libc::c_char,
                                                                                                                ),
                                                                                                            );
                                                                                                            if !(0 as libc::c_int == 0
                                                                                                                && ((*ocl).clReleaseKernel).is_none())
                                                                                                            {
                                                                                                                (*ocl).clReleaseMemObject = ::core::mem::transmute::<
                                                                                                                    *mut libc::c_void,
                                                                                                                    clReleaseMemObject_func,
                                                                                                                >(
                                                                                                                    dlsym(
                                                                                                                        (*ocl).library,
                                                                                                                        b"clReleaseMemObject\0" as *const u8 as *const libc::c_char,
                                                                                                                    ),
                                                                                                                );
                                                                                                                if !(0 as libc::c_int == 0
                                                                                                                    && ((*ocl).clReleaseMemObject).is_none())
                                                                                                                {
                                                                                                                    (*ocl).clReleaseProgram = ::core::mem::transmute::<
                                                                                                                        *mut libc::c_void,
                                                                                                                        clReleaseProgram_func,
                                                                                                                    >(
                                                                                                                        dlsym(
                                                                                                                            (*ocl).library,
                                                                                                                            b"clReleaseProgram\0" as *const u8 as *const libc::c_char,
                                                                                                                        ),
                                                                                                                    );
                                                                                                                    if !(0 as libc::c_int == 0
                                                                                                                        && ((*ocl).clReleaseProgram).is_none())
                                                                                                                    {
                                                                                                                        (*ocl).clSetKernelArg = ::core::mem::transmute::<
                                                                                                                            *mut libc::c_void,
                                                                                                                            clSetKernelArg_func,
                                                                                                                        >(
                                                                                                                            dlsym(
                                                                                                                                (*ocl).library,
                                                                                                                                b"clSetKernelArg\0" as *const u8 as *const libc::c_char,
                                                                                                                            ),
                                                                                                                        );
                                                                                                                        if !(0 as libc::c_int == 0
                                                                                                                            && ((*ocl).clSetKernelArg).is_none())
                                                                                                                        {
                                                                                                                            return ocl;
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            dlclose((*ocl).library);
        }
        x264_free(ocl as *mut libc::c_void);
    }
    std::ptr::null_mut::<x264_opencl_function_t>()
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_opencl_close_library(mut ocl: *mut x264_opencl_function_t) {
    if ocl.is_null() {
        return;
    }
    dlclose((*ocl).library);
    x264_free(ocl as *mut libc::c_void);
}
unsafe extern "C" fn opencl_cache_load(
    mut h: *mut x264_t,
    mut dev_name: *const libc::c_char,
    mut dev_vendor: *const libc::c_char,
    mut driver_version: *const libc::c_char,
) -> cl_program {
    let mut size: size_t = 0;
    let mut ptr: *const uint8_t = std::ptr::null::<uint8_t>();
    let mut status: cl_int = 0;
    let mut fp: *mut FILE = fopen(
        (*h).param.psz_clbin_file,
        b"rb\0" as *const u8 as *const libc::c_char,
    );
    if fp.is_null() {
        return 0 as cl_program;
    }
    let mut ocl: *mut x264_opencl_function_t = (*h).opencl.ocl;
    let mut program: cl_program = 0 as cl_program;
    let mut binary: *mut uint8_t = std::ptr::null_mut::<uint8_t>();
    fseeko(fp, 0 as libc::c_int as __off64_t, 2 as libc::c_int);
    let mut file_size: int64_t = ftello(fp);
    fseeko(fp, 0 as libc::c_int as __off64_t, 0 as libc::c_int);
    if !(file_size < 0 as libc::c_int as int64_t
        || file_size as uint64_t > 18446744073709551615 as libc::c_ulong)
    {
        size = file_size as size_t;
        binary = x264_malloc(size as int64_t) as *mut uint8_t;
        if !binary.is_null()
            && fread(
                binary as *mut libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                size,
                fp,
            ) == size
        {
            ptr = binary as *const uint8_t;
            let mut len: size_t = strlen(dev_name);
            if !(size <= len || strncmp(ptr as *mut libc::c_char, dev_name, len) != 0) {
                size = size.wrapping_sub(len.wrapping_add(1 as libc::c_int as size_t));
                ptr = ptr.offset(len.wrapping_add(1 as libc::c_int as size_t) as isize);
                let mut len_0: size_t = strlen(dev_vendor);
                if !(size <= len_0 || strncmp(ptr as *mut libc::c_char, dev_vendor, len_0) != 0) {
                    size = size.wrapping_sub(len_0.wrapping_add(1 as libc::c_int as size_t));
                    ptr = ptr.offset(len_0.wrapping_add(1 as libc::c_int as size_t) as isize);
                    let mut len_1: size_t = strlen(driver_version);
                    if !(size <= len_1
                        || strncmp(ptr as *mut libc::c_char, driver_version, len_1) != 0)
                    {
                        size = size.wrapping_sub(len_1.wrapping_add(1 as libc::c_int as size_t));
                        ptr = ptr.offset(len_1.wrapping_add(1 as libc::c_int as size_t) as isize);
                        let mut len_2: size_t = strlen(x264_opencl_source_hash.as_ptr());
                        if !(size <= len_2
                            || strncmp(
                                ptr as *mut libc::c_char,
                                x264_opencl_source_hash.as_ptr(),
                                len_2,
                            ) != 0)
                        {
                            size =
                                size.wrapping_sub(len_2.wrapping_add(1 as libc::c_int as size_t));
                            ptr =
                                ptr.offset(len_2.wrapping_add(1 as libc::c_int as size_t) as isize);
                            status = 0;
                            program = ((*ocl).clCreateProgramWithBinary)
                                .expect("non-null function pointer")(
                                (*h).opencl.context,
                                1 as libc::c_int as cl_uint,
                                &mut (*h).opencl.device,
                                &mut size,
                                &mut ptr,
                                std::ptr::null_mut::<cl_int>(),
                                &mut status,
                            );
                            if status != 0 as libc::c_int {
                                program = 0 as cl_program;
                            }
                        }
                    }
                }
            }
        }
    }
    fclose(fp);
    x264_free(binary as *mut libc::c_void);
    program
}
unsafe extern "C" fn opencl_cache_save(
    mut h: *mut x264_t,
    mut program: cl_program,
    mut dev_name: *const libc::c_char,
    mut dev_vendor: *const libc::c_char,
    mut driver_version: *const libc::c_char,
) {
    let mut fp: *mut FILE = fopen(
        (*h).param.psz_clbin_file,
        b"wb\0" as *const u8 as *const libc::c_char,
    );
    if fp.is_null() {
        x264_8_log(
            h,
            2 as libc::c_int,
            b"OpenCL: unable to open clbin file for write\n\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    let mut ocl: *mut x264_opencl_function_t = (*h).opencl.ocl;
    let mut binary: *mut uint8_t = std::ptr::null_mut::<uint8_t>();
    let mut size: size_t = 0 as libc::c_int as size_t;
    let mut status: cl_int = ((*ocl).clGetProgramInfo).expect("non-null function pointer")(
        program,
        0x1165 as libc::c_int as cl_program_info,
        ::core::mem::size_of::<size_t>() as libc::c_ulong,
        &mut size as *mut size_t as *mut libc::c_void,
        std::ptr::null_mut::<size_t>(),
    );
    if status != 0 as libc::c_int || size == 0 {
        x264_8_log(
            h,
            2 as libc::c_int,
            b"OpenCL: Unable to query program binary size, no cache file generated\n\0" as *const u8
                as *const libc::c_char,
        );
    } else {
        binary = x264_malloc(size as int64_t) as *mut uint8_t;
        if !binary.is_null() {
            status = ((*ocl).clGetProgramInfo).expect("non-null function pointer")(
                program,
                0x1166 as libc::c_int as cl_program_info,
                ::core::mem::size_of::<*mut uint8_t>() as libc::c_ulong,
                &mut binary as *mut *mut uint8_t as *mut libc::c_void,
                std::ptr::null_mut::<size_t>(),
            );
            if status != 0 as libc::c_int {
                x264_8_log(
                    h,
                    2 as libc::c_int,
                    b"OpenCL: Unable to query program binary, no cache file generated\n\0"
                        as *const u8 as *const libc::c_char,
                );
            } else {
                fputs(dev_name, fp);
                fputc('\n' as i32, fp);
                fputs(dev_vendor, fp);
                fputc('\n' as i32, fp);
                fputs(driver_version, fp);
                fputc('\n' as i32, fp);
                fputs(x264_opencl_source_hash.as_ptr(), fp);
                fputc('\n' as i32, fp);
                fwrite(
                    binary as *const libc::c_void,
                    1 as libc::c_int as libc::c_ulong,
                    size,
                    fp,
                );
            }
        }
    }
    fclose(fp);
    x264_free(binary as *mut libc::c_void);
}
unsafe extern "C" fn opencl_compile(mut h: *mut x264_t) -> cl_program {
    let mut log_file: *mut FILE = std::ptr::null_mut::<FILE>();
    let mut ocl: *mut x264_opencl_function_t = (*h).opencl.ocl;
    let mut program: cl_program = 0 as cl_program;
    let mut build_log: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    let mut dev_name: [libc::c_char; 64] = [0; 64];
    let mut dev_vendor: [libc::c_char; 64] = [0; 64];
    let mut driver_version: [libc::c_char; 64] = [0; 64];
    let mut status: cl_int = 0;
    status = ((*ocl).clGetDeviceInfo).expect("non-null function pointer")(
        (*h).opencl.device,
        0x102b as libc::c_int as cl_device_info,
        ::core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
        dev_name.as_mut_ptr() as *mut libc::c_void,
        std::ptr::null_mut::<size_t>(),
    );
    status |= ((*ocl).clGetDeviceInfo).expect("non-null function pointer")(
        (*h).opencl.device,
        0x102c as libc::c_int as cl_device_info,
        ::core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
        dev_vendor.as_mut_ptr() as *mut libc::c_void,
        std::ptr::null_mut::<size_t>(),
    );
    status |= ((*ocl).clGetDeviceInfo).expect("non-null function pointer")(
        (*h).opencl.device,
        0x102d as libc::c_int as cl_device_info,
        ::core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
        driver_version.as_mut_ptr() as *mut libc::c_void,
        std::ptr::null_mut::<size_t>(),
    );
    if status != 0 as libc::c_int {
        return 0 as cl_program;
    }
    let mut vectorize: libc::c_int = (strcmp(
        dev_vendor.as_mut_ptr(),
        b"Advanced Micro Devices, Inc.\0" as *const u8 as *const libc::c_char,
    ) == 0) as libc::c_int;
    (*h).opencl.b_device_AMD_SI = 0 as libc::c_int;
    if vectorize != 0 {
        if detect_switchable_graphics() != 0 {
            x264_8_log(
                h,
                2 as libc::c_int,
                b"OpenCL acceleration disabled, switchable graphics detected\n\0" as *const u8
                    as *const libc::c_char,
            );
            return 0 as cl_program;
        }
        let mut simdwidth: cl_uint = 4 as libc::c_int as cl_uint;
        status = ((*ocl).clGetDeviceInfo).expect("non-null function pointer")(
            (*h).opencl.device,
            0x4042 as libc::c_int as cl_device_info,
            ::core::mem::size_of::<cl_uint>() as libc::c_ulong,
            &mut simdwidth as *mut cl_uint as *mut libc::c_void,
            std::ptr::null_mut::<size_t>(),
        );
        if status == 0 as libc::c_int && simdwidth == 1 as libc::c_int as cl_uint {
            vectorize = 0 as libc::c_int;
            (*h).opencl.b_device_AMD_SI = 1 as libc::c_int;
        }
    }
    x264_8_log(
        h,
        2 as libc::c_int,
        b"OpenCL acceleration enabled with %s %s %s\n\0" as *const u8 as *const libc::c_char,
        dev_vendor.as_mut_ptr(),
        dev_name.as_mut_ptr(),
        if (*h).opencl.b_device_AMD_SI != 0 {
            b"(SI)\0" as *const u8 as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
    );
    program = opencl_cache_load(
        h,
        dev_name.as_mut_ptr(),
        dev_vendor.as_mut_ptr(),
        driver_version.as_mut_ptr(),
    );
    if program.is_null() {
        x264_8_log(
            h,
            2 as libc::c_int,
            b"Compiling OpenCL kernels...\n\0" as *const u8 as *const libc::c_char,
        );
        let mut strptr: *const libc::c_char = x264_opencl_source.as_ptr();
        let mut size: size_t = ::core::mem::size_of::<[libc::c_char; 74836]>() as libc::c_ulong;
        program = ((*ocl).clCreateProgramWithSource).expect("non-null function pointer")(
            (*h).opencl.context,
            1 as libc::c_int as cl_uint,
            &mut strptr,
            &mut size,
            &mut status,
        );
        if status != 0 as libc::c_int || program.is_null() {
            x264_8_log(
                h,
                1 as libc::c_int,
                b"OpenCL: unable to create program\n\0" as *const u8 as *const libc::c_char,
            );
            return 0 as cl_program;
        }
    }
    let mut buildopts: *const libc::c_char = if vectorize != 0 {
        b"-DVECTORIZE=1\0" as *const u8 as *const libc::c_char
    } else {
        b"\0" as *const u8 as *const libc::c_char
    };
    status = ((*ocl).clBuildProgram).expect("non-null function pointer")(
        program,
        1 as libc::c_int as cl_uint,
        &mut (*h).opencl.device,
        buildopts,
        None,
        std::ptr::null_mut::<libc::c_void>(),
    );
    if status == 0 as libc::c_int {
        opencl_cache_save(
            h,
            program,
            dev_name.as_mut_ptr(),
            dev_vendor.as_mut_ptr(),
            driver_version.as_mut_ptr(),
        );
        return program;
    }
    let mut build_log_len: size_t = 0 as libc::c_int as size_t;
    status = ((*ocl).clGetProgramBuildInfo).expect("non-null function pointer")(
        program,
        (*h).opencl.device,
        0x1183 as libc::c_int as cl_program_build_info,
        0 as libc::c_int as size_t,
        std::ptr::null_mut::<libc::c_void>(),
        &mut build_log_len,
    );
    if status != 0 as libc::c_int || build_log_len == 0 {
        x264_8_log(
            h,
            1 as libc::c_int,
            b"OpenCL: Compilation failed, unable to query build log\n\0" as *const u8
                as *const libc::c_char,
        );
    } else {
        build_log = x264_malloc(build_log_len as int64_t) as *mut libc::c_char;
        if build_log.is_null() {
            x264_8_log(
                h,
                1 as libc::c_int,
                b"OpenCL: Compilation failed, unable to alloc build log\n\0" as *const u8
                    as *const libc::c_char,
            );
        } else {
            status = ((*ocl).clGetProgramBuildInfo).expect("non-null function pointer")(
                program,
                (*h).opencl.device,
                0x1183 as libc::c_int as cl_program_build_info,
                build_log_len,
                build_log as *mut libc::c_void,
                std::ptr::null_mut::<size_t>(),
            );
            if status != 0 as libc::c_int {
                x264_8_log(
                    h,
                    1 as libc::c_int,
                    b"OpenCL: Compilation failed, unable to get build log\n\0" as *const u8
                        as *const libc::c_char,
                );
            } else {
                log_file = fopen(
                    b"x264_kernel_build_log.txt\0" as *const u8 as *const libc::c_char,
                    b"w\0" as *const u8 as *const libc::c_char,
                );
                if log_file.is_null() {
                    x264_8_log(
                        h,
                        1 as libc::c_int,
                        b"OpenCL: Compilation failed, unable to create file x264_kernel_build_log.txt\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                } else {
                    fwrite(
                        build_log as *const libc::c_void,
                        1 as libc::c_int as libc::c_ulong,
                        build_log_len,
                        log_file,
                    );
                    fclose(log_file);
                    x264_8_log(
                        h,
                        1 as libc::c_int,
                        b"OpenCL: kernel build errors written to x264_kernel_build_log.txt\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                }
            }
        }
    }
    x264_free(build_log as *mut libc::c_void);
    if !program.is_null() {
        ((*ocl).clReleaseProgram).expect("non-null function pointer")(program);
    }
    0 as cl_program
}
unsafe extern "C" fn opencl_lookahead_alloc(mut h: *mut x264_t) -> libc::c_int {
    let mut current_block: u64;
    if (*h).param.rc.i_lookahead == 0 {
        return -(1 as libc::c_int);
    }
    static mut kernelnames: [*const libc::c_char; 12] = [
        b"mb_intra_cost_satd_8x8\0" as *const u8 as *const libc::c_char,
        b"sum_intra_cost\0" as *const u8 as *const libc::c_char,
        b"downscale_hpel\0" as *const u8 as *const libc::c_char,
        b"downscale1\0" as *const u8 as *const libc::c_char,
        b"downscale2\0" as *const u8 as *const libc::c_char,
        b"memset_int16\0" as *const u8 as *const libc::c_char,
        b"weightp_scaled_images\0" as *const u8 as *const libc::c_char,
        b"weightp_hpel\0" as *const u8 as *const libc::c_char,
        b"hierarchical_motion\0" as *const u8 as *const libc::c_char,
        b"subpel_refine\0" as *const u8 as *const libc::c_char,
        b"mode_selection\0" as *const u8 as *const libc::c_char,
        b"sum_inter_cost\0" as *const u8 as *const libc::c_char,
    ];
    let mut kernels: [*mut cl_kernel; 12] = [
        &mut (*h).opencl.intra_kernel,
        &mut (*h).opencl.rowsum_intra_kernel,
        &mut (*h).opencl.downscale_hpel_kernel,
        &mut (*h).opencl.downscale_kernel1,
        &mut (*h).opencl.downscale_kernel2,
        &mut (*h).opencl.memset_kernel,
        &mut (*h).opencl.weightp_scaled_images_kernel,
        &mut (*h).opencl.weightp_hpel_kernel,
        &mut (*h).opencl.hme_kernel,
        &mut (*h).opencl.subpel_refine_kernel,
        &mut (*h).opencl.mode_select_kernel,
        &mut (*h).opencl.rowsum_inter_kernel,
    ];
    let mut ocl: *mut x264_opencl_function_t = (*h).opencl.ocl;
    let mut status: cl_int = 0;
    (*h).opencl.lookahead_program = opencl_compile(h);
    if !((*h).opencl.lookahead_program).is_null() {
        let mut i: libc::c_int = 0 as libc::c_int;
        loop {
            if i >= (::core::mem::size_of::<[*const libc::c_char; 12]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
                as libc::c_int
            {
                current_block = 13183875560443969876;
                break;
            }
            *kernels[i as usize] = ((*ocl).clCreateKernel).expect("non-null function pointer")(
                (*h).opencl.lookahead_program,
                kernelnames[i as usize],
                &mut status,
            );
            if status != 0 as libc::c_int {
                x264_8_log(
                    h,
                    1 as libc::c_int,
                    b"OpenCL: Unable to compile kernel '%s' (%d)\n\0" as *const u8
                        as *const libc::c_char,
                    kernelnames[i as usize],
                    status,
                );
                current_block = 15792998803295151122;
                break;
            } else {
                i += 1;
                i;
            }
        }
        match current_block {
            15792998803295151122 => {}
            _ => {
                (*h).opencl.page_locked_buffer = ((*ocl).clCreateBuffer)
                    .expect("non-null function pointer")(
                    (*h).opencl.context,
                    (((1 as libc::c_int) << 1 as libc::c_int)
                        | ((1 as libc::c_int) << 4 as libc::c_int))
                        as cl_mem_flags,
                    (32 as libc::c_int * 1024 as libc::c_int * 1024 as libc::c_int) as size_t,
                    std::ptr::null_mut::<libc::c_void>(),
                    &mut status,
                );
                if status != 0 as libc::c_int {
                    x264_8_log(
                        h,
                        1 as libc::c_int,
                        b"OpenCL: Unable to allocate page-locked buffer, error '%d'\n\0"
                            as *const u8 as *const libc::c_char,
                        status,
                    );
                } else {
                    (*h).opencl.page_locked_ptr = ((*ocl).clEnqueueMapBuffer)
                        .expect("non-null function pointer")(
                        (*h).opencl.queue,
                        (*h).opencl.page_locked_buffer,
                        1 as libc::c_int as cl_bool,
                        (((1 as libc::c_int) << 0 as libc::c_int)
                            | ((1 as libc::c_int) << 1 as libc::c_int))
                            as cl_map_flags,
                        0 as libc::c_int as size_t,
                        (32 as libc::c_int * 1024 as libc::c_int * 1024 as libc::c_int) as size_t,
                        0 as libc::c_int as cl_uint,
                        std::ptr::null::<cl_event>(),
                        std::ptr::null_mut::<cl_event>(),
                        &mut status,
                    ) as *mut libc::c_char;
                    if status != 0 as libc::c_int {
                        x264_8_log(
                            h,
                            1 as libc::c_int,
                            b"OpenCL: Unable to map page-locked buffer, error '%d'\n\0" as *const u8
                                as *const libc::c_char,
                            status,
                        );
                    } else {
                        return 0 as libc::c_int;
                    }
                }
            }
        }
    }
    x264_8_opencl_lookahead_delete(h);
    -(1 as libc::c_int)
}
unsafe extern "C" fn opencl_error_notify(
    mut errinfo: *const libc::c_char,
    mut private_info: *const libc::c_void,
    mut cb: size_t,
    mut user_data: *mut libc::c_void,
) {
    let mut h: *mut x264_t = user_data as *mut x264_t;
    (*h).param.b_opencl = 0 as libc::c_int;
    (*h).opencl.b_fatal_error = 1 as libc::c_int;
    x264_8_log(
        h,
        0 as libc::c_int,
        b"OpenCL: %s\n\0" as *const u8 as *const libc::c_char,
        errinfo,
    );
    x264_8_log(
        h,
        0 as libc::c_int,
        b"OpenCL: fatal error, aborting encode\n\0" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_opencl_lookahead_init(mut h: *mut x264_t) -> libc::c_int {
    let mut ocl: *mut x264_opencl_function_t = (*h).opencl.ocl;
    let mut platforms: *mut cl_platform_id = std::ptr::null_mut::<cl_platform_id>();
    let mut devices: *mut cl_device_id = std::ptr::null_mut::<cl_device_id>();
    let mut imageType: *mut cl_image_format = std::ptr::null_mut::<cl_image_format>();
    let mut context: cl_context = 0 as cl_context;
    let mut ret: libc::c_int = -(1 as libc::c_int);
    let mut numPlatforms: cl_uint = 0 as libc::c_int as cl_uint;
    let mut status: cl_int = ((*ocl).clGetPlatformIDs).expect("non-null function pointer")(
        0 as libc::c_int as cl_uint,
        std::ptr::null_mut::<cl_platform_id>(),
        &mut numPlatforms,
    );
    if status != 0 as libc::c_int || numPlatforms == 0 {
        x264_8_log(
            h,
            1 as libc::c_int,
            b"OpenCL: Unable to query installed platforms\n\0" as *const u8 as *const libc::c_char,
        );
    } else {
        platforms = x264_malloc(
            (::core::mem::size_of::<cl_platform_id>() as libc::c_ulong)
                .wrapping_mul(numPlatforms as libc::c_ulong) as int64_t,
        ) as *mut cl_platform_id;
        if platforms.is_null() {
            x264_8_log(
                h,
                1 as libc::c_int,
                b"OpenCL: malloc of installed platforms buffer failed\n\0" as *const u8
                    as *const libc::c_char,
            );
        } else {
            status = ((*ocl).clGetPlatformIDs).expect("non-null function pointer")(
                numPlatforms,
                platforms,
                std::ptr::null_mut::<cl_uint>(),
            );
            if status != 0 as libc::c_int {
                x264_8_log(
                    h,
                    1 as libc::c_int,
                    b"OpenCL: Unable to query installed platforms\n\0" as *const u8
                        as *const libc::c_char,
                );
            } else {
                let mut i: cl_uint = 0 as libc::c_int as cl_uint;
                while i < numPlatforms {
                    let mut gpu_count: cl_uint = 0 as libc::c_int as cl_uint;
                    status = ((*ocl).clGetDeviceIDs).expect("non-null function pointer")(
                        *platforms.offset(i as isize),
                        ((1 as libc::c_int) << 2 as libc::c_int) as cl_device_type,
                        0 as libc::c_int as cl_uint,
                        std::ptr::null_mut::<cl_device_id>(),
                        &mut gpu_count,
                    );
                    if !(status != 0 as libc::c_int || gpu_count == 0) {
                        x264_free(devices as *mut libc::c_void);
                        devices = x264_malloc(
                            (::core::mem::size_of::<cl_device_id>() as libc::c_ulong)
                                .wrapping_mul(gpu_count as libc::c_ulong)
                                as int64_t,
                        ) as *mut cl_device_id;
                        if !devices.is_null() {
                            status = ((*ocl).clGetDeviceIDs).expect("non-null function pointer")(
                                *platforms.offset(i as isize),
                                ((1 as libc::c_int) << 2 as libc::c_int) as cl_device_type,
                                gpu_count,
                                devices,
                                std::ptr::null_mut::<cl_uint>(),
                            );
                            if status == 0 as libc::c_int {
                                let mut gpu: cl_uint = 0 as libc::c_int as cl_uint;
                                while gpu < gpu_count {
                                    (*h).opencl.device = *devices.offset(gpu as isize);
                                    if !(!((*h).param.opencl_device_id).is_null()
                                        && *devices.offset(gpu as isize)
                                            != (*h).param.opencl_device_id as cl_device_id)
                                    {
                                        let mut image_support: cl_bool =
                                            0 as libc::c_int as cl_bool;
                                        status = ((*ocl).clGetDeviceInfo)
                                            .expect("non-null function pointer")(
                                            (*h).opencl.device,
                                            0x1016 as libc::c_int as cl_device_info,
                                            ::core::mem::size_of::<cl_bool>() as libc::c_ulong,
                                            &mut image_support as *mut cl_bool as *mut libc::c_void,
                                            std::ptr::null_mut::<size_t>(),
                                        );
                                        if !(status != 0 as libc::c_int || image_support == 0) {
                                            if !context.is_null() {
                                                ((*ocl).clReleaseContext)
                                                    .expect("non-null function pointer")(
                                                    context
                                                );
                                            }
                                            context = ((*ocl).clCreateContext)
                                                .expect("non-null function pointer")(
                                                std::ptr::null::<cl_context_properties>(),
                                                1 as libc::c_int as cl_uint,
                                                &mut (*h).opencl.device,
                                                ::core::mem::transmute::<
                                                    *mut libc::c_void,
                                                    Option<
                                                        unsafe extern "C" fn(
                                                            *const libc::c_char,
                                                            *const libc::c_void,
                                                            size_t,
                                                            *mut libc::c_void,
                                                        )
                                                            -> (),
                                                    >,
                                                >(
                                                    ::core::mem::transmute::<
                                                        Option<
                                                            unsafe extern "C" fn(
                                                                *const libc::c_char,
                                                                *const libc::c_void,
                                                                size_t,
                                                                *mut libc::c_void,
                                                            )
                                                                -> (),
                                                        >,
                                                        *mut libc::c_void,
                                                    >(
                                                        Some(
                                                            opencl_error_notify
                                                                as unsafe extern "C" fn(
                                                                    *const libc::c_char,
                                                                    *const libc::c_void,
                                                                    size_t,
                                                                    *mut libc::c_void,
                                                                )
                                                                    -> (),
                                                        ),
                                                    ),
                                                ),
                                                h as *mut libc::c_void,
                                                &mut status,
                                            );
                                            if !(status != 0 as libc::c_int || context.is_null()) {
                                                let mut imagecount: cl_uint =
                                                    0 as libc::c_int as cl_uint;
                                                status = ((*ocl).clGetSupportedImageFormats)
                                                    .expect("non-null function pointer")(
                                                    context,
                                                    ((1 as libc::c_int) << 0 as libc::c_int)
                                                        as cl_mem_flags,
                                                    0x10f1 as libc::c_int as cl_mem_object_type,
                                                    0 as libc::c_int as cl_uint,
                                                    std::ptr::null_mut::<cl_image_format>(),
                                                    &mut imagecount,
                                                );
                                                if !(status != 0 as libc::c_int || imagecount == 0)
                                                {
                                                    x264_free(imageType as *mut libc::c_void);
                                                    imageType = x264_malloc(
                                                        (::core::mem::size_of::<cl_image_format>()
                                                            as libc::c_ulong)
                                                            .wrapping_mul(
                                                                imagecount as libc::c_ulong,
                                                            )
                                                            as int64_t,
                                                    )
                                                        as *mut cl_image_format;
                                                    if !imageType.is_null() {
                                                        status = ((*ocl)
                                                            .clGetSupportedImageFormats)
                                                            .expect("non-null function pointer")(
                                                            context,
                                                            ((1 as libc::c_int) << 0 as libc::c_int)
                                                                as cl_mem_flags,
                                                            0x10f1 as libc::c_int
                                                                as cl_mem_object_type,
                                                            imagecount,
                                                            imageType,
                                                            std::ptr::null_mut::<cl_uint>(),
                                                        );
                                                        if status == 0 as libc::c_int {
                                                            let mut b_has_r: libc::c_int =
                                                                0 as libc::c_int;
                                                            let mut b_has_rgba: libc::c_int =
                                                                0 as libc::c_int;
                                                            let mut j: cl_uint =
                                                                0 as libc::c_int as cl_uint;
                                                            while j < imagecount {
                                                                if (*imageType.offset(j as isize))
                                                                    .image_channel_order
                                                                    == 0x10b0 as libc::c_int
                                                                        as cl_channel_order
                                                                    && (*imageType
                                                                        .offset(j as isize))
                                                                    .image_channel_data_type
                                                                        == 0x10dc as libc::c_int
                                                                            as cl_channel_type
                                                                {
                                                                    b_has_r = 1 as libc::c_int;
                                                                } else if (*imageType
                                                                    .offset(j as isize))
                                                                .image_channel_order
                                                                    == 0x10b5 as libc::c_int
                                                                        as cl_channel_order
                                                                    && (*imageType
                                                                        .offset(j as isize))
                                                                    .image_channel_data_type
                                                                        == 0x10da as libc::c_int
                                                                            as cl_channel_type
                                                                {
                                                                    b_has_rgba = 1 as libc::c_int;
                                                                }
                                                                j = j.wrapping_add(1);
                                                                j;
                                                            }
                                                            if b_has_r == 0 || b_has_rgba == 0 {
                                                                let mut dev_name: [libc::c_char;
                                                                    64] = [0; 64];
                                                                status = ((*ocl).clGetDeviceInfo)
                                                                    .expect(
                                                                        "non-null function pointer",
                                                                    )(
                                                                    (*h).opencl.device,
                                                                    0x102b as libc::c_int
                                                                        as cl_device_info,
                                                                    ::core::mem::size_of::<
                                                                        [libc::c_char; 64],
                                                                    >(
                                                                    )
                                                                        as libc::c_ulong,
                                                                    dev_name.as_mut_ptr()
                                                                        as *mut libc::c_void,
                                                                    std::ptr::null_mut::<size_t>(),
                                                                );
                                                                if status == 0 as libc::c_int {
                                                                    let mut level: libc::c_int =
                                                                        if !((*h)
                                                                            .param
                                                                            .opencl_device_id)
                                                                            .is_null()
                                                                        {
                                                                            1 as libc::c_int
                                                                        } else {
                                                                            3 as libc::c_int
                                                                        };
                                                                    x264_8_log(
                                                                        h,
                                                                        level,
                                                                        b"OpenCL: %s does not support required image formats\n\0"
                                                                            as *const u8 as *const libc::c_char,
                                                                        dev_name.as_mut_ptr(),
                                                                    );
                                                                }
                                                            } else if (*h).param.i_opencl_device
                                                                != 0
                                                            {
                                                                (*h).param.i_opencl_device -= 1;
                                                                (*h).param.i_opencl_device;
                                                            } else {
                                                                (*h).opencl.queue = ((*ocl).clCreateCommandQueue)
                                                                    .expect(
                                                                        "non-null function pointer",
                                                                    )(
                                                                    context,
                                                                    (*h).opencl.device,
                                                                    0 as libc::c_int as cl_command_queue_properties,
                                                                    &mut status,
                                                                );
                                                                if !(status != 0 as libc::c_int
                                                                    || ((*h).opencl.queue)
                                                                        .is_null())
                                                                {
                                                                    (*h).opencl.context = context;
                                                                    context = 0 as cl_context;
                                                                    ret = 0 as libc::c_int;
                                                                    break;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    gpu = gpu.wrapping_add(1);
                                    gpu;
                                }
                                if ret == 0 {
                                    break;
                                }
                            }
                        }
                    }
                    i = i.wrapping_add(1);
                    i;
                }
                if ((*h).param.psz_clbin_file).is_null() {
                    (*h).param.psz_clbin_file = b"x264_lookahead.clbin\0" as *const u8
                        as *const libc::c_char
                        as *mut libc::c_char;
                }
                if ret != 0 {
                    x264_8_log(
                        h,
                        1 as libc::c_int,
                        b"OpenCL: Unable to find a compatible device\n\0" as *const u8
                            as *const libc::c_char,
                    );
                } else {
                    ret = opencl_lookahead_alloc(h);
                }
            }
        }
    }
    if !context.is_null() {
        ((*ocl).clReleaseContext).expect("non-null function pointer")(context);
    }
    x264_free(imageType as *mut libc::c_void);
    x264_free(devices as *mut libc::c_void);
    x264_free(platforms as *mut libc::c_void);
    ret
}
unsafe extern "C" fn opencl_lookahead_free(mut h: *mut x264_t) {
    let mut ocl: *mut x264_opencl_function_t = (*h).opencl.ocl;
    if !((*h).opencl.downscale_hpel_kernel).is_null() {
        ((*ocl).clReleaseKernel).expect("non-null function pointer")(
            (*h).opencl.downscale_hpel_kernel,
        );
        (*h).opencl.downscale_hpel_kernel = 0 as cl_kernel;
    }
    if !((*h).opencl.downscale_kernel1).is_null() {
        ((*ocl).clReleaseKernel).expect("non-null function pointer")((*h).opencl.downscale_kernel1);
        (*h).opencl.downscale_kernel1 = 0 as cl_kernel;
    }
    if !((*h).opencl.downscale_kernel2).is_null() {
        ((*ocl).clReleaseKernel).expect("non-null function pointer")((*h).opencl.downscale_kernel2);
        (*h).opencl.downscale_kernel2 = 0 as cl_kernel;
    }
    if !((*h).opencl.weightp_hpel_kernel).is_null() {
        ((*ocl).clReleaseKernel).expect("non-null function pointer")(
            (*h).opencl.weightp_hpel_kernel,
        );
        (*h).opencl.weightp_hpel_kernel = 0 as cl_kernel;
    }
    if !((*h).opencl.weightp_scaled_images_kernel).is_null() {
        ((*ocl).clReleaseKernel).expect("non-null function pointer")(
            (*h).opencl.weightp_scaled_images_kernel,
        );
        (*h).opencl.weightp_scaled_images_kernel = 0 as cl_kernel;
    }
    if !((*h).opencl.memset_kernel).is_null() {
        ((*ocl).clReleaseKernel).expect("non-null function pointer")((*h).opencl.memset_kernel);
        (*h).opencl.memset_kernel = 0 as cl_kernel;
    }
    if !((*h).opencl.intra_kernel).is_null() {
        ((*ocl).clReleaseKernel).expect("non-null function pointer")((*h).opencl.intra_kernel);
        (*h).opencl.intra_kernel = 0 as cl_kernel;
    }
    if !((*h).opencl.rowsum_intra_kernel).is_null() {
        ((*ocl).clReleaseKernel).expect("non-null function pointer")(
            (*h).opencl.rowsum_intra_kernel,
        );
        (*h).opencl.rowsum_intra_kernel = 0 as cl_kernel;
    }
    if !((*h).opencl.hme_kernel).is_null() {
        ((*ocl).clReleaseKernel).expect("non-null function pointer")((*h).opencl.hme_kernel);
        (*h).opencl.hme_kernel = 0 as cl_kernel;
    }
    if !((*h).opencl.subpel_refine_kernel).is_null() {
        ((*ocl).clReleaseKernel).expect("non-null function pointer")(
            (*h).opencl.subpel_refine_kernel,
        );
        (*h).opencl.subpel_refine_kernel = 0 as cl_kernel;
    }
    if !((*h).opencl.mode_select_kernel).is_null() {
        ((*ocl).clReleaseKernel).expect("non-null function pointer")(
            (*h).opencl.mode_select_kernel,
        );
        (*h).opencl.mode_select_kernel = 0 as cl_kernel;
    }
    if !((*h).opencl.rowsum_inter_kernel).is_null() {
        ((*ocl).clReleaseKernel).expect("non-null function pointer")(
            (*h).opencl.rowsum_inter_kernel,
        );
        (*h).opencl.rowsum_inter_kernel = 0 as cl_kernel;
    }
    if !((*h).opencl.lookahead_program).is_null() {
        ((*ocl).clReleaseProgram).expect("non-null function pointer")(
            (*h).opencl.lookahead_program,
        );
        (*h).opencl.lookahead_program = 0 as cl_program;
    }
    if !((*h).opencl.page_locked_buffer).is_null() {
        ((*ocl).clReleaseMemObject).expect("non-null function pointer")(
            (*h).opencl.page_locked_buffer,
        );
        (*h).opencl.page_locked_buffer = 0 as cl_mem;
    }
    if !((*h).opencl.luma_16x16_image[0 as libc::c_int as usize]).is_null() {
        ((*ocl).clReleaseMemObject).expect("non-null function pointer")(
            (*h).opencl.luma_16x16_image[0 as libc::c_int as usize],
        );
        (*h).opencl.luma_16x16_image[0 as libc::c_int as usize] = 0 as cl_mem;
    }
    if !((*h).opencl.luma_16x16_image[1 as libc::c_int as usize]).is_null() {
        ((*ocl).clReleaseMemObject).expect("non-null function pointer")(
            (*h).opencl.luma_16x16_image[1 as libc::c_int as usize],
        );
        (*h).opencl.luma_16x16_image[1 as libc::c_int as usize] = 0 as cl_mem;
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        if !((*h).opencl.weighted_scaled_images[i as usize]).is_null() {
            ((*ocl).clReleaseMemObject).expect("non-null function pointer")(
                (*h).opencl.weighted_scaled_images[i as usize],
            );
            (*h).opencl.weighted_scaled_images[i as usize] = 0 as cl_mem;
        }
        i += 1;
        i;
    }
    if !((*h).opencl.weighted_luma_hpel).is_null() {
        ((*ocl).clReleaseMemObject).expect("non-null function pointer")(
            (*h).opencl.weighted_luma_hpel,
        );
        (*h).opencl.weighted_luma_hpel = 0 as cl_mem;
    }
    if !((*h).opencl.row_satds[0 as libc::c_int as usize]).is_null() {
        ((*ocl).clReleaseMemObject).expect("non-null function pointer")(
            (*h).opencl.row_satds[0 as libc::c_int as usize],
        );
        (*h).opencl.row_satds[0 as libc::c_int as usize] = 0 as cl_mem;
    }
    if !((*h).opencl.row_satds[1 as libc::c_int as usize]).is_null() {
        ((*ocl).clReleaseMemObject).expect("non-null function pointer")(
            (*h).opencl.row_satds[1 as libc::c_int as usize],
        );
        (*h).opencl.row_satds[1 as libc::c_int as usize] = 0 as cl_mem;
    }
    if !((*h).opencl.mv_buffers[0 as libc::c_int as usize]).is_null() {
        ((*ocl).clReleaseMemObject).expect("non-null function pointer")(
            (*h).opencl.mv_buffers[0 as libc::c_int as usize],
        );
        (*h).opencl.mv_buffers[0 as libc::c_int as usize] = 0 as cl_mem;
    }
    if !((*h).opencl.mv_buffers[1 as libc::c_int as usize]).is_null() {
        ((*ocl).clReleaseMemObject).expect("non-null function pointer")(
            (*h).opencl.mv_buffers[1 as libc::c_int as usize],
        );
        (*h).opencl.mv_buffers[1 as libc::c_int as usize] = 0 as cl_mem;
    }
    if !((*h).opencl.lowres_mv_costs).is_null() {
        ((*ocl).clReleaseMemObject).expect("non-null function pointer")(
            (*h).opencl.lowres_mv_costs,
        );
        (*h).opencl.lowres_mv_costs = 0 as cl_mem;
    }
    if !((*h).opencl.mvp_buffer).is_null() {
        ((*ocl).clReleaseMemObject).expect("non-null function pointer")((*h).opencl.mvp_buffer);
        (*h).opencl.mvp_buffer = 0 as cl_mem;
    }
    if !((*h).opencl.lowres_costs[0 as libc::c_int as usize]).is_null() {
        ((*ocl).clReleaseMemObject).expect("non-null function pointer")(
            (*h).opencl.lowres_costs[0 as libc::c_int as usize],
        );
        (*h).opencl.lowres_costs[0 as libc::c_int as usize] = 0 as cl_mem;
    }
    if !((*h).opencl.lowres_costs[1 as libc::c_int as usize]).is_null() {
        ((*ocl).clReleaseMemObject).expect("non-null function pointer")(
            (*h).opencl.lowres_costs[1 as libc::c_int as usize],
        );
        (*h).opencl.lowres_costs[1 as libc::c_int as usize] = 0 as cl_mem;
    }
    if !((*h).opencl.frame_stats[0 as libc::c_int as usize]).is_null() {
        ((*ocl).clReleaseMemObject).expect("non-null function pointer")(
            (*h).opencl.frame_stats[0 as libc::c_int as usize],
        );
        (*h).opencl.frame_stats[0 as libc::c_int as usize] = 0 as cl_mem;
    }
    if !((*h).opencl.frame_stats[1 as libc::c_int as usize]).is_null() {
        ((*ocl).clReleaseMemObject).expect("non-null function pointer")(
            (*h).opencl.frame_stats[1 as libc::c_int as usize],
        );
        (*h).opencl.frame_stats[1 as libc::c_int as usize] = 0 as cl_mem;
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_opencl_lookahead_delete(mut h: *mut x264_t) {
    let mut ocl: *mut x264_opencl_function_t = (*h).opencl.ocl;
    if ocl.is_null() {
        return;
    }
    if !((*h).opencl.queue).is_null() {
        ((*ocl).clFinish).expect("non-null function pointer")((*h).opencl.queue);
    }
    opencl_lookahead_free(h);
    if !((*h).opencl.queue).is_null() {
        ((*ocl).clReleaseCommandQueue).expect("non-null function pointer")((*h).opencl.queue);
        (*h).opencl.queue = 0 as cl_command_queue;
    }
    if !((*h).opencl.context).is_null() {
        ((*ocl).clReleaseContext).expect("non-null function pointer")((*h).opencl.context);
        (*h).opencl.context = 0 as cl_context;
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_opencl_frame_delete(mut frame: *mut x264_frame_t) {
    let mut ocl: *mut x264_opencl_function_t = (*frame).opencl.ocl;
    if ocl.is_null() {
        return;
    }
    let mut j: libc::c_int = 0 as libc::c_int;
    while j < 4 as libc::c_int {
        if !((*frame).opencl.scaled_image2Ds[j as usize]).is_null() {
            ((*ocl).clReleaseMemObject).expect("non-null function pointer")(
                (*frame).opencl.scaled_image2Ds[j as usize],
            );
            (*frame).opencl.scaled_image2Ds[j as usize] = 0 as cl_mem;
        }
        j += 1;
        j;
    }
    if !((*frame).opencl.luma_hpel).is_null() {
        ((*ocl).clReleaseMemObject).expect("non-null function pointer")((*frame).opencl.luma_hpel);
        (*frame).opencl.luma_hpel = 0 as cl_mem;
    }
    if !((*frame).opencl.inv_qscale_factor).is_null() {
        ((*ocl).clReleaseMemObject).expect("non-null function pointer")(
            (*frame).opencl.inv_qscale_factor,
        );
        (*frame).opencl.inv_qscale_factor = 0 as cl_mem;
    }
    if !((*frame).opencl.intra_cost).is_null() {
        ((*ocl).clReleaseMemObject).expect("non-null function pointer")((*frame).opencl.intra_cost);
        (*frame).opencl.intra_cost = 0 as cl_mem;
    }
    if !((*frame).opencl.lowres_mvs0).is_null() {
        ((*ocl).clReleaseMemObject).expect("non-null function pointer")(
            (*frame).opencl.lowres_mvs0,
        );
        (*frame).opencl.lowres_mvs0 = 0 as cl_mem;
    }
    if !((*frame).opencl.lowres_mvs1).is_null() {
        ((*ocl).clReleaseMemObject).expect("non-null function pointer")(
            (*frame).opencl.lowres_mvs1,
        );
        (*frame).opencl.lowres_mvs1 = 0 as cl_mem;
    }
    if !((*frame).opencl.lowres_mv_costs0).is_null() {
        ((*ocl).clReleaseMemObject).expect("non-null function pointer")(
            (*frame).opencl.lowres_mv_costs0,
        );
        (*frame).opencl.lowres_mv_costs0 = 0 as cl_mem;
    }
    if !((*frame).opencl.lowres_mv_costs1).is_null() {
        ((*ocl).clReleaseMemObject).expect("non-null function pointer")(
            (*frame).opencl.lowres_mv_costs1,
        );
        (*frame).opencl.lowres_mv_costs1 = 0 as cl_mem;
    }
}
unsafe extern "C" fn adl_malloc_wrapper(mut iSize: libc::c_int) -> *mut libc::c_void {
    x264_malloc(iSize as int64_t)
}
unsafe extern "C" fn detect_switchable_graphics() -> libc::c_int {
    let mut numAdapters: libc::c_int = 0;
    let mut hDLL: *mut libc::c_void = std::ptr::null_mut::<libc::c_void>();
    let mut ADL_Main_Control_Create: ADL_MAIN_CONTROL_CREATE = None;
    let mut ADL_Adapter_NumberOfAdapters_Get: ADL_ADAPTER_NUMBEROFADAPTERS_GET = None;
    let mut ADL_PowerXpress_Scheme_Get: ADL_POWERXPRESS_SCHEME_GET = None;
    let mut ADL_Main_Control_Destroy: ADL_MAIN_CONTROL_DESTROY = None;
    let mut ret: libc::c_int = 0 as libc::c_int;
    hDLL = dlopen(
        b"libatiadlxx.so\0" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int | 0x100 as libc::c_int,
    );
    if !hDLL.is_null() {
        ADL_Main_Control_Create =
            ::core::mem::transmute::<*mut libc::c_void, ADL_MAIN_CONTROL_CREATE>(dlsym(
                hDLL,
                b"ADL_Main_Control_Create\0" as *const u8 as *const libc::c_char,
            ));
        ADL_Main_Control_Destroy =
            ::core::mem::transmute::<*mut libc::c_void, ADL_MAIN_CONTROL_DESTROY>(dlsym(
                hDLL,
                b"ADL_Main_Control_Destroy\0" as *const u8 as *const libc::c_char,
            ));
        ADL_Adapter_NumberOfAdapters_Get =
            ::core::mem::transmute::<*mut libc::c_void, ADL_ADAPTER_NUMBEROFADAPTERS_GET>(dlsym(
                hDLL,
                b"ADL_Adapter_NumberOfAdapters_Get\0" as *const u8 as *const libc::c_char,
            ));
        ADL_PowerXpress_Scheme_Get =
            ::core::mem::transmute::<*mut libc::c_void, ADL_POWERXPRESS_SCHEME_GET>(dlsym(
                hDLL,
                b"ADL_PowerXpress_Scheme_Get\0" as *const u8 as *const libc::c_char,
            ));
        if !(ADL_Main_Control_Create.is_none()
            || ADL_Main_Control_Destroy.is_none()
            || ADL_Adapter_NumberOfAdapters_Get.is_none()
            || ADL_PowerXpress_Scheme_Get.is_none())
            && 0 as libc::c_int
                == ADL_Main_Control_Create.expect("non-null function pointer")(
                    Some(
                        adl_malloc_wrapper
                            as unsafe extern "C" fn(libc::c_int) -> *mut libc::c_void,
                    ),
                    1 as libc::c_int,
                )
        {
            numAdapters = 0 as libc::c_int;
            if 0 as libc::c_int
                == ADL_Adapter_NumberOfAdapters_Get.expect("non-null function pointer")(
                    &mut numAdapters,
                )
            {
                let mut i: libc::c_int = 0 as libc::c_int;
                while i < numAdapters {
                    let mut PXSchemeRange: libc::c_int = 0;
                    let mut PXSchemeCurrentState: libc::c_int = 0;
                    let mut PXSchemeDefaultState: libc::c_int = 0;
                    if 0 as libc::c_int
                        != ADL_PowerXpress_Scheme_Get.expect("non-null function pointer")(
                            i,
                            &mut PXSchemeRange,
                            &mut PXSchemeCurrentState,
                            &mut PXSchemeDefaultState,
                        )
                    {
                        break;
                    }
                    if PXSchemeRange >= 2 as libc::c_int {
                        ret = 1 as libc::c_int;
                        break;
                    } else {
                        i += 1;
                        i;
                    }
                }
            }
            ADL_Main_Control_Destroy.expect("non-null function pointer")();
        }
        dlclose(hDLL);
    }
    ret
}
