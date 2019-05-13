grammar TYPESCRIPT;
import TYPESCRIPT from "javascript.g";

// N.B. ensure 'program' appears first
program
    : source_elements? EOF
    # PROGRAM
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

    // typescript declarations
    | interface_declaration
    | alias_declaration
    | enum_declaration

    // must occur after everything else to handle precedence
    // with typescript-introduced contextual keywords
    | expression_statement
    | labeled_statement
    ;

interface_declaration
    : INTERFACE_KW identifier '{' interface_property '}'
    # INTERFACE_DECLARATION
    ;

interface_property
    : identifier_or_keyword '?'? ':' type_expr
    # INTERFACE_PROPERTY
    ;

alias_declaration
    : type_kw identifier '=' type_expr eos
    # ALIAS_DECLARATION
    ;

enum_declaration
    : ENUM_KW identifier '{' (enum_variant ',')* '}'
    # ENUM_DECLARATION
    ;

enum_variant
    : identifier ('=' expression)?
    # ENUM_VARIANT
    ;

type_expr
    : type_expr type_arguments                 # GENERIC_TYPE_EXPR
    | type_expr '[' ']'                        # ARRAY_TYPE_EXPR
    | type_expr '&' type_expr                  # INTERSECTION_TYPE_EXPR
    | type_expr '|' type_expr                  # UNION_TYPE_EXPR
    | type_expr '?' type_expr ':' type_expr    # CONDITIONAL_TYPE_EXPR
    | _type_expr_interface
    | _type_expr_function
    | _type_expr_tuple
    | _type_expr_typeof
    | identifier
    | literal
    ;

_type_expr_interface
    : '{' (interface_property ';')* '}'
    # INTERFACE_TYPE_EXPR
    ;

_type_expr_function
    : '(' ')' '=>' type_expr
    # FUNCTION_TYPE_EXPR
    ;

_type_expr_tuple
    : '[' (type_expr ',')* ']'
    # TUPLE_TYPE_EXPR
    ;

_type_expr_typeof
    : TYPEOF_KW identifier
    # TYPEOF_TYPE_EXPR
    ;

type_arguments
    : '<' type_argument (',' type_argument)* '>'
    ;

type_argument
    : identifier (EXTENDS_KW identifier)?
    # TYPE_ARGUMENT
    ;

type_kw
    : {at_keyword("type")}? IDENTIFIER
    # TYPE_KW
    ;

keyof_kw
    : {at_keyword("keyof")}? IDENTIFIER
    # KEYOF_KW
    ;
