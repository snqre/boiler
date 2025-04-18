/// Declares a module hierarchy from a given path using `automod`.
///
/// # Example
/// ```rust,ignore
/// bundle!("src/routes");
/// ```
///
/// Requires the [`automod`](https://docs.rs/automod) crate.
#[macro_export]
macro_rules! bundle {
    ($path:expr) => {
        automod::dir!($path);
    };
}

/// Re-exports selected modules or items from a parent module into the current scope.
///
/// # Example
/// ```rust,ignore
/// public!(models, utils);
/// ```
///
/// This expands to:
/// ```rust,ignore
/// pub use super::models::*;
/// pub use super::utils::*;
/// ```
#[macro_export]
macro_rules! public {
    ( $( $module:ident ),* $(,)? ) => {
        $(
            #[allow(unused_imports)]
            pub use super::$module::*;
        )*
    };
}