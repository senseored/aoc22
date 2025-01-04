use std::fs;
pub fn main(file_path: &str) {
    // let file_path = "input/test01.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    solve(&contents);

    // println!("{contents}");
    // println!("{:?}", populate(&contents));
}
fn solve(contents: &str) {
    let contents2 = contents;
    let contents3 = contents;

    contents.lines().for_each(|line| {
        contents2.lines().for_each(|line2| {
            if line.parse::<i32>().unwrap() + line2.parse::<i32>().unwrap() == 2020 {
                println!("{} {}", line, line2);
                println!(
                    "{} * {} = {}",
                    line,
                    line2,
                    line.parse::<i32>().unwrap() * line2.parse::<i32>().unwrap()
                );
            }
        });
    });
    contents.lines().for_each(|line| {
        contents2.lines().for_each(|line2| {
            contents3.lines().for_each(|line3| {
                if line.parse::<i32>().unwrap()
                    + line2.parse::<i32>().unwrap()
                    + line3.parse::<i32>().unwrap()
                    == 2020
                {
                    println!("{} {} {}", line, line2, line3);
                    println!(
                        "{} * {} * {} = {}",
                        line,
                        line2,
                        line3,
                        line.parse::<i32>().unwrap()
                            * line2.parse::<i32>().unwrap()
                            * line3.parse::<i32>().unwrap()
                    );
                }
            });
        });
    });
}
// fn populate(contents: &str) -> (i32, i32) {}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn test() {
//         let file_path = "input/test01.txt";
//
//         let contents =
//             fs::read_to_string(file_path).expect("Should have been able to read the file");
//         let test = populate(&contents);
//
//         assert_eq!(test, (142, 0));
//     }
// }
