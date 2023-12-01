mod a_welcome;

fn main() {
    match crate::a_welcome::solution() {
        Ok(i) => {
            println!("Solution 1: {}", i)
        }
        Err(e) => {
            println!("Solution 1 error: {:#}", e)
        }
    };
}
