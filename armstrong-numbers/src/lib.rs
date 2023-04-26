pub fn is_armstrong_number(num: u64) -> bool {
    let mut tmp = num;
    let mut vec = vec![];
    let mut mi = 0;

    loop {
        if tmp == 0 {
            break;
        }
        mi += 1;
        vec.push(tmp % 10);
        tmp /= 10;
    }

    let res = vec.iter().map(|x| x.pow(mi)).sum();

    num == res
}
