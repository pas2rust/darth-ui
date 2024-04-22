#![cfg(feature = "examples")]
use crate::components::button::button::{Button, ButtonBuild};
use crate::components::container::container::ContainerBuild;
use crate::components::form::components::text::{InputBuild, InputText, LabelBuild};
use crate::components::form::form::{Form, FormBuild};
use crate::components::icons::{icon::Icon, icons::IconBuild};
use crate::components::toast::toast::{toast, ToastBuild, ToastPosition};
use darth_rust::DarthRust;
use leptos::*;

#[derive(DarthRust, Debug)]
pub struct Form {
    #[pattern(r"^.{3,32}$")]
    #[pattern_notify("user must be valid")]
    user: String,
    #[pattern(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")]
    #[pattern_notify("e-mail must be valid")]
    email: String,
    #[pattern(r"^.{6,32}$")]
    #[pattern_notify("password must be valid")]
    password: String,
}

#[component]
pub fn RegisterForm() -> impl IntoView {
    view! {
        <Form props=FormBuild::new()
           .class("box_ghost_purple_3 p-4 m-4")
            .submit(|hashmap, _| {
                let user = hashmap.get("user").expect("Input name user not found");
                let email = hashmap.get("email").expect("Input name email not found");
                let password = hashmap.get("password").expect("Input name password not found");
                let form = Form::new()
                    .user(user)
                    .email(email)
                    .password(password)
                    .build();
                if let Ok(form) = form {
                    let message = format!(
                        " (200, user: {}, email: {}, password: {})",
                        form.user, form.email, form.password
                    );
                    toast(ToastBuild::new()
                        .class("bg-green-500")
                        .position(ToastPosition::TopEnd)
                        .duration_seconds(5)
                        .message(message)
                    );
                } else if let Err(form) = form {
                    let message = format!(" (404: {})", form);
                    toast(ToastBuild::new()
                        .class("bg-red-500")
                        .position(ToastPosition::TopEnd)
                        .duration_seconds(5)
                        .message(message)
                    );
                }
        })>
            <InputText props=InputBuild::new()
                .label(LabelBuild::new()
                    .name("User")
                    .forhtml("User")
                )
                .icon(IconBuild::new()
                    .class("text-white")
                    .icon(Icon::User)
                    .size::<usize>(24)
                )
                .input_box(ContainerBuild::new()
                    .class("box_ghost_white hover:border-indigo-400")
                )
                .container(ContainerBuild::new()
                    .class("p-2")
                )
                .class("input focus:bg-indigo-950")
                .placeholder("Your user")
                .name("user")
            />
            <InputText props=InputBuild::new()
                .label(LabelBuild::new()
                    .name("Email")
                    .forhtml("Email")
                )
                .icon(IconBuild::new()
                    .class("text-white")
                    .icon(Icon::Email)
                    .size::<usize>(24)
                )
                .input_box(ContainerBuild::new()
                    .class("box_ghost_white hover:border-indigo-400")
                )
                .container(ContainerBuild::new()
                    .class("p-2")
                )
                .class("input focus:bg-indigo-950")
                .placeholder("Your best email")
                .name("email")
            />
            <InputText props=InputBuild::new()
                .label(LabelBuild::new()
                    .name("password")
                    .forhtml("Password")
                )
                .icon(IconBuild::new()
                    .class("text-white")
                    .icon(Icon::Key)
                    .size::<usize>(24)
                )
                .input_box(ContainerBuild::new()
                    .class("box_ghost_white hover:border-indigo-400")
                )
                .container(ContainerBuild::new()
                    .class("p-2")
                )
                .class("input focus:bg-indigo-950")
                .placeholder("Your password")
                .name("password")
            />
            <Button props=ButtonBuild::new()
                .class("box_gradient_sunset w-full")
                .kind("submit")
            >
                "Register"
            </Button>
        </Form>
    }
}
