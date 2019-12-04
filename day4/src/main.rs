fn reverse_digit_iter(mut d: u32) -> impl Iterator<Item = u32> {
    ::std::iter::from_fn(move || {
        if d == 0 {
            return None;
        } else {
            let n = d % 10;
            d /= 10;
            return Some(n);
        }
    })
}

fn is_decreasing(d: u32) -> bool {
    let mut it = reverse_digit_iter(d);
    let mut last = it.next().unwrap();

    for i in it {
        if i > last {
            return false;
        }
        last = i;
    }
    true
}

fn is_password(d: u32) -> (bool, bool) {
    if !is_decreasing(d) {
        return (false, false);
    }

    let mut is_password = false;
    let mut is_password_strict = false;

    let mut it = reverse_digit_iter(d);
    let mut last = it.next().unwrap();
    let mut num_adjacent = 1;

    for i in it {
        if i == last {
            num_adjacent += 1;
            is_password = true;
        } else {
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
