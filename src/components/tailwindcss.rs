use std::fmt::format;

use darth_rust::DarthRust;

#[derive(DarthRust)]
pub struct TailwindCSS {
    rounded_md: bool,
    rounded_full: bool,
    rounded_sm: bool,
    m: Option<i32>,
    mx: Option<i32>,
    my: Option<i32>,
    ml: Option<i32>,
    mr: Option<i32>,
    mt: Option<i32>,
    mb: Option<i32>,
    p: Option<i32>,
    px: Option<i32>,
    py: Option<i32>,
    pl: Option<i32>,
    pr: Option<i32>,
    pt: Option<i32>,
    pb: Option<i32>,
    w: Option<i32>,
    h: Option<i32>,
    bg: Option<(String, u32)>,
    class: Option<String>,
}

impl TailwindCSS {
    pub fn output(&self) -> String {
        let mut classes: Vec<String> = Vec::new();
        if self.rounded_md {
            classes.push("rounded-md".to_string());
        }
        if self.rounded_full {
            classes.push("rounded-full".to_string());
        }
        if self.rounded_sm {
            classes.push("rounded-sm".to_string());
        }
        if let Some(m) = &self.m {
            classes.push(format!("m-{}", m));
        }
        if let Some(ml) = &self.ml {
            classes.push(format!("ml-{}", ml));
        }
        if let Some(mr) = &self.mr {
            classes.push(format!("mr-{}", mr));
        }
        if let Some(mt) = &self.mt {
            classes.push(format!("mt-{}", mt));
        }
        if let Some(mb) = &self.mb {
            classes.push(format!("mb-{}", mb));
        }
        if let Some(p) = &self.p {
            classes.push(format!("p-{}", p));
        }
        if let Some(pl) = &self.pl {
            classes.push(format!("pl-{}", pl));
        }
        if let Some(pr) = &self.pr {
            classes.push(format!("pr-{}", pr));
        }
        if let Some(pt) = &self.pt {
            classes.push(format!("pt-{}", pt));
        }
        if let Some(pb) = &self.pb {
            classes.push(format!("pb-{}", pb));
        }
        if let Some(w) = &self.w {
            classes.push(format!("w-{}", w));
        }
        if let Some(h) = &self.h {
            classes.push(format!("h-{}", h));
        } 
        if let Some((color, tone)) = &self.bg {
            classes.push(format!("bg-{}-{}", color, tone))
        }
        if let Some(class) = &self.class {
            classes.push(class.clone());
        }
        
        classes.join(" ")
    }
}