pub fn add_three(x: i32) -> i32 {
    x + 3
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
