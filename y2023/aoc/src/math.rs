pub fn least_common_multiple(numbers: Vec<usize>) -> usize {
    let mut lcm = numbers[0];

    for i in 1..numbers.len() {
        lcm = lcm * numbers[i] / greatest_common_divisor(lcm, numbers[i]);
    }

    lcm
}

pub fn greatest_common_divisor(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }

    greatest_common_divisor(b, a % b)
}
