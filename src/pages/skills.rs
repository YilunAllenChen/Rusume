use serde::{Deserialize, Serialize};
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;

use super::html_utils::{
    ADD_BUTTON_CLASS, INPUT_CLASS, LABEL_CLASS, REMOVE_BUTTON_CLASS, SECTION_HEADER_CLASS,
};

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct SkillCategory {
    pub category: String,
    pub skills: String,
}

pub enum SkillField {
    Category(String),
    Skills(String),
}

#[derive(Properties, PartialEq)]
pub struct SkillControllerProps {
    pub value: Vec<SkillCategory>,
    pub on_change: Callback<Vec<SkillCategory>>,
}

#[function_component(SkillController)]
pub fn skill_controller(props: &SkillControllerProps) -> Html {
    let add_skill = {
        let skills = props.value.clone();
        let on_change = props.on_change.clone();
        Callback::from(move |_| {
            let mut next = skills.clone();
            next.push(SkillCategory::default());
            on_change.emit(next);
        })
    };

    let inputs = props
        .value
        .iter()
        .enumerate()
        .map(|(idx, skill)| {
            let remove_skill = {
                let skills = props.value.clone();
                let on_change = props.on_change.clone();
                Callback::from(move |_| {
                    let mut next = skills.clone();
                    if idx < next.len() {
                        next.remove(idx);
                        on_change.emit(next);
                    }
                })
            };

            let update_category = {
                let skills = props.value.clone();
                let on_change = props.on_change.clone();
                Callback::from(move |e: InputEvent| {
                    let input: HtmlInputElement = e.target_unchecked_into();
                    let mut next = skills.clone();
                    if let Some(skill_category) = next.get_mut(idx) {
                        skill_category.category = input.value();
                        on_change.emit(next);
                    }
                })
            };
            let category_input = html! {
                <div class="">
                    <div class="relative flex-1">
                        <input type="text"
                               id={"Category".to_string()}
                               oninput={update_category}
                               class={INPUT_CLASS}
                               value={skill.category.clone()}
                        />
                        <label for={"Category".to_string()}
                               class={LABEL_CLASS}>
                            {"Category"}
                        </label>
                    </div>
                </div>
            };

            let update_skills = {
                let skills = props.value.clone();
                let on_change = props.on_change.clone();
                Callback::from(move |e: InputEvent| {
                    let input: HtmlTextAreaElement = e.target_unchecked_into();
                    let mut next = skills.clone();
                    if let Some(skill_category) = next.get_mut(idx) {
                        skill_category.skills = input.value();
                        on_change.emit(next);
                    }
                })
            };
            let skills_input = html! {
                <div class="relative">
                <textarea
                    name="description"
                    class="w-full h-30 rounded-md px-2 pt-5 text-md shadow-sm"
                    oninput={update_skills}
                    value={skill.skills.clone()}
                />
                <label for={"description".to_string()}
                       class={LABEL_CLASS} >
                    {"Description"}
                </label>
                </div>
            };
            html! {
                <div class="space-y-2 bg-slate-100 rounded-lg p-1 m-1 ">
                    <div class="flex">
                        <div class="flex-1">
                        {category_input}
                        </div>

                        <div class="w-8 mx-4 mt-2">
                            <button
                                class={REMOVE_BUTTON_CLASS}
                                onclick={remove_skill}
                            >
                                {"❌"}
                            </button>
                        </div>
                    </div>
                    {skills_input}
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
