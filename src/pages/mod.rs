use log::info;
use web_sys::{window, HtmlInputElement};
use yew::prelude::*;

use self::html_utils::with_side_tip;

mod html_utils;

mod education;
use education::Education;
use education::EducationController;
use education::EducationViewer;

mod experiences;
use experiences::ExperienceController;
use experiences::ExperienceViewer;
use experiences::Experience;

mod projects;
use projects::Project;
use projects::ProjectController;
use projects::ProjectViewer;

#[derive(Debug)]
pub enum Field {
    Name(String),
    Email(String),
    Phone(String),
    LinkedInUrl(String),
    GithubUrl(String),
}

pub struct Home {
    name: String,
    email: String,
    phone: String,
    linkedin_url: Option<String>,
    github_url: Option<String>,

    educations: Vec<Education>,
    experiences: Vec<Experience>,
    projects: Vec<Project>,
}

pub enum HomeMsg {
    UpdateField(Field),
    Print,
    UpdateEducationSection(Vec<education::Education>),
    UpdateExperienceSection(Vec<experiences::Experience>),
    UpdateProjectSection(Vec<projects::Project>),
}

#[derive(Properties, PartialEq)]
pub struct HomeProps {}

impl Component for Home {
    type Message = HomeMsg;
    type Properties = HomeProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            name: "Yilun \"Allen\" Chen".to_string(),
            email: "allenchenyilun1999@gmail.com".to_string(),
            phone: "404-409-9683".to_string(),
            linkedin_url: "https://www.linkedin.com/in/yilun-allen-chen-572b71141/"
                .to_string()
                .into(),
            github_url: "https://github.com/YilunAllenChen".to_string().into(),

            educations: [].to_vec(),
            experiences: [].to_vec(),
            projects: [].to_vec(),
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            HomeMsg::UpdateField(field) => {
                match field {
                    Field::Name(name) => {
                        self.name = name;
                    }
                    Field::Email(email) => {
                        self.email = email;
                    }
                    Field::Phone(phone) => {
                        self.phone = phone;
                    }
                    Field::LinkedInUrl(linkedin_url) => {
                        if linkedin_url.is_empty() {
                            self.linkedin_url = None;
                            return true;
                        }
                        self.linkedin_url = Some(linkedin_url);
                    }
                    Field::GithubUrl(github_url) => {
                        if github_url.is_empty() {
                            self.github_url = None;
                            return true;
                        }
                        self.github_url = Some(github_url);
                    }
                }
                true
            }
            HomeMsg::Print => {
                info!("print");
                let window = window().unwrap();
                let old_body = window.document().unwrap().body().unwrap();
                let print_content = window
                    .document()
                    .unwrap()
                    .get_element_by_id("rusume")
                    .unwrap()
                    .inner_html();
                window
                    .document()
                    .unwrap()
                    .body()
                    .unwrap()
                    .set_inner_html(print_content.as_str());
                window.print().unwrap();
                info!("print done");
                window.document().unwrap().set_body(Some(&old_body));
                info!("reset done");
                true
            }
            HomeMsg::UpdateEducationSection(educations) => {
                self.educations = educations;
                info!("update education section");
                true
            }
            HomeMsg::UpdateExperienceSection(experiences) => {
                self.experiences = experiences;
                info!("update experience section");
                true
            }
            HomeMsg::UpdateProjectSection(projects) => {
                self.projects = projects;
                info!("update project section");
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let linkedin = match &self.linkedin_url {
            Some(url) => {
                html! {
                    <a class="text-blue-700" target="_blank" href={url.clone()}>
                        <i class="devicon-linkedin-plain"/> {" LinkedIn"}
                    </a>
                }
            }
            None => html! {},
        };

        let github = match &self.github_url {
            Some(url) => {
                html! {
                    <a class="text-blue-700" target="_blank" href={url.clone()}>
                        <i class="devicon-github-original text-black"/> {" Github"}
                    </a>
                }
            }
            None => html! {},
        };

        let email = html! {
            <a class="text-blue-700" href={"mailto:".to_string() + &self.email.clone()}>
                {self.email.clone()}
            </a>
        };

        let education_cb = ctx
            .link()
            .callback(|educations| HomeMsg::UpdateEducationSection(educations));
        let experience_cb = ctx
            .link()
            .callback(|experiences| HomeMsg::UpdateExperienceSection(experiences));
        let project_cb = ctx
            .link()
            .callback(|projects| HomeMsg::UpdateProjectSection(projects));

        html! {
            <div class="w-screen h-screen flex">
                <h1 class="w-1/3 p-4 bg-slate-200 overflow-scroll">
                    <h5 class="text-xl font-bold text-left self-center pl-4 mt-4 mb-1"> {"Basic Information"} </h5>
                    <hr/>
                    {make_input(ctx, "Name".to_string(), self.name.clone(), Field::Name)}
                    {make_input(ctx, "Email".to_string(), self.email.clone(), Field::Email)}
                    {make_input(ctx, "Phone".to_string(), self.phone.clone(), Field::Phone)}
                    <h5 class="text-xl font-bold text-left self-center pl-4 mt-4 mb-1"> {"Social Media"} </h5>
                    <hr/>
                    {make_input(ctx, "LinkedIn".to_string(), self.linkedin_url.clone().unwrap_or("".to_string()), Field::LinkedInUrl)}
                    {make_input(ctx, "Github".to_string(), self.github_url.clone().unwrap_or("".to_string()), Field::GithubUrl)}

                    <h5 class="text-xl font-bold text-left self-center pl-4 mt-4 mb-1"> {"Education"} </h5>
                    <hr/>
                    <EducationController callback={education_cb}/>

                    <h5 class="text-xl font-bold text-left self-center pl-4 mt-4 mb-1"> {"Experiences"} </h5>
                    <hr/>
                    <ExperienceController callback={experience_cb} />

                    <h5 class="text-xl font-bold text-left self-center pl-4 mt-4 mb-1"> {"Projects"} </h5>
                    <hr/>
                    <ProjectController callback={project_cb} />

                    <button
                        class="rounded-md w-full mt-10 bg-green-500 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-green-300"
                        onclick={ctx.link().callback(|_| HomeMsg::Print)}
                    >
                    {"Print"}
                    </button>
                </h1>
                <div class="flex w-2/3 justify-center">
                <div id="rusume" class="w-[1080px] p-4 bg-white text-xl text-black font-['Times']">
                    <div class="font-['Times'] tracking-normal">
                        <div class="text-4xl text-center"> {self.name.clone()} </div>
                        <div class="flex gap-x-8 text-center justify-center">
                            <div> {self.phone.clone()} </div>
                            {email}
                            {linkedin}
                            {github}
                        </div>

                        <h5 class="text-2xl font-bold text-left w-full border-b-2 border-black mt-4"> {"Education"} </h5>
                        <EducationViewer educations={self.educations.clone()} />
                        <h5 class="text-2xl font-bold text-left w-full border-b-2 border-black mt-4"> {"Experiences"} </h5>
                        <ExperienceViewer experiences={self.experiences.clone()} />
                        <h5 class="text-2xl font-bold text-left w-full border-b-2 border-black mt-4"> {"Projects"} </h5>
                        <ProjectViewer projects={self.projects.clone()} />
                    </div>
                </div>
                </div>
            </div>
        }
    }
}
fn make_input<F>(ctx: &Context<Home>, name: String, value: String, cons: F) -> Html
where
    F: Fn(String) -> Field + 'static,
{
    let callback = ctx.link().callback(move |e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        HomeMsg::UpdateField(cons(input.value()))
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
