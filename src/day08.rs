use std::fs;
pub fn main(file_path: &str) {
    // let file_path = "input/test08.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // println!("{contents}");

    let mut prev: Vec<i32> = Vec::new();
    let mut instructions: Vec<(String, i32)> = Vec::new();
    let (mut i, mut a, mut part2) = (0, 0, 0);

    contents.lines().for_each(|line| {
        let (op, arg) = line.split_once(" ").unwrap();
        // if arg.chars().next().unwrap() == '+' {
        if arg.starts_with('+') {
            instructions.push((
                op.to_string(),
                arg.strip_prefix('+').unwrap().parse::<i32>().unwrap(),
            ));
        } else {
            instructions.push((op.to_string(), -arg[1..].parse::<i32>().unwrap()));
        }
    });
    while !prev.contains(&i) {
        // println!("{i}, {a}");
        prev.push(i);
        match instructions[i as usize].0.as_str() {
            "acc" => (i, a) = (i + 1, acc(instructions[i as usize].1, a)),
            "jmp" => {
                let mut i2 = instructions.clone();
                i2[i as usize].0 = "nop".to_string();
                let (a2, b) = fixed(i2);
                if b {
                    part2 = a2;
                }
                i = jmp(instructions[i as usize].1, i);
            }
            "nop" => {
                let mut i2 = instructions.clone();
                i2[i as usize].0 = "jmp".to_string();
                let (a2, b) = fixed(i2);
                if b {
                    part2 = a2;
                }
                i += 1;
            }
            _ => panic!(),
        }
    }

    println!("part1: {a} - part2: {part2}");
}

fn fixed(instructions: Vec<(String, i32)>) -> (i32, bool) {
    let mut i = 0;
    let mut a = 0;
    let mut prev: Vec<i32> = Vec::new();
    while !prev.contains(&i) {
        prev.push(i);
        if i >= instructions.len() as i32 {
            println!("part2: {a}");
            return (a, true);
        }
        match instructions[i as usize].0.as_str() {
            "acc" => (i, a) = (i + 1, acc(instructions[i as usize].1, a)),
            "jmp" => i = jmp(instructions[i as usize].1, i),
            "nop" => i += 1,
            _ => panic!(),
        }
    }
    (a, false)
}

fn acc(i: i32, acc: i32) -> i32 {
    i + acc
}

fn jmp(i: i32, jmp: i32) -> i32 {
    i + jmp
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
