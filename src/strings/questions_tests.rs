#[cfg(test)]
mod tests {
    use crate::strings::questions::*;

    #[test]
    fn dup_empty_case() {
        let input = "";

        let res = is_dup_v1(input) && is_dup_v2(input);

        assert_eq!(res, false)
    }

    #[test]
    fn dup_unique() {
        let input = "abcdefghi";

        let res = is_dup_v1(input) && is_dup_v2(input);

        assert_eq!(res, false)
    }

    #[test]
    fn simple_duplication() {
        let input = "bb";

        let res = is_dup_v1(input) && is_dup_v2(input);

        assert_eq!(res, true)
    }

    #[test]
    fn complicated_duplication() {
        let input = "abcdefghijkc";

        let res = is_dup_v1(input) && is_dup_v2(input);

        assert_eq!(res, true)
    }

    #[test]
    fn empty_permutation() {
        let str1 = "";
        let str2 = "something else";

        let res = is_permutation(str1, str2);

        assert_eq!(res, false);
    }

    #[test]
    fn diff_len_permutation() {
        let str1 = "rise";
        let str2 = "sir";

        let res = is_permutation(str1, str2);

        assert_eq!(res, false);
    }

    #[test]
    fn true_permutation() {
        let str1 = "gobin";
        let str2 = "bingo";

        let res = is_permutation(str1, str2);

        assert_eq!(res, true);
    }

    #[test]
    fn test_urlify() {
        let str1 = "Hey test awesome  something   ";

        let res = urlfy(str1);

        assert_eq!(res, "Hey%20test%20awesome%20something");
    }

    #[test]
    fn test_palindrome_permutation() {
        let str = "permumrep";

        let res = palindrome_permutation(&str);

        assert_eq!(res, true);
    }
}
