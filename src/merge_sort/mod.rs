#![warn(unreachable_pub, unused)]
#![warn(clippy::all, clippy::pedantic, clippy::cargo, clippy::nursery)]

pub fn merge_sort(vec: &Vec<i32>) -> Vec<i32> {
    let len = vec.len();
    if vec.len() < 2 {
        return vec.clone();
    }
    let size = len / 2;
    let left = merge_sort(&vec[0..size].to_vec());
    let right = merge_sort(&vec[size..].to_vec());
    merge(&left, &right)
}

pub fn merge(vec_c: &Vec<i32>, vec_d: &Vec<i32>) -> Vec<i32> {
    let mut i = 0;
    let mut j = 0;
    let mut vec = Vec::new();
    while i < vec_c.len() && j < vec_d.len() {
        if vec_c[i] < vec_d[j] {
            vec.push(vec_c[i]);
            i += 1;
        } else {
            vec.push(vec_d[j]);
            j += 1;
        }
    }
    while i < vec_c.len() {
        vec.push(vec_c[i]);
        i += 1;
    }
    while j < vec_d.len() {
        vec.push(vec_d[j]);
        j += 1;
    }
    vec
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(merge_sort(&vec![5, 8, 9, 1]), vec![1, 5, 8, 9]);
        assert_eq!(
            merge_sort(&vec![1, 9, 5115, 3, 16, 999, 24]),
            vec![1, 3, 9, 16, 24, 999, 5115]
        )
    }
}
