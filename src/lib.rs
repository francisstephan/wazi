// src/lib.rs
mod data;
mod dbase;
mod writers;

use wasm_bindgen::prelude::*;
use web_sys::{Event, HtmlFormElement, HtmlInputElement, console, window};

#[wasm_bindgen]
pub fn showdate() {
    let time = data::TIME;
    let span = window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("wasmdate")
        .unwrap();
    span.set_inner_html(time);
}

// see https://deepwiki.com/rustwasm/wasm-bindgen/2.3-closure-system
#[wasm_bindgen]
pub fn connectwhzform() -> Result<(), JsValue> {
    let document = window().unwrap().document().unwrap();
    let form = document
        .get_element_by_id("postzi")
        .unwrap()
        .dyn_into::<HtmlFormElement>()?;
    let form_ref = form.clone(); // will be moved in the closure

    let closure = Closure::<dyn FnMut(Event)>::new(move |event: Event| {
        event.prevent_default(); // stop page reload
        let input = &form_ref
            .get_with_name("pinyin_ton")
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap();
        if !input.check_validity() {
            // performs the Html checks included in the form (line 94 hereabove)
            input.report_validity();
            return;
        }
        let binding = input.value();
        let pinyin = binding.as_str();
        console::log_1(&format!("Pinyin :{}", input.value()).into());
        writers::printcandidatelist(pinyin);
    });

    form.add_event_listener_with_callback("submit", &closure.as_ref().unchecked_ref())?;

    // closure.forget(); // keep closure alive : avoid this if memory leaks are an issue
    closure.forget();
    Ok(())
}

#[wasm_bindgen]
pub fn printzi() -> Result<(), JsValue> {
    let document = window().unwrap().document().unwrap();
    let input = document
        .get_element_by_id("zistring")
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap();
    let zistring = input.value();
    writers::printzidata(zistring);
    Ok(())
}
