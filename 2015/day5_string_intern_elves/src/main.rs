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
fn twice_in_a_row(target: &str) -> bool {
    if target.len() < 2 {
        return false;
    }
    let mut prev = target.chars().nth(0).unwrap();
    for c in target.chars().skip(1) {
        if c == prev {
            return true;
        }
        prev = c;
    }
    false
}
fn forbidden(target: &str) -> bool {
    if target.contains("ab")
        || target.contains("cd")
        || target.contains("pq")
        || target.contains("xy")
    {
        return false;
    }
    true
}
fn dxex1(str: &str) -> i32 {
    let mut count = 0;
    for string in str.split("\n") {
        if has_vowels(string, 3) && twice_in_a_row(string) && forbidden(string) {
            count += 1;
        }
    }
    count
}
fn duplicate_pair(target: &str) -> bool {
    let chars = target.chars().collect::<Vec<_>>();

    for i in 0..chars.len().saturating_sub(1) {
        let pair = &chars[i..i + 2];
        for j in i + 2..chars.len().saturating_sub(1) {
            if chars[j] == pair[0] && chars[j + 1] == pair[1] {
                return true;
            }
        }
    }
    false
}

fn repeat_split(target: &str) -> bool {
    for tri in target.chars().collect::<Vec<_>>().windows(3) {
        if tri[0] == tri[2] {
            return true;
        }
    }
    false
}

fn dxex2(str: &str) -> i32 {
    let mut count = 0;
    for string in str.split("\n") {
        let string = string.trim_end();
        let dp = duplicate_pair(string);
        let rs = repeat_split(string);
        // println!("{}: {} - {}", string, dp, rs);
        if dp && rs {
            count += 1;
        }
    }
    count
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
    #[test]
    fn t5() {
        let expected = 1;
        let input = "qjhvhtzxzqqjkmpb";
        let solution = dxex2(input);
        assert_eq!(expected, solution);
    }
    #[test]
    fn t6() {
        let expected = 1;
        let input = "xxyxx";
        let solution = dxex2(input);
        assert_eq!(expected, solution);
    }
    #[test]
    fn t7() {
        let expected = 0;
        let input = "uurcxstgmygtbstg";
        let solution = dxex2(input);
        assert_eq!(expected, solution);
    }
    #[test]
    fn t8() {
        let expected = 0;
        let input = "ieodomkazucvgmuy";
        let solution = dxex2(input);
        assert_eq!(expected, solution);
    }
    #[test]
    fn t12() {
        let expected = 1;
        let input = "aaaa";
        let solution = dxex2(input);
        assert_eq!(expected, solution);
    }
}

fn main() {
    let data = std::fs::read_to_string("./data.txt").expect("no data");
    println!("part 1 = {}", dxex1(&data));
    println!("part 2 = {}", dxex2(&data));
}
