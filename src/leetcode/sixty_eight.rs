pub struct Solution;

impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let mut res = Vec::new();

        let mut words = words.iter();
        let mut current = String::from("");

        loop {
            match words.next() {
                Some(next) => {
                    if next.len() + current.len() <= max_width as usize {
                        current.push_str(next);

                        if current.len() < max_width as usize {
                            current.push(' ');
                        }
                    } else {
                        let arr = current.trim().split_whitespace();
                        let letter_len = arr.clone().map(|item| item.len()).sum::<usize>();
                        let mut rest_len = max_width - letter_len as i32;
                        let mut arr = arr.map(|item| item.to_string()).collect::<Vec<_>>();

                        let mut spaces = vec![0; arr.len() - 1];

                        if spaces.len() == 0 {
                            arr[0] = format!("{}{}", arr[0], " ".repeat(rest_len as usize));
                        } else {
                            while rest_len > 0 {
                                for space in spaces.iter_mut() {
                                    *space += 1;
                                    rest_len -= 1;

                                    if rest_len <= 0 {
                                        break;
                                    }
                                }
                            }

                            for (idx, space) in spaces.iter().enumerate() {
                                arr[idx] = format!("{}{}", arr[idx], " ".repeat(*space as usize));
                            }
                        }

                        res.push(arr.drain(..).collect::<String>());
                        current = next.to_string();
                        if current.len() < max_width as usize {
                            current.push(' ');
                        }
                    }
                }
                None => {
                    current = format!(
                        "{}{}",
                        current,
                        " ".repeat(max_width as usize - current.len())
                    );
                    res.push(current.drain(..).collect::<String>());
                    break;
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod sixty_eight_test {
    use super::*;

    #[test]
    fn test_sixty_eight() {
        assert_eq!(
            Solution::full_justify(
                vec![
                    "This".to_string(),
                    "is".to_string(),
                    "an".to_string(),
                    "example".to_string(),
                    "of".to_string(),
                    "text".to_string(),
                    "justification.".to_string(),
                ],
                16
            ),
            ["This    is    an", "example  of text", "justification.  "]
        );
        assert_eq!(
            Solution::full_justify(
                vec![
                    "Science".to_string(),
                    "is".to_string(),
                    "what".to_string(),
                    "we".to_string(),
                    "understand".to_string(),
                    "well".to_string(),
                    "enough".to_string(),
                    "to".to_string(),
                    "explain".to_string(),
                    "to".to_string(),
                    "a".to_string(),
                    "computer.".to_string(),
                    "Art".to_string(),
                    "is".to_string(),
                    "everything".to_string(),
                    "else".to_string(),
                    "we".to_string(),
                    "do".to_string(),
                ],
                20
            ),
            [
                "Science  is  what we",
                "understand      well",
                "enough to explain to",
                "a  computer.  Art is",
                "everything  else  we",
                "do                  "
            ]
        );
        // assert_eq!(
        //     Solution::full_justify(
        //         vec![
        //             "What".to_string(),
        //             "must".to_string(),
        //             "be".to_string(),
        //             "acknowledgment".to_string(),
        //             "shall".to_string(),
        //             "be".to_string()
        //         ],
        //         16,
        //     ),
        //     ["What   must   be", " acknowledgment ", "shall be        "]
        // );
        assert_eq!(
            Solution::full_justify(
                vec![
                    "What".to_string(),
                    "must".to_string(),
                    "be".to_string(),
                    "acknowledgment".to_string(),
                    "shall".to_string(),
                    "be".to_string()
                ],
                16,
            ),
            ["What   must   be", "acknowledgment  ", "shall be        "]
        )
    }
}
