
#[cfg(test)]
mod tests{
    #[test]
    fn convert_bool_to_string() {
        let my_bool: bool = true;
        let my_string: String = my_bool.to_string();
        println!("type: {}", my_string);
    }

    #[test]
    fn invert_bool() {
        let my_bool = false;
        assert_eq!("true", (!my_bool).to_string());
    }
}
