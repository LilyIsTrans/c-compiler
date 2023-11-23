use c_compiler::tokens::tokenize;

fn main() {
    let test = include_bytes!("../test_progs/ret2.c");

    let tokens = tokenize(test);

    print!("[");

    for token in tokens {
        print!("{}, ", token);
    }

    println!("]")
}
