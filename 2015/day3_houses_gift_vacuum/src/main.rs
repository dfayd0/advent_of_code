use std::vec;

fn d3ex1(str: &str) -> i32 {
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
#[derive(PartialEq, Clone, Copy)]
struct Position(i32, i32);
fn d3ex2(str: &str) -> i32 {
    let mut santa_pos = Position(0, 0);
    let mut robot_pos = Position(0, 0);
    let mut map: Vec<Position> = vec![santa_pos];
    for (index, direction) in str.chars().enumerate() {
        fn new_direction(direction: char, current_position: &Position) -> Position {
            match direction {
                '^' => Position(current_position.0, current_position.1 + 1),
                'v' => Position(current_position.0, current_position.1 - 1),
                '>' => Position(current_position.0 + 1, current_position.1),
                '<' => Position(current_position.0 - 1, current_position.1),
                _ => Position(current_position.0, current_position.1),
            }
        }
        let coordinate = if index % 2 == 0 {
            santa_pos = new_direction(direction, &santa_pos);
            santa_pos
        } else {
            robot_pos = new_direction(direction, &robot_pos);
            robot_pos
        };
        if !map.contains(&coordinate) {
            map.push(coordinate);
        }
    }
    map.len() as i32
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t0() {
        let expected = 2;
        let input = ">";
        let solution = d3ex1(input);
        assert_eq!(expected, solution);
    }

    #[test]
    fn t1() {
        let expected = 4;
        let input = "^>v<";
        let solution = d3ex1(input);
        assert_eq!(expected, solution);
    }
    #[test]
    fn t2() {
        let expected = 2;
        let input = "^v^v^v^v^v";
        let solution = d3ex1(input);
        assert_eq!(expected, solution);
    }
    #[test]
    fn t3() {
        let expected = 3;
        let input = "^v";
        let solution = d3ex2(input);
        assert_eq!(expected, solution);
    }
    #[test]
    fn t4() {
        let expected = 3;
        let input = "^>v<";
        let solution = d3ex2(input);
        assert_eq!(expected, solution);
    }
    #[test]
    fn t5() {
        let expected = 11;
        let input = "^v^v^v^v^v";
        let solution = d3ex2(input);
        assert_eq!(expected, solution);
    }
}
fn main() {
    let data = std::fs::read_to_string("./data.txt").expect("no data");
    println!("{}", data.len());
    println!("part 1 = {}", d3ex1(&data));
    println!("part 2 = {}", d3ex2(&data));
}
