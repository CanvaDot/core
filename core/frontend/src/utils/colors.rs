use palette::{FromColor, Hsl, Srgb};

pub static SUCCESS_GREEN: Srgb<u8> = Srgb::new(0, 173, 12);
pub static INFO_BLUE: Srgb<u8> = Srgb::new(0, 105, 255);
pub static ERROR_RED: Srgb<u8> = Srgb::new(230, 23, 23);
pub static SECONDARY_GREY: Srgb<u8> = Srgb::new(224, 224, 224);

const EPS: f32 = 1e-6;

// assertions made by `palette`.
// TODO: Add num_traits.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub fn decompose(rgb: Srgb<u8>) -> (u16, u8) {
    let rgb_f: Srgb<f32> = rgb.into_format();

    let max = rgb_f
        .red
        .max(rgb_f.green)
        .max(rgb_f.blue);
    let min = rgb_f
        .red
        .min(rgb_f.green)
        .min(rgb_f.blue);
    let delta = max - min;

    let hue = if delta.abs() < EPS {
        0.0
    } else if (max - rgb_f.red).abs() < EPS {
        60.0 * (((rgb_f.green - rgb_f.blue) / delta) % 6.0)
    } else if (max - rgb_f.green) < EPS {
        60.0 * (((rgb_f.blue - rgb_f.red) / delta) + 2.0)
    } else {
        60.0 * (((rgb_f.red - rgb_f.green) / delta) + 4.0)
    };

    let hue = (if hue < 0.0 { hue + 360.0 } else { hue }).round() as u16;
    let lightness = (f32::midpoint(max, min) * 100.0).round() as u8;

    (hue, lightness)
}

pub fn compose(hue: u16, lightness: u8) -> Srgb<u8> {
    let hsl = Hsl::new(f32::from(hue), 1.0, f32::from(lightness) / 100.0);
    Srgb::from_color(hsl).into_format()
}

pub fn luminance(color: Srgb<u8>) -> f32 {
    let r = f32::from(color.red) / 255.0;
    let g = f32::from(color.green) / 255.0;
    let b = f32::from(color.blue) / 255.0;

    0.2126 * r + 0.7152 * g + 0.0722 * b
}

pub fn contrasting_bw(color: Srgb<u8>) -> Srgb<u8> {
    if luminance(color) > 0.5 {
        Srgb::new(0, 0, 0)
    } else {
        Srgb::new(255, 255, 255)
    }
}
