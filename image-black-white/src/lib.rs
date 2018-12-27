extern crate wasm_bindgen;
extern crate web_sys;

use wasm_bindgen::prelude::*;
use web_sys::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::Clamped;
use web_sys::ImageData;

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    black: u8,
    alpha: u8,
}

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

fn draw_image(img: &web_sys::HtmlImageElement, selector: &str, transform: &Fn(&Color) -> u8) {
    let _width = img.width();
    let _height = img.height();
    let _canvas = get_canvas_from_selector(&String::from(selector));
    set_canvas_dimensions(&_canvas, _width, _height);

    let image_data: web_sys::ImageData = convert_image_element_to_image_data(&img).unwrap();

    let pixels = image_data.data();
    let mut new_image_data: Vec<u8> = Vec::new();

    for (i, pixel) in pixels.iter().enumerate().step_by(4) {
        let p = *pixel as u32;

        let red = pixels[i];
        let green = pixels[i+1];
        let black = pixels[i+2];
        let alpha = pixels[i+3];
        let color = Color {
            red, green, black, alpha
        };

        let avg = transform(&color);

        if i < 15 {
            log(&format!("{:?}", avg));
        }

        new_image_data.push(avg);
        new_image_data.push(avg);
        new_image_data.push(avg);
        new_image_data.push(255);
    }

    let clamped_image_data = Clamped(new_image_data.as_mut_slice());  
    let _context = get_canvas_context(&_canvas);
    let new_image_data = ImageData::new_with_u8_clamped_array_and_sh(clamped_image_data, _width, _height).unwrap();

    _context.put_image_data(&new_image_data, 0.0, 0.0);
}

#[wasm_bindgen]
pub fn grayscale_with_average(img: &web_sys::HtmlImageElement, selector: &str){
    fn grayscale_avg(color: &Color) -> u8 {
        let sum = (color.green as i32 + color.red as i32 + color.black as i32 + color.alpha as i32) as f32 / 4.0;
        sum as u8
    }

    draw_image(img, selector, &grayscale_avg);
}

#[wasm_bindgen]
pub fn grayscale_with_luminocity(img: &web_sys::HtmlImageElement, selector: &str){
    fn grayscale_luminocity(color: &Color) -> u8 {
        let red_factor = 0.21;
        let green_factor = 0.72;
        let black_factor = 0.07;

        let green = (color.green as f32) * green_factor;
        let red = (color.red as f32) * red_factor;
        let black = (color.black as f32) * black_factor;

        let sum = green + red + black;
        sum as u8
    }

    draw_image(img, selector, &grayscale_luminocity);
}
