//! # Pig Latin
//!
//! Crate to convert text into pig latin, for excersise in Rust lang book
//! in [chapter 8 summary](https://doc.rust-lang.org/book/ch08-03-hash-maps.html#summary).
//!
//! > Convert strings to pig latin. The first consonant of each word is
//! > moved to the end of the word and “ay” is added, so “first” becomes
//! > “irst-fay.” Words that start with a vowel have “hay” added to the
//! > end instead (“apple” becomes “apple-hay”). Keep in mind the details
//! > about UTF-8 encoding!

const VOWELS: [char; 10] = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];

/// Convert some **English** text to Pig latin word by word.
/// 
/// This function **doesn't** correctly handle punctuation,
/// see [punctuation example](#punctuation), see [`convert_word`]
/// for converting single words.
/// 
/// # Examples 
/// ```
/// use piglatin::convert;
///
/// let text = "This text should be converted to Pig Latin";
/// let output = "his-Tay ext-tay hould-say e-bay onverted-cay o-tay ig-Pay atin-Lay";
///
/// assert_eq!(convert(text), output);
/// ```
/// 
/// ## Punctuation
/// Incorrectly handling punctuation.
/// ```
/// use piglatin::convert;
/// 
/// let text = "\"Quotes, commas and other punctuation\" aren't handled correctly.";
/// let should_be = "\"uotes-Qay, ommas-cay and-hay other-hay unctuation-pay\" aren't-hay andled-hay orrectly-cay.";
/// let actual_output = "Quotes,-\"ay ommas-cay and-hay other-hay unctuation\"-pay aren't-hay andled-hay orrectly.-cay";
/// 
/// let output = convert(text);
/// assert_ne!(output, should_be);
/// assert_eq!(output, actual_output);
/// ```
pub fn convert(text: &str) -> String {
    let mut new_text: Vec<String> = Vec::new();

    for word in text.split_whitespace() {
        new_text.push(convert_word(word));
    }

    new_text.join(" ")
}

/// Convert a single **English** word to Pig latin.
///
/// This function may have problems with words including letters which
/// are made up of multiple characters, see [`convert`] for converting
/// full pieces of text.
///
/// # Examples
/// ```
/// use piglatin::convert_word;
///
/// assert_eq!(convert_word("rust"), "ust-ray");
/// assert_eq!(convert_word("apple"), "apple-hay");
/// ```
pub fn convert_word(word: &str) -> String {
    if word.starts_with(&VOWELS) {
        return format!("{}-hay", word);
    }

    let mut characters = word.chars();
    let first = match characters.next() {
        Some(c) => format!("-{}ay", c),
        None => "".to_string(),
    };

    return characters.collect::<String>() + &first;
}
