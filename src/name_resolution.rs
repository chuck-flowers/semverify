use std::collections::HashMap;
use std::convert::TryFrom;
use std::path::Path;
use syn::File;
use syn::Ident;
use syn::Item;

/// Rewrite the idents of a `syn::File` (and all referenced files) to be fully qualified
#[allow(dead_code)]
pub fn resolve_file_identifiers(file: &mut File, path: &Path) -> Result<(), ()> {
    Context::try_from(path)
        .map(|mut context| resolve_file_identifiers_in_context(file, path, &mut context))
}

/// Rewrite the idents of a `syn::File` (and all referenced files) to be fully
/// qualified using the provided `Context`.
fn resolve_file_identifiers_in_context<'a, 'b>(
    file: &mut File,
    path: &'a Path,
    context: &mut Context<'b>,
) where
    'a: 'b,
{
    context.step_into_file(path);
    resolve_item_identifiers_in_context(file.items.iter_mut(), context);
    context.step_out_of_file();
}

/// Rewrites all identifiers within an `Iterator` of mutable item references to
/// be fully qualified within a provided Context.
fn resolve_item_identifiers_in_context<'a>(
    items: impl Iterator<Item = &'a mut Item>,
    context: &mut Context,
) {
    for item in items {
        match item {
            Item::Const(_) => unimplemented!(),
            Item::Enum(_) => unimplemented!(),
            Item::ExternCrate(_) => unimplemented!(),
            Item::Fn(_) => unimplemented!(),
            Item::ForeignMod(_) => unimplemented!(),
            Item::Impl(_) => unimplemented!(),
            Item::Macro(_) => unimplemented!(),
            Item::Macro2(_) => unimplemented!(),
            Item::Mod(_) => unimplemented!(),
            Item::Static(_) => unimplemented!(),
            Item::Struct(_) => unimplemented!(),
            Item::Trait(_) => unimplemented!(),
            Item::TraitAlias(_) => unimplemented!(),
            Item::Type(_) => unimplemented!(),
            Item::Union(_) => unimplemented!(),
            Item::Use(_) => unimplemented!(),
            Item::Verbatim(_) => unimplemented!(),
            _ => continue,
        }
    }
}

struct Context<'a> {
    root_file: &'a Path,
    frames: Vec<ContextFrame<'a>>,
}

impl<'a> TryFrom<&'a Path> for Context<'a> {
    type Error = ();

    fn try_from(root_file: &'a Path) -> Result<Self, ()> {
        if root_file.is_file() {
            Ok(Self {
                root_file,
                frames: Vec::default(),
            })
        } else {
            Err(())
        }
    }
}

impl<'a> Context<'a> {
    /// Creates a new scope for the file
    fn step_into_file(&mut self, path: &'a Path) {
        let frame = ContextFrame::from(path);
        self.frames.push(frame);
    }

    /// Steps out of the current scope to the previous scope
    fn step_out_of_file(&mut self) {
        self.frames.pop();
    }

    /// Getter for the path to the current file
    fn current_file(&self) -> &'a Path {
        self.frames
            .iter()
            .rev()
            .filter_map(|frame| frame.file)
            .next()
            .unwrap_or(self.root_file)
    }

    /// Lookup the fully qualified version of an identifier based on the
    /// current `Context`.
    fn try_to_resolve_ident(&self, ident: &Ident) -> Option<&Ident> {
        self.frames
            .iter()
            .rev()
            .filter_map(|frame| frame.ident_lookup.get(ident))
            .next()
    }
}

#[derive(Default)]
struct ContextFrame<'a> {
    file: Option<&'a Path>,
    ident_lookup: HashMap<Ident, Ident>,
}

impl<'a> From<&'a Path> for ContextFrame<'a> {
    fn from(file: &'a Path) -> Self {
        let file = file.into();
        let ident_lookup = HashMap::new();
        Self { file, ident_lookup }
    }
}
