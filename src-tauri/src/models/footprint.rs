use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Footprint {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: FootprintCategory,
    pub package_type: PackageType,
    pub pads: Vec<Pad>,
    pub silkscreen: Vec<GraphicElement>,
    pub courtyard: Vec<GraphicElement>,
    pub assembly: Vec<GraphicElement>,
    pub keepout: Option<KeepoutArea>,
    pub dimensions: FootprintDimensions,
    pub metadata: FootprintMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FootprintCategory {
    Resistor,
    Capacitor,
    Inductor,
    Diode,
    Transistor,
    IC,
    Connector,
    Crystal,
    Module,
    MechanicalHole,
    TestPoint,
    Fiducial,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PackageType {
    // SMD Packages
    SMD0201,
    SMD0402,
    SMD0603,
    SMD0805,
    SMD1206,
    SMD1210,
    SMD1812,
    SMD2010,
    SMD2512,
    
    // IC Packages
    SOT23,
    SOT223,
    SOT89,
    SOIC8,
    SOIC14,
    SOIC16,
    SOIC24,
    SOIC28,
    SSOP8,
    SSOP14,
    SSOP16,
    SSOP20,
    SSOP24,
    SSOP28,
    TSSOP8,
    TSSOP14,
    TSSOP16,
    TSSOP20,
    TSSOP24,
    TSSOP28,
    QFP32,
    QFP44,
    QFP48,
    QFP64,
    QFP100,
    QFP144,
    QFN16,
    QFN20,
    QFN24,
    QFN32,
    QFN48,
    BGA48,
    BGA64,
    BGA100,
    BGA256,
    
    // Through-Hole
    DIP8,
    DIP14,
    DIP16,
    DIP18,
    DIP20,
    DIP24,
    DIP28,
    DIP40,
    TO92,
    TO220,
    TO247,
    TO252,
    TO263,
    
    // Connectors
    PinHeader1x2,
    PinHeader1x3,
    PinHeader1x4,
    PinHeader1x5,
    PinHeader1x6,
    PinHeader1x8,
    PinHeader1x10,
    PinHeader2x2,
    PinHeader2x3,
    PinHeader2x4,
    PinHeader2x5,
    PinHeader2x6,
    PinHeader2x8,
    PinHeader2x10,
    PinHeader2x20,
    UsbA,
    UsbB,
    UsbC,
    UsbMicro,
    UsbMini,
    RJ45,
    RJ11,
    DSUB9,
    DSUB15,
    DSUB25,
    BarrelJack,
    SDCard,
    MicroSD,
    
    // Other
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pad {
    pub id: String,
    pub pad_number: String,
    pub pad_type: PadType,
    pub shape: PadShape,
    pub position: Position,
    pub size: PadSize,
    pub drill: Option<DrillInfo>,
    pub layers: Vec<LayerType>,
    pub solder_mask_expansion: f64,
    pub solder_paste_margin: f64,
    pub thermal_relief: Option<ThermalRelief>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PadType {
    SMD,
    ThroughHole,
    NPTH, // Non-Plated Through Hole
    Via,
    Castellated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PadShape {
    Circle,
    Rectangle,
    RoundedRectangle { radius: f64 },
    Oval,
    Trapezoid { delta: f64 },
    Custom(Vec<FootprintPoint>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub rotation: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PadSize {
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrillInfo {
    pub diameter: f64,
    pub shape: DrillShape,
    pub offset: Option<Position>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DrillShape {
    Circle,
    Oval { width: f64, height: f64 },
    Slot { start: FootprintPoint, end: FootprintPoint, width: f64 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LayerType {
    TopCopper,
    BottomCopper,
    InnerCopper(u8),
    TopSolderMask,
    BottomSolderMask,
    TopSilkscreen,
    BottomSilkscreen,
    TopPaste,
    BottomPaste,
    TopCourtyard,
    BottomCourtyard,
    TopAssembly,
    BottomAssembly,
    EdgeCuts,
    UserDrawing,
    UserComments,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThermalRelief {
    pub gap: f64,
    pub spoke_width: f64,
    pub spoke_count: u8,
    pub angle: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphicElement {
    pub element_type: GraphicType,
    pub layer: LayerType,
    pub width: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GraphicType {
    Line { start: FootprintPoint, end: FootprintPoint },
    Arc { center: FootprintPoint, start: FootprintPoint, angle: f64 },
    Circle { center: FootprintPoint, radius: f64 },
    Rectangle { top_left: FootprintPoint, bottom_right: FootprintPoint },
    Polygon { points: Vec<FootprintPoint> },
    Text { position: FootprintPoint, text: String, size: f64, rotation: f64 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FootprintPoint {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeepoutArea {
    pub area: Vec<FootprintPoint>,
    pub layers: Vec<LayerType>,
    pub restrictions: KeepoutRestrictions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeepoutRestrictions {
    pub no_copper: bool,
    pub no_vias: bool,
    pub no_components: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FootprintDimensions {
    pub body_width: f64,
    pub body_height: f64,
    pub courtyard_width: f64,
    pub courtyard_height: f64,
    pub pitch_x: Option<f64>,
    pub pitch_y: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FootprintMetadata {
    pub manufacturer: Option<String>,
    pub manufacturer_part: Option<String>,
    pub datasheet: Option<String>,
    pub keywords: Vec<String>,
    pub ipc_standard: Option<String>,
    pub created_date: String,
    pub modified_date: String,
    pub author: String,
    pub license: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FootprintLibrary {
    pub id: String,
    pub name: String,
    pub description: String,
    pub footprints: HashMap<String, Footprint>,
    pub version: String,
    pub author: String,
}

impl FootprintLibrary {
    pub fn new(name: String, description: String, author: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            description,
            footprints: HashMap::new(),
            version: "1.0.0".to_string(),
            author,
        }
    }
    
    pub fn add_footprint(&mut self, footprint: Footprint) {
        self.footprints.insert(footprint.id.clone(), footprint);
    }
    
    pub fn get_footprint(&self, id: &str) -> Option<&Footprint> {
        self.footprints.get(id)
    }
    
    pub fn get_by_package(&self, package: &PackageType) -> Vec<&Footprint> {
        self.footprints
            .values()
            .filter(|f| std::mem::discriminant(&f.package_type) == std::mem::discriminant(package))
            .collect()
    }
    
    pub fn get_by_category(&self, category: &FootprintCategory) -> Vec<&Footprint> {
        self.footprints
            .values()
            .filter(|f| std::mem::discriminant(&f.category) == std::mem::discriminant(category))
            .collect()
    }
}

// Component to Footprint Mapping
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentFootprintMap {
    pub component_id: String,
    pub footprint_id: String,
    pub pin_mapping: HashMap<String, String>, // component_pin -> pad_number
    pub placement_hints: PlacementHints,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlacementHints {
    pub preferred_side: BoardSide,
    pub rotation: f64,
    pub allow_rotation: bool,
    pub thermal_considerations: bool,
    pub mechanical_constraints: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BoardSide {
    Top,
    Bottom,
    Either,
}