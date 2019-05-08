/*
 * The BSD License
 *
 * Copyright (c) 2013 Tom Everett
 * All rights reserved.
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. The name of the author may not be used to endorse or promote products
 *    derived from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE AUTHOR ``AS IS'' AND ANY EXPRESS OR
 * IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES
 * OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.
 * IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT, INDIRECT,
 * INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
 * NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

grammar HTML;

document
    : WS? doctype? WS? elements*
    # DOCUMENT
    ;

doctype
    : '<!' {at_keyword("DOCTYPE")}? IDENT WS? ((IDENT | QUOTED) WS?)+ '>'
    # DOCUMENT_TYPE
    ;

elements
    : html_misc* element html_misc*
    ;

element
    : element_pattern
    # ELEMENT
    ;

// N.B. make a separate rule for these to work around how limitation in how codegen can parse incorrect files
element_pattern
    : '<' TAG_NAME WS? (attribute WS?)* '>' html_content ('<' '/' | '</') WS? TAG_NAME WS? '>'
    | '<' TAG_NAME WS? (attribute WS?)* '>'
    | '<' TAG_NAME WS? (attribute WS?)* '/>'
    ;

attribute
    : TAG_NAME (WS? '=' WS? attribute_value)?
    # ATTRIBUTE
    ;

attribute_value
    : QUOTED
    | TAG_NAME
    ;

html_content
    : html_chardata? ((element | COMMENT) html_chardata?)*
    | script?
    ;

html_chardata
    : TEXT
    | WHITESPACE
    ;

html_misc
    : COMMENT
    | WHITESPACE
    ;

script
    : SCRIPT_BODY
    # SCRIPT
    ;
