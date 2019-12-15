use std::path::PathBuf;
use std::collections::HashMap;
use syn::Ident;

pub struct Context {
    current_path: PathBuf,
    base_scope: ContextScope,
    scopes: Vec<ContextScope>,
}

impl<T: Into<PathBuf>> From<T> for Context {
    fn from(current_path: T) -> Self {
        let current_path = current_path.into();
        let base_scope = ContextScope::from(ModuleType::RootModule);
        let scopes = Vec::default();
        Context { current_path, base_scope, scopes }
    }
}

impl Context {
    /// Creates a new scope
    pub fn step_into(&mut self) {
        unimplemented!()
    }

    /// Steps out of the current scope to the previous scope
    pub fn step_out(&mut self) {
        self.scopes.pop();
    }

    /// Registers an identifier alias within the current scope
    pub fn register_identifier(&mut self, short_ident: Ident, full_ident: Ident) -> Option<Ident> {
        self.scopes
            .last_mut()
            .unwrap_or(&mut self.base_scope)
            .ident_lookup
            .insert(short_ident, full_ident)
    }

    /// Lookup the fully qualified version of an identifier based on the
    /// current `Context`.
    pub fn try_to_resolve_ident(&self, ident: &Ident) -> Option<&Ident> {
        self.scopes
            .iter()
            .rev()
            .filter_map(|scope| scope.ident_lookup.get(ident))
            .next()
            .or(self.base_scope.ident_lookup.get(ident))
    }
}

/// Represents the type of a module (whether or not its defined
/// in the same file).
enum ModuleType {
    RootModule,
    InlineModule,
    ReferencedModule,
}

/// Represents an isolated scope within Rust source code.
struct ContextScope {
    module_type: ModuleType,
    ident_lookup: HashMap<Ident, Ident>,
}

impl From<ModuleType> for ContextScope {
    fn from(module_type: ModuleType) -> Self {
        ContextScope {
            module_type,
            ident_lookup: HashMap::new(),
        }
    }
}
