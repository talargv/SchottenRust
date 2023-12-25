mod worker;

use std::collections::{HashMap, HashSet};

use crate::common::{NUM_OF_PLAYERS, NUM_OF_STONES};
use super::{StoneCards, PresentCards, Card};
use worker::WorkCollector;

pub struct BestOption {
    manager: JobManager,
    job_ids: Vec<Vec<JobIdentifier>>,
}
impl BestOption {
    pub fn new() -> Self{
        let mut manager = JobManager { workers: HashMap::new() };

        manager.add_jobs(vec![StoneCards::new()]);

        let mut job_ids: Vec<Vec<JobIdentifier>> = Vec::with_capacity(NUM_OF_PLAYERS as usize);

        for player in 0..NUM_OF_PLAYERS {
            job_ids.push(Vec::with_capacity(NUM_OF_STONES as usize));

            for _ in 0..NUM_OF_STONES {
                job_ids[player as usize].push(
                    JobIdentifier { 
                        job: Job { cards: StoneCards::new() },
                        index: 0,
                    }
                )
            }
        }

        BestOption {
            manager,
            job_ids,
        }
    }
}

#[derive(Hash, PartialEq, Eq)]
struct Job {
    cards: StoneCards,
}

struct JobIdentifier {
    job: Job,
    index: usize,
}

struct JobManager {
    workers: HashMap<Job, WorkCollector>,
}
impl JobManager {
    fn add_jobs<T: IntoIterator<Item=StoneCards>>(&mut self, jobs: T) {
        let all_jobs: HashSet<StoneCards> = jobs.into_iter().collect();

        for job in all_jobs.into_iter().map(|cards| Job {cards: cards}) {
            let work_collector = WorkCollector::new(&job);

            let _ = self.workers.entry(job).or_insert(work_collector);
        }
    }

    fn update_job(&mut self, job_id: &mut JobIdentifier, present_cards: &PresentCards) {
        let work_collector = self.workers.get_mut(&job_id.job).unwrap();

        
        // WILL CRASH FOR FULL CARDS?
        loop {
            let curr_cards = &work_collector[job_id.index];

            if curr_cards.iter().any(|c| present_cards.is_present(c)) {
                job_id.index += 1;

                if job_id.index == work_collector.len() {
                    work_collector.work();

                    // ONLY FOR DEV 
                    assert!(job_id.index != work_collector.len());
                }
            } else {
                break;
            }
        }
    }

    fn get(&self, job_id: &JobIdentifier) -> &Vec<Card> {
        &self.workers.get(&job_id.job).unwrap()[job_id.index]
    }
}
