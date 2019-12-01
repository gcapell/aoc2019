use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

fn main() {
	let f = File::open("input.txt").unwrap();
	let b = BufReader::new(f);
	
	let sum:u64 = b.lines()
		.map(|r| r.unwrap())
		.map(|l| mapper(&l))
		.sum();
	println!("{}", sum);	
}

fn mapper(l: &str)->u64 {
	let n = l.trim().parse::<u64>().unwrap();
	rec_fuel(n)
}

fn rec_fuel(mass: u64) -> u64 {
	if mass < 9 {
		0
	} else {
		let f = fuel(mass);
		f + rec_fuel(f)
	}
}

fn fuel(mass: u64)-> u64 {mass/3 - 2}

#[test]
fn test_fuel() {
	let test_data = [ (12,2), (14,2), (1969,654), (100756,33583)];
	
	for (m,f) in test_data.iter() {
		assert_eq!(fuel(*m),*f);
	}
}
#[test]
fn test_rrec_fuel() {
	let test_data = [ (12,2), (14,2), (1969,966), (100756,50346)];
	
	for (m,f) in test_data.iter() {
		assert_eq!(rec_fuel(*m),*f);
	}
}
