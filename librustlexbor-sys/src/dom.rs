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
#[path="ns.rs"] pub mod ns;
#[path="tag.rs"] pub mod tag;
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
    LXB_DOM_ATTR_COLOR                                                 = 0x0005,
    LXB_DOM_ATTR_CONTENT                                               = 0x0006,
    LXB_DOM_ATTR_DIR                                                   = 0x0007,
    LXB_DOM_ATTR_DISABLED                                              = 0x0008,
    LXB_DOM_ATTR_FACE                                                  = 0x0009,
    LXB_DOM_ATTR_FOR                                                   = 0x000a,
    LXB_DOM_ATTR_HEIGHT                                                = 0x000b,
    LXB_DOM_ATTR_HREF                                                  = 0x000c,
    LXB_DOM_ATTR_HTML                                                  = 0x000d,
    LXB_DOM_ATTR_HTTP_EQUIV                                            = 0x000e,
    LXB_DOM_ATTR_ID                                                    = 0x000f,
    LXB_DOM_ATTR_IS                                                    = 0x0010,
    LXB_DOM_ATTR_MAXLENGTH                                             = 0x0011,
    LXB_DOM_ATTR_POOL                                                  = 0x0012,
    LXB_DOM_ATTR_PUBLIC                                                = 0x0013,
    LXB_DOM_ATTR_SCHEME                                                = 0x0014,
    LXB_DOM_ATTR_SIZE                                                  = 0x0015,
    LXB_DOM_ATTR_SLOT                                                  = 0x0016,
    LXB_DOM_ATTR_SRC                                                   = 0x0017,
    LXB_DOM_ATTR_STYLE                                                 = 0x0018,
    LXB_DOM_ATTR_SYSTEM                                                = 0x0019,
    LXB_DOM_ATTR_TITLE                                                 = 0x001a,
    LXB_DOM_ATTR_TYPE                                                  = 0x001b,
    LXB_DOM_ATTR_WIDTH                                                 = 0x001c,
    LXB_DOM_ATTR__LAST_ENTRY                                           = 0x001d
}

#[repr(C)]
pub enum lxb_dom_node_type_t {
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
pub enum lxb_dom_element_custom_state_t {
    LXB_DOM_ELEMENT_CUSTOM_STATE_UNDEFINED                             = 0x00,
    LXB_DOM_ELEMENT_CUSTOM_STATE_FAILED                                = 0x01,
    LXB_DOM_ELEMENT_CUSTOM_STATE_UNCUSTOMIZED                          = 0x02,
    LXB_DOM_ELEMENT_CUSTOM_STATE_CUSTOM                                = 0x03
}

#[repr(C)]
pub enum lxb_dom_shadow_root_mode_t {
    LXB_DOM_SHADOW_ROOT_MODE_OPEN                                      = 0x00,
    LXB_DOM_SHADOW_ROOT_MODE_CLOSED                                    = 0x01
}

#[repr(C)]
pub struct lxb_dom_event_target_t {
    pub events : *mut c_void
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
    pub node : lxb_dom_node_t,
    pub upper_name : lxb_dom_attr_id_t,
    pub qualified_name : lxb_dom_attr_id_t,
    pub is_value : *mut core::lexbor_str_t,
    pub first_attr : lxb_dom_attr_t,
    pub last_attr : lxb_dom_attr_t,
    pub attr_id : lxb_dom_attr_t,
    pub attr_class : lxb_dom_attr_t,
    pub custom_state : lxb_dom_element_custom_state_t
}

#[repr(C)]
pub struct lxb_dom_attr_data_t {
    pub entry : core::lexbor_hash_entry_t,
    pub attr_id : lxb_dom_attr_id_t,
    pub ref_count : c_uint,
    pub read_only : bool
}

#[repr(C)]
pub struct lxb_dom_attr_t {
    pub node : lxb_dom_node_t,

    pub upper_name : lxb_dom_attr_id_t,
    pub qualified_name : lxb_dom_attr_id_t,

    pub value : *mut core::lexbor_str_t,

    pub owner : *mut lxb_dom_element_t,

    pub next : *mut lxb_dom_attr_t,
    pub prev : *mut lxb_dom_attr_t
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
    pub node : lxb_dom_node_t,

    pub name : lxb_dom_attr_id_t,
    pub public_id : core::lexbor_str_t,
    pub system_id : core::lexbor_str_t
}

#[repr(C)]
pub struct lxb_dom_document_fragment_t {
    pub node : lxb_dom_node_t,

    pub host : *mut lxb_dom_element_t
}

#[repr(C)]
pub struct lxb_dom_shadow_root_t {
    pub document_fragment : lxb_dom_document_fragment_t,

    pub mode : lxb_dom_shadow_root_mode_t,
    pub host : *mut lxb_dom_element_t
}

#[repr(C)]
pub struct lxb_dom_character_data_t {
    pub node : lxb_dom_node_t,

    pub data : core::lexbor_str_t
}

#[repr(C)]
pub struct lxb_dom_text_t {
    pub char_data : lxb_dom_character_data_t
}

#[repr(C)]
pub struct lxb_dom_cdata_section_t {
    pub text : lxb_dom_text_t
}

#[repr(C)]
pub struct lxb_dom_processing_instruction_t {
    pub char_data : lxb_dom_character_data_t,
    pub target : core::lexbor_str_t
}

#[repr(C)]
pub struct lxb_dom_comment_t {
    pub char_data : lxb_dom_character_data_t
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
    pub fn lxb_dom_document_init(document : *mut lxb_dom_document_t, owner :
        *mut lxb_dom_document_t, create_interface : lxb_dom_interface_create_f,
        destroy_interface : lxb_dom_interface_destroy_f, _type : 
        lxb_dom_document_dtype_t, ns : c_uint) -> core::lxb_status_t;
    pub fn lxb_dom_document_clean(document : *mut lxb_dom_document_t)
        -> core::lxb_status_t;
    pub fn lxb_dom_document_destroy(document : *mut lxb_dom_document_t)
        -> *mut lxb_dom_document_t;
    pub fn lxb_dom_document_attach_doctype(document : *mut lxb_dom_document_t,
        doctype : *mut lxb_dom_document_type_t) -> ();
    pub fn lxb_dom_document_attach_element(document : *mut lxb_dom_document_t,
        element : *mut lxb_dom_element_t) -> ();
    pub fn lxb_dom_document_create_element(document : *mut lxb_dom_document_t,
        local_name : *const core::lxb_char_t, lname_len : c_uint,
        reserved_for_opt : *mut c_void) -> *mut lxb_dom_element_t;
    pub fn lxb_dom_document_destroy_element(element : *mut lxb_dom_element_t)
        -> *mut lxb_dom_element_t;
    pub fn lxb_dom_document_create_document_fragment(document :
        *mut lxb_dom_document_t) -> *mut lxb_dom_document_fragment_t;
    pub fn lxb_dom_document_create_text_node(document : *mut lxb_dom_document_t,
        data : *const core::lxb_char_t, len : c_uint) -> *mut lxb_dom_text_t;
    pub fn lxb_dom_document_create_cdata_section(document : 
        *mut lxb_dom_document_t, data : *const core::lxb_char_t, len : c_uint)
        -> *mut lxb_dom_cdata_section_t;
    pub fn lxb_dom_document_create_processing_instruction(document : 
        *mut lxb_dom_document_t, target : *const core::lxb_char_t, target_len :
        c_uint, data : *const core::lxb_char_t, data_len : c_uint)
        -> *mut lxb_dom_processing_instruction_t;
    pub fn lxb_dom_document_create_comment(document : *mut lxb_dom_document_t,
        data : *const core::lxb_char_t, len : c_uint) 
        -> *mut lxb_dom_comment_t;
    pub fn lxb_dom_document_create_interface_noi(document : 
        *mut lxb_dom_document_t, tag_id : tag::lxb_tag_id_t, _ns : 
        ns::lxb_ns_id_t) -> *mut lxb_dom_interface_t;
    pub fn lxb_dom_document_destroy_interface_noi(intrfc :
        *mut lxb_dom_interface_t) -> *mut lxb_dom_interface_t;
    pub fn lxb_dom_document_create_struct_noi(document : 
        *mut lxb_dom_document_t, struct_size : c_uint) -> *mut c_void;
    pub fn lxb_dom_document_destroy_struct_noi(document : 
        *mut lxb_dom_document_t, structure : *mut c_void) -> *mut c_void;
    pub fn lxb_dom_document_create_text_noi(document : *mut lxb_dom_document_t,
        len : c_uint) -> *mut core::lxb_char_t;
    pub fn lxb_dom_document_destroy_text_noi(document : *mut lxb_dom_document_t,
        text : *mut core::lxb_char_t) -> *mut c_void;
    pub fn lxb_dom_document_element_noi(document : *mut lxb_dom_document_t)
        -> *mut lxb_dom_element_t;

    // lexbor/dom/interfaces/document_type.h
    pub fn lxb_dom_document_type_interface_create(document : 
        *mut lxb_dom_document_t) -> *mut lxb_dom_document_type_t;
    pub fn lxb_dom_document_type_interface_destroy(document_type : 
        *mut lxb_dom_document_type_t) -> *mut lxb_dom_document_type_t;
    pub fn lxb_dom_document_type_name_noi(doc_type : 
        *mut lxb_dom_document_type_t, len : *mut c_uint) 
        -> *const core::lxb_char_t;
    pub fn lxb_dom_document_type_public_id_noi(doc_type :
        *mut lxb_dom_document_type_t, len : *mut c_uint) 
        -> *const core::lxb_char_t;
    pub fn lxb_dom_document_type_system_id_noi(doc_type :
        *mut lxb_dom_document_type_t, len : *mut c_uint)
        -> *const core::lxb_char_t;

    // lexbor/dom/interfaces/attr.h
    pub fn lxb_dom_attr_interface_create(document : *mut lxb_dom_document_t)
        -> *mut lxb_dom_attr_t;
    pub fn lxb_dom_attr_interface_destroy(attr : *mut lxb_dom_attr_t)
        -> *mut lxb_dom_attr_t;
    pub fn lxb_dom_attr_set_name(attr : *mut lxb_dom_attr_t, local_name :
        *const core::lxb_char_t, local_name_len : c_uint, to_lowercase : bool)
        -> core::lxb_status_t;
    pub fn lxb_dom_attr_set_value(attr : *mut lxb_dom_attr_t, value : 
        *const core::lxb_char_t, value_len : c_uint) -> core::lxb_status_t;
    pub fn lxb_dom_attr_set_value_wo_copy(attr : *mut lxb_dom_attr_t, value :
        *mut core::lxb_char_t, value_len : c_uint) -> core::lxb_status_t;
    pub fn lxb_dom_attr_set_existing_value(attr : *mut lxb_dom_attr_t, value :
        *const core::lxb_char_t, value_len : c_uint) -> core::lxb_status_t;
    pub fn lxb_dom_attr_clone_name_value(attr_from : *mut lxb_dom_attr_t,
        attr_to : *mut lxb_dom_attr_t) -> core::lxb_status_t;
    pub fn lxb_dom_attr_compare(first : *mut lxb_dom_attr_t, second :
        *mut lxb_dom_attr_t) -> bool;
    pub fn lxb_dom_attr_data_by_id(hash : *mut core::lexbor_hash_t, attr_id :
        lxb_dom_attr_id_t) -> *mut lxb_dom_attr_data_t;
    pub fn lxb_dom_attr_data_by_local_name(hash : *mut core::lexbor_hash_t,
        name : *const core::lxb_char_t, length : c_uint) 
        -> *const lxb_dom_attr_data_t;
    pub fn lxb_dom_attr_data_by_qualified_name(hash : core::lexbor_hash_t,
        name : *const core::lxb_char_t, length : c_uint)
        -> *const lxb_dom_attr_data_t;
    pub fn lxb_dom_attr_qualified_name(attr : *mut lxb_dom_attr_t, len :
        *mut c_uint) -> *const core::lxb_char_t;
    pub fn lxb_dom_attr_local_name_noi(attr : *mut lxb_dom_attr_t, len : c_uint)
        -> *const core::lxb_char_t;
    pub fn lxb_dom_attr_value_noi(attr : *mut lxb_dom_attr_t, len : *mut c_uint)
        -> *const core::lxb_char_t;

    // lexbor/dom/interfaces/character_data.h
    pub fn lxb_dom_character_data_interface_create(document : 
        *mut lxb_dom_document_t) -> *mut lxb_dom_character_data_t;
    pub fn lxb_dom_character_data_interface_destroy(character_data : 
        *mut lxb_dom_character_data_t) -> *mut lxb_dom_character_data_t;
    pub fn lxb_dom_character_data_replace(ch_data : 
        *mut lxb_dom_character_data_t, data : *const core::lxb_char_t, len :
        c_uint, offset : c_uint, count : c_uint) -> core::lxb_status_t;
    
    // lexbor/dom/interfaces/text.h
    pub fn lxb_dom_text_interface_create(document : *mut lxb_dom_document_t)
        -> *mut lxb_dom_text_t;
    pub fn lxb_dom_text_interface_destroy(text : *mut lxb_dom_text_t)
        -> *mut lxb_dom_text_t;

    // lexbor/dom/interfaces/cdata_section.h
    pub fn lxb_dom_cdata_section_interface_create(document : 
        *mut lxb_dom_document_t) -> *mut lxb_dom_cdata_section_t;
    pub fn lxb_dom_cdata_section_interface_destroy(cdata_section :
        *mut lxb_dom_cdata_section_t) -> *mut lxb_dom_cdata_section_t;

    // lexbor/dom/interfaces/comment.h
    pub fn lxb_dom_comment_interface_create(document : *mut lxb_dom_document_t)
        -> *mut lxb_dom_comment_t;
    pub fn lxb_dom_comment_interface_destroy(comment : *mut lxb_dom_comment_t)
        -> *mut lxb_dom_comment_t;

    // lexbor/dom/interfaces/document_fragment.h
    pub fn lxb_dom_document_fragment_interface_create(document : 
        *mut lxb_dom_document_t) -> *mut lxb_dom_document_fragment_t;
    pub fn lxb_dom_document_fragment_interface_destroy(document_fragment :
        *mut lxb_dom_document_fragment_t) -> *mut lxb_dom_document_fragment_t;

    // lexbor/dom/interfaces/element.h
    pub fn lxb_dom_element_interface_create(document : *mut lxb_dom_document_t)
        -> *mut lxb_dom_element_t;
    pub fn lxb_dom_element_interface_destroy(element : *mut lxb_dom_element_t)
        -> *mut lxb_dom_element_t;
    pub fn lxb_dom_element_create(document : *mut lxb_dom_document_t, 
        local_name : *const core::lxb_char_t, lname_len : c_uint, ns_name :
        *const core::lxb_char_t, ns_len : c_uint, prefix : 
        *const core::lxb_char_t, prefix_len : c_uint, is : 
        *const core::lxb_char_t, is_len : c_uint, sync_custom : bool)
        -> *mut lxb_dom_element_t;
    pub fn lxb_dom_element_destroy(element : *mut lxb_dom_element_t)
        -> *mut lxb_dom_element_t;
    pub fn lxb_dom_element_has_attributes(element : *mut lxb_dom_element_t)
        -> bool;
    pub fn lxb_dom_element_set_attribute(element : *mut lxb_dom_element_t,
        qualified_name : *const core::lxb_char_t, qn_len : c_uint, value :
        *const core::lxb_char_t, value_len : c_uint) -> *mut lxb_dom_attr_t;
    pub fn lxb_dom_element_get_attribute(element : *mut lxb_dom_element_t,
        qualified_name : *const core::lxb_char_t, qn_len : c_uint, value_len :
        *mut c_uint) -> *const core::lxb_char_t;
    pub fn lxb_dom_element_remove_attribute(element : *mut lxb_dom_element_t,
        qualified_name : *const core::lxb_char_t, qn_len : c_uint)
        -> core::lxb_status_t;
    pub fn lxb_dom_element_has_attribute(element : *mut lxb_dom_element_t,
        qualified_name : *const core::lxb_char_t, qn_len : c_uint) -> bool;
    pub fn lxb_dom_elemen_attr_append(element : *mut lxb_dom_element_t, attr :
        *mut lxb_dom_attr_t) -> core::lxb_status_t;
    pub fn lxb_dom_element_attr_remove(element : *mut lxb_dom_element_t, attr :
        *mut lxb_dom_attr_t) -> core::lxb_status_t;
    pub fn lxb_dom_element_attr_by_name(element : *mut lxb_dom_element_t,
        qualified_name : *const core::lxb_char_t, length : c_uint)
        -> *mut lxb_dom_attr_t;
    pub fn lxb_dom_element_attr_by_local_name_data(element : 
        *mut lxb_dom_element_t, data : *const lxb_dom_attr_data_t)
        -> *mut lxb_dom_attr_t;
    pub fn lxb_dom_element_attr_by_id(element : *mut lxb_dom_element_t, 
        attr_id : lxb_dom_attr_id_t) -> *mut lxb_dom_attr_t;
    pub fn lxb_dom_element_attr_by_data(element : *mut lxb_dom_element_t, data :
        *const lxb_dom_attr_data_t) -> *mut lxb_dom_attr_t;
    pub fn lxb_dom_element_compare(first : *mut lxb_dom_element_t, second :
        *mut lxb_dom_element_t) -> bool;
    pub fn lxb_dom_element_attr_is_exists(element : *mut lxb_dom_element_t,
        qualified_name : *const core::lxb_char_t, length : c_uint)
        -> *mut lxb_dom_attr_t;
    pub fn lxb_dom_element_is_set(element : *mut lxb_dom_element_t, is :
        *const core::lxb_char_t, is_len : c_uint) -> core::lxb_status_t;
    pub fn lxb_dom_elements_by_tag_name(root : *mut lxb_dom_element_t,
        collection : *mut lxb_dom_collection_t, qualified_name : 
        *const core::lxb_char_t, len : c_uint) -> core::lxb_status_t;
    pub fn lxb_dom_elements_by_class_name(root : *mut lxb_dom_element_t,
        collection : *mut lxb_dom_collection_t, class_name : 
        *const core::lxb_char_t, len : c_uint) -> core::lxb_status_t;
    pub fn lxb_dom_elements_by_attr(root : *mut lxb_dom_element_t,
        collection : *mut lxb_dom_collection_t, qualified_name : 
        *const core::lxb_char_t, qname_len : c_uint, value : 
        *const core::lxb_char_t, value_len : c_uint, case_insensitive : bool)
        -> core::lxb_status_t;
    pub fn lxb_dom_elements_by_attr_begin(root : *mut lxb_dom_element_t,
        collection : *mut lxb_dom_collection_t, qualified_name :
        *const core::lxb_char_t, qname_len : c_uint, value : 
        *const core::lxb_char_t, value_len : c_uint, case_insensitive : bool)
        -> core::lxb_status_t;
    pub fn lxb_dom_elements_by_attr_end(root : *mut lxb_dom_element_t,
        collection : *mut lxb_dom_collection_t, qualified_name :
        *const core::lxb_char_t, qname_len : c_uint, value : 
        *const core::lxb_char_t, value_len : c_uint, case_insensitive : bool)
        -> core::lxb_status_t;
    pub fn lxb_dom_elements_by_attr_contain(root : *mut lxb_dom_element_t,
        collection : *mut lxb_dom_collection_t, qualified_name :
        *const core::lxb_char_t, qname_len : c_uint, value : 
        *const core::lxb_char_t, value_len : c_uint, case_insensitive : bool)
        -> core::lxb_status_t;
    pub fn lxb_dom_element_qualified_name(element : *mut lxb_dom_element_t,
        len : *mut c_uint) -> *const core::lxb_char_t;
    pub fn lxb_dom_element_qualified_name_upper(element : 
        *mut lxb_dom_element_t, len : *mut c_uint) -> *const core::lxb_char_t;
    pub fn lxb_dom_element_local_name(element : *mut lxb_dom_element_t, len : 
        *mut c_uint) -> *const core::lxb_char_t;
    pub fn lxb_dom_element_prefix(element : *mut lxb_dom_element_t, len : 
        *mut c_uint) -> *const core::lxb_char_t;
    pub fn lxb_dom_element_tag_name(element : *mut lxb_dom_element_t, len : 
        *mut c_uint) -> *const core::lxb_char_t;
    pub fn lxb_dom_element_id_noi(element : *mut lxb_dom_element_t, len :
        *mut c_uint) -> *const core::lxb_char_t;
    pub fn lxb_dom_element_class_noi(element : *mut lxb_dom_element_t, len :
        *mut c_uint) -> *const core::lxb_char_t;
    pub fn lxb_dom_element_is_custom_noi(element : *mut lxb_dom_element_t)
        -> bool;
    pub fn lxb_dom_element_custom_is_defined_noi(element : 
        *mut lxb_dom_element_t) -> bool;
    pub fn lxb_dom_element_first_attribute_noi(element : *mut lxb_dom_element_t)
        -> *mut lxb_dom_attr_t;
    pub fn lxb_dom_element_next_attribute_noi(attr : *mut lxb_dom_attr_t)
        -> *mut lxb_dom_attr_t;
    pub fn lxb_dom_element_prev_attribute_noi(attr : *mut lxb_dom_attr_t)
        -> *mut lxb_dom_attr_t;
    pub fn lxb_dom_element_last_attribute_noi(element : *mut lxb_dom_element_t)
        -> *mut lxb_dom_attr_t;
    pub fn lxb_dom_element_id_attribute_noi(element : *mut lxb_dom_element_t)
        -> *mut lxb_dom_attr_t;
    pub fn lxb_dom_element_class_attribute_noi(element : *mut lxb_dom_element_t)
        -> *mut lxb_dom_attr_t;

    // lexbor/dom/interfaces/processing_instruction.h
    pub fn lxb_dom_processing_instruction_interface_create(document :
        *mut lxb_dom_document_t) -> *mut lxb_dom_processing_instruction_t;
    pub fn lxb_dom_processing_instruction_interface_destroy(
        processing_instruction : *mut lxb_dom_processing_instruction_t)
        -> *mut lxb_dom_processing_instruction_t;
    pub fn lxb_dom_processing_instruction_target_noi(pi :
        *mut lxb_dom_processing_instruction_t, len : *mut c_uint)
        -> *const core::lxb_char_t;
    
    // lexbor/dom/interfaces/shadow_root.h
    pub fn lxb_dom_shadow_root_interface_create(document : 
        *mut lxb_dom_element_t) -> *mut lxb_dom_shadow_root_t;
    pub fn lxb_dom_shadow_root_interface_destroy(shadow_root : 
        *mut lxb_dom_shadow_root_t) -> *mut lxb_dom_shadow_root_t;
}