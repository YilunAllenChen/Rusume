use web_sys::HtmlInputElement;
use yew::prelude::*;

use super::html_utils::{
    ADD_BUTTON_CLASS, INPUT_CLASS, LABEL_CLASS, REMOVE_BUTTON_CLASS, SECTION_HEADER_CLASS,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Experience {
    employer: String,
    team: Option<String>,
    title: String,
    dates: String,
    location: String,
    description: String,
}

pub enum ExperienceField {
    Employer(String),
    Team(String),
    Title(String),
    Dates(String),
    Location(String),
    Description(String),
}

#[derive(Properties, PartialEq)]
pub struct ExperienceControllerProps {
    pub callback: Callback<Vec<Experience>>,
}

pub enum ExperienceMsg {
    AddExperience,
    RemoveExperience(usize),
    UpdateField(usize, ExperienceField),
}

pub struct ExperienceController {
    experiences: Vec<Experience>,
}

impl Component for ExperienceController {
    type Message = ExperienceMsg;
    type Properties = ExperienceControllerProps;

    fn create(ctx: &Context<Self>) -> Self {
        let init = vec![
            Experience {
            employer: "DRW".to_string(),
            team: "Cumberland / FICC Options".to_string().into(),
            title: "Tech Lead".to_string(),
            dates: "2025/01 - Present".to_string(),
            location: "Chicago, IL".to_string(),
            description: vec![
                "Lead, manage and mentor a **global front office team of 7 exceptional engineers**, providing both direct desk service and central tooling.",
                "Standardize, optimize, document and automate **operation procedures**, boosting engineer productivy by lowering team support burden by **80%**.",
                "Prioritize and **allocate team resources** towards strategic initiatives, while tactically **managing technical debt**, eliminating 50+ legacy systems.",
                "Design, implement and own the core **FICC & equity options portfolio risk pipeline** for all (10+) trading desks within the group.",
                "Own a versatile, high performance options analytics & trading tools library, used by 60+ traders and researchers globally.", 
                "**Utilized**: Python, Java, C++, Kafka, Delta Lake, FastAPI, Plotly Dash, Kubernetes",
            ].join("\n").to_string(),
        },
            Experience {
            employer: "DRW".to_string(),
            team: "FICC Options".to_string().into(),
            title: "Senior Software Engineer".to_string(),
            dates: "2024/07 - 2025/01".to_string(),
            location: "Chicago, IL".to_string(),
            description: vec![
                "Own, develop and manage the **streaming data platform** for both historical and live use cases, used by >200 systems worldwide.",
                "Shipped an options analytical arsenal leveraging **functional domain modelling**, reduced turnaround time for new tools by **10x.**",
                "Maintain a generic **stream-processing** system to ingest, clean, transform and aggregate data following the Medallion architecture.",
                "Pioneered the implementation of a **bitemporal** binary wire transfer protocol that focuses on efficiency and the ability to time travel.",
                "**Utilized**: Python, Rust, Kafka, Delta Lake, Protobuf, Arrow, DuckDB, PostgreSQL",
            ].join("\n").to_string(),
        },
        Experience {
            employer: "DRW".to_string(),
            team: "FICC Options".to_string().into(),
            title: "Software Developer".to_string(),
            dates: "2022/07 - 2024/07".to_string(),
            location: "Chicago, IL".to_string(),
            description: vec![
                "Spearhead the design and development of core option pricing datasets & processes with **>$10M/yr** estimated materiality.",
                "Rearchitected of a volatility path dynamics visualization dashboard, making it **8x** faster and **10x** more resource efficient.",
                "Coordinated collaboration across 3 teams and 10+ engineers to integrate exotic options pricing & risks into existing trading systems.",
                "Automated, standardized and documented team support & operational procedures, saving 20 engr hrs / week.",
                "**Utilized**: Python, Rust, Java, Kafka, Delta Lake, gRPC/Protobuf, Arrow, Flight, Redis",
            ].join("\n").to_string(),
        },
        Experience {
            employer: "DRW".to_string(),
            team: "FICC Options".to_string().into(),
            title: "Software Developer Intern".to_string(),
            dates: "2021/06 - 2021/08".to_string(),
            location: "Atlanta, GA".to_string(),
            description: vec![
                "Built a proof-of-concept, language-agnostic, high performance unified streaming data platform with logical data compression.",
                "Took the initiative to create peripheral automated deployment workflows & monitoring dashboard for the above data platform.",
                "**Utilized**: Python, Java, Kafka, Presto/Trino, CephFS."
            ].join("\n").to_string(),
        },
        Experience {
            employer: "Uber Advanced Technology Group".to_string(),
            team: "Platform Validation".to_string().into(),
            title: "Software Engineering Intern".to_string(),
            dates: "2020/05 - 2020/07".to_string(),
            location: "Atlanta, GA".to_string(),
            description: vec![
                "Rearchitected a fleet orchestration system that efficiently schedule the dispatchment of autonomous vehicles for field tests",
                "**Utilized**: Python (asyncio), PostgreSQL"
            ].join("\n").to_string(),
        },
        ];
        let slf = Self { experiences: init };
        ctx.props().callback.emit(slf.experiences.clone());
        slf
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ExperienceMsg::AddExperience => {
                self.experiences.push(Experience {
                    employer: "".to_string(),
                    team: "".to_string().into(),
                    title: "".to_string(),
                    dates: "".to_string(),
                    location: "".to_string(),
                    description: "".to_string(),
                });
            }
            ExperienceMsg::UpdateField(index, field) => {
                let experience = self.experiences.get_mut(index as usize).unwrap();
                match field {
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
                    ExperienceField::Description(bullets) => {
                        experience.description = bullets;
                    }
                }
            }
            ExperienceMsg::RemoveExperience(index) => {
                self.experiences.remove(index as usize);
            }
        }
        ctx.props().callback.emit(self.experiences.clone());
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let add_experience = ctx.link().callback(|_| ExperienceMsg::AddExperience);
        let inputs = self
            .experiences
            .iter()
            .enumerate()
            .map(|(idx, experience)| {
                let remove_experience = ctx
                    .link()
                    .callback(move |_| ExperienceMsg::RemoveExperience(idx));

                let employer_input = make_input(
                    ctx,
                    idx,
                    "Employer".to_string(),
                    experience.employer.clone(),
                    ExperienceField::Employer,
                );
                let team_input = make_input(
                    ctx,
                    idx,
                    "Team".to_string(),
                    experience.team.clone().unwrap_or_default(),
                    ExperienceField::Team,
                );

                let title_input = make_input(
                    ctx,
                    idx,
                    "Title".to_string(),
                    experience.title.clone(),
                    ExperienceField::Title,
                );
                let dates_input = make_input(
                    ctx,
                    idx,
                    "Dates".to_string(),
                    experience.dates.clone(),
                    ExperienceField::Dates,
                );

                let location_input = make_input(
                    ctx,
                    idx,
                    "Location".to_string(),
                    experience.location.clone(),
                    ExperienceField::Location,
                );

                let callback = ctx.link().callback(move |e: InputEvent| {
                    let input: HtmlInputElement = e.target_unchecked_into();
                    ExperienceMsg::UpdateField(idx, ExperienceField::Description(input.value()))
                });
                let description_input = html! {
                    <div class="relative m-2">
                    <textarea
                        name="description"
                        class="w-full h-60 rounded-md px-3.5 pt-5 text-md shadow-sm"
                        oninput={callback}
                        value={experience.description.clone()}
                    />
                    <label for={"description".to_string()}
                           class={LABEL_CLASS} >
                        {"Description"}
                    </label>
                    </div>
                };
                html! {
                <div class="space-y-2 rounded-lg bg-slate-100 p-4 my-4">
                    {employer_input}
                    {team_input}
                    {title_input}
                    {dates_input}
                    {location_input}
                    {description_input}
                    <button
                        class={REMOVE_BUTTON_CLASS}
                        onclick={remove_experience}>
                        {"Remove Experience"}
                    </button>
                </div>
                }
            })
            .collect::<Html>();
        html! {
            <>
                <h5 class="text-xl font-bold text-left self-center pl-4 mt-4 mb-1"> {"Experiences"} </h5>
                <hr/>
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
}

fn make_input<F>(
    ctx: &Context<ExperienceController>,
    idx: usize,
    name: String,
    value: String,
    cons: F,
) -> Html
where
    F: Fn(String) -> ExperienceField + 'static,
{
    let callback = ctx.link().callback(move |e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        ExperienceMsg::UpdateField(idx, cons(input.value()))
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
                .split("\n")
                .collect::<Vec<&str>>()
                .iter()
                .map(|s| " - • ".to_string() + &s)
                .map(|s| markdown::to_html(&s))
                .collect::<Vec<String>>()
                .into_iter()
                .map(|html_str| Html::from_html_unchecked(html_str.into()))
                .collect::<Html>();
            html! {
                <div class="">
                <div class="flex justify-between mt-1.5">
                    <div class="flex gap-x-3">
                        <span class="font-bold"> {&experience.employer.clone()} </span>
                        { " | " }
                        <span class=""> {&experience.team} </span>
                        { " | " }
                        <span class=""> {&experience.title} </span>
                    </div>
                    <div class="flex gap-x-4">
                        <span class=""> {&experience.dates} </span>
                        { " | " }
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
