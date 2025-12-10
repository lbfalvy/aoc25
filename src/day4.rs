use itertools::Itertools;

const DATA: &str = include_str!("day4.input.txt");

/// true means occupied
fn tbl() -> Vec<Vec<bool>> {
	DATA.trim().split("\n").map(|l| l.chars().map(|c| c == '@').collect_vec()).collect_vec()
}

fn find_removables(tbl: &[Vec<bool>]) -> Vec<(usize, usize)> {
	let mut ret = Vec::new();
	for (r, c) in (0..tbl.len() as i64)
		.cartesian_product(0..tbl[0].len() as i64)
		.filter(|(r, c)| tbl[*r as usize][*c as usize])
	{
		let mut taken = 0;
		for (dr, dc) in (-1..=1).cartesian_product(-1..=1).filter(|p| *p != (0, 0)) {
			let Some(row) = usize::try_from(r + dr).ok().and_then(|r| tbl.get(r)) else { continue };
			let Some(f) = usize::try_from(c + dc).ok().and_then(|c| row.get(c)) else { continue };
			if *f {
				taken += 1
			};
		}
		if taken < 4 {
			ret.push((r as usize, c as usize));
		}
	}
	ret
}

pub fn task1() { println!("{}", find_removables(&tbl()).len()) }

pub fn task2() {
	let mut tbl = tbl();
	let mut total = 0;
	loop {
		let removables = find_removables(&tbl);
		if removables.is_empty() {
			break;
		}
		for (r, c) in removables {
			total += 1;
			tbl[r][c] = false;
		}
	}
	println!("{total}")
}
