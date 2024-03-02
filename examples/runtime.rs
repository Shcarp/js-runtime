
use js_runtime::{JsRuntime, JsRuntimeParams};

fn main() {
    JsRuntime::init();
    let mut runtime = JsRuntime::new(JsRuntimeParams::default());

    let jscode = r#"
        function hello() {
            const result = print('Hello, World!');
            print(result);
            return 'Hello, World!';
        }
        hello();
    "#;

    let result = runtime.execute_script(jscode);
    println!("result: {:?}", result);
}