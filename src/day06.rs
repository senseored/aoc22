use std::fs;
pub fn main(file_path: &str) {
    // let file_path = "input/test06.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // println!("{contents}");

    let mut part1 = 0;
    let mut part2 = 0;
    contents.split("\n\n").for_each(|group| {
        let mut char: Vec<char> = Vec::new();
        group.chars().for_each(|c| {
            if !char.contains(&c) && c != '\n' {
                char.push(c);
                part1 += 1;
            }
        });

        let mut char: Vec<(char, usize)> = Vec::new();
        group.lines().enumerate().for_each(|(i, line)| {
            if i == 0 {
                line.chars().for_each(|c| {
                    char.push((c, 1));
                });
            } else {
                char.iter_mut().for_each(|c| {
                    if line.contains(c.0) {
                        c.1 += 1;
                    }
                });
            }
        });
        char.iter().for_each(|c| {
            if c.1 == group.lines().count() {
                part2 += 1;
            }
        });

        // part1 += chars.len();
    });

    println!("part 1: {part1}");
    println!("part 2: {part2}");
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
