#[macro_use]
mod browser;
mod engine;

use rand::prelude::*;
use serde::Deserialize;
use std::{collections::HashMap, rc::Rc, sync::Mutex};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;

#[derive(Deserialize, Debug)]
struct Rect {
    x: u16,
    y: u16,
    w: u16,
    h: u16,
}

#[derive(Deserialize, Debug)]
struct Cell {
    frame: Rect,
}
#[derive(Deserialize, Debug)]
struct Sheet {
    frames: HashMap<String, Cell>,
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let context = browser::context().expect("Could Not ger browser context");
    // Your code goes hre!

    wasm_bindgen_futures::spawn_local(async move {
        // this block is spawned and started by the browser runtime

        let sheet: Sheet = browser::fetch_json("rhb.json")
            .await
            .expect("could not load fetch rhb.json")
            .into_serde()
            .expect("could not convert rhb.json into sheet structure");

        let image = engine::load_image("rhb.png")
            .await
            .expect("Could not load rhb.png");
        let mut frame = -1;
        let interval_callback = Closure::wrap(Box::new(move || {
            // interval callback
            frame = (frame + 1) % 8;
            let frame_name = format!("Run ({}).png", frame + 1);

            context.clear_rect(0.0, 0.0, 600.0, 600.0);
            let sprite = sheet.frames.get(&frame_name).expect(" Cell not found");
            context
                .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                    &image,
                    sprite.frame.x.into(),
                    sprite.frame.y.into(),
                    sprite.frame.w.into(),
                    sprite.frame.h.into(),
                    220.0,
                    300.0,
                    sprite.frame.w.into(),
                    sprite.frame.h.into(),
                )
                .expect("error drawing sprite");
        }) as Box<dyn FnMut()>);
        browser::window()
            .unwrap()
            .set_interval_with_callback_and_timeout_and_arguments_0(
                interval_callback.as_ref().unchecked_ref(),
                50,
            )
            .expect("problem setting interval timer");
        interval_callback.forget();
        // console::log_1(&JsValue::from_str(&(format!("sprite = {:?}!", sprite))));

        console::log_1(&JsValue::from_str("Hello world!"));
        log!("Test of log macro");
    });
    web_sys::console::log_1(&JsValue::from_str("main returning"));
    Ok(())
}
