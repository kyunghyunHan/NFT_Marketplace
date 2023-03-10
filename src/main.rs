use yew::{html, Children, Component, Context, Html, Properties, function_component};
mod components;
use components::{Navbar,Footer};

use yew::{classes};
mod route;
mod pages;
use route::Route;
use yew_router::prelude::*;
use pages::{Home,About};
use components::Card;

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {<Home/> },
        Route::About=>html! {<About/> },
        Route::Marketplace => html! {
         <div calss={classes!(".navnav")}>{"da"}</div>
        },
        Route::Card => html! {
            <Card/>
           },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

/// App root
#[function_component]
fn App() -> Html {
   
    html! {
    <BrowserRouter>
       <Navbar  />
       <Switch<Route> render={switch} /> 
       <Footer/>
    </BrowserRouter>
        
    }
}
fn main() {
    yew::Renderer::<App>::new().render();
}