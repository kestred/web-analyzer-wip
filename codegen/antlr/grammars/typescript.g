grammar TYPESCRIPT;
import TYPESCRIPT from "javascript.g";

// N.B. ensure 'program' appears first
program
    : source_elements? EOF
    # PROGRAM
    ;

export_declaration
    : EXPORT_KW '{' export_specifier_list '}' (from_kw {at(STRING_LITERAL)}? literal)? eos
    # EXPORT_NAMED_DECLARATION
    | EXPORT_KW variable_declaration eos
    # EXPORT_NAMED_DECLARATION
    | EXPORT_KW interface_declaration
    # EXPORT_NAMED_DECLARATION
    | EXPORT_KW alias_declaration eos
    # EXPORT_NAMED_DECLARATION
    | EXPORT_KW enum_declaration
    # EXPORT_NAMED_DECLARATION
    | EXPORT_KW DEFAULT_KW class_declaration
    # EXPORT_DEFAULT_DECLARATION
    | EXPORT_KW DEFAULT_KW function_declaration
    # EXPORT_DEFAULT_DECLARATION
    | EXPORT_KW DEFAULT_KW expression eos
    # EXPORT_DEFAULT_DECLARATION
    | EXPORT_KW ASTERISK from_kw {at(STRING_LITERAL)}? literal eos
    # EXPORT_ALL_DECLARATION
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
    : INTERFACE_KW identifier '{' (interface_property eos)* '}'
    # INTERFACE_DECLARATION
    ;

interface_property
    : identifier_or_keyword '?'? ':' type_expr
    # INTERFACE_PROPERTY
    | '[' identifier ':' type_expr ']' ':' type_expr
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
    | expression {!at_line_terminator()}? '!'              # TS_NON_NULL_EXPRESSION
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
    | expression as_kw type_expr                           # TS_AS_EXPRESSION
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

type_expr
    : type_expr '.' identifier_or_keyword      # MEMBER_TYPE_EXPR
    | type_expr type_arguments                 # GENERIC_TYPE_EXPR
    | type_expr '[' ']'                        # ARRAY_TYPE_EXPR
    | type_expr '&' type_expr                  # INTERSECTION_TYPE_EXPR
    | type_expr '|' type_expr                  # UNION_TYPE_EXPR
    | type_expr '?' type_expr ':' type_expr    # CONDITIONAL_TYPE_EXPR
    | _type_expr_interface
    | _type_expr_function
    | _type_expr_tuple
    | _type_expr_typeof
    | identifier_or_primitive
    | literal
    ;

_type_expr_interface
    : '{' (interface_property eos)* '}'
    # INTERFACE_TYPE_EXPR
    ;

_type_expr_function
    : '(' ')' '=>' type_expr
    # FUNCTION_TYPE_EXPR
    ;

_type_expr_tuple
    : '[' (type_expr (',' type_expr)*)? ']'
    # TUPLE_TYPE_EXPR
    ;

_type_expr_typeof
    : TYPEOF_KW identifier
    # TYPEOF_TYPE_EXPR
    ;

type_arguments
    : '<' type_expr (',' type_expr)* '>'
    ;

object_pattern
    : '{' (assignment_property (',' assignment_property)*)? '}' (':' type_expr)?
    # OBJECT_PATTERN
    ;

array_pattern
    : '[' ','* (pattern (','+ pattern)*)? ','* ']' (':' type_expr)?
    # ARRAY_PATTERN
    ;

identifier_pattern
    : IDENTIFIER ('?'? ':' type_expr)?
    # IDENTIFIER
    ;

identifier_or_primitive
    : (IDENTIFIER | BOOLEAN_KW)
    # IDENTIFIER
    ;

function_parameters
    : '(' formal_parameter_list? ')' (':' type_expr)?
    ;

type_kw
    : {at_keyword("type")}? IDENTIFIER
    # TYPE_KW
    ;

keyof_kw
    : {at_keyword("keyof")}? IDENTIFIER
    # KEYOF_KW
    ;
