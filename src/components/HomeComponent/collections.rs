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
           <img src="/assets/images/collection1.png" alt="" class="collection-img"/>
       </div>
       
       <div class="col-lg-4 col-md-4">
           <img src="/assets/images/collection2.png" alt="" class="collection-img"/>
       </div>
       
       <div class="col-lg-4 col-md-4">
           <img src="/assets/images/collection3.png" alt="" class="collection-img"/>
       </div>            
       
       <div class="col-lg-4 col-md-4">
           <img src="/assets/images/collection4.png" alt="" class="collection-img"/>
       </div>

       <div class="col-lg-4 col-md-4">
           <img src="/assets/images/collection5.png" alt="" class="collection-img"/>
       </div>

       <div class="col-lg-4 col-md-4">
           <img src="/assets/images/collection6.png" alt="" class="collection-img"/>
       </div>
   </div>
</section>
 </>
    }
}




