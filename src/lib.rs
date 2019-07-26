pub mod app;
pub mod error;
pub mod geometry;
pub mod graphics;
pub mod terminal;

#[cfg(test)]
mod tests {
    #[test]
    fn basic() {
        assert_eq!(true, true);
    }
}
