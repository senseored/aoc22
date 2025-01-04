use std::fs;
pub fn main(file_path: &str) {
    // let file_path = "input/test02.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // println!("{contents}");

    let mut count = 0;
    let mut count2 = 0;
    contents.lines().for_each(|line| {
        let (rule, password) = line.split_once(": ").unwrap();
        let (rule, character) = rule.split_once(" ").unwrap();
        let (min, max) = rule.split_once("-").unwrap();
        if password
            .chars()
            .filter(|x| x.to_string() == character)
            .count()
            >= min.parse::<usize>().unwrap()
            && password
                .chars()
                .filter(|x| x.to_string() == character)
                .count()
                <= max.parse::<usize>().unwrap()
        {
            count += 1;
        }
        if password
            .chars()
            .nth(min.parse::<usize>().unwrap() - 1)
            .unwrap()
            == character.chars().next().unwrap()
            && password
                .chars()
                .nth(max.parse::<usize>().unwrap() - 1)
                .unwrap()
                != character.chars().next().unwrap()
            || password
                .chars()
                .nth(min.parse::<usize>().unwrap() - 1)
                .unwrap()
                != character.chars().next().unwrap()
                && password
                    .chars()
                    .nth(max.parse::<usize>().unwrap() - 1)
                    .unwrap()
                    == character.chars().next().unwrap()
        {
            count2 += 1;
        }
    });
    println!("part1: {count}");
    println!("part2: {count2}");
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
