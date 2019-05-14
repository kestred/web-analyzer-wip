grammar TYPESCRIPT;
import TYPESCRIPT from "javascript.g";

// N.B. ensure 'program' appears first
program
    : SHEBANG? source_elements? EOF
    # PROGRAM
    ;

module_declaration
    : import_declaration
    | export_declaration
    | ts_declare_declaration
    ;

export_declaration
    : EXPORT_KW '{' export_specifier_list '}' (from_kw {at(STRING_LITERAL)}? literal)? eos
    # EXPORT_NAMED_DECLARATION
    | EXPORT_KW variable_declaration eos
    # EXPORT_NAMED_DECLARATION
    | EXPORT_KW class_declaration
    # EXPORT_NAMED_DECLARATION
    | EXPORT_KW function_declaration
    # EXPORT_NAMED_DECLARATION
    | EXPORT_KW ts_interface_declaration
    # EXPORT_NAMED_DECLARATION
    | EXPORT_KW ts_alias_declaration eos
    # EXPORT_NAMED_DECLARATION
    | EXPORT_KW ts_enum_declaration
    # EXPORT_NAMED_DECLARATION
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
    | ts_interface_declaration
    | ts_alias_declaration eos
    | ts_enum_declaration

    // must occur after everything else to handle precedence
    // with typescript-introduced contextual keywords
    | expression_statement
    | labeled_statement
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
    | expression as_kw ts_type_annotation                           # TS_AS_EXPRESSION
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

object_pattern
    : '{' (assignment_property (',' assignment_property)*)? '}' (':' ts_type_annotation)?
    # OBJECT_PATTERN
    ;

array_pattern
    : '[' ','* (pattern (','+ pattern)*)? ','* ']' (':' ts_type_annotation)?
    # ARRAY_PATTERN
    ;

identifier_pattern
    : IDENTIFIER '?'? (':' ts_type_annotation)?
    # IDENTIFIER
    ;

identifier_or_primitive
    : (IDENTIFIER | BOOLEAN_KW)
    # IDENTIFIER
    ;

function_parameters
    : ts_type_parameters? '(' formal_parameter_list? ')' (':' function_return_type)?
    ;

function_return_type
    : ts_type_predicate
    | ts_type_annotation
    ;

ts_type_predicate
    : identifier is_kw ts_type_annotation
    # TYPE_PREDICATE
    ;

ts_declare_declaration
    : declare_kw ts_module_declaration
    | declare_kw ts_namespace_declaration
    ;

ts_module_declaration
    : module_kw module_path '{' ts_d_source_element* '}'
    | module_kw module_path ';'
    ;

ts_namespace_declaration
    : namespace_kw identifier '{' ts_d_source_element* '}'
    ;

ts_d_source_element
    : import_declaration
    | export_declaration
    | ts_interface_declaration
    ;

ts_interface_declaration
    : INTERFACE_KW identifier ts_type_parameters? (EXTENDS_KW ts_type_annotation)? '{' (ts_interface_property eos)* '}'
    # INTERFACE_DECLARATION
    ;

ts_interface_property
    : identifier_or_keyword ts_type_parameters? '?'? (':' ts_type_annotation | ts_method_type)?
    # INTERFACE_PROPERTY
    | '[' identifier ':' ts_type_annotation ']' ':' ts_type_annotation
    # INTERFACE_PROPERTY
    | ts_method_type
    # INTERFACE_PROPERTY
    ;

ts_method_type
    : ts_function_parameters ':' ts_type_annotation
    # FUNCTION_TYPE_EXPR
    ;

ts_alias_declaration
    : type_kw identifier '=' ts_type_annotation
    # ALIAS_DECLARATION
    ;

ts_enum_declaration
    : ENUM_KW identifier '{' (ts_enum_variant ',')* '}'
    # ENUM_DECLARATION
    ;

ts_enum_variant
    : identifier ('=' expression)?
    # ENUM_VARIANT
    ;

ts_type_parameters
    : '<' (ts_type_parameter (',' ts_type_parameter)*)? '>'
    ;

ts_type_parameter
    : identifier (EXTENDS_KW ts_type_annotation)? ('=' ts_type_annotation)?
    # TYPE_PARAMETER_DECLARATION
    ;

ts_type_annotation
    : '|' ts_type_annotation
    | ts_type_annotation '.' identifier_or_keyword                      # MEMBER_TYPE_EXPR
    | ts_type_annotation ts_type_arguments                              # GENERIC_TYPE_EXPR
    | ts_type_annotation '[' ']'                                        # ARRAY_TYPE_EXPR
    | ts_type_annotation '&' ts_type_annotation                         # INTERSECTION_TYPE_EXPR
    | ts_type_annotation '|' ts_type_annotation                         # UNION_TYPE_EXPR
    | ts_type_annotation '?' ts_type_annotation ':' ts_type_annotation  # CONDITIONAL_TYPE_EXPR
    | _ts_type_annotation_typeof
    | _ts_type_annotation_interface
    | ts_tuple_type
    | ts_function_type
    | '(' ts_type_annotation ')'
    | identifier_or_primitive
    | literal
    ;

_ts_type_annotation_typeof
    : TYPEOF_KW identifier
    # TYPEOF_TYPE_EXPR
    ;

_ts_type_annotation_interface
    : '{' (ts_interface_property eos)* '}'
    # INTERFACE_TYPE_EXPR
    ;

ts_function_type
    : ts_function_parameters '=>' ts_type_annotation
    # FUNCTION_TYPE_EXPR
    ;

ts_function_parameters
    : '(' ts_function_parameter_list? ')'
    ;

ts_function_parameter_list
    : ts_function_parameter (',' ts_function_parameter)* (',' ts_function_parameter_rest)?
    | ts_function_parameter_rest
    ;

ts_function_parameter
    : identifier ('?'? ':' ts_type_annotation)?
    # PARAMETER_PROPERTY
    ;

ts_function_parameter_rest
    : '...' identifier_pattern
    # REST_TYPE
    ;

ts_tuple_type
    : '[' (ts_type_annotation (',' ts_type_annotation)*)? ']'
    # TUPLE_TYPE_EXPR
    ;

ts_type_arguments
    : '<' ts_type_argument (',' ts_type_argument)* {split(SHR, &[R_ANGLE, R_ANGLE])}? '>'
    ;

ts_type_argument
    : ts_type_annotation
    # TYPE_PARAMETER_INSTANTIATION
    ;

type_kw
    : {at_contextual_kw("type")}? IDENTIFIER
    # TYPE_KW
    ;

declare_kw
    : {at_contextual_kw("declare")}? IDENTIFIER
    # DECLARE_KW
    ;

namespace_kw
    : {at_contextual_kw("namespace")}? IDENTIFIER
    # NAMESPACE_KW
    ;

module_kw
    : {at_contextual_kw("module")}? IDENTIFIER
    # MODULE_KW
    ;

keyof_kw
    : {at_contextual_kw("keyof")}? IDENTIFIER
    # KEYOF_KW
    ;

is_kw
    : {at_contextual_kw("is")}? IDENTIFIER
    # IS_KW
    ;
