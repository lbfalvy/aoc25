use itertools::Itertools;

const DATA: &str = include_str!("day7.input.txt");

pub fn task12() {
	let (input_line, manifold_lines) = DATA.split_once('\n').unwrap();
	let splitters = (manifold_lines.trim().split('\n'))
		.map(|l| l.chars().map(|c| c == '^').collect_vec())
		.collect_vec();
	let mut beam_state =
		input_line.trim().chars().map(|c| if c == 'S' { 1 } else { 0 }).collect_vec();
	let mut split_count = 0;
	for layer in splitters {
		for i in 0..layer.len() {
			if !layer[i] {
				continue;
			}
			if 0 < beam_state[i] {
				split_count += 1;
				beam_state[i - 1] += beam_state[i];
				beam_state[i + 1] += beam_state[i];
				beam_state[i] = 0;
			}
		}
	}
	println!("task1: {split_count}");
	println!("task2: {}", beam_state.iter().sum::<u64>());
}
