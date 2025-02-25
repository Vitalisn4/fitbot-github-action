mod fib; // Importing the fib.rs module

fn format_fibonacci_comment(results: Vec<(u64, u64)>) -> String {
    let mut comment = "Here are the Fibonacci numbers for the numbers found in this PR:\n".to_string();
    for (num, fib) in results {
        comment.push_str(&format!("- Fibonacci({}): {}\n", num, fib));
    }
    comment
}

fn main() {
    // Simulating a pull request body (for testing purposes)
    let pr_content = "This PR includes the numbers 5, 8, and 13. We need Fibonacci for them!";
    
    // Compute Fibonacci numbers for all numbers in the PR content
    let fibonacci_results = fib::compute_fibonacci_for_pr_content(pr_content);
    
    // Format the results into a comment
    let comment = format_fibonacci_comment(fibonacci_results);
    
    // Print the comment (this would be posted to GitHub later)
    println!("{}", comment);
}
