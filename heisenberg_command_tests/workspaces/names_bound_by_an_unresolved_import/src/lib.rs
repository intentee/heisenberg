use crate::missing::helper;
use crate::missing::helper_rules;

helper_rules!();

fn unknown(_value: Unknown) {}

pub fn caller() {
    helper();
}
