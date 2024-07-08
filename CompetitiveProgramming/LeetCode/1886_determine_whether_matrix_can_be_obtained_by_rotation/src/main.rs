fn main() {
    let mat = vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]];
    let target = vec![vec![1, 1, 1], vec![0, 1, 0], vec![0, 0, 0]];
    println!("{}", find_rotation(mat, target));
}

pub fn find_rotation(mut mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
    if &mat == &target {
        return true;
    }
    for _ in 0..3 {
        mat = rotate_mat(mat.clone());
        if &mat == &target {
            return true;
        }
    }
    false
}

pub fn rotate_mat(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let size = mat.len();
    let mut new_mat: Vec<Vec<i32>> = mat.clone();
    for i in 0..size {
        for f in 0..size {
            new_mat[f][size - i - 1] = mat[i][f]
        }
    }
    new_mat
}
