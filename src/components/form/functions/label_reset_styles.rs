use leptos::*;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlDivElement, HtmlInputElement};

pub fn label_reset_styles(
    ev: Event,
    name: &'static str,
    default_label_class: String,
    current_class: String,
    label: &'static str,
) {
    let element = event_target::<HtmlInputElement>(&ev);
    let label_element = document()
        .query_selector(format!("#{}", name).as_str())
        .expect("query_selector result in error")
        .expect("label_element is none");
    let node = element.parent_node().expect("parent_node is none");
    let div = JsCast::dyn_into::<HtmlDivElement>(node).expect("get div faileid");
    let class = div.get_attribute("class").expect("get attr failed");
    let class = class.replace(&current_class, "");
    div.set_class_name(&class);
    label_element.set_class_name(&default_label_class);
    label_element.set_inner_html(&format!("{}:", label).as_str());
    label_element
        .set_attribute("title", label)
        .expect("title set error");
}
