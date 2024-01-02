use std::collections::HashMap;

impl Solution {
    pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        let mut map: HashMap<i32, usize> = HashMap::new();

        for &n in &nums {
            let row_num = *map.get(&n).unwrap_or(&0);

            if res.len() <= row_num {
                res.push(vec![n]);
            } else {
                res[row_num].push(n);
            }
            map.insert(n, row_num + 1);
        }
        res
    }
}
[
