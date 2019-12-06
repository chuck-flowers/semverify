use serde::Deserialize;
use serde::Serialize;
use std::convert::TryFrom;
use std::string::ToString;
use syn::ItemMacro;

/// Describes a Rust macro
#[derive(Deserialize, Serialize)]
pub struct MacroMetadata {
    name: String,
}

impl TryFrom<ItemMacro> for MacroMetadata {
    type Error = ();
    fn try_from(value: ItemMacro) -> Result<Self, Self::Error> {
        value
            .ident
            .map(|ident_text| ident_text.to_string())
            .map(|name| Self { name })
            .ok_or(())
    }
}
