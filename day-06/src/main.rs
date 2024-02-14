use utils::read_file;
use roots::Roots;
use roots::find_roots_quadratic;

fn main() {
    let lines = read_file("day-06/src/input.txt");

    part1(&lines);
    part2(&lines);
}

fn part2(lines: &Vec<String>) {
    let race_time = combine_and_parse(lines[0].clone());
    let distance_record = combine_and_parse(lines[1].clone());

    if let Roots::Two([a0, a1]) = find_roots_quadratic(-1f64, race_time as f64, -distance_record as f64) {
        println!("Total distance: {}" , a1.floor() - a0.floor());
    }
}

fn combine_and_parse(mut data: String) -> i64{
    data.retain(|c| !c.is_whitespace());
    let data: Vec<_> = data.split(':').filter(|k| !k.contains('T') && !k.contains('D')).map(|x| x.parse::<i64>().unwrap()).collect();
    assert_eq!(data.len(), 1);
    data[0]
}

fn part1(lines: &Vec<String>) {
    let times: Vec<_> = lines[0].split_whitespace().filter(|k| !k.contains(':')).map(|x| x.parse::<i64>().unwrap()).collect();
    let distances: Vec<_> = lines[1].split_whitespace().filter(|k| !k.contains(':')).map(|x| x.parse::<i64>().unwrap()).collect();

    let races_raw = times.iter().zip(distances.iter());

    let mut total_score = 1f64;
    for (max_time, distance_record) in races_raw {
        if let Roots::Two([a0, a1]) = find_roots_quadratic(-1f64, *max_time as f64, -*distance_record as f64) {
            total_score *= a1.floor() - a0.floor();
        }
    }
    println!("Total score: {}", total_score);
}
