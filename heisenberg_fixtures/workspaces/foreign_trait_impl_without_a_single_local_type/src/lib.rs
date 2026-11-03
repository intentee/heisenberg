pub mod first;
pub mod second;

use outside::Pair;

use crate::first::First;
use crate::second::Second;

impl Pair<First, Second> for u32 {}
