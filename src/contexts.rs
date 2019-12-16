use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use syn::Ident;
use syn::ItemMod;

pub struct Context<'a> {
    project_directory: &'a Path,
    current_path: PathBuf,
    base_scope: ContextScope,
    scopes: Vec<ContextScope>,
}

impl<'a> Context<'a> {
    /// Creates a new scope
    pub fn step_into(&mut self, module_item: &ItemMod) {
        // Determine the type of the module
        let module_type = if module_item.content.is_some() {
            ModuleType::InlineModule
        } else {
            self.move_to_submodule(&module_item.ident.to_string());
            ModuleType::ReferencedModule
        };

        let scope = ContextScope::from(module_type);
        self.scopes.push(scope);
    }

    /// Moves the PathBuf to the specified sub module
    fn move_to_submodule(&mut self, module_name: &str) {
        let path = &mut self.current_path;
        if let Some(curr_mod_name) = path.file_stem().map(|os_str| os_str.to_owned()) {
            path.pop();
            path.push(curr_mod_name);
        }

        path.push(module_name);
        path.set_extension("rs");
    }

    fn move_to_super_module(&mut self) {
        self.current_path.pop();
        self.current_path.set_extension("rs");
    }

    /// Steps out of the current scope to the previous scope
    pub fn step_out(&mut self) {
        self.scopes.pop();
        self.move_to_super_module();
    }

    /// Registers an identifier alias within the current scope
    pub fn register_identifier(&mut self, short_ident: Ident, full_ident: Ident) -> Option<Ident> {
        let scope = self.scopes.last_mut().unwrap_or(&mut self.base_scope);

        scope.ident_lookup.insert(short_ident, full_ident)
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

impl<'a> From<&'a Path> for Context<'a> {
    fn from(project_directory: &'a Path) -> Self {
        let current_path = PathBuf::new();
        let base_scope = ContextScope::base_scope();
        let scopes = Vec::new();
        Self {
            project_directory,
            current_path,
            base_scope,
            scopes,
        }
    }
}

#[cfg(test)]
mod context_tests {

    use super::*;

    #[test]
    fn move_to_submodule_of_root_test() {
        let project_directory = PathBuf::new();
        let mut context = Context::from(project_directory.as_path());
        context.move_to_submodule("foo");

        let expected_path: PathBuf = ["foo.rs"].iter().collect();
        assert_eq!(context.current_path, expected_path);
    }

    #[test]
    fn move_to_submodule_of_submodule_test() {
        let project_directory = PathBuf::new();
        let mut context = Context::from(project_directory.as_path());
        context.move_to_submodule("foo");
        context.move_to_submodule("bar");
        let expected_path: PathBuf = ["foo", "bar.rs"].iter().collect();
        assert_eq!(context.current_path, expected_path);
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

impl ContextScope {
    fn base_scope() -> Self {
        Self {
            module_type: ModuleType::RootModule,
            ident_lookup: HashMap::new(),
        }
    }
}

impl From<ModuleType> for ContextScope {
    fn from(module_type: ModuleType) -> Self {
        ContextScope {
            module_type,
            ident_lookup: HashMap::new(),
        }
    }
}
