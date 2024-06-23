use darth_rust::DarthRust;
use leptos::{web_sys::Document, web_sys::Element};

#[derive(DarthRust, Default)]
pub struct ToastBodyBuild {
    class: &'static str,
    content: String,
    tag: Option<&'static str>,
}

pub fn body(doc: &Document, props: &ToastBodyBuild) -> Element {
    let body = doc.create_element(props.tag.unwrap_or("div")).unwrap();
    let message = doc.create_element("p").unwrap();
    body.set_class_name(props.class);
    message.set_text_content(Some(&props.content));
    body.append_child(&message).unwrap();
    body
}
