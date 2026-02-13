use super::{Importer, FormatInfo};
use super::io::SUPPORTED_FORMATS;
use super::dxf::DXFImporter;
use super::svg::SVGImporter;
use super::dwg::DWGImporter;
use super::iges::IGESImporter;
use super::step::STEPImporter;
use crate::data_structure::Document;
use std::path::Path;

pub struct ImporterRegistry {
    importers: Vec<Box<dyn Importer>>,
}

impl ImporterRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            importers: Vec::new(),
        };
        registry.register_all();
        registry
    }

    fn register_all(&mut self) {
        self.importers.push(Box::new(DXFImporter::new()));
        self.importers.push(Box::new(SVGImporter::new()));
        self.importers.push(Box::new(DWGImporter::new()));
        self.importers.push(Box::new(IGESImporter::new()));
        self.importers.push(Box::new(STEPImporter::new()));
    }

    pub fn get_importer(&self, extension: &str) -> Option<&dyn Importer> {
        for importer in &self.importers {
            if importer.can_import(extension) {
                return Some(importer.as_ref());
            }
        }
        None
    }

    pub fn import_from_file(&self, filename: &str) -> Result<Document, String> {
        let path = Path::new(filename);
        let extension = path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_lowercase())
            .ok_or("无法获取文件扩展名")?;

        if let Some(importer) = self.get_importer(&extension) {
            importer.import_from_file(filename)
                .map_err(|e| e.to_string())
        } else {
            Err(format!("不支持的文件格式: {}", extension))
        }
    }

    pub fn import_from_bytes(&self, data: &[u8], extension: &str) -> Result<Document, String> {
        let ext = extension.to_lowercase();
        if let Some(importer) = self.get_importer(&ext) {
            importer.import_from_bytes(data, &ext)
                .map_err(|e| e.to_string())
        } else {
            Err(format!("不支持的文件格式: {}", ext))
        }
    }

    pub fn supported_formats(&self) -> Vec<&FormatInfo> {
        SUPPORTED_FORMATS.iter().collect()
    }
}

impl Default for ImporterRegistry {
    fn default() -> Self {
        Self::new()
    }
}

pub fn get_importer(extension: &str) -> Option<Box<dyn Importer>> {
    match extension.to_lowercase().as_str() {
        "dxf" => Some(Box::new(DXFImporter::new())),
        "svg" => Some(Box::new(SVGImporter::new())),
        "dwg" => Some(Box::new(DWGImporter::new())),
        "iges" | "igs" => Some(Box::new(IGESImporter::new())),
        "step" | "stp" => Some(Box::new(STEPImporter::new())),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_importer_registry_formats() {
        let registry = ImporterRegistry::new();
        let formats = registry.supported_formats();
        assert!(formats.len() > 0);
    }

    #[test]
    fn test_get_importer() {
        assert!(get_importer("dxf").is_some());
        assert!(get_importer("svg").is_some());
        assert!(get_importer("dwg").is_some());
        assert!(get_importer("iges").is_some());
        assert!(get_importer("step").is_some());
        assert!(get_importer("unknown").is_none());
    }
}
