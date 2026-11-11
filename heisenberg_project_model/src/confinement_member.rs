use crate::build_side::BuildSide;

pub struct ConfinementMember {
    pub id: String,
    pub manifest: String,
    pub name: String,
    pub side: BuildSide,
}
