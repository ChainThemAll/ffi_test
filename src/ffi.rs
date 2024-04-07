extern crate libc;
use libc::{c_char, c_int, c_uchar, c_void, size_t};

#[link(name = "pcre2-8")]
extern "C" {
    pub fn pcre2_compile_8(
        pattern: *const c_uchar,
        length: size_t,
        options: u32,
        errorcode: *mut c_int,
        erroroffset: *mut size_t,
        context: *mut c_void,
    ) -> *mut c_void;

    pub fn pcre2_match_data_create_from_pattern_8(
        code: *const c_void,
        context: *mut c_void,
    ) -> *mut c_void;

    pub fn pcre2_match_8(
        code: *const c_void,
        subject: *const c_uchar,
        length: size_t,
        startoffset: size_t,
        options: u32,
        match_data: *mut c_void,
        match_context: *mut c_void,
    ) -> c_int;

    pub fn pcre2_get_ovector_pointer_8(match_data: *mut c_void) -> *mut size_t;

    pub fn pcre2_match_data_free_8(match_data: *mut c_void);

    pub fn pcre2_code_free_8(code: *mut c_void);
}
