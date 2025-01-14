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
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn log10(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn round(_: libc::c_double) -> libc::c_double;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(__s: *mut libc::c_char, __n: libc::c_int, __stream: *mut FILE) -> *mut libc::c_char;
    fn fseeko(__stream: *mut FILE, __off: __off64_t, __whence: libc::c_int) -> libc::c_int;
    fn ftello(__stream: *mut FILE) -> __off64_t;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn strtoul(_: *const libc::c_char, _: *mut *mut libc::c_char, _: libc::c_int) -> libc::c_ulong;
    fn x264_cli_log(
        name: *const libc::c_char,
        i_level: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    static mut cli_input: cli_input_t;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
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
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type hnd_t = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cli_input_opt_t {
    pub index_file: *mut libc::c_char,
    pub format: *mut libc::c_char,
    pub resolution: *mut libc::c_char,
    pub colorspace: *mut libc::c_char,
    pub bit_depth: libc::c_int,
    pub timebase: *mut libc::c_char,
    pub seek: libc::c_int,
    pub progress: libc::c_int,
    pub output_csp: libc::c_int,
    pub output_range: libc::c_int,
    pub input_range: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct video_info_t {
    pub csp: libc::c_int,
    pub fps_num: uint32_t,
    pub fps_den: uint32_t,
    pub fullrange: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub interlaced: libc::c_int,
    pub num_frames: libc::c_int,
    pub sar_width: uint32_t,
    pub sar_height: uint32_t,
    pub tff: libc::c_int,
    pub thread_safe: libc::c_int,
    pub timebase_num: uint32_t,
    pub timebase_den: uint32_t,
    pub vfr: libc::c_int,
}
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
pub struct cli_input_t {
    pub open_file: Option<
        unsafe extern "C" fn(
            *mut libc::c_char,
            *mut hnd_t,
            *mut video_info_t,
            *mut cli_input_opt_t,
        ) -> libc::c_int,
    >,
    pub picture_alloc: Option<
        unsafe extern "C" fn(
            *mut cli_pic_t,
            hnd_t,
            libc::c_int,
            libc::c_int,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub read_frame: Option<unsafe extern "C" fn(*mut cli_pic_t, hnd_t, libc::c_int) -> libc::c_int>,
    pub release_frame: Option<unsafe extern "C" fn(*mut cli_pic_t, hnd_t) -> libc::c_int>,
    pub picture_clean: Option<unsafe extern "C" fn(*mut cli_pic_t, hnd_t) -> ()>,
    pub close_file: Option<unsafe extern "C" fn(hnd_t) -> libc::c_int>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timecode_hnd_t {
    pub input: cli_input_t,
    pub p_handle: hnd_t,
    pub auto_timebase_num: libc::c_int,
    pub auto_timebase_den: libc::c_int,
    pub timebase_num: uint64_t,
    pub timebase_den: uint64_t,
    pub stored_pts_num: libc::c_int,
    pub pts: *mut int64_t,
    pub assume_fps: libc::c_double,
    pub last_timecode: libc::c_double,
}
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
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    if fstat(fileno(filehandle), &mut file_stat) != 0 {
        return 1 as libc::c_int;
    }
    (file_stat.st_mode & 0o170000 as libc::c_int as __mode_t == 0o100000 as libc::c_int as __mode_t)
        as libc::c_int
}
#[inline]
unsafe extern "C" fn gcd(mut a: uint64_t, mut b: uint64_t) -> uint64_t {
    loop {
        let mut c: int64_t = (a % b) as int64_t;
        if c == 0 {
            return b;
        }
        a = b;
        b = c as uint64_t;
    }
}
#[inline]
unsafe extern "C" fn lcm(mut a: uint64_t, mut b: uint64_t) -> uint64_t {
    a / gcd(a, b) * b
}
#[inline]
unsafe extern "C" fn sigexp10(
    mut value: libc::c_double,
    mut exponent: *mut libc::c_double,
) -> libc::c_double {
    *exponent = pow(10 as libc::c_int as libc::c_double, floor(log10(value)));
    value / *exponent
}
unsafe extern "C" fn correct_fps(
    mut fps: libc::c_double,
    mut h: *mut timecode_hnd_t,
) -> libc::c_double {
    let mut i: libc::c_int = 1 as libc::c_int;
    let mut fps_num: uint64_t = 0;
    let mut fps_den: uint64_t = 0;
    let mut exponent: libc::c_double = 0.;
    let mut fps_sig: libc::c_double = sigexp10(fps, &mut exponent);
    loop {
        fps_den = i as uint64_t * (*h).timebase_num;
        fps_num = (round(fps_den as libc::c_double * fps_sig) * exponent) as uint64_t;
        if fps_num > 4294967295 as libc::c_uint as uint64_t {
            x264_cli_log(
                b"timecode\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
                b"tcfile fps correction failed.\n                  Specify an appropriate timebase manually or remake tcfile.\n\0"
                    as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int) as libc::c_double;
        }
        if fabs(fps_num as libc::c_double / fps_den as libc::c_double / exponent - fps_sig)
            < 5e-6f64
        {
            break;
        }
        i += 1;
        i;
    }
    if (*h).auto_timebase_den != 0 {
        (*h).timebase_den = if (*h).timebase_den != 0 {
            lcm((*h).timebase_den, fps_num)
        } else {
            fps_num
        };
        if (*h).timebase_den > 4294967295 as libc::c_uint as uint64_t {
            (*h).auto_timebase_den = 0 as libc::c_int;
        }
    }
    fps_num as libc::c_double / fps_den as libc::c_double
}
unsafe extern "C" fn try_mkv_timebase_den(
    mut fpss: *mut libc::c_double,
    mut h: *mut timecode_hnd_t,
    mut loop_num: libc::c_int,
) -> libc::c_int {
    (*h).timebase_num = 0 as libc::c_int as uint64_t;
    (*h).timebase_den = 1000000000 as libc::c_int as uint64_t;
    let mut num: libc::c_int = 0 as libc::c_int;
    while num < loop_num {
        let mut fps_den: uint64_t = 0;
        let mut exponent: libc::c_double = 0.;
        let mut fps_sig: libc::c_double = sigexp10(*fpss.offset(num as isize), &mut exponent);
        fps_den =
            (round(1000000000 as libc::c_int as libc::c_double / fps_sig) / exponent) as uint64_t;
        (*h).timebase_num = if fps_den != 0 && (*h).timebase_num != 0 {
            gcd((*h).timebase_num, fps_den)
        } else {
            fps_den
        };
        if (*h).timebase_num > 4294967295 as libc::c_uint as uint64_t || (*h).timebase_num == 0 {
            x264_cli_log(
                b"timecode\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
                b"automatic timebase generation failed.\n                  Specify timebase manually.\n\0"
                    as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        num += 1;
        num;
    }
    0 as libc::c_int
}
unsafe extern "C" fn parse_tcfile(
    mut tcfile_in: *mut FILE,
    mut h: *mut timecode_hnd_t,
    mut info: *mut video_info_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut buff: [libc::c_char; 256] = [0; 256];
    let mut ret: libc::c_int = 0;
    let mut tcfv: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    let mut seq_num: libc::c_int = 0;
    let mut timecodes_num: libc::c_int = 0;
    let mut timecodes: *mut libc::c_double = std::ptr::null_mut::<libc::c_double>();
    let mut fpss: *mut libc::c_double = std::ptr::null_mut::<libc::c_double>();
    ret = (!(fgets(
        buff.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
        tcfile_in,
    ))
    .is_null()
        && (sscanf(
            buff.as_mut_ptr(),
            b"# timecode format v%d\0" as *const u8 as *const libc::c_char,
            &mut tcfv as *mut libc::c_int,
        ) == 1 as libc::c_int
            || sscanf(
                buff.as_mut_ptr(),
                b"# timestamp format v%d\0" as *const u8 as *const libc::c_char,
                &mut tcfv as *mut libc::c_int,
            ) == 1 as libc::c_int)) as libc::c_int;
    if ret == 0 || tcfv != 1 as libc::c_int && tcfv != 2 as libc::c_int {
        x264_cli_log(
            b"timecode\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            b"unsupported timecode format\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if tcfv == 1 as libc::c_int {
        let mut file_pos: int64_t = 0;
        let mut assume_fps: libc::c_double = 0.;
        let mut seq_fps: libc::c_double = 0.;
        let mut start: libc::c_int = 0;
        let mut end: libc::c_int = -(1 as libc::c_int);
        let mut prev_start: libc::c_int = -(1 as libc::c_int);
        let mut prev_end: libc::c_int = -(1 as libc::c_int);
        (*h).assume_fps = 0 as libc::c_int as libc::c_double;
        num = 2 as libc::c_int;
        while !(fgets(
            buff.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
            tcfile_in,
        ))
        .is_null()
        {
            if buff[0 as libc::c_int as usize] as libc::c_int == '#' as i32
                || buff[0 as libc::c_int as usize] as libc::c_int == '\n' as i32
                || buff[0 as libc::c_int as usize] as libc::c_int == '\r' as i32
            {
                num += 1;
                num;
            } else {
                if sscanf(
                    buff.as_mut_ptr(),
                    b"assume %lf\0" as *const u8 as *const libc::c_char,
                    &mut (*h).assume_fps as *mut libc::c_double,
                ) != 1 as libc::c_int
                    && sscanf(
                        buff.as_mut_ptr(),
                        b"Assume %lf\0" as *const u8 as *const libc::c_char,
                        &mut (*h).assume_fps as *mut libc::c_double,
                    ) != 1 as libc::c_int
                {
                    x264_cli_log(
                        b"timecode\0" as *const u8 as *const libc::c_char,
                        0 as libc::c_int,
                        b"tcfile parsing error: assumed fps not found\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    return -(1 as libc::c_int);
                }
                break;
            }
        }
        if (*h).assume_fps <= 0 as libc::c_int as libc::c_double {
            x264_cli_log(
                b"timecode\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
                b"invalid assumed fps %.6f\n\0" as *const u8 as *const libc::c_char,
                (*h).assume_fps,
            );
            return -(1 as libc::c_int);
        }
        file_pos = ftello(tcfile_in);
        (*h).stored_pts_num = 0 as libc::c_int;
        seq_num = 0 as libc::c_int;
        while !(fgets(
            buff.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
            tcfile_in,
        ))
        .is_null()
        {
            if buff[0 as libc::c_int as usize] as libc::c_int == '#' as i32
                || buff[0 as libc::c_int as usize] as libc::c_int == '\n' as i32
                || buff[0 as libc::c_int as usize] as libc::c_int == '\r' as i32
            {
                if sscanf(
                    buff.as_mut_ptr(),
                    b"# TDecimate Mode 3:  Last Frame = %d\0" as *const u8 as *const libc::c_char,
                    &mut end as *mut libc::c_int,
                ) == 1 as libc::c_int
                {
                    (*h).stored_pts_num = end + 1 as libc::c_int;
                }
            } else {
                ret = sscanf(
                    buff.as_mut_ptr(),
                    b"%d,%d,%lf\0" as *const u8 as *const libc::c_char,
                    &mut start as *mut libc::c_int,
                    &mut end as *mut libc::c_int,
                    &mut seq_fps as *mut libc::c_double,
                );
                if ret != 3 as libc::c_int && ret != -(1 as libc::c_int) {
                    x264_cli_log(
                        b"timecode\0" as *const u8 as *const libc::c_char,
                        0 as libc::c_int,
                        b"invalid input tcfile\n\0" as *const u8 as *const libc::c_char,
                    );
                    return -(1 as libc::c_int);
                }
                if start > end
                    || start <= prev_start
                    || end <= prev_end
                    || seq_fps <= 0 as libc::c_int as libc::c_double
                {
                    x264_cli_log(
                        b"timecode\0" as *const u8 as *const libc::c_char,
                        0 as libc::c_int,
                        b"invalid input tcfile at line %d: %s\n\0" as *const u8
                            as *const libc::c_char,
                        num,
                        buff.as_mut_ptr(),
                    );
                    return -(1 as libc::c_int);
                }
                prev_start = start;
                prev_end = end;
                if (*h).auto_timebase_den != 0 || (*h).auto_timebase_num != 0 {
                    seq_num += 1;
                    seq_num;
                }
            }
            num += 1;
            num;
        }
        if (*h).stored_pts_num == 0 {
            (*h).stored_pts_num = end + 2 as libc::c_int;
        }
        timecodes_num = (*h).stored_pts_num;
        fseeko(tcfile_in, file_pos, 0 as libc::c_int);
        timecodes = malloc(
            (timecodes_num as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong),
        ) as *mut libc::c_double;
        if timecodes.is_null() {
            return -(1 as libc::c_int);
        }
        if (*h).auto_timebase_den != 0 || (*h).auto_timebase_num != 0 {
            fpss = malloc(
                ((seq_num + 1 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong),
            ) as *mut libc::c_double;
            if fpss.is_null() {
                current_block = 1109600788113682529;
            } else {
                current_block = 13678349939556791712;
            }
        } else {
            current_block = 13678349939556791712;
        }
        match current_block {
            1109600788113682529 => {}
            _ => {
                assume_fps = correct_fps((*h).assume_fps, h);
                if assume_fps < 0 as libc::c_int as libc::c_double {
                    current_block = 1109600788113682529;
                } else {
                    *timecodes.offset(0 as libc::c_int as isize) =
                        0 as libc::c_int as libc::c_double;
                    seq_num = 0 as libc::c_int;
                    num = seq_num;
                    loop {
                        if !(num < timecodes_num - 1 as libc::c_int
                            && !(fgets(
                                buff.as_mut_ptr(),
                                ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong
                                    as libc::c_int,
                                tcfile_in,
                            ))
                            .is_null())
                        {
                            current_block = 13660591889533726445;
                            break;
                        }
                        if buff[0 as libc::c_int as usize] as libc::c_int == '#' as i32
                            || buff[0 as libc::c_int as usize] as libc::c_int == '\n' as i32
                            || buff[0 as libc::c_int as usize] as libc::c_int == '\r' as i32
                        {
                            continue;
                        }
                        ret = sscanf(
                            buff.as_mut_ptr(),
                            b"%d,%d,%lf\0" as *const u8 as *const libc::c_char,
                            &mut start as *mut libc::c_int,
                            &mut end as *mut libc::c_int,
                            &mut seq_fps as *mut libc::c_double,
                        );
                        if ret != 3 as libc::c_int {
                            end = timecodes_num - 1 as libc::c_int;
                            start = end;
                        }
                        while num < start && num < timecodes_num - 1 as libc::c_int {
                            *timecodes.offset((num + 1 as libc::c_int) as isize) = *timecodes
                                .offset(num as isize)
                                + 1 as libc::c_int as libc::c_double / assume_fps;
                            num += 1;
                            num;
                        }
                        if num >= timecodes_num - 1 as libc::c_int {
                            continue;
                        }
                        if (*h).auto_timebase_den != 0 || (*h).auto_timebase_num != 0 {
                            let fresh0 = seq_num;
                            seq_num += 1;
                            *fpss.offset(fresh0 as isize) = seq_fps;
                        }
                        seq_fps = correct_fps(seq_fps, h);
                        if seq_fps < 0 as libc::c_int as libc::c_double {
                            current_block = 1109600788113682529;
                            break;
                        }
                        num = start;
                        while num <= end && num < timecodes_num - 1 as libc::c_int {
                            *timecodes.offset((num + 1 as libc::c_int) as isize) = *timecodes
                                .offset(num as isize)
                                + 1 as libc::c_int as libc::c_double / seq_fps;
                            num += 1;
                            num;
                        }
                    }
                    match current_block {
                        1109600788113682529 => {}
                        _ => {
                            while num < timecodes_num - 1 as libc::c_int {
                                *timecodes.offset((num + 1 as libc::c_int) as isize) = *timecodes
                                    .offset(num as isize)
                                    + 1 as libc::c_int as libc::c_double / assume_fps;
                                num += 1;
                                num;
                            }
                            if (*h).auto_timebase_den != 0 || (*h).auto_timebase_num != 0 {
                                *fpss.offset(seq_num as isize) = (*h).assume_fps;
                            }
                            if (*h).auto_timebase_num != 0 && (*h).auto_timebase_den == 0 {
                                let mut exponent: libc::c_double = 0.;
                                let mut assume_fps_sig: libc::c_double = 0.;
                                let mut seq_fps_sig: libc::c_double = 0.;
                                if try_mkv_timebase_den(fpss, h, seq_num + 1 as libc::c_int)
                                    < 0 as libc::c_int
                                {
                                    current_block = 1109600788113682529;
                                } else {
                                    fseeko(tcfile_in, file_pos, 0 as libc::c_int);
                                    assume_fps_sig = sigexp10((*h).assume_fps, &mut exponent);
                                    assume_fps = 1000000000 as libc::c_int as libc::c_double
                                        / (round(
                                            1000000000 as libc::c_int as libc::c_double
                                                / assume_fps_sig,
                                        ) / exponent);
                                    num = 0 as libc::c_int;
                                    while num < timecodes_num - 1 as libc::c_int
                                        && !(fgets(
                                            buff.as_mut_ptr(),
                                            ::core::mem::size_of::<[libc::c_char; 256]>()
                                                as libc::c_ulong
                                                as libc::c_int,
                                            tcfile_in,
                                        ))
                                        .is_null()
                                    {
                                        if buff[0 as libc::c_int as usize] as libc::c_int
                                            == '#' as i32
                                            || buff[0 as libc::c_int as usize] as libc::c_int
                                                == '\n' as i32
                                            || buff[0 as libc::c_int as usize] as libc::c_int
                                                == '\r' as i32
                                        {
                                            continue;
                                        }
                                        ret = sscanf(
                                            buff.as_mut_ptr(),
                                            b"%d,%d,%lf\0" as *const u8 as *const libc::c_char,
                                            &mut start as *mut libc::c_int,
                                            &mut end as *mut libc::c_int,
                                            &mut seq_fps as *mut libc::c_double,
                                        );
                                        if ret != 3 as libc::c_int {
                                            end = timecodes_num - 1 as libc::c_int;
                                            start = end;
                                        }
                                        seq_fps_sig = sigexp10(seq_fps, &mut exponent);
                                        seq_fps = 1000000000 as libc::c_int as libc::c_double
                                            / (round(
                                                1000000000 as libc::c_int as libc::c_double
                                                    / seq_fps_sig,
                                            ) / exponent);
                                        while num < start && num < timecodes_num - 1 as libc::c_int
                                        {
                                            *timecodes.offset((num + 1 as libc::c_int) as isize) =
                                                *timecodes.offset(num as isize)
                                                    + 1 as libc::c_int as libc::c_double
                                                        / assume_fps;
                                            num += 1;
                                            num;
                                        }
                                        num = start;
                                        while num <= end && num < timecodes_num - 1 as libc::c_int {
                                            *timecodes.offset((num + 1 as libc::c_int) as isize) =
                                                *timecodes.offset(num as isize)
                                                    + 1 as libc::c_int as libc::c_double / seq_fps;
                                            num += 1;
                                            num;
                                        }
                                    }
                                    while num < timecodes_num - 1 as libc::c_int {
                                        *timecodes.offset((num + 1 as libc::c_int) as isize) =
                                            *timecodes.offset(num as isize)
                                                + 1 as libc::c_int as libc::c_double / assume_fps;
                                        num += 1;
                                        num;
                                    }
                                    current_block = 496303045384785551;
                                }
                            } else {
                                current_block = 496303045384785551;
                            }
                            match current_block {
                                1109600788113682529 => {}
                                _ => {
                                    if !fpss.is_null() {
                                        free(fpss as *mut libc::c_void);
                                        fpss = std::ptr::null_mut::<libc::c_double>();
                                    }
                                    (*h).assume_fps = assume_fps;
                                    (*h).last_timecode = *timecodes
                                        .offset((timecodes_num - 1 as libc::c_int) as isize);
                                    current_block = 12129449210080749085;
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        let mut file_pos_0: int64_t = ftello(tcfile_in);
        (*h).stored_pts_num = 0 as libc::c_int;
        while !(fgets(
            buff.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
            tcfile_in,
        ))
        .is_null()
        {
            if buff[0 as libc::c_int as usize] as libc::c_int == '#' as i32
                || buff[0 as libc::c_int as usize] as libc::c_int == '\n' as i32
                || buff[0 as libc::c_int as usize] as libc::c_int == '\r' as i32
            {
                if (*h).stored_pts_num == 0 {
                    file_pos_0 = ftello(tcfile_in);
                }
            } else {
                (*h).stored_pts_num += 1;
                (*h).stored_pts_num;
            }
        }
        timecodes_num = (*h).stored_pts_num;
        if timecodes_num == 0 {
            x264_cli_log(
                b"timecode\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
                b"input tcfile doesn't have any timecodes!\n\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        fseeko(tcfile_in, file_pos_0, 0 as libc::c_int);
        timecodes = malloc(
            (timecodes_num as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong),
        ) as *mut libc::c_double;
        if timecodes.is_null() {
            return -(1 as libc::c_int);
        }
        num = 0 as libc::c_int;
        if !(fgets(
            buff.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
            tcfile_in,
        ))
        .is_null()
        {
            ret = sscanf(
                buff.as_mut_ptr(),
                b"%lf\0" as *const u8 as *const libc::c_char,
                &mut *timecodes.offset(0 as libc::c_int as isize) as *mut libc::c_double,
            );
            *timecodes.offset(0 as libc::c_int as isize) *= 1e-3f64;
            if ret != 1 as libc::c_int {
                x264_cli_log(
                    b"timecode\0" as *const u8 as *const libc::c_char,
                    0 as libc::c_int,
                    b"invalid input tcfile for frame 0\n\0" as *const u8 as *const libc::c_char,
                );
                return -(1 as libc::c_int);
            }
            num = 1 as libc::c_int;
            while num < timecodes_num
                && !(fgets(
                    buff.as_mut_ptr(),
                    ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
                    tcfile_in,
                ))
                .is_null()
            {
                if buff[0 as libc::c_int as usize] as libc::c_int == '#' as i32
                    || buff[0 as libc::c_int as usize] as libc::c_int == '\n' as i32
                    || buff[0 as libc::c_int as usize] as libc::c_int == '\r' as i32
                {
                    continue;
                }
                ret = sscanf(
                    buff.as_mut_ptr(),
                    b"%lf\0" as *const u8 as *const libc::c_char,
                    &mut *timecodes.offset(num as isize) as *mut libc::c_double,
                );
                *timecodes.offset(num as isize) *= 1e-3f64;
                if ret != 1 as libc::c_int
                    || *timecodes.offset(num as isize)
                        <= *timecodes.offset((num - 1 as libc::c_int) as isize)
                {
                    x264_cli_log(
                        b"timecode\0" as *const u8 as *const libc::c_char,
                        0 as libc::c_int,
                        b"invalid input tcfile for frame %d\n\0" as *const u8
                            as *const libc::c_char,
                        num,
                    );
                    return -(1 as libc::c_int);
                }
                num += 1;
                num;
            }
        }
        if num < timecodes_num {
            x264_cli_log(
                b"timecode\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
                b"failed to read input tcfile for frame %d\0" as *const u8 as *const libc::c_char,
                num,
            );
            return -(1 as libc::c_int);
        }
        if timecodes_num == 1 as libc::c_int {
            (*h).timebase_den = (*info).fps_num as uint64_t;
            current_block = 6938158527927677584;
        } else if (*h).auto_timebase_den != 0 {
            fpss = malloc(
                ((timecodes_num - 1 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong),
            ) as *mut libc::c_double;
            if fpss.is_null() {
                current_block = 1109600788113682529;
            } else {
                num = 0 as libc::c_int;
                while num < timecodes_num - 1 as libc::c_int {
                    *fpss.offset(num as isize) = 1 as libc::c_int as libc::c_double
                        / (*timecodes.offset((num + 1 as libc::c_int) as isize)
                            - *timecodes.offset(num as isize));
                    if (*h).auto_timebase_den != 0 {
                        let mut i: libc::c_int = 1 as libc::c_int;
                        let mut fps_num: uint64_t = 0;
                        let mut fps_den: uint64_t = 0;
                        let mut exponent_0: libc::c_double = 0.;
                        let mut fps_sig: libc::c_double =
                            sigexp10(*fpss.offset(num as isize), &mut exponent_0);
                        loop {
                            fps_den = i as uint64_t * (*h).timebase_num;
                            fps_num = (round(fps_den as libc::c_double * fps_sig) * exponent_0)
                                as uint64_t;
                            if fps_num > 4294967295 as libc::c_uint as uint64_t
                                || fabs(
                                    fps_num as libc::c_double
                                        / fps_den as libc::c_double
                                        / exponent_0
                                        - fps_sig,
                                ) < 5e-6f64
                            {
                                break;
                            }
                            i += 1;
                            i;
                        }
                        (*h).timebase_den = if fps_num != 0 && (*h).timebase_den != 0 {
                            lcm((*h).timebase_den, fps_num)
                        } else {
                            fps_num
                        };
                        if (*h).timebase_den > 4294967295 as libc::c_uint as uint64_t {
                            (*h).auto_timebase_den = 0 as libc::c_int;
                        }
                    }
                    num += 1;
                    num;
                }
                if (*h).auto_timebase_num != 0 && (*h).auto_timebase_den == 0 {
                    if try_mkv_timebase_den(fpss, h, timecodes_num - 1 as libc::c_int)
                        < 0 as libc::c_int
                    {
                        current_block = 1109600788113682529;
                    } else {
                        current_block = 10783567741412653655;
                    }
                } else {
                    current_block = 10783567741412653655;
                }
                match current_block {
                    1109600788113682529 => {}
                    _ => {
                        free(fpss as *mut libc::c_void);
                        fpss = std::ptr::null_mut::<libc::c_double>();
                        current_block = 6938158527927677584;
                    }
                }
            }
        } else {
            current_block = 6938158527927677584;
        }
        match current_block {
            1109600788113682529 => {}
            _ => {
                if timecodes_num > 1 as libc::c_int {
                    (*h).assume_fps = 1 as libc::c_int as libc::c_double
                        / (*timecodes.offset((timecodes_num - 1 as libc::c_int) as isize)
                            - *timecodes.offset((timecodes_num - 2 as libc::c_int) as isize));
                } else {
                    (*h).assume_fps =
                        (*info).fps_num as libc::c_double / (*info).fps_den as libc::c_double;
                }
                (*h).last_timecode = *timecodes.offset((timecodes_num - 1 as libc::c_int) as isize);
                current_block = 12129449210080749085;
            }
        }
    }
    if current_block == 12129449210080749085 {
        if (*h).auto_timebase_den != 0 || (*h).auto_timebase_num != 0 {
            let mut i_0: uint64_t = gcd((*h).timebase_num, (*h).timebase_den);
            (*h).timebase_num /= i_0;
            (*h).timebase_den /= i_0;
            x264_cli_log(
                b"timecode\0" as *const u8 as *const libc::c_char,
                2 as libc::c_int,
                b"automatic timebase generation %lu/%lu\n\0" as *const u8 as *const libc::c_char,
                (*h).timebase_num,
                (*h).timebase_den,
            );
        } else if (*h).timebase_den > 4294967295 as libc::c_uint as uint64_t
            || (*h).timebase_den == 0
        {
            x264_cli_log(
                b"timecode\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
                b"automatic timebase generation failed.\n                  Specify an appropriate timebase manually.\n\0"
                    as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        (*h).pts = malloc(
            ((*h).stored_pts_num as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<int64_t>() as libc::c_ulong),
        ) as *mut int64_t;
        if !((*h).pts).is_null() {
            num = 0 as libc::c_int;
            while num < (*h).stored_pts_num {
                *((*h).pts).offset(num as isize) = (*timecodes.offset(num as isize)
                    * ((*h).timebase_den as libc::c_double / (*h).timebase_num as libc::c_double)
                    + 0.5f64) as int64_t;
                if num > 0 as libc::c_int
                    && *((*h).pts).offset(num as isize)
                        <= *((*h).pts).offset((num - 1 as libc::c_int) as isize)
                {
                    x264_cli_log(
                        b"timecode\0" as *const u8 as *const libc::c_char,
                        0 as libc::c_int,
                        b"invalid timebase or timecode for frame %d\n\0" as *const u8
                            as *const libc::c_char,
                        num,
                    );
                    return -(1 as libc::c_int);
                }
                num += 1;
                num;
            }
            free(timecodes as *mut libc::c_void);
            return 0 as libc::c_int;
        }
    }
    if !timecodes.is_null() {
        free(timecodes as *mut libc::c_void);
    }
    if !fpss.is_null() {
        free(fpss as *mut libc::c_void);
    }
    -(1 as libc::c_int)
}
unsafe extern "C" fn open_file(
    mut psz_filename: *mut libc::c_char,
    mut p_handle: *mut hnd_t,
    mut info: *mut video_info_t,
    mut opt: *mut cli_input_opt_t,
) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut tcfile_in: *mut FILE = std::ptr::null_mut::<FILE>();
    let mut h: *mut timecode_hnd_t =
        malloc(::core::mem::size_of::<timecode_hnd_t>() as libc::c_ulong) as *mut timecode_hnd_t;
    if h.is_null() {
        x264_cli_log(
            b"timecode\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            b"malloc failed\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    (*h).input = cli_input;
    (*h).p_handle = *p_handle;
    (*h).pts = std::ptr::null_mut::<int64_t>();
    if !((*opt).timebase).is_null() {
        ret = sscanf(
            (*opt).timebase,
            b"%lu/%lu\0" as *const u8 as *const libc::c_char,
            &mut (*h).timebase_num as *mut uint64_t,
            &mut (*h).timebase_den as *mut uint64_t,
        );
        if ret == 1 as libc::c_int {
            (*h).timebase_num = strtoul(
                (*opt).timebase,
                std::ptr::null_mut::<*mut libc::c_char>(),
                10 as libc::c_int,
            );
            (*h).timebase_den = 0 as libc::c_int as uint64_t;
        }
        if (*h).timebase_num > 4294967295 as libc::c_uint as uint64_t
            || (*h).timebase_den > 4294967295 as libc::c_uint as uint64_t
        {
            x264_cli_log(
                b"timecode\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
                b"timebase you specified exceeds H.264 maximum\n\0" as *const u8
                    as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    }
    (*h).auto_timebase_num = (ret == 0) as libc::c_int;
    (*h).auto_timebase_den = (ret < 2 as libc::c_int) as libc::c_int;
    if (*h).auto_timebase_num != 0 {
        (*h).timebase_num = (*info).fps_den as uint64_t;
    }
    if (*h).auto_timebase_den != 0 {
        (*h).timebase_den = 0 as libc::c_int as uint64_t;
    }
    tcfile_in = fopen(psz_filename, b"rb\0" as *const u8 as *const libc::c_char);
    if tcfile_in.is_null() {
        x264_cli_log(
            b"timecode\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            b"can't open `%s'\n\0" as *const u8 as *const libc::c_char,
            psz_filename,
        );
        return -(1 as libc::c_int);
    }
    if x264_is_regular_file(tcfile_in) == 0 {
        x264_cli_log(
            b"timecode\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            b"tcfile input incompatible with non-regular file `%s'\n\0" as *const u8
                as *const libc::c_char,
            psz_filename,
        );
        fclose(tcfile_in);
        return -(1 as libc::c_int);
    }
    if parse_tcfile(tcfile_in, h, info) < 0 as libc::c_int {
        if !((*h).pts).is_null() {
            free((*h).pts as *mut libc::c_void);
        }
        fclose(tcfile_in);
        return -(1 as libc::c_int);
    }
    fclose(tcfile_in);
    (*info).timebase_num = (*h).timebase_num as uint32_t;
    (*info).timebase_den = (*h).timebase_den as uint32_t;
    (*info).vfr = 1 as libc::c_int;
    *p_handle = h as hnd_t;
    0 as libc::c_int
}
unsafe extern "C" fn get_frame_pts(
    mut h: *mut timecode_hnd_t,
    mut frame: libc::c_int,
    mut real_frame: libc::c_int,
) -> int64_t {
    if frame < (*h).stored_pts_num {
        *((*h).pts).offset(frame as isize)
    } else {
        if !((*h).pts).is_null() && real_frame != 0 {
            x264_cli_log(
                b"timecode\0" as *const u8 as *const libc::c_char,
                2 as libc::c_int,
                b"input timecode file missing data for frame %d and later\n                 assuming constant fps %.6f\n\0"
                    as *const u8 as *const libc::c_char,
                frame,
                (*h).assume_fps,
            );
            free((*h).pts as *mut libc::c_void);
            (*h).pts = std::ptr::null_mut::<int64_t>();
        }
        let mut timecode: libc::c_double =
            (*h).last_timecode + 1 as libc::c_int as libc::c_double / (*h).assume_fps;
        if real_frame != 0 {
            (*h).last_timecode = timecode;
        }
        (timecode * ((*h).timebase_den as libc::c_double / (*h).timebase_num as libc::c_double)
            + 0.5f64) as int64_t
    }
}
unsafe extern "C" fn read_frame(
    mut pic: *mut cli_pic_t,
    mut handle: hnd_t,
    mut frame: libc::c_int,
) -> libc::c_int {
    let mut h: *mut timecode_hnd_t = handle as *mut timecode_hnd_t;
    if ((*h).input.read_frame).expect("non-null function pointer")(pic, (*h).p_handle, frame) != 0 {
        return -(1 as libc::c_int);
    }
    (*pic).pts = get_frame_pts(h, frame, 1 as libc::c_int);
    (*pic).duration = get_frame_pts(h, frame + 1 as libc::c_int, 0 as libc::c_int) - (*pic).pts;
    0 as libc::c_int
}
unsafe extern "C" fn release_frame(mut pic: *mut cli_pic_t, mut handle: hnd_t) -> libc::c_int {
    let mut h: *mut timecode_hnd_t = handle as *mut timecode_hnd_t;
    if ((*h).input.release_frame).is_some() {
        return ((*h).input.release_frame).expect("non-null function pointer")(pic, (*h).p_handle);
    }
    0 as libc::c_int
}
unsafe extern "C" fn picture_alloc(
    mut pic: *mut cli_pic_t,
    mut handle: hnd_t,
    mut csp: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> libc::c_int {
    let mut h: *mut timecode_hnd_t = handle as *mut timecode_hnd_t;
    ((*h).input.picture_alloc).expect("non-null function pointer")(
        pic,
        (*h).p_handle,
        csp,
        width,
        height,
    )
}
unsafe extern "C" fn picture_clean(mut pic: *mut cli_pic_t, mut handle: hnd_t) {
    let mut h: *mut timecode_hnd_t = handle as *mut timecode_hnd_t;
    ((*h).input.picture_clean).expect("non-null function pointer")(pic, (*h).p_handle);
}
unsafe extern "C" fn close_file(mut handle: hnd_t) -> libc::c_int {
    let mut h: *mut timecode_hnd_t = handle as *mut timecode_hnd_t;
    if !((*h).pts).is_null() {
        free((*h).pts as *mut libc::c_void);
    }
    ((*h).input.close_file).expect("non-null function pointer")((*h).p_handle);
    free(h as *mut libc::c_void);
    0 as libc::c_int
}
#[no_mangle]
pub static mut timecode_input: cli_input_t = unsafe {
    {
        cli_input_t {
            open_file: Some(
                open_file
                    as unsafe extern "C" fn(
                        *mut libc::c_char,
                        *mut hnd_t,
                        *mut video_info_t,
                        *mut cli_input_opt_t,
                    ) -> libc::c_int,
            ),
            picture_alloc: Some(
                picture_alloc
                    as unsafe extern "C" fn(
                        *mut cli_pic_t,
                        hnd_t,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                    ) -> libc::c_int,
            ),
            read_frame: Some(
                read_frame
                    as unsafe extern "C" fn(*mut cli_pic_t, hnd_t, libc::c_int) -> libc::c_int,
            ),
            release_frame: Some(
                release_frame as unsafe extern "C" fn(*mut cli_pic_t, hnd_t) -> libc::c_int,
            ),
            picture_clean: Some(picture_clean as unsafe extern "C" fn(*mut cli_pic_t, hnd_t) -> ()),
            close_file: Some(close_file as unsafe extern "C" fn(hnd_t) -> libc::c_int),
        }
    }
};
