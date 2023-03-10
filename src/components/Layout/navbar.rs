use yew::{html, Children, Component, Context, Html, Properties, function_component};


#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
   <>
   <section id="hero-section">
   <div class="container-fluid">
       <nav class="navbar fixed-top navbar-expand-lg">
           <a href="/" class="navbar-brand">
               <img class="logo" src="/assets/images/Tess.png" alt="onex-logo"/>
           </a>
           <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarNavDropdown" aria-controls="navbarNavDropdown" aria-expanded="false" aria-label="Toggle navigation">
               <span class="navbar-toggler-icon"></span>
           </button>

           <div class="collapse navbar-collapse" id="navbarNavDropdown">
               <ul class="navbar-nav mx-auto mb-2 mb-lg-0">
               <li class="nav-item">
               <a href="/about" class="nav-link">{"About"}</a>
                </li> 
                   <li class="nav-item">
                       <a href="/marketplace" class="nav-link">{"Marketplace"}</a>
                   </li>
                   <li class="nav-item">
                       <a href="/card" class="nav-link">{"Card"}</a>
                   </li>
                 
               </ul>
               <div class="sign-in-btn ms-auto">
                   <a href="" class="sign-in btn btn-primary">{"Sign In"}</a>
               </div>
           </div>
   </nav>
   </div>
</section>
 </>
    }
}




