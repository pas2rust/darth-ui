use darth_rust::DarthRust;
use leptos::{leptos_dom::logging::console_error, *};
use std::time::Duration;

use super::body::{body, ToastBodyBuild};
use super::case::case;
use super::progress::progress;
use super::title::{title, ToastTitleBuild};

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
pub struct Toast {
    title: ToastTitleBuild,
    pub class: &'static str,
    pub position: ToastPosition,
    pub body: ToastBodyBuild,
    duration_seconds: Option<u64>,
    unique: bool,
    tag: Option<&'static str>,
}

impl Toast {
    pub fn render(&self) {
        let duration = self.duration_seconds.unwrap_or(2);
        let doc = document();
        let toaster = doc.get_element_by_id("toaster");
        if let Some(toaster) = toaster {
            let toast = doc.create_element(&self.tag.unwrap_or("div")).unwrap();
            let title = title(&doc, &self.title);
            let body = body(&doc, &self.body);
            let class_output = case(&self, &toaster, &toast);
            toast.append_child(&title).unwrap();
            toast.append_child(&body).unwrap();
            if self.unique {
                if let Some(node) = toaster.first_child() {
                    toaster.replace_child(&toast, &node).unwrap();
                } else {
                    toaster.append_child(&toast).unwrap();
                }
            } else {
                toaster.append_child(&toast).unwrap();
            }

            progress(doc, body, duration.clone());

            set_timeout(
                move || {
                    toast.set_attribute("style", class_output.as_str()).unwrap();
                    set_timeout(move || toast.remove(), Duration::new(duration, 1000));
                },
                Duration::new(duration, 1000),
            );
        } else {
            console_error("toaster id in index.html must be provided")
        }
    }
}
