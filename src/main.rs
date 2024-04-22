#![cfg(feature = "examples")]
use leptos::*;
mod components;
use components::examples::form::register::RegisterForm;

fn main() {
    mount_to_body(|| {
        view! {
          <RegisterForm/>
        }
    })
}
