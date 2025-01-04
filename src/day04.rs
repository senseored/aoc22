use std::fs;
pub fn main(file_path: &str) {
    // let file_path = "input/test04.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // println!("{contents}");

    let mut count = 0;
    let mut count2 = 0;
    contents.split("\n\n").for_each(|x| {
        let (mut byr, mut iyr, mut eyr, mut hgt, mut hcl, mut ecl, mut pid) =
            (false, false, false, false, false, false, false);
        x.replace("\n", " ").split(" ").for_each(|y| {
            y.split(":").for_each(|z| match z {
                "byr" => byr = true,
                "iyr" => iyr = true,
                "eyr" => eyr = true,
                "hgt" => hgt = true,
                "hcl" => hcl = true,
                "ecl" => ecl = true,
                "pid" => pid = true,
                _ => {}
            });
        });
        if byr && iyr && eyr && hgt && hcl && ecl && pid {
            count += 1;
        }
    });

    contents.split("\n\n").for_each(|x| {
        println!("{}", x);
        let (mut byr, mut iyr, mut eyr, mut hgt, mut hcl, mut ecl, mut pid) =
            (false, false, false, false, false, false, false);
        x.replace("\n", " ").split(" ").for_each(|y| {
            y.split_once(":").into_iter().for_each(|(z, m)| match z {
                "byr" => byr = (1920..=2002).contains(&m.parse::<i32>().unwrap()),
                "iyr" => iyr = (2010..=2020).contains(&m.parse::<i32>().unwrap()),
                "eyr" => eyr = (2020..=2030).contains(&m.parse::<i32>().unwrap()),
                "hgt" => {
                    hgt = m.ends_with("cm")
                        && (150..=193).contains(&m[..m.len() - 2].parse().unwrap())
                        || m.ends_with("in")
                            && (59..=76).contains(&m[..m.len() - 2].parse().unwrap())
                }
                "hcl" => hcl = m.starts_with("#") && m.len() == 7,
                "ecl" => {
                    ecl = m.contains("amb")
                        || m.contains("blu")
                        || m.contains("brn")
                        || m.contains("gry")
                        || m.contains("grn")
                        || m.contains("hzl")
                        || m.contains("oth")
                }
                "pid" => pid = m.len() == 9,
                _ => {}
            });
        });
        // println!("{} {} {} {} {} {} {}", byr, iyr, eyr, hgt, hcl, ecl, pid);
        if byr && iyr && eyr && hgt && hcl && ecl && pid {
            count2 += 1;
        }
    });

    println!("part1: {}", count);
    println!("part2: {}", count2);
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
