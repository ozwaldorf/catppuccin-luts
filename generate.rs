//# image = "0.24.6"
//# catppuccin = { git = "https://github.com/ozwaldorf/catppuccin-rust" }

use std::{
    io::{stdout, Write},
    path::PathBuf,
    process::Command,
};

use catppuccin::{Colour, Flavour};
use image::{
    imageops::{resize, FilterType},
    ImageBuffer, Rgb,
};

fn main() {
    std::fs::create_dir("src").ok();

    println!("Generating catppuccin LUTs: ");

    for flavor in Flavour::into_iter() {
        let name = flavor.name();

        // Create a temporary palette from our colors if it doesn't exist
        let palette_path = &format!("/tmp/{name}.png");
        if !PathBuf::from(palette_path).exists() {
            let colors: Vec<Colour> = flavor.colours().into_iter().collect();
            let len = colors.len() as u32;
            let mut palette = ImageBuffer::new(len, 1);

            // Place the colors one after another in the buffer
            for (pixel, color) in palette.pixels_mut().zip(colors.into_iter()) {
                let (r, g, b) = color.into();
                *pixel = Rgb([r, g, b]);
            }

            // Resize the palette for convenience and save it
            resize(&palette, len * 20, 20, FilterType::Nearest)
                .save(palette_path)
                .expect("error saving pallet image");
        }

        for noise_level in 1..5 {
            let dir = &format!("src/noise_{noise_level}");
            std::fs::create_dir(dir).ok();

            print!("\r{name} ... ({noise_level}/4)");
            stdout().flush().ok();

            // Generate the LUT from the pallet image
            let output = Command::new("sh")
                .arg("-c")
                .arg(format!("convert HALD:8 -duplicate 512 -attenuate {noise_level} +noise Gaussian -quantize LAB +dither -remap {palette_path} -evaluate-sequence Mean {dir}/{name}.png"))
                .output()
                .expect("failed to execute process");

            if !output.status.success() {
                panic!("magick: {}", String::from_utf8_lossy(&output.stderr));
            }
        }

        println!("\r{name} ✓        ");
    }
}
