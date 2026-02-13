use crate::data_structure::Document;
use crate::io::{Importer, Error};

pub struct DefaultImporter;

impl DefaultImporter {
    pub fn new() -> Self {
        Self {}
    }
}

impl Importer for DefaultImporter {
    fn can_import(&self, extension: &str) -> bool {
        false
    }

    fn import_from_file(&self, filename: &str) -> Result<Document, Error> {
        Err(Error::UnsupportedFormat("Not implemented".to_string()))
    }

    fn import_from_bytes(&self, data: &[u8], extension: &str) -> Result<Document, Error> {
        Err(Error::UnsupportedFormat("Not implemented".to_string()))
    }
}
