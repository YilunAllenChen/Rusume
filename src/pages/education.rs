use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;

use super::html_utils::{
    ADD_BUTTON_CLASS, INPUT_CLASS, INPUT_DIVIDER_CLASS, INPUT_FIELD_WRAPPER_CLASS,
    INPUT_SECTION_CLASS, INPUT_SECTION_ROW_CLASS, LABEL_CLASS, REMOVE_BUTTON_CLASS,
    SECTION_HEADER_CLASS,
};

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Education {
    pub school: String,
    pub degree: String,
    pub major: String,
    pub start_date: String,
    pub end_date: String,
    pub gpa: String,
}

pub enum EducationField {
    School(String),
    Degree(String),
    Major(String),
    StartDate(String),
    EndDate(String),
    Gpa(String),
}

#[derive(Properties, PartialEq)]
pub struct EducationControllerProps {
    pub value: Vec<Education>,
    pub on_change: Callback<Vec<Education>>,
}

#[function_component(EducationController)]
pub fn education_controller(props: &EducationControllerProps) -> Html {
    let add_education = {
        let educations = props.value.clone();
        let on_change = props.on_change.clone();
        Callback::from(move |_| {
            let mut next = educations.clone();
            next.push(Education::default());
            on_change.emit(next);
        })
    };

    let inputs = props
        .value
        .iter()
        .enumerate()
        .map(|(idx, education)| {
            let remove_education = {
                let educations = props.value.clone();
                let on_change = props.on_change.clone();
                Callback::from(move |_| {
                    let mut next = educations.clone();
                    if idx < next.len() {
                        next.remove(idx);
                        on_change.emit(next);
                    }
                })
            };

            let school_input = make_input(
                props,
                idx,
                "School".to_string(),
                education.school.clone(),
                EducationField::School,
            );
            let degree_input = make_input(
                props,
                idx,
                "Degree".to_string(),
                education.degree.clone(),
                EducationField::Degree,
            );
            let major_input = make_input(
                props,
                idx,
                "Major".to_string(),
                education.major.clone(),
                EducationField::Major,
            );
            let start_date_input = make_input(
                props,
                idx,
                "Start".to_string(),
                education.start_date.clone(),
                EducationField::StartDate,
            );
            let end_date_input = make_input(
                props,
                idx,
                "End".to_string(),
                education.end_date.clone(),
                EducationField::EndDate,
            );
            let gpa_input = make_input(
                props,
                idx,
                "GPA".to_string(),
                education.gpa.clone(),
                EducationField::Gpa,
            );
            html! {
                <>
                    {school_input}
                    {major_input}
                    <div class="flex">
                        <div class="w-1/2">
                            {degree_input}
                            {gpa_input}
                        </div>
                        <div class="w-1/2">
                            {start_date_input}
                            {end_date_input}
                        </div>
                    </div>
                    <button
                        class={REMOVE_BUTTON_CLASS}
                        onclick={remove_education}
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
                <h5 class={INPUT_SECTION_CLASS}> {"Education"} </h5>
                <button
                    class={ADD_BUTTON_CLASS}
                    onclick={add_education}
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
    props: &EducationControllerProps,
    idx: usize,
    name: String,
    value: String,
    cons: F,
) -> Html
where
    F: Fn(String) -> EducationField + 'static,
{
    let educations = props.value.clone();
    let on_change = props.on_change.clone();
    let callback = Callback::from(move |e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        let mut next = educations.clone();
        if let Some(education) = next.get_mut(idx) {
            match cons(input.value()) {
                EducationField::School(school) => education.school = school,
                EducationField::Degree(degree) => education.degree = degree,
                EducationField::Major(major) => education.major = major,
                EducationField::StartDate(start_date) => education.start_date = start_date,
                EducationField::EndDate(end_date) => education.end_date = end_date,
                EducationField::Gpa(gpa) => education.gpa = gpa,
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
    pub educations: Vec<Education>,
}

#[function_component(EducationViewer)]
pub fn view_education(props: &Props) -> Html {
    if props.educations.is_empty() {
        return html! {};
    }
    let educations = props
        .educations
        .iter()
        .map(|education| {
            html! {
                <div class="flex justify-between">
                    <div class="flex gap-x-1">
                        <span class="font-semibold"> {&education.school.clone()} {", "} </span>
                        <span class=""> {&education.degree} </span>
                        <span class=""> {&education.major} {", "} </span>
                        <span class=""> {"GPA: "}{&education.gpa} </span>
                    </div>
                    <div class="flex gap-x-4">
                        <span class="">
                            {education.start_date.clone() + " - " + &education.end_date.clone()}
                        </span>
                    </div>
                </div>
            }
        })
        .collect::<Html>();
    html! {
        <>
            <h5 class={SECTION_HEADER_CLASS}> {"Education"} </h5>
            <div class="mx-2 my-1 flex flex-col -space-y-1">
                {educations}
            </div>
        </>
    }
}
