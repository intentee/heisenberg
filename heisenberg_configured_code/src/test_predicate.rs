use ra_ap_cfg::CfgAtom;
use ra_ap_cfg::CfgExpr;
use ra_ap_intern::sym;

pub struct TestPredicate<'expression> {
    pub expression: &'expression CfgExpr,
}

impl TestPredicate<'_> {
    #[must_use]
    pub fn affirms_test(&self) -> bool {
        Self::affirms(self.expression, false)
    }

    #[must_use]
    pub fn is_test_alone(&self) -> bool {
        matches!(self.expression, CfgExpr::Atom(CfgAtom::Flag(flag)) if *flag == sym::test)
    }

    #[must_use]
    pub fn names_test(&self) -> bool {
        Self::names(self.expression)
    }

    fn affirms(expression: &CfgExpr, under_negation: bool) -> bool {
        match expression {
            CfgExpr::Not(inner) => Self::affirms(inner, !under_negation),
            CfgExpr::All(inner) | CfgExpr::Any(inner) => inner
                .iter()
                .any(|expression| Self::affirms(expression, under_negation)),
            CfgExpr::Atom(CfgAtom::Flag(flag)) => !under_negation && *flag == sym::test,
            CfgExpr::Atom(CfgAtom::KeyValue { .. }) | CfgExpr::Invalid => false,
        }
    }

    fn names(expression: &CfgExpr) -> bool {
        match expression {
            CfgExpr::Not(inner) => Self::names(inner),
            CfgExpr::All(inner) | CfgExpr::Any(inner) => inner.iter().any(Self::names),
            CfgExpr::Atom(CfgAtom::Flag(flag)) => *flag == sym::test,
            _ => false,
        }
    }
}
