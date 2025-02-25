mod fib;  // Add this line to include the fib.rs module

use regex::Regex;
use fib::fibonacci; // Use the fibonacci function from the fib.rs module

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

fn format_fibonacci_comment(results: Vec<(u64, u64)>) -> String {
    let mut comment = "Here are the Fibonacci numbers for the numbers found in this PR:\n".to_string();
    for (num, fib) in results {
        comment.push_str(&format!("- Fibonacci({}): {}\n", num, fib));
    }
    comment
}

fn main() {
    let pr_content = "This PR includes the numbers 5, 8, and 13. We need Fibonacci for them!";
    
    // Compute Fibonacci numbers for all numbers in the PR content
    let fibonacci_results = compute_fibonacci_for_pr_content(pr_content);
    
    // Format the results into a comment
    let comment = format_fibonacci_comment(fibonacci_results);
    
    // Print the comment (this will be posted to GitHub later)
    println!("{}", comment);
}
