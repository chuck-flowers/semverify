use serde::Deserialize;
use serde::Serialize;
use syn::File;
use syn::Item;
use syn::ItemMod;

/// Describes a Rust module
#[derive(Default, Deserialize, Serialize)]
pub struct ModuleMetadata {
    consts: Vec<crate::ConstMetadata>,
    enums: Vec<crate::EnumMetadata>,
    functions: Vec<crate::FunctionMetadata>,
    macros: Vec<crate::MacroMetadata>,
    name: String,
    structs: Vec<crate::StructMetadata>,
    submodules: Vec<crate::ModuleMetadata>,
    traits: Vec<crate::TraitMetadata>,
}

impl From<ItemMod> for ModuleMetadata {
    fn from(src: syn::ItemMod) -> Self {
        src.content
            .map(|(_, items)| items.into())
            .unwrap_or_default()
    }
}

impl From<File> for ModuleMetadata {
    fn from(src: File) -> Self {
        src.items.into()
    }
}

impl From<Vec<Item>> for ModuleMetadata {
    fn from(items: Vec<Item>) -> Self {
        let name = src.ident.to_string();
        let mut consts = Vec::new();
        let mut enums = Vec::new();
        let mut functions = Vec::new();
        let mut macros = Vec::new();
        let mut structs = Vec::new();
        let mut submodules = Vec::new();
        let mut traits = Vec::new();

        for item in items {
            match item {
                Item::Const(const_item) => consts.push(const_item.into()),
                Item::Enum(enum_item) => enums.push(enum_item.into()),
                Item::Fn(fn_item) => functions.push(fn_item.into()),
                Item::Macro(macro_item) => macros.push(macro_item.into()),
                Item::Mod(mod_item) => submodules.push(mod_item.into()),
                Item::Struct(struct_item) => structs.push(struct_item.into()),
                Item::Trait(trait_item) => traits.push(trait_item.into()),
                _ => continue,
            }
        }

        Self {
            consts,
            enums,
            functions,
            macros,
            name,
            structs,
            submodules,
            traits,
        }
    }
}
