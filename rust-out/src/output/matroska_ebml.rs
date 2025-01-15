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
use crate::types::matroska_ebml_file::c2_defs::C2RustUnnamed;
use crate::types::*;

extern "C" {
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    static mut stdout: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fseeko(__stream: *mut FILE, __off: __off64_t, __whence: libc::c_int) -> libc::c_int;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
unsafe extern "C" fn mk_create_context(
    mut w: *mut mk_writer,
    mut parent: *mut mk_context,
    mut id: libc::c_uint,
) -> *mut mk_context {
    let mut c: *mut mk_context = std::ptr::null_mut::<mk_context>();
    if !((*w).freelist).is_null() {
        c = (*w).freelist;
        (*w).freelist = (*(*w).freelist).next;
    } else {
        c = calloc(
            1 as libc::c_int as libc::c_ulong,
            ::core::mem::size_of::<mk_context>() as libc::c_ulong,
        ) as *mut mk_context;
        if c.is_null() {
            return std::ptr::null_mut::<mk_context>();
        }
    }
    (*c).parent = parent;
    (*c).owner = w;
    (*c).id = id;
    if !((*(*c).owner).actlist).is_null() {
        (*(*(*c).owner).actlist).prev = &mut (*c).next;
    }
    (*c).next = (*(*c).owner).actlist;
    (*c).prev = &mut (*(*c).owner).actlist;
    (*(*c).owner).actlist = c;
    c
}
unsafe extern "C" fn mk_append_context_data(
    mut c: *mut mk_context,
    mut data: *const libc::c_void,
    mut size: libc::c_uint,
) -> libc::c_int {
    let mut ns: libc::c_uint = ((*c).d_cur).wrapping_add(size);
    if ns > (*c).d_max {
        let mut dp: *mut libc::c_void = std::ptr::null_mut::<libc::c_void>();
        let mut dn: libc::c_uint = if (*c).d_max != 0 {
            (*c).d_max << 1 as libc::c_int
        } else {
            16 as libc::c_int as libc::c_uint
        };
        while ns > dn {
            dn <<= 1 as libc::c_int;
        }
        dp = realloc((*c).data, dn as libc::c_ulong);
        if dp.is_null() {
            return -(1 as libc::c_int);
        }
        (*c).data = dp;
        (*c).d_max = dn;
    }
    memcpy(
        ((*c).data as *mut uint8_t).offset((*c).d_cur as isize) as *mut libc::c_void,
        data,
        size as libc::c_ulong,
    );
    (*c).d_cur = ns;
    0 as libc::c_int
}
unsafe extern "C" fn mk_write_id(mut c: *mut mk_context, mut id: libc::c_uint) -> libc::c_int {
    let mut c_id: [uint8_t; 4] = [
        (id >> 24 as libc::c_int) as uint8_t,
        (id >> 16 as libc::c_int) as uint8_t,
        (id >> 8 as libc::c_int) as uint8_t,
        id as uint8_t,
    ];
    if c_id[0 as libc::c_int as usize] != 0 {
        return mk_append_context_data(
            c,
            c_id.as_mut_ptr() as *const libc::c_void,
            4 as libc::c_int as libc::c_uint,
        );
    }
    if c_id[1 as libc::c_int as usize] != 0 {
        return mk_append_context_data(
            c,
            c_id.as_mut_ptr().offset(1 as libc::c_int as isize) as *const libc::c_void,
            3 as libc::c_int as libc::c_uint,
        );
    }
    if c_id[2 as libc::c_int as usize] != 0 {
        return mk_append_context_data(
            c,
            c_id.as_mut_ptr().offset(2 as libc::c_int as isize) as *const libc::c_void,
            2 as libc::c_int as libc::c_uint,
        );
    }
    mk_append_context_data(
        c,
        c_id.as_mut_ptr().offset(3 as libc::c_int as isize) as *const libc::c_void,
        1 as libc::c_int as libc::c_uint,
    )
}
unsafe extern "C" fn mk_write_size(mut c: *mut mk_context, mut size: libc::c_uint) -> libc::c_int {
    let mut c_size: [uint8_t; 5] = [
        0x8 as libc::c_int as uint8_t,
        (size >> 24 as libc::c_int) as uint8_t,
        (size >> 16 as libc::c_int) as uint8_t,
        (size >> 8 as libc::c_int) as uint8_t,
        size as uint8_t,
    ];
    if size < 0x7f as libc::c_int as libc::c_uint {
        c_size[4 as libc::c_int as usize] =
            (c_size[4 as libc::c_int as usize] as libc::c_int | 0x80 as libc::c_int) as uint8_t;
        return mk_append_context_data(
            c,
            c_size.as_mut_ptr().offset(4 as libc::c_int as isize) as *const libc::c_void,
            1 as libc::c_int as libc::c_uint,
        );
    }
    if size < 0x3fff as libc::c_int as libc::c_uint {
        c_size[3 as libc::c_int as usize] =
            (c_size[3 as libc::c_int as usize] as libc::c_int | 0x40 as libc::c_int) as uint8_t;
        return mk_append_context_data(
            c,
            c_size.as_mut_ptr().offset(3 as libc::c_int as isize) as *const libc::c_void,
            2 as libc::c_int as libc::c_uint,
        );
    }
    if size < 0x1fffff as libc::c_int as libc::c_uint {
        c_size[2 as libc::c_int as usize] =
            (c_size[2 as libc::c_int as usize] as libc::c_int | 0x20 as libc::c_int) as uint8_t;
        return mk_append_context_data(
            c,
            c_size.as_mut_ptr().offset(2 as libc::c_int as isize) as *const libc::c_void,
            3 as libc::c_int as libc::c_uint,
        );
    }
    if size < 0xfffffff as libc::c_int as libc::c_uint {
        c_size[1 as libc::c_int as usize] =
            (c_size[1 as libc::c_int as usize] as libc::c_int | 0x10 as libc::c_int) as uint8_t;
        return mk_append_context_data(
            c,
            c_size.as_mut_ptr().offset(1 as libc::c_int as isize) as *const libc::c_void,
            4 as libc::c_int as libc::c_uint,
        );
    }
    mk_append_context_data(
        c,
        c_size.as_mut_ptr() as *const libc::c_void,
        5 as libc::c_int as libc::c_uint,
    )
}
unsafe extern "C" fn mk_flush_context_id(mut c: *mut mk_context) -> libc::c_int {
    let mut ff: uint8_t = 0xff as libc::c_int as uint8_t;
    if (*c).id == 0 {
        return 0 as libc::c_int;
    }
    if mk_write_id((*c).parent, (*c).id) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if mk_append_context_data(
        (*c).parent,
        &mut ff as *mut uint8_t as *const libc::c_void,
        1 as libc::c_int as libc::c_uint,
    ) < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    (*c).id = 0 as libc::c_int as libc::c_uint;
    0 as libc::c_int
}
unsafe extern "C" fn mk_flush_context_data(mut c: *mut mk_context) -> libc::c_int {
    if (*c).d_cur == 0 {
        return 0 as libc::c_int;
    }
    if !((*c).parent).is_null() {
        if mk_append_context_data((*c).parent, (*c).data, (*c).d_cur) < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    } else if fwrite(
        (*c).data,
        (*c).d_cur as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        (*(*c).owner).fp,
    ) != 1 as libc::c_int as libc::c_ulong
    {
        return -(1 as libc::c_int);
    }
    (*c).d_cur = 0 as libc::c_int as libc::c_uint;
    0 as libc::c_int
}
unsafe extern "C" fn mk_close_context(
    mut c: *mut mk_context,
    mut off: *mut libc::c_uint,
) -> libc::c_int {
    if (*c).id != 0 {
        if mk_write_id((*c).parent, (*c).id) < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        if mk_write_size((*c).parent, (*c).d_cur) < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    if !((*c).parent).is_null() && !off.is_null() {
        *off = (*off).wrapping_add((*(*c).parent).d_cur);
    }
    if mk_flush_context_data(c) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if !((*c).next).is_null() {
        (*(*c).next).prev = (*c).prev;
    }
    *(*c).prev = (*c).next;
    (*c).next = (*(*c).owner).freelist;
    (*(*c).owner).freelist = c;
    0 as libc::c_int
}
unsafe extern "C" fn mk_destroy_contexts(mut w: *mut mk_writer) {
    let mut next: *mut mk_context = std::ptr::null_mut::<mk_context>();
    let mut cur: *mut mk_context = (*w).freelist;
    while !cur.is_null() {
        next = (*cur).next;
        free((*cur).data);
        free(cur as *mut libc::c_void);
        cur = next;
    }
    let mut cur_0: *mut mk_context = (*w).actlist;
    while !cur_0.is_null() {
        next = (*cur_0).next;
        free((*cur_0).data);
        free(cur_0 as *mut libc::c_void);
        cur_0 = next;
    }
    (*w).root = std::ptr::null_mut::<mk_context>();
    (*w).actlist = (*w).root;
    (*w).freelist = (*w).actlist;
}
unsafe extern "C" fn mk_write_string(
    mut c: *mut mk_context,
    mut id: libc::c_uint,
    mut str: *const libc::c_char,
) -> libc::c_int {
    let mut len: size_t = strlen(str);
    if mk_write_id(c, id) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if mk_write_size(c, len as libc::c_uint) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if mk_append_context_data(c, str as *const libc::c_void, len as libc::c_uint) < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    0 as libc::c_int
}
unsafe extern "C" fn mk_write_bin(
    mut c: *mut mk_context,
    mut id: libc::c_uint,
    mut data: *const libc::c_void,
    mut size: libc::c_uint,
) -> libc::c_int {
    if mk_write_id(c, id) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if mk_write_size(c, size) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if mk_append_context_data(c, data, size) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    0 as libc::c_int
}
unsafe extern "C" fn mk_write_uint(
    mut c: *mut mk_context,
    mut id: libc::c_uint,
    mut ui: uint64_t,
) -> libc::c_int {
    let mut c_ui: [uint8_t; 8] = [
        (ui >> 56 as libc::c_int) as uint8_t,
        (ui >> 48 as libc::c_int) as uint8_t,
        (ui >> 40 as libc::c_int) as uint8_t,
        (ui >> 32 as libc::c_int) as uint8_t,
        (ui >> 24 as libc::c_int) as uint8_t,
        (ui >> 16 as libc::c_int) as uint8_t,
        (ui >> 8 as libc::c_int) as uint8_t,
        ui as uint8_t,
    ];
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    if mk_write_id(c, id) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    while i < 7 as libc::c_int as libc::c_uint && c_ui[i as usize] == 0 {
        i = i.wrapping_add(1);
        i;
    }
    if mk_write_size(c, (8 as libc::c_int as libc::c_uint).wrapping_sub(i)) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if mk_append_context_data(
        c,
        c_ui.as_mut_ptr().offset(i as isize) as *const libc::c_void,
        (8 as libc::c_int as libc::c_uint).wrapping_sub(i),
    ) < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    0 as libc::c_int
}
unsafe extern "C" fn mk_write_float_raw(
    mut c: *mut mk_context,
    mut f: libc::c_float,
) -> libc::c_int {
    let mut u: C2RustUnnamed = C2RustUnnamed { f: 0. };
    let mut c_f: [uint8_t; 4] = [0; 4];
    u.f = f;
    c_f[0 as libc::c_int as usize] = (u.u >> 24 as libc::c_int) as uint8_t;
    c_f[1 as libc::c_int as usize] = (u.u >> 16 as libc::c_int) as uint8_t;
    c_f[2 as libc::c_int as usize] = (u.u >> 8 as libc::c_int) as uint8_t;
    c_f[3 as libc::c_int as usize] = u.u as uint8_t;
    mk_append_context_data(
        c,
        c_f.as_mut_ptr() as *const libc::c_void,
        4 as libc::c_int as libc::c_uint,
    )
}
unsafe extern "C" fn mk_write_float(
    mut c: *mut mk_context,
    mut id: libc::c_uint,
    mut f: libc::c_float,
) -> libc::c_int {
    if mk_write_id(c, id) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if mk_write_size(c, 4 as libc::c_int as libc::c_uint) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if mk_write_float_raw(c, f) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    0 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn mk_create_writer(mut filename: *const libc::c_char) -> *mut mk_writer {
    let mut w: *mut mk_writer = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<mk_writer>() as libc::c_ulong,
    ) as *mut mk_writer;
    if w.is_null() {
        return std::ptr::null_mut::<mk_writer>();
    }
    (*w).root = mk_create_context(
        w,
        std::ptr::null_mut::<mk_context>(),
        0 as libc::c_int as libc::c_uint,
    );
    if ((*w).root).is_null() {
        free(w as *mut libc::c_void);
        return std::ptr::null_mut::<mk_writer>();
    }
    if strcmp(filename, b"-\0" as *const u8 as *const libc::c_char) == 0 {
        (*w).fp = stdout;
    } else {
        (*w).fp = fopen(filename, b"wb\0" as *const u8 as *const libc::c_char);
    }
    if ((*w).fp).is_null() {
        mk_destroy_contexts(w);
        free(w as *mut libc::c_void);
        return std::ptr::null_mut::<mk_writer>();
    }
    (*w).timescale = 1000000 as libc::c_int as int64_t;
    w
}
#[no_mangle]
pub unsafe extern "C" fn mk_write_header(
    mut w: *mut mk_writer,
    mut writing_app: *const libc::c_char,
    mut codec_id: *const libc::c_char,
    mut codec_private: *const libc::c_void,
    mut codec_private_size: libc::c_uint,
    mut default_frame_duration: int64_t,
    mut timescale: int64_t,
    mut width: libc::c_uint,
    mut height: libc::c_uint,
    mut d_width: libc::c_uint,
    mut d_height: libc::c_uint,
    mut display_size_units: libc::c_int,
    mut stereo_mode: libc::c_int,
) -> libc::c_int {
    let mut c: *mut mk_context = std::ptr::null_mut::<mk_context>();
    let mut ti: *mut mk_context = std::ptr::null_mut::<mk_context>();
    let mut v: *mut mk_context = std::ptr::null_mut::<mk_context>();
    if (*w).wrote_header != 0 {
        return -(1 as libc::c_int);
    }
    (*w).timescale = timescale;
    (*w).def_duration = default_frame_duration;
    c = mk_create_context(w, (*w).root, 0x1a45dfa3 as libc::c_int as libc::c_uint);
    if c.is_null() {
        return -(1 as libc::c_int);
    }
    if mk_write_uint(
        c,
        0x4286 as libc::c_int as libc::c_uint,
        1 as libc::c_int as uint64_t,
    ) < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    if mk_write_uint(
        c,
        0x42f7 as libc::c_int as libc::c_uint,
        1 as libc::c_int as uint64_t,
    ) < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    if mk_write_uint(
        c,
        0x42f2 as libc::c_int as libc::c_uint,
        4 as libc::c_int as uint64_t,
    ) < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    if mk_write_uint(
        c,
        0x42f3 as libc::c_int as libc::c_uint,
        8 as libc::c_int as uint64_t,
    ) < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    if mk_write_string(
        c,
        0x4282 as libc::c_int as libc::c_uint,
        b"matroska\0" as *const u8 as *const libc::c_char,
    ) < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    if mk_write_uint(
        c,
        0x4287 as libc::c_int as libc::c_uint,
        (if stereo_mode >= 0 as libc::c_int {
            3 as libc::c_int
        } else {
            2 as libc::c_int
        }) as uint64_t,
    ) < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    if mk_write_uint(
        c,
        0x4285 as libc::c_int as libc::c_uint,
        2 as libc::c_int as uint64_t,
    ) < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    if mk_close_context(c, std::ptr::null_mut::<libc::c_uint>()) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    c = mk_create_context(w, (*w).root, 0x18538067 as libc::c_int as libc::c_uint);
    if c.is_null() {
        return -(1 as libc::c_int);
    }
    if mk_flush_context_id(c) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if mk_close_context(c, std::ptr::null_mut::<libc::c_uint>()) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    c = mk_create_context(w, (*w).root, 0x1549a966 as libc::c_int as libc::c_uint);
    if c.is_null() {
        return -(1 as libc::c_int);
    }
    if mk_write_string(
        c,
        0x4d80 as libc::c_int as libc::c_uint,
        b"Haali Matroska Writer b0\0" as *const u8 as *const libc::c_char,
    ) < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    if mk_write_string(c, 0x5741 as libc::c_int as libc::c_uint, writing_app) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if mk_write_uint(
        c,
        0x2ad7b1 as libc::c_int as libc::c_uint,
        (*w).timescale as uint64_t,
    ) < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    if mk_write_float(
        c,
        0x4489 as libc::c_int as libc::c_uint,
        0 as libc::c_int as libc::c_float,
    ) < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    (*w).duration_ptr = ((*c).d_cur).wrapping_sub(4 as libc::c_int as libc::c_uint);
    if mk_close_context(c, &mut (*w).duration_ptr) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    c = mk_create_context(w, (*w).root, 0x1654ae6b as libc::c_int as libc::c_uint);
    if c.is_null() {
        return -(1 as libc::c_int);
    }
    ti = mk_create_context(w, c, 0xae as libc::c_int as libc::c_uint);
    if ti.is_null() {
        return -(1 as libc::c_int);
    }
    if mk_write_uint(
        ti,
        0xd7 as libc::c_int as libc::c_uint,
        1 as libc::c_int as uint64_t,
    ) < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    if mk_write_uint(
        ti,
        0x73c5 as libc::c_int as libc::c_uint,
        1 as libc::c_int as uint64_t,
    ) < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    if mk_write_uint(
        ti,
        0x83 as libc::c_int as libc::c_uint,
        1 as libc::c_int as uint64_t,
    ) < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    if mk_write_uint(
        ti,
        0x9c as libc::c_int as libc::c_uint,
        0 as libc::c_int as uint64_t,
    ) < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    if mk_write_string(ti, 0x86 as libc::c_int as libc::c_uint, codec_id) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if codec_private_size != 0
        && mk_write_bin(
            ti,
            0x63a2 as libc::c_int as libc::c_uint,
            codec_private,
            codec_private_size,
        ) < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    if default_frame_duration != 0
        && mk_write_uint(
            ti,
            0x23e383 as libc::c_int as libc::c_uint,
            default_frame_duration as uint64_t,
        ) < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    v = mk_create_context(w, ti, 0xe0 as libc::c_int as libc::c_uint);
    if v.is_null() {
        return -(1 as libc::c_int);
    }
    if mk_write_uint(v, 0xb0 as libc::c_int as libc::c_uint, width as uint64_t) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if mk_write_uint(v, 0xba as libc::c_int as libc::c_uint, height as uint64_t) < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    if mk_write_uint(
        v,
        0x54b2 as libc::c_int as libc::c_uint,
        display_size_units as uint64_t,
    ) < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    if mk_write_uint(
        v,
        0x54b0 as libc::c_int as libc::c_uint,
        d_width as uint64_t,
    ) < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    if mk_write_uint(
        v,
        0x54ba as libc::c_int as libc::c_uint,
        d_height as uint64_t,
    ) < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    if stereo_mode >= 0 as libc::c_int
        && mk_write_uint(
            v,
            0x53b8 as libc::c_int as libc::c_uint,
            stereo_mode as uint64_t,
        ) < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    if mk_close_context(v, std::ptr::null_mut::<libc::c_uint>()) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if mk_close_context(ti, std::ptr::null_mut::<libc::c_uint>()) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if mk_close_context(c, std::ptr::null_mut::<libc::c_uint>()) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if mk_flush_context_data((*w).root) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    (*w).wrote_header = 1 as libc::c_int as int8_t;
    0 as libc::c_int
}
unsafe extern "C" fn mk_close_cluster(mut w: *mut mk_writer) -> libc::c_int {
    if ((*w).cluster).is_null() {
        return 0 as libc::c_int;
    }
    if mk_close_context((*w).cluster, std::ptr::null_mut::<libc::c_uint>()) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    (*w).cluster = std::ptr::null_mut::<mk_context>();
    if mk_flush_context_data((*w).root) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    0 as libc::c_int
}
unsafe extern "C" fn mk_flush_frame(mut w: *mut mk_writer) -> libc::c_int {
    let mut delta: int64_t = 0;
    let mut fsize: libc::c_uint = 0;
    let mut c_delta_flags: [uint8_t; 3] = [0; 3];
    if (*w).in_frame == 0 {
        return 0 as libc::c_int;
    }
    delta = (*w).frame_tc / (*w).timescale - (*w).cluster_tc_scaled;
    if (delta as libc::c_longlong > 32767 as libc::c_longlong
        || (delta as libc::c_longlong) < -(32768 as libc::c_longlong))
        && mk_close_cluster(w) < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    if ((*w).cluster).is_null() {
        (*w).cluster_tc_scaled = (*w).frame_tc / (*w).timescale;
        (*w).cluster = mk_create_context(w, (*w).root, 0x1f43b675 as libc::c_int as libc::c_uint);
        if ((*w).cluster).is_null() {
            return -(1 as libc::c_int);
        }
        if mk_write_uint(
            (*w).cluster,
            0xe7 as libc::c_int as libc::c_uint,
            (*w).cluster_tc_scaled as uint64_t,
        ) < 0 as libc::c_int
        {
            return -(1 as libc::c_int);
        }
        delta = 0 as libc::c_int as int64_t;
    }
    fsize = if !((*w).frame).is_null() {
        (*(*w).frame).d_cur
    } else {
        0 as libc::c_int as libc::c_uint
    };
    if mk_write_id((*w).cluster, 0xa3 as libc::c_int as libc::c_uint) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if mk_write_size(
        (*w).cluster,
        fsize.wrapping_add(4 as libc::c_int as libc::c_uint),
    ) < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    if mk_write_size((*w).cluster, 1 as libc::c_int as libc::c_uint) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    c_delta_flags[0 as libc::c_int as usize] = (delta >> 8 as libc::c_int) as uint8_t;
    c_delta_flags[1 as libc::c_int as usize] = delta as uint8_t;
    c_delta_flags[2 as libc::c_int as usize] = ((((*w).keyframe as libc::c_int)
        << 7 as libc::c_int)
        | (*w).skippable as libc::c_int) as uint8_t;
    if mk_append_context_data(
        (*w).cluster,
        c_delta_flags.as_mut_ptr() as *const libc::c_void,
        3 as libc::c_int as libc::c_uint,
    ) < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    if !((*w).frame).is_null() {
        if mk_append_context_data((*w).cluster, (*(*w).frame).data, (*(*w).frame).d_cur)
            < 0 as libc::c_int
        {
            return -(1 as libc::c_int);
        }
        (*(*w).frame).d_cur = 0 as libc::c_int as libc::c_uint;
    }
    (*w).in_frame = 0 as libc::c_int as int8_t;
    if (*(*w).cluster).d_cur > 1048576 as libc::c_int as libc::c_uint
        && mk_close_cluster(w) < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    0 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn mk_start_frame(mut w: *mut mk_writer) -> libc::c_int {
    if mk_flush_frame(w) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    (*w).in_frame = 1 as libc::c_int as int8_t;
    (*w).keyframe = 0 as libc::c_int as int8_t;
    (*w).skippable = 0 as libc::c_int as int8_t;
    0 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn mk_set_frame_flags(
    mut w: *mut mk_writer,
    mut timestamp: int64_t,
    mut keyframe: libc::c_int,
    mut skippable: libc::c_int,
) -> libc::c_int {
    if (*w).in_frame == 0 {
        return -(1 as libc::c_int);
    }
    (*w).frame_tc = timestamp;
    (*w).keyframe = (keyframe != 0 as libc::c_int) as libc::c_int as int8_t;
    (*w).skippable = (skippable != 0 as libc::c_int) as libc::c_int as int8_t;
    if (*w).max_frame_tc < timestamp {
        (*w).max_frame_tc = timestamp;
    }
    0 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn mk_add_frame_data(
    mut w: *mut mk_writer,
    mut data: *const libc::c_void,
    mut size: libc::c_uint,
) -> libc::c_int {
    if (*w).in_frame == 0 {
        return -(1 as libc::c_int);
    }
    if ((*w).frame).is_null() {
        (*w).frame = mk_create_context(
            w,
            std::ptr::null_mut::<mk_context>(),
            0 as libc::c_int as libc::c_uint,
        );
        if ((*w).frame).is_null() {
            return -(1 as libc::c_int);
        }
    }
    mk_append_context_data((*w).frame, data, size)
}
#[no_mangle]
pub unsafe extern "C" fn mk_close(mut w: *mut mk_writer, mut last_delta: int64_t) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    if mk_flush_frame(w) < 0 as libc::c_int || mk_close_cluster(w) < 0 as libc::c_int {
        ret = -(1 as libc::c_int);
    }
    if (*w).wrote_header as libc::c_int != 0 && x264_is_regular_file((*w).fp) != 0 {
        let mut last_frametime: int64_t = if (*w).def_duration != 0 {
            (*w).def_duration
        } else {
            last_delta
        };
        let mut total_duration: int64_t = (*w).max_frame_tc + last_frametime;
        if fseeko((*w).fp, (*w).duration_ptr as __off64_t, 0 as libc::c_int) != 0
            || mk_write_float_raw(
                (*w).root,
                (total_duration as libc::c_double / (*w).timescale as libc::c_double)
                    as libc::c_float,
            ) < 0 as libc::c_int
            || mk_flush_context_data((*w).root) < 0 as libc::c_int
        {
            ret = -(1 as libc::c_int);
        }
    }
    mk_destroy_contexts(w);
    fclose((*w).fp);
    free(w as *mut libc::c_void);
    ret
}
