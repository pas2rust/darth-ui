use leptos::*;
mod components;
use components::form::components::text::{
    Label as LabelProps, Props as InputTextProps, Text as InputText,
};

use crate::components::form::components::text::{Container, Err};
fn main() {
    mount_to_body(|| {
        view! {
                <div>
                    <InputText props=InputTextProps::new()
                        .label(LabelProps::new()
                            .class("")
                            .name("email")
                            .id("email")
                            .forhtml("email")
                        )
                        .container(Container::new()
                            .class("container_ghost_white_hover_indigo_3")
                        )
                        .class("bg-transparent p-2 pl-14 outline-none w-[100%] focus:transition focus:bg-indigo-900")
                        .placeholder("Your best email")
                        .name("email")
                        .err(Err::new()
                            .pattern(r"(?i)^[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,}$")
                            .seconds_to_check_regex(None)
                            .message("🚨 email must be valid")
                            .class("opacity-60 animate-shake")
                        )
                    />
                </div>
        }
    })
}
