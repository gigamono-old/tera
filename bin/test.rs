extern crate tera;
use tera::{extensions, module_loaders, runtime::{Runtime, RuntimeOptions}};

fn main() {
    let runtime = Runtime::new(RuntimeOptions {
        module_loader: Some(Box::new(module_loaders::esm())),
        extensions: vec![
            Box::new(extensions::fs()),
        ],
    });

    runtime.execute_module();

    println!("runtime initialised");
}
