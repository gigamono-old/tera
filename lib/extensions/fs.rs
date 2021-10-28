use crate::extension::Extension;

pub fn fs() -> FS {
    FS{}
}

pub struct FS {}

impl Extension for FS {}
