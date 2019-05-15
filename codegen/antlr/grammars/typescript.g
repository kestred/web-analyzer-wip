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
    : EXPORT_KW '{' export_specifier_list '}' (@"from" module_path)? end_of_statement
    # EXPORT_NAMED_DECLARATION
    | EXPORT_KW variable_declaration end_of_statement
    # EXPORT_NAMED_DECLARATION
    | EXPORT_KW class_declaration
    # EXPORT_NAMED_DECLARATION
    | EXPORT_KW function_declaration
    # EXPORT_NAMED_DECLARATION
    | EXPORT_KW ts_interface_declaration
    # EXPORT_NAMED_DECLARATION
    | EXPORT_KW ts_alias_declaration end_of_statement
    # EXPORT_NAMED_DECLARATION
    | EXPORT_KW ts_enum_declaration
    # EXPORT_NAMED_DECLARATION
    | EXPORT_KW DEFAULT_KW expression end_of_statement
    # EXPORT_DEFAULT_DECLARATION
    | EXPORT_KW ASTERISK @"from" module_path end_of_statement
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
    | variable_declaration end_of_statement
    | function_declaration

    // typescript declarations
    | ts_interface_declaration
    | ts_alias_declaration end_of_statement
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
    | expression {!at_beginning_of_line()}? '++'           # UPDATE_EXPRESSION
    | expression {!at_beginning_of_line()}? '--'           # UPDATE_EXPRESSION
    | expression {!at_beginning_of_line()}? '!'            # TS_NON_NULL_EXPRESSION
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
    | expression (@'<<' | @'>>' | @'>>>') expression       # BINARY_EXPRESSION
    | expression ('<' | '>' | @'<=' | @'>=') expression    # BINARY_EXPRESSION
    | expression INSTANCEOF_KW expression                  # BINARY_EXPRESSION
    | expression IN_KW expression                          # BINARY_EXPRESSION
    | expression @"as" ts_type_annotation                  # TS_AS_EXPRESSION
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
    : ts_type_parameters? '(' formal_parameter_list? ')' (':' ts_return_type)?
    ;

ts_return_type
    : ts_type_predicate
    | ts_type_annotation
    ;

class_element
    : ts_class_property end_of_statement
    | method_definition
    | empty_statement
    ;

ts_class_property
    : ts_vis? identifier '?'? (':' ts_type_annotation)? ('=' expression)?
    # PROPERTY
    ;

method_definition
    : ts_vis? STATIC_KW? getter_head getter_tail
    # METHOD_DEFINITION
    | ts_vis? STATIC_KW? setter_head setter_tail
    # METHOD_DEFINITION
    | ts_vis? STATIC_KW? property_name method_tail
    # METHOD_DEFINITION
    | ts_vis? STATIC_KW? generator_method
    # METHOD_DEFINITION
    ;

getter_tail
    : '(' ')' (':' ts_return_type)? function_body
    # FUNCTION_EXPRESSION
    ;

setter_tail
    : '(' identifier_pattern ')' (':' ts_return_type)? function_body
    # FUNCTION_EXPRESSION
    ;

ts_vis
    : PUBLIC_KW
    | PROTECTED_KW
    | PRIVATE_KW
    ;

ts_type_predicate
    : identifier @"is" ts_type_annotation
    # TYPE_PREDICATE
    ;

ts_declare_declaration
    : @"declare" ts_module_declaration
    | @"declare" ts_namespace_declaration
    ;

ts_module_declaration
    : @"module" module_path '{' ts_d_source_element* '}'
    | @"module" module_path ';'
    ;

ts_namespace_declaration
    : @"namespace" identifier '{' ts_d_source_element* '}'
    ;

ts_d_source_element
    : import_declaration
    | export_declaration
    | ts_interface_declaration
    ;

ts_interface_declaration
    : INTERFACE_KW identifier ts_type_parameters? (EXTENDS_KW ts_type_annotation)? '{' (ts_interface_property end_of_statement)* '}'
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
    : @"type" identifier '=' ts_type_annotation
    # ALIAS_DECLARATION
    ;

ts_enum_declaration
    : ENUM_KW identifier '{' ts_enum_variant_list? '}'
    # ENUM_DECLARATION
    ;

ts_enum_variant_list
    : ts_enum_variant (',' ts_enum_variant)* ','?
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
    : '{' (ts_interface_property end_of_statement)* '}'
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
    : ts_function_parameter (',' ts_function_parameter)* (',' ts_function_parameter_rest)? ','?
    | ts_function_parameter_rest ','?
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
    : '<' ts_type_argument (',' ts_type_argument)* '>'
    ;

ts_type_argument
    : ts_type_annotation
    # TYPE_PARAMETER_INSTANTIATION
    ;
