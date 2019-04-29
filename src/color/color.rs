pub const WHITE: Color = Color { r: NormFloat(1.0), g: NormFloat(1.0), b: NormFloat(1.0), a: NormFloat(1.0)};
pub const BLACK: Color = Color { r: NormFloat(0.0), g: NormFloat(0.0), b: NormFloat(0.0), a: NormFloat(1.0)};
pub const CLEAR: Color = Color { r: NormFloat(0.0), g: NormFloat(0.0), b: NormFloat(0.0), a: NormFloat(0.0)};
pub const RED: Color = Color { r: NormFloat(1.0), g: NormFloat(0.0), b: NormFloat(0.0), a: NormFloat(1.0)};
pub const GREEN: Color = Color { r: NormFloat(0.0), g: NormFloat(1.0), b: NormFloat(0.0), a: NormFloat(1.0)};
pub const BLUE: Color = Color { r: NormFloat(0.0), g: NormFloat(0.0), b: NormFloat(1.0), a: NormFloat(1.0)};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color {
    pub r: NormFloat,
    pub g: NormFloat,
    pub b: NormFloat,
    pub a: NormFloat,
}

impl Color {
    pub fn new<F: Into<NormFloat>>(r: F, g: F, b: F, a: F) -> Color {
        Color { 
            r: r.into(),
            g: g.into(),
            b: b.into(),
            a: a.into()
        }
    }

    pub fn as_array(&self) -> [f32; 4] {
        [self.r.into(), self.g.into(), self.b.into(), self.a.into()]
    }

    pub fn is_opaque(&self) -> bool {
        self.a == NormFloat(1.0)
    }
}

impl<F: Into<NormFloat> + Copy> From<[F; 4]> for Color {
    fn from(arr: [F; 4]) -> Color {
        Color::new(arr[0].into(), arr[1].into(), arr[2].into(), arr[3].into())
    }
}

impl<F: Into<NormFloat> + Copy> From<[F; 3]> for Color {
    fn from(arr: [F; 3]) -> Color {
        Color::new(arr[0].into(), arr[1].into(), arr[2].into(), NormFloat::from(1.0))
    }
}

/// A float bounded to the range [0.0, 1.0]
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct NormFloat(f32);

impl From<f32> for NormFloat {
    fn from(f: f32) -> NormFloat {
        let val = f.max(0.0).min(1.0);
        NormFloat(val)
    }
}

impl From<f64> for NormFloat {
    fn from(f: f64) -> NormFloat {
        let val = f.max(0.0).min(1.0) as f32;
        NormFloat(val)
    }
}

impl From<NormFloat> for f32 {
    fn from(f: NormFloat) -> f32 {
        let NormFloat(val) = f;
        val
    }
}

impl From<NormFloat> for f64 {
    fn from(f: NormFloat) -> f64 {
        let NormFloat(val) = f;
        val as f64
    }
}