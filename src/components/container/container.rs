#![cfg(feature = "container")]
use darth_rust::DarthRust;
use leptos::*;

#[derive(DarthRust, Default)]
pub struct ContainerBuild {
    class: &'static str,
}

#[component]
pub fn Container(props: ContainerBuild, children: Children) -> impl IntoView {
    view! {
        <div class=props.class>
            {children()}
        </div>
    }
}
