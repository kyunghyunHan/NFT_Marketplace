use yew_router::prelude::*;
use yew::prelude::*;
use crate::components;
use components::{Randing,Collections,Creators};


#[function_component(Home)]
pub fn home() -> Html {

    html! {
      <>
      <Randing/>
      <Collections/>
      <Creators/>
      </>
    }
}
