use yew_router::prelude::*;
use yew::prelude::*;


#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/secure")]
    Secure,
    #[at("/card")]
    Card,
    #[not_found]
    #[at("/404")]
    NotFound,
}
