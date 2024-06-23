

#![cfg(feature = "input-email")]
use darth_rust::DarthRust;
use leptos::*;

use crate::components::{
    container::container::{Container, ContainerBuild},
    icons::icons::{Icon, IconBuild},
};

use super::label::{Label, LabelBuild};

#[derive(DarthRust)]
pub struct InputEmailBuild {
    label: LabelBuild,
    icon: IconBuild,
    placeholder: &'static str,
    name: &'static str,
    value: &'static str,
    class: &'static str,
    container: ContainerBuild,
    input_box: ContainerBuild,
}

#[component]
pub fn InputEmail(props: InputEmailBuild) -> impl IntoView {
    let (read_input, write_input) = create_signal(props.value.to_string());
    view! {
        <Container props=props.container>
            <Label props=props.label.clone() />
            <Container props=props.input_box>
                <span class="absolute z-10 pt-2 translate-x-4 pointer-events-none">
                   <Icon props=props.icon/>
                </span>
                <input
                    type="email"
                    on:input=move |ev| write_input.set(event_target_value(&ev))
                    class=props.class
                    placeholder=props.placeholder
                    name=props.name
                    value=move || read_input.get()
                    title=props.label.name
                />
            </Container>
       </Container>
    }
}