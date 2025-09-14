use crate::models::component::{ComponentSymbol, SymbolGraphics, GraphicsBounds, DrawCommand};

pub fn create_symbol(width: f64, height: f64, draw_commands: Vec<DrawCommand>) -> ComponentSymbol {
    ComponentSymbol {
        width,
        height,
        draw_commands,
        graphics: Some(SymbolGraphics {
            bounds: GraphicsBounds { width, height }
        }),
    }
}