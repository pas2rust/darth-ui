use darth_rust::DarthRust;
use leptos::*;

use crate::components::{
    container::container::{Container, ContainerBuild},
    icons::icons::{Icon, IconBuild},
};
#[derive(DarthRust, Default)]
pub struct LabelBuild {
    class: &'static str,
    id: &'static str,
    name: &'static str,
    forhtml: &'static str,
}
#[derive(DarthRust)]
pub struct InputBuild {
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
pub fn InputText(props: InputBuild) -> impl IntoView {
    let (read_input, write_input) = create_signal(props.value.to_string());
    view! {
        <Container props=props.container>
            <label class=props.label.class for=props.label.forhtml name=props.label.name id=props.label.id>
                {props.label.name}:
            </label>
            <Container props=props.input_box>
                <span class="absolute z-10 pt-2 translate-x-4 pointer-events-none">
                   <Icon props=props.icon/>
                </span>
                <input
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
