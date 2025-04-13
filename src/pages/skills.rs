use web_sys::HtmlInputElement;
use yew::prelude::*;

use super::html_utils::{
    ADD_BUTTON_CLASS, INPUT_CLASS, LABEL_CLASS, REMOVE_BUTTON_CLASS, SECTION_HEADER_CLASS,
};

#[derive(Debug, PartialEq, Clone)]
pub struct SkillCategory {
    category: String,
    skills: String,
}

pub enum SkillField {
    Category(String),
    Skills(String),
}

#[derive(Properties, PartialEq)]
pub struct SkillControllerProps {
    pub callback: Callback<Vec<SkillCategory>>,
}

pub enum SkillMsg {
    AddSkillCategory,
    RemoveSkillCategory(usize),
    UpdateField(usize, SkillField),
}

pub struct SkillController {
    skills: Vec<SkillCategory>,
}

impl Component for SkillController {
    type Message = SkillMsg;
    type Properties = SkillControllerProps;

    fn create(ctx: &Context<Self>) -> Self {
        let init = vec![
            SkillCategory {
                category: "Programming".to_string(),
                skills: "(Proficient) Python, Rust (Good) Java, Gleam, C++, JavaScript, Lua, Go (Familiar) OCaml, Haskell"
                    .to_string(),
            },
            SkillCategory {
                category: "Technologies".to_string(),
                skills: "Git, Kafka, Delta Lake, Redis, gRPC/Protobuf, Yew, Tailwind, Arrow, SQL, Kubernetes, Docker, ArgoCD, Neovim"
                    .to_string(),
            },
            SkillCategory {
                category: "The Softs".to_string(),
                skills: "Leadership, Evidence-based Entrepreneurship, Project Management, Public Speaking"
                    .to_string(),
            }
        ];
        let slf = Self { skills: init };
        ctx.props().callback.emit(slf.skills.clone());
        slf
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SkillMsg::AddSkillCategory => {
                self.skills.push(SkillCategory {
                    category: "".to_string(),
                    skills: "".to_string(),
                });
            }
            SkillMsg::UpdateField(index, field) => {
                let skill = self.skills.get_mut(index as usize).unwrap();
                match field {
                    SkillField::Category(category) => skill.category = category,
                    SkillField::Skills(skills) => skill.skills = skills,
                }
            }
            SkillMsg::RemoveSkillCategory(index) => {
                self.skills.remove(index as usize);
            }
        }
        ctx.props().callback.emit(self.skills.clone());
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let add_skill = ctx.link().callback(|_| SkillMsg::AddSkillCategory);
        let inputs = self
            .skills
            .iter()
            .enumerate()
            .map(|(idx, skill)| {
                let remove_skill = ctx
                    .link()
                    .callback(move |_| SkillMsg::RemoveSkillCategory(idx));

                let category_input = make_input(
                    ctx,
                    idx,
                    "Category".to_string(),
                    skill.category.clone(),
                    SkillField::Category,
                );
                let skills_input = make_input(
                    ctx,
                    idx,
                    "Skills".to_string(),
                    skill.skills.clone(),
                    SkillField::Skills,
                );
                html! {
                    <div class="space-y-2 bg-slate-100 rounded-lg p-4 my-4">
                        {category_input}
                        {skills_input}
                        <button
                            class={REMOVE_BUTTON_CLASS}
                            onclick={remove_skill}
                        >
                            {"Remove Skill"}
                        </button>
                    </div>
                }
            })
            .collect::<Html>();
        html! {
            <>
                <h5 class="text-xl font-bold text-left self-center pl-4 mt-4 mb-1"> {"Skills"} </h5>
                <hr/>
                {inputs}
                <button
                    class={ADD_BUTTON_CLASS}
                    onclick={add_skill}
                >
                    {"Add Skill"}
                </button>
            </>
        }
    }
}

fn make_input<F>(
    ctx: &Context<SkillController>,
    idx: usize,
    name: String,
    value: String,
    cons: F,
) -> Html
where
    F: Fn(String) -> SkillField + 'static,
{
    let callback = ctx.link().callback(move |e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        SkillMsg::UpdateField(idx, cons(input.value()))
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
    pub skills: Vec<SkillCategory>,
}

#[function_component(SkillViewer)]
pub fn view_skill(props: &Props) -> Html {
    if props.skills.is_empty() {
        return html! {};
    }
    let skills = props
        .skills
        .iter()
        .map(|skillgroup| {
            html! {
                <div class="flex gap-x-1">
                    <span class="font-bold w-1/6"> {&skillgroup.category} </span>
                    <span class=""> {&skillgroup.skills} </span>
                </div>
            }
        })
        .collect::<Html>();
    html! {
        <>
            <h5 class={SECTION_HEADER_CLASS}> {"Skills"} </h5>
            <div class="mx-2 my-1 flex flex-col -space-y-1">
                {skills}
            </div>
        </>
    }
}
