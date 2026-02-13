use thiserror::Error;
use crate::data_structure::Document;
use std::sync::LazyLock;
use std::collections::HashMap;
use std::path::PathBuf;
use std::any::Any;
use std::fmt;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(String),
    
    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),
    
    #[error("Parse error: {0}")]
    ParseError(String),
    
    #[error("Export error: {0}")]
    ExportError(String),
    
    #[error("Registration error: {0}")]
    RegistrationError(String),
    
    #[error("Conversion error: {0}")]
    ConversionError(String),
}

pub trait Importer: Send + Sync {
    fn can_import(&self, extension: &str) -> bool;
    fn import_from_file(&self, filename: &str) -> Result<Document, Error>;
    fn import_from_bytes(&self, data: &[u8], extension: &str) -> Result<Document, Error>;
    
    fn priority(&self) -> u32 { 100 }
}

pub trait Exporter: Send + Sync {
    fn can_export(&self, extension: &str) -> bool;
    fn export_to_file(&self, doc: &Document, filename: &str) -> Result<(), Error>;
    fn export_to_bytes(&self, doc: &Document) -> Result<Vec<u8>, Error>;
    
    fn priority(&self) -> u32 { 100 }
}

#[derive(Debug, Clone)]
pub struct FormatInfo {
    pub extension: String,
    pub name: String,
    pub description: String,
    pub is_binary: bool,
    pub mime_type: String,
    pub versions: Vec<String>,
}

impl FormatInfo {
    pub fn new(extension: &str, name: &str, description: &str, is_binary: bool) -> Self {
        let mime = match extension.to_lowercase().as_str() {
            "dxf" => "application/dxf",
            "svg" => "image/svg+xml",
            "json" => "application/json",
            "dwg" => "application/dwg",
            "iges" | "igs" => "applicationiges",
            "/step" | "stp" => "application/step",
            "obj" => "model/obj",
            "fbx" => "application/fbx",
            _ => "application/octet-stream",
        };
        
        Self {
            extension: extension.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            is_binary,
            mime_type: mime.to_string(),
            versions: Vec::new(),
        }
    }
    
    pub fn with_version(mut self, version: &str) -> Self {
        self.versions.push(version.to_string());
        self
    }
}

pub static SUPPORTED_FORMATS: LazyLock<Vec<FormatInfo>> = LazyLock::new(|| {
    vec![
        FormatInfo::new("dxf", "DXF", "AutoCAD Drawing Exchange Format", false)
            .with_version("R12").with_version("R14").with_version("2000").with_version("2018"),
        FormatInfo::new("svg", "SVG", "Scalable Vector Graphics", false)
            .with_version("1.1").with_version("2.0"),
        FormatInfo::new("json", "JSON", "JavaScript Object Notation", false),
        FormatInfo::new("dwg", "DWG", "AutoCAD Drawing Database Binary Format", true)
            .with_version("R12").with_version("R14").with_version("2000").with_version("2018"),
        FormatInfo::new("iges", "IGES", "Initial Graphics Exchange Specification", false)
            .with_version("5.3").with_version("6.0"),
        FormatInfo::new("igs", "IGES", "Initial Graphics Exchange Specification", false)
            .with_version("5.3").with_version("6.0"),
        FormatInfo::new("step", "STEP", "Standard for the Exchange of Product model data", false)
            .with_version("AP203").with_version("AP214").with_version("AP242"),
        FormatInfo::new("stp", "STEP", "Standard for the Exchange of Product model data", false)
            .with_version("AP203").with_version("AP214").with_version("AP242"),
        FormatInfo::new("obj", "OBJ", "Wavefront OBJ Geometry File", false),
        FormatInfo::new("fbx", "FBX", "Autodesk FBX Interchange File", true)
            .with_version("7.0").with_version("7.4").with_version("7.5"),
    ]
});

pub struct FormatRegistry {
    importers: HashMap<String, Box<dyn Importer>>,
    exporters: HashMap<String, Box<dyn Exporter>>,
    format_info: HashMap<String, FormatInfo>,
}

impl Default for FormatRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatRegistry {
    pub fn new() -> Self {
        Self {
            importers: HashMap::new(),
            exporters: HashMap::new(),
            format_info: HashMap::new(),
        }
    }
    
    pub fn register_importer<I: Importer + 'static>(&mut self, importer: I) -> Result<(), Error> {
        let ext = self.extract_extension(&importer);
        if self.importers.contains_key(&ext) {
            return Err(Error::RegistrationError(
                format!("Importer for '{}' already registered", ext)
            ));
        }
        
        self.importers.insert(ext.clone(), Box::new(importer));
        self.format_info.insert(ext.clone(), FormatInfo::new(
            &ext, &ext.to_uppercase(), 
            &format!("{} format", ext.to_uppercase()), 
            false
        ));
        
        Ok(())
    }
    
    pub fn register_exporter<E: Exporter + 'static>(&mut self, exporter: E) -> Result<(), Error> {
        let ext = self.extract_extension(&exporter);
        if self.exporters.contains_key(&ext) {
            return Err(Error::RegistrationError(
                format!("Exporter for '{}' already registered", ext)
            ));
        }
        
        self.exporters.insert(ext.clone(), Box::new(exporter));
        if !self.format_info.contains_key(&ext) {
            self.format_info.insert(ext.clone(), FormatInfo::new(
                &ext, &ext.to_uppercase(), 
                &format!("{} format", ext.to_uppercase()), 
                false
            ));
        }
        
        Ok(())
    }
    
    fn extract_extension<T: Any>(&self, _obj: &T) -> String {
        "unknown".to_string()
    }
    
    pub fn get_importer(&self, extension: &str) -> Option<&dyn Importer> {
        let ext = extension.trim_start_matches('.').to_lowercase();
        self.importers.get(&ext).map(|b| b.as_ref())
    }
    
    pub fn get_exporter(&self, extension: &str) -> Option<&dyn Exporter> {
        let ext = extension.trim_start_matches('.').to_lowercase();
        self.exporters.get(&ext).map(|b| b.as_ref())
    }
    
    pub fn get_format_info(&self, extension: &str) -> Option<&FormatInfo> {
        let ext = extension.trim_start_matches('.').to_lowercase();
        self.format_info.get(&ext)
    }
    
    pub fn supported_import_extensions(&self) -> Vec<String> {
        let mut exts: Vec<_> = self.importers.keys().cloned().collect();
        exts.sort();
        exts
    }
    
    pub fn supported_export_extensions(&self) -> Vec<String> {
        let mut exts: Vec<_> = self.exporters.keys().cloned().collect();
        exts.sort();
        exts
    }
}

#[derive(Debug, Clone)]
pub struct ImportOptions {
    pub unit: LengthUnit,
    pub layer_mapping: LayerMappingMode,
    pub entity_filter: EntityFilter,
    pub coordinate_system: CoordinateSystem,
    pub merge_coplanar_faces: bool,
    pub tessellation_tolerance: f64,
}

impl Default for ImportOptions {
    fn default() -> Self {
        Self {
            unit: LengthUnit::Millimeter,
            layer_mapping: LayerMappingMode::Preserve,
            entity_filter: EntityFilter::all(),
            coordinate_system: CoordinateSystem::WCS,
            merge_coplanar_faces: false,
            tessellation_tolerance: 0.01,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LengthUnit {
    Millimeter,
    Centimeter,
    Meter,
    Kilometer,
    Inch,
    Foot,
    Yard,
    Mile,
}

impl Default for LengthUnit {
    fn default() -> Self {
        LengthUnit::Millimeter
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LayerMappingMode {
    Preserve,
    MergeAll,
    Ignore,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EntityFilter {
    pub include_lines: bool,
    pub include_circles: bool,
    pub include_arcs: bool,
    pub include_ellipses: bool,
    pub include_polylines: bool,
    pub include_splines: bool,
    pub include_nurbs: bool,
    pub include_surfaces: bool,
    pub include_solids: bool,
    pub include_text: bool,
    pub include_dimensions: bool,
}

impl EntityFilter {
    pub fn all() -> Self {
        Self {
            include_lines: true,
            include_circles: true,
            include_arcs: true,
            include_ellipses: true,
            include_polylines: true,
            include_splines: true,
            include_nurbs: true,
            include_surfaces: true,
            include_solids: true,
            include_text: true,
            include_dimensions: true,
        }
    }
    
    pub fn geometry_only() -> Self {
        Self {
            include_lines: true,
            include_circles: true,
            include_arcs: true,
            include_ellipses: true,
            include_polylines: true,
            include_splines: true,
            include_nurbs: true,
            include_surfaces: false,
            include_solids: false,
            include_text: false,
            include_dimensions: false,
        }
    }
    
    pub fn excludes_solids_and_text(mut self) -> Self {
        self.include_surfaces = false;
        self.include_solids = false;
        self.include_text = false;
        self.include_dimensions = false;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CoordinateSystem {
    WCS,
    UCS,
}

pub struct UnifiedDataExchange {
    registry: FormatRegistry,
    default_options: ImportOptions,
}

impl UnifiedDataExchange {
    pub fn new() -> Self {
        let mut registry = FormatRegistry::new();
        
        let mut exchange = Self {
            registry,
            default_options: ImportOptions::default(),
        };
        
        exchange
    }
    
    pub fn import_file(&self, filename: &str, options: Option<ImportOptions>) -> Result<Document, Error> {
        let options = options.unwrap_or(self.default_options.clone());
        
        let path = PathBuf::from(filename);
        let ext = path.extension()
            .and_then(|e| e.to_str())
            .ok_or_else(|| Error::UnsupportedFormat("No file extension".to_string()))?;
        
        let importer = self.registry.get_importer(ext)
            .ok_or_else(|| Error::UnsupportedFormat(format!("No importer for '{}'", ext)))?;
        
        importer.import_from_file(filename)
    }
    
    pub fn import_from_data(&self, data: &[u8], extension: &str, options: Option<ImportOptions>) -> Result<Document, Error> {
        let options = options.unwrap_or(self.default_options.clone());
        
        let importer = self.registry.get_importer(extension)
            .ok_or_else(|| Error::UnsupportedFormat(format!("No importer for '{}'", extension)))?;
        
        importer.import_from_bytes(data, extension)
    }
    
    pub fn export_file(&self, doc: &Document, filename: &str, options: Option<ExportOptions>) -> Result<(), Error> {
        let path = PathBuf::from(filename);
        let ext = path.extension()
            .and_then(|e| e.to_str())
            .ok_or_else(|| Error::UnsupportedFormat("No file extension".to_string()))?;
        
        let exporter = self.registry.get_exporter(ext)
            .ok_or_else(|| Error::UnsupportedFormat(format!("No exporter for '{}'", ext)))?;
        
        exporter.export_to_file(doc, filename)
    }
    
    pub fn export_to_data(&self, doc: &Document, extension: &str, options: Option<ExportOptions>) -> Result<Vec<u8>, Error> {
        let exporter = self.registry.get_exporter(extension)
            .ok_or_else(|| Error::UnsupportedFormat(format!("No exporter for '{}'", extension)))?;
        
        exporter.export_to_bytes(doc)
    }
    
    pub fn register_importer<I: Importer + 'static>(&mut self, importer: I) -> Result<(), Error> {
        self.registry.register_importer(importer)
    }
    
    pub fn register_exporter<E: Exporter + 'static>(&mut self, exporter: E) -> Result<(), Error> {
        self.registry.register_exporter(exporter)
    }
    
    pub fn supported_import_formats(&self) -> Vec<String> {
        self.registry.supported_import_extensions()
    }
    
    pub fn supported_export_formats(&self) -> Vec<String> {
        self.registry.supported_export_extensions()
    }
    
    pub fn detect_format(&self, data: &[u8], filename: Option<&str>) -> Option<String> {
        if data.len() >= 4 {
            let header = &data[..4];
            if header == b"AC10" || header == b"AC11" || header == b"AC12" || header == b"AC13" || header == b"AC14" || header == b"AC15" || header == b"AC16" || header == b"AC17" || header == b"AC18" {
                return Some("dwg".to_string());
            }
        }
        
        if let Some(name) = filename {
            let path = PathBuf::from(name);
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                return Some(ext.to_lowercase());
            }
        }
        
        None
    }
}

#[derive(Debug, Clone)]
pub struct ExportOptions {
    pub version: Option<String>,
    pub unit: LengthUnit,
    pub coordinate_system: CoordinateSystem,
    pub export_hidden_entities: bool,
    pub export_layernames: bool,
    pub tessellation_tolerance: f64,
    pub binary_format: bool,
}

impl Default for ExportOptions {
    fn default() -> Self {
        Self {
            version: None,
            unit: LengthUnit::Millimeter,
            coordinate_system: CoordinateSystem::WCS,
            export_hidden_entities: false,
            export_layernames: true,
            tessellation_tolerance: 0.01,
            binary_format: false,
        }
    }
}

impl fmt::Display for LengthUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LengthUnit::Millimeter => write!(f, "mm"),
            LengthUnit::Centimeter => write!(f, "cm"),
            LengthUnit::Meter => write!(f, "m"),
            LengthUnit::Kilometer => write!(f, "km"),
            LengthUnit::Inch => write!(f, "in"),
            LengthUnit::Foot => write!(f, "ft"),
            LengthUnit::Yard => write!(f, "yd"),
            LengthUnit::Mile => write!(f, "mi"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_format() {
        let format = FormatInfo::new("dxf", "DXF", "AutoCAD Format", false);
        assert_eq!(format.extension, "dxf");
        assert!(!format.is_binary);
    }

    #[test]
    fn test_supported_formats() {
        assert!(SUPPORTED_FORMATS.len() > 0);
    }

    #[test]
    fn test_format_registry() {
        let mut registry = FormatRegistry::new();
        assert!(registry.supported_import_extensions().is_empty());
        assert!(registry.supported_export_extensions().is_empty());
    }

    #[test]
    fn test_entity_filter() {
        let filter = EntityFilter::all();
        assert!(filter.include_lines);
        assert!(filter.include_solids);
        
        let geo_filter = EntityFilter::geometry_only();
        assert!(geo_filter.include_lines);
        assert!(!geo_filter.include_solids);
    }
}
