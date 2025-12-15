pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
pub mod animal;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::animal::cat;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn use_cat() {
        cat::hello();
        assert_eq!(true, cat::is_cat());
    }
}
