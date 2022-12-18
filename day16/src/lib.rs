use std::{
    cell::RefCell,
    cmp,
    collections::{BTreeSet, HashMap},
};

use regex::Regex;

type ValveIndex = String;

#[derive(Debug)]
struct Valve {
    flow_rate: usize,
    tunnels: Vec<ValveIndex>,
}

impl Valve {}

pub struct Graph {
    valves: HashMap<ValveIndex, Valve>,
}

impl Graph {
    pub fn parse(input: &str) -> Graph {
        let re = Regex::new(
            r"Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? (([A-Z]{2}, )*[A-Z]{2})",
        )
        .unwrap();
        Graph {
            valves: re
                .captures_iter(input)
                .map(|cap| {
                    (
                        cap[1].to_string(),
                        Valve {
                            flow_rate: cap[2].parse().unwrap(),
                            tunnels: cap[3].split(", ").map(|v| v.to_string()).collect(),
                        },
                    )
                })
                .collect(),
        }
    }

    pub fn max_pressure(&self) -> usize {
        let opened_valves = BTreeSet::new();
        let cache = RefCell::new(HashMap::new());
        self.pressure(30, "AA", opened_valves, &cache)
    }

    fn pressure(
        &self,
        remaining_minutes: usize,
        current_valve: &str,
        opened_valves: BTreeSet<ValveIndex>,
        cache: &RefCell<HashMap<(usize, ValveIndex, BTreeSet<ValveIndex>), usize>>,
    ) -> usize {
        if remaining_minutes == 0 {
            return 0
        }
        let cache_key = (
            remaining_minutes,
            current_valve.to_owned(),
            opened_valves.clone(),
        );
        if cache.borrow().contains_key(&cache_key) {
            return cache.borrow()[&cache_key];
        }


        let mut res = self.valves[current_valve]
            .tunnels
            .iter()
            .map(|v| self.pressure(remaining_minutes - 1, v, opened_valves.clone(), cache))
            .max()
            .unwrap();

        if self.valves[current_valve].flow_rate > 0 && !opened_valves.contains(current_valve) {
            let with_opened = self.pressure(
                remaining_minutes - 1,
                current_valve,
                &opened_valves | &BTreeSet::from([current_valve.to_owned()]),
                cache,
            );
            res = cmp::max(res, with_opened);
        }
        res += opened_valves.iter().map(|v| self.valves[v].flow_rate).sum::<usize>();
        cache.borrow_mut().insert(cache_key, res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

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
        let graph = Graph::parse(input);

        // WHEN
        let result = graph.max_pressure();

        // THEN
        assert_eq!(1651, result);
    }
}
