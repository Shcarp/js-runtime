pub fn print(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let arg = args.get(0);
    let arg = arg.to_string(scope).unwrap();
    let arg = arg.to_rust_string_lossy(scope);
    rv.set(v8::undefined(scope).into());
}

pub const JS_CODE: &str = include_str!("js/print.js");
