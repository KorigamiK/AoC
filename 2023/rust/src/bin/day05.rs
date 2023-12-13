use std::io::{self, Read};
use std::str::{FromStr, Lines};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    println!("{}", part1(&buffer));
    Ok(())
}

fn part1(contents: &str) -> Id {
    let mut line_iter = contents.lines();
    let seeds = get_numbers_in_seed_line(&mut line_iter);
    let mappings = get_mappings(&mut line_iter);
    get_min_location_id(seeds.iter().copied(), &mappings)
}

type Id = u64;

struct MappingRule {
    dest_start_id: Id,
    src_start_id: Id,
    count: Id,
}

impl MappingRule {
    fn map(&self, input: Id) -> Option<Id> {
        if self.src_start_id <= input && self.src_start_id + self.count > input {
            Some(input + self.dest_start_id - self.src_start_id)
        } else {
            None
        }
    }
}

struct Mapping {
    mapping_rules: Vec<MappingRule>,
}

impl Mapping {
    fn map(&self, input: Id) -> Id {
        self.mapping_rules
            .iter()
            .find_map(|rule| rule.map(input))
            .unwrap_or(input)
    }
}

fn get_min_location_id(seed_iter: impl Iterator<Item = Id>, mappings: &[Mapping]) -> Id {
    seed_iter
        .map(|seed| mappings.iter().fold(seed, |id, mapping| mapping.map(id)))
        .min()
        .expect("No seed could be mapped to a location")
}

// ------------------------------------------------------------------------------------------------
// Shared parsing code
// ------------------------------------------------------------------------------------------------

fn get_numbers_in_seed_line(line_iter: &mut Lines) -> Vec<Id> {
    let seed_line = line_iter.next().unwrap();
    seed_line[7..]
        .split(' ')
        .map(|seed_str| Id::from_str(seed_str).expect("A seed was not a number"))
        .collect::<Vec<Id>>()
}

fn get_mappings(line_iter: &mut Lines) -> Vec<Mapping> {
    let mut mappings = Vec::new();
    for ln in line_iter {
        if ln.is_empty() {
            continue;
        } else if ln.contains("map:") {
            mappings.push(Mapping {
                mapping_rules: Vec::new(),
            });
        } else {
            let rule_params: Vec<Id> = ln
                .split(' ')
                .map(|id_str| Id::from_str(id_str).expect("Id's should be numeric"))
                .collect();

            assert_eq!(
                rule_params.len(),
                3,
                "Exactly 3 parameters are expected to each mapping"
            );

            mappings
                .last_mut()
                .expect("A current mapping was not available")
                .mapping_rules
                .push(MappingRule {
                    dest_start_id: rule_params[0],
                    src_start_id: rule_params[1],
                    count: rule_params[2],
                });
        }
    }
    mappings
}
