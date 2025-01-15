use crate::types::*;

#[no_mangle]
pub unsafe extern "C" fn x264_8_cavlc_init(mut h: *mut x264_t) {
    let mut i_suffix: libc::c_int = 0 as libc::c_int;
    while i_suffix < 7 as libc::c_int {
        let mut level: int16_t = (-(128 as libc::c_int) / 2 as libc::c_int) as int16_t;
        while (level as libc::c_int) < 128 as libc::c_int / 2 as libc::c_int {
            let mut mask: libc::c_int = level as libc::c_int >> 15 as libc::c_int;
            let mut abs_level: libc::c_int = (level as libc::c_int ^ mask) - mask;
            let mut i_level_code: libc::c_int = if abs_level != 0 {
                abs_level * 2 as libc::c_int - mask - 2 as libc::c_int
            } else {
                0 as libc::c_int
            };
            let mut i_next: libc::c_int = i_suffix;
            let mut vlc: *mut vlc_large_t =
                &mut *(*x264_8_level_token.as_mut_ptr().offset(i_suffix as isize))
                    .as_mut_ptr()
                    .offset((level as libc::c_int + 128 as libc::c_int / 2 as libc::c_int) as isize)
                    as *mut vlc_large_t;
            if i_level_code >> i_suffix < 14 as libc::c_int {
                (*vlc).i_size =
                    ((i_level_code >> i_suffix) + 1 as libc::c_int + i_suffix) as uint8_t;
                (*vlc).i_bits = (((1 as libc::c_int) << i_suffix)
                    + (i_level_code & (((1 as libc::c_int) << i_suffix) - 1 as libc::c_int)))
                    as uint16_t;
            } else if i_suffix == 0 as libc::c_int && i_level_code < 30 as libc::c_int {
                (*vlc).i_size = 19 as libc::c_int as uint8_t;
                (*vlc).i_bits = (((1 as libc::c_int) << 4 as libc::c_int)
                    + (i_level_code - 14 as libc::c_int))
                    as uint16_t;
            } else if i_suffix > 0 as libc::c_int && i_level_code >> i_suffix == 14 as libc::c_int {
                (*vlc).i_size = (15 as libc::c_int + i_suffix) as uint8_t;
                (*vlc).i_bits = (((1 as libc::c_int) << i_suffix)
                    + (i_level_code & (((1 as libc::c_int) << i_suffix) - 1 as libc::c_int)))
                    as uint16_t;
            } else {
                i_level_code -= (15 as libc::c_int) << i_suffix;
                if i_suffix == 0 as libc::c_int {
                    i_level_code -= 15 as libc::c_int;
                }
                (*vlc).i_size = 28 as libc::c_int as uint8_t;
                (*vlc).i_bits =
                    (((1 as libc::c_int) << 12 as libc::c_int) + i_level_code) as uint16_t;
            }
            if i_next == 0 as libc::c_int {
                i_next += 1;
            }
            if abs_level > (3 as libc::c_int) << (i_next - 1 as libc::c_int)
                && i_next < 6 as libc::c_int
            {
                i_next += 1;
            }
            (*vlc).i_next = i_next as uint8_t;
            level += 1;
        }
        i_suffix += 1;
    }
    x264_8_run_before[0 as libc::c_int as usize] = 0 as libc::c_int as uint32_t;
    x264_8_run_before[1 as libc::c_int as usize] = 0 as libc::c_int as uint32_t;
    let mut i: uint32_t = 2 as libc::c_int as uint32_t;
    while i < ((1 as libc::c_int) << 16 as libc::c_int) as uint32_t {
        let mut runlevel: x264_run_level_t = x264_run_level_t {
            last: 0,
            mask: 0,
            level: [0; 18],
        };
        let mut dct: [dctcoef; 16] = [0; 16];
        let mut size: libc::c_int = 0 as libc::c_int;
        let mut bits: libc::c_int = 0 as libc::c_int;
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < 16 as libc::c_int {
            dct[j as usize] = (i & ((1 as libc::c_int) << j) as uint32_t) as dctcoef;
            j += 1;
            j;
        }
        let mut total: libc::c_int =
            ((*h).quantf.coeff_level_run[DCT_LUMA_4x4 as libc::c_int as usize])
                .expect("non-null function pointer")(dct.as_mut_ptr(), &mut runlevel);
        let mut zeros: libc::c_int = runlevel.last + 1 as libc::c_int - total;
        let mut mask_0: uint32_t = i << (i.leading_zeros() as i32 + 1 as libc::c_int);
        let mut j_0: libc::c_int = 0 as libc::c_int;
        while j_0 < total - 1 as libc::c_int && zeros > 0 as libc::c_int {
            let mut idx: libc::c_int = (if zeros < 7 as libc::c_int {
                zeros
            } else {
                7 as libc::c_int
            }) - 1 as libc::c_int;
            let mut run: libc::c_int = mask_0.leading_zeros() as i32;
            let mut len: libc::c_int =
                x264_run_before_init[idx as usize][run as usize].i_size as libc::c_int;
            size += len;
            bits <<= len;
            bits |= x264_run_before_init[idx as usize][run as usize].i_bits as libc::c_int;
            zeros -= run;
            mask_0 <<= run + 1 as libc::c_int;
            j_0 += 1;
        }
        x264_8_run_before[i as usize] = ((bits << 5 as libc::c_int) + size) as uint32_t;
        i = i.wrapping_add(1);
    }
}
