use shaped::backend::Generated;
use shaped::backend::Generated as Renamed;

pub fn file_level() -> Generated {
    Generated
}

pub fn renamed() -> Renamed {
    Renamed
}

pub fn in_block() {
    use shaped::backend::Generated as Local;

    let _local = Local;
}

pub mod inline {
    use shaped::backend::Generated;

    pub fn inline_module() -> Generated {
        Generated
    }
}
