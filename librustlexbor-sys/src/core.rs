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

use libc::{c_char, c_uchar, c_short, c_int, c_uint, c_ulong, c_double };
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

#[repr(C)]
pub struct lexbor_diyfp_t {
    pub significand : u64,
    pub exp : c_int
}

pub type lexbor_fs_dir_file_f = extern "C" fn(fullpath : *const lxb_char_t,
    fullpath_len : c_uint, filename : *const lxb_char_t, filename_len : c_uint,
    ctx : *mut c_void) -> lexbor_action_t;

pub type lexbor_fs_dir_opt_t = c_int;

#[repr(C)]
pub enum lexbor_fs_dir_opt {
    LEXBOR_FS_DIR_OPT_UNDEF                                            = 0x00,  
    LEXBOR_FS_DIR_OPT_WITHOUT_DIR                                      = 0x01,
    LEXBOR_FS_DIR_OPT_WITHOUT_FILE                                     = 0x02,
    LEXBOR_FS_DIR_OPT_WITHOUT_HIDDEN                                   = 0x04 
}

#[repr(C)]
pub enum lexbor_fs_file_type_t {
    LEXBOR_FS_FILE_TYPE_UNDEF                                          = 0x00,
    LEXBOR_FS_FILE_TYPE_FILE                                           = 0x01,
    LEXBOR_FS_FILE_TYPE_DIRECTORY                                      = 0x02,
    LEXBOR_FS_FILE_TYPE_BLOCK_DEVICE                                   = 0x03,
    LEXBOR_FS_FILE_TYPE_CHARACTER_DEVICE                               = 0x04,
    LEXBOR_FS_FILE_TYPE_PIPE                                           = 0x05,
    LEXBOR_FS_FILE_TYPE_SYMLINK                                        = 0x06,
    LEXBOR_FS_FILE_TYPE_SOCKET                                         = 0x07  
}

pub const LEXBOR_HASH_SHORT_SIZE : usize                               = 16;
pub const LEXBOR_HASH_TABLE_MIN_SIZE : usize                           = 32; 

#[repr(C)]
pub union U {
    pub long_str : *mut lxb_char_t,
    pub short_str : [lxb_char_t; LEXBOR_HASH_SHORT_SIZE + 1]
}

#[repr(C)]
pub struct lexbor_hash_entry_t {
    pub u : U,
    pub length : c_uint, 
    pub next : *mut lexbor_hash_entry_t
}

#[repr(C)]
pub struct lexbor_hash_t {
    pub entries : *mut lexbor_dobject_t,
    pub mraw : *mut lexbor_mraw_t,

    pub table : *mut *mut lexbor_hash_entry_t,
    pub table_size : c_uint,
    pub struct_size : c_uint
}

pub type lexbor_hash_id_f = extern "C" fn(key : *const lxb_char_t, size : 
    c_uint) -> u32;

pub type lexbor_hash_copy_f = extern "C" fn(hash : *mut lexbor_hash_t, entry : 
    *mut lexbor_hash_entry_t, key : *const lxb_char_t, size : c_uint) 
    -> lxb_status_t;
    
pub type lexbor_hash_cmp_f = extern "C" fn(first : *const lxb_char_t, second :
    *mut lxb_char_t, size : c_uint) -> bool;

#[repr(C)]
pub struct lexbor_hash_insert_t {
    pub hash : lexbor_hash_id_f,    /* For generate a hash id. */
    pub cmp : lexbor_hash_cmp_f,    /* For compare key. */
    pub copy : lexbor_hash_copy_f   /* For copy key. */
}

#[repr(C)]
pub struct lexbor_hash_search_t {
    pub hash : lexbor_hash_id_f,    /* For generate a hash id. */
    pub cmp : lexbor_hash_cmp_f     /* For compare key. */
}

#[repr(C)]
pub enum lexbor_in_opt {
    LEXBOR_IN_OPT_UNDEF                                                 = 0x00,
    LEXBOR_IN_OPT_READONLY                                              = 0x01,
    LEXBOR_IN_OPT_DONE                                                  = 0x02,
    LEXBOR_IN_OPT_FAKE                                                  = 0x04,
    LEXBOR_IN_OPT_ALLOC                                                 = 0x08
}

pub type lexbor_in_opt_t = c_int;

#[repr(C)]
pub struct lexbor_in_t {
    pub nodes : lexbor_dobject_t
}

#[repr(C)]
pub struct lexbor_in_node_t {
    pub offset : c_uint,
    pub opt : lexbor_in_opt_t,

    pub begin : *const lxb_char_t,
    pub end : *const lxb_char_t,
    pub _use : *const lxb_char_t,
    
    pub next : *mut lexbor_in_node_t,
    pub prev : *mut lexbor_in_node_t,

    pub incoming : *mut lexbor_in_t
}

#[repr(C)]
pub struct lexbor_plog_entry_t {
    pub data : *const lxb_char_t,
    pub context : *mut c_void,
    pub id : c_uint
}

#[repr(C)]
pub struct lexbor_plog_t {
    pub list : lexbor_array_obj_t
}

#[repr(C)]
pub struct lexbor_sbst_entry_static_t {
    pub key : lxb_char_t,

    pub value : *mut c_void,
    pub value_len : c_uint,
    
    pub left : c_uint,
    pub right : c_uint,
    pub next : c_uint
}

#[repr(C)]
pub struct lexbor_shs_entry_t {
    pub key : *mut c_char,
    pub value : *mut c_void,

    pub key_len : c_uint,
    pub next : c_uint 
}

#[repr(C)]
pub struct lexbor_shs_hash_t {
    pub key : u32,
    pub value : *mut c_void,

    pub next : c_uint
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
    pub fn lexbor_bst_map_destroy(bst_map : *mut lexbor_bst_map_t, 
        self_destroy : bool) -> *mut lexbor_bst_map_t;
    pub fn lexbor_bst_map_search(bst_map : *mut lexbor_bst_map_t, scope :
        *mut lexbor_bst_entry_t, key : *const lxb_char_t, key_len : c_uint) 
        -> *mut lexbor_bst_map_entry_t;
    pub fn lexbor_bst_map_insert(bsp_map : *mut lexbor_bst_map_t, scope :
        *mut *mut lexbor_bst_entry_t, key : *const lxb_char_t, key_len : 
        c_uint, value : *mut c_void) -> lexbor_bst_map_entry_t;
    pub fn lexbor_bst_map_insert_not_exists(bsp_map : *mut lexbor_bst_map_t, 
        scope : *mut *mut lexbor_bst_entry_t, key : *const lxb_char_t, key_len : 
        c_uint) -> lexbor_bst_map_entry_t;
    pub fn lexbor_bst_map_remove(bsp_map : *mut lexbor_bst_map_t, 
        scope : *mut *mut lexbor_bst_entry_t, key : *const lxb_char_t, key_len : 
        c_uint) -> *mut c_void;
    pub fn lexbor_bst_map_mraw_noi(bst_map : *mut lexbor_bst_map_t)
        -> *mut lexbor_mraw_t;

    // lexbor/core/conv.h
    pub fn lexbor_conv_float_to_data(num : c_double, buf : *mut lxb_char_t,
        len : c_uint) -> c_uint;
    pub fn lexbor_conv_data_to_double(start : *const *const lxb_char_t, len :
        c_uint) -> c_double;
    pub fn lexbor_conv_data_to_ulong(data : *const *const lxb_char_t, length :
        c_uint) -> c_ulong;
    pub fn lexbor_conv_data_to_uint(data : *const *const lxb_char_t, length :
        c_uint) -> c_uint;

    // lexbor/core/diyfp.h
    pub fn lexbor_cached_power_dec(exp : c_int, dec_exp : *mut c_int) 
        -> lexbor_diyfp_t;
    pub fn lexbor_cached_power_bin(exp : c_int, dec_exp : *mut c_int)
        -> lexbor_diyfp_t;
    
    // lexbor/core/dtoa.h
    pub fn lexbor_dtoa(value : c_double, begin : *mut lxb_char_t, len : c_uint)
        -> c_uint;

    // lexbor/core/fs.h
    pub fn lexbor_fs_dir_read(dirpath : *const lxb_char_t, opt : 
        lexbor_fs_dir_opt_t, callback : lexbor_fs_dir_file_f, ctx : *mut c_void)
        -> lxb_status_t;
    pub fn lexbor_fs_file_type(full_path : *const lxb_char_t)
        -> lexbor_fs_file_type_t;
    pub fn lexbor_fs_file_easy_read(full_path : *const lxb_char_t, len :
        *mut c_uint) -> *mut lxb_char_t;

    // lexbor/core/hash.h
    pub fn lexbor_hash_create() -> *mut lexbor_hash_t;
    pub fn lexbor_hash_init(hash : *mut lexbor_hash_t, table_size : c_uint,
        struct_size : c_uint) -> lxb_status_t;
    pub fn lexbor_hash_clean(hash : *mut lexbor_hash_t) -> ();
    pub fn lexbor_hash_destroy(hash : *mut lexbor_hash_t, destroy_obj : bool)
        -> *mut lexbor_hash_t;
    pub fn lexbor_hash_insert(hash : *mut lexbor_hash_t, insert : 
        *const lexbor_hash_insert_t, key : *const lxb_char_t, length : c_uint)
        -> *mut c_void;
    pub fn lexbor_hash_insert_by_entry(hash : *mut lexbor_hash_t, entry :
        *mut lexbor_hash_entry_t, search : *const lexbor_hash_search_t, key :
        *const lxb_char_t, length : c_uint) -> *mut c_void;
    pub fn lexbor_hash_remove(hash : *mut lexbor_hash_t, search : 
        *const lexbor_hash_search_t, key : *const lxb_char_t, length : c_uint)
        -> ();
    pub fn lexbor_hash_search(hash : *mut lexbor_hash_t, search : 
        *const lexbor_hash_search_t, key : *const lxb_char_t, length : c_uint)
        -> *mut c_void;
    pub fn lexbor_hash_remove_by_hash_id(hash : *mut lexbor_hash_t, hash_id : 
        u32, key : *const lxb_char_t, length : c_uint, cmp_func :
        lexbor_hash_cmp_f) -> ();
    pub fn lexbor_hash_search_by_hash_id(hash : *mut lexbor_hash_t, hash_id :
        u32, key : *const lxb_char_t, length : c_uint, cmp_func :
        lexbor_hash_cmp_f) -> *mut c_void;
    pub fn lexbor_hash_make_id(key : *const lxb_char_t, length : c_uint) -> u32;
    pub fn lexbor_hash_make_id_lower(key : *const lxb_char_t, length : c_uint)
        -> u32;
    pub fn lexbor_hash_make_id_upper(key : *const lxb_char_t, length : c_uint)
        -> u32;
    pub fn lexbor_hash_copy(hash : *mut lexbor_hash_t, entry : 
        *mut lexbor_hash_entry_t, key : *const lxb_char_t, length : c_uint) 
        -> lxb_status_t;
    pub fn lexbor_hash_copy_lower(hash : *mut lexbor_hash_t, entry :
        *mut lexbor_hash_entry_t, key : *const lxb_char_t, length : c_uint)
        -> lxb_status_t;
    pub fn lexbor_hash_copy_upper(hash : *mut lexbor_hash_t, entry :
        *mut lexbor_hash_entry_t, key : *const lxb_char_t, length : c_uint)
        -> lxb_status_t;
    
    // lexbor/core/in.h
    pub fn lexbor_in_create() -> *mut lexbor_in_t;
    pub fn lexbor_in_init(incoming : *mut lexbor_in_t, chunk_size : c_uint)
        -> lxb_status_t;
    pub fn lexbor_in_clean(incoming : *mut lexbor_in_t) -> ();
    pub fn lexbor_in_destroy(incoming : *mut lexbor_in_t, self_destroy : bool)
        -> *mut lexbor_in_t;
    pub fn lexbor_in_node_make(incoming : *mut lexbor_in_t, last_node :
        *mut lexbor_in_node_t, buf : *const lxb_char_t, buf_size : c_uint)
        -> *mut lexbor_in_node_t;
    pub fn lexbor_in_node_clean(node : *mut lexbor_in_node_t) -> ();
    pub fn lexbor_in_node_destroy(incoming : *mut lexbor_in_t, node :
        *mut lexbor_in_node_t, self_destroy : bool) -> *mut lexbor_in_node_t;
    pub fn lexbor_in_node_split(node : *mut lexbor_in_node_t, pos : 
        *const lxb_char_t) -> *mut lexbor_in_node_t;
    pub fn lexbor_in_node_find(node : *mut lexbor_in_node_t, pos : 
        *const lxb_char_t) -> *mut lexbor_in_node_t;
    pub fn lexbor_in_node_pos_up(node : *mut lexbor_in_node_t, return_node :
        *mut *mut lexbor_in_node_t, pos : *const lxb_char_t, offset : c_uint)
        -> *const lxb_char_t;
    pub fn lexbor_in_node_pos_down(node : *mut lexbor_in_node_t, return_node :
        *mut *mut lexbor_in_node_t, pos : *const lxb_char_t, offset : c_uint)
        -> *const lxb_char_t;
    pub fn lexbor_in_node_begin_noi(node : *const lexbor_in_node_t) 
        -> *const lxb_char_t;
    pub fn lexbor_in_node_end_noi(node : *const lexbor_in_node_t) 
        -> *const lxb_char_t;
    pub fn lexbor_in_node_offset_noi(node : *const lexbor_in_node_t) -> c_uint;
    pub fn lexbor_in_node_next_noi(node : *const lexbor_in_node_t)
        -> *mut lexbor_in_node_t;
    pub fn lexbor_in_node_prev_noi(node : *const lexbor_in_node_t)
        -> *mut lexbor_in_node_t;
    pub fn lexbor_in_node_in_noi(node : *const lexbor_in_node_t)
        -> *mut lexbor_in_node_t;
    pub fn lexbor_in_segment_noi(node : *const lexbor_in_node_t, data :
        *const lxb_char_t) -> bool;
        
    // lexbor/core/perf.h
    pub fn lexbor_perf_create() -> *mut c_void;
    pub fn lexbor_perf_clean(perf : *mut c_void) -> ();
    pub fn lexbor_perf_destroy(perf : *mut c_void) -> ();
    pub fn lexbor_perf_begin(perf : *mut c_void) -> lxb_status_t;
    pub fn lexbor_perf_end(perf : *mut c_void) -> lxb_status_t;
    pub fn lexbor_perf_in_sec(perf : *mut c_void) -> c_double;

    // lexbor/core/plog.h
    pub fn lexbor_plog_init(plog : *mut lexbor_plog_t, init_size : c_uint,
        struct_size : c_uint) -> lxb_status_t;
    pub fn lexbor_plog_destroy(plog : *mut lexbor_plog_t, self_destroy : bool)
        -> *mut lexbor_plog_t;
    pub fn lexbor_plog_create_noi() -> *mut lexbor_plog_t;
    pub fn lexbor_plog_clean_noi(plog : *mut lexbor_plog_t) -> ();
    pub fn lexbor_plog_push_noi(plog : *mut lexbor_plog_t, data :
        *const lxb_char_t, ctx : *mut c_void, id : c_uint) -> *mut c_void;
    pub fn lexbor_plog_length_noi(plog : *mut lexbor_plog_t) -> c_uint;

    // lexbor/core/shs.h
    pub fn lexbor_shs_entry_get_static(tree : *const lexbor_shs_entry_t, key :
        *const lxb_char_t, size : c_uint) -> *const lexbor_shs_entry_t;
    pub fn lexbor_shs_entry_get_lower_static(root : *const lexbor_shs_entry_t,
        key : *const lxb_char_t, key_len : c_uint) -> *const lexbor_shs_entry_t;
    pub fn lexbor_shs_entry_get_upper_static(root : *const lexbor_shs_entry_t,
        key : *const lxb_char_t, key_len : c_uint) -> *const lexbor_shs_entry_t;
    
    // lexbor/core/strtod.h
    pub fn lexbor_strtod_internal(start : *const lxb_char_t, length : c_uint,
        exp : c_int) -> c_double;    
}