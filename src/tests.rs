use crate::RGB;

#[test]
fn hex_formatting_works() {
    let rgb: RGB = RGB::new(0, 15, 255);
    assert_eq!(rgb.hex(), "#000FFF");
}

#[test]
fn debug_formats_correctly() {
    let rgb: RGB = RGB::new(10, 20, 30);
    assert_eq!(format!("{:?}", rgb), "RGB color (#0A141E)");
}

#[test]
fn display_formats_as_hex() {
    let rgb: RGB = RGB::new(10, 20, 30);
    assert_eq!(rgb.to_string(), "#0A141E");
}

#[test]
fn default_is_black() {
    let rgb: RGB = RGB::default();
    assert_eq!(rgb, RGB::new(0, 0, 0));
}

#[test]
fn from_tuple_works() {
    let rgb: RGB = RGB::from((10, 20, 30));
    assert_eq!(rgb, RGB::new(10, 20, 30));
}

#[test]
fn from_array_works() {
    let rgb: RGB = RGB::from([10, 20, 30]);
    assert_eq!(rgb, RGB::new(10, 20, 30));
}

#[test]
fn cloning_works() {
    let rgb1: RGB = RGB::new(100, 150, 200);
    let rgb2: RGB = rgb1.clone();
    assert_eq!(rgb1, rgb2);
}

#[test]
fn equality_checks_work() {
	let rgb1: RGB = RGB::new(50, 100, 150);
	let rgb2: RGB = RGB::new(50, 100, 150);
	assert!(rgb1 == rgb2);
}

#[test]
fn inequality_checks_work() {
	let rgb1: RGB = RGB::new(50, 100, 150);
	let rgb2: RGB = RGB::new(60, 110, 160);
	assert!(rgb1 != rgb2);
}

#[test]
fn inversion_works() {
	let rgb: RGB = RGB::new(100, 150, 200);
	let inverted: RGB = rgb.invert();
	assert_eq!(inverted, RGB::new(155, 105, 55));
}

#[test]
fn luminance_calculation_works() {
	let rgb: RGB = RGB::new(200, 118, 100);
	let luminance: u8 = rgb.luminance();
	assert_eq!(luminance, 134);
}

#[test]
fn addition_works() {
	let rgb1: RGB = RGB::new(100, 150, 200);
	let rgb2: RGB = RGB::new(50, 100, 150);
	let result: RGB = rgb1 + rgb2;
	assert_eq!(result, RGB::new(150, 250, 255));
}

#[test]
fn addition_saturates() {
	let rgb1: RGB = RGB::new(200, 250, 255);
	let rgb2: RGB = RGB::new(100, 100, 100);
	let result: RGB = rgb1 + rgb2;
	assert_eq!(result, RGB::new(255, 255, 255));
}

#[test]
fn subtraction_works() {
	let rgb1: RGB = RGB::new(100, 150, 200);
	let rgb2: RGB = RGB::new(50, 100, 150);
	let result: RGB = rgb1 - rgb2;
	assert_eq!(result, RGB::new(50, 50, 50));
}

#[test]
fn subtraction_saturates() {
	let rgb1: RGB = RGB::new(50, 100, 150);
	let rgb2: RGB = RGB::new(100, 150, 200);
	let result: RGB = rgb1 - rgb2;
	assert_eq!(result, RGB::new(0, 0, 0));
}