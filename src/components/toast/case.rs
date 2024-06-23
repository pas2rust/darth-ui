use super::toast::{Toast, ToastPosition};
use js_sys::wasm_bindgen::JsValue;
use leptos::web_sys::Element;

pub fn case(props: &Toast, toaster: &Element, toast: &Element) -> String {
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
                "top-0 left-0 fixed z-50 flex flex-col-reverse gap-2 justify-start transition pointer-events-none p-4",
            );
            toast.set_class_name(
                "animate-slideLeftToRight mx-auto m-4 p-2 rounded-lg pointer-events-auto",
            );
            add_class_to_toast();
        }
        ToastPosition::TopMid => {
            class_output =
                "transition: 2s ease-in-out; transform: translateY(-100%); opacity:0;".to_string();
            toaster.set_class_name(
                "top-0 inset-x-0 fixed z-50 flex flex-col-reverse gap-2 justify-center transition pointer-events-none p-4",
            );
            toast.set_class_name(
                "animate-slideTopToBottom mx-auto m-4 p-2 rounded-lg pointer-events-auto",
            );
            add_class_to_toast();
        }
        ToastPosition::TopEnd => {
            class_output =
                "transition: 2s ease-in-out; transform: translateX(100%); opacity:0;".to_string();
            toaster.set_class_name(
                "top-0 right-0 fixed z-50 flex flex-col-reverse gap-2 justify-end transition pointer-events-none p-4",
            );
            toast.set_class_name(
                "animate-slideRightToLeft mx-auto m-4 p-2 rounded-lg pointer-events-auto",
            );
            add_class_to_toast();
        }
        ToastPosition::BottomStart => {
            class_output =
                "transition: 2s ease-in-out; transform: translateY(100%); opacity:0;".to_string();
            toaster.set_class_name(
                "animate-slideLeftToRight bottom-0 left-0 fixed z-50 flex flex-col-reverse gap-2 justify-start transition pointer-events-none p-4",
            );
            toast.set_class_name("animate-toastinmx-auto m-4 p-2 rounded-lg pointer-events-auto");
            add_class_to_toast();
        }
        ToastPosition::BottomMid => {
            class_output =
                "transition: 2s ease-in-out; transform: translateY(100%); opacity:0;".to_string();
            toaster.set_class_name(
                "bottom-0 inset-x-0 fixed z-50 flex flex-col-reverse gap-2 justify-center transition pointer-events-none p-4",
            );
            toast.set_class_name("animate-toastinmx-auto m-4 p-2 rounded-lg pointer-events-auto");
            add_class_to_toast();
        }
        ToastPosition::BottomEnd => {
            class_output =
                "transition: 2s ease-in-out; transform: translateY(100%); opacity:0;".to_string();
            toaster.set_class_name(
                "animate-slideRightToLeft bottom-0 right-0 fixed z-50 flex flex-col-reverse gap-2 justify-end transition pointer-events-none p-4",
            );
            toast.set_class_name("animate-toastinmx-auto m-4 p-2 rounded-lg pointer-events-auto");
            add_class_to_toast();
        }
    };
    class_output
}
