use serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Lineweight {
    W0 = 0,
    W5 = 5,
    W9 = 9,
    W13 = 13,
    W15 = 15,
    W18 = 18,
    W20 = 20,
    W25 = 25,
    W30 = 30,
    W35 = 35,
    W40 = 40,
    W50 = 50,
    W53 = 53,
    W60 = 60,
    W70 = 70,
    W80 = 80,
    W90 = 90,
    W100 = 100,
    W105 = 105,
    W110 = 110,
    W120 = 120,
    W130 = 130,
    W140 = 140,
    W150 = 150,
    W155 = 155,
    W160 = 160,
    W170 = 170,
    W200 = 200,
    W210 = 210,
    W220 = 220,
    W230 = 230,
    W240 = 240,
    W250 = 250,
    W280 = 280,
    W310 = 310,
    Default = -1,
    ByLayer = -2,
    ByBlock = -3,
}

impl Default for Lineweight {
    fn default() -> Self {
        Lineweight::Default
    }
}

impl Lineweight {
    #[inline]
    pub fn from_mm(mm: f64) -> Self {
        match mm.round() as i32 {
            0 => Lineweight::W0,
            5 => Lineweight::W5,
            9 => Lineweight::W9,
            13 => Lineweight::W13,
            15 => Lineweight::W15,
            18 => Lineweight::W18,
            20 => Lineweight::W20,
            25 => Lineweight::W25,
            30 => Lineweight::W30,
            35 => Lineweight::W35,
            40 => Lineweight::W40,
            50 => Lineweight::W50,
            53 => Lineweight::W53,
            60 => Lineweight::W60,
            70 => Lineweight::W70,
            80 => Lineweight::W80,
            90 => Lineweight::W90,
            100 => Lineweight::W100,
            105 => Lineweight::W105,
            110 => Lineweight::W110,
            120 => Lineweight::W120,
            130 => Lineweight::W130,
            140 => Lineweight::W140,
            150 => Lineweight::W150,
            155 => Lineweight::W155,
            160 => Lineweight::W160,
            170 => Lineweight::W170,
            200 => Lineweight::W200,
            210 => Lineweight::W210,
            220 => Lineweight::W220,
            230 => Lineweight::W230,
            240 => Lineweight::W240,
            250 => Lineweight::W250,
            280 => Lineweight::W280,
            310 => Lineweight::W310,
            _ => Lineweight::Default,
        }
    }

    #[inline]
    pub fn to_mm(&self) -> f64 {
        match self {
            Lineweight::W0 => 0.0,
            Lineweight::W5 => 0.05,
            Lineweight::W9 => 0.09,
            Lineweight::W13 => 0.13,
            Lineweight::W15 => 0.15,
            Lineweight::W18 => 0.18,
            Lineweight::W20 => 0.20,
            Lineweight::W25 => 0.25,
            Lineweight::W30 => 0.30,
            Lineweight::W35 => 0.35,
            Lineweight::W40 => 0.40,
            Lineweight::W50 => 0.50,
            Lineweight::W53 => 0.53,
            Lineweight::W60 => 0.60,
            Lineweight::W70 => 0.70,
            Lineweight::W80 => 0.80,
            Lineweight::W90 => 0.90,
            Lineweight::W100 => 1.00,
            Lineweight::W105 => 1.05,
            Lineweight::W110 => 1.10,
            Lineweight::W120 => 1.20,
            Lineweight::W130 => 1.30,
            Lineweight::W140 => 1.40,
            Lineweight::W150 => 1.50,
            Lineweight::W155 => 1.55,
            Lineweight::W160 => 1.60,
            Lineweight::W170 => 1.70,
            Lineweight::W200 => 2.00,
            Lineweight::W210 => 2.10,
            Lineweight::W220 => 2.20,
            Lineweight::W230 => 2.30,
            Lineweight::W240 => 2.40,
            Lineweight::W250 => 2.50,
            Lineweight::W280 => 2.80,
            Lineweight::W310 => 3.10,
            Lineweight::Default => 0.25,
            Lineweight::ByLayer => -1.0,
            Lineweight::ByBlock => -2.0,
        }
    }

    #[inline]
    pub fn is_valid(&self) -> bool {
        !matches!(self, Lineweight::Default | Lineweight::ByLayer | Lineweight::ByBlock)
    }

    #[inline]
    pub fn as_u32(&self) -> u32 {
        *self as u32
    }
}

impl fmt::Display for Lineweight {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Lineweight::ByLayer => write!(f, "ByLayer"),
            Lineweight::ByBlock => write!(f, "ByBlock"),
            Lineweight::Default => write!(f, "Default"),
            _ => write!(f, "{}", self.to_mm()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LinetypePattern {
    pub total_length: f64,
    pub dash_lengths: Vec<f64>,
    pub space_lengths: Vec<f64>,
    pub dash_pattern: Vec<f64>,
}

impl Default for LinetypePattern {
    fn default() -> Self {
        Self::continuous()
    }
}

impl LinetypePattern {
    #[inline]
    pub fn continuous() -> Self {
        Self {
            total_length: 0.0,
            dash_lengths: Vec::new(),
            space_lengths: Vec::new(),
            dash_pattern: Vec::new(),
        }
    }

    #[inline]
    pub fn dashed() -> Self {
        Self {
            total_length: 12.0,
            dash_lengths: vec![6.0, 2.0, 2.0],
            space_lengths: vec![2.0, 2.0],
            dash_pattern: vec![6.0, -2.0, 2.0, -2.0],
        }
    }

    #[inline]
    pub fn dotted() -> Self {
        Self {
            total_length: 6.0,
            dash_lengths: vec![0.0, 2.0],
            space_lengths: vec![2.0],
            dash_pattern: vec![0.0, -2.0],
        }
    }

    #[inline]
    pub fn center() -> Self {
        Self {
            total_length: 25.0,
            dash_lengths: vec![12.0, 2.0, 1.0, 2.0],
            space_lengths: vec![2.0, 2.0],
            dash_pattern: vec![12.0, -2.0, 1.0, -2.0, 12.0, -2.0],
        }
    }

    #[inline]
    pub fn phantom() -> Self {
        Self {
            total_length: 50.0,
            dash_lengths: vec![12.0, 2.0, 1.0, 2.0, 1.0, 2.0],
            space_lengths: vec![2.0, 2.0, 2.0, 2.0],
            dash_pattern: vec![12.0, -2.0, 1.0, -2.0, 1.0, -2.0, 12.0, -2.0],
        }
    }

    #[inline]
    pub fn hidden() -> Self {
        Self {
            total_length: 6.0,
            dash_lengths: vec![1.5],
            space_lengths: vec![4.5],
            dash_pattern: vec![1.5, -4.5],
        }
    }

    #[inline]
    pub fn is_continuous(&self) -> bool {
        self.total_length == 0.0 || self.dash_pattern.is_empty()
    }

    #[inline]
    pub fn element_count(&self) -> usize {
        self.dash_lengths.len().max(self.space_lengths.len())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LinetypeAlignment {
    FillText,
    TextFill,
}

impl Default for LinetypeAlignment {
    fn default() -> Self {
        LinetypeAlignment::FillText
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Linetype {
    pub name: String,
    pub description: String,
    pub pattern: LinetypePattern,
    pub alignment: LinetypeAlignment,
    pub is_scaling: bool,
}

impl Default for Linetype {
    fn default() -> Self {
        Self::continuous()
    }
}

impl Linetype {
    #[inline]
    pub fn continuous() -> Self {
        Self {
            name: "Continuous".to_string(),
            description: "实线".to_string(),
            pattern: LinetypePattern::continuous(),
            alignment: LinetypeAlignment::FillText,
            is_scaling: true,
        }
    }

    #[inline]
    pub fn dashed() -> Self {
        Self {
            name: "DASHED".to_string(),
            description: "虚线 __ __ __ __ __ __ __ __ __ __".to_string(),
            pattern: LinetypePattern::dashed(),
            alignment: LinetypeAlignment::FillText,
            is_scaling: true,
        }
    }

    #[inline]
    pub fn dotted() -> Self {
        Self {
            name: "DOT".to_string(),
            description: "点线 . . . . . . . . . . . . . . . . . . . .".to_string(),
            pattern: LinetypePattern::dotted(),
            alignment: LinetypeAlignment::FillText,
            is_scaling: true,
        }
    }

    #[inline]
    pub fn center() -> Self {
        Self {
            name: "CENTER".to_string(),
            description: "点划线 ---- - ---- - ---- - ---- -".to_string(),
            pattern: LinetypePattern::center(),
            alignment: LinetypeAlignment::FillText,
            is_scaling: true,
        }
    }

    #[inline]
    pub fn phantom() -> Self {
        Self {
            name: "PHANTOM".to_string(),
            description: "双点划线 - - - - - - - - - - - - - - - -".to_string(),
            pattern: LinetypePattern::phantom(),
            alignment: LinetypeAlignment::FillText,
            is_scaling: true,
        }
    }

    #[inline]
    pub fn hidden() -> Self {
        Self {
            name: "HIDDEN".to_string(),
            description: "隐藏线 _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _".to_string(),
            pattern: LinetypePattern::hidden(),
            alignment: LinetypeAlignment::FillText,
            is_scaling: true,
        }
    }

    #[inline]
    pub fn dashdot() -> Self {
        Self {
            name: "DASHED2".to_string(),
            description: "点划线 __ . __ . __ . __ . __ .".to_string(),
            pattern: LinetypePattern {
                total_length: 12.0,
                dash_lengths: vec![6.0, 2.0, 0.0, 2.0],
                space_lengths: vec![2.0, 2.0],
                dash_pattern: vec![6.0, -2.0, 0.0, -2.0],
            },
            alignment: LinetypeAlignment::FillText,
            is_scaling: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LinetypeBuilder {
    name: String,
    description: String,
    pattern: LinetypePattern,
    alignment: LinetypeAlignment,
    is_scaling: bool,
}

impl Default for LinetypeBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl LinetypeBuilder {
    #[inline]
    pub fn new() -> Self {
        Self {
            name: String::new(),
            description: String::new(),
            pattern: LinetypePattern::continuous(),
            alignment: LinetypeAlignment::FillText,
            is_scaling: true,
        }
    }

    #[inline]
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    #[inline]
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }

    #[inline]
    pub fn pattern(mut self, pattern: LinetypePattern) -> Self {
        self.pattern = pattern;
        self
    }

    #[inline]
    pub fn dashed(mut self, dash_len: f64, space_len: f64) -> Self {
        self.pattern = LinetypePattern {
            total_length: dash_len + space_len,
            dash_lengths: vec![dash_len, 0.0],
            space_lengths: vec![space_len],
            dash_pattern: vec![dash_len, -space_len],
        };
        self
    }

    #[inline]
    pub fn dotted(mut self, dot_len: f64, space_len: f64) -> Self {
        self.pattern = LinetypePattern {
            total_length: dot_len + space_len,
            dash_lengths: vec![0.0],
            space_lengths: vec![space_len],
            dash_pattern: vec![0.0, -space_len],
        };
        self
    }

    #[inline]
    pub fn alignment(mut self, alignment: LinetypeAlignment) -> Self {
        self.alignment = alignment;
        self
    }

    #[inline]
    pub fn scaling(mut self, enabled: bool) -> Self {
        self.is_scaling = enabled;
        self
    }

    #[inline]
    pub fn build(self) -> Linetype {
        Linetype {
            name: self.name,
            description: self.description,
            pattern: self.pattern,
            alignment: self.alignment,
            is_scaling: self.is_scaling,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LinetypeManager {
    linetypes: std::collections::HashMap<String, Linetype>,
    current_linetype: String,
}

impl Default for LinetypeManager {
    fn default() -> Self {
        Self::new()
    }
}

impl LinetypeManager {
    #[inline]
    pub fn new() -> Self {
        let mut manager = Self {
            linetypes: std::collections::HashMap::new(),
            current_linetype: "Continuous".to_string(),
        };
        manager.register_builtin_linetypes();
        manager
    }

    fn register_builtin_linetypes(&mut self) {
        self.linetypes.insert("Continuous".to_string(), Linetype::continuous());
        self.linetypes.insert("DASHED".to_string(), Linetype::dashed());
        self.linetypes.insert("DOT".to_string(), Linetype::dotted());
        self.linetypes.insert("CENTER".to_string(), Linetype::center());
        self.linetypes.insert("PHANTOM".to_string(), Linetype::phantom());
        self.linetypes.insert("HIDDEN".to_string(), Linetype::hidden());
        self.linetypes.insert("DASHED2".to_string(), Linetype::dashdot());
    }

    #[inline]
    pub fn register(&mut self, linetype: Linetype) -> bool {
        if linetype.name.is_empty() {
            return false;
        }
        self.linetypes.insert(linetype.name.clone(), linetype);
        true
    }

    #[inline]
    pub fn get(&self, name: &str) -> Option<&Linetype> {
        self.linetypes.get(name)
    }

    #[inline]
    pub fn get_mut(&mut self, name: &str) -> Option<&mut Linetype> {
        self.linetypes.get_mut(name)
    }

    #[inline]
    pub fn exists(&self, name: &str) -> bool {
        self.linetypes.contains_key(name)
    }

    #[inline]
    pub fn remove(&mut self, name: &str) -> bool {
        if name == "Continuous" {
            return false;
        }
        self.linetypes.remove(name).is_some()
    }

    #[inline]
    pub fn set_current(&mut self, name: &str) -> bool {
        if self.linetypes.contains_key(name) {
            self.current_linetype = name.to_string();
            true
        } else {
            false
        }
    }

    #[inline]
    pub fn current(&self) -> &str {
        &self.current_linetype
    }

    #[inline]
    pub fn names(&self) -> Vec<&str> {
        self.linetypes.keys().map(|s| s.as_str()).collect()
    }

    #[inline]
    pub fn count(&self) -> usize {
        self.linetypes.len()
    }

    #[inline]
    pub fn clear(&mut self) {
        self.linetypes.clear();
        self.register_builtin_linetypes();
        self.current_linetype = "Continuous".to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lineweight_from_mm() {
        assert_eq!(Lineweight::from_mm(0.25), Lineweight::W25);
        assert_eq!(Lineweight::from_mm(0.5), Lineweight::W50);
        assert_eq!(Lineweight::from_mm(1.0), Lineweight::W100);
    }

    #[test]
    fn test_lineweight_to_mm() {
        assert!((Lineweight::W25.to_mm() - 0.25).abs() < 1e-10);
        assert!((Lineweight::W50.to_mm() - 0.5).abs() < 1e-10);
        assert!((Lineweight::W100.to_mm() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_linetype_continuous() {
        let linetype = Linetype::continuous();
        assert_eq!(linetype.name, "Continuous");
        assert!(linetype.pattern.is_continuous());
    }

    #[test]
    fn test_linetype_dashed() {
        let linetype = Linetype::dashed();
        assert_eq!(linetype.name, "DASHED");
        assert!(!linetype.pattern.is_continuous());
    }

    #[test]
    fn test_linetype_builder() {
        let linetype = LinetypeBuilder::new()
            .name("MY_DASHED")
            .description("自定义虚线")
            .dashed(4.0, 2.0)
            .build();
        assert_eq!(linetype.name, "MY_DASHED");
        assert_eq!(linetype.pattern.total_length, 6.0);
    }

    #[test]
    fn test_linetype_manager() {
        let manager = LinetypeManager::new();
        assert!(manager.exists("Continuous"));
        assert!(manager.exists("CENTER"));
        assert_eq!(manager.count(), 7);
    }

    #[test]
    fn test_linetype_manager_operations() {
        let mut manager = LinetypeManager::new();
        let custom = LinetypeBuilder::new()
            .name("MY_LINETYPE")
            .description("自定义线型")
            .dashed(3.0, 1.5)
            .build();
        assert!(manager.register(custom));
        assert!(manager.exists("MY_LINETYPE"));
        assert!(manager.set_current("DASHED"));
        assert_eq!(manager.current(), "DASHED");
    }
}
