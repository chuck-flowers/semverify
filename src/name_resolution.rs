use std::collections::HashMap;
use std::path::PathBuf;
use syn::Ident;
use syn::ItemMod;

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
        Context {
            current_path,
            base_scope,
            scopes,
        }
    }
}

impl Context {
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
    /// # Examples
    ///
    /// ## Submodule to Root
    ///
    /// ```
    /// let path = std::path::PathBuf::new();
    /// move_to_submodule(&mut path, "foo");
    /// assert_eq!(path, &["foo.rs"].collect());
    /// ```
    ///
    /// ## Submodule to Submodule
    ///
    /// ```
    /// let path: std::path::PathBuf = ["foo.rs"].collect();
    /// move_to_submodule(&mut path, "bar");
    /// assert_eq!(path, &["foo", "bar.rs"].collect());
    /// ```
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
