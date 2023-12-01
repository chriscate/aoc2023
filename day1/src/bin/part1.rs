fn main() {
    println!("hello from part 1");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn my_test() {
        let x = 5 + 5;
        
        assert_eq!(x, 10);
    }
}
