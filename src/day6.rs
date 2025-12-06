use itertools::Itertools;

const DATA: &str = include_str!("day6.input.txt");

pub fn task1() {
	let lines =
		DATA.trim().split("\n").map(|line| line.split_whitespace().collect_vec()).collect_vec();
	let (ops, num_lines) = lines.split_last().unwrap();
	let mut total = 0;
	for i in 0..ops.len() {
		let nums = num_lines.iter().map(|line| line[i].parse::<u64>().unwrap());
		total += match ops[i] {
			"+" => nums.reduce(|a, b| a + b).unwrap(),
			"*" => nums.reduce(|a, b| a * b).unwrap(),
			op => panic!("Unrecognized operation {op}"),
		}
	}
	println!("{total}")
}

pub fn task2() {
	let lines = DATA.trim_end_matches("\n").split("\n").collect_vec();
	let (ops_line, num_lines) = lines.split_last().unwrap();
	let ops_and_starts = ops_line.char_indices().filter(|(_, c)| *c != ' ').collect_vec();
	let (last_start, last_op) = ops_and_starts.last().unwrap();
	let ops_and_ranges = (ops_and_starts.iter().tuple_windows())
		.map(|((start, op), (next, _))| (*start..(next - 1), op))
		.chain([(*last_start..ops_line.len(), last_op)])
		.collect_vec();
	let mut total = 0;
	for (range, op) in ops_and_ranges {
		let nums = range.map(|i| {
			num_lines.iter().fold(0u64, |n, c| {
				let d = c.as_bytes()[i];
				match d {
					b' ' => n,
					b'0'..=b'9' => n * 10 + ((d - b'0') as u64),
					b => panic!("unrecognized number char {b} {:?}", char::from_u32(b.into())),
				}
			})
		});
		total += match op {
			'+' => nums.reduce(|a, b| a + b).unwrap(),
			'*' => nums.reduce(|a, b| a * b).unwrap(),
			op => panic!("Unrecognized operation {op:?}"),
		}
	}
	println!("{total}")
}
