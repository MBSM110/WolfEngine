/* automatically generated by rust-bindgen 0.60.1 */

pub const _VCRT_COMPILER_PREPROCESSOR: u32 = 1;
pub const _SAL_VERSION: u32 = 20;
pub const __SAL_H_VERSION: u32 = 180000000;
pub const _USE_DECLSPECS_FOR_SAL: u32 = 0;
pub const _USE_ATTRIBUTES_FOR_SAL: u32 = 0;
pub const _CRT_PACKING: u32 = 8;
pub const _HAS_EXCEPTIONS: u32 = 1;
pub const _STL_LANG: u32 = 0;
pub const _HAS_CXX17: u32 = 0;
pub const _HAS_CXX20: u32 = 0;
pub const _HAS_CXX23: u32 = 0;
pub const _HAS_NODISCARD: u32 = 0;
pub const WCHAR_MIN: u32 = 0;
pub const WCHAR_MAX: u32 = 65535;
pub const WINT_MIN: u32 = 0;
pub const WINT_MAX: u32 = 65535;
pub const W_MAX_PATH: u32 = 260;
pub type va_list = *mut ::std::os::raw::c_char;
extern "C" {
    pub fn __va_start(arg1: *mut *mut ::std::os::raw::c_char, ...);
}
pub type size_t = ::std::os::raw::c_ulonglong;
pub type __vcrt_bool = bool;
pub type wchar_t = ::std::os::raw::c_ushort;
extern "C" {
    pub fn __security_init_cookie();
}
extern "C" {
    pub fn __security_check_cookie(_StackCookie: usize);
}
extern "C" {
    pub fn __report_gsfailure(_StackCookie: usize);
}
extern "C" {
    pub static mut __security_cookie: usize;
}
pub type int_least8_t = ::std::os::raw::c_schar;
pub type int_least16_t = ::std::os::raw::c_short;
pub type int_least32_t = ::std::os::raw::c_int;
pub type int_least64_t = ::std::os::raw::c_longlong;
pub type uint_least8_t = ::std::os::raw::c_uchar;
pub type uint_least16_t = ::std::os::raw::c_ushort;
pub type uint_least32_t = ::std::os::raw::c_uint;
pub type uint_least64_t = ::std::os::raw::c_ulonglong;
pub type int_fast8_t = ::std::os::raw::c_schar;
pub type int_fast16_t = ::std::os::raw::c_int;
pub type int_fast32_t = ::std::os::raw::c_int;
pub type int_fast64_t = ::std::os::raw::c_longlong;
pub type uint_fast8_t = ::std::os::raw::c_uchar;
pub type uint_fast16_t = ::std::os::raw::c_uint;
pub type uint_fast32_t = ::std::os::raw::c_uint;
pub type uint_fast64_t = ::std::os::raw::c_ulonglong;
pub type intmax_t = ::std::os::raw::c_longlong;
pub type uintmax_t = ::std::os::raw::c_ulonglong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVFrame {
    _unused: [u8; 0],
}
pub type w_av_frame = *mut AVFrame;
extern "C" {
    #[doc = " initialize the ffmpeg AVFrame"]
    #[doc = " @param p_frame, the ffmpeg AVFrame"]
    #[doc = " @param p_pixel_format, the pixel format of ffmpeg AVFrame"]
    #[doc = " @param p_width, the width of ffmpeg AVFrame"]
    #[doc = " @param p_height, the height of ffmpeg AVFrame"]
    #[doc = " @param p_alignment, the alignment"]
    #[doc = " @param p_data, the initial data of ffmpeg AVFrame"]
    #[doc = " @param p_error, the error buffer"]
    #[doc = " @returns zero on success"]
    pub fn w_av_frame_init(
        p_frame: *mut w_av_frame,
        p_pixel_format: u32,
        p_width: u32,
        p_height: u32,
        p_alignment: u32,
        p_data: *const u8,
        p_error: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " initialize the ffmpeg AVFrame"]
    #[doc = " @param p_frame, the ffmpeg AVFrame"]
    #[doc = " @param p_data, the initial data of ffmpeg AVFrame"]
    #[doc = " @param p_alignment, the alignment"]
    #[doc = " @param p_error, the error buffer"]
    #[doc = " @returns zero on success"]
    pub fn w_av_set_data(
        p_frame: w_av_frame,
        p_data: *const u8,
        p_alignment: u32,
        p_error: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " get data of ffmpeg AVFrame"]
    #[doc = " @param p_error, the error buffer"]
    #[doc = " @returns zero on success"]
    pub fn w_av_get_data(
        p_frame: w_av_frame,
        p_frame_data: *mut u8,
        p_error: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " initialize the ffmpeg AVFrame"]
    #[doc = " @param p_pixel_format, the pixel format of ffmpeg AVFrame"]
    #[doc = " @param p_width, the width of ffmpeg AVFrame"]
    #[doc = " @param p_height, the height of ffmpeg AVFrame"]
    #[doc = " @param p_alignment, the aligmnet which is usually 1"]
    #[doc = " @returns zero on success"]
    pub fn w_av_get_required_buffer_size(
        p_pixel_format: u32,
        p_width: u32,
        p_height: u32,
        p_alignment: u32,
    ) -> size_t;
}
extern "C" {
    #[doc = " convert the ffmpeg AVFrame"]
    #[doc = " @param p_frame, the ffmpeg AVFrame"]
    #[doc = " @param p_error, the error buffer"]
    #[doc = " @returns zero on success"]
    pub fn w_av_frame_convert(
        p_src_frame: w_av_frame,
        p_dst_frame: *mut w_av_frame,
        p_error: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " release all ffmpeg resources"]
    #[doc = " @param p_ffmpeg, the ffmpeg AVFrame"]
    pub fn w_av_frame_fini(p_ffmpeg: *mut w_av_frame);
}
