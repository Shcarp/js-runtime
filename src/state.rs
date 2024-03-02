use std::{cell::RefCell, rc::Rc};

type GlobalContext = v8::Global<v8::Context>;
type JsRuntimeStateRef = Rc<RefCell<JsRuntimeState>>;

pub struct JsRuntimeState {
    context: Option<GlobalContext>,
}

impl JsRuntimeState {
    pub fn new(isolate: &mut v8::Isolate) -> JsRuntimeStateRef {
        let context = {
            let scope = &mut v8::HandleScope::new(isolate);
            let context = v8::Context::new(scope);
            v8::Global::new(scope, context)
        };

        Rc::new(RefCell::new(JsRuntimeState { context: Some(context)}))
    }

    pub fn get_context(isolate: &mut v8::Isolate) -> GlobalContext {
        let state = isolate.get_slot::<JsRuntimeStateRef>().unwrap();

        let ctx = &state.borrow().context;

        ctx.as_ref().unwrap().clone()
    }
}
