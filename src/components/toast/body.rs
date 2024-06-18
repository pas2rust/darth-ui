use darth_rust::DarthRust;
use leptos::{set_interval, web_sys::Document, web_sys::Element};
use std::{cell::Cell, time::Duration};

#[derive(DarthRust, Default)]
pub struct ToastBodyBuild {
    class: &'static str,
    content: String,
    tag: Option<&'static str>,
}

pub fn body(doc: &Document, props: &ToastBodyBuild) -> Element {
    let body = doc.create_element(props.tag.unwrap_or("div")).unwrap();

    body.set_class_name(props.class);
    body.set_text_content(Some(&props.content));
    body
}
