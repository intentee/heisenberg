use ra_ap_cfg::CfgAtom;
use ra_ap_cfg::CfgExpr;
use ra_ap_intern::sym;

pub struct NegatedConfiguration<'expression> {
    pub expression: &'expression CfgExpr,
}

impl NegatedConfiguration<'_> {
    #[must_use]
    pub fn is_negated(&self) -> bool {
        Self::negates(self.expression, false)
    }

    #[must_use]
    pub fn negated_atoms(&self) -> Vec<CfgAtom> {
        let mut atoms = Vec::new();

        Self::collect_negated_atoms(self.expression, false, &mut atoms);

        atoms
    }

    fn collect_negated_atoms(expression: &CfgExpr, under_negation: bool, atoms: &mut Vec<CfgAtom>) {
        match expression {
            CfgExpr::Not(inner) => Self::collect_negated_atoms(inner, !under_negation, atoms),
            CfgExpr::All(inner) | CfgExpr::Any(inner) => {
                for expression in inner {
                    Self::collect_negated_atoms(expression, under_negation, atoms);
                }
            }
            CfgExpr::Atom(atom) => {
                if under_negation && !Self::negates(expression, true) {
                    atoms.push(atom.clone());
                }
            }
            CfgExpr::Invalid => {}
        }
    }

    fn negates(expression: &CfgExpr, under_negation: bool) -> bool {
        match expression {
            CfgExpr::Not(inner) => Self::negates(inner, !under_negation),
            CfgExpr::All(inner) | CfgExpr::Any(inner) => inner
                .iter()
                .any(|expression| Self::negates(expression, under_negation)),
            CfgExpr::Atom(CfgAtom::KeyValue { key, .. }) => under_negation && *key == sym::feature,
            CfgExpr::Atom(CfgAtom::Flag(flag)) => {
                under_negation && (*flag == sym::test || *flag == sym::debug_assertions)
            }
            CfgExpr::Invalid => false,
        }
    }
}
