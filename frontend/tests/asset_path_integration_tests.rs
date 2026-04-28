//! Integration tests verifying that asset URLs rendered through `asset_url()`
//! pick up the compile-time BASE_PATH and emit prefixed URLs in the DOM.
//!
//! With `BASE_PATH` unset the helper is a pass-through; with `BASE_PATH=/aml`
//! every absolute path gets prefixed. These tests run under whichever value
//! was active at compile time, so they assert the *concatenation* — not a
//! specific prefix — to remain valid in both dev and CI builds.

use dioxus::prelude::*;
use frontend::utils::asset_path::{asset_url, BASE_PATH};

#[test]
fn asset_url_prepends_base_path_to_absolute_path() {
    let url = asset_url("/assets/images/foo.svg");
    assert_eq!(url, format!("{BASE_PATH}/assets/images/foo.svg"));
}

#[test]
fn ssr_image_src_carries_base_path() {
    fn img_component() -> Element {
        let src = asset_url("/assets/alashore-logo.png");
        rsx! { img { src: "{src}" } }
    }

    let mut vdom = VirtualDom::new(img_component);
    vdom.rebuild_in_place();
    let html = dioxus::ssr::render(&vdom);

    let expected = format!("src=\"{BASE_PATH}/assets/alashore-logo.png\"");
    assert!(
        html.contains(&expected),
        "SSR HTML did not contain expected src.\nexpected substring: {expected}\nactual: {html}"
    );
}
