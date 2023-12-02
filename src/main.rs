mod a_welcome;
mod b_to;

fn main() {
    match crate::a_welcome::solution() {
        Ok(i) => {
            println!("Solution 1: {}", i)
        }
        Err(e) => {
            println!("Solution 1 error: {:#}", e)
        }
    };
    match crate::b_to::solution() {
        Ok(i) => {
            println!("Solution 2: {}", i)
        }
        Err(e) => {
            println!("Solution 2 error: {:#}", e)
        }
    };
}
