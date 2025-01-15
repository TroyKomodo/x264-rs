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
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn x264_malloc(_: int64_t) -> *mut libc::c_void;
    fn x264_free(_: *mut libc::c_void);
    fn munmap(__addr: *mut libc::c_void, __len: size_t) -> libc::c_int;
    fn madvise(__addr: *mut libc::c_void, __len: size_t, __advice: libc::c_int) -> libc::c_int;
    fn mmap(
        __addr: *mut libc::c_void,
        __len: size_t,
        __prot: libc::c_int,
        __flags: libc::c_int,
        __fd: libc::c_int,
        __offset: __off64_t,
    ) -> *mut libc::c_void;
    fn sysconf(__name: libc::c_int) -> libc::c_long;
}
#[no_mangle]
pub unsafe extern "C" fn x264_cli_csp_is_invalid(mut csp: libc::c_int) -> libc::c_int {
    let mut csp_mask: libc::c_int = csp & 0xff as libc::c_int;
    (csp_mask <= 0 as libc::c_int
        || csp_mask >= 0x11 as libc::c_int
        || csp_mask == 0xb as libc::c_int
        || csp & 0x4000 as libc::c_int != 0) as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn x264_cli_csp_depth_factor(mut csp: libc::c_int) -> libc::c_int {
    if x264_cli_csp_is_invalid(csp) != 0 {
        return 0 as libc::c_int;
    }
    if csp & 0x2000 as libc::c_int != 0 {
        2 as libc::c_int
    } else {
        1 as libc::c_int
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_cli_pic_plane_size(
    mut csp: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut plane: libc::c_int,
) -> int64_t {
    let mut csp_mask: libc::c_int = csp & 0xff as libc::c_int;
    if x264_cli_csp_is_invalid(csp) != 0
        || plane < 0 as libc::c_int
        || plane >= x264_cli_csps[csp_mask as usize].planes
    {
        return 0 as libc::c_int as int64_t;
    }
    let mut size: int64_t = width as int64_t * height as int64_t;
    size = (size as libc::c_float
        * (x264_cli_csps[csp_mask as usize].width[plane as usize]
            * x264_cli_csps[csp_mask as usize].height[plane as usize])) as int64_t;
    size *= x264_cli_csp_depth_factor(csp) as int64_t;
    size
}
#[no_mangle]
pub unsafe extern "C" fn x264_cli_pic_size(
    mut csp: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> int64_t {
    if x264_cli_csp_is_invalid(csp) != 0 {
        return 0 as libc::c_int as int64_t;
    }
    let mut size: int64_t = 0 as libc::c_int as int64_t;
    let mut csp_mask: libc::c_int = csp & 0xff as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < x264_cli_csps[csp_mask as usize].planes {
        size += x264_cli_pic_plane_size(csp, width, height, i);
        i += 1;
        i;
    }
    size
}
unsafe extern "C" fn cli_pic_init_internal(
    mut pic: *mut cli_pic_t,
    mut csp: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut align: libc::c_int,
    mut alloc: libc::c_int,
) -> libc::c_int {
    memset(
        pic as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<cli_pic_t>() as libc::c_ulong,
    );
    let mut csp_mask: libc::c_int = csp & 0xff as libc::c_int;
    if x264_cli_csp_is_invalid(csp) != 0 {
        (*pic).img.planes = 0 as libc::c_int;
    } else {
        (*pic).img.planes = x264_cli_csps[csp_mask as usize].planes;
    }
    (*pic).img.csp = csp;
    (*pic).img.width = width;
    (*pic).img.height = height;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*pic).img.planes {
        let mut stride: libc::c_int = (width as libc::c_float
            * x264_cli_csps[csp_mask as usize].width[i as usize])
            as libc::c_int;
        stride *= x264_cli_csp_depth_factor(csp);
        stride = (stride + (align - 1 as libc::c_int)) & !(align - 1 as libc::c_int);
        (*pic).img.stride[i as usize] = stride;
        if alloc != 0 {
            let mut size: int64_t = (height as libc::c_float
                * x264_cli_csps[csp_mask as usize].height[i as usize])
                as int64_t
                * stride as int64_t;
            (*pic).img.plane[i as usize] = x264_malloc(size) as *mut uint8_t;
            if ((*pic).img.plane[i as usize]).is_null() {
                return -(1 as libc::c_int);
            }
        }
        i += 1;
        i;
    }
    0 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn x264_cli_pic_alloc(
    mut pic: *mut cli_pic_t,
    mut csp: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> libc::c_int {
    cli_pic_init_internal(pic, csp, width, height, 1 as libc::c_int, 1 as libc::c_int)
}
#[no_mangle]
pub unsafe extern "C" fn x264_cli_pic_alloc_aligned(
    mut pic: *mut cli_pic_t,
    mut csp: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> libc::c_int {
    cli_pic_init_internal(pic, csp, width, height, 64 as libc::c_int, 1 as libc::c_int)
}
#[no_mangle]
pub unsafe extern "C" fn x264_cli_pic_init_noalloc(
    mut pic: *mut cli_pic_t,
    mut csp: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> libc::c_int {
    cli_pic_init_internal(pic, csp, width, height, 1 as libc::c_int, 0 as libc::c_int)
}
#[no_mangle]
pub unsafe extern "C" fn x264_cli_pic_clean(mut pic: *mut cli_pic_t) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*pic).img.planes {
        x264_free((*pic).img.plane[i as usize] as *mut libc::c_void);
        i += 1;
        i;
    }
    memset(
        pic as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<cli_pic_t>() as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn x264_cli_get_csp(mut csp: libc::c_int) -> *const x264_cli_csp_t {
    if x264_cli_csp_is_invalid(csp) != 0 {
        return std::ptr::null::<x264_cli_csp_t>();
    }
    x264_cli_csps
        .as_ptr()
        .offset((csp & 0xff as libc::c_int) as isize)
}
#[no_mangle]
pub unsafe extern "C" fn x264_cli_mmap_init(
    mut h: *mut cli_mmap_t,
    mut fh: *mut FILE,
) -> libc::c_int {
    let mut fd: libc::c_int = fileno(fh);
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
    if fstat(fd, &mut file_stat) == 0 {
        (*h).file_size = file_stat.st_size;
        (*h).align_mask = (sysconf(_SC_PAGESIZE as libc::c_int) - 1 as libc::c_int as libc::c_long)
            as libc::c_int;
        (*h).fd = fd;
        return ((*h).align_mask < 0 as libc::c_int || fd < 0 as libc::c_int) as libc::c_int;
    }
    -(1 as libc::c_int)
}
#[no_mangle]
pub unsafe extern "C" fn x264_cli_mmap(
    mut h: *mut cli_mmap_t,
    mut offset: int64_t,
    mut size: int64_t,
) -> *mut libc::c_void {
    let mut base: *mut uint8_t = std::ptr::null_mut::<uint8_t>();
    let mut align: libc::c_int = (offset & (*h).align_mask as int64_t) as libc::c_int;
    if offset < 0 as libc::c_int as int64_t
        || size < 0 as libc::c_int as int64_t
        || size as uint64_t
            > (18446744073709551615 as libc::c_ulong)
                .wrapping_sub(64 as libc::c_int as libc::c_ulong)
                .wrapping_sub(align as libc::c_ulong)
    {
        return std::ptr::null_mut::<libc::c_void>();
    }
    offset -= align as int64_t;
    size += align as int64_t;
    let mut padded_size: size_t = (size + 64 as libc::c_int as int64_t) as size_t;
    base = mmap(
        std::ptr::null_mut::<libc::c_void>(),
        padded_size,
        0x1 as libc::c_int,
        0x2 as libc::c_int,
        (*h).fd,
        offset,
    ) as *mut uint8_t;
    if base != -(1 as libc::c_int) as *mut libc::c_void as *mut uint8_t {
        madvise(base as *mut libc::c_void, size as size_t, 3 as libc::c_int);
        let mut aligned_size: size_t =
            padded_size.wrapping_sub(1 as libc::c_int as size_t) & !(*h).align_mask as size_t;
        if (offset as size_t).wrapping_add(aligned_size) >= (*h).file_size as size_t {
            mmap(
                base.offset(aligned_size as isize) as *mut libc::c_void,
                padded_size.wrapping_sub(aligned_size),
                0x1 as libc::c_int,
                0x2 as libc::c_int | 0x10 as libc::c_int,
                (*h).fd,
                (offset + size - 1 as libc::c_int as int64_t) & !(*h).align_mask as int64_t,
            );
        }
        return base.offset(align as isize) as *mut libc::c_void;
    }
    std::ptr::null_mut::<libc::c_void>()
}
#[no_mangle]
pub unsafe extern "C" fn x264_cli_munmap(
    mut h: *mut cli_mmap_t,
    mut addr: *mut libc::c_void,
    mut size: int64_t,
) -> libc::c_int {
    let mut base: *mut libc::c_void =
        (addr as intptr_t & !(*h).align_mask as intptr_t) as *mut libc::c_void;
    if size < 0 as libc::c_int as int64_t
        || size as libc::c_ulong
            > (18446744073709551615 as libc::c_ulong)
                .wrapping_sub(64 as libc::c_int as libc::c_ulong)
                .wrapping_sub((addr as intptr_t - base as intptr_t) as libc::c_ulong)
    {
        return -(1 as libc::c_int);
    }
    munmap(
        base,
        (size + 64 as libc::c_int as int64_t + addr as intptr_t - base as intptr_t) as size_t,
    )
}
#[no_mangle]
pub unsafe extern "C" fn x264_cli_mmap_close(mut h: *mut cli_mmap_t) {}
