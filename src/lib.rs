use wasm_bindgen::prelude::*;
use web_sys::console;
use screeps::JsContainerIntoValue;

#[wasm_bindgen(start)]
pub fn main()
{
    for spawner in screeps::spawns().keys()
    {
        console::log_1(&spawner.into_value());
    }
}