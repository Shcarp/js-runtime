use super::utils::run_loacal_future;

pub fn fetch(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let url: String = serde_v8::from_v8(scope, args.get(0)).unwrap();

    println!("fetching {}", url);

    let fut = async move {
        let result = reqwest::get(url).await.unwrap().text().await.unwrap();

        println!("{}", result);

        rv.set(serde_v8::to_v8(scope, result).unwrap().into());
    };

    run_loacal_future(fut);
}

pub const JS_CODE: &str = include_str!("js/fetch.js");

