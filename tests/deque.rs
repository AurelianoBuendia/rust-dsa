
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
    use rand::RngExt;
    let range = 1000;
    let mut values: Box<Vec<i32>> = Box::new(Vec::new());
    let mut rng = rand::rng();
    let mut deque: Box<Deque<i32>> = Box::new(Deque::new());
    for _ in 0..range {
        let n = rng.random_range(0..=range);
        values.push(n);
        deque.append(n);
    }
    let mut results: Box<Vec<i32>> = Box::new(Vec::new());
    for _ in 0..range {
        results.push(deque.pop().unwrap());
    }
    values.reverse();
    assert_eq!(*results, *values);
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
    let range = 1000;
    let mut values: Box<Vec<i32>> = Box::new(Vec::new());
    let mut rng = rand::rng();
    let mut deque: Box<Deque<i32>> = Box::new(Deque::new());
    for _ in 0..range {
        let n = rng.random_range(0..=range);
        values.push(n);
        deque.append(n);
    }
    let mut results: Box<Vec<i32>> = Box::new(Vec::new());
    for _ in 0..range {
        results.push(deque.dequeue().unwrap());
    }
    assert_eq!(*results, *values);
}
