fn d1ntl(str: &str) -> i32 {
    0
}
fn d1ntl2(str: &str) -> i32 {
    0
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
