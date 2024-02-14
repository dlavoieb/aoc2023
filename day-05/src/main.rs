use anyhow::Context;
use utils::read_file;

use nom::{
    bytes::complete::take_while,
    character::complete,
    error::context,
    IResult,
};

type ParseResult<'a, T> = IResult<&'a str, T, nom::error::VerboseError<&'a str>>;
type ValueRange = std::ops::Range<u64>;

fn main() {
    let lines = read_file("src/bin/day05/input.txt");

    let seeds = parse_seeds(&lines);
    let almanac = Almanac::parse(&lines);

    let value = seeds.iter().map(|seed| almanac.plot(*seed)).min().unwrap();
    println!("{:?}",value);

    let value = seeds.chunks_exact(2).map(|chunk|{
        let start = chunk[0];
        let len = chunk[1];
        almanac.smallest_plot(start..start+len)
    }).min().context("no seed found");

    println!("{:?}",value);
}

struct Almanac {
    seed_soil_map: AlmanacMap,
    soil_fertilizer_map: AlmanacMap,
    fertilizer_water_map: AlmanacMap,
    water_light_map: AlmanacMap,
    light_temperature_map: AlmanacMap,
    temperature_humidity_map: AlmanacMap,
    humidity_location_map: AlmanacMap,
}

impl Almanac {
    fn parse(lines: &Vec<String>) -> Self {
        let seed_soil_map = build_map_from_line(&lines, "seed-to-soil map:");
        let soil_fertilizer_map = build_map_from_line(&lines, "soil-to-fertilizer map:");
        let fertilizer_water_map = build_map_from_line(&lines, "fertilizer-to-water map:");
        let water_light_map = build_map_from_line(&lines, "water-to-light map:");
        let light_temperature_map = build_map_from_line(&lines, "light-to-temperature map:");
        let temperature_humidity_map = build_map_from_line(&lines, "temperature-to-humidity map:");
        let humidity_location_map = build_map_from_line(&lines, "humidity-to-location map:");

        Almanac{
            seed_soil_map:AlmanacMap{0:seed_soil_map},
            soil_fertilizer_map:AlmanacMap{0:soil_fertilizer_map},
            fertilizer_water_map:AlmanacMap{0:fertilizer_water_map},
            water_light_map:AlmanacMap{0:water_light_map},
            light_temperature_map:AlmanacMap{0:light_temperature_map},
            temperature_humidity_map:AlmanacMap{0:temperature_humidity_map},
            humidity_location_map:AlmanacMap{0:humidity_location_map},
        }
    }

    fn plot(&self, seed: u64) -> u64 {
        let soil = Almanac::map(seed, &self.seed_soil_map.0);
        let fertilizer = Almanac::map(soil, &self.soil_fertilizer_map.0);
        let water = Almanac::map(fertilizer, &self.fertilizer_water_map.0);
        let light = Almanac::map(water, &self.water_light_map.0);
        let temp = Almanac::map(light, &self.light_temperature_map.0);
        let humidity = Almanac::map(temp, &self.temperature_humidity_map.0);
        let location = Almanac::map(humidity, &self.humidity_location_map.0);
        location
    }

    fn smallest_plot(&self, range: ValueRange) -> u64 {
        self.seed_soil_map
            .ranges(range)
            .into_iter()
            .flat_map(|range| self.soil_fertilizer_map.ranges(range).into_iter())
            .flat_map(|range| self.fertilizer_water_map.ranges(range).into_iter())
            .flat_map(|range| self.water_light_map.ranges(range).into_iter())
            .flat_map(|range| self.light_temperature_map.ranges(range).into_iter())
            .flat_map(|range| self.temperature_humidity_map.ranges(range).into_iter())
            .flat_map(|range| self.humidity_location_map.ranges(range).into_iter())
            .map(|range| range.start)
            .min()
            .expect("almanac has minimal value")
    }

    fn map(input_value: u64, almanac_mapping: &Vec<AlmanacMapping>) -> u64 {
        for mapping in almanac_mapping {
            if mapping.input_range.contains(&input_value) {
                return mapping.offset(input_value);
            }
        }
        input_value
    }
}

fn parse_seeds(lines: &Vec<String>) -> Vec<u64> {
    let seeds_line = lines.iter().find(|x| x.starts_with("seeds")).unwrap();

    seeds_line
        .strip_prefix("seeds: ").unwrap()
        .split(' ')
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<u64>().unwrap())
        .collect()
}

fn build_map_from_line(lines: &Vec<String>, start_key: &str) -> Vec<AlmanacMapping> {
    let header_line = lines.iter().position(|x| x.starts_with(start_key)).unwrap();
    let footer_line = lines[header_line..lines.len()].iter().position(|x| x.is_empty()).unwrap_or(lines.len() - header_line);

    let map_lines = &lines[header_line + 1..header_line + footer_line];

    let mut map = Vec::new();
    for line in map_lines {
        if let Ok((_, a)) = mapping(line.as_str()) {
            map.push(a);
        }
    }
    map
}
struct AlmanacMap(Vec<AlmanacMapping>);

impl AlmanacMap {
    fn ranges(&self, range: ValueRange) -> Vec<ValueRange> {
        let (start, end) = (range.start, range.end);
        let (mut ranges, remaining) =
            self.0
                .iter()
                .fold((Vec::new(), start), |(mut acc, mut left), entry| {
                    // values before the current range
                    if left < entry.input_range.start {
                        // clip the end of this range to the beginning of the current entry
                        let cur_end = u64::min(entry.input_range.start, end);
                        // if non-empty
                        if left < cur_end {
                            acc.push(left..cur_end);
                            // and move the beginning to the right
                            left = cur_end;
                        }
                    }

                    // values in the current range
                    if entry.input_range.start <= left && left < end {
                        // move the end to the end of the current entry
                        let cur_end = u64::min(entry.input_range.end, end);
                        if left < cur_end {
                            let res = entry.offset(left)..entry.offset(cur_end - 1) + 1;
                            acc.push(res);
                            left = cur_end;
                        }
                    }
                    (acc, left)
                });

        if remaining < end {
            ranges.push(remaining..end);
        }

        ranges
    }
}

fn whitespace(input: &str) -> ParseResult<&str> {
    context("whitespace", take_while(|c| " \t".contains(c)))(input)
}

#[derive(Debug, Clone)]
struct AlmanacMapping {
    input_range: ValueRange,
    offset: i64,
}
impl AlmanacMapping {
    fn offset(&self, idx: u64) -> u64 {
        (i64::try_from(idx).expect("idx out of bound") + self.offset)
            .try_into()
            .expect("almanac value out of bound")
    }
}

fn mapping(input: &str) -> ParseResult<AlmanacMapping> {
    let (input, destination_start) = complete::u64(input)?;
    let (input, _) = whitespace(input)?;
    let (input, source_start) = complete::u64(input)?;
    let (input, _) = whitespace(input)?;
    let (input, window_length) = complete::u64(input)?;

    Ok((
        input,
        AlmanacMapping {
            input_range: source_start..source_start + window_length,
            offset: i64::try_from(destination_start).unwrap() - i64::try_from(source_start).unwrap(),
        },
    ))
}