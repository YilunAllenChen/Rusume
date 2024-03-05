use web_sys::HtmlInputElement;
use yew::prelude::*;

use super::html_utils::{
    ADD_BUTTON_CLASS, INPUT_CLASS, LABEL_CLASS, REMOVE_BUTTON_CLASS, SECTION_HEADER_CLASS,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Project {
    name: String,
    description: String,
    technologies: String,
    url: Option<String>,
}

pub enum ProjectField {
    Name(String),
    Description(String),
    Technologies(String),
    Url(String),
}

#[derive(Properties, PartialEq)]
pub struct ProjectControllerProps {
    pub callback: Callback<Vec<Project>>,
}

pub enum ProjectMsg {
    AddProject,
    RemoveProject(usize),
    UpdateField(usize, ProjectField),
}

pub struct ProjectController {
    projects: Vec<Project>,
}

impl Component for ProjectController {
    type Message = ProjectMsg;
    type Properties = ProjectControllerProps;

    fn create(ctx: &Context<Self>) -> Self {
        let init = vec![
            Project {
                name: "Incrementars".to_string(),
                description: "Incremental / self-adapting computing framework for Rust.".to_string(),
                technologies: "Rust".to_string(),
                url: Some("https://github.com/YilunAllenChen/incrementars".to_string())
            },
            Project {
                name: "Museum of Code".to_string(),
                description: "Educational web app caputuring the beauty of programming."
                    .to_string(),
                technologies: "Rust/Yew".to_string(),
                url: Some("https://yilunallenchen.github.io/museum_of_code/".to_string()),
            },
            Project {
                name: "Exchange Simulator".to_string(),
                description: "FIFO matching engine with materialized market impacts.".to_string(),
                technologies: "Python, JavaScript".to_string(),
                url: Some("https://tradingsim.allenchen.dev/".to_string()),
            },
            Project {
                name: "Project Karage".to_string(),
                description: "Toy trading platform with market-making / arbitrage bot.".to_string(),
                technologies: "Rust".to_string(),
                url: Some("https://github.com/YilunAllenChen/Karage".to_string()),
            },
            Project {
                name: "DaVinci Ergo Lab".to_string(),
                description: "Ergonomic split mechanical keyboards built from the ground up. "
                    .to_string(),
                technologies: "Python, C++".to_string(),
                url: Some("https://davinci-ergo-lab.com/".to_string()),
            },
            Project {
                name: "PDE-based Anti-Aliasing".to_string(),
                description: "Enhance computer graphics with partial differential equations. "
                    .to_string(),
                technologies: "Python".to_string(),
                url: Some("https://github.com/YilunAllenChen/Dowwin_legacy/".to_string()),
            },
            Project {
                name: "SDC in GTAV".to_string(),
                description: "Self-driving cars in Grant Theft Auto V.".to_string(),
                technologies: "Python, C++".to_string(),
                url: Some("https://github.com/YilunAllenChen/GTAV_SDC/".to_string()),
            },
            Project {
                name: "Rusume".to_string(),
                description: "Real-time resume builder that was used to craft this very resume"
                    .to_string(),
                technologies: "Rust/Yew".to_string(),
                url: Some("https://yilunallenchen.github.io/Rusume/#/".to_string()),
            },
        ];
        let slf = Self { projects: init };
        ctx.props().callback.emit(slf.projects.clone());
        slf
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ProjectMsg::AddProject => {
                self.projects.push(Project {
                    name: "".to_string(),
                    description: "".to_string(),
                    technologies: "".to_string(),
                    url: None,
                });
            }
            ProjectMsg::UpdateField(index, field) => {
                let project = self.projects.get_mut(index as usize).unwrap();
                match field {
                    ProjectField::Name(name) => project.name = name,
                    ProjectField::Description(description) => project.description = description,
                    ProjectField::Technologies(technologies) => project.technologies = technologies,
                    ProjectField::Url(url) => project.url = url.into(),
                }
            }
            ProjectMsg::RemoveProject(index) => {
                self.projects.remove(index as usize);
            }
        }
        ctx.props().callback.emit(self.projects.clone());
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let add_project = ctx.link().callback(|_| ProjectMsg::AddProject);
        let inputs = self
            .projects
            .iter()
            .enumerate()
            .map(|(idx, project)| {
                let remove_project = ctx.link().callback(move |_| ProjectMsg::RemoveProject(idx));

                let name_input = make_input(
                    ctx,
                    idx,
                    "Name".to_string(),
                    project.name.clone(),
                    ProjectField::Name,
                );

                let description_input = make_input(
                    ctx,
                    idx,
                    "Description".to_string(),
                    project.description.clone(),
                    ProjectField::Description,
                );

                let technologies_input = make_input(
                    ctx,
                    idx,
                    "Technologies".to_string(),
                    project.technologies.clone(),
                    ProjectField::Technologies,
                );

                let url_input = make_input(
                    ctx,
                    idx,
                    "Url".to_string(),
                    project.url.clone().unwrap_or("".to_string()),
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
}

fn make_input<F>(
    ctx: &Context<ProjectController>,
    idx: usize,
    name: String,
    value: String,
    cons: F,
) -> Html
where
    F: Fn(String) -> ProjectField + 'static,
{
    let callback = ctx.link().callback(move |e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        ProjectMsg::UpdateField(idx, cons(input.value()))
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
                html! { <a href={project.url.clone().unwrap()} class="text-blue-600"> {project.name.clone()} </a> }
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
