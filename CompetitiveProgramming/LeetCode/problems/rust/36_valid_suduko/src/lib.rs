use std::collections::HashSet;

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut row_sets = vec![HashSet::new(); 9];
    let mut col_sets = vec![HashSet::new(); 9];
    let mut box_sets = vec![HashSet::new(); 9];

    for i in 0..9 {
        for j in 0..9 {
            if board[i][j] == '.' {
                continue;
            }

            if row_sets[i].contains(&board[i][j])
                || col_sets[j].contains(&board[i][j])
                || box_sets[3 * (i / 3) + j / 3].contains(&board[i][j])
            {
                return false;
            }

            row_sets[i].insert(&board[i][j]);
            col_sets[j].insert(&board[i][j]);
            box_sets[3 * (i / 3) + j /3].insert(&board[i][j]);
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vlaid_suduko_test() {
        let board = vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];

        assert_eq!(is_valid_sudoku(board), false);
    }
}
