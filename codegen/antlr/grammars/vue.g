grammar VUE;
import HTML from "html.g";

component
    : root_misc* component_pattern root_misc*
    # COMPONENT
    ;

component_pattern
    : script_block root_misc* template_block
    | template_block root_misc* script_block
    ;

template_block
    : template_element
    # COMPONENT_TEMPLATE
    ;

template_element
    : '<' template_tag WS? (attribute WS?)* '>' html_content ('<' '/' | '</') WS? template_tag WS? '>'
    # ELEMENT
    ;

template_tag
    : {at_keyword("template")}? TAG_NAME
    ;

script_block
    : script_element
    # COMPONENT_SCRIPT
    ;

script_element
    : '<' script_tag WS? (attribute WS?)* '>' SCRIPT_CONTENT ('<' '/' | '</') WS? script_tag WS? '>'
    # ELEMENT
    ;

script_tag
    : {at_keyword("script")}? TAG_NAME
    ;

style_block
    : style_element
    # COMPONENT_STYLE
    ;

style_element
    : '<' style_tag WS? (attribute WS?)* '>' script ('<' '/' | '</') WS? style_tag WS? '>'
    # ELEMENT
    ;

style_tag
    : {at_keyword("style")}? TAG_NAME
    ;

root_misc
    : style_block
    | COMMENT
    | WS
    ;

attribute
    : ({at_keyword("v-bind")}? TAG_NAME)? ':' attribute_key attribute_modifier* WS? '=' WS? attribute_value
    # ATTRIBUTE_BINDING
    | ({at_keyword("v-on")}? TAG_NAME | '@') attribute_key attribute_modifier* WS? '=' WS? attribute_value
    # ATTRIBUTE_LISTENER
    | TAG_NAME WS? '=' WS? attribute_value
    # ATTRIBUTE
    | TAG_NAME
    # ATTRIBUTE
    ;

attribute_modifier
    : '.' TAG_NAME
    # ATTRIBUTE_MODIFIER
    ;

attribute_key
    : '[' WS? TAG_NAME  WS? ']'
    # ATTRIBUTE_KEY
    | TAG_NAME
    # ATTRIBUTE_KEY
    ;
