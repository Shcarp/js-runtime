use v8::TryCatch;

use crate::LocalValue;

pub fn execute_script<'a>(
    handle_scope: &mut v8::HandleScope<'a>,
    code: impl AsRef<str>,
) -> Result<LocalValue<'a>, LocalValue<'a>> {
    // todo!()
    let scope = &mut TryCatch::new(handle_scope);

    let code = v8::String::new(scope, code.as_ref()).unwrap();

    v8::Script::compile(scope, code.into(), None)
        .and_then(|script| script.run(scope))
        .map_or_else(|| Err(scope.stack_trace().unwrap()), Ok)
}
