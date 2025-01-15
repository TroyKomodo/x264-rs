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
    fn x264_8_frame_shift(list: *mut *mut x264_frame_t) -> *mut x264_frame_t;
    fn x264_8_sync_frame_list_init(
        slist: *mut x264_sync_frame_list_t,
        nelem: libc::c_int,
    ) -> libc::c_int;
    fn x264_8_sync_frame_list_delete(slist: *mut x264_sync_frame_list_t);
    fn x264_8_sync_frame_list_push(slist: *mut x264_sync_frame_list_t, frame: *mut x264_frame_t);
    fn x264_8_sync_frame_list_pop(slist: *mut x264_sync_frame_list_t) -> *mut x264_frame_t;
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
    fn x264_malloc(_: int64_t) -> *mut libc::c_void;
    fn x264_free(_: *mut libc::c_void);
}
unsafe extern "C" fn threadpool_thread(mut pool: *mut x264_threadpool_t) -> *mut libc::c_void {
    while (*pool).exit == 0 {
        let mut job: *mut x264_threadpool_job_t = std::ptr::null_mut::<x264_threadpool_job_t>();
        pthread_mutex_lock(&mut (*pool).run.mutex);
        while (*pool).exit == 0 && (*pool).run.i_size == 0 {
            pthread_cond_wait(&mut (*pool).run.cv_fill, &mut (*pool).run.mutex);
        }
        if (*pool).run.i_size != 0 {
            job = x264_8_frame_shift((*pool).run.list) as *mut libc::c_void
                as *mut x264_threadpool_job_t;
            (*pool).run.i_size -= 1;
            (*pool).run.i_size;
        }
        pthread_mutex_unlock(&mut (*pool).run.mutex);
        if job.is_null() {
            continue;
        }
        (*job).ret = ((*job).func).expect("non-null function pointer")((*job).arg);
        x264_8_sync_frame_list_push(
            &mut (*pool).done,
            job as *mut libc::c_void as *mut x264_frame_t,
        );
    }
    std::ptr::null_mut::<libc::c_void>()
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_threadpool_init(
    mut p_pool: *mut *mut x264_threadpool_t,
    mut threads: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    if threads <= 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if (0 as libc::c_int) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    let mut pool: *mut x264_threadpool_t = std::ptr::null_mut::<x264_threadpool_t>();
    pool = x264_malloc(::core::mem::size_of::<x264_threadpool_t>() as libc::c_ulong as int64_t)
        as *mut x264_threadpool_t;
    if !pool.is_null() {
        memset(
            pool as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<x264_threadpool_t>() as libc::c_ulong,
        );
        *p_pool = pool;
        (*pool).threads = threads;
        (*pool).thread_handle = x264_malloc(
            ((*pool).threads as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<pthread_t>() as libc::c_ulong)
                as int64_t,
        ) as *mut pthread_t;
        if !((*pool).thread_handle).is_null()
            && !(x264_8_sync_frame_list_init(&mut (*pool).uninit, (*pool).threads) != 0
                || x264_8_sync_frame_list_init(&mut (*pool).run, (*pool).threads) != 0
                || x264_8_sync_frame_list_init(&mut (*pool).done, (*pool).threads) != 0)
        {
            let mut i: libc::c_int = 0 as libc::c_int;
            loop {
                if i >= (*pool).threads {
                    current_block = 11584701595673473500;
                    break;
                }
                let mut job: *mut x264_threadpool_job_t =
                    std::ptr::null_mut::<x264_threadpool_job_t>();
                job = x264_malloc(
                    ::core::mem::size_of::<x264_threadpool_job_t>() as libc::c_ulong as int64_t,
                ) as *mut x264_threadpool_job_t;
                if job.is_null() {
                    current_block = 17998364143390584866;
                    break;
                }
                x264_8_sync_frame_list_push(
                    &mut (*pool).uninit,
                    job as *mut libc::c_void as *mut x264_frame_t,
                );
                i += 1;
                i;
            }
            match current_block {
                17998364143390584866 => {}
                _ => {
                    let mut i_0: libc::c_int = 0 as libc::c_int;
                    loop {
                        if i_0 >= (*pool).threads {
                            current_block = 5634871135123216486;
                            break;
                        }
                        if pthread_create(
                            ((*pool).thread_handle).offset(i_0 as isize),
                            std::ptr::null::<pthread_attr_t>(),
                            ::core::mem::transmute::<
                                *mut libc::c_void,
                                Option<
                                    unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
                                >,
                            >(::core::mem::transmute::<
                                Option<
                                    unsafe extern "C" fn(
                                        *mut x264_threadpool_t,
                                    )
                                        -> *mut libc::c_void,
                                >,
                                *mut libc::c_void,
                            >(Some(
                                threadpool_thread
                                    as unsafe extern "C" fn(
                                        *mut x264_threadpool_t,
                                    )
                                        -> *mut libc::c_void,
                            ))),
                            pool as *mut libc::c_void,
                        ) != 0
                        {
                            current_block = 17998364143390584866;
                            break;
                        }
                        i_0 += 1;
                        i_0;
                    }
                    match current_block {
                        17998364143390584866 => {}
                        _ => return 0 as libc::c_int,
                    }
                }
            }
        }
    }
    -(1 as libc::c_int)
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_threadpool_run(
    mut pool: *mut x264_threadpool_t,
    mut func: Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
    mut arg: *mut libc::c_void,
) {
    let mut job: *mut x264_threadpool_job_t = x264_8_sync_frame_list_pop(&mut (*pool).uninit)
        as *mut libc::c_void
        as *mut x264_threadpool_job_t;
    (*job).func = func;
    (*job).arg = arg;
    x264_8_sync_frame_list_push(
        &mut (*pool).run,
        job as *mut libc::c_void as *mut x264_frame_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_threadpool_wait(
    mut pool: *mut x264_threadpool_t,
    mut arg: *mut libc::c_void,
) -> *mut libc::c_void {
    pthread_mutex_lock(&mut (*pool).done.mutex);
    loop {
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < (*pool).done.i_size {
            if (*(*((*pool).done.list).offset(i as isize) as *mut x264_threadpool_job_t)).arg == arg
            {
                let mut job: *mut x264_threadpool_job_t =
                    x264_8_frame_shift(((*pool).done.list).offset(i as isize)) as *mut libc::c_void
                        as *mut x264_threadpool_job_t;
                (*pool).done.i_size -= 1;
                (*pool).done.i_size;
                pthread_mutex_unlock(&mut (*pool).done.mutex);
                let mut ret: *mut libc::c_void = (*job).ret;
                x264_8_sync_frame_list_push(
                    &mut (*pool).uninit,
                    job as *mut libc::c_void as *mut x264_frame_t,
                );
                return ret;
            }
            i += 1;
            i;
        }
        pthread_cond_wait(&mut (*pool).done.cv_fill, &mut (*pool).done.mutex);
    }
}
unsafe extern "C" fn threadpool_list_delete(mut slist: *mut x264_sync_frame_list_t) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while !(*((*slist).list).offset(i as isize)).is_null() {
        x264_free(*((*slist).list).offset(i as isize) as *mut libc::c_void);
        let fresh0 = &mut (*((*slist).list).offset(i as isize));
        *fresh0 = std::ptr::null_mut::<x264_frame_t>();
        i += 1;
        i;
    }
    x264_8_sync_frame_list_delete(slist);
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_threadpool_delete(mut pool: *mut x264_threadpool_t) {
    pthread_mutex_lock(&mut (*pool).run.mutex);
    ::core::ptr::write_volatile(&mut (*pool).exit as *mut libc::c_int, 1 as libc::c_int);
    pthread_cond_broadcast(&mut (*pool).run.cv_fill);
    pthread_mutex_unlock(&mut (*pool).run.mutex);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*pool).threads {
        pthread_join(
            *((*pool).thread_handle).offset(i as isize),
            std::ptr::null_mut::<*mut libc::c_void>(),
        );
        i += 1;
        i;
    }
    threadpool_list_delete(&mut (*pool).uninit);
    threadpool_list_delete(&mut (*pool).run);
    threadpool_list_delete(&mut (*pool).done);
    x264_free((*pool).thread_handle as *mut libc::c_void);
    x264_free(pool as *mut libc::c_void);
}
