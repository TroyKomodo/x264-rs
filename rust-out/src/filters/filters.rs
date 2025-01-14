#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(label_break_value)]
extern "C" {
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn x264_cli_log(
        name: *const libc::c_char,
        i_level: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn x264_split_options(
    mut opt_str: *const libc::c_char,
    mut options: *const *const libc::c_char,
) -> *mut *mut libc::c_char {
    let mut opt_count: libc::c_int = 0 as libc::c_int;
    let mut options_count: libc::c_int = 0 as libc::c_int;
    let mut found_named: libc::c_int = 0 as libc::c_int;
    let mut size: size_t = 0 as libc::c_int as size_t;
    let mut opt: *const libc::c_char = opt_str;
    if opt_str.is_null() {
        return std::ptr::null_mut::<*mut libc::c_char>();
    }
    while !(*options.offset(options_count as isize)).is_null() {
        options_count += 1;
        options_count;
    }
    loop {
        let mut length: size_t = strcspn(
            opt,
            b"=,\0" as *const u8 as *const libc::c_char,
        );
        if *opt.offset(length as isize) as libc::c_int == '=' as i32 {
            let mut option: *const *const libc::c_char = options;
            while !(*option).is_null()
                && (strlen(*option) != length || strncmp(opt, *option, length) != 0)
            {
                option = option.offset(1);
                option;
            }
            if (*option).is_null() {
                x264_cli_log(
                    b"options\0" as *const u8 as *const libc::c_char,
                    0 as libc::c_int,
                    b"Invalid option '%.*s'\n\0" as *const u8 as *const libc::c_char,
                    length,
                    opt,
                );
                return std::ptr::null_mut::<*mut libc::c_char>();
            }
            found_named = 1 as libc::c_int;
            length = (length as libc::c_ulong)
                .wrapping_add(
                    strcspn(
                        opt.offset(length as isize),
                        b",\0" as *const u8 as *const libc::c_char,
                    ),
                ) as size_t as size_t;
        } else {
            if opt_count >= options_count {
                x264_cli_log(
                    b"options\0" as *const u8 as *const libc::c_char,
                    0 as libc::c_int,
                    b"Too many options given\n\0" as *const u8 as *const libc::c_char,
                );
                return std::ptr::null_mut::<*mut libc::c_char>();
            }
            if found_named != 0 {
                x264_cli_log(
                    b"options\0" as *const u8 as *const libc::c_char,
                    0 as libc::c_int,
                    b"Ordered option given after named\n\0" as *const u8
                        as *const libc::c_char,
                );
                return std::ptr::null_mut::<*mut libc::c_char>();
            }
            size = (size as libc::c_ulong)
                .wrapping_add(
                    (strlen(*options.offset(opt_count as isize)))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                ) as size_t as size_t;
        }
        opt_count += 1;
        opt_count;
        opt = opt.offset(length as isize);
        let fresh0 = opt;
        opt = opt.offset(1);
        if *fresh0 == 0 {
            break;
        }
    }
    let mut offset: size_t = ((2 as libc::c_int * (opt_count + 1 as libc::c_int))
        as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong);
    size = size
        .wrapping_add(
            offset.wrapping_add(opt.offset_from(opt_str) as libc::c_long as size_t),
        );
    let mut opts: *mut *mut libc::c_char = calloc(
        1 as libc::c_int as libc::c_ulong,
        size,
    ) as *mut *mut libc::c_char;
    if opts.is_null() {
        x264_cli_log(
            b"options\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
            b"malloc failed\n\0" as *const u8 as *const libc::c_char,
        );
        return std::ptr::null_mut::<*mut libc::c_char>();
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 2 as libc::c_int * opt_count {
        let mut length_0: size_t = strcspn(
            opt_str,
            b"=,\0" as *const u8 as *const libc::c_char,
        );
        if *opt_str.offset(length_0 as isize) as libc::c_int == '=' as i32 {
            let fresh1 = i;
            i += 1;
            let fresh2 = &mut (*opts.offset(fresh1 as isize));
            *fresh2 = memcpy(
                (opts as *mut libc::c_char).offset(offset as isize) as *mut libc::c_void,
                opt_str as *const libc::c_void,
                length_0,
            ) as *mut libc::c_char;
            offset = offset
                .wrapping_add(length_0.wrapping_add(1 as libc::c_int as size_t));
            opt_str = opt_str
                .offset(length_0.wrapping_add(1 as libc::c_int as size_t) as isize);
            length_0 = strcspn(opt_str, b",\0" as *const u8 as *const libc::c_char);
        } else {
            let mut option_0: *const libc::c_char = *options
                .offset((i / 2 as libc::c_int) as isize);
            let mut option_length: size_t = strlen(option_0);
            let fresh3 = i;
            i += 1;
            let fresh4 = &mut (*opts.offset(fresh3 as isize));
            *fresh4 = memcpy(
                (opts as *mut libc::c_char).offset(offset as isize) as *mut libc::c_void,
                option_0 as *const libc::c_void,
                option_length,
            ) as *mut libc::c_char;
            offset = offset
                .wrapping_add(option_length.wrapping_add(1 as libc::c_int as size_t));
            option_0 = option_0
                .offset(option_length.wrapping_add(1 as libc::c_int as size_t) as isize);
        }
        let fresh5 = i;
        i += 1;
        let fresh6 = &mut (*opts.offset(fresh5 as isize));
        *fresh6 = memcpy(
            (opts as *mut libc::c_char).offset(offset as isize) as *mut libc::c_void,
            opt_str as *const libc::c_void,
            length_0,
        ) as *mut libc::c_char;
        offset = offset.wrapping_add(length_0.wrapping_add(1 as libc::c_int as size_t));
        opt_str = opt_str
            .offset(length_0.wrapping_add(1 as libc::c_int as size_t) as isize);
    }
    if offset == size {} else {
        __assert_fail(
            b"offset == size\0" as *const u8 as *const libc::c_char,
            b"filters/filters.c\0" as *const u8 as *const libc::c_char,
            96 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 61],
                &[libc::c_char; 61],
            >(b"char **x264_split_options(const char *, const char *const *)\0"))
                .as_ptr(),
        );
    }
    'c_9983: {
        if offset == size {} else {
            __assert_fail(
                b"offset == size\0" as *const u8 as *const libc::c_char,
                b"filters/filters.c\0" as *const u8 as *const libc::c_char,
                96 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 61],
                    &[libc::c_char; 61],
                >(b"char **x264_split_options(const char *, const char *const *)\0"))
                    .as_ptr(),
            );
        }
    };
    opts
}
#[no_mangle]
pub unsafe extern "C" fn x264_get_option(
    mut name: *const libc::c_char,
    mut split_options: *mut *mut libc::c_char,
) -> *mut libc::c_char {
    if !split_options.is_null() {
        let mut last_i: libc::c_int = -(1 as libc::c_int);
        let mut i: libc::c_int = 0 as libc::c_int;
        while !(*split_options.offset(i as isize)).is_null() {
            if strcmp(*split_options.offset(i as isize), name) == 0 {
                last_i = i;
            }
            i += 2 as libc::c_int;
        }
        if last_i >= 0 as libc::c_int
            && *(*split_options.offset((last_i + 1 as libc::c_int) as isize))
                .offset(0 as libc::c_int as isize) as libc::c_int != 0
        {
            return *split_options.offset((last_i + 1 as libc::c_int) as isize);
        }
    }
    std::ptr::null_mut::<libc::c_char>()
}
#[no_mangle]
pub unsafe extern "C" fn x264_otob(
    mut str: *const libc::c_char,
    mut def: libc::c_int,
) -> libc::c_int {
    if !str.is_null() {
        return (strcasecmp(str, b"true\0" as *const u8 as *const libc::c_char) == 0
            || strcmp(str, b"1\0" as *const u8 as *const libc::c_char) == 0
            || strcasecmp(str, b"yes\0" as *const u8 as *const libc::c_char) == 0)
            as libc::c_int;
    }
    def
}
#[no_mangle]
pub unsafe extern "C" fn x264_otof(
    mut str: *const libc::c_char,
    mut def: libc::c_double,
) -> libc::c_double {
    let mut ret: libc::c_double = def;
    if !str.is_null() {
        let mut end: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
        ret = strtod(str, &mut end);
        if end == str as *mut libc::c_char || *end as libc::c_int != '\0' as i32 {
            ret = def;
        }
    }
    ret
}
#[no_mangle]
pub unsafe extern "C" fn x264_otoi(
    mut str: *const libc::c_char,
    mut def: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = def;
    if !str.is_null() {
        let mut end: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
        ret = strtol(str, &mut end, 0 as libc::c_int) as libc::c_int;
        if end == str as *mut libc::c_char || *end as libc::c_int != '\0' as i32 {
            ret = def;
        }
    }
    ret
}
#[no_mangle]
pub unsafe extern "C" fn x264_otos(
    mut str: *mut libc::c_char,
    mut def: *mut libc::c_char,
) -> *mut libc::c_char {
    if !str.is_null() { str } else { def }
}
