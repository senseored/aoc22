use std::fs;
pub fn main(file_path: &str) {
    // let file_path = "input/test05.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // println!("{contents}");

    let mut seats: Vec<i32> = Vec::new();
    contents.lines().for_each(|line| {
        let (mut rmin, mut rmax, mut cmin, mut cmax) = (0, 127, 0, 7);
        line.chars().enumerate().for_each(|(i, c)| {
            if i < 7 {
                if c == 'F' {
                    rmax = ((rmax - rmin) + 1) / 2 + rmin;
                } else {
                    rmin = ((rmax - rmin) + 1) / 2 + rmin;
                }
            } else if i >= 7 {
                if c == 'L' {
                    cmax = ((cmax - cmin) + 1) / 2 + cmin;
                } else {
                    cmin = ((cmax - cmin) + 1) / 2 + cmin;
                }
            }
        });
        seats.push(rmin * 8 + cmin);
    });
    let maxseat = seats.iter().max().unwrap();
    println!("part1: {}", maxseat);

    for i in 0..*maxseat {
        if !seats.contains(&i) && seats.contains(&(i + 1)) && seats.contains(&(i - 1)) {
            println!("part2: {}", i);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let file_path = "input/test00.txt";

        let contents =
            fs::read_to_string(file_path).expect("Should have been able to read the file");
        println!("{contents}");

        assert_eq!(0, 0);
    }
}
