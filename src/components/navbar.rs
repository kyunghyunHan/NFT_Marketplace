use yew::{html, Children, classes, Context, Html, Properties, function_component};

#[derive(Clone, PartialEq)]
pub struct Theme {
   pub  foreground: String,
   pub background: String,
}

#[derive(PartialEq, Properties)]
pub struct NavbarProps {
   pub theme: Theme,
}

#[function_component(Navbar)]
pub fn navbar(props: &NavbarProps) -> Html {

    html! {
        <div class="navbar">
        <div styles="color: red;">{"dd"}</div>
          
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct ThemeProps {
    theme: Theme,
    children: Children,
}

