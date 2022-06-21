use piglatin;

fn main() {
    let text = "\"Quotes, commas and other punctuation\" \
        aren't handled correctly.";
    let output = piglatin::convert(text);
    println!("{}", output);
}
