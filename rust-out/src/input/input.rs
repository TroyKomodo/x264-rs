#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn x264_malloc(_: int64_t) -> *mut libc::c_void;
    fn x264_free(_: *mut libc::c_void);
    fn munmap(__addr: *mut libc::c_void, __len: size_t) -> libc::c_int;
    fn madvise(
        __addr: *mut libc::c_void,
        __len: size_t,
        __advice: libc::c_int,
    ) -> libc::c_int;
    fn mmap(
        __addr: *mut libc::c_void,
        __len: size_t,
        __prot: libc::c_int,
        __flags: libc::c_int,
        __fd: libc::c_int,
        __offset: __off64_t,
    ) -> *mut libc::c_void;
    fn sysconf(__name: libc::c_int) -> libc::c_long;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
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
pub type uint64_t = __uint64_t;
pub type intptr_t = libc::c_long;
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
pub struct cli_mmap_t {
    pub file_size: int64_t,
    pub align_mask: libc::c_int,
    pub fd: libc::c_int,
}
pub const _SC_PAGESIZE: C2RustUnnamed = 30;
pub type C2RustUnnamed = libc::c_uint;
pub const _SC_SIGSTKSZ: C2RustUnnamed = 250;
pub const _SC_MINSIGSTKSZ: C2RustUnnamed = 249;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: C2RustUnnamed = 248;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: C2RustUnnamed = 247;
pub const _SC_XOPEN_STREAMS: C2RustUnnamed = 246;
pub const _SC_TRACE_USER_EVENT_MAX: C2RustUnnamed = 245;
pub const _SC_TRACE_SYS_MAX: C2RustUnnamed = 244;
pub const _SC_TRACE_NAME_MAX: C2RustUnnamed = 243;
pub const _SC_TRACE_EVENT_NAME_MAX: C2RustUnnamed = 242;
pub const _SC_SS_REPL_MAX: C2RustUnnamed = 241;
pub const _SC_V7_LPBIG_OFFBIG: C2RustUnnamed = 240;
pub const _SC_V7_LP64_OFF64: C2RustUnnamed = 239;
pub const _SC_V7_ILP32_OFFBIG: C2RustUnnamed = 238;
pub const _SC_V7_ILP32_OFF32: C2RustUnnamed = 237;
pub const _SC_RAW_SOCKETS: C2RustUnnamed = 236;
pub const _SC_IPV6: C2RustUnnamed = 235;
pub const _SC_LEVEL4_CACHE_LINESIZE: C2RustUnnamed = 199;
pub const _SC_LEVEL4_CACHE_ASSOC: C2RustUnnamed = 198;
pub const _SC_LEVEL4_CACHE_SIZE: C2RustUnnamed = 197;
pub const _SC_LEVEL3_CACHE_LINESIZE: C2RustUnnamed = 196;
pub const _SC_LEVEL3_CACHE_ASSOC: C2RustUnnamed = 195;
pub const _SC_LEVEL3_CACHE_SIZE: C2RustUnnamed = 194;
pub const _SC_LEVEL2_CACHE_LINESIZE: C2RustUnnamed = 193;
pub const _SC_LEVEL2_CACHE_ASSOC: C2RustUnnamed = 192;
pub const _SC_LEVEL2_CACHE_SIZE: C2RustUnnamed = 191;
pub const _SC_LEVEL1_DCACHE_LINESIZE: C2RustUnnamed = 190;
pub const _SC_LEVEL1_DCACHE_ASSOC: C2RustUnnamed = 189;
pub const _SC_LEVEL1_DCACHE_SIZE: C2RustUnnamed = 188;
pub const _SC_LEVEL1_ICACHE_LINESIZE: C2RustUnnamed = 187;
pub const _SC_LEVEL1_ICACHE_ASSOC: C2RustUnnamed = 186;
pub const _SC_LEVEL1_ICACHE_SIZE: C2RustUnnamed = 185;
pub const _SC_TRACE_LOG: C2RustUnnamed = 184;
pub const _SC_TRACE_INHERIT: C2RustUnnamed = 183;
pub const _SC_TRACE_EVENT_FILTER: C2RustUnnamed = 182;
pub const _SC_TRACE: C2RustUnnamed = 181;
pub const _SC_HOST_NAME_MAX: C2RustUnnamed = 180;
pub const _SC_V6_LPBIG_OFFBIG: C2RustUnnamed = 179;
pub const _SC_V6_LP64_OFF64: C2RustUnnamed = 178;
pub const _SC_V6_ILP32_OFFBIG: C2RustUnnamed = 177;
pub const _SC_V6_ILP32_OFF32: C2RustUnnamed = 176;
pub const _SC_2_PBS_CHECKPOINT: C2RustUnnamed = 175;
pub const _SC_STREAMS: C2RustUnnamed = 174;
pub const _SC_SYMLOOP_MAX: C2RustUnnamed = 173;
pub const _SC_2_PBS_TRACK: C2RustUnnamed = 172;
pub const _SC_2_PBS_MESSAGE: C2RustUnnamed = 171;
pub const _SC_2_PBS_LOCATE: C2RustUnnamed = 170;
pub const _SC_2_PBS_ACCOUNTING: C2RustUnnamed = 169;
pub const _SC_2_PBS: C2RustUnnamed = 168;
pub const _SC_USER_GROUPS_R: C2RustUnnamed = 167;
pub const _SC_USER_GROUPS: C2RustUnnamed = 166;
pub const _SC_TYPED_MEMORY_OBJECTS: C2RustUnnamed = 165;
pub const _SC_TIMEOUTS: C2RustUnnamed = 164;
pub const _SC_SYSTEM_DATABASE_R: C2RustUnnamed = 163;
pub const _SC_SYSTEM_DATABASE: C2RustUnnamed = 162;
pub const _SC_THREAD_SPORADIC_SERVER: C2RustUnnamed = 161;
pub const _SC_SPORADIC_SERVER: C2RustUnnamed = 160;
pub const _SC_SPAWN: C2RustUnnamed = 159;
pub const _SC_SIGNALS: C2RustUnnamed = 158;
pub const _SC_SHELL: C2RustUnnamed = 157;
pub const _SC_REGEX_VERSION: C2RustUnnamed = 156;
pub const _SC_REGEXP: C2RustUnnamed = 155;
pub const _SC_SPIN_LOCKS: C2RustUnnamed = 154;
pub const _SC_READER_WRITER_LOCKS: C2RustUnnamed = 153;
pub const _SC_NETWORKING: C2RustUnnamed = 152;
pub const _SC_SINGLE_PROCESS: C2RustUnnamed = 151;
pub const _SC_MULTI_PROCESS: C2RustUnnamed = 150;
pub const _SC_MONOTONIC_CLOCK: C2RustUnnamed = 149;
pub const _SC_FILE_SYSTEM: C2RustUnnamed = 148;
pub const _SC_FILE_LOCKING: C2RustUnnamed = 147;
pub const _SC_FILE_ATTRIBUTES: C2RustUnnamed = 146;
pub const _SC_PIPE: C2RustUnnamed = 145;
pub const _SC_FIFO: C2RustUnnamed = 144;
pub const _SC_FD_MGMT: C2RustUnnamed = 143;
pub const _SC_DEVICE_SPECIFIC_R: C2RustUnnamed = 142;
pub const _SC_DEVICE_SPECIFIC: C2RustUnnamed = 141;
pub const _SC_DEVICE_IO: C2RustUnnamed = 140;
pub const _SC_THREAD_CPUTIME: C2RustUnnamed = 139;
pub const _SC_CPUTIME: C2RustUnnamed = 138;
pub const _SC_CLOCK_SELECTION: C2RustUnnamed = 137;
pub const _SC_C_LANG_SUPPORT_R: C2RustUnnamed = 136;
pub const _SC_C_LANG_SUPPORT: C2RustUnnamed = 135;
pub const _SC_BASE: C2RustUnnamed = 134;
pub const _SC_BARRIERS: C2RustUnnamed = 133;
pub const _SC_ADVISORY_INFO: C2RustUnnamed = 132;
pub const _SC_XOPEN_REALTIME_THREADS: C2RustUnnamed = 131;
pub const _SC_XOPEN_REALTIME: C2RustUnnamed = 130;
pub const _SC_XOPEN_LEGACY: C2RustUnnamed = 129;
pub const _SC_XBS5_LPBIG_OFFBIG: C2RustUnnamed = 128;
pub const _SC_XBS5_LP64_OFF64: C2RustUnnamed = 127;
pub const _SC_XBS5_ILP32_OFFBIG: C2RustUnnamed = 126;
pub const _SC_XBS5_ILP32_OFF32: C2RustUnnamed = 125;
pub const _SC_NL_TEXTMAX: C2RustUnnamed = 124;
pub const _SC_NL_SETMAX: C2RustUnnamed = 123;
pub const _SC_NL_NMAX: C2RustUnnamed = 122;
pub const _SC_NL_MSGMAX: C2RustUnnamed = 121;
pub const _SC_NL_LANGMAX: C2RustUnnamed = 120;
pub const _SC_NL_ARGMAX: C2RustUnnamed = 119;
pub const _SC_USHRT_MAX: C2RustUnnamed = 118;
pub const _SC_ULONG_MAX: C2RustUnnamed = 117;
pub const _SC_UINT_MAX: C2RustUnnamed = 116;
pub const _SC_UCHAR_MAX: C2RustUnnamed = 115;
pub const _SC_SHRT_MIN: C2RustUnnamed = 114;
pub const _SC_SHRT_MAX: C2RustUnnamed = 113;
pub const _SC_SCHAR_MIN: C2RustUnnamed = 112;
pub const _SC_SCHAR_MAX: C2RustUnnamed = 111;
pub const _SC_SSIZE_MAX: C2RustUnnamed = 110;
pub const _SC_NZERO: C2RustUnnamed = 109;
pub const _SC_MB_LEN_MAX: C2RustUnnamed = 108;
pub const _SC_WORD_BIT: C2RustUnnamed = 107;
pub const _SC_LONG_BIT: C2RustUnnamed = 106;
pub const _SC_INT_MIN: C2RustUnnamed = 105;
pub const _SC_INT_MAX: C2RustUnnamed = 104;
pub const _SC_CHAR_MIN: C2RustUnnamed = 103;
pub const _SC_CHAR_MAX: C2RustUnnamed = 102;
pub const _SC_CHAR_BIT: C2RustUnnamed = 101;
pub const _SC_XOPEN_XPG4: C2RustUnnamed = 100;
pub const _SC_XOPEN_XPG3: C2RustUnnamed = 99;
pub const _SC_XOPEN_XPG2: C2RustUnnamed = 98;
pub const _SC_2_UPE: C2RustUnnamed = 97;
pub const _SC_2_C_VERSION: C2RustUnnamed = 96;
pub const _SC_2_CHAR_TERM: C2RustUnnamed = 95;
pub const _SC_XOPEN_SHM: C2RustUnnamed = 94;
pub const _SC_XOPEN_ENH_I18N: C2RustUnnamed = 93;
pub const _SC_XOPEN_CRYPT: C2RustUnnamed = 92;
pub const _SC_XOPEN_UNIX: C2RustUnnamed = 91;
pub const _SC_XOPEN_XCU_VERSION: C2RustUnnamed = 90;
pub const _SC_XOPEN_VERSION: C2RustUnnamed = 89;
pub const _SC_PASS_MAX: C2RustUnnamed = 88;
pub const _SC_ATEXIT_MAX: C2RustUnnamed = 87;
pub const _SC_AVPHYS_PAGES: C2RustUnnamed = 86;
pub const _SC_PHYS_PAGES: C2RustUnnamed = 85;
pub const _SC_NPROCESSORS_ONLN: C2RustUnnamed = 84;
pub const _SC_NPROCESSORS_CONF: C2RustUnnamed = 83;
pub const _SC_THREAD_PROCESS_SHARED: C2RustUnnamed = 82;
pub const _SC_THREAD_PRIO_PROTECT: C2RustUnnamed = 81;
pub const _SC_THREAD_PRIO_INHERIT: C2RustUnnamed = 80;
pub const _SC_THREAD_PRIORITY_SCHEDULING: C2RustUnnamed = 79;
pub const _SC_THREAD_ATTR_STACKSIZE: C2RustUnnamed = 78;
pub const _SC_THREAD_ATTR_STACKADDR: C2RustUnnamed = 77;
pub const _SC_THREAD_THREADS_MAX: C2RustUnnamed = 76;
pub const _SC_THREAD_STACK_MIN: C2RustUnnamed = 75;
pub const _SC_THREAD_KEYS_MAX: C2RustUnnamed = 74;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: C2RustUnnamed = 73;
pub const _SC_TTY_NAME_MAX: C2RustUnnamed = 72;
pub const _SC_LOGIN_NAME_MAX: C2RustUnnamed = 71;
pub const _SC_GETPW_R_SIZE_MAX: C2RustUnnamed = 70;
pub const _SC_GETGR_R_SIZE_MAX: C2RustUnnamed = 69;
pub const _SC_THREAD_SAFE_FUNCTIONS: C2RustUnnamed = 68;
pub const _SC_THREADS: C2RustUnnamed = 67;
pub const _SC_T_IOV_MAX: C2RustUnnamed = 66;
pub const _SC_PII_OSI_M: C2RustUnnamed = 65;
pub const _SC_PII_OSI_CLTS: C2RustUnnamed = 64;
pub const _SC_PII_OSI_COTS: C2RustUnnamed = 63;
pub const _SC_PII_INTERNET_DGRAM: C2RustUnnamed = 62;
pub const _SC_PII_INTERNET_STREAM: C2RustUnnamed = 61;
pub const _SC_IOV_MAX: C2RustUnnamed = 60;
pub const _SC_UIO_MAXIOV: C2RustUnnamed = 60;
pub const _SC_SELECT: C2RustUnnamed = 59;
pub const _SC_POLL: C2RustUnnamed = 58;
pub const _SC_PII_OSI: C2RustUnnamed = 57;
pub const _SC_PII_INTERNET: C2RustUnnamed = 56;
pub const _SC_PII_SOCKET: C2RustUnnamed = 55;
pub const _SC_PII_XTI: C2RustUnnamed = 54;
pub const _SC_PII: C2RustUnnamed = 53;
pub const _SC_2_LOCALEDEF: C2RustUnnamed = 52;
pub const _SC_2_SW_DEV: C2RustUnnamed = 51;
pub const _SC_2_FORT_RUN: C2RustUnnamed = 50;
pub const _SC_2_FORT_DEV: C2RustUnnamed = 49;
pub const _SC_2_C_DEV: C2RustUnnamed = 48;
pub const _SC_2_C_BIND: C2RustUnnamed = 47;
pub const _SC_2_VERSION: C2RustUnnamed = 46;
pub const _SC_CHARCLASS_NAME_MAX: C2RustUnnamed = 45;
pub const _SC_RE_DUP_MAX: C2RustUnnamed = 44;
pub const _SC_LINE_MAX: C2RustUnnamed = 43;
pub const _SC_EXPR_NEST_MAX: C2RustUnnamed = 42;
pub const _SC_EQUIV_CLASS_MAX: C2RustUnnamed = 41;
pub const _SC_COLL_WEIGHTS_MAX: C2RustUnnamed = 40;
pub const _SC_BC_STRING_MAX: C2RustUnnamed = 39;
pub const _SC_BC_SCALE_MAX: C2RustUnnamed = 38;
pub const _SC_BC_DIM_MAX: C2RustUnnamed = 37;
pub const _SC_BC_BASE_MAX: C2RustUnnamed = 36;
pub const _SC_TIMER_MAX: C2RustUnnamed = 35;
pub const _SC_SIGQUEUE_MAX: C2RustUnnamed = 34;
pub const _SC_SEM_VALUE_MAX: C2RustUnnamed = 33;
pub const _SC_SEM_NSEMS_MAX: C2RustUnnamed = 32;
pub const _SC_RTSIG_MAX: C2RustUnnamed = 31;
pub const _SC_VERSION: C2RustUnnamed = 29;
pub const _SC_MQ_PRIO_MAX: C2RustUnnamed = 28;
pub const _SC_MQ_OPEN_MAX: C2RustUnnamed = 27;
pub const _SC_DELAYTIMER_MAX: C2RustUnnamed = 26;
pub const _SC_AIO_PRIO_DELTA_MAX: C2RustUnnamed = 25;
pub const _SC_AIO_MAX: C2RustUnnamed = 24;
pub const _SC_AIO_LISTIO_MAX: C2RustUnnamed = 23;
pub const _SC_SHARED_MEMORY_OBJECTS: C2RustUnnamed = 22;
pub const _SC_SEMAPHORES: C2RustUnnamed = 21;
pub const _SC_MESSAGE_PASSING: C2RustUnnamed = 20;
pub const _SC_MEMORY_PROTECTION: C2RustUnnamed = 19;
pub const _SC_MEMLOCK_RANGE: C2RustUnnamed = 18;
pub const _SC_MEMLOCK: C2RustUnnamed = 17;
pub const _SC_MAPPED_FILES: C2RustUnnamed = 16;
pub const _SC_FSYNC: C2RustUnnamed = 15;
pub const _SC_SYNCHRONIZED_IO: C2RustUnnamed = 14;
pub const _SC_PRIORITIZED_IO: C2RustUnnamed = 13;
pub const _SC_ASYNCHRONOUS_IO: C2RustUnnamed = 12;
pub const _SC_TIMERS: C2RustUnnamed = 11;
pub const _SC_PRIORITY_SCHEDULING: C2RustUnnamed = 10;
pub const _SC_REALTIME_SIGNALS: C2RustUnnamed = 9;
pub const _SC_SAVED_IDS: C2RustUnnamed = 8;
pub const _SC_JOB_CONTROL: C2RustUnnamed = 7;
pub const _SC_TZNAME_MAX: C2RustUnnamed = 6;
pub const _SC_STREAM_MAX: C2RustUnnamed = 5;
pub const _SC_OPEN_MAX: C2RustUnnamed = 4;
pub const _SC_NGROUPS_MAX: C2RustUnnamed = 3;
pub const _SC_CLK_TCK: C2RustUnnamed = 2;
pub const _SC_CHILD_MAX: C2RustUnnamed = 1;
pub const _SC_ARG_MAX: C2RustUnnamed = 0;
#[no_mangle]
pub static mut x264_cli_csps: [x264_cli_csp_t; 17] = [
    x264_cli_csp_t {
        name: 0 as *const libc::c_char,
        planes: 0,
        width: [0.; 4],
        height: [0.; 4],
        mod_width: 0,
        mod_height: 0,
    },
    {
        
        x264_cli_csp_t {
            name: b"i400\0" as *const u8 as *const libc::c_char,
            planes: 1 as libc::c_int,
            width: [1 as libc::c_int as libc::c_float, 0., 0., 0.],
            height: [1 as libc::c_int as libc::c_float, 0., 0., 0.],
            mod_width: 1 as libc::c_int,
            mod_height: 1 as libc::c_int,
        }
    },
    {
        
        x264_cli_csp_t {
            name: b"i420\0" as *const u8 as *const libc::c_char,
            planes: 3 as libc::c_int,
            width: [
                1 as libc::c_int as libc::c_float,
                0.5f64 as libc::c_float,
                0.5f64 as libc::c_float,
                0.,
            ],
            height: [
                1 as libc::c_int as libc::c_float,
                0.5f64 as libc::c_float,
                0.5f64 as libc::c_float,
                0.,
            ],
            mod_width: 2 as libc::c_int,
            mod_height: 2 as libc::c_int,
        }
    },
    {
        
        x264_cli_csp_t {
            name: b"yv12\0" as *const u8 as *const libc::c_char,
            planes: 3 as libc::c_int,
            width: [
                1 as libc::c_int as libc::c_float,
                0.5f64 as libc::c_float,
                0.5f64 as libc::c_float,
                0.,
            ],
            height: [
                1 as libc::c_int as libc::c_float,
                0.5f64 as libc::c_float,
                0.5f64 as libc::c_float,
                0.,
            ],
            mod_width: 2 as libc::c_int,
            mod_height: 2 as libc::c_int,
        }
    },
    {
        
        x264_cli_csp_t {
            name: b"nv12\0" as *const u8 as *const libc::c_char,
            planes: 2 as libc::c_int,
            width: [
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0.,
                0.,
            ],
            height: [1 as libc::c_int as libc::c_float, 0.5f64 as libc::c_float, 0., 0.],
            mod_width: 2 as libc::c_int,
            mod_height: 2 as libc::c_int,
        }
    },
    {
        
        x264_cli_csp_t {
            name: b"nv21\0" as *const u8 as *const libc::c_char,
            planes: 2 as libc::c_int,
            width: [
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0.,
                0.,
            ],
            height: [1 as libc::c_int as libc::c_float, 0.5f64 as libc::c_float, 0., 0.],
            mod_width: 2 as libc::c_int,
            mod_height: 2 as libc::c_int,
        }
    },
    {
        
        x264_cli_csp_t {
            name: b"i422\0" as *const u8 as *const libc::c_char,
            planes: 3 as libc::c_int,
            width: [
                1 as libc::c_int as libc::c_float,
                0.5f64 as libc::c_float,
                0.5f64 as libc::c_float,
                0.,
            ],
            height: [
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0.,
            ],
            mod_width: 2 as libc::c_int,
            mod_height: 1 as libc::c_int,
        }
    },
    {
        
        x264_cli_csp_t {
            name: b"yv16\0" as *const u8 as *const libc::c_char,
            planes: 3 as libc::c_int,
            width: [
                1 as libc::c_int as libc::c_float,
                0.5f64 as libc::c_float,
                0.5f64 as libc::c_float,
                0.,
            ],
            height: [
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0.,
            ],
            mod_width: 2 as libc::c_int,
            mod_height: 1 as libc::c_int,
        }
    },
    {
        
        x264_cli_csp_t {
            name: b"nv16\0" as *const u8 as *const libc::c_char,
            planes: 2 as libc::c_int,
            width: [
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0.,
                0.,
            ],
            height: [
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0.,
                0.,
            ],
            mod_width: 2 as libc::c_int,
            mod_height: 1 as libc::c_int,
        }
    },
    {
        
        x264_cli_csp_t {
            name: b"yuyv\0" as *const u8 as *const libc::c_char,
            planes: 1 as libc::c_int,
            width: [2 as libc::c_int as libc::c_float, 0., 0., 0.],
            height: [1 as libc::c_int as libc::c_float, 0., 0., 0.],
            mod_width: 2 as libc::c_int,
            mod_height: 1 as libc::c_int,
        }
    },
    {
        
        x264_cli_csp_t {
            name: b"uyvy\0" as *const u8 as *const libc::c_char,
            planes: 1 as libc::c_int,
            width: [2 as libc::c_int as libc::c_float, 0., 0., 0.],
            height: [1 as libc::c_int as libc::c_float, 0., 0., 0.],
            mod_width: 2 as libc::c_int,
            mod_height: 1 as libc::c_int,
        }
    },
    x264_cli_csp_t {
        name: 0 as *const libc::c_char,
        planes: 0,
        width: [0.; 4],
        height: [0.; 4],
        mod_width: 0,
        mod_height: 0,
    },
    {
        
        x264_cli_csp_t {
            name: b"i444\0" as *const u8 as *const libc::c_char,
            planes: 3 as libc::c_int,
            width: [
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0.,
            ],
            height: [
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0.,
            ],
            mod_width: 1 as libc::c_int,
            mod_height: 1 as libc::c_int,
        }
    },
    {
        
        x264_cli_csp_t {
            name: b"yv24\0" as *const u8 as *const libc::c_char,
            planes: 3 as libc::c_int,
            width: [
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0.,
            ],
            height: [
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0.,
            ],
            mod_width: 1 as libc::c_int,
            mod_height: 1 as libc::c_int,
        }
    },
    {
        
        x264_cli_csp_t {
            name: b"bgr\0" as *const u8 as *const libc::c_char,
            planes: 1 as libc::c_int,
            width: [3 as libc::c_int as libc::c_float, 0., 0., 0.],
            height: [1 as libc::c_int as libc::c_float, 0., 0., 0.],
            mod_width: 1 as libc::c_int,
            mod_height: 1 as libc::c_int,
        }
    },
    {
        
        x264_cli_csp_t {
            name: b"bgra\0" as *const u8 as *const libc::c_char,
            planes: 1 as libc::c_int,
            width: [4 as libc::c_int as libc::c_float, 0., 0., 0.],
            height: [1 as libc::c_int as libc::c_float, 0., 0., 0.],
            mod_width: 1 as libc::c_int,
            mod_height: 1 as libc::c_int,
        }
    },
    {
        
        x264_cli_csp_t {
            name: b"rgb\0" as *const u8 as *const libc::c_char,
            planes: 1 as libc::c_int,
            width: [3 as libc::c_int as libc::c_float, 0., 0., 0.],
            height: [1 as libc::c_int as libc::c_float, 0., 0., 0.],
            mod_width: 1 as libc::c_int,
            mod_height: 1 as libc::c_int,
        }
    },
];
#[no_mangle]
pub unsafe extern "C" fn x264_cli_csp_is_invalid(mut csp: libc::c_int) -> libc::c_int {
    let mut csp_mask: libc::c_int = csp & 0xff as libc::c_int;
    (csp_mask <= 0 as libc::c_int || csp_mask >= 0x11 as libc::c_int
        || csp_mask == 0xb as libc::c_int || csp & 0x4000 as libc::c_int != 0)
        as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn x264_cli_csp_depth_factor(mut csp: libc::c_int) -> libc::c_int {
    if x264_cli_csp_is_invalid(csp) != 0 {
        return 0 as libc::c_int;
    }
    if csp & 0x2000 as libc::c_int != 0 {
        2 as libc::c_int
    } else {
        1 as libc::c_int
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_cli_pic_plane_size(
    mut csp: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut plane: libc::c_int,
) -> int64_t {
    let mut csp_mask: libc::c_int = csp & 0xff as libc::c_int;
    if x264_cli_csp_is_invalid(csp) != 0 || plane < 0 as libc::c_int
        || plane >= x264_cli_csps[csp_mask as usize].planes
    {
        return 0 as libc::c_int as int64_t;
    }
    let mut size: int64_t = width as int64_t * height as int64_t;
    size = (size as libc::c_float
        * (x264_cli_csps[csp_mask as usize].width[plane as usize]
            * x264_cli_csps[csp_mask as usize].height[plane as usize])) as int64_t;
    size *= x264_cli_csp_depth_factor(csp) as int64_t;
    size
}
#[no_mangle]
pub unsafe extern "C" fn x264_cli_pic_size(
    mut csp: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> int64_t {
    if x264_cli_csp_is_invalid(csp) != 0 {
        return 0 as libc::c_int as int64_t;
    }
    let mut size: int64_t = 0 as libc::c_int as int64_t;
    let mut csp_mask: libc::c_int = csp & 0xff as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < x264_cli_csps[csp_mask as usize].planes {
        size += x264_cli_pic_plane_size(csp, width, height, i);
        i += 1;
        i;
    }
    size
}
unsafe extern "C" fn cli_pic_init_internal(
    mut pic: *mut cli_pic_t,
    mut csp: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut align: libc::c_int,
    mut alloc: libc::c_int,
) -> libc::c_int {
    memset(
        pic as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<cli_pic_t>() as libc::c_ulong,
    );
    let mut csp_mask: libc::c_int = csp & 0xff as libc::c_int;
    if x264_cli_csp_is_invalid(csp) != 0 {
        (*pic).img.planes = 0 as libc::c_int;
    } else {
        (*pic).img.planes = x264_cli_csps[csp_mask as usize].planes;
    }
    (*pic).img.csp = csp;
    (*pic).img.width = width;
    (*pic).img.height = height;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*pic).img.planes {
        let mut stride: libc::c_int = (width as libc::c_float
            * x264_cli_csps[csp_mask as usize].width[i as usize]) as libc::c_int;
        stride *= x264_cli_csp_depth_factor(csp);
        stride = (stride + (align - 1 as libc::c_int)) & !(align - 1 as libc::c_int);
        (*pic).img.stride[i as usize] = stride;
        if alloc != 0 {
            let mut size: int64_t = (height as libc::c_float
                * x264_cli_csps[csp_mask as usize].height[i as usize]) as int64_t
                * stride as int64_t;
            (*pic).img.plane[i as usize] = x264_malloc(size) as *mut uint8_t;
            if ((*pic).img.plane[i as usize]).is_null() {
                return -(1 as libc::c_int);
            }
        }
        i += 1;
        i;
    }
    0 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn x264_cli_pic_alloc(
    mut pic: *mut cli_pic_t,
    mut csp: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> libc::c_int {
    cli_pic_init_internal(
        pic,
        csp,
        width,
        height,
        1 as libc::c_int,
        1 as libc::c_int,
    )
}
#[no_mangle]
pub unsafe extern "C" fn x264_cli_pic_alloc_aligned(
    mut pic: *mut cli_pic_t,
    mut csp: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> libc::c_int {
    cli_pic_init_internal(
        pic,
        csp,
        width,
        height,
        64 as libc::c_int,
        1 as libc::c_int,
    )
}
#[no_mangle]
pub unsafe extern "C" fn x264_cli_pic_init_noalloc(
    mut pic: *mut cli_pic_t,
    mut csp: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> libc::c_int {
    cli_pic_init_internal(
        pic,
        csp,
        width,
        height,
        1 as libc::c_int,
        0 as libc::c_int,
    )
}
#[no_mangle]
pub unsafe extern "C" fn x264_cli_pic_clean(mut pic: *mut cli_pic_t) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*pic).img.planes {
        x264_free((*pic).img.plane[i as usize] as *mut libc::c_void);
        i += 1;
        i;
    }
    memset(
        pic as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<cli_pic_t>() as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn x264_cli_get_csp(
    mut csp: libc::c_int,
) -> *const x264_cli_csp_t {
    if x264_cli_csp_is_invalid(csp) != 0 {
        return std::ptr::null::<x264_cli_csp_t>();
    }
    x264_cli_csps.as_ptr().offset((csp & 0xff as libc::c_int) as isize)
}
#[no_mangle]
pub unsafe extern "C" fn x264_cli_mmap_init(
    mut h: *mut cli_mmap_t,
    mut fh: *mut FILE,
) -> libc::c_int {
    let mut fd: libc::c_int = fileno(fh);
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
    if fstat(fd, &mut file_stat) == 0 {
        (*h).file_size = file_stat.st_size;
        (*h)
            .align_mask = (sysconf(_SC_PAGESIZE as libc::c_int)
            - 1 as libc::c_int as libc::c_long) as libc::c_int;
        (*h).fd = fd;
        return ((*h).align_mask < 0 as libc::c_int || fd < 0 as libc::c_int)
            as libc::c_int;
    }
    -(1 as libc::c_int)
}
#[no_mangle]
pub unsafe extern "C" fn x264_cli_mmap(
    mut h: *mut cli_mmap_t,
    mut offset: int64_t,
    mut size: int64_t,
) -> *mut libc::c_void {
    let mut base: *mut uint8_t = std::ptr::null_mut::<uint8_t>();
    let mut align: libc::c_int = (offset & (*h).align_mask as int64_t) as libc::c_int;
    if offset < 0 as libc::c_int as int64_t || size < 0 as libc::c_int as int64_t
        || size as uint64_t
            > (18446744073709551615 as libc::c_ulong)
                .wrapping_sub(64 as libc::c_int as libc::c_ulong)
                .wrapping_sub(align as libc::c_ulong)
    {
        return std::ptr::null_mut::<libc::c_void>();
    }
    offset -= align as int64_t;
    size += align as int64_t;
    let mut padded_size: size_t = (size + 64 as libc::c_int as int64_t) as size_t;
    base = mmap(
        std::ptr::null_mut::<libc::c_void>(),
        padded_size,
        0x1 as libc::c_int,
        0x2 as libc::c_int,
        (*h).fd,
        offset,
    ) as *mut uint8_t;
    if base != -(1 as libc::c_int) as *mut libc::c_void as *mut uint8_t {
        madvise(base as *mut libc::c_void, size as size_t, 3 as libc::c_int);
        let mut aligned_size: size_t = padded_size
            .wrapping_sub(1 as libc::c_int as size_t) & !(*h).align_mask as size_t;
        if (offset as size_t).wrapping_add(aligned_size) >= (*h).file_size as size_t {
            mmap(
                base.offset(aligned_size as isize) as *mut libc::c_void,
                padded_size.wrapping_sub(aligned_size),
                0x1 as libc::c_int,
                0x2 as libc::c_int | 0x10 as libc::c_int,
                (*h).fd,
                (offset + size - 1 as libc::c_int as int64_t) & !(*h).align_mask as int64_t,
            );
        }
        return base.offset(align as isize) as *mut libc::c_void;
    }
    std::ptr::null_mut::<libc::c_void>()
}
#[no_mangle]
pub unsafe extern "C" fn x264_cli_munmap(
    mut h: *mut cli_mmap_t,
    mut addr: *mut libc::c_void,
    mut size: int64_t,
) -> libc::c_int {
    let mut base: *mut libc::c_void = (addr as intptr_t & !(*h).align_mask as intptr_t)
        as *mut libc::c_void;
    if size < 0 as libc::c_int as int64_t
        || size as libc::c_ulong
            > (18446744073709551615 as libc::c_ulong)
                .wrapping_sub(64 as libc::c_int as libc::c_ulong)
                .wrapping_sub((addr as intptr_t - base as intptr_t) as libc::c_ulong)
    {
        return -(1 as libc::c_int);
    }
    munmap(
        base,
        (size + 64 as libc::c_int as int64_t + addr as intptr_t - base as intptr_t)
            as size_t,
    )
}
#[no_mangle]
pub unsafe extern "C" fn x264_cli_mmap_close(mut h: *mut cli_mmap_t) {}
