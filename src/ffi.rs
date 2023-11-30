#![allow(clippy::all, non_camel_case_types)]

use core::ffi::{c_int, c_uchar};

/* automatically generated by rust-bindgen 0.57.0 */

pub const MINIMP3_MAX_SAMPLES_PER_FRAME: u32 = 2304;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mp3dec_frame_info_t {
    pub frame_bytes: c_int,
    pub frame_offset: c_int,
    pub channels: c_int,
    pub hz: c_int,
    pub layer: c_int,
    pub bitrate_kbps: c_int,
}
#[test]
fn bindgen_test_layout_mp3dec_frame_info_t() {
    assert_eq!(
        ::core::mem::size_of::<mp3dec_frame_info_t>(),
        24usize,
        concat!("Size of: ", stringify!(mp3dec_frame_info_t))
    );
    assert_eq!(
        ::core::mem::align_of::<mp3dec_frame_info_t>(),
        4usize,
        concat!("Alignment of ", stringify!(mp3dec_frame_info_t))
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<mp3dec_frame_info_t>())).frame_bytes as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(mp3dec_frame_info_t),
            "::",
            stringify!(frame_bytes)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<mp3dec_frame_info_t>())).frame_offset as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(mp3dec_frame_info_t),
            "::",
            stringify!(frame_offset)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<mp3dec_frame_info_t>())).channels as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(mp3dec_frame_info_t),
            "::",
            stringify!(channels)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<mp3dec_frame_info_t>())).hz as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(mp3dec_frame_info_t),
            "::",
            stringify!(hz)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<mp3dec_frame_info_t>())).layer as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(mp3dec_frame_info_t),
            "::",
            stringify!(layer)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<mp3dec_frame_info_t>())).bitrate_kbps as *const _ as usize
        },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(mp3dec_frame_info_t),
            "::",
            stringify!(bitrate_kbps)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mp3dec_t {
    pub mdct_overlap: [[f32; 288usize]; 2usize],
    pub qmf_state: [f32; 960usize],
    pub reserv: c_int,
    pub free_format_bytes: c_int,
    pub header: [c_uchar; 4usize],
    pub reserv_buf: [c_uchar; 511usize],
}
#[test]
fn bindgen_test_layout_mp3dec_t() {
    assert_eq!(
        ::core::mem::size_of::<mp3dec_t>(),
        6668usize,
        concat!("Size of: ", stringify!(mp3dec_t))
    );
    assert_eq!(
        ::core::mem::align_of::<mp3dec_t>(),
        4usize,
        concat!("Alignment of ", stringify!(mp3dec_t))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<mp3dec_t>())).mdct_overlap as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(mp3dec_t),
            "::",
            stringify!(mdct_overlap)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<mp3dec_t>())).qmf_state as *const _ as usize },
        2304usize,
        concat!(
            "Offset of field: ",
            stringify!(mp3dec_t),
            "::",
            stringify!(qmf_state)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<mp3dec_t>())).reserv as *const _ as usize },
        6144usize,
        concat!(
            "Offset of field: ",
            stringify!(mp3dec_t),
            "::",
            stringify!(reserv)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<mp3dec_t>())).free_format_bytes as *const _ as usize },
        6148usize,
        concat!(
            "Offset of field: ",
            stringify!(mp3dec_t),
            "::",
            stringify!(free_format_bytes)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<mp3dec_t>())).header as *const _ as usize },
        6152usize,
        concat!(
            "Offset of field: ",
            stringify!(mp3dec_t),
            "::",
            stringify!(header)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<mp3dec_t>())).reserv_buf as *const _ as usize },
        6156usize,
        concat!(
            "Offset of field: ",
            stringify!(mp3dec_t),
            "::",
            stringify!(reserv_buf)
        )
    );
}
extern "C" {
    pub fn mp3dec_init(dec: *mut mp3dec_t);
}
#[cfg(not(feature = "float"))]
pub type mp3d_sample_t = i16;
#[cfg(feature = "float")]
pub type mp3d_sample_t = f32;
extern "C" {
    pub fn mp3dec_decode_frame(
        dec: *mut mp3dec_t,
        mp3: *const u8,
        mp3_bytes: c_int,
        pcm: *mut mp3d_sample_t,
        info: *mut mp3dec_frame_info_t,
    ) -> c_int;
}
