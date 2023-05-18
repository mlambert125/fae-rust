mod rule_parsing;
mod rules;

fn main() {
    let rules = 
        rule_parsing::parse_rules_file(String::from("./sample-files/cdi.rules"));

    for rule in rules {
        println!("Rule for code: {0}", rule.code);
    }
}
