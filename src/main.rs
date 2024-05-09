use std::{collections::HashMap, fs, path::Path, usize};

fn main() {
    let book_path = Path::new("assets/frankenstein.txt");
    let contents = fs::read_to_string(book_path);
    if let Err(err) = contents {
        panic!("Error: {}", err)
    }
    let contents = contents.unwrap();
    let word_count = count_words(&contents);
    let letter_freq = letter_frequency(&contents);
    let report_lines = generate_report_lines(book_path.to_str().unwrap(), word_count, letter_freq);
    println!("{}", report_lines.join("\n"))
}

fn count_words(contents: &str) -> usize {
    contents.split_whitespace().count()
}

fn letter_frequency(contents: &str) -> HashMap<char, usize> {
    contents
        .to_lowercase()
        .chars()
        .fold(HashMap::new(), |mut map, letter| {
            if let Some(&value) = map.get(&letter) {
                map.insert(letter, value + 1)
            } else {
                map.insert(letter, 1)
            };
            map
        })
}

fn generate_report_lines(
    filename: &str,
    word_count: usize,
    frequency_map: HashMap<char, usize>,
) -> Vec<String> {
    let header = format!("--- Begin report of {} ---", filename);
    let summary = format!("{} words found in the document", word_count);
    let mut report_items: Vec<(char, usize)> = frequency_map
        .into_iter()
        .filter(|(char, _)| char.is_alphabetic())
        .collect();

    report_items.sort_by_key(|(_, v)| -(*v as isize));

    let mut headers = Vec::from([header, summary]);
    headers.extend(report_items.iter().map(to_report_line));
    headers
}

fn to_report_line((char, count): &(char, usize)) -> String {
    format!("The {} character was found {} times", char, count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_zero() {
        assert_eq!(count_words(""), 0);
        assert_eq!(count_words("\n"), 0);
    }
    #[test]
    fn counts_two() {
        assert_eq!(count_words("String literal"), 2);
    }
    #[test]
    fn counts_multiple() {
        assert_eq!(
            count_words("This is a really big sentence with lots of words.\n And lines"),
            12
        );
    }

    #[test]
    fn freq_letter_0() {
        assert_eq!(letter_frequency(""), HashMap::from([]));
        assert!(letter_frequency("").is_empty());
    }

    #[test]
    fn freq_letter_hello_world() {
        let mut result = HashMap::with_capacity(7);
        result.insert('h', 1);
        result.insert('o', 2);
        result.insert('e', 1);
        result.insert('l', 3);
        result.insert('w', 1);
        result.insert('r', 1);
        result.insert('d', 1);
        result.insert(' ', 1);
        result.insert(',', 1);
        assert_eq!(letter_frequency("Hello, World"), result);
    }

    #[test]
    fn freq_letter_5as_and_3spaces() {
        let mut result = HashMap::with_capacity(7);
        result.insert('a', 5);
        result.insert(' ', 3);
        assert_eq!(letter_frequency("a a  aaa"), result);
        assert_eq!(letter_frequency("aa   aaa"), result);
        assert_eq!(letter_frequency("a a a aa"), result);
    }
}
