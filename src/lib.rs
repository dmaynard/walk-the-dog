use rand::prelude::*;
use serde::Deserialize;
use std::{collections::HashMap, rc::Rc, sync::Mutex};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;

#[macro_use]
mod browser;

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

    // Your code goes hre!

    // let window = browser::window().expect("Np Window Found");
    let document = browser::document().expect("No Document Found");
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    wasm_bindgen_futures::spawn_local(async move {
        // this block is spawned and started by the browser runtime

        let json = fetch_json(&"rhb.json").await.expect(
            "Couldn't load json
        ",
        );
        let sheet: Sheet = json.into_serde().expect("Couldn't convert json into sheet");
        let (success_tx, success_rx) = futures::channel::oneshot::channel::<Result<(), JsValue>>();
        let success_tx = Rc::new(Mutex::new(success_tx));
        // let error_tx = Rc::clone(&success_tx);

        let image = web_sys::HtmlImageElement::new().unwrap();
        let (success_tx, success_rx) = futures::channel::oneshot::channel::<Result<(), JsValue>>();
        let success_tx = Rc::new(Mutex::new(Some(success_tx)));
        let error_tx = Rc::clone(&success_tx);

        let callback = Closure::once(move || {
            if let Some(success_tx) = success_tx.lock().ok().and_then(|mut opt| opt.take()) {
                web_sys::console::log_1(&JsValue::from_str("success callback"));
                success_tx.send(Ok(()));
            }
        });

        let error_callback = Closure::once(move |err: JsValue| {
            if let Some(error_tx) = error_tx.lock().ok().and_then(|mut opt| opt.take()) {
                web_sys::console::log_1(&JsValue::from_str("error callback"));
                web_sys::console::log_1(&err);
                error_tx.send(Err(err));
            }
        });

        image.set_onload(Some(callback.as_ref().unchecked_ref()));
        image.set_onerror(Some(error_callback.as_ref().unchecked_ref()));
        image.set_src("./rhb.png");

        success_rx.await;
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

async fn fetch_json(jason_path: &str) -> Result<JsValue, JsValue> {
    let window = web_sys::window().unwrap();
    console::log_1(&JsValue::from_str("got window"));
    console::log_1(&JsValue::from_str(jason_path));
    let resp_value =
        wasm_bindgen_futures::JsFuture::from(window.fetch_with_str(jason_path)).await?;
    console::log_1(&JsValue::from_str("fetch with str finished"));
    let resp: web_sys::Response = resp_value.dyn_into()?;
    wasm_bindgen_futures::JsFuture::from(resp.json()?).await
}
