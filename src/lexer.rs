use super::utils;
use utils::char_index;
use utils::substr;
use utils::match_str;
use utils::match_int;

use super::types;
use types::Chars;

fn is_ident_char(c: char) -> bool {
    ('a' <= c && c <= 'z') || ('0' <= c && c <= '9') || c == '_'
}

fn find_non_ident_char(input: &Chars, start_pos: usize) -> usize {
    let mut pos = start_pos;

    while is_ident_char(input[pos]) {
        pos += 1;
    }

    return pos;
}

fn match_ident(input: &Chars, start_pos: usize) -> usize {
    let end_pos = find_non_ident_char(input, start_pos);
    return end_pos - start_pos;
}

fn match_sym_one_char(input: &Chars, start_pos: usize) -> usize {
    let c = input[start_pos];
    match c {
        '(' | ')' | '{' | '}' | ';' | '=' | ',' | '+' | '*' => 1,
        _ => 0
    }
}

fn match_sym(input: &Chars, start_pos: usize) -> usize {
    if input.len() - start_pos < 2 {
        match_sym_one_char(input, start_pos)
    } else {
        let head = substr(input, start_pos, start_pos + 2);

        if head == "==" || head == "!=" {
            2
        } else {
            match_sym_one_char(input, start_pos)
        }
    }
}

fn match_comment(input: &Chars, start_pos: usize) -> usize {
    if input.len() - start_pos < 2 {
        0
    } else {
        let c0 = input[start_pos];
        let c1 = input[start_pos + 1];

        if c0 == '/' && c1 == '/' {
            let end_pos = char_index(input, '\n', start_pos).unwrap();
            return end_pos - start_pos;
        } else {
            0
        }
    }
}

fn print_token(kind: &str, input: &Chars, pos: usize, size: usize) {
    let value = substr(input, pos, pos + size);
    println!("{}:{}", kind, value);
}

fn is_kw(value: &str) -> bool {
    match value {
        "func" | "var" | "set" | "call_set" | "call" |
        "return" | "case" | "while" | "_cmt" => {
            true
        },
        _ => {
            false
        }
    }
}

pub fn tokenize() {
    let input: String = utils::read_stdin_all();
    let chars: Chars = input.chars().collect();

    let mut pos: usize = 0;
    let mut lineno: usize = 1;

    while pos < chars.len() {
        if 0 < match_sym(&chars, pos) {
            let size = match_sym(&chars, pos);
            print_token("sym", &chars, pos, size);
            pos += size;
        } else if 0 < match_comment(&chars, pos) {
            let size = match_comment(&chars, pos);
            pos += size;
        } else if 0 < match_int(&chars, pos) {
            let size = match_int(&chars, pos);
            print_token("int", &chars, pos, size);
            pos += size;
        } else if 0 < match_str(&chars, pos) {
            let size = match_str(&chars, pos);
            print_token("str", &chars, pos + 1, size);
            pos += size + 2;
        } else if 0 < match_ident(&chars, pos) {
            let size = match_ident(&chars, pos);
            let value = substr(&chars, pos, pos + size);
            if is_kw(&value) {
                print_token("kw", &chars, pos, size);
            } else {
                print_token("ident", &chars, pos, size);
            }
            pos += size;
        } else {
            let c = chars[pos];
            match c {
                ' ' => {
                    pos += 1
                },
                '\n' => {
                    pos += 1;
                    lineno += 1;
                },
                _ => {
                    panic!("must not happen: pos({}) line({}) c({})", pos, lineno, c);
                },
            }
        }
    }
}
