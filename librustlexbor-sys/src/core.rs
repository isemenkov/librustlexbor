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

extern crate libc;

pub mod base;
use base::{lxb_status_t};
use libc::{c_schar, c_short, c_ushort, c_int, c_uint, c_longlong, c_double};
use std::os::raw::c_void;

#[repr(C)]
pub struct lexbor_array_t {
    pub list : *mut *mut c_void;
    pub size : c_uint;
    pub length : c_uint;
}

#[link(name = "lexbor")]
extern "C" {
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

}