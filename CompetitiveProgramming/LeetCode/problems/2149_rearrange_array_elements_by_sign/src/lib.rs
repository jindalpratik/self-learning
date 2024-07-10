pub fn add(left: usize, right: usize) -> usize {
    left + right
}
// TODO: Try Completing inplace code.
// pub fn rearrange_array(mut nums: Vec<i32>) -> Vec<i32> {
//     let len = nums.len();
//     for i in 0..len {
//         if (i % 2 == 0 && nums[i] > 0) || i % 2 != 0 && nums[i] < 0 {
//             continue;
//         }
//         let req_positive = if i % 2 == 0 { true } else { false };
//         for f in (i + 1)..len {
//             if f < len - 1 {
//                 if !req_positive && nums[f] < 0 && nums[f + 1] > 0 {
//                     nums.swap(i, f);
//                     break;
//                 } else if req_positive && nums[f] > 0 && nums[f + 1] < 0 {
//                     nums.swap(i, f);
//                     break;
//                 }
//             } else {
//                 if !req_positive && nums[f] < 0 {
//                     nums.swap(i, f);
//                     break;
//                 } else if req_positive && nums[f] > 0 {
//                     nums.swap(i, f);
//                     break;
//                 }
//             }
//         }
//     }

//     nums
// }

pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
    let mut new_nums = nums.clone();
    let mut pos = 0;
    let mut neg = 1;
    for i in nums {
        if i > 0 {
            new_nums[pos] = i;
            pos += 2;
        } else {
            new_nums[neg] = i;
            neg += 2;
        }
    }

    new_nums
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rearrange_array_test() {
        assert_eq!(
            rearrange_array(vec![3, 1, -2, -5, 2, -4]),
            vec![3, -2, 1, -5, 2, -4]
        );
    }
}
