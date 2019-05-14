//! An example of how to implement parsing using the utils.
//!
//! ```rust
//! use code_grammar::{Lexer, Parser, Scanner, SyntaxKind, SyntaxLanguage, TreeNode};
//! use code_grammar::parser::Continue;
//! use code_grammar::syntax_kind::*;
//!
//! struct MyLexer;
//!
//! impl Lexer for MyLexer {
//!     fn scan(&mut self, c: char, s: &mut Scanner) -> SyntaxKind {
//!         unimplemented!()
//!     }
//! }
//!
//! const MY_LANGUAGE: SyntaxLanguage = SyntaxLanguage(1);
//! const AST_ROOT: SyntaxKind = MY_LANGUAGE.syntax_kind(1);
//!
//! fn my_grammar(p: &mut Parser) -> Option<Continue> {
//!     let start = p.start();
//!     p.expect(EOF);
//!     p.complete(start, AST_ROOT);
//!     None
//! }
//!
//! /// Parse a syntax tree from text
//! pub fn parse(text: &str) -> TreeNode {
//!     let tokens = MyLexer.tokenize(text);
//!     let parser = Parser::new((text, &tokens).into(), Default::default());
//!     let (root, _remainder) = parser.parse(&my_grammar);
//!     root
//! }
//! ```

mod token_set;
mod token_source;
mod tree_sink;

use crate::syntax_kind::*;
use rowan::SyntaxKind;
use std::cell::Cell;
use std::fmt::Debug;
use std::rc::Rc;

pub use self::token_set::*;
pub use self::token_source::*;
pub use self::tree_sink::*;

pub trait ParseError: 'static + From<String> + Debug + Send + Sync {}
impl<E> ParseError for E where E: 'static + From<String> + Debug + Send + Sync {}

#[derive(Debug)]
pub struct ParseConfig {
    pub debug_repr: fn(SyntaxKind) -> Option<SyntaxKindMeta>,
    pub max_rollback_size: u16,
    pub preserve_comments: bool,
    pub preserve_whitespace: bool,
}

impl ParseConfig {
    fn skip_predicate(&self) -> fn(SyntaxKind) -> bool {
        fn skip_everything(k: SyntaxKind) -> bool {
            k == WHITESPACE || k == COMMENT
        }
        fn skip_whitespace(k: SyntaxKind) -> bool {
            k == WHITESPACE
        }
        fn skip_comments(k: SyntaxKind) -> bool {
            k == COMMENT
        }
        fn skip_none(_: SyntaxKind) -> bool {
            false
        }

        match (self.preserve_whitespace, self.preserve_comments) {
            (false, false) => skip_everything,
            (false, true) => skip_whitespace,
            (true, false) => skip_comments,
            (true, true) => skip_none,
        }
    }
}


impl Default for ParseConfig {
    fn default() -> ParseConfig {
        ParseConfig {
            debug_repr: default::as_debug_repr,
            max_rollback_size: 32,
            preserve_comments: false,
            preserve_whitespace: false,
        }
    }
}

pub struct Parser<'a, 'b, E: 'static + Debug + Send + Sync = String> {
    config: ParseConfig,
    events: Vec<Event<E>>,
    source_pos: usize,
    source: TextTokenSource<'a>,
    sink: TextTreeSink<'a, 'b, E>,
    steps: Cell<u32>,
    checkpoints: Rc<Cell<u32>>,
}

impl<'a, 'b, E: ParseError> Parser<'a, 'b, E> {
    pub fn new(input: TokenInput<'a, 'b>, config: ParseConfig) -> Parser<'a, 'b, E> {
        let skip_predicate = config.skip_predicate();
        Parser {
            config,
            events: Vec::new(),
            source_pos: 0,
            source: TextTokenSource::extract(input, skip_predicate),
            sink: TextTreeSink::new(input),
            steps: Cell::new(0),
            checkpoints: Rc::new(Cell::new(0)),
        }
    }

    pub fn parse<F>(mut self, grammar: F) -> (TreeNode, TokenInput<'a, 'b>)
    where
        F: Fn(&mut Parser<'a, 'b, E>) -> Option<Continue>
    {
        grammar(&mut self);
        self.finalize()
    }

    /// Consume the parser and build a syntax tree.
    pub fn finalize(mut self) -> (TreeNode, TokenInput<'a, 'b>) {
        for op in self.events {
            match op {
                Event::Error { error } => self.sink.error(error),
                Event::StartNode { kind } if kind == TOMBSTONE => {}
                Event::StartNode { kind } => {
                    // eprintln!("start {} {{", (self.config.debug_repr)(kind).map(|k| k.name).unwrap_or("UNKNOWN"));
                    self.sink.start_node(kind, self.config.skip_predicate())
                }
                Event::CompleteNode => {
                    // eprintln!("}}");
                    self.sink.complete_node()
                }
                Event::Span { kind, len } => {
                    // eprintln!("  @{}", (self.config.debug_repr)(kind).map(|k| k.name).unwrap_or("_"));
                    self.sink.span(kind, len, self.config.skip_predicate())
                }
            }
        }
        self.sink.finalize()
    }


    /// Checks if the current token is a specified keyword.
    ///
    /// Does not consider the SyntaxKind (e.g. to detect contextual keywords)
    pub fn at_keyword(&self, kw: &str) -> bool {
        self.source.is_keyword(self.source_pos, kw)
    }

    /// Checks if the next token is separated by ignored tokens (e.g. whitespace or comments).
    pub fn at_whitespace(&self) -> bool {
        !self.source.is_token_joint_to_next(self.source_pos)
    }

    /// Checks if the next token on the same line as the current token.
    pub fn at_line_terminator(&self) -> bool {
        !self.source.is_token_inline_to_next(self.source_pos)
    }

    /// Checks if the current token is `kind`.
    pub fn at(&self, kind: SyntaxKind) -> bool {
        self.current() == kind
    }

    /// Checks if the current token is `kind`.
    pub fn at_ts(&self, ts: &TokenSet) -> bool {
        ts.contains(&self.current())
    }

    /// Lookahead returning the kind of the next nth token.
    pub fn nth(&self, n: usize) -> SyntaxKind {
        let steps = self.steps.get();
        assert!(steps <= 10_000_000, "the parser seems stuck");
        self.steps.set(steps + 1);
        self.source.token_kind(self.source_pos + n)
    }

    /// Returns the kind of the current token.
    /// If parser has already reached the end of input,
    /// the special `EOF` kind is returned.
    pub fn current(&self) -> SyntaxKind {
        self.nth(0)
    }

    /// Records the current state of the parser so that it can either:
    ///
    /// - Be passed to `Parser::commit` which may possibly bactrack.
    /// - Be used in relative comparisons against a marker.
    ///
    pub fn checkpoint(&self, allow_rollback: bool) -> Checkpoint {
        let incr = self.checkpoints.get();
        self.checkpoints.set(incr + 1);
        Checkpoint {
            event_pos: self.events.len(),
            source_pos: self.source_pos,
            branch_pos: None,
            rollback: if allow_rollback {
                Rollback::Allow { checkpoints: Rc::clone(&self.checkpoints) }
            } else {
                Rollback::Prevent
            },
        }
    }

    /// Consumes the checkpoint.
    /// If there were any errors since the checkpoint began, restores from the checkpoint.
    ///
    /// If the current branch is beyond the max lookahead (configured with `max_rollback_size`)
    /// then commit will instead return `None` to indicate that parsing should not continue;
    /// in this case, no backtracking will occur.
    pub fn commit(&mut self, checkpoint: Checkpoint) -> Option<Result<(), E>> {
        let error_idx = self.events[checkpoint.event_pos..]
            .iter()
            .enumerate()
            .find(|(_key, val)| Event::is_err(val))
            .map(|(key, _val)| key + checkpoint.event_pos);
        if let Some(idx) = error_idx {
            let distance = self.source_pos - checkpoint.source_pos;
            if checkpoint.allows_rollback() && distance <= self.config.max_rollback_size as usize {
                let error = match self.events.remove(idx) {
                    Event::Error { error } => error,
                    _ => unreachable!()
                };
                // eprintln!("rollback of size {} at {}: {:?}", self.source_pos - checkpoint.source_pos, (self.config.debug_repr)(self.current()).unwrap().name, error);
                self.rollback(checkpoint);
                Some(Err(error))
            } else {
                None
            }
        } else {
            Some(Ok(()))
        }
    }

    /// Backtracks to the checkpoint, undoing any events that were emitted.
    fn rollback(&mut self, checkpoint: Checkpoint) {
        assert!(checkpoint.allows_rollback(), "attempted to rollback invalid checkpoint");
        assert!(self.source_pos >= checkpoint.source_pos, "attempted to rollback expired checkpoint");
        assert!(self.config.max_rollback_size as usize >= self.source_pos - checkpoint.source_pos, "a rollback exceeded the max rollback size");
        if let Some(branch_pos) = checkpoint.branch_pos {
            match self.events[branch_pos] {
                Event::StartNode { kind: ref mut slot, .. } => {
                    *slot = TOMBSTONE;
                }
                _ => unreachable!(),
            }
        }
        self.events.truncate(checkpoint.event_pos);
        self.source_pos = checkpoint.source_pos;
    }

    /// Emit error for the current node in the parse tree
    pub fn error<T: Into<E>>(&mut self, error: T) -> Option<Continue> {
        self.events.push(Event::Error { error: error.into() });
        if self.checkpoints.get() > 0 {
            return None;
        }
        Some(Continue)
    }

    /// Consume the next token if it is `kind` or emit an error otherwise.
    pub fn expect(&mut self, kind: SyntaxKind) -> Option<Continue> {
        if self.eat(kind) {
            Some(Continue)
        } else {
            self.expected(kind)
        }
    }

    /// Emit an `unexpected` error message, with a single expected kind.
    pub fn expected(&mut self, kind: SyntaxKind) -> Option<Continue> {
        let unexpected: &str = self.as_str(self.current());
        self.error(format!("unexpected '{}', expected '{}'", unexpected, self.as_str(kind)))
    }

    /// The same as `expected` but with a contextual description of what syntax is being parsed.
    pub fn expected_in(&mut self, rule: &str, kind: SyntaxKind) -> Option<Continue> {
        let unexpected: &str = self.as_str(self.current());
        self.error(format!("unexpected '{}' in '{}', expected '{}'", unexpected, rule, self.as_str(kind)))
    }

    /// Consume the next token if it is one of `kinds` or emit an error otherwise.
    pub fn expect_ts(&mut self, kinds: &TokenSet) -> Option<Continue> {
        if self.at_ts(kinds) {
            self.bump();
            Some(Continue)
        } else {
            self.expected_ts(kinds)
        }
    }

    /// Emit an `unexpected` error message, with an expected set of syntax kinds.
    pub fn expected_ts(&mut self, expected: &TokenSet) -> Option<Continue> {
        let unexpected: &str = self.as_str(self.current());
        let expected: Vec<&str> = expected.tokens().map(|k| self.as_str(*k)).collect();
        self.error(format!("unexpected '{}', expected one of: {:?}", unexpected, expected))
    }

    /// The same as `expected_ts` but with a contextual description of what syntax is being parsed.
    pub fn expected_ts_in(&mut self, rule: &str, expected: &TokenSet) -> Option<Continue> {
        let unexpected: &str = self.as_str(self.current());
        let expected: Vec<&str> = expected.tokens().map(|k| self.as_str(*k)).collect();
        self.error(format!("unexpected '{}' in '{}', expected one of: {:?}", unexpected, rule, expected))
    }

    /// Consume the next token if `kind` matches.
    pub fn eat(&mut self, kind: SyntaxKind) -> bool {
        if !self.at(kind) {
            return false;
        }
        self.bump();
        true
    }

    /// Advances the parser by one token unconditionally.
    pub fn bump(&mut self) {
        let kind = self.nth(0);
        if kind == EOF {
            return;
        }
        self.advance(kind, 1);
    }

    /// Starts a new node in the syntax tree. All nodes and tokens
    /// consumed between the `start` and the corresponding `Parser::complete_marker`
    /// belong to the same node.
    pub fn start(&mut self) -> Marker {
        let start = Marker { pos: self.events.len() };
        self.events.push(Event::StartNode { kind: TOMBSTONE });
        start
    }

    /// Finishes the syntax tree node and assigns `kind` to it.
    pub fn complete(&mut self, marker: Marker, kind: SyntaxKind) {
        debug_assert!(self.events.len() - 1 > marker.pos, "expected node to span at least 1 token");
        match self.events[marker.pos] {
            Event::StartNode { kind: ref mut slot, .. } => {
                *slot = kind;
            }
            _ => unreachable!(),
        }
        self.events.push(Event::CompleteNode);
    }

    /// Finish a syntax tree node, but insert a new placeholder before it
    /// nesting the completed node within a new yet to be completed node.
    ///
    /// This supports building a tree with correct associativity in a left recursive garmmar.
    pub fn complete_and_wrap(&mut self, marker: &Marker, kind: SyntaxKind) {
        debug_assert!(self.events.len() - 1 > marker.pos, "expected node to span at least 1 token");
        match self.events[marker.pos] {
            Event::StartNode { kind: ref mut slot, .. } => {
                *slot = kind;
            }
            _ => unreachable!(),
        }
        self.events.insert(marker.pos, Event::StartNode { kind: TOMBSTONE });
        self.events.push(Event::CompleteNode);
    }

    /// Advance the parser.
    fn advance(&mut self, kind: SyntaxKind, len: usize) {
        self.source_pos += len;
        self.events.push(Event::Span { kind, len });
    }

    /// Get a debug string representing the syntax kind.
    fn as_str(&self, kind: SyntaxKind) -> &'static str {
        (self.config.debug_repr)(kind)
            .map(|info| info.canonical.unwrap_or(info.name) )
            .unwrap_or("<anonymous token>")
    }
}

/// The `Parser` builds up a list of `Event`s which are
/// then converted to a tree structure at the end of parsing.
///
/// This allows for more fine-grained control of parsing in the middle.
enum Event<E> {
    StartNode { kind: SyntaxKind },
    CompleteNode,
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

/// A zero-sized type returned by `error` to indicate that parsing should
/// continue (to produce a partial parse tree) rather than returning.
///
/// Used within `Option` to control flow with `?`.
pub struct Continue;

/// See `Parser::start_marker`.
pub struct Marker {
    /// An offset into the parser's events
    pos: usize
}

/// See `Parser::checkpoint`.
pub struct Checkpoint {
    /// The number of events processed by the parser when the checkpoint was created
    event_pos: usize,
    /// The offset into the token source when the checkpoint was created
    source_pos: usize,
    /// The position of the marker to undo if this checkpoint is rolled-back.
    branch_pos: Option<usize>,
    /// Whether the checkpoint allows rollback or not
    rollback: Rollback,
}

impl Checkpoint {
    /// Create a copy of a node start position that will be cleared
    /// if `rollback` is called with this checkpoint.
    pub fn branch(&mut self, marker: &Marker) -> Marker {
        assert!(self.branch_pos.is_none(), "attempted to branch twice with the same checkpoint");
        self.branch_pos = Some(marker.pos);
        Marker { pos: marker.pos }
    }

    fn allows_rollback(&self) -> bool {
        match self.rollback {
            Rollback::Allow { .. } => true,
            Rollback::Prevent => false
        }
    }
}

enum Rollback {
    Prevent,
    Allow { checkpoints: Rc<Cell<u32>>,  }
}

impl Drop for Rollback {
    fn drop(&mut self) {
        match self {
            Rollback::Prevent => (),
            Rollback::Allow { checkpoints } => {
                let decr = checkpoints.get();
                checkpoints.set(decr - 1);
            }
        }
    }
}