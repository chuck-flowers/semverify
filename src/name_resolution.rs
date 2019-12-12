use std::collections::HashMap;
use std::path::Path;
use syn::File;
use syn::Ident;
use syn::Item;

/// Rewrite the idents of a `syn::File` (and all referenced files) to be fully qualified
#[allow(dead_code)]
pub fn resolve_file_identifiers(file: &mut File, path: &Path) {
    let mut context = Context::default();
    resolve_file_identifiers_in_context(file, path, &mut context)
}

/// Rewrite the idents of a `syn::File` (and all referenced files) to be fully
/// qualified using the provided `Context`.
fn resolve_file_identifiers_in_context(file: &mut File, path: &Path, context: &mut Context) {
    context.step_into();
    resolve_item_identifiers_in_context(file.items.iter_mut(), path, context);
    context.step_out();
}

/// Rewrites all identifiers within an `Iterator` of mutable item references to
/// be fully qualified within a provided Context.
fn resolve_item_identifiers_in_context<'a>(
    items: impl Iterator<Item = &'a mut Item>,
    path: &Path,
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

#[derive(Default)]
struct Context {
    frames: Vec<ContextFrame>,
}

impl Context {
    /// Creates and steps into a new scope
    fn step_into(&mut self) {
        let frame = ContextFrame::default();
        self.frames.push(frame);
    }

    /// Steps out of the current scope to the previous scope
    fn step_out(&mut self) {
        self.frames.pop();
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
struct ContextFrame {
    ident_lookup: HashMap<Ident, Ident>,
}
