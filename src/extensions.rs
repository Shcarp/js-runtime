mod print;
use v8::{FunctionCallback, HandleScope, MapFnTo};

use crate::utils;

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

        if let Ok(result) = utils::execute_script(scope, jscode) {
            let func = v8::Local::<v8::Function>::try_from(result).unwrap();
            let v = v8::undefined(scope).into();
            let args = [binddings.into()];
            func.call(scope, v, &args).unwrap();
        }

        // let func = func.unwrap();
        // let func = func.into();
        // scope.get_current_context().unwrap().global(scope).set(scope, name.into(), func);
    }
}


pub fn install(scope: &mut HandleScope) {
    Extensions::install(scope, "print", print::print, print::PRINT)
}

