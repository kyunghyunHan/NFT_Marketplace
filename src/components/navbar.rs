use yew::{html, Children, Component, Context, Html, Properties, function_component};

#[derive(Clone, PartialEq)]
pub struct Theme {
   pub foreground: String,
   pub background: String,
}

#[derive(PartialEq, Properties)]
pub struct NavbarProps {
    pub  theme: Theme,
}

#[function_component(Navbar)]
pub fn navbar(props: &NavbarProps) -> Html {
    html! {
   <>
   <header class="box-shadow">
   <div class="section logo">
     <span class="white">{"the"}</span><span class="green">{"devplace"}</span>
   </div>
   <div class="section">
     <ul>
       <li><a class="active" href="/">{"About"}</a></li>
       <li><a href="/secure">{"Services"}</a></li>
       <li><a href="#">{"Gallery"}</a></li>
       <li><a href="#">{"Contact"}</a></li>
     </ul>
   </div>
 </header>
 </>
    }
}




