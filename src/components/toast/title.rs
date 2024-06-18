use darth_rust::DarthRust;
use leptos::{web_sys::Document, web_sys::Element};

#[derive(DarthRust, Default)]
pub struct ToastTitleBuild {
    class: &'static str,
    content: String,
    tag: Option<&'static str>,
}

pub fn title(doc: &Document, props: &ToastTitleBuild) -> Element {
    let title = doc.create_element(props.tag.unwrap_or("div")).unwrap();

    title.set_class_name(props.class);
    title.set_text_content(Some(&props.content));
    title
}
