use std::{
    cell::RefCell,
    cmp::min,
    collections::{BTreeMap, HashSet},
    ops::{Index, IndexMut},
};

use itertools::Itertools;

use regex::Regex;

#[derive(Debug)]
struct NamedValve {
    flow_rate: usize,
    tunnels: Vec<String>,
}

struct Valve {
    flow_rate: usize,
    tunnels: Vec<usize>,
}

impl Valve {
    fn from_named_valves(named_valves: BTreeMap<String, NamedValve>) -> Vec<Valve> {
        let valve_index_mapping: BTreeMap<&str, usize> = named_valves
            .keys()
            .enumerate()
            .map(|(i, valve_name)| (valve_name.as_ref(), i))
            .collect();

        named_valves
            .values()
            .map(|valve| Valve {
                flow_rate: valve.flow_rate,
                tunnels: valve
                    .tunnels
                    .iter()
                    .map(|neighbor: &String| valve_index_mapping[neighbor.as_str()])
                    .collect(),
            })
            .collect()
    }
}

#[derive(Debug, PartialEq)]
struct Grid {
    width: usize,
    cells: Vec<usize>,
}
impl Grid {
    fn from_edge_vector(valves: &[Valve]) -> Self {
        let width = valves.len();
        let mut adjacency_matrix = Grid {
            width,
            cells: vec![usize::MAX; width * width],
        };
        for (valve_index, valve) in valves.iter().enumerate() {
            for neighbor_index in valve.tunnels.iter() {
                adjacency_matrix[(valve_index, *neighbor_index)] = 1;
            }
        }

        adjacency_matrix.floyd_warshall();
        adjacency_matrix
    }

    fn floyd_warshall(&mut self) -> () {
        for i in 0..self.width {
            self[(i, i)] = 0;
        }

        for k in 0..self.width {
            for i in 0..self.width {
                for j in 0..self.width {
                    self[(i, j)] = min(self[(i, j)], self[(i, k)].saturating_add(self[(k, j)]));
                }
            }
        }
    }
}

impl Index<(usize, usize)> for Grid {
    type Output = usize;

    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        &self.cells[i + j * self.width]
    }
}

impl IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        &mut self.cells[i + j * self.width]
    }
}

pub struct Cave {
    valves: Grid,
    potential_valves: HashSet<usize>,
    flow_rates: Vec<usize>,
}

impl Cave {
    pub fn parse(input: &str) -> Cave {
        let re = Regex::new(
            r"Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? (([A-Z]{2}, )*[A-Z]{2})",
        )
        .unwrap();
        let named_valves: BTreeMap<String, NamedValve> = re
            .captures_iter(input)
            .map(|cap| {
                (
                    cap[1].to_string(),
                    NamedValve {
                        flow_rate: cap[2].parse().unwrap(),
                        tunnels: cap[3].split(", ").map(|v| v.to_string()).collect(),
                    },
                )
            })
            .collect();

        let valves_list = Valve::from_named_valves(named_valves);
        let valves = Grid::from_edge_vector(&valves_list);

        let flow_rates = valves_list.iter().map(|v| v.flow_rate).collect();

        let potential_valves = valves_list
            .iter()
            .enumerate()
            .filter(|(_i, v)| v.flow_rate > 0)
            .map(|(i, _v)| i)
            .collect();

        Cave {
            valves,
            potential_valves,
            flow_rates,
        }
    }

    pub fn max_pressure(&self) -> usize {
        self.pressure(30, 0, 0, self.potential_valves.clone(), &RefCell::new(0))
    }

    pub fn max_pressure_with_elephant(&self) -> usize {
        self.potential_valves
            .iter()
            .copied()
            .powerset()
            .map(|set| {
                let set = HashSet::from_iter(set);
                let complement = &self.potential_valves - &set;

                self.pressure(26, 0, 0, set, &RefCell::new(0))
                    + self.pressure(26, 0, 0, complement, &RefCell::new(0))
            })
            .max()
            .unwrap()
    }

    fn pressure(
        &self,
        remaining_minutes: usize,
        current_valve: usize,
        current_pressure: usize,
        remaining_valves: HashSet<usize>,
        lower_bound: &RefCell<usize>,
    ) -> usize {
        if *lower_bound.borrow() < current_pressure {
            *lower_bound.borrow_mut() = current_pressure;
        }

        let best_potential_pressure = current_pressure
            + remaining_valves
                .iter()
                .map(|v| self.flow_rates[*v] * (remaining_minutes))
                .sum::<usize>();

        if best_potential_pressure < *lower_bound.borrow() {
            return current_pressure;
        }

        remaining_valves
            .iter()
            .copied()
            .filter(|valve| remaining_minutes > (self.valves[(current_valve, *valve)] + 1))
            .map(|valve| {
                let new_remaining_minutes =
                    remaining_minutes - (self.valves[(current_valve, valve)] + 1);
                let new_pressure =
                    current_pressure + new_remaining_minutes * self.flow_rates[valve];
                let new_remaining_valves = &remaining_valves - &HashSet::from([valve]);
                self.pressure(
                    new_remaining_minutes,
                    valve,
                    new_pressure,
                    new_remaining_valves,
                    lower_bound,
                )
            })
            .max()
            .unwrap_or(current_pressure)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn graph_to_adjacency() {
        // GIVEN
        let input = indoc! {"
            Valve AA has flow rate=0; tunnel leads to valve BB
            Valve BB has flow rate=13; tunnels lead to valves AA, CC
            Valve CC has flow rate=2; tunnel leads to valve BB
            "};

        // WHEN
        let graph = Cave::parse(input);

        // THEN
        assert_eq!(
            Grid {
                width: 3,
                cells: vec![0, 1, 2, 1, 0, 1, 2, 1, 0,]
            },
            graph.valves
        );
    }

    #[test]
    fn graph_max_pressure() {
        // GIVEN
        let input = indoc! {"
            Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
            Valve BB has flow rate=13; tunnels lead to valves CC, AA
            Valve CC has flow rate=2; tunnels lead to valves DD, BB
            Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
            Valve EE has flow rate=3; tunnels lead to valves FF, DD
            Valve FF has flow rate=0; tunnels lead to valves EE, GG
            Valve GG has flow rate=0; tunnels lead to valves FF, HH
            Valve HH has flow rate=22; tunnel leads to valve GG
            Valve II has flow rate=0; tunnels lead to valves AA, JJ
            Valve JJ has flow rate=21; tunnel leads to valve II
            "};
        let graph = Cave::parse(input);

        // WHEN
        let result = graph.max_pressure();

        // THEN
        assert_eq!(1651, result);
    }

    #[test]
    fn graph_max_pressure_with_elephant() {
        // GIVEN
        let input = indoc! {"
            Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
            Valve BB has flow rate=13; tunnels lead to valves CC, AA
            Valve CC has flow rate=2; tunnels lead to valves DD, BB
            Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
            Valve EE has flow rate=3; tunnels lead to valves FF, DD
            Valve FF has flow rate=0; tunnels lead to valves EE, GG
            Valve GG has flow rate=0; tunnels lead to valves FF, HH
            Valve HH has flow rate=22; tunnel leads to valve GG
            Valve II has flow rate=0; tunnels lead to valves AA, JJ
            Valve JJ has flow rate=21; tunnel leads to valve II
            "};
        let graph = Cave::parse(input);

        // WHEN
        let result = graph.max_pressure_with_elephant();

        // THEN
        assert_eq!(1707, result);
    }
}
