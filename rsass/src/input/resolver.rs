use super::LoadError;

pub struct ResolutionResult<TFile: std::io::Read> {
    path: String,
    content: TFile,
}

pub trait Resolver: std::fmt::Debug {
    type File: std::io::Read;

    /// Resolves the imported string used in an @use or @import
    /// statement to a the contents of the loaded file.
    fn resolve(
        self,
        importer_path: &str,
        url: &str,
    ) -> Result<Option<ResolutionResult<Self::File>>, LoadError>;
}
