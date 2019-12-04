fn is_password(mut d: i32) -> (bool, bool) {
    let mut is_password = false;
    let mut is_password_strict = false;

    let mut num_adjacent = 1;
    let mut last = d % 10;
    d /= 10;

    while d > 0 {
        let i = d % 10;
        d /= 10;
        if i == last {
            num_adjacent += 1;
            is_password = true;
        } else {
            if i > last {
                // should be decreasing since we are iterating in reverse order
                return (false, false);
            }
            if num_adjacent == 2 {
                is_password_strict = true;
            }
            num_adjacent = 1;
        }
        last = i;
    }
    if num_adjacent == 2 {
        is_password_strict = true;
    }

    (is_password, is_password_strict)
}

fn main() {
    let mut count1: u64 = 0;
    let mut count2: u64 = 0;
    (172930..683082).for_each(|x| {
        let (p1, p2) = is_password(x);
        if p1 {
            count1 += 1;
        }
        if p2 {
            count2 += 1;
        }
    });
    println!("Part1: {:?}", count1);
    println!("Part2: {:?}", count2);
}
