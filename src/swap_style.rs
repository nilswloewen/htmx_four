use serde::{Serialize, Serializer};
use strum::{Display, IntoStaticStr};

/// An [`hx-swap`](https://four.htmx.org/reference/attributes/hx-swap) style.
///
/// `upsert` needs [`hx-upsert`](https://four.htmx.org/extensions/hx-upsert).
/// `download` needs [`hx-download`](https://four.htmx.org/extensions/hx-download).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Display, IntoStaticStr)]
pub enum SwapStyle {
    /// [`innerHTML`](https://four.htmx.org/reference/attributes/hx-swap#innerhtml): replace the element's children.
    #[strum(serialize = "innerHTML")]
    InnerHtml,
    /// [`outerHTML`](https://four.htmx.org/reference/attributes/hx-swap#outerhtml): replace the element.
    #[strum(serialize = "outerHTML")]
    OuterHtml,
    /// [`textContent`](https://four.htmx.org/reference/attributes/hx-swap#textcontent): replace the text, without parsing HTML.
    #[strum(serialize = "textContent")]
    TextContent,
    /// [`beforebegin`](https://four.htmx.org/reference/attributes/hx-swap#beforebegin--before): insert before the element.
    #[strum(serialize = "beforebegin")]
    BeforeBegin,
    /// [`afterbegin`](https://four.htmx.org/reference/attributes/hx-swap#afterbegin--prepend): insert as the first child.
    #[strum(serialize = "afterbegin")]
    AfterBegin,
    /// [`beforeend`](https://four.htmx.org/reference/attributes/hx-swap#beforeend--append): insert as the last child.
    #[strum(serialize = "beforeend")]
    BeforeEnd,
    /// [`afterend`](https://four.htmx.org/reference/attributes/hx-swap#afterend--after): insert after the element.
    #[strum(serialize = "afterend")]
    AfterEnd,
    /// [`innerMorph`](https://four.htmx.org/reference/attributes/hx-swap#innermorph): morph the element's children.
    #[strum(serialize = "innerMorph")]
    InnerMorph,
    /// [`outerMorph`](https://four.htmx.org/reference/attributes/hx-swap#outermorph): morph the element.
    #[strum(serialize = "outerMorph")]
    OuterMorph,
    /// [`outerSync`](https://four.htmx.org/reference/attributes/hx-swap#outersync): sync the element's attributes and replace its children.
    #[strum(serialize = "outerSync")]
    OuterSync,
    /// [`upsert`](https://four.htmx.org/reference/attributes/hx-swap#upsert): update elements with the same id and insert the rest.
    #[strum(serialize = "upsert")]
    Upsert,
    /// [`download`](https://four.htmx.org/extensions/hx-download#explicit-swap-style): save the response as a file.
    #[strum(serialize = "download")]
    Download,
    /// [`delete`](https://four.htmx.org/reference/attributes/hx-swap#delete): remove the element.
    #[strum(serialize = "delete")]
    Delete,
    /// [`none`](https://four.htmx.org/reference/attributes/hx-swap#none): do not insert the response.
    #[strum(serialize = "none")]
    None,
}

impl Serialize for SwapStyle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl SwapStyle {
    /// The [`hx-swap`](https://four.htmx.org/reference/attributes/hx-swap) style string.
    pub fn as_str(self) -> &'static str {
        self.into()
    }
}
