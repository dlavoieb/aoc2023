use std::collections::HashSet;
use std::fs;
use itertools::Itertools;

type Coordinate = (usize, usize);
#[derive(Debug)]
struct Map{
    inner: Vec<Coordinate>,
    width: usize,
    height: usize,
}


fn main() {
    let content = fs::read_to_string("day-11/input.txt").unwrap();

    let part_1 = parse(content.as_str()).expand(2).distance_sum();
    let part_2 = parse(content.as_str()).expand(1000000).distance_sum();
    println!("part1: {part_1}");
    println!("part2: {part_2}");
}

fn parse(content: &str) -> Map {
    let width = content.lines().next().unwrap().len();
    let height = content.lines().count();

    let inner: Vec<Coordinate> = content.lines().enumerate().flat_map(|(y, line)| line.char_indices().filter(|(_, c)| *c == '#').map(move |(x, _)| (x, y))).collect();
    
    Map{ inner, height, width }
}

impl Map {
    fn expand(mut self, rate: usize) -> Self {
        let xs = self.inner.iter().map(|(x, _)| *x).collect::<HashSet<_>>();
        let ys = self.inner.iter().map(|(_, y)| *y).collect::<HashSet<_>>();
        let rate = rate - 1;

        let mut shift = 0;
        for x in 0..self.width {
            if !xs.contains(&x) {
                for galaxy in &mut self.inner {
                    if galaxy.0 > x + shift {
                        galaxy.0 += rate;
                    }
                }
                shift += rate;
            }
        }

        let mut shift = 0;
        for y in 0..self.height {
            if !ys.contains(&y) {
                // galaxies are laid in rows of increasing y, so they’re effectively sorted by y
                let Err(first) = self.inner.binary_search_by_key(&(y + shift), |(_, galaxy_y)| *galaxy_y)
                else {
                    unreachable!("needle guaranteed to not be in haystack")
                };
                for galaxy in &mut self.inner[first..] {
                    galaxy.1 += rate;
                }
                shift += rate;
            }
        }

        self
    }

    fn distance_sum(self) -> isize {
        self.inner.iter().combinations(2).map(|pair| {
            ((pair[0].0 as isize - pair[1].0 as isize).abs() +
                (pair[0].1 as isize - pair[1].1 as isize).abs())
        }).sum()
    }
}