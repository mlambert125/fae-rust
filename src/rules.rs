use mongodb::bson::oid::ObjectId;

#[derive(Debug)]
pub struct RuleDefinition {
	pub id: Option<ObjectId>,
	pub code: String,
	pub primary_component: RuleComponent,
	pub secondary_components: Vec<RuleComponent>,
	pub capture: Option<RuleCapture>,
}

#[derive(Debug)]
pub struct RuleComponent {
	pub subject: RuleTerm,
	pub modifiers: Vec<RuleTerm>,
	pub demographics: Vec<RuleTerm>,
	pub body_parts: Vec<RuleTerm>,
	// This is probably supposed to be parsed at some point.
	pub age_range: Option<String>,
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
pub struct RuleCapture {
	pub pattern: String,
	pub formula: String,
}

#[derive(Debug)]
// This could also be a field within RuleTerm but the information is only really
// useful to the parser and would be redundant inside of RuleComponent.
pub enum RuleTermType {
    Subject(RuleTerm),
    Modifier(RuleTerm),
    Demographic(RuleTerm),
    BodyPart(RuleTerm),
    AgeRange(String),
	Capture(RuleCapture)
}

