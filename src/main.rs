use std::fs;
use std::io::Read;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    /// Evaluate HQ9+ source code
    #[structopt(short = "e", long = "eval")]
    source_code: Option<String>,
    /// Evaluate HQ9+ source code from a file
    #[structopt(short = "f", long = "file")]
    source_file: Option<String>,
    /// Force evaluation of the `H` instruction
    #[structopt(short = "h", long = "hello-world")]
    hello_world: bool,
    /// Force evaluation of the `9` instruction
    #[structopt(short = "9", long = "nintey-nine-bottles")]
    ninety_nine_bottles: bool,
}

fn print_hello_world() {
    println!("Hello, world!\n");
}

fn print_nintey_nine_bottles_of_beer_on_the_wall() {
    for i in (1..100).rev() {
        println!("{} bottles of beer on the wall,\n{} bottles of beer.\nTake one down, pass it around,\n{} bottles of beer on the wall.", i, i, i - 1);
    }
}

fn run(source_code: String) {
    #[allow(unused_variables)]
    let mut accumulator = 0;

    for c in source_code.chars() {
        match c {
            'H' => print_hello_world(),
            '9' => print_nintey_nine_bottles_of_beer_on_the_wall(),
            'Q' => println!("{}\n", source_code),
            '+' => accumulator += 1,
            _ => (),
        }
    }
}

fn main() {
    let opt = Opt::from_args();

    if opt.hello_world {
        print_hello_world();
        return;
    }

    if opt.ninety_nine_bottles {
        print_nintey_nine_bottles_of_beer_on_the_wall();
        return;
    }

    if let Some(src) = opt.source_code {
        run(src);
        return;
    }
    if let Some(src_file) = opt.source_file {
        let mut src_file_handle =
            fs::File::open(&src_file).expect(&format!("Could not open file: {}", &src_file));
        let mut buf = String::new();
        src_file_handle
            .read_to_string(&mut buf)
            .expect(&format!("Failed to read data from file: {}", &src_file));
        run(buf);
    } else {
        eprintln!(
            "Error: please pass HQ9+ source code via the `-e` flag or as a file via the `-f` flag"
        );
        ::std::process::exit(-1);
    }
}
