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

use libc::{c_int, c_uint, c_ulong, uintptr_t};
use std::os::raw::c_void;

pub type lxb_html_tag_category_t = c_int;
pub type lxb_html_document_opt_t = c_uint;
pub type lxb_html_tokenizer_opt_t = c_uint;

#[repr(C)]
pub struct lxb_html_tree_t {
    pub tkz_ref : *mut lxb_html_tokenizer_t,

    pub document : *mut lxb_html_document_t,
    pub fragment : *mut dom::lxb_dom_node_t,

    pub form : *mut lxb_html_form_element_t,

    pub open_elements : *mut core::lexbor_array_t,
    pub active_formatting : *mut core::lexbor_array_t,
    pub template_insertion_modes : *mut core::lexbor_array_obj_t,

    pub pending_table : lxb_html_tree_pending_table_t,

    pub parse_errors : *mut core::lexbor_array_obj_t,

    pub foster_parenting : bool,
    pub frameset_ok : bool,
    pub scripting : bool,

    pub mode : lxb_html_tree_insertion_mode_f,
    pub original_mode : lxb_html_tree_insertion_mode_f,
    pub before_append_attr : lxb_html_tree_append_attr_f,

    pub status : core::lxb_status_t,

    pub ref_count : c_uint
}

#[repr(C)]
pub enum lxb_html_status_t {
    LXB_HTML_STATUS_OK                                                 = 0x0000
}

#[repr(C)]
pub enum lxb_html_document_ready_state_t {
    LXB_HTML_DOCUMENT_READY_STATE_UNDEF                                = 0x00,
    LXB_HTML_DOCUMENT_READY_STATE_LOADING                              = 0x01,
    LXB_HTML_DOCUMENT_READY_STATE_INTERACTIVE                          = 0x02,
    LXB_HTML_DOCUMENT_READY_STATE_COMPLETE                             = 0x03
}

#[repr(C)]
pub enum lxb_html_document_opt {
    LXB_HTML_DOCUMENT_OPT_UNDEF                                        = 0x00,
    LXB_HTML_DOCUMENT_PARSE_WO_COPY                                    = 0x01
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
pub enum lxb_html_token_type_t {
    LXB_HTML_TOKEN_TYPE_OPEN                                           = 0x0000,
    LXB_HTML_TOKEN_TYPE_CLOSE                                          = 0x0001,
    LXB_HTML_TOKEN_TYPE_CLOSE_SELF                                     = 0x0002,
    LXB_HTML_TOKEN_TYPE_FORCE_QUIRKS                                   = 0x0004,
    LXB_HTML_TOKEN_TYPE_DONE                                           = 0x0008
}

#[repr(C)]
pub enum lxb_html_token_attr_type_t {
    LXB_HTML_TOKEN_ATTR_TYPE_UNDEF                                     = 0x0000,
    LXB_HTML_TOKEN_ATTR_TYPE_NAME_NULL                                 = 0x0001,
    LXB_HTML_TOKEN_ATTR_TYPE_VALUE_NULL                                = 0x0002
}

#[repr(C)]
pub enum lxb_html_tree_insertion_position_t {
    LXB_HTML_TREE_INSERTION_POSITION_CHILD                             = 0x00,
    LXB_HTML_TREE_INSERTION_POSITION_BEFORE                            = 0x01
}

#[repr(C)]
pub enum lxb_html_tokenizer_error_id_t {
    /* abrupt-closing-of-empty-comment */
    LXB_HTML_TOKENIZER_ERROR_ABCLOFEMCO                                = 0x0000,
    /* abrupt-doctype-public-identifier */
    LXB_HTML_TOKENIZER_ERROR_ABDOPUID                                  = 0x0001,
    /* abrupt-doctype-system-identifier */
    LXB_HTML_TOKENIZER_ERROR_ABDOSYID                                  = 0x0002,
    /* absence-of-digits-in-numeric-character-reference */
    LXB_HTML_TOKENIZER_ERROR_ABOFDIINNUCHRE                            = 0x0003,
    /* cdata-in-html-content */
    LXB_HTML_TOKENIZER_ERROR_CDINHTCO                                  = 0x0004,
    /* character-reference-outside-unicode-range */
    LXB_HTML_TOKENIZER_ERROR_CHREOUUNRA                                = 0x0005,
    /* control-character-in-input-stream */
    LXB_HTML_TOKENIZER_ERROR_COCHININST                                = 0x0006,
    /* control-character-reference */
    LXB_HTML_TOKENIZER_ERROR_COCHRE                                    = 0x0007,
    /* end-tag-with-attributes */
    LXB_HTML_TOKENIZER_ERROR_ENTAWIAT                                  = 0x0008,
    /* duplicate-attribute */
    LXB_HTML_TOKENIZER_ERROR_DUAT                                      = 0x0009,
    /* end-tag-with-trailing-solidus */
    LXB_HTML_TOKENIZER_ERROR_ENTAWITRSO                                = 0x000A,
    /* eof-before-tag-name */
    LXB_HTML_TOKENIZER_ERROR_EOBETANA                                  = 0x000B,
    /* eof-in-cdata */
    LXB_HTML_TOKENIZER_ERROR_EOINCD                                    = 0x000C,
    /* eof-in-comment */
    LXB_HTML_TOKENIZER_ERROR_EOINCO                                    = 0x000D,
    /* eof-in-doctype */
    LXB_HTML_TOKENIZER_ERROR_EOINDO                                    = 0x000E,
    /* eof-in-script-html-comment-like-text */
    LXB_HTML_TOKENIZER_ERROR_EOINSCHTCOLITE                            = 0x000F,
    /* eof-in-tag */
    LXB_HTML_TOKENIZER_ERROR_EOINTA                                    = 0x0010,
    /* incorrectly-closed-comment */
    LXB_HTML_TOKENIZER_ERROR_INCLCO                                    = 0x0011,
    /* incorrectly-opened-comment */
    LXB_HTML_TOKENIZER_ERROR_INOPCO                                    = 0x0012,
    /* invalid-character-sequence-after-doctype-name */
    LXB_HTML_TOKENIZER_ERROR_INCHSEAFDONA                              = 0x0013,
    /* invalid-first-character-of-tag-name */
    LXB_HTML_TOKENIZER_ERROR_INFICHOFTANA                              = 0x0014,
    /* missing-attribute-value */
    LXB_HTML_TOKENIZER_ERROR_MIATVA                                    = 0x0015,
    /* missing-doctype-name */
    LXB_HTML_TOKENIZER_ERROR_MIDONA                                    = 0x0016,
    /* missing-doctype-public-identifier */
    LXB_HTML_TOKENIZER_ERROR_MIDOPUID                                  = 0x0017,
    /* missing-doctype-system-identifier */
    LXB_HTML_TOKENIZER_ERROR_MIDOSYID                                  = 0x0018,
    /* missing-end-tag-name */
    LXB_HTML_TOKENIZER_ERROR_MIENTANA                                  = 0x0019,
    /* missing-quote-before-doctype-public-identifier */
    LXB_HTML_TOKENIZER_ERROR_MIQUBEDOPUID                              = 0x001A,
    /* missing-quote-before-doctype-system-identifier */
    LXB_HTML_TOKENIZER_ERROR_MIQUBEDOSYID                              = 0x001B,
    /* missing-semicolon-after-character-reference */
    LXB_HTML_TOKENIZER_ERROR_MISEAFCHRE                                = 0x001C,
    /* missing-whitespace-after-doctype-public-keyword */
    LXB_HTML_TOKENIZER_ERROR_MIWHAFDOPUKE                              = 0x001D,
    /* missing-whitespace-after-doctype-system-keyword */
    LXB_HTML_TOKENIZER_ERROR_MIWHAFDOSYKE                              = 0x001E,
    /* missing-whitespace-before-doctype-name */
    LXB_HTML_TOKENIZER_ERROR_MIWHBEDONA                                = 0x001F,
    /* missing-whitespace-between-attributes */
    LXB_HTML_TOKENIZER_ERROR_MIWHBEAT                                  = 0x0020,
    /* missing-whitespace-between-doctype-public-and-system-identifiers */
    LXB_HTML_TOKENIZER_ERROR_MIWHBEDOPUANSYID                          = 0x0021,
    /* nested-comment */
    LXB_HTML_TOKENIZER_ERROR_NECO                                      = 0x0022,
    /* noncharacter-character-reference */
    LXB_HTML_TOKENIZER_ERROR_NOCHRE                                    = 0x0023,
    /* noncharacter-in-input-stream */
    LXB_HTML_TOKENIZER_ERROR_NOININST                                  = 0x0024,
    /* non-void-html-element-start-tag-with-trailing-solidus */
    LXB_HTML_TOKENIZER_ERROR_NOVOHTELSTTAWITRSO                        = 0x0025,
    /* null-character-reference */
    LXB_HTML_TOKENIZER_ERROR_NUCHRE                                    = 0x0026,
    /* surrogate-character-reference */
    LXB_HTML_TOKENIZER_ERROR_SUCHRE                                    = 0x0027,
    /* surrogate-in-input-stream */
    LXB_HTML_TOKENIZER_ERROR_SUININST                                  = 0x0028,
    /* unexpected-character-after-doctype-system-identifier */
    LXB_HTML_TOKENIZER_ERROR_UNCHAFDOSYID                              = 0x0029,
    /* unexpected-character-in-attribute-name */
    LXB_HTML_TOKENIZER_ERROR_UNCHINATNA                                = 0x002A,
    /* unexpected-character-in-unquoted-attribute-value */
    LXB_HTML_TOKENIZER_ERROR_UNCHINUNATVA                              = 0x002B,
    /* unexpected-equals-sign-before-attribute-name */
    LXB_HTML_TOKENIZER_ERROR_UNEQSIBEATNA                              = 0x002C,
    /* unexpected-null-character */
    LXB_HTML_TOKENIZER_ERROR_UNNUCH                                    = 0x002D,
    /* unexpected-question-mark-instead-of-tag-name */
    LXB_HTML_TOKENIZER_ERROR_UNQUMAINOFTANA                            = 0x002E,
    /* unexpected-solidus-in-tag */
    LXB_HTML_TOKENIZER_ERROR_UNSOINTA                                  = 0x002F,
    /* unknown-named-character-reference */
    LXB_HTML_TOKENIZER_ERROR_UNNACHRE                                  = 0x0030,
    LXB_HTML_TOKENIZER_ERROR_LAST_ENTRY                                = 0x0031,
}

#[repr(C)]
pub enum lxb_html_tree_error_id_t {
    /* unexpected-token */
    LXB_HTML_RULES_ERROR_UNTO                                          = 0x0000,
    /* unexpected-closed-token */
    LXB_HTML_RULES_ERROR_UNCLTO,
    /* null-character */
    LXB_HTML_RULES_ERROR_NUCH,
    /* unexpected-character-token */
    LXB_HTML_RULES_ERROR_UNCHTO,
    /* unexpected-token-in-initial-mode */
    LXB_HTML_RULES_ERROR_UNTOININMO,
    /* bad-doctype-token-in-initial-mode */
    LXB_HTML_RULES_ERROR_BADOTOININMO,
    /* doctype-token-in-before-html-mode */
    LXB_HTML_RULES_ERROR_DOTOINBEHTMO,
    /* unexpected-closed-token-in-before-html-mode */
    LXB_HTML_RULES_ERROR_UNCLTOINBEHTMO,
    /* doctype-token-in-before-head-mode */
    LXB_HTML_RULES_ERROR_DOTOINBEHEMO,
    /* unexpected-closed_token-in-before-head-mode */
    LXB_HTML_RULES_ERROR_UNCLTOINBEHEMO,
    /* doctype-token-in-head-mode */
    LXB_HTML_RULES_ERROR_DOTOINHEMO,
    /* non-void-html-element-start-tag-with-trailing-solidus */
    LXB_HTML_RULES_ERROR_NOVOHTELSTTAWITRSO,
    /* head-token-in-head-mode */
    LXB_HTML_RULES_ERROR_HETOINHEMO,
    /* unexpected-closed-token-in-head-mode */
    LXB_HTML_RULES_ERROR_UNCLTOINHEMO,
    /* template-closed-token-without-opening-in-head-mode */
    LXB_HTML_RULES_ERROR_TECLTOWIOPINHEMO,
    /* template-element-is-not-current-in-head-mode */
    LXB_HTML_RULES_ERROR_TEELISNOCUINHEMO,
    /* doctype-token-in-head-noscript-mode */
    LXB_HTML_RULES_ERROR_DOTOINHENOMO,
    /* doctype-token-after-head-mode */
    LXB_HTML_RULES_ERROR_DOTOAFHEMO,
    /* head-token-after-head-mode */
    LXB_HTML_RULES_ERROR_HETOAFHEMO,
    /* doctype-token-in-body-mode */
    LXB_HTML_RULES_ERROR_DOTOINBOMO,
    /* bad-ending-open-elements-is-wrong */
    LXB_HTML_RULES_ERROR_BAENOPELISWR,
    /* open-elements-is-wrong */
    LXB_HTML_RULES_ERROR_OPELISWR,
    /* unexpected-element-in-open-elements-stack */
    LXB_HTML_RULES_ERROR_UNELINOPELST,
    /* missing-element-in-open-elements-stack */
    LXB_HTML_RULES_ERROR_MIELINOPELST,
    /* no-body-element-in-scope */
    LXB_HTML_RULES_ERROR_NOBOELINSC,
    /* missing-element-in-scope */
    LXB_HTML_RULES_ERROR_MIELINSC,
    /* unexpected-element-in-scope */
    LXB_HTML_RULES_ERROR_UNELINSC,
    /* unexpected-element-in-active-formatting-stack */
    LXB_HTML_RULES_ERROR_UNELINACFOST,
    /* unexpected-end-of-file */
    LXB_HTML_RULES_ERROR_UNENOFFI,
    /* characters-in-table-text */
    LXB_HTML_RULES_ERROR_CHINTATE,
    /* doctype-token-in-table-mode */
    LXB_HTML_RULES_ERROR_DOTOINTAMO,
    /* doctype-token-in-select-mode */
    LXB_HTML_RULES_ERROR_DOTOINSEMO,
    /* doctype-token-after-body-mode */
    LXB_HTML_RULES_ERROR_DOTOAFBOMO,
    /* doctype-token-in-frameset-mode */
    LXB_HTML_RULES_ERROR_DOTOINFRMO,
    /* doctype-token-after-frameset-mode */
    LXB_HTML_RULES_ERROR_DOTOAFFRMO,
    /* doctype-token-foreign-content-mode */
    LXB_HTML_RULES_ERROR_DOTOFOCOMO,

    LXB_HTML_RULES_ERROR_LAST_ENTRY
}

#[repr(C)]
pub enum lxb_html_parser_state_t {
    LXB_HTML_PARSER_STATE_BEGIN                                        = 0x00,
    LXB_HTML_PARSER_STATE_PROCESS                                      = 0x01,
    LXB_HTML_PARSER_STATE_END                                          = 0x02,
    LXB_HTML_PARSER_STATE_FRAGMENT_PROCESS                             = 0x03,
    LXB_HTML_PARSER_STATE_ERROR                                        = 0x04
}

#[repr(C)]
pub struct lxb_html_parser_t {
    pub tkz : *mut lxb_html_tokenizer_t,
    pub tree : *mut lxb_html_tree_t,
    pub original_tree : *mut lxb_html_tree_t,

    pub root : *mut dom::lxb_dom_node_t,
    pub form : *mut dom::lxb_dom_node_t,

    pub state : lxb_html_parser_state_t,
    pub status : core::lxb_status_t,

    pub ref_count : c_uint
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
    pub dom_document : dom::lxb_dom_document_t,

    pub iframe_srcdoc : *mut c_void,

    pub head : *mut lxb_html_head_element_t,
    pub body : *mut lxb_html_body_element_t,

    pub ready_state : lxb_html_document_ready_state_t,

    pub opt : lxb_html_document_opt_t
}

#[repr(C)]
pub struct lxb_html_anchor_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_area_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_audio_element_t {
    pub media_element : lxb_html_media_element_t
}

#[repr(C)]
pub struct lxb_html_br_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_base_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_body_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_button_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_canvas_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_d_list_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_data_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_data_list_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_details_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_dialog_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_directory_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_div_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_element_t {
    pub element : dom::lxb_dom_element_t
}

#[repr(C)]
pub struct lxb_html_embed_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_field_set_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_font_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_form_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_frame_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_frame_set_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_hr_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_head_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_heading_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_html_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_iframe_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_image_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_input_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_li_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_label_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_legend_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_link_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_map_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_marquee_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_media_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_menu_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_meta_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_meter_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_mod_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_o_list_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_object_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_opt_group_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_option_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_output_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_paragraph_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_param_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_picture_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_pre_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_progress_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_quote_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_script_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_select_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_slot_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_source_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_span_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_style_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_table_caption_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_table_cell_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_table_col_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_table_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_table_row_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_table_section_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_template_element_t {
    pub element : lxb_html_element_t,
    pub content : *mut dom::lxb_dom_document_fragment_t
}

#[repr(C)]
pub struct lxb_html_text_area_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_time_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_title_element_t {
    pub element : lxb_html_element_t,
    pub strict_text : *mut core::lexbor_str_t
}

#[repr(C)]
pub struct lxb_html_track_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_u_list_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_unknown_element_t {
    pub element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_video_element_t {
    pub media_element : lxb_html_element_t
}

#[repr(C)]
pub struct lxb_html_window_t {
    pub event_target : dom::lxb_dom_event_target_t
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

#[repr(C)]
pub struct lxb_html_token_attr_t {
    pub name_begin : *const core::lxb_char_t,
    pub name_end : *const core::lxb_char_t,

    pub value_begin : *const core::lxb_char_t,
    pub value_end : *const core::lxb_char_t,

    pub name : *const dom::lxb_dom_attr_data_t,
    pub value : *mut core::lxb_char_t,
    pub value_size : c_uint,

    pub in_name : *mut core::lexbor_in_node_t,
    pub in_value : *mut core::lexbor_in_node_t,

    pub next : *mut lxb_html_token_attr_t,
    pub prev : *mut lxb_html_token_attr_t,

    pub type_ : lxb_html_token_attr_type_t
}

#[repr(C)]
pub struct lxb_html_token_t {
    pub begin : *const core::lxb_char_t,
    pub end : *const core::lxb_char_t,

    pub text_start : *const core::lxb_char_t,
    pub text_end : *const core::lxb_char_t,

    pub in_begin : *mut core::lexbor_in_node_t,

    pub attr_first : *mut lxb_html_token_attr_t,
    pub attr_last : *mut lxb_html_token_attr_t,

    pub base_element : *mut c_void,

    pub null_count : c_uint,
    pub tag_id : tag::lxb_tag_id_t,
    pub type_ : lxb_html_token_type_t
}

#[repr(C)]
pub struct lxb_html_tokenizer_t {
    pub state : lxb_html_tokenizer_state_f,
    pub state_return : lxb_html_tokenizer_state_f,

    pub callback_token_done : lxb_html_tokenizer_token_f,
    pub callback_token_ctx : *mut c_void,

    pub tags : *mut core::lexbor_hash_t,
    pub attr : *mut core::lexbor_hash_t,
    pub attrs_mraw : *mut core::lexbor_mraw_t,

    /* For a temp strings and other templary data */
    pub mraw : *mut core::lexbor_mraw_t,

    /* Current process token */
    pub token : *mut lxb_html_token_t,

    /* Memory for token and attr */
    pub dobj_token : *mut core::lexbor_dobject_t,
    pub dobj_token_attr : *mut core::lexbor_dobject_t,

    /* Parse error */
    pub parse_errors : *mut core::lexbor_array_obj_t,

    /*
     * Leak abstractions.
     * The only place where the specification causes mixing Tree Builder
     * and Tokenizer. We kill all beauty.
     * Current Tree parser. This is not ref (not ref count).
     */
    pub tree : *mut lxb_html_tree_t,

    /* Temp */
    pub markup : *const core::lxb_char_t,
    pub temp : *const core::lxb_char_t,
    pub tmp_tag_id : tag::lxb_tag_id_t,

    pub start : *mut core::lxb_char_t,
    pub pos : *mut core::lxb_char_t,
    pub end : *const core::lxb_char_t,
    pub begin : *const core::lxb_char_t,
    pub last : *const core::lxb_char_t,

    /* Entities */
    pub entity : *const core::lexbor_sbst_entry_static_t,
    pub entity_match : *const core::lexbor_sbst_entry_static_t,
    pub entity_start : uintptr_t,
    pub entity_end : uintptr_t,
    pub entity_length : u32,
    pub entity_number : u32,
    pub is_attribute : bool,

    /* Process */
    pub opt : lxb_html_tokenizer_opt_t,
    pub status : core::lexbor_status_t,
    pub is_eof : bool,

    pub base : *mut lxb_html_tokenizer_t,
    pub ref_count : c_uint
}

pub type lxb_html_tokenizer_state_f = extern "C" fn(tkz : 
    *mut lxb_html_tokenizer_t, data : *const core::lxb_char_t, end : 
    *const core::lxb_char_t) -> *const core::lxb_char_t;

pub type lxb_html_tokenizer_token_f = extern "C" fn(tkz : 
    *mut lxb_html_tokenizer_t, token : *mut lxb_html_token_t, ctx : *mut c_void)
    -> *mut lxb_html_token_t;

pub type lxb_html_tree_insertion_mode_f = extern "C" fn(tree :
    *mut lxb_html_tree_t, token : *mut lxb_html_token_t) -> bool;

pub type lxb_html_tree_append_attr_f = extern "C" fn(tree :
    *mut lxb_html_tree_t, attr : *mut dom::lxb_dom_attr_t, ctx : *mut c_void)
    -> core::lxb_status_t;

#[repr(C)]
pub struct lxb_html_tree_pending_table_t {
    pub text_list : *mut core::lexbor_array_obj_t,
    pub have_non_ws : bool
}

#[repr(C)]
pub struct lxb_html_tokenizer_error_t {
    pub pos : *const core::lxb_char_t,
    pub id : lxb_html_tokenizer_error_id_t
}

#[repr(C)]
pub struct lxb_html_tree_error_t {
    pub id : lxb_html_tree_error_id_t
}

#[repr(C)]
pub struct lxb_html_tree_template_insertion_t {
    pub mode : lxb_html_tree_insertion_mode_f
}

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
    
    // lexbor/html/token_attr.h
    pub fn lxb_html_token_attr_create(dobj : *mut core::lexbor_dobject_t)
        -> *mut lxb_html_token_attr_t;
    pub fn lxb_html_token_attr_clean(attr : *mut lxb_html_token_attr_t) -> ();
    pub fn lxb_html_token_attr_destroy(attr : *mut lxb_html_token_attr_t,
        dobj : *mut core::lexbor_dobject_t) -> *mut lxb_html_token_attr_t;
    pub fn lxb_html_token_attr_name(attr : *mut lxb_html_token_attr_t, length :
        *mut c_uint) -> *const core::lxb_char_t;

    // lexbor/html/token.h
    pub fn lxb_html_token_create(dobj : *mut core::lexbor_dobject_t)
        -> *mut lxb_html_token_t;
    pub fn lxb_html_token_destroy(token : *mut lxb_html_token_t, dobj : 
        *mut core::lexbor_dobject_t) -> *mut lxb_html_token_t;
    pub fn lxb_html_token_attr_append(token : *mut lxb_html_token_t, dobj :
        *mut core::lexbor_dobject_t) -> *mut lxb_html_token_attr_t;
    pub fn lxb_html_token_attr_remove(token : *mut lxb_html_token_t, attr :
        *mut lxb_html_token_attr_t) -> ();
    pub fn lxb_html_token_attr_delete(token : *mut lxb_html_token_t, attr :
        *mut lxb_html_token_attr_t, dobj : core::lexbor_dobject_t) -> ();
    pub fn lxb_html_token_make_text(token : *mut lxb_html_token_t, str_ :
        *mut core::lexbor_str_t, mraw : *mut core::lexbor_mraw_t) 
        -> core::lxb_status_t;
    pub fn lxb_html_token_make_text_drop_null(token : *mut lxb_html_token_t,
        str_ : *mut core::lexbor_str_t, mraw : *mut core::lexbor_mraw_t)
        -> core::lxb_status_t;
    pub fn lxb_html_token_make_text_replace_null(token : *mut lxb_html_token_t,
        str_ : *mut core::lexbor_str_t, mraw : *mut core::lexbor_mraw_t)
        -> core::lxb_status_t;
    pub fn lxb_html_token_data_skip_ws_begin(token : *mut lxb_html_token_t)
        -> core::lxb_status_t;
    pub fn lxb_html_token_data_skip_one_newline_begin(token : 
        *mut lxb_html_token_t) -> core::lxb_status_t;
    pub fn lxb_html_token_data_split_ws_begin(token : *mut lxb_html_token_t,
        ws_token : *mut lxb_html_token_t) -> core::lxb_status_t;
    pub fn lxb_html_token_doctype_parse(token : *mut lxb_html_token_t, 
        doc_type : dom::lxb_dom_document_type_t) -> core::lxb_status_t;
    pub fn lxb_html_token_find_attr(tkz : *mut lxb_html_tokenizer_t, token :
        *mut lxb_html_token_t, name : *const core::lxb_status_t, name_len :
        c_uint) -> *mut lxb_html_token_attr_t;
    pub fn lxb_html_token_clean_noi(token : *mut lxb_html_token_t) -> ();
    pub fn lxb_html_token_create_eof_noi(dobj : *mut core::lexbor_dobject_t)
        -> *mut lxb_html_token_t;

    // lexbor/html/tokenizer.h
    pub fn lxb_html_tokenizer_create() -> *mut lxb_html_tokenizer_t;
    pub fn lxb_html_tokenizer_init(tzk : *mut lxb_html_tokenizer_t) 
        -> core::lexbor_status_t;
    pub fn lxb_html_tokenizer_inherit(tkz_to : *mut lxb_html_tokenizer_t,
        tkz_from : *mut lxb_html_tokenizer_t) -> core::lexbor_status_t;
    pub fn lxb_html_tokenizer_ref(tkz : *mut lxb_html_tokenizer_t)
        -> *mut lxb_html_tokenizer_t;
    pub fn lxb_html_tokenizer_unref(tkz : *mut lxb_html_tokenizer_t)
        -> *mut lxb_html_tokenizer_t;
    pub fn lxb_html_tokenizer_clean(tkz : *mut lxb_html_tokenizer_t) -> ();
    pub fn lxb_html_tokenizer_destroy(tkz : *mut lxb_html_tokenizer_t)
        -> *mut lxb_html_tokenizer_t;
    pub fn lxb_html_tokenizer_tags_make(tkz : *mut lxb_html_tokenizer_t,
        table_size : c_uint) -> core::lexbor_status_t;
    pub fn lxb_html_tokenizer_tags_destroy(tkz : *mut lxb_html_tokenizer_t)
        -> ();
    pub fn lxb_html_tokenizer_attrs_make(tkz : *mut lxb_html_tokenizer_t,
        table_size : c_uint) -> core::lexbor_status_t;
    pub fn lxb_html_tokenizer_attrs_destroy(tkz : *mut lxb_html_tokenizer_t)
        -> ();
    pub fn lxb_html_tokenizer_begin(tkz : *mut lxb_html_tokenizer_t)
        -> core::lexbor_status_t;
    pub fn lxb_html_tokenizer_chunk(tkz : *mut lxb_html_tokenizer_t, data :
        *const core::lxb_char_t, size : c_uint) -> core::lexbor_status_t;
    pub fn lxb_html_tokenizer_end(tkz : *mut lxb_html_tokenizer_t)
        -> core::lexbor_status_t;
    pub fn lxb_html_tokenizer_change_incoming(tkz : *mut lxb_html_tokenizer_t,
        pos : *const core::lxb_char_t) -> *const core::lxb_char_t;
    pub fn lxb_html_tokenizer_current_namespace(tkz : *mut lxb_html_tokenizer_t)
        -> ns::lxb_ns_id_t;
    pub fn lxb_html_tokenizer_set_state_by_tag(tkz : *mut lxb_html_tokenizer_t,
        scripting : bool, tag_id : tag::lxb_tag_id_t, ns_ : ns::lxb_ns_id_t)
        -> ();
    pub fn lxb_html_tokenizer_status_set_noi(tkz : *mut lxb_html_tokenizer_t,
        status : core::lxb_status_t) -> ();
    pub fn lxb_html_tokenizer_callback_token_done_set_noi(tkz : 
        *mut lxb_html_tokenizer_t, call_func : lxb_html_tokenizer_token_f,
        ctx : *mut c_void) -> ();
    pub fn lxb_html_tokenizer_callback_token_done_ctx_noi(tkz : 
        *mut lxb_html_tokenizer_t) -> *mut c_void;
    pub fn lxb_html_tokenizer_state_set_noi(tkz : *mut lxb_html_tokenizer_t,
        state : lxb_html_tokenizer_state_f) -> ();
    pub fn lxb_html_tokenizer_tmp_tag_id_set_noi(tkz : 
        *mut lxb_html_tokenizer_t, tag_id : tag::lxb_tag_id_t) -> ();
    pub fn lxb_html_tokenizer_tree_noi(tkz : *mut lxb_html_tokenizer_t)
        -> *mut lxb_html_tree_t;
    pub fn lxb_html_tokenizer_tree_set_noi(tkz : *mut lxb_html_tokenizer_t,
        tree : *mut lxb_html_tree_t) -> ();
    pub fn lxb_html_tokenizer_mraw_noi(tkz : *mut lxb_html_tokenizer_t)
        -> *mut core::lexbor_mraw_t;
    pub fn lxb_html_tokenizer_tags_noi(tkz : *mut lxb_html_tokenizer_t)
        -> *mut core::lexbor_hash_t;

    // lexbor/html/tree.h
    pub fn lxb_html_tree_create() -> *mut lxb_html_tree_t;
    pub fn lxb_html_tree_init(tree : *mut lxb_html_tree_t, tkz :
        *mut lxb_html_tokenizer_t) -> core::lxb_status_t;
    pub fn lxb_html_tree_ref(tree : *mut lxb_html_tree_t) 
        -> *mut lxb_html_tree_t;
    pub fn lxb_html_tree_unref(tree : *mut lxb_html_tree_t)
        -> *mut lxb_html_tree_t;
    pub fn lxb_html_tree_clean(tree : *mut lxb_html_tree_t) -> ();
    pub fn lxb_html_tree_destroy(tree : *mut lxb_html_tree_t) 
        -> *mut lxb_html_tree_t;
    pub fn lxb_html_tree_stop_parsing(tree : *mut lxb_html_tree_t)
        -> core::lxb_status_t;
    pub fn lxb_html_tree_process_abort(tree : *mut lxb_html_tree_t) -> bool;
    pub fn lxb_html_tree_parse_error(tree : *mut lxb_html_tree_t, token :
        *mut lxb_html_token_t, id : lxb_html_tree_error_id_t) -> ();
    pub fn lxb_html_tree_construction_dispatcher(tree : lxb_html_tree_t,
        token : *mut lxb_html_token_t) -> bool;
    pub fn lxb_html_tree_appropriate_place_inserting_node(tree : 
        *mut lxb_html_tree_t, override_target : *mut dom::lxb_dom_node_t, ipos :
        *mut lxb_html_tree_insertion_position_t) -> *mut dom::lxb_dom_node_t;
    pub fn lxb_html_tree_insert_foreign_element(tree : *mut lxb_html_tree_t,
        token : *mut lxb_html_token_t, _ns : ns::lxb_ns_id_t) 
        -> *mut lxb_html_element_t;
    pub fn lxb_html_tree_create_element_for_token(tree : *mut lxb_html_tree_t,
        token : *mut lxb_html_token_t, _ns : ns::lxb_ns_id_t, parent :
        *mut dom::lxb_dom_node_t) -> *mut lxb_html_element_t;
    pub fn lxb_html_tree_append_attributes(tree : *mut lxb_html_tree_t, 
        element : *mut dom::lxb_dom_element_t, token : *mut lxb_html_token_t,
        _ns : ns::lxb_ns_id_t) -> core::lxb_status_t;
    pub fn lxb_html_tree_append_attributes_from_element(tree : 
        *mut lxb_html_tree_t, element : *mut dom::lxb_dom_element_t, from :
        *mut dom::lxb_dom_element_t, _ns : ns::lxb_ns_id_t) 
        -> core::lxb_status_t;
    pub fn lxb_html_tree_adjust_mathml_attributes(tree : *mut lxb_html_tree_t,
        attr : *mut dom::lxb_dom_attr_t, ctx : *mut c_void) 
        -> core::lxb_status_t;
    pub fn lxb_html_tree_adjust_svg_attributes(tree : *mut lxb_html_tree_t,
        attr : *mut dom::lxb_dom_attr_t, ctx : *mut c_void) 
        -> core::lxb_status_t;
    pub fn lxb_html_tree_adjust_foreign_attributes(tree : *mut lxb_html_tree_t,
        attr : *mut dom::lxb_dom_attr_t, ctx : *mut c_void)
        -> core::lxb_status_t;
    pub fn lxb_html_tree_insert_character(tree : *mut lxb_html_tree_t, token :
        *mut lxb_html_token_t, ret_node : *mut *mut dom::lxb_dom_node_t)
        -> core::lxb_status_t;
    pub fn lxb_html_tree_insert_character_for_data(tree : *mut lxb_html_tree_t,
        _str : *mut core::lexbor_str_t, ret_node : 
        *mut *mut dom::lxb_dom_node_t) -> core::lxb_status_t;
    pub fn lxb_html_tree_insert_comment(tree : *mut lxb_html_tree_t, token :
        *mut lxb_html_token_t, pos : *mut dom::lxb_dom_node_t) 
        -> dom::lxb_dom_comment_t;
    pub fn lxb_html_tree_create_document_type_from_token(tree : 
        *mut lxb_html_tree_t, token : *mut lxb_html_token_t)
        -> *mut dom::lxb_dom_document_type_t;
    pub fn lxb_html_tree_node_delete_deep(tree : *mut lxb_html_tree_t, node :
        *mut dom::lxb_dom_node_t) -> ();
    pub fn lxb_html_tree_generic_rawtext_parsing(tree : *mut lxb_html_tree_t,
        token : *mut lxb_html_token_t) -> *mut lxb_html_element_t;
    pub fn lxb_html_tree_generic_rcdata_parsing(tree : *mut lxb_html_tree_t,
        token : *mut lxb_html_token_t) -> *mut lxb_html_element_t;
    pub fn lxb_html_tree_generate_implied_end_tags(tree : *mut lxb_html_tree_t,
        ex_tag : tag::lxb_tag_id_t, ex_ns : ns::lxb_ns_id_t) -> ();
    pub fn lxb_html_tree_generate_all_implied_end_tags_thoroughly(tree :
        *mut lxb_html_tree_t, ex_tag : tag::lxb_tag_id_t, ex_ns : 
        ns::lxb_ns_id_t) -> ();
    pub fn lxb_html_tree_reset_insertion_mode_appropriately(tree : 
        *mut lxb_html_tree_t) -> ();
    pub fn lxb_html_tree_element_in_scope(tree : *mut lxb_html_tree_t, tag_id :
        tag::lxb_tag_id_t, _ns : ns::lxb_ns_id_t, ct : lxb_html_tag_category_t)
        -> *mut dom::lxb_dom_node_t;
    pub fn lxb_html_tree_element_in_scope_by_node(tree : *mut lxb_html_tree_t,
        by_node : *mut dom::lxb_dom_node_t, ct : lxb_html_tag_category_t)
        -> *mut dom::lxb_dom_node_t;
    pub fn lxb_html_tree_element_in_scope_h123456(tree : *mut lxb_html_tree_t)
        -> *mut dom::lxb_dom_node_t;
    pub fn lxb_html_tree_element_in_scope_tbody_thread_tfoot(tree : 
        *mut lxb_html_tree_t) -> *mut dom::lxb_dom_node_t;
    pub fn lxb_html_tree_element_in_scope_td_th(tree : *mut lxb_html_tree_t)
        -> *mut dom::lxb_dom_node_t;
    pub fn lxb_html_tree_check_scope_element(tree : *mut lxb_html_tree_t) 
        -> bool;
    pub fn lxb_html_tree_close_p_element(tree : *mut lxb_html_tree_t, token :
        *mut lxb_html_token_t) -> ();
    pub fn lxb_html_tree_adoption_agency_algorithm(tree : *mut lxb_html_tree_t,
        token : *mut lxb_html_token_t, status : *mut core::lxb_status_t)
        -> bool;
    pub fn lxb_html_tree_html_intergation_point(node : *mut dom::lxb_dom_node_t)
        -> bool;
    pub fn lxb_html_tree_adjust_attributes_mathml(tree : *mut lxb_html_tree_t,
        attr : *mut dom::lxb_dom_attr_t, ctx : *mut c_void) 
        -> core::lxb_status_t;
    pub fn lxb_html_tree_adjust_attributes_svg(tree : *mut lxb_html_tree_t,
        attr : *mut dom::lxb_dom_attr_t, ctx : *mut c_void) 
        -> core::lxb_status_t;
    pub fn lxb_html_tree_begin_noi(tree : *mut lxb_html_tree_t, document :
        *mut lxb_html_document_t) -> core::lxb_status_t;
    pub fn lxb_html_tree_chunk_noi(tree : *mut lxb_html_tree_t, html :
        *const core::lxb_char_t, size : c_uint) -> core::lxb_status_t;
    pub fn lxb_html_tree_end_noi(tree : *mut lxb_html_tree_t)
        -> core::lxb_status_t;
    pub fn lxb_html_tree_build_noi(tree : *mut lxb_html_tree_t, document :
        *mut lxb_html_document_t, html : *const core::lxb_char_t, size :
        c_uint) -> core::lxb_status_t;
    pub fn lxb_html_tree_create_node_noi(tree : *mut lxb_html_tree_t, tag_id :
        tag::lxb_tag_id_t, _ns : ns::lxb_ns_id_t) 
        -> *mut dom::lxb_dom_node_t;
    pub fn lxb_html_tree_node_is_noi(node : *mut dom::lxb_dom_node_t, tag_id :
        tag::lxb_tag_id_t) -> bool;
    pub fn lxb_html_tree_current_node_noi(tree : *mut lxb_html_tree_t) 
        -> *mut dom::lxb_dom_node_t;
    pub fn lxb_html_tree_adjusted_current_node_noi(tree : *mut lxb_html_tree_t)
        -> *mut dom::lxb_dom_node_t;
    pub fn lxb_html_tree_insert_html_element_noi(tree : *mut lxb_html_tree_t,
        token : *mut lxb_html_token_t) -> *mut lxb_html_element_t;
    pub fn lxb_html_tree_insert_node_noi(to : *mut dom::lxb_dom_node_t, node :
        *mut dom::lxb_dom_node_t, ipos : 
        *mut lxb_html_tree_insertion_position_t) -> ();
    pub fn lxb_html_tree_acknowledge_token_self_closing_noi(tree : 
        *mut lxb_html_tree_t, token : *mut lxb_html_token_t) -> ();
    pub fn lxb_html_tree_mathml_text_intergation_point_noi(node :
        *mut dom::lxb_dom_node_t) -> bool;
    pub fn lxb_html_tree_scripting_noi(tree : *mut lxb_html_tree_t) -> bool;
    pub fn lxb_html_tree_scripting_set_noi(tree : *mut lxb_html_tree_t,
        scripting : bool) -> ();
    pub fn lxb_html_tree_attach_document_noi(tree : *mut lxb_html_tree_t, doc :
        *mut lxb_html_document_t) -> ();

    // lexbor/html/interfaces/document.h
    pub fn lxb_html_document_interface_create(document : 
        *mut lxb_html_document_t) -> *mut lxb_html_document_t;
    pub fn lxb_html_document_interface_destroy(document : 
        *mut lxb_html_document_t) -> *mut lxb_html_document_t;
    pub fn lxb_html_document_create() -> *mut lxb_html_document_t;
    pub fn lxb_html_document_clean(document : *mut lxb_html_document_t) -> ();
    pub fn lxb_html_document_destroy(document : *mut lxb_html_document_t)
        -> *mut lxb_html_document_t;
    pub fn lxb_html_document_parse(document : *mut lxb_html_document_t, html :
        *const core::lxb_char_t, size : c_uint) -> core::lxb_status_t;
    pub fn lxb_html_document_parse_chunk_begin(document : 
        *mut lxb_html_document_t) -> core::lxb_status_t;
    pub fn lxb_html_document_parse_chunk(document : *mut lxb_html_document_t,
        html : *const core::lxb_char_t, size : c_uint) -> core::lxb_status_t;
    pub fn lxb_html_document_parse_chunk_end(document : 
        *mut lxb_html_document_t) -> core::lxb_status_t;
    pub fn lxb_html_document_parse_fragment(document : *mut lxb_html_document_t,
        element : *mut dom::lxb_dom_element_t, html : *const core::lxb_char_t,
        size : c_uint) -> *mut dom::lxb_dom_node_t;
    pub fn lxb_html_document_parse_fragment_chunk_begin(document : 
        *mut lxb_html_document_t, element : *mut dom::lxb_dom_element_t)
        -> core::lxb_status_t;
    pub fn lxb_html_document_parse_fragment_chunk(document : 
        *mut lxb_html_document_t, html : *const core::lxb_char_t, size : c_uint)
        -> core::lxb_status_t;
    pub fn lxb_html_document_parse_fragment_chunk_end(document :
        *mut lxb_html_document_t) -> *mut dom::lxb_dom_node_t;
    pub fn lxb_html_document_title(document : *mut lxb_html_document_t, len :
        *mut c_uint) -> *const core::lxb_char_t;
    pub fn lxb_html_document_title_set(document : *mut lxb_html_document_t,
        title : *const core::lxb_char_t, len : c_uint) -> core::lxb_status_t;
    pub fn lxb_html_document_title_raw(document : *mut lxb_html_document_t,
        len : *mut c_uint) -> *const core::lxb_char_t;
    pub fn lxb_html_document_head_element_noi(document : 
        *mut lxb_html_document_t) -> *mut lxb_html_head_element_t;
    pub fn lxb_html_document_body_element_noi(document : 
        *mut lxb_html_document_t) -> *mut lxb_html_body_element_t;
    pub fn lxb_html_document_original_ref_noi(document : 
        *mut lxb_html_document_t) -> *mut dom::lxb_dom_document_t;
    pub fn lxb_html_document_is_original_noi(document : 
        *mut lxb_html_document_t) -> bool;
    pub fn lxb_html_document_mraw_noi(document : *mut lxb_html_document_t)
        -> *mut core::lexbor_mraw_t;
    pub fn lxb_html_document_mraw_text_noi(document : *mut lxb_html_document_t)
        -> *mut core::lexbor_mraw_t;
    pub fn lxb_html_document_opt_set_noi(document : *mut lxb_html_document_t,
        opt : lxb_html_document_opt_t) -> ();
    pub fn lxb_html_document_opt_noi(document : *mut lxb_html_document_t)
        -> lxb_html_document_opt_t;
    pub fn lxb_html_document_create_struct_noi(document : 
        *mut lxb_html_document_t, struct_size : c_uint) -> *mut c_void;
    pub fn lxb_html_document_destroy_struct_noi(document : 
        *mut lxb_html_document_t, data : *mut c_void) -> *mut c_void;
    pub fn lxb_html_document_create_element_noi(document : 
        *mut lxb_html_document_t, local_name : *const core::lxb_char_t,
        lname_len : c_uint, reserved_for_opt : *mut c_void) 
        -> *mut lxb_html_element_t;
    pub fn lxb_html_document_destroy_element_noi(element : 
        *mut dom::lxb_dom_element_t) -> *mut dom::lxb_dom_element_t;
    
    // lexbor/html/interfaces/element.h
    pub fn lxb_html_element_interface_create(document : 
        *mut lxb_html_document_t) -> *mut lxb_html_element_t;
    pub fn lxb_html_element_interface_destroy(element : 
        *mut lxb_html_document_t) -> *mut lxb_html_element_t;
    pub fn lxb_html_element_inner_html_set(element : *mut lxb_html_element_t,
        html : *const core::lxb_char_t, size : c_uint) 
        -> *mut lxb_html_element_t;   

    // lexbor/html/interfaces/anchor_element.h
    pub fn lxb_html_anchor_element_interface_create(document : 
        *mut lxb_html_document_t) -> *mut lxb_html_anchor_element_t;
    pub fn lxb_html_anchor_element_interface_destroy(anchor_element : 
        *mut lxb_html_anchor_element_t) -> *mut lxb_html_anchor_element_t;
    
    // lexbor/html/interfaces/area_element.h
    pub fn lxb_html_area_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_area_element_t;
    pub fn lxb_html_area_element_interface_destroy(area_element :
        *mut lxb_html_area_element_t) -> *mut lxb_html_area_element_t;

    // lexbor/html/interfaces/audio_element.h
    pub fn lxb_html_audio_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_audio_element_t;
    pub fn lxb_html_audio_element_interface_destroy(audio_element :
        *mut lxb_html_audio_element_t) -> *mut lxb_html_audio_element_t;
    
    // lexbor/html/interfaces/base_element.h
    pub fn lxb_html_base_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_base_element_t;
    pub fn lxb_html_base_element_interface_destroy(base_element :
        *mut lxb_html_base_element_t) -> *mut lxb_html_base_element_t;

    // lexbor/html/interfaces/body_element.h
    pub fn lxb_html_body_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_body_element_t;
    pub fn lxb_html_body_element_interface_destroy(body_element :
        *mut lxb_html_body_element_t) -> *mut lxb_html_body_element_t;
    
    // lexbor/html/interfaces/br_element.h
    pub fn lxb_html_br_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_br_element_t;
    pub fn lxb_html_br_element_interface_destroy(br_element :
        *mut lxb_html_br_element_t) -> *mut lxb_html_br_element_t;

    // lexbor/html/interfaces/button_element.h
    pub fn lxb_html_button_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_button_element_t;
    pub fn lxb_html_button_element_interface_destroy(button_element :
        *mut lxb_html_button_element_t) -> *mut lxb_html_button_element_t;

    // lexbor/html/interfaces/canvas_element.h
    pub fn lxb_html_canvas_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_canvas_element_t;
    pub fn lxb_html_canvas_element_interface_destroy(canvas_element :
        *mut lxb_html_canvas_element_t) -> *mut lxb_html_canvas_element_t;

    // lexbor/html/interfaces/d_list_element.h
    pub fn lxb_html_d_list_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_d_list_element_t;
    pub fn lxb_html_d_list_element_interface_destroy(d_list_element :
        *mut lxb_html_d_list_element_t) -> *mut lxb_html_d_list_element_t;

    // lexbor/html/interfaces/data_element.h
    pub fn lxb_html_data_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_data_element_t;
    pub fn lxb_html_data_element_interface_destroy(data_element :
        *mut lxb_html_data_element_t) -> *mut lxb_html_data_element_t;

    // lexbor/html/interfaces/data_list_element.h
    pub fn lxb_html_data_list_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_data_list_element_t;
    pub fn lxb_html_data_list_element_interface_destroy(data_list_element :
        *mut lxb_html_data_list_element_t) -> *mut lxb_html_data_list_element_t;

    // lexbor/html/interfaces/details_element.h
    pub fn lxb_html_details_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_details_element_t;
    pub fn lxb_html_details_element_interface_destroy(details_element :
        *mut lxb_html_details_element_t) -> *mut lxb_html_details_element_t;

    // lexbor/html/interfaces/dialog_element.h
    pub fn lxb_html_dialog_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_dialog_element_t;
    pub fn lxb_html_dialog_element_interface_destroy(dialog_element :
        *mut lxb_html_dialog_element_t) -> *mut lxb_html_dialog_element_t;

    // lexbor/html/interfaces/directory_element.h
    pub fn lxb_html_directory_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_directory_element_t;
    pub fn lxb_html_directory_element_interface_destroy(directory_element :
        *mut lxb_html_directory_element_t) -> *mut lxb_html_directory_element_t;

    // lexbor/html/interfaces/div_element.h
    pub fn lxb_html_div_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_div_element_t;
    pub fn lxb_html_div_element_interface_destroy(div_element :
        *mut lxb_html_div_element_t) -> *mut lxb_html_div_element_t;

    // lexbor/html/interfaces/embed_element.h
    pub fn lxb_html_embed_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_embed_element_t;
    pub fn lxb_html_embed_element_interface_destroy(embed_element :
        *mut lxb_html_embed_element_t) -> *mut lxb_html_embed_element_t;
    
    // lexbor/html/interfaces/field_set_element.h
    pub fn lxb_html_field_set_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_field_set_element_t;
    pub fn lxb_html_field_set_element_interface_destroy(field_set_element :
        *mut lxb_html_field_set_element_t) -> *mut lxb_html_field_set_element_t;

    // lexbor/html/interfaces/font_element.h
    pub fn lxb_html_font_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_font_element_t;
    pub fn lxb_html_font_element_interface_destroy(font_element :
        *mut lxb_html_font_element_t) -> *mut lxb_html_font_element_t;
    
    // lexbor/html/interfaces/form_element.h
    pub fn lxb_html_form_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_form_element_t;
    pub fn lxb_html_form_element_interface_destroy(form_element :
        *mut lxb_html_form_element_t) -> *mut lxb_html_form_element_t;    
    
    // lexbor/html/interfaces/frame_element.h
    pub fn lxb_html_frame_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_frame_element_t;
    pub fn lxb_html_frame_element_interface_destroy(frame_element :
        *mut lxb_html_frame_element_t) -> *mut lxb_html_frame_element_t;
        
    // lexbor/html/interfaces/frame_set_element.h
    pub fn lxb_html_frame_set_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_frame_set_element_t;
    pub fn lxb_html_frame_set_element_interface_destroy(frame_set_element :
        *mut lxb_html_frame_set_element_t) -> *mut lxb_html_frame_set_element_t;

    // lexbor/html/interfaces/head_element.h
    pub fn lxb_html_head_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_head_element_t;
    pub fn lxb_html_head_element_interface_destroy(head_element :
        *mut lxb_html_head_element_t) -> *mut lxb_html_head_element_t;

    // lexbor/html/interfaces/heading_element.h
    pub fn lxb_html_heading_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_heading_element_t;
    pub fn lxb_html_heading_element_interface_destroy(heading_element :
        *mut lxb_html_heading_element_t) -> *mut lxb_html_heading_element_t;

    // lexbor/html/interfaces/hr_element.h
    pub fn lxb_html_hr_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_hr_element_t;
    pub fn lxb_html_hr_element_interface_destroy(hr_element :
        *mut lxb_html_hr_element_t) -> *mut lxb_html_hr_element_t;

    // lexbor/html/interfaces/html_element.h
    pub fn lxb_html_html_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_html_element_t;
    pub fn lxb_html_html_element_interface_destroy(html_element :
        *mut lxb_html_html_element_t) -> *mut lxb_html_html_element_t;

    // lexbor/html/interfaces/iframe_element.h
    pub fn lxb_html_iframe_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_iframe_element_t;
    pub fn lxb_html_iframe_element_interface_destroy(iframe_element :
        *mut lxb_html_iframe_element_t) -> *mut lxb_html_iframe_element_t;

    // lexbor/html/interfaces/image_element.h
    pub fn lxb_html_image_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_image_element_t;
    pub fn lxb_html_image_element_interface_destroy(image_element :
        *mut lxb_html_image_element_t) -> *mut lxb_html_image_element_t;

    // lexbor/html/interfaces/input_element.h
    pub fn lxb_html_input_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_input_element_t;
    pub fn lxb_html_input_element_interface_destroy(input_element :
        *mut lxb_html_input_element_t) -> *mut lxb_html_input_element_t;

    // lexbor/html/interfaces/label_element.h
    pub fn lxb_html_label_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_label_element_t;
    pub fn lxb_html_label_element_interface_destroy(label_element :
        *mut lxb_html_label_element_t) -> *mut lxb_html_label_element_t;

    // lexbor/html/interfaces/legend_element.h
    pub fn lxb_html_legend_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_legend_element_t;
    pub fn lxb_html_legend_element_interface_destroy(legend_element :
        *mut lxb_html_legend_element_t) -> *mut lxb_html_legend_element_t;

    // lexbor/html/interfaces/li_element.h
    pub fn lxb_html_li_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_li_element_t;
    pub fn lxb_html_li_element_interface_destroy(li_element :
        *mut lxb_html_li_element_t) -> *mut lxb_html_li_element_t;

    // lexbor/html/interfaces/link_element.h
    pub fn lxb_html_link_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_link_element_t;
    pub fn lxb_html_link_element_interface_destroy(link_element :
        *mut lxb_html_link_element_t) -> *mut lxb_html_link_element_t;

    // lexbor/html/interfaces/map_element.h
    pub fn lxb_html_map_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_map_element_t;
    pub fn lxb_html_map_element_interface_destroy(map_element :
        *mut lxb_html_map_element_t) -> *mut lxb_html_map_element_t;

    // lexbor/html/interfaces/marquee_element.h
    pub fn lxb_html_marquee_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_marquee_element_t;
    pub fn lxb_html_marquee_element_interface_destroy(marquee_element :
        *mut lxb_html_marquee_element_t) -> *mut lxb_html_marquee_element_t;

    // lexbor/html/interfaces/media_element.h
    pub fn lxb_html_media_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_media_element_t;
    pub fn lxb_html_media_element_interface_destroy(media_element :
        *mut lxb_html_media_element_t) -> *mut lxb_html_media_element_t;

    // lexbor/html/interfaces/menu_element.h
    pub fn lxb_html_menu_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_menu_element_t;
    pub fn lxb_html_menu_element_interface_destroy(menu_element :
        *mut lxb_html_menu_element_t) -> *mut lxb_html_menu_element_t;

    // lexbor/html/interfaces/meta_element.h
    pub fn lxb_html_meta_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_meta_element_t;
    pub fn lxb_html_meta_element_interface_destroy(meta_element :
        *mut lxb_html_meta_element_t) -> *mut lxb_html_meta_element_t;

    // lexbor/html/interfaces/meter_element.h
    pub fn lxb_html_meter_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_meter_element_t;
    pub fn lxb_html_meter_element_interface_destroy(meter_element :
        *mut lxb_html_meter_element_t) -> *mut lxb_html_meter_element_t;

    // lexbor/html/interfaces/mod_element.h
    pub fn lxb_html_mod_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_mod_element_t;
    pub fn lxb_html_mod_element_interface_destroy(mod_element :
        *mut lxb_html_mod_element_t) -> *mut lxb_html_mod_element_t;

    // lexbor/html/interfaces/o_list_element.h
    pub fn lxb_html_o_list_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_o_list_element_t;
    pub fn lxb_html_o_list_element_interface_destroy(o_list_element :
        *mut lxb_html_o_list_element_t) -> *mut lxb_html_o_list_element_t;

    // lexbor/html/interfaces/object_element.h
    pub fn lxb_html_object_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_object_element_t;
    pub fn lxb_html_object_element_interface_destroy(object_element :
        *mut lxb_html_object_element_t) -> *mut lxb_html_object_element_t;

    // lexbor/html/interfaces/opt_group_element.h
    pub fn lxb_html_opt_group_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_opt_group_element_t;
    pub fn lxb_html_opt_group_element_interface_destroy(opt_group_element :
        *mut lxb_html_opt_group_element_t) -> *mut lxb_html_opt_group_element_t;

    // lexbor/html/interfaces/option_element.h
    pub fn lxb_html_option_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_option_element_t;
    pub fn lxb_html_option_element_interface_destroy(option_element :
        *mut lxb_html_option_element_t) -> *mut lxb_html_option_element_t;

    // lexbor/html/interfaces/output_element.h
    pub fn lxb_html_output_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_output_element_t;
    pub fn lxb_html_output_element_interface_destroy(output_element :
        *mut lxb_html_output_element_t) -> *mut lxb_html_output_element_t;    

    // lexbor/html/interfaces/paragraph_element.h
    pub fn lxb_html_paragraph_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_paragraph_element_t;
    pub fn lxb_html__element_interface_destroy(_element :
        *mut lxb_html_paragraph_element_t) -> *mut lxb_html_paragraph_element_t;    

    // lexbor/html/interfaces/param_element.h
    pub fn lxb_html_param_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_param_element_t;
    pub fn lxb_html_param_element_interface_destroy(param_element :
        *mut lxb_html_param_element_t) -> *mut lxb_html_param_element_t; 
    
    // lexbor/html/interfaces/picture_element.h
    pub fn lxb_html_picture_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_picture_element_t;
    pub fn lxb_html_picture_element_interface_destroy(picture_element :
        *mut lxb_html_picture_element_t) -> *mut lxb_html_picture_element_t;  

    // lexbor/html/interfaces/pre_element.h
    pub fn lxb_html_pre_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_pre_element_t;
    pub fn lxb_html_pre_element_interface_destroy(pre_element :
        *mut lxb_html_pre_element_t) -> *mut lxb_html_pre_element_t; 

    // lexbor/html/interfaces/progress_element.h
    pub fn lxb_html_progress_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_progress_element_t;
    pub fn lxb_html_progress_element_interface_destroy(progress_element :
        *mut lxb_html_progress_element_t) -> *mut lxb_html_progress_element_t; 

    // lexbor/html/interfaces/quote_element.h
    pub fn lxb_html_quote_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_quote_element_t;
    pub fn lxb_html_quote_element_interface_destroy(quote_element :
        *mut lxb_html_quote_element_t) -> *mut lxb_html_quote_element_t; 

    // lexbor/html/interfaces/script_element.h
    pub fn lxb_html_script_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_script_element_t;
    pub fn lxb_html_script_element_interface_destroy(script_element :
        *mut lxb_html_script_element_t) -> *mut lxb_html_script_element_t; 

    // lexbor/html/interfaces/select_element.h
    pub fn lxb_html_select_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_select_element_t;
    pub fn lxb_html_select_element_interface_destroy(select_element :
        *mut lxb_html_select_element_t) -> *mut lxb_html_select_element_t;
        
    // lexbor/html/interfaces/slot_element.h
    pub fn lxb_html_slot_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_slot_element_t;
    pub fn lxb_html_slot_element_interface_destroy(slot_element :
        *mut lxb_html_slot_element_t) -> *mut lxb_html_slot_element_t; 

    // lexbor/html/interfaces/source_element.h
    pub fn lxb_html_source_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_source_element_t;
    pub fn lxb_html_source_element_interface_destroy(source_element :
        *mut lxb_html_source_element_t) -> *mut lxb_html_source_element_t; 

    // lexbor/html/interfaces/span_element.h
    pub fn lxb_html_span_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_span_element_t;
    pub fn lxb_html_span_element_interface_destroy(span_element :
        *mut lxb_html_span_element_t) -> *mut lxb_html_span_element_t; 

    // lexbor/html/interfaces/style_element.h
    pub fn lxb_html_style_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_style_element_t;
    pub fn lxb_html_style_element_interface_destroy(style_element :
        *mut lxb_html_style_element_t) -> *mut lxb_html_style_element_t; 
    
    // lexbor/html/interfaces/table_caption_element.h
    pub fn lxb_html_table_caption_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_table_caption_element_t;
    pub fn lxb_html_table_caption_element_interface_destroy(
        table_caption_element : *mut lxb_html_table_caption_element_t) 
        -> *mut lxb_html_table_caption_element_t; 

    // lexbor/html/interfaces/table_cell_element.h
    pub fn lxb_html_table_cell_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_table_cell_element_t;
    pub fn lxb_html_table_cell_element_interface_destroy(table_cell_element :
        *mut lxb_html_table_cell_element_t) 
        -> *mut lxb_html_table_cell_element_t; 

    // lexbor/html/interfaces/table_col_element.h
    pub fn lxb_html_table_col_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_table_col_element_t;
    pub fn lxb_html_table_col_element_interface_destroy(table_col_element :
        *mut lxb_html_table_col_element_t) -> *mut lxb_html_table_col_element_t; 

    // lexbor/html/interfaces/table_element.h
    pub fn lxb_html_table_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_table_element_t;
    pub fn lxb_html_table_element_interface_destroy(table_element :
        *mut lxb_html_table_element_t) -> *mut lxb_html_table_element_t; 

    // lexbor/html/interfaces/table_row_element.h
    pub fn lxb_html_table_row_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_table_row_element_t;
    pub fn lxb_html_table_row_element_interface_destroy(table_row_element :
        *mut lxb_html_table_row_element_t) -> *mut lxb_html_table_row_element_t; 

    // lexbor/html/interfaces/table_section_element.h
    pub fn lxb_html_table_section_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_table_section_element_t;
    pub fn lxb_html_table_section_element_interface_destroy(
        table_section_element : *mut lxb_html_table_section_element_t) 
        -> *mut lxb_html_table_section_element_t; 

    // lexbor/html/interfaces/template_element.h
    pub fn lxb_html_template_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_template_element_t;
    pub fn lxb_html_template_element_interface_destroy(template_element :
        *mut lxb_html_template_element_t) -> *mut lxb_html_template_element_t; 

    // lexbor/html/interfaces/text_area_element.h
    pub fn lxb_html_text_area_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_text_area_element_t;
    pub fn lxb_html_text_area_element_interface_destroy(text_area_element :
        *mut lxb_html_text_area_element_t) -> *mut lxb_html_text_area_element_t; 

    // lexbor/html/interfaces/time_element.h
    pub fn lxb_html_time_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_time_element_t;
    pub fn lxb_html_time_element_interface_destroy(time_element :
        *mut lxb_html_time_element_t) -> *mut lxb_html_time_element_t; 

    // lexbor/html/interfaces/title_element.h
    pub fn lxb_html_title_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_title_element_t;
    pub fn lxb_html_title_element_interface_destroy(title_element :
        *mut lxb_html_title_element_t) -> *mut lxb_html_title_element_t;
        
    // lexbor/html/interfaces/track_element.h
    pub fn lxb_html_track_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_track_element_t;
    pub fn lxb_html_track_element_interface_destroy(track_element :
        *mut lxb_html_track_element_t) -> *mut lxb_html_track_element_t; 

    // lexbor/html/interfaces/u_list_element.h
    pub fn lxb_html_u_list_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_u_list_element_t;
    pub fn lxb_html_u_list_element_interface_destroy(u_list_element :
        *mut lxb_html_u_list_element_t) -> *mut lxb_html_u_list_element_t; 

    // lexbor/html/interfaces/unknown_element.h
    pub fn lxb_html_unknown_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_unknown_element_t;
    pub fn lxb_html_unknown_element_interface_destroy(unknown_element :
        *mut lxb_html_unknown_element_t) -> *mut lxb_html_unknown_element_t;
        
    // lexbor/html/interfaces/video_element.h
    pub fn lxb_html_video_element_interface_create(document :
        *mut lxb_html_document_t) -> *mut lxb_html_video_element_t;
    pub fn lxb_html_video_element_interface_destroy(video_element :
        *mut lxb_html_video_element_t) -> *mut lxb_html_video_element_t;
        
    // lexbor/html/interfaces/window.h
    pub fn lxb_html_window_create(document : *mut lxb_html_document_t)
        -> *mut lxb_html_window_t;
    pub fn lxb_html_window_destroy(window : *mut lxb_html_window_t)
        -> *mut lxb_html_window_t;

    // lexbor/html/tokenizer/error.h
    pub fn lxb_html_tokenizer_error_add(parse_errors : 
        *mut core::lexbor_array_obj_t, pos : *const core::lxb_char_t, id :
        lxb_html_tokenizer_error_id_t) -> *mut lxb_html_tokenizer_error_t;

    // lexbor/html/tokenizer/state.h
    pub fn lxb_html_tokenizer_state_data_before(tkz : *mut lxb_html_tokenizer_t,
        data : *const core::lxb_char_t, end : *const core::lxb_char_t)
        -> *const core::lxb_char_t;
    pub fn lxb_html_tokenizer_state_plaintext_before(tkz : 
        *mut lxb_html_tokenizer_t, data : *const core::lxb_char_t, end :
        *const core::lxb_char_t) -> *const core::lxb_char_t;
    pub fn lxb_html_tokenizer_state_before_attribute_name(tkz :
        *mut lxb_html_tokenizer_t, data : *const core::lxb_char_t, end :
        *const core::lxb_char_t) -> *const core::lxb_char_t;
    pub fn lxb_html_tokenizer_state_self_closing_start_tag(tkz :
        *mut lxb_html_tokenizer_t, data : *const core::lxb_char_t, end :
        *const core::lxb_char_t) -> *const core::lxb_char_t;
    pub fn lxb_html_tokenizer_state_cr(tkz : *mut lxb_html_tokenizer_t, data :
        *const core::lxb_char_t, end : *const core::lxb_char_t)
        -> *const core::lxb_char_t;
    pub fn lxb_html_tokenizer_state_char_ref(tkz : *mut lxb_html_tokenizer_t,
        data : *const core::lxb_char_t, end : *const core::lxb_char_t)
        -> *const core::lxb_char_t;

    // lexbor/html/tokenizer/state_comment.h
    pub fn lxb_html_tokenizer_state_comment_before_start(tkz :
        *mut lxb_html_tokenizer_t, data : *const core::lxb_char_t, end :
        *const core::lxb_char_t) -> *const core::lxb_char_t;
    
    // lexbor/html/tokenizer/state_doctype.h
    pub fn lxb_html_tokenizer_state_doctype_before(tkz : 
        *mut lxb_html_tokenizer_t, data : *const core::lxb_char_t, end :
        *const core::lxb_char_t) -> *const core::lxb_char_t;
    
    // lexbor/html/tokenizer/state_rawtext.h
    pub fn lxb_html_tokenizer_state_rawtext_before(tkz :
        *mut lxb_html_tokenizer_t, data : *const core::lxb_char_t, end :
        *const core::lxb_char_t) -> *const core::lxb_char_t;
    
    // lexbor/html/tokenizer/state_rcdata.h
    pub fn lxb_html_tokenizer_state_rcdata_before(tkz :
        *mut lxb_html_tokenizer_t, data : *const core::lxb_char_t, end :
        *const core::lxb_char_t) -> *const core::lxb_char_t;
    
    // lexbor/html/tokenizer/state_script.h
    pub fn lxb_html_tokenizer_state_script_data_before(tkz :
        *mut lxb_html_tokenizer_t, data : *const core::lxb_char_t, end :
        *const core::lxb_char_t) -> *const core::lxb_char_t;

    // lexbor/html/tree/active_formatting.h
    pub fn lxb_html_tree_active_formatting_maker() -> *mut lxb_html_element_t;
    pub fn lxb_html_tree_active_formatting_up_to_last_marker(tree :
        *mut lxb_html_tree_t) -> ();
    pub fn lxb_html_tree_active_formatting_remove_by_node(tree : 
        *mut lxb_html_tree_t, node : *mut dom::lxb_dom_node_t) -> ();
    pub fn lxb_html_tree_active_formatting_find_by_node(tree :
        *mut lxb_html_tree_t, node : *mut dom::lxb_dom_node_t, return_pos : 
        *mut c_uint) -> bool;
    pub fn lxb_html_tree_active_formatting_find_by_node_reverse(tree :
        *mut lxb_html_tree_t, node : *mut dom::lxb_dom_node_t, return_pos :
        *mut c_uint) -> bool;
    pub fn lxb_html_tree_active_formatting_reconstruct_elements(tree :
        *mut lxb_html_tree_t) -> core::lxb_status_t;
    pub fn lxb_html_tree_active_formatting_between_last_marker(tree :
        *mut lxb_html_tree_t, tag_idx : tag::lxb_tag_id_t, return_idx :
        *mut c_uint) -> *mut dom::lxb_dom_node_t;
    pub fn lxb_html_tree_active_formatting_push_with_check_dupl(tree :
        *mut lxb_html_tree_t, node : *mut dom::lxb_dom_node_t) -> ();

    // lexbor/html/tree/error.h
    pub fn lxb_html_tree_error_add(parse_errors : *mut core::lexbor_array_obj_t,
        token : *mut lxb_html_token_t, id : lxb_html_tree_error_id_t)
        -> *mut lxb_html_tree_error_t;
    
    // lexbor/html/tree/insertion_mode.h
    pub fn lxb_html_tree_insertion_mode_initial(tree : *mut lxb_html_tree_t,
        token : *mut lxb_html_token_t) -> bool;
    pub fn lxb_html_tree_insertion_mode_before_html(tree : *mut lxb_html_tree_t,
        token : *mut lxb_html_token_t) -> bool;
    pub fn lxb_html_tree_insertion_mode_before_head(tree : *mut lxb_html_tree_t,
        token : *mut lxb_html_token_t) -> bool;
    pub fn lxb_html_tree_insertion_mode_in_head(tree : *mut lxb_html_tree_t,
        token : *mut lxb_html_token_t) -> bool;
    pub fn lxb_html_tree_insertion_mode_in_head_noscript(tree : 
        *mut lxb_html_tree_t, token : *mut lxb_html_token_t) -> bool;
    pub fn lxb_html_tree_insertion_mode_after_head(tree : *mut lxb_html_tree_t,
        token : *mut lxb_html_token_t) -> bool;
    pub fn lxb_html_tree_insertion_mode_in_body(tree : *mut lxb_html_tree_t,
        token : *mut lxb_html_token_t) -> bool;
    pub fn lxb_html_tree_insertion_mode_in_body_skip_new_line(tree :
        *mut lxb_html_tree_t, token : *mut lxb_html_token_t) -> bool;
    pub fn lxb_html_tree_insertion_mode_in_body_skip_new_line_textarea(tree : 
        *mut lxb_html_tree_t, token : *mut lxb_html_token_t) -> bool;
    pub fn lxb_html_tree_insertion_mode_in_body_text_append(tree : 
        *mut lxb_html_tree_t, _str : core::lexbor_str_t) -> core::lxb_status_t;
    pub fn lxb_html_tree_insertion_mode_text(tree : *mut lxb_html_tree_t,
        token : *mut lxb_html_token_t) -> bool;
    pub fn lxb_html_tree_insertion_mode_in_table(tree : *mut lxb_html_tree_t,
        token : *mut lxb_html_token_t) -> bool;
    pub fn lxb_html_tree_insertion_mode_in_table_anything_else(tree : 
        *mut lxb_html_tree_t, token : *mut lxb_html_token_t) -> bool;
    pub fn lxb_html_tree_insertion_mode_in_table_text(tree : 
        *mut lxb_html_tree_t, token : *mut lxb_html_token_t) -> bool;
    pub fn lxb_html_tree_insertion_mode_in_caption(tree : *mut lxb_html_tree_t,
        token : *mut lxb_html_token_t) -> bool;
    pub fn lxb_html_tree_insertion_mode_in_column_group(tree : 
        *mut lxb_html_tree_t, token : *mut lxb_html_token_t) -> bool;
    pub fn lxb_html_tree_insertion_mode_in_table_body(tree : 
        *mut lxb_html_tree_t, token : *mut lxb_html_token_t) -> bool;
    pub fn lxb_html_tree_insertion_mode_in_row(tree : *mut lxb_html_tree_t,
        token : *mut lxb_html_token_t) -> bool;
    pub fn lxb_html_tree_insertion_mode_in_cell(tree : *mut lxb_html_tree_t,
        token : *mut lxb_html_token_t) -> bool;
    pub fn lxb_html_tree_insertion_mode_in_select(tree : *mut lxb_html_tree_t,
        token : *mut lxb_html_token_t) -> bool;
    pub fn lxb_html_tree_insertion_mode_in_select_in_table(tree : 
        *mut lxb_html_tree_t, token : *mut lxb_html_token_t) -> bool;
    pub fn lxb_html_tree_insertion_mode_in_template(tree : *mut lxb_html_tree_t,
        token : *mut lxb_html_token_t) -> bool;
    pub fn lxb_html_tree_insertion_mode_after_body(tree : *mut lxb_html_tree_t,
        token : *mut lxb_html_token_t) -> bool;
    pub fn lxb_html_tree_insertion_mode_in_frameset(tree : *mut lxb_html_tree_t,
        token : *mut lxb_html_token_t) -> bool;
    pub fn lxb_html_tree_insertion_mode_after_frameset(tree : 
        *mut lxb_html_tree_t, token : *mut lxb_html_token_t) -> bool;
    pub fn lxb_html_tree_insertion_mode_after_after_body(tree : 
        *mut lxb_html_tree_t, token : *mut lxb_html_token_t) -> bool;
    pub fn lxb_html_tree_insertion_mode_after_after_fremeset(tree : 
        *mut lxb_html_tree_t, token : *mut lxb_html_token_t) -> bool;
    pub fn lxb_html_tree_insertion_mode_foreign_content(tree : 
        *mut lxb_html_tree_t, token : *mut lxb_html_token_t) -> bool;

    // lexbor/html/tree/open_elements.h
    pub fn lxb_html_tree_open_elements_remove_by_node(tree : 
        *mut lxb_html_tree_t, node : *mut dom::lxb_dom_node_t) -> ();
    pub fn lxb_html_tree_open_elements_pop_until_tag_id(tree :
        *mut lxb_html_tree_t, tag_id : tag::lxb_tag_id_t, _ns : ns::lxb_ns_id_t,
        exclude : bool) -> ();
    pub fn lxb_html_tree_open_elements_pop_until_h123456(tree : 
        *mut lxb_html_tree_t) -> ();
    pub fn lxb_html_tree_open_elements_pop_until_td_th(tree :
        *mut lxb_html_tree_t) -> ();
    pub fn lxb_html_tree_open_elements_pop_until_node(tree : 
        *mut lxb_html_tree_t, node : *mut dom::lxb_dom_node_t, exclude : bool)
        -> ();
    pub fn lxb_html_tree_open_elements_pop_until(tree : *mut lxb_html_tree_t,
        idx : c_uint, exclude : bool) -> ();
    pub fn lxb_html_tree_open_elements_find_by_node(tree : *mut lxb_html_tree_t,
        node : dom::lxb_dom_node_t, return_node : *mut c_uint) -> bool;
    pub fn lxb_html_tree_open_elements_find_by_node_reverse(tree :
        *mut lxb_html_tree_t, node : dom::lxb_dom_node_t, return_pos :
        *mut c_uint) -> bool;
    pub fn lxb_html_tree_open_elements_find(tree : *mut lxb_html_tree_t, 
        tag_id : tag::lxb_tag_id_t, _ns : ns::lxb_ns_id_t, return_index :
        *mut c_uint) -> *mut dom::lxb_dom_node_t;
    pub fn lxb_html_tree_open_elements_find_reverse(tree : *mut lxb_html_tree_t,
        tag_id : tag::lxb_tag_id_t, _ns : ns::lxb_ns_id_t, return_index :
        *mut c_uint) -> *mut dom::lxb_dom_node_t;

    // lexbor/html/parser.h
    pub fn lxb_html_parser_create() -> *mut lxb_html_parser_t;
    pub fn lxb_html_parser_init(parser : *mut lxb_html_parser_t) 
        -> core::lxb_status_t;
    pub fn lxb_html_parser_clean(parser : *mut lxb_html_parser_t) -> ();
    pub fn lxb_html_parser_destroy(parser : *mut lxb_html_parser_t)
        -> *mut lxb_html_parser_t;
    pub fn lxb_html_parser_ref(parser : *mut lxb_html_parser_t)
        -> *mut lxb_html_parser_t;
    pub fn lxb_html_parser_unref(parser : *mut lxb_html_parser_t)
        -> *mut lxb_html_parser_t;
    pub fn lxb_html_parse(parser : *mut lxb_html_parser_t, html : 
        *const core::lxb_char_t, size : c_uint) -> *mut lxb_html_document_t;
    pub fn lxb_html_parse_fragment(parser : *mut lxb_html_parser_t, element :
        *mut lxb_html_element_t, html : *const core::lxb_char_t, size : c_uint)
        -> *mut dom::lxb_dom_node_t;
    pub fn lxb_html_parse_fragment_by_tag_id(parser : *mut lxb_html_parser_t,
        document : *mut lxb_html_document_t, tag_id : tag::lxb_tag_id_t, _ns :
        ns::lxb_ns_id_t, html : *const core::lxb_char_t, size : c_uint)
        -> *mut dom::lxb_dom_node_t;
    pub fn lxb_html_parse_chunk_begin(parser : *mut lxb_html_parser_t)
        -> *mut lxb_html_document_t;
    pub fn lxb_html_parse_chunk_process(parser : *mut lxb_html_parser_t,
        html : *const core::lxb_char_t, size : c_uint) -> core::lxb_status_t;
    pub fn lxb_html_parse_chunk_end(parser : *mut lxb_html_parser_t)
        -> core::lxb_status_t;
    pub fn lxb_html_parse_fragment_chunk_begin(parser : *mut lxb_html_parser_t,
        document : *mut lxb_html_document_t, tag_id : tag::lxb_tag_id_t, _ns :
        ns::lxb_ns_id_t) -> core::lxb_status_t;
    pub fn lxb_html_parse_fragment_chunk_process(parser : 
        *mut lxb_html_parser_t, html : *const core::lxb_char_t, size : c_uint)
        -> core::lxb_status_t;
    pub fn lxb_html_parse_fragment_chunk_end(parser : *mut lxb_html_parser_t)
        -> *mut dom::lxb_dom_node_t;
    pub fn lxb_html_parser_tokenizer_noi(parser : *mut lxb_html_parser_t)
        -> *mut lxb_html_tokenizer_t;
    pub fn lxb_html_parser_tree_noi(parser : *mut lxb_html_parser_t)
        -> *mut lxb_html_tree_t;
    pub fn lxb_html_parser_status_noi(parser : *mut lxb_html_parser_t)
        -> core::lxb_status_t;
    pub fn lxb_html_parser_state_noi(parser : *mut lxb_html_parser_t)
        -> core::lxb_status_t;
}