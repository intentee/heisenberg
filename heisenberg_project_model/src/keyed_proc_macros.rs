use ra_ap_hir_expand::proc_macro::ProcMacro;

pub struct KeyedProcMacros<TKey> {
    pub key: TKey,
    pub macros: Vec<ProcMacro>,
}
