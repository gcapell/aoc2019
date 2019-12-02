use std::io::prelude::*;
use std::fs::File;

fn main() {
	let mut s = filename_to_nums("input2.txt").unwrap();
	s[1] = 12;
	s[2] = 2;
	sim(&mut s);
	println!("after: {:?}", s[0]);	
}

fn sim(a : &mut[u64]) {
	let mut pos = 0;
	loop {
		match a[pos] {
		    1 | 2 => 
		    	if let [src1,src2,dst] = a[pos+1..pos+4] {
					let v1 = a[src1 as usize];
					let v2 = a[src2 as usize];
					let v= if a[pos]==1 {v1 + v2 } else { v1 * v2};
					a[dst as usize] = v;		
				},
		    99 => return,
		    _ => panic!("argh")
		}
		pos += 4;
	}
}

#[test]
fn test_sim() {
	let test_data = vec!(
	  (vec!(1,0,0,0,99), vec!(2,0,0,0,99)),
	  (vec!(2,3,0,3,99), vec!(2,3,0,6,99)),	  
	  (vec!(2,4,4,5,99,0), vec!(2,4,4,5,99,9801)),
	  (vec!(1,1,1,4,99,5,6,0,99), vec!(30,1,1,4,2,5,6,0,99)),	  
    );
	for (mut input, output) in test_data {
		sim(&mut input);
		assert_eq!(input, output);
	}
}

fn filename_to_nums(s: &str) -> std::io::Result<Vec<u64>> {
    let mut file = File::open(s)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
	Ok(s.split(',').map(|l| l.trim().parse::<u64>().unwrap()).collect())
}
