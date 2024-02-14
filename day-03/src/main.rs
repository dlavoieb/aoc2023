use utils::read_file;

fn main() {
    let lines = read_file("day-03/src/input.txt");

    let schematic = build_schematic(&lines);
    let mut sum = 0;

    for nums in &schematic {
        for num in nums {
            if num.is_part_number(&lines){
                sum = sum + num.value;
            }
        }
    }

    // find the stars
    let mut ratios = 0;
    let stars = find_stars(&lines);
    for star in stars {
        if let Some(gears) = star.is_gear(&schematic) {
            ratios = ratios + gears[0].value * gears[1].value;
        }
    }
    
    println!("Sum of parts: {}", sum);
    println!("Ratios of gears: {}", ratios);
}

fn build_schematic(lines: &Vec<String>) -> Vec<Vec<NumberDetails>> {
    let mut schematic = Vec::new();

    for line_number in 0..lines.len() {
        let line = &lines[line_number];
        let nums = find_numbers(line, line_number);

        schematic.push(nums);
    }
    schematic
}

struct NumberDetails {
    value: u32,
    start_index: usize,
    end_index: usize,
    line_index: usize,
}

impl NumberDetails {
    fn new(index: usize) -> Self {
        NumberDetails {
            value: 0,
            start_index: 0,
            end_index: 0,
            line_index: index,
        }
    }
    
    fn is_part_number(&self, schematic: &Vec<String>) -> bool {
        // check same line
        let current_line = schematic.get(self.line_index).unwrap();
        if self.start_index >= 1 {
            if let Some(symbol) = current_line.chars().nth(self.start_index - 1) {
                if symbol != '.' {
                    return true;
                }
            } 
        }
        if self.end_index + 1 < current_line.len() {
            if let Some(symbol) = current_line.chars().nth(self.end_index + 1) {
                if symbol != '.' {
                    return true;
                }
            }
        }
        
        // check line above
        if self.line_index >= 1 {
            if let Some(line) = schematic.get(self.line_index - 1) {
                if self.check_other_line(line) {
                    return true;
                }
            }
        }
        
        // check line below
        if self.line_index + 1 < schematic.len() {
            if let Some(line) = schematic.get(self.line_index + 1) {
                if self.check_other_line(line) {
                    return true;
                }
            }
        }
        false
    }

    fn check_other_line(&self, line: &String) -> bool {
        for index in self.start_index..self.end_index + 1 {
            let symbol = line.chars().nth(index).unwrap();
            if !symbol.is_digit(10) && symbol != '.' {
                return true;
            }
        }
        // check diagonals
        if self.start_index >= 1 {
            let symbol = line.chars().nth(self.start_index - 1).unwrap();
            if !symbol.is_digit(10) && symbol != '.' {
                return true;
            }
        }
        if self.end_index + 1 < line.len() {
            if let Some(symbol) = line.chars().nth(self.end_index + 1) {
                if !symbol.is_digit(10) && symbol != '.' {
                    return true;
                }
            }
        }
        false
    }
}

struct Star {
    line_index: usize,
    symbol_index: usize,
}

impl Star {
    fn is_gear<'a>(&'a self, schematic: &'a Vec<Vec<NumberDetails>>) -> Option<[&NumberDetails; 2]> {
        // check on the line above and below
        let mut neighbors = self.check_on_line(schematic.get(self.line_index).unwrap());
        if self.line_index > 0 {
            neighbors.append(&mut self.check_on_line(schematic.get(self.line_index - 1).unwrap()));
        }
        if self.line_index < schematic.len() {
            neighbors.append(&mut self.check_on_line(schematic.get(self.line_index + 1).unwrap()));
        }

        return if neighbors.len() == 2 {
            Some([*neighbors.get(0).unwrap(), *neighbors.get(1).unwrap()])
        } else {
            None
        }
    }

    fn check_on_line<'a>(&'a self, line: &'a Vec<NumberDetails>) -> Vec<&NumberDetails>{
        let mut neighboors = Vec::new();

        let index_before = match self.symbol_index > 0 {
            true => {Some(self.symbol_index - 1)}
            false => {None}
        };
        let index_after = self.symbol_index + 1;

        for number in line {
            if index_before.is_some() && number.start_index <= index_before.unwrap() && number.end_index >= index_before.unwrap() {
                neighboors.push(number);
            } else if number.start_index <= index_after && number.end_index >= index_after {
                neighboors.push(number);
            } else if number.start_index <= self.symbol_index && number.end_index >= self.symbol_index {
                neighboors.push(number);
            }
        }

        neighboors
    }
}

fn find_numbers(line: &String, line_number: usize) -> Vec<NumberDetails> {
    let mut num_details = Vec::new();
    
    let mut current_details = NumberDetails::new(line_number);
    let mut currently_in_number = false;
    
    for symbol_index in 0..line.len() {
        if let Some(symbol) =  line.chars().nth(symbol_index) {
            if symbol.is_digit(10) {
                current_details.value = current_details.value * 10 + symbol.to_digit(10).unwrap();
                if !currently_in_number {
                    currently_in_number = true;
                    current_details.start_index = symbol_index;
                }
            }
            else {
                if currently_in_number {
                    currently_in_number = false;
                    current_details.end_index = symbol_index - 1;
                    num_details.push(current_details);
                    current_details = NumberDetails::new(line_number);
                }
            }
        }
    }
    if currently_in_number {
        current_details.end_index = line.len()-1;
        num_details.push(current_details);
    }

    num_details
}

fn find_stars(lines: &Vec<String>) -> Vec<Star> {
    let mut stars = Vec::new();

    for line_index in 0..lines.len() {
        let line = lines.get(line_index).unwrap();
        for symbol_index in 0..line.len() {
            let symbol = line.chars().nth(symbol_index).unwrap();
            if symbol == '*' {
                stars.push(Star{line_index, symbol_index});
            }
        }
    }

    stars
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_number_in_line_simple() {
        let line = String::from("467..114..");
        let details = find_numbers(&line, 0);

        assert_eq!(details.len(), 2);
        assert_eq!(details.get(0).unwrap().value, 467);
        assert_eq!(details.get(1).unwrap().value, 114);
    }

    #[test]
    fn find_number_in_line_real() {
        let line = String::from("313.....753.....................596............*................270..../........*........................38.......836..850..914......942*215");
        let details = find_numbers(&line, 0);
        assert_eq!(details.len(), 10);
        assert_eq!(details.get(0).unwrap().value, 313);
        assert_eq!(details.get(9).unwrap().value, 215);
    }

    #[test]
    fn test_is_number() {
        let lines = test_schematic();

        assert!(NumberDetails {
            value: 592,
            start_index: 2,
            end_index: 4,
            line_index: 6,
        }.is_part_number(&lines));

        assert!(!NumberDetails {
            value: 114,
            start_index: 5,
            end_index: 7,
            line_index: 0,
        }.is_part_number(&lines));

        assert!(!NumberDetails {
            value: 58,
            start_index: 7,
            end_index: 8,
            line_index: 5,
        }.is_part_number(&lines));

        assert!(NumberDetails {
            value: 592,
            start_index: 2,
            end_index: 4,
            line_index: 6,
        }.is_part_number(&lines));
    }

    #[test]
    fn test_other_line() {
        let lines = test_schematic();

        assert!(!NumberDetails {
            value: 592,
            start_index: 2,
            end_index: 4,
            line_index: 6,
        }.check_other_line(lines.get(7).unwrap()));

        assert!(NumberDetails {
            value: 664,
            start_index: 1,
            end_index: 3,
            line_index: 9,
        }.check_other_line(lines.get(8).unwrap()));
    }

    fn test_schematic() -> Vec<String> {
        vec![String::from("467..114."),
             String::from("...*....."),
             String::from("..35..633"),
             String::from("......#.."),
             String::from("617*....."),
             String::from(".....+.58"),
             String::from("..592...."),
             String::from("......755"),
             String::from("...$.*..."),
             String::from(".664.598."), ]
    }

    #[test]
    fn test_find_star() {
        let stars = find_stars(&test_schematic());
        assert_eq!(stars.len(), 3);
    }

    #[test]
    fn test_star_check_line () {
        let schematic = build_schematic(&test_schematic());
        let star = Star{ line_index: 0, symbol_index: 3 };
        let neighbors = star.check_on_line(schematic.get(0).unwrap());
        assert_eq!(neighbors.len(), 1);
        let neighbors = star.check_on_line(schematic.get(2).unwrap());
        assert_eq!(neighbors.len(), 1);
    }

    #[test]
    fn test_star_find_gear () {
        let schematic = build_schematic(&test_schematic());
        let star = Star{ line_index: 1, symbol_index: 3 };
        let possible = star.is_gear(&schematic);
        assert!(possible.is_some());
        let [a,b] = possible.unwrap();
        assert_eq!(a.value, 467);
        assert_eq!(b.value, 35);

    }

    #[test]
    fn test_star_find_all_gears () {
        let schematic = build_schematic(&test_schematic());
        let stars = find_stars(&test_schematic());

        let mut ratios = 0;
        for star in stars {
            if let Some(gears) = star.is_gear(&schematic) {
                ratios = ratios + gears[0].value * gears[1].value;
            }
        }
        assert_eq!(ratios, 467835);

    }
}
