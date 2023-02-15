use yew_router::prelude::*;
use yew::prelude::*;
#[function_component(Home)]
pub fn home() -> Html {

    html! {
      <>
      <div class="row">
      <div class="hero-text col-lg-6">
          <h1 class="hero-heading">{"Discover, collect and sell"} <span class="nft">{"NFTs"}</span></h1>
          <p class="hero-subheading">{"Digital marketplace for unique NFTs collections with best price and most secure platform to buy and trade digital ARTs"}</p>
          <a href="#" class="explore-btn btn btn-primary">{"Explore"}</a>
          <a href="#" class="create-btn btn btn-outline-primary">{"Create"}</a>
      </div>

      <div class="col-lg-6">
          <img class="hero-img" src="/assets/images/hero-img.png" alt=""/>
      </div>
      </div>
      </>
    }
}
