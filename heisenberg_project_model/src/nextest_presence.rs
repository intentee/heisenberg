#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NextestPresence {
    Missing { manifest: String },
    Present { file: String },
}
