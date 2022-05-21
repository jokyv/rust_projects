use clap::Arg;
use clap::Command;

fn main() {
    let matches = Command::new("echo-rs")
        .version("0.1.0")
        .author("John kyvetos")
        .about("a rust implemtation of echo")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("input text")
                .required(true)
                .allow_invalid_utf8(true)
                .min_values(1),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .help("Do not print new line")
                .takes_value(false),
        )
        .get_matches();

    let text = matches.values_of_lossy("text").unwrap();
    let omit_newline = matches.is_present("omit_newline");

    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });

    // println!("{:#?}", matches);
}
