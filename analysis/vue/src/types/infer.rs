use crate::types::{InterfaceTy, PropertyTy, Ty, TypeOf};
use grammar_utils::{AstNode, SmolStr};
use javascript_grammar::ast;
use javascript_grammar::syntax_kind::*;

pub(crate) fn infer_expression_type(expr: &ast::Expression) -> Ty {
    match expr.kind() {
        ast::ExpressionKind::Identifier(node) => {
            match node.syntax.first_token().map(|t| t.text()) {
                // N.B. strangely, `undefined` is considered neither a keyword nor a reserved word ¯\_(ツ)_/¯
                Some(raw) if raw == "undefined" => Ty::Hint(TypeOf::Undefined),

                // TODO: Lookup identifier declaration and perform flow typing up to this point
                _ => Ty::Any,
            }
        }
        ast::ExpressionKind::Literal(node) => infer_literal_type(node),
        ast::ExpressionKind::ThisExpression(node) => Ty::Any, // TODO: Lookup `self` type (maybe leave inference placeholder?)
        ast::ExpressionKind::ArrayExpression(node) => infer_array_expression_type(node),
        ast::ExpressionKind::ObjectExpression(node) => infer_object_expression_type(node),
        ast::ExpressionKind::FunctionExpression(node) => Ty::Hint(TypeOf::Function),
        ast::ExpressionKind::UnaryExpression(node) => infer_unary_expression_type(node),
        ast::ExpressionKind::UpdateExpression(node) => Ty::Number,
        ast::ExpressionKind::BinaryExpression(node) => Ty::Any, // TODO: Implement
        ast::ExpressionKind::AssignmentExpression(node) => Ty::Any, // TODO: Implement
        ast::ExpressionKind::LogicalExpression(node) => Ty::Any, // TODO: Implement
        ast::ExpressionKind::MemberExpression(node) => Ty::Any, // TODO: Implement
        ast::ExpressionKind::ConditionalExpression(node) => Ty::Any, // TODO: Implement
        ast::ExpressionKind::CallExpression(node) => Ty::Any, // TODO: Implement
        ast::ExpressionKind::NewExpression(node) => Ty::Hint(TypeOf::Object),
        ast::ExpressionKind::SequenceExpression(node) => node.expressions().last().map(infer_expression_type).unwrap_or(Ty::Unknown),
        ast::ExpressionKind::ArrowFunctionExpression(node) => Ty::Hint(TypeOf::Function),
        ast::ExpressionKind::YieldExpression(node) => Ty::Any, // TODO: Implement
        ast::ExpressionKind::TemplateLiteral(node) => Ty::String,
        ast::ExpressionKind::TaggedTemplateExpression(node) => Ty::Any, // TODO: Implement
        ast::ExpressionKind::ClassExpression(node) => Ty::Hint(TypeOf::Function),
        ast::ExpressionKind::MetaProperty(node) => Ty::Any, // TODO: Implement
        ast::ExpressionKind::AwaitExpression(node) => Ty::Any, // TODO: Implement
    }
}

pub(crate) fn infer_literal_type(expr: &ast::Literal) -> Ty {
    match expr.syntax.first_token().map(|t| t.kind()) {
        Some(NUMBER_LITERAL) => Ty::Number,
        Some(REGEXP_LITERAL) => Ty::Hint(TypeOf::Object), // TODO: Ty::Class(RegexpClassId)
        Some(STRING_LITERAL) => Ty::String,
        Some(TEMPLATE_LITERAL) => Ty::String,
        Some(FALSE_KW) => Ty::Boolean,
        Some(TRUE_KW) => Ty::Boolean,

        // N.B. Depending on the filetype or strictness level we'd like this
        //      to be `Ty::Null` instead of `Ty::Any`; but when type-checking
        //      unannotated javascript we have to be pretty lax.
        Some(NULL_KW) => Ty::Hint(TypeOf::Null),
        _ => Ty::Any,
    }
}

pub(crate) fn infer_array_expression_type(expr: &ast::ArrayExpression) -> Ty {
    Ty::Array(Ty::Any.into()) // TODO: Infer inner type
}

pub(crate) fn infer_object_expression_type(expr: &ast::ObjectExpression) -> Ty {
    let mut object = InterfaceTy::default();
    object.typeof_ = Some(vec![TypeOf::Object].into());
    for prop in expr.properties() {
        let ident = match infer_property_name(prop) {
            Some(name) => name,
            None => continue,
        };
        let value = match prop.value() {
            Some(value) => value,
            None => continue,
        };
        let type_ = infer_expression_type(value).into();
        object.properties.push(PropertyTy { ident, type_ });
    }
    Ty::from(object)
}

pub(crate) fn infer_unary_expression_type(expr: &ast::UnaryExpression) -> Ty {
    match expr.syntax.first_token().map(|t| t.kind()) {
        Some(DELETE_KW) => Ty::Boolean,
        Some(VOID_KW) => Ty::Undefined,
        Some(TYPEOF_KW) => Ty::String,
        Some(INCREMENT) => Ty::Number,
        Some(DECREMENT) => Ty::Number,
        Some(MINUS) => Ty::Number,
        Some(PLUS) => Ty::Number,
        Some(TILDE) => Ty::Number,
        Some(BANG) => Ty::Boolean,
        _ => Ty::Any,
    }
}

/// The string value of the property's key, if it not computed and is an identifier or literal
///
/// These example would return a value:
///
///     `0`, `hello`, `"goodbye"`, `false`
///
/// But these examples would not:
///
///     `0x55`, `["evaluated_literal"]`, `[1 + 2]`, `['Hello ${name}']`,
///
pub(crate) fn infer_property_name(prop: &ast::Property) -> Option<SmolStr> {
    if prop.computed() {
        return None;
    }

    let key = prop.key()?;
    match key.kind() {
        ast::ExpressionKind::Identifier(ident) => {
            let token = ident.syntax.first_token()?;
            Some(token.text().clone())
        }
        ast::ExpressionKind::Literal(lit) => {
            let lit = lit.syntax.first_token()?;
            if lit.kind() == STRING_LITERAL {
                let raw = lit.text();
                unescape::unescape(&raw[1 .. raw.len() - 1]).map(|s| s.into())
            } else if lit.kind() == NUMBER_LITERAL {
                let num: f64 = lit.text().parse().ok()?;
                Some(num.to_string().into())
            } else {
                Some(lit.text().clone())
            }
        }
        ast::ExpressionKind::FunctionExpression(func) => {
            let ident = func.syntax.children().find_map(ast::Identifier::cast)?;
            let token = ident.syntax.first_token()?;
            Some(token.text().clone())
        }
        _ => None,
    }
}
