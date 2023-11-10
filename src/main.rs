mod imagemagick;
mod imageproc;

use std::time::Instant;

fn main() {
    let now = Instant::now();
    {
        let _ = imagemagick::draw_text_image("output_magick_rust.png");
    }
    println!("imagemagick: {:?}", now.elapsed());

    let now = Instant::now();
    {
        let _ = imageproc::draw_text_image("output_imageproc.png");
    }
    println!("imageproc: {:?}", now.elapsed());
}
