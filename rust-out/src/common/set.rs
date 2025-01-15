use crate::types::*;
extern "C" {
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strpbrk(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn x264_malloc(_: int64_t) -> *mut libc::c_void;
    fn x264_free(_: *mut libc::c_void);
    fn x264_slurp_file(filename: *const libc::c_char) -> *mut libc::c_char;
    fn x264_8_log(h: *mut x264_t, i_level: libc::c_int, psz_fmt: *const libc::c_char, _: ...);
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_cqm_init(mut h: *mut x264_t) -> libc::c_int {
    let mut current_block: u64;
    let mut def_quant4: [[libc::c_int; 16]; 6] = [[0; 16]; 6];
    let mut def_quant8: [[libc::c_int; 64]; 6] = [[0; 64]; 6];
    let mut def_dequant4: [[libc::c_int; 16]; 6] = [[0; 16]; 6];
    let mut def_dequant8: [[libc::c_int; 64]; 6] = [[0; 64]; 6];
    let mut quant4_mf: [[[libc::c_int; 16]; 6]; 4] = [[[0; 16]; 6]; 4];
    let mut quant8_mf: [[[libc::c_int; 64]; 6]; 4] = [[[0; 64]; 6]; 4];
    let mut deadzone: [libc::c_int; 4] = [
        32 as libc::c_int - (*h).param.analyse.i_luma_deadzone[1 as libc::c_int as usize],
        32 as libc::c_int - (*h).param.analyse.i_luma_deadzone[0 as libc::c_int as usize],
        32 as libc::c_int - 11 as libc::c_int,
        32 as libc::c_int - 21 as libc::c_int,
    ];
    let mut max_qp_err: libc::c_int = -(1 as libc::c_int);
    let mut max_chroma_qp_err: libc::c_int = -(1 as libc::c_int);
    let mut min_qp_err: libc::c_int = 51 as libc::c_int
        + 6 as libc::c_int * (8 as libc::c_int - 8 as libc::c_int)
        + 18 as libc::c_int
        + 1 as libc::c_int;
    let mut num_8x8_lists: libc::c_int =
        if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as libc::c_int {
            4 as libc::c_int
        } else if (*h).param.analyse.b_transform_8x8 != 0 {
            2 as libc::c_int
        } else {
            0 as libc::c_int
        };
    let mut i: libc::c_int = 0 as libc::c_int;
    loop {
        if i >= 4 as libc::c_int {
            current_block = 5529461102203738653;
            break;
        }
        let mut size: libc::c_int = 4 as libc::c_int * 4 as libc::c_int;
        let mut start: libc::c_int = if 4 as libc::c_int == 8 as libc::c_int {
            4 as libc::c_int
        } else {
            0 as libc::c_int
        };
        let mut j: libc::c_int = 0;
        j = 0 as libc::c_int;
        while j < i {
            if memcmp(
                (*((*h).sps).as_mut_ptr()).scaling_list[(i + start) as usize]
                    as *const libc::c_void,
                (*((*h).sps).as_mut_ptr()).scaling_list[(j + start) as usize]
                    as *const libc::c_void,
                (size as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<uint8_t>() as libc::c_ulong),
            ) == 0
            {
                break;
            }
            j += 1;
        }
        if j < i {
            (*h).quant4_mf[i as usize] = (*h).quant4_mf[j as usize];
            (*h).dequant4_mf[i as usize] = (*h).dequant4_mf[j as usize];
            (*h).unquant4_mf[i as usize] = (*h).unquant4_mf[j as usize];
        } else {
            (*h).quant4_mf[i as usize] = x264_malloc(
                (((51 as libc::c_int
                    + 6 as libc::c_int * (8 as libc::c_int - 8 as libc::c_int)
                    + 1 as libc::c_int)
                    * size) as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<udctcoef>() as libc::c_ulong)
                    as int64_t,
            ) as *mut [udctcoef; 16];
            if ((*h).quant4_mf[i as usize]).is_null() {
                current_block = 9316118096792316650;
                break;
            }
            (*h).dequant4_mf[i as usize] = x264_malloc(
                ((6 as libc::c_int * size) as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                    as int64_t,
            ) as *mut [libc::c_int; 16];
            if ((*h).dequant4_mf[i as usize]).is_null() {
                current_block = 9316118096792316650;
                break;
            }
            (*h).unquant4_mf[i as usize] = x264_malloc(
                (((51 as libc::c_int
                    + 6 as libc::c_int * (8 as libc::c_int - 8 as libc::c_int)
                    + 1 as libc::c_int)
                    * size) as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                    as int64_t,
            ) as *mut [libc::c_int; 16];
            if ((*h).unquant4_mf[i as usize]).is_null() {
                current_block = 9316118096792316650;
                break;
            }
        }
        j = 0 as libc::c_int;
        while j < i {
            if deadzone[j as usize] == deadzone[i as usize]
                && memcmp(
                    (*((*h).sps).as_mut_ptr()).scaling_list[(i + start) as usize]
                        as *const libc::c_void,
                    (*((*h).sps).as_mut_ptr()).scaling_list[(j + start) as usize]
                        as *const libc::c_void,
                    (size as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<uint8_t>() as libc::c_ulong),
                ) == 0
            {
                break;
            }
            j += 1;
        }
        if j < i {
            (*h).quant4_bias[i as usize] = (*h).quant4_bias[j as usize];
            (*h).quant4_bias0[i as usize] = (*h).quant4_bias0[j as usize];
        } else {
            (*h).quant4_bias[i as usize] = x264_malloc(
                (((51 as libc::c_int
                    + 6 as libc::c_int * (8 as libc::c_int - 8 as libc::c_int)
                    + 1 as libc::c_int)
                    * size) as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<udctcoef>() as libc::c_ulong)
                    as int64_t,
            ) as *mut [udctcoef; 16];
            if ((*h).quant4_bias[i as usize]).is_null() {
                current_block = 9316118096792316650;
                break;
            }
            (*h).quant4_bias0[i as usize] = x264_malloc(
                (((51 as libc::c_int
                    + 6 as libc::c_int * (8 as libc::c_int - 8 as libc::c_int)
                    + 1 as libc::c_int)
                    * size) as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<udctcoef>() as libc::c_ulong)
                    as int64_t,
            ) as *mut [udctcoef; 16];
            if ((*h).quant4_bias0[i as usize]).is_null() {
                current_block = 9316118096792316650;
                break;
            }
        }
        i += 1;
    }
    if current_block == 5529461102203738653 {
        let mut i_0: libc::c_int = 0 as libc::c_int;
        loop {
            if i_0 >= num_8x8_lists {
                current_block = 7419121793134201633;
                break;
            }
            let mut size_0: libc::c_int = 8 as libc::c_int * 8 as libc::c_int;
            let mut start_0: libc::c_int = if 8 as libc::c_int == 8 as libc::c_int {
                4 as libc::c_int
            } else {
                0 as libc::c_int
            };
            let mut j_0: libc::c_int = 0;
            j_0 = 0 as libc::c_int;
            while j_0 < i_0 {
                if memcmp(
                    (*((*h).sps).as_mut_ptr()).scaling_list[(i_0 + start_0) as usize]
                        as *const libc::c_void,
                    (*((*h).sps).as_mut_ptr()).scaling_list[(j_0 + start_0) as usize]
                        as *const libc::c_void,
                    (size_0 as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<uint8_t>() as libc::c_ulong),
                ) == 0
                {
                    break;
                }
                j_0 += 1;
            }
            if j_0 < i_0 {
                (*h).quant8_mf[i_0 as usize] = (*h).quant8_mf[j_0 as usize];
                (*h).dequant8_mf[i_0 as usize] = (*h).dequant8_mf[j_0 as usize];
                (*h).unquant8_mf[i_0 as usize] = (*h).unquant8_mf[j_0 as usize];
            } else {
                (*h).quant8_mf[i_0 as usize] = x264_malloc(
                    (((51 as libc::c_int
                        + 6 as libc::c_int * (8 as libc::c_int - 8 as libc::c_int)
                        + 1 as libc::c_int)
                        * size_0) as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<udctcoef>() as libc::c_ulong)
                        as int64_t,
                ) as *mut [udctcoef; 64];
                if ((*h).quant8_mf[i_0 as usize]).is_null() {
                    current_block = 9316118096792316650;
                    break;
                }
                (*h).dequant8_mf[i_0 as usize] = x264_malloc(
                    ((6 as libc::c_int * size_0) as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                        as int64_t,
                ) as *mut [libc::c_int; 64];
                if ((*h).dequant8_mf[i_0 as usize]).is_null() {
                    current_block = 9316118096792316650;
                    break;
                }
                (*h).unquant8_mf[i_0 as usize] = x264_malloc(
                    (((51 as libc::c_int
                        + 6 as libc::c_int * (8 as libc::c_int - 8 as libc::c_int)
                        + 1 as libc::c_int)
                        * size_0) as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                        as int64_t,
                ) as *mut [libc::c_int; 64];
                if ((*h).unquant8_mf[i_0 as usize]).is_null() {
                    current_block = 9316118096792316650;
                    break;
                }
            }
            j_0 = 0 as libc::c_int;
            while j_0 < i_0 {
                if deadzone[j_0 as usize] == deadzone[i_0 as usize]
                    && memcmp(
                        (*((*h).sps).as_mut_ptr()).scaling_list[(i_0 + start_0) as usize]
                            as *const libc::c_void,
                        (*((*h).sps).as_mut_ptr()).scaling_list[(j_0 + start_0) as usize]
                            as *const libc::c_void,
                        (size_0 as libc::c_ulong)
                            .wrapping_mul(::core::mem::size_of::<uint8_t>() as libc::c_ulong),
                    ) == 0
                {
                    break;
                }
                j_0 += 1;
            }
            if j_0 < i_0 {
                (*h).quant8_bias[i_0 as usize] = (*h).quant8_bias[j_0 as usize];
                (*h).quant8_bias0[i_0 as usize] = (*h).quant8_bias0[j_0 as usize];
            } else {
                (*h).quant8_bias[i_0 as usize] = x264_malloc(
                    (((51 as libc::c_int
                        + 6 as libc::c_int * (8 as libc::c_int - 8 as libc::c_int)
                        + 1 as libc::c_int)
                        * size_0) as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<udctcoef>() as libc::c_ulong)
                        as int64_t,
                ) as *mut [udctcoef; 64];
                if ((*h).quant8_bias[i_0 as usize]).is_null() {
                    current_block = 9316118096792316650;
                    break;
                }
                (*h).quant8_bias0[i_0 as usize] = x264_malloc(
                    (((51 as libc::c_int
                        + 6 as libc::c_int * (8 as libc::c_int - 8 as libc::c_int)
                        + 1 as libc::c_int)
                        * size_0) as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<udctcoef>() as libc::c_ulong)
                        as int64_t,
                ) as *mut [udctcoef; 64];
                if ((*h).quant8_bias0[i_0 as usize]).is_null() {
                    current_block = 9316118096792316650;
                    break;
                }
            }
            i_0 += 1;
        }
        match current_block {
            9316118096792316650 => {}
            _ => {
                let mut q: libc::c_int = 0 as libc::c_int;
                while q < 6 as libc::c_int {
                    let mut i_1: libc::c_int = 0 as libc::c_int;
                    while i_1 < 16 as libc::c_int {
                        let mut j_1: libc::c_int = (i_1 & 1 as libc::c_int)
                            + ((i_1 >> 2 as libc::c_int) & 1 as libc::c_int);
                        def_dequant4[q as usize][i_1 as usize] =
                            dequant4_scale[q as usize][j_1 as usize] as libc::c_int;
                        def_quant4[q as usize][i_1 as usize] =
                            quant4_scale[q as usize][j_1 as usize] as libc::c_int;
                        i_1 += 1;
                    }
                    let mut i_2: libc::c_int = 0 as libc::c_int;
                    while i_2 < 64 as libc::c_int {
                        let mut j_2: libc::c_int =
                            quant8_scan[((i_2 >> 1 as libc::c_int) & 12 as libc::c_int
                                | i_2 & 3 as libc::c_int)
                                as usize] as libc::c_int;
                        def_dequant8[q as usize][i_2 as usize] =
                            dequant8_scale[q as usize][j_2 as usize] as libc::c_int;
                        def_quant8[q as usize][i_2 as usize] =
                            quant8_scale[q as usize][j_2 as usize] as libc::c_int;
                        i_2 += 1;
                    }
                    q += 1;
                }
                let mut q_0: libc::c_int = 0 as libc::c_int;
                while q_0 < 6 as libc::c_int {
                    let mut i_list: libc::c_int = 0 as libc::c_int;
                    while i_list < 4 as libc::c_int {
                        let mut i_3: libc::c_int = 0 as libc::c_int;
                        while i_3 < 16 as libc::c_int {
                            (*((*h).dequant4_mf[i_list as usize]).offset(q_0 as isize))
                                [i_3 as usize] = def_dequant4[q_0 as usize][i_3 as usize]
                                * *((*((*h).sps).as_mut_ptr()).scaling_list[i_list as usize])
                                    .offset(i_3 as isize)
                                    as libc::c_int;
                            quant4_mf[i_list as usize][q_0 as usize][i_3 as usize] =
                                (def_quant4[q_0 as usize][i_3 as usize] * 16 as libc::c_int
                                    + (*((*((*h).sps).as_mut_ptr()).scaling_list[i_list as usize])
                                        .offset(i_3 as isize)
                                        as libc::c_int
                                        >> 1 as libc::c_int))
                                    / *((*((*h).sps).as_mut_ptr()).scaling_list[i_list as usize])
                                        .offset(i_3 as isize)
                                        as libc::c_int;
                            i_3 += 1;
                        }
                        i_list += 1;
                    }
                    let mut i_list_0: libc::c_int = 0 as libc::c_int;
                    while i_list_0 < num_8x8_lists {
                        let mut i_4: libc::c_int = 0 as libc::c_int;
                        while i_4 < 64 as libc::c_int {
                            (*((*h).dequant8_mf[i_list_0 as usize]).offset(q_0 as isize))
                                [i_4 as usize] = def_dequant8[q_0 as usize][i_4 as usize]
                                * *((*((*h).sps).as_mut_ptr()).scaling_list
                                    [(4 as libc::c_int + i_list_0) as usize])
                                    .offset(i_4 as isize)
                                    as libc::c_int;
                            quant8_mf[i_list_0 as usize][q_0 as usize][i_4 as usize] =
                                (def_quant8[q_0 as usize][i_4 as usize] * 16 as libc::c_int
                                    + (*((*((*h).sps).as_mut_ptr()).scaling_list
                                        [(4 as libc::c_int + i_list_0) as usize])
                                        .offset(i_4 as isize)
                                        as libc::c_int
                                        >> 1 as libc::c_int))
                                    / *((*((*h).sps).as_mut_ptr()).scaling_list
                                        [(4 as libc::c_int + i_list_0) as usize])
                                        .offset(i_4 as isize)
                                        as libc::c_int;
                            i_4 += 1;
                        }
                        i_list_0 += 1;
                    }
                    q_0 += 1;
                }
                let mut q_1: libc::c_int = 0 as libc::c_int;
                while q_1
                    <= 51 as libc::c_int + 6 as libc::c_int * (8 as libc::c_int - 8 as libc::c_int)
                {
                    let mut j_3: libc::c_int = 0;
                    let mut i_list_1: libc::c_int = 0 as libc::c_int;
                    while i_list_1 < 4 as libc::c_int {
                        let mut i_5: libc::c_int = 0 as libc::c_int;
                        while i_5 < 16 as libc::c_int {
                            (*((*h).unquant4_mf[i_list_1 as usize]).offset(q_1 as isize))
                                [i_5 as usize] = ((1 as libc::c_ulonglong)
                                << (q_1 / 6 as libc::c_int + 15 as libc::c_int + 8 as libc::c_int))
                                .wrapping_div(
                                    quant4_mf[i_list_1 as usize][(q_1 % 6 as libc::c_int) as usize]
                                        [i_5 as usize]
                                        as libc::c_ulonglong,
                                ) as libc::c_int;
                            j_3 = if q_1 / 6 as libc::c_int - 1 as libc::c_int <= 0 as libc::c_int {
                                quant4_mf[i_list_1 as usize][(q_1 % 6 as libc::c_int) as usize]
                                    [i_5 as usize]
                                    << -(q_1 / 6 as libc::c_int - 1 as libc::c_int)
                            } else {
                                (quant4_mf[i_list_1 as usize][(q_1 % 6 as libc::c_int) as usize]
                                    [i_5 as usize]
                                    + ((1 as libc::c_int)
                                        << (q_1 / 6 as libc::c_int
                                            - 1 as libc::c_int
                                            - 1 as libc::c_int)))
                                    >> (q_1 / 6 as libc::c_int - 1 as libc::c_int)
                            };
                            (*((*h).quant4_mf[i_list_1 as usize]).offset(q_1 as isize))
                                [i_5 as usize] = j_3 as uint16_t;
                            if j_3 == 0 {
                                min_qp_err = if min_qp_err < q_1 { min_qp_err } else { q_1 };
                            } else {
                                (*((*h).quant4_bias[i_list_1 as usize]).offset(q_1 as isize))
                                    [i_5 as usize] = (if ((deadzone[i_list_1 as usize]
                                    << 10 as libc::c_int)
                                    + (j_3 >> 1 as libc::c_int))
                                    / j_3
                                    < ((1 as libc::c_int) << 15 as libc::c_int) / j_3
                                {
                                    ((deadzone[i_list_1 as usize] << 10 as libc::c_int)
                                        + (j_3 >> 1 as libc::c_int))
                                        / j_3
                                } else {
                                    ((1 as libc::c_int) << 15 as libc::c_int) / j_3
                                }) as udctcoef;
                                (*((*h).quant4_bias0[i_list_1 as usize]).offset(q_1 as isize))
                                    [i_5 as usize] =
                                    (((1 as libc::c_int) << 15 as libc::c_int) / j_3) as udctcoef;
                                if j_3
                                    > (if (0xffff as libc::c_int)
                                        < ((1 as libc::c_int)
                                            << (25 as libc::c_int - 8 as libc::c_int))
                                            - 1 as libc::c_int
                                    {
                                        0xffff as libc::c_int
                                    } else {
                                        ((1 as libc::c_int)
                                            << (25 as libc::c_int - 8 as libc::c_int))
                                            - 1 as libc::c_int
                                    })
                                    && q_1 > max_qp_err
                                    && (i_list_1 == CQM_4IY as libc::c_int
                                        || i_list_1 == CQM_4PY as libc::c_int)
                                {
                                    max_qp_err = q_1;
                                }
                                if j_3
                                    > (if (0xffff as libc::c_int)
                                        < ((1 as libc::c_int)
                                            << (25 as libc::c_int - 8 as libc::c_int))
                                            - 1 as libc::c_int
                                    {
                                        0xffff as libc::c_int
                                    } else {
                                        ((1 as libc::c_int)
                                            << (25 as libc::c_int - 8 as libc::c_int))
                                            - 1 as libc::c_int
                                    })
                                    && q_1 > max_chroma_qp_err
                                    && (i_list_1 == CQM_4IC as libc::c_int
                                        || i_list_1 == CQM_4PC as libc::c_int)
                                {
                                    max_chroma_qp_err = q_1;
                                }
                            }
                            i_5 += 1;
                        }
                        i_list_1 += 1;
                    }
                    if (*h).param.analyse.b_transform_8x8 != 0 {
                        let mut i_list_2: libc::c_int = 0 as libc::c_int;
                        while i_list_2 < num_8x8_lists {
                            let mut i_6: libc::c_int = 0 as libc::c_int;
                            while i_6 < 64 as libc::c_int {
                                (*((*h).unquant8_mf[i_list_2 as usize]).offset(q_1 as isize))
                                    [i_6 as usize] = ((1 as libc::c_ulonglong)
                                    << (q_1 / 6 as libc::c_int
                                        + 16 as libc::c_int
                                        + 8 as libc::c_int))
                                    .wrapping_div(
                                        quant8_mf[i_list_2 as usize]
                                            [(q_1 % 6 as libc::c_int) as usize]
                                            [i_6 as usize]
                                            as libc::c_ulonglong,
                                    )
                                    as libc::c_int;
                                j_3 = if q_1 / 6 as libc::c_int <= 0 as libc::c_int {
                                    quant8_mf[i_list_2 as usize][(q_1 % 6 as libc::c_int) as usize]
                                        [i_6 as usize]
                                        << -(q_1 / 6 as libc::c_int)
                                } else {
                                    (quant8_mf[i_list_2 as usize]
                                        [(q_1 % 6 as libc::c_int) as usize]
                                        [i_6 as usize]
                                        + ((1 as libc::c_int)
                                            << (q_1 / 6 as libc::c_int - 1 as libc::c_int)))
                                        >> (q_1 / 6 as libc::c_int)
                                };
                                (*((*h).quant8_mf[i_list_2 as usize]).offset(q_1 as isize))
                                    [i_6 as usize] = j_3 as uint16_t;
                                if j_3 == 0 {
                                    min_qp_err = if min_qp_err < q_1 { min_qp_err } else { q_1 };
                                } else {
                                    (*((*h).quant8_bias[i_list_2 as usize]).offset(q_1 as isize))
                                        [i_6 as usize] =
                                        (if ((deadzone[i_list_2 as usize] << 10 as libc::c_int)
                                            + (j_3 >> 1 as libc::c_int))
                                            / j_3
                                            < ((1 as libc::c_int) << 15 as libc::c_int) / j_3
                                        {
                                            ((deadzone[i_list_2 as usize] << 10 as libc::c_int)
                                                + (j_3 >> 1 as libc::c_int))
                                                / j_3
                                        } else {
                                            ((1 as libc::c_int) << 15 as libc::c_int) / j_3
                                        }) as udctcoef;
                                    (*((*h).quant8_bias0[i_list_2 as usize])
                                        .offset(q_1 as isize))
                                        [i_6 as usize] = (((1 as libc::c_int) << 15 as libc::c_int)
                                        / j_3)
                                        as udctcoef;
                                    if j_3
                                        > (if (0xffff as libc::c_int)
                                            < ((1 as libc::c_int)
                                                << (25 as libc::c_int - 8 as libc::c_int))
                                                - 1 as libc::c_int
                                        {
                                            0xffff as libc::c_int
                                        } else {
                                            ((1 as libc::c_int)
                                                << (25 as libc::c_int - 8 as libc::c_int))
                                                - 1 as libc::c_int
                                        })
                                        && q_1 > max_qp_err
                                        && (i_list_2 == CQM_8IY as libc::c_int
                                            || i_list_2 == CQM_8PY as libc::c_int)
                                    {
                                        max_qp_err = q_1;
                                    }
                                    if j_3
                                        > (if (0xffff as libc::c_int)
                                            < ((1 as libc::c_int)
                                                << (25 as libc::c_int - 8 as libc::c_int))
                                                - 1 as libc::c_int
                                        {
                                            0xffff as libc::c_int
                                        } else {
                                            ((1 as libc::c_int)
                                                << (25 as libc::c_int - 8 as libc::c_int))
                                                - 1 as libc::c_int
                                        })
                                        && q_1 > max_chroma_qp_err
                                        && (i_list_2 == CQM_8IC as libc::c_int
                                            || i_list_2 == CQM_8PC as libc::c_int)
                                    {
                                        max_chroma_qp_err = q_1;
                                    }
                                }
                                i_6 += 1;
                            }
                            i_list_2 += 1;
                        }
                    }
                    q_1 += 1;
                }
                (*h).nr_offset_emergency = x264_malloc(
                    (::core::mem::size_of::<[[udctcoef; 64]; 4]>() as libc::c_ulong).wrapping_mul(
                        (51 as libc::c_int
                            + 6 as libc::c_int * (8 as libc::c_int - 8 as libc::c_int)
                            + 18 as libc::c_int
                            - (51 as libc::c_int
                                + 6 as libc::c_int * (8 as libc::c_int - 8 as libc::c_int)))
                            as libc::c_ulong,
                    ) as int64_t,
                ) as *mut [[udctcoef; 64]; 4];
                if !((*h).nr_offset_emergency).is_null() {
                    let mut q_2: libc::c_int = 0 as libc::c_int;
                    while q_2
                        < 51 as libc::c_int
                            + 6 as libc::c_int * (8 as libc::c_int - 8 as libc::c_int)
                            + 18 as libc::c_int
                            - (51 as libc::c_int
                                + 6 as libc::c_int * (8 as libc::c_int - 8 as libc::c_int))
                    {
                        let mut cat: libc::c_int = 0 as libc::c_int;
                        while cat
                            < 3 as libc::c_int
                                + ((*((*h).sps).as_mut_ptr()).i_chroma_format_idc
                                    == CHROMA_444 as libc::c_int)
                                    as libc::c_int
                        {
                            let mut dct8x8: libc::c_int = cat & 1 as libc::c_int;
                            if !((*h).param.analyse.b_transform_8x8 == 0 && dct8x8 != 0) {
                                let mut size_1: libc::c_int = if dct8x8 != 0 {
                                    64 as libc::c_int
                                } else {
                                    16 as libc::c_int
                                };
                                let mut nr_offset: *mut udctcoef = ((*((*h).nr_offset_emergency)
                                    .offset(q_2 as isize))[cat as usize])
                                    .as_mut_ptr();
                                let mut dc_threshold: libc::c_int = (51 as libc::c_int
                                    + 6 as libc::c_int * (8 as libc::c_int - 8 as libc::c_int)
                                    + 18 as libc::c_int
                                    - (51 as libc::c_int
                                        + 6 as libc::c_int
                                            * (8 as libc::c_int - 8 as libc::c_int)))
                                    * 2 as libc::c_int
                                    / 3 as libc::c_int;
                                let mut luma_threshold: libc::c_int = (51 as libc::c_int
                                    + 6 as libc::c_int * (8 as libc::c_int - 8 as libc::c_int)
                                    + 18 as libc::c_int
                                    - (51 as libc::c_int
                                        + 6 as libc::c_int
                                            * (8 as libc::c_int - 8 as libc::c_int)))
                                    * 2 as libc::c_int
                                    / 3 as libc::c_int;
                                let mut chroma_threshold: libc::c_int = 0 as libc::c_int;
                                let mut i_7: libc::c_int = 0 as libc::c_int;
                                while i_7 < size_1 {
                                    let mut max: libc::c_int = ((1 as libc::c_int)
                                        << (7 as libc::c_int + 8 as libc::c_int))
                                        - 1 as libc::c_int;
                                    if q_2
                                        == 51 as libc::c_int
                                            + 6 as libc::c_int
                                                * (8 as libc::c_int - 8 as libc::c_int)
                                            + 18 as libc::c_int
                                            - (51 as libc::c_int
                                                + 6 as libc::c_int
                                                    * (8 as libc::c_int - 8 as libc::c_int))
                                            - 1 as libc::c_int
                                    {
                                        *nr_offset.offset(i_7 as isize) = max as udctcoef;
                                    } else {
                                        let mut thresh: libc::c_int = if i_7 == 0 as libc::c_int {
                                            dc_threshold
                                        } else if cat >= 2 as libc::c_int {
                                            chroma_threshold
                                        } else {
                                            luma_threshold
                                        };
                                        if q_2 < thresh {
                                            *nr_offset.offset(i_7 as isize) =
                                                0 as libc::c_int as udctcoef;
                                        } else {
                                            let mut pos: libc::c_double =
                                                (q_2 - thresh + 1 as libc::c_int) as libc::c_double
                                                    / (51 as libc::c_int
                                                        + 6 as libc::c_int
                                                            * (8 as libc::c_int - 8 as libc::c_int)
                                                        + 18 as libc::c_int
                                                        - (51 as libc::c_int
                                                            + 6 as libc::c_int
                                                                * (8 as libc::c_int
                                                                    - 8 as libc::c_int))
                                                        - thresh)
                                                        as libc::c_double;
                                            let mut start_1: libc::c_double = (if dct8x8 != 0 {
                                                (*((*h).unquant8_mf
                                                    [CQM_8PY as libc::c_int as usize])
                                                    .offset(
                                                        (51 as libc::c_int
                                                            + 6 as libc::c_int
                                                                * (8 as libc::c_int
                                                                    - 8 as libc::c_int))
                                                            as isize,
                                                    ))
                                                    [i_7 as usize]
                                            } else {
                                                (*((*h).unquant4_mf
                                                    [CQM_4PY as libc::c_int as usize])
                                                    .offset(
                                                        (51 as libc::c_int
                                                            + 6 as libc::c_int
                                                                * (8 as libc::c_int
                                                                    - 8 as libc::c_int))
                                                            as isize,
                                                    ))
                                                    [i_7 as usize]
                                            })
                                                as libc::c_double;
                                            let mut bias: libc::c_double = (pow(
                                                2 as libc::c_int as libc::c_double,
                                                pos * (51 as libc::c_int
                                                    + 6 as libc::c_int
                                                        * (8 as libc::c_int - 8 as libc::c_int)
                                                    + 18 as libc::c_int
                                                    - (51 as libc::c_int
                                                        + 6 as libc::c_int
                                                            * (8 as libc::c_int
                                                                - 8 as libc::c_int)))
                                                    as libc::c_double
                                                    / 10.0f64,
                                            ) * 0.003f64
                                                - 0.003f64)
                                                * start_1;
                                            *nr_offset.offset(i_7 as isize) =
                                                (if bias + 0.5f64 < max as libc::c_double {
                                                    bias + 0.5f64
                                                } else {
                                                    max as libc::c_double
                                                })
                                                    as udctcoef;
                                        }
                                    }
                                    i_7 += 1;
                                }
                            }
                            cat += 1;
                        }
                        q_2 += 1;
                    }
                    if (*h).mb.b_lossless == 0 {
                        while *((*h).chroma_qp_table).offset(
                            (if (*h).param.rc.i_qp_min
                                < 51 as libc::c_int
                                    + 6 as libc::c_int * (8 as libc::c_int - 8 as libc::c_int)
                            {
                                (*h).param.rc.i_qp_min
                            } else {
                                51 as libc::c_int
                                    + 6 as libc::c_int * (8 as libc::c_int - 8 as libc::c_int)
                            }) as isize,
                        ) as libc::c_int
                            <= max_chroma_qp_err
                        {
                            (*h).param.rc.i_qp_min += 1;
                            (*h).param.rc.i_qp_min;
                        }
                        if min_qp_err <= (*h).param.rc.i_qp_max {
                            (*h).param.rc.i_qp_max = min_qp_err - 1 as libc::c_int;
                        }
                        if max_qp_err >= (*h).param.rc.i_qp_min {
                            (*h).param.rc.i_qp_min = max_qp_err + 1 as libc::c_int;
                        }
                        if (*h).param.b_cabac == 0
                            && (*((*h).sps).as_mut_ptr()).i_profile_idc
                                < PROFILE_HIGH as libc::c_int
                        {
                            while *((*h).chroma_qp_table).offset(
                                (if (*h).param.rc.i_qp_max
                                    < 51 as libc::c_int
                                        + 6 as libc::c_int * (8 as libc::c_int - 8 as libc::c_int)
                                {
                                    (*h).param.rc.i_qp_max
                                } else {
                                    51 as libc::c_int
                                        + 6 as libc::c_int * (8 as libc::c_int - 8 as libc::c_int)
                                }) as isize,
                            ) as libc::c_int
                                <= 12 as libc::c_int
                                || (*h).param.rc.i_qp_max <= 12 as libc::c_int
                            {
                                (*h).param.rc.i_qp_max += 1;
                                (*h).param.rc.i_qp_max;
                            }
                        }
                        if (*h).param.rc.i_qp_min > (*h).param.rc.i_qp_max {
                            x264_8_log(
                                h,
                                0 as libc::c_int,
                                b"Impossible QP constraints for CQM (min=%d, max=%d)\n\0"
                                    as *const u8
                                    as *const libc::c_char,
                                (*h).param.rc.i_qp_min,
                                (*h).param.rc.i_qp_max,
                            );
                            return -(1 as libc::c_int);
                        }
                    }
                    return 0 as libc::c_int;
                }
            }
        }
    }
    x264_8_cqm_delete(h);
    -(1 as libc::c_int)
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_cqm_delete(mut h: *mut x264_t) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        let mut j: libc::c_int = 0;
        j = 0 as libc::c_int;
        while j < i {
            if (*h).quant4_mf[i as usize] == (*h).quant4_mf[j as usize] {
                break;
            }
            j += 1;
        }
        if j == i {
            x264_free((*h).quant4_mf[i as usize] as *mut libc::c_void);
            x264_free((*h).dequant4_mf[i as usize] as *mut libc::c_void);
            x264_free((*h).unquant4_mf[i as usize] as *mut libc::c_void);
        }
        j = 0 as libc::c_int;
        while j < i {
            if (*h).quant4_bias[i as usize] == (*h).quant4_bias[j as usize] {
                break;
            }
            j += 1;
        }
        if j == i {
            x264_free((*h).quant4_bias[i as usize] as *mut libc::c_void);
            x264_free((*h).quant4_bias0[i as usize] as *mut libc::c_void);
        }
        i += 1;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0
        < (if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as libc::c_int {
            4 as libc::c_int
        } else {
            2 as libc::c_int
        })
    {
        let mut j_0: libc::c_int = 0;
        j_0 = 0 as libc::c_int;
        while j_0 < i_0 {
            if (*h).quant8_mf[i_0 as usize] == (*h).quant8_mf[j_0 as usize] {
                break;
            }
            j_0 += 1;
        }
        if j_0 == i_0 {
            x264_free((*h).quant8_mf[i_0 as usize] as *mut libc::c_void);
            x264_free((*h).dequant8_mf[i_0 as usize] as *mut libc::c_void);
            x264_free((*h).unquant8_mf[i_0 as usize] as *mut libc::c_void);
        }
        j_0 = 0 as libc::c_int;
        while j_0 < i_0 {
            if (*h).quant8_bias[i_0 as usize] == (*h).quant8_bias[j_0 as usize] {
                break;
            }
            j_0 += 1;
        }
        if j_0 == i_0 {
            x264_free((*h).quant8_bias[i_0 as usize] as *mut libc::c_void);
            x264_free((*h).quant8_bias0[i_0 as usize] as *mut libc::c_void);
        }
        i_0 += 1;
    }
    x264_free((*h).nr_offset_emergency as *mut libc::c_void);
}
unsafe extern "C" fn cqm_parse_jmlist(
    mut h: *mut x264_t,
    mut buf: *const libc::c_char,
    mut name: *const libc::c_char,
    mut cqm: *mut uint8_t,
    mut jvt: *const uint8_t,
    mut length: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut p: *mut libc::c_char = strstr(buf, name);
    if p.is_null() {
        memset(
            cqm as *mut libc::c_void,
            16 as libc::c_int,
            length as libc::c_ulong,
        );
        return 0 as libc::c_int;
    }
    p = p.offset(strlen(name) as isize);
    if *p as libc::c_int == 'U' as i32 || *p as libc::c_int == 'V' as i32 {
        p = p.offset(1);
    }
    let mut nextvar: *mut libc::c_char = strstr(p, b"INT\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < length
        && {
            p = strpbrk(p, b" \t\n,\0" as *const u8 as *const libc::c_char);
            !p.is_null()
        }
        && {
            p = strpbrk(p, b"0123456789\0" as *const u8 as *const libc::c_char);
            !p.is_null()
        }
    {
        let mut coef: libc::c_int = -(1 as libc::c_int);
        sscanf(
            p,
            b"%d\0" as *const u8 as *const libc::c_char,
            &mut coef as *mut libc::c_int,
        );
        if i == 0 as libc::c_int && coef == 0 as libc::c_int {
            memcpy(
                cqm as *mut libc::c_void,
                jvt as *const libc::c_void,
                length as libc::c_ulong,
            );
            return 0 as libc::c_int;
        }
        if coef < 1 as libc::c_int || coef > 255 as libc::c_int {
            x264_8_log(
                h,
                0 as libc::c_int,
                b"bad coefficient in list '%s'\n\0" as *const u8 as *const libc::c_char,
                name,
            );
            return -(1 as libc::c_int);
        }
        *cqm.offset(i as isize) = coef as uint8_t;
        i += 1;
        i;
    }
    if !nextvar.is_null() && p > nextvar || i != length {
        x264_8_log(
            h,
            0 as libc::c_int,
            b"not enough coefficients in list '%s'\n\0" as *const u8 as *const libc::c_char,
            name,
        );
        return -(1 as libc::c_int);
    }
    0 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_cqm_parse_file(
    mut h: *mut x264_t,
    mut filename: *const libc::c_char,
) -> libc::c_int {
    let mut p: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    let mut b_error: libc::c_int = 0 as libc::c_int;
    (*h).param.i_cqm_preset = 2 as libc::c_int;
    let mut buf: *mut libc::c_char = x264_slurp_file(filename);
    if buf.is_null() {
        x264_8_log(
            h,
            0 as libc::c_int,
            b"can't open file '%s'\n\0" as *const u8 as *const libc::c_char,
            filename,
        );
        return -(1 as libc::c_int);
    }
    loop {
        p = strchr(buf, '#' as i32);
        if p.is_null() {
            break;
        }
        memset(
            p as *mut libc::c_void,
            ' ' as i32,
            strcspn(p, b"\n\0" as *const u8 as *const libc::c_char),
        );
    }
    b_error |= cqm_parse_jmlist(
        h,
        buf,
        b"INTRA4X4_LUMA\0" as *const u8 as *const libc::c_char,
        ((*h).param.cqm_4iy).as_mut_ptr(),
        x264_cqm_jvt4i.as_ptr(),
        16 as libc::c_int,
    );
    b_error |= cqm_parse_jmlist(
        h,
        buf,
        b"INTER4X4_LUMA\0" as *const u8 as *const libc::c_char,
        ((*h).param.cqm_4py).as_mut_ptr(),
        x264_cqm_jvt4p.as_ptr(),
        16 as libc::c_int,
    );
    b_error |= cqm_parse_jmlist(
        h,
        buf,
        b"INTRA4X4_CHROMA\0" as *const u8 as *const libc::c_char,
        ((*h).param.cqm_4ic).as_mut_ptr(),
        x264_cqm_jvt4i.as_ptr(),
        16 as libc::c_int,
    );
    b_error |= cqm_parse_jmlist(
        h,
        buf,
        b"INTER4X4_CHROMA\0" as *const u8 as *const libc::c_char,
        ((*h).param.cqm_4pc).as_mut_ptr(),
        x264_cqm_jvt4p.as_ptr(),
        16 as libc::c_int,
    );
    b_error |= cqm_parse_jmlist(
        h,
        buf,
        b"INTRA8X8_LUMA\0" as *const u8 as *const libc::c_char,
        ((*h).param.cqm_8iy).as_mut_ptr(),
        x264_cqm_jvt8i.as_ptr(),
        64 as libc::c_int,
    );
    b_error |= cqm_parse_jmlist(
        h,
        buf,
        b"INTER8X8_LUMA\0" as *const u8 as *const libc::c_char,
        ((*h).param.cqm_8py).as_mut_ptr(),
        x264_cqm_jvt8p.as_ptr(),
        64 as libc::c_int,
    );
    if (*((*h).sps).as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as libc::c_int {
        b_error |= cqm_parse_jmlist(
            h,
            buf,
            b"INTRA8X8_CHROMA\0" as *const u8 as *const libc::c_char,
            ((*h).param.cqm_8ic).as_mut_ptr(),
            x264_cqm_jvt8i.as_ptr(),
            64 as libc::c_int,
        );
        b_error |= cqm_parse_jmlist(
            h,
            buf,
            b"INTER8X8_CHROMA\0" as *const u8 as *const libc::c_char,
            ((*h).param.cqm_8pc).as_mut_ptr(),
            x264_cqm_jvt8p.as_ptr(),
            64 as libc::c_int,
        );
    }
    x264_free(buf as *mut libc::c_void);
    b_error
}
