
use wasm_bindgen::prelude::*;
use expression_parser::{ExpressionFile, Variables};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, lmoa!");
}

#[wasm_bindgen]
pub fn parse_and_eval(text: &str) -> Result<JsValue, JsValue>  {    
    let value = match ExpressionFile::parse(text){
        Ok(x) => x,
        Err(e) => return  Err(format!("{}", e).into())

    };
    let value = match ExpressionFile::eval(value, &mut Variables::default()){
        Ok(x) => x,
        Err(e) => return  Err(format!("{}", e).into())

    };

    // let text = serde_json::to_string(&value).unwrap();

    // Ok((atoms::ok(), text).encode(env))
    Ok(JsValue::from_serde(&value).unwrap())
}

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}