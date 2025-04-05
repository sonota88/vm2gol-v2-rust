require "rake/clean"
CLEAN.include "bin/"
CLEAN.include "target/"

# --------------------------------

desc "Build mrclc (Mini Ruccola compiler)"
task :build => "bin/mrclc"

deps = []
deps += Dir.glob("src/*.rs").to_a
deps << "Cargo.toml"

file "bin/mrclc" => deps do
  sh "cargo build"
  sh "mkdir -p bin"
  sh "cp target/debug/vm2gol-v2 bin/mrclc"
end
