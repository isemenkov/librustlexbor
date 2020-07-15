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

pub type lxb_codepoint_t = u32;
pub type lxb_char_t = c_uchar;
pub type lxb_status_t = c_uint;

pub type lexbor_callback_f = extern "C" fn(buffer : *const lxb_char_t, size :
    c_uint, ctx : *mut c_void);

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

#[repr(C)]
pub struct lexbor_bst_entry_t {
    pub value : *mut c_void,

    pub rigth : *mut lexbor_bst_entry_t,
    pub left : *mut lexbor_bst_entry_t,
    pub next : *mut lexbor_bst_entry_t,
    pub parent : *mut lexbor_bst_entry_t,

    pub size : c_uint
}

#[repr(C)]
pub struct lexbor_bst_t {
    pub dobject : *mut lexbor_dobject_t,
    pub root : *mut lexbor_bst_entry_t,

    pub tree_length : c_uint
}

pub type lexbor_bst_entry_f = extern "C" fn(bst : *mut lexbor_bst_t, entry :
    *mut lexbor_bst_entry_t, ctx : *mut c_void) -> bool;

#[repr(C)]
pub struct lexbor_mraw_t {
    pub mem : *mut lexbor_mem_t,
    pub cache : *mut lexbor_bst_t
}

#[repr(C)]
pub struct lexbor_str_t {
    pub data : *mut lxb_char_t,
    pub length : c_uint
}

#[repr(C)]
pub struct lexbor_bst_map_entry_t {
    pub _str : lexbor_str_t,
    pub value : *mut c_void
}

#[repr(C)]
pub struct lexbor_bst_map_t {
    pub bst : *mut lexbor_bst_t,
    pub mraw : *mut lexbor_mraw_t,
    pub entries : lexbor_dobject_t
}

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

    // lexbor/core/bst.h
    pub fn lexbor_bst_create() -> *mut lexbor_bst_t;
    pub fn lexbor_bst_init(bst : *mut lexbor_bst_t, size : c_uint) -> 
        lxb_status_t;
    pub fn lexbor_bst_clean(bst : *mut lexbor_bst_t) -> ();
    pub fn lexbor_bst_destroy(bst : *mut lexbor_bst_t, self_destroy : bool) ->
        *mut lexbor_bst_t;
    pub fn lexbor_bst_entry_make(bst : *mut lexbor_bst_t, size : c_uint) ->
        *mut lexbor_bst_entry_t;
    pub fn lexbor_bst_insert(bst : *mut lexbor_bst_t, scope : 
        *mut *mut lexbor_bst_entry_t, size : c_uint, value : *mut c_void) ->
        *mut lexbor_bst_entry_t;
    pub fn lexbor_bst_insert_not_exists(bst : *mut lexbor_bst_t, scope :
        *mut *mut lexbor_bst_entry_t, size : c_uint) -> *mut lexbor_bst_entry_t;
    pub fn lexbor_bst_search(bst : *mut lexbor_bst_t, scope : 
        *mut lexbor_bst_entry_t, size : c_uint) -> *mut lexbor_bst_entry_t;
    pub fn lexbor_bst_search_close(bst : *mut lexbor_bst_t, scope :
        *mut lexbor_bst_entry_t, size : c_uint) -> *mut lexbor_bst_entry_t;
    pub fn lexbor_bst_remove(bst : *mut lexbor_bst_t, root : 
        *mut *mut lexbor_bst_entry_t, size : c_uint) -> *mut c_void;
    pub fn lexbor_bst_remove_close(bst : *mut lexbor_bst_t, root :
        *mut *mut lexbor_bst_entry_t, size : c_uint, found_size : *mut c_uint) 
        -> *mut c_void;
    pub fn lexbor_bst_remove_by_pointer(bst : *mut lexbor_bst_t, entry :
        *mut lexbor_bst_entry_t, root : *mut *mut lexbor_bst_entry_t) ->
        *mut c_void;
    pub fn lexbor_bst_serilize(bst : *mut lexbor_bst_t, callbalck :
        lexbor_callback_f, ctx : *mut c_void) -> ();
    pub fn lexbor_bst_serialize_entry(entry : *mut lexbor_bst_entry_t, 
        callback : lexbor_callback_f, ctx : *mut c_void, tabs : c_uint) -> ();

    // lexbor/core/mraw.h
    pub fn lexbor_mraw_create() -> *mut lexbor_mraw_t;
    pub fn lexbor_mraw_init(mraw : *mut lexbor_mraw_t, chunk_size : c_uint) ->
        lxb_status_t;
    pub fn lexbor_mraw_clean(mraw : *mut lexbor_mraw_t) -> ();
    pub fn lexbor_mraw_destroy(mraw : *mut lexbor_mraw_t, destroy_self : bool)
        -> *mut lexbor_mraw_t;
    pub fn lexbor_mraw_alloc(mraw : *mut lexbor_mraw_t, size : c_uint) ->
        *mut c_void;
    pub fn lexbor_mraw_calloc(mraw : *mut lexbor_mraw_t, size : c_uint) ->
        *mut c_void;
    pub fn lexbor_mraw_realloc(mraw : *mut lexbor_mraw_t, data : *mut c_void,
        new_size : c_uint) -> *mut c_void;
    pub fn lexbor_mraw_free(mraw : *mut lexbor_mraw_t, data : *mut c_void) ->
        *mut c_void;
    pub fn lexbor_mraw_data_size_noi(data : *mut c_void) -> c_uint;
    pub fn lexbor_mraw_data_size_set_noi(data : *mut c_void, size : c_uint) 
        -> ();
    pub fn lexbor_mraw_dup_noi(mraw : *mut lexbor_mraw_t, src : *const c_void,
        size : c_uint) -> *mut c_void;

    // lexbor/core/utils.h
    pub fn lexbor_utils_power(t : c_uint, k : c_uint) -> c_uint;
    pub fn lexbor_utils_hash_hash(key : *const lxb_char_t, key_size : c_uint) 
        -> c_uint;
    
    // lexbor/core/str.h
    pub fn lexbor_str_create() -> *mut lexbor_str_t;
    pub fn lexbor_str_init(string : *mut lexbor_str_t, mraw : 
        *mut lexbor_mraw_t, size : c_uint) -> *mut lxb_char_t;
    pub fn lexbor_str_clean(string : *mut lexbor_str_t) -> ();
    pub fn lexbor_str_clean_all(string : *mut lexbor_str_t) -> ();
    pub fn lexbor_str_destroy(string : *mut lexbor_str_t, mraw : 
        *mut lexbor_mraw_t, destroy_obj : bool) -> *mut lexbor_str_t;
    pub fn lexbor_str_realloc(string : *mut lexbor_str_t, mraw : 
        *mut lexbor_mraw_t, new_size : *mut c_uint) -> lxb_char_t;
    pub fn lexbor_str_check_size(string : *mut lexbor_str_t, mraw :
        *mut lexbor_mraw_t, plus_len : c_uint) -> *mut lxb_char_t;
    pub fn lexbor_str_append(string : *mut lexbor_str_t, mraw : 
        *mut lexbor_mraw_t, data : *const lxb_char_t, length : c_uint) ->
        *mut lxb_char_t;
    pub fn lexbor_str_append_before(string : *mut lexbor_str_t, mraw : 
        *mut lexbor_mraw_t, buff : *const lxb_char_t, length : c_uint) ->
        *mut lxb_char_t;
    pub fn lexbor_str_append_one(string : *mut lexbor_str_t, mraw : 
        *mut lexbor_mraw_t, data : *const lxb_char_t) -> *mut lxb_char_t;
    pub fn lexbor_str_append_lowercase(string : lexbor_str_t, mraw :
        *mut lexbor_mraw_t, data : *const lxb_char_t, length : c_uint) ->
        *mut lxb_char_t;
    pub fn lexbor_str_append_with_rep_null_chars(string : *mut lexbor_str_t,
        mraw : *mut lexbor_mraw_t, buff : *const lxb_char_t, length : c_uint)
        -> *mut lxb_char_t;
    pub fn lexbor_str_copy(dest : *mut lexbor_str_t, target : 
        *const lexbor_mraw_t, buff : *const lxb_char_t, length : c_uint) ->
        *mut lxb_char_t;
    pub fn lexbor_str_stay_only_whitespace(target : *mut lexbor_str_t) -> ();
    pub fn lexbor_str_strip_collapse_whitespace(target : *mut lexbor_str_t) 
        -> ();
    pub fn lexbor_str_crop_whitespace_from_begin(target : *mut lexbor_str_t) ->
        c_uint;
    pub fn lexbor_str_whitespace_from_begin(target : *mut lexbor_str_t) ->
        c_uint;
    pub fn lexbor_str_whitespace_from_end(target : *mut lexbor_str_t) -> c_uint;
    pub fn lexbor_str_data_ncasecmp_first(first : *const lxb_char_t, sec :
        *const lxb_char_t, sec_size : c_uint) -> *const lxb_char_t;
    pub fn lexbor_str_data_ncasecmp_end(first : *const lxb_char_t, sec :
        *const lxb_char_t, size : c_uint) -> bool;
    pub fn lexbor_str_data_ncasecmp_contain(_where : *const lxb_char_t, 
        where_size : c_uint, _what : *const lxb_char_t, what_size : c_uint) ->
        bool;
    pub fn lexbor_str_data_ncasecmp(first : *const lxb_char_t, sec : 
        *const lxb_char_t, size : c_uint) -> bool;
    pub fn lexbor_str_data_nlocmp_right(first : *const lxb_char_t, sec : 
        *const lxb_char_t, size : c_uint) -> bool;
    pub fn lexbor_str_data_nupcmp_right(first : *const lxb_char_t, sec : 
        *const lxb_char_t, size : c_uint) -> bool;
    pub fn lexbor_str_data_casecmp(first : *const lxb_char_t, sec : 
        *const lxb_char_t) -> bool;
    pub fn lexbor_str_data_ncmp_end(first : *const lxb_char_t, sec : 
        *const lxb_char_t, size : c_uint) -> bool;
    pub fn lexbor_str_data_ncmp_contain(_where : *const lxb_char_t, 
        _where_size : c_uint, _what : *const lxb_char_t, _what_size : c_uint) 
        -> bool;
    pub fn lexbor_str_data_ncmp(first : *const lxb_char_t, sec : 
        *const lxb_char_t, size : c_uint) -> bool;
    pub fn lexbor_str_data_cmp(first : *const lxb_char_t, sec :
        *const lxb_char_t) -> bool;
    pub fn lexbor_str_data_cmp_ws(first : *const lxb_char_t, sec : 
        *const lxb_char_t) -> bool;
    pub fn lexbor_str_data_to_lowercase(to : *mut lxb_char_t, from : 
        *const lxb_char_t, len : c_uint) -> ();
    pub fn lexbor_str_data_to_uppercase(to : *mut lxb_char_t, from :
        *const lxb_char_t, len : c_uint) -> ();
    pub fn lexbor_str_data_find_lowercase(data : *const lxb_char_t, len : 
        c_uint) -> *const lxb_char_t;
    pub fn lexbor_str_data_find_uppercase(data : *const lxb_char_t, len : 
        c_uint) -> *const lxb_char_t;
    pub fn lexbor_str_data_noi(_str : *mut lexbor_str_t) -> *mut lxb_char_t;
    pub fn lexbor_str_length_noi(_str : *mut lexbor_str_t) -> c_uint;
    pub fn lexbor_str_size_noi(_str : *mut lexbor_str_t) -> c_uint;
    pub fn lexbor_str_data_set_noi(_str : *mut lexbor_str_t, data :
        *mut lxb_char_t) -> ();
    pub fn lexbor_str_length_set_noi(_str : *mut lexbor_str_t, mraw :
        *mut lexbor_mraw_t, length : c_uint) -> *mut lxb_char_t;

    // lexbor/core/bst_map.h
    pub fn lexbor_bst_map_create() -> *mut lexbor_bst_map_t;
    pub fn lexbor_bst_map_init(bst_map : *mut lexbor_bst_map_t, size : 
        c_uint) -> lxb_status_t;
    pub fn lexbor_bst_map_clean(bst_map : *mut lexbor_bst_map_t) -> ();

}