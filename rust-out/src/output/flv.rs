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
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fseeko(__stream: *mut FILE, __off: __off64_t, __whence: libc::c_int) -> libc::c_int;
    fn ftello(__stream: *mut FILE) -> __off64_t;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn x264_cli_log(
        name: *const libc::c_char,
        i_level: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    fn flv_create_writer(filename: *const libc::c_char) -> *mut flv_buffer;
    fn flv_append_data(c: *mut flv_buffer, data: *mut uint8_t, size: libc::c_uint) -> libc::c_int;
    fn flv_flush_data(c: *mut flv_buffer) -> libc::c_int;
    fn flv_rewrite_amf_be24(c: *mut flv_buffer, length: libc::c_uint, start: libc::c_uint);
    fn flv_dbl2int(value: libc::c_double) -> uint64_t;
    fn flv_put_byte(c: *mut flv_buffer, b: uint8_t);
    fn flv_put_be32(c: *mut flv_buffer, val: uint32_t);
    fn flv_put_be16(c: *mut flv_buffer, val: uint16_t);
    fn flv_put_be24(c: *mut flv_buffer, val: uint32_t);
    fn flv_put_tag(c: *mut flv_buffer, tag: *const libc::c_char);
    fn flv_put_amf_string(c: *mut flv_buffer, str: *const libc::c_char);
    fn flv_put_amf_double(c: *mut flv_buffer, d: libc::c_double);
}
#[inline]
unsafe extern "C" fn x264_is_regular_file(mut filehandle: *mut FILE) -> libc::c_int {
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
    if fstat(fileno(filehandle), &mut file_stat) != 0 {
        return 1 as libc::c_int;
    }
    (file_stat.st_mode & 0o170000 as libc::c_int as __mode_t == 0o100000 as libc::c_int as __mode_t)
        as libc::c_int
}
#[inline(always)]
unsafe extern "C" fn endian_fix32(mut x: uint32_t) -> uint32_t {
    (x << 24 as libc::c_int)
        .wrapping_add((x << 8 as libc::c_int) & 0xff0000 as libc::c_int as uint32_t)
        .wrapping_add((x >> 8 as libc::c_int) & 0xff00 as libc::c_int as uint32_t)
        .wrapping_add(x >> 24 as libc::c_int)
}
#[inline(always)]
unsafe extern "C" fn endian_fix64(mut x: uint64_t) -> uint64_t {
    (endian_fix32((x >> 32 as libc::c_int) as uint32_t) as uint64_t)
        .wrapping_add((endian_fix32(x as uint32_t) as uint64_t) << 32 as libc::c_int)
}
unsafe extern "C" fn write_header(mut c: *mut flv_buffer) -> libc::c_int {
    flv_put_tag(c, b"FLV\0" as *const u8 as *const libc::c_char);
    flv_put_byte(c, 1 as libc::c_int as uint8_t);
    flv_put_byte(c, 1 as libc::c_int as uint8_t);
    flv_put_be32(c, 9 as libc::c_int as uint32_t);
    flv_put_be32(c, 0 as libc::c_int as uint32_t);
    flv_flush_data(c)
}
pub unsafe extern "C" fn open_file(
    mut psz_filename: *mut libc::c_char,
    mut p_handle: *mut hnd_t,
    mut opt: *mut cli_output_opt_t,
) -> libc::c_int {
    let mut p_flv: *mut flv_hnd_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<flv_hnd_t>() as libc::c_ulong,
    ) as *mut flv_hnd_t;
    if !p_flv.is_null() {
        let mut c: *mut flv_buffer = flv_create_writer(psz_filename);
        if !c.is_null() {
            if write_header(c) == 0 {
                (*p_flv).c = c;
                (*p_flv).b_dts_compress = (*opt).use_dts_compress;
                *p_handle = p_flv as hnd_t;
                return 0 as libc::c_int;
            }
            fclose((*c).fp);
            free((*c).data as *mut libc::c_void);
            free(c as *mut libc::c_void);
        }
        free(p_flv as *mut libc::c_void);
    }
    *p_handle = std::ptr::null_mut::<libc::c_void>();
    -(1 as libc::c_int)
}
pub unsafe extern "C" fn set_param(
    mut handle: hnd_t,
    mut p_param: *mut x264_param_t,
) -> libc::c_int {
    let mut p_flv: *mut flv_hnd_t = handle as *mut flv_hnd_t;
    let mut c: *mut flv_buffer = (*p_flv).c;
    flv_put_byte(c, FLV_TAG_TYPE_META as libc::c_int as uint8_t);
    let mut start: libc::c_int = (*c).d_cur as libc::c_int;
    flv_put_be24(c, 0 as libc::c_int as uint32_t);
    flv_put_be24(c, 0 as libc::c_int as uint32_t);
    flv_put_be32(c, 0 as libc::c_int as uint32_t);
    flv_put_byte(c, AMF_DATA_TYPE_STRING as libc::c_int as uint8_t);
    flv_put_amf_string(c, b"onMetaData\0" as *const u8 as *const libc::c_char);
    flv_put_byte(c, AMF_DATA_TYPE_MIXEDARRAY as libc::c_int as uint8_t);
    flv_put_be32(c, 7 as libc::c_int as uint32_t);
    flv_put_amf_string(c, b"width\0" as *const u8 as *const libc::c_char);
    flv_put_amf_double(c, (*p_param).i_width as libc::c_double);
    flv_put_amf_string(c, b"height\0" as *const u8 as *const libc::c_char);
    flv_put_amf_double(c, (*p_param).i_height as libc::c_double);
    flv_put_amf_string(c, b"framerate\0" as *const u8 as *const libc::c_char);
    if (*p_param).b_vfr_input == 0 {
        flv_put_amf_double(
            c,
            (*p_param).i_fps_num as libc::c_double / (*p_param).i_fps_den as libc::c_double,
        );
    } else {
        (*p_flv).i_framerate_pos = ((*c).d_cur as uint64_t)
            .wrapping_add((*c).d_total)
            .wrapping_add(1 as libc::c_int as uint64_t);
        flv_put_amf_double(c, 0 as libc::c_int as libc::c_double);
    }
    flv_put_amf_string(c, b"videocodecid\0" as *const u8 as *const libc::c_char);
    flv_put_amf_double(c, FLV_CODECID_H264 as libc::c_int as libc::c_double);
    flv_put_amf_string(c, b"duration\0" as *const u8 as *const libc::c_char);
    (*p_flv).i_duration_pos = ((*c).d_cur as uint64_t)
        .wrapping_add((*c).d_total)
        .wrapping_add(1 as libc::c_int as uint64_t);
    flv_put_amf_double(c, 0 as libc::c_int as libc::c_double);
    flv_put_amf_string(c, b"filesize\0" as *const u8 as *const libc::c_char);
    (*p_flv).i_filesize_pos = ((*c).d_cur as uint64_t)
        .wrapping_add((*c).d_total)
        .wrapping_add(1 as libc::c_int as uint64_t);
    flv_put_amf_double(c, 0 as libc::c_int as libc::c_double);
    flv_put_amf_string(c, b"videodatarate\0" as *const u8 as *const libc::c_char);
    (*p_flv).i_bitrate_pos = ((*c).d_cur as uint64_t)
        .wrapping_add((*c).d_total)
        .wrapping_add(1 as libc::c_int as uint64_t);
    flv_put_amf_double(c, 0 as libc::c_int as libc::c_double);
    flv_put_amf_string(c, b"\0" as *const u8 as *const libc::c_char);
    flv_put_byte(c, 0x9 as libc::c_int as uint8_t);
    let mut length: libc::c_uint = ((*c).d_cur).wrapping_sub(start as libc::c_uint);
    flv_rewrite_amf_be24(
        c,
        length.wrapping_sub(10 as libc::c_int as libc::c_uint),
        start as libc::c_uint,
    );
    flv_put_be32(c, length.wrapping_add(1 as libc::c_int as libc::c_uint));
    (*p_flv).i_fps_num = (*p_param).i_fps_num as int64_t;
    (*p_flv).i_fps_den = (*p_param).i_fps_den as int64_t;
    (*p_flv).d_timebase =
        (*p_param).i_timebase_num as libc::c_double / (*p_param).i_timebase_den as libc::c_double;
    (*p_flv).b_vfr_input = (*p_param).b_vfr_input;
    (*p_flv).i_delay_frames = if (*p_param).i_bframe != 0 {
        if (*p_param).i_bframe_pyramid != 0 {
            2 as libc::c_int
        } else {
            1 as libc::c_int
        }
    } else {
        0 as libc::c_int
    };
    0 as libc::c_int
}
pub unsafe extern "C" fn write_headers(
    mut handle: hnd_t,
    mut p_nal: *mut x264_nal_t,
) -> libc::c_int {
    let mut p_flv: *mut flv_hnd_t = handle as *mut flv_hnd_t;
    let mut c: *mut flv_buffer = (*p_flv).c;
    let mut sps_size: libc::c_int = (*p_nal.offset(0 as libc::c_int as isize)).i_payload;
    let mut pps_size: libc::c_int = (*p_nal.offset(1 as libc::c_int as isize)).i_payload;
    let mut sei_size: libc::c_int = (*p_nal.offset(2 as libc::c_int as isize)).i_payload;
    (*p_flv).sei = malloc(sei_size as libc::c_ulong) as *mut uint8_t;
    if ((*p_flv).sei).is_null() {
        return -(1 as libc::c_int);
    }
    (*p_flv).sei_len = sei_size;
    memcpy(
        (*p_flv).sei as *mut libc::c_void,
        (*p_nal.offset(2 as libc::c_int as isize)).p_payload as *const libc::c_void,
        sei_size as libc::c_ulong,
    );
    let mut sps: *mut uint8_t =
        ((*p_nal.offset(0 as libc::c_int as isize)).p_payload).offset(4 as libc::c_int as isize);
    flv_put_byte(c, FLV_TAG_TYPE_VIDEO as libc::c_int as uint8_t);
    flv_put_be24(c, 0 as libc::c_int as uint32_t);
    flv_put_be24(c, 0 as libc::c_int as uint32_t);
    flv_put_byte(c, 0 as libc::c_int as uint8_t);
    flv_put_be24(c, 0 as libc::c_int as uint32_t);
    (*p_flv).start = (*c).d_cur;
    flv_put_byte(
        c,
        (FLV_FRAME_KEY as libc::c_int | FLV_CODECID_H264 as libc::c_int) as uint8_t,
    );
    flv_put_byte(c, 0 as libc::c_int as uint8_t);
    flv_put_be24(c, 0 as libc::c_int as uint32_t);
    flv_put_byte(c, 1 as libc::c_int as uint8_t);
    flv_put_byte(c, *sps.offset(1 as libc::c_int as isize));
    flv_put_byte(c, *sps.offset(2 as libc::c_int as isize));
    flv_put_byte(c, *sps.offset(3 as libc::c_int as isize));
    flv_put_byte(c, 0xff as libc::c_int as uint8_t);
    flv_put_byte(c, 0xe1 as libc::c_int as uint8_t);
    flv_put_be16(c, (sps_size - 4 as libc::c_int) as uint16_t);
    flv_append_data(c, sps, (sps_size - 4 as libc::c_int) as libc::c_uint);
    flv_put_byte(c, 1 as libc::c_int as uint8_t);
    flv_put_be16(c, (pps_size - 4 as libc::c_int) as uint16_t);
    flv_append_data(
        c,
        ((*p_nal.offset(1 as libc::c_int as isize)).p_payload).offset(4 as libc::c_int as isize),
        (pps_size - 4 as libc::c_int) as libc::c_uint,
    );
    let mut length: libc::c_uint = ((*c).d_cur).wrapping_sub((*p_flv).start);
    flv_rewrite_amf_be24(
        c,
        length,
        ((*p_flv).start).wrapping_sub(10 as libc::c_int as libc::c_uint),
    );
    flv_put_be32(c, length.wrapping_add(11 as libc::c_int as libc::c_uint));
    if flv_flush_data(c) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    sei_size + sps_size + pps_size
}
pub unsafe extern "C" fn write_frame(
    mut handle: hnd_t,
    mut p_nalu: *mut uint8_t,
    mut i_size: libc::c_int,
    mut p_picture: *mut x264_picture_t,
) -> libc::c_int {
    let mut p_flv: *mut flv_hnd_t = handle as *mut flv_hnd_t;
    let mut c: *mut flv_buffer = (*p_flv).c;
    if (*p_flv).i_framenum == 0 {
        (*p_flv).i_delay_time = (*p_picture).i_dts * -(1 as libc::c_int) as int64_t;
        if (*p_flv).b_dts_compress == 0 && (*p_flv).i_delay_time != 0 {
            x264_cli_log(
                b"flv\0" as *const u8 as *const libc::c_char,
                2 as libc::c_int,
                b"initial delay %ld ms\n\0" as *const u8 as *const libc::c_char,
                (((*p_picture).i_pts + (*p_flv).i_delay_time) as libc::c_double
                    * (*p_flv).d_timebase
                    * 1000 as libc::c_int as libc::c_double
                    + 0.5f64) as int64_t,
            );
        }
    }
    let mut dts: int64_t = 0;
    let mut cts: int64_t = 0;
    let mut offset: int64_t = 0;
    if (*p_flv).b_dts_compress != 0 {
        if (*p_flv).i_framenum == 1 as libc::c_int as int64_t {
            (*p_flv).i_init_delta = (((*p_picture).i_dts + (*p_flv).i_delay_time) as libc::c_double
                * (*p_flv).d_timebase
                * 1000 as libc::c_int as libc::c_double
                + 0.5f64) as int64_t;
        }
        dts = if (*p_flv).i_framenum > (*p_flv).i_delay_frames as int64_t {
            ((*p_picture).i_dts as libc::c_double
                * (*p_flv).d_timebase
                * 1000 as libc::c_int as libc::c_double
                + 0.5f64) as int64_t
        } else {
            (*p_flv).i_framenum * (*p_flv).i_init_delta
                / ((*p_flv).i_delay_frames + 1 as libc::c_int) as int64_t
        };
        cts = ((*p_picture).i_pts as libc::c_double
            * (*p_flv).d_timebase
            * 1000 as libc::c_int as libc::c_double
            + 0.5f64) as int64_t;
    } else {
        dts = (((*p_picture).i_dts + (*p_flv).i_delay_time) as libc::c_double
            * (*p_flv).d_timebase
            * 1000 as libc::c_int as libc::c_double
            + 0.5f64) as int64_t;
        cts = (((*p_picture).i_pts + (*p_flv).i_delay_time) as libc::c_double
            * (*p_flv).d_timebase
            * 1000 as libc::c_int as libc::c_double
            + 0.5f64) as int64_t;
    }
    offset = cts - dts;
    if (*p_flv).i_framenum != 0 {
        if (*p_flv).i_prev_dts == dts {
            x264_cli_log(
                b"flv\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int,
                b"duplicate DTS %ld generated by rounding\n               decoding framerate cannot exceed 1000fps\n\0"
                    as *const u8 as *const libc::c_char,
                dts,
            );
        }
        if (*p_flv).i_prev_cts == cts {
            x264_cli_log(
                b"flv\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int,
                b"duplicate CTS %ld generated by rounding\n               composition framerate cannot exceed 1000fps\n\0"
                    as *const u8 as *const libc::c_char,
                cts,
            );
        }
    }
    (*p_flv).i_prev_dts = dts;
    (*p_flv).i_prev_cts = cts;
    flv_put_byte(c, FLV_TAG_TYPE_VIDEO as libc::c_int as uint8_t);
    flv_put_be24(c, 0 as libc::c_int as uint32_t);
    flv_put_be24(c, dts as uint32_t);
    flv_put_byte(c, (dts >> 24 as libc::c_int) as uint8_t);
    flv_put_be24(c, 0 as libc::c_int as uint32_t);
    (*p_flv).start = (*c).d_cur;
    flv_put_byte(
        c,
        ((if (*p_picture).b_keyframe != 0 {
            FLV_FRAME_KEY as libc::c_int
        } else {
            FLV_FRAME_INTER as libc::c_int
        }) | FLV_CODECID_H264 as libc::c_int) as uint8_t,
    );
    flv_put_byte(c, 1 as libc::c_int as uint8_t);
    flv_put_be24(c, offset as uint32_t);
    if !((*p_flv).sei).is_null() {
        flv_append_data(c, (*p_flv).sei, (*p_flv).sei_len as libc::c_uint);
        free((*p_flv).sei as *mut libc::c_void);
        (*p_flv).sei = std::ptr::null_mut::<uint8_t>();
    }
    flv_append_data(c, p_nalu, i_size as libc::c_uint);
    let mut length: libc::c_uint = ((*c).d_cur).wrapping_sub((*p_flv).start);
    flv_rewrite_amf_be24(
        c,
        length,
        ((*p_flv).start).wrapping_sub(10 as libc::c_int as libc::c_uint),
    );
    flv_put_be32(c, (11 as libc::c_int as libc::c_uint).wrapping_add(length));
    if flv_flush_data(c) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    (*p_flv).i_framenum += 1;
    (*p_flv).i_framenum;
    i_size
}
unsafe extern "C" fn rewrite_amf_double(
    mut fp: *mut FILE,
    mut position: uint64_t,
    mut value: libc::c_double,
) -> libc::c_int {
    let mut x: uint64_t = endian_fix64(flv_dbl2int(value));
    if fseeko(fp, position as __off64_t, 0 as libc::c_int) == 0
        && fwrite(
            &mut x as *mut uint64_t as *const libc::c_void,
            8 as libc::c_int as libc::c_ulong,
            1 as libc::c_int as libc::c_ulong,
            fp,
        ) == 1 as libc::c_int as libc::c_ulong
    {
        0 as libc::c_int
    } else {
        -(1 as libc::c_int)
    }
}
pub unsafe extern "C" fn close_file(
    mut handle: hnd_t,
    mut largest_pts: int64_t,
    mut second_largest_pts: int64_t,
) -> libc::c_int {
    let mut total_duration: libc::c_double = 0.;
    let mut current_block: u64;
    let mut ret: libc::c_int = -(1 as libc::c_int);
    let mut p_flv: *mut flv_hnd_t = handle as *mut flv_hnd_t;
    let mut c: *mut flv_buffer = (*p_flv).c;
    if flv_flush_data(c) >= 0 as libc::c_int {
        total_duration = 0.;
        if (*p_flv).i_framenum == 1 as libc::c_int as int64_t {
            total_duration = if (*p_flv).i_fps_num != 0 {
                (*p_flv).i_fps_den as libc::c_double / (*p_flv).i_fps_num as libc::c_double
            } else {
                0 as libc::c_int as libc::c_double
            };
        } else {
            total_duration = (2 as libc::c_int as int64_t * largest_pts - second_largest_pts)
                as libc::c_double
                * (*p_flv).d_timebase;
        }
        if x264_is_regular_file((*c).fp) != 0 && total_duration > 0 as libc::c_int as libc::c_double
        {
            let mut framerate: libc::c_double = 0.;
            let mut filesize: int64_t = ftello((*c).fp);
            if (*p_flv).i_framerate_pos != 0 {
                framerate = (*p_flv).i_framenum as libc::c_double / total_duration;
                if rewrite_amf_double((*c).fp, (*p_flv).i_framerate_pos, framerate)
                    < 0 as libc::c_int
                {
                    current_block = 4818846981140956894;
                } else {
                    current_block = 13586036798005543211;
                }
            } else {
                current_block = 13586036798005543211;
            }
            match current_block {
                4818846981140956894 => {}
                _ => {
                    if rewrite_amf_double((*c).fp, (*p_flv).i_duration_pos, total_duration)
                        < 0 as libc::c_int
                    {
                        current_block = 4818846981140956894;
                    } else if rewrite_amf_double(
                        (*c).fp,
                        (*p_flv).i_filesize_pos,
                        filesize as libc::c_double,
                    ) < 0 as libc::c_int
                    {
                        current_block = 4818846981140956894;
                    } else if rewrite_amf_double(
                        (*c).fp,
                        (*p_flv).i_bitrate_pos,
                        filesize as libc::c_double * 8.0f64
                            / (total_duration * 1000 as libc::c_int as libc::c_double),
                    ) < 0 as libc::c_int
                    {
                        current_block = 4818846981140956894;
                    } else {
                        current_block = 224731115979188411;
                    }
                }
            }
        } else {
            current_block = 224731115979188411;
        }
        match current_block {
            4818846981140956894 => {}
            _ => {
                ret = 0 as libc::c_int;
            }
        }
    }
    fclose((*c).fp);
    free((*c).data as *mut libc::c_void);
    free(c as *mut libc::c_void);
    free(p_flv as *mut libc::c_void);
    ret
}
