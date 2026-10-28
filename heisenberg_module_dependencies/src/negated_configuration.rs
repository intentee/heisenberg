use ra_ap_cfg::CfgAtom;
use ra_ap_cfg::CfgExpr;
use ra_ap_intern::sym;

pub struct NegatedConfiguration<'expression> {
    pub expression: &'expression CfgExpr,
}

impl NegatedConfiguration<'_> {
    pub fn is_negated(&self) -> bool {
        Self::negates(self.expression, false)
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
