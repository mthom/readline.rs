extern crate libc;

use readline::libc::fileno;

use std::ffi::{CString, CStr};

extern "C" {
    #[link_name = "\u{1}rl_point"]
    pub static mut rl_point: ::std::os::raw::c_int;
}

extern "C" {
    #[link_name = "\u{1}rl_end"]
    pub static mut rl_end: ::std::os::raw::c_int;
}

extern "C" {
    #[link_name = "\u{1}rl_done"]
    pub static mut rl_done: ::std::os::raw::c_int;
}

extern "C" {
    #[link_name = "\u{1}rl_line_buffer"]
    static mut rl_line_buffer: *mut ::std::os::raw::c_char;
}

extern "C" {
    #[link_name = "\u{1}rl_instream"]
    static mut rl_instream: *mut libc::FILE;
}

pub fn rl_line_buffer_as_str() -> Option<&'static str> {
    unsafe {
        if !rl_line_buffer.is_null() {
            CStr::from_ptr(rl_line_buffer).to_str().ok()
        } else {
            None
        }
    }
}

extern "C" {
    fn rl_read_key() -> ::std::os::raw::c_int;
}

pub fn read_key_rl() -> char {
    unsafe {
        (rl_read_key() as u8) as char
    }
}

extern "C" {
    fn readline(prompt: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}

extern "C" {
    fn rl_copy_text(start: ::std::os::raw::c_int, end: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char;
}

pub fn copy_text_rl(start: i32, end: i32) -> Option<String> {
    unsafe {        
        CStr::from_ptr(rl_copy_text(start, end)).to_str()
            .ok().map(|string_buf| string_buf.to_string())
    }
}

pub fn readline_rl(prompt: &str) -> Option<&'static str> {
    unsafe {
        let prompt = match CString::new(prompt) {
            Ok(prompt) => prompt,
            Err(_) => return None
        };

        let prompt = prompt.into_raw();
        let input = readline(prompt);

        if input.is_null() {
            None
        } else {
            CStr::from_ptr(input).to_str().ok()
        }
    }
}

extern "C" {
    fn rl_clear_pending_input() -> ::std::os::raw::c_int;
}

pub fn clear_pending_input_rl() -> i32 {
    unsafe {
        rl_clear_pending_input() as i32
    }
}

extern "C" {
    fn rl_insert_text(text: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}

pub fn insert_text_rl(text: &str) -> Option<i32> {
    unsafe {
        let text = match CString::new(text) {
            Ok(text) => text,
            Err(_) => return None
        };

        Some(rl_insert_text(text.into_raw()))
    }
}

extern "C" {
    fn rl_initialize() -> ::std::os::raw::c_int;
}

pub fn initialize_rl() -> i32 {
    unsafe {
        rl_initialize() as i32
    }
}

#[allow(non_camel_case_types)]
pub type rl_command_func_t =
    unsafe extern "C" fn(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

#[allow(non_camel_case_types)]
pub type rl_hook_func_t =
    unsafe extern "C" fn() -> ::std::os::raw::c_int;

extern "C" {
    pub static mut rl_startup_hook: rl_hook_func_t;
}

extern "C" {
    fn rl_bind_key(
        arg1: ::std::os::raw::c_int,
        arg2: rl_command_func_t,
    ) -> ::std::os::raw::c_int;
}

pub fn bind_key_rl(key: i32, f: rl_command_func_t) -> i32 {
    unsafe {
        rl_bind_key(key, f) as i32
    }
}

extern "C" {
    fn rl_unbind_key(
        arg1: ::std::os::raw::c_int
    ) -> ::std::os::raw::c_int;
}

pub fn unbind_key_rl(key: i32) -> i32 {
    unsafe {
        rl_unbind_key(key) as i32
    }
}

extern "C" {
    fn rl_bind_keyseq(
        arg1: *const ::std::os::raw::c_char,
        arg2: rl_command_func_t,
    ) -> ::std::os::raw::c_int;
}

pub fn bind_keyseq_rl(chord: &str, f: rl_command_func_t) -> Option<i32> {
    unsafe {
        let chord = match CString::new(chord) {
            Ok(chord) => chord,
            Err(_) => return None
        };

        Some(rl_bind_keyseq(chord.into_raw(), f) as i32)
    }
}
