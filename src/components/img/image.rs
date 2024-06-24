#![cfg(feature = "image")]
use darth_rust::DarthRust;
use leptos::*;

#[derive(DarthRust, Default)]
pub struct ImageBuild {
    class: String,
    src: &'static str,
    title: &'static str,
    alt: &'static str,
    w: &'static str,
    h: &'static str,
}

#[component]
pub fn Image(props: ImageBuild) -> impl IntoView {
    view! {
        <img class=props.class src=props.src title=props.title alt=props.alt width=props.w height=props.h/>
    }
}
