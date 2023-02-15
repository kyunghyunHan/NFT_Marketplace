use yew_router::prelude::*;
use yew::prelude::*;


#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/marketplace")]
    Marketplace,
    #[at("/card")]
    Card,
    #[not_found]
    #[at("/404")]
    NotFound,
}
