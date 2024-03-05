
use js_runtime::{JsRuntime, JsRuntimeParams};

fn main() {
    JsRuntime::init();
    let mut runtime = JsRuntime::new(JsRuntimeParams::default());

    let jscode = r#"
        async function hello() {
            const result = print('Hello, World!');
            const web = await fetch('https://www.rust-lang.org');

            console.log(web);

            return 'Hello, World!';
        }
        await hello();
    "#;

    let result = runtime.execute_script(jscode, true);
    println!("result: {:?}", result);
}