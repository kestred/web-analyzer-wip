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
    ast_node!(Function, enum FunctionKind {
        FunctionDeclaration = FUNCTION_DECLARATION,
        FunctionExpression = FUNCTION_EXPRESSION,
        ArrowFunctionExpression = ARROW_FUNCTION_EXPRESSION,
    });
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
    ast_node!(Pattern, enum PatternKind {
        Identifier = IDENTIFIER,
        MemberExpression = MEMBER_EXPRESSION,
        ObjectPattern = OBJECT_PATTERN,
        ArrayPattern = ARRAY_PATTERN,
        RestElement = REST_ELEMENT,
        AssignmentPattern = ASSIGNMENT_PATTERN,
    });
    ast_node!(Class, enum ClassKind {
        ClassDeclaration = CLASS_DECLARATION,
        ClassExpression = CLASS_EXPRESSION,
    });
    ast_node!(ModuleDeclaration, enum ModuleDeclarationKind {
        ImportDeclaration = IMPORT_DECLARATION,
        ExportNamedDeclaration = EXPORT_NAMED_DECLARATION,
        ExportDefaultDeclaration = EXPORT_DEFAULT_DECLARATION,
        ExportAllDeclaration = EXPORT_ALL_DECLARATION,
    });
    ast_node!(ModuleSpecifier, enum ModuleSpecifierKind {
        ImportSpecifier = IMPORT_SPECIFIER,
        ImportDefaultSpecifier = IMPORT_DEFAULT_SPECIFIER,
        ImportNamespaceSpecifier = IMPORT_NAMESPACE_SPECIFIER,
        ExportSpecifier = EXPORT_SPECIFIER,
    });
    ast_node!(Declaration, enum DeclarationKind {
        FunctionDeclaration = FUNCTION_DECLARATION,
        VariableDeclaration = VARIABLE_DECLARATION,
        ClassDeclaration = CLASS_DECLARATION,
    });
    ast_node!(ArrayExpression, ARRAY_EXPRESSION);
    impl ArrayExpression {
        fn type_() -> &'static str {
            "ArrayExpression"
        }
    }
    ast_node!(ArrayPattern, ARRAY_PATTERN);
    impl ArrayPattern {
        fn type_() -> &'static str {
            "ArrayPattern"
        }
    }
    ast_node!(ArrowFunctionExpression, ARROW_FUNCTION_EXPRESSION);
    impl ArrowFunctionExpression {
        fn type_() -> &'static str {
            "ArrowFunctionExpression"
        }
    }
    ast_node!(AssignmentExpression, ASSIGNMENT_EXPRESSION);
    impl AssignmentExpression {
        fn type_() -> &'static str {
            "AssignmentExpression"
        }
    }
    ast_node!(AssignmentPattern, ASSIGNMENT_PATTERN);
    impl AssignmentPattern {
        fn type_() -> &'static str {
            "AssignmentPattern"
        }
    }
    ast_node!(AwaitExpression, AWAIT_EXPRESSION);
    impl AwaitExpression {
        fn type_() -> &'static str {
            "AwaitExpression"
        }
    }
    ast_node!(BinaryExpression, BINARY_EXPRESSION);
    impl BinaryExpression {
        fn type_() -> &'static str {
            "BinaryExpression"
        }
    }
    ast_node!(BlockStatement, BLOCK_STATEMENT);
    impl BlockStatement {
        fn type_() -> &'static str {
            "BlockStatement"
        }
    }
    ast_node!(BreakStatement, BREAK_STATEMENT);
    impl BreakStatement {
        fn type_() -> &'static str {
            "BreakStatement"
        }
    }
    ast_node!(CallExpression, CALL_EXPRESSION);
    impl CallExpression {
        fn type_() -> &'static str {
            "CallExpression"
        }
    }
    ast_node!(CatchClause, CATCH_CLAUSE);
    impl CatchClause {
        fn type_() -> &'static str {
            "CatchClause"
        }
    }
    ast_node!(ClassBody, CLASS_BODY);
    impl ClassBody {
        fn type_() -> &'static str {
            "ClassBody"
        }
    }
    ast_node!(ClassDeclaration, CLASS_DECLARATION);
    impl ClassDeclaration {
        fn type_() -> &'static str {
            "ClassDeclaration"
        }
    }
    ast_node!(ClassExpression, CLASS_EXPRESSION);
    impl ClassExpression {
        fn type_() -> &'static str {
            "ClassExpression"
        }
    }
    ast_node!(ConditionalExpression, CONDITIONAL_EXPRESSION);
    impl ConditionalExpression {
        fn type_() -> &'static str {
            "ConditionalExpression"
        }
    }
    ast_node!(ContinueStatement, CONTINUE_STATEMENT);
    impl ContinueStatement {
        fn type_() -> &'static str {
            "ContinueStatement"
        }
    }
    ast_node!(DebuggerStatement, DEBUGGER_STATEMENT);
    impl DebuggerStatement {
        fn type_() -> &'static str {
            "DebuggerStatement"
        }
    }
    ast_node!(DoWhileStatement, DO_WHILE_STATEMENT);
    impl DoWhileStatement {
        fn type_() -> &'static str {
            "DoWhileStatement"
        }
    }
    ast_node!(EmptyStatement, EMPTY_STATEMENT);
    impl EmptyStatement {
        fn type_() -> &'static str {
            "EmptyStatement"
        }
    }
    ast_node!(ExportAllDeclaration, EXPORT_ALL_DECLARATION);
    impl ExportAllDeclaration {
        fn type_() -> &'static str {
            "ExportAllDeclaration"
        }
    }
    ast_node!(ExportDefaultDeclaration, EXPORT_DEFAULT_DECLARATION);
    impl ExportDefaultDeclaration {
        fn type_() -> &'static str {
            "ExportDefaultDeclaration"
        }
    }
    ast_node!(ExportNamedDeclaration, EXPORT_NAMED_DECLARATION);
    impl ExportNamedDeclaration {
        fn type_() -> &'static str {
            "ExportNamedDeclaration"
        }
    }
    ast_node!(ExportSpecifier, EXPORT_SPECIFIER);
    impl ExportSpecifier {
        fn type_() -> &'static str {
            "ExportSpecifier"
        }
    }
    ast_node!(ExpressionStatement, EXPRESSION_STATEMENT);
    impl ExpressionStatement {
        fn type_() -> &'static str {
            "ExpressionStatement"
        }
    }
    ast_node!(ForInStatement, FOR_IN_STATEMENT);
    impl ForInStatement {
        fn type_() -> &'static str {
            "ForInStatement"
        }
    }
    ast_node!(ForOfStatement, FOR_OF_STATEMENT);
    impl ForOfStatement {
        fn type_() -> &'static str {
            "ForOfStatement"
        }
    }
    ast_node!(ForStatement, FOR_STATEMENT);
    impl ForStatement {
        fn type_() -> &'static str {
            "ForStatement"
        }
    }
    ast_node!(FunctionDeclaration, FUNCTION_DECLARATION);
    impl FunctionDeclaration {
        fn type_() -> &'static str {
            "FunctionDeclaration"
        }
    }
    ast_node!(FunctionExpression, FUNCTION_EXPRESSION);
    impl FunctionExpression {
        fn type_() -> &'static str {
            "FunctionExpression"
        }
    }
    ast_node!(Identifier, IDENTIFIER);
    impl Identifier {
        fn type_() -> &'static str {
            "Identifier"
        }
    }
    ast_node!(IfStatement, IF_STATEMENT);
    impl IfStatement {
        fn type_() -> &'static str {
            "IfStatement"
        }
    }
    ast_node!(ImportDeclaration, IMPORT_DECLARATION);
    impl ImportDeclaration {
        fn type_() -> &'static str {
            "ImportDeclaration"
        }
    }
    ast_node!(ImportDefaultSpecifier, IMPORT_DEFAULT_SPECIFIER);
    impl ImportDefaultSpecifier {
        fn type_() -> &'static str {
            "ImportDefaultSpecifier"
        }
    }
    ast_node!(ImportNamespaceSpecifier, IMPORT_NAMESPACE_SPECIFIER);
    impl ImportNamespaceSpecifier {
        fn type_() -> &'static str {
            "ImportNamespaceSpecifier"
        }
    }
    ast_node!(ImportSpecifier, IMPORT_SPECIFIER);
    impl ImportSpecifier {
        fn type_() -> &'static str {
            "ImportSpecifier"
        }
    }
    ast_node!(LabeledStatement, LABELED_STATEMENT);
    impl LabeledStatement {
        fn type_() -> &'static str {
            "LabeledStatement"
        }
    }
    ast_node!(Literal, LITERAL);
    impl Literal {
        fn type_() -> &'static str {
            "Literal"
        }
    }
    ast_node!(LogicalExpression, LOGICAL_EXPRESSION);
    impl LogicalExpression {
        fn type_() -> &'static str {
            "LogicalExpression"
        }
    }
    ast_node!(MemberExpression, MEMBER_EXPRESSION);
    impl MemberExpression {
        fn type_() -> &'static str {
            "MemberExpression"
        }
    }
    ast_node!(MetaProperty, META_PROPERTY);
    impl MetaProperty {
        fn type_() -> &'static str {
            "MetaProperty"
        }
    }
    ast_node!(MethodDefinition, METHOD_DEFINITION);
    impl MethodDefinition {
        fn type_() -> &'static str {
            "MethodDefinition"
        }
    }
    ast_node!(NewExpression, NEW_EXPRESSION);
    impl NewExpression {
        fn type_() -> &'static str {
            "NewExpression"
        }
    }
    ast_node!(ObjectExpression, OBJECT_EXPRESSION);
    impl ObjectExpression {
        fn type_() -> &'static str {
            "ObjectExpression"
        }
    }
    ast_node!(ObjectPattern, OBJECT_PATTERN);
    impl ObjectPattern {
        fn type_() -> &'static str {
            "ObjectPattern"
        }
    }
    ast_node!(Program, PROGRAM);
    impl Program {
        fn type_() -> &'static str {
            "Program"
        }
    }
    ast_node!(Property, PROPERTY);
    impl Property {
        fn type_() -> &'static str {
            "Property"
        }
    }
    ast_node!(RestElement, REST_ELEMENT);
    impl RestElement {
        fn type_() -> &'static str {
            "RestElement"
        }
    }
    ast_node!(ReturnStatement, RETURN_STATEMENT);
    impl ReturnStatement {
        fn type_() -> &'static str {
            "ReturnStatement"
        }
    }
    ast_node!(SequenceExpression, SEQUENCE_EXPRESSION);
    impl SequenceExpression {
        fn type_() -> &'static str {
            "SequenceExpression"
        }
    }
    ast_node!(SpreadElement, SPREAD_ELEMENT);
    impl SpreadElement {
        fn type_() -> &'static str {
            "SpreadElement"
        }
    }
    ast_node!(Super, SUPER);
    impl Super {
        fn type_() -> &'static str {
            "Super"
        }
    }
    ast_node!(SwitchCase, SWITCH_CASE);
    impl SwitchCase {
        fn type_() -> &'static str {
            "SwitchCase"
        }
    }
    ast_node!(SwitchStatement, SWITCH_STATEMENT);
    impl SwitchStatement {
        fn type_() -> &'static str {
            "SwitchStatement"
        }
    }
    ast_node!(TaggedTemplateExpression, TAGGED_TEMPLATE_EXPRESSION);
    impl TaggedTemplateExpression {
        fn type_() -> &'static str {
            "TaggedTemplateExpression"
        }
    }
    ast_node!(TemplateElement, TEMPLATE_ELEMENT);
    impl TemplateElement {
        fn type_() -> &'static str {
            "TemplateElement"
        }
    }
    ast_node!(TemplateLiteral, TEMPLATE_LITERAL);
    impl TemplateLiteral {
        fn type_() -> &'static str {
            "TemplateLiteral"
        }
    }
    ast_node!(ThisExpression, THIS_EXPRESSION);
    impl ThisExpression {
        fn type_() -> &'static str {
            "ThisExpression"
        }
    }
    ast_node!(ThrowStatement, THROW_STATEMENT);
    impl ThrowStatement {
        fn type_() -> &'static str {
            "ThrowStatement"
        }
    }
    ast_node!(TryStatement, TRY_STATEMENT);
    impl TryStatement {
        fn type_() -> &'static str {
            "TryStatement"
        }
    }
    ast_node!(UnaryExpression, UNARY_EXPRESSION);
    impl UnaryExpression {
        fn type_() -> &'static str {
            "UnaryExpression"
        }
    }
    ast_node!(UpdateExpression, UPDATE_EXPRESSION);
    impl UpdateExpression {
        fn type_() -> &'static str {
            "UpdateExpression"
        }
    }
    ast_node!(VariableDeclaration, VARIABLE_DECLARATION);
    impl VariableDeclaration {
        fn type_() -> &'static str {
            "VariableDeclaration"
        }
    }
    ast_node!(VariableDeclarator, VARIABLE_DECLARATOR);
    impl VariableDeclarator {
        fn type_() -> &'static str {
            "VariableDeclarator"
        }
    }
    ast_node!(WhileStatement, WHILE_STATEMENT);
    impl WhileStatement {
        fn type_() -> &'static str {
            "WhileStatement"
        }
    }
    ast_node!(WithStatement, WITH_STATEMENT);
    impl WithStatement {
        fn type_() -> &'static str {
            "WithStatement"
        }
    }
    ast_node!(YieldExpression, YIELD_EXPRESSION);
    impl YieldExpression {
        fn type_() -> &'static str {
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
