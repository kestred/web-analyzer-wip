#![allow(dead_code)]

pub mod ast {
    use crate::syntax_kind::*;
    use web_grammars_utils::ast_node;

    ast_node!(Node, enum NodeKind {
        Program = PROGRAM,
        Function,
        Statement,
        SwitchCase = SWITCH_CASE,
        CatchClause = CATCH_CLAUSE,
        VariableDeclarator = VARIABLE_DECLARATOR,
        Expression,
        Property = PROPERTY,
        Pattern,
        Super = SUPER,
        SpreadElement = SPREAD_ELEMENT,
        TemplateElement = TEMPLATE_ELEMENT,
        Class,
        ClassBody = CLASS_BODY,
        MethodDefinition = METHOD_DEFINITION,
        ModuleDeclaration,
        ModuleSpecifier,
    });
    impl Node {
        pub fn type_(&self) -> &'static str {
            match self.kind() {
                NodeKind::Program(node) => node.type_(),
                NodeKind::Function(node) => node.type_(),
                NodeKind::Statement(node) => node.type_(),
                NodeKind::SwitchCase(node) => node.type_(),
                NodeKind::CatchClause(node) => node.type_(),
                NodeKind::VariableDeclarator(node) => node.type_(),
                NodeKind::Expression(node) => node.type_(),
                NodeKind::Property(node) => node.type_(),
                NodeKind::Pattern(node) => node.type_(),
                NodeKind::Super(node) => node.type_(),
                NodeKind::SpreadElement(node) => node.type_(),
                NodeKind::TemplateElement(node) => node.type_(),
                NodeKind::Class(node) => node.type_(),
                NodeKind::ClassBody(node) => node.type_(),
                NodeKind::MethodDefinition(node) => node.type_(),
                NodeKind::ModuleDeclaration(node) => node.type_(),
                NodeKind::ModuleSpecifier(node) => node.type_(),
            }
        }
    }
    ast_node!(Function, enum FunctionKind {
        FunctionDeclaration = FUNCTION_DECLARATION,
        FunctionExpression = FUNCTION_EXPRESSION,
        ArrowFunctionExpression = ARROW_FUNCTION_EXPRESSION,
    });
    impl Function {
        pub fn type_(&self) -> &'static str {
            match self.kind() {
                FunctionKind::FunctionDeclaration(node) => node.type_(),
                FunctionKind::FunctionExpression(node) => node.type_(),
                FunctionKind::ArrowFunctionExpression(node) => node.type_(),
            }
        }
    }
    ast_node!(Statement, enum StatementKind {
        ExpressionStatement = EXPRESSION_STATEMENT,
        BlockStatement = BLOCK_STATEMENT,
        EmptyStatement = EMPTY_STATEMENT,
        DebuggerStatement = DEBUGGER_STATEMENT,
        WithStatement = WITH_STATEMENT,
        ReturnStatement = RETURN_STATEMENT,
        LabeledStatement = LABELED_STATEMENT,
        BreakStatement = BREAK_STATEMENT,
        ContinueStatement = CONTINUE_STATEMENT,
        IfStatement = IF_STATEMENT,
        SwitchStatement = SWITCH_STATEMENT,
        ThrowStatement = THROW_STATEMENT,
        TryStatement = TRY_STATEMENT,
        WhileStatement = WHILE_STATEMENT,
        DoWhileStatement = DO_WHILE_STATEMENT,
        ForStatement = FOR_STATEMENT,
        ForInStatement = FOR_IN_STATEMENT,
        Declaration,
    });
    impl Statement {
        pub fn type_(&self) -> &'static str {
            match self.kind() {
                StatementKind::ExpressionStatement(node) => node.type_(),
                StatementKind::BlockStatement(node) => node.type_(),
                StatementKind::EmptyStatement(node) => node.type_(),
                StatementKind::DebuggerStatement(node) => node.type_(),
                StatementKind::WithStatement(node) => node.type_(),
                StatementKind::ReturnStatement(node) => node.type_(),
                StatementKind::LabeledStatement(node) => node.type_(),
                StatementKind::BreakStatement(node) => node.type_(),
                StatementKind::ContinueStatement(node) => node.type_(),
                StatementKind::IfStatement(node) => node.type_(),
                StatementKind::SwitchStatement(node) => node.type_(),
                StatementKind::ThrowStatement(node) => node.type_(),
                StatementKind::TryStatement(node) => node.type_(),
                StatementKind::WhileStatement(node) => node.type_(),
                StatementKind::DoWhileStatement(node) => node.type_(),
                StatementKind::ForStatement(node) => node.type_(),
                StatementKind::ForInStatement(node) => node.type_(),
                StatementKind::Declaration(node) => node.type_(),
            }
        }
    }
    ast_node!(Expression, enum ExpressionKind {
        Identifier = IDENTIFIER,
        Literal = LITERAL,
        ThisExpression = THIS_EXPRESSION,
        ArrayExpression = ARRAY_EXPRESSION,
        ObjectExpression = OBJECT_EXPRESSION,
        FunctionExpression = FUNCTION_EXPRESSION,
        UnaryExpression = UNARY_EXPRESSION,
        UpdateExpression = UPDATE_EXPRESSION,
        BinaryExpression = BINARY_EXPRESSION,
        AssignmentExpression = ASSIGNMENT_EXPRESSION,
        LogicalExpression = LOGICAL_EXPRESSION,
        MemberExpression = MEMBER_EXPRESSION,
        ConditionalExpression = CONDITIONAL_EXPRESSION,
        CallExpression = CALL_EXPRESSION,
        NewExpression = NEW_EXPRESSION,
        SequenceExpression = SEQUENCE_EXPRESSION,
        ArrowFunctionExpression = ARROW_FUNCTION_EXPRESSION,
        YieldExpression = YIELD_EXPRESSION,
        TemplateLiteral = TEMPLATE_LITERAL,
        TaggedTemplateExpression = TAGGED_TEMPLATE_EXPRESSION,
        ClassExpression = CLASS_EXPRESSION,
        MetaProperty = META_PROPERTY,
        AwaitExpression = AWAIT_EXPRESSION,
    });
    impl Expression {
        pub fn type_(&self) -> &'static str {
            match self.kind() {
                ExpressionKind::Identifier(node) => node.type_(),
                ExpressionKind::Literal(node) => node.type_(),
                ExpressionKind::ThisExpression(node) => node.type_(),
                ExpressionKind::ArrayExpression(node) => node.type_(),
                ExpressionKind::ObjectExpression(node) => node.type_(),
                ExpressionKind::FunctionExpression(node) => node.type_(),
                ExpressionKind::UnaryExpression(node) => node.type_(),
                ExpressionKind::UpdateExpression(node) => node.type_(),
                ExpressionKind::BinaryExpression(node) => node.type_(),
                ExpressionKind::AssignmentExpression(node) => node.type_(),
                ExpressionKind::LogicalExpression(node) => node.type_(),
                ExpressionKind::MemberExpression(node) => node.type_(),
                ExpressionKind::ConditionalExpression(node) => node.type_(),
                ExpressionKind::CallExpression(node) => node.type_(),
                ExpressionKind::NewExpression(node) => node.type_(),
                ExpressionKind::SequenceExpression(node) => node.type_(),
                ExpressionKind::ArrowFunctionExpression(node) => node.type_(),
                ExpressionKind::YieldExpression(node) => node.type_(),
                ExpressionKind::TemplateLiteral(node) => node.type_(),
                ExpressionKind::TaggedTemplateExpression(node) => node.type_(),
                ExpressionKind::ClassExpression(node) => node.type_(),
                ExpressionKind::MetaProperty(node) => node.type_(),
                ExpressionKind::AwaitExpression(node) => node.type_(),
            }
        }
    }
    ast_node!(Pattern, enum PatternKind {
        Identifier = IDENTIFIER,
        MemberExpression = MEMBER_EXPRESSION,
        ObjectPattern = OBJECT_PATTERN,
        ArrayPattern = ARRAY_PATTERN,
        RestElement = REST_ELEMENT,
        AssignmentPattern = ASSIGNMENT_PATTERN,
    });
    impl Pattern {
        pub fn type_(&self) -> &'static str {
            match self.kind() {
                PatternKind::Identifier(node) => node.type_(),
                PatternKind::MemberExpression(node) => node.type_(),
                PatternKind::ObjectPattern(node) => node.type_(),
                PatternKind::ArrayPattern(node) => node.type_(),
                PatternKind::RestElement(node) => node.type_(),
                PatternKind::AssignmentPattern(node) => node.type_(),
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
        ImportDeclaration = IMPORT_DECLARATION,
        ExportNamedDeclaration = EXPORT_NAMED_DECLARATION,
        ExportDefaultDeclaration = EXPORT_DEFAULT_DECLARATION,
        ExportAllDeclaration = EXPORT_ALL_DECLARATION,
    });
    impl ModuleDeclaration {
        pub fn type_(&self) -> &'static str {
            match self.kind() {
                ModuleDeclarationKind::ImportDeclaration(node) => node.type_(),
                ModuleDeclarationKind::ExportNamedDeclaration(node) => node.type_(),
                ModuleDeclarationKind::ExportDefaultDeclaration(node) => node.type_(),
                ModuleDeclarationKind::ExportAllDeclaration(node) => node.type_(),
            }
        }
    }
    ast_node!(ModuleSpecifier, enum ModuleSpecifierKind {
        ImportSpecifier = IMPORT_SPECIFIER,
        ImportDefaultSpecifier = IMPORT_DEFAULT_SPECIFIER,
        ImportNamespaceSpecifier = IMPORT_NAMESPACE_SPECIFIER,
        ExportSpecifier = EXPORT_SPECIFIER,
    });
    impl ModuleSpecifier {
        pub fn type_(&self) -> &'static str {
            match self.kind() {
                ModuleSpecifierKind::ImportSpecifier(node) => node.type_(),
                ModuleSpecifierKind::ImportDefaultSpecifier(node) => node.type_(),
                ModuleSpecifierKind::ImportNamespaceSpecifier(node) => node.type_(),
                ModuleSpecifierKind::ExportSpecifier(node) => node.type_(),
            }
        }
    }
    ast_node!(Declaration, enum DeclarationKind {
        FunctionDeclaration = FUNCTION_DECLARATION,
        VariableDeclaration = VARIABLE_DECLARATION,
        ClassDeclaration = CLASS_DECLARATION,
    });
    impl Declaration {
        pub fn type_(&self) -> &'static str {
            match self.kind() {
                DeclarationKind::FunctionDeclaration(node) => node.type_(),
                DeclarationKind::VariableDeclaration(node) => node.type_(),
                DeclarationKind::ClassDeclaration(node) => node.type_(),
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
    ast_node!(Super, SUPER);
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
    ast_node!(TemplateLiteral, TEMPLATE_LITERAL);
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
    use web_grammars_utils::SyntaxKind;

    pub const ARRAY_EXPRESSION: SyntaxKind = JAVASCRIPT.syntax_kind(205);
    pub const ARRAY_PATTERN: SyntaxKind = JAVASCRIPT.syntax_kind(206);
    pub const ARROW_FUNCTION_EXPRESSION: SyntaxKind = JAVASCRIPT.syntax_kind(207);
    pub const ASSIGNMENT_EXPRESSION: SyntaxKind = JAVASCRIPT.syntax_kind(208);
    pub const ASSIGNMENT_PATTERN: SyntaxKind = JAVASCRIPT.syntax_kind(209);
    pub const AWAIT_EXPRESSION: SyntaxKind = JAVASCRIPT.syntax_kind(210);
    pub const BINARY_EXPRESSION: SyntaxKind = JAVASCRIPT.syntax_kind(211);
    pub const BLOCK_STATEMENT: SyntaxKind = JAVASCRIPT.syntax_kind(212);
    pub const BREAK_STATEMENT: SyntaxKind = JAVASCRIPT.syntax_kind(213);
    pub const CALL_EXPRESSION: SyntaxKind = JAVASCRIPT.syntax_kind(214);
    pub const CATCH_CLAUSE: SyntaxKind = JAVASCRIPT.syntax_kind(215);
    pub const CLASS_BODY: SyntaxKind = JAVASCRIPT.syntax_kind(216);
    pub const CLASS_DECLARATION: SyntaxKind = JAVASCRIPT.syntax_kind(217);
    pub const CLASS_EXPRESSION: SyntaxKind = JAVASCRIPT.syntax_kind(218);
    pub const CONDITIONAL_EXPRESSION: SyntaxKind = JAVASCRIPT.syntax_kind(219);
    pub const CONTINUE_STATEMENT: SyntaxKind = JAVASCRIPT.syntax_kind(220);
    pub const DEBUGGER_STATEMENT: SyntaxKind = JAVASCRIPT.syntax_kind(221);
    pub const DO_WHILE_STATEMENT: SyntaxKind = JAVASCRIPT.syntax_kind(222);
    pub const EMPTY_STATEMENT: SyntaxKind = JAVASCRIPT.syntax_kind(223);
    pub const EXPORT_ALL_DECLARATION: SyntaxKind = JAVASCRIPT.syntax_kind(224);
    pub const EXPORT_DEFAULT_DECLARATION: SyntaxKind = JAVASCRIPT.syntax_kind(225);
    pub const EXPORT_NAMED_DECLARATION: SyntaxKind = JAVASCRIPT.syntax_kind(226);
    pub const EXPORT_SPECIFIER: SyntaxKind = JAVASCRIPT.syntax_kind(227);
    pub const EXPRESSION_STATEMENT: SyntaxKind = JAVASCRIPT.syntax_kind(228);
    pub const FOR_IN_STATEMENT: SyntaxKind = JAVASCRIPT.syntax_kind(229);
    pub const FOR_OF_STATEMENT: SyntaxKind = JAVASCRIPT.syntax_kind(230);
    pub const FOR_STATEMENT: SyntaxKind = JAVASCRIPT.syntax_kind(231);
    pub const FUNCTION_DECLARATION: SyntaxKind = JAVASCRIPT.syntax_kind(232);
    pub const FUNCTION_EXPRESSION: SyntaxKind = JAVASCRIPT.syntax_kind(233);
    pub const IDENTIFIER: SyntaxKind = JAVASCRIPT.syntax_kind(234);
    pub const IF_STATEMENT: SyntaxKind = JAVASCRIPT.syntax_kind(235);
    pub const IMPORT_DECLARATION: SyntaxKind = JAVASCRIPT.syntax_kind(236);
    pub const IMPORT_DEFAULT_SPECIFIER: SyntaxKind = JAVASCRIPT.syntax_kind(237);
    pub const IMPORT_NAMESPACE_SPECIFIER: SyntaxKind = JAVASCRIPT.syntax_kind(238);
    pub const IMPORT_SPECIFIER: SyntaxKind = JAVASCRIPT.syntax_kind(239);
    pub const LABELED_STATEMENT: SyntaxKind = JAVASCRIPT.syntax_kind(240);
    pub const LITERAL: SyntaxKind = JAVASCRIPT.syntax_kind(241);
    pub const LOGICAL_EXPRESSION: SyntaxKind = JAVASCRIPT.syntax_kind(242);
    pub const MEMBER_EXPRESSION: SyntaxKind = JAVASCRIPT.syntax_kind(243);
    pub const META_PROPERTY: SyntaxKind = JAVASCRIPT.syntax_kind(244);
    pub const METHOD_DEFINITION: SyntaxKind = JAVASCRIPT.syntax_kind(245);
    pub const NEW_EXPRESSION: SyntaxKind = JAVASCRIPT.syntax_kind(246);
    pub const OBJECT_EXPRESSION: SyntaxKind = JAVASCRIPT.syntax_kind(247);
    pub const OBJECT_PATTERN: SyntaxKind = JAVASCRIPT.syntax_kind(248);
    pub const PROGRAM: SyntaxKind = JAVASCRIPT.syntax_kind(249);
    pub const PROPERTY: SyntaxKind = JAVASCRIPT.syntax_kind(250);
    pub const REST_ELEMENT: SyntaxKind = JAVASCRIPT.syntax_kind(251);
    pub const RETURN_STATEMENT: SyntaxKind = JAVASCRIPT.syntax_kind(252);
    pub const SEQUENCE_EXPRESSION: SyntaxKind = JAVASCRIPT.syntax_kind(253);
    pub const SPREAD_ELEMENT: SyntaxKind = JAVASCRIPT.syntax_kind(254);
    pub const SUPER: SyntaxKind = JAVASCRIPT.syntax_kind(255);
    pub const SWITCH_CASE: SyntaxKind = JAVASCRIPT.syntax_kind(256);
    pub const SWITCH_STATEMENT: SyntaxKind = JAVASCRIPT.syntax_kind(257);
    pub const TAGGED_TEMPLATE_EXPRESSION: SyntaxKind = JAVASCRIPT.syntax_kind(258);
    pub const TEMPLATE_ELEMENT: SyntaxKind = JAVASCRIPT.syntax_kind(259);
    pub const TEMPLATE_LITERAL: SyntaxKind = JAVASCRIPT.syntax_kind(260);
    pub const THIS_EXPRESSION: SyntaxKind = JAVASCRIPT.syntax_kind(261);
    pub const THROW_STATEMENT: SyntaxKind = JAVASCRIPT.syntax_kind(262);
    pub const TRY_STATEMENT: SyntaxKind = JAVASCRIPT.syntax_kind(263);
    pub const UNARY_EXPRESSION: SyntaxKind = JAVASCRIPT.syntax_kind(264);
    pub const UPDATE_EXPRESSION: SyntaxKind = JAVASCRIPT.syntax_kind(265);
    pub const VARIABLE_DECLARATION: SyntaxKind = JAVASCRIPT.syntax_kind(266);
    pub const VARIABLE_DECLARATOR: SyntaxKind = JAVASCRIPT.syntax_kind(267);
    pub const WHILE_STATEMENT: SyntaxKind = JAVASCRIPT.syntax_kind(268);
    pub const WITH_STATEMENT: SyntaxKind = JAVASCRIPT.syntax_kind(269);
    pub const YIELD_EXPRESSION: SyntaxKind = JAVASCRIPT.syntax_kind(270);
}
