use yew_router::prelude::*;
use yew::prelude::*;
use yew::{classes};
#[function_component(Home)]
pub fn home() -> Html {
    let navigator = use_navigator().unwrap();

 
    html! {
        <div>
            <h1 class={classes!("navnav")}> { "Secure" }</h1>
           
        </div>
    }
}
