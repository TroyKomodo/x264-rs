#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn __sched_cpucount(__setsize: size_t, __setp: *const cpu_set_t) -> libc::c_int;
    fn sched_getaffinity(
        __pid: __pid_t,
        __cpusetsize: size_t,
        __cpuset: *mut cpu_set_t,
    ) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type __pid_t = libc::c_int;
pub type uint32_t = __uint32_t;
pub type __cpu_mask = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpu_set_t {
    pub __bits: [__cpu_mask; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_cpu_name_t {
    pub name: *const libc::c_char,
    pub flags: uint32_t,
}
#[no_mangle]
pub static mut x264_cpu_names: [x264_cpu_name_t; 28] = [
    {
        
        x264_cpu_name_t {
            name: b"MMX2\0" as *const u8 as *const libc::c_char,
            flags: ((1 as libc::c_uint) << 0 as libc::c_int) | ((1 as libc::c_uint) << 1 as libc::c_int),
        }
    },
    {
        
        x264_cpu_name_t {
            name: b"MMXEXT\0" as *const u8 as *const libc::c_char,
            flags: ((1 as libc::c_uint) << 0 as libc::c_int) | ((1 as libc::c_uint) << 1 as libc::c_int),
        }
    },
    {
        
        x264_cpu_name_t {
            name: b"SSE\0" as *const u8 as *const libc::c_char,
            flags: ((1 as libc::c_uint) << 0 as libc::c_int) | ((1 as libc::c_uint) << 1 as libc::c_int) | ((1 as libc::c_uint) << 2 as libc::c_int),
        }
    },
    {
        
        x264_cpu_name_t {
            name: b"SSE2Slow\0" as *const u8 as *const libc::c_char,
            flags: ((1 as libc::c_uint) << 0 as libc::c_int) | ((1 as libc::c_uint) << 1 as libc::c_int) | ((1 as libc::c_uint) << 2 as libc::c_int) | ((1 as libc::c_uint) << 3 as libc::c_int) | ((1 as libc::c_uint) << 19 as libc::c_int),
        }
    },
    {
        
        x264_cpu_name_t {
            name: b"SSE2\0" as *const u8 as *const libc::c_char,
            flags: ((1 as libc::c_uint) << 0 as libc::c_int) | ((1 as libc::c_uint) << 1 as libc::c_int) | ((1 as libc::c_uint) << 2 as libc::c_int) | ((1 as libc::c_uint) << 3 as libc::c_int),
        }
    },
    {
        
        x264_cpu_name_t {
            name: b"SSE2Fast\0" as *const u8 as *const libc::c_char,
            flags: ((1 as libc::c_uint) << 0 as libc::c_int) | ((1 as libc::c_uint) << 1 as libc::c_int) | ((1 as libc::c_uint) << 2 as libc::c_int) | ((1 as libc::c_uint) << 3 as libc::c_int) | ((1 as libc::c_uint) << 20 as libc::c_int),
        }
    },
    {
        
        x264_cpu_name_t {
            name: b"LZCNT\0" as *const u8 as *const libc::c_char,
            flags: ((1 as libc::c_uint) << 0 as libc::c_int) | ((1 as libc::c_uint) << 1 as libc::c_int) | ((1 as libc::c_uint) << 2 as libc::c_int) | ((1 as libc::c_uint) << 3 as libc::c_int) | ((1 as libc::c_uint) << 4 as libc::c_int),
        }
    },
    {
        
        x264_cpu_name_t {
            name: b"SSE3\0" as *const u8 as *const libc::c_char,
            flags: ((1 as libc::c_uint) << 0 as libc::c_int) | ((1 as libc::c_uint) << 1 as libc::c_int) | ((1 as libc::c_uint) << 2 as libc::c_int) | ((1 as libc::c_uint) << 3 as libc::c_int) | ((1 as libc::c_uint) << 5 as libc::c_int),
        }
    },
    {
        
        x264_cpu_name_t {
            name: b"SSSE3\0" as *const u8 as *const libc::c_char,
            flags: ((1 as libc::c_uint) << 0 as libc::c_int) | ((1 as libc::c_uint) << 1 as libc::c_int) | ((1 as libc::c_uint) << 2 as libc::c_int) | ((1 as libc::c_uint) << 3 as libc::c_int) | ((1 as libc::c_uint) << 5 as libc::c_int) | ((1 as libc::c_uint) << 6 as libc::c_int),
        }
    },
    {
        
        x264_cpu_name_t {
            name: b"SSE4.1\0" as *const u8 as *const libc::c_char,
            flags: ((1 as libc::c_uint) << 0 as libc::c_int) | ((1 as libc::c_uint) << 1 as libc::c_int) | ((1 as libc::c_uint) << 2 as libc::c_int) | ((1 as libc::c_uint) << 3 as libc::c_int) | ((1 as libc::c_uint) << 5 as libc::c_int) | ((1 as libc::c_uint) << 6 as libc::c_int) | ((1 as libc::c_uint) << 7 as libc::c_int),
        }
    },
    {
        
        x264_cpu_name_t {
            name: b"SSE4\0" as *const u8 as *const libc::c_char,
            flags: ((1 as libc::c_uint) << 0 as libc::c_int) | ((1 as libc::c_uint) << 1 as libc::c_int) | ((1 as libc::c_uint) << 2 as libc::c_int) | ((1 as libc::c_uint) << 3 as libc::c_int) | ((1 as libc::c_uint) << 5 as libc::c_int) | ((1 as libc::c_uint) << 6 as libc::c_int) | ((1 as libc::c_uint) << 7 as libc::c_int),
        }
    },
    {
        
        x264_cpu_name_t {
            name: b"SSE4.2\0" as *const u8 as *const libc::c_char,
            flags: ((1 as libc::c_uint) << 0 as libc::c_int) | ((1 as libc::c_uint) << 1 as libc::c_int) | ((1 as libc::c_uint) << 2 as libc::c_int) | ((1 as libc::c_uint) << 3 as libc::c_int) | ((1 as libc::c_uint) << 5 as libc::c_int) | ((1 as libc::c_uint) << 6 as libc::c_int) | ((1 as libc::c_uint) << 7 as libc::c_int) | ((1 as libc::c_uint) << 8 as libc::c_int),
        }
    },
    {
        
        x264_cpu_name_t {
            name: b"AVX\0" as *const u8 as *const libc::c_char,
            flags: ((1 as libc::c_uint) << 0 as libc::c_int) | ((1 as libc::c_uint) << 1 as libc::c_int) | ((1 as libc::c_uint) << 2 as libc::c_int) | ((1 as libc::c_uint) << 3 as libc::c_int) | ((1 as libc::c_uint) << 5 as libc::c_int) | ((1 as libc::c_uint) << 6 as libc::c_int) | ((1 as libc::c_uint) << 7 as libc::c_int) | ((1 as libc::c_uint) << 8 as libc::c_int) | ((1 as libc::c_uint) << 9 as libc::c_int),
        }
    },
    {
        
        x264_cpu_name_t {
            name: b"XOP\0" as *const u8 as *const libc::c_char,
            flags: ((1 as libc::c_uint) << 0 as libc::c_int) | ((1 as libc::c_uint) << 1 as libc::c_int) | ((1 as libc::c_uint) << 2 as libc::c_int) | ((1 as libc::c_uint) << 3 as libc::c_int) | ((1 as libc::c_uint) << 5 as libc::c_int) | ((1 as libc::c_uint) << 6 as libc::c_int) | ((1 as libc::c_uint) << 7 as libc::c_int) | ((1 as libc::c_uint) << 8 as libc::c_int) | ((1 as libc::c_uint) << 9 as libc::c_int) | ((1 as libc::c_uint) << 10 as libc::c_int),
        }
    },
    {
        
        x264_cpu_name_t {
            name: b"FMA4\0" as *const u8 as *const libc::c_char,
            flags: ((1 as libc::c_uint) << 0 as libc::c_int) | ((1 as libc::c_uint) << 1 as libc::c_int) | ((1 as libc::c_uint) << 2 as libc::c_int) | ((1 as libc::c_uint) << 3 as libc::c_int) | ((1 as libc::c_uint) << 5 as libc::c_int) | ((1 as libc::c_uint) << 6 as libc::c_int) | ((1 as libc::c_uint) << 7 as libc::c_int) | ((1 as libc::c_uint) << 8 as libc::c_int) | ((1 as libc::c_uint) << 9 as libc::c_int) | ((1 as libc::c_uint) << 11 as libc::c_int),
        }
    },
    {
        
        x264_cpu_name_t {
            name: b"FMA3\0" as *const u8 as *const libc::c_char,
            flags: ((1 as libc::c_uint) << 0 as libc::c_int) | ((1 as libc::c_uint) << 1 as libc::c_int) | ((1 as libc::c_uint) << 2 as libc::c_int) | ((1 as libc::c_uint) << 3 as libc::c_int) | ((1 as libc::c_uint) << 5 as libc::c_int) | ((1 as libc::c_uint) << 6 as libc::c_int) | ((1 as libc::c_uint) << 7 as libc::c_int) | ((1 as libc::c_uint) << 8 as libc::c_int) | ((1 as libc::c_uint) << 9 as libc::c_int) | ((1 as libc::c_uint) << 12 as libc::c_int),
        }
    },
    {
        
        x264_cpu_name_t {
            name: b"BMI1\0" as *const u8 as *const libc::c_char,
            flags: ((1 as libc::c_uint) << 0 as libc::c_int) | ((1 as libc::c_uint) << 1 as libc::c_int) | ((1 as libc::c_uint) << 2 as libc::c_int) | ((1 as libc::c_uint) << 3 as libc::c_int) | ((1 as libc::c_uint) << 5 as libc::c_int) | ((1 as libc::c_uint) << 6 as libc::c_int) | ((1 as libc::c_uint) << 7 as libc::c_int) | ((1 as libc::c_uint) << 8 as libc::c_int) | ((1 as libc::c_uint) << 9 as libc::c_int) | ((1 as libc::c_uint) << 4 as libc::c_int) | ((1 as libc::c_uint) << 13 as libc::c_int),
        }
    },
    {
        
        x264_cpu_name_t {
            name: b"BMI2\0" as *const u8 as *const libc::c_char,
            flags: ((1 as libc::c_uint) << 0 as libc::c_int) | ((1 as libc::c_uint) << 1 as libc::c_int) | ((1 as libc::c_uint) << 2 as libc::c_int) | ((1 as libc::c_uint) << 3 as libc::c_int) | ((1 as libc::c_uint) << 5 as libc::c_int) | ((1 as libc::c_uint) << 6 as libc::c_int) | ((1 as libc::c_uint) << 7 as libc::c_int) | ((1 as libc::c_uint) << 8 as libc::c_int) | ((1 as libc::c_uint) << 9 as libc::c_int) | ((1 as libc::c_uint) << 4 as libc::c_int) | ((1 as libc::c_uint) << 13 as libc::c_int) | ((1 as libc::c_uint) << 14 as libc::c_int),
        }
    },
    {
        
        x264_cpu_name_t {
            name: b"AVX2\0" as *const u8 as *const libc::c_char,
            flags: ((1 as libc::c_uint) << 0 as libc::c_int) | ((1 as libc::c_uint) << 1 as libc::c_int) | ((1 as libc::c_uint) << 2 as libc::c_int) | ((1 as libc::c_uint) << 3 as libc::c_int) | ((1 as libc::c_uint) << 5 as libc::c_int) | ((1 as libc::c_uint) << 6 as libc::c_int) | ((1 as libc::c_uint) << 7 as libc::c_int) | ((1 as libc::c_uint) << 8 as libc::c_int) | ((1 as libc::c_uint) << 9 as libc::c_int) | ((1 as libc::c_uint) << 12 as libc::c_int) | ((1 as libc::c_uint) << 4 as libc::c_int) | ((1 as libc::c_uint) << 13 as libc::c_int) | ((1 as libc::c_uint) << 14 as libc::c_int) | ((1 as libc::c_uint) << 15 as libc::c_int),
        }
    },
    {
        
        x264_cpu_name_t {
            name: b"AVX512\0" as *const u8 as *const libc::c_char,
            flags: ((1 as libc::c_uint) << 0 as libc::c_int) | ((1 as libc::c_uint) << 1 as libc::c_int) | ((1 as libc::c_uint) << 2 as libc::c_int) | ((1 as libc::c_uint) << 3 as libc::c_int) | ((1 as libc::c_uint) << 5 as libc::c_int) | ((1 as libc::c_uint) << 6 as libc::c_int) | ((1 as libc::c_uint) << 7 as libc::c_int) | ((1 as libc::c_uint) << 8 as libc::c_int) | ((1 as libc::c_uint) << 9 as libc::c_int) | ((1 as libc::c_uint) << 12 as libc::c_int) | ((1 as libc::c_uint) << 4 as libc::c_int) | ((1 as libc::c_uint) << 13 as libc::c_int) | ((1 as libc::c_uint) << 14 as libc::c_int) | ((1 as libc::c_uint) << 15 as libc::c_int) | ((1 as libc::c_uint) << 16 as libc::c_int),
        }
    },
    {
        
        x264_cpu_name_t {
            name: b"Cache32\0" as *const u8 as *const libc::c_char,
            flags: (1 as libc::c_uint) << 17 as libc::c_int,
        }
    },
    {
        
        x264_cpu_name_t {
            name: b"Cache64\0" as *const u8 as *const libc::c_char,
            flags: (1 as libc::c_uint) << 18 as libc::c_int,
        }
    },
    {
        
        x264_cpu_name_t {
            name: b"SlowAtom\0" as *const u8 as *const libc::c_char,
            flags: (1 as libc::c_uint) << 23 as libc::c_int,
        }
    },
    {
        
        x264_cpu_name_t {
            name: b"SlowPshufb\0" as *const u8 as *const libc::c_char,
            flags: (1 as libc::c_uint) << 24 as libc::c_int,
        }
    },
    {
        
        x264_cpu_name_t {
            name: b"SlowPalignr\0" as *const u8 as *const libc::c_char,
            flags: (1 as libc::c_uint) << 25 as libc::c_int,
        }
    },
    {
        
        x264_cpu_name_t {
            name: b"SlowShuffle\0" as *const u8 as *const libc::c_char,
            flags: (1 as libc::c_uint) << 21 as libc::c_int,
        }
    },
    {
        
        x264_cpu_name_t {
            name: b"UnalignedStack\0" as *const u8 as *const libc::c_char,
            flags: (1 as libc::c_uint) << 22 as libc::c_int,
        }
    },
    {
        
        x264_cpu_name_t {
            name: b"\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int as uint32_t,
        }
    },
];
#[no_mangle]
pub unsafe extern "C" fn x264_cpu_detect() -> uint32_t {
    0 as libc::c_int as uint32_t
}
#[no_mangle]
pub unsafe extern "C" fn x264_cpu_num_processors() -> libc::c_int {
    let mut p_aff: cpu_set_t = cpu_set_t { __bits: [0; 16] };
    memset(
        &mut p_aff as *mut cpu_set_t as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<cpu_set_t>() as libc::c_ulong,
    );
    if sched_getaffinity(
        0 as libc::c_int,
        ::core::mem::size_of::<cpu_set_t>() as libc::c_ulong,
        &mut p_aff,
    ) != 0
    {
        return 1 as libc::c_int;
    }
    __sched_cpucount(
        ::core::mem::size_of::<cpu_set_t>() as libc::c_ulong,
        &mut p_aff,
    )
}
