mod utils;
mod print;
mod fetch;
use v8::{FunctionCallback, HandleScope, MapFnTo};

pub struct Extensions;

impl Extensions {
    pub fn install(
        scope: &mut HandleScope,
        name: impl AsRef<str>,
        func: impl MapFnTo<FunctionCallback>,
        jscode: impl AsRef<str>,
    ) {
        let binddings = v8::Object::new(scope);
        let name = v8::String::new(scope, name.as_ref()).unwrap();
        let func = v8::Function::new(scope, func).unwrap();
        binddings.set(scope, name.into(), func.into());

        match crate::utils::execute_script(scope, jscode, false){
            Ok(result) => {
                let func = v8::Local::<v8::Function>::try_from(result).unwrap();
                let v = v8::undefined(scope).into();
                let args = [binddings.into()];
                func.call(scope, v, &args).unwrap();
                println!("installed done");
            },
            Err(_) => {
                println!("Error in executing js code");
            },
        }
    }
}

pub fn install(scope: &mut HandleScope) {
    Extensions::install(scope, "fetch", fetch::fetch, fetch::JS_CODE);
    Extensions::install(scope, "print", print::print, print::JS_CODE);
}

