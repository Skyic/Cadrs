//! 2D CAD SDK - Rust实现的专业CAD几何核心库
//!
//! 本库提供完整的2D CAD几何处理功能，包括几何曲线、标注、公差、图案填充、
//! 文字处理、图层管理、空间索引和约束求解等模块。
//!
//! # 主要特性
//!
//! - **几何模块**：支持多种2D几何曲线，包括渐开线、齿轮齿廓、椭圆弧等
//! - **标注模块**：完整的尺寸标注系统，包括线性、角度、半径、坐标标注
//! - **图案填充**：支持ANSI/ISO标准图案和渐变填充
//! - **文字处理**：多行文字、字段、样式系统
//! - **图层管理**：完整的图层控制、过滤、状态管理
//! - **空间索引**：R-Tree、四叉树、网格索引
//! - **约束求解**：几何约束和尺寸约束求解
//! - **API层**：命令系统、事件处理、Python绑定

pub mod geometry;
pub mod dimension;
pub mod geometric_tolerance;
pub mod hatch;
pub mod text;
pub mod layer;
pub mod spatial;
pub mod constraint;
pub mod api;

pub use geometry::*;
pub use dimension::*;
pub use geometric_tolerance::*;
pub use hatch::*;
pub use text::*;
pub use layer::*;
pub use spatial::*;
pub use constraint::*;
pub use api::*;

/// CAD SDK主版本号
pub const CAD_SDK_VERSION: (u32, u32, u32) = (0, 1, 0);

/// 获取SDK版本字符串
#[inline]
pub fn version() -> String {
    format!("{}.{}.{}", CAD_SDK_VERSION.0, CAD_SDK_VERSION.1, CAD_SDK_VERSION.2)
}

/// SDK初始化配置
#[derive(Debug, Clone)]
pub struct SDKConfig {
    /// 默认图形单位
    pub default_units: DrawingUnits,
    /// 默认文字高度
    pub default_text_height: f64,
    /// 默认标注箭头大小
    pub default_arrow_size: f64,
    /// 默认标注文字高度
    pub default_dim_text_height: f64,
    /// 默认图层名称
    pub default_layer: String,
    /// 精度小数位数
    pub decimal_places: u32,
    /// 角度显示精度
    pub angular_precision: u32,
}

impl Default for SDKConfig {
    fn default() -> Self {
        Self {
            default_units: DrawingUnits::Millimeters,
            default_text_height: 2.5,
            default_arrow_size: 2.5,
            default_dim_text_height: 3.5,
            default_layer: String::from("0"),
            decimal_places: 4,
            angular_precision: 0,
        }
    }
}

/// SDK全局状态
struct GlobalState {
    config: SDKConfig,
    is_initialized: bool,
}

impl GlobalState {
    fn new() -> Self {
        Self {
            config: SDKConfig::default(),
            is_initialized: false,
        }
    }
}

static mut GLOBAL_STATE: Option<GlobalState> = None;

/// 初始化SDK
///
/// # 示例
///
/// ```
/// use cad_sdk::initialize;
///
/// let config = cad_sdk::SDKConfig::default();
/// initialize(config).expect("SDK初始化失败");
/// ```
pub fn initialize(config: SDKConfig) -> Result<(), String> {
    if unsafe { GLOBAL_STATE.is_some() } {
        return Err("SDK已经初始化".to_string());
    }

    unsafe {
        GLOBAL_STATE = Some(GlobalState::new());
    }
    Ok(())
}

/// 检查SDK是否已初始化
#[inline]
pub fn is_initialized() -> bool {
    unsafe { GLOBAL_STATE.is_some() }
}

/// 获取当前配置
pub fn get_config() -> SDKConfig {
    unsafe {
        GLOBAL_STATE.as_ref()
            .map(|s| s.config.clone())
            .unwrap_or_default()
    }
}

/// 更新配置
pub fn update_config(config: SDKConfig) {
    unsafe {
        if let Some(state) = GLOBAL_STATE.as_mut() {
            state.config = config;
        }
    }
}

/// 关闭SDK
pub fn shutdown() {
    unsafe {
        GLOBAL_STATE = None;
    }
}

/// 便捷函数：创建新图形文档
///
/// # 示例
///
/// ```
/// use cad_sdk::*;
///
/// let mut doc = create_document("MyDrawing".to_string());
/// ```
pub fn create_document(name: String) -> CADDocument {
    CADDocument::new(name)
}

/// 便捷函数：创建图层管理器
///
/// # 示例
///
/// ```
/// use cad_sdk::*;
///
/// let mut layer_manager = create_layer_manager();
/// ```
pub fn create_layer_manager() -> LayerManager {
    LayerManager::new()
}

/// 便捷函数：创建图案库
///
/// # 示例
///
/// ```
/// use cad_sdk::*;
///
/// let pattern_library = PatternLibrary;
/// let ansi31 = pattern_library.get_pattern("ANSI31");
/// ```
pub fn get_pattern_library() -> impl HatchPatternProvider {
    PatternLibrary
}

/// 图案库提供者特征
pub trait HatchPatternProvider {
    fn get_pattern(&self, name: &str) -> Option<HatchPattern>;
    fn available_patterns(&self) -> Vec<&'static str>;
}

/// 标准图案库实现
pub struct StandardPatternLibrary;

impl HatchPatternProvider for StandardPatternLibrary {
    fn get_pattern(&self, name: &str) -> Option<HatchPattern> {
        match name {
            "ANSI31" => Some(self.create_ansi31()),
            "ANSI32" => Some(self.create_ansi32()),
            "ANSI33" => Some(self.create_ansi33()),
            "ANSI34" => Some(self.create_ansi34()),
            "ANSI35" => Some(self.create_ansi35()),
            "ANSI36" => Some(self.create_ansi36()),
            "ANSI37" => Some(self.create_ansi37()),
            "ANSI38" => Some(self.create_ansi38()),
            "ISO01" => Some(self.create_iso01()),
            "ISO02" => Some(self.create_iso02()),
            "ISO03" => Some(self.create_iso03()),
            "ISO04" => Some(self.create_iso04()),
            "ISO05" => Some(self.create_iso05()),
            "BRICK" => Some(self.create_brick()),
            "GRID" => Some(self.create_grid()),
            "CROSS" => Some(self.create_cross()),
            _ => None,
        }
    }

    fn available_patterns(&self) -> Vec<&'static str> {
        vec![
            "ANSI31", "ANSI32", "ANSI33", "ANSI34", "ANSI35",
            "ANSI36", "ANSI37", "ANSI38", "ISO01", "ISO02",
            "ISO03", "ISO04", "ISO05", "BRICK", "GRID", "CROSS",
        ]
    }
}

impl StandardPatternLibrary {
    fn create_ansi31(&self) -> HatchPattern {
        let mut pattern = HatchPattern::new(
            "ANSI31".to_string(),
            "ANSI Iron, Brick, and Masonry".to_string(),
        );
        pattern = pattern.with_angle(45.0);
        pattern
    }

    fn create_ansi32(&self) -> HatchPattern {
        let mut pattern = HatchPattern::new(
            "ANSI32".to_string(),
            "ANSI Steel".to_string(),
        );
        pattern = pattern.with_angle(45.0);
        let mut line1 = HatchLine::new(45.0, 8.0);
        let mut line2 = HatchLine::new(135.0, 8.0);
        pattern = pattern.add_line(line1);
        pattern = pattern.add_line(line2);
        pattern
    }

    fn create_ansi33(&self) -> HatchPattern {
        let mut pattern = HatchPattern::new(
            "ANSI33".to_string(),
            "ANSI Bronze, Brass, Copper".to_string(),
        );
        let mut line1 = HatchLine::new(45.0, 4.0);
        let mut line2 = HatchLine::new(135.0, 4.0);
        pattern = pattern.add_line(line1);
        pattern = pattern.add_line(line2);
        pattern = pattern.as_double();
        pattern
    }

    fn create_ansi34(&self) -> HatchPattern {
        let mut pattern = HatchPattern::new(
            "ANSI34".to_string(),
            "ANSI Plastics".to_string(),
        );
        let mut line1 = HatchLine::new(45.0, 6.0);
        let mut line2 = HatchLine::new(135.0, 6.0);
        pattern = pattern.add_line(line1);
        pattern = pattern.add_line(line2);
        pattern = pattern.as_double();
        pattern
    }

    fn create_ansi35(&self) -> HatchPattern {
        let mut pattern = HatchPattern::new(
            "ANSI35".to_string(),
            "ANSI Hard Rock".to_string(),
        );
        let mut line1 = HatchLine::new(45.0, 5.0);
        let mut line2 = HatchLine::new(135.0, 5.0);
        pattern = pattern.add_line(line1);
        pattern = pattern.add_line(line2);
        pattern = pattern.as_double();
        pattern
    }

    fn create_ansi36(&self) -> HatchPattern {
        let mut pattern = HatchPattern::new(
            "ANSI36".to_string(),
            "ANSI Earth".to_string(),
        );
        let mut line1 = HatchLine::new(45.0, 12.0);
        let mut line2 = HatchLine::new(135.0, 12.0);
        pattern = pattern.add_line(line1);
        pattern = pattern.add_line(line2);
        pattern = pattern.as_double();
        pattern
    }

    fn create_ansi37(&self) -> HatchPattern {
        let mut pattern = HatchPattern::new(
            "ANSI37".to_string(),
            "ANSI Concrete".to_string(),
        );
        let mut line1 = HatchLine::new(45.0, 10.0);
        let mut line2 = HatchLine::new(135.0, 10.0);
        pattern = pattern.add_line(line1);
        pattern = pattern.add_line(line2);
        pattern = pattern.as_double();
        pattern
    }

    fn create_ansi38(&self) -> HatchPattern {
        let mut pattern = HatchPattern::new(
            "ANSI38".to_string(),
            "ANSI Lead, Zinc, Magnesium, Aluminum".to_string(),
        );
        let mut line1 = HatchLine::new(45.0, 2.0);
        let mut line2 = HatchLine::new(135.0, 2.0);
        pattern = pattern.add_line(line1);
        pattern = pattern.add_line(line2);
        pattern = pattern.as_double();
        pattern
    }

    fn create_iso01(&self) -> HatchPattern {
        HatchPattern::new("ISO01".to_string(), "ISO Light".to_string())
    }

    fn create_iso02(&self) -> HatchPattern {
        HatchPattern::new("ISO02".to_string(), "ISO Medium".to_string())
    }

    fn create_iso03(&self) -> HatchPattern {
        HatchPattern::new("ISO03".to_string(), "ISO Dense".to_string())
    }

    fn create_iso04(&self) -> HatchPattern {
        let mut pattern = HatchPattern::new(
            "ISO04".to_string(),
            "ISO Light double".to_string(),
        );
        pattern = pattern.as_double();
        pattern
    }

    fn create_iso05(&self) -> HatchPattern {
        let mut pattern = HatchPattern::new(
            "ISO05".to_string(),
            "ISO Medium double".to_string(),
        );
        pattern = pattern.as_double();
        pattern
    }

    fn create_brick(&self) -> HatchPattern {
        HatchPattern::new("BRICK".to_string(), "Brick pattern".to_string())
    }

    fn create_grid(&self) -> HatchPattern {
        HatchPattern::new("GRID".to_string(), "Grid pattern".to_string())
    }

    fn create_cross(&self) -> HatchPattern {
        HatchPattern::new("CROSS".to_string(), "Crosshatch pattern".to_string())
    }
}

/// 便捷函数：获取标准图案库
#[inline]
pub fn standard_pattern_library() -> impl HatchPatternProvider {
    StandardPatternLibrary
}

/// 常用几何计算工具
pub mod geom_tools {
    use super::*;

    /// 计算两条直线的交点
    ///
    /// # 参数
    ///
    /// * `l1` - 第一条直线
    /// * `l2` - 第二条直线
    /// * `extend` - 是否允许延长直线
    ///
    /// # 返回
    ///
    /// 如果相交返回交点，否则返回None
    pub fn line_intersection(l1: &Line, l2: &Line, extend: bool) -> Option<Point> {
        let x1 = l1.start.x;
        let y1 = l1.start.y;
        let x2 = l1.end.x;
        let y2 = l1.end.y;
        let x3 = l2.start.x;
        let y3 = l2.start.y;
        let x4 = l2.end.x;
        let y4 = l2.end.y;

        let denom = (y4 - y3) * (x2 - x1) - (x4 - x3) * (y2 - y1);
        if denom.abs() < 1e-10 {
            return None;
        }

        let ua = ((x4 - x3) * (y1 - y3) - (y4 - y3) * (x1 - x3)) / denom;

        if !extend {
            if ua < 0.0 || ua > 1.0 {
                return None;
            }
        }

        let ub = ((x2 - x1) * (y1 - y3) - (y2 - y1) * (x1 - x3)) / denom;

        if !extend {
            if ub < 0.0 || ub > 1.0 {
                return None;
            }
        }

        Some(Point::new(
            x1 + ua * (x2 - x1),
            y1 + ua * (y2 - y1),
        ))
    }

    /// 计算点到直线的投影
    pub fn point_line_projection(point: &Point, line: &Line) -> Point {
        let dx = line.end.x - line.start.x;
        let dy = line.end.y - line.start.y;
        let len_sq = dx * dx + dy * dy;

        if len_sq < 1e-10 {
            return line.start;
        }

        let t = ((point.x - line.start.x) * dx + (point.y - line.start.y) * dy) / len_sq;
        let t = t.clamp(0.0, 1.0);

        Point::new(
            line.start.x + t * dx,
            line.start.y + t * dy,
        )
    }

    /// 计算点到直线的最短距离
    pub fn point_line_distance(point: &Point, line: &Line) -> f64 {
        point.distance_to(&point_line_projection(point, line))
    }

    /// 计算两条直线的最短距离
    pub fn line_line_distance(l1: &Line, l2: &Line) -> f64 {
        let intersection = line_intersection(l1, l2, true);
        if intersection.is_some() {
            return 0.0;
        }

        let d1 = point_line_distance(&l1.start, l2);
        let d2 = point_line_distance(&l1.end, l2);
        let d3 = point_line_distance(&l2.start, l1);
        let d4 = point_line_distance(&l2.end, l1);

        d1.min(d2).min(d3).min(d4)
    }

    /// 判断点是否在直线段上
    pub fn point_on_line(point: &Point, line: &Line, tolerance: f64) -> bool {
        point_line_distance(point, line) < tolerance
    }

    /// 计算多边形面积
    pub fn polygon_area(points: &[Point]) -> f64 {
        if points.len() < 3 {
            return 0.0;
        }

        let mut area = 0.0;
        for i in 0..points.len() {
            let j = (i + 1) % points.len();
            area += points[i].x * points[j].y;
            area -= points[j].x * points[i].y;
        }

        area.abs() / 2.0
    }

    /// 计算多边形的形心
    pub fn polygon_centroid(points: &[Point]) -> Option<Point> {
        if points.len() < 3 {
            return None;
        }

        let area = polygon_area(points);
        if area < 1e-10 {
            return None;
        }

        let mut cx = 0.0;
        let mut cy = 0.0;

        for i in 0..points.len() {
            let j = (i + 1) % points.len();
            let factor = points[i].x * points[j].y - points[j].x * points[i].y;
            cx += (points[i].x + points[j].x) * factor;
            cy += (points[i].y + points[j].y) * factor;
        }

        cx /= 6.0 * area;
        cy /= 6.0 * area;

        Some(Point::new(cx, cy))
    }

    /// 判断点是否在多边形内
    pub fn point_in_polygon(point: &Point, points: &[Point]) -> bool {
        let mut inside = false;
        let n = points.len();

        for i in 0..n {
            let j = (i + 1) % n;
            let xi = points[i].x;
            let yi = points[i].y;
            let xj = points[j].x;
            let yj = points[j].y;

            if ((yi > point.y) != (yj > point.y)) &&
               (point.x < (xj - xi) * (point.y - yi) / (yj - yi) + xi) {
                inside = !inside;
            }
        }

        inside
    }

    /// 角度转换为弧度
    #[inline]
    pub fn deg_to_rad(degrees: f64) -> f64 {
        degrees * std::f64::consts::PI / 180.0
    }

    /// 弧度转换为角度
    #[inline]
    pub fn rad_to_deg(radians: f64) -> f64 {
        radians * 180.0 / std::f64::consts::PI
    }

    /// 角度规范化到0-360范围
    #[inline]
    pub fn normalize_angle(angle: f64) -> f64 {
        let mut angle = angle % 360.0;
        if angle < 0.0 {
            angle += 360.0;
        }
        angle
    }

    /// 角度规范化到0-2π范围
    #[inline]
    pub fn normalize_angle_rad(angle: f64) -> f64 {
        let mut angle = angle % (2.0 * std::f64::consts::PI);
        if angle < 0.0 {
            angle += 2.0 * std::f64::consts::PI;
        }
        angle
    }

    /// 线性插值
    #[inline]
    pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
        a + (b - a) * t.clamp(0.0, 1.0)
    }

    /// 点线性插值
    #[inline]
    pub fn point_lerp(p1: &Point, p2: &Point, t: f64) -> Point {
        Point::new(
            lerp(p1.x, p2.x, t),
            lerp(p1.y, p2.y, t),
        )
    }

    /// 计算两条直线的夹角（弧度）
    pub fn angle_between_lines(l1: &Line, l2: &Line) -> f64 {
        let a1 = l1.start.angle_to(l1.end);
        let a2 = l2.start.angle_to(l2.end);
        let diff = a2 - a1;
        diff.abs().min(2.0 * std::f64::consts::PI - diff.abs())
    }

    /// 计算点到圆的最短距离
    pub fn point_circle_distance(point: &Point, circle: &Circle) -> f64 {
        point.distance_to(&circle.center) - circle.radius
    }

    /// 计算点到圆弧的最短距离
    pub fn point_arc_distance(point: &Point, arc: &Arc) -> f64 {
        let center_dist = point.distance_to(&arc.center);
        let radial_dist = center_dist - arc.radius;

        let angle = arc.center.angle_to(*point);
        let angle = normalize_angle_rad(angle);

        let start = normalize_angle_rad(arc.start_angle);
        let end = normalize_angle_rad(arc.end_angle);

        let on_arc = if start <= end {
            angle >= start && angle <= end
        } else {
            angle >= start || angle <= end
        };

        if on_arc {
            radial_dist.abs()
        } else {
            let start_dist = point.distance_to(&Point::new(
                arc.center.x + arc.radius * start.cos(),
                arc.center.y + arc.radius * start.sin(),
            ));
            let end_dist = point.distance_to(&Point::new(
                arc.center.x + arc.radius * end.cos(),
                arc.center.y + arc.radius * end.sin(),
            ));

            start_dist.min(end_dist)
        }
    }

    /// 判断两条线是否平行
    pub fn lines_parallel(l1: &Line, l2: &Line, tolerance: f64) -> bool {
        let angle1 = l1.start.angle_to(l1.end);
        let angle2 = l2.start.angle_to(l2.end);
        let diff = (angle1 - angle2).abs();
        diff.min(2.0 * std::f64::consts::PI - diff) < tolerance
    }

    /// 判断两条线是否垂直
    pub fn lines_perpendicular(l1: &Line, l2: &Line, tolerance: f64) -> bool {
        let angle1 = l1.start.angle_to(l1.end);
        let angle2 = l2.start.angle_to(l2.end);
        let diff = ((angle1 - angle2) % std::f64::consts::PI).abs();
        (diff - std::f64::consts::PI / 2.0).abs() < tolerance
    }
}

/// 单位转换工具
pub mod unit_conversion {
    use super::*;

    /// 单位转换因子（相对于毫米）
    const MM_FACTOR: f64 = 1.0;
    const CM_FACTOR: f64 = 10.0;
    const M_FACTOR: f64 = 1000.0;
    const INCH_FACTOR: f64 = 25.4;
    const FT_FACTOR: f64 = 304.8;

    /// 将数值从一个单位转换到另一个单位
    pub fn convert(value: f64, from: DrawingUnits, to: DrawingUnits) -> f64 {
        let mm_value = value * get_factor(from);
        mm_value / get_factor(to)
    }

    /// 获取单位转换因子
    fn get_factor(unit: DrawingUnits) -> f64 {
        match unit {
            DrawingUnits::Millimeters => MM_FACTOR,
            DrawingUnits::Centimeters => CM_FACTOR,
            DrawingUnits::Meters => M_FACTOR,
            DrawingUnits::Inches => INCH_FACTOR,
            DrawingUnits::Feet => FT_FACTOR,
            _ => MM_FACTOR,
        }
    }

    /// 毫米转英寸
    #[inline]
    pub fn mm_to_inch(mm: f64) -> f64 {
        mm / INCH_FACTOR
    }

    /// 英寸转毫米
    #[inline]
    pub fn inch_to_mm(inch: f64) -> f64 {
        inch * INCH_FACTOR
    }

    /// 毫米转英尺
    #[inline]
    pub fn mm_to_foot(mm: f64) -> f64 {
        mm / FT_FACTOR
    }

    /// 英尺转毫米
    #[inline]
    pub fn foot_to_mm(foot: f64) -> f64 {
        foot * FT_FACTOR
    }
}

/// 导入常用类型
pub use geom_tools::*;
pub use unit_conversion::*;
