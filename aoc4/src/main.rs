fn main() {
    println!("{}", (382345..=843167i32).filter(valid).count());
}

fn valid(orig_n: &i32) -> bool {
    let mut n = *orig_n;
    let mut dup_count = 0;
    let mut good_dup = false;
    let mut prev = 0;
    let mut first = true;

    while n > 0 {
        let digit = n % 10;
        n = n / 10;
        if first {
            first = false;
        } else {
            if digit == prev {
                dup_count += 1;
            } else {
                if dup_count == 1 {
                    good_dup = true
                }
                dup_count = 0;
            }
            if digit > prev {
                return false;
            }
        }
        prev = digit;
    }

    return dup_count == 1 || good_dup;
}
