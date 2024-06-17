use darth_rust::DarthRust;
use leptos::{leptos_dom::logging::console_error, *};
use std::cell::Cell;
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
    let total_duration_seconds = props.duration_seconds.unwrap_or(2);
    let doc = document();
    let toaster = doc.get_element_by_id("toaster");
    if let Some(toaster) = toaster {
        let toast = doc.create_element(props.tag.unwrap_or("div")).unwrap();
        let title = doc.create_element(props.title.tag.unwrap_or("h2")).unwrap();
        let body = doc.create_element(props.body.tag.unwrap_or("div")).unwrap();
        let progress_content = doc.create_element("div").unwrap();
        let progress_bar = doc.create_element("div").unwrap();
        progress_content.set_class_name("-translate-x-5 translate-y-5 w-[125%] bg-gray-200 h-2.5");
        progress_bar.set_class_name("bg-red-600 h-2.5 w-[100%]");
        progress_content.append_child(&progress_bar).unwrap();
        title.set_class_name(props.title.class);
        title.set_text_content(Some(&props.title.content));
        body.set_class_name(props.body.class);
        body.set_text_content(Some(&props.body.content));
        body.append_child(&progress_content).unwrap();
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

        let elapsed_time = Cell::new(0);
        let progress = Cell::new(100);
        let active = Cell::new(true);

        set_interval(
            move || {
                if elapsed_time.get() < total_duration_seconds && active.get() {
                    let discount = 100 / total_duration_seconds;
                    progress.set(progress.get() - discount); // diminui o valor de progresso
                    progress_bar
                        .set_attribute(
                            "style",
                            format!("width:{}%;transition:1s;", progress.get())
                                .as_str(),
                        )
                        .unwrap();
                    elapsed_time.set(elapsed_time.get() + 1);
                } else {
                    active.set(false); // desativa o intervalo
                }
            },
            Duration::from_secs(1),
        );

        set_timeout(
            move || {
                toast.set_attribute("style", class_output.as_str()).unwrap();
                set_timeout(
                    move || toast.remove(),
                    Duration::new(total_duration_seconds, 1000),
                );
            },
            Duration::new(total_duration_seconds, 1000),
        );
    } else {
        console_error("toaster id in index.html must be provided")
    }
}
