#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use crate::types::*;
extern "C" {
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn x264_mdate() -> int64_t {
    let mut ts: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(1 as libc::c_int, &mut ts);
    ts.tv_sec * 1000000 as libc::c_int as int64_t + ts.tv_nsec / 1000 as libc::c_int as int64_t
}
