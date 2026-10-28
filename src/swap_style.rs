use serde::{Serialize, Serializer};
use strum::{Display, IntoStaticStr};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display, IntoStaticStr)]
pub enum SwapStyle {
    #[strum(serialize = "innerHTML")]
    InnerHtml,
    #[strum(serialize = "outerHTML")]
    OuterHtml,
    #[strum(serialize = "textContent")]
    TextContent,
    #[strum(serialize = "beforebegin")]
    BeforeBegin,
    #[strum(serialize = "afterbegin")]
    AfterBegin,
    #[strum(serialize = "beforeend")]
    BeforeEnd,
    #[strum(serialize = "afterend")]
    AfterEnd,
    #[strum(serialize = "innerMorph")]
    InnerMorph,
    #[strum(serialize = "outerMorph")]
    OuterMorph,
    #[strum(serialize = "outerSync")]
    OuterSync,
    #[strum(serialize = "upsert")]
    Upsert,
    #[strum(serialize = "download")]
    Download,
    #[strum(serialize = "delete")]
    Delete,
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
    pub fn as_str(self) -> &'static str {
        self.into()
    }
}
