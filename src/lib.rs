mod core;
mod platform;

#[cfg(test)]
mod tests {
    use crate::platform;

    #[test]
    fn test_app() {
        assert_eq!(platform::init(), true);
    }
}
