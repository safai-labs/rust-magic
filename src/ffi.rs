//! Internal Foreign Function Interface module for `magic_sys` / `libmagic`
//!
//! Contains `unsafe` as a medium level binding.

#![allow(unsafe_code)]

extern crate libc;
extern crate magic_sys as libmagic;
extern crate thiserror;

use std::ffi::CStr;

#[non_exhaustive]
#[derive(thiserror::Error, Debug)]
pub(crate) enum LibmagicError {
    /// Error during `magic_open`
    #[error("Error calling `magic_open`, errno: {errno}")]
    Open {
        #[source]
        errno: errno::Errno,
    },

    /// Error for opened `magic_t` instance
    #[error("Error for cookie call ({}): {explanation}", match .errno {
        Some(errno) => format!("OS errno: {}", errno),
        None => "no OS errno".to_string(),
    })]
    Cookie {
        explanation: String,
        #[source]
        errno: Option<errno::Errno>,
    },
}

pub(crate) fn last_error(cookie: self::libmagic::magic_t) -> Option<LibmagicError> {
    unsafe {
        let error = self::libmagic::magic_error(cookie);
        let errno = self::libmagic::magic_errno(cookie);
        if error.is_null() {
            None
        } else {
            let slice = CStr::from_ptr(error).to_bytes();
            Some(LibmagicError::Cookie {
                explanation: std::str::from_utf8(slice).unwrap().to_string(),
                errno: match errno {
                    0 => None,
                    _ => Some(errno::Errno(errno)),
                },
            })
        }
    }
}

pub(crate) fn close(cookie: self::libmagic::magic_t) {
    unsafe { self::libmagic::magic_close(cookie) }
}

pub(crate) fn file(
    cookie: self::libmagic::magic_t,
    filename: &std::ffi::CStr,
) -> Result<std::ffi::CString, LibmagicError> {
    let filename_ptr = filename.as_ptr();
    let res = unsafe { self::libmagic::magic_file(cookie, filename_ptr) };

    if res.is_null() {
        Err(last_error(cookie).unwrap())
    } else {
        let c_str = unsafe { std::ffi::CStr::from_ptr(res) };
        Ok(c_str.into())
    }
}

pub(crate) fn buffer(
    cookie: self::libmagic::magic_t,
    buffer: &[u8],
) -> Result<std::ffi::CString, LibmagicError> {
    let buffer_ptr = buffer.as_ptr();
    let buffer_len = buffer.len() as libc::size_t;
    let res = unsafe { self::libmagic::magic_buffer(cookie, buffer_ptr, buffer_len) };

    if res.is_null() {
        Err(last_error(cookie).unwrap())
    } else {
        let c_str = unsafe { std::ffi::CStr::from_ptr(res) };
        Ok(c_str.into())
    }
}
