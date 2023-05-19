use lalrpop_util::lalrpop_mod;
use lalrpop_util::ParseError;
use std::collections::VecDeque;
use std::io::Read;

pub mod rules;
lalrpop_mod!(pub rule_parser);
pub use rule_parser::RulesParser;

pub type StaticParseError<'a> = ParseError<usize, lalrpop_util::lexer::Token<'a>, &'static str>;

// This is just a helper to reduce the verbosity of error messages
fn user_error(error: &'static str) -> StaticParseError {
	ParseError::User { error }
}

fn strip_token_edges(token: &str) -> Result<String, StaticParseError> {
	// lalrpop doesn't support capture groups (annoying) and will complain
	// about ambiguity if we try to seperate the square brackets out (very annoying).
	// To work around this, we have to trim the brackets off after recieving the string.
	let mut bytes = token.bytes().collect::<VecDeque<u8>>();
	bytes.pop_front();
	bytes.pop_back();
	let mut result = String::new();
	bytes.read_to_string(&mut result)
		.map_err(|_| user_error("Invalid UTF-8"))?;
	Ok(result)
}

fn strip_token_end(token: &str) -> Result<String, StaticParseError> {
	let mut bytes = token.bytes().collect::<VecDeque<u8>>();
	bytes.pop_back();
	let mut result = String::new();
	bytes.read_to_string(&mut result)
		.map_err(|_| user_error("Invalid UTF-8"))?;
	Ok(result)
}
