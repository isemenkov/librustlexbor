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

#[repr(C)]
pub enum lxb_ns_id_enum_t {
    LXB_NS__UNDEF                                                      = 0x00,
    LXB_NS__ANY                                                        = 0x01,
    LXB_NS_HTML                                                        = 0x02,
    LXB_NS_MATH                                                        = 0x03,
    LXB_NS_SVG                                                         = 0x04,
    LXB_NS_XLINK                                                       = 0x05,
    LXB_NS_XML                                                         = 0x06,
    LXB_NS_XMLNS                                                       = 0x07,
    LXB_NS__LAST_ENTRY                                                 = 0x08       
}

pub type lxb_ns_id_t = usize;
pub type lxb_ns_prefix_id_t = usize;

#[repr(C)]
pub struct lxb_ns_data_t {
    pub entry : core::lexbor_hash_entry_t,

    pub ns_id : lxb_ns_id_t,
    pub ref_count : c_uint,
    pub read_only : bool
}

#[repr(C)]
pub struct lxb_ns_prefix_data_t {
    pub entry : core::lexbor_hash_entry_t,

    pub prefix_id : lxb_ns_prefix_id_t,
    pub ref_count : c_uint,
    pub read_only : bool
}

#[link(name = "lexbor")] 
extern "C" {
    // lexbor/ns/ns.h
    pub fn lxb_ns_by_id(hash : *mut core::lexbor_hash_t, ns_id : lxb_ns_id_t,
        length : *mut c_uint) -> *const core::lxb_char_t;
    pub fn lxb_ns_data_by_id(hash : *mut core::lexbor_hash_t, ns_id : 
        lxb_ns_id_t) -> *const lxb_ns_data_t;
    pub fn lxb_ns_data_by_link(hash : *mut core::lexbor_hash_t, name :
        *const core::lxb_char_t, length : c_uint) -> *const lxb_ns_data_t;
    pub fn lxb_ns_prefix_append(hash : *mut core::lexbor_hash_t, prefix :
        *const core::lxb_char_t, length : c_uint) 
        -> *const lxb_ns_prefix_data_t;
    pub fn lxb_ns_prefix_data_by_id(hash : *mut core::lexbor_hash_t, prefix_id :
        lxb_ns_prefix_id_t) -> *const lxb_ns_prefix_data_t;
    pub fn lxb_ns_prefix_data_by_name(hash : *mut core::lexbor_hash_t, name :
        *const core::lxb_char_t, length : c_uint) 
        -> *const lxb_ns_prefix_data_t;
}