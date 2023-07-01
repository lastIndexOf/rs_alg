pub struct Solution;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut res = Vec::new();
        let path = path.split("/");

        for subpath in path {
            match subpath {
                "" | "/" | "." => continue,
                ".." => {
                    if !res.is_empty() {
                        res.pop();
                    }
                }
                val => res.push(val),
            };
        }

        format!("/{}", res.join("/"))
    }
}

#[cfg(test)]
mod seventy_one_test {
    use super::*;

    #[test]
    fn test_seventy_one() {
        println!("{:?}", "/2".split("/").collect::<Vec<_>>());
        assert_eq!(Solution::simplify_path("/home/".to_string()), "/home");
        assert_eq!(Solution::simplify_path("/../".to_string()), "/");
        assert_eq!(
            Solution::simplify_path("/home//foo/".to_string()),
            "/home/foo"
        );
        assert_eq!(Solution::simplify_path("/a/./b/../../c/".to_string()), "/c");
    }
}
