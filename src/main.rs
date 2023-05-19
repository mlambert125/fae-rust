use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use codespan_reporting::term;
use fae::RulesParser;
use fae::StaticParseError;
use lalrpop_util::ParseError;
use std::error::Error;
use std::fs;
use std::process::exit;

fn display_parse_error(err: StaticParseError, input: &str, input_path: &str) {
    let (message, range) = match err {
        ParseError::InvalidToken { location } => {
            (String::from("Invalid token"), Some(location..location))
        }
        ParseError::UnrecognizedEof { location, expected } => {
            let mut message = format!("Unexpected EOF, expected:");
            for i in expected {
                message += " ";
                message += &i;
            }
            (message, Some(location..location))
        }
        ParseError::UnrecognizedToken { token, expected } => {
            let (l, t, r) = token;
            let mut message = format!("Unexepected token. Got \"{t}\", expected one of:");
            for i in expected {
                message += " ";
                message += &i;
            }
            (message, Some(l..r))
        }
        ParseError::ExtraToken { token } => {
            let (l, t, r) = token;
            (format!("Extra token: \"{t}\""), Some(l..r))
        }
        // TODO: change the error type to include a token location.
        ParseError::User { error } => (error.to_string(), None),
    };
    let mut files = SimpleFiles::new();
    let file_id = files.add(input_path, input);

    let diagnostic = if let Some(range) = range {
        Diagnostic::error()
            .with_labels(vec![Label::primary(file_id, range)])
            .with_message(message)
    } else {
        Diagnostic::error()
            .with_message(message)
    };

    let writer = StandardStream::stderr(ColorChoice::Auto);
    let config = term::Config::default();
    match term::emit(&mut writer.lock(), &config, &files, &diagnostic) {
        Err(err) => eprintln!("Failed to print error: {err}"),
        _ => {}
    };
}

fn main() -> Result<(), Box<dyn Error>> {
    let input_path = "./sample-files/broken.rules";
    let input = fs::read_to_string(input_path)?;

    let rule = match RulesParser::new().parse(&input) {
        Ok(rule) => rule,
        Err(err) => {
            display_parse_error(err, &input, input_path);
            exit(1);
        }
    };

	println!("{:#?}", rule);
    Ok(())
}
