#![cfg(feature = "full")]
use components::img::image::ImageBuild;
use leptos::*;
mod components;
use components::avatar::avatar::{Avatar, AvatarBuild, AvatarSize};
use components::examples::form::register::RegisterForm;

fn main() {
    mount_to_body(|| {
        view! {
          <RegisterForm/>
          <Avatar props=AvatarBuild::new()
            .size(AvatarSize::Sm)
            .rounded_full(true)
            .image(
                ImageBuild::new()
                  .src("https://i.postimg.cc/vZGDkMcp/cropped-1920-1080-110647-1-1.jpg")
                  .alt("profile")
                  .title("avatar")
              )
          />
        }
    })
}
