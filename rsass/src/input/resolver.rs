use super::{LoadError, SourceKind};

/// Result of a resolution in a [`Resolver`]
pub struct ResolutionResult<TFile: std::io::Read> {
    /// Path of the resolved file on the file system
    pub path: String,
    /// Content of the resolved file
    pub file: TFile,
}

/// Trait responsible for resolving imported URLs into the loaded
/// source files.
pub trait Resolver: std::fmt::Debug {
    /// Type of file resolved by this resolver.
    type File: std::io::Read;

    /// Resolves the imported string used in an @use or @import
    /// statement to a the contents of the loaded file.
    fn resolve(
        &self,
        url: &str,
        from: &SourceKind,
    ) -> Result<Option<ResolutionResult<Self::File>>, LoadError>;
}
