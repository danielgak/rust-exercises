#[cfg(test)]
mod tests {
    use crate::advent_2024::day03::*;

    #[test]
    fn it_does_not_multiply_this() {
        let str: &str = "where(1,1)";

        let total = solve_str(str);

        assert_eq!(total, 0)
    }

    #[test]
    fn it_mutiplies_case1() {
        let str: &str = "mul(964,991)how()mul(764,507)where(487,815)";

        let total = solve_str(str);

        assert_eq!(total, (964 * 991) + (764 * 507))
    }
}
