use utils::read_file;

fn main() {
    let lines = read_file("day-01/src/input.txt");

    let mut numbers = Vec::new();
    for line in lines {
        numbers.push(parse_line(&line));
    }
    let sum: u32 = numbers.iter().sum();
    println!("Sum: {}", sum);
}

fn parse_line(line: &String) -> u32 {
    let first = find_first_digit(line);
    let last = find_last_digit(line);
    first * 10 + last
}

fn find_first_digit(line: &String) -> u32 {
    let mut first_char = line.find(|ch| ch >= '0' && ch <= '9').unwrap_or(line.len());
    let first = line.chars().nth(first_char).unwrap();
    let mut value = first.to_digit(10).unwrap();

    for (digit, ref_value) in get_digits() {
        if let Some(index) = line.find(digit) {
            if index < first_char {
                value = ref_value as u32;
                first_char = index;
            }
        }
    }

    value
}

fn find_last_digit(line: &String) -> u32 {
    let mut last_char = line.rfind(|ch| ch >= '0' && ch <= '9').unwrap_or(line.len());
    let last = line.chars().nth(last_char).unwrap();
    let mut value = last.to_digit(10).unwrap();

    for (digit, ref_value) in get_digits() {
        if let Some(index) = line.rfind(digit) {
            if index > last_char {
                value = ref_value as u32;
                last_char = index;
            }
        }
    }

    value
}

fn get_digits() -> Vec<(&'static str, i32)> {
    vec![("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9), ("zero", 0)]
}
