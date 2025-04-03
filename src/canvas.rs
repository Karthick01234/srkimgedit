use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement, window};

#[wasm_bindgen]
pub struct Canvas {
    canvas: Rc<RefCell<HtmlCanvasElement>>,
    ctx: CanvasRenderingContext2d,
}

#[wasm_bindgen]
impl Canvas {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: Option<HtmlCanvasElement>, width: Option<u32>, height: Option<u32>) -> Self {
        let html_canvas_element = Self::get_or_create_canvas(canvas, width, height);

        let ctx = html_canvas_element
            .get_context("2d")
            .unwrap()
            .expect("Failed to get 2D context")
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        Self {
            canvas: Rc::new(RefCell::new(html_canvas_element)),
            ctx,
        }
    }

    fn get_or_create_canvas(
        canvas: Option<HtmlCanvasElement>,
        width: Option<u32>,
        height: Option<u32>,
    ) -> HtmlCanvasElement {
        let new_canvas = canvas.unwrap_or({
            let document = web_sys::window().unwrap().document().unwrap();
            let canvas_width = width.unwrap_or(320);
            let canvas_height = height.unwrap_or(240);
            let html_canvas_element = document
                .create_element("canvas")
                .unwrap()
                .dyn_into::<HtmlCanvasElement>()
                .unwrap();
            html_canvas_element.set_width(canvas_width);
            html_canvas_element.set_height(canvas_height);
            html_canvas_element
        });
        new_canvas
    }
    #[wasm_bindgen]
    pub fn get_canvas(&self) -> HtmlCanvasElement {
        self.canvas.borrow().clone()
    }

    #[wasm_bindgen]
    pub fn draw_image(
        &self,
        path: String,
        fit_canvas_to_image: Option<bool>,
        fit_image_to_canvas: Option<bool>,
    ) {
        let canvas = self.canvas.borrow().clone();
        let fit_canvas_to_image = fit_canvas_to_image.unwrap_or(false);
        let fit_image_to_canvas = fit_image_to_canvas.unwrap_or(false);
        let img = HtmlImageElement::new().unwrap();
        img.set_src(&path);
        let ctx_clone = self.ctx.clone();
        let img_clone = img.clone();
        let onload = Closure::wrap(Box::new(move || {
            if fit_image_to_canvas && !fit_canvas_to_image {
                let win = window().expect("No global `window` exists");
                let max_width = win
                    .inner_width()
                    .expect("Failed to get inner width")
                    .as_f64()
                    .unwrap();
                let max_height = win
                    .inner_height()
                    .expect("Failed to get inner width")
                    .as_f64()
                    .unwrap();
                let img_width = img_clone.natural_width() as f64;
                let img_height = img_clone.natural_height() as f64;

                let scale = (max_width / img_width).min(max_height / img_height);

                let width = if img_width >= max_width {
                    img_width * scale
                } else {
                    img_width
                };

                let height = if img_height >= max_height {
                    img_height * scale
                } else {
                    img_height
                };
                canvas.set_width(width.round() as u32);
                canvas.set_height(height.round() as u32);
            }
            if ctx_clone
                .draw_image_with_html_image_element_and_dw_and_dh(
                    &img_clone,
                    0.0,
                    0.0,
                    canvas.width().into(),
                    canvas.height().into(),
                )
                .is_err()
            {
                web_sys::console::error_1(&"Failed to draw image!".into());
            }
        }) as Box<dyn Fn()>);

        img.set_onload(Some(onload.as_ref().unchecked_ref()));
        onload.forget();

        let onerror = Closure::wrap(Box::new(move || {
            web_sys::console::error_1(&"Failed to load image!".into());
        }) as Box<dyn Fn()>);

        img.set_onerror(Some(onerror.as_ref().unchecked_ref()));
        onerror.forget();
    }
}
