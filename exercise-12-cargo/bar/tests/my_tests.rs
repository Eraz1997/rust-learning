#[cfg(test)]
mod tests {
    use bar::add;

    #[test]
    fn integration_it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}