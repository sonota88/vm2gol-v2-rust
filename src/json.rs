use super::types;
use types::NodeVal;
use types::List;
use types::Chars;
use types::chars_from;

use super::utils;
use utils::substr;
use utils::subchars;
use utils::match_int;
use utils::match_str;

fn parse_list(rest: &Chars) -> (List, usize) {
    let mut pos = 1;
    let mut list = List::new();

    while pos < rest.len() {
        let c = rest[pos];

        match c {
            ']' => {
                pos += 1;
                break;
            },
            '[' => {
                let rest_temp = subchars(rest, pos, rest.len());
                let (child_list, size) = parse_list(&rest_temp);
                list.add_list(child_list);
                pos += size;
            },
            '\n' | ' ' | ',' => {
                pos += 1;
            },
            _ => {
                if 0 < match_int(rest, pos) {
                    let size = match_int(rest, pos);
                    let temp = substr(rest, pos, pos + size);
                    let n = temp.parse::<i32>().unwrap();
                    list.add_int(n);
                    pos += size;
                } else if 0 < match_str(rest, pos) {
                    let size = match_str(rest, pos);
                    let temp = substr(rest, pos + 1, pos + 1 + size);
                    list.add_str(&temp);
                    pos += size + 2;
                } else {
                    panic!("must not happen ({}) ({:?})", pos, rest);
                }
            },
        }
    }

    return (list, pos);
}

pub fn parse(json: &str) -> List {
    let chars = chars_from(json);
    let (list, _) = parse_list(&chars);
    return list;
}

fn print_indent(lv: u32) {
    let mut i = 0;
    while i < lv {
        print!("  ");
        i += 1;
    }
}

fn _print(list: &List, lv: u32) {
    print_indent(lv);
    print!("[\n");

    let mut i = 0;
    loop {
        if list.size() <= i {
            break;
        }

        let node = &list.get(i);
        match &node.val {
            NodeVal::Int(n) => {
                print_indent(lv + 1);
                print!("{}", n);
            },
            NodeVal::Str(s) => {
                print_indent(lv + 1);
                print!("\"{}\"", s);
            },
            NodeVal::List(list) => {
                _print(list, lv + 1)
            },
        };

        if i < list.size() - 1 {
            print!(",");
        }
        print!("\n");
        i += 1;
    }
    print_indent(lv);
    print!("]");
}

pub fn print(list: &List) {
    _print(list, 0);
}
