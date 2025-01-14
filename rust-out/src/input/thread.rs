#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type x264_threadpool_t;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn x264_cli_log(
        name: *const libc::c_char,
        i_level: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    static mut cli_input: cli_input_t;
    fn x264_8_threadpool_delete(pool: *mut x264_threadpool_t);
    fn x264_8_threadpool_run(
        pool: *mut x264_threadpool_t,
        func: Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
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
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
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
pub struct thread_hnd_t {
    pub input: cli_input_t,
    pub p_handle: hnd_t,
    pub pic: cli_pic_t,
    pub pool: *mut x264_threadpool_t,
    pub next_frame: libc::c_int,
    pub frame_total: libc::c_int,
    pub next_args: *mut thread_input_arg_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct thread_input_arg_t {
    pub h: *mut thread_hnd_t,
    pub pic: *mut cli_pic_t,
    pub i_frame: libc::c_int,
    pub status: libc::c_int,
}
unsafe extern "C" fn open_file(
    mut psz_filename: *mut libc::c_char,
    mut p_handle: *mut hnd_t,
    mut info: *mut video_info_t,
    mut opt: *mut cli_input_opt_t,
) -> libc::c_int {
    let mut h: *mut thread_hnd_t = malloc(
        ::core::mem::size_of::<thread_hnd_t>() as libc::c_ulong,
    ) as *mut thread_hnd_t;
    if h.is_null()
        || (cli_input.picture_alloc)
            .expect(
                "non-null function pointer",
            )(&mut (*h).pic, *p_handle, (*info).csp, (*info).width, (*info).height) != 0
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
    (*h)
        .next_args = malloc(
        ::core::mem::size_of::<thread_input_arg_t>() as libc::c_ulong,
    ) as *mut thread_input_arg_t;
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
    (*i)
        .status = ((*(*i).h).input.read_frame)
        .expect("non-null function pointer")((*i).pic, (*(*i).h).p_handle, (*i).i_frame);
}
unsafe extern "C" fn read_frame(
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
            (thread_8_input.release_frame)
                .expect("non-null function pointer")(&mut (*h).pic, handle);
        }
        ret
            |= ((*h).input.read_frame)
                .expect("non-null function pointer")(p_pic, (*h).p_handle, i_frame);
    }
    if (*h).frame_total == 0 || (i_frame + 1 as libc::c_int) < (*h).frame_total {
        (*(*h).next_args).i_frame = i_frame + 1 as libc::c_int;
        (*h).next_frame = (*(*h).next_args).i_frame;
        (*(*h).next_args).pic = &mut (*h).pic;
        x264_8_threadpool_run(
            (*h).pool,
            ::core::mem::transmute::<
                *mut libc::c_void,
                Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
            >(
                ::core::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut thread_input_arg_t) -> ()>,
                    *mut libc::c_void,
                >(
                    Some(
                        read_frame_thread_int
                            as unsafe extern "C" fn(*mut thread_input_arg_t) -> (),
                    ),
                ),
            ),
            (*h).next_args as *mut libc::c_void,
        );
    } else {
        (*h).next_frame = -(1 as libc::c_int);
    }
    ret
}
unsafe extern "C" fn release_frame(
    mut pic: *mut cli_pic_t,
    mut handle: hnd_t,
) -> libc::c_int {
    let mut h: *mut thread_hnd_t = handle as *mut thread_hnd_t;
    if ((*h).input.release_frame).is_some() {
        return ((*h).input.release_frame)
            .expect("non-null function pointer")(pic, (*h).p_handle);
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
    let mut h: *mut thread_hnd_t = handle as *mut thread_hnd_t;
    ((*h).input.picture_alloc)
        .expect("non-null function pointer")(pic, (*h).p_handle, csp, width, height)
}
unsafe extern "C" fn picture_clean(mut pic: *mut cli_pic_t, mut handle: hnd_t) {
    let mut h: *mut thread_hnd_t = handle as *mut thread_hnd_t;
    ((*h).input.picture_clean).expect("non-null function pointer")(pic, (*h).p_handle);
}
unsafe extern "C" fn close_file(mut handle: hnd_t) -> libc::c_int {
    let mut h: *mut thread_hnd_t = handle as *mut thread_hnd_t;
    x264_8_threadpool_delete((*h).pool);
    ((*h).input.picture_clean)
        .expect("non-null function pointer")(&mut (*h).pic, (*h).p_handle);
    ((*h).input.close_file).expect("non-null function pointer")((*h).p_handle);
    free((*h).next_args as *mut libc::c_void);
    free(h as *mut libc::c_void);
    0 as libc::c_int
}
#[no_mangle]
pub static mut thread_8_input: cli_input_t = unsafe {
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
        }
    }
};
