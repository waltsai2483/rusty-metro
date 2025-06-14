use ggez::graphics::Color;

pub struct ShapePalette {
    filled: Color,
    outline: Color,
}

impl ShapePalette {
    pub fn new(filled: Color, outline: Color) -> Self {
        ShapePalette { filled, outline }
    }
    
    pub fn fill(filled: Color) -> Self {
        ShapePalette { filled, outline: filled.clone() }
    }

    pub fn filled(&self) -> Color {
        self.filled
    }

    pub fn outline(&self) -> Color {
        self.outline
    }
}