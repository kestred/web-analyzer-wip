use crate::{Location, Token};
use rowan::{GreenNodeBuilder, SmolStr, SyntaxKind, SyntaxNode, TextRange, TextUnit, TreeArc};
use std::fmt::Debug;

pub struct TextTreeSink<'a, E: 'static + Debug + Send + Sync> {
    text: &'a str,
    tokens: &'a [Token],
    text_pos: TextUnit,
    token_pos: usize,
    builder: GreenNodeBuilder,
    errors: Vec<(E, Location)>,
}

impl<'a, E: 'static + Debug + Send + Sync> TextTreeSink<'a, E> {
    pub fn new(text: &'a str, tokens: &'a [Token]) -> TextTreeSink<'a, E> {
        TextTreeSink {
            text,
            tokens,
            text_pos: 0.into(),
            token_pos: 0,
            builder: GreenNodeBuilder::new(),
            errors: Vec::new(),
        }
    }

    pub fn error(&mut self, error: E) {
        self.errors.push((error, self.text_pos.into()))
    }

    pub fn span<F>(&mut self, kind: SyntaxKind, num_tokens: usize, skip: F)
    where
        F: Fn(SyntaxKind) -> bool
    {
        while let Some(&token) = self.tokens.get(self.token_pos) {
            if !skip(token.kind) {
                break;
            }
            self.advance(token.kind, token.len, 1);
        }
        let len = self.tokens[self.token_pos..self.token_pos + num_tokens]
            .iter()
            .map(|it| it.len)
            .sum::<TextUnit>();
        self.advance(kind, len, num_tokens);
    }

    pub fn start_node<F>(&mut self, kind: SyntaxKind, skip: F)
    where
        F: Fn(SyntaxKind) -> bool
    {
        while let Some(&token) = self.tokens.get(self.token_pos) {
            if !skip(token.kind) {
                break;
            }
            self.advance(token.kind, token.len, 1);
        }
        self.builder.start_node(kind);
    }

    pub fn finish_node(&mut self) {
        self.builder.finish_node();
    }

    pub fn finish(self) -> TreeArc<SyntaxNode> {
        let green = self.builder.finish();
        SyntaxNode::new(green, Some(Box::new(self.errors)))
    }

    fn advance(&mut self, kind: SyntaxKind, len: TextUnit, num_tokens: usize) {
        let range = TextRange::offset_len(self.text_pos, len);
        let text: SmolStr = self.text[range].into();
        self.text_pos += len;
        self.token_pos += num_tokens;
        self.builder.token(kind, text);
    }
}