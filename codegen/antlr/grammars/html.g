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

html_document
    : WS? doctype? WS? html_elements*
    # DOCUMENT
    ;

html_elements
    : html_misc* html_element html_misc*
    ;

html_element
    : '<' TAG_NAME WS? (html_attribute WS?)* '>' html_content ('<' '/' | '</') WS? TAG_NAME WS? '>'
    # ELEMENT
    | '<' TAG_NAME WS? (html_attribute WS?)* '/>'
    # ELEMENT
    | '<' TAG_NAME WS? (html_attribute WS?)* '>'
    # ELEMENT
    ;

html_content
    : html_chardata? ((html_element | COMMENT) html_chardata?)*
    | SCRIPT?
    ;

html_attribute
    : TAG_NAME (WS? '=' WS? html_attribute_value)?
    # ATTRIBUTE
    ;

html_attribute_value
    : QUOTED
    | TAG_NAME
    ;

html_chardata
    : TEXT
    | WHITESPACE
    ;

html_misc
    : COMMENT
    | WHITESPACE
    ;

doctype
    : '<!' {at_keyword("DOCTYPE")}? IDENT WS? ((IDENT | QUOTED) WS?)+ '>'
    # DOCUMENT_TYPE
    ;
