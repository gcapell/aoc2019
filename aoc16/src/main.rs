fn main() {
    let s = "59781998462438675006185496762485925436970503472751174459080326994618036736403094024111488348676644802419244196591075975610084280308059415695059918368911890852851760032000543205724091764633390765212561307082338287866715489545069566330303873914343745198297391838950197434577938472242535458546669655890258618400619467693925185601880453581947475741536786956920286681271937042394272034410161080365044440682830248774547018223347551308590698989219880430394446893636437913072055636558787182933357009123440661477321673973877875974028654688639313502382365854245311641198762520478010015968789202270746880399268251176490599427469385384364675153461448007234636949";
    let t = s.repeat(10000);
    println!("{:?}", fft100(&t));
}

fn fft100(s: &str) -> Vec<i32> {
	println!("fft100 {}", s.len());
    let ranges = ranges_to_size(s.len());
    	println!("ranges");

    let mut src = unpack(s);
    	println!("src");
    let mut dst = src.clone();
    	println!("dst");

    for _ in 0..50 {
        fft(&src, &mut dst, &ranges);
        println!("+");
        fft(&dst, &mut src, &ranges);
        println!(".");
    }

    let mut v = Vec::new();
    v.extend_from_slice(&src[..8]);
    v
}

type RangeVec = Vec<(usize, usize)>;

fn ranges_to_size(size: usize) -> Vec<(RangeVec, RangeVec)> {
    (0..size)
        .map(|r| {
            let i = r + 1;
            (ranges(i, i - 1, size), ranges(i, 3 * i - 1, size))
        })
        .collect()
}

fn ranges(delta: usize, mut s: usize, e: usize) -> RangeVec {
    let mut v = Vec::new();
    while s + delta < e {
        v.push((s, s + delta));
        s += delta * 4;
    }
    if s < e {
        v.push((s, e));
    }
    v
}

fn unpack(s: &str) -> Vec<i32> {
    s.bytes().map(|c| (c - '0' as u8) as i32).collect()
}

fn fft(src: &[i32], dst: &mut [i32], ranges: &Vec<(RangeVec, RangeVec)>) {
    for j in 0..src.len() {
    if j%10000 == 0 {
    	println!("fft {}", j);
    	}
        let (adds, dels) = &ranges[j];
        let n = sum(src, adds) - sum(src, dels);
        dst[j] = n.abs() % 10;
    }
}

fn sum(s: &[i32], rs: &RangeVec) -> i32 {
    rs.iter().map(|t| s[t.0..t.1].iter().sum::<i32>()).sum()
}

#[test]
fn test_fft() {
    let seq = &["12345678", "48226158", "34040438", "03415518", "01029498"];

    let ranges = ranges_to_size(seq[0].len());
    println!("{:?}", ranges);

    for (a, b) in seq.iter().zip(seq.iter().skip(1)) {
        let src = unpack(a);
        let mut dst = src.clone();
        let expected = unpack(b);
        fft(&src, &mut dst, &ranges);
        assert_eq!(expected, dst);
        println!("{}, {}", a, b);
    }
    //assert!(false, "oops");
}

#[test]
fn test100() {
    let data = &[
        ("80871224585914546619083218645595", "24176176"),
        ("19617804207202209144916044189917", "73745418"),
        ("69317163492948606335995924319873", "52432133"),
    ];

    for (orig, first8) in data {
        assert_eq!(unpack(first8), fft100(orig));
    }
}

