use web_sys::HtmlInputElement;
use yew::prelude::*;

use super::html_utils::{
    ADD_BUTTON_CLASS, INPUT_CLASS, LABEL_CLASS, REMOVE_BUTTON_CLASS, SECTION_HEADER_CLASS,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Education {
    school: String,
    degree: String,
    major: String,
    start_date: String,
    end_date: String,
    gpa: String,
}

pub enum EducationField {
    School(String),
    Degree(String),
    Major(String),
    StartDate(String),
    EndDate(String),
    GPA(String),
}

#[derive(Properties, PartialEq)]
pub struct EducationControllerProps {
    pub callback: Callback<Vec<Education>>,
}

pub enum EducationMsg {
    AddEducation,
    RemoveEducation(usize),
    UpdateField(usize, EducationField),
}

pub struct EducationController {
    educations: Vec<Education>,
}

impl Component for EducationController {
    type Message = EducationMsg;
    type Properties = EducationControllerProps;

    fn create(ctx: &Context<Self>) -> Self {
        let init = vec![
            Education {
                school: "Georgia Tech".to_string(),
                degree: "M.S.".to_string(),
                major: "Computer Engineering".to_string(),
                start_date: "August 2021".to_string(),
                end_date: "May 2022".to_string(),
                gpa: "3.87/4.0".to_string(),
            },
            Education {
                school: "Georgia Tech".to_string(),
                degree: "B.S.".to_string(),
                major: "Computer Engineering".to_string(),
                start_date: "August 2017".to_string(),
                end_date: "May 2021".to_string(),
                gpa: "3.86/4.0".to_string(),
            },
        ];
        let slf = Self { educations: init };
        ctx.props().callback.emit(slf.educations.clone());
        slf
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            EducationMsg::AddEducation => {
                self.educations.push(Education {
                    school: "".to_string(),
                    degree: "".to_string(),
                    major: "".to_string(),
                    start_date: "".to_string(),
                    end_date: "".to_string(),
                    gpa: "".to_string(),
                });
            }
            EducationMsg::UpdateField(index, field) => {
                let education = self.educations.get_mut(index as usize).unwrap();
                match field {
                    EducationField::School(school) => {
                        education.school = school;
                    }
                    EducationField::Degree(degree) => {
                        education.degree = degree;
                    }
                    EducationField::Major(major) => {
                        education.major = major;
                    }
                    EducationField::StartDate(start_date) => {
                        education.start_date = start_date;
                    }
                    EducationField::EndDate(end_date) => {
                        education.end_date = end_date;
                    }
                    EducationField::GPA(gpa) => {
                        education.gpa = gpa;
                    }
                }
            }
            EducationMsg::RemoveEducation(index) => {
                self.educations.remove(index as usize);
            }
        }
        ctx.props().callback.emit(self.educations.clone());
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let add_education = ctx.link().callback(|_| EducationMsg::AddEducation);
        let inputs = self
            .educations
            .iter()
            .enumerate()
            .map(|(idx, education)| {
                let remove_education = ctx
                    .link()
                    .callback(move |_| EducationMsg::RemoveEducation(idx));
                let school_input = make_input(
                    ctx,
                    idx,
                    "School".to_string(),
                    education.school.clone(),
                    EducationField::School,
                );
                let degree_input = make_input(
                    ctx,
                    idx,
                    "Degree".to_string(),
                    education.degree.clone(),
                    EducationField::Degree,
                );
                let major_input = make_input(
                    ctx,
                    idx,
                    "Major".to_string(),
                    education.major.clone(),
                    EducationField::Major,
                );
                let start_date_input = make_input(
                    ctx,
                    idx,
                    "Start".to_string(),
                    education.start_date.clone(),
                    EducationField::StartDate,
                );
                let end_date_input = make_input(
                    ctx,
                    idx,
                    "End".to_string(),
                    education.end_date.clone(),
                    EducationField::EndDate,
                );
                let gpa_input = make_input(
                    ctx,
                    idx,
                    "GPA".to_string(),
                    education.gpa.clone(),
                    EducationField::GPA,
                );
                html! {
                    <div class="space-y-2 bg-slate-100 rounded-lg p-4 my-4">
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
                            {"Remove Education"}
                        </button>
                    </div>
                }
            })
            .collect::<Html>();
        html! {
            <>
                <h5 class="text-xl font-bold text-left self-center pl-4 mt-4 mb-1"> {"Education"} </h5>
                <hr/>
                {inputs}
                <button
                    class={ADD_BUTTON_CLASS}
                    onclick={add_education}
                >
                    {"Add Education"}
                </button>
            </>
        }
    }
}

fn make_input<F>(
    ctx: &Context<EducationController>,
    idx: usize,
    name: String,
    value: String,
    cons: F,
) -> Html
where
    F: Fn(String) -> EducationField + 'static,
{
    let callback = ctx.link().callback(move |e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        EducationMsg::UpdateField(idx, cons(input.value()))
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
            <div class="mx-2 my-1 flex flex-col space-y-0">
                {educations}
            </div>
        </>
    }
}
