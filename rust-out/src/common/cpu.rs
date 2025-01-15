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
    fn __sched_cpucount(__setsize: size_t, __setp: *const cpu_set_t) -> libc::c_int;
    fn sched_getaffinity(
        __pid: __pid_t,
        __cpusetsize: size_t,
        __cpuset: *mut cpu_set_t,
    ) -> libc::c_int;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
}
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
