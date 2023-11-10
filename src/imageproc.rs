use image::{Rgb, RgbImage};
use imageproc::drawing::{draw_filled_rect_mut, draw_text_mut, text_size};
use imageproc::rect::Rect;
use rusttype::{Font, Scale};

pub fn draw_text_image(file_name: &str) {
    let mut image = RgbImage::new(100, 16);

    let rect = Rect::at(0, 0).of_size(100, 16);
    let color = Rgb([226u8, 226u8, 226u8]);
    draw_filled_rect_mut(&mut image, rect, color);

    let font = Vec::from(include_bytes!("DejaVuSans.ttf") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();

    let height = 12.4;
    let scale = Scale {
        x: height,
        y: height,
    };

    let text = "Hello, world!";
    draw_text_mut(&mut image, Rgb([0u8, 0u8, 0u8]), 12, 2, scale, &font, text);
    let (_w, _h) = text_size(scale, &font, text);

    let _ = image.save(file_name).unwrap();
}
