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

extern crate libc;

use libc::{c_uchar, c_short, c_uint };
use std::os::raw::c_void;

pub type lxb_codepoint_t = c_uint;
pub type lxb_char_t = c_uchar;
pub type lxb_status_t = c_uint;

#[repr(C)]
pub enum lexbor_status_t {
    LXB_STATUS_OK                                                      = 0x0000,
    LXB_STATUS_ERROR                                                   = 0x0001,
    LXB_STATUS_ERROR_MEMORY_ALLOCATION,
    LXB_STATUS_ERROR_OBJECT_IS_NULL,
    LXB_STATUS_ERROR_SMALL_BUFFER,
    LXB_STATUS_ERROR_INCOMPLETE_OBJECT,
    LXB_STATUS_ERROR_NO_FREE_SLOT,
    LXB_STATUS_ERROR_TOO_SMALL_SIZE,
    LXB_STATUS_ERROR_NOT_EXISTS,
    LXB_STATUS_ERROR_WRONG_ARGS,
    LXB_STATUS_ERROR_WRONG_STAGE,
    LXB_STATUS_ERROR_UNEXPECTED_RESULT,
    LXB_STATUS_ERROR_UNEXPECTED_DATA,
    LXB_STATUS_CONTINUE,
    LXB_STATUS_SMALL_BUFFER,
    LXB_STATUS_ABORTED,
    LXB_STATUS_STOPPED,
    LXB_STATUS_NEXT,
    LXB_STATUS_STOP
}

#[repr(C)]
pub enum lexbor_action_t {
    LEXBOR_ACTION_OK                                                   = 0x00,
    LEXBOR_ACTION_STOP                                                 = 0x01,
    LEXBOR_ACTION_NEXT                                                 = 0x02 
}

#[repr(C)]
pub struct lexbor_array_t {
    pub list : *mut *mut c_void,
    pub size : c_uint,
    pub length : c_uint
}

#[repr(C)]
pub struct lexbor_array_obj_t {
    pub list : *mut u8,
    pub size : c_uint,
    pub length : c_uint,
    pub struct_size : c_uint
}

#[repr(C)]
pub struct lexbor_mem_chunk_t {
    pub data : *mut u8,
    pub length : c_uint,
    pub size : c_uint,

    pub next : *mut lexbor_mem_chunk_t,
    pub prev : *mut lexbor_mem_chunk_t
}

#[repr(C)]
pub struct lexbor_mem_t {
    pub chunk : *mut lexbor_mem_chunk_t,
    pub chunk_first : *mut lexbor_mem_chunk_t,
    
    pub chunk_min_size : c_uint,
    pub chunk_length : c_uint
}

#[repr(C)]
pub struct lexbor_dobject_t {
    pub mem : *mut lexbor_mem_t,
    pub cache : *mut lexbor_array_t,

    pub allocated : c_uint,
    pub struct_size : c_uint
}

#[repr(C)]
pub struct lexbor_avl_node_t {
    pub node_type : c_uint,
    pub height : c_short,
    pub value : *mut c_void,

    pub left : *mut lexbor_avl_node_t,
    pub right : *mut lexbor_avl_node_t,
    pub parent : *mut lexbor_avl_node_t
}

#[repr(C)]
pub struct lexbor_avl_t {
    pub nodes : *mut lexbor_dobject_t
}

pub type lexbor_avl_node_f = extern "C" fn(avl_node : *mut lexbor_avl_node_t,
    ctx : *mut c_void) -> ();

#[link(name = "lexbor")]
extern "C" {
    // lexbor/core/lexbor.h
    pub fn lexbor_malloc (size : c_uint) -> *mut c_void;
    pub fn lexbor_realloc (dst : *mut c_void, size : c_uint) -> *mut c_void;
    pub fn lexbor_calloc (num : c_uint, size : c_uint) -> *mut c_void;
    pub fn lexbor_free (dst : *mut c_void) -> *mut c_void;

    // lexbor/core/array.h
    pub fn lexbor_array_create() -> lexbor_array_t;
    pub fn lexbor_array_init(array : *mut lexbor_array_t, size : c_uint) ->
        lxb_status_t;
    pub fn lexbor_array_clean(array : *mut lexbor_array_t) -> ();
    pub fn lexbor_array_destroy(array : *mut lexbor_array_t, self_destroy : 
        bool) -> *mut lexbor_array_t;
    pub fn lexbor_array_expand(array : *mut lexbor_array_t, up_to : c_uint) ->
        *mut *mut c_void;
    pub fn lexbor_array_push(array : *mut lexbor_array_t, value : *mut c_void)
        -> lxb_status_t;
    pub fn lexbor_array_pop(array : *mut lexbor_array_t) -> *mut c_void;
    pub fn lexbor_array_insert(array : *mut lexbor_array_t, idx : c_uint,
        value : *mut c_void) -> lxb_status_t;
    pub fn lexbor_array_set(array : *mut lexbor_array_t, idx : c_uint, value :
        *mut c_void) -> lxb_status_t;
    pub fn lexbor_array_delete(array : *mut lexbor_array_t, begin : c_uint,
        length : c_uint) -> ();
    pub fn lexbor_array_get_noi(array : *mut lexbor_array_t, idx : c_uint) 
        -> *mut c_void;
    pub fn lexbor_array_length_noi(array : *mut lexbor_array_t) -> c_uint;
    pub fn lexbor_array_size_noi(array : *mut lexbor_array_t) -> c_uint;
    
    // lexbor/core/array_obj.h
    pub fn lexbor_array_obj_create() -> *mut lexbor_array_obj_t;
    pub fn lexbor_array_obj_init (array : *mut lexbor_array_obj_t, size : 
        c_uint, struct_size : c_uint) -> lxb_status_t;
    pub fn lexbor_array_obj_clean(array : *mut lexbor_array_obj_t) -> ();
    pub fn lexbor_array_obj_destroy(array : *mut lexbor_array_obj_t,
        self_destroy : bool) -> *mut lexbor_array_obj_t;
    pub fn lexbor_array_obj_expand(array : *mut lexbor_array_obj_t, up_to :
        c_uint) -> *mut u8;
    pub fn lexbor_array_obj_push(array : *mut lexbor_array_obj_t) -> 
        *mut c_void;
    pub fn lexbor_array_obj_pop(array : *mut lexbor_array_obj_t) -> 
        *mut c_void;
    pub fn lexbor_array_obj_delete(array : *mut lexbor_array_obj_t, begin :
        c_uint, length : c_uint) -> ();
    pub fn lexbor_array_obj_erase_noi(array : *mut lexbor_array_obj_t) -> ();
    pub fn lexbor_array_obj_get_noi(array : *mut lexbor_array_obj_t) -> c_uint;
    pub fn lexbor_array_obj_length_noi(array : *mut lexbor_array_obj_t) ->
        c_uint;
    pub fn lexbor_array_obj_size_noi(array : *mut lexbor_array_obj_t) -> c_uint;
    pub fn lexbor_array_obj_struct_size_noi(array : *mut lexbor_array_obj_t) ->
        c_uint;
    pub fn lexbor_array_obj_last_noi(array : *mut lexbor_array_obj_t) ->
        *mut c_void;

    // lexbor/core/mem.h
    pub fn lexbor_mem_create() -> *mut lexbor_mem_t;
    pub fn lexbor_mem_init(mem : *mut lexbor_mem_t, min_chunk_size : c_uint) ->
        lxb_status_t;
    pub fn lexbor_mem_destroy(mem : *mut lexbor_mem_t, destroy_self : bool) ->
        *mut lexbor_mem_t;
    pub fn lexbor_mem_chunk_init(mem : *mut lexbor_mem_t, chunk : 
        *mut lexbor_mem_chunk_t, length : c_uint) -> *mut u8;
    pub fn lexbor_mem_chunk_make(mem : *mut lexbor_mem_t, length : c_uint) ->
        *mut lexbor_mem_chunk_t;
    pub fn lexbor_mem_chunk_destroy(mem : *mut lexbor_mem_t, chunk :
        *mut lexbor_mem_chunk_t, self_destroy : bool) -> 
        *mut lexbor_mem_chunk_t;
    pub fn lexbor_mem_alloc(mem : *mut lexbor_mem_t, length : c_uint) ->
        *mut c_void;
    pub fn lexbor_mem_calloc(mem : *mut lexbor_mem_t, length : c_uint) ->
        *mut c_void;
    pub fn lexbor_mem_current_length_noi(mem : *mut lexbor_mem_t) -> c_uint;
    pub fn lexbor_mem_current_size_noi(mem : *mut lexbor_mem_t) -> c_uint;
    pub fn lexbor_mem_chunk_length_noi(mem : *mut lexbor_mem_t) -> c_uint;
    pub fn lexbor_mem_align_floor_noi(size : c_uint) -> c_uint;

    // lexbor/core/dobject.h
    pub fn lexbor_dobject_create() -> *mut lexbor_dobject_t;
    pub fn lexbor_dobject_init(dobject : *mut lexbor_dobject_t, chunk_size : 
        c_uint, struct_size : c_uint) -> lxb_status_t;
    pub fn lexbor_dobject_clean(dobject : *mut lexbor_dobject_t) -> ();
    pub fn lexbor_dobject_destroy(dobject : *mut lexbor_dobject_t, 
        destroy_self : bool) -> *mut lexbor_dobject_t;
    pub fn lexbor_dobject_init_list_entries(dobject : *mut lexbor_dobject_t,
        pos : c_uint) -> *mut u8;
    pub fn lexbor_dobject_alloc(dobject : *mut lexbor_dobject_t) -> *mut c_void;
    pub fn lexbor_dobject_calloc(dobject : *mut lexbor_dobject_t) ->
        *mut c_void;
    pub fn lexbor_dobject_free(dobject : *mut lexbor_dobject_t, data : 
        *mut c_void) -> *mut c_void;
    pub fn lexbor_dobject_by_absolute_position(dobject : *mut lexbor_dobject_t,
        pos : c_uint) -> *mut c_void;
    pub fn lexbor_dobject_allocated_noi(dobject : *mut lexbor_dobject_t) ->
        c_uint;
    pub fn lexbor_dobject_cache_length_noi(dobject : *mut lexbor_dobject_t) ->
        c_uint;

    // lexbor/core/avl.h
    pub fn lexbor_avl_create() -> *mut lexbor_avl_t;
    pub fn lexbor_avl_init(avl : *mut lexbor_avl_t, chunk_len : c_uint) ->
        lxb_status_t;
    pub fn lexbor_avl_clean(avl : *mut lexbor_avl_t) -> ();
    pub fn lexbor_avl_destroy(avl : *mut lexbor_avl_t, self_destroy : bool) ->
        *mut lexbor_avl_t;
    pub fn lexbor_avl_node_make(avl : *mut lexbor_avl_t, node_type : c_uint,
        value : *mut c_void) -> *mut lexbor_avl_node_t;
    pub fn lexbor_avl_node_clean(node : *mut lexbor_avl_node_t) -> ();
    pub fn lexbor_avl_node_destroy(avl : *mut lexbor_avl_t, node :
        *mut lexbor_avl_node_t, self_destroy : bool) -> *mut lexbor_avl_node_t;
    pub fn lexbor_avl_insert(avl : *mut lexbor_avl_t, scope : 
        *mut *mut lexbor_avl_node_t, node_type : c_uint, value : *mut c_void) ->
        lexbor_avl_node_t;
    pub fn lexbor_avl_search(avl : *mut lexbor_avl_t, scope : 
        *mut lexbor_avl_node_t, node_type : c_uint) -> *mut lexbor_avl_node_t;
    pub fn lexbor_avl_remove(avl : *mut lexbor_avl_t, scope :
        *mut *mut lexbor_avl_node_t, node_type : c_uint) -> *mut c_void;
    pub fn lexbor_avl_foreach_recursion(avl : *mut lexbor_avl_t, scope :
        *mut lexbor_avl_node_t, callback : lexbor_avl_node_f, ctx : 
        *mut c_void) -> ();
}