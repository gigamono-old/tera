use crate::module_loader::ModuleLoader;

pub fn esm() -> ESMLoader {
    ESMLoader{}
}

pub struct ESMLoader {}

impl ModuleLoader for ESMLoader {}
