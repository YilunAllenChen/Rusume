use web_sys::HtmlInputElement;
use yew::prelude::*;

use super::html_utils::{
    ADD_BUTTON_CLASS, INPUT_CLASS, LABEL_CLASS, REMOVE_BUTTON_CLASS, SECTION_HEADER_CLASS,
};

#[derive(Debug, PartialEq, Clone)]
pub struct OpenSource {
    name: String,
    url: String,
}

pub enum OpenSourceField {
    Name(String),
    Url(String),
}

#[derive(Properties, PartialEq)]
pub struct OpenSourceControllerProps {
    pub callback: Callback<Vec<OpenSource>>,
}

pub enum OpenSourceMsg {
    AddOpenSource,
    RemoveOpenSource(usize),
    UpdateField(usize, OpenSourceField),
}

pub struct OpenSourceController {
    open_sources: Vec<OpenSource>,
}

impl Component for OpenSourceController {
    type Message = OpenSourceMsg;
    type Properties = OpenSourceControllerProps;

    fn create(ctx: &Context<Self>) -> Self {
        let init = vec![
            OpenSource {
                name: "kafka-rust #222".to_string(),
                url: "https://github.com/kafka-rust/kafka-rust/pull/222".to_string(),
            },
            OpenSource {
                name: "kafka-rust #223".to_string(),
                url: "https://github.com/kafka-rust/kafka-rust/pull/223".to_string(),
            },
            OpenSource {
                name: "gleam-stdlib #769".to_string(),
                url: "https://github.com/gleam-lang/stdlib/pull/769".to_string(),
            },
        ];
        let slf = Self { open_sources: init };
        ctx.props().callback.emit(slf.open_sources.clone());
        slf
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            OpenSourceMsg::AddOpenSource => {
                self.open_sources.push(OpenSource {
                    name: "".to_string(),
                    url: "".to_string(),
                });
            }
            OpenSourceMsg::UpdateField(index, field) => {
                let open_source = self.open_sources.get_mut(index as usize).unwrap();
                match field {
                    OpenSourceField::Name(name) => open_source.name = name,
                    OpenSourceField::Url(url) => open_source.url = url,
                }
            }
            OpenSourceMsg::RemoveOpenSource(index) => {
                self.open_sources.remove(index as usize);
            }
        }
        ctx.props().callback.emit(self.open_sources.clone());
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let add_open_source = ctx.link().callback(|_| OpenSourceMsg::AddOpenSource);
        let inputs = self
            .open_sources
            .iter()
            .enumerate()
            .map(|(idx, open_source)| {
                let remove_open_source = ctx
                    .link()
                    .callback(move |_| OpenSourceMsg::RemoveOpenSource(idx));

                let name_input = make_input(
                    ctx,
                    idx,
                    "Name".to_string(),
                    open_source.name.clone(),
                    OpenSourceField::Name,
                );

                let url_input = make_input(
                    ctx,
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
}

fn make_input<F>(
    ctx: &Context<OpenSourceController>,
    idx: usize,
    name: String,
    value: String,
    cons: F,
) -> Html
where
    F: Fn(String) -> OpenSourceField + 'static,
{
    let callback = ctx.link().callback(move |e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        OpenSourceMsg::UpdateField(idx, cons(input.value()))
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
