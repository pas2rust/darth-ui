use darth_rust::DarthRust;
use js_sys::{wasm_bindgen::JsValue, Array};
use leptos::{leptos_dom::logging::console_error, *};
use std::time::Duration;

#[derive(Default)]
pub enum ToastPosition {
    TopStart,
    TopMid,
    #[default]
    TopEnd,
    BottomStart,
    BottomMid,
    BottomEnd,
}

#[derive(DarthRust)]
pub struct ToastBuild {
    class: &'static str,
    position: ToastPosition,
    message: String,
    duration_seconds: Option<u64>,
}

pub fn toast(props: ToastBuild) {
    let duration_seconds = props.duration_seconds.unwrap_or(2);
    let doc = document();
    let toaster = doc.get_element_by_id("toaster");
    if let Some(toaster) = toaster {
        let div = doc.create_element("div").unwrap();
        let h1 = doc.create_element("h1").unwrap();
        h1.set_class_name("p-2 font-bold flex");
        h1.set_text_content(Some(&props.message));
        match props.position {
            ToastPosition::TopStart => {
                toaster.set_class_name(
                    "top-0 left-0 fixed z-50 flex flex-col-reverse gap-4 justify-start transition",
                );
                div.set_class_name("animate-slideInTop mx-auto rounded-md");
                let class = Array::new();
                let js_value = JsValue::from_str(props.class);
                class.push(&js_value);
                div.class_list().add(&class).unwrap();
            }
            ToastPosition::TopMid => {
                toaster.set_class_name(
                    "top-0 inset-x-0 fixed z-50 flex flex-col-reverse gap-4 justify-center transition",
                );
                div.set_class_name("animate-slideInTop mx-auto rounded-md");
                let class = Array::new();
                let js_value = JsValue::from_str(props.class);
                class.push(&js_value);
                div.class_list().add(&class).unwrap();
            }
            ToastPosition::TopEnd => {
                toaster.set_class_name(
                    "top-0 right-0 fixed z-50 flex flex-col-reverse gap-4 justify-end transition",
                );
                div.set_class_name("animate-slideInTop mx-auto rounded-md");
                let class = Array::new();
                let js_value = JsValue::from_str(props.class);
                class.push(&js_value);
                div.class_list().add(&class).unwrap();
            }
            ToastPosition::BottomStart => {
                toaster.set_class_name(
                    "bottom-0 left-0 fixed z-50 flex flex-col-reverse gap-4 justify-start transition",
                );
                div.set_class_name("animate-toastin mx-auto rounded-md");
                let class = Array::new();
                let js_value = JsValue::from_str(props.class);
                class.push(&js_value);
                div.class_list().add(&class).unwrap();
            }
            ToastPosition::BottomMid => {
                toaster.set_class_name(
                    "bottom-0 inset-x-0 fixed z-50 flex flex-col-reverse gap-4 justify-center transition",
                );
                div.set_class_name("animate-toastin mx-auto rounded-md");
                let class = Array::new();
                let js_value = JsValue::from_str(props.class);
                class.push(&js_value);
                div.class_list().add(&class).unwrap();
            }
            ToastPosition::BottomEnd => {
                toaster.set_class_name(
                    "bottom-0 right-0 fixed z-50 flex flex-col-reverse gap-4 justify-end transition",
                );
                div.set_class_name("animate-toastin mx-auto rounded-md");
                let class = Array::new();
                let js_value = JsValue::from_str(props.class);
                class.push(&js_value);
                div.class_list().add(&class).unwrap();
            }
        };
        div.append_child(&h1).unwrap();
        toaster.append_child(&div).unwrap();
        set_timeout(
            move || {
                let value = format!(
                    "transition: {}s ease-in-out; transform: translateY(100%); opacity:0;",
                    duration_seconds
                );
                div.set_attribute("style", value.as_str()).unwrap();
                set_timeout(move || div.remove(), Duration::new(duration_seconds, 1000));
            },
            Duration::new(duration_seconds, 1000),
        );
    } else {
        console_error("toaster id in index.html must be provided")
    }
}
