use serde::{Deserialize, Serialize};
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;

use super::html_utils::{
    ADD_BUTTON_CLASS, INPUT_CLASS, INPUT_DIVIDER_CLASS, INPUT_FIELD_WRAPPER_CLASS,
    INPUT_SECTION_CLASS, LABEL_CLASS, REMOVE_BUTTON_CLASS, SECTION_HEADER_CLASS, TEXTAREA_CLASS,
};

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Experience {
    pub employer: String,
    pub team: Option<String>,
    pub title: String,
    pub dates: String,
    pub location: String,
    pub description: String,
}

pub enum ExperienceField {
    Employer(String),
    Team(String),
    Title(String),
    Dates(String),
    Location(String),
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
            next.push(Experience::default());
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
            let title_input = make_input(
                props,
                idx,
                "Title".to_string(),
                experience.title.clone(),
                ExperienceField::Title,
            );
            let dates_input = make_input(
                props,
                idx,
                "Dates".to_string(),
                experience.dates.clone(),
                ExperienceField::Dates,
            );
            let location_input = make_input(
                props,
                idx,
                "Location".to_string(),
                experience.location.clone(),
                ExperienceField::Location,
            );

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
                    {title_input}
                    {dates_input}
                    {location_input}
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
            <h5 class={INPUT_SECTION_CLASS}> {"Experiences"} </h5>
            <div class={INPUT_DIVIDER_CLASS}></div>
            {inputs}
            <button
                class={ADD_BUTTON_CLASS}
                onclick={add_experience}
            >
                {"Add Experience"}
            </button>
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
                ExperienceField::Title(title) => {
                    experience.title = title;
                }
                ExperienceField::Dates(dates) => {
                    experience.dates = dates;
                }
                ExperienceField::Location(location) => {
                    experience.location = location;
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
            html! {
                <div class="">
                <div class="flex justify-between mt-1.5">
                    <div class="flex gap-x-3">
                        <span class="font-bold"> {&experience.employer.clone()} </span>
                        {" | "}
                        <span class=""> {&experience.team} </span>
                        {" | "}
                        <span class=""> {&experience.title} </span>
                    </div>
                    <div class="flex gap-x-4">
                        <span class=""> {&experience.dates} </span>
                        {" | "}
                        <span class=""> {&experience.location} </span>
                    </div>
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
