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
        let expected = 0;
        let input = "(())";
        let solution = dxex1(input);
        assert_eq!(expected, solution);
    }
}
fn main() {
    let data = std::fs::read_to_string("./data.txt").expect("no data");
    println!("part 1 = {}", dxex1(&data));
    println!("part 2 = {}", dxex2(&data));
}
