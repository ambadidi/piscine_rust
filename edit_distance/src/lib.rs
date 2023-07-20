//FROM: https://github.com/QMHTMY/RustBook/blob/0b42a56489b6de36d6353f9f6171ea8720f1ff78/publication/code/chapter10/edit_distance.rs#L5
use std::cmp::min;
pub fn edit_distance(source: &str, target: &str) -> usize {
	// 极端情况：空字符串到字符串的转换
    if source.is_empty() {
        return target.len();
    } else if target.is_empty() {
        return source.len();
    }

    // distances 存储了到各种字符串的编辑距离
    let target_c = target.chars().count();
    let mut distances = (0..=target_c).collect::<Vec<_>>();
    for (i, cs) in source.chars().enumerate() {
        let mut substt = i;
        distances[0] = substt + 1;

        // 不断组合计算各个距离
        for (j, ct) in target.chars().enumerate() {
            let dist = min(min(distances[j], distances[j+1])+1,
                           substt + (cs != ct) as usize);
            substt = distances[j+1];
            distances[j+1] = dist;
        }
    }

    // 最后一个距离值就是最终答案
    distances.pop().unwrap()
}