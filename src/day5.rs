use std::ops::Range;

use itertools::Itertools;

const DATA: &str = include_str!("day5.input.txt");

fn parse() -> (Vec<Range<u64>>, Vec<u64>) {
	let (fresh_s, avail_s) = DATA.split_once("\n\n").unwrap();
	let avail = avail_s.split_whitespace().map(|s| s.parse::<u64>().unwrap()).collect_vec();
	let fresh = (fresh_s.split_whitespace())
		.map(|rs| {
			let (s, e) = rs.split_once("-").unwrap();
			let start = s.parse::<u64>().unwrap();
			let end = e.parse::<u64>().unwrap();
			assert!(start <= end, "weird range found");
			start..end + 1
		})
		.collect_vec();
	(fresh, avail)
}

pub fn task1() {
	let (fresh, avail) = parse();
	println!("{}", avail.iter().filter(|id| fresh.iter().any(|r| r.contains(*id))).count());
}

pub fn task2() {
	let (mut fresh, _) = parse();
	fresh.sort_by_key(|ri| ri.start);
	let sum = (fresh.into_iter())
		.coalesce(|a, b| if b.start <= a.end { Ok(a.start..a.end.max(b.end)) } else { Err((a, b)) })
		.fold(0, |sum, r| sum + r.end - r.start);
	println!("{sum}")
}
