mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let day = &args[1];
    match day.as_str() {
        "01" => day01::main("input/day_01.txt"),
        "02" => day02::main("input/day_02.txt"),
        "03" => day03::main("input/day_03.txt"),
        "04" => day04::main("input/day_04.txt"),
        "05" => day05::main("input/day_05.txt"),
        "06" => day06::main("input/day_06.txt"),
        "07" => day07::main("input/day_07.txt"),
        "08" => day08::main("input/day_08.txt"),
        "09" => day09::main("input/day_09.txt"),
        "10" => day10::main("input/day_10.txt"),
        _ => println!("Unknown day"),
    }
}
