fn main() {
    let input = include_str!("../resource/input.txt");
    println!("{}", part_1(input));
    println!("{}", part_2(input));
}

fn part_1(input: &str) -> usize {
    let (ranges, mut ids) = parse_input_to_ranges_and_ids(input);
    let range_start_indexes: Vec<Id> = ranges
        .iter()
        .map(|r| Id {
            id: r.0,
            t: Some(RangeEnd::Start),
        })
        .collect();
    let range_end_indexes: Vec<Id> = ranges
        .iter()
        .map(|r| Id {
            id: r.1,
            t: Some(RangeEnd::End),
        })
        .collect();
    let ids: Vec<Id> = ids.iter().map(|i| Id { id: *i, t: None }).collect();

    let mut all_ids: Vec<Id> = [range_start_indexes, range_end_indexes, ids].concat();
    all_ids.sort_by(|a, b| {
        if a.id == b.id {
            match (&a.t, &b.t) {
                (Some(RangeEnd::Start), _) => std::cmp::Ordering::Less,
                (_, Some(RangeEnd::Start)) => std::cmp::Ordering::Greater,
                (Some(RangeEnd::End), _) => std::cmp::Ordering::Greater,
                (_, Some(RangeEnd::End)) => std::cmp::Ordering::Less,
                _ => std::cmp::Ordering::Equal,
            }
        } else {
            a.id.cmp(&b.id)
        }
    });

    let mut start_count = 0;
    let mut total = 0;

    all_ids.iter().for_each(|id| match id.t {
        Some(RangeEnd::Start) => start_count += 1,
        Some(RangeEnd::End) => start_count -= 1,
        None => {
            if start_count != 0 {
                total += 1;
            }
        }
    });

    total
}

fn part_2(input: &str) -> usize {
    let (ranges, mut ids) = parse_input_to_ranges_and_ids(input);
    let range_start_indexes: Vec<Id> = ranges
        .iter()
        .map(|r| Id {
            id: r.0,
            t: Some(RangeEnd::Start),
        })
        .collect();
    let range_end_indexes: Vec<Id> = ranges
        .iter()
        .map(|r| Id {
            id: r.1,
            t: Some(RangeEnd::End),
        })
        .collect();

    let mut all_ids: Vec<Id> = [range_start_indexes, range_end_indexes].concat();
    all_ids.sort_by(|a, b| {
        if a.id == b.id {
            match (&a.t, &b.t) {
                (Some(RangeEnd::Start), _) => std::cmp::Ordering::Less,
                (_, Some(RangeEnd::Start)) => std::cmp::Ordering::Greater,
                (Some(RangeEnd::End), _) => std::cmp::Ordering::Greater,
                (_, Some(RangeEnd::End)) => std::cmp::Ordering::Less,
                _ => std::cmp::Ordering::Equal,
            }
        } else {
            a.id.cmp(&b.id)
        }
    });

    let mut start_count = 0;
    let mut previous_start = 0;
    let mut total = 0;

    all_ids.iter().for_each(|id| match id.t {
        Some(RangeEnd::Start) => {
            if start_count == 0 {
                previous_start = id.id;
            }
            start_count += 1
        }
        Some(RangeEnd::End) => {
            start_count -= 1;
            if start_count == 0 {
                total += id.id + 1 - previous_start;
            }
        }
        _ => {}
    });

    total
}

#[derive(Clone)]
struct Id {
    id: usize,
    t: Option<RangeEnd>,
}

#[derive(Clone)]
enum RangeEnd {
    Start,
    End,
}

fn parse_input_to_ranges_and_ids(input: &str) -> (Vec<(usize, usize)>, Vec<usize>) {
    let parts = input.split("\n\n").collect::<Vec<_>>();
    let ranges = parts[0]
        .lines()
        .map(|l| {
            let (a, b) = l.split_once("-").unwrap();
            let a = a.parse::<usize>().unwrap();
            let b = b.parse::<usize>().unwrap();
            if a < b { (a, b) } else { (b, a) }
        })
        .collect::<Vec<(usize, usize)>>();
    let ids = parts[1]
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect();
    (ranges, ids)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(include_str!("../resource/test.txt")), 3);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(include_str!("../resource/test.txt")), 14);
    }
}
