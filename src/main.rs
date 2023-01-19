use std::env;

mod convert;

fn run_subcommand(s: &[String]) {
    match s[0].as_str() {
	"convert" => { convert::convert(); }
	_ => { panic!("Invalid Subcommand"); }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
	convert::convert();
    }
    else if args.len() > 1 {
	run_subcommand(&args[1..])
    }
    else {
	panic!("Invalid subcommand");
    }
}
