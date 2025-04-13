use web_sys::window;
use yew::prelude::*;

mod html_utils;

mod basic;
use basic::Basic;
use basic::BasicController;
use basic::BasicViewer;

mod education;
use education::Education;
use education::EducationController;
use education::EducationViewer;

mod experiences;
use experiences::Experience;
use experiences::ExperienceController;
use experiences::ExperienceViewer;

mod projects;
use projects::Project;
use projects::ProjectController;
use projects::ProjectViewer;

mod skills;
use skills::SkillCategory;
use skills::SkillController;
use skills::SkillViewer;

mod open_source;
use open_source::OpenSource;
use open_source::OpenSourceController;
use open_source::OpenSourceViewer;

pub struct Home {
    basic: Basic,
    educations: Vec<Education>,
    skills: Vec<SkillCategory>,
    experiences: Vec<Experience>,
    projects: Vec<Project>,
    open_sources: Vec<OpenSource>,
}

pub enum HomeMsg {
    Print,
    UpdateBasicSection(Basic),
    UpdateEducationSection(Vec<Education>),
    UpdateExperienceSection(Vec<Experience>),
    UpdateProjectSection(Vec<projects::Project>),
    UpdateSkillSection(Vec<skills::SkillCategory>),
    UpdateOpenSourceSection(Vec<open_source::OpenSource>),
}

#[derive(Properties, PartialEq)]
pub struct HomeProps {}

impl Component for Home {
    type Message = HomeMsg;
    type Properties = HomeProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            basic: Basic::default(),
            educations: [].to_vec(),
            skills: [].to_vec(),
            experiences: [].to_vec(),
            projects: [].to_vec(),
            open_sources: [].to_vec(),
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            HomeMsg::Print => {
                let window = window().unwrap();
                let doc = window.document().unwrap();
                let print_content = doc.get_element_by_id("rusume").unwrap().inner_html();
                doc.body().unwrap().set_inner_html(print_content.as_str());
                window.print().unwrap();
            }
            HomeMsg::UpdateBasicSection(basic) => {
                self.basic = basic;
            }
            HomeMsg::UpdateEducationSection(educations) => {
                self.educations = educations;
            }
            HomeMsg::UpdateExperienceSection(experiences) => {
                self.experiences = experiences;
            }
            HomeMsg::UpdateProjectSection(projects) => {
                self.projects = projects;
            }
            HomeMsg::UpdateSkillSection(skills) => {
                self.skills = skills;
            }
            HomeMsg::UpdateOpenSourceSection(open_sources) => {
                self.open_sources = open_sources;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let education_cb = ctx
            .link()
            .callback(|educations| HomeMsg::UpdateEducationSection(educations));
        let experience_cb = ctx
            .link()
            .callback(|experience| HomeMsg::UpdateExperienceSection(experience));
        let project_cb = ctx
            .link()
            .callback(|projects| HomeMsg::UpdateProjectSection(projects));
        let basic_cb = ctx
            .link()
            .callback(|basic| HomeMsg::UpdateBasicSection(basic));
        let skill_cb = ctx
            .link()
            .callback(|skills| HomeMsg::UpdateSkillSection(skills));
        let open_source_cb = ctx
            .link()
            .callback(|open_sources| HomeMsg::UpdateOpenSourceSection(open_sources));

        let print_button = html! {
            <button
                class="rounded-md w-full mt-10 bg-green-500 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-green-300"
                onclick={ctx.link().callback(|_| HomeMsg::Print)}
            >
            {"Print"}
            </button>
        };

        html! {
            <div class="w-screen h-screen flex">
                <h1 class="w-1/3 p-4 bg-slate-200 overflow-y-scroll">

                    {print_button.clone()}
                    <BasicController callback={basic_cb}/>
                    <SkillController callback={skill_cb} />
                    <EducationController callback={education_cb}/>
                    <ExperienceController callback={experience_cb} />
                    <ProjectController callback={project_cb} />
                    <OpenSourceController callback={open_source_cb} />
                    {print_button.clone()}
                </h1>
                <div class="flex w-2/3 justify-center bg-slate-100 ">
                <div id="rusume" class="flex-none max-w-[816x] bg-white overflow-scroll">
                    <div class="font-['Arial'] text-lg tracking-normal p-10">
                        <BasicViewer basic={self.basic.clone()} />
                        <SkillViewer skills={self.skills.clone()} />
                        <ExperienceViewer experiences={self.experiences.clone()} />
                        <ProjectViewer projects={self.projects.clone()} />
                        <OpenSourceViewer open_sources={self.open_sources.clone()} />
                        <EducationViewer educations={self.educations.clone()} />
                    </div>
                </div>
                </div>
            </div>
        }
    }
}
