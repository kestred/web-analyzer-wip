//! An example of how to implement parsing using the utils.
//!
//! ```ignore
//! use web_grammars_utils::{Lexer, Parser, SyntaxKind};
//! use web_grammars_utils::grammar::*;
//!
//! struct MyLexer { ... }
//!
//! impl Lexer for MyLexer { ... }
//!
//! fn my_grammar(p: &mut Parser) -> SyntaxKind { ... }
//!
//! fn parse(text: &str) -> SyntaxNode {
//!     let tokens = MyLexer::new().tokenize(text);
//!     let parser = Parser::new(text, &tokens);
//!     parser.parse(my_grammar)
//! }
//! ```

mod token_source;
mod tree_sink;

use crate::grammar::Grammar;
use crate::lexer::Token;
use crate::syntax_kind::{COMMENT, EOF, TOMBSTONE, WHITESPACE};
use rowan::{SyntaxKind, SyntaxNode, TreeArc};
use std::fmt::Debug;

use self::token_source::{TokenSource, TextTokenSource};
use self::tree_sink::TextTreeSink;

pub trait ParseError: 'static + From<String> + Debug + Send + Sync {}
impl<E> ParseError for E where E: 'static + From<String> + Debug + Send + Sync {}

pub struct Parser<'a, E: 'static + Debug + Send + Sync = String> {
    events: Vec<Event<E>>,
    source_pos: usize,
    source: TextTokenSource<'a>,
    sink: TextTreeSink<'a, E>,
    preserve_whitespace: bool,
}

impl<'a, E: ParseError> Parser<'a, E> {
    pub fn new(text: &'a str, tokens: &'a [Token], preserve_whitespace: bool) -> Parser<'a, E> {
        Parser {
            events: Vec::new(),
            source_pos: 0,
            source: TextTokenSource::extract(text, tokens, if preserve_whitespace { skip_comments } else { skip_whitespace }),
            sink: TextTreeSink::new(text, tokens),
            preserve_whitespace,
        }
    }

    /// Parse the grammar completely and return the result root syntax node.
    pub fn parse<G: Grammar<E>>(mut self, grammar: &G) -> TreeArc<SyntaxNode> {
        self.eval(grammar);
        self.finish()
    }

    /// Evaluate a single grammar rule.
    ///
    /// This is intended to be called within a `Grammar` parsing function
    /// to begin parsing a sub_grammar.
    pub fn eval<G: Grammar<E>>(&mut self, grammar: &G) -> SyntaxKind {
        let start = self.start_marker();
        let kind = grammar.parse(self);
        self.complete_marker(start, kind);
        kind
    }

    /// [Internal API]
    /// Checks if the current token is `kind`.
    pub(crate) fn at(&self, kind: SyntaxKind) -> bool {
        self.current() == kind
    }

    /// [Internal API]
    /// Lookahead returning the kind of the next nth token.
    pub(crate) fn nth(&self, n: usize) -> SyntaxKind {
        self.source.token_kind(self.source_pos + n)
    }

    /// [Internal API]
    /// Returns the kind of the current token.
    /// If parser has already reached the end of input,
    /// the special `EOF` kind is returned.
    pub(crate) fn current(&self) -> SyntaxKind {
        self.nth(0)
    }

    /// [Internal API]
    /// Records the current state of the parser
    /// so that it can be restored later with `Parser::rollback`
    /// if it is necessary to backtrack.
    pub(crate) fn checkpoint(&self) -> Checkpoint {
        Checkpoint {
            event_len: self.events.len(),
            source_pos: self.source_pos
        }
    }

    /// [Internal API]
    /// Backtracks to the checkpoint, undoing any events that were emitted.
    pub(crate) fn rollback(&mut self, checkpoint: Checkpoint) {
        self.events.truncate(checkpoint.event_len);
        self.source_pos = checkpoint.source_pos;
    }


    /// [Internal API]
    /// Consumes the checkpoint.
    /// If there were any errors since the checkpoint began, restores from the checkpoint.
    pub(crate) fn commit(&mut self, checkpoint: Checkpoint) -> Result<(), E> {
        let error_idx = self.events[checkpoint.event_len..]
            .iter()
            .enumerate()
            .find(|(_key, val)| Event::is_err(val))
            .map(|(key, _val)| key);
        if let Some(idx) = error_idx {
            let error = match self.events.remove(idx) {
                Event::Error { error } => error,
                _ => unreachable!()
            };
            self.rollback(checkpoint);
            Err(error)
        } else {
            Ok(())
        }
    }

    /// [Internal API]
    /// Emit error for the current node in the parse tree
    pub(crate) fn error<T: Into<E>>(&mut self, error: T) {
        self.events.push(Event::Error { error: error.into() });
    }

    /// [Internal API]
    /// Consume the next token if `kind` matches.
    pub(crate) fn eat(&mut self, kind: SyntaxKind) -> bool {
        if !self.at(kind) {
            return false;
        }
        self.bump();
        true
    }

    /// [Internal API]
    /// Advances the parser by one token unconditionally.
    pub(crate) fn bump(&mut self) {
        let kind = self.nth(0);
        if kind == EOF {
            return;
        }
        self.advance(kind, 1);
    }

    /// Starts a new node in the syntax tree. All nodes and tokens
    /// consumed between the `start` and the corresponding `Parser::complete_marker`
    /// belong to the same node.
    fn start_marker(&mut self) -> Marker {
        let start = Marker { pos: self.events.len() };
        self.events.push(Event::Start { kind: TOMBSTONE });
        start
    }

    /// Finishes the syntax tree node and assigns `kind` to it.
    fn complete_marker(&mut self, marker: Marker, kind: SyntaxKind) {
        debug_assert!(self.events.len() - 1 > marker.pos, "Expected node to span at least 1 token");
        match self.events[marker.pos] {
            Event::Start { kind: ref mut slot, .. } => {
                *slot = kind;
            }
            _ => unreachable!(),
        }
        self.events.push(Event::Finish { });
    }

    /// Advance the parser.
    fn advance(&mut self, kind: SyntaxKind, len: usize) {
        self.source_pos += len;
        self.events.push(Event::Span { kind, len });
    }

    /// Consume the parser and apply it's events to create the syntax tree.
    fn finish(mut self) -> TreeArc<SyntaxNode> {
        for op in self.events {
            match op {
                Event::Start { kind } if kind == TOMBSTONE => {}
                Event::Start { kind } => self.sink.start_node(kind, if self.preserve_whitespace { skip_comments } else { skip_whitespace }),
                Event::Finish => self.sink.finish_node(),
                Event::Error { error } => self.sink.error(error),
                Event::Span { kind, len } => self.sink.span(kind, len, if self.preserve_whitespace { skip_comments } else { skip_whitespace }),
            }
        }
        self.sink.finish()
    }
}

/// This method defines the default ignore behavior.
fn skip_whitespace(k: SyntaxKind) -> bool {
    k == WHITESPACE
}
fn skip_comments(k: SyntaxKind) -> bool {
    k == COMMENT
}

/// The `Parser` builds up a list of `Event`s which are
/// then converted to a tree structure at the end of parsing.
///
/// This allows for more fine-grained control of parsing in the middle.
enum Event<E> {
    Start { kind: SyntaxKind },
    Finish,
    Error { error: E },
    Span { kind: SyntaxKind, len: usize },
}

impl<E> Event<E> {
    fn is_err(&self) -> bool {
        match self {
            Event::Error { .. } => true,
            _ => false,
        }
    }
}

/// See `Parser::start_marker`.
pub(crate) struct Marker {
    /// An offset into the parser's events
    pos: usize
}

/// See `Parser::checkpoint`.
pub(crate) struct Checkpoint {
    /// The number of events processed by the parser when the checkpoint was created
    event_len: usize,
    /// The offset into the token source when the checkpoint was created
    source_pos: usize
}