use std::collections::BTreeMap;
use std::fmt;
use std::path::Path;

use super::{
    CargoLoader, CargoLoaderFile, FsLoader, FsLoaderFile, LoadError, Loader,
    LoaderResolver, ResolutionResult, Resolver, SourceFile, SourceKind,
};
use crate::output::{handle_parsed, CssData, Format};
use crate::{Error, ScopeRef};

/// Utility keeping track of loading files.
///
/// The context is generic over the [`Loader`].
/// [`FsContext`] and [`CargoContext`] are type aliases for `Context`
/// where the loader is a [`FsLoader`] or [`CargoLoader`],
/// respectively.
///
/// # Examples
///
/// The Context here is a [`FsContext`].
/// Input is usually a scss file.
///
/// ```
/// # use rsass::input::{FsContext, SourceFile, SourceName};
/// # use rsass::output::{Format, Style};
/// # fn main() -> Result<(), rsass::Error> {
/// let context = FsContext::for_cwd()
///     .with_format(Format { style: Style::Compressed, precision: 2 });
/// let scss_input = SourceFile::scss_bytes(
///     "$gap: 4em / 3;
///     \np {\
///     \n    margin: $gap 0;
///     \n}\n",
///     SourceName::root("-")
/// );
/// assert_eq!(
///     context.transform(scss_input)?,
///     b"p{margin:1.33em 0}\n"
/// );
/// # Ok(()) }
/// ```
///
/// This method can also be used as a plain css compression.
/// ```
/// # use rsass::input::{FsContext, SourceFile, SourceName};
/// # use rsass::output::{Format, Style};
/// # fn main() -> Result<(), rsass::Error> {
/// # let context = FsContext::for_cwd().with_format(Format { style: Style::Compressed, precision: 2 });
/// let css_input = SourceFile::css_bytes(
///     "p {\
///     \n    margin: 1.333333333em 0;\
///     \n}\n",
///     SourceName::root("-")
/// );
/// assert_eq!(
///     context.transform(css_input)?,
///     b"p{margin:1.33em 0}\n"
/// );
/// # Ok(()) }
/// ```
pub struct Context<AnyResolver: Resolver> {
    resolver: AnyResolver,
    scope: Option<ScopeRef>,
    loading: BTreeMap<String, SourceKind>,
    // TODO: Maybe have a map to loaded SourceFiles as well?  Or even Parsed?
}

/// A file-system based [`Context`].
pub type FsContext = Context<LoaderResolver<FsLoaderFile, FsLoader>>;

impl FsContext {
    /// Create a new `Context`, loading files based on the current
    /// working directory.
    pub fn for_cwd() -> Self {
        Context::for_loader(FsLoader::for_cwd())
    }

    /// Create a new `Context` and load a file.
    ///
    /// The directory part of `path` is used as a base directory for the loader.
    pub fn for_path(path: &Path) -> Result<(Self, SourceFile), LoadError> {
        let (file_loader, file) = FsLoader::for_path(path)?;
        Ok((
            Context::for_resolver(LoaderResolver::new(file_loader)),
            file,
        ))
    }

    /// Add a path to search for files.
    pub fn push_path(&mut self, path: &Path) {
        self.resolver.loader.push_path(path);
    }
}

/// A file-system based [`Context`] for use in cargo build scripts.
///
/// This is very similar to a [`FsContext`], but has a
/// `for_crate` constructor that uses the `CARGO_MANIFEST_DIR`
/// environment variable instead of the current working directory, and
/// it prints `cargo:rerun-if-changed` messages for each path that it
/// loads.
pub type CargoContext = Context<LoaderResolver<CargoLoaderFile, CargoLoader>>;

impl CargoContext {
    /// Create a new `Context`, loading files based in the manifest
    /// directory of the current crate.
    ///
    /// Relative paths will be resolved from the directory containing the
    /// manifest of your package.
    /// This assumes the program is called by `cargo` as a build script, so
    /// the `CARGO_MANIFEST_DIR` environment variable is set.
    pub fn for_crate() -> Result<Self, LoadError> {
        Ok(Context::for_resolver(LoaderResolver::new(
            CargoLoader::for_crate()?,
        )))
    }

    /// Create a new `Context` and load a file.
    ///
    /// The directory part of `path` is used as a base directory for the loader.
    /// If `path` is relative, it will be resolved from the directory
    /// containing the manifest of your package.
    pub fn for_path(path: &Path) -> Result<(Self, SourceFile), LoadError> {
        let (file_context, file) = CargoLoader::for_path(path)?;
        Ok((
            Context::for_resolver(LoaderResolver::new(file_context)),
            file,
        ))
    }

    /// Add a path to search for files.
    ///
    /// If `path` is relative, it will be resolved from the directory
    /// containing the manifest of your package.
    pub fn push_path(&mut self, path: &Path) -> Result<(), LoadError> {
        self.resolver.loader.push_path(path)
    }
}

impl<
        AnyFile: std::fmt::Debug + std::io::Read,
        AnyLoader: Loader<File = AnyFile>,
    > Context<LoaderResolver<AnyFile, AnyLoader>>
{
    /// Create a new `Context` for a given file [`Loader`].
    ///
    /// Internally, this is identical to [`for_loader`][Context::for_resolver],
    /// using the default implementation of Resolver for Loader, and only
    /// exists for backwards compatibility
    pub fn for_loader(loader: AnyLoader) -> Self {
        Context::for_resolver(
            <LoaderResolver<AnyLoader::File, AnyLoader>>::new(loader),
        )
    }
}

impl<AnyResolver: Resolver> Context<AnyResolver> {
    /// Create a new `Context` for a given [`Resolver`].
    pub fn for_resolver(resolver: AnyResolver) -> Self {
        Context {
            resolver: resolver,
            scope: None,
            loading: Default::default(),
        }
    }

    /// Transform some input source to css.
    ///
    /// The css output is returned as a raw byte vector.
    pub fn transform(mut self, file: SourceFile) -> Result<Vec<u8>, Error> {
        let scope = self
            .scope
            .clone()
            .unwrap_or_else(|| ScopeRef::new_global(Default::default()));
        self.lock_loading(&file, false)?;
        let mut css = CssData::new();
        let format = scope.get_format();
        handle_parsed(file.parse()?, &mut css, scope, &mut self)?;
        self.unlock_loading(&file);
        css.into_buffer(format)
    }

    /// Set the output format for this context.
    ///
    /// Note that this resets the scope.  If you use both `with_format` and
    /// [`get_scope`][Self::get_scope], you need to call `with_format`
    /// _before_ `get_scope`.
    pub fn with_format(mut self, format: Format) -> Self {
        self.scope = Some(ScopeRef::new_global(format));
        self
    }

    /// Get the scope for this context.
    ///
    /// A ScopeRef dereferences to a [`crate::Scope`], which uses internal
    /// mutability.
    /// So this can be used for predefining variables, functions, mixins,
    /// or modules before transforming some scss input.
    ///
    /// Note that if you use both [`with_format`][Self::with_format] and
    /// `get_scope`, you need to call `with_format` _before_ `get_scope`.
    pub fn get_scope(&mut self) -> ScopeRef {
        self.scope
            .get_or_insert_with(|| ScopeRef::new_global(Default::default()))
            .clone()
    }

    /// Find a file.
    ///
    /// This method handles sass file name resolution, but delegates
    /// the actual url -> path resolution to the [`Resolver`].
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
    pub fn find_file(
        &mut self,
        url: &str,
        from: SourceKind,
    ) -> Result<Option<SourceFile>, Error> {
        // Note: Should a "full stack" of bases be used here?
        // Or is this fine?
        if let Some(ResolutionResult { path, mut file }) =
            self.resolver.resolve(url, &from)?
        {
            let is_module = !from.is_import();
            let source = from.url(&path);
            let file = SourceFile::read(&mut file, source)?;
            self.lock_loading(&file, is_module)?;
            Ok(Some(file))
        } else {
            Ok(None)
        }
    }

    pub(crate) fn lock_loading(
        &mut self,
        file: &SourceFile,
        as_module: bool,
    ) -> Result<(), Error> {
        let name = file.source().name();
        let pos = &file.source().imported;
        if let Some(old) = self.loading.insert(name.into(), pos.clone()) {
            Err(Error::ImportLoop(
                as_module,
                pos.next().unwrap().clone(),
                old.next().cloned(),
            ))
        } else {
            Ok(())
        }
    }

    /// Unlock a file that is locked for input processing.
    ///
    /// The lock exists to break circular dependency chains.
    /// Each file that is locked (by [`Self::find_file`]) needs to be unlocked
    /// when processing of it is done.
    pub fn unlock_loading(&mut self, file: &SourceFile) {
        self.loading.remove(file.path());
    }
}

impl<T: fmt::Debug + Resolver> fmt::Debug for Context<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Context")
            .field("resolver", &self.resolver)
            .field(
                "scope",
                &if self.scope.is_some() { "loaded" } else { "no" },
            )
            .field("locked", &self.loading.keys())
            .finish()
    }
}
