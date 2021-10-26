use deno_core::{v8::Global, v8::Value, JsRuntime};
use utilities::{messages::error::SystemError, result::Result};

pub struct Runtime(JsRuntime);

pub struct Script<'a, 'b> {
    filename: &'a str,
    code: &'b str,
}

impl Runtime {
    pub fn new() -> Self {
        let runtime = JsRuntime::new(Default::default());
        Self(runtime)
    }

    pub fn execute(&mut self, script: &Script) -> Result<Global<Value>> {
        self.0
            .execute_script(script.filename, script.code)
            .map_err(|err| SystemError::DenoAny {
                ctx: "".to_string(),
                src: err,
            })
    }
}

impl<'a, 'b> Script<'a, 'b> {
    pub fn new(filename: &'a str, code: &'b str) -> Self {
        Self { filename, code }
    }
}
