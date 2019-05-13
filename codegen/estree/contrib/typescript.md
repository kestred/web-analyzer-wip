This document specifies custom extensions to the core ESTree AST types to support the TypeScript grammar.

# Expressions

## AsExpression

```js
interface TSAsExpression <: Expression {
    type: "TSAsExpression";
    expression: Expression;
    typeAnnotation: TypeAnnotation;
}
```

## NonNullExpression

```js
interface TSNonNullExpression <: Expression {
    type: "TSNonNullExpression";
    expression: Expression;
    typeAnnotation: TypeAnnotation;
}
```
