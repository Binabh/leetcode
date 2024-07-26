#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut row: HashMap<usize, String> = HashMap::new();
        let mut column: HashMap<usize, String> = HashMap::new();
        let mut grid: HashMap<Vec<usize>, String> = HashMap::new();
        for (i, v) in board.iter().enumerate() {
            for (j, c) in v.iter().enumerate() {
                if *c == '.' {
                    continue;
                }
                let grid_row = j / 3;
                let grid_col = i / 3;
                match row.get(&i) {
                    Some(val) => {
                        if val.contains([*c]) {
                            return false;
                        }
                        let mut new_str = val.to_string();
                        new_str.push(*c);
                        row.insert(i, new_str);
                    }
                    None => {
                        row.insert(i, c.to_string());
                    }
                }
                match column.get(&j) {
                    Some(val) => {
                        if val.contains([*c]) {
                            return false;
                        }
                        let mut new_str = val.to_string();
                        new_str.push(*c);
                        column.insert(j, new_str);
                    }
                    None => {
                        column.insert(j, c.to_string());
                    }
                }
                match grid.get(&vec![grid_row, grid_col]) {
                    Some(val) => {
                        if val.contains([*c]) {
                            return false;
                        }
                        let mut new_str = val.to_string();
                        new_str.push(*c);
                        grid.insert(vec![grid_row, grid_col], new_str);
                    }
                    None => {
                        grid.insert(vec![grid_row, grid_col], c.to_string());
                    }
                }
            }
        }

        true
    }
    #[test]
    fn test() {
        assert_eq!(
            is_valid_sudoku(vec![
                vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
            ]),
            true
        );
        assert_eq!(
            is_valid_sudoku(vec![
                vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '8', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
            ]),
            false
        );
        assert_eq!(
            is_valid_sudoku(vec![
                vec!['.', '8', '7', '6', '5', '4', '3', '2', '1'],
                vec!['2', '.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['3', '.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['4', '.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['5', '.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['6', '.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['8', '.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['7', '.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['9', '.', '.', '.', '.', '.', '.', '.', '.']
            ]),
            true
        )
    }
}
