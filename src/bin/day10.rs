use std::{
    cmp::Ordering,
    collections::{BTreeMap, HashMap, HashSet},
};

use num::{abs, integer::gcd};

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn dist(&self, other: &Self) -> i32 {
        abs(self.x - other.x) + abs(self.y - other.y)
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Slope {
    num: i32,
    den: i32,
}

impl From<(&Point, &Point)> for Slope {
    fn from((p1, p2): (&Point, &Point)) -> Self {
        let (den, num) = (p1.x - p2.x, p2.y - p1.y);
        let div = gcd(num, den);
        Self {
            num: num / div,
            den: den / div,
        }
    }
}

impl Slope {
    fn is_right_half(&self) -> bool {
        self.den > 0 || (self.den == 0 && self.num > 0)
    }
}

impl Ord for Slope {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_right = self.is_right_half();
        let other_right = other.is_right_half();

        if self_right != other_right {
            return if self_right {
                Ordering::Less
            } else {
                Ordering::Greater
            };
        }

        let left = self.num as i64 * other.den as i64;
        let right = self.den as i64 * other.num as i64;

        right.cmp(&left)
    }
}

impl PartialOrd for Slope {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn day10a(input: &str) -> (Point, usize) {
    let mut grid: HashMap<Point, HashSet<Slope>> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, char)| match char {
                    '#' => Some(Point {
                        x: x as i32,
                        y: y as i32,
                    }),
                    _ => None,
                })
        })
        .map(|p| (p, HashSet::new()))
        .collect();

    let points: Vec<Point> = grid.keys().cloned().collect();

    for p1 in points.iter() {
        for p2 in points.iter() {
            if p1 == p2 {
                continue;
            }
            if let Some(set) = grid.get_mut(p1) {
                set.insert((p1, p2).into());
            }
        }
    }

    grid.into_iter()
        .max_by_key(|(_p, set)| set.len())
        .map(|(p, set)| (p, set.len()))
        .unwrap()
}

fn day10b(input: &str) -> Option<i32> {
    let (center, _) = day10a(input);

    let grid: HashSet<Point> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, char)| match char {
                    '#' => {
                        if x as i32 == center.x && y as i32 == center.y {
                            None
                        } else {
                            Some(Point {
                                x: x as i32,
                                y: y as i32,
                            })
                        }
                    }
                    _ => None,
                })
        })
        .collect();

    let mut targets: BTreeMap<Slope, BTreeMap<i32, Point>> = BTreeMap::new();
    for p in grid.into_iter() {
        let dir = (&p, &center).into();
        targets.entry(dir).or_default().insert(center.dist(&p), p);
    }
    let mut vaporized_count = 0;

    loop {
        let mut removed_any = false;

        // Sweep clockwise through every angle in sorted order
        for points in targets.values_mut() {
            // Remove the closest point along this angle
            if let Some((_, point)) = points.pop_first() {
                vaporized_count += 1;
                removed_any = true;

                if vaporized_count == 200 {
                    return Some(point.x * 100 + point.y);
                }
            }
        }

        // Safety exit if the map has fewer than 200 points total
        if !removed_any {
            return None;
        }
    }
}

fn main() {
    println!("{:?}", day10a(include_str!("../../input/day10.input")));
    println!("{:?}", day10b(include_str!("../../input/day10.input")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####";
        let r = day10a(input);
        assert_eq!(r, (Point { x: 5, y: 8 }, 33));
    }

    #[test]
    fn part_1_2() {
        let input = "#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.";
        let r = day10a(input);
        assert_eq!(r, (Point { x: 1, y: 2 }, 35));
    }

    #[test]
    fn part_1_3() {
        let input = ".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..";
        let r = day10a(input);
        assert_eq!(r, (Point { x: 6, y: 3 }, 41));
    }

    // #[test]
    // fn part_2() {
    //     let input = "";
    //     let r = day10b(input);
    //     assert_eq!(r, 0);
    // }
}
