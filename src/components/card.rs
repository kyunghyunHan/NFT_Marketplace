use yew::{html, Children, Component, Context, Html, Properties, function_component};



#[function_component(Card)]
pub fn card() -> Html {
    html! {
   <>
   <article class="card">
     <a class="item__img__container" href="#">
       <img class="item__img" src="https://kellychi22.github.io/frontend-mentor-solutions/04-nft-preview-card-component/images/image-equilibrium.jpg" alt="equilibrium-item-image"/>
     </a>
     <div class="item__info">
       <h1><a class="item__title" href="#">{"Equilibrium #3429"}</a></h1>
       <p class="item__desc">{"Our Equilibrium collection promotes balance and calm."}</p>
       <div class="item__price__time">
         <div class="item__price">
           <img class="item__icon" src="https://kellychi22.github.io/frontend-mentor-solutions/04-nft-preview-card-component/images/icon-ethereum.svg" alt="ethereum-icon"/>
           <span class="price-eth">{"0.041 ETH"}</span>
         </div>
         <div class="item__time">
           <img class="item__icon" src="https://kellychi22.github.io/frontend-mentor-solutions/04-nft-preview-card-component/images/icon-clock.svg" alt="clock-icon"/>
           <span class="days-left">{"3 days left"}</span>
         </div>
       </div>
       <div class="item__creator">
         <img class="creator__img" src="https://kellychi22.github.io/frontend-mentor-solutions/04-nft-preview-card-component/images/image-avatar.png" alt="creator_avator"/>
         <p class="creator__info">{"Creation of"} <a class="creator__name" href="#">{"Jules Wyvern"}</a></p>
       </div>
     </div>
   </article>

 </>
    }
}




