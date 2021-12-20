use std::collections::HashMap;
use std::collections::HashSet;

use crate::aoc2021::Aoc2021;
use advent_of_code_traits::days::Day19;
use advent_of_code_traits::ParseInput;
use advent_of_code_traits::Solution;
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    values: [i32; 3],
}

impl Point {
    fn offset(self, offset: [i32; 3]) -> Point {
        let mut p = Point { values: [0; 3] };
        for i in 0..3 {
            p.values[i] = self.values[i] - offset[i];
        }
        p
    }

    fn diff(self, other: Point) -> [i32; 3] {
        let mut res = [0; 3];
        for i in 0..3 {
            res[i] = other.values[i] - self.values[i];
        }
        res
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rotation {
    values: [[i32; 3]; 3],
}

impl Rotation {
    fn product_rot(&self, other: &Rotation) -> Rotation {
        let mut res = Rotation {
            values: [[0; 3]; 3],
        };

        for p in 0..3 {
            let point = Point {
                values: [other.values[0][p], other.values[1][p], other.values[2][p]],
            };

            let respoint = self.product(point);
            for i in 0..3 {
                res.values[i][p] = respoint.values[i];
            }
        }

        res
    }

    fn product(&self, point: Point) -> Point {
        let mut values = [0; 3];
        for i in 0..3 {
            values[i] = self.values[i]
                .iter()
                .zip(point.values.iter())
                .map(|(r, p)| r * p)
                .sum();
        }
        Point { values }
    }
}

#[derive(Debug, Default)]
pub struct ScannerInput {
    id: usize,
    points: Vec<Point>,
}

#[derive(Debug)]
pub struct PuzzleInput {
    scanners: Vec<ScannerInput>,
}

impl ParseInput<Day19> for Aoc2021 {
    type Parsed = PuzzleInput;

    fn parse_input(input: &str) -> PuzzleInput {
        let re = Regex::new(r"--- scanner (\d+) ---").unwrap();

        let mut scanners = Vec::new();
        let mut current = ScannerInput::default();

        for line in input.lines() {
            let line = line.trim();
            if line.is_empty() {
                scanners.push(current);
                current = ScannerInput::default();
            } else if let Some(captures) = re.captures(line) {
                let id = captures[1].parse().unwrap();
                current.id = id;
            } else {
                let values: Vec<_> = line.split(',').map(|n| n.parse().unwrap()).collect();
                assert_eq!(values.len(), 3);
                current.points.push(Point {
                    values: values.try_into().unwrap(),
                })
            }
        }

        if !current.points.is_empty() {
            scanners.push(current);
        }

        PuzzleInput { scanners }
    }
}

fn generate_rotation_matrices() -> Vec<Rotation> {
    let rotation_a = vec![
        Rotation {
            values: [[1, 0, 0], [0, 1, 0], [0, 0, 1]],
        },
        Rotation {
            values: [[0, 1, 0], [0, 0, 1], [1, 0, 0]],
        },
        Rotation {
            values: [[0, 0, 1], [1, 0, 0], [0, 1, 0]],
        },
    ];

    let rotation_b = vec![
        Rotation {
            values: [[1, 0, 0], [0, 1, 0], [0, 0, 1]],
        },
        Rotation {
            values: [[-1, 0, 0], [0, -1, 0], [0, 0, 1]],
        },
        Rotation {
            values: [[-1, 0, 0], [0, 1, 0], [0, 0, -1]],
        },
        Rotation {
            values: [[1, 0, 0], [0, -1, 0], [0, 0, -1]],
        },
    ];

    let rotation_c = vec![
        Rotation {
            values: [[1, 0, 0], [0, 1, 0], [0, 0, 1]],
        },
        Rotation {
            values: [[0, 0, -1], [0, -1, 0], [-1, 0, 0]],
        },
    ];

    let mut res = Vec::with_capacity(24);
    for a in &rotation_a {
        for b in &rotation_b {
            for c in &rotation_c {
                let rot = a.product_rot(b).product_rot(c);
                res.push(rot);
            }
        }
    }
    res
}

#[derive(Debug)]
struct ScannerSuite {
    position: Option<[i32; 3]>,
    entries: Vec<ScannerSuiteEntry>,
}

impl ScannerSuite {
    fn set_position(&mut self, mut dir: [i32; 3]) {
        for i in dir.iter_mut() {
            *i = -*i;
        }

        self.position = Some(dir);
    }
}

#[derive(Debug)]
struct ScannerSuiteEntry {
    points: Vec<Point>,
}

fn evaluate_similarity(
    base: &HashSet<Point>,
    entry: &ScannerSuiteEntry,
) -> Option<([i32; 3], usize)> {
    let mut counter = HashMap::new();
    for a in base {
        for b in &entry.points {
            let diff = a.diff(*b);
            *counter.entry(diff).or_default() += 1;
        }
    }
    let res = counter
        .iter()
        .max_by_key(|(_, c)| *c)
        .map(|(k, c)| (*k, *c));
    if let Some((_, 1)) = res {
        None
    } else {
        res
    }
}

fn build_scanner_suites(scanners: &[ScannerInput]) -> Vec<ScannerSuite> {
    let matrices = generate_rotation_matrices();

    let mut suites = Vec::new();
    for scanner in scanners {
        let entries = matrices
            .iter()
            .map(|matrix| {
                let points = scanner.points.iter().map(|p| matrix.product(*p)).collect();
                ScannerSuiteEntry { points }
            })
            .collect();
        suites.push(ScannerSuite {
            position: None,
            entries,
        });
    }
    suites
}

fn decode_scanners(input: &PuzzleInput) -> (HashSet<Point>, Vec<ScannerSuite>) {
    let mut current_base: HashSet<Point> = input.scanners[0].points.iter().copied().collect();

    let mut suites = build_scanner_suites(&input.scanners[1..]);
    let mut working = true;

    while working {
        working = false;
        let mut max = None;
        for (si, other) in suites.iter().enumerate() {
            if other.position.is_some() {
                continue;
            }

            for (pi, entry) in other.entries.iter().enumerate() {
                if let Some((dir, similarity)) = evaluate_similarity(&current_base, entry) {
                    if let Some((_, m, _, _)) = max {
                        if m <= similarity {
                            max = Some((dir, similarity, si, pi));
                        }
                    } else {
                        max = Some((dir, similarity, si, pi));
                    }
                }
            }
        }

        if let Some((dir, _, si, pi)) = max {
            current_base.extend(suites[si].entries[pi].points.iter().map(|p| p.offset(dir)));
            suites[si].set_position(dir);
            working = true;
        }
    }

    (current_base, suites)
}

impl Solution<Day19> for Aoc2021 {
    type Part1Output = usize;
    type Part2Output = u32;

    fn part1(input: &PuzzleInput) -> usize {
        let (beacons, _) = decode_scanners(input);
        beacons.len()
    }

    fn part2(input: &PuzzleInput) -> u32 {
        let (_, suites) = decode_scanners(input);
        let mut scanners = vec![[0; 3]];
        for s in suites {
            if let Some(pos) = s.position {
                scanners.push(pos);
            }
        }

        let mut max = 0;
        for a in &scanners {
            for b in &scanners {
                let mut distance = 0;
                for i in 0..3 {
                    distance += (a[i] - b[i]).abs() as u32;
                }

                if distance > max {
                    max = distance;
                }
            }
        }
        max
    }
}
