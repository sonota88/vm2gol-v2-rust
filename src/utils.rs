use std::io::prelude::*;

use super::types;
use types::Chars;

pub fn read_stdin_all() -> String {
    let mut bytes: Vec<u8> = vec!();

    for byte_opt in std::io::stdin().bytes() {
        let byte: u8 = byte_opt.unwrap();
        bytes.push(byte);
    }

    let u8str = String::from_utf8(bytes).unwrap();

    u8str
}

pub fn char_index(chars: &Chars, target: char, from: usize) -> Option<usize> {
    let mut i = from;

    while i < chars.len() {
        if chars[i] == target {
            return Some(i);
        }
        i += 1;
    }

    return None;
}

pub fn substr(s: &Chars, from: usize, to: usize) -> String {
    let mut s2 = String::new();

    let mut i = from;
    while i < to {
        s2.push(s[i]);
        i += 1;
    }

    return s2;
}

pub fn subchars(chars: &Chars, from: usize, to: usize) -> Chars {
    let s = substr(chars, from, to);
    return types::chars_from(s.as_str());
}

pub fn is_digit(c: char) -> bool {
    match c {
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => true,
        _ => false,
    }
}

pub fn non_digit_index(s: &Chars, start_pos: usize) -> usize {
    let mut i = start_pos;

    while i < s.len() {
        let c = s[i];
        if !(is_digit(c) || c == '-') {
            return i;
        }
        i += 1;
    }

    return i;
}

pub fn match_int(input: &Chars, start_pos: usize) -> usize {
    let c = input[start_pos];
    if ! (is_digit(c) || c == '-') {
        return 0;
    }

    let number_start_pos =
        if c == '-' {
            start_pos + 1
        } else {
            start_pos
        };

    let end_pos = non_digit_index(input, number_start_pos);
    return end_pos - start_pos;
}

pub fn match_str(input: &Chars, start_pos: usize) -> usize {
    let c = input[start_pos];
    if c != '"' {
        return 0;
    }

    let end_pos = char_index(input, '"', start_pos + 1).unwrap();

    return end_pos - start_pos - 1;
}
