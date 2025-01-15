use crate::types::*;
use libc::memset;

unsafe extern "C" fn nal_escape_c(
    mut dst: *mut uint8_t,
    mut src: *mut uint8_t,
    mut end: *mut uint8_t,
) -> *mut uint8_t {
    if src < end {
        let fresh0 = src;
        src = src.offset(1);
        let fresh1 = dst;
        dst = dst.offset(1);
        *fresh1 = *fresh0;
    }
    if src < end {
        let fresh2 = src;
        src = src.offset(1);
        let fresh3 = dst;
        dst = dst.offset(1);
        *fresh3 = *fresh2;
    }
    while src < end {
        if *src.offset(0 as libc::c_int as isize) as libc::c_int <= 0x3 as libc::c_int
            && *dst.offset(-(2 as libc::c_int) as isize) == 0
            && *dst.offset(-(1 as libc::c_int) as isize) == 0
        {
            let fresh4 = dst;
            dst = dst.offset(1);
            *fresh4 = 0x3 as libc::c_int as uint8_t;
        }
        let fresh5 = src;
        src = src.offset(1);
        let fresh6 = dst;
        dst = dst.offset(1);
        *fresh6 = *fresh5;
    }
    dst
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_nal_encode(
    mut h: *mut x264_t,
    mut dst: *mut uint8_t,
    mut nal: *mut x264_nal_t,
) {
    let mut src: *mut uint8_t = (*nal).p_payload;
    let mut end: *mut uint8_t = ((*nal).p_payload).offset((*nal).i_payload as isize);
    let mut orig_dst: *mut uint8_t = dst;
    if (*h).param.b_annexb != 0 {
        if (*nal).b_long_startcode != 0 {
            let fresh7 = dst;
            dst = dst.offset(1);
            *fresh7 = 0 as libc::c_int as uint8_t;
        }
        let fresh8 = dst;
        dst = dst.offset(1);
        *fresh8 = 0 as libc::c_int as uint8_t;
        let fresh9 = dst;
        dst = dst.offset(1);
        *fresh9 = 0 as libc::c_int as uint8_t;
        let fresh10 = dst;
        dst = dst.offset(1);
        *fresh10 = 0x1 as libc::c_int as uint8_t;
    } else {
        dst = dst.offset(4 as libc::c_int as isize);
    }
    let fresh11 = dst;
    dst = dst.offset(1);
    *fresh11 = (((0 as libc::c_int) << 7 as libc::c_int)
        | ((*nal).i_ref_idc << 5 as libc::c_int)
        | (*nal).i_type) as uint8_t;
    dst = ((*h).bsf.nal_escape).expect("non-null function pointer")(dst, src, end);
    let mut size: libc::c_int = dst.offset_from(orig_dst) as libc::c_long as libc::c_int;
    if (*h).param.i_avcintra_class != 0 {
        let mut padding: libc::c_int =
            (*nal).i_payload + (*nal).i_padding + 5 as libc::c_int - size;
        if padding > 0 as libc::c_int {
            memset(
                dst as *mut libc::c_void,
                0 as libc::c_int,
                padding as usize,
            );
            size += padding;
        }
        (*nal).i_padding = if padding > 0 as libc::c_int {
            padding
        } else {
            0 as libc::c_int
        };
    }
    if (*h).param.b_annexb == 0 {
        let mut chunk_size: libc::c_int = size - 4 as libc::c_int;
        *orig_dst.offset(0 as libc::c_int as isize) = (chunk_size >> 24 as libc::c_int) as uint8_t;
        *orig_dst.offset(1 as libc::c_int as isize) = (chunk_size >> 16 as libc::c_int) as uint8_t;
        *orig_dst.offset(2 as libc::c_int as isize) = (chunk_size >> 8 as libc::c_int) as uint8_t;
        *orig_dst.offset(3 as libc::c_int as isize) = (chunk_size >> 0 as libc::c_int) as uint8_t;
    }
    (*nal).i_payload = size;
    (*nal).p_payload = orig_dst;
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_bitstream_init(
    mut _cpu: uint32_t,
    mut pf: *mut x264_bitstream_function_t,
) {
    memset(
        pf as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<x264_bitstream_function_t>(),
    );
    (*pf).nal_escape = Some(
        nal_escape_c
            as unsafe extern "C" fn(*mut uint8_t, *mut uint8_t, *mut uint8_t) -> *mut uint8_t,
    );
}
