/*
 * The MIT License (MIT)
 *
 * Copyright (c) 2014 by Bart Kiers (original author) and Alexandre Vitorelli (contributor -> ported to CSharp)
 * Copyright (c) 2017 by Ivan Kochurkin (Positive Technologies):
    added ES6 support, cleared and transformed to the universal grammar.
 * Copyright (c) 2018 by Juan Alvarez (contributor -> ported to Go)
 *
 * Permission is hereby granted, free of charge, to any person
 * obtaining a copy of this software and associated documentation
 * files (the "Software"), to deal in the Software without
 * restriction, including without limitation the rights to use,
 * copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following
 * conditions:
 *
 * The above copyright notice and this permission notice shall be
 * included in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
 * OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
 * HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
 * WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

grammar JAVASCRIPT;

program
    : source_elements? EOF
    # PROGRAM
    ;

source_element
    : statement
    | module_declaration
    ;

module_declaration
    : import_declaration
    | export_declaration
    ;

export_declaration
    : EXPORT_KW '{' export_specifier_list '}' (from_kw {at(STRING_LITERAL)}? literal)? eos
    # EXPORT_NAMED_DECLARATION
    | EXPORT_KW variable_declaration eos
    # EXPORT_NAMED_DECLARATION
    | EXPORT_KW DEFAULT_KW class_declaration eos
    # EXPORT_DEFAULT_DECLARATION
    | EXPORT_KW DEFAULT_KW function_declaration eos
    # EXPORT_DEFAULT_DECLARATION
    | EXPORT_KW DEFAULT_KW expression eos
    # EXPORT_DEFAULT_DECLARATION
    | EXPORT_KW ASTERISK from_kw {at(STRING_LITERAL)}? literal eos
    # EXPORT_ALL_DECLARATION
    ;

export_specifier_list
    : export_specifier_atom (',' export_specifier_atom)*
    ;

export_specifier_atom
    : identifier (as_kw identifier)?
    # EXPORT_SPECIFIER
    ;

import_declaration
    : IMPORT_KW import_declaration_list from_kw {at(STRING_LITERAL)}? literal eos
    # IMPORT_DECLARATION
    ;

import_declaration_list
    : import_specifier_special (',' '{' import_specifier_list '}')?
    | '{' import_specifier_list '}'
    ;

import_specifier_list
    : import_specifier_atom (',' import_specifier_atom)*
    ;

import_specifier_atom
    : identifier (as_kw identifier)?
    # IMPORT_SPECIFIER
    ;

import_specifier_special
    : identifier
    # IMPORT_DEFAULT_SPECIFIER
    | ASTERISK as_kw identifier
    # IMPORT_NAMESPACE_SPECIFIER
    ;

statement
    : block
    | empty_statement
    | if_statement
    | for_statement
    | while_statement
    | do_while_statement
    | continue_statement
    | break_statement
    | return_statement
    | with_statement
    | switch_statement
    | throw_statement
    | try_statement
    | debugger_statement
    | class_declaration
    | variable_declaration eos
    | function_declaration
    | expression_statement
    | labeled_statement
    ;

block
    : '{' statement_list? '}'
    # BLOCK_STATEMENT
    ;

statement_list
    : statement+
    ;

variable_declaration
    : variable_declaration_kind variable_declarator_list
    # VARIABLE_DECLARATION
    ;

variable_declaration_kind  // let, const - ES6
    : VAR_KW
    | LET_KW
    | CONST_KW
    ;

variable_declarator_list
    : variable_declarator (',' variable_declarator)*
    ;

variable_declarator
    : (identifier | array_expression | object_expression) ('=' expression)?  // ES6: Array & Object Matching
    # VARIABLE_DECLARATOR
    ;

empty_statement
    : ';'
    # EMPTY_STATEMENT
    ;

expression_statement
    : {!at(L_CURLY) && !at(FUNCTION_KW)}? expression_list eos
    # EXPRESSION_STATEMENT
    ;

if_statement
    : IF_KW '(' expression_list ')' statement (ELSE_KW statement)?
    # IF_STATEMENT
    ;

for_statement
    : FOR_KW '(' expression_list? ';' expression_list? ';' expression_list? ')' statement_list
    # FOR_STATEMENT

    | FOR_KW '(' variable_declaration ';' expression_list? ';' expression_list? ')' statement
    # FOR_STATEMENT

    | FOR_KW '(' expression IN_KW expression ')' statement_list
    # FOR_IN_STATEMENT

    | FOR_KW '(' variable_declaration IN_KW expression ')' statement
    # FOR_IN_STATEMENT

    | FOR_KW '(' expression of_kw expression ')' statement_list
    # FOR_OF_STATEMENT

    | FOR_KW '(' variable_declaration of_kw expression ')' statement
    # FOR_OF_STATEMENT
    ;

while_statement
    : WHILE_KW '(' expression_list ')' statement
    # WHILE_STATEMENT
    ;

do_while_statement
    : DO_KW statement WHILE_KW '(' expression_list ')' eos
    # DO_WHILE_STATEMENT
    ;

continue_statement
    : CONTINUE_KW ({!at_line_terminator()}? identifier)? eos
    # CONTINUE_STATEMENT
    ;

break_statement
    : BREAK_KW ({!at_line_terminator()}? identifier)? eos
    # BREAK_STATEMENT
    ;

return_statement
    : RETURN_KW ({!at_line_terminator()}? expression_list)? eos
    # RETURN_STATEMENT
    ;

with_statement
    : WITH_KW '(' expression_list ')' statement
    # WITH_STATEMENT
    ;

switch_statement
    : SWITCH_KW '(' expression_list ')' case_block
    # SWITCH_STATEMENT
    ;

case_block
    : '{' case_clauses? (default_clause case_clauses?)? '}'
    ;

case_clauses
    : case_clause+
    ;

case_clause
    : CASE_KW expression_list ':' statement_list?
    # SWITCH_CASE
    ;

default_clause
    : DEFAULT_KW ':' statement_list?
    # SWITCH_CASE
    ;

labeled_statement
    : identifier ':' statement
    # LABELED_STATEMENT
    ;

throw_statement
    : THROW_KW {!at_line_terminator()}? expression_list eos
    # THROW_STATEMENT
    ;

try_statement
    : TRY_KW block (catch_clause finally_clause? | finally_clause)
    # TRY_STATEMENT
    ;

catch_clause
    : CATCH_KW '(' identifier_pattern ')' block
    # CATCH_CLAUSE
    ;

finally_clause
    : FINALLY_KW block
    ;

debugger_statement
    : DEBUGGER_KW eos
    # DEBUGGER_STATEMENT
    ;

function_declaration
    : (async_kw FUNCTION_KW | FUNCTION_KW '*'?) identifier function_parameters function_body
    # FUNCTION_DECLARATION
    ;

class_declaration
    : CLASS_KW identifier class_tail
    # CLASS_DECLARATION
    ;

class_tail
    : (EXTENDS_KW expression)? class_body
    ;

class_body
    : '{' class_element* '}'
    # CLASS_BODY
    ;

class_element
    : method_definition
    | empty_statement
    ;

method_definition
    : STATIC_KW? property_name method_tail
    # METHOD_DEFINITION
    | STATIC_KW? getter getter_tail
    # METHOD_DEFINITION
    | STATIC_KW? setter setter_tail
    # METHOD_DEFINITION
    | STATIC_KW? generator_method
    # METHOD_DEFINITION
    ;

method_tail
    : function_parameters function_body
    # FUNCTION_EXPRESSION
    ;

generator_method
    : ('*' | async_kw)? identifier_or_keyword function_parameters function_body
    # FUNCTION_EXPRESSION
    ;

formal_parameter_list
    : formal_parameter (',' formal_parameter)* (',' formal_parameter_last)?
    | formal_parameter_last
    | array_expression                         // ES6: Parameter Context Matching
    # ARRAY_PATTERN
    | object_expression                        // ES6: Parameter Context Matching
    # OBJECT_PATTERN
    ;

formal_parameter
    : identifier_pattern ('=' expression)?     // ES6: Initialization
    # ASSIGNMENT_PATTERN
    ;

formal_parameter_last                          // ES6: Rest Parameter
    : '...' identifier_pattern
    # REST_ELEMENT
    ;

identifier_pattern
    : IDENTIFIER         // N.B. this overriden in `typescript.g` to capture type annotations
    # IDENTIFIER
    ;

function_parameters
    : '(' formal_parameter_list? ')'
    ;

function_body
    : '{' source_elements? '}'
    # BLOCK_STATEMENT
    ;

source_elements
    : source_element+
    ;

array_expression
    : '[' ','* element_list? ','* ']'
    # ARRAY_EXPRESSION
    ;

element_list
    : element_or_spread (','+ element_or_spread)*
    ;

element_or_spread
    : expression
    | spread_element
    ;

object_expression
    : '{' property_list? ','? '}'
    # OBJECT_EXPRESSION
    ;

property_list
    : property_or_spread (',' property_or_spread)*
    ;

property_or_spread
    : property
    | spread_element
    ;

spread_element
    : '...' expression  // ES6: Spread Operator
    # SPREAD_ELEMENT
    ;

identifier
    /* The `identifer` rule converts an IDENTIFIER token into a node.
     *
     * This rule should be used for identifiers which are non-keyword and
     * which can never have a type annotation.
     *
     *  - In rules where the identifier can be a keyword, use `identifier_or_keyword` instead
     *  - In rules that accept a type annotation, use `identifier_pattern` instead
     */

    : IDENTIFIER
    # IDENTIFIER // convert to node
    ;

property
    : property_name ':' expression
    # PROPERTY
    | '[' expression ']' ':' expression
    # PROPERTY
    | getter getter_tail
    # PROPERTY
    | setter setter_tail
    # PROPERTY
    | generator_method
    # PROPERTY
    | identifier
    # PROPERTY
    ;

property_name
    : identifier_or_keyword
    | STRING_LITERAL   # LITERAL
    | NUMBER_LITERAL   # LITERAL
    ;

getter_tail
    : '(' ')' function_body
    # FUNCTION_EXPRESSION
    ;

setter_tail
    : '(' identifier_pattern ')' function_body
    # FUNCTION_EXPRESSION
    ;

arguments
    : '(' ( expression (',' expression)* (',' last_argument)? | last_argument )? ')'
    ;

last_argument                        // ES6: Spread Operator
    : '...' identifier
    # SPREAD_ELEMENT
    ;

expression_list
    : expression (',' expression)+
    # SEQUENCE_EXPRESSION
    | expression                     // avoid wrapping in sequence
    ;

expression
    : function_expression
    | class_expression
    | expression '[' expression_list ']'                   # MEMBER_EXPRESSION
    | expression '.' identifier_or_keyword                 # MEMBER_EXPRESSION
    | expression arguments                                 # CALL_EXPRESSION
    | NEW_KW expression arguments?                         # NEW_EXPRESSION

    /* Unary Operators */
    | expression {!at_line_terminator()}? '++'             # UPDATE_EXPRESSION
    | expression {!at_line_terminator()}? '--'             # UPDATE_EXPRESSION
    | DELETE_KW expression                                 # UNARY_EXPRESSION
    | VOID_KW expression                                   # UNARY_EXPRESSION
    | TYPEOF_KW expression                                 # UNARY_EXPRESSION
    | '++' expression                                      # UPDATE_EXPRESSION
    | '--' expression                                      # UPDATE_EXPRESSION
    | '+' expression                                       # UNARY_EXPRESSION
    | '-' expression                                       # UNARY_EXPRESSION
    | '~' expression                                       # UNARY_EXPRESSION
    | '!' expression                                       # UNARY_EXPRESSION

    /* Binary Operators */
    | expression ('*' | '/' | '%') expression              # BINARY_EXPRESSION
    | expression ('+' | '-') expression                    # BINARY_EXPRESSION
    | expression ('<<' | '>>' | '>>>') expression          # BINARY_EXPRESSION
    | expression ('<' | '>' | '<=' | '>=') expression      # BINARY_EXPRESSION
    | expression INSTANCEOF_KW expression                  # BINARY_EXPRESSION
    | expression IN_KW expression                          # BINARY_EXPRESSION
    | expression ('==' | '!=' | '===' | '!==') expression  # BINARY_EXPRESSION
    | expression '&' expression                            # BINARY_EXPRESSION
    | expression '^' expression                            # BINARY_EXPRESSION
    | expression '|' expression                            # BINARY_EXPRESSION
    | expression '&&' expression                           # LOGICAL_EXPRESSION
    | expression '||' expression                           # LOGICAL_EXPRESSION

    | expression '?' expression ':' expression             # CONDITIONAL_EXPRESSION
    | expression assignment_operator expression            # ASSIGNMENT_EXPRESSION
    | expression TEMPLATE_LITERAL                          # TAGGED_TEMPLATE_EXPRESSION
    | TEMPLATE_LITERAL                                     # TEMPLATE_EXPRESSION
    | IDENTIFIER                                           # IDENTIFIER  // convert to node
    | THIS_KW                                              # THIS_EXPRESSION
    | SUPER_KW                                             # SUPER_EXPRESSION
    | AWAIT_KW expression                                  # AWAIT_EXPRESSION
    | YIELD_KW '*'? expression                             # YIELD_EXPRESSION
    | literal
    | array_expression
    | object_expression
    | arrow_function_expression
    | '(' expression_list ')'
    ;

class_expression
    : CLASS_KW identifier? class_tail
    # CLASS_EXPRESSION
    ;

function_expression
    : FUNCTION_KW identifier? function_parameters function_body
    # FUNCTION_EXPRESSION
    ;

arrow_function_expression
    : async_kw? arrow_function_parameters '=>' arrow_function_body
    # ARROW_FUNCTION_EXPRESSION
    ;

arrow_function_parameters
    : identifier_pattern
    | function_parameters
    ;

arrow_function_body
    : function_body
    | expression
    ;

assignment_operator
    : EQ
    | ASTERISK_EQ
    | SLASH_EQ
    | PERCENT_EQ
    | PLUS_EQ
    | MINUS_EQ
    | SHL_EQ
    | SHR_EQ
    | SHU_EQ
    | AMPERSAND_EQ
    | CARET_EQ
    | PIPE_EQ
    ;

literal
    : ( NULL_KW
      | TRUE_KW
      | FALSE_KW
      | STRING_LITERAL
      | TEMPLATE_LITERAL
      | REGEXP_LITERAL
      | NUMBER_LITERAL
      )
    # LITERAL
    ;

// numeric_literal
//     : DecimalLiteral
//     | HexIntegerLiteral
//     | OctalIntegerLiteral
//     | OctalIntegerLiteral2
//     | BinaryIntegerLiteral
//     ;

identifier_or_keyword
    : (IDENTIFIER | reserved_word)
    # IDENTIFIER
    ;

reserved_word
    : keyword
    | NULL_KW
    | TRUE_KW
    | FALSE_KW
    ;

keyword
    : BREAK_KW
    | DO_KW
    | INSTANCEOF_KW
    | TYPEOF_KW
    | CASE_KW
    | ELSE_KW
    | NEW_KW
    | VAR_KW
    | CATCH_KW
    | FINALLY_KW
    | RETURN_KW
    | VOID_KW
    | CONTINUE_KW
    | FOR_KW
    | SWITCH_KW
    | WHILE_KW
    | DEBUGGER_KW
    | FUNCTION_KW
    | THIS_KW
    | WITH_KW
    | DEFAULT_KW
    | IF_KW
    | THROW_KW
    | DELETE_KW
    | IN_KW
    | TRY_KW

    | CLASS_KW
    | ENUM_KW
    | EXTENDS_KW
    | SUPER_KW
    | CONST_KW
    | EXPORT_KW
    | IMPORT_KW
    | IMPLEMENTS_KW
    | LET_KW
    | PRIVATE_KW
    | PUBLIC_KW
    | INTERFACE_KW
    | PACKAGE_KW
    | PROTECTED_KW
    | STATIC_KW
    | YIELD_KW
    ;

getter
    : get_kw property_name
    ;

setter
    : set_kw property_name
    ;

as_kw
    : {at_keyword("as")}? IDENTIFIER
    # AS_KW
    ;

from_kw
    : {at_keyword("from")}? IDENTIFIER
    # FROM_KW
    ;

get_kw
    : {at_keyword("get")}? IDENTIFIER
    # GET_KW
    ;

set_kw
    : {at_keyword("set")}? IDENTIFIER
    # SET_KW
    ;

of_kw
    : {at_keyword("of")}? IDENTIFIER
    # OF_KW
    ;

async_kw
    : {at_keyword("async")}? IDENTIFIER
    # ASYNC_KW
    ;

eos
    : ';'
    | EOF
    | {at_line_terminator()}?
    | {at(R_CURLY)}?
    ;
