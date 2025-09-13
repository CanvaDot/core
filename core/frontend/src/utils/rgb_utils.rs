use palette::{FromColor, Hsl, Srgb};


pub fn decompose(rgb: Srgb<u8>) -> (u16, u8) {
    let rgb_f: Srgb<f32> = rgb.into_format();

    let max = rgb_f.red.max(rgb_f.green).max(rgb_f.blue);
    let min = rgb_f.red.min(rgb_f.green).min(rgb_f.blue);
    let delta = max - min;

    let hue = if delta == 0.0 {
        0.0
    } else if max == rgb_f.red {
        60.0 * (((rgb_f.green - rgb_f.blue) / delta) % 6.0)
    } else if max == rgb_f.green {
        60.0 * (((rgb_f.blue - rgb_f.red) / delta) + 2.0)
    } else {
        60.0 * (((rgb_f.red - rgb_f.green) / delta) + 4.0)
    };

    let hue = (if hue < 0.0 { hue + 360.0 } else { hue }).round() as u16;
    let lightness = ((max + min) / 2.0 * 100.0).round() as u8;

    (hue, lightness)
}

pub fn compose(hue: u16, lightness: u8) -> Srgb<u8> {
    let hsl = Hsl::new(hue as f32, 1.0, lightness as f32 / 100.0);
    Srgb::from_color(hsl).into_format()
}

