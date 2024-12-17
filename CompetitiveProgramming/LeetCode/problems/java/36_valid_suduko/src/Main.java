import java.util.HashSet;

public class Main {
    public static void main(String[] args) {
        char[][] board = {
                { '8', '3', '.', '.', '7', '.', '.', '.', '.' },
                { '6', '.', '.', '1', '9', '5', '.', '.', '.' },
                { '.', '9', '8', '.', '.', '.', '.', '6', '.' },
                { '8', '.', '.', '.', '6', '.', '.', '.', '3' },
                { '4', '.', '.', '8', '.', '3', '.', '.', '1' },
                { '7', '.', '.', '.', '2', '.', '.', '.', '6' },
                { '.', '6', '.', '.', '.', '.', '2', '8', '.' },
                { '.', '.', '.', '4', '1', '9', '.', '.', '5' },
                { '.', '.', '.', '.', '8', '.', '.', '7', '9' }
        };
        System.out.println(isValidSudoku(board));
    }

    @SuppressWarnings("unchecked")
    public static boolean isValidSudoku(char[][] board) {
        HashSet<Character>[] rowSets = new HashSet[9];
        HashSet<Character>[] colSets = new HashSet[9];
        HashSet<Character>[] boxSets = new HashSet[9];

        for (int i = 0; i < 9; i++) {
            rowSets[i] = new HashSet<>();
            colSets[i] = new HashSet<>();
            boxSets[i] = new HashSet<>();
        }

        for (int i = 0; i < 9; i++) {
            for (int j = 0; j < 9; j++) {
                if (board[i][j] == '.') {
                    continue;
                }

                if (rowSets[i].contains(board[i][j]) || colSets[j].contains(board[i][j])
                        || boxSets[3 * (i / 3) + j / 3].contains(board[i][j])) {
                    return false;
                }

                rowSets[i].add(board[i][j]);
                colSets[j].add(board[i][j]);
                boxSets[3 * (i / 3) + j / 3].add(board[i][j]);
            }
        }
        return true;
    }
}
