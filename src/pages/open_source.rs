use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;

use super::html_utils::{
    ADD_BUTTON_CLASS, INPUT_CLASS, LABEL_CLASS, REMOVE_BUTTON_CLASS, SECTION_HEADER_CLASS,
};

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct OpenSource {
    pub name: String,
    pub url: String,
}

pub enum OpenSourceField {
    Name(String),
    Url(String),
}

#[derive(Properties, PartialEq)]
pub struct OpenSourceControllerProps {
    pub value: Vec<OpenSource>,
    pub on_change: Callback<Vec<OpenSource>>,
}

#[function_component(OpenSourceController)]
pub fn open_source_controller(props: &OpenSourceControllerProps) -> Html {
    let add_open_source = {
        let open_sources = props.value.clone();
        let on_change = props.on_change.clone();
        Callback::from(move |_| {
            let mut next = open_sources.clone();
            next.push(OpenSource::default());
            on_change.emit(next);
        })
    };

    let inputs = props
        .value
        .iter()
        .enumerate()
        .map(|(idx, open_source)| {
            let remove_open_source = {
                let open_sources = props.value.clone();
                let on_change = props.on_change.clone();
                Callback::from(move |_| {
                    let mut next = open_sources.clone();
                    if idx < next.len() {
                        next.remove(idx);
                        on_change.emit(next);
                    }
                })
            };

            let name_input = make_input(
                props,
                idx,
                "Name".to_string(),
                open_source.name.clone(),
                OpenSourceField::Name,
            );

            let url_input = make_input(
                props,
                idx,
                "URL".to_string(),
                open_source.url.clone(),
                OpenSourceField::Url,
            );

            html! {
                <div class="space-y-2 rounded-lg bg-slate-100 p-4 my-4">
                    {name_input}
                    {url_input}
                    <button
                        class={REMOVE_BUTTON_CLASS}
                        onclick={remove_open_source}
                    >
                        {"Remove"}
                    </button>
                </div>
            }
        })
        .collect::<Html>();

    html! {
        <>
            <h5 class="text-xl font-bold text-left self-center pl-4 mt-4 mb-1"> {"Open Source"} </h5>
            <hr/>
            {inputs}
            <button
                class={ADD_BUTTON_CLASS}
                onclick={add_open_source}
            >
                {"Add Open Source"}
            </button>
        </>
    }
}

fn make_input<F>(
    props: &OpenSourceControllerProps,
    idx: usize,
    name: String,
    value: String,
    cons: F,
) -> Html
where
    F: Fn(String) -> OpenSourceField + 'static,
{
    let open_sources = props.value.clone();
    let on_change = props.on_change.clone();
    let callback = Callback::from(move |e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        let mut next = open_sources.clone();
        if let Some(open_source) = next.get_mut(idx) {
            match cons(input.value()) {
                OpenSourceField::Name(name) => open_source.name = name,
                OpenSourceField::Url(url) => open_source.url = url,
            }
            on_change.emit(next);
        }
    });
    html! {
    <div class="m-2">
        <div class="relative">
            <input type="text"
                   id={name.clone()}
                   oninput={callback}
                   class={INPUT_CLASS}
                   value={value}
            />
            <label for={name.clone()}
                   class={LABEL_CLASS}>
                {name}
            </label>
        </div>
    </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub open_sources: Vec<OpenSource>,
}

#[function_component(OpenSourceViewer)]
pub fn view_open_source(props: &Props) -> Html {
    if props.open_sources.is_empty() {
        return html! {};
    }
    let open_sources = props
        .open_sources
        .iter()
        .map(|open_source| {
            html! {
                <a href={open_source.url.clone()} class="text-blue-600 mx-2">
                    {format!("[ {} ]", open_source.name)}
                </a>
            }
        })
        .collect::<Html>();
    html! {
        <>
            <h5 class={SECTION_HEADER_CLASS}> {"Open Source"} </h5>
            <div class="mx-2 my-2 flex flex-wrap">
                {open_sources}
            </div>
        </>
    }
}
