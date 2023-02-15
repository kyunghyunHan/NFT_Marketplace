use yew_router::prelude::*;
use yew::prelude::*;
use crate::components;
use components::{FirstHome,Collections};


#[function_component(Home)]
pub fn home() -> Html {

    html! {
      <>
      <FirstHome/>
      <Collections/>
      </>
    }
}
