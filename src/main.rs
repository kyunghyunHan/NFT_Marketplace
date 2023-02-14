use yew::{html, Children, Component, Context, Html, Properties, function_component};
mod components;
use components::Navbar;
use components::Theme;
use yew::{classes};
mod route;
mod pages;
use route::Route;
use yew_router::prelude::*;
use yew::prelude::*;
use pages::Home;

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {<Home/> },
        Route::Secure => html! {
         <div>{"da"}</div>
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

/// App root
#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
        <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
    </BrowserRouter>
        
    }
}
fn main() {
    yew::Renderer::<App>::new().render();
}