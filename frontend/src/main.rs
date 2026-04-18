#![allow(non_snake_case)]

pub mod certifications;
mod components;
pub mod config;
mod pages;
pub mod products;
mod seo;
mod utils;

use dioxus::prelude::*;

// Re-export page components so the Routable derive macro can find them
use pages::about::AboutPage;
use pages::certifications::CertificationsPage;
use pages::contact::ContactPage;
use pages::home::HomePage;
use pages::inquiry::InquiryPage;
use pages::not_found::NotFoundPage;
use pages::product_detail::ProductDetailPage;
use pages::products::ProductsPage;
use pages::sustainability::SustainabilityPage;

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Info).ok();
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(components::layout::Layout)]
    #[route("/")]
    HomePage {},
    #[route("/about")]
    AboutPage {},
    #[route("/products")]
    ProductsPage {},
    #[route("/products/:id")]
    ProductDetailPage { id: String },
    #[route("/certifications")]
    CertificationsPage {},
    #[route("/sustainability")]
    SustainabilityPage {},
    #[route("/contact")]
    ContactPage {},
    #[route("/inquiry")]
    InquiryPage {},
    #[route("/:..route")]
    NotFoundPage { route: Vec<String> },
}
