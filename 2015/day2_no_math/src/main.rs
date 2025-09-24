use std::cmp;
struct Dim {
    l: u32,
    w: u32,
    h: u32,
}

impl Dim {
    fn smallest(&self) -> u32 {
        let a = self.l * self.w;
        let b = self.w * self.h;
        let c = self.h * self.l;
        cmp::min(a, cmp::min(b, c))
    }
    fn ribbon(&self) -> u32 {
        let mut a = vec![self.l, self.w, self.h];
        a.sort();
        let b = a[0];
        let c = a[1];

        2 * b + 2 * c + self.l * self.w * self.h
    }
}

fn d2_problem(str: &str) -> i32 {
    let mut r = 0;
    let data: Vec<&str> = str.split('\n').collect();
    for d in data.iter() {
        if d.trim().is_empty() {
            continue;
        }
        let nums: Vec<u32> =
            d.trim().split('x').map(|v| v.parse().unwrap()).collect();
        let d = Dim { l: nums[0], w: nums[1], h: nums[2] };
        r += 2 * d.l * d.w + 2 * d.w * d.h + 2 * d.h * d.l + d.smallest();
    }
    r as i32
}

fn d2_problem2(str: &str) -> i32 {
    let mut r = 0;
    let data: Vec<&str> = str.split('\n').collect();
    for d in data.iter() {
        // dbg!(&d);
        if d.trim().is_empty() {
            continue;
        }
        let nums: Vec<u32> =
            d.trim().split('x').map(|v| v.parse().unwrap()).collect();
        let d = Dim { l: nums[0], w: nums[1], h: nums[2] };
        r += d.ribbon()
    }
    r as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t0() {
        let expected = 58;
        let input = "2x3x4";
        let solution = d2_problem(input);
        assert_eq!(expected, solution);
    }
    #[test]
    fn t1() {
        let expected = 43;
        let input = "1x1x10";
        let solution = d2_problem(input);
        assert_eq!(expected, solution);
    }
    #[test]
    fn t2() {
        let expected = 34;
        let input = "2x3x4";
        let solution = d2_problem2(input);
        assert_eq!(expected, solution);
    }
    #[test]
    fn t3() {
        let expected = 14;
        let input = "1x1x10";
        let solution = d2_problem2(input);
        assert_eq!(expected, solution);
    }
}
fn main() {
    let data = std::fs::read_to_string("./data.txt").expect("no data");
    println!("part 1 = {}", d2_problem(&data));
    println!("part 2 = {}", d2_problem2(&data));
}
