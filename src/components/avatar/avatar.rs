#![cfg(all(feature = "image", feature = "avatar"))]
use std::ops::Add;

use darth_rust::DarthRust;
use leptos::*;

use crate::components::img::image::{Image, ImageBuild};

#[derive(Default)]
pub enum AvatarSize {
    #[default]
    Sm,
    Md,
    Lg,
    Xl,
    Custom(&'static str, &'static str)
}

#[derive(Default)]
pub enum AvatarStatus {
    Online,
    Offline,
    Away,
    Busy,
    #[default]
    None,
}
#[derive(DarthRust, Default)]
pub struct AvatarBuild {
    image: ImageBuild,
    status: AvatarStatus,
    size: AvatarSize,
    rounded_full: bool,
    m: i32,
    p: i32,
}

#[component]
pub fn Avatar(props: AvatarBuild) -> impl IntoView {
    let (w, h) = match props.size {
        AvatarSize::Sm => ("w-10", "h-10"),
        AvatarSize::Md => ("w-20", "h-20"),
        AvatarSize::Lg => ("w-40", "h-40"),
        AvatarSize::Xl => ("w-80", "h-80"),
        AvatarSize::Custom(w, h) => (w,h)
    };
    let mut class = format!("{} {} object-cover m-{} p-{}", w, h, props.m, props.p);
    if props.rounded_full {
        class.push_str(" rounded-full")
    };
    match props.status {
        AvatarStatus::None => view! {
            <Image props=props.image.class(class) />
        },
        _ => panic!(""),
    }
}

/*
 <div class="relative">
            <img class="w-10 h-10 rounded-full" src="/docs/images/people/profile-picture-5.jpg" alt="" />
            <span class="top-0 left-7 absolute  w-3.5 h-3.5 bg-green-400 border-2 border-white dark:border-gray-800 rounded-full"></span>
        </div>
        <div class="relative">
            <img class="w-10 h-10 rounded" src="/docs/images/people/profile-picture-5.jpg" alt="" />
            <span class="absolute top-0 left-8 transform -translate-y-1/2 w-3.5 h-3.5 bg-red-400 border-2 border-white dark:border-gray-800 rounded-full"></span>
        </div>
        <div class="relative">
            <img class="w-10 h-10 rounded-full" src="/docs/images/people/profile-picture-5.jpg" alt="" />
            <span class="bottom-0 left-7 absolute  w-3.5 h-3.5 bg-green-400 border-2 border-white dark:border-gray-800 rounded-full"></span>
        </div>
        <div class="relative">
            <img class="w-10 h-10 rounded" src="/docs/images/people/profile-picture-5.jpg" alt="" />
            <span class="absolute bottom-0 left-8 transform translate-y-1/4 w-3.5 h-3.5 bg-green-400 border-2 border-white dark:border-gray-800 rounded-full"></span>
        </div>
*/
