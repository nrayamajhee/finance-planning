use console_log;
use fluid::{body, Context};
use fluid_macro::html;
use log::Level;
use wasm_bindgen::JsValue;

fn main() -> Result<(), JsValue> {
    console_log::init_with_level(Level::Info).unwrap();
    let ctx = Context::new();
    let counter = ctx.create_signal(0);
    let p = html! {
        p { "Hello world" }
    };
    body()?.append_child(&p)?;
    Ok(())
}
