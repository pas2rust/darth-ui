use darth_rust::DarthRust;
use leptos::{leptos_dom::logging::console_error, *};
use std::time::Duration;

use super::case::case;

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

#[derive(DarthRust, Default)]
pub struct ToastBodyBuild {
    class: &'static str,
    content: String,
    tag: Option<&'static str>,
}

#[derive(DarthRust, Default)]
pub struct ToastTitleBuild {
    class: &'static str,
    content: String,
    tag: Option<&'static str>,
}

#[derive(DarthRust)]
pub struct ToastBuild {
    title: ToastTitleBuild,
    pub class: &'static str,
    pub position: ToastPosition,
    body: ToastBodyBuild,
    duration_seconds: Option<u64>,
    unique: bool,
    tag: Option<&'static str>,
}

pub fn toast(props: ToastBuild) {
    let duration_seconds = props.duration_seconds.unwrap_or(2);
    let doc = document();
    let toaster = doc.get_element_by_id("toaster");
    if let Some(toaster) = toaster {
        let toast = doc.create_element(props.tag.unwrap_or("div")).unwrap();
        let title = doc.create_element(props.title.tag.unwrap_or("h2")).unwrap();
        let body = doc.create_element(props.body.tag.unwrap_or("p")).unwrap();
        title.set_class_name(props.title.class);
        title.set_text_content(Some(&props.title.content));
        body.set_class_name(props.body.class);
        body.set_text_content(Some(&props.body.content));
        let class_output = case(&props, &toaster, &toast);
        toast.append_child(&title).unwrap();
        toast.append_child(&body).unwrap();
        if props.unique {
            if let Some(node) = toaster.first_child() {
                toaster.replace_child(&toast, &node).unwrap();
            } else {
                toaster.append_child(&toast).unwrap();
            }
        } else {
            toaster.append_child(&toast).unwrap();
        }
        set_timeout(
            move || {
                toast.set_attribute("style", class_output.as_str()).unwrap();
                set_timeout(
                    move || toast.remove(),
                    Duration::new(duration_seconds, 1000),
                );
            },
            Duration::new(duration_seconds, 1000),
        );
    } else {
        console_error("toaster id in index.html must be provided")
    }
}
