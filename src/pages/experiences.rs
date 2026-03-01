use serde::{Deserialize, Serialize};
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;

use super::html_utils::{
    ADD_BUTTON_CLASS, INPUT_CLASS, INPUT_DIVIDER_CLASS, INPUT_FIELD_WRAPPER_CLASS,
    INPUT_SECTION_CLASS, INPUT_SECTION_ROW_CLASS, LABEL_CLASS, REMOVE_BUTTON_CLASS,
    SECTION_HEADER_CLASS, TEXTAREA_CLASS,
};

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct RolePeriod {
    pub title: String,
    pub dates: String,
    #[serde(default)]
    pub location: String,
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Experience {
    pub employer: String,
    pub team: Option<String>,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub dates: String,
    pub location: String,
    pub description: String,
    #[serde(default)]
    pub roles: Vec<RolePeriod>,
}

pub enum ExperienceField {
    Employer(String),
    Team(String),
}

#[derive(Properties, PartialEq)]
pub struct ExperienceControllerProps {
    pub value: Vec<Experience>,
    pub on_change: Callback<Vec<Experience>>,
}

#[function_component(ExperienceController)]
pub fn experience_controller(props: &ExperienceControllerProps) -> Html {
    let add_experience = {
        let experiences = props.value.clone();
        let on_change = props.on_change.clone();
        Callback::from(move |_| {
            let mut next = experiences.clone();
            next.push(Experience {
                roles: vec![RolePeriod::default()],
                ..Experience::default()
            });
            on_change.emit(next);
        })
    };

    let inputs = props
        .value
        .iter()
        .enumerate()
        .map(|(idx, experience)| {
            let remove_experience = {
                let experiences = props.value.clone();
                let on_change = props.on_change.clone();
                Callback::from(move |_| {
                    let mut next = experiences.clone();
                    if idx < next.len() {
                        next.remove(idx);
                        on_change.emit(next);
                    }
                })
            };

            let employer_input = make_input(
                props,
                idx,
                "Employer".to_string(),
                experience.employer.clone(),
                ExperienceField::Employer,
            );
            let team_input = make_input(
                props,
                idx,
                "Team".to_string(),
                experience.team.clone().unwrap_or_default(),
                ExperienceField::Team,
            );
            let roles = roles_for_display(experience);

            let role_rows = roles
                .iter()
                .enumerate()
                .map(|(role_idx, role)| {
                    let update_title = {
                        let experiences = props.value.clone();
                        let on_change = props.on_change.clone();
                        Callback::from(move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            let mut next = experiences.clone();
                            if let Some(experience) = next.get_mut(idx) {
                                let mut roles = roles_for_display(experience);
                                if let Some(role) = roles.get_mut(role_idx) {
                                    role.title = input.value();
                                    apply_roles_to_experience(experience, roles);
                                    on_change.emit(next);
                                }
                            }
                        })
                    };

                    let update_dates = {
                        let experiences = props.value.clone();
                        let on_change = props.on_change.clone();
                        Callback::from(move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            let mut next = experiences.clone();
                            if let Some(experience) = next.get_mut(idx) {
                                let mut roles = roles_for_display(experience);
                                if let Some(role) = roles.get_mut(role_idx) {
                                    role.dates = input.value();
                                    apply_roles_to_experience(experience, roles);
                                    on_change.emit(next);
                                }
                            }
                        })
                    };

                    let update_location = {
                        let experiences = props.value.clone();
                        let on_change = props.on_change.clone();
                        Callback::from(move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            let mut next = experiences.clone();
                            if let Some(experience) = next.get_mut(idx) {
                                let mut roles = roles_for_display(experience);
                                if let Some(role) = roles.get_mut(role_idx) {
                                    role.location = input.value();
                                    apply_roles_to_experience(experience, roles);
                                    on_change.emit(next);
                                }
                            }
                        })
                    };

                    let remove_role = {
                        let experiences = props.value.clone();
                        let on_change = props.on_change.clone();
                        Callback::from(move |_| {
                            let mut next = experiences.clone();
                            if let Some(experience) = next.get_mut(idx) {
                                let mut roles = roles_for_display(experience);
                                if roles.len() > 1 && role_idx < roles.len() {
                                    roles.remove(role_idx);
                                    apply_roles_to_experience(experience, roles);
                                    on_change.emit(next);
                                }
                            }
                        })
                    };

                    let remove_role_button = if roles.len() > 1 {
                        html! {
                            <button
                                class="rounded-md px-3 py-2 text-xs font-semibold text-white bg-slate-700 hover:bg-slate-600"
                                onclick={remove_role}
                            >
                                {"Remove Role"}
                            </button>
                        }
                    } else {
                        html! {}
                    };

                    html! {
                        <div class="border border-slate-800/80 rounded-lg p-0.5 my-0.5">
                            <div class="grid grid-cols-1 md:grid-cols-3 gap-1">
                                <div class="relative flex-1">
                                    <input
                                        type="text"
                                        class={INPUT_CLASS}
                                        value={role.title.clone()}
                                        oninput={update_title}
                                    />
                                    <label class={LABEL_CLASS}> {"Role"} </label>
                                </div>
                                <div class="relative flex-1">
                                    <input
                                        type="text"
                                        class={INPUT_CLASS}
                                        value={role.dates.clone()}
                                        oninput={update_dates}
                                    />
                                    <label class={LABEL_CLASS}> {"Date Range"} </label>
                                </div>
                                <div class="relative flex-1">
                                    <input
                                        type="text"
                                        class={INPUT_CLASS}
                                        value={role.location.clone()}
                                        oninput={update_location}
                                    />
                                    <label class={LABEL_CLASS}> {"Location"} </label>
                                </div>
                            </div>
                            <div class="mt-0 flex justify-end">
                                {remove_role_button}
                            </div>
                        </div>
                    }
                })
                .collect::<Html>();

            let add_role = {
                let experiences = props.value.clone();
                let on_change = props.on_change.clone();
                Callback::from(move |_| {
                    let mut next = experiences.clone();
                    if let Some(experience) = next.get_mut(idx) {
                        let mut roles = roles_for_display(experience);
                        roles.push(RolePeriod::default());
                        apply_roles_to_experience(experience, roles);
                        on_change.emit(next);
                    }
                })
            };

            let update_description = {
                let experiences = props.value.clone();
                let on_change = props.on_change.clone();
                Callback::from(move |e: InputEvent| {
                    let input: HtmlTextAreaElement = e.target_unchecked_into();
                    let mut next = experiences.clone();
                    if let Some(experience) = next.get_mut(idx) {
                        experience.description = input.value();
                        on_change.emit(next);
                    }
                })
            };
            let description_input = html! {
                <div class={INPUT_FIELD_WRAPPER_CLASS}>
                <div class="relative">
                <textarea
                    name="description"
                    class={TEXTAREA_CLASS}
                    oninput={update_description}
                    value={experience.description.clone()}
                />
                <label for={"description".to_string()}
                       class={LABEL_CLASS} >
                    {"Description"}
                </label>
                </div>
                </div>
            };
            html! {
                <>
                    {employer_input}
                    {team_input}
                    <div class={INPUT_FIELD_WRAPPER_CLASS}>
                        <div class="flex items-center justify-between px-1 py-1">
                            <div class="text-xs tracking-wide uppercase text-slate-400 font-semibold">{"Roles"}</div>
                            <button class="rounded-md px-3 py-2 text-xs font-semibold text-slate-950 bg-emerald-400 hover:bg-emerald-300" onclick={add_role}>
                                {"Add Role + Dates"}
                            </button>
                        </div>
                        {role_rows}
                    </div>
                    {description_input}
                    <button
                        class={REMOVE_BUTTON_CLASS}
                        onclick={remove_experience}>
                        {"Remove"}
                    </button>
                </>
            }
        })
        .collect::<Html>();
    html! {
        <>
            <div class={INPUT_SECTION_ROW_CLASS}>
                <h5 class={INPUT_SECTION_CLASS}> {"Experiences"} </h5>
                <button
                    class={ADD_BUTTON_CLASS}
                    onclick={add_experience}
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
    props: &ExperienceControllerProps,
    idx: usize,
    name: String,
    value: String,
    cons: F,
) -> Html
where
    F: Fn(String) -> ExperienceField + 'static,
{
    let experiences = props.value.clone();
    let on_change = props.on_change.clone();
    let callback = Callback::from(move |e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        let mut next = experiences.clone();
        if let Some(experience) = next.get_mut(idx) {
            match cons(input.value()) {
                ExperienceField::Employer(employer) => {
                    experience.employer = employer;
                }
                ExperienceField::Team(team) => {
                    experience.team = Some(team);
                }
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

fn roles_for_display(experience: &Experience) -> Vec<RolePeriod> {
    if !experience.roles.is_empty() {
        return experience
            .roles
            .iter()
            .map(|role| RolePeriod {
                title: role.title.clone(),
                dates: role.dates.clone(),
                location: if role.location.is_empty() {
                    experience.location.clone()
                } else {
                    role.location.clone()
                },
            })
            .collect();
    }

    if experience.title.is_empty() && experience.dates.is_empty() && experience.location.is_empty()
    {
        vec![RolePeriod::default()]
    } else {
        vec![RolePeriod {
            title: experience.title.clone(),
            dates: experience.dates.clone(),
            location: experience.location.clone(),
        }]
    }
}

fn apply_roles_to_experience(experience: &mut Experience, roles: Vec<RolePeriod>) {
    let normalized = if roles.is_empty() {
        vec![RolePeriod::default()]
    } else {
        roles
    };

    experience.roles = normalized.clone();
    if let Some(first) = normalized.first() {
        experience.title = first.title.clone();
        experience.dates = first.dates.clone();
        experience.location = first.location.clone();
    } else {
        experience.title.clear();
        experience.dates.clear();
        experience.location.clear();
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub experiences: Vec<Experience>,
}

#[function_component(ExperienceViewer)]
pub fn view_experience(props: &Props) -> Html {
    if props.experiences.is_empty() {
        return html! {};
    }
    let experiences = props
        .experiences
        .iter()
        .map(|experience| {
            let parsed = experience
                .description
                .split('\n')
                .collect::<Vec<&str>>()
                .iter()
                .map(|s| " - • ".to_string() + s)
                .map(|s| markdown::to_html(&s))
                .map(|html_str| Html::from_html_unchecked(html_str.into()))
                .collect::<Html>();
            let roles = roles_for_display(experience)
                .into_iter()
                .enumerate()
                .map(|(idx, role)| {
                    let employer = if idx == 0 {
                        html! { <span class="font-bold"> {experience.employer.clone()} </span> }
                    } else {
                        html! { <span class="invisible font-bold"> {experience.employer.clone()} </span> }
                    };
                    let employer_team_sep = if idx == 0 {
                        html! { <span> {" | "} </span> }
                    } else {
                        html! { <span class="invisible"> {" | "} </span> }
                    };
                    let team = if idx == 0 {
                        html! { <span> {experience.team.clone()} </span> }
                    } else {
                        html! { <span class="invisible"> {experience.team.clone()} </span> }
                    };
                    html! {
                        <div class="grid grid-cols-[minmax(0,1fr)_auto] items-start gap-x-1 mt-0">
                            <div class="flex gap-x-1 min-w-0">
                                {employer}
                                {employer_team_sep}
                                {team}
                                <span> {" | "} </span>
                                <span> {role.title} </span>
                            </div>
                            <div class="flex gap-x-1 whitespace-nowrap">
                                <span> {role.dates} </span>
                                <span> {" | "} </span>
                                <span> {role.location} </span>
                            </div>
                        </div>
                    }
                })
                .collect::<Html>();
            html! {
                <div class="mb-3">
                <div>
                    {roles}
                </div>
                <div>
                    {parsed}
                </div>
                </div>
            }
        })
        .collect::<Html>();
    html! {

        <>
        <h5 class={SECTION_HEADER_CLASS}> {"Experiences"} </h5>
        <div class="mx-2 my-2">
            {experiences}
        </div>
        </>
    }
}
