//! assumptions:
//! * input breaks on eof (ctr+d)
//! * input words are delimited by spaces and newlines
//! * remove punctuaiton
//! * all letters should be converted to lowercase
//! * hyphenated words count as new word


use std::io::{BufRead, BufReader, Read, stdin};
use std::collections::HashMap;
use std::iter::FromIterator;


fn main() {

    // get the list of words froms
    let words = read_words(stdin());

    // stem the words in the vector
    let cleaned_words = clean_words(words);

    // create a hashmap with frequencies
    let frequencies = count_frequencies(cleaned_words);

    for f in &frequencies {
        println!("{}: {}", f.0, f.1);
    }

}


fn read_words<R: Read>(reader: R) -> Vec<String> {
    //! reads words from stdin and out puts a vector containing the words
    //! delimited by spaces

    let mut words: Vec<String> = vec![];

    let mut lines = BufReader::new(reader).lines();

    while let Some(Ok(line)) = lines.next() {

        let mut split_line = line.split(" ");

        for s in split_line {
            words.push(s.to_string());
        }

    }

    words

}

#[cfg(test)]
mod read_words_tests {

    use super::read_words;
    use std::io::Cursor;

    #[test]
    fn reads() {
        assert_read(&["hello", "world,", "bye", "world"], "hello world,\nbye world");
    }

    fn assert_read(expected: &[&str], input: &str) {
        let mock_read = Cursor::new(input);
        let words = read_words(mock_read);
        assert_eq!(expected.to_owned(), words);
    }

}


fn clean_words(words: Vec<String>) -> Vec<String> {
    //! cleans additional characters from words

    let mut cleaned_words: Vec<String> = vec![];

    let punctuaiton = ['(', ')', '?', ':', ';', ',', '.', '!', '/', '"', '\'', '”', '“'];

    for w in words {

        let mut new_word = "".to_string();
        'char_loop: for c in w.chars() {

            for p in &punctuaiton {

                // if the character is in punctuaiton or a letter
                // don't add it to the word
                if c == *p || c.is_numeric() {
                    continue 'char_loop;
                }

            }

            // new_word = new_word + &c.to_lowercase().next().expect("Could not get character").to_string(); // this would make things lowercase...kind of
                                                                                                             // but then there's the problem if its not the first
                                                                                                             // in the iterator so not using it
            new_word = new_word + &c.to_string();

        }

        // if the word is blank don't add it
        if new_word.len() == 0 {
            continue;
        }

        cleaned_words.push(new_word);

    }

    cleaned_words

}


#[cfg(test)]
mod cleaned_words_tests {

    use super::clean_words;

    #[test]
    fn blank() {
        let expected: Vec<String> = vec![];
        let empty: Vec<String> = vec![];
        assert_eq!(expected, clean_words(empty));
    }

    #[test]
    fn basic() {

        let mut expected: Vec<String> = vec![];
        expected.push("hello".to_string());
        expected.push("world".to_string());

        let mut basic: Vec<String> = vec![];
        basic.push("hello".to_string());
        basic.push("world".to_string());

        assert_eq!(expected, clean_words(basic));
    }



    #[test]
    fn punctuation() {

        let mut expected: Vec<String> = vec![];
        expected.push("hello".to_string());
        expected.push("world".to_string());

        let mut punc: Vec<String> = vec![];
        punc.push("hello,!'".to_string());
        punc.push("world".to_string());

        assert_eq!(expected, clean_words(punc));
    }

}


fn count_frequencies(words: Vec<String>) -> Vec<(String, i64)> {
    //! takes in a vector of words creates a hashmap with the frequencies
    //! of the words from the vector

    let mut frequencies: HashMap<String, i64> = HashMap::new();

    for w in &words {

        // fixed this hopefully?
        let count: i64 = *frequencies.entry(w.to_string()).or_insert(0);
        frequencies.insert(w.to_string(), count + 1);

    }

    let mut ordered = Vec::from_iter(frequencies);
    ordered.sort_by(|&(_, a), &(_, b)| b.cmp(&a)); // this is pretty much from stackoverflow gonna be honest
                                                   // but I understand what's going on

    ordered

}

#[cfg(test)]
mod count_frequencies_tests {

    use super::count_frequencies;

    #[test]
    fn blank() {
        fn blank() {
            let expected: Vec<(String, i64)> = vec![];
            let empty: Vec<String> = vec![];
            assert_eq!(expected[0], count_frequencies(empty)[0]);
        }
    }

    #[test]
    fn count() {
        let mut expected: Vec<(String, i64)> = vec![];
        expected.push(("world".to_string(), 2));
        expected.push(("hello".to_string(), 1));

        let mut words: Vec<String> = vec![];
        words.push("hello".to_string());
        words.push("world".to_string());
        words.push("world".to_string());

        assert_eq!(expected[0].0, "world");
        assert_eq!(expected[0].1, 2);

        assert_eq!(expected[1].0, "hello");
        assert_eq!(expected[1].1, 1);

    }

}
