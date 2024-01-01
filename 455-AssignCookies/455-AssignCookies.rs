impl Solution {
    pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
        let mut content_children = 0;

        let mut greed_factors = g.clone();
        let mut cookie_sizes = s.clone();

        greed_factors.sort();
        cookie_sizes.sort();

        let mut i = 0;
        let mut j = 0;

        while i < greed_factors.len() && j < cookie_sizes.len() {
            if cookie_sizes[j] >= greed_factors[i] {
                content_children += 1;
                i += 1;
            }
            j += 1;
        }

        content_children
    }
}
