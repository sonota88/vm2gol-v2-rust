use super::types::List;
use super::utils;
use super::json;

#[allow(dead_code)]
fn test_json_1() {
    let list = List::new(); // dummy

    json::print(&list);
}

#[allow(dead_code)]
fn test_json_2() {
    let mut list = List::new();
    list.add_int(1);

    json::print(&list);
}

#[allow(dead_code)]
fn test_json_3() {
    let mut list = List::new();
    list.add_str("fdsa");
    json::print(&list);
}

#[allow(dead_code)]
fn test_json_4() {
    let mut list = List::new();
    list.add_int(-123);
    list.add_str("fdsa");
    json::print(&list);
}

#[allow(dead_code)]
fn test_json_5() {
    let mut list = List::new();
    let child_list = List::new();
    list.add_list(child_list);
    json::print(&list);
}

#[allow(dead_code)]
fn test_json_6() {
    let mut list = List::new();

    list.add_int(1);
    list.add_str("a");

    let mut child_list = List::new();
    child_list.add_int(2);
    child_list.add_str("b");

    list.add_list(child_list);

    list.add_int(3);
    list.add_str("c");

    // println!("{:?}", list.get(0));
    // println!("{:?}", list.get(1));
    // println!("{:?}", list.get(2));
    // println!("{:?}", list.get(3));
    // println!("{:?}", list.get(4));

    json::print(&list);
}

fn test_json_parse() {
    let input: String = utils::read_stdin_all();
    let json = input.as_str();
    let list = json::parse(&json);

    json::print(&list);
}

pub fn test_json() {
    // test_json_1();
    // test_json_2();
    // test_json_3();
    // test_json_4();
    // test_json_5();
    // test_json_6();

    test_json_parse();
}
