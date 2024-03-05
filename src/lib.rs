mod extensions;
mod state;
mod utils;

use std::sync::OnceLock;

use state::JsRuntimeState;
use v8::{CreateParams, OwnedIsolate};

type LocalValue<'a> = v8::Local<'a, v8::Value>;

pub struct JsRuntime {
    pub isolate: v8::OwnedIsolate,
}

#[derive(Default)]
pub struct JsRuntimeParams(v8::CreateParams);

impl JsRuntimeParams {
    pub fn new(_snopshot: Option<Vec<u8>>) -> Self {
        JsRuntimeParams(CreateParams::default())
    }

    pub fn into_inner(self) -> v8::CreateParams {
        self.0
    }
}

impl JsRuntime {
    pub fn init() {
        static V8_INIT: OnceLock<()> = OnceLock::new();
        V8_INIT.get_or_init(|| {
            let platform = v8::new_default_platform(0, false).make_shared();
            v8::V8::initialize_platform(platform);
            v8::V8::initialize();
        });
    }

    pub fn new(params: JsRuntimeParams) -> Self {
        let isolate = v8::Isolate::new(params.into_inner());
        JsRuntime::init_isolate(isolate)
    }

    pub fn execute_script(
        &mut self,
        code: impl AsRef<str>,
        is_module: bool,
    ) -> Result<serde_json::Value, serde_json::Value> {
        let ctx = JsRuntimeState::get_context(&mut self.isolate);
        let handle_scope = &mut v8::HandleScope::with_context(&mut self.isolate, ctx);
        match utils::execute_script(handle_scope, code, is_module) {
            Ok(v) => Ok(serde_v8::from_v8(handle_scope, v).unwrap()),
            Err(err) => Err(serde_v8::from_v8(handle_scope, err).unwrap()),
        }
    }

    pub fn create_snopshot() -> Vec<u8> {
        todo!()
    }

    fn init_isolate(mut isolate: OwnedIsolate) -> Self {
        let state = JsRuntimeState::new(&mut isolate);
        isolate.set_slot(state);
        {
            let context = JsRuntimeState::get_context(&mut isolate);
            let mut scope = v8::HandleScope::with_context(&mut isolate, context);
            // Extensions::install(&mut scope);
            extensions::install(&mut scope);
        }

        JsRuntime { isolate }
    }
}
