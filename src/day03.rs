use std::fs;
pub fn main(file_path: &str) {
    // let file_path = "input/test03.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // println!("{contents}");

    let map: Vec<Vec<char>> = contents.lines().map(|x| x.chars().collect()).collect();

    let mut count: usize;

    let newpos: (usize, usize) = (1, 3);
    count = do_moves(map.clone(), newpos);

    println!("{}", count);

    let newpos: Vec<(usize, usize)> = vec![(1, 1), (1, 5), (1, 7), (2, 1)];

    newpos.iter().for_each(|x| {
        count *= do_moves(map.clone(), *x);
    });

    println!("{}", count);
}

fn do_moves(map: Vec<Vec<char>>, newpos: (usize, usize)) -> usize {
    let (height, width) = (map.len(), map[0].len());
    let mut pos = (0, 0);
    let mut count = 0;
    while pos.0 < height {
        if map[pos.0][pos.1] == '#' {
            count += 1;
        }
        pos.0 += newpos.0;
        pos.1 += newpos.1;
        if pos.1 >= width {
            pos.1 -= width;
        }
    }
    count
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
