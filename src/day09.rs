use std::fs;
pub fn main(file_path: &str) {
    // let file_path = "input/test09.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // println!("{contents}");
    let preamble = 25;
    // let preamble = 5;
    let mut part1 = 0;

    let numbers: Vec<i64> = contents
        .lines()
        .map(|l| l.parse::<i64>().unwrap())
        .collect();

    for i in preamble..numbers.len() {
        let mut found = false;
        for j in i - preamble..i {
            for k in j + 1..i {
                if numbers[j] + numbers[k] == numbers[i] {
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }
        if !found {
            part1 = numbers[i];
            println!("part1: {}", part1);
            break;
        }
    }
    let mut part2;
    for i in 0..numbers.len() {
        let mut sum = 0;
        for j in i..numbers.len() {
            sum += numbers[j];
            if sum == part1 {
                let min = numbers[i..=j].iter().min().unwrap();
                let max = numbers[i..=j].iter().max().unwrap();
                part2 = min + max;
                println!("part2: {}", part2);
                break;
            }
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
