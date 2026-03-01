use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;

use super::html_utils::{
    ADD_BUTTON_CLASS, INPUT_CLASS, LABEL_CLASS, REMOVE_BUTTON_CLASS, SECTION_HEADER_CLASS,
};

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub technologies: String,
    pub url: Option<String>,
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

            html! {
                <div class="space-y-2 rounded-lg bg-slate-100 p-4 my-4">
                    {name_input}
                    {description_input}
                    {technologies_input}
                    {url_input}
                    <button
                        class={REMOVE_BUTTON_CLASS}
                        onclick={remove_project}
                    >
                        {"Remove"}
                    </button>
                </div>
            }
        })
        .collect::<Html>();
    html! {
        <>
            <h5 class="text-xl font-bold text-left self-center pl-4 mt-4 mb-1"> {"Projects"} </h5>
            <hr/>
            {inputs}
            <button
                class={ADD_BUTTON_CLASS}
                onclick={add_project}
            >
                {"Add Project"}
            </button>
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
    <>
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
    </>
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
                html! { <div> {project.name.clone()} </div> }
            };
            html! {
                <div class="flex justify-between">
                    <div>
                        {name_maybe_with_url} {" : "}
                        {project.description.clone()}
                    </div>
                    <div class="font-bold"> {project.technologies.clone()} </div>
                </div>
            }
        })
        .collect::<Html>();
    html! {
        <>
        <h5 class={SECTION_HEADER_CLASS}> {"Projects"} </h5>
        <div class="m-2 flex flex-col space-y-[0.5]">
            {projects}
        </div>
        </>
    }
}
