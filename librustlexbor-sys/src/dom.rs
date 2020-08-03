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
pub mod ns;
pub mod tag;
extern crate libc;

use libc::{c_uint};
use std::os::raw::c_void;

#[repr(C)]
pub enum lxb_dom_exception_code_t {
    LXB_DOM_INDEX_SIZE_ERR                                             = 0x00,
    LXB_DOM_DOMSTRING_SIZE_ERR,
    LXB_DOM_HIERARCHY_REQUEST_ERR,
    LXB_DOM_WRONG_DOCUMENT_ERR,
    LXB_DOM_INVALID_CHARACTER_ERR,
    LXB_DOM_NO_DATA_ALLOWED_ERR,
    LXB_DOM_NO_MODIFICATION_ALLOWED_ERR,
    LXB_DOM_NOT_FOUND_ERR,
    LXB_DOM_NOT_SUPPORTED_ERR,
    LXB_DOM_INUSE_ATTRIBUTE_ERR,
    LXB_DOM_INVALID_STATE_ERR,
    LXB_DOM_SYNTAX_ERR,
    LXB_DOM_INVALID_MODIFICATION_ERR,
    LXB_DOM_NAMESPACE_ERR,
    LXB_DOM_INVALID_ACCESS_ERR,
    LXB_DOM_VALIDATION_ERR,
    LXB_DOM_TYPE_MISMATCH_ERR,
    LXB_DOM_SECURITY_ERR,
    LXB_DOM_NETWORK_ERR,
    LXB_DOM_ABORT_ERR,
    LXB_DOM_URL_MISMATCH_ERR,
    LXB_DOM_QUOTA_EXCEEDED_ERR,
    LXB_DOM_TIMEOUT_ERR,
    LXB_DOM_INVALID_NODE_TYPE_ERR,
    LXB_DOM_DATA_CLONE_ERR
}

#[repr(C)]
pub struct lxb_dom_event_target_t {

}

#[repr(C)]
pub struct lxb_dom_node_t {

}

#[repr(C)]
pub struct lxb_dom_element_t {

}

#[repr(C)]
pub struct lxb_dom_attr_t {

}

#[repr(C)]
pub struct lxb_dom_document_t {

}

#[repr(C)]
pub struct lxb_dom_document_type_t {

}

#[repr(C)]
pub struct lxb_dom_document_fragment_t {

}

#[repr(C)]
pub struct lxb_dom_shadow_root_t {

}

#[repr(C)]
pub struct lxb_dom_character_data_t {

}

#[repr(C)]
pub struct lxb_dom_text_t {

}

#[repr(C)]
pub struct lxb_dom_cdata_section_t {

}

#[repr(C)]
pub struct lxb_dom_processing_instruction_t {

}

#[repr(C)]
pub struct lxb_dom_comment_t {

}

#[repr(C)]
pub struct lxb_dom_collection_t {
    pub array : core::lexbor_array_t,
    pub document : *mut lxb_dom_document_t 
}

pub type lxb_dom_interface_t = c_void;

pub type lxb_dom_interface_constructor_f = extern "C" fn(document : *mut c_void)
    -> *mut c_void;

pub type lxb_dom_interface_destructor_f = extern "C" fn(intrfc : *mut c_void)
    -> *mut c_void;

pub type lxb_dom_interface_create_f = extern "C" fn(document : 
    *mut lxb_dom_document_t, tag_id : tag::lxb_tag_id_t, _ns : ns::lxb_ns_id_t)
    -> *mut lxb_dom_interface_t;

pub type lxb_dom_interface_destroy_f = extern "C" fn(intrfc : 
    *mut lxb_dom_interface_t) -> *mut lxb_dom_interface_t;

#[link(name = "lexbor")]
extern "C" {
    // lexbor/dom/exception.h
    pub fn lxb_dom_exception_code_ref_set_noi(var : 
        *mut lxb_dom_exception_code_t, code : lxb_dom_exception_code_t)
        -> *mut c_void;
    
    // lexbor/dom/interface.h
    pub fn lxb_dom_interface_create(document : *mut lxb_dom_document_t, tag_id :
        tag::lxb_tag_id_t, _ns : ns::lxb_ns_id_t) -> *mut lxb_dom_interface_t;
    pub fn lxb_dom_interface_destroy(intrfc : *mut lxb_dom_interface_t)
        -> *mut lxb_dom_interface_t;

    // lexbor/dom/collection.h
    pub fn lxb_dom_collection_create(document : *mut lxb_dom_document_t)
        -> *mut lxb_dom_collection_t;
    pub fn lxb_dom_collection_init(col : *mut lxb_dom_collection_t,
        start_list_size : c_uint) -> core::lxb_status_t;
    pub fn lxb_dom_collection_destroy(col : *mut lxb_dom_collection_t,
        self_destroy : bool) -> *mut lxb_dom_collection_t;
    pub fn lxb_dom_collection_make_noi(document : *mut lxb_dom_document_t,
        start_list_size : c_uint) -> *mut lxb_dom_collection_t;
    pub fn lxb_dom_collection_clean_noi(col : *mut lxb_dom_collection_t) -> ();
    pub fn lxb_dom_collection_append_noi(col : *mut lxb_dom_collection_t,
        value : *mut c_void) -> core::lxb_status_t;
    pub fn lxb_dom_collection_element_noi(col : *mut lxb_dom_collection_t, idx :
        c_uint) -> *mut lxb_dom_element_t;
    pub fn lxb_dom_collection_node_noi(col : *mut lxb_dom_collection_t, idx :
        c_uint) -> *mut lxb_dom_node_t;
    pub fn lxb_dom_collection_length_noi(col : *mut lxb_dom_collection_t) 
        -> c_uint;    
}