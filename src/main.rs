extern crate num;
use num::{Float, FromPrimitive};

extern crate png;
use png::HasParameters;

use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;


extern crate rayon;
use rayon::prelude::*;

const MAX_ITER: u32 = 1500;
const WIDTH: u32 = 3600*4;
const HEIGHT: u32 = 2400*4;


fn linspace<T>(start: T, stop: T, nstep: u32) -> Vec<T>
	where
		T: Float + FromPrimitive,
{
	let delta: T = (stop - start) / T::from_u32(nstep - 1).expect("out of range");
	return (0..(nstep))
		.map(|i| start + T::from_u32(i)
			.expect("out of range") * delta)
		.collect();
}

fn x_scale(x: f64, x_max : f64, x_min: f64) -> f64 {
	x * (x_max-x_min) / WIDTH as f64 + x_min
}

fn y_scale(y: f64, y_max : f64, y_min: f64) -> f64 {
	y * (y_max-y_min) / HEIGHT as f64 + y_min
}

fn mandelbrot(i: f64, j: f64, x_min: f64, y_min: f64, x_max: f64, y_max : f64) ->Vec<u8> {
	let x0 = x_scale(i, x_max, x_min);
	let y0 = y_scale(j, y_max, y_min);

	let mut x = 0.0;
	let mut y = 0.0;
	let mut t = 0;

	for _ in 0..MAX_ITER {
		let xtemp = x.powi(2) - y.powi(2) + x0;
		y = 2.0 * x * y + y0;
		x = xtemp;
		t += 1;
		if x.powi(2) + y.powi(2) > (1<<8) as f64 {
			break
		}
	}
	
	if t < MAX_ITER {
		let result_t = (t + 1) as f64 - (x.powi(2) + y.powi(2)).sqrt().log(2.0).ln();
		let c = result_t;//3.0 * result_t.ln() / (result_t - 1.0).ln();
		if c < 1.0 {
			return vec![(255.0*c).round() as u8, 0, 0];
		} else if c < 2.0 {
			return vec![0, (255.0*(c-1.0)).round() as u8, 0];
		} else {
			return vec![0, 0, (255.0*(c-2.0)).round() as u8];
			//return vec![(255.0*(c-2.0)).round() as u8, 0, 0];
			//return vec![0,(255.0*(c-2.0)).round() as u8, 0];
		}
	} else {
		return vec![0,0,0];
	}
}

fn write_img(name : &str, data: &Vec<u8>) {
	let path = Path::new(name);
	let file = match File::create(&path) {
		Err(why) => panic!("couldn't open: {}", why.description()),
		Ok(file) => file
	};
	let ref mut w = BufWriter::new(file);
	let mut encoder = png::Encoder::new(w, WIDTH, HEIGHT);
	encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
	let mut writer = encoder.write_header().unwrap();
	writer.write_image_data(&data).unwrap();
}

fn mandelbrot_space(x_min: f64, y_min: f64, x_max: f64, y_max : f64, id: usize) {
	let mut data = vec![];
	for i in 0..HEIGHT {
		for j in 0..WIDTH{
			data.append(&mut mandelbrot(j as f64, i as f64, x_min, y_min, x_max, y_max ));
		}
	}

	let name = "result/".to_owned() + &id.to_string() + ".png";
	write_img(&name, &data);
}

fn mandelbrot_par_space(x_min: f64, y_min: f64, x_max: f64, y_max : f64, id: usize) {
	let data : Vec<Vec<_>> = (0..(WIDTH*HEIGHT)).into_par_iter()
		.map(|p| mandelbrot((p%WIDTH) as f64, (p/WIDTH) as f64, x_min, y_min, x_max, y_max))
		.collect();

	let data2 = data
		.iter()
		.flat_map(|array| array.iter())
		.cloned()
		.collect();

	let name = "result/".to_owned() + &id.to_string() + ".png";
	write_img(&name, &data2);
}

fn wrapper(id: usize, in_x_min: f64, in_y_min: f64, in_x_max: f64, in_y_max: f64) {
	/*
	let x_rate = (1.0-(-2.5))/(WIDTH as f64- 0.0);
	let x_offset = -2.5 - (0.0 * x_rate);
	
	let x_min = in_x_min as f64 * x_rate + x_offset;
	let x_max = in_x_max as f64 * x_rate + x_offset;

	let y_rate = (1.0 - (-1.0))/(HEIGHT as f64- 0.0);
	let y_offset = -1.0 - (0.0 * y_rate);

	let y_min = in_y_min as f64 * y_rate + y_offset;
	let y_max = in_y_max as f64  * y_rate + y_offset;
	*/
	//mandelbrot_space(-2.5,-1.0,1.0,1.0)
	//println!("{},{},{},{},", x_min, y_min, x_max, y_max);
	//mandelbrot_space(x_min, y_min, x_max, y_max, id);
	mandelbrot_par_space(-1.25066, 0.02012, -1.25067, 0.02013, id);
}

fn main() {
	//let rng = 0..((WIDTH * HEIGHT));
	//rng.into_par_iter().enumerate().for_each(|(id,p)| wrapper(id, (p%WIDTH) as f64, (p/WIDTH) as f64, (p%WIDTH+1) as f64, (p/WIDTH+1) as f64));
	//wrapper(2usize, 2014.54, 462.54,2014.55,462.55);
	//wrapper(1 as usize,1466.0,409.0,1468.0,411.0)
	/*
	let slice_size_x =  (-1.25066+(0.00017/3.0) - (-1.25066) ) / 20.0;
	let slice_size_y = 0.02012+(0.00017/3.0) - 0.02012 /20.0;
	let rng_x = linspace(-1.25066, -1.25066+(0.00017/3.0), 20);
	let mut rng_y = linspace(0.02012, 0.02012+(0.00017/3.0), 20);
	rng_x
		.par_iter()
		.enumerate()
		.for_each(|(id,x)| rng_y
			.iter().
			for_each(|y| mandelbrot_par_space(*x, *y, *x+slice_size_x, *y + slice_size_y, id)));
	*/
	mandelbrot_par_space(-1.25066, 0.02012, -1.25066+(0.00017/3.0), 0.02012+(0.00017/3.0), 2);
}