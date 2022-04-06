mod document;
mod query;
mod repl;
mod storage;

fn main() {
    repl::run().unwrap();
}
