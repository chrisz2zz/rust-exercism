pub fn nth(n: u32) -> u32 {
    let mut v = vec![2, 3, 5, 7, 11, 13];
    if n+1 > v.len() as u32 {
        for i in 14..1_000_000_000_000_u64 {
            let mut f = false;
            for j in 2..=((i as f64).sqrt())as u64 {
                if i % j == 0 {
                    f = true;
                }
            }

            if !f {
                v.push(i);
            }

            if n + 1 == v.len() as u32 {
                break;
            }
        }
    } else {
        return v[n as usize] as u32;
    }


    v.pop().unwrap() as u32
}
