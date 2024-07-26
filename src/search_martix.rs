#[cfg(test)]
mod tests {
    fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut row_left_pointer: usize = 0;
        let mut row_right_pointer: usize = matrix.len();
        while row_left_pointer < row_right_pointer {
            let target_row = (row_left_pointer + row_right_pointer) / 2;
            if target < *matrix[target_row].first().unwrap() {
                row_right_pointer = target_row;
            } else if target > *matrix[target_row].last().unwrap() {
                row_left_pointer = target_row;
            } else if target >= *matrix[target_row].first().unwrap()
                && target <= *matrix[target_row].last().unwrap()
            {
                let mut left_pointer: usize = 0;
                let mut right_pointer: usize = matrix[target_row].len();
                while left_pointer < right_pointer {
                    let target_index = (left_pointer + right_pointer) / 2;
                    if matrix[target_row][target_index] < target {
                        left_pointer = target_index;
                    } else if matrix[target_row][target_index] > target {
                        right_pointer = target_index;
                    } else {
                        return true;
                    }
                    if target_index == (left_pointer + right_pointer) / 2 {
                        break;
                    }
                }
            } else {
                break;
            }
            if target_row == (row_left_pointer + row_right_pointer) / 2 {
                break;
            }
        }
        false
    }
    #[test]
    fn test() {
        assert_eq!(
            search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
                3
            ),
            true
        );
        assert_eq!(
            search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
                13
            ),
            false
        );
        assert_eq!(search_matrix(vec![vec![1]], 1), true);
        assert_eq!(
            search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
                10
            ),
            true
        );
        assert_eq!(
            search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
                60
            ),
            true
        );
        assert_eq!(
            search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
                1
            ),
            true
        );
        assert_eq!(
            search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
                9
            ),
            false
        );
        assert_eq!(
            search_matrix(
                vec![vec![2, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
                1
            ),
            false
        );
        assert_eq!(
            search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
                90
            ),
            false
        );
    }
}
