use darth_rust::DarthRust;
use leptos::*;
use leptos_font_icons::icons::{colors::Color, icon::Icon, icons::Icons};

use crate::components::container::container::{Container, ContainerBuild};
#[derive(DarthRust, Default)]
pub struct Label {
    class: &'static str,
    id: &'static str,
    name: &'static str,
    forhtml: &'static str,
}

#[derive(DarthRust, Default)]
pub struct Err {
    message: &'static str,
    class: &'static str,
    pattern: &'static str,
}
#[derive(DarthRust)]
pub struct InputBuild {
    label: Label,
    placeholder: &'static str,
    name: &'static str,
    value: &'static str,
    err: Err,
    class: &'static str,
    container: ContainerBuild,
    input_container: ContainerBuild,
}

#[component]
pub fn Text(props: InputBuild) -> impl IntoView {
    let (read_input, write_input) = create_signal(props.value.to_string());

    view! {
        <Container props=props.container>
            <label class=props.label.class for=props.label.forhtml name=props.label.name id=props.label.id>
                {props.label.name}:
            </label>
            <Container props=props.input_container>
                <span class="absolute z-10 pt-2 translate-x-4">
                    <Icons style=(Icon::Email, Color::White, 24) />
                </span>
                <input
                    on:input=move |ev| write_input.set(event_target_value(&ev))
                    class=props.class
                    placeholder=props.placeholder
                    name=props.name
                    value=read_input()
                    title=props.label.name
                />
            </Container>
       </Container>
    }
}
