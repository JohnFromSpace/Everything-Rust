use std::collections::HashMap;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut answers = HashMap::<char, usize>::new();
    
    if input.is_empty() {
        return answers;
    }
    
    let input = input.join("");
    
    if input.len() == 0 {
        return answers;
    }
    
    let mut churn = input.chars();
    let real_worker_count = min(input.len(), worker_count);
    let mut thread_pool = Vec::with_capacity(real_worker_count);
    let mut work_length = (input.len() / real_worker_count).max(1);
    
    if work_length * real_worker_count < input.len() {
        work_length = work_length + 1;
    }
    
    for _ in 0..real_worker_count {
        let chunk = churn.by_ref().take(work_length).collect::<String>();
        
        let my_thread = thread::spawn(move || {
            let mut answer = HashMap::<char, usize>::new();
            chunk.chars().for_each(|c| {
                if c.is_alphabetic() {
                    *answer.entry(c.to_ascii_lowercase()).or_default() += 1;
                }
                
            });
            
            answer
        });
        
        thread_pool.push(my_thread);
    }
    
    for my_thread in thread_pool {
        let answer = my_thread.join().unwrap();
       
        for (key, val) in answer.iter() {
            *answers.entry(*key).or_default() += val;
        }
    }
    answers
}
