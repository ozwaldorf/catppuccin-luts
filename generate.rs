//# image = "0.24.6"
//# catppuccin = "1.2.1"

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
    println!("Generating catppuccin pallets and LUTs: ");

    std::fs::create_dir("./pallets").ok();
    std::fs::create_dir("./luts").ok();

    for flavor in Flavour::into_iter() {
        // Get our colors and create an image buffer
        let name = flavor.name();
        print!("{name}: pallet ...");
        stdout().flush().ok();

        let colors: Vec<Colour> = flavor.colours().into_iter().collect();
        let len = colors.len() as u32;
        let mut imgbuf = ImageBuffer::new(len, 1);

        // Iterate over the coordinates and pixels of the image
        for (pixel, color) in imgbuf.pixels_mut().zip(colors.into_iter()) {
            let (r, g, b) = color.into();
            *pixel = Rgb([r, g, b]);
        }

        // Resize the image for convenience
        let imgbuf = resize(&imgbuf, len * 20, 20, FilterType::Nearest);
        let pallet = &format!("pallets/{name}.png");
        imgbuf.save(pallet).unwrap();

        print!("\r{name}: pallet ✓ lut ...");
        stdout().flush().ok();

        // Generate the LUT from the pallet image
        let output = Command::new("sh")
            .arg("-c")
            .arg(format!("convert HALD:8 -duplicate 512 -attenuate 2 +noise Gaussian -quantize LAB +dither -remap {pallet} -evaluate-sequence Mean luts/{name}.png"))
            .output()
            .expect("failed to execute process");

        if !output.status.success() {
            panic!("magick: {}", String::from_utf8_lossy(&output.stderr));
        }

        println!("\r{name}: pallet ✓ lut ✓  ");
    }
}
