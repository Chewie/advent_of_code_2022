use std::{collections::HashSet, fmt::Display};
use rayon::prelude::*;

use fixedbitset::FixedBitSet;

use regex::Regex;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn distance(&self, other: &Point) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Clone, PartialEq)]
pub enum Cell {
    Unknown,
    Sensor,
    Beacon,
    NotBeacon,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Cell::Unknown => '.',
            Cell::Sensor => 'S',
            Cell::Beacon => 'B',
            Cell::NotBeacon => '#',
        };
        write!(f, "{}", c)
    }
}

pub struct Tunnel {
    pairs: Vec<(Point, Point)>,
    x_bound: i64,
    y_bound: i64,
}

impl Tunnel {
    pub fn parse(input: &str, x_bound: i64, y_bound: i64) -> Tunnel {
        let re = Regex::new(
            r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)",
        )
        .unwrap();
        Tunnel {
            pairs: re
                .captures_iter(input)
                .map(|cap| {
                    (
                        Point {
                            x: cap[1].parse().unwrap(),
                            y: cap[2].parse().unwrap(),
                        },
                        Point {
                            x: cap[3].parse().unwrap(),
                            y: cap[4].parse().unwrap(),
                        },
                    )
                })
                .collect(),
            x_bound,
            y_bound,
        }
    }

    pub fn number_of_not_beacons_on_line(&self, y: i64) -> usize {
        self.not_beacons_on_line(y)
            .iter()
            .filter(|p| self.pairs.iter().all(|(s, b)| *p != s && *p != b))
            .count()
    }

    fn find_beacon(&self) -> Point {
        (0..=self.y_bound)
            .into_par_iter()
            .find_map_any(|y| {
                let mut set = FixedBitSet::with_capacity(self.x_bound as usize + 1);

                for (sensor, beacon) in self.pairs.iter() {
                    let dist_to_line = (sensor.y - y).abs();
                    let range = sensor.distance(beacon) - dist_to_line;

                    if range < 0 {
                        continue;
                    }

                    let lower = num::clamp(sensor.x - range, 0, self.x_bound) as usize;
                    let higher = num::clamp(sensor.x + range, 0, self.x_bound) as usize + 1;
                    set.insert_range(lower..higher);
                }
                set.toggle_range(..);
                if !set.is_clear() {
                    Some(Point {
                        x: set.ones().next().unwrap() as i64,
                        y,
                    })
                } else {
                    None
                }
            })
            .unwrap()
    }

    pub fn tuning_frequency(&self) -> i64 {
        let beacon = self.find_beacon();
        beacon.x * 4000000 + beacon.y
    }

    fn not_beacons_on_line(&self, y: i64) -> HashSet<Point> {
        let mut set = HashSet::new();

        for (sensor, beacon) in self.pairs.iter() {
            let dist_to_line = (sensor.y - y).abs();
            let range = sensor.distance(beacon) - dist_to_line;

            for offset in -range..=range {
                let candidate = Point {
                    x: sensor.x + offset,
                    y,
                };
                set.insert(candidate);
            }
        }

        set
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn tunnel_no_beacon_number_on_line() {
        // GIVEN
        let input = indoc! {"
            Sensor at x=2, y=18: closest beacon is at x=-2, y=15
            Sensor at x=9, y=16: closest beacon is at x=10, y=16
            Sensor at x=13, y=2: closest beacon is at x=15, y=3
            Sensor at x=12, y=14: closest beacon is at x=10, y=16
            Sensor at x=10, y=20: closest beacon is at x=10, y=16
            Sensor at x=14, y=17: closest beacon is at x=10, y=16
            Sensor at x=8, y=7: closest beacon is at x=2, y=10
            Sensor at x=2, y=0: closest beacon is at x=2, y=10
            Sensor at x=0, y=11: closest beacon is at x=2, y=10
            Sensor at x=20, y=14: closest beacon is at x=25, y=17
            Sensor at x=17, y=20: closest beacon is at x=21, y=22
            Sensor at x=16, y=7: closest beacon is at x=15, y=3
            Sensor at x=14, y=3: closest beacon is at x=15, y=3
            Sensor at x=20, y=1: closest beacon is at x=15, y=3
        "};
        let tunnel = Tunnel::parse(input, 20, 20);

        // WHEN
        let result = tunnel.number_of_not_beacons_on_line(10);

        // THEN
        assert_eq!(26, result);
    }

    #[test]
    fn tunnel_tuning_frequency() {
        // GIVEN
        let input = indoc! {"
            Sensor at x=2, y=18: closest beacon is at x=-2, y=15
            Sensor at x=9, y=16: closest beacon is at x=10, y=16
            Sensor at x=13, y=2: closest beacon is at x=15, y=3
            Sensor at x=12, y=14: closest beacon is at x=10, y=16
            Sensor at x=10, y=20: closest beacon is at x=10, y=16
            Sensor at x=14, y=17: closest beacon is at x=10, y=16
            Sensor at x=8, y=7: closest beacon is at x=2, y=10
            Sensor at x=2, y=0: closest beacon is at x=2, y=10
            Sensor at x=0, y=11: closest beacon is at x=2, y=10
            Sensor at x=20, y=14: closest beacon is at x=25, y=17
            Sensor at x=17, y=20: closest beacon is at x=21, y=22
            Sensor at x=16, y=7: closest beacon is at x=15, y=3
            Sensor at x=14, y=3: closest beacon is at x=15, y=3
            Sensor at x=20, y=1: closest beacon is at x=15, y=3
        "};
        let tunnel = Tunnel::parse(input, 20, 20);

        // WHEN
        let result = tunnel.tuning_frequency();

        // THEN
        assert_eq!(56000011, result);
    }
}
