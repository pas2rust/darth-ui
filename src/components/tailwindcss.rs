use std::fmt::format;

use darth_rust::DarthRust;

#[derive(Debug, Default)]
enum TailwindColor {
    Red50, Red100, Red200, Red300, Red400, #[default]Red500, Red600, Red700, Red800, Red900,
    Pink50, Pink100, Pink200, Pink300, Pink400, Pink500, Pink600, Pink700, Pink800, Pink900,
    Purple50, Purple100, Purple200, Purple300, Purple400, Purple500, Purple600, Purple700, Purple800, Purple900,
    DeepPurple50, DeepPurple100, DeepPurple200, DeepPurple300, DeepPurple400, DeepPurple500, DeepPurple600, DeepPurple700, DeepPurple800, DeepPurple900,
    Indigo50, Indigo100, Indigo200, Indigo300, Indigo400, Indigo500, Indigo600, Indigo700, Indigo800, Indigo900,
    Blue50, Blue100, Blue200, Blue300, Blue400, Blue500, Blue600, Blue700, Blue800, Blue900,
    LightBlue50, LightBlue100, LightBlue200, LightBlue300, LightBlue400, LightBlue500, LightBlue600, LightBlue700, LightBlue800, LightBlue900,
    Cyan50, Cyan100, Cyan200, Cyan300, Cyan400, Cyan500, Cyan600, Cyan700, Cyan800, Cyan900,
    Teal50, Teal100, Teal200, Teal300, Teal400, Teal500, Teal600, Teal700, Teal800, Teal900,
    Green50, Green100, Green200, Green300, Green400, Green500, Green600, Green700, Green800, Green900,
    LightGreen50, LightGreen100, LightGreen200, LightGreen300, LightGreen400, LightGreen500, LightGreen600, LightGreen700, LightGreen800, LightGreen900,
    Lime50, Lime100, Lime200, Lime300, Lime400, Lime500, Lime600, Lime700, Lime800, Lime900,
    Yellow50, Yellow100, Yellow200, Yellow300, Yellow400, Yellow500, Yellow600, Yellow700, Yellow800, Yellow900,
    Amber50, Amber100, Amber200, Amber300, Amber400, Amber500, Amber600, Amber700, Amber800, Amber900,
    Orange50, Orange100, Orange200, Orange300, Orange400, Orange500, Orange600, Orange700, Orange800, Orange900,
    DeepOrange50, DeepOrange100, DeepOrange200, DeepOrange300, DeepOrange400, DeepOrange500, DeepOrange600, DeepOrange700, DeepOrange800, DeepOrange900,
    Brown50, Brown100, Brown200, Brown300, Brown400, Brown500, Brown600, Brown700, Brown800, Brown900,
    Grey50, Grey100, Grey200, Grey300, Grey400, Grey500, Grey600, Grey700, Grey800, Grey900,
    BlueGrey50, BlueGrey100, BlueGrey200, BlueGrey300, BlueGrey400, BlueGrey500, BlueGrey600, BlueGrey700, BlueGrey800, BlueGrey900,
}

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
    bg: Option<TailwindColor>,
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
        if let Some(tailwind_color) = &self.bg {
            classes.push(format!("bg-{}", tailwind_color.to_string()))
        }
        if let Some(class) = &self.class {
            classes.push(class.clone());
        }
        
        classes.join(" ")
    }
}

impl TailwindColor {
    fn to_string(&self) -> String {
        match &self {
            TailwindColor::Red50 => "red-50",
            TailwindColor::Red100 => "red-100",
            TailwindColor::Red200 => "red-200",
            TailwindColor::Red300 => "red-300",
            TailwindColor::Red400 => "red-400",
            TailwindColor::Red500 => "red-500",
            TailwindColor::Red600 => "red-600",
            TailwindColor::Red700 => "red-700",
            TailwindColor::Red800 => "red-800",
            TailwindColor::Red900 => "red-900",
            TailwindColor::Pink50 => "pink-50",
            TailwindColor::Pink100 => "pink-100",
            TailwindColor::Pink200 => "pink-200",
            TailwindColor::Pink300 => "pink-300",
            TailwindColor::Pink400 => "pink-400",
            TailwindColor::Pink500 => "pink-500",
            TailwindColor::Pink600 => "pink-600",
            TailwindColor::Pink700 => "pink-700",
            TailwindColor::Pink800 => "pink-800",
            TailwindColor::Pink900 => "pink-900",
            TailwindColor::Purple50 => "purple-50",
            TailwindColor::Purple100 => "purple-100",
            TailwindColor::Purple200 => "purple-200",
            TailwindColor::Purple300 => "purple-300",
            TailwindColor::Purple400 => "purple-400",
            TailwindColor::Purple500 => "purple-500",
            TailwindColor::Purple600 => "purple-600",
            TailwindColor::Purple700 => "purple-700",
            TailwindColor::Purple800 => "purple-800",
            TailwindColor::Purple900 => "purple-900",
            TailwindColor::DeepPurple50 => "deep-purple-50",
            TailwindColor::DeepPurple100 => "deep-purple-100",
            TailwindColor::DeepPurple200 => "deep-purple-200",
            TailwindColor::DeepPurple300 => "deep-purple-300",
            TailwindColor::DeepPurple400 => "deep-purple-400",
            TailwindColor::DeepPurple500 => "deep-purple-500",
            TailwindColor::DeepPurple600 => "deep-purple-600",
            TailwindColor::DeepPurple700 => "deep-purple-700",
            TailwindColor::DeepPurple800 => "deep-purple-800",
            TailwindColor::DeepPurple900 => "deep-purple-900",
            TailwindColor::Indigo50 => "indigo-50",
            TailwindColor::Indigo100 => "indigo-100",
            TailwindColor::Indigo200 => "indigo-200",
            TailwindColor::Indigo300 => "indigo-300",
            TailwindColor::Indigo400 => "indigo-400",
            TailwindColor::Indigo500 => "indigo-500",
            TailwindColor::Indigo600 => "indigo-600",
            TailwindColor::Indigo700 => "indigo-700",
            TailwindColor::Indigo800 => "indigo-800",
            TailwindColor::Indigo900 => "indigo-900",
            TailwindColor::Blue50 => "blue-50",
            TailwindColor::Blue100 => "blue-100",
            TailwindColor::Blue200 => "blue-200",
            TailwindColor::Blue300 => "blue-300",
            TailwindColor::Blue400 => "blue-400",
            TailwindColor::Blue500 => "blue-500",
            TailwindColor::Blue600 => "blue-600",
            TailwindColor::Blue700 => "blue-700",
            TailwindColor::Blue800 => "blue-800",
            TailwindColor::Blue900 => "blue-900",
            TailwindColor::LightBlue50 => "light-blue-50",
            TailwindColor::LightBlue100 => "light-blue-100",
            TailwindColor::LightBlue200 => "light-blue-200",
            TailwindColor::LightBlue300 => "light-blue-300",
            TailwindColor::LightBlue400 => "light-blue-400",
            TailwindColor::LightBlue500 => "light-blue-500",
            TailwindColor::LightBlue600 => "light-blue-600",
            TailwindColor::LightBlue700 => "light-blue-700",
            TailwindColor::LightBlue800 => "light-blue-800",
            TailwindColor::LightBlue900 => "light-blue-900",
            TailwindColor::Cyan50 => "cyan-50",
            TailwindColor::Cyan100 => "cyan-100",
            TailwindColor::Cyan200 => "cyan-200",
            TailwindColor::Cyan300 => "cyan-300",
            TailwindColor::Cyan400 => "cyan-400",
            TailwindColor::Cyan500 => "cyan-500",
            TailwindColor::Cyan600 => "cyan-600",
            TailwindColor::Cyan700 => "cyan-700",
            TailwindColor::Cyan800 => "cyan-800",
            TailwindColor::Cyan900 => "cyan-900",
            TailwindColor::Teal50 => "teal-50",
            TailwindColor::Teal100 => "teal-100",
            TailwindColor::Teal200 => "teal-200",
            TailwindColor::Teal300 => "teal-300",
            TailwindColor::Teal400 => "teal-400",
            TailwindColor::Teal500 => "teal-500",
            TailwindColor::Teal600 => "teal-600",
            TailwindColor::Teal700 => "teal-700",
            TailwindColor::Teal800 => "teal-800",
            TailwindColor::Teal900 => "teal-900",
            TailwindColor::Green50 => "green-50",
            TailwindColor::Green100 => "green-100",
            TailwindColor::Green200 => "green-200",
            TailwindColor::Green300 => "green-300",
            TailwindColor::Green400 => "green-400",
            TailwindColor::Green500 => "green-500",
            TailwindColor::Green600 => "green-600",
            TailwindColor::Green700 => "green-700",
            TailwindColor::Green800 => "green-800",
            TailwindColor::Green900 => "green-900",
            TailwindColor::LightGreen50 => "light-green-50",
            TailwindColor::LightGreen100 => "light-green-100",
            TailwindColor::LightGreen200 => "light-green-200",
            TailwindColor::LightGreen300 => "light-green-300",
            TailwindColor::LightGreen400 => "light-green-400",
            TailwindColor::LightGreen500 => "light-green-500",
            TailwindColor::LightGreen600 => "light-green-600",
            TailwindColor::LightGreen700 => "light-green-700",
            TailwindColor::LightGreen800 => "light-green-800",
            TailwindColor::LightGreen900 => "light-green-900",
            TailwindColor::Lime50 => "lime-50",
            TailwindColor::Lime100 => "lime-100",
            TailwindColor::Lime200 => "lime-200",
            TailwindColor::Lime300 => "lime-300",
            TailwindColor::Lime400 => "lime-400",
            TailwindColor::Lime500 => "lime-500",
            TailwindColor::Lime600 => "lime-600",
            TailwindColor::Lime700 => "lime-700",
            TailwindColor::Lime800 => "lime-800",
            TailwindColor::Lime900 => "lime-900",
            TailwindColor::Yellow50 => "yellow-50",
            TailwindColor::Yellow100 => "yellow-100",
            TailwindColor::Yellow200 => "yellow-200",
            TailwindColor::Yellow300 => "yellow-300",
            TailwindColor::Yellow400 => "yellow-400",
            TailwindColor::Yellow500 => "yellow-500",
            TailwindColor::Yellow600 => "yellow-600",
            TailwindColor::Yellow700 => "yellow-700",
            TailwindColor::Yellow800 => "yellow-800",
            TailwindColor::Yellow900 => "yellow-900",
            TailwindColor::Amber50 => "amber-50",
            TailwindColor::Amber100 => "amber-100",
            TailwindColor::Amber200 => "amber-200",
            TailwindColor::Amber300 => "amber-300",
            TailwindColor::Amber400 => "amber-400",
            TailwindColor::Amber500 => "amber-500",
            TailwindColor::Amber600 => "amber-600",
            TailwindColor::Amber700 => "amber-700",
            TailwindColor::Amber800 => "amber-800",
            TailwindColor::Amber900 => "amber-900",
            TailwindColor::Orange50 => "orange-50",
            TailwindColor::Orange100 => "orange-100",
            TailwindColor::Orange200 => "orange-200",
            TailwindColor::Orange300 => "orange-300",
            TailwindColor::Orange400 => "orange-400",
            TailwindColor::Orange500 => "orange-500",
            TailwindColor::Orange600 => "orange-600",
            TailwindColor::Orange700 => "orange-700",
            TailwindColor::Orange800 => "orange-800",
            TailwindColor::Orange900 => "orange-900",
            TailwindColor::DeepOrange50 => "deep-orange-50",
            TailwindColor::DeepOrange100 => "deep-orange-100",
            TailwindColor::DeepOrange200 => "deep-orange-200",
            TailwindColor::DeepOrange300 => "deep-orange-300",
            TailwindColor::DeepOrange400 => "deep-orange-400",
            TailwindColor::DeepOrange500 => "deep-orange-500",
            TailwindColor::DeepOrange600 => "deep-orange-600",
            TailwindColor::DeepOrange700 => "deep-orange-700",
            TailwindColor::DeepOrange800 => "deep-orange-800",
            TailwindColor::DeepOrange900 => "deep-orange-900",
            TailwindColor::Brown50 => "brown-50",
            TailwindColor::Brown100 => "brown-100",
            TailwindColor::Brown200 => "brown-200",
            TailwindColor::Brown300 => "brown-300",
            TailwindColor::Brown400 => "brown-400",
            TailwindColor::Brown500 => "brown-500",
            TailwindColor::Brown600 => "brown-600",
            TailwindColor::Brown700 => "brown-700",
            TailwindColor::Brown800 => "brown-800",
            TailwindColor::Brown900 => "brown-900",
            TailwindColor::Grey50 => "grey-50",
            TailwindColor::Grey100 => "grey-100",
            TailwindColor::Grey200 => "grey-200",
            TailwindColor::Grey300 => "grey-300",
            TailwindColor::Grey400 => "grey-400",
            TailwindColor::Grey500 => "grey-500",
            TailwindColor::Grey600 => "grey-600",
            TailwindColor::Grey700 => "grey-700",
            TailwindColor::Grey800 => "grey-800",
            TailwindColor::Grey900 => "grey-900",
            TailwindColor::BlueGrey50 => "blue-grey-50",
            TailwindColor::BlueGrey100 => "blue-grey-100",
            TailwindColor::BlueGrey200 => "blue-grey-200",
            TailwindColor::BlueGrey300 => "blue-grey-300",
            TailwindColor::BlueGrey400 => "blue-grey-400",
            TailwindColor::BlueGrey500 => "blue-grey-500",
            TailwindColor::BlueGrey600 => "blue-grey-600",
            TailwindColor::BlueGrey700 => "blue-grey-700",
            TailwindColor::BlueGrey800 => "blue-grey-800",
            TailwindColor::BlueGrey900 => "blue-grey-900",
        }.to_string()
    }
}