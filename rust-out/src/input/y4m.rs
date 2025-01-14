#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgetc(__stream: *mut FILE) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fseeko(
        __stream: *mut FILE,
        __off: __off64_t,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn ftello(__stream: *mut FILE) -> __off64_t;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn x264_reduce_fraction(n: *mut uint32_t, d: *mut uint32_t);
    fn x264_cli_log(
        name: *const libc::c_char,
        i_level: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    fn x264_cli_mmap_close(h: *mut cli_mmap_t);
    fn x264_cli_pic_clean(pic: *mut cli_pic_t);
    fn x264_cli_munmap(
        h: *mut cli_mmap_t,
        addr: *mut libc::c_void,
        size: int64_t,
    ) -> libc::c_int;
    fn x264_cli_csp_depth_factor(csp: libc::c_int) -> libc::c_int;
    fn x264_cli_mmap(
        h: *mut cli_mmap_t,
        offset: int64_t,
        size: int64_t,
    ) -> *mut libc::c_void;
    fn x264_cli_pic_alloc(
        pic: *mut cli_pic_t,
        csp: libc::c_int,
        width: libc::c_int,
        height: libc::c_int,
    ) -> libc::c_int;
    fn x264_cli_pic_init_noalloc(
        pic: *mut cli_pic_t,
        csp: libc::c_int,
        width: libc::c_int,
        height: libc::c_int,
    ) -> libc::c_int;
    fn x264_cli_mmap_init(h: *mut cli_mmap_t, fh: *mut FILE) -> libc::c_int;
    fn x264_cli_pic_plane_size(
        csp: libc::c_int,
        width: libc::c_int,
        height: libc::c_int,
        plane: libc::c_int,
    ) -> int64_t;
    fn x264_cli_get_csp(csp: libc::c_int) -> *const x264_cli_csp_t;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
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
pub type uint16_t = __uint16_t;
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
    pub open_file: Option::<
        unsafe extern "C" fn(
            *mut libc::c_char,
            *mut hnd_t,
            *mut video_info_t,
            *mut cli_input_opt_t,
        ) -> libc::c_int,
    >,
    pub picture_alloc: Option::<
        unsafe extern "C" fn(
            *mut cli_pic_t,
            hnd_t,
            libc::c_int,
            libc::c_int,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub read_frame: Option::<
        unsafe extern "C" fn(*mut cli_pic_t, hnd_t, libc::c_int) -> libc::c_int,
    >,
    pub release_frame: Option::<
        unsafe extern "C" fn(*mut cli_pic_t, hnd_t) -> libc::c_int,
    >,
    pub picture_clean: Option::<unsafe extern "C" fn(*mut cli_pic_t, hnd_t) -> ()>,
    pub close_file: Option::<unsafe extern "C" fn(hnd_t) -> libc::c_int>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct y4m_hnd_t {
    pub fh: *mut FILE,
    pub next_frame: libc::c_int,
    pub seq_header_len: libc::c_int,
    pub frame_header_len: libc::c_int,
    pub frame_size: int64_t,
    pub plane_size: [int64_t; 3],
    pub bit_depth: libc::c_int,
    pub mmap: cli_mmap_t,
    pub use_mmap: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cli_mmap_t {
    pub file_size: int64_t,
    pub align_mask: libc::c_int,
    pub fd: libc::c_int,
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
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    if fstat(fileno(filehandle), &mut file_stat) != 0 {
        return 1 as libc::c_int;
    }
    return (file_stat.st_mode & 0o170000 as libc::c_int as __mode_t
        == 0o100000 as libc::c_int as __mode_t) as libc::c_int;
}
unsafe extern "C" fn parse_csp_and_depth(
    mut csp_name: *mut libc::c_char,
    mut bit_depth: *mut libc::c_int,
) -> libc::c_int {
    let mut csp: libc::c_int = 0x11 as libc::c_int;
    if strncmp(
        b"mono\0" as *const u8 as *const libc::c_char,
        csp_name,
        4 as libc::c_int as libc::c_ulong,
    ) == 0
    {
        csp = 0x1 as libc::c_int;
    } else if strncmp(
        b"420\0" as *const u8 as *const libc::c_char,
        csp_name,
        3 as libc::c_int as libc::c_ulong,
    ) == 0
    {
        csp = 0x2 as libc::c_int;
    } else if strncmp(
        b"422\0" as *const u8 as *const libc::c_char,
        csp_name,
        3 as libc::c_int as libc::c_ulong,
    ) == 0
    {
        csp = 0x6 as libc::c_int;
    } else if strncmp(
        b"444\0" as *const u8 as *const libc::c_char,
        csp_name,
        3 as libc::c_int as libc::c_ulong,
    ) == 0
        && strncmp(
            b"444alpha\0" as *const u8 as *const libc::c_char,
            csp_name,
            8 as libc::c_int as libc::c_ulong,
        ) != 0
    {
        csp = 0xc as libc::c_int;
    }
    if sscanf(csp_name, b"mono%d\0" as *const u8 as *const libc::c_char, bit_depth)
        != 1 as libc::c_int
        && sscanf(
            csp_name,
            b"%*d%*[pP]%d\0" as *const u8 as *const libc::c_char,
            bit_depth,
        ) != 1 as libc::c_int
    {
        *bit_depth = 8 as libc::c_int;
    }
    return csp;
}
unsafe extern "C" fn open_file(
    mut psz_filename: *mut libc::c_char,
    mut p_handle: *mut hnd_t,
    mut info: *mut video_info_t,
    mut opt: *mut cli_input_opt_t,
) -> libc::c_int {
    let mut h: *mut y4m_hnd_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<y4m_hnd_t>() as libc::c_ulong,
    ) as *mut y4m_hnd_t;
    let mut i: libc::c_int = 0;
    let mut n: uint32_t = 0;
    let mut d: uint32_t = 0;
    let mut header: [libc::c_char; 266] = [0; 266];
    let mut tokend: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut header_end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut colorspace: libc::c_int = 0 as libc::c_int;
    let mut alt_colorspace: libc::c_int = 0 as libc::c_int;
    let mut alt_bit_depth: libc::c_int = 8 as libc::c_int;
    if h.is_null() {
        return -(1 as libc::c_int);
    }
    (*info).vfr = 0 as libc::c_int;
    if strcmp(psz_filename, b"-\0" as *const u8 as *const libc::c_char) == 0 {
        (*h).fh = stdin;
    } else {
        (*h).fh = fopen(psz_filename, b"rb\0" as *const u8 as *const libc::c_char);
    }
    if ((*h).fh).is_null() {
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        header[i as usize] = fgetc((*h).fh) as libc::c_char;
        if header[i as usize] as libc::c_int == '\n' as i32 {
            header[(i + 1 as libc::c_int)
                as usize] = 0x20 as libc::c_int as libc::c_char;
            header[(i + 2 as libc::c_int) as usize] = 0 as libc::c_int as libc::c_char;
            break;
        } else {
            i += 1;
            i;
        }
    }
    if strncmp(
        header.as_mut_ptr(),
        b"YUV4MPEG2\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    ) != 0
    {
        x264_cli_log(
            b"y4m\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            b"bad sequence header magic\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if i == 256 as libc::c_int {
        x264_cli_log(
            b"y4m\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            b"bad sequence header length\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    header_end = &mut *header.as_mut_ptr().offset((i + 1 as libc::c_int) as isize)
        as *mut libc::c_char;
    (*h).seq_header_len = i + 1 as libc::c_int;
    let mut tokstart: *mut libc::c_char = header
        .as_mut_ptr()
        .offset(::core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong as isize);
    while tokstart < header_end {
        if !(*tokstart as libc::c_int == 0x20 as libc::c_int) {
            let fresh0 = tokstart;
            tokstart = tokstart.offset(1);
            match *fresh0 as libc::c_int {
                87 => {
                    (*info)
                        .width = strtol(tokstart, &mut tokend, 10 as libc::c_int)
                        as libc::c_int;
                    tokstart = tokend;
                }
                72 => {
                    (*info)
                        .height = strtol(tokstart, &mut tokend, 10 as libc::c_int)
                        as libc::c_int;
                    tokstart = tokend;
                }
                67 => {
                    colorspace = parse_csp_and_depth(tokstart, &mut (*h).bit_depth);
                    tokstart = strchr(tokstart, 0x20 as libc::c_int);
                }
                73 => {
                    let fresh1 = tokstart;
                    tokstart = tokstart.offset(1);
                    match *fresh1 as libc::c_int {
                        116 => {
                            (*info).interlaced = 1 as libc::c_int;
                            (*info).tff = 1 as libc::c_int;
                        }
                        98 => {
                            (*info).interlaced = 1 as libc::c_int;
                            (*info).tff = 0 as libc::c_int;
                        }
                        109 => {
                            (*info).interlaced = 1 as libc::c_int;
                        }
                        _ => {}
                    }
                }
                70 => {
                    if sscanf(
                        tokstart,
                        b"%u:%u\0" as *const u8 as *const libc::c_char,
                        &mut n as *mut uint32_t,
                        &mut d as *mut uint32_t,
                    ) == 2 as libc::c_int && n != 0 && d != 0
                    {
                        x264_reduce_fraction(&mut n, &mut d);
                        (*info).fps_num = n;
                        (*info).fps_den = d;
                    }
                    tokstart = strchr(tokstart, 0x20 as libc::c_int);
                }
                65 => {
                    if sscanf(
                        tokstart,
                        b"%u:%u\0" as *const u8 as *const libc::c_char,
                        &mut n as *mut uint32_t,
                        &mut d as *mut uint32_t,
                    ) == 2 as libc::c_int && n != 0 && d != 0
                    {
                        x264_reduce_fraction(&mut n, &mut d);
                        (*info).sar_width = n;
                        (*info).sar_height = d;
                    }
                    tokstart = strchr(tokstart, 0x20 as libc::c_int);
                }
                88 => {
                    if strncmp(
                        b"YSCSS=\0" as *const u8 as *const libc::c_char,
                        tokstart,
                        6 as libc::c_int as libc::c_ulong,
                    ) == 0
                    {
                        tokstart = tokstart.offset(6 as libc::c_int as isize);
                        alt_colorspace = parse_csp_and_depth(
                            tokstart,
                            &mut alt_bit_depth,
                        );
                    } else if strncmp(
                        b"COLORRANGE=\0" as *const u8 as *const libc::c_char,
                        tokstart,
                        11 as libc::c_int as libc::c_ulong,
                    ) == 0
                    {
                        tokstart = tokstart.offset(11 as libc::c_int as isize);
                        if strncmp(
                            b"FULL\0" as *const u8 as *const libc::c_char,
                            tokstart,
                            4 as libc::c_int as libc::c_ulong,
                        ) == 0
                        {
                            (*info).fullrange = 1 as libc::c_int;
                        } else if strncmp(
                            b"LIMITED\0" as *const u8 as *const libc::c_char,
                            tokstart,
                            7 as libc::c_int as libc::c_ulong,
                        ) == 0
                        {
                            (*info).fullrange = 0 as libc::c_int;
                        }
                    }
                    tokstart = strchr(tokstart, 0x20 as libc::c_int);
                }
                _ => {}
            }
        }
        tokstart = tokstart.offset(1);
        tokstart;
    }
    if colorspace == 0 as libc::c_int {
        colorspace = alt_colorspace;
        (*h).bit_depth = alt_bit_depth;
    }
    if colorspace == 0 as libc::c_int {
        colorspace = 0x2 as libc::c_int;
        (*h).bit_depth = 8 as libc::c_int;
    }
    if colorspace <= 0 as libc::c_int || colorspace >= 0x11 as libc::c_int {
        x264_cli_log(
            b"y4m\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            b"colorspace unhandled\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (*h).bit_depth < 8 as libc::c_int || (*h).bit_depth > 16 as libc::c_int {
        x264_cli_log(
            b"y4m\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            b"unsupported bit depth `%d'\n\0" as *const u8 as *const libc::c_char,
            (*h).bit_depth,
        );
        return -(1 as libc::c_int);
    }
    (*info).thread_safe = 1 as libc::c_int;
    (*info).num_frames = 0 as libc::c_int;
    (*info).csp = colorspace;
    if (*h).bit_depth > 8 as libc::c_int {
        (*info).csp |= 0x2000 as libc::c_int;
    }
    let mut csp: *const x264_cli_csp_t = x264_cli_get_csp((*info).csp);
    i = 0 as libc::c_int;
    while i < (*csp).planes {
        (*h)
            .plane_size[i
            as usize] = x264_cli_pic_plane_size(
            (*info).csp,
            (*info).width,
            (*info).height,
            i,
        );
        (*h).frame_size += (*h).plane_size[i as usize];
        (*h).plane_size[i as usize] /= x264_cli_csp_depth_factor((*info).csp) as int64_t;
        i += 1;
        i;
    }
    if x264_is_regular_file((*h).fh) != 0 {
        let mut init_pos: int64_t = ftello((*h).fh);
        let mut len: size_t = 1 as libc::c_int as size_t;
        while len <= 256 as libc::c_int as size_t && fgetc((*h).fh) != '\n' as i32 {
            len = len.wrapping_add(1);
            len;
        }
        if len > 256 as libc::c_int as size_t
            || len < ::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong
        {
            x264_cli_log(
                b"y4m\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
                b"bad frame header length\n\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        (*h).frame_header_len = len as libc::c_int;
        (*h)
            .frame_size = ((*h).frame_size as size_t).wrapping_add(len) as int64_t
            as int64_t;
        fseeko((*h).fh, 0 as libc::c_int as __off64_t, 2 as libc::c_int);
        let mut i_size: int64_t = ftello((*h).fh);
        fseeko((*h).fh, init_pos, 0 as libc::c_int);
        (*info)
            .num_frames = ((i_size - (*h).seq_header_len as int64_t) / (*h).frame_size)
            as libc::c_int;
        if (*info).num_frames == 0 {
            x264_cli_log(
                b"y4m\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
                b"empty input file\n\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        if (*h).bit_depth & 7 as libc::c_int == 0 {
            (*h)
                .use_mmap = (x264_cli_mmap_init(&mut (*h).mmap, (*h).fh) == 0)
                as libc::c_int;
        }
    }
    *p_handle = h as hnd_t;
    return 0 as libc::c_int;
}
static mut slen: size_t = 0;
unsafe extern "C" fn read_frame_internal(
    mut pic: *mut cli_pic_t,
    mut h: *mut y4m_hnd_t,
    mut bit_depth_uc: libc::c_int,
) -> libc::c_int {
    let mut pixel_depth: libc::c_int = x264_cli_csp_depth_factor((*pic).img.csp);
    let mut i: libc::c_int = ::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong
        as libc::c_int;
    let mut header_buf: [libc::c_char; 16] = [0; 16];
    let mut header: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*h).use_mmap != 0 {
        header = (*pic).img.plane[0 as libc::c_int as usize] as *mut libc::c_char;
        (*pic)
            .img
            .plane[0 as libc::c_int
            as usize] = ((*pic).img.plane[0 as libc::c_int as usize])
            .offset((*h).frame_header_len as isize);
        while i <= (*h).frame_header_len
            && *header.offset((i - 1 as libc::c_int) as isize) as libc::c_int
                != '\n' as i32
        {
            i += 1;
            i;
        }
        if i != (*h).frame_header_len {
            x264_cli_log(
                b"y4m\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
                b"bad frame header length\n\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    } else {
        header = header_buf.as_mut_ptr();
        if fread(
            header as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            slen,
            (*h).fh,
        ) != slen
        {
            return -(1 as libc::c_int);
        }
        while i <= 256 as libc::c_int && fgetc((*h).fh) != '\n' as i32 {
            i += 1;
            i;
        }
        if i > 256 as libc::c_int {
            x264_cli_log(
                b"y4m\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
                b"bad frame header length\n\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    }
    if memcmp(
        header as *const libc::c_void,
        b"FRAME\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        slen,
    ) != 0
    {
        x264_cli_log(
            b"y4m\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            b"bad frame header magic\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < (*pic).img.planes {
        if (*h).use_mmap != 0 {
            if i != 0 {
                (*pic)
                    .img
                    .plane[i
                    as usize] = ((*pic).img.plane[(i - 1 as libc::c_int) as usize])
                    .offset(
                        (pixel_depth as int64_t
                            * (*h).plane_size[(i - 1 as libc::c_int) as usize]) as isize,
                    );
            }
        } else if fread(
            (*pic).img.plane[i as usize] as *mut libc::c_void,
            pixel_depth as libc::c_ulong,
            (*h).plane_size[i as usize] as libc::c_ulong,
            (*h).fh,
        ) != (*h).plane_size[i as usize] as uint64_t
        {
            return -(1 as libc::c_int)
        }
        if bit_depth_uc != 0 {
            let mut plane: *mut uint16_t = (*pic).img.plane[i as usize] as *mut uint16_t;
            let mut pixel_count: int64_t = (*h).plane_size[i as usize];
            let mut lshift: libc::c_int = 16 as libc::c_int - (*h).bit_depth;
            let mut j: int64_t = 0 as libc::c_int as int64_t;
            while j < pixel_count {
                *plane
                    .offset(
                        j as isize,
                    ) = ((*plane.offset(j as isize) as libc::c_int) << lshift)
                    as uint16_t;
                j += 1;
                j;
            }
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn read_frame(
    mut pic: *mut cli_pic_t,
    mut handle: hnd_t,
    mut i_frame: libc::c_int,
) -> libc::c_int {
    let mut h: *mut y4m_hnd_t = handle as *mut y4m_hnd_t;
    if (*h).use_mmap != 0 {
        (*pic)
            .img
            .plane[0 as libc::c_int
            as usize] = x264_cli_mmap(
            &mut (*h).mmap,
            (*h).frame_size * i_frame as int64_t + (*h).seq_header_len as int64_t,
            (*h).frame_size,
        ) as *mut uint8_t;
        if ((*pic).img.plane[0 as libc::c_int as usize]).is_null() {
            return -(1 as libc::c_int);
        }
    } else if i_frame > (*h).next_frame {
        if x264_is_regular_file((*h).fh) != 0 {
            fseeko(
                (*h).fh,
                (*h).frame_size * i_frame as int64_t + (*h).seq_header_len as int64_t,
                0 as libc::c_int,
            );
        } else {
            while i_frame > (*h).next_frame {
                if read_frame_internal(pic, h, 0 as libc::c_int) != 0 {
                    return -(1 as libc::c_int);
                }
                (*h).next_frame += 1;
                (*h).next_frame;
            }
        }
    }
    if read_frame_internal(pic, h, (*h).bit_depth & 7 as libc::c_int) != 0 {
        return -(1 as libc::c_int);
    }
    (*h).next_frame = i_frame + 1 as libc::c_int;
    return 0 as libc::c_int;
}
unsafe extern "C" fn release_frame(
    mut pic: *mut cli_pic_t,
    mut handle: hnd_t,
) -> libc::c_int {
    let mut h: *mut y4m_hnd_t = handle as *mut y4m_hnd_t;
    if (*h).use_mmap != 0 {
        return x264_cli_munmap(
            &mut (*h).mmap,
            ((*pic).img.plane[0 as libc::c_int as usize])
                .offset(-((*h).frame_header_len as isize)) as *mut libc::c_void,
            (*h).frame_size,
        );
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn picture_alloc(
    mut pic: *mut cli_pic_t,
    mut handle: hnd_t,
    mut csp: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> libc::c_int {
    let mut h: *mut y4m_hnd_t = handle as *mut y4m_hnd_t;
    return if (*h).use_mmap != 0 {
        Some(
            x264_cli_pic_init_noalloc
                as unsafe extern "C" fn(
                    *mut cli_pic_t,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                ) -> libc::c_int,
        )
    } else {
        Some(
            x264_cli_pic_alloc
                as unsafe extern "C" fn(
                    *mut cli_pic_t,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                ) -> libc::c_int,
        )
    }
        .expect("non-null function pointer")(pic, csp, width, height);
}
unsafe extern "C" fn picture_clean(mut pic: *mut cli_pic_t, mut handle: hnd_t) {
    let mut h: *mut y4m_hnd_t = handle as *mut y4m_hnd_t;
    if (*h).use_mmap != 0 {
        memset(
            pic as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<cli_pic_t>() as libc::c_ulong,
        );
    } else {
        x264_cli_pic_clean(pic);
    };
}
unsafe extern "C" fn close_file(mut handle: hnd_t) -> libc::c_int {
    let mut h: *mut y4m_hnd_t = handle as *mut y4m_hnd_t;
    if h.is_null() || ((*h).fh).is_null() {
        return 0 as libc::c_int;
    }
    if (*h).use_mmap != 0 {
        x264_cli_mmap_close(&mut (*h).mmap);
    }
    fclose((*h).fh);
    free(h as *mut libc::c_void);
    return 0 as libc::c_int;
}
#[no_mangle]
pub static mut y4m_input: cli_input_t = unsafe {
    {
        let mut init = cli_input_t {
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
                    as unsafe extern "C" fn(
                        *mut cli_pic_t,
                        hnd_t,
                        libc::c_int,
                    ) -> libc::c_int,
            ),
            release_frame: Some(
                release_frame
                    as unsafe extern "C" fn(*mut cli_pic_t, hnd_t) -> libc::c_int,
            ),
            picture_clean: Some(
                picture_clean as unsafe extern "C" fn(*mut cli_pic_t, hnd_t) -> (),
            ),
            close_file: Some(close_file as unsafe extern "C" fn(hnd_t) -> libc::c_int),
        };
        init
    }
};
unsafe extern "C" fn run_static_initializers() {
    slen = (::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
