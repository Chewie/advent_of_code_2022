use std::collections::HashSet;

pub fn index_after_start_of_packet(input: &str) -> usize {
    index_after_unique_sequence(input, 4)
}

pub fn index_after_start_of_message(input: &str) -> usize {
    index_after_unique_sequence(input, 14)
}

fn index_after_unique_sequence(input: &str, sequence_length: usize) -> usize {
    let start_of_packet = input
        .as_bytes()
        .windows(sequence_length)
        .map(|window| window.iter().collect::<HashSet<_>>())
        .enumerate()
        .find(|(_idx, set)| set.len() == sequence_length);

    match start_of_packet {
        Some((idx, _)) => idx + sequence_length,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_after_start_of_packet_test() {
        // GIVEN
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

        // WHEN
        let idx = index_after_start_of_packet(input);

        // THEN
        assert_eq!(7, idx);
    }

    #[test]
    fn index_after_start_of_message_test() {
        // GIVEN
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

        // WHEN
        let idx = index_after_start_of_message(input);

        // THEN
        assert_eq!(19, idx);
    }
}
