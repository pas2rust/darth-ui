use darth_rust::DarthRust;
use leptos::*;

#[derive(DarthRust, Default, Clone)]
pub struct LabelBuild {
    class: &'static str,
    id: &'static str,
    pub name: &'static str,
    forhtml: &'static str,
}

#[component]
pub fn Label(props: LabelBuild) -> impl IntoView {
    view! {
        <label class=props.class for=props.forhtml name=props.name id=props.id>
            {props.name}:
        </label>
    }
}
