use std::collections::HashMap;
use std::fs;
pub fn main(file_path: &str) {
    // let file_path = "input/test07.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // println!("{contents}");
    let mut bags: Vec<(String, Vec<(String, usize)>)> = Vec::new();
    let mut gold: HashMap<String, bool> = HashMap::new();

    contents.lines().for_each(|line| {
        let mut contains: Vec<(String, usize)> = Vec::new();
        let (bag, contain) = line.split_once(" contain ").unwrap();
        if contain != "no other bags." {
            contain.split(", ").for_each(|c| {
                let (count, color) = c.split_once(" ").unwrap();
                let color = color
                    .replace(".", "")
                    .replace(" bags", "")
                    .replace(" bag", "");
                contains.push((color.to_string(), count.parse().unwrap()));
            });
        }
        bags.push((bag.replace(" bags", "").to_string(), contains));
    });

    bags.iter().for_each(|bag| {
        if bag.1.iter().any(|c| c.0 == "shiny gold") {
            gold.insert(bag.0.clone(), true);
        }
    });

    for _ in 0..bags.len() {
        bags.iter().for_each(|bag| {
            if !gold.contains_key(&bag.0) {
                bag.1.iter().for_each(|c| {
                    if gold.contains_key(&c.0) {
                        gold.insert(bag.0.clone(), true);
                    }
                });
            }
        });
    }
    let part1 = gold.len();
    println!("{part1}");

    let part2 = count_bags(&bags, "shiny gold") - 1;
    println!("{part2}");
}

fn count_bags(bags: &Vec<(String, Vec<(String, usize)>)>, bag: &str) -> usize {
    fn recurse(bags: &Vec<(String, Vec<(String, usize)>)>, bag: &str) -> usize {
        let mut count = 0;
        bags.iter().for_each(|b| {
            if b.0 == bag {
                count += 1;
                b.1.iter().for_each(|c| {
                    count += c.1 * recurse(bags, &c.0);
                });
            }
        });
        count
    }
    recurse(bags, bag)
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
