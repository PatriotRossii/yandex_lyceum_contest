pub fn gcd(mut u: u64, mut v: u64) -> u64 {
    use std::cmp::min;
    use std::mem::swap;

    // Base cases: gcd(n, 0) = gcd(0, n) = n
    if u == 0 {
        return v;
    } else if v == 0 {
        return u;
    }

    // Using identities 2 and 3:
    // gcd(2ⁱ u, 2ʲ v) = 2ᵏ gcd(u, v) with u, v odd and k = min(i, j)
    // 2ᵏ is the greatest power of two that divides both u and v
    let i = u.trailing_zeros();  u >>= i;
    let j = v.trailing_zeros();  v >>= j;
    let k = min(i, j);

    loop {
        // u and v are odd at the start of the loop
        debug_assert!(u % 2 == 1, "u = {} is even", u);
        debug_assert!(v % 2 == 1, "v = {} is even", v);

        // Swap if necessary so u <= v
        if u > v {
            swap(&mut u, &mut v);
        }

        // Using identity 4 (gcd(u, v) = gcd(|v-u|, min(u, v))
        v -= u;

        // Identity 1: gcd(u, 0) = u
        // The shift by k is necessary to add back the 2ᵏ factor that was removed before the loop
        if v == 0 {
            return u << k;
        }

        // Identity 3: gcd(u, 2ʲ v) = gcd(u, v) (u is known to be odd)
        v >>= v.trailing_zeros();
    }
}

pub fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed to read a line");
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).expect("Failed to read a line");

    let mut values = buffer.split(' ').map(|x| x.parse::<u64>().expect("Please, enter valid count of mushrooms"));
    let accumulator = values.next().unwrap();
    
    values.fold(accumulator, |acc, x| {
        gcd(acc, x)
    });
}