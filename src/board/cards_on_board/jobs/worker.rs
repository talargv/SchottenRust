use std::fs::File;
use std::io::{BufReader, BufRead, Error, Lines};
use std::ops::Index;

use crate::common::{STONE_CARDS_LIMIT, NUM_OF_NUMS, NUM_OF_COLORS};
use crate::components::Card;
use super::{StoneCards, Job};

use itertools::Itertools;

pub struct WorkCollector {
    worker: Worker,
    data: Vec<Vec<Card>>,
}
impl WorkCollector {
    pub fn new(job: &Job) -> Self {
        // Current implementation assumes the following: 
        assert!(STONE_CARDS_LIMIT == 3);

        WorkCollector {
            worker: Worker::new(&job.cards),
            data: Vec::new(),
        }
    }

    pub fn work(&mut self) {
        if let Some(cards) = self.worker.work() {
            self.data.push(cards);
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}
impl Index<usize> for WorkCollector {
    type Output = Vec<Card>;

    fn index(&self, i: usize) -> &Self::Output {
        &self.data[i]
    }
}

enum Worker {
    LessThanTwo(Lines<BufReader<File>>),
    Two(<Vec<Card> as IntoIterator>::IntoIter),
    Full(),
}
impl Worker {
    fn new(cards: &StoneCards) -> Worker {
        match cards.len() {
            3 => Worker::Full(),
            2 => Worker::Two(two(cards)),
            1 => {
                let card = cards.iter().last().unwrap();
    
                Worker::LessThanTwo(
                    BufReader::new(
                        File::open(
                            format!("duos_sorted_{}{}", card.num(), card.color())
                        ).unwrap()
                    ).lines()
                )
            }
            0 => Worker::LessThanTwo(BufReader::new(File::open("triplets_sorted.txt").unwrap()).lines()),
            other => panic!("Unexpected len of cards: {}", other),
        }
    }

    fn work(&mut self) -> Option<Vec<Card>> {
        match self {
            Worker::Full() => None,
            Worker::Two(it) => {
                if let Some(card) = it.next() {
                    Some(vec![card])
                } else {
                    None
                }
            }
            Worker::LessThanTwo(br) => {
                if let Some(wrapped_line) = br.next() {
                    Some(parse_line(wrapped_line))
                } else {
                    None
                }
            }

        }
    }
}

fn two(cards: &StoneCards) -> <Vec<Card> as IntoIterator>::IntoIter {
    let mut all_cards: Vec<Card> = (1..=NUM_OF_NUMS)
        .cartesian_product(1..=NUM_OF_COLORS)
        .map(|(num,color)| Card::build(num, color))
        .filter(|card1| cards.iter().all(|card2| card2 != card1))
        .collect();

    all_cards.sort_by_cached_key(|card| {
        let mut tmp = cards.clone();

        tmp.push(card.clone());

        tmp.strength()  
    });

    all_cards.into_iter()
}

fn parse_line(line: Result<String, Error>) -> Vec<Card> {
    line
        .unwrap()
        .split("\n")
        .map(|s| {
            let num: u8 = s.parse().unwrap();

            Card::build(num / 10, num & 10)
        })
        .collect::<Vec<Card>>()
}



