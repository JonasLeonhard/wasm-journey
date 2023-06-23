use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str); // bind to the browsers javascript alert function
}

#[wasm_bindgen]
pub fn greet(name: &str) -> Result<(), JsValue> {
    alert(&format!(
        "Hello, {name}! - lets insert something to the dom..."
    ));

    // the web_sys crate contains predefined extern "C" byindings to most existing web-apis like
    // window, document, body, etc.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let val = document.create_element("p")?;
    val.set_inner_html(&format!("Hello from Rust: {name}!"));

    body.append_child(&val)?;

    Ok(())
}
