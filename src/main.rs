use leptos::*;
mod components;
use components::form::components::text::{InputBuild, Label as LabelProps, Text as InputText};

use crate::components::container::container::ContainerBuild;
use crate::components::form::components::text::Err;
fn main() {
    mount_to_body(|| {
        view! {
                <div>
                    <InputText props=InputBuild::new()
                        .label(LabelProps::new()
                            .class("")
                            .name("email")
                            .id("email")
                            .forhtml("email")
                        )
                        .input_container(ContainerBuild::new()
                            .class("box_ghost_white hover:border-indigo-400")
                        )
                        .container(ContainerBuild::new()
                            .class("p-4")
                        )
                        .class("input_focus_indigo_9")
                        .placeholder("Your best email")
                        .name("email")
                        .err(Err::new()
                            .pattern(r"(?i)^[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,}$")
                            .message("🚨 email must be valid")
                            .class("opacity-60 animate-shake")
                        )
                    />
                </div>
        }
    })
}
