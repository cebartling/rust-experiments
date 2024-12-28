use crate::components::product_listing_view::ProductListingView;
use crate::components::shopping_cart_view::ShoppingCartView;
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    
    
    view! {
        <div class="flex flex-col align-item-start justify-item-start p-5">
            <h1 class="text-4xl mb-8">"Leptos Shopping Cart Example"</h1>
            <div class="flex flex-row">
                <ProductListingView />
                <ShoppingCartView />
            </div>
        </div>
    }
}
