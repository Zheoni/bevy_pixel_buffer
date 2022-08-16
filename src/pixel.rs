//! Pixel struct with the ability to send it to the GPU.

use bevy::{
    math::{DVec3, DVec4, Vec3, Vec4},
    prelude::Color,
    render::render_resource::TextureFormat,
};

/// An RGBA pixel. 0-255 linear. Probably you don't need to use this
/// directly but convert it from and into another types such as [Color].
#[derive(Clone, Copy, Debug, PartialEq, Eq, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Pixel {
    /// Red channel
    pub r: u8,
    /// Green channel
    pub g: u8,
    /// Blue channel
    pub b: u8,
    /// Alpha channel
    pub a: u8,
}

impl Pixel {
    /// WGPU texture format the pixel of the pixel
    pub const FORMAT: TextureFormat = TextureFormat::Rgba8Unorm;

    #[allow(missing_docs)]
    pub const WHITE: Self = Self {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };
    #[allow(missing_docs)]
    pub const BLACK: Self = Self {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };
    #[allow(missing_docs)]
    pub const TRANSPARENT: Self = Self {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    };
    #[allow(missing_docs)]
    pub const RED: Self = Self {
        r: 255,
        g: 0,
        b: 0,
        a: 255,
    };
    #[allow(missing_docs)]
    pub const GREEN: Self = Self {
        r: 0,
        g: 255,
        b: 0,
        a: 255,
    };
    #[allow(missing_docs)]
    pub const BLUE: Self = Self {
        r: 0,
        g: 0,
        b: 255,
        a: 255,
    };

    /// Gets a random pixel. Solid, alpha is 255.
    ///
    /// Uses [rand::thread_rng] to get the random value.
    #[cfg(feature = "rand")]
    pub fn random() -> Self {
        let c = rand::random::<u32>() | 0xff000000;
        c.into()
    }

    /// As a bevy [Color]
    pub fn as_color(self) -> Color {
        Color::rgba_linear(
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
            self.a as f32 / 255.0,
        )
    }
}

impl From<[u8; 4]> for Pixel {
    fn from(c: [u8; 4]) -> Self {
        Self {
            r: c[0],
            g: c[1],
            b: c[2],
            a: c[3],
        }
    }
}

impl From<u32> for Pixel {
    fn from(c: u32) -> Self {
        let [r, g, b, a] = c.to_le_bytes();
        Self { r, g, b, a }
    }
}

impl From<[f32; 4]> for Pixel {
    fn from(c: [f32; 4]) -> Self {
        Self {
            r: (c[0] * 255.0) as u8,
            g: (c[1] * 255.0) as u8,
            b: (c[2] * 255.0) as u8,
            a: (c[3] * 255.0) as u8,
        }
    }
}

impl From<[f32; 3]> for Pixel {
    fn from(c: [f32; 3]) -> Self {
        Self {
            r: (c[0] * 255.0) as u8,
            g: (c[1] * 255.0) as u8,
            b: (c[2] * 255.0) as u8,
            a: 255,
        }
    }
}

impl From<[f64; 4]> for Pixel {
    fn from(c: [f64; 4]) -> Self {
        Self {
            r: (c[0] * 255.0) as u8,
            g: (c[1] * 255.0) as u8,
            b: (c[2] * 255.0) as u8,
            a: (c[3] * 255.0) as u8,
        }
    }
}

impl From<[f64; 3]> for Pixel {
    fn from(c: [f64; 3]) -> Self {
        Self {
            r: (c[0] * 255.0) as u8,
            g: (c[1] * 255.0) as u8,
            b: (c[2] * 255.0) as u8,
            a: 255,
        }
    }
}

impl From<Vec4> for Pixel {
    fn from(v: Vec4) -> Self {
        Self {
            r: (v.x * 255.0) as u8,
            g: (v.y * 255.0) as u8,
            b: (v.z * 255.0) as u8,
            a: (v.w * 255.0) as u8,
        }
    }
}

impl From<Vec3> for Pixel {
    fn from(v: Vec3) -> Self {
        Self {
            r: (v.x * 255.0) as u8,
            g: (v.y * 255.0) as u8,
            b: (v.z * 255.0) as u8,
            a: 255,
        }
    }
}

impl From<DVec4> for Pixel {
    fn from(v: DVec4) -> Self {
        Self {
            r: (v.x * 255.0) as u8,
            g: (v.y * 255.0) as u8,
            b: (v.z * 255.0) as u8,
            a: (v.w * 255.0) as u8,
        }
    }
}

impl From<DVec3> for Pixel {
    fn from(v: DVec3) -> Self {
        Self {
            r: (v.x * 255.0) as u8,
            g: (v.y * 255.0) as u8,
            b: (v.z * 255.0) as u8,
            a: 255,
        }
    }
}

impl From<Color> for Pixel {
    fn from(c: Color) -> Self {
        c.as_linear_rgba_u32().into()
    }
}
