use regex::Regex;

pub fn fibonacci(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    let mut a = 0;
    let mut b = 1;
    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}

pub fn extract_numbers_from_text(text: &str) -> Vec<u64> {
    let re = Regex::new(r"\b\d+\b").unwrap();
    re.captures_iter(text)
        .filter_map(|cap| cap.get(0))
        .filter_map(|num| num.as_str().parse::<u64>().ok())
        .collect()
}

pub fn compute_fibonacci_for_pr_content(text: &str) -> Vec<(u64, u64)> {
    let numbers = extract_numbers_from_text(text);
    numbers.into_iter().map(|num| (num, fibonacci(num))).collect()
}
