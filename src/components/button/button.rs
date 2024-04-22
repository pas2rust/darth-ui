#![cfg(feature = "button")]
use darth_rust::DarthRust;
use leptos::*;

#[derive(DarthRust, Default)]
pub struct ButtonBuild {
    class: &'static str,
    kind: &'static str,
}

#[component]
pub fn Button(props: ButtonBuild, children: Children) -> impl IntoView {
    view! {
        <button class=props.class type=props.kind>
            {children()}
        </button>
    }
}
