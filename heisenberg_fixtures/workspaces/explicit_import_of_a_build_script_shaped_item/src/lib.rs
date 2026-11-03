use shaped::backend::Generated;
use shaped::backend::Generated as Renamed;

pub struct Imports;

impl Imports {
    pub fn file_level() -> Generated {
        Generated
    }

    pub fn in_block() {
        use shaped::backend::Generated as Local;

        let _local = Local;
    }

    pub fn renamed() -> Renamed {
        Renamed
    }
}

pub mod inline {
    use shaped::backend::Generated;

    pub fn inline() -> Generated {
        Generated
    }
}
