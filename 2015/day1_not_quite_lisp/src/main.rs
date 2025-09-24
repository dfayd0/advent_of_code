fn d1ntl(str: &str) -> i32 {
    let mut r = 0;
    for c in str.chars() {
        match c {
            '(' => r += 1,
            ')' => r -= 1,
            // _ => println!("unexpected stair! : {}", c),
            _ => (),
        };
    }
    r
}
fn d1ntl2(str: &str) -> i32 {
    let mut r = 0;
    for (i, c) in str.chars().enumerate() {
        match c {
            '(' => r += 1,
            ')' => r -= 1,
            // _ => println!("unexpected stair! : {}", c),
            _ => (),
        };
        if r == -1 {
            return i as i32 + 1;
        }
    }
    r
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t0() {
        let expected = 0;
        let input = "(())";
        let solution = d1ntl(input);
        assert_eq!(expected, solution);
    }
}
fn main() {
    let data = std::fs::read_to_string("./data.txt").expect("no data");
    println!("part 1 = {}", d1ntl(&data));
    println!("part 2 = {}", d1ntl2(&data));
}
