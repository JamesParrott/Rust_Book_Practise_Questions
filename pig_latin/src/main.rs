use unicode_segmentation::UnicodeSegmentation;
use get_args::get_args;

fn main() {
    println!("Translation into Pig Latin: ");
    let args = get_args();
    for arg in args{
        let translated = translate_text(&arg);
        print!("{}, ", translated);
    }
}

fn translate_text(text : &String) -> String {  // String.replace_range is used 
                                               // below, so Deref coercion of 
                                               // &String to &str is not 
                                               // allowed.
    let mut translation = text.clone();
    let words: Vec<_> = text.unicode_words().collect();
    let mut offset = 0;
    for (index, word) in text.unicode_word_indices(){
        if words.contains(&word) {
            let new_word = translate_word(word);
            translation.replace_range((index + offset)..(index + offset + word.len())
                                     ,&new_word
                                     );
            offset += new_word.len() - word.len();
        }
    }
    translation
}

fn translate_word(word : &str) -> String{
    let mut graphemes: Vec<&str> = word.graphemes(true).collect();
    let len = graphemes.len();
    let mut translated = String::from(word.clone());
    let first_grapheme = graphemes[0];
    
    translated.push('-');
    if is_vowel(first_grapheme) {
        translated.push('h');

    } else if len >= 2 {
        let mut first_consonant = translated.remove(0);  // any other index than 0
                                                               // needs much more care
        if first_consonant.is_uppercase() {
            first_consonant = first_consonant.to_lowercase()
                                             .next()
                                             .expect("Could not map to lower case");
            let mut next_char = translated.remove(0);
            next_char = next_char.to_uppercase()
                                 .next()
                                 .expect("could not map to upper case. ");
            translated = String::from(next_char) + &translated;
        }
        translated.push(first_consonant);
    }
    translated.push_str("ay");

    translated
}

fn is_vowel(grapheme : &str) -> bool{
    "aeiou".contains(grapheme)
}