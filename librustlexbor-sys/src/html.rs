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
#[path="ns.rs"] pub mod ns;
#[path="dom.rs"] pub mod dom;
extern crate libc;

use libc::{c_int, c_uint, c_ulong};
use std::os::raw::c_void;

pub type lxb_html_tag_category_t = c_int;

#[repr(C)]
pub struct lxb_html_tree_t {

}

#[repr(C)]
pub enum lxb_html_status_t {
    LXB_HTML_STATUS_OK                                                 = 0x0000
}

#[repr(C)]
pub enum lxb_html_tag_category {
    LXB_HTML_TAG_CATEGORY__UNDEF                                       = 0x0000,
    LXB_HTML_TAG_CATEGORY_ORDINARY                                     = 0x0001,
    LXB_HTML_TAG_CATEGORY_SPECIAL                                      = 0x0002,
    LXB_HTML_TAG_CATEGORY_FORMATTING                                   = 0x0004,
    LXB_HTML_TAG_CATEGORY_SCOPE                                        = 0x0008,
    LXB_HTML_TAG_CATEGORY_SCOPE_LIST_ITEM                              = 0x0010,
    LXB_HTML_TAG_CATEGORY_SCOPE_BUTTON                                 = 0x0020,
    LXB_HTML_TAG_CATEGORY_SCOPE_TABLE                                  = 0x0040,
    LXB_HTML_TAG_CATEGORY_SCOPE_SELECT                                 = 0x0080
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

#[repr(C)]
pub struct lxb_html_button_element_t {

}

#[repr(C)]
pub struct lxb_html_canvas_element_t {

}

#[repr(C)]
pub struct lxb_html_d_list_element_t {

}

#[repr(C)]
pub struct lxb_html_data_element_t {

}

#[repr(C)]
pub struct lxb_html_data_list_element_t {

}

#[repr(C)]
pub struct lxb_html_details_element_t {

}

#[repr(C)]
pub struct lxb_html_dialog_element_t {

}

#[repr(C)]
pub struct lxb_html_directory_element_t {

}

#[repr(C)]
pub struct lxb_html_div_element_t {

}

#[repr(C)]
pub struct lxb_html_element_t {
    
}

#[repr(C)]
pub struct lxb_html_embed_element_t {
    
}

#[repr(C)]
pub struct lxb_html_field_set_element_t {
    
}

#[repr(C)]
pub struct lxb_html_font_element_t {
    
}

#[repr(C)]
pub struct lxb_html_form_element_t {
    
}

#[repr(C)]
pub struct lxb_html_frame_element_t {
    
}

#[repr(C)]
pub struct lxb_html_frame_set_element_t {
    
}

#[repr(C)]
pub struct lxb_html_hr_element_t {
    
}

#[repr(C)]
pub struct lxb_html_head_element_t {
    
}

#[repr(C)]
pub struct lxb_html_heading_element_t {
    
}

#[repr(C)]
pub struct lxb_html_html_element_t {
    
}

#[repr(C)]
pub struct lxb_html_iframe_element_t {
    
}

#[repr(C)]
pub struct lxb_html_image_element_t {
    
}

#[repr(C)]
pub struct lxb_html_input_element_t {
    
}

#[repr(C)]
pub struct lxb_html_li_element_t {
    
}

#[repr(C)]
pub struct lxb_html_label_element_t {
    
}

#[repr(C)]
pub struct lxb_html_legend_element_t {
    
}

#[repr(C)]
pub struct lxb_html_link_element_t {
    
}

#[repr(C)]
pub struct lxb_html_map_element_t {
    
}

#[repr(C)]
pub struct lxb_html_marquee_element_t {
    
}

#[repr(C)]
pub struct lxb_html_media_element_t {
    
}

#[repr(C)]
pub struct lxb_html_menu_element_t {
    
}

#[repr(C)]
pub struct lxb_html_meta_element_t {
    
}

#[repr(C)]
pub struct lxb_html_meter_element_t {
    
}

#[repr(C)]
pub struct lxb_html_mod_element_t {
    
}

#[repr(C)]
pub struct lxb_html_o_list_element_t {
    
}

#[repr(C)]
pub struct lxb_html_object_element_t {
    
}

#[repr(C)]
pub struct lxb_html_opt_group_element_t {
    
}

#[repr(C)]
pub struct lxb_html_option_element_t {
    
}

#[repr(C)]
pub struct lxb_html_output_element_t {
    
}

#[repr(C)]
pub struct lxb_html_paragraph_element_t {
    
}

#[repr(C)]
pub struct lxb_html_param_element_t {
    
}

#[repr(C)]
pub struct lxb_html_picture_element_t {
    
}

#[repr(C)]
pub struct lxb_html_pre_element_t {
    
}

#[repr(C)]
pub struct lxb_html_progress_element_t {
    
}

#[repr(C)]
pub struct lxb_html_quore_element_t {
    
}

#[repr(C)]
pub struct lxb_html_script_element_t {
    
}

#[repr(C)]
pub struct lxb_html_select_element_t {
    
}

#[repr(C)]
pub struct lxb_html_slot_element_t {
    
}

#[repr(C)]
pub struct lxb_html_source_element_t {
    
}

#[repr(C)]
pub struct lxb_html_span_element_t {
    
}

#[repr(C)]
pub struct lxb_html_style_element_t {
    
}

#[repr(C)]
pub struct lxb_html_table_caption_element_t {
    
}

#[repr(C)]
pub struct lxb_html_table_cell_element_t {
    
}

#[repr(C)]
pub struct lxb_html_table_col__element_t {
    
}

#[repr(C)]
pub struct lxb_html_table_element_t {
    
}

#[repr(C)]
pub struct lxb_html_table_row_element_t {
    
}

#[repr(C)]
pub struct lxb_html_table_section_element_t {
    
}

#[repr(C)]
pub struct lxb_html_template_element_t {
    
}

#[repr(C)]
pub struct lxb_html_text_area_element_t {
    
}

#[repr(C)]
pub struct lxb_html_time_element_t {
    
}

#[repr(C)]
pub struct lxb_html_title_element_t {
    
}

#[repr(C)]
pub struct lxb_html_track_element_t {
    
}

#[repr(C)]
pub struct lxb_html_u_list_element_t {
    
}

#[repr(C)]
pub struct lxb_html_unknown_element_t {
    
}

#[repr(C)]
pub struct lxb_html_video_element_t {
    
}

#[repr(C)]
pub struct lxb_html_window_t {
    
}

#[repr(C)]
pub struct lxb_html_tag_fixname_t {
    pub name : *const core::lxb_char_t,
    pub len : c_uint
}

#[repr(C)]
pub union U {
    pub len : c_uint,
    pub num : c_ulong
}

#[repr(C)]
pub struct lxb_html_parser_char_t {
    /* It is necessary to initialize before use */
    pub state : lxb_html_parser_char_state_f,
    pub mraw : *mut core::lexbor_mraw_t,

    pub replace_null : bool,
    pub drop_null : bool,
    pub is_attribute : bool,

    /* Do not change out! Internal variables! */
    pub tmp : U,
    
    pub status : core::lxb_status_t,
    pub is_eof : bool,

    /* Parse error */
    pub parse_errors : *mut core::lexbor_array_obj_t,

    /* Entities */
    pub entity : *const core::lexbor_sbst_entry_static_t,
    pub entity_match : *const core::lexbor_sbst_entry_static_t,
    pub entity_begin : *const core::lxb_char_t,
    pub entity_str_len : c_uint
}

pub type lxb_html_parser_char_state_f = extern "C" fn(pc : 
    *mut lxb_html_parser_char_t, _str : *mut core::lexbor_str_t, data :
    *const core::lxb_char_t, end : *const core::lxb_char_t) 
    -> *const core::lxb_char_t; 

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
    
    // lexbor/html/interface.h
    pub fn lxb_html_interface_create(document : *mut lxb_html_document_t, 
        tag_id : tag::lxb_tag_id_t, _ns : ns::lxb_ns_id_t)
        -> *mut dom::lxb_dom_interface_t;
    pub fn lxb_html_interface_destroy(intrfc : *mut dom::lxb_dom_interface_t)
        -> *mut dom::lxb_dom_interface_t;

    // lexbor/html/node.h
    pub fn lxb_html_node_is_void_noi(node : dom::lxb_dom_node_t) -> bool;

    // lexbor/html/parser_char.h
    pub fn lxb_html_parser_char_process(pc : *mut lxb_html_parser_char_t, _str :
        *mut core::lexbor_str_t, in_node : *const core::lexbor_in_node_t,
        data : *const core::lxb_char_t, end : *const core::lxb_char_t)
        -> core::lxb_status_t;
    pub fn lxb_html_parser_char_copy(_str : *mut core::lexbor_str_t, mraw :
        *mut core::lexbor_mraw_t, in_node : *const core::lexbor_in_node_t,
        data : *const core::lxb_char_t, end : *const core::lxb_char_t)
        -> core::lxb_status_t;
}