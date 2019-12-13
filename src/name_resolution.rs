use std::collections::HashMap;
use syn::Ident;

pub struct Context {
    base_scope: ContextScope,
    scopes: Vec<ContextScope>,
}

impl Default for Context {
    fn default() -> Self {
        let base_scope = ContextScope::from(ModuleType::InlineModule);
        let scopes = Vec::default();
        Context { base_scope, scopes }
    }
}

impl Context {

    /// Creates a new scope
    pub fn step_into(&mut self) {
        unimplemented!()
    }

    /// Steps out of the current scope to the previous scope
    pub fn step_out(&mut self) {
        unimplemented!()
    }

    /// Registers an identifier alias within the current scope
    pub fn register_identifier(&mut self, short_ident: Ident, full_ident: Ident) {
        unimplemented!()
    }

    /// Lookup the fully qualified version of an identifier based on the
    /// current `Context`.
    pub fn try_to_resolve_ident(&self, ident: &Ident) -> Option<&Ident> {
        unimplemented!()
    }
}

/// Represents the type of a module (whether or not its defined 
/// in the same file).
enum ModuleType {
    InlineModule,
    ReferencedModule
}

/// Represents an isolated scope within Rust source code.
struct ContextScope {
    module_type: ModuleType,
    ident_lookup: HashMap<Ident, Ident>,
}

impl From<ModuleType> for ContextScope {
    fn from(module_type: ModuleType) -> Self { ContextScope { module_type, ident_lookup: HashMap::new() }}
}
