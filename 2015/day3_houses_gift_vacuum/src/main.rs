use std::vec;

fn d1ntl(str: &str) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let mut map: Vec<(i32, i32)> = vec![(0, 0)];
    for direction in str.chars() {
        match direction {
            '^' => y += 1,
            'v' => y -= 1,
            '>' => x += 1,
            '<' => x -= 1,
            _ => (),
        }
        let coordinate = (x, y);
        if !map.contains(&coordinate) {
            map.push(coordinate);
        }
    }
    map.len() as i32
}
fn d1ntl2(str: &str) -> i32 {
    0
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t0() {
        let expected = 2;
        let input = ">";
        let solution = d1ntl(input);
        assert_eq!(expected, solution);
    }

    #[test]
    fn t1() {
        let expected = 4;
        let input = "^>v<";
        let solution = d1ntl(input);
        assert_eq!(expected, solution);
    }
    #[test]
    fn t2() {
        let expected = 2;
        let input = "^v^v^v^v^v";
        let solution = d1ntl(input);
        assert_eq!(expected, solution);
    }
}
fn main() {
    let data = std::fs::read_to_string("./data.txt").expect("no data");
    println!("part 1 = {}", d1ntl(&data));
    println!("part 2 = {}", d1ntl2(&data));
}
