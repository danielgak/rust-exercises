#[cfg(test)]
mod tests {
    use crate::strings::questions::*;

    #[test]
    fn dup_empty_case() {
        let input = "";

        let res = dup_v2(input);

        assert_eq!(res, true)
    }

    #[test]
    fn dup_unique() {
        let input = "abcdefghi";

        let res = dup_v2(input);

        assert_eq!(res, true)
    }

    #[test]
    fn simple_duplication() {
        let input = "bb";

        let res = dup_v2(input);

        assert_eq!(res, true)
    }

    #[test]
    fn complicated_duplication() {
        let input = "abcdefghijkc";

        let res = dup_v2(input);

        assert_eq!(res, true)
    }

}