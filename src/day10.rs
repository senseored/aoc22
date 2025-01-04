use std::fs;
pub fn main(file_path: &str) {
    let file_path = "input/test10.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // println!("{contents}");
    let mut sum1 = 1;
    let mut sum3 = 1;

    let mut adapters: Vec<u32> = Vec::new();
    contents.lines().for_each(|line| {
        adapters.push(line.parse().unwrap());
    });

    adapters.sort();
    adapters
        .iter()
        .skip(1)
        .enumerate()
        .for_each(|(i, adapter)| {
            // println!("{i}");
            if adapter - adapters[i] == 1 {
                sum1 += 1;
            } else if adapter - adapters[i] == 3 {
                sum3 += 1;
            }
        });

    println!("Part 1: {} ({sum1} * {sum3})", sum1 * sum3);

    let mut opts: Vec<u32> = Vec::new();

    let mut falseres = 0;
    let mut trueres = 0;
    adapters.iter().enumerate().for_each(|(i, adapter)| {
        opts.push(1);
        let mut j = i + 1;
        while j < adapters.len() {
            if adapter + 3 <= adapters[j] {
                falseres += adapters.len() - j;
                // println!("{} {}", adapter, adapters[j]);
            } else {
                trueres += adapters.len() - j;
                opts[i] += 1;
            }
            j += 1;
        }
        // let mut j = adapters.len() - 1;
        // while j > i {
        //     if adapter + 3 < adapters[j] {
        //         falseres += 1;
        //         break;
        //     }
        //     j -= 1;
        // }
    });

    println!("{:?}", opts);
    let part2 = opts.iter().product::<u32>();
    // let part2 = trueres * adapters.len();
    println!("Part 2: {} ({}, {})", part2, falseres, trueres);
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
