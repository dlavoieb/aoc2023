use crate::Direction::*;

fn main() {
    let lines = utils::read_file("day-10/input.txt");
    let lines: Vec<Vec<char>> = lines.into_iter().map(|line| line.chars().collect()).collect();

    if let Some(location) = find_value(lines.clone(), 'S') {
        println!("Start point: {location:?}");

        let pipes = find_two_connections(&lines, location);
        println!("The two points: {pipes:?}");

        let mut pipe_a = pipes.0.unwrap();
        let mut pipe_b = pipes.1.unwrap();

        let mut sequence_a = Vec::new();
        let mut sequence_b = Vec::new();

        // follow each pipe until the meet at the same location
        let mut i = 1;
        while pipe_a.position() != pipe_b.position() {
            sequence_a.push(pipe_a.clone());
            let next_a = find_next_step(pipe_a.clone(), &lines);
            if next_a.is_none() {
                eprintln!("A none: {pipe_a:?}");
                break;
            }
            pipe_a = next_a.unwrap();

            sequence_b.push(pipe_a.clone());
            let next_b = find_next_step(pipe_b.clone(), &lines);
            if next_b.is_none() {
                eprintln!("A none: {pipe_b:?}");
                break;
            }
            pipe_b = next_b.unwrap();
            i += 1;
        }
        println!("Loop finished on {:?} with {} elements", pipe_a.position(), sequence_b.len() + 1)
    }
}

fn find_value<T, Outer, Inner>(my_array: Outer, target: T) -> Option<(usize, usize)>
where
    Outer: IntoIterator<Item=Inner>,
    Inner: IntoIterator<Item=T>,
    T: std::cmp::PartialEq<T>,
{
    for (row_index, row) in my_array.into_iter().enumerate() {
        for (col_index, element) in row.into_iter().enumerate() {
            if element == target {
                return Some((row_index, col_index));
            }
        }
    }
    None
}

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    North((usize, usize)),
    South((usize, usize)),
    East((usize, usize)),
    West((usize, usize)),
}

impl Direction {
    fn position(&self) -> &(usize, usize) {
        match self {
            Direction::North(position) => { position }
            Direction::South(position) => { position }
            Direction::East(position) => { position }
            Direction::West(position) => { position }
        }
    }

    fn possible_steps(&self) -> Vec<Direction> {
        let (x, y) = self.position();
        let mut all_possible=  vec![Direction::South((x + 1, *y)), Direction::East((*x, y + 1))];
        if *x > 0 {
            all_possible.push(Direction::North((x - 1, *y)))
        }
        if *y > 0 {
            all_possible.push(Direction::West((*x, y - 1)))
        }

        match self {
            Direction::North(_) => { all_possible.remove(0);}
            Direction::West(_) => { all_possible.remove(1);}
            Direction::South(_) => { if *x > 0 {all_possible.remove(2);}}
            Direction::East(_) => { if *y > 0 {if *x > 0 {all_possible.remove(3);} else {all_possible.remove(2);}}}
        }
        all_possible
    }
}

fn find_two_connections(my_array: &Vec<Vec<char>>, location: (usize, usize)) -> (Option<Direction>, Option<Direction>) {
    let mut first: Option<Direction> = None;
    let mut second: Option<Direction> = None;

    // check above if connects
    if location.0 > 0 {
        if let Some(above) = my_array.get(location.0 - 1) {
            let above = above[location.1];

            if ['|', '7', 'F'].contains(&above) {
                first = Some(Direction::North((location.0 - 1, location.1)));
            }
        }
    }

    // check below if connects
    if let Some(below) = my_array.get(location.0 + 1) {
        let below = below[location.1];

        if ['|', 'L', 'J'].contains(&below) {
            if first.is_none() {
                first = Some(Direction::South((location.0 + 1, location.1)));
            } else {
                second = Some(Direction::South((location.0 + 1, location.1)));
            }
        }
    }

    // check left if connects
    if location.1 > 0 {
        if let Some(left) = my_array[location.0].get(location.1 - 1) {
            if ['-', 'L', 'F'].contains(left) {
                if first.is_none() {
                    first = Some(Direction::West((location.0, location.1 - 1)));
                } else {
                    second = Some(Direction::West((location.0, location.1 - 1)));
                }
            }
        }
    }


    // check right if connects
    if let Some(right) = my_array[location.0].get(location.1 + 1) {
        if ['-', 'J', '7'].contains(right) {
            if first.is_none() {
                first = Some(Direction::East((location.0, location.1 + 1)));
            } else {
                second = Some(Direction::East((location.0, location.1 + 1)));
            }
        }
    }

    (first, second)
}

fn find_next_step(current_step: Direction, array: &Vec<Vec<char>>) -> Option<Direction> {
    if let Some(cell) = safe_get(array, current_step.position()) {
        match current_step {
                North((x, y)) => {
                    match cell {
                        '|' => {return Some(North((x-1, y)))}
                        'F' => {return Some(East((x, y+1)))}
                        '7' => {return Some(West((x, y-1)))}
                        _ => {unreachable!()}
                    }
                }
                South((x, y)) => {match cell {
                        '|' => {return Some(South((x+1, y)))}
                        'L' => {return Some(East((x, y+1)))}
                        'J' => {return Some(West((x, y-1)))}
                        _ => {unreachable!()}
                    }
                }
                East((x, y)) => { match cell {
                        '-' => {return Some(East((x, y+1)))}
                        '7' => {return Some(South((x+1, y)))}
                        'J' => {return Some(North((x-1, y)))}
                        _ => {unreachable!()}
                    }
                }
                West((x, y)) => { match cell {
                        '-' => {return Some(West((x, y-1)))}
                        'F' => {return Some(South((x+1, y)))}
                        'L' => {return Some(North((x-1, y)))}
                        _ => {unreachable!()}
                    }
                }
            }
        }
    None
}

fn safe_get(array: &Vec<Vec<char>>, location: &(usize, usize)) -> Option<char> {
    if let Some(row) = array.get(location.0) {
        if let Some(cell) = row.get(location.1) {
            return Some(*cell);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn simple_loop() -> Vec<Vec<char>> {
        vec![
            vec!['.', '.', '.', '.', '.'],
            vec!['.', 'S', '-', '7', '.'],
            vec!['.', '|', '.', '|', '.'],
            vec!['.', 'L', '-', 'J', '.'],
            vec!['.', '.', '.', '.', '.'],
        ]
    }

    fn complex_loop() -> Vec<Vec<char>> {
        vec![
            vec!['.','.','F','7','.'],
            vec!['.','F','J','|','.'],
            vec!['S','J','.','L','7'],
            vec!['|','F','-','-','J'],
            vec!['L','J','.','.','.'],
        ]
    }

    #[test]
    fn find_start() {
        let maze = simple_loop();
        assert_eq!(find_value(maze, 'S'), Some((1,1)));
    }

    #[test]
    fn find_pipes() {
        let maze = simple_loop();
        let start = (1,1);
        assert_eq!(find_two_connections(&maze, start), (Some(Direction::South((2, 1))), Some(Direction::East((1, 2)))));


        let maze = complex_loop();
        let start = (2,0);
        assert_eq!(find_two_connections(&maze, start), (Some(Direction::South((3, 0))), Some(Direction::East((2, 1)))));
    }

    #[test]
    fn find_possible() {
        let maze = simple_loop();
        let start = find_value(&maze, &'S').unwrap();
        let pipes = find_two_connections(&maze, start);
        let mut pipe_a = pipes.0.unwrap();
        let mut pipe_b = pipes.1.unwrap();
        while pipe_a.position() != pipe_b.position() {
            pipe_a = find_next_step(pipe_a, &maze).unwrap();
        }
    }

    #[test]
    fn find_possible_complex() {
        let maze = complex_loop();
        let start = find_value(&maze, &'S').unwrap();
        let pipes = find_two_connections(&maze, start);
        let mut pipe_a = pipes.0.unwrap();
        let mut pipe_b = pipes.1.unwrap();
        while pipe_a.position() != pipe_b.position() {
            let possible_a = pipe_a.possible_steps();
            pipe_a = find_next_step(pipe_a, &maze).unwrap();
        }
    }
}