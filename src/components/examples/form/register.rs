#![cfg(feature = "full")]
use crate::components::button::button::{Button, ButtonBuild};
use crate::components::container::container::ContainerBuild;
use crate::components::form::form::{Form, FormBuild};
use crate::components::form::{
    email::{InputEmail, InputEmailBuild},
    label::LabelBuild,
    password::{InputPassword, InputPasswordBuild},
    text::{InputText, InputTextBuild},
};
use crate::components::icons::{icon::Icon, icons::IconBuild};
use crate::components::toast::body::ToastBodyBuild;
use crate::components::toast::toast::{Toast, ToastPosition};
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
           .class("")
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
                    let body_content = format!(
                        "(200, user: {}, email: {}, password: {})",
                        form.user, form.email, form.password
                    );
                    Toast::new()
                        .class("bg-green-500 overflow-hidden")
                            .position(ToastPosition::TopEnd)
                            .duration_seconds(10)
                            .body(
                                ToastBodyBuild::new()
                                .class("p-2")
                                .content(body_content)
                                .tag("div")
                            )
                        .render()

                } else if let Err(content) = form {
                    Toast::new()
                        .class("bg-blue-600")
                        .position(ToastPosition::TopMid)
                        .duration_seconds(10)
                        .body(
                            ToastBodyBuild::new()
                            .class("pt-2 pl-2 pr-2 font-bold grid")
                            .content(content)
                        )
                        .unique(false)
                        .render()
                }
        })>
            <InputText props=InputTextBuild::new()
                .label(LabelBuild::new()
                    .name("User")
                    .forhtml("User")
                    .class("")
                )
                .icon(IconBuild::new()
                    .class("text-white")
                    .icon(Icon::User)
                    .size::<usize>(24)
                )
                .input_box(ContainerBuild::new()
                    .class("box_ghost rounded-full hover:border-indigo-400")
                )
                .container(ContainerBuild::new()
                    .class("p-2")
                )
                .class("input focus:bg-indigo-950")
                .placeholder("Your user")
                .name("user")
            />
            <InputEmail props=InputEmailBuild::new()
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
                    .class("box_ghost rounded-full hover:border-indigo-400")
                )
                .container(ContainerBuild::new()
                    .class("p-2")
                )
                .class("input focus:bg-indigo-950")
                .placeholder("Your best email")
                .name("email")
            />
            <InputPassword props=InputPasswordBuild::new()
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
                    .class("box_ghost rounded-full hover:border-indigo-400")
                )
                .container(ContainerBuild::new()
                    .class("p-2")
                )
                .class("input focus:bg-indigo-950")
                .placeholder("Your password")
                .name("password")
            />
            <Button props=ButtonBuild::new()
                .class("button_sunset w-full")
                .kind("submit")
            >
                "Register"
            </Button>
        </Form>
    }
}
