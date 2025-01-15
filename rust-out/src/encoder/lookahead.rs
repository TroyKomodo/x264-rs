#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types, label_break_value)]
use crate::types::*;
extern "C" {
    fn x264_8_frame_push(list: *mut *mut x264_frame_t, frame: *mut x264_frame_t);
    fn x264_8_frame_shift(list: *mut *mut x264_frame_t) -> *mut x264_frame_t;
    fn x264_8_frame_push_unused(h: *mut x264_t, frame: *mut x264_frame_t);
    fn x264_8_sync_frame_list_init(
        slist: *mut x264_sync_frame_list_t,
        nelem: libc::c_int,
    ) -> libc::c_int;
    fn x264_8_sync_frame_list_delete(slist: *mut x264_sync_frame_list_t);
    fn x264_8_sync_frame_list_push(slist: *mut x264_sync_frame_list_t, frame: *mut x264_frame_t);
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_join(__th: pthread_t, __thread_return: *mut *mut libc::c_void) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_wait(__cond: *mut pthread_cond_t, __mutex: *mut pthread_mutex_t)
        -> libc::c_int;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn x264_malloc(_: int64_t) -> *mut libc::c_void;
    fn x264_free(_: *mut libc::c_void);
    fn x264_8_macroblock_cache_allocate(h: *mut x264_t) -> libc::c_int;
    fn x264_8_macroblock_cache_free(h: *mut x264_t);
    fn x264_8_macroblock_thread_allocate(h: *mut x264_t, b_lookahead: libc::c_int) -> libc::c_int;
    fn x264_8_macroblock_thread_free(h: *mut x264_t, b_lookahead: libc::c_int);
    fn x264_8_slicetype_decide(h: *mut x264_t);
    fn x264_8_slicetype_analyse(h: *mut x264_t, intra_minigop: libc::c_int);
}
unsafe extern "C" fn lookahead_shift(
    mut dst: *mut x264_sync_frame_list_t,
    mut src: *mut x264_sync_frame_list_t,
    mut count: libc::c_int,
) {
    let mut i: libc::c_int = count;
    loop {
        let fresh0 = i;
        i -= 1;
        if fresh0 == 0 {
            break;
        }
        if (*dst).i_size < (*dst).i_max_size {
        } else {
            __assert_fail(
                b"dst->i_size < dst->i_max_size\0" as *const u8 as *const libc::c_char,
                b"encoder/lookahead.c\0" as *const u8 as *const libc::c_char,
                47 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 78],
                    &[libc::c_char; 78],
                >(
                    b"void lookahead_shift(x264_sync_frame_list_t *, x264_sync_frame_list_t *, int)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_28292: {
            if (*dst).i_size < (*dst).i_max_size {
            } else {
                __assert_fail(
                    b"dst->i_size < dst->i_max_size\0" as *const u8
                        as *const libc::c_char,
                    b"encoder/lookahead.c\0" as *const u8 as *const libc::c_char,
                    47 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 78],
                        &[libc::c_char; 78],
                    >(
                        b"void lookahead_shift(x264_sync_frame_list_t *, x264_sync_frame_list_t *, int)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        if (*src).i_size != 0 {
        } else {
            __assert_fail(
                b"src->i_size\0" as *const u8 as *const libc::c_char,
                b"encoder/lookahead.c\0" as *const u8 as *const libc::c_char,
                48 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 78],
                    &[libc::c_char; 78],
                >(
                    b"void lookahead_shift(x264_sync_frame_list_t *, x264_sync_frame_list_t *, int)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_28254: {
            if (*src).i_size != 0 {
            } else {
                __assert_fail(
                    b"src->i_size\0" as *const u8 as *const libc::c_char,
                    b"encoder/lookahead.c\0" as *const u8 as *const libc::c_char,
                    48 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 78],
                        &[libc::c_char; 78],
                    >(
                        b"void lookahead_shift(x264_sync_frame_list_t *, x264_sync_frame_list_t *, int)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        let fresh1 = (*dst).i_size;
        (*dst).i_size += 1;
        let fresh2 = &mut (*((*dst).list).offset(fresh1 as isize));
        *fresh2 = x264_8_frame_shift((*src).list);
        (*src).i_size -= 1;
        (*src).i_size;
    }
    if count != 0 {
        pthread_cond_broadcast(&mut (*dst).cv_fill);
        pthread_cond_broadcast(&mut (*src).cv_empty);
    }
}
unsafe extern "C" fn lookahead_update_last_nonb(
    mut h: *mut x264_t,
    mut new_nonb: *mut x264_frame_t,
) {
    if !((*(*h).lookahead).last_nonb).is_null() {
        x264_8_frame_push_unused(h, (*(*h).lookahead).last_nonb);
    }
    (*(*h).lookahead).last_nonb = new_nonb;
    (*new_nonb).i_reference_count += 1;
    (*new_nonb).i_reference_count;
}
unsafe extern "C" fn lookahead_slicetype_decide(mut h: *mut x264_t) {
    x264_8_slicetype_decide(h);
    lookahead_update_last_nonb(
        h,
        *((*(*h).lookahead).next.list).offset(0 as libc::c_int as isize),
    );
    let mut shift_frames: libc::c_int = (**((*(*h).lookahead).next.list)
        .offset(0 as libc::c_int as isize))
    .i_bframes as libc::c_int
        + 1 as libc::c_int;
    pthread_mutex_lock(&mut (*(*h).lookahead).ofbuf.mutex);
    while (*(*h).lookahead).ofbuf.i_size == (*(*h).lookahead).ofbuf.i_max_size {
        pthread_cond_wait(
            &mut (*(*h).lookahead).ofbuf.cv_empty,
            &mut (*(*h).lookahead).ofbuf.mutex,
        );
    }
    pthread_mutex_lock(&mut (*(*h).lookahead).next.mutex);
    lookahead_shift(
        &mut (*(*h).lookahead).ofbuf,
        &mut (*(*h).lookahead).next,
        shift_frames,
    );
    pthread_mutex_unlock(&mut (*(*h).lookahead).next.mutex);
    if (*(*h).lookahead).b_analyse_keyframe as libc::c_int != 0
        && ((*(*(*h).lookahead).last_nonb).i_type == 0x2 as libc::c_int
            || (*(*(*h).lookahead).last_nonb).i_type == 0x1 as libc::c_int
            || (*(*(*h).lookahead).last_nonb).i_type == 0x6 as libc::c_int)
    {
        x264_8_slicetype_analyse(h, shift_frames);
    }
    pthread_mutex_unlock(&mut (*(*h).lookahead).ofbuf.mutex);
}
unsafe extern "C" fn lookahead_thread(mut h: *mut x264_t) -> *mut libc::c_void {
    loop {
        pthread_mutex_lock(&mut (*(*h).lookahead).ifbuf.mutex);
        if (*(*h).lookahead).b_exit_thread != 0 {
            pthread_mutex_unlock(&mut (*(*h).lookahead).ifbuf.mutex);
            break;
        } else {
            pthread_mutex_lock(&mut (*(*h).lookahead).next.mutex);
            let mut shift: libc::c_int = if (*(*h).lookahead).next.i_max_size
                - (*(*h).lookahead).next.i_size
                < (*(*h).lookahead).ifbuf.i_size
            {
                (*(*h).lookahead).next.i_max_size - (*(*h).lookahead).next.i_size
            } else {
                (*(*h).lookahead).ifbuf.i_size
            };
            lookahead_shift(
                &mut (*(*h).lookahead).next,
                &mut (*(*h).lookahead).ifbuf,
                shift,
            );
            pthread_mutex_unlock(&mut (*(*h).lookahead).next.mutex);
            if (*(*h).lookahead).next.i_size
                <= (*(*h).lookahead).i_slicetype_length + (*h).param.b_vfr_input
            {
                while (*(*h).lookahead).ifbuf.i_size == 0 && (*(*h).lookahead).b_exit_thread == 0 {
                    pthread_cond_wait(
                        &mut (*(*h).lookahead).ifbuf.cv_fill,
                        &mut (*(*h).lookahead).ifbuf.mutex,
                    );
                }
                pthread_mutex_unlock(&mut (*(*h).lookahead).ifbuf.mutex);
            } else {
                pthread_mutex_unlock(&mut (*(*h).lookahead).ifbuf.mutex);
                lookahead_slicetype_decide(h);
            }
        }
    }
    pthread_mutex_lock(&mut (*(*h).lookahead).ifbuf.mutex);
    pthread_mutex_lock(&mut (*(*h).lookahead).next.mutex);
    lookahead_shift(
        &mut (*(*h).lookahead).next,
        &mut (*(*h).lookahead).ifbuf,
        (*(*h).lookahead).ifbuf.i_size,
    );
    pthread_mutex_unlock(&mut (*(*h).lookahead).next.mutex);
    pthread_mutex_unlock(&mut (*(*h).lookahead).ifbuf.mutex);
    while (*(*h).lookahead).next.i_size != 0 {
        lookahead_slicetype_decide(h);
    }
    pthread_mutex_lock(&mut (*(*h).lookahead).ofbuf.mutex);
    (*(*h).lookahead).b_thread_active = 0 as libc::c_int as uint8_t;
    pthread_cond_broadcast(&mut (*(*h).lookahead).ofbuf.cv_fill);
    pthread_mutex_unlock(&mut (*(*h).lookahead).ofbuf.mutex);
    std::ptr::null_mut::<libc::c_void>()
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_lookahead_init(
    mut h: *mut x264_t,
    mut i_slicetype_length: libc::c_int,
) -> libc::c_int {
    let mut look_h: *mut x264_t = std::ptr::null_mut::<x264_t>();
    let mut look: *mut x264_lookahead_t = std::ptr::null_mut::<x264_lookahead_t>();
    look = x264_malloc(::core::mem::size_of::<x264_lookahead_t>() as libc::c_ulong as int64_t)
        as *mut x264_lookahead_t;
    if !look.is_null() {
        memset(
            look as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<x264_lookahead_t>() as libc::c_ulong,
        );
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < (*h).param.i_threads {
            (*(*h).thread[i as usize]).lookahead = look;
            i += 1;
            i;
        }
        (*look).i_last_keyframe = -(*h).param.i_keyint_max;
        (*look).b_analyse_keyframe = (((*h).param.rc.b_mb_tree != 0
            || (*h).param.rc.i_vbv_buffer_size != 0 && (*h).param.rc.i_lookahead != 0)
            && (*h).param.rc.b_stat_read == 0) as libc::c_int
            as uint8_t;
        (*look).i_slicetype_length = i_slicetype_length;
        if !(x264_8_sync_frame_list_init(
            &mut (*look).ifbuf,
            (*h).param.i_sync_lookahead + 3 as libc::c_int,
        ) != 0
            || x264_8_sync_frame_list_init(
                &mut (*look).next,
                (*h).frames.i_delay + 3 as libc::c_int,
            ) != 0
            || x264_8_sync_frame_list_init(
                &mut (*look).ofbuf,
                (*h).frames.i_delay + 3 as libc::c_int,
            ) != 0)
        {
            if (*h).param.i_sync_lookahead == 0 {
                return 0 as libc::c_int;
            }
            look_h = (*h).thread[(*h).param.i_threads as usize];
            *look_h = *h;
            if x264_8_macroblock_cache_allocate(look_h) == 0
                && x264_8_macroblock_thread_allocate(look_h, 1 as libc::c_int) >= 0 as libc::c_int
                && pthread_create(
                    &mut (*look).thread_handle,
                    std::ptr::null::<pthread_attr_t>(),
                    ::core::mem::transmute::<
                        *mut libc::c_void,
                        Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
                    >(::core::mem::transmute::<
                        Option<unsafe extern "C" fn(*mut x264_t) -> *mut libc::c_void>,
                        *mut libc::c_void,
                    >(Some(
                        lookahead_thread as unsafe extern "C" fn(*mut x264_t) -> *mut libc::c_void,
                    ))),
                    look_h as *mut libc::c_void,
                ) == 0
            {
                (*look).b_thread_active = 1 as libc::c_int as uint8_t;
                return 0 as libc::c_int;
            }
        }
    }
    x264_free(look as *mut libc::c_void);
    -(1 as libc::c_int)
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_lookahead_delete(mut h: *mut x264_t) {
    if (*h).param.i_sync_lookahead != 0 {
        pthread_mutex_lock(&mut (*(*h).lookahead).ifbuf.mutex);
        ::core::ptr::write_volatile(
            &mut (*(*h).lookahead).b_exit_thread as *mut uint8_t,
            1 as libc::c_int as uint8_t,
        );
        pthread_cond_broadcast(&mut (*(*h).lookahead).ifbuf.cv_fill);
        pthread_mutex_unlock(&mut (*(*h).lookahead).ifbuf.mutex);
        pthread_join(
            (*(*h).lookahead).thread_handle,
            std::ptr::null_mut::<*mut libc::c_void>(),
        );
        x264_8_macroblock_cache_free((*h).thread[(*h).param.i_threads as usize]);
        x264_8_macroblock_thread_free((*h).thread[(*h).param.i_threads as usize], 1 as libc::c_int);
        x264_free((*h).thread[(*h).param.i_threads as usize] as *mut libc::c_void);
    }
    x264_8_sync_frame_list_delete(&mut (*(*h).lookahead).ifbuf);
    x264_8_sync_frame_list_delete(&mut (*(*h).lookahead).next);
    if !((*(*h).lookahead).last_nonb).is_null() {
        x264_8_frame_push_unused(h, (*(*h).lookahead).last_nonb);
    }
    x264_8_sync_frame_list_delete(&mut (*(*h).lookahead).ofbuf);
    x264_free((*h).lookahead as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_lookahead_put_frame(
    mut h: *mut x264_t,
    mut frame: *mut x264_frame_t,
) {
    if (*h).param.i_sync_lookahead != 0 {
        x264_8_sync_frame_list_push(&mut (*(*h).lookahead).ifbuf, frame);
    } else {
        x264_8_sync_frame_list_push(&mut (*(*h).lookahead).next, frame);
    };
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_lookahead_is_empty(mut h: *mut x264_t) -> libc::c_int {
    pthread_mutex_lock(&mut (*(*h).lookahead).ofbuf.mutex);
    pthread_mutex_lock(&mut (*(*h).lookahead).next.mutex);
    let mut b_empty: libc::c_int =
        ((*(*h).lookahead).next.i_size == 0 && (*(*h).lookahead).ofbuf.i_size == 0) as libc::c_int;
    pthread_mutex_unlock(&mut (*(*h).lookahead).next.mutex);
    pthread_mutex_unlock(&mut (*(*h).lookahead).ofbuf.mutex);
    b_empty
}
unsafe extern "C" fn lookahead_encoder_shift(mut h: *mut x264_t) {
    if (*(*h).lookahead).ofbuf.i_size == 0 {
        return;
    }
    let mut i_frames: libc::c_int = (**((*(*h).lookahead).ofbuf.list)
        .offset(0 as libc::c_int as isize))
    .i_bframes as libc::c_int
        + 1 as libc::c_int;
    loop {
        let fresh3 = i_frames;
        i_frames -= 1;
        if fresh3 == 0 {
            break;
        }
        x264_8_frame_push(
            (*h).frames.current,
            x264_8_frame_shift((*(*h).lookahead).ofbuf.list),
        );
        (*(*h).lookahead).ofbuf.i_size -= 1;
        (*(*h).lookahead).ofbuf.i_size;
    }
    pthread_cond_broadcast(&mut (*(*h).lookahead).ofbuf.cv_empty);
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_lookahead_get_frames(mut h: *mut x264_t) {
    if (*h).param.i_sync_lookahead != 0 {
        pthread_mutex_lock(&mut (*(*h).lookahead).ofbuf.mutex);
        while (*(*h).lookahead).ofbuf.i_size == 0
            && (*(*h).lookahead).b_thread_active as libc::c_int != 0
        {
            pthread_cond_wait(
                &mut (*(*h).lookahead).ofbuf.cv_fill,
                &mut (*(*h).lookahead).ofbuf.mutex,
            );
        }
        lookahead_encoder_shift(h);
        pthread_mutex_unlock(&mut (*(*h).lookahead).ofbuf.mutex);
    } else {
        if !(*((*h).frames.current).offset(0 as libc::c_int as isize)).is_null()
            || (*(*h).lookahead).next.i_size == 0
        {
            return;
        }
        x264_8_slicetype_decide(h);
        lookahead_update_last_nonb(
            h,
            *((*(*h).lookahead).next.list).offset(0 as libc::c_int as isize),
        );
        let mut shift_frames: libc::c_int = (**((*(*h).lookahead).next.list)
            .offset(0 as libc::c_int as isize))
        .i_bframes as libc::c_int
            + 1 as libc::c_int;
        lookahead_shift(
            &mut (*(*h).lookahead).ofbuf,
            &mut (*(*h).lookahead).next,
            shift_frames,
        );
        if (*(*h).lookahead).b_analyse_keyframe as libc::c_int != 0
            && ((*(*(*h).lookahead).last_nonb).i_type == 0x2 as libc::c_int
                || (*(*(*h).lookahead).last_nonb).i_type == 0x1 as libc::c_int
                || (*(*(*h).lookahead).last_nonb).i_type == 0x6 as libc::c_int)
        {
            x264_8_slicetype_analyse(h, shift_frames);
        }
        lookahead_encoder_shift(h);
    };
}
