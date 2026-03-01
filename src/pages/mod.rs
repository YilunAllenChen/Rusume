use wasm_bindgen::JsCast;
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

mod state;
use state::default_seed_state;
use state::load_state_from_url;
use state::write_state_to_url;
use state::AppState;

pub struct Home {
    state: AppState,
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
        let state = load_state_from_url();
        write_state_to_url(&state);
        Self { state }
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        let mut state_changed = false;
        match msg {
            HomeMsg::Print => {
                if let Some(window) = window() {
                    if exceeds_single_page_estimate() {
                        let _ = window
                            .alert_with_message("Warning - resumes typically is 1-page long!");
                    }
                    let _ = window.print();
                }
            }
            HomeMsg::UpdateBasicSection(basic) => {
                if self.state.basic != basic {
                    self.state.basic = basic;
                    state_changed = true;
                }
            }
            HomeMsg::UpdateEducationSection(educations) => {
                if self.state.educations != educations {
                    self.state.educations = educations;
                    state_changed = true;
                }
            }
            HomeMsg::UpdateExperienceSection(experiences) => {
                if self.state.experiences != experiences {
                    self.state.experiences = experiences;
                    state_changed = true;
                }
            }
            HomeMsg::UpdateProjectSection(projects) => {
                if self.state.projects != projects {
                    self.state.projects = projects;
                    state_changed = true;
                }
            }
            HomeMsg::UpdateSkillSection(skills) => {
                if self.state.skills != skills {
                    self.state.skills = skills;
                    state_changed = true;
                }
            }
            HomeMsg::UpdateOpenSourceSection(open_sources) => {
                if self.state.open_sources != open_sources {
                    self.state.open_sources = open_sources;
                    state_changed = true;
                }
            }
        }

        if state_changed {
            write_state_to_url(&self.state);
        }

        state_changed
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
                <h1 class="hidden md:block resize-x overflow-x-auto bg-slate-200 overflow-y-scroll min-w-[28rem] no-print">
                    {print_button.clone()}
                    <BasicController value={self.state.basic.clone()} on_change={basic_cb}/>
                    <SkillController value={self.state.skills.clone()} on_change={skill_cb} />
                    <EducationController value={self.state.educations.clone()} on_change={education_cb}/>
                    <ExperienceController value={self.state.experiences.clone()} on_change={experience_cb} />
                    <ProjectController value={self.state.projects.clone()} on_change={project_cb} />
                    <OpenSourceController value={self.state.open_sources.clone()} on_change={open_source_cb} />
                    {print_button.clone()}
                </h1>
                <div class="flex-1 justify-center bg-slate-100 overflow-scroll">
                <div id="rusume" class="flex-none bg-white min-h-full">
                    <div class="font-['Arial'] text-lg tracking-normal p-10">
                        <BasicViewer basic={self.state.basic.clone()} />
                        <SkillViewer skills={self.state.skills.clone()} />
                        <ExperienceViewer experiences={self.state.experiences.clone()} />
                        <ProjectViewer projects={self.state.projects.clone()} />
                        <OpenSourceViewer open_sources={self.state.open_sources.clone()} />
                        <EducationViewer educations={self.state.educations.clone()} />
                    </div>
                </div>
                </div>
            </div>
        }
    }
}

fn exceeds_single_page_estimate() -> bool {
    let Some(window) = window() else {
        return false;
    };
    let Some(document) = window.document() else {
        return false;
    };
    let Some(rusume) = document.get_element_by_id("rusume") else {
        return false;
    };
    let Ok(rusume) = rusume.dyn_into::<web_sys::HtmlElement>() else {
        return false;
    };

    let unscaled_height_px = f64::from(rusume.scroll_height());
    let print_scale = 0.7;
    let printable_one_page_height_px = 960.0;
    (unscaled_height_px * print_scale) > printable_one_page_height_px
}

impl Default for Home {
    fn default() -> Self {
        Self {
            state: default_seed_state(),
        }
    }
}
