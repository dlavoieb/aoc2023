use std::str::FromStr;

fn main() {
    part1();
    part2();
}

fn part1() {
    let lines = utils::read_file("day-09/input.txt");
    let mut extrapolated_values = Vec::new();
    for line in lines {
        let line: Vec<i32> = line.split(" ").map(|c| i32::from_str(c).unwrap()).collect();
        let history = create_history(line);
        let extrapolated_value = extrapolate_line(history);
        extrapolated_values.push(extrapolated_value);
    }
    let sum: i32 = extrapolated_values.iter().sum();
    println!("Sum: {sum}");
}

fn part2() {
    let lines = utils::read_file("day-09/input.txt");
    let mut extrapolated_values = Vec::new();
    for line in lines {
        let line: Vec<i32> = line.split(" ").map(|c| i32::from_str(c).unwrap()).collect();
        let history = create_history(line);
        let extrapolated_value = extrapolate_backwards(history);
        extrapolated_values.push(extrapolated_value);
    }
    let sum: i32 = extrapolated_values.iter().sum();
    println!("Sum: {sum}");
}

fn create_history(mut line: Vec<i32>) -> Vec<Vec<i32>> {
    let mut history = Vec::new();
    while line.iter().find(|&&d| d != 0).is_some() {
        history.push(line.clone());
        line = diff_line(line);
    }
    history
}

fn extrapolate_line(mut history: Vec<Vec<i32>>) -> i32 {
    let mut previous_value = 0;
    for rline in history.iter_mut().rev() {
        previous_value = rline.last().unwrap() + previous_value;
        rline.push(previous_value);
    }
    let extrapolated_value = *history[0].last().unwrap();
    extrapolated_value
}

fn extrapolate_backwards(mut history: Vec<Vec<i32>>) -> i32 {
    let mut previous_value = 0;
    for rline in history.iter_mut().rev() {
        previous_value = rline.first().unwrap() - previous_value;
        rline.push(previous_value);
    }
    let extrapolated_value = *history[0].last().unwrap();
    extrapolated_value
}

fn diff_line(line: Vec<i32>) -> Vec<i32> {
    let mut peekable_iter = line.iter().peekable();
    let mut new_line = Vec::new();

    loop {
        if let Some(&current) = peekable_iter.next() {
            match peekable_iter.peek() {
                None => { break }
                Some(&&next) => {
                    new_line.push(next - current)
                }
            }
        }
    }
    new_line
}