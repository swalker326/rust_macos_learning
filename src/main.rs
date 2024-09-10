use std::{fs::File, io::BufWriter, path::Path};

use core_graphics::{display::CGDisplay, image::CGImage};
use image::{EncodableLayout, ImageBuffer, Rgba};

fn capture_screenshot() -> Option<CGImage> {
    let display = CGDisplay::main();
    let image = display.image()?;
    return Some(image);
}
fn save_image_to_file(image: CGImage, path: &Path) {
    let width = image.width() as u32;
    let height = image.height() as u32;
    let data = image.data();
    let bytes = data.as_bytes();
    let bytes_per_row = image.bytes_per_row();
    let mut img_buf: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(width, height);
        for (y, row) in img_buf.enumerate_rows_mut().enumerate() {
            for (x, pixel) in row.1.enumerate() {
            let idx = y * bytes_per_row + x * 4;  // account for row padding
            *pixel.2 = Rgba([bytes[idx], bytes[idx + 1], bytes[idx + 2], bytes[idx + 3]]);
        }
    }
    let file = File::create(path).expect("Failed to create file");
    let mut writer = BufWriter::new(file);
    img_buf
        .write_to(&mut writer, image::ImageFormat::Png)
        .expect("Failed to save image as PNG");
}
fn main() {
    if let Some(image) = capture_screenshot() {
        save_image_to_file(image, Path::new("screenshot.png"));
        println!("Screenshot saved as screenshot.png");
    } else {
        println!("Failed to capture screenshot");
    }
}
