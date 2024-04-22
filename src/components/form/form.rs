use leptos::{leptos_dom::logging::console_warn, *};
use std::collections::HashMap;
use wasm_bindgen::JsCast;
use web_sys::{HtmlFormElement, HtmlInputElement};
pub struct FormBuild {
    class: &'static str,
    submit: fn(HashMap<String, String>, form: HtmlFormElement),
}

impl FormBuild {
    pub fn new() -> Self {
        Self {
            class: "",
            submit: |_, _| {},
        }
    }

    pub fn class(mut self, class: &'static str) -> Self {
        self.class = class;
        self
    }

    pub fn submit(mut self, f: fn(HashMap<String, String>, form: HtmlFormElement)) -> Self {
        self.submit = f;
        self
    }
}

type Hash = (
    ReadSignal<HashMap<String, String>>,
    WriteSignal<HashMap<String, String>>,
);

#[component]
pub fn Form(props: FormBuild, children: Children) -> impl IntoView {
    let (read_hash, write_hash): Hash = create_signal(HashMap::new());

    view! {
       <form class=props.class on:submit=move |ev| {
         ev.prevent_default();
         let target = ev.target().unwrap();
         let form: HtmlFormElement = target.dyn_into().expect("Failed to cast target to HtmlFormElement");
         let collection = form.get_elements_by_tag_name("input");
         for i in 0..collection.length() {
            let element = collection.item(i).expect("Failed to get item from HtmlCollection");
            let input: HtmlInputElement = element.dyn_into().expect("Failed to cast Element to HtmlInputElement");
            write_hash.update(|old| {
               let name = input.name();
               let value = input.value();
               old.insert(name, value);
            })
         }
         let submit = props.submit;
         submit(read_hash.get(), form);
         console_warn(format!("{:#?}", read_hash.get()).as_str())
       }>
        {children()}
       </form>
    }
}
