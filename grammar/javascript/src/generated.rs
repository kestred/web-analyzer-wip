// This file is automatically generated by running `cargo run -p estree_codegen`.
//
// =====================
// Do not edit manually.
// =====================
//
#![allow(dead_code)]

//! This module contains an auto-generated JAVASCRIPT AST.

pub mod ast {
    use crate::syntax_kind::*;
    use code_grammar::ast_node;

    ast_node!(Node, enum NodeKind {
        CatchClause = CATCH_CLAUSE,
        Class,
        ClassBody = CLASS_BODY,
        Expression,
        Function,
        MethodDefinition = METHOD_DEFINITION,
        ModuleDeclaration,
        ModuleSpecifier,
        Pattern,
        Program = PROGRAM,
        Property = PROPERTY,
        SpreadElement = SPREAD_ELEMENT,
        Statement,
        Super = SUPER_EXPRESSION,
        SwitchCase = SWITCH_CASE,
        TemplateElement = TEMPLATE_ELEMENT,
        VariableDeclarator = VARIABLE_DECLARATOR,
    });
    impl Node {
        pub fn type_(&self) -> &'static str {
            match self.kind() {
                NodeKind::CatchClause(node) => node.type_(),
                NodeKind::Class(node) => node.type_(),
                NodeKind::ClassBody(node) => node.type_(),
                NodeKind::Expression(node) => node.type_(),
                NodeKind::Function(node) => node.type_(),
                NodeKind::MethodDefinition(node) => node.type_(),
                NodeKind::ModuleDeclaration(node) => node.type_(),
                NodeKind::ModuleSpecifier(node) => node.type_(),
                NodeKind::Pattern(node) => node.type_(),
                NodeKind::Program(node) => node.type_(),
                NodeKind::Property(node) => node.type_(),
                NodeKind::SpreadElement(node) => node.type_(),
                NodeKind::Statement(node) => node.type_(),
                NodeKind::Super(node) => node.type_(),
                NodeKind::SwitchCase(node) => node.type_(),
                NodeKind::TemplateElement(node) => node.type_(),
                NodeKind::VariableDeclarator(node) => node.type_(),
            }
        }
    }
    ast_node!(Function, enum FunctionKind {
        ArrowFunctionExpression = ARROW_FUNCTION_EXPRESSION,
        FunctionDeclaration = FUNCTION_DECLARATION,
        FunctionExpression = FUNCTION_EXPRESSION,
    });
    impl Function {
        pub fn type_(&self) -> &'static str {
            match self.kind() {
                FunctionKind::ArrowFunctionExpression(node) => node.type_(),
                FunctionKind::FunctionDeclaration(node) => node.type_(),
                FunctionKind::FunctionExpression(node) => node.type_(),
            }
        }
    }
    ast_node!(Statement, enum StatementKind {
        BlockStatement = BLOCK_STATEMENT,
        BreakStatement = BREAK_STATEMENT,
        ContinueStatement = CONTINUE_STATEMENT,
        DebuggerStatement = DEBUGGER_STATEMENT,
        Declaration,
        DoWhileStatement = DO_WHILE_STATEMENT,
        EmptyStatement = EMPTY_STATEMENT,
        ExpressionStatement = EXPRESSION_STATEMENT,
        ForInStatement = FOR_IN_STATEMENT,
        ForOfStatement = FOR_OF_STATEMENT,
        ForStatement = FOR_STATEMENT,
        IfStatement = IF_STATEMENT,
        LabeledStatement = LABELED_STATEMENT,
        ReturnStatement = RETURN_STATEMENT,
        SwitchStatement = SWITCH_STATEMENT,
        ThrowStatement = THROW_STATEMENT,
        TryStatement = TRY_STATEMENT,
        WhileStatement = WHILE_STATEMENT,
        WithStatement = WITH_STATEMENT,
    });
    impl Statement {
        pub fn type_(&self) -> &'static str {
            match self.kind() {
                StatementKind::BlockStatement(node) => node.type_(),
                StatementKind::BreakStatement(node) => node.type_(),
                StatementKind::ContinueStatement(node) => node.type_(),
                StatementKind::DebuggerStatement(node) => node.type_(),
                StatementKind::Declaration(node) => node.type_(),
                StatementKind::DoWhileStatement(node) => node.type_(),
                StatementKind::EmptyStatement(node) => node.type_(),
                StatementKind::ExpressionStatement(node) => node.type_(),
                StatementKind::ForInStatement(node) => node.type_(),
                StatementKind::ForOfStatement(node) => node.type_(),
                StatementKind::ForStatement(node) => node.type_(),
                StatementKind::IfStatement(node) => node.type_(),
                StatementKind::LabeledStatement(node) => node.type_(),
                StatementKind::ReturnStatement(node) => node.type_(),
                StatementKind::SwitchStatement(node) => node.type_(),
                StatementKind::ThrowStatement(node) => node.type_(),
                StatementKind::TryStatement(node) => node.type_(),
                StatementKind::WhileStatement(node) => node.type_(),
                StatementKind::WithStatement(node) => node.type_(),
            }
        }
    }
    ast_node!(Expression, enum ExpressionKind {
        ArrayExpression = ARRAY_EXPRESSION,
        ArrowFunctionExpression = ARROW_FUNCTION_EXPRESSION,
        AssignmentExpression = ASSIGNMENT_EXPRESSION,
        AwaitExpression = AWAIT_EXPRESSION,
        BinaryExpression = BINARY_EXPRESSION,
        CallExpression = CALL_EXPRESSION,
        ClassExpression = CLASS_EXPRESSION,
        ConditionalExpression = CONDITIONAL_EXPRESSION,
        FunctionExpression = FUNCTION_EXPRESSION,
        Identifier = IDENTIFIER,
        Literal = LITERAL,
        LogicalExpression = LOGICAL_EXPRESSION,
        MemberExpression = MEMBER_EXPRESSION,
        MetaProperty = META_PROPERTY,
        NewExpression = NEW_EXPRESSION,
        ObjectExpression = OBJECT_EXPRESSION,
        SequenceExpression = SEQUENCE_EXPRESSION,
        TSAsExpression = TS_AS_EXPRESSION,
        TSNonNullExpression = TS_NON_NULL_EXPRESSION,
        TaggedTemplateExpression = TAGGED_TEMPLATE_EXPRESSION,
        TemplateLiteral = TEMPLATE_EXPRESSION,
        ThisExpression = THIS_EXPRESSION,
        UnaryExpression = UNARY_EXPRESSION,
        UpdateExpression = UPDATE_EXPRESSION,
        YieldExpression = YIELD_EXPRESSION,
    });
    impl Expression {
        pub fn type_(&self) -> &'static str {
            match self.kind() {
                ExpressionKind::ArrayExpression(node) => node.type_(),
                ExpressionKind::ArrowFunctionExpression(node) => node.type_(),
                ExpressionKind::AssignmentExpression(node) => node.type_(),
                ExpressionKind::AwaitExpression(node) => node.type_(),
                ExpressionKind::BinaryExpression(node) => node.type_(),
                ExpressionKind::CallExpression(node) => node.type_(),
                ExpressionKind::ClassExpression(node) => node.type_(),
                ExpressionKind::ConditionalExpression(node) => node.type_(),
                ExpressionKind::FunctionExpression(node) => node.type_(),
                ExpressionKind::Identifier(node) => node.type_(),
                ExpressionKind::Literal(node) => node.type_(),
                ExpressionKind::LogicalExpression(node) => node.type_(),
                ExpressionKind::MemberExpression(node) => node.type_(),
                ExpressionKind::MetaProperty(node) => node.type_(),
                ExpressionKind::NewExpression(node) => node.type_(),
                ExpressionKind::ObjectExpression(node) => node.type_(),
                ExpressionKind::SequenceExpression(node) => node.type_(),
                ExpressionKind::TSAsExpression(node) => node.type_(),
                ExpressionKind::TSNonNullExpression(node) => node.type_(),
                ExpressionKind::TaggedTemplateExpression(node) => node.type_(),
                ExpressionKind::TemplateLiteral(node) => node.type_(),
                ExpressionKind::ThisExpression(node) => node.type_(),
                ExpressionKind::UnaryExpression(node) => node.type_(),
                ExpressionKind::UpdateExpression(node) => node.type_(),
                ExpressionKind::YieldExpression(node) => node.type_(),
            }
        }
    }
    ast_node!(Pattern, enum PatternKind {
        ArrayPattern = ARRAY_PATTERN,
        AssignmentPattern = ASSIGNMENT_PATTERN,
        Identifier = IDENTIFIER,
        MemberExpression = MEMBER_EXPRESSION,
        ObjectPattern = OBJECT_PATTERN,
        RestElement = REST_ELEMENT,
    });
    impl Pattern {
        pub fn type_(&self) -> &'static str {
            match self.kind() {
                PatternKind::ArrayPattern(node) => node.type_(),
                PatternKind::AssignmentPattern(node) => node.type_(),
                PatternKind::Identifier(node) => node.type_(),
                PatternKind::MemberExpression(node) => node.type_(),
                PatternKind::ObjectPattern(node) => node.type_(),
                PatternKind::RestElement(node) => node.type_(),
            }
        }
    }
    ast_node!(Class, enum ClassKind {
        ClassDeclaration = CLASS_DECLARATION,
        ClassExpression = CLASS_EXPRESSION,
    });
    impl Class {
        pub fn type_(&self) -> &'static str {
            match self.kind() {
                ClassKind::ClassDeclaration(node) => node.type_(),
                ClassKind::ClassExpression(node) => node.type_(),
            }
        }
    }
    ast_node!(ModuleDeclaration, enum ModuleDeclarationKind {
        ExportAllDeclaration = EXPORT_ALL_DECLARATION,
        ExportDefaultDeclaration = EXPORT_DEFAULT_DECLARATION,
        ExportNamedDeclaration = EXPORT_NAMED_DECLARATION,
        ImportDeclaration = IMPORT_DECLARATION,
    });
    impl ModuleDeclaration {
        pub fn type_(&self) -> &'static str {
            match self.kind() {
                ModuleDeclarationKind::ExportAllDeclaration(node) => node.type_(),
                ModuleDeclarationKind::ExportDefaultDeclaration(node) => node.type_(),
                ModuleDeclarationKind::ExportNamedDeclaration(node) => node.type_(),
                ModuleDeclarationKind::ImportDeclaration(node) => node.type_(),
            }
        }
    }
    ast_node!(ModuleSpecifier, enum ModuleSpecifierKind {
        ExportSpecifier = EXPORT_SPECIFIER,
        ImportDefaultSpecifier = IMPORT_DEFAULT_SPECIFIER,
        ImportNamespaceSpecifier = IMPORT_NAMESPACE_SPECIFIER,
        ImportSpecifier = IMPORT_SPECIFIER,
    });
    impl ModuleSpecifier {
        pub fn type_(&self) -> &'static str {
            match self.kind() {
                ModuleSpecifierKind::ExportSpecifier(node) => node.type_(),
                ModuleSpecifierKind::ImportDefaultSpecifier(node) => node.type_(),
                ModuleSpecifierKind::ImportNamespaceSpecifier(node) => node.type_(),
                ModuleSpecifierKind::ImportSpecifier(node) => node.type_(),
            }
        }
    }
    ast_node!(Declaration, enum DeclarationKind {
        ClassDeclaration = CLASS_DECLARATION,
        FunctionDeclaration = FUNCTION_DECLARATION,
        VariableDeclaration = VARIABLE_DECLARATION,
    });
    impl Declaration {
        pub fn type_(&self) -> &'static str {
            match self.kind() {
                DeclarationKind::ClassDeclaration(node) => node.type_(),
                DeclarationKind::FunctionDeclaration(node) => node.type_(),
                DeclarationKind::VariableDeclaration(node) => node.type_(),
            }
        }
    }
    ast_node!(ArrayExpression, ARRAY_EXPRESSION);
    impl ArrayExpression {
        pub fn type_(&self) -> &'static str {
            "ArrayExpression"
        }
    }
    ast_node!(ArrayPattern, ARRAY_PATTERN);
    impl ArrayPattern {
        pub fn type_(&self) -> &'static str {
            "ArrayPattern"
        }
    }
    ast_node!(ArrowFunctionExpression, ARROW_FUNCTION_EXPRESSION);
    impl ArrowFunctionExpression {
        pub fn type_(&self) -> &'static str {
            "ArrowFunctionExpression"
        }
    }
    ast_node!(AssignmentExpression, ASSIGNMENT_EXPRESSION);
    impl AssignmentExpression {
        pub fn type_(&self) -> &'static str {
            "AssignmentExpression"
        }
    }
    ast_node!(AssignmentPattern, ASSIGNMENT_PATTERN);
    impl AssignmentPattern {
        pub fn type_(&self) -> &'static str {
            "AssignmentPattern"
        }
    }
    ast_node!(AwaitExpression, AWAIT_EXPRESSION);
    impl AwaitExpression {
        pub fn type_(&self) -> &'static str {
            "AwaitExpression"
        }
    }
    ast_node!(BinaryExpression, BINARY_EXPRESSION);
    impl BinaryExpression {
        pub fn type_(&self) -> &'static str {
            "BinaryExpression"
        }
    }
    ast_node!(BlockStatement, BLOCK_STATEMENT);
    impl BlockStatement {
        pub fn type_(&self) -> &'static str {
            "BlockStatement"
        }
    }
    ast_node!(BreakStatement, BREAK_STATEMENT);
    impl BreakStatement {
        pub fn type_(&self) -> &'static str {
            "BreakStatement"
        }
    }
    ast_node!(CallExpression, CALL_EXPRESSION);
    impl CallExpression {
        pub fn type_(&self) -> &'static str {
            "CallExpression"
        }
    }
    ast_node!(CatchClause, CATCH_CLAUSE);
    impl CatchClause {
        pub fn type_(&self) -> &'static str {
            "CatchClause"
        }
    }
    ast_node!(ClassBody, CLASS_BODY);
    impl ClassBody {
        pub fn type_(&self) -> &'static str {
            "ClassBody"
        }
    }
    ast_node!(ClassDeclaration, CLASS_DECLARATION);
    impl ClassDeclaration {
        pub fn type_(&self) -> &'static str {
            "ClassDeclaration"
        }
    }
    ast_node!(ClassExpression, CLASS_EXPRESSION);
    impl ClassExpression {
        pub fn type_(&self) -> &'static str {
            "ClassExpression"
        }
    }
    ast_node!(ConditionalExpression, CONDITIONAL_EXPRESSION);
    impl ConditionalExpression {
        pub fn type_(&self) -> &'static str {
            "ConditionalExpression"
        }
    }
    ast_node!(ContinueStatement, CONTINUE_STATEMENT);
    impl ContinueStatement {
        pub fn type_(&self) -> &'static str {
            "ContinueStatement"
        }
    }
    ast_node!(DebuggerStatement, DEBUGGER_STATEMENT);
    impl DebuggerStatement {
        pub fn type_(&self) -> &'static str {
            "DebuggerStatement"
        }
    }
    ast_node!(DoWhileStatement, DO_WHILE_STATEMENT);
    impl DoWhileStatement {
        pub fn type_(&self) -> &'static str {
            "DoWhileStatement"
        }
    }
    ast_node!(EmptyStatement, EMPTY_STATEMENT);
    impl EmptyStatement {
        pub fn type_(&self) -> &'static str {
            "EmptyStatement"
        }
    }
    ast_node!(ExportAllDeclaration, EXPORT_ALL_DECLARATION);
    impl ExportAllDeclaration {
        pub fn type_(&self) -> &'static str {
            "ExportAllDeclaration"
        }
    }
    ast_node!(ExportDefaultDeclaration, EXPORT_DEFAULT_DECLARATION);
    impl ExportDefaultDeclaration {
        pub fn type_(&self) -> &'static str {
            "ExportDefaultDeclaration"
        }
    }
    ast_node!(ExportNamedDeclaration, EXPORT_NAMED_DECLARATION);
    impl ExportNamedDeclaration {
        pub fn type_(&self) -> &'static str {
            "ExportNamedDeclaration"
        }
    }
    ast_node!(ExportSpecifier, EXPORT_SPECIFIER);
    impl ExportSpecifier {
        pub fn type_(&self) -> &'static str {
            "ExportSpecifier"
        }
    }
    ast_node!(ExpressionStatement, EXPRESSION_STATEMENT);
    impl ExpressionStatement {
        pub fn type_(&self) -> &'static str {
            "ExpressionStatement"
        }
    }
    ast_node!(ForInStatement, FOR_IN_STATEMENT);
    impl ForInStatement {
        pub fn type_(&self) -> &'static str {
            "ForInStatement"
        }
    }
    ast_node!(ForOfStatement, FOR_OF_STATEMENT);
    impl ForOfStatement {
        pub fn type_(&self) -> &'static str {
            "ForOfStatement"
        }
    }
    ast_node!(ForStatement, FOR_STATEMENT);
    impl ForStatement {
        pub fn type_(&self) -> &'static str {
            "ForStatement"
        }
    }
    ast_node!(FunctionDeclaration, FUNCTION_DECLARATION);
    impl FunctionDeclaration {
        pub fn type_(&self) -> &'static str {
            "FunctionDeclaration"
        }
    }
    ast_node!(FunctionExpression, FUNCTION_EXPRESSION);
    impl FunctionExpression {
        pub fn type_(&self) -> &'static str {
            "FunctionExpression"
        }
    }
    ast_node!(Identifier, IDENTIFIER);
    impl Identifier {
        pub fn type_(&self) -> &'static str {
            "Identifier"
        }
    }
    ast_node!(IfStatement, IF_STATEMENT);
    impl IfStatement {
        pub fn type_(&self) -> &'static str {
            "IfStatement"
        }
    }
    ast_node!(ImportDeclaration, IMPORT_DECLARATION);
    impl ImportDeclaration {
        pub fn type_(&self) -> &'static str {
            "ImportDeclaration"
        }
    }
    ast_node!(ImportDefaultSpecifier, IMPORT_DEFAULT_SPECIFIER);
    impl ImportDefaultSpecifier {
        pub fn type_(&self) -> &'static str {
            "ImportDefaultSpecifier"
        }
    }
    ast_node!(ImportNamespaceSpecifier, IMPORT_NAMESPACE_SPECIFIER);
    impl ImportNamespaceSpecifier {
        pub fn type_(&self) -> &'static str {
            "ImportNamespaceSpecifier"
        }
    }
    ast_node!(ImportSpecifier, IMPORT_SPECIFIER);
    impl ImportSpecifier {
        pub fn type_(&self) -> &'static str {
            "ImportSpecifier"
        }
    }
    ast_node!(LabeledStatement, LABELED_STATEMENT);
    impl LabeledStatement {
        pub fn type_(&self) -> &'static str {
            "LabeledStatement"
        }
    }
    ast_node!(Literal, LITERAL);
    impl Literal {
        pub fn type_(&self) -> &'static str {
            "Literal"
        }
    }
    ast_node!(LogicalExpression, LOGICAL_EXPRESSION);
    impl LogicalExpression {
        pub fn type_(&self) -> &'static str {
            "LogicalExpression"
        }
    }
    ast_node!(MemberExpression, MEMBER_EXPRESSION);
    impl MemberExpression {
        pub fn type_(&self) -> &'static str {
            "MemberExpression"
        }
    }
    ast_node!(MetaProperty, META_PROPERTY);
    impl MetaProperty {
        pub fn type_(&self) -> &'static str {
            "MetaProperty"
        }
    }
    ast_node!(MethodDefinition, METHOD_DEFINITION);
    impl MethodDefinition {
        pub fn type_(&self) -> &'static str {
            "MethodDefinition"
        }
    }
    ast_node!(NewExpression, NEW_EXPRESSION);
    impl NewExpression {
        pub fn type_(&self) -> &'static str {
            "NewExpression"
        }
    }
    ast_node!(ObjectExpression, OBJECT_EXPRESSION);
    impl ObjectExpression {
        pub fn type_(&self) -> &'static str {
            "ObjectExpression"
        }
    }
    ast_node!(ObjectPattern, OBJECT_PATTERN);
    impl ObjectPattern {
        pub fn type_(&self) -> &'static str {
            "ObjectPattern"
        }
    }
    ast_node!(Program, PROGRAM);
    impl Program {
        pub fn type_(&self) -> &'static str {
            "Program"
        }
    }
    ast_node!(Property, PROPERTY);
    impl Property {
        pub fn type_(&self) -> &'static str {
            "Property"
        }
    }
    ast_node!(RestElement, REST_ELEMENT);
    impl RestElement {
        pub fn type_(&self) -> &'static str {
            "RestElement"
        }
    }
    ast_node!(ReturnStatement, RETURN_STATEMENT);
    impl ReturnStatement {
        pub fn type_(&self) -> &'static str {
            "ReturnStatement"
        }
    }
    ast_node!(SequenceExpression, SEQUENCE_EXPRESSION);
    impl SequenceExpression {
        pub fn type_(&self) -> &'static str {
            "SequenceExpression"
        }
    }
    ast_node!(SpreadElement, SPREAD_ELEMENT);
    impl SpreadElement {
        pub fn type_(&self) -> &'static str {
            "SpreadElement"
        }
    }
    ast_node!(Super, SUPER_EXPRESSION);
    impl Super {
        pub fn type_(&self) -> &'static str {
            "Super"
        }
    }
    ast_node!(SwitchCase, SWITCH_CASE);
    impl SwitchCase {
        pub fn type_(&self) -> &'static str {
            "SwitchCase"
        }
    }
    ast_node!(SwitchStatement, SWITCH_STATEMENT);
    impl SwitchStatement {
        pub fn type_(&self) -> &'static str {
            "SwitchStatement"
        }
    }
    ast_node!(TSAsExpression, TS_AS_EXPRESSION);
    impl TSAsExpression {
        pub fn type_(&self) -> &'static str {
            "TSAsExpression"
        }
    }
    ast_node!(TSNonNullExpression, TS_NON_NULL_EXPRESSION);
    impl TSNonNullExpression {
        pub fn type_(&self) -> &'static str {
            "TSNonNullExpression"
        }
    }
    ast_node!(TaggedTemplateExpression, TAGGED_TEMPLATE_EXPRESSION);
    impl TaggedTemplateExpression {
        pub fn type_(&self) -> &'static str {
            "TaggedTemplateExpression"
        }
    }
    ast_node!(TemplateElement, TEMPLATE_ELEMENT);
    impl TemplateElement {
        pub fn type_(&self) -> &'static str {
            "TemplateElement"
        }
    }
    ast_node!(TemplateLiteral, TEMPLATE_EXPRESSION);
    impl TemplateLiteral {
        pub fn type_(&self) -> &'static str {
            "TemplateLiteral"
        }
    }
    ast_node!(ThisExpression, THIS_EXPRESSION);
    impl ThisExpression {
        pub fn type_(&self) -> &'static str {
            "ThisExpression"
        }
    }
    ast_node!(ThrowStatement, THROW_STATEMENT);
    impl ThrowStatement {
        pub fn type_(&self) -> &'static str {
            "ThrowStatement"
        }
    }
    ast_node!(TryStatement, TRY_STATEMENT);
    impl TryStatement {
        pub fn type_(&self) -> &'static str {
            "TryStatement"
        }
    }
    ast_node!(UnaryExpression, UNARY_EXPRESSION);
    impl UnaryExpression {
        pub fn type_(&self) -> &'static str {
            "UnaryExpression"
        }
    }
    ast_node!(UpdateExpression, UPDATE_EXPRESSION);
    impl UpdateExpression {
        pub fn type_(&self) -> &'static str {
            "UpdateExpression"
        }
    }
    ast_node!(VariableDeclaration, VARIABLE_DECLARATION);
    impl VariableDeclaration {
        pub fn type_(&self) -> &'static str {
            "VariableDeclaration"
        }
    }
    ast_node!(VariableDeclarator, VARIABLE_DECLARATOR);
    impl VariableDeclarator {
        pub fn type_(&self) -> &'static str {
            "VariableDeclarator"
        }
    }
    ast_node!(WhileStatement, WHILE_STATEMENT);
    impl WhileStatement {
        pub fn type_(&self) -> &'static str {
            "WhileStatement"
        }
    }
    ast_node!(WithStatement, WITH_STATEMENT);
    impl WithStatement {
        pub fn type_(&self) -> &'static str {
            "WithStatement"
        }
    }
    ast_node!(YieldExpression, YIELD_EXPRESSION);
    impl YieldExpression {
        pub fn type_(&self) -> &'static str {
            "YieldExpression"
        }
    }
}

pub mod syntax_kind {
    use crate::syntax_kind::JAVASCRIPT;
    use code_grammar::syntax_kinds;

    syntax_kinds! {
        language JAVASCRIPT;

        nodes {
            ARRAY_EXPRESSION 205
            ARRAY_PATTERN 206
            ARROW_FUNCTION_EXPRESSION 207
            ASSIGNMENT_EXPRESSION 208
            ASSIGNMENT_PATTERN 209
            AWAIT_EXPRESSION 210
            BINARY_EXPRESSION 211
            BLOCK_STATEMENT 212
            BREAK_STATEMENT 213
            CALL_EXPRESSION 214
            CATCH_CLAUSE 215
            CLASS_BODY 216
            CLASS_DECLARATION 217
            CLASS_EXPRESSION 218
            CONDITIONAL_EXPRESSION 219
            CONTINUE_STATEMENT 220
            DEBUGGER_STATEMENT 221
            DO_WHILE_STATEMENT 222
            EMPTY_STATEMENT 223
            EXPORT_ALL_DECLARATION 224
            EXPORT_DEFAULT_DECLARATION 225
            EXPORT_NAMED_DECLARATION 226
            EXPORT_SPECIFIER 227
            EXPRESSION_STATEMENT 228
            FOR_IN_STATEMENT 229
            FOR_OF_STATEMENT 230
            FOR_STATEMENT 231
            FUNCTION_DECLARATION 232
            FUNCTION_EXPRESSION 233
            IF_STATEMENT 234
            IMPORT_DECLARATION 235
            IMPORT_DEFAULT_SPECIFIER 236
            IMPORT_NAMESPACE_SPECIFIER 237
            IMPORT_SPECIFIER 238
            LABELED_STATEMENT 239
            LITERAL 240
            LOGICAL_EXPRESSION 241
            MEMBER_EXPRESSION 242
            META_PROPERTY 243
            METHOD_DEFINITION 244
            NEW_EXPRESSION 245
            OBJECT_EXPRESSION 246
            OBJECT_PATTERN 247
            PROGRAM 248
            PROPERTY 249
            REST_ELEMENT 250
            RETURN_STATEMENT 251
            SEQUENCE_EXPRESSION 252
            SPREAD_ELEMENT 253
            SUPER_EXPRESSION 254
            SWITCH_CASE 255
            SWITCH_STATEMENT 256
            TS_AS_EXPRESSION 257
            TS_NON_NULL_EXPRESSION 258
            TAGGED_TEMPLATE_EXPRESSION 259
            TEMPLATE_ELEMENT 260
            TEMPLATE_EXPRESSION 261
            THIS_EXPRESSION 262
            THROW_STATEMENT 263
            TRY_STATEMENT 264
            UNARY_EXPRESSION 265
            UPDATE_EXPRESSION 266
            VARIABLE_DECLARATION 267
            VARIABLE_DECLARATOR 268
            WHILE_STATEMENT 269
            WITH_STATEMENT 270
            YIELD_EXPRESSION 271
        }
    }
}
