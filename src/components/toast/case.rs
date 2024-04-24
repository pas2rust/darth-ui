use super::toast::{ToastBuild, ToastPosition};
use js_sys::{wasm_bindgen::JsValue, Array};
use leptos::web_sys::Element;

pub fn case(props: &ToastBuild, toaster: &Element, toast: &Element) -> String {
    let add_class_to_toast = || {
        let class_split = props.class.split_whitespace();
        let js_array = js_sys::Array::new();
        for class in class_split {
            let js_value = JsValue::from_str(class);
            js_array.push(&js_value);
        }
        toast.class_list().add(&js_array).unwrap();
    };
    let mut class_output = String::new();
    match props.position {
        ToastPosition::TopStart => {
            class_output =
                "transition: 2s ease-in-out; transform: translateY(-100%); opacity:0;".to_string();
            toaster.set_class_name(
                "top-0 left-0 fixed z-50 flex flex-col-reverse gap-4 justify-start transition",
            );
            toast.set_class_name("animate-slideInTop mx-auto rounded-br-lg");
            add_class_to_toast();
        }
        ToastPosition::TopMid => {
            class_output =
                "transition: 2s ease-in-out; transform: translateY(-100%); opacity:0;".to_string();
            toaster.set_class_name(
                "top-0 inset-x-0 fixed z-50 flex flex-col-reverse gap-4 justify-center transition",
            );
            toast.set_class_name("animate-slideInTop mx-auto rounded-b-lg");
            add_class_to_toast();
        }
        ToastPosition::TopEnd => {
            class_output =
                "transition: 2s ease-in-out; transform: translateY(-100%); opacity:0;".to_string();
            toaster.set_class_name(
                "top-0 right-0 fixed z-50 flex flex-col-reverse gap-4 justify-end transition",
            );
            toast.set_class_name("animate-slideInTop mx-auto rounded-bl-lg");
            add_class_to_toast();
        }
        ToastPosition::BottomStart => {
            class_output =
                "transition: 2s ease-in-out; transform: translateY(100%); opacity:0;".to_string();
            toaster.set_class_name(
                "bottom-0 left-0 fixed z-50 flex flex-col-reverse gap-4 justify-start transition",
            );
            toast.set_class_name("animate-toastin mx-auto rounded-l-lg");
            add_class_to_toast();
        }
        ToastPosition::BottomMid => {
            class_output =
                "transition: 2s ease-in-out; transform: translateY(100%); opacity:0;".to_string();
            toaster.set_class_name(
                "bottom-0 inset-x-0 fixed z-50 flex flex-col-reverse gap-4 justify-center transition",
            );
            toast.set_class_name("animate-toastin mx-auto rounded-l-lg");
            add_class_to_toast();
        }
        ToastPosition::BottomEnd => {
            class_output =
                "transition: 2s ease-in-out; transform: translateY(100%); opacity:0;".to_string();
            toaster.set_class_name(
                "bottom-0 right-0 fixed z-50 flex flex-col-reverse gap-4 justify-end transition",
            );
            toast.set_class_name("animate-toastin mx-auto rounded-l-lg");
            add_class_to_toast();
        }
    };
    class_output
}
