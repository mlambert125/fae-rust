use mongodb::bson::oid::ObjectId;

pub struct RuleDefinition {
    pub id : ObjectId,
    pub code : String,
    pub primary_component : RuleComponent,
    pub secondary_components : Vec<RuleComponent>,
    pub capture_pattern : String,
    pub capture_formula : String
}

pub struct RuleComponent {
    pub subject : NegatableTerm,
    pub modifiers : Vec<NegatableTerm>,
    pub demographics : Vec<NegatableTerm>,
    pub body_parts : Vec<NegatableTerm>
}
pub struct NegatableTerm {    
    pub value : String,
    pub positive : bool
}
