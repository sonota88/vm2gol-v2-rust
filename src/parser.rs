use super::json;

use super::utils;
use utils::char_index;
use utils::subchars;

use super::types;
use types::Chars;
use types::List;
use types::Node;
use types::NodeId;
// use types::NodeKind;
use types::Token;

// --------------------------------

static mut TOKENS: Vec<Token> = vec![];
static mut POS: usize = 0;

// --------------------------------

fn read_tokens() {
    let input_string: String = utils::read_stdin_all();
    let chars: Chars = input_string.chars().collect();
    let mut pos: usize = 0;

    while pos < chars.len() {
        let line_size = char_index(&chars, '\n', pos).unwrap() - pos;
        let line_chars = subchars(&chars, pos, pos + line_size);
        let line = line_chars.iter().cloned().collect::<String>();

        let list = json::parse(line.as_str());

        let kind = list.get_str(1);
        let value = list.get_str(2);

        unsafe {
            TOKENS.push(Token::new(kind, value));
        }

        pos += line_size + 1;
    }
}

// --------------------------------

fn peek(offset: usize) -> &'static Token { // '
    unsafe {
        return &TOKENS[POS + offset];
    }
}

fn inc_pos() {
    unsafe {
        POS += 1;
    }
}

fn is_end() -> bool {
    unsafe {
        TOKENS.len() <= POS
    }
}

fn consume(value: &str) {
    if peek(0).value != value {
        panic!("unexpected token ({})", peek(0).value);
    }

    inc_pos();
}

// --------------------------------

fn _parse_arg() -> NodeId {
    let t = peek(0);

    if t.kind == "int" {
        inc_pos();
        let n: i32 = t.value.parse().unwrap();
        return Node::new_int(n);
    } else if t.kind == "ident" {
        inc_pos();
        let s = &t.value;
        return Node::new_str(String::from(s));
    } else {
        panic!("must not happen");
    }
}

fn parse_args() -> List {
    let mut args = List::new();

    if peek(0).value == ")" {
        return args;
    }

    args.add_node(_parse_arg());

    while peek(0).value == "," {
        consume(",");
        args.add_node(_parse_arg());
    }

    return args;
}

fn _parse_var_declare() -> List {
    let var_name = &peek(0).value;
    inc_pos();

    consume(";");

    let mut stmt = List::new();
    stmt.add_str("var");
    stmt.add_str(&var_name);

    return stmt;
}

fn _parse_var_init() -> List {
    let var_name = &peek(0).value;
    inc_pos();

    consume("=");

    let expr = parse_expr();

    consume(";");

    let mut stmt = List::new();
    stmt.add_str("var");
    stmt.add_str(&var_name);
    stmt.add_node(expr);

    return stmt;
}

fn parse_var() -> List {
    consume("var");

    match peek(1).value.as_str() {
        ";" => _parse_var_declare(),
        "=" => _parse_var_init(),
        _ => panic!("unexpected token {:?}", peek(0))
    }
}

fn is_binary_op(t: &Token) -> bool {
    t.value == "+"
    || t.value == "*"
    || t.value == "=="
    || t.value == "!="
}

fn _parse_expr_factor() -> NodeId {
    let t = peek(0);

    match t.kind.as_str() {
        "sym" => {
            consume("(");
            let expr = parse_expr();
            consume(")");
            expr
        },
        "int" => {
            inc_pos();
            let n: i32 = t.value.parse().unwrap();
            Node::new_int(n)
        },
        "ident" => {
            inc_pos();
            Node::new_str(String::from(&t.value))
        },
        _ => panic!("not supported: {:?}", t)
    }
}

fn parse_expr() -> NodeId {
    let mut expr = _parse_expr_factor();

    while is_binary_op(peek(0)) {
        let op =
            match peek(0).value.as_str() {
                "+" => "+",
                "*" => "*",
                "==" => "eq",
                "!=" => "neq",
                _ => panic!("not supported: {:?}", peek(0))
            };
        inc_pos();

        let expr_r = _parse_expr_factor();

        let mut list = List::new();
        list.add_str(op);
        list.add_node(expr);
        list.add_node(expr_r);

        expr = Node::new_list(list);
    }

    return expr;
}

fn parse_set() -> List {
    consume("set");

    let t = peek(0); inc_pos();
    let var_name = &(t.value);

    consume("=");

    let expr: NodeId = parse_expr();

    consume(";");

    let mut stmt = List::new();
    stmt.add_str("set");
    stmt.add_str(var_name);
    stmt.add_node(expr);

    return stmt;
}

fn parse_funcall() -> List {
    let t = peek(0); inc_pos();
    let fn_name = &t.value;

    consume("(");
    let args = parse_args();
    consume(")");

    let mut funcall = List::new();
    funcall.add_str(fn_name);
    funcall.add_all(&args);

    return funcall;
}

fn parse_call() -> List {
    consume("call");

    let funcall = parse_funcall();

    consume(";");

    let mut stmt = List::new();
    stmt.add_str("call");
    stmt.add_all(&funcall);

    return stmt;
}

fn parse_call_set() -> List {
    consume("call_set");

    let t = peek(0); inc_pos();
    let var_name = &t.value;

    consume("=");

    let funcall = parse_funcall();

    consume(";");

    let mut stmt = List::new();
    stmt.add_str("call_set");
    stmt.add_str(var_name);
    stmt.add_list(funcall);

    return stmt;
}

fn parse_return() -> List {
    consume("return");

    if peek(0).value == ";" {
        todo!("{:?}", peek(0));
    } else {
        let expr = parse_expr();
        consume(";");

        let mut stmt = List::new();
        stmt.add_str("return");
        stmt.add_node(expr);

        return stmt;
    }
}

fn parse_while() -> List {
    consume("while");

    consume("(");
    let expr = parse_expr();
    consume(")");

    consume("{");
    let stmts = parse_stmts();
    consume("}");

    let mut stmt = List::new();
    stmt.add_str("while");
    stmt.add_node(expr);
    stmt.add_list(stmts);

    return stmt;
}

fn _parse_when_clause() -> List {
    consume("(");
    let expr = parse_expr();
    consume(")");

    consume("{");
    let stmts = parse_stmts();
    consume("}");

    let mut when_clause = List::new();
    when_clause.add_node(expr);
    when_clause.add_all(&stmts);

    return when_clause;
}

fn parse_case() -> List {
    consume("case");

    consume("{");

    let mut when_clauses = List::new();

    while peek(0).value != "}" {
        when_clauses.add_list(_parse_when_clause());
    }

    consume("}");

    let mut stmt = List::new();
    stmt.add_str("case");
    stmt.add_all(&when_clauses);

    return stmt;
}

fn parse_vm_comment() -> List {
    consume("_cmt");
    consume("(");

    let comment = &peek(0).value;
    inc_pos();

    consume(")");
    consume(";");

    let mut stmt = List::new();
    stmt.add_str("_cmt");
    stmt.add_str(comment);

    return stmt;
}

fn parse_stmt() -> List {
    let peek_value = &peek(0).value;

    match peek_value.as_str() {
        "set" => parse_set(),
        "call" => parse_call(),
        "call_set" => parse_call_set(),
        "return" => parse_return(),
        "while" => parse_while(),
        "case" => parse_case(),
        "_cmt" => parse_vm_comment(),
        _ => {
            let mut i = 0;
            while i < 20 {
                eprintln!("{}: {:?}", i, peek(i));
                i += 1;
            }
            panic!("not yet impl ({})", peek_value)
        }
    }
}

#[allow(dead_code)]
fn parse_stmts() -> List {
    let mut stmts = List::new();

    while peek(0).value != "}" {
        stmts.add_list(parse_stmt());
    }

    return stmts;
}

fn parse_func() -> List {
    let mut func = List::new();

    inc_pos();

    let fn_name = peek(0).value.as_str();
    inc_pos();

    consume("(");
    let fn_args = parse_args();
    consume(")");

    consume("{");

    let mut stmts = List::new();
    while peek(0).value != "}" {
        if peek(0).value == "var" {
            stmts.add_list(parse_var());
        } else {
            stmts.add_list(parse_stmt());
        }
    }

    consume("}");

    func.add_str("func");
    func.add_str(fn_name);
    func.add_list(fn_args);
    func.add_list(stmts);

    return func;
}

fn parse_top_stmt() -> List {
    return parse_func();
}

fn parse_top_stmts() -> List {
    let mut top_stmts = List::new();

    top_stmts.add_str("top_stmts");

    while ! is_end() {
        top_stmts.add_list(parse_top_stmt());
    }

    return top_stmts;
}

pub fn parse() {
    read_tokens();
    let top_stmts = parse_top_stmts();
    json::print(&top_stmts);
}
