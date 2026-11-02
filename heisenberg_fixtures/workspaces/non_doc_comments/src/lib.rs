//! The crate is documented by a doc comment.

/// The item is documented by a doc comment.
pub struct Documented;

/** The block doc comment documents a constant. */
const LIMIT: u8 = 1;

// A line comment explains what names should.
/* A block comment explains what names should. */

#[cfg(unix)]
fn platform() -> u8 {
    // A comment in code active on one target.
    LIMIT
}

#[cfg(windows)]
fn platform() -> u8 {
    // A comment in code active on the other target.
    LIMIT
}

macro_rules! commented {
    () => {
        // A comment in a macro body.
    };
}
