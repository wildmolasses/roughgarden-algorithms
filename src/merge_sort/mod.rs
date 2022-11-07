pub fn merge_sort(vec: Vec<usize>) -> Vec<usize> {
    let len = vec.len();
    if vec.len() < 2 {
        return vec;
    }
    let vecs: Vec<Vec<usize>> = vec
        .chunks((len + 2 - 1) / 2)
        .map(|x| merge_sort(x.to_vec()))
        .collect();
    merge(&vecs[0], &vecs[1])
}

pub fn merge(vec_c: &Vec<usize>, vec_d: &Vec<usize>) -> Vec<usize> {
    let mut i = 0;
    let mut j = 0;
    let mut vec = Vec::new();
    for _k in 0..vec_c.len() + vec_d.len() {
        if vec_c.get(i).is_none() {
            vec.push(vec_d[j]);
            j += 1;
        } else if vec_d.get(j).is_none() || vec_c.get(i) < vec_d.get(j) {
            vec.push(vec_c[i]);
            i += 1;
        } else {
            vec.push(vec_d[j]);
            j += 1;
        }
    }
    vec
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(merge_sort(vec![5, 8, 9, 1]), vec![1, 5, 8, 9]);
        assert_eq!(
            merge_sort(vec![1, 9, 5115, 3, 16, 999, 24]),
            vec![1, 3, 9, 16, 24, 999, 5115]
        )
    }
}
