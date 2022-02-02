use rand::prelude::*;
use std::ops::{Add, Mul};

use std::iter::successors;

const ONES: [&str; 20] = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];
const TENS: [&str; 10] = [
    "zero", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];
const ORDERS: [&str; 7] = [
    "zero",
    "thousand",
    "million",
    "billion",
    "trillion",
    "quadrillion",
    "quintillion", // enough for u64::MAX
];

pub fn encode(num: u64) -> String {
    match num {
        0..=19 => ONES[num as usize].to_string(),
        20..=99 => {
            let upper = (num / 10) as usize;
            match num % 10 {
                0 => TENS[upper].to_string(),
                lower => format!("{}-{}", TENS[upper], encode(lower)),
            }
        }
        100..=999 => format_num(num, 100, "hundred"),
        _ => {
            let (div, order) = successors(Some(1u64), |v| v.checked_mul(1000))
                .zip(ORDERS.iter())
                .find(|&(e, _)| e > num / 1000)
                .unwrap();

            format_num(num, div, order)
        }
    }
}

fn format_num(num: u64, div: u64, order: &str) -> String {
    match (num / div, num % div) {
        (upper, 0) => format!("{} {}", encode(upper), order),
        (upper, lower) => {
            format!("{} {} {}", encode(upper), order, encode(lower))
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Pos {
    x: isize,
    y: isize,
}

impl Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Pos {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Mul<usize> for Pos {
    type Output = Pos;

    fn mul(self, rhs: usize) -> Self::Output {
        Pos {
            x: self.x * rhs as isize,
            y: self.y * rhs as isize,
        }
    }
}

impl Mul<isize> for Pos {
    type Output = Pos;

    fn mul(self, rhs: isize) -> Self::Output {
        Pos {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Dir {
    North,
    South,
    East,
    West,
    NE,
    NW,
    SE,
    SW,
}

impl Dir {
    fn pos_from_dir(&self) -> Pos {
        match self {
            Dir::North => Pos { x: 0, y: 1 },
            Dir::South => Pos { x: 0, y: -1 },
            Dir::East => Pos { x: 1, y: 0 },
            Dir::West => Pos { x: -1, y: 0 },
            Dir::NE => Pos { x: 1, y: 1 },
            Dir::NW => Pos { x: -1, y: 1 },
            Dir::SE => Pos { x: 1, y: -1 },
            Dir::SW => Pos { x: -1, y: -1 },
        }
    }
}

#[derive(Clone, Debug)]

struct PlacedWord {
    word: String,
    position: Pos,
    direction: Dir,
}
#[derive(Clone, Debug)]
struct Solution {
    placed_words: Vec<PlacedWord>,
    board: Vec<Vec<char>>,
}

impl Solution {
    fn new<'a>(width: isize, height: isize) -> Solution {
        Solution {
            placed_words: Vec::new(),
            board: (0..width)
                .map(|_| {
                    let mut ret = Vec::new();
                    ret.resize(height as usize, '_');
                    ret
                })
                .collect(),
        }
    }
}

fn inbounds(pos: Pos, w: &isize, h: &isize) -> bool {
    pos.x < *w && pos.y < *h && pos.x >= 0 && pos.y >= 0
}

fn word_in_bounds(start: Pos, dir: Dir, w: &isize, h: &isize, l: &usize) -> bool {
    inbounds(start + dir.pos_from_dir() * *l, w, h) && inbounds(start, w, h)
}

fn check_collision(sol: &Solution) -> Option<Vec<Vec<char>>> {
    let last_word = sol.placed_words[sol.placed_words.len() - 1].clone();
    let mut ret = sol.board.clone();
    for i in 0..last_word.word.len() {
        let pos = last_word.position + last_word.direction.pos_from_dir() * i;
        let word_char = last_word
            .word
            .chars()
            .into_iter()
            .enumerate()
            .fold(' ', |acc, (j, c)| if i == j { c } else { acc }); // TODO getIndex from iter
                                                                    // println!("{},{}", pos.x as usize, pos.y as usize);
        match ret[pos.x as usize][pos.y as usize] {
            '_' => {
                ret[pos.x as usize][pos.y as usize] = word_char;
            }
            w if word_char == w => {}
            _ => {
                return None;
            }
        }
    }
    Some(ret)
}

fn backtrack(
    words: &Vec<String>,
    width: &isize,
    height: &isize,
    t: &usize,
    board_so_far: &Solution,
) -> Option<Solution> {
    if *t == words.len() {
        Some(board_so_far.clone())
    } else {
        let mut ret = board_so_far.clone();

        let mut rng = rand::thread_rng();

        let mut w: Vec<isize> = (0..*width).collect();
        w.shuffle(&mut rng);

        for x in w {
            let mut h: Vec<isize> = (0..*height).collect();
            h.shuffle(&mut rng);

            for y in h {
                let mut d: Vec<Dir> = vec![
                    Dir::North,
                    Dir::South,
                    Dir::East,
                    Dir::West,
                    Dir::NE,
                    Dir::NW,
                    Dir::SE,
                    Dir::SW,
                ];
                d.shuffle(&mut rng);
                for dir in d {
                    if word_in_bounds(Pos { x: x, y: y }, dir, width, height, &words[*t].len()) {
                        // println!("a1");
                        ret.placed_words.push(PlacedWord {
                            word: words[*t].clone(),
                            position: Pos { x: x, y: y },
                            direction: dir,
                        });
                        let old_board = ret.board.clone();

                        if let Some(new_board) = check_collision(&ret) {
                            // println!("b1");
                            ret.board = new_board;
                            let next_level = backtrack(words, width, height, &(*t + 1), &ret);
                            if next_level.is_some() {
                                return next_level;
                            }
                        }

                        ret.board = old_board;
                        ret.placed_words.pop();
                    }
                }
            }
        }

        None
    }
}

fn main() {
    let width = 12;
    let height = 12;
    let words = (0..25).map(encode).collect();

    match backtrack(&words, &width, &height, &0, &Solution::new(width, height)) {
        None => println!("no hay solucion"),
        Some(Solution {
            placed_words: _,
            board,
        }) => {
            for line in board {
                println!("{:?}", line);
            }
        }
    };
}
