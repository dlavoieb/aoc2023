use utils::read_file;
use std::collections::{HashMap, HashSet};

fn main() {
    let lines = read_file("day-04/src/input.txt");

    let mut total = 0;

    let mut cards = HashMap::new();

    for line in lines {
        let card_number:&Vec<i32> = &line[5..line.find(':').unwrap()].split(' ').filter(|k| !k.is_empty()).map(|k| k.parse::<i32>().unwrap()).collect();

        let entry = cards.entry(card_number[0]).or_insert(0);
        *entry += 1;
        let number_of_cards = *entry;

        assert_eq!(card_number.len(), 1);

        let winnings = &line[line.find(':').unwrap() + 1..line.find('|').unwrap()];
        let haves = &line[line.find('|').unwrap() + 1..line.len()];

        let winnings: Vec<i32> = extract_numbers(winnings);
        let haves: Vec<i32> = extract_numbers(haves);

        let intersect_result = intersection(vec![winnings, haves]);

        if intersect_result.len() > 0 {
            for new_index in card_number[0]+1..card_number[0]+1+intersect_result.len() as i32 {
                let new_entry = cards.entry(new_index).or_insert(0);
                *new_entry += number_of_cards;
            }

            total = total + (1 << (intersect_result.len() - 1));
        }
    }
    println!("Total of points: {}", total);

    let mut total = 0;
    for (_, number) in cards {
        total = total + number;
    }
    println!("Total of cards: {}", total);

}

pub fn intersection(nums: Vec<Vec<i32>>) -> Vec<i32> {
    let mut intersect_result: Vec<i32> = nums[0].clone();

    for temp_vec in nums {
        let unique_a: HashSet<i32> = temp_vec.into_iter().collect();
        intersect_result = unique_a
            .intersection(&intersect_result.into_iter().collect())
            .map(|i| *i)
            .collect::<Vec<_>>();
    }
    intersect_result
}

fn extract_numbers(numbers: &str) -> Vec<i32> {
    numbers.split(&[' ']).filter(|k| !k.is_empty()).map(|k| k.parse::<i32>().unwrap()).collect()
}
