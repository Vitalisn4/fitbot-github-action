use tokio; 


fn fibonacci(n: u64) -> u64 {
    if n <= 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

#[tokio::main]
async fn main() {
    
    let numbers = vec![5, 8, 13];
    
    for num in numbers {
        let fib_result = fibonacci(num);
        println!("Fibonacci of {}: {}", num, fib_result);
    }

   
}

