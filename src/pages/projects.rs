use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;

use super::html_utils::{
    ADD_BUTTON_CLASS, INPUT_CLASS, INPUT_DIVIDER_CLASS, INPUT_FIELD_WRAPPER_CLASS,
    INPUT_SECTION_CLASS, INPUT_SECTION_ROW_CLASS, LABEL_CLASS, REMOVE_BUTTON_CLASS,
    SECTION_HEADER_CLASS,
};

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct ProjectLink {
    pub name: String,
    pub url: String,
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub technologies: String,
    pub url: Option<String>,
    #[serde(default)]
    pub links: Vec<ProjectLink>,
}

pub enum ProjectField {
    Name(String),
    Description(String),
    Technologies(String),
    Url(String),
}

#[derive(Properties, PartialEq)]
pub struct ProjectControllerProps {
    pub value: Vec<Project>,
    pub on_change: Callback<Vec<Project>>,
}

#[function_component(ProjectController)]
pub fn project_controller(props: &ProjectControllerProps) -> Html {
    let add_project = {
        let projects = props.value.clone();
        let on_change = props.on_change.clone();
        Callback::from(move |_| {
            let mut next = projects.clone();
            next.push(Project::default());
            on_change.emit(next);
        })
    };

    let inputs = props
        .value
        .iter()
        .enumerate()
        .map(|(idx, project)| {
            let remove_project = {
                let projects = props.value.clone();
                let on_change = props.on_change.clone();
                Callback::from(move |_| {
                    let mut next = projects.clone();
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
                project.name.clone(),
                ProjectField::Name,
            );

            let description_input = make_input(
                props,
                idx,
                "Description".to_string(),
                project.description.clone(),
                ProjectField::Description,
            );

            let technologies_input = make_input(
                props,
                idx,
                "Technologies".to_string(),
                project.technologies.clone(),
                ProjectField::Technologies,
            );

            let url_input = make_input(
                props,
                idx,
                "Url".to_string(),
                project.url.clone().unwrap_or_default(),
                ProjectField::Url,
            );

            let add_link = {
                let projects = props.value.clone();
                let on_change = props.on_change.clone();
                Callback::from(move |_| {
                    let mut next = projects.clone();
                    if let Some(project) = next.get_mut(idx) {
                        project.links.push(ProjectLink::default());
                        on_change.emit(next);
                    }
                })
            };

            let link_rows = project.links.iter().enumerate().map(|(link_idx, link)| {
                let update_link_name = {
                    let projects = props.value.clone();
                    let on_change = props.on_change.clone();
                    Callback::from(move |e: web_sys::InputEvent| {
                        let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                        let mut next = projects.clone();
                        if let Some(project) = next.get_mut(idx) {
                            if let Some(link) = project.links.get_mut(link_idx) {
                                link.name = input.value();
                                on_change.emit(next);
                            }
                        }
                    })
                };
                let update_link_url = {
                    let projects = props.value.clone();
                    let on_change = props.on_change.clone();
                    Callback::from(move |e: web_sys::InputEvent| {
                        let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                        let mut next = projects.clone();
                        if let Some(project) = next.get_mut(idx) {
                            if let Some(link) = project.links.get_mut(link_idx) {
                                link.url = input.value();
                                on_change.emit(next);
                            }
                        }
                    })
                };
                let remove_link = {
                    let projects = props.value.clone();
                    let on_change = props.on_change.clone();
                    Callback::from(move |_| {
                        let mut next = projects.clone();
                        if let Some(project) = next.get_mut(idx) {
                            if link_idx < project.links.len() {
                                project.links.remove(link_idx);
                                on_change.emit(next);
                            }
                        }
                    })
                };
                html! {
                    <div class="border border-slate-800/80 rounded-lg p-0.5 my-0.5">
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-1">
                            <div class="relative flex-1">
                                <input type="text" class={INPUT_CLASS} value={link.name.clone()} oninput={update_link_name} />
                                <label class={LABEL_CLASS}>{"Link Name"}</label>
                            </div>
                            <div class="relative flex-1">
                                <input type="text" class={INPUT_CLASS} value={link.url.clone()} oninput={update_link_url} />
                                <label class={LABEL_CLASS}>{"Link URL"}</label>
                            </div>
                        </div>
                        <div class="mt-0 flex justify-end">
                            <button class={REMOVE_BUTTON_CLASS} onclick={remove_link}>{"Remove Link"}</button>
                        </div>
                    </div>
                }
            }).collect::<Html>();

            html! {
                <>
                    {name_input}
                    {description_input}
                    {technologies_input}
                    {url_input}
                    <div class={INPUT_FIELD_WRAPPER_CLASS}>
                        <div class="flex items-center justify-between px-1 py-1">
                            <div class="text-xs tracking-wide uppercase text-slate-400 font-semibold">{"Links"}</div>
                            <button class="rounded-md px-3 py-2 text-xs font-semibold text-slate-950 bg-emerald-400 hover:bg-emerald-300" onclick={add_link}>
                                {"Add Link"}
                            </button>
                        </div>
                        {link_rows}
                    </div>
                    <button
                        class={REMOVE_BUTTON_CLASS}
                        onclick={remove_project}
                    >
                        {"Remove"}
                    </button>
                </>
            }
        })
        .collect::<Html>();
    html! {
        <>
            <div class={INPUT_SECTION_ROW_CLASS}>
                <h5 class={INPUT_SECTION_CLASS}> {"Projects"} </h5>
                <button
                    class={ADD_BUTTON_CLASS}
                    onclick={add_project}
                >
                    {"Add New"}
                </button>
            </div>
            <div class={INPUT_DIVIDER_CLASS}></div>
            {inputs}
        </>
    }
}

fn make_input<F>(
    props: &ProjectControllerProps,
    idx: usize,
    name: String,
    value: String,
    cons: F,
) -> Html
where
    F: Fn(String) -> ProjectField + 'static,
{
    let projects = props.value.clone();
    let on_change = props.on_change.clone();
    let callback = Callback::from(move |e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        let mut next = projects.clone();
        if let Some(project) = next.get_mut(idx) {
            match cons(input.value()) {
                ProjectField::Name(name) => project.name = name,
                ProjectField::Description(description) => project.description = description,
                ProjectField::Technologies(technologies) => project.technologies = technologies,
                ProjectField::Url(url) => project.url = Some(url),
            }
            on_change.emit(next);
        }
    });
    html! {
    <div class={INPUT_FIELD_WRAPPER_CLASS}>
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
    pub projects: Vec<Project>,
}

#[function_component(ProjectViewer)]
pub fn view_project(props: &Props) -> Html {
    if props.projects.is_empty() {
        return html! {};
    }
    let projects = props
        .projects
        .iter()
        .map(|project| {
            let name_maybe_with_url = if project.url.is_some() {
                html! { <a href={project.url.clone().unwrap_or_default()} class="text-blue-600"> {project.name.clone()} </a> }
            } else {
                html! { <span> {project.name.clone()} </span> }
            };
            let links = if !project.links.is_empty() {
                let link_items = project.links.iter().enumerate().map(|(i, link)| {
                    let sep = if i > 0 { " · " } else { "" };
                    html! {
                        <>
                            {sep}
                            <a href={link.url.clone()} class="text-blue-600">{link.name.clone()}</a>
                        </>
                    }
                }).collect::<Html>();
                html! { <span class="ml-2">{link_items}</span> }
            } else {
                html! {}
            };
            let row_class = if project.links.is_empty() { "flex justify-between" } else { "flex justify-between mb-1" };
            html! {
                <div class={row_class}>
                    <div>
                        {name_maybe_with_url}
                        if !project.description.is_empty() {
                            <span>{" : "}{project.description.clone()}</span>
                        }
                        {links}
                    </div>
                    <div class="font-bold"> {project.technologies.clone()} </div>
                </div>
            }
        })
        .collect::<Html>();
    html! {
        <>
        <h5 class={SECTION_HEADER_CLASS}> {"Personal Projects"} </h5>
        <div class="m-2 flex flex-col space-y-[0.5]">
            {projects}
        </div>
        </>
    }
}
