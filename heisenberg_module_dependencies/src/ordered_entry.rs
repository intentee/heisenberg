use ra_ap_syntax::SyntaxNode;

pub struct OrderedEntry<TKey> {
    pub key: TKey,
    pub node: SyntaxNode,
}
