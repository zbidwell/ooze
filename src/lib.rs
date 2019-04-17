pub mod error;
pub mod app;
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
