// FINAL PROJECT
//
// Create an image processing application.  Exactly what it does and how it does
// it is up to you, though I've stubbed a good amount of suggestions for you.
// Look for comments labeled **OPTION** below.
//
// Two image files are included in the project root for your convenience: dyson.png and pens.png
// Feel free to use them or provide (or generate) your own images.
//
// Don't forget to have fun and play around with the code!
//
// Documentation for the image library is here: https://docs.rs/image/0.21.0/image/
//
// NOTE 1: Image processing is very CPU-intensive.  Your program will run *noticeably* faster if you
// run it with the `--release` flag.
//
//     cargo run --release [ARG1 [ARG2]]
//
// For example:
//
//     cargo run --release blur image.png blurred.png
//
// NOTE 2: This is how you parse a number from a string (or crash with a
// message). It works with any integer or float type.
//
//     let positive_number: u32 = some_string.parse().expect("Failed to parse a number");

use clap::{App, AppSettings, Arg, SubCommand};

fn main() {
  // 1. First, you need to implement some basic command-line argument handling
  // so you can make your program do different things.  Here's a little bit
  // to get you started doing manual parsing.
  //
  // Challenge: If you're feeling really ambitious, you could delete this code
  // and use the "clap" library instead: https://docs.rs/clap/2.32.0/clap/
  let matches: clap::ArgMatches = App::new("Nick Fractal Generator (NFG)")
    .version("0.0.1")
    .author("Nicolás C. S. V. <nicolas.sabbatini.95@gmail.com>")
    .about("Generate fractals in the command line")
    .arg(
      Arg::with_name("output_file")
        .value_name("OUTPUT FILE")
        .help("Path to the output file")
        .required(true)
        .takes_value(true),
    )
    .subcommand(
      SubCommand::with_name("blur")
        .about("Blurs input image")
        .arg(
          Arg::with_name("input_file")
            .value_name("INPUT FILE")
            .help("Path to the input to blur")
            .index(1)
            .takes_value(true)
            .required(true),
        )
        .arg(
          Arg::with_name("blur_lvl")
            .value_name("BLUR LVL")
            .help("Amount of blurriness float >= 0 ")
            .index(2)
            .takes_value(true)
            .required(true),
        ),
    )
    .subcommand(
      SubCommand::with_name("brighten")
        .about("Brighten input image")
        .setting(AppSettings::AllowNegativeNumbers)
        .arg(
          Arg::with_name("input_file")
            .value_name("INPUT FILE")
            .help("Path to the input to brighten.")
            .index(1)
            .takes_value(true)
            .required(true),
        )
        .arg(
          Arg::with_name("brighten_lvl")
            .value_name("BRIGHTEN LVL")
            .help("Amount of brighten Positive numbers brighten the image. Negative numbers darken it.")
            .index(2)
            .takes_value(true)
            .required(true),
        ),
    )
    .subcommand(
      SubCommand::with_name("crop")
        .about("Crop input image")
        .arg(
          Arg::with_name("input_file")
            .value_name("INPUT FILE")
            .help("Path to the input to crop")
            .index(1)
            .takes_value(true)
            .required(true),
        )
        .arg(
          Arg::with_name("x")
            .value_name("X")
            .help("Start crop X position.")
            .index(2)
            .takes_value(true)
            .required(true),
        )
        .arg(
          Arg::with_name("y")
            .value_name("Y")
            .help("Start crop Y position.")
            .index(3)
            .takes_value(true)
            .required(true),
        )
        .arg(
          Arg::with_name("width")
            .value_name("WIDTH")
            .help("Width of the new image.")
            .index(4)
            .takes_value(true)
            .required(true),
        )
        .arg(
          Arg::with_name("height")
            .value_name("HEIGHT")
            .help("Height of the new image.")
            .index(5)
            .takes_value(true)
            .required(true),
        ),
    )
    .subcommand(
      SubCommand::with_name("rotate")
        .about("Rotate input image")
        .arg(
          Arg::with_name("input_file")
            .value_name("INPUT FILE")
            .help("Path to the input to rotate.")
            .index(1)
            .takes_value(true)
            .required(true),
        )
        .arg(
          Arg::with_name("rotate")
            .short("r")
            .value_name("90, 180, 270")
            .help("Rotate image <90, 180, 270> degrees.")
            .takes_value(true)
            .index(2)
            .required(true),
        ),
    )
    .subcommand(
      SubCommand::with_name("invert")
        .about("Invert input image")
        .arg(
          Arg::with_name("input_file")
            .value_name("INPUT FILE")
            .help("Path to the input to invert.")
            .index(1)
            .takes_value(true)
            .required(true),
        )
    )
    .subcommand(
      SubCommand::with_name("grayscale")
        .about("Grayscale input image")
        .arg(
          Arg::with_name("input_file")
            .value_name("INPUT FILE")
            .help("Path to the input to grayscale.")
            .index(1)
            .takes_value(true)
            .required(true),
        )
    )
    .get_matches();
  // Get out file
  let outfile = matches.value_of("output_file").unwrap();
  match matches.subcommand() {
    ("blur", Some(args)) => blur(outfile, args),
    ("brighten", Some(args)) => brighten(outfile, args),
    ("crop", Some(args)) => crop(outfile, args),
    ("rotate", Some(args)) => rotate(outfile, args),
    ("invert", Some(args)) => invert(outfile, args),
    ("grayscale", Some(args)) => grayscale(outfile, args),
    ("", None) => println!("Nothing to do, please enter a valid subcommand, for more help run the desired program or subcommand with the suffix --help [-h]."),
    _ => (),
  }
}

fn blur(outfile: &str, args: &clap::ArgMatches) {
  // Here's how you open an existing image file
  let input_file = args.value_of("input_file").unwrap();
  let img = image::open(input_file).expect("Failed to open INFILE.");
  // **OPTION**
  // Parse the blur amount (an f32) from the command-line and pass it through
  // to this function, instead of hard-coding it to 2.0.
  let blur_lvl: f32 = args
    .value_of("blur_lvl")
    .unwrap()
    .parse()
    .expect("Amount of blurriness float >= 0 ");
  assert!(blur_lvl >= 0.0, "Amount of blurriness float >= 0 ");
  let img2 = img.blur(blur_lvl);
  // Here's how you save an image to a file.
  img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn brighten(outfile: &str, args: &clap::ArgMatches) {
  let input_file = args.value_of("input_file").unwrap();
  let img = image::open(input_file).expect("Failed to open INFILE.");
  let brighten_lvl: i32 = args.value_of("brighten_lvl").unwrap().parse().expect(
    "Amount of brighten Positive numbers brighten the image. Negative numbers darken it ",
  );
  let img2 = img.brighten(brighten_lvl);
  // Here's how you save an image to a file.
  img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn crop(outfile: &str, args: &clap::ArgMatches) {
  let input_file = args.value_of("input_file").unwrap();
  let mut img = image::open(input_file).expect("Failed to open INFILE.");

  let x: u32 = args.value_of("x").unwrap().parse().expect(
    "Start crop X position.",
  );
  let y: u32 = args.value_of("y").unwrap().parse().expect(
    "Start crop Y position.",
  );
  let width: u32 = args.value_of("width").unwrap().parse().expect(
    "Width of the new image.",
  );
  let height: u32 = args.value_of("height").unwrap().parse().expect(
    "Height of the new image.",
  );
  let img2 = img.crop(x, y, width, height);
  // Here's how you save an image to a file.
  img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn rotate(outfile: &str, args: &clap::ArgMatches) {
  let input_file = args.value_of("input_file").unwrap();
  let mut img = image::open(input_file).expect("Failed to open INFILE.");

  let rotation = args.value_of("rotate").unwrap();

  let img2 = match rotation {
    "90" => Ok(img.rotate90()),
    "180" => Ok(img.rotate180()),
    "270" => Ok(img.rotate270()),
    _ => Err("Unespected rotation"),
  };
  img2.unwrap().save(outfile).expect("Failed writing OUTFILE.");
}

fn invert(outfile: &str, args: &clap::ArgMatches) {
  let input_file = args.value_of("input_file").unwrap();
  let mut img = image::open(input_file).expect("Failed to open INFILE.");

  img.invert();
  img.save(outfile).expect("Failed writing OUTFILE.");
}

fn grayscale(outfile: &str, args: &clap::ArgMatches) {
  let input_file = args.value_of("input_file").unwrap();
  let mut img = image::open(input_file).expect("Failed to open INFILE.");

  let img2 = img.grayscale();
  img2.save(outfile).expect("Failed writing OUTFILE.");
}

// This code was adapted from https://github.com/PistonDevelopers/image
fn fractal(outfile: String) {
  let width = 800;
  let height = 800;

  let mut image_buffer = image::ImageBuffer::new(width, height);

  let scale_x = 3.0 / width as f32;
  let scale_y = 3.0 / height as f32;

  // Iterate over the coordinates and pixels of the image
  for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
    // Use red and blue to be a pretty gradient background
    let red = (0.3 * x as f32) as u8;
    let blue = (0.3 * y as f32) as u8;

    // Use green as the fractal foreground (here is the fractal math part)
    let cx = y as f32 * scale_x - 1.5;
    let cy = x as f32 * scale_y - 1.5;

    let c = num_complex::Complex::new(-0.4, 0.6);
    let mut z = num_complex::Complex::new(cx, cy);

    let mut green = 0;
    while green < 255 && z.norm() <= 2.0 {
      z = z * z + c;
      green += 1;
    }

    // Actually set the pixel. red, green, and blue are u8 values!
    *pixel = image::Rgb([red, green, blue]);
  }

  image_buffer.save(outfile).unwrap();
}
