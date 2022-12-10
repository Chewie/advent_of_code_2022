pub struct Cpu {
    x: i32,
    history: Vec<i32>,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            x: 1,
            history: vec![0], // Dummy value to start at 1
        }
    }

    pub fn run(&mut self, input: &str) -> () {
        input.lines().for_each(|line| self.run_command(line));
    }

    pub fn x_history(&self) -> &Vec<i32> {
        &self.history
    }

    pub fn sum_of_interesting_signal_strengths(&self) -> i32 {
        [20, 60, 100, 140, 180, 220]
            .iter()
            .map(|cycle| self.signal_strength(*cycle))
            .sum()
    }

    fn run_command(&mut self, command: &str) {
        if command == "noop" {
            self.history.push(self.x);
        }
        if let Some(("addx", n)) = command.split_once(' ') {
            self.history.push(self.x);
            self.history.push(self.x);

            let n: i32 = n.parse().unwrap();
            self.x += n;
        }
    }

    fn signal_strength(&self, cycle: usize) -> i32 {
        cycle as i32 * self.history[cycle]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cpu_x_history() {
        // GIVEN
        let input = "noop
addx 3
addx -5
noop
";
        let mut cpu = Cpu::new();

        // WHEN
        cpu.run(input);

        // THEN
        assert_eq!(vec![0, 1, 1, 1, 4, 4, -1], *cpu.x_history());
    }

    #[test]
    fn cpu_sum_of_interesting_signal_strengths() {
        // GIVEN
        let input = "addx 15
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
";
        let mut cpu = Cpu::new();

        // WHEN
        cpu.run(input);

        // THEN
        assert_eq!(13140, cpu.sum_of_interesting_signal_strengths());
    }
}
