use std::cmp::{max, min};

fn main() {
    let bottom_left = vec![vec![1, 4], vec![1, 1], vec![3, 8]];
    let top_right = vec![vec![6, 9], vec![6, 4], vec![8, 10]];
    println!("{}", largest_square_area(bottom_left, top_right));
}

pub fn largest_square_area(bottom_left: Vec<Vec<i32>>, top_right: Vec<Vec<i32>>) -> i64 {
    let num_rectangles = bottom_left.len();
    let mut max_square = 0;
    for i in 0..num_rectangles {
        for j in (i + 1)..num_rectangles {
            if (bottom_left[i][0] < top_right[j][0] && bottom_left[i][1] < top_right[j][1])
                && (top_right[i][0] > bottom_left[j][0] && top_right[i][1] > bottom_left[j][1])
            {
                let mut rect = Vec::new();
                rect.push(max(bottom_left[i][0], bottom_left[j][0]));
                rect.push(max(bottom_left[i][1], bottom_left[j][1]));
                rect.push(min(top_right[i][0], top_right[j][0]));
                rect.push(min(top_right[i][1], top_right[j][1]));
                // dbg!(&rect);
                let square_len = min((rect[0] - rect[2]).abs(), (rect[1] - rect[3]).abs());
                max_square = max(square_len * square_len, max_square);
                // dbg!(&max_square, &i, &j);
            }
        }
    }
    max_square.into()
}
