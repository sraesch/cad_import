use std::{collections::HashMap, rc::Rc};

use multimap::MultiMap;

use super::loader::Loader;

/// A list of loaders sorted by priority
type LoaderList = MultiMap<u32, Rc<dyn Loader>>;

/// A map of loaders
type LoaderMap = HashMap<String, LoaderList>;

pub struct Manager {
    /// Map from file extensions to a list of loaders
    map_ext: LoaderMap,

    /// Map from file mime types to a list of loaders
    map_mime: LoaderMap,
}

impl Manager {
    /// Creates and returns a new empty loader manager
    pub fn new_empty() -> Self {
        Self {
            map_ext: HashMap::new(),
            map_mime: HashMap::new(),
        }
    }

    /// Registers a new loader in the manager
    ///
    /// # Arguments
    /// * `loader` - The loader to register.
    pub fn register_loader(&mut self, loader: Box<dyn Loader>) {
        let priority = loader.get_priority();
        let extensions = loader.as_ref().get_extensions();
        let mime_types = loader.get_mime_types();

        // create reference counter of loader
        let loader: Rc<dyn Loader> = loader.into();

        // register loader based on its extension
        for ext in extensions.iter() {
            let loader_list = self
                .map_ext
                .entry(ext.clone())
                .or_insert_with(|| LoaderList::new());

            loader_list.insert(priority, loader.clone());
        }

        // register loader based on its mime type
        for mim_type in mime_types.iter() {
            let loader_list = self
                .map_mime
                .entry(mim_type.clone())
                .or_insert_with(|| LoaderList::new());

            loader_list.insert(priority, loader.clone());
        }
    }

    /// Tries to find a loader by its extension.
    ///
    /// # Arguments
    /// * `ext` - The extension of the loader without a preceding dot, e.g. "png".
    pub fn get_loader_by_extension(&self, ext: &str) -> Option<Rc<dyn Loader>> {
        let ext = ext.to_lowercase();

        match self.map_ext.get(&ext) {
            Some(lst) => lst.iter().last().map(|(_, l)| l.clone()),
            None => None,
        }
    }

    /// Tries to find a loader by its mime type.
    ///
    /// # Arguments
    /// * `mime_type` - The mime type of the loader, e.g. "image/png".
    pub fn get_loader_by_mime_type(&self, mime_type: &str) -> Option<Rc<dyn Loader>> {
        let mime_type = mime_type.to_lowercase();

        match self.map_mime.get(&mime_type) {
            Some(lst) => lst.iter().last().map(|(_, l)| l.clone()),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FakeLoader {
        identifier: String,
        extensions: Vec<String>,
        mime_types: Vec<String>,
        priority: u32,
    }

    impl FakeLoader {
        pub fn new(
            identifier: String,
            extensions: Vec<String>,
            mime_types: Vec<String>,
            priority: u32,
        ) -> Self {
            Self {
                identifier,
                extensions,
                mime_types,
                priority,
            }
        }
    }

    impl Loader for FakeLoader {
        fn get_mime_types(&self) -> Vec<String> {
            self.mime_types.clone()
        }

        fn get_extensions(&self) -> Vec<String> {
            self.extensions.clone()
        }

        fn get_priority(&self) -> u32 {
            self.priority
        }

        fn read_file(
            &self,
            _: &mut dyn std::io::Read,
        ) -> Result<crate::structure::cad_data::CADData, crate::error::Error> {
            todo!()
        }

        fn get_name(&self) -> &str {
            &self.identifier
        }
    }

    #[test]
    fn test_loader_registry() {
        let mut m = Manager::new_empty();

        let l = FakeLoader::new(
            "loader1".to_owned(),
            vec!["foobar".to_owned()],
            vec!["foobar/x-test".to_owned()],
            42,
        );
        m.register_loader(Box::new(l));

        assert!(m.get_loader_by_extension("foobar").is_some());
        assert!(m.get_loader_by_extension("FOobar").is_some());
        assert!(m.get_loader_by_extension("FOobar2").is_none());
        assert!(m.get_loader_by_extension("FOob").is_none());

        let l2 = FakeLoader::new(
            "loader2".to_owned(),
            vec!["foobar".to_owned()],
            vec!["foobar/x-test".to_owned()],
            43,
        );

        m.register_loader(Box::new(l2));

        assert!(m.get_loader_by_extension("foobar").is_some());
        assert!(m.get_loader_by_extension("FOobar").is_some());
        assert!(m.get_loader_by_extension("FOobar2").is_none());
        assert!(m.get_loader_by_extension("FOob").is_none());
        assert_eq!(
            m.get_loader_by_extension("foobar").unwrap().get_priority(),
            43
        );
        assert_eq!(
            m.get_loader_by_extension("foobar").unwrap().get_name(),
            "loader2"
        );
    }
}