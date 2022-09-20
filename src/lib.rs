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

    // Your code goes hre!

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
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
        image.set_src("Idle (1).png");

        success_rx.await;

        web_sys::console::log_1(&JsValue::from_str("resuming execution"));
        context.draw_image_with_html_image_element(&image, 0.0, 0.0);
        serpinsky(
            &context,
            [(300.0, 0.0), (0.0, 600.0), (600.0, 600.0)],
            (0, 200, 0),
            5,
        );
        let json = fetch_json(&"./rhb.json").await.expect(
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
        image.set_src("rhb.png");

        success_rx.await;
        let sprite = sheet.frames.get("Run (1).png").expect(" Cell not found");
        console::log_1(&JsValue::from_str(&(format!("sprite = {:?}!", sprite))));

        console::log_1(&JsValue::from_str("Hello world!"));
        console::log_1(&JsValue::from_str(
            &(format!("sheet = {:?}!", sheet.frames.keys())),
        ));
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

fn draw_triangle(
    context: &web_sys::CanvasRenderingContext2d,
    points: [(f64, f64); 3],
    color: (u8, u8, u8),
) {
    let [top, left, right] = points;
    let color_str = format!("rgb({},{},{})", color.0, color.1, color.2);
    context.set_fill_style(&wasm_bindgen::JsValue::from_str(&color_str));
    context.move_to(top.0, top.1);
    context.begin_path();
    context.line_to(left.0, left.1);
    context.line_to(right.0, right.1);
    context.line_to(top.0, top.1);
    context.close_path();
    context.fill();
}
fn serpinsky(
    context: &web_sys::CanvasRenderingContext2d,
    points: [(f64, f64); 3],
    color: (u8, u8, u8),
    depth: u8,
) {
    draw_triangle(context, points, color);
    let [top, left, right] = points;
    if depth > 0 {
        let mut rng = thread_rng();
        let next_color = (
            rng.gen_range(0..255),
            rng.gen_range(0..255),
            rng.gen_range(0..255),
        );

        let left_middle = midpoint(left, top);
        let right_middle = midpoint(right, top);
        let bottom_middle = midpoint(left, right);
        serpinsky(
            &context,
            [top, left_middle, right_middle],
            next_color,
            depth - 1,
        );
        serpinsky(
            &context,
            [left_middle, left, bottom_middle],
            next_color,
            depth - 1,
        );
        serpinsky(
            &context,
            [right_middle, bottom_middle, right],
            next_color,
            depth - 1,
        );
    }
}
fn midpoint(point_1: (f64, f64), point_2: (f64, f64)) -> (f64, f64) {
    ((point_1.0 + point_2.0) / 2.0, (point_1.1 + point_2.1) / 2.0)
}
