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

use librustlexbor_sys as raw;

//use std::ffi::{CStr, CString};

/// HTML document.
pub struct HTMLDocument {
    document : *mut raw::html::lxb_html_document_t,
    //status : raw::core::lxb_status_t
}

impl HTMLDocument {

    /// Constructor.
    /// Create new HTMLDocument struct.
    /// 
    pub fn new() -> HTMLDocument {
        HTMLDocument {
            document : unsafe { raw::html::lxb_html_document_create() },
            //status : 0
        }
    }



}

/// Destructor.
/// Clear HTMLDocument and delete all allocated memory.
/// 
impl Drop for HTMLDocument {
    fn drop(&mut self) {
        unsafe { raw::html::lxb_html_document_destroy(self.document) };
    }
}