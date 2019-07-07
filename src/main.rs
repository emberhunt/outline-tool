extern crate image;
extern crate time;

use std::env;
use std::path::Path;

fn main() {

	let start = time::precise_time_s();

	// Get the args
	let args: Vec<String> = env::args().collect();

	// Get the image
	if args.len() == 1 {
		println!("Usage: ./rust-outline path/to/image.png [R] [G] [B]");
		return;
	}
	let path_to_file = Path::new(&args[1]);
	let file = image::open(path_to_file.to_str().unwrap());
	let img = match file.unwrap() {
		image::ImageRgba8(image) => image,
		_ => panic!("Bad image!"),
	};

	// Get the outline color, if specified
	let mut outline_color: [u8; 3] = [0,0,0];
	if args.len() == 5 {
		outline_color[0] = args[2].parse::<u8>().unwrap();
		outline_color[1] = args[3].parse::<u8>().unwrap();
		outline_color[2] = args[4].parse::<u8>().unwrap();
	}

	// Make the resized versions
	println!("Resizing the images...");
	let mut img1 = image::imageops::resize(&img, img.dimensions().0*3, img.dimensions().1*3, image::FilterType::Nearest);
	let mut img2 = image::imageops::resize(&img, img.dimensions().0*6, img.dimensions().1*6, image::FilterType::Nearest);
	let mut img3 = image::imageops::resize(&img, img.dimensions().0*12, img.dimensions().1*12, image::FilterType::Nearest);
	
	// Add the outline
	println!("Adding the outline to the {}x{} image.\n", img1.dimensions().0, img1.dimensions().1);
	let start1 = time::precise_time_s();
	add_outline(&mut img1, outline_color);
	img1.save(format!("{}_{}x{}.png", path_to_file.file_stem().unwrap().to_str().unwrap(), img1.dimensions().0, img1.dimensions().1)).unwrap();
	println!("\n\n{}x{} image generated! It took {} seconds.", img1.dimensions().0, img1.dimensions().1, time::precise_time_s()-start1);

	println!("Adding the outline to the {}x{} image.\n", img2.dimensions().0, img2.dimensions().1);
	let start2 = time::precise_time_s();
	add_outline(&mut img2, outline_color);
	img2.save(format!("{}_{}x{}.png", path_to_file.file_stem().unwrap().to_str().unwrap(), img2.dimensions().0, img2.dimensions().1)).unwrap();
	println!("\n\n{}x{} image generated! It took {} seconds.", img2.dimensions().0, img2.dimensions().1, time::precise_time_s()-start2);

	println!("Adding the outline to the {}x{} image.\n", img3.dimensions().0, img3.dimensions().1);
	let start3 = time::precise_time_s();
	add_outline(&mut img3, outline_color);
	img3.save(format!("{}_{}x{}.png", path_to_file.file_stem().unwrap().to_str().unwrap(), img3.dimensions().0, img3.dimensions().1)).unwrap();
	println!("\n\n{}x{} image generated! It took {} seconds.", img3.dimensions().0, img3.dimensions().1, time::precise_time_s()-start3);

	println!("\nTotal time taken: {} seconds", time::precise_time_s()-start);
}

fn add_outline(image: &mut image::ImageBuffer<image::Rgba<u8>, Vec<u8>>, color: [u8; 3]) {
	// Copy the image
	let original_image = image.clone();

	// Get width
	let width: u32 = ((image.dimensions().0+image.dimensions().1)/2)/27;

	let total_iterations: u64 = image.dimensions().0 as u64 * image.dimensions().1 as u64 * 4u64 * width as u64 * width as u64;

	let start = time::precise_time_s();

	let mut iterations_done: u64 = 0;

	// Iterate through the image pixel by pixel
	for x in 0..image.dimensions().0 {
		for y in 0..image.dimensions().1 {
			let message = format!("{:>12} of {:>12} iterations done ({:>5.2}%). ETA: {:>05.1} seconds.\r", 
				iterations_done,
				total_iterations,
				(iterations_done as f64 / total_iterations as f64)*100f64,
				(total_iterations as f64 - iterations_done as f64) / (iterations_done as f64 / (time::precise_time_s()-start))
			);
			iterations_done += (4*width*width) as u64;
			print!("{:<}", message);
			// If this pixel is not transparent, ignore it
			if original_image.get_pixel(x, y)[3]!=0 { continue; }
			// Change the pixel color to the outline color
			image.get_pixel_mut(x, y)[0] = color[0];
			image.get_pixel_mut(x, y)[1] = color[1];
			image.get_pixel_mut(x, y)[2] = color[2];
			// Change it's alpha
			image.get_pixel_mut(x, y)[3] = get_pixel_alpha(width as i32, &original_image, x as i32, y as i32);
		}
	}
}

fn get_pixel_alpha(width: i32, image: &image::ImageBuffer<image::Rgba<u8>, Vec<u8>>, ox: i32, oy: i32) -> u8{
	let mut alpha: f64 = 0.0;
	// Iterate through it's neighbors
	for x in -width..width+1 {
		for y in -width..width+1 {
			// Check if this neighbor pixel is not outside of the actual image
			if ox+x < image.dimensions().0 as i32 && oy+y < image.dimensions().1 as i32 && ox+x >= 0 && oy+y >= 0 {
				// If this pixel is transparent, or if it's the self pixel, ignore it
				if x+y == 0 {
					continue;
				}
				if image.get_pixel((ox+x) as u32, (oy+y) as u32)[3] == 0 {
					continue;
				}

				// Okay we can calculate how much alpha to add now
				alpha += 255_f64/( (x.pow(2)+y.pow(2)) as f64 * (1f64 - (2_f64).powi(-width) ) );
				if alpha > 255_f64 {
					return 255_u8;
				}
			}
		}
	}
	alpha.round() as u8
}