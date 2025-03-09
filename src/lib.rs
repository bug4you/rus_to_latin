mod build;

use wasm_bindgen::prelude::*;
use std::collections::HashMap;

#[wasm_bindgen]
pub fn russian_to_latin(text: &str) -> String {
    let translit_map: HashMap<char, &str> = [
        ('а', "a"), ('б', "b"), ('в', "v"), ('г', "g"), ('д', "d"),
        ('е', "e"), ('ё', "yo"), ('ж', "zh"), ('з', "z"), ('и', "i"),
        ('й', "y"), ('к', "k"), ('л', "l"), ('м', "m"), ('н', "n"),
        ('о', "o"), ('п', "p"), ('р', "r"), ('с', "s"), ('т', "t"),
        ('у', "u"), ('ф', "f"), ('х', "kh"), ('ц', "ts"), ('ч', "ch"),
        ('ш', "sh"), ('щ', "shch"), ('ъ', ""), ('ы', "y"), ('ь', ""),
        ('э', "e"), ('ю', "yu"), ('я', "ya"),
    ].iter().cloned().collect();

    text.chars()
        .map(|c| translit_map.get(&c).unwrap_or(&c.to_string().as_str()).to_string())
        .collect()
}

#[wasm_bindgen]
pub fn latin_to_russian(text: &str) -> String {
    let translit_map: HashMap<&str, char> = [
        ("a", 'а'), ("b", 'б'), ("v", 'в'), ("g", 'г'), ("d", 'д'),
        ("e", 'е'), ("yo", 'ё'), ("zh", 'ж'), ("z", 'з'), ("i", 'и'),
        ("y", 'й'), ("k", 'к'), ("l", 'л'), ("m", 'м'), ("n", 'н'),
        ("o", 'о'), ("p", 'п'), ("r", 'р'), ("s", 'с'), ("t", 'т'),
        ("u", 'у'), ("f", 'ф'), ("kh", 'х'), ("ts", 'ц'), ("ch", 'ч'),
        ("sh", 'ш'), ("shch", 'щ'), ("y", 'ы'), ("e", 'э'), ("yu", 'ю'), ("ya", 'я')
    ].iter().cloned().collect();

    let mut result = String::new();
    let mut i = 0;
    let chars: Vec<char> = text.chars().collect();

    while i < chars.len() {
        let mut found = false;
        for len in (1..=3).rev() {
            if i + len <= chars.len() {
                let slice: String = chars[i..i + len].iter().collect();
                if let Some(&rus_char) = translit_map.get(slice.as_str()) {
                    result.push(rus_char);
                    i += len;
                    found = true;
                    break;
                }
            }
        }
        if !found {
            result.push(chars[i]);
            i += 1;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(russian_to_latin("Привет, мир!"), "Privet, mir!");
        assert_eq!(latin_to_russian("Privet, mir!"), "Привет, мир!");
    }
}
