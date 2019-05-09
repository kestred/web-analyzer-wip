use crate::grammar;
use crate::syntax_kind::{self, *};
use grammar_utils::{ast_node, AstNode, Lexer, Location, Parser, SyntaxError, SyntaxNode, TreeArc};
use grammar_utils::parser::ParseConfig;
use html_grammar::lexer::HtmlLexer;

pub use html_grammar::ast::Element;

ast_node!(Component, COMPONENT);
ast_node!(ComponentTemplate, COMPONENT_TEMPLATE);
ast_node!(ComponentScript, COMPONENT_SCRIPT);
ast_node!(ComponentStyle, COMPONENT_STYLE);
ast_node!(AttributeBinding, ATTRIBUTE_BINDING);
ast_node!(AttributeListener, ATTRIBUTE_LISTENER);
ast_node!(AttributeModifier, ATTRIBUTE_MODIFIER);
ast_node!(AttributeKey, ATTRIBUTE_KEY);

impl Component {
    fn new(root: TreeArc<SyntaxNode>) -> TreeArc<Component> {
        assert_eq!(root.kind(), COMPONENT);
        TreeArc::cast(root)
    }

    pub fn parse(text: &str) -> (TreeArc<Component>, &str) {
        let mut lexer = HtmlLexer::new();
        lexer.set_template_pattern("{{", "}}");
        let tokens = lexer.tokenize(text);
        let parser = Parser::new((text, &tokens).into(), ParseConfig {
            debug_repr: syntax_kind::as_debug_repr,
            max_rollback_size: 4,
            preserve_comments: true,
            preserve_whitespace: true,
        });
        let (root, remainder) = parser.parse(grammar::component);
        let node = Component::new(root.to_owned());
        (node, remainder.text)
    }

    pub fn errors(&self) -> Vec<SyntaxError> {
        self.syntax
            .root_data().unwrap()
            .downcast_ref::<Vec<(String, Location)>>().unwrap()
            .into_iter()
            .cloned()
            .map(|(msg, loc)| SyntaxError::new(msg, loc))
            .collect()
    }

    pub fn root_templates(&self) -> impl Iterator<Item = &ComponentTemplate> {
        self.syntax.children().filter_map(ComponentTemplate::cast)
    }

    pub fn root_scripts(&self) -> impl Iterator<Item = &ComponentScript> {
        self.syntax.children().filter_map(ComponentScript::cast)
    }

    pub fn root_styles(&self) -> impl Iterator<Item = &ComponentStyle> {
        self.syntax.children().filter_map(ComponentStyle::cast)
    }
}
