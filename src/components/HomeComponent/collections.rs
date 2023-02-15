use yew::{html, Children, Component, Context, Html, Properties, function_component};


#[function_component(Collections)]
pub fn collections() -> Html {
    html! {
   <>
   <section id="collections">
   <div class="collection-row row">
       <h2 class="collection-heading">{"Top Collections"}</h2>
       <a href="#" class="collection-more">{"See More"}</a>
   
       <div class="col-lg-4 col-md-4">
           <img src="/assets/images/팬더.png" alt="" class="collection-img"/>
       </div>
       
       <div class="col-lg-4 col-md-4">
           <img src="/assets/images/인조인간.jpeg" alt="" class="collection-img"/>
       </div>
       
       <div class="col-lg-4 col-md-4">
           <img src="/assets/images/인조인간2.jpeg" alt="" class="collection-img"/>
       </div>            
       
       <div class="col-lg-4 col-md-4">
           <img src="/assets/images/파이리.jpeg" alt="" class="collection-img"/>
       </div>

       <div class="col-lg-4 col-md-4">
           <img src="/assets/images/합성인간.jpeg" alt="" class="collection-img"/>
       </div>

       <div class="col-lg-4 col-md-4">
           <img src="/assets/images/합성인간2.jpeg" alt="" class="collection-img"/>
       </div>
       
   </div>
</section>
 </>
    }
}




