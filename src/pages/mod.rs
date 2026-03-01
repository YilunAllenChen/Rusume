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
use projects::ProjectController;
use projects::ProjectViewer;

mod skills;
use skills::SkillController;
use skills::SkillViewer;

mod open_source;
use open_source::OpenSourceController;
use open_source::OpenSourceViewer;

mod state;
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

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        let mut state_changed = false;
        match msg {
            HomeMsg::Print => {
                if let Some(window) = window() {
                    scroll_print_view_to_top(&window);
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
        let education_cb = ctx.link().callback(HomeMsg::UpdateEducationSection);
        let experience_cb = ctx.link().callback(HomeMsg::UpdateExperienceSection);
        let project_cb = ctx.link().callback(HomeMsg::UpdateProjectSection);
        let basic_cb = ctx.link().callback(HomeMsg::UpdateBasicSection);
        let skill_cb = ctx.link().callback(HomeMsg::UpdateSkillSection);
        let open_source_cb = ctx.link().callback(HomeMsg::UpdateOpenSourceSection);

        let print_button = html! {
            <button
                class="sidebar-print-btn"
                onclick={ctx.link().callback(|_| HomeMsg::Print)}
            >
            {"Print"}
            </button>
        };

        html! {
            <div class="w-screen h-screen flex">
                <aside class="hidden md:flex md:flex-col resize-x min-w-[31rem] no-print sidebar-shell">
                    <div class="sidebar-header">
                        <div>
                            <div class="sidebar-kicker">{"Rusume"}</div>
                            <h2 class="sidebar-title">{"Resume Editor"}</h2>
                            <p class="sidebar-subtitle">{"Craft and tune your resume with live preview."}</p>
                        </div>
                        {print_button.clone()}
                    </div>
                    <div class="sidebar-scroll">
                        <BasicController value={self.state.basic.clone()} on_change={basic_cb}/>
                        <SkillController value={self.state.skills.clone()} on_change={skill_cb} />
                        <EducationController value={self.state.educations.clone()} on_change={education_cb}/>
                        <ExperienceController value={self.state.experiences.clone()} on_change={experience_cb} />
                        <ProjectController value={self.state.projects.clone()} on_change={project_cb} />
                        <OpenSourceController value={self.state.open_sources.clone()} on_change={open_source_cb} />
                    </div>
                </aside>
                <div id="preview-scroll-area" class="flex-1 bg-slate-100 overflow-scroll">
                    <div class="preview-stage">
                        <div id="rusume" class="preview-paper">
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
            </div>
        }
    }
}

fn scroll_print_view_to_top(window: &web_sys::Window) {
    window.scroll_to_with_x_and_y(0.0, 0.0);
    let Some(document) = window.document() else {
        return;
    };
    if let Some(preview) = document.get_element_by_id("preview-scroll-area") {
        if let Ok(preview) = preview.dyn_into::<web_sys::HtmlElement>() {
            preview.set_scroll_top(0);
        }
    }
}
