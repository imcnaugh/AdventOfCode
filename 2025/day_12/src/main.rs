use crate::Error::ParsingError;
use std::collections::HashMap;

fn main() {
    let input = include_str!("../resource/input.txt");
    println!("Part 1: {}", part_1(input));
}

fn part_1(input: &str) -> usize {
    let mut shapes: HashMap<u8, Shape> = HashMap::new();
    let mut areas: Vec<Area> = Vec::new();

    input.split("\n\n").for_each(|s| match Shape::try_from(s) {
        Ok(shape) => {
            shapes.insert(shape.id, shape);
            ()
        }
        Err(_) => {
            s.lines()
                .map(|s| Area::try_from(s))
                .for_each(|a| areas.push(a.unwrap()));
        }
    });

    let totals = areas.iter().fold((0, 0), |acc, area| {
        let shape_areas = area
            .shapes_to_fit
            .iter()
            .map(|(id, count)| (shapes.get(id).unwrap().area as usize) * (*count as usize))
            .sum::<usize>();
        if shape_areas <= area.area() {
            (acc.0 + 1, acc.1)
        } else {
            (acc.0, acc.1 + 1)
        }
    });

    totals.0
}

#[derive(Debug)]
struct Shape {
    id: u8,
    width: u8,
    height: u8,
    area: u8,
}

#[derive(Debug)]
struct Area {
    width: u8,
    height: u8,
    shapes_to_fit: Vec<(u8, u8)>,
}

impl TryFrom<&str> for Shape {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut lines = value.lines();
        let id = lines
            .next()
            .ok_or(ParsingError)?
            .split(":")
            .next()
            .ok_or(ParsingError)?
            .parse::<u8>()
            .map_err(|_| ParsingError)?;

        let mut width = 0;
        let mut height = 0;
        let mut area = 0;
        for line in lines {
            width = width.max(line.chars().count() as u8);
            height += 1;
            area += line.chars().filter(|&c| c == '#').count() as u8;
        }

        Ok(Self {
            id,
            width,
            height,
            area,
        })
    }
}

impl TryFrom<&str> for Area {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parts = value.split(":");
        let mut width_height = parts.next().ok_or(ParsingError)?.split("x");

        let width = width_height
            .next()
            .ok_or(ParsingError)?
            .parse::<u8>()
            .map_err(|_| ParsingError)?;
        let height = width_height
            .next()
            .ok_or(ParsingError)?
            .parse::<u8>()
            .map_err(|_| ParsingError)?;

        let shapes_to_fit = parts
            .next()
            .ok_or(ParsingError)?
            .trim()
            .split(" ")
            .enumerate()
            .map(|(id, count)| (id as u8, count.parse::<u8>().unwrap()))
            .collect();

        Ok(Self {
            width,
            height,
            shapes_to_fit,
        })
    }
}

impl Area {
    fn area(&self) -> usize {
        self.width as usize * self.height as usize
    }
}

#[derive(Debug)]
enum Error {
    ParsingError,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("../resource/test.txt");
        assert_eq!(part_1(input), 2);
    }
}
