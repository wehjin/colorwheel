#[derive(Copy, Clone)]
pub enum Color {
    Black,
    Grey,
    White,
    Red,
    Green,
    Blue,
    Magenta,
    Cyan,
    Yellow,
    Violet,
    Rose,
    Orange,
    ChartreuseGreen,
    SpringGreen,
    Azure,
    Custom(u32)
}

impl Color {
    pub fn to_argb(&self) -> u32 {
        match *self {
            Color::Black => 0xff000000,
            Color::Grey => 0xff7f7f7f,
            Color::White => 0xffffffff,
            Color::Blue => 0xff0000ff,
            Color::Violet => 0xff7f00ff,
            Color::Magenta => 0xffff00ff,
            Color::Rose => 0xffff007f,
            Color::Red => 0xffff0000,
            Color::Orange => 0xffff7f00,
            Color::Yellow => 0xffffff00,
            Color::ChartreuseGreen => 0xff7fff00,
            Color::Green => 0xff00ff00,
            Color::SpringGreen => 0xff00ff7f,
            Color::Cyan => 0xff00ffff,
            Color::Azure => 0xff007fff,
            Color::Custom(value) => value
        }
    }
    pub fn mix_rgb(&self, other: Color, extent: f32) -> Self {
        let (other_r, other_g, other_b) = to_rgb_components(other.to_argb());
        let (my_a, my_r, my_g, my_b) = to_argb_components(self.to_argb());
        let (new_a, new_r, new_g, new_b) = (my_a as u32,
                                            mix(my_r, other_r, extent),
                                            mix(my_g, other_g, extent),
                                            mix(my_b, other_b, extent));
        Color::Custom(to_argb(new_a, new_r, new_g, new_b))
    }
    pub fn lighten(&self, extent: f32) -> Self {
        self.mix_rgb(Color::White, extent)
    }
    pub fn darken(&self, extent: f32) -> Self {
        self.mix_rgb(Color::Black, extent)
    }
    pub fn strengthen(&self, extent: f32) -> Self {
        self.mix_a(0xff, extent)
    }
    pub fn weaken(&self, extent: f32) -> Self {
        self.mix_a(0x00, extent)
    }

    fn mix_a(&self, a: u8, extent: f32) -> Self {
        let (my_a, my_r, my_g, my_b) = to_argb_components(self.to_argb());
        let new_a = mix(my_a, a, extent);
        Color::Custom(to_argb(new_a, my_r as u32, my_g as u32, my_b as u32))
    }
}

fn mix(a: u8, b: u8, extent: f32) -> u32 {
    let (a32, b32) = (a as i32, b as i32);
    let range = b32 - a32;
    let delta = extent * range as f32;
    (a32 as f32 + delta) as u32
}

fn to_rgb_components(argb: u32) -> (u8, u8, u8) {
    ((argb >> 16 & 0xff) as u8, (argb >> 8 & 0xff) as u8, (argb & 0xff) as u8)
}

fn to_argb_components(argb: u32) -> (u8, u8, u8, u8) {
    ((argb >> 24) as u8, (argb >> 16 & 0xff) as u8, (argb >> 8 & 0xff) as u8, (argb & 0xff) as u8)
}

fn to_argb(a: u32, r: u32, g: u32, b: u32) -> u32 {
    (a << 24) | (r << 16) | (g << 8) | b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weaken_moves_opaque_to_transparent() {
        let result = Color::White.weaken(1.0);
        assert_eq!(0x00ffffff, result.to_argb())
    }

    #[test]
    fn strengthen_moves_transparent_to_opaque() {
        let result = Color::Custom(0x00ffffff).strengthen(1.0);
        assert_eq!(0xffffffff, result.to_argb())
    }

    #[test]
    fn lighten_half_moves_black_to_grey() {
        let result = Color::Black.lighten(0.5);
        assert_eq!(Color::Grey.to_argb(), result.to_argb())
    }

    #[test]
    fn lighten_zero_moves_black_to_black() {
        let result = Color::Black.lighten(0.0);
        assert_eq!(Color::Black.to_argb(), result.to_argb())
    }

    #[test]
    fn darken_half_moves_white_to_grey() {
        let result = Color::White.darken(0.5);
        assert_eq!(Color::Grey.to_argb(), result.to_argb())
    }

    #[test]
    fn darken_zero_moves_white_to_white() {
        let result = Color::White.darken(0.0);
        assert_eq!(Color::White.to_argb(), result.to_argb())
    }

    #[test]
    fn mix_rgb_averages_rgb_components() {
        let a = Color::Custom(0x00010203);
        let b = Color::Custom(0xff030201);
        let result = a.mix_rgb(b, 0.5);
        assert_eq!(0x00020202, result.to_argb())
    }

    #[test]
    fn to_argb_produces_value_for_red_color() {
        let result = Color::Red.to_argb();
        assert_eq!(0xffff0000, result)
    }

    #[test]
    fn to_argb_produces_value_for_custom_color() {
        let result = Color::Custom(0x01020304).to_argb();
        assert_eq!(0x01020304, result)
    }
}
