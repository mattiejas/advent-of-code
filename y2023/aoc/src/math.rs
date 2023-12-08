pub fn least_common_multiple(numbers: Vec<usize>) -> usize {
    let mut lcm = numbers[0];

    for i in 1..numbers.len() {
        lcm = lcm * numbers[i] / greatest_common_divisor(lcm, numbers[i]);
    }

    lcm
}

pub fn greatest_common_divisor(a: usize, b: usize) -> usize {
    let mut max = a;
    let mut min = b;

    if a < b {
        max = b;
        min = a;
    }

    let mut remainder = max % min;

    while remainder != 0 {
        max = min;
        min = remainder;
        remainder = max % min;
    }

    min
}
