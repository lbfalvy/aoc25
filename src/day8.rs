use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::mem;
use std::ops::Range;

use itertools::Itertools;

const DATA: &str = include_str!("day8.input.txt");

fn load() -> Vec<[u64; 3]> {
	(DATA.trim().split("\n"))
		.map(|s| s.split(",").map(|s| s.parse::<u64>().unwrap()).collect_array::<3>().unwrap())
		.collect_vec()
}

type Vec3 = [u64; 3];

const BITS: [usize; 2] = [0, 1];
const AXES: [usize; 3] = [0, 1, 2];

fn in_bounds(r: impl Borrow<Range<Vec3>>, pt: Vec3) -> bool {
	AXES.into_iter().all(|i| r.borrow().start[i] <= pt[i] && pt[i] < r.borrow().end[i])
}

fn nearest_in_cube(cube: impl Borrow<Range<Vec3>>, pt: Vec3) -> Vec3 {
	AXES.map(|i| cube.borrow().start[i].max(pt[i]).min(cube.borrow().end[i] - 1))
}

/// Squared diostance
fn sqdist(a: Vec3, b: Vec3) -> u64 { AXES.map(|i| a[i].abs_diff(b[i]).pow(2)).iter().sum() }

/// Given a cube and a point, finds which octant is closest (`[u1; 3]`) and also
/// how much closer the closer side is along each axis (permutation of `[0, 1,
/// 2]`)
fn order_of_proximity(cube: impl Borrow<Range<Vec3>>, b: Vec3) -> ([usize; 3], [usize; 3]) {
	let cube = cube.borrow();
	let nearest =
		AXES.map(|i| if cube.start[i].abs_diff(b[i]) < cube.end[i].abs_diff(b[i]) { 0 } else { 1 });
	let mut axes_by_offset = AXES.map(|i| (b[i].abs_diff((cube.start[i] + cube.end[i]) / 2), i));
	axes_by_offset.sort_unstable_by_key(|(dist, _)| *dist);
	(nearest, axes_by_offset.map(|(_, i)| i))
}

#[derive(Debug)]
struct Octree {
	kind: OctreeKind,
	bounds: Range<Vec3>,
	center: Vec3,
}

#[derive(Debug)]
enum OctreeKind {
	Empty,
	Leaf(Vec3),
	Node([[[Box<Octree>; 2]; 2]; 2]),
}
impl Octree {
	fn new(bounds: Range<Vec3>) -> Self {
		let center = AXES.map(|i| (bounds.start[i] + bounds.end[i]) / 2);
		Self { center, bounds, kind: OctreeKind::Empty }
	}
	/// The point itself may also be found. If it is included in the octree, you
	/// should exclude it in the filter.
	///
	/// Points are not passed to the filter in any particular order.
	fn find_nearest(&self, pt: Vec3, cur: &mut Option<Vec3>, filter: &mut impl FnMut(Vec3) -> bool) {
		match (&self.kind, *cur) {
			(OctreeKind::Empty, _) => (),
			(OctreeKind::Leaf(l), Some(old)) if sqdist(pt, old) < sqdist(pt, *l) || filter(*l) => (),
			(OctreeKind::Leaf(l), _) => *cur = Some(*l),
			(OctreeKind::Node(..), Some(old))
				if sqdist(pt, old) < sqdist(nearest_in_cube(&self.bounds, pt), pt) =>
				(),
			(OctreeKind::Node(octs), _) => {
				let (mut iv, axes_perm) = order_of_proximity(&self.bounds, pt);
				let mut iv_opp = iv.map(|bit| if bit == 0 { 1 } else { 0 });
				for _ in 0..2 {
					for _ in 0..2 {
						for _ in 0..2 {
							octs[iv[0]][iv[1]][iv[2]].find_nearest(pt, cur, filter);
							mem::swap(&mut iv[axes_perm[0]], &mut iv_opp[axes_perm[0]]);
						}
						mem::swap(&mut iv[axes_perm[1]], &mut iv_opp[axes_perm[1]]);
					}
					mem::swap(&mut iv[axes_perm[2]], &mut iv_opp[axes_perm[2]]);
				}
			},
		}
	}
	fn assert_in_bounds(&self, pt: Vec3) {
		assert!(in_bounds(&self.bounds, pt), "Point {pt:?} outside octree at {:?}", self.bounds)
	}
	// remove a point if found
	fn rm(&mut self, pt: Vec3) -> bool {
		if !in_bounds(&self.bounds, pt) {
			return false;
		}
		match &mut self.kind {
			OctreeKind::Empty => false,
			OctreeKind::Leaf(l) if *l != pt => false,
			kind @ OctreeKind::Leaf(_) => {
				*kind = OctreeKind::Empty;
				true
			},
			OctreeKind::Node(nodes) => {
				for node in nodes.iter_mut().flatten().flatten() {
					if node.rm(pt) {
						let new_kind =
							nodes.iter().flatten().flatten().fold(Some(OctreeKind::Empty), |s, t| {
								match (s, &t.kind) {
									(n, OctreeKind::Empty) => n,
									(Some(OctreeKind::Empty), OctreeKind::Leaf(l)) => Some(OctreeKind::Leaf(*l)),
									(_, OctreeKind::Leaf(_)) => None,
									(_, OctreeKind::Node(_)) => None,
								}
							});
						if let Some(kind) = new_kind {
							self.kind = kind
						}
						return true;
					}
				}
				false
			},
		}
	}
	fn add(&mut self, pt: Vec3) {
		self.assert_in_bounds(pt);
		match &mut self.kind {
			OctreeKind::Empty => self.kind = OctreeKind::Leaf(pt),
			OctreeKind::Leaf(l) => {
				let l = *l;
				let upper_bound = [self.center, self.bounds.end];
				let lower_bound = [self.bounds.start, self.center];
				let quads = BITS.map(|x| {
					BITS.map(|y| {
						BITS.map(|z| {
							let coords = [x, y, z];
							let [min, max] =
								[lower_bound, upper_bound].map(|bounds| AXES.map(|i| bounds[coords[i]][i]));
							Box::new(Octree::new(min..max))
						})
					})
				});
				self.kind = OctreeKind::Node(quads);
				self.add(l);
				self.add(pt);
			},
			OctreeKind::Node(boxes) => {
				let oct_pt = AXES.map(|i| if self.center[i] <= pt[i] { 1 } else { 0 });
				let sub = &mut boxes[oct_pt[0]][oct_pt[1]][oct_pt[2]];
				sub.add(pt);
			},
		}
	}
}

pub fn task1() {
	let ptv = load();
	let mut t = Octree::new([0, 0, 0]..[100_000, 100_000, 100_000]);
	for pt in &ptv {
		t.add(*pt);
	}
	let mut groups = HashMap::new();
	let mut links = HashSet::new();
	while !links.len() == 2000 {
		let mut best_pair = None;
		for pt in &ptv {
			let mut nearest = None;
			t.find_nearest(*pt, &mut nearest, &mut |p| p != *pt && !links.contains(&(*pt, p)));
			let nearest = nearest.unwrap();
			if best_pair.is_none_or(|(l, r)| sqdist(*pt, nearest) < sqdist(l, r)) {
				best_pair = Some((*pt, nearest));
			}
		}
		let best_pair = best_pair.unwrap();
		links.extend([best_pair, (best_pair.1, best_pair.0)]);
		groups.insert(best_pair.0, best_pair.1);
		groups.insert(best_pair.1, best_pair.1);
	}
	println!("done");
	// let mut near = None;
	// t.find_nearest([30, 30, 30], &mut near);
	// println!("{t:?}, {near:?}")
}
