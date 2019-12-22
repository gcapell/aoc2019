use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let f = File::open("input.txt").unwrap();
    let mut pos = 2019;
    let cards = 10007;

    for line in io::BufReader::new(f).lines().map(|x| x.unwrap()) {
        let v: Vec<&str> = line.split(" ").collect();
        let tail = &v[v.len() - 2..];
        // println!("line: {:?}", tail);
        match tail[0] {
            "cut" => {
                let mut n = tail[1].parse::<i32>().unwrap();
                println!("cut {}", n);
                if n < 0 {
                    n += cards;
                }
                pos += if pos >= n { -n } else { cards - n };
            }
            "increment" => {
                let n = tail[1].parse::<i32>().unwrap();
                println!("inc {}", n);
                pos = pos * n % cards;
            }
            "new" => {
                println!("new");
                pos = cards - pos - 1;
            }
            _ => println!("unknown"),
        }
        println!("pos {}", pos)
    }
}
