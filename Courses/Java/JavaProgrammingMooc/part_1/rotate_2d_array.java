public class rotate_2d_array {
    public static void main(String[] args) {
        int[][] matrix = {
            {1, 2, 3},
            {4, 5, 6},
            {7, 8, 9}
        };
        
        // Rotate the matrix by 90 degrees
        int[][] rotatedMatrix90 = rotate90Degrees(matrix);
        System.out.println("Matrix rotated by 90 degrees:");
        printMatrix(rotatedMatrix90);
        
        // Rotate the matrix by 180 degrees
        int[][] rotatedMatrix180 = rotate90Degrees(rotatedMatrix90);
        System.out.println("Matrix rotated by 180 degrees:");
        printMatrix(rotatedMatrix180);
        
        // Rotate the matrix by 270 degrees
        int[][] rotatedMatrix270 = rotate90Degrees(rotatedMatrix180);
        System.out.println("Matrix rotated by 270 degrees:");
        printMatrix(rotatedMatrix270);
    }
    
    // Method to rotate the matrix by 90 degrees
    public static int[][] rotate90Degrees(int[][] matrix) {
        int rows = matrix.length;
        int columns = matrix[0].length;
        int[][] rotatedMatrix = new int[columns][rows];
        
        for (int i = 0; i < rows; i++) {
            for (int j = 0; j < columns; j++) {
                rotatedMatrix[j][rows - 1 - i] = matrix[i][j];
            }
        }
        
        return rotatedMatrix;
    }
    
    // Method to print the matrix
    public static void printMatrix(int[][] matrix) {
        for (int i = 0; i < matrix.length; i++) {
            for (int j = 0; j < matrix[i].length; j++) {
                System.out.print(matrix[i][j] + " ");
            }
            System.out.println();
        }
    }
}