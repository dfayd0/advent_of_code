fn has_vowels(target: &str, nb: u8) -> bool {
    let mut count = 0;
    let vowels = "aeiou";
    for c in target.chars() {
        if vowels.contains(c) {
            count += 1;
        }
        if count == nb {
            return true;
        }
    }
    false
}
fn dxex1(str: &str) -> i32 {
    0
}
fn dxex2(str: &str) -> i32 {
    0
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t0() {
        let expected = 1;
        let input = "ugknbfddgicrmopn";
        let solution = dxex1(input);
        assert_eq!(expected, solution);
    }
    #[test]
    fn t1() {
        let expected = 1;
        let input = "aaa";
        let solution = dxex1(input);
        assert_eq!(expected, solution);
    }
    #[test]
    fn t2() {
        let expected = 0;
        let input = "jchzalrnumimnmhp";
        let solution = dxex1(input);
        assert_eq!(expected, solution);
    }
    #[test]
    fn t3() {
        let expected = 0;
        let input = "haegwjzuvuyypxyu";
        let solution = dxex1(input);
        assert_eq!(expected, solution);
    }
    #[test]
    fn t4() {
        let expected = 0;
        let input = "dvszwmarrgswjxmb";
        let solution = dxex1(input);
        assert_eq!(expected, solution);
    }
}
fn main() {
    let data = std::fs::read_to_string("./data.txt").expect("no data");
    println!("part 1 = {}", dxex1(&data));
    println!("part 2 = {}", dxex2(&data));
}
