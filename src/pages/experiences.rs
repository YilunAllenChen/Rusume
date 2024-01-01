use web_sys::HtmlInputElement;
use yew::{callback, prelude::*};

use super::html_utils::with_side_tip;

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
        let init = vec![Experience {
            employer: "DRW".to_string(),
            team: "FICC Options".to_string().into(),
            title: "Software Developer".to_string(),
            dates: "2022/07 - Present".to_string(),
            location: "Chicago, IL".to_string(),
            description: vec![
                "• Own, develop and manage the core data streaming library enjoyed by 20 systems and 13 trading desks.",
                "• Spearhead the design and development of option pricing dataset processes with estimated impact of $2M/yr",
                "• Revamped and modernized group’s core risk product, boosting speed by 5x and reducing error rate by 85%",
                "• Automated, documented and standardized team operational procedures, saving 20 engr hrs / week.",
                "• Orchestrated collaboration across 3 teams to integrate exotic options pricing into existing trading systems.",
                "• Utilized: Python, Rust, Java, Kafka, Delta Lake, gRPC/Protobuf, S3, OracleDB, Arrow, Flight, Redis",
            ].join("\n").to_string(),
        }, Experience {
            employer: "DRW".to_string(),
            team: "FICC Options".to_string().into(),
            title: "Software Developer Intern".to_string(),
            dates: "2022/06 - 2022/08".to_string(),
            location: "Chicago, IL".to_string(),
            description: vec![
                "• Pioneered the design of a performant, language-agnostic data ingestion engine, handling >300k messages/sec.",
                "• Shipped unified data APIs to serve both historical and live data, widely used in critical trading infrastructure.",
                "• Devised compression and normalization algorithms for high-dimensional data, reducing sizes by 80%.",
                "• Utilized: Python, Java, Kafka, S3, Parquet, gRPC/Protobuf, Trino/Presto.",
            ].join("\n").to_string(),
        }, Experience {
            employer: "Georgia Tech".to_string(),
            team: "GRITS Lab".to_string().into(),
            title: "Robotics Researcher".to_string(),
            dates: "2019/05 - 2021/6".to_string(),
            location: "Atlanta, GA".to_string(),
            description: vec![
                "• Led a team of 4 PhD/MS students to build hardware, firmware and software for robots (The SlothBot).",
                "• Architected asynchronous over-the-air software update infrastructure for field-deployed robots swarms.",
                "• Utilized: C/C++, Python, React+Redux, Google Firebase, DigitalOcean, Kubernetes, Docker, ROS.",
            ].join("\n").to_string(),
        }, Experience {
            employer: "Uber ATG".to_string(),
            team: "Platform Validation".to_string().into(),
            title: "Hardawre Engineering Intern".to_string(),
            dates: "2020/05 - 2020/07".to_string(),
            location: "Atlanta, GA".to_string(),
            description: vec![
                "• Modularized and optimized the architecture of a legacy autonomous vehicle fleet orchestration system.",
                "• Designed task scheduling algorithms, improving system capacity by 700% and reducing memory usage by 80%.",
                "• Utilized: Python (asyncio), Bash, AWS (EC2), PostgreSQL.",
            ].join("\n").to_string(),
       }
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
                let remove_experience = ctx.link().callback(move |_| {
                    ExperienceMsg::RemoveExperience(idx)
                });

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
                let description_input = with_side_tip(
                    html! {
                        <textarea
                            class="w-full h-32 rounded-md mb-2 px-3.5 py-2.5 text-md shadow-sm"
                            oninput={callback}
                            value={experience.description.clone()}
                        />
                    },
                    "Description".to_string(),
                    )
                    ;
                    html! {
                    <div class="space-y-2 rounded-lg bg-slate-100 p-4 my-4">
                        {employer_input}
                        {team_input}
                        {title_input}
                        {dates_input}
                        {location_input}
                        {description_input}
                        <button
                            class="w-full rounded-md mb-2 px-3.5 py-2.5 text-md shadow-sm bg-red-500 text-white"
                            onclick={remove_experience}
                        >
                            {"Remove Experience"}
                        </button>
                    </div>
                }
            }).collect::<Html>();
        html! {
            <>
                {inputs}
                <button
                    class="w-full rounded-md mt-4 py-2.5 text-md shadow-sm bg-green-500 text-white"
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
    let input = html! {
        <input
            type="text"
            oninput={callback}
            class="w-full rounded-md mb-2 px-3.5 py-2.5 text-md shadow-sm"
            value={value}
        />
    };
    with_side_tip(input, name)
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub experiences: Vec<Experience>,
}

#[function_component(ExperienceViewer)]
pub fn view_experience(props: &Props) -> Html {
    let experiences = props
        .experiences
        .iter()
        .map(|experience| {
            html! {
                <div class="">
                <div class="flex justify-between mt-2 mb-1">
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
                <div class="flex gap-x-4">
                    <pre class="font-['Times']"> {&experience.description} </pre>
                </div>
                </div>
            }
        })
        .collect::<Html>();
    html! {
        <div class="m-4 flex flex-col space-y-1">
            {experiences}
        </div>
    }
}
