use leptos::{set_interval, web_sys::Document, web_sys::Element};
use std::{cell::Cell, time::Duration};

pub fn progress(doc: Document, body: Element, duration: u64) {
    let progress_content = doc.create_element("div").unwrap();
    let progress_bar = doc.create_element("div").unwrap();
    progress_content.set_class_name("-translate-x-5 translate-y-5 w-[125%] bg-gray-200 h-2.5");
    progress_bar.set_class_name("bg-red-600 h-2.5 w-[100%]");
    progress_content.append_child(&progress_bar).unwrap();
    body.append_child(&progress_content).unwrap();

    let elapsed_time = Cell::new(0);
    let progress = Cell::new(100);
    let active = Cell::new(true);

    set_interval(
        move || {
            if elapsed_time.get() < duration && active.get() {
                let discount = 100 / duration;
                progress.set(progress.get() - discount); // diminui o valor de progresso
                progress_bar
                    .set_attribute(
                        "style",
                        format!("width:{}%;transition:1s;", progress.get()).as_str(),
                    )
                    .unwrap();
                elapsed_time.set(elapsed_time.get() + 1);
            } else {
                active.set(false); // desativa o intervalo
            }
        },
        Duration::from_secs(1),
    );
}
