use crate::data_structure::Document;
use crate::io::{Exporter, Error};

pub struct DefaultExporter;

impl DefaultExporter {
    pub fn new() -> Self {
        Self {}
    }
}

impl Exporter for DefaultExporter {
    fn can_export(&self, extension: &str) -> bool {
        false
    }

    fn export_to_file(&self, doc: &Document, filename: &str) -> Result<(), Error> {
        Err(Error::ExportError("Not implemented".to_string()))
    }

    fn export_to_bytes(&self, doc: &Document) -> Result<Vec<u8>, Error> {
        Err(Error::ExportError("Not implemented".to_string()))
    }
}
