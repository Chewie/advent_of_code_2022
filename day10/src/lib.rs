pub struct Cpu {
    x: i32,
    history: Vec<i32>,
    crt: [u8; 240],
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            x: 1,
            history: vec![0], // Dummy value to start at 1
            crt: [b'.'; 240],
        }
    }

    pub fn run(&mut self, input: &str) {
        input.lines().for_each(|line| self.run_command(line));
    }

    pub fn sum_of_interesting_signal_strengths(&self) -> i32 {
        [20, 60, 100, 140, 180, 220]
            .iter()
            .map(|cycle| self.signal_strength(*cycle))
            .sum()
    }

    pub fn crt(&self) -> String {
        self.crt
            .chunks(40)
            .flat_map(std::str::from_utf8)
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn run_command(&mut self, command: &str) {
        self.update_crt();
        self.history.push(self.x);
        if let Some(("addx", n)) = command.split_once(' ') {
            self.update_crt();
            self.history.push(self.x);

            let n: i32 = n.parse().unwrap();
            self.x += n;
        }
    }

    fn signal_strength(&self, cycle: usize) -> i32 {
        cycle as i32 * self.history[cycle]
    }

    fn update_crt(&mut self) {
        let pos = (self.current_cycle()) % 40;
        if [self.x - 1, self.x, self.x + 1].contains(&(pos as i32)) {
            self.crt[self.current_cycle()] = b'#';
        }
    }

    fn current_cycle(&self) -> usize {
        self.history.len() - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    impl Cpu {
        fn x_history(&self) -> &[i32] {
            &self.history
        }
    }

    #[test]
    fn cpu_x_history() {
        // GIVEN
        let input = indoc! {"
            noop
            addx 3
            addx -5
            noop
        "};
        let mut cpu = Cpu::new();

        // WHEN
        cpu.run(input);

        // THEN
        assert_eq!(vec![0, 1, 1, 1, 4, 4, -1], cpu.x_history());
    }

    #[test]
    fn cpu_sum_of_interesting_signal_strengths() {
        // GIVEN
        let input = indoc! {"
            addx 15
            addx -11
            addx 6
            addx -3
            addx 5
            addx -1
            addx -8
            addx 13
            addx 4
            noop
            addx -1
            addx 5
            addx -1
            addx 5
            addx -1
            addx 5
            addx -1
            addx 5
            addx -1
            addx -35
            addx 1
            addx 24
            addx -19
            addx 1
            addx 16
            addx -11
            noop
            noop
            addx 21
            addx -15
            noop
            noop
            addx -3
            addx 9
            addx 1
            addx -3
            addx 8
            addx 1
            addx 5
            noop
            noop
            noop
            noop
            noop
            addx -36
            noop
            addx 1
            addx 7
            noop
            noop
            noop
            addx 2
            addx 6
            noop
            noop
            noop
            noop
            noop
            addx 1
            noop
            noop
            addx 7
            addx 1
            noop
            addx -13
            addx 13
            addx 7
            noop
            addx 1
            addx -33
            noop
            noop
            noop
            addx 2
            noop
            noop
            noop
            addx 8
            noop
            addx -1
            addx 2
            addx 1
            noop
            addx 17
            addx -9
            addx 1
            addx 1
            addx -3
            addx 11
            noop
            noop
            addx 1
            noop
            addx 1
            noop
            noop
            addx -13
            addx -19
            addx 1
            addx 3
            addx 26
            addx -30
            addx 12
            addx -1
            addx 3
            addx 1
            noop
            noop
            noop
            addx -9
            addx 18
            addx 1
            addx 2
            noop
            noop
            addx 9
            noop
            noop
            noop
            addx -1
            addx 2
            addx -37
            addx 1
            addx 3
            noop
            addx 15
            addx -21
            addx 22
            addx -6
            addx 1
            noop
            addx 2
            addx 1
            noop
            addx -10
            noop
            noop
            addx 20
            addx 1
            addx 2
            addx 2
            addx -6
            addx -11
            noop
            noop
            noop
        "};
        let mut cpu = Cpu::new();

        // WHEN
        cpu.run(input);

        // THEN
        assert_eq!(13140, cpu.sum_of_interesting_signal_strengths());
    }

    #[test]
    fn cpu_crt_draw() {
        // GIVEN
        let input = indoc! {"
            addx 15
            addx -11
            addx 6
            addx -3
            addx 5
            addx -1
            addx -8
            addx 13
            addx 4
            noop
            addx -1
            addx 5
            addx -1
            addx 5
            addx -1
            addx 5
            addx -1
            addx 5
            addx -1
            addx -35
            addx 1
            addx 24
            addx -19
            addx 1
            addx 16
            addx -11
            noop
            noop
            addx 21
            addx -15
            noop
            noop
            addx -3
            addx 9
            addx 1
            addx -3
            addx 8
            addx 1
            addx 5
            noop
            noop
            noop
            noop
            noop
            addx -36
            noop
            addx 1
            addx 7
            noop
            noop
            noop
            addx 2
            addx 6
            noop
            noop
            noop
            noop
            noop
            addx 1
            noop
            noop
            addx 7
            addx 1
            noop
            addx -13
            addx 13
            addx 7
            noop
            addx 1
            addx -33
            noop
            noop
            noop
            addx 2
            noop
            noop
            noop
            addx 8
            noop
            addx -1
            addx 2
            addx 1
            noop
            addx 17
            addx -9
            addx 1
            addx 1
            addx -3
            addx 11
            noop
            noop
            addx 1
            noop
            addx 1
            noop
            noop
            addx -13
            addx -19
            addx 1
            addx 3
            addx 26
            addx -30
            addx 12
            addx -1
            addx 3
            addx 1
            noop
            noop
            noop
            addx -9
            addx 18
            addx 1
            addx 2
            noop
            noop
            addx 9
            noop
            noop
            noop
            addx -1
            addx 2
            addx -37
            addx 1
            addx 3
            noop
            addx 15
            addx -21
            addx 22
            addx -6
            addx 1
            noop
            addx 2
            addx 1
            noop
            addx -10
            noop
            noop
            addx 20
            addx 1
            addx 2
            addx 2
            addx -6
            addx -11
            noop
            noop
            noop
        "};
        let mut cpu = Cpu::new();

        // WHEN
        cpu.run(input);

        // THEN
        let expected_crt = indoc! {"
            ##..##..##..##..##..##..##..##..##..##..
            ###...###...###...###...###...###...###.
            ####....####....####....####....####....
            #####.....#####.....#####.....#####.....
            ######......######......######......####
            #######.......#######.......#######....."};
        assert_eq!(expected_crt, cpu.crt());
    }
}
