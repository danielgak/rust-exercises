#[cfg(test)]
mod tests {
    use crate::advent_2024::day02::*;

    #[test]
    fn it_evaluates_a_basic_vec_chain() {
        let vec = vec![5, 4, 2, 0];

        let (resut, position) = is_a_valid_sequence(vec);

        assert_eq!(resut, true);
        assert_eq!(position, 3);
    }

    #[test]
    fn it_says_were_it_goes_wrong() {
        let vec = vec![5, 4, 3, 4, 2];

        let (resut, position) = is_a_valid_sequence(vec);

        assert_eq!(resut, false);
        assert_eq!(position, 2);
    }

    #[test]
    fn it_detects_something_wrong_in_the_second_position() {
        let vec = vec![5, 6, 4, 3, 2];

        let (resut, position) = is_a_valid_sequence(vec);

        assert_eq!(resut, false);
        assert_eq!(position, 1);
    }

    #[test]
    fn it_detects_something_wrong_in_2nd_position() {
        let vec = vec![48, 46, 47, 49, 51, 54, 56];

        let (resut, position) = is_a_valid_sequence(vec);

        assert_eq!(resut, false);
        assert_eq!(position, 1);
    }

    #[test]
    fn it_splits_on_weird_indexes_case1() {
        let mut vec = vec![48, 46, 47, 49, 51, 54, 56];

        let vec = push_away_checked(0, &mut vec);

        assert_eq!(vec, vec![46, 47, 49, 51, 54, 56]);
    }

    #[test]
    fn it_splits_on_weird_indexes_case2() {
        let mut vec = vec![48, 46, 47, 49, 51, 54, 56];

        let vec = push_away_checked(1000, &mut vec);

        assert_eq!(vec, vec![48, 46, 47, 49, 51, 54, 56]);
    }

    #[test]
    fn it_solves_a_simple_case() {
        let input = vec![7, 6, 4, 2, 1];

        let result = search_for_valid_sequence(input);

        assert_eq!(result, true);
    }

    #[test]
    fn it_is_able_to_fix_himself_when_an_error_is_in_the_middle() {
        let vec = vec![5, 6, 4, 3, 2];

        let result = search_for_valid_sequence(vec);

        assert_eq!(result, true);
    }

    #[test]
    fn it_solves_a_problem_in_the_first_position() {
        let input = vec![100, 4, 3, 2];

        let result = search_for_valid_sequence(input);

        assert_eq!(result, true);
    }

    #[test]
    fn it_solves_a_problem_in_the_first_position_asc() {
        let input = vec![100, 12, 14, 15];

        let result = search_for_valid_sequence(input);

        assert_eq!(result, true);
    }

    #[test]
    fn it_solves_a_problem_in_the_last_position() {
        let input = vec![10, 9, 8, 2];

        let result = search_for_valid_sequence(input);

        assert_eq!(result, true);
    }

    #[test]
    fn it_solves_a_problem_in_prev_to_last_position() {
        let input = vec![10, 9, 2, 7];

        let result = search_for_valid_sequence(input);

        assert_eq!(result, true);
    }

    #[test]
    fn sometimes_it_is_not_possible_to_solve_the_problem() {
        let input = vec![10, 9, 50, 48, 8, 2];

        let result = search_for_valid_sequence(input);

        assert_eq!(result, false);
    }

    #[test]
    fn edge_case1() {
        let input = vec![1, 1, 2, 3, 4, 5];

        let result = search_for_valid_sequence(input);

        assert_eq!(result, true);
    }

    #[test]
    fn edge_case2() {
        let input = vec![48, 46, 47, 49, 51, 54, 56];

        let result = search_for_valid_sequence(input);

        assert_eq!(result, true);
    }
}
