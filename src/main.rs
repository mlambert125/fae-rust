use fae::RulesParser;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./sample-files/simple.rules")?;
    let rule = RulesParser::new().parse(&input);
	println!("{:#?}", rule);
    Ok(())
}
