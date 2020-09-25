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
use std::ffi::{CString};

/// HTML document.
pub struct Document {
    document : *mut raw::html::lxb_html_document_t,
    parser   : *mut raw::html::lxb_html_parser_t
}

#[derive(Debug, PartialEq)]
pub enum Errors {
    ParseError(String)
}

///
type Result<T> = std::result::Result<T, Errors>;

impl Document {

    /// Constructor.
    /// Create new HTMLDocument struct.
    /// 
    pub fn new() -> Document {
        Document {
            document : MaybeUninit::<raw::html::lxb_html_document_t>::uninit()
                .as_mut_ptr(),
            parser : MaybeUninit::<raw::html::lxb_html_parser_t>::uninit()
                .as_mut_ptr()
        }
    }

    /// Parse html document.
    /// 
    pub fn parse(&mut self, html : &str) -> Result<()> {
        
        // Create new document parser.
        self.parser = unsafe { raw::html::lxb_html_parser_create() };
        let status = unsafe { raw::html::lxb_html_parser_init(self.parser) };

        if status != (raw::core::lexbor_status_t::LXB_STATUS_OK as u32) {
            return Err(Errors::ParseError("Parser initialization error."
                .to_string()));
        }

        // Get html document text data.
        let doc_html = CString::new(html).unwrap();
        let doc_size = doc_html.as_bytes_with_nul().len();

        // Try to parse document.
        self.document = unsafe { raw::html::lxb_html_parse(self.parser, 
            doc_html.as_ptr() as *const u8, doc_size as u32) };
        
        if self.document.is_null() {
            return Err(Errors::ParseError("Document parse error.".to_string()));
        }
        
        // Destroy document parser.
        unsafe { raw::html::lxb_html_parser_destroy(self.parser) };
        return Ok(());
    }

    /// Inialize document to parse chunk.
    /// 
    pub fn parse_chunk_start(&mut self) -> Result<()> {

        // Create new document parser.
        self.parser = unsafe { raw::html::lxb_html_parser_create() };
        let status = unsafe { raw::html::lxb_html_parser_init(self.parser) };

        if status != (raw::core::lexbor_status_t::LXB_STATUS_OK as u32) {
            return Err(Errors::ParseError("Parser initialization error."
                .to_string()));
        }

        // Initialize parser.
        self.document = unsafe { raw::html::lxb_html_parse_chunk_begin(
            self.parser) };
        
        if self.document.is_null() {
            return Err(Errors::ParseError("Document parse error.".to_string()));
        }

        return Ok(());
    }

    /// Parse document chunk.
    /// 
    pub fn parse_chunk(&mut self, html : &str) -> Result<()> {

        // Get html document text data.
        let doc_html = CString::new(html).unwrap();
        let doc_size = doc_html.as_bytes_with_nul().len();

        let status = unsafe { raw::html::lxb_html_parse_chunk_process(
            self.parser, doc_html.as_ptr() as *const u8, doc_size as u32) };

        if status != (raw::core::lexbor_status_t::LXB_STATUS_OK as u32) {
            return Err(Errors::ParseError("Chunk parse error.".to_string()));
        }

        return Ok(());
    }

    /// Finalize parse docuemnt chunks.
    /// 
    pub fn parse_chunk_end(&mut self) -> Result<()> {

        let status = unsafe { raw::html::lxb_html_parse_chunk_end(
            self.parser) };
        
        if status != (raw::core::lexbor_status_t::LXB_STATUS_OK as u32) {
            return Err(Errors::ParseError("Document parse error."
                .to_string()));
        }

        unsafe { raw::html::lxb_html_parser_destroy(self.parser) };
        return Ok(());
    }
}
 
impl Drop for Document {
    
    /// Destructor.
    /// Clear Document and free all allocated memory.
    ///
    fn drop(&mut self) {
        if !self.document.is_null() {
            unsafe { raw::html::lxb_html_document_destroy(self.document) };
        }
    }
}