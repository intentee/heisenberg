use ra_ap_cfg::CfgExpr;
use ra_ap_hir::InFile;
use ra_ap_syntax::SyntaxNodePtr;

pub struct InactiveCodeSite {
    pub cfg: CfgExpr,
    pub node: InFile<SyntaxNodePtr>,
}
