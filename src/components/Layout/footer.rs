use yew::{html, Children, Component, Context, Html, Properties, function_component};


#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
   <>
   <footer class="footer-section" id="footer">
   <div class="row">
       <div class="col-lg-3">
           <img class="footer-logo" src="images/logo.png" alt=""/>
           <p class="footer-subtitle">{"Get started with the easiest and most secure platform to buy and trade digital ART and NFTs."}</p>
       </div>

       <div class="col-lg-2">
           <h4 class="footer-link-heading">{"Company"}</h4>
           <ul>
               <li>
                   <a href="">{"About"}</a>
               </li>
               <li>
                   <a href="">{"Careers"}</a>
               </li>
               <li>
                   <a href="">{"Ventures"}</a>
               </li>
               <li>
                   <a href="">{"Grants"}</a>
               </li>
           </ul>
       </div>

       <div class="col-lg-2">
           <h4 class="footer-link-heading">{"Resources"}</h4>
           <ul>
               <li>
                   <a href="">{"Help Center"}</a>
               </li>
               <li>
                   <a href="">{"Platform Status"}</a>
               </li>
               <li>
                   <a href="">{"Partners"}</a>
               </li>
               <li>
                   <a href="">{"Blog"}</a>
               </li>
               <li>
                   <a href="">{"Newsletter"}</a>
               </li>
           </ul>
       </div>

       <div class="col-lg-2">
           <h4 class="footer-link-heading">{"Marketplace"}</h4>
           <ul>
               <li>
                   <a href="">{"All NFTs"}</a>
               </li>
               <li>
                   <a href="">{"Art"}</a>
               </li>
               <li>
                   <a href="">{"Collections"}</a>
               </li>
               <li>
                   <a href="">{"Music"}</a>
               </li>
               <li>
                   <a href="">{"Photography"}</a>
               </li>
               <li>
                   <a href="">{"Sports"}</a>
               </li>
               <li>
                   <a href="">{"Worlds"}</a>
               </li>
           </ul>
       </div>

       <div class="col-lg-3">
           <h5 class="footer-subscribe-heading">{"Subscribe to get updates about NFTs"}</h5>
           <div class="input-group mb-3">
               <input type="text" class="subscribe-input form-control" placeholder="Enter your email" aria-label="Recipient's username" aria-describedby="button-addon2"/>
               <button class="subscribe-btn btn" type="button" id="button-addon2">{"Subscribe"}</button>
             </div>
       </div>
       <p class="copyright">{"Copyright"} {"&copy; 2022 ONEX Access, Inc. "}<br/> <br/>
           {"{Designed with &#9825 by"} <a href="https://www.behance.net/ugommaa">{"Ugomma Hilda"}</a> <br/><br/>
           {"Built with &#9825 by "}<a href="https://github.com/victoriaEssien">{"Victoria Essien"}</a>
       </p>
   </div>
</footer>
 </>
    }
}




