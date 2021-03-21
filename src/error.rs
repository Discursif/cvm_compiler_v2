use std::{fmt::{self, Display}, rc::Rc};

use pest::{RuleType, Span, error::{self, ErrorVariant}, iterators::{Pair, Pairs, pairs, queueable_token::QueueableToken}};

use crate::Rule;

pub enum ParseError {
    ValueExpectedStringOrByteArray(Paire<Rule>),
    ByteArrayIsNotMadeOfU8(Paire<Rule>),
    CantGetFunction(Paire<Rule>, String, Vec<String>),
    InvalidReturnType(Paire<Rule>, String /* Expected */, String /* Found */),
    TypeNotFound(Paire<Rule>, String),
    PanicTypeReached(Paire<Rule>),
    ForbiddenMethodUse(Paire<Rule>, String /* Method name */, String /* Type name */, Vec<String> /* Method args */),
    ForbiddenFunctionUse(Paire<Rule>, String /* Function name */, Vec<String> /* Function args */),
    MethodNotFound(Paire<Rule>, String /* Method name */, String /* Type name */, Vec<String> /* Method args */),
    CantFindVariableOrType(Paire<Rule>, String),
    InvalidAssignement(Paire<Rule>, String, String),
    UnexpectedType(Paire<Rule>),
    ExpectedTypeInVariant(Paire<Rule>),
    VariantNotFound(Paire<Rule>, String, String),
    CantGetVariable(Paire<Rule>, String)
}

impl ParseError {

    pub fn as_custom_error(&self) -> pest::error::Error<Rule> {
        error::Error::new_from_span(ErrorVariant::CustomError { message: self.get_error_message().to_owned()}, self.get_inner().as_span())
    }

    pub fn get_inner(&self) -> &Paire<Rule> {
        match self {
            Self::ValueExpectedStringOrByteArray(e) |
            Self::UnexpectedType(e) |
            Self::PanicTypeReached(e) |
            Self::ByteArrayIsNotMadeOfU8(e) |
            Self::InvalidReturnType(e,..) |
            Self::TypeNotFound(e,..) |
            Self::ForbiddenMethodUse(e,..) |
            Self::ForbiddenFunctionUse(e,..) |
            Self::MethodNotFound(e,..) |
            Self::InvalidAssignement(e, _, _) |
            Self::CantFindVariableOrType(e,..) |
            Self::ExpectedTypeInVariant(e) |
            Self::VariantNotFound(e, ..) |
            Self::CantGetVariable(e, ..) |
            Self::CantGetFunction(e,..) => e
        }
    }

    pub fn get_error_message(&self) -> String {
        match self {
            ParseError::ValueExpectedStringOrByteArray(_) => "A value statement require a String or a Byte array".to_owned(),
            ParseError::ByteArrayIsNotMadeOfU8(_) => "A byte array must contains only u8s (0-255)".to_owned(),
            Self::CantGetFunction(_,b,c) => format!("Can't get function: {}({})",b,c.join(", ")),
            ParseError::InvalidReturnType(_, c, d) => format!("Invalid return type expected `{}` found `{}`",c,d),
            ParseError::TypeNotFound(_, a) => format!("Can't find type `{}`",a),
            ParseError::PanicTypeReached(_) => "You can't use anything that uses the `Panic` type (You are probably using a forbidden method)".to_owned(),
            ParseError::ForbiddenMethodUse(_, a, b, c) => format!("Method `{}({}) -> Panic` in `{}` can't be used because of `Panic` return type",
                a,c.join(", "),b),
            ParseError::ForbiddenFunctionUse(_, a, b) => format!("Function `{}({}) -> Panic` can't be used because of `Panic` return type",
            a,b.join(", ")),
            ParseError::MethodNotFound(_, a, b, c) => format!("Method `{}({})` in `{}` not found",
            a,c.join(", "),b),
            ParseError::CantFindVariableOrType(_, a) => format!("Can't find Variable or Type `{}`",a),
            ParseError::InvalidAssignement(_, a,b) => format!("Invalid type in assignement! Expected `{}` found `{}`",a,b),
            ParseError::UnexpectedType(_) => "No type was expected in this context".to_owned(),
            ParseError::ExpectedTypeInVariant(_) => "Expected type in variant found expression!".to_owned(),
            ParseError::VariantNotFound(_, a,b) => format!("Variant `{}` not found in `{}`",a,b),
            ParseError::CantGetVariable(_, a) => format!("Can't find variable `{}`",a),
        }
    }
}

pub fn to_static(pair: &Pair<Rule>) -> Paire<Rule> {
    Paire {
        queue: pair.queue.clone(),
        input: pair.input.to_owned(),
        start: pair.start,
    }
}

#[derive(Clone, Debug)]
pub struct Paire<R> {
    pub queue: Rc<Vec<QueueableToken<R>>>,
    pub input: String,
    pub start: usize,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        write!(f,"{}",self.as_custom_error())
    }
}

impl <R: RuleType> Paire<R> {
    #[inline]
    pub fn as_rule(&self) -> R {
        match self.queue[self.pair()] {
            QueueableToken::End { rule, .. } => rule,
            _ => unreachable!(),
        }
    }

    fn pair(&self) -> usize {
        match self.queue[self.start] {
            QueueableToken::Start {
                end_token_index, ..
            } => end_token_index,
            _ => unreachable!(),
        }
    }

    fn pos(&self, index: usize) -> usize {
        match self.queue[index] {
            QueueableToken::Start { input_pos, .. } | QueueableToken::End { input_pos, .. } => {
                input_pos
            }
        }
    }
    
    pub fn as_span<'a>(&'a self) -> Span<'a> {
        let start = self.pos(self.start);
        let end = self.pos(self.pair());

        // Generated positions always come from Positions and are UTF-8 borders.
        unsafe { Span::new_unchecked(&self.input, start, end) }
    }

    pub fn into_inner<'a>(&'a self) -> Pairs<'a, R> {
        let pair = self.pair();

        pairs::new(self.queue.clone(), &self.input, self.start + 1, pair)
    }
}

pub enum CVMError {

}