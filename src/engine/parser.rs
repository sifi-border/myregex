//! parse regular expression and convert to AST

use std::{
    error::Error,
    fmt::{self, Display},
    mem::take,
};

/// Type for implimenting AST(abstract syntax tree)
#[derive(Debug)]
pub enum AST {
    Char(char),
    Plus(Box<AST>),
    Star(Box<AST>),
    Question(Box<AST>),
    Or(Box<AST>, Box<AST>),
    Seq(Vec<AST>),
}

#[derive(Debug)]
pub enum ParseError {
    InvalidEscape(usize, char), // wrong escape
    InvalidRightParen(usize),   // doesn't exist left par
    NoPrev(usize),              // no expression before +, |, *, ?
    NoRightParen,               // doesn't exist right par
    Empty,                      // empty expression
}

/// For displaying ParseError
impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidEscape(pos, c) => {
                write!(f, "ParseError: invalid escape: pos = {pos}, char = '{c}'")
            }
            ParseError::InvalidRightParen(pos) => {
                write!(f, "ParseError: invalid right parenthesis: pos = {pos}")
            }
            ParseError::NoPrev(pos) => {
                write!(f, "ParseError: no previous expression: pos = {pos}")
            }
            ParseError::NoRightParen => {
                write!(f, "ParseError: no right parenthesis")
            }
            ParseError::Empty => {
                write!(f, "ParseError: empty expression")
            }
        }
    }
}

impl Error for ParseError {}

fn parse_escape(pos: usize, c: char) -> Result<AST, ParseError> {
    match c {
        '\\' | '(' | ')' | '+' | '*' | '?' => Ok(AST::Char(c)),
        _ => {
            let err = ParseError::InvalidEscape(pos, c);
            Err(err)
        }
    }
}

/// enum type for `parse_plus_star_question`
enum PSQ {
    Plus,
    Star,
    Question,
}

/// convert +, *, ? into AST
///
/// return `ParseError` if no pattern exists before PSQ
///
/// example: "*ab", "abc|+" return error
fn parse_plus_star_question(
    seq: &mut Vec<AST>,
    ast_type: PSQ,
    pos: usize,
) -> Result<(), ParseError> {
    if let Some(prev) = seq.pop() {
        let ast = match ast_type {
            PSQ::Plus => AST::Plus(Box::new(prev)),
            PSQ::Star => AST::Star(Box::new(prev)),
            PSQ::Question => AST::Question(Box::new(prev)),
        };
        seq.push(ast);
        Ok(())
    } else {
        Err(ParseError::NoPrev(pos))
    }
}

/// convert some expression connected by Or into AST
///
/// example: abs|def|ghi -> `AST::Or("abc", AST::Or("def", "ghi"))`
fn fold_or(mut seq_or: Vec<AST>) -> Option<AST> {
    if seq_or.len() > 1 {
        let mut ast = seq_or.pop().unwrap();
        seq_or.reverse();
        for s in seq_or {
            ast = AST::Or(Box::new(s), Box::new(ast));
        }
        Some(ast)
    } else {
        seq_or.pop()
    }
}

/// convert regular expression intp AST
pub fn parse(expr: &str) -> Result<AST, ParseError> {
    // Char:    processing string
    // Escape:  processing escape sequence
    enum ParseState {
        Char,
        Escape,
    }

    let mut seq = Vec::new(); // current Seq context
    let mut seq_or = Vec::new(); // current Or context
    let mut stack = Vec::new(); // context stack
    let mut state = ParseState::Char; // current state

    for (i, c) in expr.chars().enumerate() {
        match &state {
            ParseState::Char => match c {
                '+' => parse_plus_star_question(&mut seq, PSQ::Plus, i)?,
                '*' => parse_plus_star_question(&mut seq, PSQ::Star, i)?,
                '?' => parse_plus_star_question(&mut seq, PSQ::Question, i)?,
                '(' => {
                    // save current context in stack
                    // and make current context empty
                    let prev = take(&mut seq);
                    let prev_or = take(&mut seq_or);
                    stack.push((prev, prev_or));
                }
                ')' => {
                    if let Some((mut prev, prev_or)) = stack.pop() {
                        // if exp is empty (ex: "()"), does not push
                        if !seq.is_empty() {
                            seq_or.push(AST::Seq(seq));
                        }
                        if let Some(ast) = fold_or(seq_or) {
                            prev.push(ast);
                        }
                        // update context
                        seq = prev;
                        seq_or = prev_or;
                    } else {
                        // example: abc)
                        return Err(ParseError::InvalidRightParen(i));
                    }
                }
                '|' => {
                    if seq.is_empty() {
                        // example: "||", "(|abd)"
                        return Err(ParseError::NoPrev(i));
                    } else {
                        let prev = take(&mut seq);
                        seq_or.push(AST::Seq(prev));
                    }
                    unimplemented!()
                }
                '\\' => state = ParseState::Escape,
                _ => seq.push(AST::Char(c)),
            },
            ParseState::Escape => {
                let ast = parse_escape(i, c)?;
                seq.push(ast);
                state = ParseState::Char;
            }
        }
    }

    if !stack.is_empty() {
        return Err(ParseError::NoRightParen);
    }

    // commit current seq unless it's not empty
    if !seq.is_empty() {
        seq_or.push(AST::Seq(seq));
    }

    // nanka iikanji ni naruppoi
    if let Some(ast) = fold_or(seq_or) {
        Ok(ast)
    } else {
        Err(ParseError::Empty)
    }
}
