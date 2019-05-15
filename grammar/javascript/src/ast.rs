use crate::grammar;
use crate::lexer::JavascriptLexer;
use crate::syntax_kind::{self, *};
use code_grammar::{
    ast_node, AstNode, Lexer, Location, Parser,
    SyntaxElement, SyntaxError, SyntaxNode, SyntaxToken,
    TreeArc
};
use code_grammar::parser::ParseConfig;

// Re-export automatically generated nodes
pub use crate::generated::ast::*;

impl Program {
    fn new(root: &SyntaxNode) -> TreeArc<Program> {
        assert_eq!(root.kind(), PROGRAM);
        Program::cast(root).unwrap().to_owned()
    }

    pub fn parse(text: &str) -> (TreeArc<Program>, &str) {
        let tokens = JavascriptLexer::new().tokenize(text);
        let parser = Parser::new((text, &tokens).into(), ParseConfig {
            debug_repr: syntax_kind::as_debug_repr,
            max_rollback_size: 4,
            preserve_comments: false,
            preserve_whitespace: false,
        });
        let (root, remainder) = parser.parse(grammar::program);
        (Program::new(&root), remainder.text)
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
}

impl Expression {
    fn new(root: &SyntaxNode) -> TreeArc<Expression> {
        // N.B. an `Expression` can be one of many different syntax kinds
        Expression::cast(root).unwrap().to_owned()
    }

    pub fn parse(text: &str) -> (TreeArc<Expression>, &str) {
        let tokens = JavascriptLexer::new().tokenize(text);
        let parser = Parser::new((text, &tokens).into(), ParseConfig {
            debug_repr: syntax_kind::as_debug_repr,
            max_rollback_size: 4,
            preserve_comments: false,
            preserve_whitespace: false,
        });
        let (root, remainder) = parser.parse(grammar::expression);
        (Expression::new(&root), remainder.text)
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
}

impl Pattern {
    fn new(root: &SyntaxNode) -> TreeArc<Pattern> {
        // N.B. an `Pattern` can be one of many different syntax kinds
        Pattern::cast(root).unwrap().to_owned()
    }

    pub fn parse(text: &str) -> (TreeArc<Pattern>, &str) {
        let tokens = JavascriptLexer::new().tokenize(text);
        let parser = Parser::new((text, &tokens).into(), ParseConfig {
            debug_repr: syntax_kind::as_debug_repr,
            max_rollback_size: 4,
            preserve_comments: false,
            preserve_whitespace: false,
        });
        let (root, remainder) = parser.parse(grammar::pattern);
        (Pattern::new(&root), remainder.text)
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
}

impl ClassDeclaration {
    pub fn id(&self) -> &Identifier {
        self.syntax.first_child().and_then(Identifier::cast).unwrap()
    }
}

impl FunctionDeclaration {
    pub fn id(&self) -> &Identifier {
        self.syntax.first_child().and_then(Identifier::cast).unwrap()
    }
    pub fn params(&self) -> impl Iterator<Item = &Pattern> {
        self.syntax.children_with_tokens()
            .skip_while(|syn| syn.kind() != L_PAREN)
            .filter_map(|syn| match syn {
                SyntaxElement::Node(node) => Pattern::cast(node),
                SyntaxElement::Token(_) => None,
            })
    }
    pub fn body(&self) -> Option<&BlockStatement> {
        self.syntax.last_child().and_then(BlockStatement::cast)
    }
    pub fn async_(&self) -> bool {
        self.syntax.first_token().map(|tok| tok.kind() == ASYNC_KW).unwrap_or(false)
    }
    pub fn generator(&self) -> bool {
        self.syntax.children_with_tokens().any(|syn| syn.kind() == ASTERISK)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VariableDeclarationKind {
    Var,
    Let,
    Const,
}

impl VariableDeclarationKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            VariableDeclarationKind::Var => "var",
            VariableDeclarationKind::Let => "let",
            VariableDeclarationKind::Const => "const",
        }
    }
}

impl VariableDeclaration {
    pub fn declarations(&self) -> impl Iterator<Item = &VariableDeclarator> {
        self.syntax.children().filter_map(VariableDeclarator::cast)
    }
    pub fn kind(&self) -> VariableDeclarationKind {
        match self.syntax.first_token().unwrap().kind() {
            k if k == VAR_KW => VariableDeclarationKind::Var,
            k if k == LET_KW => VariableDeclarationKind::Let,
            k if k == CONST_KW => VariableDeclarationKind::Const,
            _ => unreachable!(),
        }
    }
}

impl VariableDeclarator {
    pub fn id(&self) -> Option<&Pattern> {
        self.syntax.first_child().and_then(Pattern::cast)
    }
    pub fn init(&self) -> Option<&Expression> {
        self.syntax.last_child().and_then(Expression::cast)
    }
}

impl BlockStatement {
    pub fn body(&self) -> impl Iterator<Item = &Statement> {
        self.syntax.children().filter_map(Statement::cast)
    }
}

impl ExpressionStatement {
    pub fn expression(&self) -> Option<&Expression> {
        self.syntax.first_child().and_then(Expression::cast)
    }
}

impl ReturnStatement {
    pub fn argument(&self) -> Option<&Expression> {
        self.syntax.first_child().and_then(Expression::cast)
    }
}

impl ThrowStatement {
    pub fn argument(&self) -> Option<&Expression> {
        self.syntax.first_child().and_then(Expression::cast)
    }
}

impl LabeledStatement {
    pub fn body(&self) -> Option<&Statement> {
        self.syntax.first_child().and_then(Statement::cast)
    }
}

impl ArrayExpression {
    pub fn elements(&self) -> impl Iterator<Item = &Expression> {
        self.syntax.children().filter_map(Expression::cast)
    }
}

impl ObjectExpression {
    pub fn properties(&self) -> impl Iterator<Item = &Property> {
        self.syntax.children().filter_map(Property::cast)
    }
}

impl SequenceExpression {
    pub fn expressions(&self) -> impl Iterator<Item = &Expression> {
        self.syntax.children().filter_map(Expression::cast)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum UnaryOperator {
    Negative,
    Positive,
    LogicalNegation,
    BitwiseNegation,
    TypeOf,
    Void,
    Delete,
}

impl UnaryOperator {
    pub fn as_str(&self) -> &'static str {
        match self {
            UnaryOperator::Negative => "-",
            UnaryOperator::Positive => "-",
            UnaryOperator::LogicalNegation => "!",
            UnaryOperator::BitwiseNegation => "~",
            UnaryOperator::TypeOf => "typeof",
            UnaryOperator::Void => "void",
            UnaryOperator::Delete => "delete",
        }
    }
}

impl UnaryExpression {
    pub fn operator(&self) -> UnaryOperator {
        match self.syntax.first_token().unwrap().kind() {
            k if k == MINUS => UnaryOperator::Negative,
            k if k == PLUS => UnaryOperator::Positive,
            k if k == BANG => UnaryOperator::LogicalNegation,
            k if k == TILDE => UnaryOperator::BitwiseNegation,
            k if k == TYPEOF_KW => UnaryOperator::TypeOf,
            k if k == VOID_KW => UnaryOperator::Void,
            k if k == DELETE_KW => UnaryOperator::Delete,
            _ => unreachable!(),
        }
    }
    pub fn prefix(&self) -> bool {
        true
    }
    pub fn argument(&self) -> Option<&Expression> {
        self.syntax.last_child().and_then(Expression::cast)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum UpdateOperator {
    Increment,
    Decrement,
}

impl UpdateOperator {
    pub fn as_str(&self) -> &'static str {
        match self {
            UpdateOperator::Increment => "++",
            UpdateOperator::Decrement => "--",
        }
    }
}

impl UpdateExpression {
    pub fn operator(&self) -> UpdateOperator {
        let tok = if self.prefix() {
            self.syntax.first_token()
        } else {
            self.syntax.last_token()
        };
        match tok.unwrap().kind() {
            k if k == INCREMENT => UpdateOperator::Increment,
            k if k == DECREMENT => UpdateOperator::Decrement,
            _ => unreachable!(),
        }
    }
    pub fn argument(&self) -> Option<&Expression> {
        self.syntax.children().find_map(Expression::cast)
    }
    pub fn prefix(&self) -> bool {
        match self.syntax.first_child_or_token().unwrap() {
            SyntaxElement::Node(_) => false,
            SyntaxElement::Token(_) => true,
        }
    }
}

impl BinaryExpression {
    // TODO: Implement binary operator
    // pub fn operator(&self) -> BinaryOperator;
    pub fn left(&self) -> Option<&Expression> {
        self.syntax.first_child().and_then(Expression::cast)
    }
    pub fn right(&self) -> Option<&Expression> {
        self.syntax.last_child().and_then(Expression::cast)
    }
}

impl LogicalExpression {
    // TODO: Implement logical operator
    // pub fn operator(&self) -> LogicalOperator;
    pub fn left(&self) -> Option<&Expression> {
        self.syntax.first_child().and_then(Expression::cast)
    }
    pub fn right(&self) -> Option<&Expression> {
        self.syntax.last_child().and_then(Expression::cast)
    }
}

impl AssignmentExpression {
    // TODO: Implement assignment operator
    // pub fn operator(&self) -> AssignmentOperator;
    pub fn left(&self) -> Option<&Pattern> {
        // TODO: Implement Patterns during parsing
        /*
            ObjectPattern = OBJECT_PATTERN,
            ArrayPattern = ARRAY_PATTERN,
            RestElement = REST_ELEMENT,
            AssignmentPattern = ASSIGNMENT_PATTERN,
        */
        self.syntax.first_child().and_then(Pattern::cast)
    }
    pub fn right(&self) -> Option<&Expression> {
        self.syntax.last_child().and_then(Expression::cast)
    }
}

impl MemberExpression {
    pub fn object(&self) -> Option<&Expression> {
        self.syntax.first_child().and_then(Expression::cast)
    }
    pub fn property(&self) -> Option<&Expression> {
        self.syntax.last_child().and_then(Expression::cast)
    }
    pub fn computed(&self) -> bool {
        self.syntax.children_with_tokens().any(|syn| syn.kind() == L_SQUARE)
    }
}

impl ConditionalExpression {
    pub fn test(&self) -> Option<&Expression> {
        self.syntax.first_child().and_then(Expression::cast)
    }
    pub fn alternate(&self) -> Option<&Expression> {
        self.syntax.children().nth(1).and_then(Expression::cast)
        /*
        self.syntax
            .children_with_tokens()
            .skip_while(|syn| syn.kind() != QUESTION)
            .skip(1) // eat `QUESTION`
            .next()
            .and_then(|syn| match syn {
                SyntaxElement::Node(node) => Expression::cast(node),
                SyntaxElement::Token(_) => None,
            })
        */
    }
    pub fn consequent(&self) -> Option<&Expression> {
        self.syntax.children().nth(2).and_then(Expression::cast)
        /*
        self.syntax
            .children_with_tokens()
            .skip_while(|syn| syn.kind() != COLON)
            .skip(1) // eat `COLON`
            .next()
            .and_then(|syn| match syn {
                SyntaxElement::Node(node) => Expression::cast(node),
                SyntaxElement::Token(_) => None,
            })
        */
    }
}

impl CallExpression {
    pub fn callee(&self) -> Option<&Expression> {
        self.syntax.first_child().and_then(Expression::cast)
    }
    pub fn arguments(&self) -> impl Iterator<Item = &Expression> {
        self.syntax.children().skip(1).filter_map(Expression::cast)
        /*
        self.syntax
            .children_with_tokens()
            .skip_while(|syn| syn.kind() != L_PAREN)
            .skip(1) // eat `L_PAREN`
            .filter_map(|syn| match syn {
                SyntaxElement::Node(node) => Expression::cast(node),
                SyntaxElement::Token(_) => None,
            })
        */
    }
}

impl NewExpression {
    pub fn callee(&self) -> Option<&Expression> {
        self.syntax.first_child().and_then(Expression::cast)
    }
    pub fn arguments(&self) -> impl Iterator<Item = &Expression> {
        self.syntax.children().skip(1).filter_map(Expression::cast)
        /*
        self.syntax
            .children_with_tokens()
            .skip_while(|syn| syn.kind() != L_PAREN)
            .skip(1) // eat `L_PAREN`
            .filter_map(|syn| match syn {
                SyntaxElement::Node(node) => Expression::cast(node),
                SyntaxElement::Token(_) => None,
            })
        */
    }
}

impl YieldExpression {
    pub fn argument(&self) -> Option<&Expression> {
        self.syntax.first_child().and_then(Expression::cast)
    }
    pub fn delegate(&self) -> bool {
        self.syntax.children_with_tokens().any(|tok| tok.kind() == ASTERISK)
    }
}

impl AwaitExpression {
    pub fn argument(&self) -> Option<&Expression> {
        self.syntax.first_child().and_then(Expression::cast)
    }
}

impl TaggedTemplateExpression {
    pub fn tag(&self) -> Option<&Expression> {
        self.syntax.first_child().and_then(Expression::cast)
    }
    // pub fn quasi(&self) -> TemplateLiteral;
}

impl FunctionExpression {
    pub fn id(&self) -> Option<&Identifier> {
        self.syntax.first_child().and_then(Identifier::cast)
    }
    pub fn params(&self) -> impl Iterator<Item = &Pattern> {
        self.syntax.children_with_tokens()
            .skip_while(|syn| syn.kind() != L_PAREN)
            .filter_map(|syn| match syn {
                SyntaxElement::Node(node) => Pattern::cast(node),
                SyntaxElement::Token(_) => None,
            })
    }
    pub fn body(&self) -> Option<&BlockStatement> {
        self.syntax.last_child().and_then(BlockStatement::cast)
    }
    pub fn async_(&self) -> bool {
        self.syntax.first_token().map(|tok| tok.kind() == ASYNC_KW).unwrap_or(false)
    }
    pub fn generator(&self) -> bool {
        self.syntax.children_with_tokens().any(|syn| syn.kind() == ASTERISK)
    }
}

pub enum ArrowFunctionBody<'a> {
    FunctionBody(&'a BlockStatement),
    Expression(&'a Expression),
}

impl ArrowFunctionExpression {
    pub fn id(&self) -> Option<&Identifier> {
        None // implements the estree spec's `Function` interface
    }
    pub fn params(&self) -> impl Iterator<Item = &Pattern> {
        self.syntax.children().filter_map(Pattern::cast)
        /*
        if self.syntax.first_token().map(|k| k.kind()) == Some(L_PAREN) {
            let iter = self.syntax
                .children_with_tokens()
                .take_while(|syn| syn.kind() != R_PAREN)
                .filter_map(|syn| match syn {
                    SyntaxElement::Node(node) => Pattern::cast(node),
                    SyntaxElement::Token(_) => None,
                });
            Box::new(iter) as Box<dyn Iterator<Item = &Pattern>>
        } else {
            let iter = self.syntax.first_child().and_then(Pattern::cast).into_iter();
            Box::new(iter) as Box<dyn Iterator<Item = &Pattern>>
        }
        */
    }
    pub fn body(&self) -> Option<ArrowFunctionBody> {
        self.syntax.last_child().and_then(|node| {
            if node.kind() == BLOCK_STATEMENT {
                BlockStatement::cast(node).map(ArrowFunctionBody::FunctionBody)
            } else {
                Expression::cast(node).map(ArrowFunctionBody::Expression)
            }
        })
        /*
        self.syntax
            .children_with_tokens()
            .skip_while(|syn| syn.kind() != FAT_ARROW)
            .skip(1) // eat `FAT_ARROW`
            .next()
            .and_then(|syn| match {
                SyntaxElement::Node(node) => {
                    if node.kind() == BLOCK_STATEMENT {
                        BlockStatement::cast(node).map(ArrowFunctionBody::FunctionBody)
                    } else {
                        Expression::cast(node).map(ArrowFunctionBody::Expression)
                    }
                }
                _ => None
            })
        */
    }
    pub fn expression(&self) -> bool {
        self.syntax.last_child().map(|k| k.kind() != BLOCK_STATEMENT).unwrap_or(false)
    }
    pub fn async_(&self) -> bool {
        self.syntax.first_token().map(|tok| tok.kind() == ASYNC_KW).unwrap_or(false)
    }
    pub fn generator(&self) -> bool {
        false // implements the estree spec's `Function` interface
    }
}

impl Property {
    pub fn key(&self) -> Option<&Expression> {
        self.syntax.first_child().and_then(Expression::cast)
    }

    pub fn value(&self) -> Option<&Expression> {
        self.syntax.last_child().and_then(Expression::cast)
    }

    pub fn computed(&self) -> bool {
        self.syntax.first_child_or_token().map(|t| t.kind()) == Some(L_SQUARE)
    }

    pub fn shorthand(&self) -> bool {
        self.syntax.children().count() == 1
    }
}

impl ArrayPattern {
    pub fn elements(&self) -> impl Iterator<Item = Option<&Pattern>> {
        let mut elements = Vec::new();
        let mut iter = self.syntax.children_with_tokens();
        while let Some(el) = iter.next() {
            match el {
                SyntaxElement::Node(node) => {
                    elements.push(Pattern::cast(node));
                    if let Some(bump) = iter.next() {
                        if bump.kind() == COMMA {
                            continue;
                        }
                    }
                    break;
                }
                SyntaxElement::Token(tok) => {
                    if tok.kind() == COMMA {
                        elements.push(None);
                    } else {
                        break;
                    }
                }
            }
        }
        elements.into_iter()
    }
}

impl ObjectPattern {
    pub fn properties(&self) -> impl Iterator<Item = &AssignmentProperty> {
        self.syntax.children().filter_map(AssignmentProperty::cast)
    }
}

ast_node!(AssignmentProperty, PROPERTY);
impl AssignmentProperty {
    pub fn key(&self) -> Option<&Expression> {
        self.syntax.first_child().and_then(Expression::cast)
    }

    pub fn value(&self) -> Option<&Pattern> {
        self.syntax.last_child().and_then(Pattern::cast)
    }
}

impl Identifier {
    pub fn name(&self) -> &str {
        // N.B. Token may be an `IDENTIFIER` but it may also be a `*_KW` token
        self.syntax.first_token().unwrap().text().as_str()
    }
}

pub enum LiteralKind<'a> {
    Null(SyntaxToken<'a>),
    Boolean(SyntaxToken<'a>),
    Number(SyntaxToken<'a>),
    String(SyntaxToken<'a>),
    Regexp(SyntaxToken<'a>),
    Template(SyntaxToken<'a>),
}

impl Literal {
    pub fn kind(&self) -> LiteralKind {
        let tok = self.syntax.first_token().unwrap();
        match tok.kind() {
            k if k == NULL_KW => LiteralKind::Null(tok),
            k if k == TRUE_KW => LiteralKind::Boolean(tok),
            k if k == FALSE_KW => LiteralKind::Boolean(tok),
            k if k == NUMBER_LITERAL => LiteralKind::Number(tok),
            k if k == STRING_LITERAL => LiteralKind::String(tok),
            k if k == REGEXP_LITERAL => LiteralKind::Regexp(tok),
            k if k == TEMPLATE_LITERAL => LiteralKind::Template(tok),
            _ => unreachable!(),
        }
    }
}
