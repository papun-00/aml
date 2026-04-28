pub mod api;
pub mod asset_path;

pub use asset_path::asset_url;

#[allow(dead_code)]
pub fn canonical(path: &str) -> String {
    format!("https://alashoremarine.com{path}")
}
