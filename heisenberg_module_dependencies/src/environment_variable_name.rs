use ra_ap_hir_expand::tt::Leaf;
use ra_ap_hir_expand::tt::LitKind;
use ra_ap_hir_expand::tt::Literal;
use ra_ap_hir_expand::tt::TopSubtree;
use ra_ap_hir_expand::tt::TtElement;
use ra_ap_syntax::unescape::unescape_str;

fn string_literal(element: TtElement<'_>) -> Option<String> {
    match element {
        TtElement::Leaf(Leaf::Literal(
            literal @ Literal {
                kind: LitKind::Str, ..
            },
        )) => {
            let mut value = String::new();

            unescape_str(literal.text(), |_, character| value.extend(character));

            Some(value)
        }
        TtElement::Leaf(Leaf::Literal(
            literal @ Literal {
                kind: LitKind::StrRaw(_),
                ..
            },
        )) => Some(literal.text().to_owned()),
        TtElement::Leaf(_) | TtElement::Subtree(..) => None,
    }
}

pub fn environment_variable_name(argument: &TopSubtree) -> Option<String> {
    argument.iter().next().and_then(string_literal)
}
