extern crate wasm_bindgen;
extern crate web_sys;

use wasm_bindgen::prelude::*;
use web_sys::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::Clamped;
use web_sys::ImageData;

fn average(numbers: &[i32]) -> f32 {
    numbers.iter().sum::<i32>() as f32 / numbers.len() as f32
}

fn create_invisible_canvas() -> web_sys::HtmlCanvasElement {
    let document = window().unwrap().document().unwrap();
    let _invisible_canvas: web_sys::HtmlCanvasElement = document.create_element("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    _invisible_canvas
}

fn set_canvas_dimensions(canvas: &web_sys::HtmlCanvasElement, width: u32, height: u32) {
    canvas.set_width(width);
    canvas.set_height(height);
}

fn get_canvas_context(canvas: &web_sys::HtmlCanvasElement) -> web_sys::CanvasRenderingContext2d {
    canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap()
}

fn convert_image_element_to_image_data(img: &web_sys::HtmlImageElement) -> Result<ImageData, JsValue> {
    let _width = img.width();
    let _height = img.height();
    let _invisible_canvas = create_invisible_canvas();
    set_canvas_dimensions(&_invisible_canvas, _width, _height);
    let _invisible_context = get_canvas_context(&_invisible_canvas);

    _invisible_context.draw_image_with_html_image_element(&img, 0.0, 0.0);

    _invisible_context.get_image_data(0.0, 0.0, _width.into(), _height.into())
}

fn get_canvas_from_selector(selector: &str) -> web_sys::HtmlCanvasElement {
    let document = window().unwrap().document().unwrap();

    let error_selector_not_found = String::from("Selector not found.");
    let error_querying = String::from("Error querying canvas");

    let _canvas = match document.query_selector(selector) {
        Ok(Some(element)) => element,
        Ok(None) => {
            log(&error_selector_not_found);
            panic!(error_selector_not_found);
        },
        Err(_) => {
            log(&error_querying);
            panic!(error_querying);
        }
    };

    _canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap()
}


#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[wasm_bindgen]
pub fn greet(img: &web_sys::HtmlImageElement){
    let _width = img.width();
    let _height = img.height();
    let _canvas = get_canvas_from_selector(&String::from("#canvas"));

    let image_data: web_sys::ImageData = convert_image_element_to_image_data(&img).unwrap();

    let pixels = image_data.data();
    let mut new_image_data: Vec<u8> = Vec::new();
    let mut i = 0;
    let iterate_on_pixels = pixels.iter().step_by(4);

    for pixel in iterate_on_pixels {
        let p = *pixel as u32;

        let r = pixels[i];
        let g = pixels[i+1];
        let b = pixels[i+2];
        let a = pixels[i+3];

        let numbers = [r as i32,g as i32,b as i32,a as i32];

        let avg = average(&numbers[..]);

        i += 4;
        new_image_data.push(avg as u8);
        new_image_data.push(avg as u8);
        new_image_data.push(avg as u8);
        new_image_data.push(255);
    }

    let b = new_image_data.as_mut_slice();  

    let _context = get_canvas_context(&_canvas);
    

    _canvas.set_width(_width);
    _canvas.set_height(_height);

    let new_image_data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(b), _width, _height).unwrap();


    _context.put_image_data(&new_image_data, 0.0, 0.0);
}
