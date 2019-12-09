/// An enum that specifies the type of code change that was made (e.g. major, minor, patch).
#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum ChangeType {
    Patch,
    Minor,
    Major,
}

/// A trait that should be implemented by structs representing source code changes.
pub trait Delta {
    /// Determines the ChangeType that occurs as a result of this `Delta`.
    fn change_type(&self) -> ChangeType;
}

/// A trait that should be implemented by structs representing source code.
/// Provides an interface for determining the difference between two instances
/// of the same `Diffable`.
pub trait Diffable {
    /// The type that is produced by a diff describing the changes that were
    /// made.
    type Delta: Delta;

    /// Compares this `Diffable` to another of the same type to determine the
    /// change(s) that were made.
    fn diff(&self, new_type: &Self) -> Vec<Self::Delta>;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn change_type_order_tests() {
        let mut types = vec![ChangeType::Minor, ChangeType::Major, ChangeType::Patch];
        types.sort();

        assert_eq!(
            types,
            &[ChangeType::Patch, ChangeType::Minor, ChangeType::Major]
        );
    }
}
