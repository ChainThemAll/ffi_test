extern crate libc;
use libc::{c_char, c_int, c_uchar, c_void, size_t};

use crate::ffi::{
    pcre2_code_free_8, pcre2_compile_8, pcre2_get_ovector_pointer_8, pcre2_match_8,
    pcre2_match_data_create_from_pattern_8, pcre2_match_data_free_8,
};
pub struct Pcre2Regex {
    code: *mut c_void,
}

impl Pcre2Regex {
    pub fn new(pattern: &str) -> Result<Self, String> {
        let pattern_c = std::ffi::CString::new(pattern).unwrap();
        let mut errorcode: c_int = 0;
        let mut erroroffset: size_t = 0;
        let code = unsafe {
            pcre2_compile_8(
                pattern_c.as_ptr() as *const c_uchar,
                pattern.len() as size_t,
                0, // options
                &mut errorcode,
                &mut erroroffset,
                std::ptr::null_mut(), // context
            )
        };
        if code.is_null() {
            Err(format!(
                "Compilation failed at offset {}: error code {}",
                erroroffset, errorcode
            ))
        } else {
            Ok(Pcre2Regex { code })
        }
    }

    pub fn match_str(&self, subject: &str) -> bool {
        let subject_c = std::ffi::CString::new(subject).unwrap();
        let match_data =
            unsafe { pcre2_match_data_create_from_pattern_8(self.code, std::ptr::null_mut()) };
        let result = unsafe {
            pcre2_match_8(
                self.code,
                subject_c.as_ptr() as *const c_uchar,
                subject.len() as size_t,
                0, // startoffset
                0, // options
                match_data,
                std::ptr::null_mut(), // match_context
            )
        };
        let matched = result > 0;
        unsafe {
            pcre2_match_data_free_8(match_data);
        }
        matched
    }
    // 新增方法以获取所有匹配结果
    pub fn find_all_matches(&self, subject: &str) -> Vec<String> {
        let mut matches = Vec::new();
        let subject_c = std::ffi::CString::new(subject).expect("CString::new failed");
        let mut offset: size_t = 0;
        let subject_len = subject.len() as size_t;

        while offset < subject_len {
            let match_data =
                unsafe { pcre2_match_data_create_from_pattern_8(self.code, std::ptr::null_mut()) };
            let result = unsafe {
                pcre2_match_8(
                    self.code,
                    subject_c.as_ptr() as *const c_uchar,
                    subject_len,
                    offset,
                    0, // options
                    match_data,
                    std::ptr::null_mut(), // match_context
                )
            };

            if result > 0 {
                let ovector = unsafe { pcre2_get_ovector_pointer_8(match_data) };
                if !ovector.is_null() {
                    let start = unsafe { *ovector.offset(0) } as usize;
                    let end = unsafe { *ovector.offset(1) } as usize;

                    if let Ok(match_str) = std::str::from_utf8(&subject.as_bytes()[start..end]) {
                        matches.push(match_str.to_string());
                        offset = end as size_t;
                    } else {
                        break; // Stop if unable to convert bytes to str
                    }
                }
            } else {
                break; // No more matches or an error occurred
            }

            unsafe {
                pcre2_match_data_free_8(match_data);
            }
        }

        matches
    }
}

impl Drop for Pcre2Regex {
    fn drop(&mut self) {
        unsafe {
            pcre2_code_free_8(self.code);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_regex_match() {
        let regex = Pcre2Regex::new("a.*b").unwrap();
        assert!(regex.match_str("acb"));
    }

    #[test]
    fn test_invalid_regex_match() {
        let regex = Pcre2Regex::new("a.*b").unwrap();
        assert!(!regex.match_str("abc"));
    }
}
