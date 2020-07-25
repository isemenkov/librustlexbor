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

use libc::{c_char, c_uint, c_double};
use std::os::raw::c_void;

#[repr(C)]
pub struct lxb_utils_http_version_t {
    pub name : core::lexbor_str_t,
    pub number : c_double,
    pub status : c_uint
}

#[repr(C)]
pub struct lxb_utils_http_field_t {
    pub name : core::lexbor_str_t,
    pub value : core::lexbor_str_t
}

#[repr(C)]
pub struct lxb_utils_http_t {
    pub mraw : core::lexbor_mraw_t,
    pub fields : core::lexbor_array_obj_t,

    pub tmp : core::lexbor_str_t,
    pub version : lxb_utils_http_version_t,

    pub error : *const c_char,
    pub state : c_uint 
}

#[repr(C)]
pub struct lxb_utils_warc_version_t {
    pub _type : core::lexbor_str_t,
    pub number : c_double
}

#[repr(C)]
pub struct lxb_utils_warc_field_t {
    pub name : core::lexbor_str_t,
    pub value : core::lexbor_str_t
}

#[repr(C)]
pub struct lxb_utils_warc_t {
    pub mraw : *mut core::lexbor_mraw_t,
    pub fields : *mut core::lexbor_array_obj_t,

    pub tmp : core::lexbor_str_t,
    pub version : lxb_utils_warc_version_t,

    pub header_cb : lxb_utils_warc_header_cb_f,
    pub content_cb : lxb_utils_warc_content_cb_f,
    pub content_end_cb : lxb_utils_warc_content_end_cb_f,
    pub ctx : *mut c_void,

    pub content_length : c_uint,
    pub content_read : c_uint,
    pub count : c_uint,

    pub error : *const c_char,
    pub state : c_uint,
    pub ends : c_uint,
    pub skip : bool
}

pub type lxb_utils_warc_header_cb_f = extern "C" fn(warc : 
    *mut lxb_utils_warc_t) -> core::lxb_status_t;

pub type lxb_utils_warc_content_cb_f = extern "C" fn(warc :
    *mut lxb_utils_warc_t, data : *const core::lxb_char_t, end : 
    *const core::lxb_char_t) -> core::lxb_status_t;

pub type lxb_utils_warc_content_end_cb_f = extern "C" fn(warc : 
    *mut lxb_utils_warc_t) -> core::lxb_status_t;

#[link(name = "lexbor")]
extern "C" {
    // lexbor/utils/http.h
    pub fn lxb_utils_http_create() -> *mut lxb_utils_http_t;
    pub fn lxb_utils_http_init(http : *mut lxb_utils_http_t, mraw : 
        *mut core::lexbor_mraw_t) -> core::lxb_char_t;
    pub fn lxb_utils_http_clear(http : *mut lxb_utils_http_t) 
        -> core::lxb_status_t;
    pub fn lxb_utils_http_destroy(http : *mut lxb_utils_http_t, self_destroy :
        bool) -> *mut lxb_utils_http_t;
    pub fn lxb_utils_http_parse(http : *mut lxb_utils_http_t, data :
        *const *mut core::lxb_char_t, end : *const core::lxb_char_t) 
        -> core::lxb_status_t;
    pub fn lxb_utils_http_header_parse_eof(http : *mut lxb_utils_http_t)
        -> core::lxb_status_t;
    pub fn lxb_utils_http_header_field(http : *mut lxb_utils_http_t, name :
        *const core::lxb_char_t, len : c_uint, offset : c_uint) 
        -> *mut lxb_utils_http_field_t;
    pub fn lxb_utils_http_header_serialize(http : *mut lxb_utils_http_t, _str :
        *mut core::lexbor_str_t) -> core::lxb_status_t;
    pub fn lxb_utils_http_field_serialize(http : *mut lxb_utils_http_t, _str :
        *mut core::lexbor_str_t, field : *const lxb_utils_http_field_t)
        -> core::lxb_status_t;

    // lexbor/utils/warc.h
    pub fn lxb_utils_warc_create() -> *mut lxb_utils_warc_t;
    pub fn lxb_utils_warc_init(warc : *mut lxb_utils_warc_t, h_cd :
        lxb_utils_warc_header_cb_f, c_cb : lxb_utils_warc_content_cb_f,
        c_end_cb : lxb_utils_warc_content_end_cb_f, ctx : *mut c_void)
        -> core::lxb_status_t;
    pub fn lxb_utils_warc_clear(warc : *mut lxb_utils_warc_t) 
        -> core::lxb_status_t;
    pub fn lxb_utils_warc_destroy(warc : *mut lxb_utils_warc_t, self_destroy :
        bool) -> core::lxb_status_t;
    pub fn lxb_utils_warc_parse(warc : *mut lxb_utils_warc_t, data :
        *const *mut core::lxb_char_t, end : *const core::lxb_char_t)
        -> core::lxb_status_t;
    pub fn lxb_utils_warc_parse_eof(warc : *mut lxb_utils_warc_t) 
        -> core::lxb_status_t;
    pub fn lxb_utils_warc_header_field(warc : *mut lxb_utils_warc_t, name :
        *const core::lxb_char_t, len : c_uint, offset : c_uint) 
        -> *mut lxb_utils_warc_field_t;
    pub fn lxb_utils_warc_header_serialize(warc : *mut lxb_utils_warc_t, _str :
        *mut core::lexbor_str_t) -> core::lxb_status_t;
    pub fn lxb_utils_warc_content_length_noi(warc : *mut lxb_utils_warc_t) 
        -> c_uint;    
}
