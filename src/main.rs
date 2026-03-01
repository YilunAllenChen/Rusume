use std::fmt::Display;

use log::info;
use yew::prelude::*;
use yew_router::prelude::*;

mod pages;

use pages::Home;

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/contact")]
    Contact,
}

impl Display for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Route::Home => "Home",
            Route::Contact => "Contact",
        };
        write!(f, "{}", name)
    }
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! {<Home />},
        Route::Contact => html! {"WIP"},
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <HashRouter>
            <Switch<Route> render={switch} />
        </HashRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    info!("Logging initialized");
    yew::Renderer::<App>::new().render();
}
