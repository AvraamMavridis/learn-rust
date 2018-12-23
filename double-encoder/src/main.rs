use std::collections::HashMap;

fn duplicate_encode(word:&str) -> String {
    let mut uniq_chars = HashMap::new();
    let original_string = (word.clone()).to_lowercase();

    for character in original_string.chars() {
        let new_value = uniq_chars.get(&character);
        
        let x = match new_value {
            Some(value) => value + 1,
            None => 1
        };
        uniq_chars.insert(character, x);
    }

    let mut string = String::new();

    for character in original_string.chars() {
        let times = uniq_chars.get(&character);

        match times {
            Some(&1) => string.push_str("("),
            _ => string.push_str(")"),
        };
    }

    return string
}

fn main() {
  assert_eq!(duplicate_encode("din"),"(((");
  assert_eq!(duplicate_encode("recede"),"()()()");
  assert_eq!(duplicate_encode("Success"),")())())","should ignore case");
  assert_eq!(duplicate_encode("(( @"),"))((");
}
