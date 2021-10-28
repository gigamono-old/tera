use std::fmt::Debug;

use v8::UnboundScript;

use super::error_wrapper::ErrorWrapper;
use super::extension::{self, Extension};
use super::module_loader::ModuleLoader;

pub struct Runtime {
    isolate: v8::OwnedIsolate,
}

#[derive(Default)]
pub struct RuntimeOptions {
    pub module_loader: Option<Box<dyn ModuleLoader>>,
    pub extensions: Vec<Box<dyn Extension>>,
}

impl Runtime {
    pub fn new(options: RuntimeOptions) -> Self {
        let params = v8::Isolate::create_params();
        let isolate = v8::Isolate::new(params);

        Self { isolate }
    }

    pub fn execute_module(&self) -> () {
        // TODO
        print!("Doing nothing right now");
    }
}

mod tests {
    use crate::{extensions, module_loaders};
    use super::{Runtime, RuntimeOptions};

    #[test]
    fn default_runtime() {
        let rt = Runtime::new(Default::default());
        rt.execute_module();
    }
    
    #[test]
    fn runtime_with_options() {
        let rt = Runtime::new(RuntimeOptions {
            module_loader: Some(Box::new(module_loaders::esm())),
            extensions: vec![Box::new(extensions::fs())],
        });

        rt.execute_module();
    }
}
