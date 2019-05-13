use crate::grammar;
use crate::syntax_kind::{self, *};
use code_grammar::{ast_node, AstNode, Lexer, Location, Parser, SyntaxError, SyntaxNode, TreeArc};
use code_grammar::parser::ParseConfig;
use javascript_grammar::lexer::JavascriptLexer;

pub use javascript_grammar::ast::*;

// N.B. shadow `javascript_grammar::ast::Program`
ast_node!(Program, PROGRAM);
impl Program {
    pub fn type_(&self) -> &'static str {
        "Program"
    }
}

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

// TODO: Match these AST types to typescript's real types;
//       these syntax kinds where defined in the initial implementation just to get things working
/*
    INTERFACE_DECLARATION 201
    INTERFACE_PROPERTY 202
    ALIAS_DECLARATION 203
    ENUM_DECLARATION 204
    ENUM_VARIANT 205

    GENERIC_TYPE_EXPR 210
    ARRAY_TYPE_EXPR 211
    UNION_TYPE_EXPR 212
    INTERSECTION_TYPE_EXPR 213
    CONDITIONAL_TYPE_EXPR 214
    INTERFACE_TYPE_EXPR 215
    FUNCTION_TYPE_EXPR 216
    TUPLE_TYPE_EXPR 217
    TYPEOF_TYPE_EXPR 218

    TYPE_ARGUMENT 230
*/

// TODO: These `AstNodes` are defined by `ts-estree`
// See https://github.com/typescript-eslint/typescript-eslint/blob/master/packages/typescript-estree/src/ts-estree/ast-node-types.ts
/*
    TSAbstractClassProperty = 'TSAbstractClassProperty',
    TSAbstractKeyword = 'TSAbstractKeyword',
    TSAbstractMethodDefinition = 'TSAbstractMethodDefinition',
    TSAnyKeyword = 'TSAnyKeyword',
    TSArrayType = 'TSArrayType',
    TSAsExpression = 'TSAsExpression',
    TSAsyncKeyword = 'TSAsyncKeyword',
    TSBooleanKeyword = 'TSBooleanKeyword',
    TSBigIntKeyword = 'TSBigIntKeyword',
    TSConditionalType = 'TSConditionalType',
    TSConstructorType = 'TSConstructorType',
    TSCallSignatureDeclaration = 'TSCallSignatureDeclaration',
    TSClassImplements = 'TSClassImplements',
    TSConstructSignatureDeclaration = 'TSConstructSignatureDeclaration',
    TSDeclareKeyword = 'TSDeclareKeyword',
    TSDeclareFunction = 'TSDeclareFunction',
    TSEmptyBodyFunctionExpression = 'TSEmptyBodyFunctionExpression',
    TSEnumDeclaration = 'TSEnumDeclaration',
    TSEnumMember = 'TSEnumMember',
    TSExportAssignment = 'TSExportAssignment',
    TSExportKeyword = 'TSExportKeyword',
    TSExternalModuleReference = 'TSExternalModuleReference',
    TSImportType = 'TSImportType',
    TSInferType = 'TSInferType',
    TSLiteralType = 'TSLiteralType',
    TSIndexedAccessType = 'TSIndexedAccessType',
    TSIndexSignature = 'TSIndexSignature',
    TSInterfaceBody = 'TSInterfaceBody',
    TSInterfaceDeclaration = 'TSInterfaceDeclaration',
    TSInterfaceHeritage = 'TSInterfaceHeritage',
    TSImportEqualsDeclaration = 'TSImportEqualsDeclaration',
    TSFunctionType = 'TSFunctionType',
    TSMethodSignature = 'TSMethodSignature',
    TSModuleBlock = 'TSModuleBlock',
    TSModuleDeclaration = 'TSModuleDeclaration',
    TSNamespaceExportDeclaration = 'TSNamespaceExportDeclaration',
    TSNonNullExpression = 'TSNonNullExpression',
    TSNeverKeyword = 'TSNeverKeyword',
    TSNullKeyword = 'TSNullKeyword',
    TSNumberKeyword = 'TSNumberKeyword',
    TSMappedType = 'TSMappedType',
    TSObjectKeyword = 'TSObjectKeyword',
    TSParameterProperty = 'TSParameterProperty',
    TSPrivateKeyword = 'TSPrivateKeyword',
    TSPropertySignature = 'TSPropertySignature',
    TSProtectedKeyword = 'TSProtectedKeyword',
    TSPublicKeyword = 'TSPublicKeyword',
    TSQualifiedName = 'TSQualifiedName',
    TSQuestionToken = 'TSQuestionToken',
    TSReadonlyKeyword = 'TSReadonlyKeyword',
    TSRestType = 'TSRestType',
    TSStaticKeyword = 'TSStaticKeyword',
    TSStringKeyword = 'TSStringKeyword',
    TSSymbolKeyword = 'TSSymbolKeyword',
    TSThisType = 'TSThisType',
    TSTypeAnnotation = 'TSTypeAnnotation',
    TSTypeAliasDeclaration = 'TSTypeAliasDeclaration',
    TSTypeAssertion = 'TSTypeAssertion',
    TSTypeLiteral = 'TSTypeLiteral',
    TSTypeOperator = 'TSTypeOperator',
    TSTypeParameter = 'TSTypeParameter',
    TSTypeParameterDeclaration = 'TSTypeParameterDeclaration',
    TSTypeParameterInstantiation = 'TSTypeParameterInstantiation',
    TSTypePredicate = 'TSTypePredicate',
    TSTypeReference = 'TSTypeReference',
    TSTypeQuery = 'TSTypeQuery',
    TSIntersectionType = 'TSIntersectionType',
    TSTupleType = 'TSTupleType',
    TSOptionalType = 'TSOptionalType',
    TSParenthesizedType = 'TSParenthesizedType',
    TSUnionType = 'TSUnionType',
    TSUndefinedKeyword = 'TSUndefinedKeyword',
    TSUnknownKeyword = 'TSUnknownKeyword',
    TSVoidKeyword = 'TSVoidKeyword',
*/
