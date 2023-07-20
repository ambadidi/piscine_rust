pub fn bubble_sort(vec: &mut Vec<i32>) {
    let n = vec.len();
    for i in 1..n+1 {
        for j in 0..n - i {
            if vec[j] > vec[j+1] {
                vec.swap(j, j+1)
            }
        }
    }
}