use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;

use super::html_utils::{
    INPUT_CLASS, INPUT_DIVIDER_CLASS, INPUT_FIELD_WRAPPER_CLASS, INPUT_SECTION_CLASS, LABEL_CLASS,
};

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Basic {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub linkedin_url: Option<String>,
    pub github_url: Option<String>,
}

pub enum BasicField {
    Name(String),
    Email(String),
    Phone(String),
    LinkedInUrl(String),
    GithubUrl(String),
}

#[derive(Properties, PartialEq)]
pub struct BasicControllerProps {
    pub value: Basic,
    pub on_change: Callback<Basic>,
}

#[function_component(BasicController)]
pub fn basic_controller(props: &BasicControllerProps) -> Html {
    let name_input = make_input(
        "Name".to_string(),
        props.value.name.clone(),
        BasicField::Name,
        props,
    );

    let email_input = make_input(
        "Email".to_string(),
        props.value.email.clone(),
        BasicField::Email,
        props,
    );

    let phone_input = make_input(
        "Phone".to_string(),
        props.value.phone.clone(),
        BasicField::Phone,
        props,
    );

    let linkedin_url_input = make_input(
        "LinkedIn URL".to_string(),
        props.value.linkedin_url.clone().unwrap_or_default(),
        BasicField::LinkedInUrl,
        props,
    );

    let github_url_input = make_input(
        "Github URL".to_string(),
        props.value.github_url.clone().unwrap_or_default(),
        BasicField::GithubUrl,
        props,
    );

    html! {
        <>
        <h5 class={INPUT_SECTION_CLASS}> {"Basic Information"} </h5>
        <div class={INPUT_DIVIDER_CLASS}></div>
        {name_input}
        {email_input}
        {phone_input}
        {linkedin_url_input}
        {github_url_input}
        </>
    }
}

fn make_input<F>(name: String, value: String, cons: F, props: &BasicControllerProps) -> Html
where
    F: Fn(String) -> BasicField + 'static,
{
    let current_basic = props.value.clone();
    let on_change = props.on_change.clone();
    let callback = Callback::from(move |e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        let mut next_basic = current_basic.clone();
        match cons(input.value()) {
            BasicField::Name(name) => next_basic.name = name,
            BasicField::Email(email) => next_basic.email = email,
            BasicField::Phone(phone) => next_basic.phone = phone,
            BasicField::LinkedInUrl(url) => next_basic.linkedin_url = Some(url),
            BasicField::GithubUrl(url) => next_basic.github_url = Some(url),
        }
        on_change.emit(next_basic.clone());
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
    pub basic: Basic,
}

#[function_component(BasicViewer)]
pub fn view_basic(props: &Props) -> Html {
    let linkedin = match &props.basic.linkedin_url {
        Some(url) => match url.is_empty() {
            true => html! {},
            false => html! {
                <a class="text-blue-700" target="_blank" href={url.clone()}>
                    <i class="devicon-linkedin-plain"/> {" LinkedIn"}
                </a>
            },
        },
        None => html! {},
    };

    let github = match &props.basic.github_url {
        Some(url) => match url.is_empty() {
            true => html! {},
            false => html! {
                <a class="text-blue-700" target="_blank" href={url.clone()}>
                    <i class="devicon-github-original text-black"/> {" Github"}
                </a>
            },
        },
        None => html! {},
    };

    let email = html! {
        <a class="text-blue-700" href={"mailto:".to_string() + &props.basic.email.clone()}>
            {props.basic.email.clone()}
        </a>
    };

    html! {
        <>
        <div class="text-4xl text-center"> {&props.basic.name} </div>
        <div class="flex gap-x-8 text-center justify-center">
            <div> {&props.basic.phone} </div>
            {email}
            {linkedin}
            {github}
        </div>
        </>
    }
}
