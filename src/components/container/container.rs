#![cfg(feature = "container")]
use darth_rust::DarthRust;
use leptos::*;

use crate::components::tailwindcss::TailwindCSS;

#[derive(DarthRust, Default)]
pub struct ContainerBuild {
    class: &'static str,
    tailwind: TailwindCSS
    //build: fn(ContainerBuild) -> ContainerBuild
}

#[component]
pub fn Container(props: ContainerBuild, children: Children) -> impl IntoView {
    view! {
        <div class=props.class>
            {children()}
        </div>
    }
}
