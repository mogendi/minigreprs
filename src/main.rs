
use std::env;
use std::process;

mod minigrep;
use minigrep::Ctx;

fn main() {
    let ctx = Ctx::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing args: {}", err);
        process::exit(120);
    });

    if let Err(e) = ctx.grep_ctx() {
        eprintln!("File Error: {}", e);
        process::exit(121);
    }
}

