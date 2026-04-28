//! Asset path helper.
//!
//! Returns asset URLs prefixed with the deployment base path so the same source
//! works in dev (served from `/`) and in production (served from `/aml/` on
//! GitHub Pages). The prefix is read from the `BASE_PATH` env var at compile
//! time and falls back to an empty string when unset.

pub const BASE_PATH: &str = match option_env!("BASE_PATH") {
    Some(p) => p,
    None => "",
};

pub fn asset_url(path: &str) -> String {
    format!("{BASE_PATH}{path}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prefixes_absolute_paths() {
        let url = asset_url("/assets/foo.svg");
        assert!(url.ends_with("/assets/foo.svg"));
        assert_eq!(url, format!("{BASE_PATH}/assets/foo.svg"));
    }
}
