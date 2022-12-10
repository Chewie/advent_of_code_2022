use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut cpu = day10::Cpu::new();
    cpu.run(&input);
    let sum = cpu.sum_of_interesting_signal_strengths();

    println!("Sum of interesting signal strengths: {sum}");

    let crt = cpu.crt();
    print!("{crt}");
}
