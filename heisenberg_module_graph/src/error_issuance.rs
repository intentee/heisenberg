use crate::untyped_error_name::UntypedErrorName;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ErrorIssuance {
    Constructed { written: String },
    Returned { error: UntypedErrorName },
}
