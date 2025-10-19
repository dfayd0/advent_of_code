use std::fmt::format;

fn dxex1(str: &str) -> i32 {
    let mut suffix = 0;
    loop {
        let s = str.to_owned();
        let data = s + &suffix.to_string();
        let digest = md5::compute(data);
        println!("{}: {:?}", &suffix, &digest);
        let digest = format!("{:?}", digest);
        if digest.starts_with("00000") {
            return suffix;
        }
        suffix += 1;
    }
}
fn dxex2(str: &str) -> i32 {
    0
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t0() {
        let expected = 609043;
        let input = "abcdef";
        let solution = dxex1(input);
        assert_eq!(expected, solution);
    }
    #[test]
    fn t1() {
        let expected = 1048970;
        let input = "pqrstuv";
        let solution = dxex1(input);
        assert_eq!(expected, solution);
    }
}
fn main() {
    let data = "iwrupvqb";
    println!("part 1 = {}", dxex1(&data));
    println!("part 2 = {}", dxex2(&data));
}
