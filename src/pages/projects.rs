use web_sys::HtmlInputElement;
use yew::prelude::*;

use super::html_utils::with_side_tip;

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
                name: "Project Dowwin".to_string(),
                description:
                    "Genetic algorithm inspired robo-advisor training system and market data feeds."
                        .to_string(),
                technologies: "Python".to_string(),
                url: Some("https://github.com/YilunAllenChen/Dowwin_legacy/".to_string()),
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
                url: Some("https://github.com/YilunAllenChen/Dowwin_legacy/".to_string()),
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
                let remove_project = ctx.link().callback(move |_| {
                    ProjectMsg::RemoveProject(idx)
                });

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
                            class="w-full rounded-md mb-2 px-3.5 py-2.5 text-md shadow-sm bg-red-500 text-white"
                            onclick={remove_project}
                        >
                            {"Remove"}
                        </button>
                    </div>
                }
            }).collect::<Html>();
        html! {
            <>
                {inputs}
                <button
                    class="w-full rounded-md mt-4 py-2.5 text-md shadow-sm bg-green-500 text-white"
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
    <div class="m-2">
        <div class="relative">
            <input type="text"
                   id={name.clone()}
                   oninput={callback}
                   class="peer rounded-md px-2 pt-5 pb-1 block w-full border-0 border-b-2 border-gray-300 focus:ring-0 focus:border-black pt-2"
                   value={value}
            />
            <label for={name.clone()}
                   class="absolute -top-0 left-2 text-gray-500 text-sm transition-all peer-placeholder-shown:text-xl peer-placeholder-shown:text-gray-400 peer-placeholder-shown:top-0">
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
        <div class="m-4 flex flex-col space-y-[0.5]">
            {projects}
        </div>
    }
}
