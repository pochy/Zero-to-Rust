use std::collections::VecDeque;

fn main() {
    let mut undo_stack = Vec::new();
    undo_stack.push("type title");
    undo_stack.push("insert paragraph");
    undo_stack.push("delete heading");

    println!("undo order:");
    while let Some(action) = undo_stack.pop() {
        println!("{action}");
    }

    let mut job_queue = VecDeque::new();
    job_queue.push_back("send welcome email");
    job_queue.push_back("resize avatar");
    job_queue.push_back("update search index");

    println!("job order:");
    while let Some(job) = job_queue.pop_front() {
        println!("{job}");
    }
}
