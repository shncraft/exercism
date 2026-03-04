use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    // Best and efficient approach
    return input.graphemes(true).rev().collect();

    // Two pointer approach
    // if input.is_empty() {
    //     return String::from("");
    // }
    // let mut chars: Vec<&str> = input.graphemes(true).collect();
    // let (mut left, mut right) = (0, chars.len() - 1);

    // while left < right {
    //     chars.swap(left, right);
    //     left += 1;
    //     right -= 1;
    // }
    // let str: String = chars.into_iter().collect();
    // str
}
