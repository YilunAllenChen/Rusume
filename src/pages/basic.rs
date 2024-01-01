use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Basic {
    name: String,
    email: String,
    phone: String,
    linkedin_url: Option<String>,
    github_url: Option<String>,
}

impl Default for Basic {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            email: "".to_string(),
            phone: "".to_string(),
            linkedin_url: None,
            github_url: None,
        }
    }
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
    pub callback: Callback<Basic>,
}

pub enum BasicMsg {
    UpdateField(BasicField),
}

pub struct BasicController {
    basic: Basic,
}

impl Component for BasicController {
    type Message = BasicMsg;
    type Properties = BasicControllerProps;

    fn create(ctx: &Context<Self>) -> Self {
        let init = Basic {
            name: "Yilun \"Allen\" Chen".to_string(),
            email: "allenchenyilun1999@gmail.com".to_string(),
            phone: "404-409-9683".to_string(),
            linkedin_url: "https://www.linkedin.com/in/yilun-allen-chen-572b71141/"
                .to_string()
                .into(),
            github_url: "https://github.com/YilunAllenChen".to_string().into(),
        };
        let slf = Self { basic: init };
        ctx.props().callback.emit(slf.basic.clone());
        slf
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            BasicMsg::UpdateField(field) => match field {
                BasicField::Name(name) => self.basic.name = name,
                BasicField::Email(email) => self.basic.email = email,
                BasicField::Phone(phone) => self.basic.phone = phone,
                BasicField::LinkedInUrl(url) => self.basic.linkedin_url = url.into(),
                BasicField::GithubUrl(url) => self.basic.github_url = url.into(),
            },
        }
        ctx.props().callback.emit(self.basic.clone());
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let name_input = make_input(
            ctx,
            "Name".to_string(),
            self.basic.name.clone(),
            BasicField::Name,
        );

        let email_input = make_input(
            ctx,
            "Email".to_string(),
            self.basic.email.clone(),
            BasicField::Email,
        );

        let phone_input = make_input(
            ctx,
            "Phone".to_string(),
            self.basic.phone.clone(),
            BasicField::Phone,
        );

        let linkedin_url_input = make_input(
            ctx,
            "LinkedIn URL".to_string(),
            self.basic.linkedin_url.clone().unwrap_or_default(),
            BasicField::LinkedInUrl,
        );

        let github_url_input = make_input(
            ctx,
            "Github URL".to_string(),
            self.basic.github_url.clone().unwrap_or_default(),
            BasicField::GithubUrl,
        );

        html! {
            <>
            <h5 class="text-xl font-bold text-left self-center pl-4 mt-4 mb-1"> {"Basic Information"} </h5>
            <hr/>
            <div class="space-y-2 bg-slate-100 rounded-lg p-4 my-4">
                {name_input}
                {email_input}
                {phone_input}
                {linkedin_url_input}
                {github_url_input}
            </div>
            </>
        }
    }
}

fn make_input<F>(ctx: &Context<BasicController>, name: String, value: String, cons: F) -> Html
where
    F: Fn(String) -> BasicField + 'static,
{
    let callback = ctx.link().callback(move |e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        BasicMsg::UpdateField(cons(input.value()))
    });
    html! {
    <div class="m-2">
        <div class="relative">
            <input type="text"
                   id={name.clone()}
                   oninput={callback}
                   class="peer rounded-md px-2 pt-5 pb-1 block w-full border-0 border-b-2 border-gray-300 focus:ring-0 focus:border-black pt-2"
                   value={value}
            />
            <label for={name.clone()}
                   class="absolute -top-0 left-2 text-gray-500 text-sm transition-all peer-placeholder-shown:text-xl peer-placeholder-shown:text-gray-400 peer-placeholder-shown:top-0">
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
        Some(url) => {
            html! {
                <a class="text-blue-700" target="_blank" href={url.clone()}>
                    <i class="devicon-linkedin-plain"/> {" LinkedIn"}
                </a>
            }
        }
        None => html! {},
    };

    let github = match &props.basic.github_url {
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
