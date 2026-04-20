
use std::collections::VecDeque;
use memory_stats::{memory_stats};

use rust_dsa::structures::deque::Deque;


#[cfg(test)]

#[test]
fn test_create_empty_deque() {
    let mut deque: Deque<i32> = Deque::new();
    let result = deque.pop();
    assert_eq!(result, None);
}

#[test]
fn test_create_unitary_deque() {
    let mut deque = Deque::new();
    deque.append(2);
    let result = deque.pop();
    assert_eq!(result, Some(2));
}

#[test]
fn test_create_deque_multiple_items() {
    let mut deque = Deque::new();
    deque.append(3);
    deque.append(6);
    deque.append(9);
    deque.append(12);
    deque.append(15);
    deque.append(42);
    let mut results = vec![];
    results.push(deque.pop().unwrap());
    results.push(deque.pop().unwrap());
    results.push(deque.pop().unwrap());
    results.push(deque.pop().unwrap());
    results.push(deque.pop().unwrap());
    results.push(deque.pop().unwrap());
    assert_eq!(results, vec![42, 15, 12, 9, 6, 3]);
}

#[test]
fn test_create_large_deque() {
    if let Some(usage) = memory_stats() {
        println!("Physical memory usage before execution: {:.2} MB", usage.physical_mem as f32 / 1_000_000f32);
    }
    let (values, deque) = fill_deque();
    if let Some(usage) = memory_stats() {
        println!("Physical memory usage after execution: {:.2} MB", usage.physical_mem as f32 / 1_000_000f32);
    }
    let results = reverse_results(deque, values.len());
    assert!(values.iter().eq(results.iter()));
}

#[test]
fn test_create_empty_queue() {
    let mut deque: Deque<i32> = Deque::new();
    let result = deque.dequeue();
    assert_eq!(result, None);
}

#[test]
fn test_create_unitary_queue() {
    let mut deque = Deque::new();
    deque.append(2);
    let result = deque.dequeue();
    assert_eq!(result, Some(2));
}

#[test]
fn test_create_queue_multiple_items() {
    let mut deque = Deque::new();
    deque.append(3);
    deque.append(6);
    deque.append(9);
    deque.append(12);
    deque.append(15);
    deque.append(42);
    let mut results = vec![];
    results.push(deque.dequeue().unwrap());
    results.push(deque.dequeue().unwrap());
    results.push(deque.dequeue().unwrap());
    results.push(deque.dequeue().unwrap());
    results.push(deque.dequeue().unwrap());
    results.push(deque.dequeue().unwrap());
    assert_eq!(results, vec![3, 6, 9, 12, 15, 42]);
}

#[test]
fn test_create_large_queue() {
    use rand::RngExt;
    let range = 10_000_000;
    let mut values: Vec<i32> = Vec::new();
    let mut rng = rand::rng();
    let mut deque: Deque<i32> = Deque::new();
    if let Some(usage) = memory_stats() {
        println!("Physical memory usage before execution: {:.2} MB", usage.physical_mem as f32 / 1_000_000f32);
    }
    for _ in 0..range {
        let n = rng.random_range(0..=range);
        values.push(n);
        deque.append(n);
    }
    if let Some(usage) = memory_stats() {
        println!("Physical memory usage after execution: {:.2} MB", usage.physical_mem as f32 / 1_000_000f32);
    }
    let mut results: Vec<i32> = Vec::new();
    for _ in 0..range {
        results.push(deque.dequeue().unwrap());
    }
    assert!(results.iter().eq(values.iter()));
}

fn fill_deque() -> (Vec<i32>, Deque<i32>) {
    use rand::RngExt;
    let range = 10_000_000;
    let mut values: Vec<i32> = Vec::new();
    let mut rng = rand::rng();
    let mut deque: Deque<i32> = Deque::new();
    for _ in 0..range {
        let n = rng.random_range(0..=range);
        values.push(n);
        deque.append(n);
    }
    (values, deque)
}

fn reverse_results(mut deque: Deque<i32>, length: usize) -> VecDeque<i32> {
    use std::collections::VecDeque;
    let mut results: VecDeque::<i32> = VecDeque::new();
    for _ in 0..length {
        results.push_front(deque.pop().unwrap());
    }
    results
}
