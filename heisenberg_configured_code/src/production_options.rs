use ra_ap_cfg::CfgAtom;
use ra_ap_cfg::CfgDiff;
use ra_ap_cfg::CfgOptions;
use ra_ap_intern::sym;

#[must_use]
pub fn production_options(options: &CfgOptions) -> CfgOptions {
    let mut production = options.clone();

    production.apply_diff(CfgDiff::new(Vec::new(), vec![CfgAtom::Flag(sym::test)]));

    production
}
