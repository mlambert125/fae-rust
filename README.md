## Formatting

Run `cargo fmt` to format the project.
Formatting can be configured using `./rustfmt.toml`

## Syntax

LINE:
    code COMPONENT+ ~capture-pattern-string::capture-formula-string~

    * components are separated by ampersand

COMPONENT:
    [subject-term] {modifier-term}* `body-part-term`* "demographic-term"*

    * all of these "terms" can be prefixed (outside delimeter) with an ! denoting that they are negative
