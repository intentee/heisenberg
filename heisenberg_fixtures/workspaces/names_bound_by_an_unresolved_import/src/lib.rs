use crate::missing::helper;
use crate::missing::helper_rules;

pub fn caller() {
    helper();
}

helper_rules!();

fn unknown(_value: Unknown) {}
