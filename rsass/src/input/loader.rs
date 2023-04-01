use std::{borrow::Cow, fmt};
use tracing::instrument;
use crate::{Error};
use super::{
    SourceFile, SourceKind,
    resolver::Resolver
};

type Combine = &'static dyn Fn(&str, &str) -> String;

/// A file context manages finding and loading files.
///
/// # Example
/// ```
/// use std::collections::HashMap;
/// use rsass::input::{Loader, LoadError};
///
/// #[derive(Clone, Debug)]
/// struct MemoryLoader<'a> {
///     files: HashMap<String, &'a[u8]>,
/// }
///
/// impl<'a> Loader for MemoryLoader<'a> {
///     type File = &'a [u8];
///
///     fn find_file(&self, name: &str) -> Result<Option<Self::File>, LoadError> {
///         Ok(self.files.get(name).map(|data| *data))
///     }
/// }
/// ```
pub trait Loader: std::fmt::Debug {
    /// Anything that can be read can be a File in an implementation.
    type File: std::io::Read;

    /// Find a file.
    ///
    /// If a file named `base/input.scss` uses a file named `module`, the
    /// name is converted to `base/module.scss` and variants by
    /// [`Context::find_file`][crate::input::Context::find_file], and
    /// this method is called for each variant to check if it exists.
    ///
    /// Note that if a file with the given name does not exist, that is not
    /// an error.
    /// In that case, `find_file` is expected to return `Ok(None)`.
    /// Things like illegal file names (for the given backend) or lacking
    /// permissions, are handled as errors.
    ///
    /// The official Sass specification prescribes that files are loaded by
    /// url instead of by path to ensure universal compatibility of style sheets.
    /// This effectively mandates the use of forward slashes on all platforms.
    fn find_file(&self, url: &str) -> Result<Option<Self::File>, LoadError>;
}

/// An error loading a file.
#[non_exhaustive]
pub enum LoadError {
    /// Reading {0} failed: {1}
    Input(String, std::io::Error),
    /// {0} is not a css or sass file.
    UnknownFormat(String),
    /// Expected a cargo environment, but none found.
    NotCalledFromCargo,
}
impl std::error::Error for LoadError {}

impl fmt::Display for LoadError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        write!(out, "Error: {:?}", self)
    }
}

impl fmt::Debug for LoadError {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoadError::Input(path, err) => {
                write!(out, "Reading {:?} failed: {}", path, err)
            }
            LoadError::UnknownFormat(name) => {
                write!(out, "{:?} is not a css or sass file.", name)
            }
            LoadError::NotCalledFromCargo => {
                write!(out, "Expected a cargo environment, but none found.")
            }
        }
    }
}

#[derive(Debug)]
pub struct LoaderResolver<AnyFile: std::fmt::Debug, AnyLoader: Loader<File = AnyFile>> {
    loader: AnyLoader,
}

impl<AnyFile: std::fmt::Debug, AnyLoader: Loader<File = AnyFile>> LoaderResolver<AnyFile, AnyLoader> {
    pub fn new(
        loader: AnyLoader,
    ) -> Self {
        Self { loader }
    }

    /// Find a file.
    ///
    /// This method handles sass file name resolution, but delegates
    /// the actual checking for existing files to the [`Loader`].
    ///
    /// Given a url like `my/util`, this method will check for
    /// `my/util`, `my/util.scss`, `my/_util.scss`,
    /// `my/util/index.scss`, and `my/util/_index.scss`.
    /// The variants that are not a directory index will also be
    /// checked for `.css` files (and in the future it may also check
    /// for `.sass` files if rsass suports that format).
    ///
    /// If `from` indicates that the loading is for an `@import` rule,
    /// some [extra file names][import-only] are checked.
    ///
    /// The `Context` keeps track of "locked" files (files currently beeing
    /// parsed or transformed into css).
    /// The source file returned from this function is locked, so the
    /// caller of this method need to call [`Self::unlock_loading`] after
    /// handling it.
    ///
    /// [import-only]: https://sass-lang.com/documentation/at-rules/import#import-only-files
    #[instrument]
    pub fn find_file(
        &mut self,
        url: &str,
        from: SourceKind,
    ) -> Result<Option<SourceFile>, Error> {
        let names: &[Combine] = if from.is_import() {
            &[
                // base will either be empty or end with a slash.
                &|base, name| format!("{}{}.import.scss", base, name),
                &|base, name| format!("{}_{}.import.scss", base, name),
                &|base, name| format!("{}{}.scss", base, name),
                &|base, name| format!("{}_{}.scss", base, name),
                &|base, name| format!("{}{}/index.import.scss", base, name),
                &|base, name| format!("{}{}/_index.import.scss", base, name),
                &|base, name| format!("{}{}/index.scss", base, name),
                &|base, name| format!("{}{}/_index.scss", base, name),
                &|base, name| format!("{}{}.css", base, name),
                &|base, name| format!("{}_{}.css", base, name),
            ]
        } else {
            &[
                // base will either be empty or end with a slash.
                &|base, name| format!("{}{}.scss", base, name),
                &|base, name| format!("{}_{}.scss", base, name),
                &|base, name| format!("{}{}/index.scss", base, name),
                &|base, name| format!("{}{}/_index.scss", base, name),
                &|base, name| format!("{}{}.css", base, name),
                &|base, name| format!("{}_{}.css", base, name),
            ]
        };
        // Note: Should a "full stack" of bases be used here?
        // Or is this fine?
        let url = relative(&from, url);
        if let Some((path, mut file)) = self.do_find_file(&url, names)? {
            let is_module = !from.is_import();
            let source = from.url(&path);
            let file = SourceFile::read(&mut file, source)?;
            Ok(Some(file))
        } else {
            Ok(None)
        }
    }

    /// Find a file in a given filecontext matching a url over a set of
    /// name rules.
    fn do_find_file(
        &self,
        url: &str,
        names: &[Combine],
    ) -> Result<Option<(String, AnyFile)>, LoadError> {
        if url.ends_with(".css")
            || url.ends_with(".sass")
            || url.ends_with(".scss")
        {
            self.loader
                .find_file(url)
                .map(|file| file.map(|file| (url.into(), file)))
        } else {
            let (base, name) = url
                .rfind('/')
                .map(|p| url.split_at(p + 1))
                .unwrap_or(("", url));

            for name in names.iter().map(|f| f(base, name)) {
                if let Some(result) = self.loader.find_file(&name)? {
                    return Ok(Some((name, result)));
                }
            }
            Ok(None)
        }
    } 
}

/// Make a url relative to a given base.
fn relative<'a>(base: &SourceKind, url: &'a str) -> Cow<'a, str> {
    base.next()
        .map(|pos| pos.file_url())
        .and_then(|base| {
            base.rfind('/')
                .map(|p| base.split_at(p + 1).0)
                .map(|base| format!("{}{}", base, url).into())
        })
        .unwrap_or_else(|| url.into())
}

/// A basic Resolver that uses fallback logic to resolve a single Loader.
impl<AnyFile: std::fmt::Debug, AnyLoader: Loader<File = AnyFile>> Resolver for LoaderResolver<AnyFile, AnyLoader> {
    type File = AnyFile;

    fn resolve(&self, from_path: &str, url: &str) -> Result<Option<Self::File>, LoadError> {
        self.find_file(from_path, url)
    }
}