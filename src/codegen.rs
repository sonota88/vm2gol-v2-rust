use super::utils;

use super::types;
use types::Node;
use types::NodeVal;
use types::List;

use super::json;

static mut LABEL_ID: u32 = 0;

fn get_label_id() -> u32 {
    unsafe {
        LABEL_ID += 1;
        return LABEL_ID;
    }
}

fn asm_prologue() {
    println!("  push bp");
    println!("  cp sp bp");
}

fn asm_epilogue() {
    println!("  cp bp sp");
    println!("  pop bp");
}

// --------------------------------

fn str_index(list: &List, s: &str) -> i32 {
    let mut i: usize = 0;
    while i < list.size() {
        let node = list.get(i);
        match &node.val {
            NodeVal::Str(str_val) => {
                if str_val == s {
                    return i as i32;
                }
            }
            _ => ()
        }
        i += 1;
    }

    return -1;
}

fn to_lvar_disp(lvar_names: &List, lvar_name: &str) -> i32 {
    let i = str_index(lvar_names, lvar_name);
    return - (i + 1);
}

fn to_fn_arg_disp(fn_arg_names: &List, fn_arg_name: &str) -> i32 {
    let i = str_index(fn_arg_names, fn_arg_name);
    return i + 2;
}

// --------------------------------

fn _gen_expr_add() {
    println!("  pop reg_b");
    println!("  pop reg_a");
    println!("  add_ab");
}

fn _gen_expr_mult() {
    println!("  pop reg_b");
    println!("  pop reg_a");
    println!("  mult_ab");
}

fn _gen_expr_eq() {
    let label_id = get_label_id();

    let label_end = format!("end_eq_{}", label_id);
    let label_then = format!("then_{}", label_id);

    println!("  pop reg_b");
    println!("  pop reg_a");

    println!("  compare");
    println!("  jump_eq {}", label_then);

    println!("  cp 0 reg_a");
    println!("  jump {}", label_end);

    println!("label {}", label_then);
    println!("  cp 1 reg_a");

    println!("label {}", label_end);
}

fn _gen_expr_neq() {
    let label_id = get_label_id();

    let label_end = format!("end_neq_{}", label_id);
    let label_then = format!("then_{}", label_id);

    println!("  pop reg_b");
    println!("  pop reg_a");

    println!("  compare");
    println!("  jump_eq {}", label_then);

    println!("  cp 1 reg_a");
    println!("  jump {}", label_end);

    println!("label {}", label_then);
    println!("  cp 0 reg_a");

    println!("label {}", label_end);
}

fn _gen_expr_binary(fn_arg_names: &List, lvar_names: &List, list: &List) {
    gen_expr(fn_arg_names, lvar_names, list.get(1));
    println!("  push reg_a");
    gen_expr(fn_arg_names, lvar_names, list.get(2));
    println!("  push reg_a");

    match list.get_str(0) {
        "+"  => _gen_expr_add(),
        "*"  => _gen_expr_mult(),
        "==" => _gen_expr_eq(),
        "!=" => _gen_expr_neq(),
        _ => panic!("unsupported: {:?}", list)
    }
}

fn gen_expr(fn_arg_names: &List, lvar_names: &List, expr: &Node) {
    match &expr.val {
        NodeVal::Int(n) => {
            println!("  cp {} reg_a", n);
        },
        NodeVal::List(list) => {
            _gen_expr_binary(fn_arg_names, lvar_names, &list);
        },
        NodeVal::Str(s) => {
            if 0 <= str_index(lvar_names, s) {
                let disp = to_lvar_disp(lvar_names, s);
                println!("  cp [bp:{}] reg_a", disp);
            } else if 0 <= str_index(fn_arg_names, s) {
                let disp = to_fn_arg_disp(fn_arg_names, s);
                println!("  cp [bp:{}] reg_a", disp);
            } else {
                todo!("{:?}", expr);
            }
        }
    }
}

fn gen_call(fn_arg_names: &List, lvar_names: &List, stmt: &List) {
    let fn_name = stmt.get_str(1);
    let fn_args = stmt.rest(2);

    if 0 < fn_args.size() {
        let mut i = fn_args.size() - 1;
        loop {
            gen_expr(fn_arg_names, lvar_names, fn_args.get(i));
            println!("  push reg_a");
            if i == 0 {
                break;
            }
            i -= 1;
        }
    }

    let mut vm_comment_stmt = List::new();
    vm_comment_stmt.add_str("_cmt");
    vm_comment_stmt.add_str(&format!("call  {}", fn_name));
    gen_vm_comment(&vm_comment_stmt);

    println!("  call {}", fn_name);
    println!("  add_sp {}", fn_args.size());
}

fn gen_call_set(fn_arg_names: &List, lvar_names: &List, stmt: &List) {
    let lvar_name = stmt.get_str(1);
    let funcall = stmt.get_list(2);

    let mut call_stmt = List::new();
    call_stmt.add_str("call");
    call_stmt.add_all(funcall);

    gen_call(fn_arg_names, lvar_names, &call_stmt);

    let disp = to_lvar_disp(lvar_names, lvar_name);
    println!("  cp reg_a [bp:{}]", disp);
}

fn _gen_set(
    fn_arg_names: &List,
    lvar_names: &List,
    dest: &str,
    expr: &Node
) {
    gen_expr(fn_arg_names, lvar_names, expr);

    if 0 <= str_index(lvar_names, dest) {
        let disp = to_lvar_disp(lvar_names, dest);
        println!("  cp reg_a [bp:{}]", disp);
    } else {
        unimplemented!("{:?} {}", lvar_names, dest);
    }
}

fn gen_set(fn_arg_names: &List, lvar_names: &List, stmt: &List) {
    let dest = stmt.get_str(1);
    let expr = stmt.get(2);

    _gen_set(fn_arg_names, lvar_names, dest, expr);
}

fn gen_return(fn_arg_names: &List, lvar_names: &List, stmt: &List) {
    let expr = stmt.get(1);
    gen_expr(fn_arg_names, lvar_names, expr);
}

fn gen_while(fn_arg_names: &List, lvar_names: &List, stmt: &List) {
    let cond_expr = stmt.get(1);
    let stmts = stmt.get_list(2);

    let label_id = get_label_id();

    let label_begin = format!("while_{}", label_id);
    let label_end = format!("end_while_{}", label_id);

    println!("");

    println!("label {}", label_begin);

    gen_expr(fn_arg_names, lvar_names, cond_expr);

    println!("  cp 0 reg_b");
    println!("  compare");

    println!("  jump_eq {}", label_end);

    gen_stmts(fn_arg_names, lvar_names, stmts);

    println!("  jump {}", label_begin);

    println!("label {}", label_end);
    println!("");
}

fn gen_case(fn_arg_names: &List, lvar_names: &List, stmt: &List) {
    let label_id = get_label_id();

    let mut when_idx = -1;

    let label_end = format!("end_case_{}", label_id);
    let label_end_when_head = format!("end_when_{}", label_id);

    println!("");
    println!("  # -->> case_{}", label_id);

    let mut i = 1;
    while i < stmt.size() {
        let when_clause = stmt.get_list(i);
        when_idx += 1;
        let cond = when_clause.get(0);
        let stmts = when_clause.rest(1);

        gen_expr(fn_arg_names, lvar_names, cond);

        println!("  cp 0 reg_b");

        println!("  compare");
        println!("  jump_eq {}_{}", label_end_when_head, when_idx);

        gen_stmts(fn_arg_names, lvar_names, &stmts);

        println!("  jump {}", label_end);

        println!("label {}_{}", label_end_when_head, when_idx);

        i += 1;
    }

    println!("label {}", label_end);
    println!("");
}

fn gen_vm_comment(stmt: &List) {
    println!("  _cmt {}", stmt.get_str(1).replace(" ", "~"));
}

fn gen_debug() {
    println!("  _debug");
}

fn gen_stmt(fn_arg_names: &List, lvar_names: &List, stmt: &List) {
    let head = stmt.get_str(0);

    match head {
        "set" => gen_set(fn_arg_names, lvar_names, stmt),
        "call" => gen_call(fn_arg_names, lvar_names, stmt),
        "call_set" => gen_call_set(fn_arg_names, lvar_names, stmt),
        "return" => gen_return(fn_arg_names, lvar_names, stmt),
        "while" => gen_while(fn_arg_names, lvar_names, stmt),
        "case" => gen_case(fn_arg_names, lvar_names, stmt),
        "_cmt" => gen_vm_comment(stmt),
        "_debug" => gen_debug(),
        _ => panic!("{:?}", stmt)
    }
}

fn gen_stmts(fn_arg_names: &List, lvar_names: &List, stmts: &List) {
    let mut i = 0;
    while i < stmts.size() {
        let stmt = stmts.get_list(i);
        gen_stmt(fn_arg_names, lvar_names, stmt);
        i += 1;
    }
}

fn gen_var(fn_arg_names: &List, lvar_names: &List, stmt: &List) {
    println!("  add_sp -1");

    if stmt.size() == 3 {
        let dest = stmt.get_str(1);
        let expr = stmt.get(2);
        _gen_set(fn_arg_names, lvar_names, dest, expr);
    }
}

fn gen_func(func: &List) {
    let fn_name = func.get_str(1);
    let fn_arg_names = func.get_list(2);
    let body: &List = func.get_list(3);

    println!("");
    println!("label {}", fn_name);

    asm_prologue();

    let mut lvar_names = List::new();

    let mut i = 0;
    while i < body.size() {
        let stmt = body.get_list(i);
        let head = stmt.get_str(0);
        if head == "var" {
            lvar_names.add_node(stmt.get(1).id);
            gen_var(fn_arg_names, &lvar_names, stmt);
        } else {
            gen_stmt(fn_arg_names, &lvar_names, stmt);
        }
        i += 1;
    }

    asm_epilogue();

    println!("  ret");
}

fn gen_top_stmt(top_stmt: &List) {
    gen_func(top_stmt);
}

fn gen_top_stmts(top_stmts: &List) {
    let mut i = 1;
    while i < top_stmts.size() {
        let top_stmt = top_stmts.get_list(i);
        gen_top_stmt(top_stmt);
        i += 1;
    }
}

fn gen_builtin_set_vram() {
    println!("");
    println!("label set_vram");

    asm_prologue();
    println!("  set_vram [bp:2] [bp:3]"); // vram_addr value
    asm_epilogue();
    println!("  ret");
}

fn gen_builtin_get_vram() {
    println!("");
    println!("label get_vram");

    asm_prologue();
    println!("  get_vram [bp:2] reg_a"); // vram_addr dest
    asm_epilogue();
    println!("  ret");
}

pub fn main() {
    let input: String = utils::read_stdin_all();

    let top_stmts = json::parse(input.as_str());

    println!("  call main");
    println!("  exit");

    gen_top_stmts(&top_stmts);

    println!("#>builtins");
    gen_builtin_set_vram();
    gen_builtin_get_vram();
    println!("#<builtins");
}
