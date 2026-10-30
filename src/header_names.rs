/// Declares header names as `&'static str` constants, and the same names as
/// [`http::HeaderName`](https://docs.rs/http/1/http/header/struct.HeaderName.html) constants in a
/// child `http` module when the `http` feature is on.
///
/// A re-export names the module that declares the constant, as a `crate::` path. The `http`
/// module re-exports the `HeaderName` constant from that module's own `http` child.
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

        /// The same header names as [`http::HeaderName`](::http::HeaderName) values.
        #[cfg(feature = "http")]
        #[cfg_attr(docsrs, doc(cfg(feature = "http")))]
        pub mod http {
            $(
                $(#[$meta])*
                pub const $name: ::http::HeaderName = ::http::HeaderName::from_static(super::$name);
            )*
            $($(
                $(#[$re_meta])*
                pub use $($re_path)::+::http::$re_name;
            )*)?
        }
    };
}

#[cfg(all(test, feature = "http"))]
mod tests {
    use crate::ext::{hx_multipart, hx_prompt, hx_ptag, hx_sse};
    use crate::{request_headers, response_headers};

    #[test]
    fn http_names_match_str_names() {
        assert_eq!(
            request_headers::http::HX_REQUEST,
            request_headers::HX_REQUEST
        );
        assert_eq!(
            response_headers::http::HX_TRIGGER,
            response_headers::HX_TRIGGER
        );
        assert_eq!(
            hx_sse::request_headers::http::LAST_EVENT_ID,
            hx_sse::request_headers::LAST_EVENT_ID
        );
    }

    #[test]
    fn http_reexports_follow_their_source() {
        assert_eq!(
            hx_multipart::part_headers::http::HX_TARGET,
            request_headers::http::HX_TARGET
        );
        assert_eq!(
            hx_prompt::request_headers::http::HX_PROMPT,
            request_headers::http::HX_PROMPT
        );
        assert_eq!(
            hx_ptag::response_headers::http::HX_PTAG,
            hx_ptag::http::HX_PTAG
        );
    }

    #[test]
    fn status_code_converts_to_exact() {
        assert_eq!(
            crate::StatusPattern::from(::http::StatusCode::NOT_FOUND),
            crate::StatusPattern::Exact(404)
        );
    }
}
