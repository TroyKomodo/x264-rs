#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    static mut stdout: *mut FILE;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type C2RustUnnamed = libc::c_uint;
pub const AMF_DATA_TYPE_UNSUPPORTED: C2RustUnnamed = 13;
pub const AMF_DATA_TYPE_LONG_STRING: C2RustUnnamed = 12;
pub const AMF_DATA_TYPE_DATE: C2RustUnnamed = 11;
pub const AMF_DATA_TYPE_ARRAY: C2RustUnnamed = 10;
pub const AMF_DATA_TYPE_OBJECT_END: C2RustUnnamed = 9;
pub const AMF_DATA_TYPE_MIXEDARRAY: C2RustUnnamed = 8;
pub const AMF_DATA_TYPE_REFERENCE: C2RustUnnamed = 7;
pub const AMF_DATA_TYPE_UNDEFINED: C2RustUnnamed = 6;
pub const AMF_DATA_TYPE_NULL: C2RustUnnamed = 5;
pub const AMF_DATA_TYPE_OBJECT: C2RustUnnamed = 3;
pub const AMF_DATA_TYPE_STRING: C2RustUnnamed = 2;
pub const AMF_DATA_TYPE_BOOL: C2RustUnnamed = 1;
pub const AMF_DATA_TYPE_NUMBER: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct flv_buffer {
    pub data: *mut uint8_t,
    pub d_cur: libc::c_uint,
    pub d_max: libc::c_uint,
    pub fp: *mut FILE,
    pub d_total: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub f: libc::c_double,
    pub i: uint64_t,
}
#[no_mangle]
pub unsafe extern "C" fn flv_dbl2int(mut value: libc::c_double) -> uint64_t {
    C2RustUnnamed_0 { f: value }.i
}
#[no_mangle]
pub unsafe extern "C" fn flv_put_byte(mut c: *mut flv_buffer, mut b: uint8_t) {
    flv_append_data(c, &mut b, 1 as libc::c_int as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn flv_put_be32(mut c: *mut flv_buffer, mut val: uint32_t) {
    flv_put_byte(c, (val >> 24 as libc::c_int) as uint8_t);
    flv_put_byte(c, (val >> 16 as libc::c_int) as uint8_t);
    flv_put_byte(c, (val >> 8 as libc::c_int) as uint8_t);
    flv_put_byte(c, val as uint8_t);
}
#[no_mangle]
pub unsafe extern "C" fn flv_put_be64(mut c: *mut flv_buffer, mut val: uint64_t) {
    flv_put_be32(c, (val >> 32 as libc::c_int) as uint32_t);
    flv_put_be32(c, val as uint32_t);
}
#[no_mangle]
pub unsafe extern "C" fn flv_put_be16(mut c: *mut flv_buffer, mut val: uint16_t) {
    flv_put_byte(c, (val as libc::c_int >> 8 as libc::c_int) as uint8_t);
    flv_put_byte(c, val as uint8_t);
}
#[no_mangle]
pub unsafe extern "C" fn flv_put_be24(mut c: *mut flv_buffer, mut val: uint32_t) {
    flv_put_be16(c, (val >> 8 as libc::c_int) as uint16_t);
    flv_put_byte(c, val as uint8_t);
}
#[no_mangle]
pub unsafe extern "C" fn flv_put_tag(
    mut c: *mut flv_buffer,
    mut tag: *const libc::c_char,
) {
    while *tag != 0 {
        let fresh0 = tag;
        tag = tag.offset(1);
        flv_put_byte(c, *fresh0 as uint8_t);
    }
}
#[no_mangle]
pub unsafe extern "C" fn flv_put_amf_string(
    mut c: *mut flv_buffer,
    mut str: *const libc::c_char,
) {
    let mut len: uint16_t = strlen(str) as uint16_t;
    flv_put_be16(c, len);
    flv_append_data(c, str as *mut uint8_t, len as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn flv_put_amf_double(
    mut c: *mut flv_buffer,
    mut d: libc::c_double,
) {
    flv_put_byte(c, AMF_DATA_TYPE_NUMBER as libc::c_int as uint8_t);
    flv_put_be64(c, flv_dbl2int(d));
}
#[no_mangle]
pub unsafe extern "C" fn flv_create_writer(
    mut filename: *const libc::c_char,
) -> *mut flv_buffer {
    let mut c: *mut flv_buffer = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<flv_buffer>() as libc::c_ulong,
    ) as *mut flv_buffer;
    if c.is_null() {
        return std::ptr::null_mut::<flv_buffer>();
    }
    if strcmp(filename, b"-\0" as *const u8 as *const libc::c_char) == 0 {
        (*c).fp = stdout;
    } else {
        (*c).fp = fopen(filename, b"wb\0" as *const u8 as *const libc::c_char);
    }
    if ((*c).fp).is_null() {
        free(c as *mut libc::c_void);
        return std::ptr::null_mut::<flv_buffer>();
    }
    c
}
#[no_mangle]
pub unsafe extern "C" fn flv_append_data(
    mut c: *mut flv_buffer,
    mut data: *mut uint8_t,
    mut size: libc::c_uint,
) -> libc::c_int {
    let mut ns: libc::c_uint = ((*c).d_cur).wrapping_add(size);
    if ns > (*c).d_max {
        let mut dp: *mut libc::c_void = std::ptr::null_mut::<libc::c_void>();
        let mut dn: libc::c_uint = 16 as libc::c_int as libc::c_uint;
        while ns > dn {
            dn <<= 1 as libc::c_int;
        }
        dp = realloc((*c).data as *mut libc::c_void, dn as libc::c_ulong);
        if dp.is_null() {
            return -(1 as libc::c_int);
        }
        (*c).data = dp as *mut uint8_t;
        (*c).d_max = dn;
    }
    memcpy(
        ((*c).data).offset((*c).d_cur as isize) as *mut libc::c_void,
        data as *const libc::c_void,
        size as libc::c_ulong,
    );
    (*c).d_cur = ns;
    0 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn flv_rewrite_amf_be24(
    mut c: *mut flv_buffer,
    mut length: libc::c_uint,
    mut start: libc::c_uint,
) {
    *((*c).data)
        .offset(start as isize)
        .offset(0 as libc::c_int as isize) = (length >> 16 as libc::c_int) as uint8_t;
    *((*c).data)
        .offset(start as isize)
        .offset(1 as libc::c_int as isize) = (length >> 8 as libc::c_int) as uint8_t;
    *((*c).data)
        .offset(start as isize)
        .offset(2 as libc::c_int as isize) = (length >> 0 as libc::c_int) as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn flv_flush_data(mut c: *mut flv_buffer) -> libc::c_int {
    if (*c).d_cur == 0 {
        return 0 as libc::c_int;
    }
    if fwrite(
        (*c).data as *const libc::c_void,
        (*c).d_cur as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        (*c).fp,
    ) != 1 as libc::c_int as libc::c_ulong
    {
        return -(1 as libc::c_int);
    }
    (*c).d_total = ((*c).d_total).wrapping_add((*c).d_cur as uint64_t);
    (*c).d_cur = 0 as libc::c_int as libc::c_uint;
    0 as libc::c_int
}
