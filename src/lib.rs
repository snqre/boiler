/// Declares a module hierarchy from a given path using [`automod`].
///
/// This macro uses the `automod::dir!` macro to include all Rust files in a specified
/// directory as modules. It's particularly useful for organizing route handlers,
/// feature folders, or other modular structures.
///
/// # Example
/// ```rust,ignore
/// bundle!("src/routes");
/// ```
///
/// # Requirements
/// - Requires the [`automod`](https://docs.rs/automod) crate to be added as a dependency.
///
/// # Note
/// The path must be a string literal relative to the root of your crate.
#[macro_export]
macro_rules! bundle {
    ($path:expr) => {
        automod::dir!($path);
    };
}

/// Re-exports the provided modules directly into the current scope.
///
/// This macro assumes that the modules are available **in the same scope**
/// (i.e., they're siblings, not from `super::`). Use this when you want to
/// flatten modules and avoid nested access like `mod::Type`.
///
/// # Example
/// ```rust
/// expose!(models, utils);
/// ```
///
/// This expands to:
/// ```rust
/// pub use models::*;
/// pub use utils::*;
/// ```
#[macro_export]
macro_rules! expose {
    ( $( $module:ident ),* $(,)? ) => {
        $( 
            #[allow(unused_imports)]
            pub use $module::*;
        )*
    };
}

/// Re-exports modules from the **parent module** into the current scope.
///
/// Use this macro to publicly expose modules defined in a parent or sibling
/// scope. It simplifies your API by flattening module access — making it easier
/// to consume in other parts of your application.
///
/// # Example
///
/// ```rust
/// // src/app/mod.rs
/// pub mod models;
/// pub mod services;
/// pub mod prelude;
///
/// // src/app/prelude.rs
/// package!(models, services);
/// ```
///
/// This expands to:
/// ```rust
/// pub use super::models::*;
/// pub use super::services::*;
/// ```
///
/// # Result
/// Any module that imports `app::prelude::*` will now have access to everything
/// from `models` and `services` as if it were defined in `prelude`.
#[macro_export]
macro_rules! package {
    ( $( $module:ident ),* $(,)? ) => {
        $( 
            #[allow(unused_imports)]
            pub use super::$module::*;
        )*
    };
}

/// Marks a module as an extension of its parent by importing all parent items.
/// 
/// This macro expands to `use super::*;`, bringing all public items from the parent
/// module into the current scope. It is functionally equivalent to writing that line
/// manually, but serves as a **semantic indicator** that the current module is meant
/// to build upon or extend its parent.
/// 
/// # Purpose
/// Use `extend!` when you want to clearly signal intent — that this module
/// relies on or enhances the parent — rather than just importing for utility.
/// It's especially helpful in larger codebases where clarity and structure matter.
///
/// # Note
/// This macro does not perform any re-exports. It is purely a semantic helper
/// for organizing and understanding module relationships.
#[macro_export]
macro_rules! extend {
    () => {
        use super::*;   
    };
}