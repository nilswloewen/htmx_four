/// Declares header names as `&'static str` constants.
///
/// A re-export names the module that declares the constant, as a `crate::` path.
macro_rules! header_names {
    (
        $( $(#[$meta:meta])* $name:ident = $value:literal; )*
        $( @reexports {
            $( $(#[$re_meta:meta])* $re_name:ident from $($re_path:ident)::+; )*
        } )?
    ) => {
        $(
            $(#[$meta])*
            pub const $name: &str = $value;
        )*
        $($(
            $(#[$re_meta])*
            pub use $($re_path)::+::$re_name;
        )*)?
    };
}
