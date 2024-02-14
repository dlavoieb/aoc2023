use utils::read_file;

struct Round {
    reds: Option<u32>,
    greens: Option<u32>,
    blues: Option<u32>,
}

impl Round {
    fn new() -> Self {
        Round{
            reds: None,
            greens: None,
            blues: None,
        }
    }

    fn product(&self) -> u32{
        self.reds.unwrap_or(0) * self.greens.unwrap_or(0) * self.blues.unwrap_or(0)
    }
}

struct Game {
    id: u32,
    rounds: Vec<Round>,
}

impl Game {
    fn new(record: &String) -> Self {
        let id_end = record.find(":").expect("Cannot find id separator");
        let id = &record[5..id_end];
        let mut game = Game {id: id.parse().unwrap(), rounds: Vec::new()};

        let record = &record[id_end+1..record.len()];
        let rounds : Vec<&str> = record.split(";").collect();
        for round in rounds {
            let blocks : Vec<&str> = round.split(",").collect();
            let mut round = Round::new();

            for block in blocks {
                let parts:Vec<&str> = block.trim_start().split(" ").collect();
                match parts[1] {
                    "red" => {
                        round.reds = Some(parts[0].parse().unwrap());
                    }
                    "blue" => {
                        round.blues = Some(parts[0].parse().unwrap());
                    }
                    "green" => {
                        round.greens = Some(parts[0].parse().unwrap());
                    }
                    &_ => {}
                }
            }
            game.rounds.push(round);
        }
        game
    }
}
fn main() {
    let lines = read_file("day-02/src/input.txt");
    let mut games = Vec::new();
    for line in lines {
        games.push(Game::new(&line));
    }

    let mut sum = 0;
    let mut min_product = 0;
    for game in &games {
        if is_game_possible(game, 12, 13, 14) {
            sum = sum + game.id;
        }
        let minimums = minimums_for_rounds(game);
        min_product = min_product + minimums.product();

    }
    println!("Sum: {}", sum);
    println!("Product: {}", min_product);
}

fn is_game_possible(game: &Game, reds: u32, greens: u32, blues: u32) -> bool {
    for round in &game.rounds {
        if too_large(reds, &round.reds) || too_large(greens, &round.greens) || too_large(blues, &round.blues) {
            return false
        }
    }
    true
}

fn too_large(v: u32, option: &Option<u32>) -> bool{
    if let Some(value) = option {
        if v < *value {
            return true;
        }
    }
    false
}

fn minimums_for_rounds(game: &Game) -> Round {
    let mut minimums = Round{
        reds: Some(0),
        greens: Some(0),
        blues: Some(0),
    };

    for round in &game.rounds {
        if let Some(red) = round.reds {
            if red > minimums.reds.unwrap() {
                minimums.reds = Some(red);
            }
        }
        if let Some(green) = round.greens {
            if green > minimums.greens.unwrap() {
                minimums.greens = Some(green);
            }
        }
        if let Some(blue) = round.blues {
            if blue > minimums.blues.unwrap() {
                minimums.blues = Some(blue);
            }
        }
    }
    minimums
}
