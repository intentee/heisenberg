use cargo_platform::Cfg;
use ra_ap_cfg::CfgAtom;
use ra_ap_intern::Symbol;

pub fn cfg_atom(cfg: Cfg) -> CfgAtom {
    match cfg {
        Cfg::Name(name) => CfgAtom::Flag(Symbol::intern(&name.name)),
        Cfg::KeyPair(key, value) => CfgAtom::KeyValue {
            key: Symbol::intern(&key.name),
            value: Symbol::intern(&value),
        },
    }
}
