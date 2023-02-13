use a_short_fibonacci_sequence::fibonacci;

fn test_empty() {
    assert_eq!(create_empty(), Vec::new());
}

fn test_buffer() {
    for n in 0..10 {
        let zeroized = create_buffer(n);
        assert_eq!(zeroized.len(), n);
        assert!(zeroized.iter().all(|&v| v == 0));
    }
}

fn test_fibonacci() {
    let fibb = fibonacci();
    assert_eq!(fibb.len(), 5);
    assert_eq!(fibb[0], 1);
    assert_eq!(fibb[1], 1);
    for window in fibb.windows(3) {
        assert_eq!(window[0] + window[1], window[2]);
    }
}

fn main() {
    println!("{}", test_empty());
    println!("{}", test_buffer());
    println!("{}", test_fibonacci());
}
