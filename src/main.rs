use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};
use std::env;
use std::fmt;
use std::time::{Duration, Instant};

struct Elapsed(Duration);

impl Elapsed {
    fn from(start: &Instant) -> Self {
        Elapsed(start.elapsed())
    }
}

impl fmt::Display for Elapsed {
    fn fmt(&self, out: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match (self.0.as_secs(), self.0.subsec_nanos()) {
            (0, n) if n < 1000 => write!(out, "{} ns", n),
            (0, n) if n < 1_000_000 => write!(out, "{} µs", n / 1000),
            (0, n) => write!(out, "{} ms", n / 1_000_000),
            (s, n) if s < 10 => write!(out, "{}.{:02} s", s, n / 10_000_000),
            (s, _) => write!(out, "{} s", s),
        }
    }
}

fn apply_box_kernel(pixel: Rgba<u8>, img: &DynamicImage, x: u32, y: u32, ksize: u32) -> Rgba<u8> {
    let mut red: u32 = 0;
    let mut green: u32 = 0;
    let mut blue: u32 = 0;

    let kernel_size: u32 = ksize * ksize; // 5x5 box kernel
    let half_kernel: u32 = ksize / 2; // Half of the kernel size

    if x < half_kernel || x >= img.width() - half_kernel {
        return pixel;
    }
    if y < half_kernel || y >= img.height() - half_kernel {
        return pixel;
    }

    for i in (x - half_kernel)..=(x + half_kernel) {
        for j in (y - half_kernel)..=(y + half_kernel) {
            let neighbor_pixel = img.get_pixel(i, j);
            let rgba = neighbor_pixel.0;
            red += rgba[0] as u32;
            green += rgba[1] as u32;
            blue += rgba[2] as u32;
        }
    }

    let averaged_red = (red / kernel_size) as u8;
    let averaged_green = (green / kernel_size) as u8;
    let averaged_blue = (blue / kernel_size) as u8;

    Rgba([averaged_red, averaged_green, averaged_blue, pixel[3]])
}

fn handle_inputs(args: Vec<String>) -> (u32, String, String) {
    if args.len() < 4 {
        println!("-------------------------------------------------");
        println!("Please provide kernel size and filepath for input file and output");
        println!("Ex.\n./blur 5 in.jpg out.jpg");
        println!("-------------------------------------------------");
        return (0, "".to_owned(), "".to_owned());
    }
    let val: &String = &args[1];
    let val: u32 = val.trim().parse().expect("Please type a number!");
    let file_path: &String = &args[2];
    let out_path: &String = &args[3];
    (val, file_path.to_string(), out_path.to_string())
}

fn main() {
    let timer = Instant::now();
    let args: Vec<String> = env::args().collect();
    let (kernel_size, file_path, out_path) = handle_inputs(args);

    let img: DynamicImage = image::open(file_path).expect("File not found!");
    let (width, height) = img.dimensions();
    let mut output: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    for (x, y, pixel) in img.pixels() {
        output.put_pixel(x, y, apply_box_kernel(pixel, &img, x, y, kernel_size));
    }

    output.save(out_path).unwrap();
    println!("Save in {}", Elapsed::from(&timer));
}
