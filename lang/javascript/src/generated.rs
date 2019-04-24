pub mod ast {
    use crate::syntax_kind::*;
    use web_grammars_utils::ast_node;

    ast_node!(Node, enum NodeKind {
        Function,
        Statement,
        Directive = DIRECTIVE,
        SwitchCase = SWITCH_CASE,
        VariableDeclarator = VARIABLE_DECLARATOR,
        Expression,
        Property,
        Pattern,
        Super = SUPER,
        SpreadElement = SPREAD_ELEMENT,
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
        AnonymousDefaultExportedFunctionDeclaration = ANONYMOUS_DEFAULT_EXPORTED_FUNCTION_DECLARATION,
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
        FunctionExpression = FUNCTION_EXPRESSION,
        UnaryExpression = UNARY_EXPRESSION,
        UpdateExpression = UPDATE_EXPRESSION,
        BinaryExpression = BINARY_EXPRESSION,
        LogicalExpression = LOGICAL_EXPRESSION,
        ConditionalExpression = CONDITIONAL_EXPRESSION,
        SequenceExpression = SEQUENCE_EXPRESSION,
        ArrowFunctionExpression = ARROW_FUNCTION_EXPRESSION,
        YieldExpression = YIELD_EXPRESSION,
        TemplateLiteral = TEMPLATE_LITERAL,
        TaggedTemplateExpression = TAGGED_TEMPLATE_EXPRESSION,
        ClassExpression = CLASS_EXPRESSION,
        MetaProperty = META_PROPERTY,
        AwaitExpression = AWAIT_EXPRESSION,
    });
    ast_node!(Property, enum PropertyKind {
        AssignmentProperty = ASSIGNMENT_PROPERTY,
    });
    ast_node!(Pattern, enum PatternKind {
        Identifier = IDENTIFIER,
        ArrayPattern = ARRAY_PATTERN,
        RestElement = REST_ELEMENT,
        AssignmentPattern = ASSIGNMENT_PATTERN,
    });
    ast_node!(Class, enum ClassKind {
        ClassDeclaration = CLASS_DECLARATION,
        ClassExpression = CLASS_EXPRESSION,
        AnonymousDefaultExportedClassDeclaration = ANONYMOUS_DEFAULT_EXPORTED_CLASS_DECLARATION,
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
        ClassDeclaration = CLASS_DECLARATION,
    });
    ast_node!(AnonymousDefaultExportedClassDeclaration, ANONYMOUS_DEFAULT_EXPORTED_CLASS_DECLARATION);
    ast_node!(AnonymousDefaultExportedFunctionDeclaration, ANONYMOUS_DEFAULT_EXPORTED_FUNCTION_DECLARATION);
    ast_node!(ArrayPattern, ARRAY_PATTERN);
    ast_node!(ArrowFunctionExpression, ARROW_FUNCTION_EXPRESSION);
    ast_node!(AssignmentPattern, ASSIGNMENT_PATTERN);
    ast_node!(AssignmentProperty, ASSIGNMENT_PROPERTY);
    ast_node!(AwaitExpression, AWAIT_EXPRESSION);
    ast_node!(BinaryExpression, BINARY_EXPRESSION);
    ast_node!(BlockStatement, BLOCK_STATEMENT);
    ast_node!(BreakStatement, BREAK_STATEMENT);
    ast_node!(ClassBody, CLASS_BODY);
    ast_node!(ClassDeclaration, CLASS_DECLARATION);
    ast_node!(ClassExpression, CLASS_EXPRESSION);
    ast_node!(ConditionalExpression, CONDITIONAL_EXPRESSION);
    ast_node!(ContinueStatement, CONTINUE_STATEMENT);
    ast_node!(DebuggerStatement, DEBUGGER_STATEMENT);
    ast_node!(Directive, DIRECTIVE);
    ast_node!(DoWhileStatement, DO_WHILE_STATEMENT);
    ast_node!(EmptyStatement, EMPTY_STATEMENT);
    ast_node!(ExportAllDeclaration, EXPORT_ALL_DECLARATION);
    ast_node!(ExportDefaultDeclaration, EXPORT_DEFAULT_DECLARATION);
    ast_node!(ExportNamedDeclaration, EXPORT_NAMED_DECLARATION);
    ast_node!(ExportSpecifier, EXPORT_SPECIFIER);
    ast_node!(ExpressionStatement, EXPRESSION_STATEMENT);
    ast_node!(ForInStatement, FOR_IN_STATEMENT);
    ast_node!(ForStatement, FOR_STATEMENT);
    ast_node!(FunctionDeclaration, FUNCTION_DECLARATION);
    ast_node!(FunctionExpression, FUNCTION_EXPRESSION);
    ast_node!(Identifier, IDENTIFIER);
    ast_node!(IfStatement, IF_STATEMENT);
    ast_node!(ImportDeclaration, IMPORT_DECLARATION);
    ast_node!(ImportDefaultSpecifier, IMPORT_DEFAULT_SPECIFIER);
    ast_node!(ImportNamespaceSpecifier, IMPORT_NAMESPACE_SPECIFIER);
    ast_node!(ImportSpecifier, IMPORT_SPECIFIER);
    ast_node!(LabeledStatement, LABELED_STATEMENT);
    ast_node!(Literal, LITERAL);
    ast_node!(LogicalExpression, LOGICAL_EXPRESSION);
    ast_node!(MetaProperty, META_PROPERTY);
    ast_node!(MethodDefinition, METHOD_DEFINITION);
    ast_node!(RestElement, REST_ELEMENT);
    ast_node!(ReturnStatement, RETURN_STATEMENT);
    ast_node!(SequenceExpression, SEQUENCE_EXPRESSION);
    ast_node!(SpreadElement, SPREAD_ELEMENT);
    ast_node!(Super, SUPER);
    ast_node!(SwitchCase, SWITCH_CASE);
    ast_node!(SwitchStatement, SWITCH_STATEMENT);
    ast_node!(TaggedTemplateExpression, TAGGED_TEMPLATE_EXPRESSION);
    ast_node!(TemplateLiteral, TEMPLATE_LITERAL);
    ast_node!(ThisExpression, THIS_EXPRESSION);
    ast_node!(ThrowStatement, THROW_STATEMENT);
    ast_node!(TryStatement, TRY_STATEMENT);
    ast_node!(UnaryExpression, UNARY_EXPRESSION);
    ast_node!(UpdateExpression, UPDATE_EXPRESSION);
    ast_node!(VariableDeclarator, VARIABLE_DECLARATOR);
    ast_node!(WhileStatement, WHILE_STATEMENT);
    ast_node!(WithStatement, WITH_STATEMENT);
    ast_node!(YieldExpression, YIELD_EXPRESSION);
}

pub mod syntax_kind {
    use crate::syntax_kind::JAVASCRIPT;
    use web_grammars_utils::SyntaxKind;

    pub const ANONYMOUS_DEFAULT_EXPORTED_CLASS_DECLARATION: SyntaxKind = JAVASCRIPT.syntax_kind(205);
    pub const ANONYMOUS_DEFAULT_EXPORTED_FUNCTION_DECLARATION: SyntaxKind = JAVASCRIPT.syntax_kind(206);
    pub const ARRAY_PATTERN: SyntaxKind = JAVASCRIPT.syntax_kind(207);
    pub const ARROW_FUNCTION_EXPRESSION: SyntaxKind = JAVASCRIPT.syntax_kind(208);
    pub const ASSIGNMENT_PATTERN: SyntaxKind = JAVASCRIPT.syntax_kind(209);
    pub const ASSIGNMENT_PROPERTY: SyntaxKind = JAVASCRIPT.syntax_kind(210);
    pub const AWAIT_EXPRESSION: SyntaxKind = JAVASCRIPT.syntax_kind(211);
    pub const BINARY_EXPRESSION: SyntaxKind = JAVASCRIPT.syntax_kind(212);
    pub const BLOCK_STATEMENT: SyntaxKind = JAVASCRIPT.syntax_kind(213);
    pub const BREAK_STATEMENT: SyntaxKind = JAVASCRIPT.syntax_kind(214);
    pub const CLASS_BODY: SyntaxKind = JAVASCRIPT.syntax_kind(215);
    pub const CLASS_DECLARATION: SyntaxKind = JAVASCRIPT.syntax_kind(216);
    pub const CLASS_EXPRESSION: SyntaxKind = JAVASCRIPT.syntax_kind(217);
    pub const CONDITIONAL_EXPRESSION: SyntaxKind = JAVASCRIPT.syntax_kind(218);
    pub const CONTINUE_STATEMENT: SyntaxKind = JAVASCRIPT.syntax_kind(219);
    pub const DEBUGGER_STATEMENT: SyntaxKind = JAVASCRIPT.syntax_kind(220);
    pub const DIRECTIVE: SyntaxKind = JAVASCRIPT.syntax_kind(221);
    pub const DO_WHILE_STATEMENT: SyntaxKind = JAVASCRIPT.syntax_kind(222);
    pub const EMPTY_STATEMENT: SyntaxKind = JAVASCRIPT.syntax_kind(223);
    pub const EXPORT_ALL_DECLARATION: SyntaxKind = JAVASCRIPT.syntax_kind(224);
    pub const EXPORT_DEFAULT_DECLARATION: SyntaxKind = JAVASCRIPT.syntax_kind(225);
    pub const EXPORT_NAMED_DECLARATION: SyntaxKind = JAVASCRIPT.syntax_kind(226);
    pub const EXPORT_SPECIFIER: SyntaxKind = JAVASCRIPT.syntax_kind(227);
    pub const EXPRESSION_STATEMENT: SyntaxKind = JAVASCRIPT.syntax_kind(228);
    pub const FOR_IN_STATEMENT: SyntaxKind = JAVASCRIPT.syntax_kind(229);
    pub const FOR_STATEMENT: SyntaxKind = JAVASCRIPT.syntax_kind(230);
    pub const FUNCTION_DECLARATION: SyntaxKind = JAVASCRIPT.syntax_kind(231);
    pub const FUNCTION_EXPRESSION: SyntaxKind = JAVASCRIPT.syntax_kind(232);
    pub const IDENTIFIER: SyntaxKind = JAVASCRIPT.syntax_kind(233);
    pub const IF_STATEMENT: SyntaxKind = JAVASCRIPT.syntax_kind(234);
    pub const IMPORT_DECLARATION: SyntaxKind = JAVASCRIPT.syntax_kind(235);
    pub const IMPORT_DEFAULT_SPECIFIER: SyntaxKind = JAVASCRIPT.syntax_kind(236);
    pub const IMPORT_NAMESPACE_SPECIFIER: SyntaxKind = JAVASCRIPT.syntax_kind(237);
    pub const IMPORT_SPECIFIER: SyntaxKind = JAVASCRIPT.syntax_kind(238);
    pub const LABELED_STATEMENT: SyntaxKind = JAVASCRIPT.syntax_kind(239);
    pub const LITERAL: SyntaxKind = JAVASCRIPT.syntax_kind(240);
    pub const LOGICAL_EXPRESSION: SyntaxKind = JAVASCRIPT.syntax_kind(241);
    pub const META_PROPERTY: SyntaxKind = JAVASCRIPT.syntax_kind(242);
    pub const METHOD_DEFINITION: SyntaxKind = JAVASCRIPT.syntax_kind(243);
    pub const REST_ELEMENT: SyntaxKind = JAVASCRIPT.syntax_kind(244);
    pub const RETURN_STATEMENT: SyntaxKind = JAVASCRIPT.syntax_kind(245);
    pub const SEQUENCE_EXPRESSION: SyntaxKind = JAVASCRIPT.syntax_kind(246);
    pub const SPREAD_ELEMENT: SyntaxKind = JAVASCRIPT.syntax_kind(247);
    pub const SUPER: SyntaxKind = JAVASCRIPT.syntax_kind(248);
    pub const SWITCH_CASE: SyntaxKind = JAVASCRIPT.syntax_kind(249);
    pub const SWITCH_STATEMENT: SyntaxKind = JAVASCRIPT.syntax_kind(250);
    pub const TAGGED_TEMPLATE_EXPRESSION: SyntaxKind = JAVASCRIPT.syntax_kind(251);
    pub const TEMPLATE_LITERAL: SyntaxKind = JAVASCRIPT.syntax_kind(252);
    pub const THIS_EXPRESSION: SyntaxKind = JAVASCRIPT.syntax_kind(253);
    pub const THROW_STATEMENT: SyntaxKind = JAVASCRIPT.syntax_kind(254);
    pub const TRY_STATEMENT: SyntaxKind = JAVASCRIPT.syntax_kind(255);
    pub const UNARY_EXPRESSION: SyntaxKind = JAVASCRIPT.syntax_kind(256);
    pub const UPDATE_EXPRESSION: SyntaxKind = JAVASCRIPT.syntax_kind(257);
    pub const VARIABLE_DECLARATOR: SyntaxKind = JAVASCRIPT.syntax_kind(258);
    pub const WHILE_STATEMENT: SyntaxKind = JAVASCRIPT.syntax_kind(259);
    pub const WITH_STATEMENT: SyntaxKind = JAVASCRIPT.syntax_kind(260);
    pub const YIELD_EXPRESSION: SyntaxKind = JAVASCRIPT.syntax_kind(261);
}
