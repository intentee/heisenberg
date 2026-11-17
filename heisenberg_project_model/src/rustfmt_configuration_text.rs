use std::io::ErrorKind;

pub enum RustfmtConfigurationText {
    Absent,
    Read { text: String },
    Unreadable { kind: ErrorKind },
}
