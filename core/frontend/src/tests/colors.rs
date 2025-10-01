use palette::Srgb;

use crate::utils::colors::{compose, contrasting_bw, decompose, luminance};

#[test]
fn test_decompose_basic_colors() {
    // Red
    let (h, l) = decompose(&Srgb::new(255u8, 0, 0));
    assert_eq!(h, 0);
    assert_eq!(l, 50);

    // Green
    let (h, l) = decompose(&Srgb::new(0u8, 255, 0));
    assert_eq!(h, 120);
    assert_eq!(l, 50);

    // Blue
    let (h, l) = decompose(&Srgb::new(0u8, 0, 255));
    assert_eq!(h, 240);
    assert_eq!(l, 50);

    // Gray (no hue, mid-lightness)
    let (h, l) = decompose(&Srgb::new(128u8, 128, 128));
    assert_eq!(h, 0); // hue meaningless, normalized to 0
    assert_eq!(l, 50);
}

#[test]
fn test_compose_round_trip() {
    // Hue + lightness back to Srgb<u8>
    let rgb = compose(0, 50); // red-ish
    assert!(rgb.red > 200);
    assert!(rgb.green < 60);
    assert!(rgb.blue < 60);

    let rgb = compose(120, 50); // green-ish
    assert!(rgb.green > 200);
}

#[test]
fn test_luminance_edges() {
    let black = Srgb::new(0u8, 0, 0);
    assert_eq!(luminance(&black), 0.0);

    let white = Srgb::new(255u8, 255, 255);
    assert!((luminance(&white) - 1.0).abs() < 1e-6);

    let mid_gray = Srgb::new(128u8, 128, 128);
    let lum = luminance(&mid_gray);
    assert!(lum > 0.49 && lum < 0.51);
}

#[test]
fn test_contrasting_bw() {
    let black = Srgb::new(0u8, 0, 0);
    assert_eq!(contrasting_bw(&black), Srgb::new(255, 255, 255));

    let white = Srgb::new(255u8, 255, 255);
    assert_eq!(contrasting_bw(&white), Srgb::new(0, 0, 0));

    let mid_gray = Srgb::new(128u8, 128, 128);
    // Around 0.5 luminance → implementation chooses black
    let c = contrasting_bw(&mid_gray);
    assert!(c == Srgb::new(0, 0, 0) || c == Srgb::new(255, 255, 255));
}
