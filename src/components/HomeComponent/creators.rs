use yew::{html, Children, Component, Context, Html, Properties, function_component};


#[function_component(Creators)]
pub fn creators() -> Html {
    html! {
   <>
   <section id="creators">
   <div class="row">
       <h2 class="creator-heading">{"Follow Top Creators"}</h2>
       <div class="creator-div col-lg-5 col-md-4">
           <img class="creator-img" src="/assets/images/사람1.jpeg" alt=""/>
           <h6 class="creator-tag">{"Creator"}</h6>
           <p class="creator-name">{"Nelson Jeff"}</p>
           <a class="follow-btn" href="">{"Follow"}</a>
       </div>

       <div class="creator-div col-lg-5 col-md-4">
           <img class="creator-img" src="/assets/images/creator2.png" alt=""/>
           <h6 class="creator-tag">{"Creator"}</h6>
           <p class="creator-name">{"Joy Phillip"}</p>
           <a class="follow-btn" href="">{"Follow"}</a>
       </div>

       <div class="creator-div col-lg-5 col-md-4">
           <img class="creator-img" src="/assets/images/creator3.png" alt=""/>
           <h6 class="creator-tag">{"Creator"}</h6>
           <p class="creator-name">{"Collins Mark"}</p>
           <a class="follow-btn" href="">{"Follow"}</a>
       </div>

       <div class="creator-div col-lg-5 col-md-4">
           <img class="creator-img" src="/assets/images/creator4.png" alt=""/>
           <h6 class="creator-tag">{"Creator"}</h6>
           <p class="creator-name">{"Davis Ben"}</p>
           <a class="follow-btn" href="">{"Follow"}</a>
       </div>

   </div>
   <div class="link-container">
       <a href="#" class="view-more">{"See More"}</a>
   </div>
</section>
 </>
    }
}




