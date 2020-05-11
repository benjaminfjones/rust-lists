pub mod fifth; // mutable queue using only boxes and &mut
pub mod fifth_unsafe; // mutable queue using raw pointers
pub mod first; // a naive stack
pub mod second; // an Ok, generic stack
pub mod third; // a persistent singly-linked stack

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
