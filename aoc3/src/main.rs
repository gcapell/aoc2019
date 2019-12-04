use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::fmt;

fn main() -> std::io::Result<()> {
	let mut b = BufReader::new(File::open("input.txt")?);
	
	let mut line = String::new();
	b.read_line(&mut line)?;
	println!("First {}", line);
	let seg1:Vec<Movement> = line.split(',').map(direction).collect();
	println!("vec {}", seg1);
	

	Ok(())
}

#[derive(Debug)]
enum Dir {
    U,
    D,
	L,
	R,
}

impl fmt::Display for Movement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let d = match &self.d {
             U=>'U',
			 D=>'D',
			 L=>'L',
			 R=>'R',
        };
        write!(f, "xx{}{}", d, self.l)
    }
}

#[derive(Debug)]
struct Movement {
	d:Dir,
	l:u32,
}



fn direction(s: &str) -> Movement {
	if let Some(c) = s.chars().nth(0) {
		let d = match c {
		'U'=>Dir::U,
		'D'=>Dir::D,
		'L'=>Dir::L,
		'R'=>Dir::R,
		_ => panic!("bad direction")
	   };
	   Movement{d:d,l:0}
	} else {
		panic!("bad str")
	}
}