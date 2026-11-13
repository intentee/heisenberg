use ra_ap_syntax::AstNode;
use ra_ap_syntax::ast;

use crate::signature_scope::SignatureScope;

pub struct SignatureTuple<'tuple> {
    pub tuple: &'tuple ast::TupleType,
}

impl SignatureTuple<'_> {
    pub fn is_in_a_signature(&self) -> bool {
        self.tuple.fields().nth(1).is_some()
            && SignatureScope {
                node: self.tuple.syntax(),
            }
            .is_in_a_signature()
    }
}
