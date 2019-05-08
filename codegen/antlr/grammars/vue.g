grammar VUE;
import HTML from "html.g";

component
    : component_pattern*
    # COMPONENT
    ;

component_pattern
    : component_template
    | component_script
    | component_style
    | html_misc
    ;

component_template
    : '<' template_tag WS? (attribute WS?)* '>' html_content ('<' '/' | '</') WS? template_tag WS? '>'
    # COMPONENT_TEMPLATE
    ;

component_script
    : '<' script_tag WS? (attribute WS?)* '>' script_block ('<' '/' | '</') WS? script_tag WS? '>'
    # COMPONENT_SCRIPT
    ;

component_style
    : '<' style_tag WS? (attribute WS?)* '>' style_block ('<' '/' | '</') WS? style_tag WS? '>'
    # COMPONENT_STYLE
    ;

template_tag
    : {at_keyword("template")}? TAG_NAME
    ;

script_tag
    : {at_keyword("script")}? TAG_NAME
    ;

style_tag
    : {at_keyword("style")}? TAG_NAME
    ;

html_content
    : html_chardata? ((element | MUSTACHE | COMMENT) html_chardata?)*
    | script_block?
    | style_block?
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
