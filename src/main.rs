use std::env;
use mandelbrot::calculation::*;
use mandelbrot::img::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!("Usage: {} <file> <pixels> <upper-left> <lower-right>", args[0]);
        eprintln!("Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20", args[0]);
        std::process::exit(1);
    }

    let bounds: (usize, usize) = parse_pair(&args[2], 'x').expect("Error parsing image dimensions");
    let upper_left = parse_complex(&args[3]).expect("Error parsing upper left corner point");
    let lower_right = parse_complex(&args[4]).expect("Error parsing lower right corner point");

    let mut px = vec![0; bounds.0 * bounds.1];

    render(&mut px, bounds, upper_left, lower_right);

    write_image(&args[1], &px, bounds).expect("Error writing .png file");
    println!("Your fractal has been generated successfully!\n");
}


