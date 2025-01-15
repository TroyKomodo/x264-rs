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
use crate::types::*;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn x264_cli_log(
        name: *const libc::c_char,
        i_level: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    fn x264_8_threadpool_delete(pool: *mut x264_threadpool_t);
    fn x264_8_threadpool_run(
        pool: *mut x264_threadpool_t,
        func: Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
        arg: *mut libc::c_void,
    );
    fn x264_8_threadpool_wait(
        pool: *mut x264_threadpool_t,
        arg: *mut libc::c_void,
    ) -> *mut libc::c_void;
    fn x264_8_threadpool_init(
        p_pool: *mut *mut x264_threadpool_t,
        threads: libc::c_int,
    ) -> libc::c_int;
}
pub unsafe extern "C" fn open_file(
    mut psz_filename: *mut libc::c_char,
    mut p_handle: *mut hnd_t,
    mut info: *mut video_info_t,
    mut opt: *mut cli_input_opt_t,
) -> libc::c_int {
    let mut h: *mut thread_hnd_t =
        malloc(::core::mem::size_of::<thread_hnd_t>() as libc::c_ulong) as *mut thread_hnd_t;
    if h.is_null()
        || (cli_input.picture_alloc).expect("non-null function pointer")(
            &mut (*h).pic,
            *p_handle,
            (*info).csp,
            (*info).width,
            (*info).height,
        ) != 0
    {
        x264_cli_log(
            b"x264\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            b"malloc failed\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    (*h).input = cli_input;
    (*h).p_handle = *p_handle;
    (*h).next_frame = -(1 as libc::c_int);
    (*h).next_args = malloc(::core::mem::size_of::<thread_input_arg_t>() as libc::c_ulong)
        as *mut thread_input_arg_t;
    if ((*h).next_args).is_null() {
        return -(1 as libc::c_int);
    }
    (*(*h).next_args).h = h;
    (*(*h).next_args).status = 0 as libc::c_int;
    (*h).frame_total = (*info).num_frames;
    if x264_8_threadpool_init(&mut (*h).pool, 1 as libc::c_int) != 0 {
        return -(1 as libc::c_int);
    }
    *p_handle = h as hnd_t;
    0 as libc::c_int
}
unsafe extern "C" fn read_frame_thread_int(mut i: *mut thread_input_arg_t) {
    (*i).status = ((*(*i).h).input.read_frame).expect("non-null function pointer")(
        (*i).pic,
        (*(*i).h).p_handle,
        (*i).i_frame,
    );
}
pub unsafe extern "C" fn read_frame(
    mut p_pic: *mut cli_pic_t,
    mut handle: hnd_t,
    mut i_frame: libc::c_int,
) -> libc::c_int {
    let mut h: *mut thread_hnd_t = handle as *mut thread_hnd_t;
    let mut ret: libc::c_int = 0 as libc::c_int;
    if (*h).next_frame >= 0 as libc::c_int {
        x264_8_threadpool_wait((*h).pool, (*h).next_args as *mut libc::c_void);
        ret |= (*(*h).next_args).status;
    }
    if (*h).next_frame == i_frame {
        core::ptr::swap(p_pic, &mut (*h).pic);
    } else {
        if (*h).next_frame >= 0 as libc::c_int {
            (thread_8_input.release_frame).expect("non-null function pointer")(
                &mut (*h).pic,
                handle,
            );
        }
        ret |= ((*h).input.read_frame).expect("non-null function pointer")(
            p_pic,
            (*h).p_handle,
            i_frame,
        );
    }
    if (*h).frame_total == 0 || (i_frame + 1 as libc::c_int) < (*h).frame_total {
        (*(*h).next_args).i_frame = i_frame + 1 as libc::c_int;
        (*h).next_frame = (*(*h).next_args).i_frame;
        (*(*h).next_args).pic = &mut (*h).pic;
        x264_8_threadpool_run(
            (*h).pool,
            ::core::mem::transmute::<
                *mut libc::c_void,
                Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
            >(::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut thread_input_arg_t) -> ()>,
                *mut libc::c_void,
            >(Some(
                read_frame_thread_int as unsafe extern "C" fn(*mut thread_input_arg_t) -> (),
            ))),
            (*h).next_args as *mut libc::c_void,
        );
    } else {
        (*h).next_frame = -(1 as libc::c_int);
    }
    ret
}
pub unsafe extern "C" fn release_frame(mut pic: *mut cli_pic_t, mut handle: hnd_t) -> libc::c_int {
    let mut h: *mut thread_hnd_t = handle as *mut thread_hnd_t;
    if ((*h).input.release_frame).is_some() {
        return ((*h).input.release_frame).expect("non-null function pointer")(pic, (*h).p_handle);
    }
    0 as libc::c_int
}
pub unsafe extern "C" fn picture_alloc(
    mut pic: *mut cli_pic_t,
    mut handle: hnd_t,
    mut csp: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> libc::c_int {
    let mut h: *mut thread_hnd_t = handle as *mut thread_hnd_t;
    ((*h).input.picture_alloc).expect("non-null function pointer")(
        pic,
        (*h).p_handle,
        csp,
        width,
        height,
    )
}
pub unsafe extern "C" fn picture_clean(mut pic: *mut cli_pic_t, mut handle: hnd_t) {
    let mut h: *mut thread_hnd_t = handle as *mut thread_hnd_t;
    ((*h).input.picture_clean).expect("non-null function pointer")(pic, (*h).p_handle);
}
pub unsafe extern "C" fn close_file(mut handle: hnd_t) -> libc::c_int {
    let mut h: *mut thread_hnd_t = handle as *mut thread_hnd_t;
    x264_8_threadpool_delete((*h).pool);
    ((*h).input.picture_clean).expect("non-null function pointer")(&mut (*h).pic, (*h).p_handle);
    ((*h).input.close_file).expect("non-null function pointer")((*h).p_handle);
    free((*h).next_args as *mut libc::c_void);
    free(h as *mut libc::c_void);
    0 as libc::c_int
}
