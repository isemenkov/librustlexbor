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
use std::{mem::MaybeUninit};
use std::ffi::{CStr, CString};

/// HTML document.
pub struct HTMLDocument {
    document : *mut raw::html::lxb_html_document_t,
    parser : *mut raw::html::lxb_html_parser_t,
    status : raw::core::lxb_status_t
}

/// HTML tag node element.
pub struct HTMLTagNode {

}

/// Start parse document from.
/// 
pub enum DocumentParseFrom {
    /// Start parse document from root.
    Root,   

    /// Start parse document from HTML tag.
    Html,   

    /// Start parse document from HEAD tag.
    Head,   
    
    /// Start parse document from BODY tag.
    Body,   
}



impl HTMLDocument {

    /// Constructor.
    /// Create new HTMLDocument struct.
    /// 
    pub fn new() -> HTMLDocument {
        let parser = unsafe { raw::html::lxb_html_parser_create() };
        
        HTMLDocument {
            document : MaybeUninit::<raw::html::lxb_html_document_t>::uninit()
                .as_mut_ptr(),
            status : unsafe { raw::html::lxb_html_parser_init(parser) },
            parser : parser,
        }
    }

    /// Parse HTML document.
    /// 
    pub fn parse<S>(&mut self, html : S) -> HTMLTagNode
        where S: Into<String> {
    
            
        HTMLTagNode {

        }
    }

}
 
impl Drop for HTMLDocument {
    
    /// Destructor.
    /// Clear HTMLDocument and delete all allocated memory.
    ///
    fn drop(&mut self) {
        unsafe { 
            raw::html::lxb_html_parser_destroy(self.parser);
            raw::html::lxb_html_document_destroy(self.document) 
        }; 
    }
}