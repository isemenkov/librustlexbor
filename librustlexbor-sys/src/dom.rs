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

pub type lxb_dom_attr_id_t = usize;

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
pub enum lxb_dom_attr_id_enum_t {
    LXB_DOM_ATTR__UNDEF                                                = 0x0000,
    LXB_DOM_ATTR_ALT                                                   = 0x0001,
    LXB_DOM_ATTR_CHARSER                                               = 0x0002,
    LXB_DOM_ATTR_CHECKED                                               = 0x0003,
    LXB_DOM_ATTR_CLASS                                                 = 0x0004,
    LXB_DOM_ATTR_CONTENT                                               = 0x0005,
    LXB_DOM_ATTR_DIR                                                   = 0x0006,
    LXB_DOM_ATTR_DISABLED                                              = 0x0007,
    LXB_DOM_ATTR_FOR                                                   = 0x0008,
    LXB_DOM_ATTR_HEIGHT                                                = 0x0009,
    LXB_DOM_ATTR_HREF                                                  = 0x000a,
    LXB_DOM_ATTR_HTTP_EQUIV                                            = 0x000b,
    LXB_DOM_ATTR_ID                                                    = 0x000c,
    LXB_DOM_ATTR_IS                                                    = 0x000d,
    LXB_DOM_ATTR_MAXLENGTH                                             = 0x000e,
    LXB_DOM_ATTR_POOL                                                  = 0x000f,
    LXB_DOM_ATTR_SCHEME                                                = 0x0010,
    LXB_DOM_ATTR_SLOT                                                  = 0x0011,
    LXB_DOM_ATTR_SRC                                                   = 0x0012,
    LXB_DOM_ATTR_STYLE                                                 = 0x0013,
    LXB_DOM_ATTR_TITLE                                                 = 0x0014,
    LXB_DOM_ATTR_WIDTH                                                 = 0x0015,
    LXB_DOM_ATTR__LAST_ENTRY                                           = 0x0016
}

#[repr(C)]
pub struct lxb_dom_node_type_t {
    LXB_DOM_NODE_TYPE_UNDEF                                            = 0x00,
    LXB_DOM_NODE_TYPE_ELEMENT                                          = 0x01,
    LXB_DOM_NODE_TYPE_ATTRIBUTE                                        = 0x02,
    LXB_DOM_NODE_TYPE_TEXT                                             = 0x03,
    LXB_DOM_NODE_TYPE_CDATA_SECTION                                    = 0x04,
    LXB_DOM_NODE_TYPE_ENTITY_REFERENCE       /* historical */          = 0x05, 
    LXB_DOM_NODE_TYPE_ENTITY                 /* historical */          = 0x06, 
    LXB_DOM_NODE_TYPE_PROCESSING_INSTRUCTION                           = 0x07,
    LXB_DOM_NODE_TYPE_COMMENT                                          = 0x08,
    LXB_DOM_NODE_TYPE_DOCUMENT                                         = 0x09,
    LXB_DOM_NODE_TYPE_DOCUMENT_TYPE                                    = 0x0A,
    LXB_DOM_NODE_TYPE_DOCUMENT_FRAGMENT                                = 0x0B,
    LXB_DOM_NODE_TYPE_NOTATION               /* historical */          = 0x0C, 
    LXB_DOM_NODE_TYPE_LAST_ENTRY                                       = 0x0D
}

#[repr(C)]
pub enum lxb_dom_document_cmode_t {
    LXB_DOM_DOCUMENT_CMODE_NO_QUIRKS                                   = 0x00,
    LXB_DOM_DOCUMENT_CMODE_QUIRKS                                      = 0x01,
    LXB_DOM_DOCUMENT_CMODE_LIMITED_QUIRKS                              = 0x02
}

#[repr(C)]
pub enum lxb_dom_document_dtype_t {
    LXB_DOM_DOCUMENT_DTYPE_UNDEF                                       = 0x00,
    LXB_DOM_DOCUMENT_DTYPE_HTML                                        = 0x01,
    LXB_DOM_DOCUMENT_DTYPE_XML                                         = 0x02
}

#[repr(C)]
pub struct lxb_dom_event_target_t {
    pub events : *mut c_void;
}

#[repr(C)]
pub struct lxb_dom_node_t {
    pub event_target : lxb_dom_event_target_t,
    
    pub local_name : usize,
    pub prefix : usize,
    pub ns : usize,

    pub owner_document : *mut lxb_dom_document_t,

    pub next : *mut lxb_dom_node_t,
    pub prev : *mut lxb_dom_node_t,
    pub parent : *mut lxb_dom_node_t,
    pub first_child : *mut lxb_dom_node_t,
    pub last_child : *mut lxb_dom_node_t,
    pub user : *mut c_void,

    pub _type : lxb_dom_node_type_t
}

#[repr(C)]
pub struct lxb_dom_element_t {

}

#[repr(C)]
pub struct lxb_dom_attr_t {

}

#[repr(C)]
pub struct lxb_dom_document_t {
    pub node : lxb_dom_node_t,
    
    pub compat_mode : lxb_dom_document_cmode_t,
    pub _type : lxb_dom_document_dtype_t,

    pub doctype : *mut lxb_dom_document_type_t,
    pub element : *mut lxb_dom_element_t,

    pub create_interface : lxb_dom_interface_create_f,
    pub destroy_interface : lxb_dom_interface_destroy_f,

    pub mraw : *mut core::lexbor_mraw_t,
    pub text : *mut core::lexbor_mraw_t,
    pub tags : *mut core::lexbor_hash_t,
    pub attrs : *mut core::lexbor_hash_t,
    pub prefix : *mut core::lexbor_hash_t,
    pub ns : *mut core::lexbor_hash_t,

    pub parser : *mut c_void,
    pub user : *mut c_void,

    pub tags_inherited : bool,
    pub ns_inherited : bool,

    pub scripting : bool
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

pub type lxb_dom_node_simple_walker_f = extern "C" fn(node : 
    *mut lxb_dom_node_t, ctx : *mut c_void) -> core::lexbor_action_t;

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
        
    // lexbor/dom/interfaces/event_target.h
    pub fn lxb_dom_event_target_create(document : *mut lxb_dom_document_t)
        -> *mut lxb_dom_event_target_t;
    pub fn lxb_dom_event_target_destroy(event_target : 
        *mut lxb_dom_event_target_t, document : *mut lxb_dom_document_t)
        -> *mut lxb_dom_event_target_t;
    
    // lexbor/dom/interfaces/node.h
    pub fn lxb_dom_node_interface_create(document : *mut lxb_dom_document_t)
        -> *mut lxb_dom_node_t;
    pub fn lxb_dom_node_interface_destroy(node : *mut lxb_dom_node_t)
        -> *mut lxb_dom_node_t;
    pub fn lxb_dom_node_destroy(node : *mut lxb_dom_node_t) 
        -> *mut lxb_dom_node_t;
    pub fn lxb_dom_node_destroy_deep(root : *mut lxb_dom_node_t)
        -> *mut lxb_dom_node_t;
    pub fn lxb_dom_node_name(node : *mut lxb_dom_node_t, len : *mut c_uint)
        -> *const core::lxb_char_t;
    pub fn lxb_dom_node_insert_child(to : *mut lxb_dom_node_t, node :
        *mut lxb_dom_node_t) -> ();
    pub fn lxb_dom_node_insert_before(to : *mut lxb_dom_node_t, node : 
        *mut lxb_dom_node_t) -> ();
    pub fn lxb_dom_node_insert_after(to : *mut lxb_dom_node_t, node :
        *mut lxb_dom_node_t) -> ();
    pub fn lxb_dom_node_remove(node : *mut lxb_dom_node_t) -> ();
    pub fn lxb_dom_node_replace_all(parent : *mut lxb_dom_node_t, node :
        *mut lxb_dom_node_t) -> core::lxb_status_t;
    pub fn lxb_dom_node_simple_walk(root : *mut lxb_dom_node_t, walker_cb :
        lxb_dom_node_simple_walker_f, ctx : *mut c_void) -> ();
    pub fn lxb_dom_node_text_content(node : *mut lxb_dom_node_t, len :
        *mut c_uint) -> *mut core::lxb_char_t;
    pub fn lxb_dom_text_content_set(node : *mut lxb_dom_node_t, content :
        *const core::lxb_char_t, len : c_uint) -> core::lxb_status_t;
    pub fn lxb_dom_node_tag_id_noi(node : *mut lxb_dom_node_t) 
        -> tag::lxb_tag_id_t;
    pub fn lxb_dom_node_next_noi(node : *mut lxb_dom_node_t)
        -> *mut lxb_dom_node_t;
    pub fn lxb_dom_node_prev_noi(node : *mut lxb_dom_node_t)
        -> *mut lxb_dom_node_t;
    pub fn lxb_dom_node_parent_noi(node : *mut lxb_dom_node_t)
        -> *mut lxb_dom_node_t;
    pub fn lxb_dom_node_first_child_noi(node : *mut lxb_dom_node_t)
        -> *mut lxb_dom_node_t;
    pub fn lxb_dom_node_last_child_noi(node : *mut lxb_dom_node_t)
        -> *mut lxb_dom_node_t;

    // lexbor/dom/interfaces/document.h
    pub fn lxb_dom_document_interface_create(document : *mut lxb_dom_document_t)
        -> *mut lxb_dom_document_t;
    pub fn lxb_dom_document_interface_destroy(document : 
        *mut lxb_dom_document_t) -> *mut lxb_dom_document_t;
    pub fn lxb_dom_document_create(owner : *mut lxb_dom_document_t)
        -> *mut lxb_dom_document_t;
    
}