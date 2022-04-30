素朴な自作言語のコンパイラをRustに移植した - memo88  
https://memo88.hatenablog.com/entry/20210407_vm2gol_v2_rust


```
$ rustc -V
rustc 1.49.0 (e1884a8e3 2020-12-29)
```


```
git clone https://github.com/sonota88/vm2gol-v2-rust.git --recursive
cd vm2gol-v2-rust
./test.sh all
```


```
LANG=C wc -l src/{utils,json,types}.rs src/{lexer,parser,codegen,main}.rs
   96 src/utils.rs
  107 src/json.rs
  176 src/types.rs
  132 src/lexer.rs
  450 src/parser.rs
  408 src/codegen.rs
   22 src/main.rs
 1391 total

cat src/{utils,json,types}.rs src/{lexer,parser,codegen,main}.rs \
  | grep -v '^ *//' \
  | wc -l
1380
```
