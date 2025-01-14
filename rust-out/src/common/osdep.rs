#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;
}
pub type __int64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type int64_t = __int64_t;
pub type clockid_t = __clockid_t;
#[no_mangle]
pub unsafe extern "C" fn x264_mdate() -> int64_t {
    let mut ts: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    clock_gettime(1 as libc::c_int, &mut ts);
    ts.tv_sec * 1000000 as libc::c_int as int64_t
        + ts.tv_nsec / 1000 as libc::c_int as int64_t
}
