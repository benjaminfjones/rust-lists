pub mod fifth;
pub mod first; // a bad stack
pub mod second; // an Ok, generic, singly-linked stack
pub mod third; // a persistent singly-linked stack

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
