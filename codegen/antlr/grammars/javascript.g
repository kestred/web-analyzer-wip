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
    : EXPORT_KW '{' export_specifier_list '}' (from_kw {at(STRING_LITERAL)}? literal)?
    # EXPORT_NAMED_DECLARATION
    | EXPORT_KW variable_declaration
    # EXPORT_NAMED_DECLARATION
    | EXPORT_KW DEFAULT_KW class_declaration
    # EXPORT_DEFAULT_DECLARATION
    | EXPORT_KW DEFAULT_KW function_declaration
    # EXPORT_DEFAULT_DECLARATION
    | EXPORT_KW DEFAULT_KW expression eos
    # EXPORT_DEFAULT_DECLARATION
    | EXPORT_KW ASTERISK from_kw {at(STRING_LITERAL)}? literal
    # EXPORT_ALL_DECLARATION
    ;

export_specifier_list
    : export_specifier_atom (',' export_specifier_atom)*
    ;

export_specifier_atom
    : IDENTIFIER (as_kw IDENTIFIER)
    # EXPORT_SPECIFIER
    ;

import_declaration
    : IMPORT_KW import_declaration_list from_kw {at(STRING_LITERAL)}? literal
    # IMPORT_DECLARATION
    ;

import_declaration_list
    : import_specifier_special
    | import_specifier_special ',' '{' import_specifier_list '}'
    | '{' import_specifier_list '}'
    ;

import_specifier_list
    : import_specifier_atom (',' import_specifier_atom)*
    ;

import_specifier_atom
    : IDENTIFIER (as_kw IDENTIFIER)
    # IMPORT_SPECIFIER
    ;

import_specifier_special
    : IDENTIFIER
    # IMPORT_DEFAULT_SPECIFIER
    | ASTERISK as_kw IDENTIFIER
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
    | variable_declaration
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
    : var_modifier variable_declaration_list eos
    # VARIABLE_DECLARATION
    ;

variable_declaration_list
    : variable_declaration_atom (',' variable_declaration_atom)*
    ;

variable_declaration_atom
    : (IDENTIFIER | array_expression | object_expression) ('=' expression)? // ES6: Array & Object Matching
    ;

empty_statement
    : ';'
    # EMPTY_STATEMENT
    ;

expression_statement
    : {!at(L_CURLY) && !at(FUNCTION_KW)}? expression_sequence eos
    # EXPRESSION_STATEMENT
    ;

if_statement
    : IF_KW '(' expression_sequence ')' statement (ELSE_KW statement)?
    # IF_STATEMENT
    ;

for_statement
    : FOR_KW '(' expression_sequence? ';' expression_sequence? ';' expression_sequence? ')' statement_list
    # FOR_STATEMENT

    | FOR_KW '(' var_modifier variable_declaration_list ';' expression_sequence? ';' expression_sequence? ')' statement
    # FOR_STATEMENT

    | FOR_KW '(' expression (IN_KW | of_kw) expression_sequence ')' statement_list
    # FOR_IN_STATEMENT

    | FOR_KW '(' var_modifier variable_declaration_atom (IN_KW | of_kw) expression_sequence ')' statement
    # FOR_IN_STATEMENT
    ;

while_statement
    : WHILE_KW '(' expression_sequence ')' statement
    # WHILE_STATEMENT
    ;

do_while_statement
    : DO_KW statement WHILE_KW '(' expression_sequence ')' eos
    # DO_WHILE_STATEMENT
    ;

var_modifier  // let, const - ES6
    : VAR_KW
    | LET_KW
    | CONST_KW
    ;

continue_statement
    : CONTINUE_KW ({!at_line_terminator()}? IDENTIFIER)? eos
    # CONTINUE_STATEMENT
    ;

break_statement
    : BREAK_KW ({!at_line_terminator()}? IDENTIFIER)? eos
    # BREAK_STATEMENT
    ;

return_statement
    : RETURN_KW ({!at_line_terminator()}? expression_sequence)? eos
    # RETURN_STATEMENT
    ;

with_statement
    : WITH_KW '(' expression_sequence ')' statement
    # WITH_STATEMENT
    ;

switch_statement
    : SWITCH_KW '(' expression_sequence ')' case_block
    # SWITCH_STATEMENT
    ;

case_block
    : '{' case_clauses? (default_clause case_clauses?)? '}'
    ;

case_clauses
    : case_clause+
    ;

case_clause
    : CASE_KW expression_sequence ':' statement_list?
    ;

default_clause
    : DEFAULT_KW ':' statement_list?
    ;

labeled_statement
    : IDENTIFIER ':' statement
    # LABELED_STATEMENT
    ;

throw_statement
    : THROW_KW {!at_line_terminator()}? expression_sequence eos
    # THROW_STATEMENT
    ;

try_statement
    : TRY_KW block (catch_production finally_production? | finally_production)
    # TRY_STATEMENT
    ;

catch_production
    : CATCH_KW '(' IDENTIFIER ')' block
    ;

finally_production
    : FINALLY_KW block
    ;

debugger_statement
    : DEBUGGER_KW eos
    # DEBUGGER_STATEMENT
    ;

function_declaration
    : FUNCTION_KW '*'? IDENTIFIER '(' formal_parameter_list? ')' '{' function_body '}'
    # FUNCTION_DECLARATION
    ;

class_declaration
    : CLASS_KW IDENTIFIER class_tail
    # CLASS_DECLARATION
    ;

class_tail
    : (EXTENDS_KW expression)? '{' class_element* '}'
    ;

class_element
    : STATIC_KW? method_definition
    | empty_statement
    ;

method_definition
    : property_name '(' formal_parameter_list? ')' '{' function_body '}'
    | getter '(' ')' '{' function_body '}'
    | setter '(' formal_parameter_list? ')' '{' function_body '}'
    | generator_method
    ;

generator_method
    : ('*' | async_kw)? identifier_name '(' formal_parameter_list? ')' '{' function_body '}'
    ;

formal_parameter_list
    : formal_parameter_arg (',' formal_parameter_arg)* (',' last_formal_parameter_arg)?
    | last_formal_parameter_arg
    | array_expression                            // ES6: Parameter Context Matching
    | object_expression                           // ES6: Parameter Context Matching
    ;

formal_parameter_arg
    : IDENTIFIER ('=' expression)?      // ES6: Initialization
    ;

last_formal_parameter_arg                      // ES6: Rest Parameter
    : '...' IDENTIFIER
    ;

function_body
    : source_elements?
    ;

source_elements
    : source_element+
    ;

array_expression
    : '[' ','* element_list? ','* ']'
    # ARRAY_EXPRESSION
    ;

element_list
    : expression (','+ expression)* (','+ last_element)?
    | last_element
    ;

last_element
    : '...' IDENTIFIER  // ES6: Spread Operator
    ;

object_expression
    : '{' (property (',' property)*)? ','? '}'
    # OBJECT_EXPRESSION
    ;

property
    : property_name (':' |'=') expression
    | '[' expression ']' ':' expression
    | getter '(' ')' '{' function_body '}'
    | setter '(' IDENTIFIER ')' '{' function_body '}'
    | generator_method
    | IDENTIFIER
    ;

property_name
    : identifier_name
    | STRING_LITERAL
    | NUMBER_LITERAL
    ;

arguments
    : '(' ( expression (',' expression)* (',' last_argument)? | last_argument )? ')'
    ;

last_argument                                  // ES6: Spread Operator
    : '...' IDENTIFIER
    ;

expression_sequence
    : expression (',' expression)*
    # SEQUENCE_EXPRESSION
    ;

expression
    : function_expression
    | class_expression
    | expression '[' expression_sequence ']'               # MEMBER_EXPRESSION
    | expression '.' identifier_name                       # MEMBER_EXPRESSION
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
    | IDENTIFIER
    | THIS_KW                                              # THIS_EXPRESSION
    | SUPER_KW                                             # SUPER_EXPRESSION
    | AWAIT_KW expression                                  # AWAIT_EXPRESSION
    | YIELD_KW '*'? expression                             # YIELD_EXPRESSION
    | literal
    | array_expression
    | object_expression
    | '(' expression_sequence ')'
    | arrow_function_expression
    ;

class_expression
    : CLASS_KW IDENTIFIER? class_tail
    # CLASS_EXPRESSION
    ;

function_expression
    : FUNCTION_KW IDENTIFIER? '(' formal_parameter_list? ')' '{' function_body '}'
    # FUNCTION_EXPRESSION
    ;

arrow_function_expression
    : arrow_function_parameters '=>' arrow_function_body
    # ARROW_FUNCTION_EXPRESSION
    ;

arrow_function_parameters
    : IDENTIFIER
    | '(' formal_parameter_list? ')'
    ;

arrow_function_body
    : expression
    | '{' function_body '}'
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

identifier_name
    : IDENTIFIER
    | reserved_word
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