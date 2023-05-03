//# image = "0.24.6"
//# catppuccin = "1.2.1"

use std::{
    io::{stdout, Write},
    process::Command,
};

use catppuccin::{Colour, Flavour};
use image::{
    imageops::{resize, FilterType},
    ImageBuffer, Rgb,
};

fn main() {
    // Ensure directories exist
    std::fs::create_dir("./palettes").ok();
    std::fs::create_dir("./src").ok();

    println!("Generating catppuccin LUTs: ");

    for flavor in Flavour::into_iter() {
        // Get our colors and create an image buffer
        let name = flavor.name();
        print!("{name} ...");
        stdout().flush().ok();

        let colors: Vec<Colour> = flavor.colours().into_iter().collect();
        let len = colors.len() as u32;
        let mut palette = ImageBuffer::new(len, 1);

        // Place the colors one after another in the buffer
        for (pixel, color) in palette.pixels_mut().zip(colors.into_iter()) {
            let (r, g, b) = color.into();
            *pixel = Rgb([r, g, b]);
        }

        // Resize the palette for convenience
        let palette_path = &format!("palettes/{name}.png");
        resize(&palette, len * 20, 20, FilterType::Nearest)
            .save(palette_path)
            .expect("error saving pallet image");

        // Generate the LUT from the pallet image
        let output = Command::new("sh")
            .arg("-c")
            .arg(format!("convert HALD:8 -duplicate 512 -attenuate 2 +noise Gaussian -quantize LAB +dither -remap {palette_path} -evaluate-sequence Mean src/{name}.png"))
            .output()
            .expect("failed to execute process");

        if !output.status.success() {
            panic!("magick: {}", String::from_utf8_lossy(&output.stderr));
        }

        println!("\r{name} ✓  ");
    }
}
