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

use libc::{c_int, c_uint, c_double};
use std::os::raw::c_void;

#[repr(C)]
pub struct lxb_css_syntax_token_data_t {
    pub cb : lxb_css_syntax_token_data_cb_f,
    pub node_done : *mut core::lexbor_in_node_t,
    pub status : core::lxb_status_t,
    pub count : c_int,
    pub num : u32,
    pub is_last : bool
}

pub type lxb_css_syntax_token_data_cb_f = extern "C" fn(begin : 
    *const core::lxb_char_t, end : *const core::lxb_char_t, _str : 
    *mut core::lexbor_str_t, mraw : *mut core::lexbor_mraw_t, td :
    *mut lxb_css_syntax_token_data_t) -> *const core::lxb_char_t;

pub type lxb_css_syntax_token_cb_f = extern "C" fn(data : 
    *const core::lxb_char_t, len : c_uint, ctx : *mut c_void) 
    -> core::lxb_status_t;

pub type lxb_css_syntax_token_type_t = c_uint;
pub type lxb_css_syntax_token_data_type_t = c_uint;

#[repr(C)]
pub enum lxb_css_syntax_token_type {
    LXB_CSS_SYNTAX_TOKEN_UNDEF                                         = 0x00,
    LXB_CSS_SYNTAX_TOKEN_IDENT,
    LXB_CSS_SYNTAX_TOKEN_FUNCTION,
    LXB_CSS_SYNTAX_TOKEN_AT_KEYWORD,
    LXB_CSS_SYNTAX_TOKEN_HASH,
    LXB_CSS_SYNTAX_TOKEN_STRING,
    LXB_CSS_SYNTAX_TOKEN_BAD_STRING,
    LXB_CSS_SYNTAX_TOKEN_URL,
    LXB_CSS_SYNTAX_TOKEN_BAD_URL,
    LXB_CSS_SYNTAX_TOKEN_DELIM,
    LXB_CSS_SYNTAX_TOKEN_NUMBER,
    LXB_CSS_SYNTAX_TOKEN_PERCENTAGE,
    LXB_CSS_SYNTAX_TOKEN_DIMENSION,
    LXB_CSS_SYNTAX_TOKEN_WHITESPACE,
    LXB_CSS_SYNTAX_TOKEN_CDO,
    LXB_CSS_SYNTAX_TOKEN_CDC,
    LXB_CSS_SYNTAX_TOKEN_COLON,
    LXB_CSS_SYNTAX_TOKEN_SEMICOLON,
    LXB_CSS_SYNTAX_TOKEN_COMMA,
    LXB_CSS_SYNTAX_TOKEN_LS_BRACKET,   /* U+005B LEFT SQUARE BRACKET ([) */
    LXB_CSS_SYNTAX_TOKEN_RS_BRACKET,  /* U+005D RIGHT SQUARE BRACKET (]) */
    LXB_CSS_SYNTAX_TOKEN_L_PARENTHESIS,   /* U+0028 LEFT PARENTHESIS (() */
    LXB_CSS_SYNTAX_TOKEN_R_PARENTHESIS,  /* U+0029 RIGHT PARENTHESIS ()) */
    LXB_CSS_SYNTAX_TOKEN_LC_BRACKET,    /* U+007B LEFT CURLY BRACKET ({) */
    LXB_CSS_SYNTAX_TOKEN_RC_BRACKET,   /* U+007D RIGHT CURLY BRACKET (}) */
    LXB_CSS_SYNTAX_TOKEN_COMMENT,                /* not in specification */
    LXB_CSS_SYNTAX_TOKEN__EOF,
    LXB_CSS_SYNTAX_TOKEN__LAST_ENTRY
}

#[repr(C)]
pub enum lxb_css_syntax_token_data_type {
    LXB_CSS_SYNTAX_TOKEN_DATA_SIMPLE                                   = 0x00,
    LXB_CSS_SYNTAX_TOKEN_DATA_CR                                       = 0x01,
    LXB_CSS_SYNTAX_TOKEN_DATA_FF                                       = 0x02,
    LXB_CSS_SYNTAX_TOKEN_DATA_ESCAPED                                  = 0x04,
    LXB_CSS_SYNTAX_TOKEN_DATA_HAVE_NULL                                = 0x08
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lxb_css_syntax_token_base_t {
    pub _type : lxb_css_syntax_token_type_t,
    pub data_type : lxb_css_syntax_token_data_type_t
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lxb_css_syntax_token_number_t {
    pub base : lxb_css_syntax_token_base_t,
    pub num : c_double,
    pub is_float : bool
}

#[repr(C)]
pub struct lxb_css_syntax_token_dimension_t {
    /* Do not change it */
    pub num : lxb_css_syntax_token_number_t,
    pub data : core::lexbor_str_t,
    
    /* Ident */
    pub begin : *mut core::lxb_char_t,
    pub end : *mut core::lxb_char_t
}

#[repr(C)]
pub struct lxb_css_syntax_token_string_t {
    pub base : lxb_css_syntax_token_base_t,
    pub data : core::lexbor_str_t,
    pub begin : *mut core::lxb_char_t,
    pub end : *mut core::lxb_char_t
}

#[repr(C)]
pub struct lxb_css_syntax_token_delim_t {
    pub base : lxb_css_syntax_token_base_t,
    pub character : core::lxb_char_t,
    pub begin : *const core::lxb_char_t,
    pub end : *const core::lxb_char_t
}

pub type lxb_css_syntax_token_ident_t = lxb_css_syntax_token_string_t;
pub type lxb_css_syntax_token_function_t = lxb_css_syntax_token_string_t;
pub type lxb_css_syntax_token_at_keyword_t = lxb_css_syntax_token_string_t;
pub type lxb_css_syntax_token_hash_t = lxb_css_syntax_token_string_t;
pub type lxb_css_syntax_token_bad_string_t = lxb_css_syntax_token_string_t;
pub type lxb_css_syntax_token_url_t = lxb_css_syntax_token_string_t;
pub type lxb_css_syntax_token_bad_url_t = lxb_css_syntax_token_string_t;
pub type lxb_css_syntax_token_percentage_t = lxb_css_syntax_token_number_t;
pub type lxb_css_syntax_token_whitespace_t = lxb_css_syntax_token_string_t;
pub type lxb_css_syntax_token_cdo_t = lxb_css_syntax_token_base_t;
pub type lxb_css_syntax_token_cdc_t = lxb_css_syntax_token_base_t;
pub type lxb_css_syntax_token_colon_t = lxb_css_syntax_token_base_t;
pub type lxb_css_syntax_token_semicolon_t = lxb_css_syntax_token_base_t;
pub type lxb_css_syntax_token_comma_t = lxb_css_syntax_token_base_t;
pub type lxb_css_syntax_token_ls_bracket_t = lxb_css_syntax_token_base_t;
pub type lxb_css_syntax_token_rs_bracket_t = lxb_css_syntax_token_base_t;
pub type lxb_css_syntax_token_l_parenthesis_t = lxb_css_syntax_token_base_t;
pub type lxb_css_syntax_token_r_parenthesis_t = lxb_css_syntax_token_base_t;
pub type lxb_css_syntax_token_lc_bracket_t = lxb_css_syntax_token_base_t;
pub type lxb_css_syntax_token_rc_bracket_t = lxb_css_syntax_token_base_t;
pub type lxb_css_syntax_token_comment_t = lxb_css_syntax_token_string_t;

/*
#[repr(C)]
pub union Types {
    pub base : lxb_css_syntax_token_base_t,
    pub comment : lxb_css_syntax_token_comment_t,
    pub number : lxb_css_syntax_token_number_t,
    pub dimension : lxb_css_syntax_token_dimension_t,
    pub percentage : lxb_css_syntax_token_percentage_t,
    pub hash : lxb_css_syntax_token_hash_t,
    pub string : lxb_css_syntax_token_string_t,
    pub bad_string : lxb_css_syntax_token_bad_string_t,
    pub delim : lxb_css_syntax_token_delim_t,
    pub lparenthesis : lxb_css_syntax_token_l_parenthesis_t,
    pub rparenthesis : lxb_css_syntax_token_r_parenthesis_t,
    pub cdc : lxb_css_syntax_token_cdc_t,
    pub function : lxb_css_syntax_token_function_t,
    pub ident : lxb_css_syntax_token_ident_t,
    pub url : lxb_css_syntax_token_url_t,
    pub bad_url : lxb_css_syntax_token_bad_url_t,
    pub at_keyword : lxb_css_syntax_token_at_keyword_t,
    pub whitespace : lxb_css_syntax_token_whitespace_t
}
*/

pub type lxb_css_syntax_token_t = *mut c_void;

pub type lxb_css_syntax_tokenizer_state_f = extern "C" fn(tkz :
    *mut lxb_css_syntax_tokenizer_t, data : *const core::lxb_char_t, end :
    *const core::lxb_char_t) -> *const core::lxb_char_t;

pub type lxb_css_syntax_tokenizer_cb_f = extern "C" fn(tkz :
    *mut lxb_css_syntax_tokenizer_t, token : *mut lxb_css_syntax_token_t, ctx :
    *mut c_void) -> *mut lxb_css_syntax_token_t;

#[repr(C)]
pub enum lxb_css_syntax_tokenizer_opt {
    LXB_CSS_SYNTAX_TOKENIZER_OPT_UNDEF                                 = 0x00,
    LXB_CSS_SYNTAX_TOKENIZER_OPT_WO_COPY                               = 0x01 
}

#[repr(C)]
pub enum lxb_css_syntax_process_state_t {
    LXB_CSS_SYNTAX_TOKENIZER_BEGIN                                     = 0x00,
    LXB_CSS_SYNTAX_TOKENIZER_PROCESS                                   = 0x01,
    LXB_CSS_SYNTAX_TOKENIZER_END                                       = 0x02   
}

#[link(name = "lexbor")]
extern "C" {
    // lexbor/css/systax/token.h
    pub fn lxb_css_syntax_token_type_name_by_id(_type : 
        lxb_css_syntax_token_type_t) -> *const core::lxb_char_t;
    pub fn lxb_css_syntax_token_type_id_by_name(type_name : 
        *const core::lxb_char_t, len : c_uint) -> lxb_css_syntax_token_type_t;
    pub fn lxb_css_syntax_token_make_data(token : *mut lxb_css_syntax_token_t,
        _in : *mut core::lexbor_in_node_t, mraw : *mut core::lexbor_mraw_t,
        td : *mut lxb_css_syntax_token_data_t) -> core::lxb_status_t;
    pub fn lxb_css_syntax_token_serialize_cb(token : 
        *mut lxb_css_syntax_token_t, cb : lxb_css_syntax_token_cb_f, mraw : 
        *mut core::lexbor_mraw_t) -> core::lxb_status_t;
    pub fn lxb_css_syntax_token_serialize_str(token : 
        *mut lxb_css_syntax_token_t, _str : *mut core::lexbor_str_t, mraw :
        *mut core::lexbor_mraw_t) -> core::lxb_status_t;
    pub fn lxb_css_syntax_token_create_noi(dobj : *mut core::lexbor_dobject_t)
        -> *mut lxb_css_syntax_token_t;
    pub fn lxb_css_syntax_token_clean_noi(token : *mut lxb_css_syntax_token_t)
        -> ();
    pub fn lxb_css_syntax_token_destroy_noi(token : *mut lxb_css_syntax_token_t,
        dobj : *mut core::lexbor_dobject_t) -> *mut lxb_css_syntax_token_t;
    pub fn lxb_css_syntax_token_type_name_noi(token : 
        *mut lxb_css_syntax_token_t) -> *const core::lxb_char_t;
    pub fn lxb_css_syntax_token_type_noi(token : *mut lxb_css_syntax_token_t)
        -> lxb_css_syntax_token_type_t;
}