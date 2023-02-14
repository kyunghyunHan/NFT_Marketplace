use yew_router::prelude::*;
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    let navigator = use_navigator().unwrap();

 
    html! {
        <div>
            <h1>{ "Secure" }</h1>
           
        </div>
    }
}
