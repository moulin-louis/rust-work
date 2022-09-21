use std::io::{stdin, stdout, Write};
use rand::Rng;
use image::{ImageBuffer, Rgb};


fn get_input() -> String {
    let mut temp = String::new();
    stdin().read_line(&mut temp).unwrap();
    temp.pop();
    temp
}

fn print_line(display_txt:&str) {
    print!("{}", display_txt);
    stdout().flush().unwrap();
}

fn main() {
    // Challenge: If you're feeling really ambitious, you could delete this code
    // and use the "clap" library instead: https://docs.rs/clap/2.32.0/clap/
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        print_usage_and_exit();
    }
    let subcommand = args.remove(0);
	if args.len() == 1 {
		let outfile = args.remove(0);
		match subcommand.as_str() {
			"fractal" => {
				fractal(outfile);
				std::process::exit(0);
			}
			"generate" => {
				generate(outfile);
				std::process::exit(0);
			}
			_ => {
				print_line("gib ./mirage arg outfile pls");
			}
		}
	}
	let infile = args.remove(0);
    let outfile = args.remove(0);
    match subcommand.as_str() {
        "blur" => {
			print_line("Gib your blur lvl : ");
			let blur_lvl = get_input().parse().expect("Gib a nbr you idiot\n");
            blur(infile, outfile, blur_lvl);
        }
		"brighten" => {
			print_line("Gib your brighten lvl : ");
			let brighten_lvl = get_input().parse().expect("Gib a nbr you idiot\n");
			brighten(infile, outfile, brighten_lvl);
		}

       "rotate" => {
		print_line("Gib a rotation amout between 90, 180 and 270 : ");
		let rotate_lvl = get_input().parse().expect("Gib a nbr you idiot");
		if rotate_lvl != 90 && rotate_lvl != 180 && rotate_lvl != 270 {
			print_line("i told you between 90, 180 and 270 idiot\n");
			std::process::exit(-1);
		}
		rotate(infile, outfile, rotate_lvl);
	   }

        "invert" => { invert(infile, outfile); }

        "grayscale" => { grayscale(infile, outfile); }


        _ => { print_usage_and_exit(); }
    }
}

fn print_usage_and_exit() {
    println!("USAGE ./mirage option infile.png outfile.png");
    println!("here a list of option avaible : blur/ brighten/ rotate/ invert/ grayscale");
    std::process::exit(-1);
}

fn blur(infile: String, outfile: String, blur_lvl: f32) {
    let img = image::open(infile).expect("Failed to open INFILE.");
    let result = img.blur(blur_lvl);
	result.save(outfile).expect("Failed writing OUTFILE.");
}

fn brighten(infile: String, outfile: String, brighten_lvl: i32) {
	let img = image::open(infile).expect("cant open the file\n");
	let result = img.brighten(brighten_lvl);
	result.save(outfile).expect("cant create new image");
}

fn rotate(infile: String, outfile: String, rotate_lvl: i32) {
    let img = image::open(infile).expect("cant open image");
	let result;
	if rotate_lvl == 90 {
		result = img.rotate90();
	}
	else if rotate_lvl == 180 {
		result = img.rotate180();
	}
	else {
		result = img.rotate270();
	}
	result.save(outfile).expect("cant write new image");

}

fn invert(infile: String, outfile: String) {
	let mut img = image::open(infile).expect("cant open image");
	img.invert();
	img.save(outfile).expect("cant write new image");
}

fn grayscale(infile: String, outfile: String) {
    let img = image::open(infile).expect("cant open image");
	let result = img.grayscale();
	result.save(outfile).expect("cant write new image");
}

fn generate(outfile: String) {
	let (width, height) = (500, 500);
	let mut img:ImageBuffer<Rgb<u8>, Vec<u8>> = image::ImageBuffer::new(width, height);
	let mut rng = rand::thread_rng();
	let red:u8 = rng.gen_range(0..=255);
	let green:u8 = rng.gen_range(0..=255);
	let blue:u8 = rng.gen_range(0..=255);
	for (_x, _y , pixel) in img.enumerate_pixels_mut() {
		*pixel = image::Rgb([red, green, blue]);
	}
	img.save(outfile).expect("cant write new image");
}

// This code was adapted from https://github.com/PistonDevelopers/image
fn fractal(outfile: String) {
    let width = 800;
    let height = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    let scale_x = 3.0 / width as f32;
    let scale_y = 3.0 / height as f32;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {	
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

    imgbuf.save(outfile).unwrap();
}

// **SUPER CHALLENGE FOR LATER** - Let's face it, you don't have time for this during class.
//
// Make all of the subcommands stackable!
//
// For example, if you run:
//
//   cargo run infile.png outfile.png blur 2.5 invert rotate 180 brighten 10
//
// ...then your program would:
// - read infile.png
// - apply a blur of 2.5
// - invert the colors
// - rotate the image 180 degrees clockwise
// - brighten the image by 10
// - and write the result to outfile.png
//
// Good luck!
