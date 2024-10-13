use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let reversed_string = input.graphemes(true).rev();
    reversed_string.collect()
}
