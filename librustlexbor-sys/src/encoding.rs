/******************************************************************************/
/*                               libRustLexbor                                */
/*      rust wrapper around Lexbor an open source HTML Renderer library       */
/*                      https://github.com/lexbor/lexbor                      */
/*                                                                            */
/* Copyright (c) 2020                                       Ivan Semenkov     */
/* https://github.com/isemenkov/librustlexbor               ivan@semenkov.pro */
/*                                                          Ukraine           */
/******************************************************************************/
/*                                                                            */
/* Permission is hereby granted,  free of charge,  to any person obtaining a  */
/* copy of this software and associated documentation files (the "Software"), */
/* to deal in the Software without restriction, including without limitation  */
/* the rights to use, copy,  modify, merge, publish, distribute,  sublicense, */
/* and/or  sell copies  of the Software,  and to permit persons  to whom  the */
/* Software  is furnished to  do  so,  subject to  the following  conditions: */
/*                                                                            */
/* The above copyright notice and this permission notice shall be included in */
/* all copies or substantial portions of the Software.                        */
/*                                                                            */
/* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR */
/* IMPLIED,  INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF  MERCHANTABILITY, */
/* FITNESS  FOR A PARTICULAR PURPOSE  AND NONINFRINGEMENT. IN  NO EVENT SHALL */
/* THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER */
/* LIABILITY,  WHETHER IN AN ACTION  OF CONTRACT,  TORT OR OTHERWISE, ARISING */
/* FROM,  OUT OF  OR IN  CONNECTION WITH  THE SOFTWARE  OR THE  USE OR  OTHER */
/* DEALINGS IN THE SOFTWARE.                                                  */
/*                                                                            */
/******************************************************************************/

#![allow(non_camel_case_types)]

#[path="core.rs"] pub mod core;
extern crate libc;

use libc::{c_uint};

// In UTF-8 0x10FFFF value is maximum (inclusive)
pub const LXB_ENCODING_REPLACEMENT_SIZE : usize                      = 0x03;
pub const LXB_ENCODING_REPLACEMENT_CODEPOINT : usize                 = 0xFFFD;
pub const LXB_ENCODING_MAX_CODEPOINT : usize                         = 0x10FFFF;
pub const LXB_ENCODING_ERROR_CODEPOINT : usize                       = 0x1FFFFF;

pub const LXB_ENCODING_ENCODE_OK : usize                             = 0x00;
pub const LXB_ENCODING_ENCODE_ERROR : i32                            = -0x01;
pub const LXB_ENCODING_ENCODE_SMALL_BUFFER : i32                     = -0x02;

pub const LXB_ENCODING_DECODE_MAX_CODEPOINT : usize 
    = LXB_ENCODING_MAX_CODEPOINT;
pub const LXB_ENCODING_DECODE_ERROR : usize
    = LXB_ENCODING_ERROR_CODEPOINT;
pub const LXB_ENCODING_DECODE_CONTINUE : usize                       = 0x2FFFFF;

pub const LXB_ENCODING_DECODE_2022_JP_ASCII : usize                  = 0x00;
pub const LXB_ENCODING_DECODE_2022_JP_ROMAN : usize                  = 0x01;
pub const LXB_ENCODING_DECODE_2022_JP_KATAKANA : usize               = 0x02;
pub const LXB_ENCODING_DECODE_2022_JP_LEAD : usize                   = 0x03;
pub const LXB_ENCODING_DECODE_2022_JP_TRAIL : usize                  = 0x04;
pub const LXB_ENCODING_DECODE_2022_JP_ESCAPE_START : usize           = 0x05;
pub const LXB_ENCODING_DECODE_2022_JP_ESCAPE : usize                 = 0x06;
pub const LXB_ENCODING_DECODE_2022_JP_UNSET : usize                  = 0x07;

pub const LXB_ENCODING_ENCODE_2022_JP_ASCII : usize                  = 0x00;
pub const LXB_ENCODING_ENCODE_2022_JP_ROMAN : usize                  = 0x01;
pub const LXB_ENCODING_ENCODE_2022_JP_JIS0208 : usize                = 0x02;

#[repr(C)]
pub enum lxb_encoding_t {
    LXB_ENCODING_DEFAULT                                             = 0x00,
    LXB_ENCODING_AUTO                                                = 0x01,
    LXB_ENCODING_UNDEFINED                                           = 0x02,
    LXB_ENCODING_BIG5                                                = 0x03,
    LXB_ENCODING_EUC_JP                                              = 0x04,
    LXB_ENCODING_EUC_KR                                              = 0x05,
    LXB_ENCODING_GBK                                                 = 0x06,
    LXB_ENCODING_IBM866                                              = 0x07,
    LXB_ENCODING_ISO_2022_JP                                         = 0x08,
    LXB_ENCODING_ISO_8859_10                                         = 0x09,
    LXB_ENCODING_ISO_8859_13                                         = 0x0a,
    LXB_ENCODING_ISO_8859_14                                         = 0x0b,
    LXB_ENCODING_ISO_8859_15                                         = 0x0c,
    LXB_ENCODING_ISO_8859_16                                         = 0x0d,
    LXB_ENCODING_ISO_8859_2                                          = 0x0e,
    LXB_ENCODING_ISO_8859_3                                          = 0x0f,
    LXB_ENCODING_ISO_8859_4                                          = 0x10,
    LXB_ENCODING_ISO_8859_5                                          = 0x11,
    LXB_ENCODING_ISO_8859_6                                          = 0x12,
    LXB_ENCODING_ISO_8859_7                                          = 0x13,
    LXB_ENCODING_ISO_8859_8                                          = 0x14,
    LXB_ENCODING_ISO_8859_8_I                                        = 0x15,
    LXB_ENCODING_KOI8_R                                              = 0x16,
    LXB_ENCODING_KOI8_U                                              = 0x17,
    LXB_ENCODING_SHIFT_JIS                                           = 0x18,
    LXB_ENCODING_UTF_16BE                                            = 0x19,
    LXB_ENCODING_UTF_16LE                                            = 0x1a,
    LXB_ENCODING_UTF_8                                               = 0x1b,
    LXB_ENCODING_GB18030                                             = 0x1c,
    LXB_ENCODING_MACINTOSH                                           = 0x1d,
    LXB_ENCODING_REPLACEMENT                                         = 0x1e,
    LXB_ENCODING_WINDOWS_1250                                        = 0x1f,
    LXB_ENCODING_WINDOWS_1251                                        = 0x20,
    LXB_ENCODING_WINDOWS_1252                                        = 0x21,
    LXB_ENCODING_WINDOWS_1253                                        = 0x22,
    LXB_ENCODING_WINDOWS_1254                                        = 0x23,
    LXB_ENCODING_WINDOWS_1255                                        = 0x24,
    LXB_ENCODING_WINDOWS_1256                                        = 0x25,
    LXB_ENCODING_WINDOWS_1257                                        = 0x26,
    LXB_ENCODING_WINDOWS_1258                                        = 0x27,
    LXB_ENCODING_WINDOWS_874                                         = 0x28,
    LXB_ENCODING_X_MAC_CYRILLIC                                      = 0x29,
    LXB_ENCODING_X_USER_DEFINED                                      = 0x2a,
    LXB_ENCODING_LAST_ENTRY                                          = 0x2b
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lxb_encoding_ctx_utf_8_t {
    pub need : c_uint,
    pub lower : core::lxb_char_t,
    pub upper : core::lxb_char_t
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lxb_encoding_ctx_gb18030_t {
    pub first : core::lxb_char_t,
    pub second : core::lxb_char_t,
    pub third : core::lxb_char_t
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lxb_encoding_ctx_euc_jp_t {
    pub lead : core::lxb_char_t,
    pub is_jis0212 : bool
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lxb_encoding_ctx_2022_jp_t {
    pub lead : core::lxb_char_t,
    pub prepand : core::lxb_char_t,
    pub state : c_uint,
    pub out_state : c_uint,
    pub out_flag : bool
}

#[repr(C)]
pub union U {
    pub utf_8 : lxb_encoding_ctx_utf_8_t,
    pub gb18030 : lxb_encoding_ctx_gb18030_t,
    pub lead : c_uint,
    pub euc_jp : lxb_encoding_ctx_euc_jp_t,
    pub iso_2022_jp : lxb_encoding_ctx_2022_jp_t
}

#[repr(C)]
pub struct lxb_encoding_decode_t {
    pub encoding_data : *const lxb_encoding_data_t,

    /* Out buffer */
    pub buffer_out : *mut core::lxb_codepoint_t,
    pub buffer_length : c_uint,
    pub buffer_used : c_uint,

    /*
     * Bad code points will be replaced to user code point.
     * If replace_to == 0 stop parsing and return error ot user.
     */
    pub replace_to : *const core::lxb_codepoint_t,
    pub replace_len : c_uint,

    /* Not for users */
    pub codepoint : core::lxb_codepoint_t,
    pub second_codepoint : core::lxb_codepoint_t,
    pub prepend : bool,
    pub have_error : bool,

    pub status : core::lxb_status_t,
    pub u : U
}

#[repr(C)]
pub struct lxb_encoding_encode_t {
    pub encoding_data : *const lxb_encoding_data_t,

    /* Out buffer */
    pub buffer_out : *mut core::lxb_char_t,
    pub buffer_length : c_uint,
    pub buffer_used : c_uint,

    /*
     * Bad code points will be replaced to user bytes.
     * If replace_to == NULL stop parsing and return error ot user.
     */
    pub replace_to : *const core::lxb_char_t,
    pub replace_len : c_uint,
    pub state : c_uint
}

pub type lxb_encoding_encode_f = extern "C" fn(ctx : *mut lxb_encoding_encode_t,
    *const *mut core::lxb_codepoint_t, end : *const core::lxb_codepoint_t) 
    -> core::lxb_status_t;

pub type lxb_encoding_decode_f = extern "C" fn(ctx : *mut lxb_encoding_decode_t,
    *const *mut core::lxb_char_t, end : *const core::lxb_char_t)
    -> core::lxb_status_t;

pub type lxb_encoding_encode_single_f = extern "C" fn(ctx : 
    *mut lxb_encoding_encode_t, data : *mut *mut core::lxb_char_t, end :
    *const core::lxb_char_t, cp : core::lxb_codepoint_t) -> i8;

pub type lxb_encoding_decode_single_f = extern "C" fn(ctx :
    *mut lxb_encoding_decode_t, data : *const *mut core::lxb_char_t, end :
    *const core::lxb_char_t) -> core::lxb_codepoint_t;

#[repr(C)]
pub struct lxb_encoding_data_t {
    pub encoding : lxb_encoding_t,
    pub encode : lxb_encoding_encode_f,
    pub decode : lxb_encoding_decode_f,
    pub encode_single : lxb_encoding_encode_single_f,
    pub decode_single : lxb_encoding_decode_single_f,
    pub name : *mut core::lxb_char_t
}

#[repr(C)]
pub struct lxb_encoding_single_index_t {
    pub name : *mut core::lxb_char_t,
    pub size : c_uint,
    pub codepoint : core::lxb_codepoint_t
}

pub type lxb_encoding_multi_index_t = lxb_encoding_single_index_t;

#[repr(C)]
pub struct lxb_encoding_range_index_t {
    pub index : c_uint,
    pub codepoint : core::lxb_codepoint_t
}

#[link(name = "lexbor")]
extern "C" {
    // lexbor/encoding/decode.h
    pub fn lxb_encoding_decode_default(ctx : *mut lxb_encoding_decode_t,
        data : *const *mut core::lxb_char_t, end : *const core::lxb_char_t)
        -> core::lxb_status_t;
    pub fn lxb_encoding_decode_auto(ctx : *mut lxb_encoding_decode_t,
        data : *const *mut core::lxb_char_t, end : *const core::lxb_char_t)
        -> core::lxb_status_t;
    pub fn lxb_encoding_decode_undefined(ctx : *mut lxb_encoding_decode_t,
        data : *const *mut core::lxb_char_t, end : *const core::lxb_char_t)
        -> core::lxb_status_t;        
    pub fn lxb_encoding_decode_big5(ctx : *mut lxb_encoding_decode_t,
        data : *const *mut core::lxb_char_t, end : *const core::lxb_char_t)
        -> core::lxb_status_t;
    pub fn lxb_encoding_decode_euc_jp(ctx : *mut lxb_encoding_decode_t,
        data : *const *mut core::lxb_char_t, end : *const core::lxb_char_t)
        -> core::lxb_status_t;
    pub fn lxb_encoding_decode_euc_kr(ctx : *mut lxb_encoding_decode_t,
        data : *const *mut core::lxb_char_t, end : *const core::lxb_char_t)
        -> core::lxb_status_t;
    pub fn lxb_encoding_decode_gbk(ctx : *mut lxb_encoding_decode_t,
        data : *const *mut core::lxb_char_t, end : *const core::lxb_char_t)
        -> core::lxb_status_t;
    pub fn lxb_encoding_decode_ibm866(ctx : *mut lxb_encoding_decode_t,
        data : *const *mut core::lxb_char_t, end : *const core::lxb_char_t)
        -> core::lxb_status_t;
    pub fn lxb_encoding_decode_iso_2022_jp(ctx : *mut lxb_encoding_decode_t,
        data : *const *mut core::lxb_char_t, end : *const core::lxb_char_t)
        -> core::lxb_status_t;
    pub fn lxb_encoding_decode_iso_8859_10(ctx : *mut lxb_encoding_decode_t,
        data : *const *mut core::lxb_char_t, end : *const core::lxb_char_t)
        -> core::lxb_status_t;    
    pub fn lxb_encoding_decode_iso_8859_13(ctx : *mut lxb_encoding_decode_t,
        data : *const *mut core::lxb_char_t, end : *const core::lxb_char_t)
        -> core::lxb_status_t;
    pub fn lxb_encoding_decode_iso_8859_14(ctx : *mut lxb_encoding_decode_t,
        data : *const *mut core::lxb_char_t, end : *const core::lxb_char_t)
        -> core::lxb_status_t;
    pub fn lxb_encoding_decode_iso_8859_15(ctx : *mut lxb_encoding_decode_t,
        data : *const *mut core::lxb_char_t, end : *const core::lxb_char_t)
        -> core::lxb_status_t;
    pub fn lxb_encoding_decode_iso_8859_16(ctx : *mut lxb_encoding_decode_t,
        data : *const *mut core::lxb_char_t, end : *const core::lxb_char_t)
        -> core::lxb_status_t;
    pub fn lxb_encoding_decode_iso_8859_2(ctx : *mut lxb_encoding_decode_t,
        data : *const *mut core::lxb_char_t, end : *const core::lxb_char_t)
        -> core::lxb_status_t;
    pub fn lxb_encoding_decode_iso_8859_3(ctx : *mut lxb_encoding_decode_t,
        data : *const *mut core::lxb_char_t, end : *const core::lxb_char_t)
        -> core::lxb_status_t;
    pub fn lxb_encoding_decode_iso_8859_4(ctx : *mut lxb_encoding_decode_t,
        data : *const *mut core::lxb_char_t, end : *const core::lxb_char_t)
        -> core::lxb_status_t;
    pub fn lxb_encoding_decode_iso_8859_5(ctx : *mut lxb_encoding_decode_t,
        data : *const *mut core::lxb_char_t, end : *const core::lxb_char_t)
        -> core::lxb_status_t;
    pub fn lxb_encoding_decode_iso_8859_6(ctx : *mut lxb_encoding_decode_t,
        data : *const *mut core::lxb_char_t, end : *const core::lxb_char_t)
        -> core::lxb_status_t;
    pub fn lxb_encoding_decode_iso_8859_7(ctx : *mut lxb_encoding_decode_t,
        data : *const *mut core::lxb_char_t, end : *const core::lxb_char_t)
        -> core::lxb_status_t;
    pub fn lxb_encoding_decode_iso_8859_8(ctx : *mut lxb_encoding_decode_t,
        data : *const *mut core::lxb_char_t, end : *const core::lxb_char_t)
        -> core::lxb_status_t;
    pub fn lxb_encoding_decode_iso_8859_8_i(ctx : *mut lxb_encoding_decode_t,
        data : *const *mut core::lxb_char_t, end : *const core::lxb_char_t)
        -> core::lxb_status_t;
    pub fn lxb_encoding_decode_koi8_r(ctx : *mut lxb_encoding_decode_t,
        data : *const *mut core::lxb_char_t, end : *const core::lxb_char_t)
        -> core::lxb_status_t;
    pub fn lxb_encoding_decode_koi8_u(ctx : *mut lxb_encoding_decode_t,
        data : *const *mut core::lxb_char_t, end : *const core::lxb_char_t)
        -> core::lxb_status_t;
    pub fn lxb_encoding_decode_shift_jis(ctx : *mut lxb_encoding_decode_t,
        data : *const *mut core::lxb_char_t, end : *const core::lxb_char_t)
        -> core::lxb_status_t;
    pub fn lxb_encoding_decode_uft_16be(ctx : *mut lxb_encoding_decode_t,
        data : *const *mut core::lxb_char_t, end : *const core::lxb_char_t)
        -> core::lxb_status_t;
    pub fn lxb_encoding_decode_utf_16le(ctx : *mut lxb_encoding_decode_t,
        data : *const *mut core::lxb_char_t, end : *const core::lxb_char_t)
        -> core::lxb_status_t;
    pub fn lxb_encoding_decode_utf_8(ctx : *mut lxb_encoding_decode_t,
        data : *const *mut core::lxb_char_t, end : *const core::lxb_char_t)
        -> core::lxb_status_t;
    pub fn lxb_encoding_decode_gb18030(ctx : *mut lxb_encoding_decode_t,
        data : *const *mut core::lxb_char_t, end : *const core::lxb_char_t)
        -> core::lxb_status_t;
    pub fn lxb_encoding_decode_macintosh(ctx : *mut lxb_encoding_decode_t,
        data : *const *mut core::lxb_char_t, end : *const core::lxb_char_t)
        -> core::lxb_status_t;
    pub fn lxb_encoding_decode_replacement(ctx : *mut lxb_encoding_decode_t,
        data : *const *mut core::lxb_char_t, end : *const core::lxb_char_t)
        -> core::lxb_status_t;
    pub fn lxb_encoding_decode_windows_1250(ctx : *mut lxb_encoding_decode_t,
        data : *const *mut core::lxb_char_t, end : *const core::lxb_char_t)
        -> core::lxb_status_t;
    pub fn lxb_encoding_decode_windows_1251(ctx : *mut lxb_encoding_decode_t,
        data : *const *mut core::lxb_char_t, end : *const core::lxb_char_t)
        -> core::lxb_status_t;
    pub fn lxb_encoding_decode_windows_1252(ctx : *mut lxb_encoding_decode_t,
        data : *const *mut core::lxb_char_t, end : *const core::lxb_char_t)
        -> core::lxb_status_t;
    pub fn lxb_encoding_decode_windows_1253(ctx : *mut lxb_encoding_decode_t,
        data : *const *mut core::lxb_char_t, end : *const core::lxb_char_t)
        -> core::lxb_status_t;
    pub fn lxb_encoding_decode_windows_1254(ctx : *mut lxb_encoding_decode_t,
        data : *const *mut core::lxb_char_t, end : *const core::lxb_char_t)
        -> core::lxb_status_t;
    pub fn lxb_encoding_decode_windows_1255(ctx : *mut lxb_encoding_decode_t,
        data : *const *mut core::lxb_char_t, end : *const core::lxb_char_t)
        -> core::lxb_status_t;
    pub fn lxb_encoding_decode_windows_1256(ctx : *mut lxb_encoding_decode_t,
        data : *const *mut core::lxb_char_t, end : *const core::lxb_char_t)
        -> core::lxb_status_t;
    pub fn lxb_encoding_decode_windows_1257(ctx : *mut lxb_encoding_decode_t,
        data : *const *mut core::lxb_char_t, end : *const core::lxb_char_t)
        -> core::lxb_status_t;
    pub fn lxb_encoding_decode_windows_1258(ctx : *mut lxb_encoding_decode_t,
        data : *const *mut core::lxb_char_t, end : *const core::lxb_char_t)
        -> core::lxb_status_t;
    pub fn lxb_encoding_decode_windows_874(ctx : *mut lxb_encoding_decode_t,
        data : *const *mut core::lxb_char_t, end : *const core::lxb_char_t)
        -> core::lxb_status_t;
    pub fn lxb_encoding_decode_x_mac_cyrillic(ctx : *mut lxb_encoding_decode_t,
        data : *const *mut core::lxb_char_t, end : *const core::lxb_char_t)
        -> core::lxb_status_t;
    pub fn lxb_encoding_decode_x_user_defined(ctx : *mut lxb_encoding_decode_t,
        data : *const *mut core::lxb_char_t, end : *const core::lxb_char_t)
        -> core::lxb_status_t;
    pub fn lxb_encoding_decode_default_single(ctx : *mut lxb_encoding_decode_t,
        data : *const *mut core::lxb_char_t, end : *const core::lxb_char_t)
        -> core::lxb_codepoint_t;
    pub fn lxb_encoding_decode_auto_single(ctx : *mut lxb_encoding_decode_t,
        data : *const *mut core::lxb_char_t, end : *const core::lxb_char_t)
        -> core::lxb_codepoint_t;
    pub fn lxb_encoding_decode_undefined_single(ctx : 
        *mut lxb_encoding_decode_t, data : *const *mut core::lxb_char_t, end : 
        *const core::lxb_char_t) -> core::lxb_codepoint_t;
    pub fn lxb_encoding_decode_big5_single(ctx : *mut lxb_encoding_decode_t,
        data : *const *mut core::lxb_char_t, end : *const core::lxb_char_t)
        -> core::lxb_codepoint_t;
    pub fn lxb_encoding_decode_euc_jp_single(ctx : *mut lxb_encoding_decode_t,
        data : *const *mut core::lxb_char_t, end : *const core::lxb_char_t)
        -> core::lxb_codepoint_t;
    pub fn lxb_encoding_decode_euc_kr_single(ctx : *mut lxb_encoding_decode_t,
        data : *const *mut core::lxb_char_t, end : *const core::lxb_char_t)
        -> core::lxb_codepoint_t;
    pub fn lxb_encoding_decode_gbk_single(ctx : *mut lxb_encoding_decode_t,
        data : *const *mut core::lxb_char_t, end : *const core::lxb_char_t)
        -> core::lxb_codepoint_t;
    pub fn lxb_encoding_decode_ibm866_single(ctx : *mut lxb_encoding_decode_t,
        data : *const *mut core::lxb_char_t, end : *const core::lxb_char_t)
        -> core::lxb_codepoint_t;
    pub fn lxb_encoding_decode_iso_2022_jp_single(ctx : 
        *mut lxb_encoding_decode_t, data : *const *mut core::lxb_char_t, end : 
        *const core::lxb_char_t) -> core::lxb_codepoint_t;
    pub fn lxb_encoding_decode_iso_8859_10_single(ctx : 
        *mut lxb_encoding_decode_t, data : *const *mut core::lxb_char_t, end : 
        *const core::lxb_char_t) -> core::lxb_codepoint_t;
    pub fn lxb_encoding_decode_iso_8859_13_single(ctx : 
        *mut lxb_encoding_decode_t, data : *const *mut core::lxb_char_t, end : 
        *const core::lxb_char_t) -> core::lxb_codepoint_t;
    pub fn lxb_encoding_decode_iso_8859_14_single(ctx : 
        *mut lxb_encoding_decode_t, data : *const *mut core::lxb_char_t, end : 
        *const core::lxb_char_t) -> core::lxb_codepoint_t;
    pub fn lxb_encoding_decode_iso_8859_15_single(ctx : 
        *mut lxb_encoding_decode_t, data : *const *mut core::lxb_char_t, end : 
        *const core::lxb_char_t) -> core::lxb_codepoint_t;
    pub fn lxb_encoding_decode_iso_8859_16_single(ctx : 
        *mut lxb_encoding_decode_t, data : *const *mut core::lxb_char_t, end : 
        *const core::lxb_char_t) -> core::lxb_codepoint_t;
    pub fn lxb_encoding_decode_iso_8859_2_single(ctx : 
        *mut lxb_encoding_decode_t, data : *const *mut core::lxb_char_t, end : 
        *const core::lxb_char_t) -> core::lxb_codepoint_t;
    pub fn lxb_encoding_decode_iso_8859_3_single(ctx : 
        *mut lxb_encoding_decode_t, data : *const *mut core::lxb_char_t, end : 
        *const core::lxb_char_t) -> core::lxb_codepoint_t;
    pub fn lxb_encoding_decode_iso_8859_4_single(ctx : 
        *mut lxb_encoding_decode_t, data : *const *mut core::lxb_char_t, end : 
        *const core::lxb_char_t) -> core::lxb_codepoint_t;
    pub fn lxb_encoding_decode_iso_8859_5_single(ctx : 
        *mut lxb_encoding_decode_t, data : *const *mut core::lxb_char_t, end : 
        *const core::lxb_char_t) -> core::lxb_codepoint_t;
    pub fn lxb_encoding_decode_iso_8859_6_single(ctx : 
        *mut lxb_encoding_decode_t, data : *const *mut core::lxb_char_t, end : 
        *const core::lxb_char_t) -> core::lxb_codepoint_t;
    pub fn lxb_encoding_decode_iso_8859_7_single(ctx : 
        *mut lxb_encoding_decode_t, data : *const *mut core::lxb_char_t, end : 
        *const core::lxb_char_t) -> core::lxb_codepoint_t;
    pub fn lxb_encoding_decode_iso_8859_8_single(ctx : 
        *mut lxb_encoding_decode_t, data : *const *mut core::lxb_char_t, end : 
        *const core::lxb_char_t) -> core::lxb_codepoint_t;
    pub fn lxb_encoding_decode_iso_8859_8_i_single(ctx : 
        *mut lxb_encoding_decode_t, data : *const *mut core::lxb_char_t, end : 
        *const core::lxb_char_t) -> core::lxb_codepoint_t;
    pub fn lxb_encoding_decode_koi8_r_single(ctx : 
        *mut lxb_encoding_decode_t, data : *const *mut core::lxb_char_t, end : 
        *const core::lxb_char_t) -> core::lxb_codepoint_t;
    pub fn lxb_encoding_decode_koi8_u_single(ctx : 
        *mut lxb_encoding_decode_t, data : *const *mut core::lxb_char_t, end : 
        *const core::lxb_char_t) -> core::lxb_codepoint_t;
    pub fn lxb_encoding_decode_shift_jis_single(ctx : 
        *mut lxb_encoding_decode_t, data : *const *mut core::lxb_char_t, end : 
        *const core::lxb_char_t) -> core::lxb_codepoint_t;
    pub fn lxb_encoding_decode_utf_16be_single(ctx : 
        *mut lxb_encoding_decode_t, data : *const *mut core::lxb_char_t, end : 
        *const core::lxb_char_t) -> core::lxb_codepoint_t; 
    pub fn lxb_encoding_decode_utf_16le_single(ctx : 
        *mut lxb_encoding_decode_t, data : *const *mut core::lxb_char_t, end : 
        *const core::lxb_char_t) -> core::lxb_codepoint_t;
    pub fn lxb_encoding_decode_utf_8_single(ctx : 
        *mut lxb_encoding_decode_t, data : *const *mut core::lxb_char_t, end : 
        *const core::lxb_char_t) -> core::lxb_codepoint_t;
    pub fn lxb_encoding_decode_gb18030_single(ctx : 
        *mut lxb_encoding_decode_t, data : *const *mut core::lxb_char_t, end : 
        *const core::lxb_char_t) -> core::lxb_codepoint_t;
    pub fn lxb_encoding_decode_macintosh_single(ctx : 
        *mut lxb_encoding_decode_t, data : *const *mut core::lxb_char_t, end : 
        *const core::lxb_char_t) -> core::lxb_codepoint_t;
    pub fn lxb_encoding_decode_replacement_single(ctx : 
        *mut lxb_encoding_decode_t, data : *const *mut core::lxb_char_t, end : 
        *const core::lxb_char_t) -> core::lxb_codepoint_t;
    pub fn lxb_encoding_decode_windows_1250_single(ctx : 
        *mut lxb_encoding_decode_t, data : *const *mut core::lxb_char_t, end : 
        *const core::lxb_char_t) -> core::lxb_codepoint_t;
    pub fn lxb_encoding_decode_windows_1251_single(ctx : 
        *mut lxb_encoding_decode_t, data : *const *mut core::lxb_char_t, end : 
        *const core::lxb_char_t) -> core::lxb_codepoint_t;
    pub fn lxb_encoding_decode_windows_1252_single(ctx : 
        *mut lxb_encoding_decode_t, data : *const *mut core::lxb_char_t, end : 
        *const core::lxb_char_t) -> core::lxb_codepoint_t;
    pub fn lxb_encoding_decode_windows_1253_single(ctx : 
        *mut lxb_encoding_decode_t, data : *const *mut core::lxb_char_t, end : 
        *const core::lxb_char_t) -> core::lxb_codepoint_t;
    pub fn lxb_encoding_decode_windows_1254_single(ctx : 
        *mut lxb_encoding_decode_t, data : *const *mut core::lxb_char_t, end : 
        *const core::lxb_char_t) -> core::lxb_codepoint_t;
    pub fn lxb_encoding_decode_windows_1255_single(ctx : 
        *mut lxb_encoding_decode_t, data : *const *mut core::lxb_char_t, end : 
        *const core::lxb_char_t) -> core::lxb_codepoint_t;
    pub fn lxb_encoding_decode_windows_1256_single(ctx : 
        *mut lxb_encoding_decode_t, data : *const *mut core::lxb_char_t, end : 
        *const core::lxb_char_t) -> core::lxb_codepoint_t;
    pub fn lxb_encoding_decode_windows_1257_single(ctx : 
        *mut lxb_encoding_decode_t, data : *const *mut core::lxb_char_t, end : 
        *const core::lxb_char_t) -> core::lxb_codepoint_t;
    pub fn lxb_encoding_decode_windows_1258_single(ctx : 
        *mut lxb_encoding_decode_t, data : *const *mut core::lxb_char_t, end : 
        *const core::lxb_char_t) -> core::lxb_codepoint_t;
    pub fn lxb_encoding_decode_windows_874_single(ctx : 
        *mut lxb_encoding_decode_t, data : *const *mut core::lxb_char_t, end : 
        *const core::lxb_char_t) -> core::lxb_codepoint_t;
    pub fn lxb_encoding_decode_x_mac_cyrillic_single(ctx : 
        *mut lxb_encoding_decode_t, data : *const *mut core::lxb_char_t, end : 
        *const core::lxb_char_t) -> core::lxb_codepoint_t;
    pub fn lxb_encoding_decode_x_user_defined_single(ctx : 
        *mut lxb_encoding_decode_t, data : *const *mut core::lxb_char_t, end : 
        *const core::lxb_char_t) -> core::lxb_codepoint_t;       
}