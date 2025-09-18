use crate::charset;
use crate::password::PwArgs;
use crate::utils::sanitize_pattern;
use log::debug;
use std::collections::{BTreeMap};

fn get_total_chars(scale_map: &BTreeMap<usize, usize>) -> usize {
    // Characters with scale 0 appear once.
    let static_chars = scale_map.values().filter(|&&v| v == 0).count();
    // Characters with scale > 0 appear 'value' times.
    let scaled_chars: usize = scale_map.values().sum();
    static_chars + scaled_chars
}

fn get_scale_map(pattern: String) -> BTreeMap<usize, usize> {
    let mut scale_map = BTreeMap::new();
    let mut multiplier: usize = 0;
    for i in 0..pattern.len() {
        match pattern.chars().nth(i) {
            Some('(') => multiplier += 1,
            Some(')') => multiplier = multiplier.saturating_sub(1),
            _ => {
                scale_map.insert(i, multiplier);
            }
        }
    }

    scale_map
}

fn scale_pattern_for_length(pattern: String, length: usize) -> String {
    let scale_map = get_scale_map(pattern.clone());
    let output = generate_scaled_pattern(pattern, scale_map, length);
    output
}

fn generate_scaled_pattern(
    pattern: String,
    scale_map: BTreeMap<usize, usize>,
    length: usize,
) -> String {
    let mut char_map = scale_map;
    let mut current_index = 0;
    let mut current_length = get_total_chars(&char_map);
    while current_length < length {
        if current_index >= char_map.len() {
            current_index = 0;
        }

        let (&char_index, &scale) = char_map.iter().nth(current_index).unwrap();
        if scale > 0 {
            char_map.insert(char_index, scale + 1);
            current_length += 1;
        }

        current_index += 1;
    }

    let mut output = String::new();
    for (key, value) in char_map {
        let current_char = pattern.chars().nth(key).unwrap();
        if value == 0 {
            output.push(current_char);
        } else {
            for _ in 0..value {
                output.push(current_char)
            }
        }
    }

    output
}

fn scale_pattern(pattern: String, length: usize) -> String {
    if length <= pattern.trim_matches(&['(', ')']).len() {
        pattern
    } else {
        let sanitized_pattern = sanitize_pattern(pattern);
        debug!("Scaling with sanitized pattern {}", sanitized_pattern);
        scale_pattern_for_length(sanitized_pattern, length)
    }
}

pub fn generate_password(pw_args: PwArgs) -> Result<String, String> {
    let PwArgs {
        pattern,
        length,
        format,
    } = pw_args;
    if pattern == None && format {
        Err(String::from("Cannot format password without a pattern."))
    } else if format {
        let pattern = Some(scale_pattern(pattern.unwrap(), length));
        Ok(generate_for_length(pattern, length))
    } else {
        Ok(generate_for_length(pattern, length))
    }
}

fn generate_for_length(pattern: Option<String>, length: usize) -> String {
    let charset = charset::default_charset();
    let mut generated = String::new();
    match pattern {
        None => {
            for _ in 0..length {
                let source = *charset.get(&'_').unwrap();
                let source_length = source.len();
                let random_index = rand::random::<usize>() % source_length;
                let random_char = source.chars().nth(random_index).unwrap();
                generated = format!("{}{}", generated, random_char);
            }
        }
        Some(pattern) => {
            let chars_to_trim: &[char] = &['(', ')'];
            let trimmed_pattern: &str = pattern.trim_matches(chars_to_trim);
            let pattern_length = trimmed_pattern.len();
            for source_index in 0..pattern_length {
                let source_key = pattern.chars().nth(source_index).unwrap();
                let source = *charset.get(&source_key).unwrap();
                let source_length = source.len();
                let random_index = rand::random::<usize>() % source_length;
                let random_char = source.chars().nth(random_index).unwrap();
                generated = format!("{}{}", generated, random_char);
            }
        }
    }
    generated
}

#[cfg(test)]
mod tests {
    use crate::logger::set_verbose;
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_generate_for_pattern() {
        let length = 4;
        let pattern = "snu";
        let generated = generate_for_length(
            Some(String::from(pattern)),
            length,
        );
        assert_eq!(generated.len(), 3);
    }

    #[test]
    fn test_generate_for_length() {
        let length = 4;
        let generated = generate_for_length(None, length);
        assert_eq!(generated.len(), 4);
    }

    #[test]
    fn test_scale_pattern_for_exact() {
        let pattern = "snu";
        let length = 9;
        let scaled = scale_pattern(String::from(pattern), length);
        assert_eq!("sssnnnuuu", scaled);
    }

    #[test]
    fn test_scale_pattern_for_inexact() {
        set_verbose(true);
        let pattern = "snu";
        let length = 10;
        let scaled = scale_pattern(String::from(pattern), length);
        assert_eq!("ssssnnnuuu", scaled);
    }

    #[test]
    fn test_scale_pattern_if_needed_for_long_pattern() {
        let pattern = "snusnusnu";
        let length = 6;
        let scaled = scale_pattern(String::from(pattern), length);
        assert_eq!("snusnusnu", scaled);
    }

    #[test]
    fn test_get_scale_map_with_parenthesis() {
        let pattern = "snu(l)";
        let scale_map = get_scale_map(String::from(pattern));
        let expected = BTreeMap::from([(0, 0), (1, 0), (2, 0), (4, 1)]);
        assert_eq!(scale_map, expected);
    }

    #[test]
    fn test_get_scale_map_with_double_parenthesis() {
        let pattern = "s((nu))(l)";
        let scale_map = get_scale_map(String::from(pattern), );
        let expected = BTreeMap::from([(0, 0), (3, 2), (4, 2), (8, 1)]);
        assert_eq!(scale_map, expected);
    }
}
