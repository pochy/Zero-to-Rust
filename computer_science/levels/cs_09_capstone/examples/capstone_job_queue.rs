use std::collections::{HashMap, VecDeque};
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum JobState {
    Ready,
    InProgress,
    Done,
    Dead,
}

#[derive(Debug)]
struct Job {
    id: u64,
    payload: String,
    attempts: u8,
    state: JobState,
}

#[derive(Debug, Default)]
struct JobQueue {
    next_id: u64,
    ready: VecDeque<u64>,
    jobs: HashMap<u64, Job>,
}

impl JobQueue {
    fn enqueue(&mut self, payload: String) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.ready.push_back(id);
        self.jobs.insert(
            id,
            Job {
                id,
                payload,
                attempts: 0,
                state: JobState::Ready,
            },
        );
        id
    }

    fn dequeue(&mut self) -> Option<(u64, String)> {
        let id = self.ready.pop_front()?;
        let job = self.jobs.get_mut(&id)?;
        job.state = JobState::InProgress;
        job.attempts += 1;
        Some((job.id, job.payload.clone()))
    }

    fn ack(&mut self, id: u64) -> bool {
        let Some(job) = self.jobs.get_mut(&id) else {
            return false;
        };
        job.state = JobState::Done;
        true
    }

    fn retry_or_dead_letter(&mut self, id: u64, max_attempts: u8) -> bool {
        let Some(job) = self.jobs.get_mut(&id) else {
            return false;
        };

        if job.attempts >= max_attempts {
            job.state = JobState::Dead;
        } else {
            job.state = JobState::Ready;
            self.ready.push_back(id);
        }

        true
    }
}

fn main() {
    let start = Instant::now();
    let mut queue = JobQueue::default();

    let email = queue.enqueue("send email".to_string());
    let resize = queue.enqueue("resize avatar".to_string());

    println!("enqueued: {email}, {resize}");
    println!("dequeue: {:?}", queue.dequeue());
    queue.retry_or_dead_letter(email, 3);
    println!("dequeue again: {:?}", queue.dequeue());
    queue.ack(resize);
    println!("dequeue retry: {:?}", queue.dequeue());
    queue.retry_or_dead_letter(email, 1);

    println!("elapsed: {:?}", start.elapsed());
    println!("queue: {queue:#?}");
}

#[cfg(test)]
mod tests {
    use super::{JobQueue, JobState};

    #[test]
    fn retries_then_dead_letters() {
        let mut queue = JobQueue::default();
        let id = queue.enqueue("work".to_string());

        assert_eq!(queue.dequeue(), Some((id, "work".to_string())));
        assert!(queue.retry_or_dead_letter(id, 1));
        assert_eq!(
            queue.jobs.get(&id).map(|job| job.state),
            Some(JobState::Dead)
        );
    }
}
