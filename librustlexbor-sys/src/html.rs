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
#[path="tag.rs"] pub mod tag;
extern crate libc;

use libc::{c_uint};
use std::os::raw::c_void;

#[repr(C)]
pub enum lxb_html_status_t {
    LXB_HTML_STATUS_OK                                                 = 0x0000
}

#[repr(C)]
pub struct lxb_html_encoding_entry_t {
    pub name : *const core::lxb_char_t,
    pub end : *const core::lxb_char_t
}

#[repr(C)]
pub struct lxb_html_encoding_t {
    pub cache : core::lexbor_array_obj_t,
    pub result : core::lexbor_array_obj_t
}

#[repr(C)]
pub struct lxb_html_document_t {

}

#[repr(C)]
pub struct lxb_html_anchor_element_t {

}

#[repr(C)]
pub struct lxb_html_area_element_t {

}

#[repr(C)]
pub struct lxb_html_audio_element_t {

}

#[repr(C)]
pub struct lxb_html_br_element_t {

}

#[repr(C)]
pub struct lxb_html_base_element_t {

}

#[repr(C)]
pub struct lxb_html_body_element_t {
    
}

#[link(name = "lexbor")]
extern "C" {
    // lexbor/html/encoding.h
    pub fn lxb_html_encoding_init(em : *mut lxb_html_encoding_t) 
        -> core::lxb_status_t;
    pub fn lxb_html_encoding_destroy(em : *mut lxb_html_encoding_t, 
        self_destroy : bool) -> *mut lxb_html_encoding_t;
    pub fn lxb_html_encoding_determine(em : *mut lxb_html_encoding_t, data :
        *const core::lxb_char_t, end : *const core::lxb_char_t) 
        -> core::lxb_status_t;
    pub fn lxb_html_encoding_content(data : *const core::lxb_char_t, end :
        *const core::lxb_char_t, name_end : *const *mut core::lxb_char_t)
        -> *const core::lxb_char_t;
    pub fn lxb_html_encoding_create_noi() -> *mut lxb_html_encoding_t;
    pub fn lxb_html_encoding_clean_noi(em : *mut lxb_html_encoding_t) -> ();
    pub fn lxb_html_encoding_meta_entry_noi(em : *mut lxb_html_encoding_t, idx :
        c_uint) -> *mut lxb_html_encoding_entry_t;
    pub fn lxb_html_encoding_meta_length_noi(em : *mut lxb_html_encoding_t)
        -> c_uint;
    pub fn lxb_html_encoding_meta_result_noi(em : *mut lxb_html_encoding_t)
        -> *mut core::lexbor_array_obj_t;
    
    // lexbor/html/in.h
    pub fn lxb_html_in_make(node : *mut core::lexbor_in_node_t, begin :
        *const core::lxb_char_t, end : *const core::lxb_char_t, _str :
        *mut core::lexbor_str_t, mraw : *mut core::lexbor_mraw_t)
        -> core::lxb_status_t;
    pub fn lxb_html_in_tag_id(node : *mut core::lexbor_in_node_t, hash :
        *mut core::lexbor_hash_t, begin : *const core::lxb_char_t, end : 
        *const core::lxb_char_t, mraw : *mut core::lexbor_mraw_t)
        -> tag::lxb_tag_id_t;
    pub fn lxb_html_in_ncasecmp(node : *mut core::lexbor_in_node_t, begin :
        *const core::lxb_char_t, end : *const core::lxb_char_t, data :
        *const core::lxb_char_t, len : c_uint) -> bool;
    
}