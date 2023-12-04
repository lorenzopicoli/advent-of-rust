mod a_welcome;
mod b_to;
mod c_advent;
mod d_of;

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
    match crate::c_advent::solution() {
        Ok(i) => {
            println!("Solution 3: {}", i)
        }
        Err(e) => {
            println!("Solution 3 error: {:#}", e)
        }
    };
    match crate::d_of::solution() {
        Ok(i) => {
            println!("Solution 4: {}", i)
        }
        Err(e) => {
            println!("Solution 4 error: {:#}", e)
        }
    };
}
