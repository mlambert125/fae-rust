use mongodb::bson::oid::ObjectId;

#[derive(Debug)]
pub struct RuleDefinition {
	pub id: Option<ObjectId>,
	pub code: String,
	pub primary_component : RuleComponent,
	pub secondary_components : Vec<RuleComponent>,
	pub capture_pattern : String,
	pub capture_formula : String
}

#[derive(Debug)]
pub struct RuleComponent {
	pub subject: RuleTerm,
	pub modifiers: Vec<RuleTerm>,
	pub demographics: Vec<RuleTerm>,
	pub body_parts: Vec<RuleTerm>,
}

#[derive(Debug)]
pub struct RuleTerm {
	pub value: String,
	pub negate: bool,
}

impl From<String> for RuleTerm {
	fn from(value: String) -> Self {
		Self {
			value,
			negate: false,
		}
	}
}

#[derive(Debug)]
// This could also be a field within RuleTerm but the information is only really
// useful to the parser and would be redundant inside of RuleComponent.
pub enum RuleTermType {
    Subject(RuleTerm),
    Modifier(RuleTerm),
    Demographic(RuleTerm),
    BodyPart(RuleTerm),
}
