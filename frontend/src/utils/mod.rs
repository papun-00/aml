pub mod api;

#[allow(dead_code)]
pub fn canonical(path: &str) -> String {
    format!("https://alashoremarine.com{path}")
}
